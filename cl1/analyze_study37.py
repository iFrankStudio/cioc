#!/usr/bin/env python3
"""Study37: independently audit native epochs and an EXTERNAL pool access rule.

This environment-side access policy is not autonomous self-maintenance.
"""
from __future__ import annotations

import argparse
import json
import sqlite3
from collections import Counter, defaultdict
from pathlib import Path

import analyze
from analyze_study02 import wilson_95
from analyze_study33 import expected_program_continuations
from analyze_study14 import simulated_state

FIRST = 2_800_000
SEEDS = 128
EPOCHS = 24
QUOTA = 12
ARMS = (
    "control", "pool-access", "empty-pool", "pool-access-empty-pool",
    "sham", "external-refill", "pool-access-zero-credit",
)
ACCESS_ARMS = {"pool-access", "pool-access-empty-pool", "pool-access-zero-credit"}
NULL_POOL_ARMS = {"empty-pool", "pool-access-empty-pool"}


def inventory(state: dict) -> int:
    return state["compute_pool"] + sum(
        p["credits"] for p in state["processes"].values()
    )


def runnable(state: dict) -> bool:
    return any(
        not p["stopped"] and p["credits"] > 0
        for p in state["processes"].values()
    )


def candidate(state: dict) -> tuple[int, int] | None:
    """Independent translation of the frozen generic policy; NO fitness search."""
    if runnable(state):
        return None
    pool = state["compute_pool"]
    for actor in (1, 2, 3):
        p = state["processes"][str(actor)]
        if p["stopped"] or p["credits"] != 0:
            continue
        pc = p["pc"]
        nodes = p["code"]["nodes"]
        if not isinstance(pc, int) or not 0 <= pc < len(nodes):
            continue
        op = nodes[pc]
        if not isinstance(op, dict) or "DrawCompute" not in op:
            continue
        operand = op["DrawCompute"]["amount"]
        if not isinstance(operand, int) or operand not in range(4):
            continue
        q = p["registers"][operand]
        if isinstance(q, int) and 1 <= q < (1 << 64) - 1 and pool >= q + 1:
            return actor, pool
    return None


def base_actions(arm: str, epoch: int) -> list[dict]:
    at = lambda action: {"epoch": epoch, "action": action}
    if epoch == 0:
        if arm in NULL_POOL_ARMS:
            return [at({"SetComputePool": {"credits": 0}})]
        if arm == "sham":
            return [at({"DropGraph": {"graph": 0}})]
        if arm == "pool-access-zero-credit":
            return [
                at({"SetCredits": {"process": p, "credits": 0}})
                for p in (1, 2, 3)
            ]
    if arm == "external-refill" and epoch in (8, 16):
        return [
            at({"SetCredits": {"process": p, "credits": 47}})
            for p in (1, 2, 3)
        ]
    return []


def expected_actions(previous: dict, arm: str, epoch: int) -> list[dict]:
    actions = base_actions(arm, epoch)
    if arm not in ACCESS_ARMS:
        return actions
    # Account for external baseline edits occurring immediately before policy.
    shadow = {
        **previous,
        "compute_pool": previous["compute_pool"],
        "processes": {
            k: {**v} for k, v in previous["processes"].items()
        },
    }
    if epoch == 0 and arm == "pool-access-empty-pool":
        shadow["compute_pool"] = 0
    if epoch == 0 and arm == "pool-access-zero-credit":
        for p in shadow["processes"].values():
            p["credits"] = 0
    selected = candidate(shadow)
    if selected is not None:
        actor, pool = selected
        actions.extend([
            {"epoch": epoch, "action": {"SetComputePool": {"credits": pool - 1}}},
            {"epoch": epoch, "action": {"SetCredits": {
                "process": actor, "credits": 1,
            }}},
        ])
    return actions


def native_event(e: dict) -> bool:
    return (isinstance(e["origin"], dict)
            and e["origin"].get("Internal") == e["process"])


def event_action_name(action: dict) -> str:
    return next(iter(action["action"]))


