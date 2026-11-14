#!/usr/bin/env python3
"""Study45: transparently POST-HOC source-pinned executable graph edge closure.

384 original worlds reused; no new independent ecology or life claim.
"""
from __future__ import annotations

import argparse
import json
import sqlite3
from collections import Counter, defaultdict
from pathlib import Path

import analyze
import analyze_study41
import analyze_study42
import analyze_study44

SOURCES = (
    ("41", 3_100_000, "d52840573e4ee0ebee889ab008d964f5d1272e62",
     analyze_study41.audit),
    ("42", 3_200_000, "e2b74edb2f3b39f895f4d69410dde15431bc6309",
     analyze_study42.audit),
    ("44", 3_300_000, "1f83c2225f4f7ac681b957256b596b0683659520",
     analyze_study44.audit),
)
ARMS = ("ambient-general", "ambient-only", "closed-general",
        "zero-credit-ambient-general")


def op_type(op) -> str:
    return "Halt" if op == "Halt" else next(iter(op))


def invalid_edges(code: list) -> list[dict]:
    """Exactly the VM's absolute program edges, no fabricated relocation."""
    issues = []
    count = len(code)
    for index, op in enumerate(code):
        if op == "Halt":
            continue
        if not isinstance(op, dict) or len(op) != 1:
            raise ValueError("invalid child CIL code node")
        name, fields = next(iter(op.items()))
        targets = (
            ("nonzero", "zero") if name == "Branch" else ("next",)
        )
        for field in targets:
            if field not in fields:
                raise ValueError("unsupported native edge field")
            target = fields[field]
            if not isinstance(target, int) or not 0 <= target < count:
                issues.append({
                    "pc": index, "op": name, "edge": field, "target": target,
                    "code_node_count": count,
                })
    return issues


def event_name(kind) -> str:
    return next(iter(kind)) if isinstance(kind, dict) else kind


