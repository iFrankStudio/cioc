#!/usr/bin/env python3
"""Study 18: conservative observer-only functional lineage witness auditor.

A witness is a time-ordered, explicitly attributed execution trace, NOT
a causal proof that the copied instruction determines the child's output.
"""
from __future__ import annotations

import argparse
import json
import sqlite3
from collections import Counter
from pathlib import Path

import analyze


def _internal(event: dict) -> int | None:
    return analyze.internal_process(event)


def audit_events(events: list[dict]) -> dict:
    """Audit the full append-only world event log, never infer omitted links."""
    cells: dict[int, tuple[str, int | None, int, int]] = {}
    graphs: dict[int, dict] = {}
    children: dict[int, dict] = {}
    last_read: dict[tuple[int, int], tuple[int, int]] = {}
    counts: Counter = Counter()
    witnesses: list[dict] = []
    rejections: list[dict] = []
    traps: list[int] = []

    def reject(index: int, kind: str, reason: str) -> None:
        counts[reason] += 1
        rejections.append({"event_id": index, "operation": kind, "reason": reason})

    for i, event in enumerate(events):
        kind = event.get("kind")
        if isinstance(kind, str):
            continue
        if not isinstance(kind, dict) or len(kind) != 1:
            raise ValueError(f"bad event kind at {i}")
        name, data = next(iter(kind.items()))
        actor = _internal(event)
        if name == "Trapped" and actor is not None:
            traps.append(i)
        if name == "Intervened" and event.get("origin") == "External":
            # External interventions do not create internal writer claims.
            import re
            match = re.fullmatch(r"SetCell \{ key: (-?\d+), value: (-?\d+) \}", str(data))
            if match:
                key, value = map(int, match.groups())
                cells[key] = ("external", None, value, i)
            elif "DropGraph" in str(data):
                match = re.search(r"graph: (\d+)", str(data))
                if match:
                    graphs.pop(int(match.group(1)), None)
            continue
        if name == "CellWritten":
            if actor is None or event.get("process") != actor:
                raise ValueError(f"internal cell write has invalid actor at {i}")
            key, value = int(data["key"]), int(data["value"])
            cells[key] = ("internal-executed", actor, value, i)
            if actor in children:
                child = children[actor]
                node = child["code"].get(event.get("pc"))
                if node is None or node["opcode_tag"] != 4:
                    reject(i, name, "child_write_not_linked_to_descriptor_node")
                elif child.get("input_read") is None:
                    reject(i, name, "missing_child_input_read")
                elif child.get("output_write") is None:
                    child["output_write"] = i
                    if child["origin"] == "internal-executed":
                        witnesses.append({
                            "descriptor_event_id": child["descriptor_event_id"],
                            "graph_creation_event_id": child["graph_creation_event_id"],
                            "launch_event_id": child["launch_event_id"],
                            "child_input_event_id": child["input_read"],
                            "child_output_event_id": i,
                            "graph": child["graph"],
                            "child_process": actor,
                            "status": "TRACEABLE_EXECUTION_ONLY",
                            "functional_causality": "NOT EVALUABLE",
                            "autonomous_design": "NOT EVALUABLE",
                        })
                        counts["traceable_execution_chain"] += 1
                    else:
                        reject(i, name, child["origin"] + "_descriptor_source")
            continue
        if name == "CellRead":
            if actor is None or event.get("process") != actor:
                raise ValueError(f"internal cell read has invalid actor at {i}")
            key, value = int(data["key"]), int(data["value"])
            last_read[(actor, key)] = (value, i)
            if actor in children:
                child = children[actor]
                node = child["code"].get(event.get("pc"))
                if node is not None and node["opcode_tag"] == 3 and child.get("input_read") is None:
                    child["input_read"] = i
            continue
        if name == "GraphCreated":
            if actor is None or actor != data["owner"]:
                raise ValueError(f"bad graph allocation actor at {i}")
            gid = int(data["graph"])
            if gid in graphs:
                reject(i, name, "graph_identity_reused")
            graphs[gid] = {"created": i, "owner": actor, "code": {}, "invalid": False}
            continue
        if name in ("GraphEncoded", "GraphReplaced"):
            if actor is None or actor != data["owner"]:
                raise ValueError(f"bad descriptor graph actor at {i}")
            gid = int(data["graph"])
            graph = graphs.get(gid)
            if graph is None or graph["owner"] != actor:
                reject(i, name, "missing_graph_ancestry")
                continue
            words = data["descriptor"]
            if not isinstance(words, list) or len(words) != 5:
                reject(i, name, "invalid_descriptor")
                continue
            base = int(data["descriptor_base"])
            origins = set()
            read_ids = []
            writer_ids = []
            for offset, word in enumerate(words):
                cell = cells.get(base + offset)
                if cell is None or cell[2] != word:
                    origins.add("unknown")
                else:
                    origins.add(cell[0])
                    writer_ids.append(cell[3])
                read = last_read.get((actor, base + offset))
                # Explicit successful CellRead witnesses, not an inferred CIL read.
                if read is not None and read[0] == word and read[1] < i:
                    read_ids.append(read[1])
            if "unknown" in origins:
                origin = "unknown"
            elif "external" in origins:
                origin = "external"
            else:
                origin = "internal-executed"
            if len(read_ids) != 5 or any(
                (base + j in cells and read_ids[j] <= cells[base + j][3])
                for j in range(5)
            ):
                reject(i, name, "missing_explicit_descriptor_reads")
                graph["invalid"] = True
                continue
            node = int(data.get("new_node", data.get("node", -1)))
            if node < 0 or (name == "GraphEncoded" and node in graph["code"]):
                reject(i, name, "invalid_node_lineage")
                graph["invalid"] = True
                continue
            # The event records the exact data words; not causal design authorship.
            graph["code"][node] = {
                "descriptor_event_id": i,
                "descriptor_read_event_ids": read_ids,
                "writer_event_ids": writer_ids,
                "origin": origin,
                "opcode_tag": int(words[0]),
            }
            counts["descriptor_" + origin] += 1
            continue
        if name in ("GraphNodeEdited", "GraphNodeCopied", "GraphCopied"):
            gid = int(data.get("graph", data.get("target_graph", -1)))
            if gid in graphs and graphs[gid]["code"]:
                graphs[gid]["invalid"] = True
            continue
        if name == "GraphTransferred":
            gid = int(data["graph"])
            if gid in graphs:
                graphs[gid]["owner"] = int(data["new_owner"])
            continue
        if name == "GraphLaunched":
            if actor is None or actor != data["parent_process"]:
                raise ValueError(f"bad launch actor at {i}")
            gid = int(data["source_graph"])
            child_id = int(data["child_process"])
            graph = graphs.pop(gid, None)
            if child_id in children:
                reject(i, name, "child_identity_reused")
                continue
            if graph is None:
                reject(i, name, "missing_graph_ancestry")
                continue
            if graph["owner"] != actor or not graph["code"] or graph["invalid"]:
                reject(i, name, "missing_effective_code_link")
                continue
            moved = int(data["moved_nodes"])
            if moved < 2 or set(graph["code"]) != set(range(moved)):
                reject(i, name, "incomplete_executable_node_ancestry")
                continue
            origins = {node["origin"] for node in graph["code"].values()}
            origin = ("unknown" if "unknown" in origins else
                      "external" if "external" in origins else "internal-executed")
            children[child_id] = {
                "code": graph["code"],
                "descriptor_event_id": [node["descriptor_event_id"] for node in graph["code"].values()],
                "origin": origin,
                "graph_creation_event_id": graph["created"],
                "launch_event_id": i,
                "graph": gid,
                "input_read": None,
                "output_write": None,
            }
    return {
        "status": "OBSERVER_CALIBRATION_ONLY",
        "missing_causal_value_dependency": "NOT EVALUABLE: event provenance alone has no register/dataflow taint",
        "functional_causality": "NOT EVALUABLE",
        "autonomous_maintenance": "NOT EVALUABLE",
        "counts": dict(sorted(counts.items())),
        "trapped_event_ids": traps,
        "witnesses": witnesses,
        "rejections": rejections,
    }


