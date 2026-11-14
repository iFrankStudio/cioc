#!/usr/bin/env python3
"""Study74 post-result original-Study73 zero-credit native Draw entry gates.

0 new worlds. Complete original Study73 audit must pass before diagnostics.
"""
from __future__ import annotations

import argparse
import json
import sqlite3
from collections import Counter, defaultdict
from pathlib import Path

import analyze
import analyze_study73

REVISION = "2011f00cedc3121a155b078f9c127d1b8a8ec774"
FIRST = 4_900_000
N = 128
ARM_SET = set(analyze_study73.MODES)


def native_name(event: dict) -> str:
    kind = event["kind"]
    return next(iter(kind)) if isinstance(kind, dict) else kind


def classify_snapshot(snapshot: dict) -> str:
    """Disjoint physical states: credit viability, exact next opcode and fatal source."""
    p = snapshot["processes"]["1"]
    pc = p["pc"]
    program = p["code"]["nodes"]
    if not isinstance(pc, int) or not 0 <= pc < len(program):
        raise ValueError("source-pinned original process has invalid PC")
    opcode = program[pc]
    next_name = next(iter(opcode)) if isinstance(opcode, dict) else opcode
    credit, pool = p["credits"], snapshot["compute_pool"]
    if credit < 0 or pool < 0:
        raise ValueError("negative resource inventory")
    if p["stopped"]:
        last_native = next(
            (x for x in reversed(snapshot["events"])
             if x["process"] == 1 and analyze.internal_process(x) == 1),
            None,
        )
        if last_native is None:
            raise ValueError("stopped actor lacks a native event")
        if (native_name(last_native) == "Trapped"
                and "insufficient world compute pool"
                in str(last_native["kind"]["Trapped"])
                and next_name == "DrawCompute"):
            return "stopped_fatal_insufficient_pool_Draw"
        return "stopped_other"
    if credit > 0:
        return "runnable_positive_local_credits"
    if pool == 0:
        return "alive_zero_credits_empty_pool"
    if next_name == "DrawCompute":
        return "alive_zero_credits_positive_pool_at_Draw"
    return "alive_zero_credits_positive_pool_not_Draw"


def audit(db_path: Path) -> dict:
    # Previously independently designed, full-source, strict 49,152-hash auditor.
    original = analyze_study73.audit(db_path, REVISION)
    if (original["original_independent_seed_genomes"] != N
            or original["matched_native_runs"] != N*len(ARM_SET)
            or original["SHA256_verified_original_snapshots"] != N*len(ARM_SET)*24
            or original["registered_primary_A_native_resource_feedback"]["broad_worlds"] != 0
            or original["registered_primary_A_native_resource_feedback"]["short_worlds"] != 0):
        raise ValueError("Study73 original independent source and null not reproduced")
    db = sqlite3.connect(f"{db_path.resolve().as_uri()}?mode=ro", uri=True)
    try:
        by_arm = Counter()
        examples = []
        seeds: dict[int, set[str]] = defaultdict(set)
        checks = 0
        for run_id, label, source in db.execute(
            "SELECT id,condition,source_revision FROM epoch_runs ORDER BY id"
        ):
            seed_str, arm = label.split("/", 1)
            seed = int(seed_str.removeprefix("seed="))
            if (source != REVISION or not FIRST <= seed < FIRST + N
                    or arm not in ARM_SET or arm in seeds[seed]):
                raise ValueError("unregistered original source cohort or duplicate paired arm")
            seeds[seed].add(arm)
            statuses = {}
            reachable = False
            row_count = 0
            for epoch, digest, raw in db.execute(
                "SELECT epoch,state_sha256,state_json FROM epoch_snapshots "
                "WHERE run_id=? ORDER BY epoch", (run_id,)
            ):
                if epoch != row_count or analyze.digest(raw) != digest:
                    raise ValueError("original epoch source hash or order corrupted")
                state = json.loads(raw)
                cls = classify_snapshot(state)
                by_arm[f"{arm}.epoch_{epoch}.state.{cls}"] += 1
                if epoch in (7, 15, 23):
                    statuses[epoch] = {
                        "class": cls,
                        "pc": state["processes"]["1"]["pc"],
                        "credits": state["processes"]["1"]["credits"],
                        "pool": state["compute_pool"],
                    }
                if cls == "alive_zero_credits_positive_pool_at_Draw":
                    reachable = True
                row_count += 1
                checks += 1
            if row_count != 24 or set(statuses) != {7,15,23}:
                raise ValueError("missing W2/W3 entry or final snapshot")
            by_arm[f"{arm}.any_epoch_boundary_pool_paid_Draw_eligible"] += int(reachable)
            for epoch in (7,15,23):
                name = statuses[epoch]["class"]
                by_arm[f"{arm}.entry_epoch_{epoch}.{name}"] += 1
            if arm == "broad-ambient-intact-low" and len(examples) < 12:
                examples.append({"seed":seed,"epochs":statuses})
        if (len(seeds) != N
                or any(m != ARM_SET for m in seeds.values())
                or checks != N*len(ARM_SET)*24):
            raise ValueError("incomplete or unbalanced original source-pinned cohort")
        return {
            "study":"CL-1 Study74 retrospective source-pinned compute eligibility gates",
            "registered_after_Study73_primary_null":True,
            "new_independent_worlds":0,
            "reused_independent_original_genomes":N,
            "reused_matched_runs":N*len(ARM_SET),
            "full_SHA256_verified_original_snapshots":checks,
            "source_revision":REVISION,
            "attribution":"original native Draw and shared energy are NOT generated by host starter",
            "registered_Study73_primary":original[
                "registered_primary_A_native_resource_feedback"],
            "diagnostic_counts":dict(sorted(by_arm.items())),
            "first_frozen_seed_examples":examples,
            "life_claim":"NONE",
        }
    finally:
        db.close()


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("sqlite", type=Path)
    parser.add_argument("--json", type=Path, required=True)
    args = parser.parse_args()
    result = audit(args.sqlite)
    args.json.write_text(json.dumps(result,indent=2)+"\n",encoding="utf-8")
    print(json.dumps({
        "study":result["study"],
        "new_worlds":0,
        "snapshots":result["full_SHA256_verified_original_snapshots"],
        "W3_entry":{key:value for key,value
                    in result["diagnostic_counts"].items()
                    if ".entry_epoch_15." in key and
                    ("broad-ambient-intact-low." in key or
                     "short-ambient-intact-low." in key)},
        "ever_gateway":{key:value for key,value
                        in result["diagnostic_counts"].items()
                        if key.endswith(".any_epoch_boundary_pool_paid_Draw_eligible")},
        "life_claim":"NONE",
    },indent=2))


if __name__ == "__main__":
    main()