def audit_one(path: Path, study: str, start: int, revision: str, auditor) -> dict:
    base = auditor(path, revision)
    if (base["independent_worlds"] != 128 or base["matched_runs"] != 512
            or base["hashed_epoch_snapshots"] != 12288):
        raise ValueError("source-pinned original native plasticity audit failed")
    db = sqlite3.connect(f"{path.resolve().as_uri()}?mode=ro", uri=True)
    try:
        counters = Counter()
        witnesses = []
        per_seed = defaultdict(dict)
        hashes = 0
        for (run_id, label, initial_raw, final_raw, src) in db.execute(
            "SELECT id,condition,initial_json,final_json,source_revision "
            "FROM epoch_runs ORDER BY id"
        ):
            if src != revision:
                raise ValueError("foreign frozen source revision")
            prefix, arm = label.split("/", 1)
            seed = int(prefix.removeprefix("seed="))
            if not start <= seed < start + 128 or arm not in ARMS or arm in per_seed[seed]:
                raise ValueError("seed or treatment mismatch")
            initial = json.loads(initial_raw)
            final = json.loads(final_raw)
            observed_epochs = 0
            previous_events = initial["events"]
            for epoch, digest, raw in db.execute(
                "SELECT epoch,state_sha256,state_json FROM epoch_snapshots "
                "WHERE run_id=? ORDER BY epoch", (run_id,)
            ):
                if epoch != observed_epochs or analyze.digest(raw) != digest:
                    raise ValueError("wrong original epoch SHA256")
                snap = json.loads(raw)
                if snap["events"][:len(previous_events)] != previous_events:
                    raise ValueError("cumulative event history rewritten")
                previous_events = snap["events"]
                observed_epochs += 1
                hashes += 1
            if observed_epochs != 24 or previous_events != final["events"]:
                raise ValueError("incomplete source-pinned 24-epoch history")
            launches = {}
            child_stats = defaultdict(Counter)
            child_codes = {}
            for event in final["events"][len(initial["events"]):]:
                typ = event_name(event["kind"])
                actor = event["process"]
                origin = analyze.internal_process(event)
                if typ == "GraphLaunched" and origin == actor:
                    info = event["kind"]["GraphLaunched"]
                    child = info["child_process"]
                    if (child in launches or child <= 3 or child not in (
                        int(k) for k in final["processes"]
                    ) or info["parent_process"] != actor):
                        raise ValueError("broken native parent-to-child launch attribution")
                    launches[child] = {
                        "parent": actor, "tick": event["tick"],
                        "source_graph": info["source_graph"],
                    }
                    child_codes[child] = final["processes"][str(child)]["code"]["nodes"]
                if actor > 3 and origin == actor:
                    if actor not in launches or event["tick"] <= launches[actor]["tick"]:
                        raise ValueError("child work lacks earlier internally attributed launch")
                    child_stats[actor][typ] += 1
                    if typ == "Trapped":
                        error = event["kind"]["Trapped"]
                        if "invalid edge" in error:
                            child_stats[actor]["invalid_edge_trap"] += 1
                            op_pc = event["pc"]
                            if op_pc in {
                                issue["pc"] for issue in invalid_edges(child_codes[actor])
                            }:
                                child_stats[actor]["edge_trap_correlated_with_final_code"] += 1
            worlds_with_child_success = False
            any_non_GraphNew = False
            any_native_shared_write = False
            child_has_open_edges = False
            for child, source in launches.items():
                code = child_codes[child]
                if len(code) != final["processes"][str(child)]["code"]["nodes"].__len__():
                    raise ValueError("child program changed during audit")
                issues = invalid_edges(code)
                child_has_open_edges |= bool(issues)
                types = [op_type(op) for op in code]
                any_non_GraphNew |= any(t not in ("GraphNew", "Halt") for t in types)
                worlds_with_child_success |= any(
                    n > 0 for t, n in child_stats[child].items()
                    if t not in ("Trapped", "Halted", "invalid_edge_trap",
                                 "edge_trap_correlated_with_final_code")
                )
                any_native_shared_write |= child_stats[child]["CellWritten"] > 0
                counters[f"{arm}.child_program_nodes_total"] += len(code)
                counters[f"{arm}.child_all_edges_closed"] += int(not issues)
                counters[f"{arm}.child_with_open_edge"] += int(bool(issues))
                counters[f"{arm}.child_with_non_GraphNew_node"] += int(
                    any(t not in ("GraphNew", "Halt") for t in types)
                )
                for name in types:
                    counters[f"{arm}.child_code_op_{name}"] += 1
                for k, n in child_stats[child].items():
                    counters[f"{arm}.child_native_{k}"] += n
                if issues and len(witnesses) < 12:
                    witnesses.append({
                        "study": study, "seed": seed, "arm": arm,
                        "child": child, "parent": source["parent"],
                        "code": code, "invalid_edges": issues[:4],
                        "child_events": dict(child_stats[child]),
                    })
            counters[f"{arm}.worlds_with_children"] += bool(launches)
            counters[f"{arm}.worlds_with_open_edge_child"] += child_has_open_edges
            counters[f"{arm}.worlds_with_child_code_other_than_GraphNew"] += any_non_GraphNew
            counters[f"{arm}.worlds_with_native_child_success"] += worlds_with_child_success
            counters[f"{arm}.worlds_with_native_child_CellWritten"] += any_native_shared_write
            per_seed[seed][arm] = {
                "children": len(launches),
                "child_has_open_edges": child_has_open_edges,
                "child_code_other_than_GraphNew": any_non_GraphNew,
                "child_native_shared_write": any_native_shared_write,
            }
        if (len(per_seed) != 128 or any(set(arms) != set(ARMS) for arms in per_seed.values())
                or hashes != 12288):
            raise ValueError("missing original cohort, arm or epoch")
        for arm in ARMS:
            if counters[f"{arm}.worlds_with_children"] != base["secondary"].get(
                f"{arm}.child_launched", 0
            ):
                raise ValueError("source-verified child launch count mismatch")
            if counters[f"{arm}.worlds_with_native_child_success"] != base["secondary"].get(
                f"{arm}.child_success", 0
            ):
                raise ValueError("source-verified successful child count mismatch")
        return {
            "original_study": study,
            "source_revision": revision,
            "reused_worlds": 128, "new_worlds": 0,
            "matched_runs": 512, "verified_epoch_hashes": hashes,
            "topology_metrics": dict(sorted(counters.items())),
            "sample_invalid_edge_witnesses": witnesses,
            "life_claim": "NONE",
        }
    finally:
        db.close()


def main() -> None:
    p = argparse.ArgumentParser(description=__doc__)
    p.add_argument("study41", type=Path)
    p.add_argument("study42", type=Path)
    p.add_argument("study44", type=Path)
    p.add_argument("--json", type=Path, required=True)
    args = p.parse_args()
    paths = {"41": args.study41, "42": args.study42, "44": args.study44}
    cohorts = {
        study: audit_one(paths[study], study, start, revision, auditor)
        for study, start, revision, auditor in SOURCES
    }
    result = {
        "study": "CL-1 Study45 post-hoc graph-child executable edge-closure diagnostic",
        "new_independent_worlds": 0,
        "reused_original_independent_worlds": 384,
        "matched_runs": 1536,
        "verified_epoch_hashes": 36864,
        "cohorts": cohorts,
        "prior_invalid_child_Write_edge_observation_disclosed": True,
        "life_claim": "NONE",
    }
    args.json.write_text(json.dumps(result, indent=2) + "\n", encoding="utf-8")
    print(json.dumps({
        "study": result["study"],
        "new_worlds": 0,
        "study41": {k: v for k, v in cohorts["41"]["topology_metrics"].items()
                    if k.startswith("ambient-general.")},
        "study42": {k: v for k, v in cohorts["42"]["topology_metrics"].items()
                    if k.startswith("ambient-general.")},
        "study44": {k: v for k, v in cohorts["44"]["topology_metrics"].items()
                    if k.startswith("ambient-general.")},
        "life_claim": "NONE",
    }, indent=2))


if __name__ == "__main__":
    main()
