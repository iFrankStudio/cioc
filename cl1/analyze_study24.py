#!/usr/bin/env python3
"""Study 24: read-only source-graph-read to same-actor output exposure audit.

Temporal opportunity is NOT a causal register-dataflow or a life demonstration.
"""
from __future__ import annotations

import argparse
import json
import sqlite3
from collections import Counter, defaultdict
from pathlib import Path

import analyze
import analyze_study22
from analyze_study02 import wilson_95

WINDOWS = ((0, 8), (8, 16), (16, 24))
LABELS = analyze_study22.LABELS
SEEDS = analyze_study22.SEEDS


def epoch_exposure(events: list[dict]) -> dict:
    """Count explicit successful foreign-graph reads followed by same-actor writes.

    Every epoch resets the read set. Tick and event order must both agree.
    """
    read_ticks: dict[int, int] = {}
    counts = Counter()
    pairs = Counter()
    first_candidates: list[dict] = []
    prior_tick = -1
    for ordinal, event in enumerate(events):
        tick = int(event["tick"])
        if tick < prior_tick:
            raise ValueError("event ticks go backwards within epoch")
        prior_tick = tick
        kind = event["kind"]
        if not isinstance(kind, dict):
            continue
        actor = analyze.internal_process(event)
        if actor is None:
            continue
        if event["process"] != actor:
            raise ValueError("internal event actor/process conflict")
        payload = kind.get("GraphTagRead", kind.get("GraphFieldRead"))
        if payload is not None:
            for key in ("graph", "owner", "node"):
                if key not in payload:
                    raise ValueError("incomplete source-graph read attribution")
            read_type = "GraphTagRead" if "GraphTagRead" in kind else "GraphFieldRead"
            expected_field = "tag" if read_type == "GraphTagRead" else "word"
            if expected_field not in payload:
                raise ValueError("missing observed graph read value")
            if read_type == "GraphFieldRead" and "field" not in payload:
                raise ValueError("missing field index")
            counts["all_graph_reads"] += 1
            if int(payload["owner"]) != actor:
                counts["foreign_graph_reads"] += 1
                pairs[(int(payload["owner"]), actor)] += 1
                read_ticks.setdefault(actor, tick)
        if "CellWritten" in kind:
            counts["cell_writes"] += 1
            if actor in read_ticks and tick > read_ticks[actor]:
                counts["same_actor_read_then_write"] += 1
                first_candidates.append({
                    "actor": actor,
                    "write_tick": tick,
                    "write_event_index": ordinal,
                    "write_cell": int(kind["CellWritten"]["key"]),
                })
    return {
        "counts": counts,
        "pairs": pairs,
        "candidates": first_candidates,
    }


