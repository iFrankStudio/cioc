#!/usr/bin/env python3
"""Study-05 independent epoch audit; external pulses are not endogenous activity."""
from __future__ import annotations

import argparse
import json
import sqlite3
from collections import Counter
from pathlib import Path

import analyze
from analyze_study02 import wilson_95

SEED_FIRST = 1_050_000
SEED_COUNT = 64
EPOCHS = 24
QUOTA = 8
CONDITIONS = ("closed", "initial-pool", "pulse-pool", "zero-credit-pulse")


def expected_actions(condition: str) -> list[int]:
    return {
        "closed": [],
        "initial-pool": [0],
        "pulse-pool": [0, 4, 8, 12, 16, 20],
        "zero-credit-pulse": [0, 0, 0, 0, 4, 8, 12, 16, 20],
    }[condition]


def supply_events(world: dict) -> int:
    n = 0
    for event in world["events"]:
        kind = event["kind"]
        if isinstance(kind, dict) and "Intervened" in kind and "SetComputePool" in kind["Intervened"]:
            if event["origin"] != "External":
                raise ValueError("externally supplied compute misattributed")
            n += 1
    return n


def semantic_state(world: dict) -> dict:
    """Exclude directly injected pool balance and provenance/event timeline."""
    return {
        "processes": world["processes"],
        "graph_objects": world["graph_objects"],
        "cells": world["cells"],
    }


