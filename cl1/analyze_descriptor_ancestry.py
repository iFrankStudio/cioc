#!/usr/bin/env python3
"""Read-only lineage audit of successful CIL descriptor->graph transitions.

Attribution describes the most recent *observed writer of each word*,
not causal ownership of a design and never evidence of life.
"""
from __future__ import annotations

import argparse
import json
import re
import sqlite3
from collections import Counter
from pathlib import Path

import analyze

SET_CELL = re.compile(r"^SetCell \{ key: (-?\d+), value: (-?\d+) \}$")
TRANSITIONS = ("GraphEncoded", "GraphReplaced")


def audit_events(events: list[dict]) -> tuple[list[dict], Counter]:
    latest: dict[int, tuple[str, int | None, int]] = {}
    records = []
    classes: Counter = Counter()
    for ordinal, event in enumerate(events):
        kind = event.get("kind")
        if not isinstance(kind, dict):
            continue
        if "CellWritten" in kind:
            value = kind["CellWritten"]
            actor = analyze.internal_process(event)
            if actor is None:
                raise ValueError("CellWritten must have internal actor")
            latest[int(value["key"])] = ("internal-executed", actor, int(value["value"]))
        elif "Intervened" in kind and event["origin"] == "External":
            match = SET_CELL.fullmatch(str(kind["Intervened"]))
            if match:
                key, value = map(int, match.groups())
                latest[key] = ("external-set", None, value)
        for transition in TRANSITIONS:
            if transition not in kind:
                continue
            actor = analyze.internal_process(event)
            if actor is None:
                raise ValueError("graph descriptor event lacks internal actor")
            payload = kind[transition]
            if payload["owner"] != actor:
                raise ValueError("graph descriptor owner/actor mismatch")
            base = int(payload["descriptor_base"])
            words = payload["descriptor"]
            if len(words) != 5:
                raise ValueError("descriptor must contain exactly five words")
            ancestry = []
            for offset, word in enumerate(words):
                source = latest.get(base + offset)
                if source is None:
                    origin = "unknown"
                    source_actor = None
                else:
                    origin, source_actor, last_value = source
                    if last_value != word:
                        raise ValueError("recorded descriptor differs from observed last writer")
                ancestry.append({"cell": base + offset, "source": origin, "writer": source_actor})
                classes[origin] += 1
            records.append({
                "event_index": ordinal,
                "transition": transition,
                "actor": actor,
                "graph": payload["graph"],
                "node": payload.get("new_node", payload.get("node")),
                "words": words,
                "ancestry": ancestry,
            })
    return records, classes


def audit_database(path: Path) -> dict:
    db = sqlite3.connect(f"{path.resolve().as_uri()}?mode=ro", uri=True)
    try:
        result = {
            "kind": "observer-only descriptor source audit",
            "interpretation": "last observed writers, NOT autonomous design ancestry",
            "runs": 0,
            "transitions": 0,
            "word_provenance": {},
            "unknown_is_not_internal": True,
        }
        counts: Counter = Counter()
        query = "SELECT final_json,final_sha256 FROM epoch_runs"
        for final_json, final_sha in db.execute(query):
            if analyze.digest(final_json) != final_sha:
                raise ValueError("damaged epoch-run final snapshot")
            world = json.loads(final_json)
            records, local = audit_events(world["events"])
            counts.update(local)
            result["runs"] += 1
            result["transitions"] += len(records)
        result["word_provenance"] = dict(sorted(counts.items()))
        return result
    finally:
        db.close()


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("sqlite", type=Path)
    parser.add_argument("--json", type=Path, required=True)
    args = parser.parse_args()
    report = audit_database(args.sqlite)
    args.json.write_text(json.dumps(report, indent=2) + "\n", encoding="utf-8")
    print(json.dumps(report))


if __name__ == "__main__":
    main()
