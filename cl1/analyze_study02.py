#!/usr/bin/env python3
"""Study-02 read-only analysis of preregistered reciprocal process-state sensitivity."""
from __future__ import annotations

import argparse
import json
import math
import sqlite3
from collections import Counter
from pathlib import Path

import analyze

FIRST = 720000
SEEDS = 128
LABELS = ("control", "disable-p1", "disable-p2", "disable-p3", "sham-disable", "zero-credit")
HORIZON = 96


def wilson_95(successes: int, trials: int) -> tuple[float, float]:
    """Two-sided 95% Wilson score interval, n = independent initial worlds."""
    if not 0 <= successes <= trials or trials < 1:
        raise ValueError("invalid binomial count")
    z = 1.959963984540054
    p = successes / trials
    denominator = 1 + z * z / trials
    center = (p + z * z / (2 * trials)) / denominator
    half = (
        z
        * math.sqrt((p * (1 - p) + z * z / (4 * trials)) / trials)
        / denominator
    )
    return max(0.0, center - half), min(1.0, center + half)


def semantic_world(world: dict) -> dict:
    """Exclude external provenance/ticks; retain the entire simulated state."""
    return {
        "processes": world["processes"],
        "graph_objects": world["graph_objects"],
        "cells": world["cells"],
        "compute_pool": world["compute_pool"],
    }


def reciprocal_pairs(control: dict, treatments: dict[int, dict], exposed: set[int]) -> dict:
    """Return directional and mutual relations without asserting organizational closure."""
    directional = {
        (i, j): (i in exposed and control["processes"][str(j)]
                 != treatments[i]["processes"][str(j)])
        for i in (1, 2, 3)
        for j in (1, 2, 3)
        if i != j
    }
    mutual = [f"{i}<->{j}" for i, j in ((1, 2), (1, 3), (2, 3))
              if directional[(i, j)] and directional[(j, i)]]
    return {"directional": directional, "reciprocal": mutual}


