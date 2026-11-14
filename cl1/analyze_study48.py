#!/usr/bin/env python3
"""Study48 retrospective native graph copy/edit/launch chronology; ZERO new worlds.

All source data were prospectively generated earlier. This audit is post-result.
"""
from __future__ import annotations

import argparse
import json
import sqlite3
from collections import Counter, defaultdict
from pathlib import Path

import analyze
import analyze_study44
import analyze_study46

COHORTS = (
    ("44", 3_300_000, "1f83c2225f4f7ac681b957256b596b0683659520",
     analyze_study44.audit),
    ("46", 3_400_000, "6bed2ef33122584570502bbd644592248bf83e27",
     analyze_study46.audit),
)
ARMS = ("ambient-general", "ambient-only", "closed-general",
        "zero-credit-ambient-general")


def event_type(event: dict) -> str:
    k = event["kind"]
    return next(iter(k)) if isinstance(k, dict) else k


def native(event: dict) -> bool:
    return analyze.internal_process(event) == event["process"]


def edge_issues(program: list) -> list:
    issues = []
    for pc, op in enumerate(program):
        if op == "Halt":
            continue
        if not isinstance(op, dict) or len(op) != 1:
            raise ValueError("unexpected child CIL opcode")
        name, fields = next(iter(op.items()))
        for edge in (("nonzero", "zero") if name == "Branch" else ("next",)):
            if edge not in fields:
                raise ValueError("missing executable edge")
            if fields[edge] not in range(len(program)):
                issues.append((pc, edge, fields[edge]))
    return issues


def trace_assembly(events: list[dict], final: dict) -> list[dict]:
    """Treat only successful actor-attributed source graph operations as native."""
    copies = defaultdict(list)
    edits = defaultdict(list)
    launches = []
    children = set()
    for event in events:
        kind = event_type(event)
        if kind not in ("GraphNodeCopied", "GraphNodeEdited", "GraphLaunched"):
            continue
        if not native(event):
            raise ValueError("fake native graph operation")
        details = event["kind"][kind]
        actor, tick = event["process"], event["tick"]
        if kind == "GraphNodeCopied":
            if details["owner"] != actor or details["source_process"] != actor:
                raise ValueError("copy event has mismatched source actor ownership")
            copies[(actor, details["graph"])].append({
                "tick": tick, "source_node": details["source_node"],
                "new_node": details["new_node"],
            })
        elif kind == "GraphNodeEdited":
            if details["owner"] != actor:
                raise ValueError("edit event has mismatched owner")
            edits[(actor, details["graph"])].append({
                "tick": tick, "target_node": details["node"],
                "edit": details["edit"],
            })
        else:
            if details["parent_process"] != actor:
                raise ValueError("GraphLaunched parent attribution mismatch")
            child = details["child_process"]
            if child <= 3 or child in children or str(child) not in final["processes"]:
                raise ValueError("broken child identity lineage")
            children.add(child)
            graph = details["source_graph"]
            copied = [x for x in copies[(actor, graph)]
                      if x["tick"] < tick and x["source_node"] != 0]
            edited = [x for x in edits[(actor, graph)] if x["tick"] < tick]
            ordered = any(c["tick"] < e["tick"]
                          for c in copied for e in edited)
            code = final["processes"][str(child)]["code"]["nodes"]
            if not code:
                raise ValueError("launched child executable empty")
            opcodes = ["Halt" if n == "Halt" else next(iter(n)) for n in code]
            child_events = [
                e for e in events
                if e["process"] == child and native(e) and e["tick"] > tick
            ]
            launches.append({
                "parent": actor, "child": child, "source_graph": graph,
                "tick": tick, "copies_nonzero": copied, "edits": edited,
                "copy_before_edit_before_launch": ordered,
                "has_non_GraphNew_opcode": any(
                    op not in ("GraphNew", "Halt") for op in opcodes
                ),
                "has_Write": "Write" in opcodes,
                "final_child_edges_closed": not edge_issues(code),
                "child_successful_CellWritten": any(
                    event_type(e) == "CellWritten" for e in child_events
                ),
                "child_invalid_edge_trap": any(
                    event_type(e) == "Trapped"
                    and "invalid edge" in e["kind"]["Trapped"]
                    for e in child_events
                ),
            })
    return launches