def audit(path: Path) -> dict:
    # First perform the full frozen cohort, matched-plan and hash audit.
    baseline = analyze_study22.audit(path)
    if baseline["matched_runs"] != 768 or baseline["hashed_epoch_snapshots"] != 18432:
        raise ValueError("not a complete preregistered Study-22 generator replay")
    db = sqlite3.connect(f"{path.resolve().as_uri()}?mode=ro", uri=True)
    try:
        rows = db.execute(
            "SELECT id,condition,initial_json FROM epoch_runs ORDER BY id"
        ).fetchall()
        snapshots: dict[int, list[tuple[int, str]]] = defaultdict(list)
        for run_id, epoch, raw in db.execute(
            "SELECT run_id,epoch,state_json FROM epoch_snapshots ORDER BY run_id,epoch"
        ):
            snapshots[run_id].append((epoch, raw))
        worlds: dict[int, dict[str, dict]] = defaultdict(dict)
        totals = Counter()
        source_pairs = Counter()
        trace_examples = []
        for run_id, condition, initial_json in rows:
            prefix, arm = condition.split("/", 1)
            seed = int(prefix.removeprefix("seed="))
            if arm in worlds[seed]:
                raise ValueError("duplicate branch")
            initial = json.loads(initial_json)
            previous = len(initial["events"])
            windows = [Counter(), Counter(), Counter()]
            epoch_candidates = [False] * 24
            for epoch, raw in snapshots[run_id]:
                snapshot = json.loads(raw)
                events = snapshot["events"][previous:]
                previous = len(snapshot["events"])
                result = epoch_exposure(events)
                window = epoch // 8
                windows[window].update(result["counts"])
                for (owner, reader), n in result["pairs"].items():
                    source_pairs[f"{arm}.W{window+1}.owner={owner}.reader={reader}"] += n
                epoch_candidates[epoch] = bool(result["candidates"])
                if result["candidates"] and len(trace_examples) < 12:
                    trace_examples.append({
                        "seed": seed, "arm": arm, "epoch": epoch,
                        **result["candidates"][0],
                    })
            for w, counts in enumerate(windows):
                totals.update({f"{arm}.W{w+1}.{k}": v for k, v in counts.items()})
            worlds[seed][arm] = {
                "windows": windows,
                "candidates": [
                    any(epoch_candidates[lo:hi]) for lo, hi in WINDOWS
                ],
            }
        if len(worlds) != SEEDS or any(set(arms) != set(LABELS) for arms in worlds.values()):
            raise ValueError("unbalanced complete independent seed worlds")
        proportions = Counter()
        paired = Counter()
        for arms in worlds.values():
            for arm in LABELS:
                for w, counts in enumerate(arms[arm]["windows"]):
                    for metric in ("foreign_graph_reads", "cell_writes",
                                   "same_actor_read_then_write"):
                        proportions[f"{arm}.W{w+1}.{metric}"] += counts[metric] > 0
            baseline_candidate = arms["control"]["candidates"][2]
            refill_candidate = arms["refill"]["candidates"][2]
            paired["refill_only_w3_candidate"] += refill_candidate and not baseline_candidate
            paired["control_only_w3_candidate"] += baseline_candidate and not refill_candidate
        intervals = {
            key: {
                "seed_worlds": n,
                "denominator": SEEDS,
                "wilson_95": [round(q, 6) for q in wilson_95(n, SEEDS)],
            } for key, n in sorted(proportions.items())
        }
        return {
            "study": "CL-1 Study 24",
            "source_revision": baseline["source_revision"],
            "method": "retrospective instrumented rerun of frozen Study-22 cohort",
            "independent_new_seed_worlds": 0,
            "reused_independent_seed_ids": SEEDS,
            "matched_runs": baseline["matched_runs"],
            "hashed_epoch_snapshots": baseline["hashed_epoch_snapshots"],
            "counts": dict(sorted(totals.items())),
            "source_owner_reader_pairs": dict(sorted(source_pairs.items())),
            "per_arm_window_exposure": intervals,
            "paired_w3_candidate_discordance": dict(paired),
            "trace_examples_noncausal": trace_examples,
            "functional_value_causality": "NOT EVALUABLE",
            "internal_code_design_origin": "NOT EVALUABLE",
            "autonomous_functional_recovery": "NOT EVALUABLE",
            "life_claim": "NONE",
            "caveat": "Read then write in one epoch is temporal association, never dataflow taint; external scaffold, compute subsidies and structural restoration cannot be claimed endogenous.",
        }
    finally:
        db.close()


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("sqlite", type=Path)
    parser.add_argument("--json", type=Path, required=True)
    args = parser.parse_args()
    report = audit(args.sqlite)
    args.json.write_text(json.dumps(report, indent=2) + "\n", encoding="utf-8")
    print(json.dumps({
        "study": report["study"],
        "source_revision": report["source_revision"],
        "reused_seed_worlds": report["reused_independent_seed_ids"],
        "matched_runs": report["matched_runs"],
        "paired_w3_candidate": report["paired_w3_candidate_discordance"],
        "functional_causality": report["functional_value_causality"],
    }))


if __name__ == "__main__":
    main()
