#!/usr/bin/env python3
"""Study 28: post-result retrospective graph1 necessity falsification.

Reuses all Study27 paired seed-worlds; no new ecological replicates.
"""
from __future__ import annotations

import argparse
import json
import sqlite3
from collections import Counter, defaultdict
from pathlib import Path

import analyze
import analyze_study27
from analyze_study27 import strict_output

ARMS = ("control", "mutate-g1", "mutate-g2", "mutate-both")


def graph1_reads_before_output(events: list[dict], address: int) -> int:
    """Successful graph1 reads by process1 before its first matching output."""
    output_tick = next((
        int(e["tick"]) for e in events
        if analyze.internal_process(e) == 1
        and isinstance(e["kind"], dict)
        and isinstance(e["kind"].get("CellWritten"), dict)
        and e["kind"]["CellWritten"]["key"] == address
    ), None)
    if output_tick is None:
        return 0
    return sum(
        analyze.internal_process(e) == 1
        and int(e["tick"]) < output_tick
        and isinstance(e["kind"], dict)
        and any(
            isinstance(e["kind"].get(t), dict)
            and e["kind"][t].get("graph") == 1
            and e["kind"][t].get("owner") == 1
            for t in ("GraphTagRead", "GraphFieldRead")
        )
        for e in events
    )


def classify(arms: dict[str, list[dict]]) -> dict:
    baseline = strict_output(arms["control"], 11, 11)
    g1 = strict_output(arms["mutate-g1"], 11, 11)
    g2 = strict_output(arms["mutate-g2"], 19, 19)
    both = strict_output(arms["mutate-both"], 19, 19)
    if not baseline["qualified"] or not g2["qualified"] or (
        baseline["value"] != g2["value"]
    ):
        return {
            "original_exposure": False,
            "category": "unexposed",
            "second_component_source_read": False,
        }
    reads = {
        arm: graph1_reads_before_output(
            arms[arm], 19 if arm in ("mutate-g2", "mutate-both") else 11
        ) for arm in ARMS
    }
    g1_ok = g1["qualified"] and g1["value"] == baseline["value"]
    both_ok = both["qualified"] and both["value"] == g2["value"]
    if g1_ok and both_ok:
        category = "graph1_not_necessary_observed"
    elif not g1_ok or not both_ok:
        category = "graph1_interference_observed"
    else:
        category = "indeterminate"
    # An independent graph1 component can only be read-mediated if its source
    # was actually read before a relevant successful original output.
    second_source_exposed = reads["control"] > 0 or reads["mutate-g2"] > 0
    return {
        "original_exposure": True,
        "category": category,
        "second_component_source_read": second_source_exposed,
        "source1_reads_before_output": reads,
        "original_value": baseline["value"],
        "graph1_modified_qualified": g1_ok,
        "graph2_modified_qualified": g2["qualified"],
        "both_modified_qualified": both_ok,
        "two_component_read_mediated_dependency": (
            category == "graph1_interference_observed"
            and second_source_exposed
        ),
    }


def audit(path: Path) -> dict:
    # Full exact baseline schedule/hash/capacity/event/provenance audit first.
    prior = analyze_study27.audit(path)
    if prior["primary_address_relocation"]["seed_worlds"] != 2:
        raise ValueError("known Study27 strict positive witness count changed")
    if (prior["matched_runs"] != 1344
            or prior["hashed_epoch_snapshots"] != 10752):
        raise ValueError("incomplete Study27 replay")
    db = sqlite3.connect(f"{path.resolve().as_uri()}?mode=ro", uri=True)
    try:
        by_run = {}
        for id, label, initial in db.execute(
            "SELECT id,condition,initial_json FROM epoch_runs"
        ):
            by_run[id] = (label, len(json.loads(initial)["events"]))
        events0 = {}
        for id, state_json in db.execute(
            "SELECT run_id,state_json FROM epoch_snapshots WHERE epoch=0"
        ):
            label, previous_count = by_run[id]
            events0[label] = json.loads(state_json)["events"][previous_count:]
        if len(events0) != 1344:
            raise ValueError("missing one or more registered epoch0 snapshots")
        known = {w["seed"] for w in prior["strict_matched_witnesses"]}
        counts = Counter()
        witnesses = []
        for seed in range(analyze_study27.FIRST, analyze_study27.FIRST + 192):
            arms = {
                arm: events0[f"seed={seed}/{arm}"]
                for arm in ARMS
            }
            found = classify(arms)
            counts["all_worlds"] += 1
            counts["exposed_worlds"] += found["original_exposure"]
            if found["original_exposure"]:
                counts[found["category"]] += 1
                counts["source1_read_before_output"] += found["second_component_source_read"]
                counts["read_mediated_two_component_witness"] += found[
                    "two_component_read_mediated_dependency"
                ]
                witnesses.append({"seed": seed, **found})
        if {w["seed"] for w in witnesses} != known:
            raise ValueError("Study27 matched strict source witnesses changed")
        return {
            "study": "CL-1 Study 28",
            "source_revision": prior["source_revision"],
            "scope": "post-result secondary-source falsification in known Study27 winners",
            "new_independent_worlds": 0,
            "reused_independent_worlds": 192,
            "matched_runs_replayed": prior["matched_runs"],
            "hashed_snapshots_replayed": prior["hashed_epoch_snapshots"],
            "counts": dict(sorted(counts.items())),
            "known_source2_strict_witnesses": witnesses,
            "scientific_second_component_evidence": (
                "READ-MEDIATED TWO-COMPONENT EXPOSURE ONLY"
                if counts["read_mediated_two_component_witness"]
                else "NOT DEMONSTRATED"
            ),
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
    report = audit(args.sqlite)
    args.json.write_text(json.dumps(report, indent=2) + "\n", encoding="utf-8")
    print(json.dumps({
        "study": report["study"],
        "new_worlds": report["new_independent_worlds"],
        "counts": report["counts"],
        "second_component": report["scientific_second_component_evidence"],
    }))


if __name__ == "__main__":
    main()
