#!/usr/bin/env python3
"""CL-1 Study 04: independent externally sourced compute-resource audit."""
from __future__ import annotations

import argparse
import json
import sqlite3
from collections import Counter
from pathlib import Path

import analyze
from analyze_study02 import wilson_95

FIRST = 940000
SEEDS = 128
CONDITIONS = ("closed", "initial-pool", "replenished", "disable-p1", "zero-credit-pool")
HORIZON = 512
INITIAL_CREDITS = 84


def code_node_count(world: dict) -> int:
    processes = sum(len(p["code"]["nodes"]) for p in world["processes"].values())
    objects = sum(len(o["nodes"]) for o in world["graph_objects"].values())
    return processes + objects


def pool_supply_events(world: dict) -> int:
    count = 0
    for event in world["events"]:
        kind = event["kind"]
        if isinstance(kind, dict) and "Intervened" in kind and "SetComputePool" in kind["Intervened"]:
            if event["origin"] != "External":
                raise ValueError("compute supply wrongly attributed to internal action")
            count += 1
    return count


def expected_offsets(condition: str) -> list[int]:
    return {
        "closed": [],
        "initial-pool": [0],
        "replenished": [0, 128, 256, 384],
        "disable-p1": [0, 32],
        "zero-credit-pool": [0, 0, 0, 0],
    }[condition]


def process_non_target_signature(world: dict) -> dict:
    return {str(i): world["processes"][str(i)] for i in (2, 3)}


