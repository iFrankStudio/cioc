#!/usr/bin/env python3
"""Study51 prospective independent native graph assembly and functional-child audit.

Primary A (strict copy→edit→launch) is independently predeclared after Study50;
Primary B is the original native child Write provenance endpoint.
Neither primary is digital life.
"""
from __future__ import annotations

import argparse
import json
import sqlite3
from collections import Counter, defaultdict
from pathlib import Path

import analyze
import analyze_study51
from analyze_study02 import wilson_95
from analyze_study50 import trace_assembly

FIRST = 3_600_000
SEEDS = 256
ARMS = ("ambient-general", "ambient-only", "closed-general",
        "zero-credit-ambient-general")


def audit(path: Path, expected_revision: str) -> dict:
    original = analyze_study51.audit(path, expected_revision)
    if (original["independent_worlds"] != SEEDS or
            original["matched_runs"] != SEEDS * len(ARMS) or
            original["hashed_epoch_snapshots"] != SEEDS * len(ARMS) * 24):
        raise ValueError("original independent Study51 cohort mismatch")
    db = sqlite3.connect(f"{path.resolve().as_uri()}?mode=ro", uri=True)
    try:
        worlds = defaultdict(set)
        flags = Counter()
        evidence = []
        source_revisions = set()
        n = 0
        snapshots = 0
        for rid, condition, source, initial_sha, raw_initial, final_sha, raw_final in db.execute(
            "SELECT id,condition,source_revision,initial_sha256,initial_json,"
            "final_sha256,final_json FROM epoch_runs ORDER BY id"
        ):
            if "/" not in condition:
                raise ValueError("missing matched arm")
            stem, arm = condition.split("/", 1)
            seed = int(stem.removeprefix("seed="))
            if not FIRST <= seed < FIRST + SEEDS or arm not in ARMS:
                raise ValueError("outside preregistered independent cohort")
            if arm in worlds[seed]:
                raise ValueError("duplicate paired arm")
            worlds[seed].add(arm)
            source_revisions.add(source)
            if source != expected_revision or analyze.digest(raw_initial) != initial_sha or analyze.digest(raw_final) != final_sha:
                raise ValueError("historical source / hash mismatch")
            initial, final = json.loads(raw_initial), json.loads(raw_final)
            previous_events = initial["events"]
            epoch_count = 0
            for epoch, digest, raw in db.execute(
                "SELECT epoch,state_sha256,state_json FROM epoch_snapshots "
                "WHERE run_id=? ORDER BY epoch", (rid,)
            ):
                if epoch != epoch_count or analyze.digest(raw) != digest:
                    raise ValueError("missing/reordered/corrupted full epoch hash")
                state = json.loads(raw)
                if state["events"][:len(previous_events)] != previous_events:
                    raise ValueError("historical native event prefix modified")
                previous_events = state["events"]
                snapshots += 1
                epoch_count += 1
            if epoch_count != 24 or previous_events != final["events"]:
                raise ValueError("truncated epoch history")
            launched = trace_assembly(
                final["events"][len(initial["events"]):], final
            )
            primary_a = any(x["copy_before_edit_before_launch"] for x in launched)
            native_write = any(x["child_successful_CellWritten"] for x in launched)
            if native_write and not original["secondary"].get(f"{arm}.child_native_CellWritten", 0):
                raise ValueError("native child output missed by original independent auditor")
            for key, value in {
                "native_launch": bool(launched),
                "native_copy_prelaunch": any(x["copies_nonzero"] for x in launched),
                "native_edit_prelaunch": any(x["edits"] for x in launched),
                "native_copy_and_edit": any(x["copies_nonzero"] and x["edits"] for x in launched),
                "strict_copy_edit_launch": primary_a,
                "child_has_non_GraphNew": any(x["has_non_GraphNew_opcode"] for x in launched),
                "child_has_Write": any(x["has_Write"] for x in launched),
                "child_final_edges_closed": any(x["final_child_edges_closed"] for x in launched),
                "native_child_CellWritten": native_write,
            }.items():
                flags[f"{arm}.{key}"] += int(value)
            flags[f"{arm}.native_launch_events"] += len(launched)
            if (primary_a or native_write) and len(evidence) < 16:
                evidence.append({
                    "seed": seed, "arm": arm,
                    "witnesses": [x for x in launched
                                  if x["copy_before_edit_before_launch"] or x["child_successful_CellWritten"]][:3],
                })
            n += 1
        if (n != 1024 or snapshots != 24576 or len(worlds) != SEEDS
                or any(a != set(ARMS) for a in worlds.values())
                or source_revisions != {expected_revision}):
            raise ValueError("incomplete source-pinned paired native cohort")
        if (flags["ambient-general.native_launch"] !=
                original["secondary"].get("ambient-general.child_launched", 0)):
            raise ValueError("native child-launch outcomes differ between independent audits")
        if (flags["ambient-general.native_child_CellWritten"] !=
                original["primary_native_child_shared_cell_write"]["worlds"]):
            raise ValueError("functional-child primary contradicts original auditor")
        a = flags["ambient-general.strict_copy_edit_launch"]
        b = flags["ambient-general.native_child_CellWritten"]
        return {
            "study": "CL-1 Study51 independent prospective assembly-path replication",
            "source_revision": expected_revision,
            "new_independent_worlds": SEEDS,
            "matched_runs": n,
            "SHA256_verified_original_epochs": snapshots,
            "primary_A_native_copy_edit_launch": {
                "worlds": a, "denominator": SEEDS,
                "wilson_95": [round(v, 6) for v in wilson_95(a, SEEDS)],
                "prospective_mechanistic_replication": a > 0,
            },
            "primary_B_functional_child_CellWritten": {
                "worlds": b, "denominator": SEEDS,
                "wilson_95": [round(v, 6) for v in wilson_95(b, SEEDS)],
            },
            "frozen_secondary_by_arm": dict(sorted(flags.items())),
            "first_true_native_assembly_witnesses": evidence,
            "endogenous_repair": "NOT DEMONSTRATED",
            "life_claim": "NONE",
        }
    finally:
        db.close()


def main() -> None:
    p = argparse.ArgumentParser(description=__doc__)
    p.add_argument("sqlite", type=Path)
    p.add_argument("--expected-revision", required=True)
    p.add_argument("--json", type=Path, required=True)
    args = p.parse_args()
    result = audit(args.sqlite, args.expected_revision)
    args.json.write_text(json.dumps(result, indent=2) + "\n", encoding="utf-8")
    print(json.dumps({
        "study": result["study"],
        "original_worlds": result["new_independent_worlds"],
        "primary_A": result["primary_A_native_copy_edit_launch"],
        "primary_B": result["primary_B_functional_child_CellWritten"],
        "life_claim": result["life_claim"],
    }, indent=2))


if __name__ == "__main__":
    main()
