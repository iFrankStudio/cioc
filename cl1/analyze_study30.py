#!/usr/bin/env python3
"""Study 30: post-outcome native two-source read/Add/write pipeline attrition.

This reuses Study29 independent seeds and does not retune the old outcome.
"""
from __future__ import annotations

import argparse
import json
import sqlite3
from collections import Counter, defaultdict
from pathlib import Path

import analyze
import analyze_study29

TARGET = {
    "control": (11, 11, 22),
    "sham": (11, 11, 22),
    "mutate-g1": (19, 11, 30),
    "mutate-g2": (11, 19, 30),
    "mutate-both": (19, 19, 38),
    "exact-original-g2": (11, 11, 22),
    "zero-credit": (11, 11, 22),
}


def stages(events: list[dict], program: list, tag1: int, tag2: int,
           output_address: int) -> dict:
    """Observe native success events at the original program PC, no inferred Add."""
    read_ticks = {1: [], 2: []}
    add_ticks = []
    writes = []
    traps = []
    for event in events:
        actor = analyze.internal_process(event)
        if actor != 1 or event["process"] != 1:
            continue
        tick = int(event["tick"])
        kind = event["kind"]
        if isinstance(kind, dict) and "GraphTagRead" in kind:
            payload = kind["GraphTagRead"]
            graph = payload.get("graph")
            if graph in (1, 2):
                expected_tag = tag1 if graph == 1 else tag2
                if (payload.get("owner") == graph
                        and payload.get("node") == 0
                        and payload.get("tag") == expected_tag):
                    read_ticks[graph].append(tick)
        if kind == "Executed":
            pc = event["pc"]
            if not isinstance(pc, int) or not 0 <= pc < len(program):
                raise ValueError("native executed PC absent from prefork code")
            if isinstance(program[pc], dict) and "Add" in program[pc]:
                add_ticks.append(tick)
        if isinstance(kind, dict) and "CellWritten" in kind:
            writes.append((tick, int(kind["CellWritten"]["key"])))
        if isinstance(kind, dict) and "Trapped" in kind:
            traps.append(tick)

    both = bool(read_ticks[1] and read_ticks[2])
    both_complete = max(read_ticks[1][0], read_ticks[2][0]) if both else None
    later_adds = [tick for tick in add_ticks if both and tick > both_complete]
    later_writes = [
        (tick, key)
        for tick, key in writes
        if later_adds and any(tick > add for add in later_adds)
    ]
    return {
        "source1": bool(read_ticks[1]),
        "source2": bool(read_ticks[2]),
        "both": both,
        "add_after_both": bool(later_adds),
        "write_after_add": bool(later_writes),
        "target_write_after_add": any(
            key == output_address for _, key in later_writes
        ),
        "any_write": bool(writes),
        "trap_after_both": any(tick > both_complete for tick in traps) if both else False,
        "add_success_events": len(add_ticks),
        "write_success_events": len(writes),
    }


def audit(path: Path) -> dict:
    prior = analyze_study29.audit(path)
    if prior["primary_dual_source_additive_function"]["seed_worlds"] != 0:
        raise ValueError("original known strict Study29 null changed")
    if (prior["matched_runs"] != 1792
            or prior["hashed_epoch_snapshots"] != 14336):
        raise ValueError("original Study29 complete cohort missing")
    db = sqlite3.connect(f"{path.resolve().as_uri()}?mode=ro", uri=True)
    try:
        rows = db.execute(
            "SELECT id,condition,initial_json FROM epoch_runs ORDER BY id"
        ).fetchall()
        first = {
            run_id: json.loads(state_json)["events"]
            for run_id, state_json in db.execute(
                "SELECT run_id,state_json FROM epoch_snapshots WHERE epoch=0"
            )
        }
        if len(rows) != 1792 or len(first) != 1792:
            raise ValueError("missing registered epoch0 snapshot")
        worlds = defaultdict(dict)
        totals = Counter()
        for run_id, label, initial_raw in rows:
            prefix, arm = label.split("/", 1)
            seed = int(prefix.removeprefix("seed="))
            if arm not in TARGET or seed not in range(2_400_000, 2_400_256):
                raise ValueError("unexpected seed or matched arm")
            initial = json.loads(initial_raw)
            prefork_events = len(initial["events"])
            events = first[run_id][prefork_events:]
            code = initial["processes"]["1"]["code"]["nodes"]
            result = stages(events, code, *TARGET[arm])
            worlds[seed][arm] = result
            for metric, value in result.items():
                if metric not in ("add_success_events", "write_success_events"):
                    totals[f"{arm}.{metric}"] += int(value)
                else:
                    totals[f"raw.{arm}.{metric}"] += value
        if (len(worlds) != 256
                or any(set(arms) != set(TARGET) for arms in worlds.values())):
            raise ValueError("incomplete independent-seed pipeline audit")
        for arm in TARGET:
            names = ("source1", "source2", "both", "add_after_both",
                     "write_after_add", "target_write_after_add")
            values = [totals[f"{arm}.{name}"] for name in names]
            if values[2] > min(values[0], values[1]):
                raise ValueError("both-source read exceeded individual exposures")
            if any(x < y for x, y in zip(values[2:], values[3:])):
                raise ValueError("later pipeline stage exceeded prerequisite")
            if arm == "zero-credit" and any(
                totals[f"{arm}.{name}"] for name in names
            ):
                raise ValueError("zero-credit branch acquired native exposure")
        return {
            "study": "CL-1 Study 30",
            "source_revision": prior["source_revision"],
            "scope": "post-result exploratory bottleneck on frozen Study29 cohort",
            "new_independent_worlds": 0,
            "reused_independent_worlds": 256,
            "matched_runs_replayed": 1792,
            "hashed_snapshots_replayed": 14336,
            "stage_world_counts_out_of_256": {
                k: v for k, v in sorted(totals.items()) if not k.startswith("raw.")
            },
            "raw_add_write_events": {
                k: v for k, v in sorted(totals.items()) if k.startswith("raw.")
            },
            "dual_code_content_causality": "NOT DEMONSTRATED",
            "endogenous_maintenance": "NOT EVALUABLE",
            "life_claim": "NONE",
        }
    finally:
        db.close()


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("sqlite", type=Path)
    parser.add_argument("--json", required=True, type=Path)
    args = parser.parse_args()
    result = audit(args.sqlite)
    args.json.write_text(json.dumps(result, indent=2) + "\n", encoding="utf-8")
    print(json.dumps({
        "study": result["study"],
        "source_revision": result["source_revision"],
        "new_worlds": result["new_independent_worlds"],
        "control_pipeline": {
            k: v for k, v in result["stage_world_counts_out_of_256"].items()
            if k.startswith("control.")
        },
        "conclusion": result["dual_code_content_causality"],
    }))


if __name__ == "__main__":
    main()
