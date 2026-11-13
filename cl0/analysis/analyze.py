#!/usr/bin/env python3
"""Read-only CL-0 SQLite analysis. Descriptive engineering metrics, never a life detector."""
from __future__ import annotations
import argparse
import csv
import hashlib
import json
import sqlite3
from collections import Counter
from pathlib import Path

def _digest(value: str) -> str:
    return hashlib.sha256(value.encode("utf-8")).hexdigest()

def _events(world: dict) -> list[dict]:
    return world.get("events", [])

def summarize_world(world: dict) -> dict:
    events = _events(world)
    actors = world.get("processes", {})
    cells = world.get("cells", {})
    graphs = world.get("graph_objects", {})
    kinds = Counter(next(iter(e.get("kind", {})), str(e.get("kind"))) if isinstance(e.get("kind"), dict)
                    else str(e.get("kind")) for e in events)
    internals = [e for e in events if isinstance(e.get("origin"), dict) and "Internal" in e["origin"]]
    ext = [e for e in events if e.get("origin") == "External"]
    opcodes = Counter()
    for actor in actors.values():
        for node in actor.get("code", {}).get("nodes", []):
            if isinstance(node, str):
                opcodes[node] += 1
            elif isinstance(node, dict):
                opcodes[next(iter(node), "unknown")] += 1
    return {
        "attempt_events": len(internals),
        "external_events": len(ext),
        "cells": len(cells),
        "processes": len(actors),
        "graph_objects": len(graphs),
        "code_nodes": sum(len(a.get("code", {}).get("nodes", [])) for a in actors.values())
                      + sum(len(g.get("nodes", [])) for g in graphs.values()),
        "remaining_credits": sum(a.get("credits", 0) for a in actors.values()),
        "distinct_opcode_types": len(opcodes),
        "opcode_histogram": dict(sorted(opcodes.items())),
        "event_histogram": dict(sorted(kinds.items())),
        "active_processes": sum(not a.get("stopped", False) and a.get("credits", 0) > 0 for a in actors.values()),
    }

def analyze(db: Path) -> dict:
    # SQLite immutable read-only URI prevents accidentally editing the experiment registry.
    uri = db.resolve().as_uri() + "?mode=ro&immutable=1"
    connection = sqlite3.connect(uri, uri=True)
    connection.row_factory = sqlite3.Row
    records = []
    for table, kind in (("runs", "initial"), ("scheduled_runs", "scheduled")):
        available = connection.execute(
            "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name=?", (table,)
        ).fetchone()[0]
        if not available:
            continue
        for row in connection.execute(
            f"SELECT id, source_revision, condition, initial_sha256, initial_json,"
            f"final_sha256, final_json, attempt_limit, attempts FROM {table} ORDER BY id"
        ):
            baseline = row["initial_json"]
            final = row["final_json"]
            integrity = _digest(baseline) == row["initial_sha256"] and _digest(final) == row["final_sha256"]
            try:
                world = json.loads(final)
                metrics = summarize_world(world)
            except (ValueError, TypeError, AttributeError):
                integrity = False
                metrics = {}
            records.append({
                "type": kind, "id": row["id"], "revision": row["source_revision"],
                "condition": row["condition"], "initial_sha256": row["initial_sha256"],
                "final_sha256": row["final_sha256"], "hashes_valid": integrity,
                "attempt_limit": row["attempt_limit"], "attempts": row["attempts"],
                **metrics,
            })
    connection.close()
    grouped = {}
    for record in records:
        grouped.setdefault(record["initial_sha256"], []).append(record)
    # Contrast is intentionally descriptive: no inferred biological goals or fitness.
    pairs = []
    for key, group in sorted(grouped.items()):
        if len(group) < 2:
            continue
        by_condition = sorted(group, key=lambda item: (item["condition"], item["id"]))
        for row in by_condition[1:]:
            ref = by_condition[0]
            pairs.append({
                "initial_sha256": key, "reference": ref["condition"], "treatment": row["condition"],
                "delta_attempts": row["attempts"] - ref["attempts"],
                "delta_cells": row.get("cells", 0) - ref.get("cells", 0),
                "delta_code_nodes": row.get("code_nodes", 0) - ref.get("code_nodes", 0),
            })
    return {
        "schema": "cl0-analysis-v1",
        "counts": {"runs": len(records), "matched_initials": sum(len(g) >= 2 for g in grouped.values()),
                   "invalid_hash_records": sum(not r["hashes_valid"] for r in records)},
        "records": records,
        "descriptive_contrasts": pairs,
        "inference_warning": "Instruction activity, opcode diversity and graph changes do not establish life or open-ended evolution.",
    }

def main() -> None:
    parser = argparse.ArgumentParser(description="CL-0 SQLite observation only (no life classifier)")
    parser.add_argument("database", type=Path)
    parser.add_argument("--json", type=Path, help="write self-contained dashboard data")
    parser.add_argument("--csv", type=Path, help="write flat run metrics")
    args = parser.parse_args()
    result = analyze(args.database)
    if args.json:
        args.json.parent.mkdir(parents=True, exist_ok=True)
        args.json.write_text(json.dumps(result, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
    if args.csv:
        args.csv.parent.mkdir(parents=True, exist_ok=True)
        cols = ["type", "id", "revision", "condition", "initial_sha256",
                "hashes_valid", "attempt_limit", "attempts", "cells", "processes",
                "graph_objects", "code_nodes", "distinct_opcode_types", "remaining_credits"]
        with args.csv.open("w", encoding="utf-8", newline="") as file:
            writer = csv.DictWriter(file, fieldnames=cols, extrasaction="ignore")
            writer.writeheader()
            writer.writerows(result["records"])
    print(json.dumps(result["counts"], ensure_ascii=False, sort_keys=True))

if __name__ == "__main__":
    main()
