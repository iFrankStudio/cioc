#!/usr/bin/env python3
"""Study52 post-result native parent-source opcode ancestry of graph children.

No new independent worlds; original frozen native SQLite is immutable.
Initial-source opcode is not claimed to equal runtime opcode after native mutations.
"""
from __future__ import annotations

import argparse
import json
import sqlite3
from collections import Counter, defaultdict
from pathlib import Path

import analyze
import analyze_study49
import analyze_study51
from analyze_study50 import trace_assembly

COHORTS = (
    ("49", 3_500_000, "7df61b22b5bcdf7dd345cc43b28d8f344ad5891c",
     analyze_study49.audit, 1),
    ("51", 3_600_000, "ee55ade592a9152fb1fa8e9ad6edda49ea3090ba",
     analyze_study51.audit, 3),
)
ARMS = ("ambient-general", "ambient-only", "closed-general",
        "zero-credit-ambient-general")
MUTATORS = ("PatchNext", "PatchConst", "CloneNode")


def op_name(node) -> str:
    if node == "Halt":
        return "Halt"
    if not isinstance(node, dict) or len(node) != 1:
        raise ValueError("unrecognized frozen source CIL opcode")
    return next(iter(node))


def runtime_source_uncertainty(initial: dict, snapshots: list[dict],
                               tick: int, actor: int) -> bool:
    """Fail conservatively when native parent mutation may affect a copied node."""
    frozen = initial["processes"][str(actor)]["code"]["nodes"]
    prev = initial
    for snapshot in snapshots:
        if snapshot["events"][-1]["tick"] >= tick:
            for e in snapshot["events"][len(prev["events"]):]:
                if e["tick"] >= tick:
                    break
                if (analyze.internal_process(e) == actor
                        and e["pc"] < len(frozen)
                        and op_name(frozen[e["pc"]]) in MUTATORS
                        and e["kind"] == "Executed"):
                    return True
            return prev["processes"][str(actor)]["code"]["nodes"] != frozen
        if snapshot["processes"][str(actor)]["code"]["nodes"] != frozen:
            return True
        prev = snapshot
    raise ValueError("source copy event outside persisted epoch history")


def classify_strict_launch(path_info: dict, initial: dict,
                           snapshots: list[dict]) -> dict:
    """Actor/source-index ancestry only; no assumption of heritable function."""
    parent = path_info["parent"]
    frozen = initial["processes"][str(parent)]["code"]["nodes"]
    edits = path_info["edits"]
    eligible_copies = [
        copy for copy in path_info["copies_nonzero"]
        if any(copy["tick"] < edit["tick"] < path_info["tick"]
               for edit in edits)
    ]
    if not eligible_copies or not path_info["copy_before_edit_before_launch"]:
        raise ValueError("strict native assembly path was not actually ordered")
    source = []
    for copy in eligible_copies:
        src = copy["source_node"]
        if not isinstance(src, int) or not 0 <= src < len(frozen):
            raise ValueError("copied parent source opcode outside frozen program")
        name = op_name(frozen[src])
        source.append({
            "copied_at_tick": copy["tick"],
            "frozen_parent_source_index": src,
            "frozen_parent_source_opcode": name,
            "new_graph_node_index": copy["new_node"],
            "runtime_source_opcode": (
                "NOT EVALUABLE: possible native executable mutation"
                if runtime_source_uncertainty(initial, snapshots, copy["tick"], parent)
                else name
            ),
        })
    return {
        "parent": parent,
        "source_graph": path_info["source_graph"],
        "child": path_info["child"],
        "launch_tick": path_info["tick"],
        "ordered_copies": source,
        "edge_edits": edits,
        "child_has_Write": path_info["has_Write"],
        "child_non_GraphNew": path_info["has_non_GraphNew_opcode"],
        "child_final_edges_closed": path_info["final_child_edges_closed"],
        "native_child_CellWritten": path_info["child_successful_CellWritten"],
        "native_child_invalid_edge_trap": path_info["child_invalid_edge_trap"],
    }


