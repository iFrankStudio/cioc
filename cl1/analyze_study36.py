#!/usr/bin/env python3
"""Study36: retrospective compute inventory and native scheduling deadlock audit.

This diagnostic reuses Study35 and creates ZERO independent worlds.
"""
from __future__ import annotations

import argparse
import hashlib
import json
import sqlite3
import statistics
from collections import Counter, defaultdict
from pathlib import Path

from analyze_study14 import simulated_state

FIRST = 2_700_000
SEEDS = 192
EPOCHS = 24
ARMS = (
    "control", "mutate-g2", "empty-pool", "mutate-g2-empty-pool",
    "refill", "mutate-g2-refill", "sham", "exact-original-g2",
    "zero-credit",
)
SOURCE_REVISION = "9ee57f2536b8cd4eed2a84817f2e7e777d9bcc00"
NO_REFILL = {
    "control", "mutate-g2", "sham", "exact-original-g2",
    "empty-pool", "mutate-g2-empty-pool", "zero-credit",
}


def inventory(state: dict) -> int:
    return state["compute_pool"] + sum(
        p["credits"] for p in state["processes"].values()
    )


def runnable(state: dict) -> bool:
    """Exact predicate implemented by Rust World::step scheduler."""
    return any(
        not p["stopped"] and p["credits"] > 0
        for p in state["processes"].values()
    )


def classify(state: dict) -> str:
    """Disjoint categories for a non-runnable state with stranded compute."""
    if runnable(state):
        return "runnable"
    active = any(not p["stopped"] for p in state["processes"].values())
    positive_pool = state["compute_pool"] > 0
    if positive_pool and active:
        return "pool_positive_active_zero_credit"
    if positive_pool:
        return "pool_positive_all_stopped"
    if active:
        return "pool_zero_active_zero_credit"
    return "pool_zero_all_stopped"


def digest(raw: str) -> str:
    return hashlib.sha256(raw.encode("utf-8")).hexdigest()


def native_event(event: dict) -> bool:
    return isinstance(event["origin"], dict) and "Internal" in event["origin"]


