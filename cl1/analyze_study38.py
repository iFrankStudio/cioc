#!/usr/bin/env python3
"""Study38: independently audit native epochs and an EXTERNAL pool access rule.

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

FIRST = 2_900_000
SEEDS = 128
EPOCHS = 24
QUOTA = 12
ARMS = (
    "closed", "closed-draw", "closed-general", "ambient", "ambient-draw",
    "ambient-general", "ambient-general-empty-pool", "ambient-general-zero-credit",
    "external-refill", "sham",
)
AMBIENT_ARMS = {
    "ambient", "ambient-draw", "ambient-general", "ambient-general-empty-pool",
    "ambient-general-zero-credit",
}
GENERAL_ARMS = {
    "closed-general", "ambient-general", "ambient-general-empty-pool",
    "ambient-general-zero-credit",
}
DRAW_ARMS = {"closed-draw", "ambient-draw"}


def inventory(state: dict) -> int:
    return state["compute_pool"] + sum(
        p["credits"] for p in state["processes"].values()
    )


def runnable(state: dict) -> bool:
    return any(
        not p["stopped"] and p["credits"] > 0
        for p in state["processes"].values()
    )


def candidate(state: dict, mode: str) -> tuple[int, int] | None:
    """Uniform lowest-ID environment scheduling, not behavior/reward selection."""
    if mode == "none" or runnable(state):
        return None
    pool = state["compute_pool"]
    for actor in (1, 2, 3):
        p = state["processes"][str(actor)]
        if p["stopped"] or p["credits"] != 0:
            continue
        if mode == "general":
            if pool >= 1:
                return actor, pool
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
        if isinstance(q, int) and q > 0 and pool > q:
            return actor, pool
    return None


def ignition_mode(arm: str) -> str:
    if arm in GENERAL_ARMS:
        return "general"
    if arm in DRAW_ARMS:
        return "draw"
    return "none"


def base_actions(arm: str, epoch: int) -> list[dict]:
    at = lambda action: {"epoch": epoch, "action": action}
    if epoch == 0:
        if arm == "ambient-general-empty-pool":
            return [at({"SetComputePool": {"credits": 0}})]
        if arm == "sham":
            return [at({"DropGraph": {"graph": 0}})]
        if arm == "ambient-general-zero-credit":
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
    """Derive every allowed action from the *previous* persisted world state."""
    actions = base_actions(arm, epoch)
    shadow = {
        **previous,
        "compute_pool": previous["compute_pool"],
        "processes": {k: {**v} for k, v in previous["processes"].items()},
    }
    if epoch == 0 and arm == "ambient-general-empty-pool":
        shadow["compute_pool"] = 0
    if epoch == 0 and arm == "ambient-general-zero-credit":
        for p in shadow["processes"].values():
            p["credits"] = 0
    if epoch >= 1 and arm in AMBIENT_ARMS:
        new_pool = shadow["compute_pool"] + 8
        if new_pool >= 1 << 64:
            raise ValueError("ambient pool overflow")
        actions.append({
            "epoch": epoch,
            "action": {"SetComputePool": {"credits": new_pool}},
        })
        shadow["compute_pool"] = new_pool
    selected = candidate(shadow, ignition_mode(arm))
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


def audit(path: Path, expected_revision: str) -> dict:
    db = sqlite3.connect(f"{path.resolve().as_uri()}?mode=ro", uri=True)
    try:
        rows = db.execute(
            "SELECT id,source_revision,condition,initial_sha256,initial_json,"
            "actions_json,epoch_count,quota,attempts_json,final_sha256,final_json "
            "FROM epoch_runs ORDER BY id"
        ).fetchall()
        if len(rows) != SEEDS * len(ARMS):
            raise ValueError("not 128 independent matched ten-arm worlds")
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
                raise ValueError("wrong Study38 independent seed / arm")
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
                if arm == "ambient-general-empty-pool" and epoch == 0:
                    starting -= 48
                if arm == "ambient-general-zero-credit" and epoch == 0:
                    starting -= 141
                if epoch >= 1 and arm in AMBIENT_ARMS:
                    starting += 8
                if arm == "external-refill" and epoch in (8, 16):
                    starting += sum(
                        47 - previous["processes"][str(actor)]["credits"]
                        for actor in (1, 2, 3)
                    )
                if inventory(snap) != starting - attempts[epoch]:
                    raise ValueError("compute conservation violated by native work or environment")
                ambient_length = int(epoch >= 1 and arm in AMBIENT_ARMS)
                if ambient_length:
                    metrics[f"{arm}.ambient_input_actions"] += 1
                access = expected[len(base_actions(arm, epoch)) + ambient_length:]
                if access:
                    if len(access) != 2 or not native:
                        raise ValueError("external starter failed to enable a native attempt")
                    actor = access[1]["action"]["SetCredits"]["process"]
                    p = previous["processes"][str(actor)]
                    first = native[0]
                    if first["process"] != actor or first["pc"] != p["pc"]:
                        raise ValueError("environment starter did not execute chosen native actor")
                    access_epochs.append(epoch)
                    if arm in DRAW_ARMS:
                        op = p["code"]["nodes"][p["pc"]]
                        if (not isinstance(op, dict) or "DrawCompute" not in op
                                or first["kind"] != "Executed"):
                            raise ValueError("draw-gated starter did not execute native draw")
                        policy_draw_epochs.append(epoch)
                    if len(witness_transfers) < 16:
                        witness_transfers.append({
                            "seed": seed, "arm": arm, "epoch": epoch,
                            "actor": actor,
                            "ignition_mode": ignition_mode(arm),
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
            if arm in ("closed", "ambient", "sham", "external-refill") and access_epochs:
                raise ValueError("non-ignition arm received unregistered starter")
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
            if arm == "ambient-general-zero-credit" and not access_epochs and sum(attempts) > 0:
                raise ValueError("externally zeroed actor executed without a starter")
        if (len(worlds) != SEEDS or any(set(w) != set(ARMS) for w in worlds.values())
                or source_revisions != {expected_revision}
                or snapshots_total != SEEDS * len(ARMS) * EPOCHS):
            raise ValueError("incomplete or mixed cohort and revision")
        seed_rows = []
        discordance = Counter()
        for seed in sorted(worlds):
            records = worlds[seed]
            if simulated_state(records["closed"]["final"]) != simulated_state(records["sham"]["final"]):
                raise ValueError("sham affects native closed baseline")
            d, g = records["ambient-draw"], records["ambient-general"]
            discordance["general_only"] += int(g["w3"] and not d["w3"])
            discordance["draw_only"] += int(d["w3"] and not g["w3"])
            discordance["both"] += int(d["w3"] and g["w3"])
            discordance["neither"] += int(not d["w3"] and not g["w3"])
            seed_rows.append({
                "seed": seed,
                "closed_W3": records["closed"]["w3"],
                "closed_general_W3": records["closed-general"]["w3"],
                "ambient_W3": records["ambient"]["w3"],
                "ambient_draw_W3": d["w3"],
                "ambient_general_W3": g["w3"],
                "ambient_general_starters": g["access_epochs"],
                "ambient_general_W3_attempts": g["w3_attempts"],
                "zero_credit_starters": records["ambient-general-zero-credit"]["access_epochs"],
            })
        n_c = metrics["ambient-draw.W3_native_activity"]
        n_p = metrics["ambient-general.W3_native_activity"]
        return {
            "study": "CL-1 Study38 prospective ambient supply and neutral ignition",
            "source_revision": next(iter(source_revisions)),
            "independent_seed_worlds": SEEDS,
            "matched_runs": len(rows),
            "hashed_epoch_snapshots": snapshots_total,
            "registered_primary": {
                "ambient_draw_W3": {
                    "worlds": n_c, "denominator": SEEDS,
                    "wilson_95": [round(x, 6) for x in wilson_95(n_c, SEEDS)],
                },
                "ambient_general_W3": {
                    "worlds": n_p, "denominator": SEEDS,
                    "wilson_95": [round(x, 6) for x in wilson_95(n_p, SEEDS)],
                },
                "paired_discordance": dict(sorted(discordance.items())),
                "paired_general_minus_draw_worlds": n_p - n_c,
            },
            "secondary": dict(sorted(metrics.items())),
            "first_external_ignition_witnesses": witness_transfers,
            "all_seed_outcomes": seed_rows,
            "source_of_ambient_compute": "EXTERNAL ENVIRONMENT INPUT: 8 CREDITS/EPOCHS 1..23",
            "source_of_ignition": "EXTERNAL ENVIRONMENT POLICY: ZERO NET COMPUTE INPUT",
            "native_self_maintenance": "NOT ESTABLISHED",
            "life_claim": "NONE",
        }
    finally:
        db.close()


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("sqlite", type=Path)
    parser.add_argument("--json", type=Path, required=True)
    parser.add_argument("--expected-revision", required=True)
    args = parser.parse_args()
    report = audit(args.sqlite, args.expected_revision)
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