def audit(db: Path) -> dict:
    conn = sqlite3.connect(f"{db.resolve().as_uri()}?mode=ro", uri=True)
    try:
        rows = conn.execute(
            "SELECT id,source_revision,condition,initial_sha256,initial_json,"
            "actions_json,attempt_limit,attempts,final_sha256,final_json "
            "FROM scheduled_runs ORDER BY id"
        ).fetchall()
        if len(rows) != SEEDS * len(CONDITIONS):
            raise ValueError("incomplete 640-run cohort")
        checkpoints = {}
        for run_id, ordinal, offset, hash_value, json_value in conn.execute(
            "SELECT run_id,ordinal,after_attempts,state_sha256,state_json "
            "FROM scheduled_checkpoints"
        ):
            if analyze.digest(json_value) != hash_value:
                raise ValueError(f"invalid checkpoint digest {run_id}")
            checkpoints.setdefault(run_id, {})[ordinal] = offset

        grouped = {}
        source_revisions = set()
        matched_hashes = {}
        for (rid, revision, label, initial_sha, initial_json, actions_json,
             limit, attempts, final_sha, final_json) in rows:
            if not label.startswith("seed=") or "/" not in label:
                raise ValueError(f"invalid experimental condition: {label}")
            prefix, condition = label.split("/", 1)
            seed = int(prefix[5:])
            if seed not in range(FIRST, FIRST + SEEDS) or condition not in CONDITIONS:
                raise ValueError(f"unregistered seed or condition: {label}")
            if condition in grouped.get(seed, {}):
                raise ValueError(f"duplicate matched condition: {label}")
            if (
                not revision or analyze.digest(initial_json) != initial_sha
                or analyze.digest(final_json) != final_sha
                or limit != HORIZON or not 0 <= attempts <= HORIZON
            ):
                raise ValueError(f"invalid run hash or bounds: {label}")
            if seed in matched_hashes and matched_hashes[seed] != initial_sha:
                raise ValueError(f"initial snapshot not matched: {label}")
            matched_hashes[seed] = initial_sha
            source_revisions.add(revision)
            initial, final = json.loads(initial_json), json.loads(final_json)
            if (
                initial["total_storage_limit"] != 160
                or not initial["strict_accounting"]
                or analyze.occupied_units(initial) != 51
                or analyze.occupied_units(final) > 160
                or code_node_count(initial) != 48
            ):
                raise ValueError(f"invalid resource accounting: {label}")
            actions = json.loads(actions_json)
            if [x["after_attempts"] for x in actions] != expected_offsets(condition):
                raise ValueError(f"unexpected intervention schedule: {label}")
            saved = checkpoints.get(rid, {})
            if any(
                ordinal >= len(actions) or at != actions[ordinal]["after_attempts"]
                for ordinal, at in saved.items()
            ) or len(saved) > len(actions):
                raise ValueError(f"unregistered checkpoint: {label}")
            if condition == "closed" and attempts > INITIAL_CREDITS:
                raise ValueError(f"closed world created compute credits: {label}")
            if condition == "zero-credit-pool" and (
                attempts != 0
                or any(analyze.internal_process(e) is not None for e in final["events"])
            ):
                raise ValueError(f"zero-credit world ran CIL: {label}")
            supply_event_count = pool_supply_events(final)
            expected_supply_count = sum(
                ordinal in saved and "SetComputePool" in action["action"]
                for ordinal, action in enumerate(actions)
            )
            if supply_event_count != expected_supply_count:
                raise ValueError(f"pool source attribution mismatch: {label}")
            grouped.setdefault(seed, {})[condition] = {
                "final": final,
                "attempts": attempts,
                "checkpoints": saved,
                "pool_actions_reached": supply_event_count,
                "beyond_initial_budget": attempts > INITIAL_CREDITS,
                "late_256": attempts > 256,
                "late_384": attempts > 384,
                "created_process": any(int(pid) > 3 for pid in final["processes"]),
                "occupied_node_growth": code_node_count(final) > 48,
            }
        if len(source_revisions) != 1 or len(grouped) != SEEDS:
            raise ValueError("source revision mismatch or missing seeds")
        if any(set(cases) != set(CONDITIONS) for cases in grouped.values()):
            raise ValueError("incomplete condition set per seed")

        summaries = {}
        for condition in CONDITIONS:
            rows_by_seed = [grouped[seed][condition] for seed in sorted(grouped)]
            metrics = {}
            for name in (
                "beyond_initial_budget", "late_256", "late_384",
                "created_process", "occupied_node_growth",
            ):
                numerator = sum(row[name] for row in rows_by_seed)
                lo, hi = wilson_95(numerator, SEEDS)
                metrics[name] = {
                    "seeds": numerator,
                    "of": SEEDS,
                    "wilson_95": [round(lo, 6), round(hi, 6)],
                }
            summaries[condition] = {
                "total_attempts": sum(x["attempts"] for x in rows_by_seed),
                "reached_pool_supply_actions": sum(x["pool_actions_reached"] for x in rows_by_seed),
                "metrics": metrics,
            }
        n_disable_reached = sum(
            1 in grouped[seed]["disable-p1"]["checkpoints"] for seed in grouped
        )
        n_other_affected = sum(
            1 in grouped[seed]["disable-p1"]["checkpoints"]
            and process_non_target_signature(grouped[seed]["disable-p1"]["final"])
            != process_non_target_signature(grouped[seed]["initial-pool"]["final"])
            for seed in grouped
        )
        return {
            "study": "CL-1 Study 04",
            "status": "open external compute availability and generic activity",
            "source_revision": next(iter(source_revisions)),
            "independent_seed_worlds": SEEDS,
            "matched_runs": len(rows),
            "all_hashes_replay_and_provenance_verified": True,
            "conditions": summaries,
            "disable_p1_reached_seeds": n_disable_reached,
            "other_process_state_sensitivity_seeds": n_other_affected,
            "organizational_closure": "NOT ESTABLISHED",
            "self_maintenance": "NOT ESTABLISHED",
            "life_claim": "NONE",
            "caveat": "Any added compute supply is explicitly external. Long activity is not self-maintenance.",
        }
    finally:
        conn.close()


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("sqlite", type=Path)
    parser.add_argument("--json", type=Path, required=True)
    args = parser.parse_args()
    report = audit(args.sqlite)
    args.json.write_text(json.dumps(report, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
    print(json.dumps({
        "study": report["study"],
        "seeds": report["independent_seed_worlds"],
        "runs": report["matched_runs"],
        "closed_beyond_initial": report["conditions"]["closed"]["metrics"]["beyond_initial_budget"]["seeds"],
        "initial_pool_beyond_initial": report["conditions"]["initial-pool"]["metrics"]["beyond_initial_budget"]["seeds"],
        "replenished_late_256": report["conditions"]["replenished"]["metrics"]["late_256"]["seeds"],
    }))


if __name__ == "__main__":
    main()