def audit_one(path: Path, study: str, first: int, revision: str,
              original_auditor, known_strict_count: int) -> dict:
    original = original_auditor(path, revision)
    if (original["independent_worlds"] != 256 or
            original["matched_runs"] != 1024 or
            original["hashed_epoch_snapshots"] != 24576):
        raise ValueError("source-pinned original Study49/51 auditor failed")
    db = sqlite3.connect(f"{path.resolve().as_uri()}?mode=ro", uri=True)
    try:
        seen = defaultdict(set)
        counts = Counter()
        witnesses = []
        verified_epochs = 0
        for rid, condition, source, initial_sha, raw_initial, final_sha, raw_final in db.execute(
            "SELECT id,condition,source_revision,initial_sha256,initial_json,"
            "final_sha256,final_json FROM epoch_runs ORDER BY id"
        ):
            label, arm = condition.split("/", 1)
            seed = int(label.removeprefix("seed="))
            if (source != revision or not first <= seed < first + 256
                    or arm not in ARMS or arm in seen[seed]
                    or analyze.digest(raw_initial) != initial_sha
                    or analyze.digest(raw_final) != final_sha):
                raise ValueError("incorrect historical cohort or mutated SQLite bytes")
            seen[seed].add(arm)
            initial, final = json.loads(raw_initial), json.loads(raw_final)
            prior = initial["events"]
            snapshots = []
            for epoch, sha, raw in db.execute(
                "SELECT epoch,state_sha256,state_json FROM epoch_snapshots "
                "WHERE run_id=? ORDER BY epoch", (rid,)
            ):
                if epoch != len(snapshots) or analyze.digest(raw) != sha:
                    raise ValueError("epoch missing, reordered, or hash changed")
                state = json.loads(raw)
                if state["events"][:len(prior)] != prior:
                    raise ValueError("source history was rewritten")
                snapshots.append(state)
                prior = state["events"]
                verified_epochs += 1
            if len(snapshots) != 24 or prior != final["events"]:
                raise ValueError("incomplete native original event history")
            launches = trace_assembly(
                final["events"][len(initial["events"]):], final
            )
            strict = [
                classify_strict_launch(launch, initial, snapshots)
                for launch in launches if launch["copy_before_edit_before_launch"]
            ]
            counts[f"{arm}.worlds_any_native_launch"] += int(bool(launches))
            counts[f"{arm}.worlds_strict_native_assembly"] += int(bool(strict))
            counts[f"{arm}.strict_native_assembly_events"] += len(strict)
            counts[f"{arm}.worlds_strict_with_Write_source_ancestry"] += int(any(
                copy["frozen_parent_source_opcode"] == "Write"
                for item in strict for copy in item["ordered_copies"]
            ))
            counts[f"{arm}.worlds_strict_with_mutation_uncertainty"] += int(any(
                copy["runtime_source_opcode"].startswith("NOT EVALUABLE")
                for item in strict for copy in item["ordered_copies"]
            ))
            counts[f"{arm}.worlds_strict_child_final_edge_closure"] += int(any(
                item["child_final_edges_closed"] for item in strict
            ))
            counts[f"{arm}.worlds_strict_child_has_Write"] += int(any(
                item["child_has_Write"] for item in strict
            ))
            counts[f"{arm}.worlds_strict_child_functional_write"] += int(any(
                item["native_child_CellWritten"] for item in strict
            ))
            for item in strict:
                for copy in item["ordered_copies"]:
                    counts[f"{arm}.initial_source_op.{copy['frozen_parent_source_opcode']}"] += 1
                if len(witnesses) < 24:
                    witnesses.append({"seed": seed, "arm": arm, **item})
        if (len(seen) != 256 or any(a != set(ARMS) for a in seen.values())
                or verified_epochs != 24576):
            raise ValueError("incomplete four-arm historical native cohort")
        if counts["ambient-general.worlds_strict_native_assembly"] != known_strict_count:
            raise ValueError("Study50/51 frozen strict copy-edit-launch count disagrees")
        if counts["ambient-general.worlds_strict_child_functional_write"] != 0:
            raise ValueError("previously confirmed functional-child null changed")
        return {
            "source_study": study,
            "source_revision": revision,
            "retrospective_new_worlds": 0,
            "original_independent_worlds_reused": 256,
            "matched_runs": 1024,
            "SHA256_verified_epoch_snapshots": verified_epochs,
            "source_opcode_and_path_counts": dict(sorted(counts.items())),
            "strict_assembly_source_ancestry_witnesses": witnesses,
            "life_claim": "NONE",
        }
    finally:
        db.close()


def main() -> None:
    p = argparse.ArgumentParser(description=__doc__)
    p.add_argument("study49", type=Path)
    p.add_argument("study51", type=Path)
    p.add_argument("--json", type=Path, required=True)
    args = p.parse_args()
    inputs = {"49": args.study49, "51": args.study51}
    cohorts = {
        study: audit_one(inputs[study], study, first, rev, original, known)
        for study, first, rev, original, known in COHORTS
    }
    result = {
        "study": "CL-1 Study52 historical native child opcode ancestry",
        "new_independent_worlds": 0,
        "original_worlds_reused": 512,
        "matched_runs": 2048,
        "original_SHA256_epochs": 49152,
        "cohorts": cohorts,
        "life_claim": "NONE",
    }
    args.json.write_text(json.dumps(result, indent=2) + "\n", encoding="utf-8")
    print(json.dumps({
        "study": result["study"],
        "new_worlds": 0,
        "study49": {
            k:v for k,v in cohorts["49"]["source_opcode_and_path_counts"].items()
            if k.startswith("ambient-general.")
        },
        "study51": {
            k:v for k,v in cohorts["51"]["source_opcode_and_path_counts"].items()
            if k.startswith("ambient-general.")
        },
        "life_claim": "NONE",
    }, indent=2))


if __name__ == "__main__":
    main()