def audit(path: Path) -> dict:
    connection = sqlite3.connect(f"{path.resolve().as_uri()}?mode=ro", uri=True)
    try:
        rows = connection.execute(
            "SELECT id,source_revision,condition,initial_sha256,initial_json,"
            "actions_json,epoch_count,quota,attempts_json,final_sha256,final_json "
            "FROM epoch_runs ORDER BY id"
        ).fetchall()
        if len(rows) != SEED_COUNT * len(CONDITIONS):
            raise ValueError(f"invalid cohort size: {len(rows)}")
        snapshots_by_run = {}
        for rid, epoch, sha, state in connection.execute(
            "SELECT run_id,epoch,state_sha256,state_json FROM epoch_snapshots "
            "ORDER BY run_id,epoch"
        ):
            if epoch < 0 or epoch >= EPOCHS or analyze.digest(state) != sha:
                raise ValueError(f"invalid epoch snapshot: run={rid} epoch={epoch}")
            snapshots_by_run.setdefault(rid, []).append((epoch, sha, state))
        by_seed: dict[int, dict[str, dict]] = {}
        hash_by_seed: dict[int, str] = {}
        source_revisions: set[str] = set()
        for rid, revision, label, initial_sha, initial_json, actions_json, epochs, quota, attempts_json, final_sha, final_json in rows:
            if not label.startswith("seed=") or "/" not in label:
                raise ValueError(f"unexpected record: {label}")
            prefix, condition = label.split("/", 1)
            seed = int(prefix[5:])
            if seed not in range(SEED_FIRST, SEED_FIRST + SEED_COUNT) or condition not in CONDITIONS:
                raise ValueError(f"unregistered world or condition: {label}")
            if condition in by_seed.get(seed, {}):
                raise ValueError(f"duplicate matched condition: {label}")
            if (
                not revision
                or analyze.digest(initial_json) != initial_sha
                or analyze.digest(final_json) != final_sha
                or epochs != EPOCHS or quota != QUOTA
            ):
                raise ValueError(f"invalid revision/hash/epoch schedule: {label}")
            if seed in hash_by_seed and hash_by_seed[seed] != initial_sha:
                raise ValueError(f"unmatched initial state: {label}")
            hash_by_seed[seed] = initial_sha
            source_revisions.add(revision)
            actions = json.loads(actions_json)
            if [action["epoch"] for action in actions] != expected_actions(condition):
                raise ValueError(f"incorrect or non-frozen external action schedule: {label}")
            attempts = json.loads(attempts_json)
            if (
                len(attempts) != EPOCHS
                or any(not isinstance(n, int) or not 0 <= n <= QUOTA for n in attempts)
            ):
                raise ValueError(f"invalid epoch CIL attempt counts: {label}")
            snapshots = snapshots_by_run.get(rid, [])
            if len(snapshots) != EPOCHS or any(epoch != i for i, (epoch, _, _) in enumerate(snapshots)):
                raise ValueError(f"missing or disordered epoch snapshots: {label}")
            if snapshots[-1][2] != final_json or snapshots[-1][1] != final_sha:
                raise ValueError(f"final epoch diverges from recorded final state: {label}")
            initial, final = json.loads(initial_json), json.loads(final_json)
            if (
                not initial["strict_accounting"]
                or initial["total_storage_limit"] != 160
                or analyze.occupied_units(initial) != 51
                or analyze.occupied_units(final) > 160
            ):
                raise ValueError(f"resource accounting failed: {label}")
            supply_expected = 0 if condition == "closed" else 1 if condition == "initial-pool" else 6
            if supply_events(final) != supply_expected:
                raise ValueError(f"inert epoch pulses were missed: {label}")
            if condition == "closed" and sum(attempts) > 84:
                raise ValueError(f"free energy in closed world: {label}")
            if condition == "zero-credit-pulse" and sum(attempts) != 0:
                raise ValueError(f"inert zero-credit world unexpectedly ran: {label}")
            by_seed.setdefault(seed, {})[condition] = {
                "world": final,
                "attempts": attempts,
                "snapshots": snapshots,
                "pulse_events": supply_events(final),
                "total_attempts": sum(attempts),
                "active_late": any(x > 0 for x in attempts[12:]),
                "became_active_after_zero": any(
                    attempts[index] > 0 and attempts[index - 1] == 0
                    for index in range(1, EPOCHS)
                ),
                "created_process": any(int(pid) > 3 for pid in final["processes"]),
            }
        if len(source_revisions) != 1 or len(by_seed) != SEED_COUNT or any(
            set(conditions) != set(CONDITIONS) for conditions in by_seed.values()
        ):
            raise ValueError("study incomplete or inconsistent source revision")
        result = {}
        for condition in CONDITIONS:
            worlds = [by_seed[seed][condition] for seed in sorted(by_seed)]
            report = {
                "total_attempts": sum(row["total_attempts"] for row in worlds),
                "externally_delivered_compute_pulses": sum(row["pulse_events"] for row in worlds),
            }
            for name in ("active_late", "became_active_after_zero", "created_process"):
                n = sum(row[name] for row in worlds)
                lo, hi = wilson_95(n, SEED_COUNT)
                report[name] = {
                    "seeds": n,
                    "of": SEED_COUNT,
                    "wilson_95": [round(lo, 6), round(hi, 6)],
                }
            result[condition] = report
        semantic_differences = sum(
            semantic_state(by_seed[seed]["initial-pool"]["world"])
            != semantic_state(by_seed[seed]["pulse-pool"]["world"])
            for seed in by_seed
        )
        return {
            "study": "CL-1 Study 05",
            "status": "independent external epoch experiment (not life evidence)",
            "source_revision": next(iter(source_revisions)),
            "independent_worlds": SEED_COUNT,
            "matched_runs": len(rows),
            "saved_and_sha256_verified_epoch_snapshots": sum(map(len, snapshots_by_run.values())),
            "exact_replay": "every run and each epoch state independently reconstructed by Rust laboratory",
            "conditions": result,
            "initial_pool_vs_pulse_pool_semantic_difference_seeds": semantic_differences,
            "autonomous_maintenance": "NOT ESTABLISHED",
            "organizational_closure": "NOT ESTABLISHED",
            "life_claim": "NONE",
            "caveat": "Environment epochs are researcher administered; they cannot grant process credits without CIL.",
        }
    finally:
        connection.close()


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("database", type=Path)
    parser.add_argument("--json", type=Path, required=True)
    args = parser.parse_args()
    report = audit(args.database)
    args.json.write_text(json.dumps(report, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
    print(json.dumps({
        "study": report["study"],
        "worlds": report["independent_worlds"],
        "runs": report["matched_runs"],
        "snapshots": report["saved_and_sha256_verified_epoch_snapshots"],
        "pulse_late": report["conditions"]["pulse-pool"]["active_late"]["seeds"],
        "zero_credit_attempts": report["conditions"]["zero-credit-pulse"]["total_attempts"],
    }))


if __name__ == "__main__":
    main()