def audit(path: Path) -> dict:
    db = sqlite3.connect(f"{path.resolve().as_uri()}?mode=ro", uri=True)
    try:
        rows = db.execute(
            "SELECT id,condition,source_revision,initial_sha256,initial_json,"
            "final_sha256,final_json,attempts_json FROM epoch_runs ORDER BY id"
        )
        worlds = defaultdict(dict)
        counts = Counter()
        control_initial = []
        control_final = []
        control_pool = []
        control_stranded = []
        witnesses = []
        num_runs = 0
        hash_count = 0
        for run_id, label, revision, initial_sha, initial_raw, final_sha, final_raw, attempts_raw in rows:
            if not label.startswith("seed=") or "/" not in label:
                raise ValueError("unregistered Study35 label")
            seedpart, arm = label.split("/", 1)
            seed = int(seedpart.removeprefix("seed="))
            if seed not in range(FIRST, FIRST + SEEDS) or arm not in ARMS:
                raise ValueError("unregistered Study35 seed or arm")
            if arm in worlds[seed]:
                raise ValueError("duplicate paired arm")
            if revision != SOURCE_REVISION:
                raise ValueError("wrong historical Study35 pinned revision")
            if digest(initial_raw) != initial_sha or digest(final_raw) != final_sha:
                raise ValueError("initial/final state SHA256 integrity failure")
            initial, final = json.loads(initial_raw), json.loads(final_raw)
            initial_balance = inventory(initial)
            if initial_balance != 189:
                raise ValueError("Study35 prefork 189-credit world invariant broken")
            attempts = json.loads(attempts_raw)
            if len(attempts) != EPOCHS or any(
                not isinstance(n, int) or n < 0 or n > 12 for n in attempts
            ):
                raise ValueError("invalid native per-epoch attempts ledger")
            previous_events = initial["events"]
            previous_snapshot = initial
            at_epoch15 = None
            last_raw = None
            observed_epochs = 0
            for epoch, expected_sha, raw in db.execute(
                "SELECT epoch,state_sha256,state_json FROM epoch_snapshots "
                "WHERE run_id=? ORDER BY epoch", (run_id,)
            ):
                if epoch != observed_epochs or digest(raw) != expected_sha:
                    raise ValueError("epoch ordering or immutable digest mismatch")
                state = json.loads(raw)
                current_events = state["events"]
                if current_events[:len(previous_events)] != previous_events:
                    raise ValueError("event history rewritten between epochs")
                newly_charged = sum(
                    native_event(e)
                    for e in current_events[len(previous_events):]
                )
                if newly_charged != attempts[epoch]:
                    raise ValueError("native attempt ledger is not event-replay consistent")
                previous_events = current_events
                previous_snapshot = state
                if arm in NO_REFILL:
                    external_removed = (
                        48 if "empty-pool" in arm else
                        141 if arm == "zero-credit" else 0
                    )
                    if inventory(state) != 189 - external_removed - sum(attempts[:epoch + 1]):
                        raise ValueError("native charged-attempt inventory conservation violated")
                if epoch == 15:
                    at_epoch15 = state
                observed_epochs += 1
                hash_count += 1
                last_raw = raw
            if observed_epochs != EPOCHS or last_raw != final_raw:
                raise ValueError("incomplete 24-epoch native snapshot replay")
            if at_epoch15 is None:
                raise ValueError("W3 entry snapshot missing")
            w3_attempts = sum(attempts[16:24])
            if arm in NO_REFILL and not runnable(at_epoch15) and w3_attempts != 0:
                raise ValueError("unrunnable state performed native W3 work without external credits")
            if arm == "zero-credit" and sum(attempts) != 0:
                raise ValueError("zero-credit negative control unexpectedly executed")
            initial_state = simulated_state(initial)
            final_state = simulated_state(final)
            worlds[seed][arm] = {
                "initial_sha": initial_sha, "final_simulated": final_state,
                "initial_simulated": initial_state,
                "entry16_class": classify(at_epoch15),
                "entry16_inventory": inventory(at_epoch15),
                "final_class": classify(final),
                "final_inventory": inventory(final),
                "final_pool": final["compute_pool"],
                "w3_attempts": w3_attempts,
            }
            counts[f"{arm}.entry16.{classify(at_epoch15)}"] += 1
            counts[f"{arm}.W3_active"] += int(w3_attempts > 0)
            counts[f"{arm}.end_inventory_positive"] += int(inventory(final) > 0)
            if arm == "control":
                control_initial.append(initial_balance)
                control_final.append(inventory(final))
                control_pool.append(final["compute_pool"])
                control_stranded.append(sum(
                    p["credits"] for p in final["processes"].values()
                    if p["stopped"]
                ))
                if len(witnesses) < 8 and classify(final) == "pool_positive_active_zero_credit":
                    witnesses.append({
                        "seed": seed,
                        "final_pool": final["compute_pool"],
                        "final_inventory": inventory(final),
                        "stopped_process_credits": control_stranded[-1],
                        "active_zero_credit_count": sum(
                            not p["stopped"] and p["credits"] == 0
                            for p in final["processes"].values()
                        ),
                    })
            num_runs += 1
        if (num_runs != SEEDS * len(ARMS) or hash_count != num_runs * EPOCHS
                or len(worlds) != SEEDS
                or any(set(arms) != set(ARMS) for arms in worlds.values())):
            raise ValueError("unbalanced Study35 cohort or missing epoch snapshots")
        for seed, arms in worlds.items():
            for arm in ARMS:
                if arms[arm]["initial_sha"] != arms["control"]["initial_sha"]:
                    raise ValueError("paired initial worlds mismatch")
            for arm in ("sham", "exact-original-g2"):
                if arms[arm]["final_simulated"] != arms["control"]["final_simulated"]:
                    raise ValueError("sham/original native final-state control mismatch")
        if counts["control.W3_active"] != 0:
            raise ValueError("historical Study35 W3 untreated null not reproduced")
        return {
            "study": "CL-1 Study36 retrospective compute ignition-deadlock audit",
            "source_revision": SOURCE_REVISION,
            "new_independent_worlds": 0,
            "reused_study35_worlds": SEEDS,
            "matched_runs": num_runs,
            "validated_epoch_sha256_snapshots": hash_count,
            "initial_control_compute_inventory": sorted(set(control_initial)),
            "final_control_compute_inventory": {
                "min": min(control_final),
                "median": statistics.median(control_final),
                "max": max(control_final),
            },
            "final_control_pool": {
                "min": min(control_pool),
                "median": statistics.median(control_pool),
                "max": max(control_pool),
            },
            "final_control_stopped_process_credits_median": statistics.median(control_stranded),
            "disjoint_ignition_deadlock_counts": {
                key.removeprefix("control.entry16."): n
                for key, n in sorted(counts.items())
                if key.startswith("control.entry16.")
            },
            "all_arm_registered_counts": dict(sorted(counts.items())),
            "reproducible_stranded_pool_examples": witnesses,
            "native_unbounded_execution_in_closed_finite_inventory":
                "IMPOSSIBLE WITHOUT EXTERNAL CREDIT INFLOW",
            "native_life_claim": "NONE",
        }
    finally:
        db.close()


def main() -> None:
    p = argparse.ArgumentParser(description=__doc__)
    p.add_argument("sqlite", type=Path)
    p.add_argument("--json", type=Path, required=True)
    args = p.parse_args()
    result = audit(args.sqlite)
    args.json.write_text(json.dumps(result, indent=2) + "\n", encoding="utf-8")
    print(json.dumps(result, indent=2))


if __name__ == "__main__":
    main()