def audit(path: Path) -> dict:
    db = sqlite3.connect(f"{path.resolve().as_uri()}?mode=ro", uri=True)
    try:
        rows = db.execute(
            "SELECT id,source_revision,condition,initial_sha256,initial_json,"
            "actions_json,epoch_count,quota,attempts_json,final_sha256,final_json "
            "FROM epoch_runs ORDER BY id"
        ).fetchall()
        if len(rows) != SEEDS * len(ARMS):
            raise ValueError("not 128 independent matched seven-arm worlds")
        worlds: dict[int, dict[str, dict]] = defaultdict(dict)
        snapshots_total = 0
        metrics = Counter()
        source_revisions = set()
        initial_hashes = {}
        witness_transfers = []
        for (run_id, revision, condition, initial_hash, initial_raw, actions_raw,
             epochs, quota, attempts_raw, final_hash, final_raw) in rows:
            if not condition.startswith("seed=") or "/" not in condition:
                raise ValueError("unregistered seed label")
            prefix, arm = condition.split("/", 1)
            seed = int(prefix.removeprefix("seed="))
            if seed not in range(FIRST, FIRST + SEEDS) or arm not in ARMS:
                raise ValueError("wrong Study37 independent seed / arm")
            if arm in worlds[seed] or not revision or epochs != EPOCHS or quota != QUOTA:
                raise ValueError("duplicate or invalid scientific branch")
            source_revisions.add(revision)
            if analyze.digest(initial_raw) != initial_hash or analyze.digest(final_raw) != final_hash:
                raise ValueError("initial/final SHA256 corruption")
            if seed in initial_hashes and initial_hashes[seed] != initial_hash:
                raise ValueError("matched branches have different original world")
            initial_hashes[seed] = initial_hash
            initial = json.loads(initial_raw)
            final = json.loads(final_raw)
            if (inventory(initial) != 189 or initial["compute_pool"] != 48
                    or not initial["strict_accounting"]
                    or initial["total_storage_limit"] != 192):
                raise ValueError("incorrect initial world compute/storage endowment")
            grammar = expected_program_continuations(seed)
            for actor in (1, 2, 3):
                process = initial["processes"][str(actor)]
                if (process["credits"] != 47
                        or process["registers"][1] != actor % 3 + 1
                        or process["code"]["nodes"][9:27] != grammar[actor]):
                    raise ValueError("registered native program grammar was changed")
                if process["code"]["nodes"] != final["processes"][str(actor)]["code"]["nodes"]:
                    raise ValueError("native program changed in this frozen experiment")
            attempts = json.loads(attempts_raw)
            actions = json.loads(actions_raw)
            if len(attempts) != EPOCHS or any(
                not isinstance(n, int) or not 0 <= n <= QUOTA for n in attempts
            ):
                raise ValueError("invalid charged attempt ledger")
            previous = initial
            action_offset = 0
            access_epochs = []
            policy_draw_epochs = []
            w3_attempts = 0
            observed_snapshots = 0
            for epoch, digest, raw in db.execute(
                "SELECT epoch,state_sha256,state_json FROM epoch_snapshots "
                "WHERE run_id=? ORDER BY epoch", (run_id,)
            ):
                if epoch != observed_snapshots or analyze.digest(raw) != digest:
                    raise ValueError("missing, reordered or modified epoch SHA256")
                snap = json.loads(raw)
                expected = expected_actions(previous, arm, epoch)
                scheduled = actions[action_offset:action_offset + len(expected)]
                if scheduled != expected:
                    raise ValueError(
                        f"unregistered or absent state-derived external policy: {condition}/epoch{epoch}"
                    )
                action_offset += len(expected)
                last_len = len(previous["events"])
                current_events = snap["events"]
                if current_events[:last_len] != previous["events"]:
                    raise ValueError("cumulative native events changed")
                events = current_events[last_len:]
                if len(events) != len(expected) + attempts[epoch]:
                    raise ValueError("native events / external schedule do not balance")
                for i, action in enumerate(expected):
                    event = events[i]
                    if event["origin"] != "External":
                        raise ValueError("registered environmental transfer falsely attributed to CIL")
                    kind = event["kind"]
                    observed_type = (
                        "InterventionRejected" if event_action_name(action) == "DropGraph"
                        else "Intervened"
                    )
                    if not isinstance(kind, dict) or observed_type not in kind:
                        raise ValueError("missing registered external action provenance")
                    if event_action_name(action) not in str(kind[observed_type]):
                        raise ValueError("environment intervention kind mismatch")
                native = events[len(expected):]
                if any(not native_event(e) for e in native):
                    raise ValueError("native charged event provenance is not internal")
                if sum(attempts[:epoch + 1]) > EPOCHS * QUOTA:
                    raise ValueError("native quota exceeded")
                starting = inventory(previous)
                if arm in NULL_POOL_ARMS and epoch == 0:
                    starting -= 48
                if arm == "pool-access-zero-credit" and epoch == 0:
                    starting -= 141
                if arm == "external-refill" and epoch in (8, 16):
                    starting += sum(
                        47 - previous["processes"][str(actor)]["credits"]
                        for actor in (1, 2, 3)
                    )
                if inventory(snap) != starting - attempts[epoch]:
                    raise ValueError("compute conservation violated by native work or environment")
                access = expected[len(base_actions(arm, epoch)):]
                if access:
                    if len(access) != 2 or not native:
                        raise ValueError("external pool access failed to wake a native actor")
                    actor = access[1]["action"]["SetCredits"]["process"]
                    p = previous["processes"][str(actor)]
                    # Zero-credit treatment changes credits, not code or pc.
                    first = native[0]
                    if (first["process"] != actor or first["pc"] != p["pc"]
                            or first["kind"] != "Executed"
                            or "DrawCompute" not in p["code"]["nodes"][p["pc"]]):
                        raise ValueError("claimed environmental ignition did not execute a native DrawCompute")
                    access_epochs.append(epoch)
                    policy_draw_epochs.append(epoch)
                    if len(witness_transfers) < 16:
                        witness_transfers.append({
                            "seed": seed, "arm": arm, "epoch": epoch,
                            "actor": actor, "pool_before": (
                                0 if arm == "pool-access-empty-pool" and epoch == 0
                                else previous["compute_pool"]
                            ),
                            "pool_after": snap["compute_pool"],
                        })
                if epoch >= 16:
                    w3_attempts += attempts[epoch]
                observed_snapshots += 1
                snapshots_total += 1
                previous = snap
            if observed_snapshots != EPOCHS or action_offset != len(actions):
                raise ValueError("unconsumed external interventions or epoch count")
            if json.dumps(previous, separators=(",", ":")) != json.dumps(final, separators=(",", ":")):
                raise ValueError("final snapshot differs from epoch23")
            if arm == "control" and access_epochs:
                raise ValueError("control received unregistered pool access")
            if arm == "pool-access-empty-pool" and access_epochs:
                raise ValueError("zero world-pool negative control gained access")
            metrics[f"{arm}.W3_native_activity"] += int(w3_attempts > 0)
            metrics[f"{arm}.environment_access_any"] += int(bool(access_epochs))
            metrics[f"{arm}.environment_access_transfers"] += len(access_epochs)
            metrics[f"{arm}.native_Draw_after_access"] += len(policy_draw_epochs)
            metrics[f"{arm}.W1_native_activity"] += int(sum(attempts[:8]) > 0)
            metrics[f"{arm}.W2_native_activity"] += int(sum(attempts[8:16]) > 0)
            worlds[seed][arm] = {
                "w3": w3_attempts > 0,
                "w3_attempts": w3_attempts,
                "access_epochs": access_epochs,
                "final": final,
                "total_attempts": sum(attempts),
                "final_compute_inventory": inventory(final),
                "final_pool": final["compute_pool"],
            }
            if arm == "pool-access-zero-credit" and not access_epochs and sum(attempts) > 0:
                raise ValueError("zero-credit arm executed without external ignition")
        if (len(worlds) != SEEDS or any(set(w) != set(ARMS) for w in worlds.values())
                or len(source_revisions) != 1
                or snapshots_total != SEEDS * len(ARMS) * EPOCHS):
            raise ValueError("incomplete or mixed cohort and revision")
        seed_rows = []
        discordance = Counter()
        for seed in sorted(worlds):
            records = worlds[seed]
            if simulated_state(records["control"]["final"]) != simulated_state(records["sham"]["final"]):
                raise ValueError("sham affects native initial/final world")
            if (records["empty-pool"]["total_attempts"] !=
                    records["pool-access-empty-pool"]["total_attempts"]):
                raise ValueError("empty-pool matched negative control changed")
            c, p = records["control"], records["pool-access"]
            discordance["policy_only"] += int(p["w3"] and not c["w3"])
            discordance["control_only"] += int(c["w3"] and not p["w3"])
            discordance["both"] += int(c["w3"] and p["w3"])
            discordance["neither"] += int(not c["w3"] and not p["w3"])
            seed_rows.append({
                "seed": seed, "control_W3": c["w3"],
                "pool_access_W3": p["w3"], "pool_access_epochs": p["access_epochs"],
                "pool_access_W3_attempts": p["w3_attempts"],
                "zero_credit_ignition_epochs": records["pool-access-zero-credit"]["access_epochs"],
            })
        n_c = metrics["control.W3_native_activity"]
        n_p = metrics["pool-access.W3_native_activity"]
        return {
            "study": "CL-1 Study37 prospective external conserved pool-access",
            "source_revision": next(iter(source_revisions)),
            "independent_seed_worlds": SEEDS,
            "matched_runs": len(rows),
            "hashed_epoch_snapshots": snapshots_total,
            "registered_primary": {
                "control_W3": {
                    "worlds": n_c, "denominator": SEEDS,
                    "wilson_95": [round(x, 6) for x in wilson_95(n_c, SEEDS)],
                },
                "pool_access_W3": {
                    "worlds": n_p, "denominator": SEEDS,
                    "wilson_95": [round(x, 6) for x in wilson_95(n_p, SEEDS)],
                },
                "paired_discordance": dict(sorted(discordance.items())),
                "paired_uplift_worlds": n_p - n_c,
            },
            "secondary": dict(sorted(metrics.items())),
            "first_external_ignition_witnesses": witness_transfers,
            "all_seed_outcomes": seed_rows,
            "source_of_resource_access": "EXTERNAL ENVIRONMENT POLICY, ZERO NET CREDIT INJECTION",
            "native_actor_resource_acquisition": "NEXT-OP DRAWCOMPUTE ONLY",
            "closed_world_compute_generation": "NONE",
            "life_claim": "NONE",
        }
    finally:
        db.close()


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("sqlite", type=Path)
    parser.add_argument("--json", type=Path, required=True)
    args = parser.parse_args()
    report = audit(args.sqlite)
    args.json.write_text(json.dumps(report, indent=2) + "\n", encoding="utf-8")
    print(json.dumps({
        "study": report["study"],
        "independent_worlds": report["independent_seed_worlds"],
        "matched_runs": report["matched_runs"],
        "primary": report["registered_primary"],
        "life_claim": report["life_claim"],
    }, indent=2))


if __name__ == "__main__":
    main()