def analyze_db(path: Path) -> dict:
    conn = sqlite3.connect(f"{path.resolve().as_uri()}?mode=ro", uri=True)
    try:
        records = conn.execute(
            "SELECT id,source_revision,condition,initial_sha256,initial_json,"
            "actions_json,attempt_limit,attempts,final_sha256,final_json "
            "FROM scheduled_runs ORDER BY id"
        ).fetchall()
        if len(records) != SEEDS * len(LABELS):
            raise ValueError(f"expected 768 condition records, found {len(records)}")
        checkpoints = Counter(
            row[0]
            for row in conn.execute(
                "SELECT run_id,ordinal,after_attempts,state_sha256,state_json "
                "FROM scheduled_checkpoints"
            )
        )
        for run_id, ordinal, at, sha, state in conn.execute(
            "SELECT run_id,ordinal,after_attempts,state_sha256,state_json "
            "FROM scheduled_checkpoints"
        ):
            if analyze.digest(state) != sha or ordinal < 0 or not 0 <= at <= HORIZON:
                raise ValueError(f"corrupt scheduled checkpoint {run_id}")
        revision_set = set()
        matched_hashes = {}
        by_seed = {}
        seen = set()
        for (rid, revision, label, initial_hash, initial_json, actions_json, limit,
             attempts, final_hash, final_json) in records:
            if not label.startswith("seed=") or "/" not in label:
                raise ValueError(f"invalid label: {label}")
            prefix, condition = label.split("/", 1)
            seed = int(prefix.removeprefix("seed="))
            if (
                not prefix.startswith("seed=") or seed not in range(FIRST, FIRST + SEEDS)
                or condition not in LABELS or (seed, condition) in seen
            ):
                raise ValueError(f"unexpected or repeated run: {label}")
            seen.add((seed, condition))
            revision_set.add(revision)
            if (
                not revision or analyze.digest(initial_json) != initial_hash
                or analyze.digest(final_json) != final_hash
                or limit != HORIZON or not 0 <= attempts <= HORIZON
            ):
                raise ValueError(f"integrity check failed: {label}")
            if seed in matched_hashes and matched_hashes[seed] != initial_hash:
                raise ValueError(f"nonidentical matched baselines: seed={seed}")
            matched_hashes[seed] = initial_hash
            initial, final = json.loads(initial_json), json.loads(final_json)
            if (
                not initial["strict_accounting"] or initial["total_storage_limit"] != 160
                or analyze.occupied_units(initial) != 51
                or analyze.occupied_units(final) > 160
            ):
                raise ValueError(f"invalid resource boundary: {label}")
            actions = json.loads(actions_json)
            expected = 0 if condition == "control" else 3 if condition == "zero-credit" else 1
            if len(actions) != expected or checkpoints[rid] > expected:
                raise ValueError(f"condition intervention count invalid: {label}")
            if condition == "zero-credit":
                if attempts != 0 or any(
                    analyze.internal_process(e) is not None for e in final["events"]
                ):
                    raise ValueError(f"zero-compute condition executed: {label}")
            elif condition in {"disable-p1", "disable-p2", "disable-p3", "sham-disable"}:
                if actions[0]["after_attempts"] != 12:
                    raise ValueError(f"intervention at unexpected offset: {label}")
            if condition == "control" and actions:
                raise ValueError(f"treated control: {label}")
            by_seed.setdefault(seed, {})[condition] = {
                "world": final,
                "attempts": attempts,
                "reached": checkpoints[rid] == expected,
                "initial_hash": initial_hash,
            }
        if len(revision_set) != 1 or len(by_seed) != SEEDS:
            raise ValueError("missing seeds or multiple source revisions")
        if any(set(x) != set(LABELS) for x in by_seed.values()):
            raise ValueError("missing condition in a matched unit")

        directional_counts = Counter()
        exposed_counts = Counter()
        mutual_pair_counts = Counter()
        seeds_with_any_mutual = 0
        control_attempts = []
        all_sham_matches = True
        for seed in sorted(by_seed):
            cases = by_seed[seed]
            control = cases["control"]["world"]
            sham = cases["sham-disable"]["world"]
            if semantic_world(control) != semantic_world(sham):
                all_sham_matches = False
            exposed = {
                i for i in (1, 2, 3) if cases[f"disable-p{i}"]["reached"]
            }
            treatments = {
                i: cases[f"disable-p{i}"]["world"] for i in (1, 2, 3)
            }
            pairs = reciprocal_pairs(control, treatments, exposed)
            for i in (1, 2, 3):
                for j in (1, 2, 3):
                    if i != j:
                        key = f"{i}->{j}"
                        exposed_counts[key] += i in exposed
                        directional_counts[key] += pairs["directional"][(i, j)]
            for pair in pairs["reciprocal"]:
                mutual_pair_counts[pair] += 1
            seeds_with_any_mutual += bool(pairs["reciprocal"])
            control_attempts.append(cases["control"]["attempts"])
        if not all_sham_matches:
            raise ValueError("sham intervention changed simulated world state")
        lo, hi = wilson_95(seeds_with_any_mutual, SEEDS)
        return {
            "study": "CL-1 Study 02",
            "status": "reciprocal state-sensitivity study, not self-maintenance evidence",
            "source_revision": next(iter(revision_set)),
            "independent_seed_worlds": SEEDS,
            "matched_runs": len(records),
            "all_final_and_checkpoint_hashes_valid": True,
            "all_sham_controls_state_equivalent": True,
            "exact_replay": "verified for all 768 runs by Rust experiment runner",
            "control_attempts": {
                "sum": sum(control_attempts),
                "min": min(control_attempts),
                "max": max(control_attempts),
            },
            "directional_sensitivity_counts": dict(sorted(directional_counts.items())),
            "exposed_directional_count": dict(sorted(exposed_counts.items())),
            "reciprocal_pair_counts": {
                k: mutual_pair_counts[k] for k in ("1<->2", "1<->3", "2<->3")
            },
            "seeds_with_any_reciprocal_sensitivity": seeds_with_any_mutual,
            "reciprocal_seeds_wilson_95": [round(lo, 6), round(hi, 6)],
            "organizational_closure": "NOT EVALUATED",
            "autonomous_maintenance": "NOT EVALUATED",
            "life_claim": "NONE",
            "caveat": "Reciprocal state sensitivity is not necessarily maintenance or closure; the program population is externally seeded.",
        }
    finally:
        conn.close()


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("sqlite", type=Path)
    parser.add_argument("--json", type=Path, required=True)
    args = parser.parse_args()
    result = analyze_db(args.sqlite)
    args.json.write_text(json.dumps(result, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
    print(json.dumps({
        "study": result["study"],
        "seeds": result["independent_seed_worlds"],
        "runs": result["matched_runs"],
        "reciprocal_seeds": result["seeds_with_any_reciprocal_sensitivity"],
        "wilson_95": result["reciprocal_seeds_wilson_95"],
    }))


if __name__ == "__main__":
    main()