def audit_database(path: Path) -> dict:
    db = sqlite3.connect(f"{path.resolve().as_uri()}?mode=ro", uri=True)
    try:
        revisions: set[str] = set()
        versions: set[int] = set()
        totals: Counter = Counter()
        witnesses: list[dict] = []
        rejections: list[dict] = []
        runs = 0
        for run_id, revision, initial_json, initial_sha, final_json, final_sha in db.execute(
            "SELECT id,source_revision,initial_json,initial_sha256,final_json,final_sha256 "
            "FROM epoch_runs ORDER BY id"
        ):
            if analyze.digest(initial_json) != initial_sha or analyze.digest(final_json) != final_sha:
                raise ValueError(f"snapshot hash mismatch at run {run_id}")
            if not revision:
                raise ValueError(f"missing source revision at run {run_id}")
            revisions.add(revision)
            initial, final = json.loads(initial_json), json.loads(final_json)
            versions.add(int(final["format_version"]))
            if final["format_version"] != initial["format_version"]:
                raise ValueError(f"world schema version changed within run {run_id}")
            if len(final["events"]) < len(initial["events"]):
                raise ValueError(f"event history truncated at run {run_id}")
            if final["events"][: len(initial["events"])] != initial["events"]:
                raise ValueError(f"initial event prefix mutated at run {run_id}")
            result = audit_events(final["events"])
            totals.update(result["counts"])
            totals["trapped_internal_instructions"] += len(result["trapped_event_ids"])
            witnesses.extend({"run_id": run_id, **v} for v in result["witnesses"])
            rejections.extend({"run_id": run_id, **v} for v in result["rejections"])
            runs += 1
        return {
            "study": "CL-1 Study 18",
            "source_revision": sorted(revisions),
            "schema": "epoch_runs / versioned World event log",
            "world_format_versions": sorted(versions),
            "runs": runs,
            "counts": dict(sorted(totals.items())),
            "witnesses": witnesses,
            "rejections": rejections,
            "functional_causality": "NOT EVALUABLE",
            "life_claim": "NONE",
            "caveat": "Time-ordered execution provenance is not causal dependence or internally invented design.",
        }
    finally:
        db.close()


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("sqlite", type=Path)
    parser.add_argument("--json", type=Path, required=True)
    args = parser.parse_args()
    report = audit_database(args.sqlite)
    args.json.write_text(json.dumps(report, indent=2) + "\n", encoding="utf-8")
    print(json.dumps({k: report[k] for k in ("study", "runs", "counts", "functional_causality")}))


if __name__ == "__main__":
    main()