def audit_one(path: Path, study: str, first: int, revision: str, original_auditor) -> dict:
    original = original_auditor(path, revision)
    if (original["independent_worlds"] != 128
            or original["matched_runs"] != 512
            or original["hashed_epoch_snapshots"] != 12288):
        raise ValueError("prior source-pinned independent auditor mismatch")
    db = sqlite3.connect(f"{path.resolve().as_uri()}?mode=ro", uri=True)
    try:
        counts = Counter()
        witnesses = []
        worlds = defaultdict(set)
        verified_hashes = 0
        total_runs = 0
        for rid, condition, raw_initial, raw_final, source in db.execute(
            "SELECT id,condition,initial_json,final_json,source_revision "
            "FROM epoch_runs ORDER BY id"
        ):
            if source != revision or "/" not in condition:
                raise ValueError("wrong historical source revision")
            label, arm = condition.split("/", 1)
            seed = int(label.removeprefix("seed="))
            if not first <= seed < first + 128 or arm not in ARMS or arm in worlds[seed]:
                raise ValueError("wrong seed or duplicate matched condition")
            worlds[seed].add(arm)
            initial, final = json.loads(raw_initial), json.loads(raw_final)
            prev = initial["events"]
            n = 0
            for epoch, sha, raw in db.execute(
                "SELECT epoch,state_sha256,state_json FROM epoch_snapshots "
                "WHERE run_id=? ORDER BY epoch", (rid,)
            ):
                if epoch != n or analyze.digest(raw) != sha:
                    raise ValueError("SHA256 missing or corrupted")
                state = json.loads(raw)
                if state["events"][:len(prev)] != prev:
                    raise ValueError("native event prefix overwritten")
                prev = state["events"]
                n += 1
                verified_hashes += 1
            if n != 24 or prev != final["events"]:
                raise ValueError("historical epoch record incomplete")
            events = final["events"][len(initial["events"]):]
            launches = trace_assembly(events, final)
            total_runs += 1
            flags = {
                "launched": bool(launches),
                "copied_nonzero_before_launch": any(
                    x["copies_nonzero"] for x in launches
                ),
                "edited_same_graph_before_launch": any(x["edits"] for x in launches),
                "copy_and_edit": any(
                    x["copies_nonzero"] and x["edits"] for x in launches
                ),
                "copy_then_edit_then_launch": any(
                    x["copy_before_edit_before_launch"] for x in launches
                ),
                "child_non_GraphNew": any(x["has_non_GraphNew_opcode"] for x in launches),
                "child_Write": any(x["has_Write"] for x in launches),
                "child_edges_closed": any(x["final_child_edges_closed"] for x in launches),
                "child_CellWritten": any(x["child_successful_CellWritten"] for x in launches),
            }
            for name, v in flags.items():
                counts[f"{arm}.worlds_{name}"] += int(v)
            counts[f"{arm}.native_launches"] += len(launches)
            for info in launches:
                if info["copies_nonzero"] or info["edits"]:
                    if len(witnesses) < 20:
                        witnesses.append({"seed": seed, "arm": arm, **info})
        if (total_runs != 512 or verified_hashes != 12288 or len(worlds) != 128
                or any(arms != set(ARMS) for arms in worlds.values())):
            raise ValueError("original four-arm source population incomplete")
        if (counts["ambient-general.worlds_launched"]
                != original["secondary"].get("ambient-general.child_launched", 0)):
            raise ValueError("native launch prevalence differs from original audited result")
        if counts["ambient-general.worlds_child_CellWritten"] != 0:
            raise ValueError("historical preregistered child write null altered")
        return {
            "original_study": study,
            "source_revision": revision,
            "new_independent_worlds": 0,
            "original_worlds_reused": 128,
            "matched_runs": total_runs,
            "verified_original_sha256_epochs": verified_hashes,
            "stage_counts_by_arm": dict(sorted(counts.items())),
            "first_graph_assembly_witnesses": witnesses,
            "life_claim": "NONE",
        }
    finally:
        db.close()


def main() -> None:
    p = argparse.ArgumentParser(description=__doc__)
    p.add_argument("study44", type=Path)
    p.add_argument("study46", type=Path)
    p.add_argument("--json", type=Path, required=True)
    args = p.parse_args()
    paths = {"44": args.study44, "46": args.study46}
    cohorts = {
        study: audit_one(paths[study], study, first, revision, auditor)
        for study, first, revision, auditor in COHORTS
    }
    report = {
        "study": "CL-1 Study48 retrospective native graph assembly sequence",
        "new_independent_worlds": 0,
        "original_worlds_reused": 256,
        "matched_runs": 1024,
        "original_epoch_sha256_verified": 24576,
        "prior_Study44_and_Study46_null_known_before_registration": True,
        "cohorts": cohorts,
        "life_claim": "NONE",
    }
    args.json.write_text(json.dumps(report, indent=2) + "\n", encoding="utf-8")
    print(json.dumps({
        "study": report["study"],
        "new_worlds": 0,
        "study44": cohorts["44"]["stage_counts_by_arm"],
        "study46": cohorts["46"]["stage_counts_by_arm"],
        "life_claim": "NONE",
    }, indent=2))


if __name__ == "__main__":
    main()
