#!/usr/bin/env python3
"""Study 26: post-result exploratory audit of Study-25 g2 output dropouts.

Uses the exact Study-25 cohort and does not provide new independent evidence.
"""
from __future__ import annotations

import argparse
import json
import sqlite3
from collections import Counter, defaultdict
from pathlib import Path

import analyze
import analyze_study25
from analyze_study25 import _targeted_first_outputs

CATEGORY = (
    "changed_numeric_output",
    "same_numeric_output_after_changed_tag",
    "changed_read_no_output",
    "no_changed_read_no_output",
    "other_or_ambiguous",
)


def _actor_events(events: list[dict], actor: int) -> list[dict]:
    return [e for e in events if analyze.internal_process(e) == actor]


def _read_events(events: list[dict], actor: int) -> list[dict]:
    return [
        e for e in _actor_events(events, actor)
        if isinstance(e["kind"], dict)
        and isinstance(e["kind"].get("GraphTagRead"), dict)
        and int(e["kind"]["GraphTagRead"]["graph"]) == 2
        and int(e["kind"]["GraphTagRead"]["owner"]) != actor
    ]


def classify(control: list[dict], modified: list[dict], actor: int, cell: int) -> dict:
    """Classify one paired actor+cell+epoch without inferring source causality."""
    base, ambiguous_old = _targeted_first_outputs(control, 2)
    altered, ambiguous_new = _targeted_first_outputs(modified, 2)
    key = (actor, cell)
    if key not in base or key in ambiguous_old:
        raise ValueError("selected control output not qualified")
    old = base[key]
    reads = _read_events(modified, actor)
    tags = {int(e["kind"]["GraphTagRead"]["tag"]) for e in reads}
    if old["tag"] in tags:
        raise ValueError("mutated graph2 successful read retained original tag")
    changed = bool(reads and tags != {old["tag"]})
    writer_events = [
        e for e in _actor_events(modified, actor)
        if isinstance(e["kind"], dict) and "CellWritten" in e["kind"]
    ]
    writes_same_cell = [
        e for e in writer_events
        if int(e["kind"]["CellWritten"]["key"]) == cell
    ]
    traps = sum(
        isinstance(e["kind"], dict) and "Trapped" in e["kind"]
        for e in _actor_events(modified, actor)
    )
    foreign_fields = sum(
        isinstance(e["kind"], dict)
        and isinstance(e["kind"].get("GraphFieldRead"), dict)
        and int(e["kind"]["GraphFieldRead"]["graph"]) == 2
        for e in _actor_events(modified, actor)
    )
    later_other_cell = any(
        int(e["kind"]["CellWritten"]["key"]) != cell for e in writer_events
    )
    if key in altered and key not in ambiguous_new and changed:
        category = ("changed_numeric_output" if altered[key]["value"] != old["value"]
                    else "same_numeric_output_after_changed_tag")
    elif changed and not writes_same_cell:
        category = "changed_read_no_output"
    elif not reads and not writes_same_cell:
        category = "no_changed_read_no_output"
    else:
        category = "other_or_ambiguous"
    return {
        "category": category,
        "control_value": old["value"],
        "control_tag": old["tag"],
        "altered_tags": sorted(tags),
        "altered_target_reads": len(reads),
        "altered_same_cell_writes": len(writes_same_cell),
        "altered_actor_traps": traps,
        "altered_foreign_field_reads": foreign_fields,
        "altered_other_cell_write": later_other_cell,
    }


def audit(path: Path) -> dict:
    verified = analyze_study25.audit(path)
    if (verified["independent_seed_worlds"] != 96 or verified["matched_runs"] != 864
            or verified["hashed_epoch_snapshots"] != 20736):
        raise ValueError("study25 complete replay not verified")
    if verified["source_tag_and_value_paired_contrasts"].get(
        "mutate-g2.output_missing_after_exposure"
    ) != 3:
        raise ValueError("known prior Study-25 g2 dropout count differs")
    db = sqlite3.connect(f"{path.resolve().as_uri()}?mode=ro", uri=True)
    try:
        runs = {
            condition: (run_id, json.loads(initial))
            for run_id, condition, initial in db.execute(
                "SELECT id,condition,initial_json FROM epoch_runs"
            )
        }
        raw = defaultdict(list)
        for run_id, epoch, data in db.execute(
            "SELECT run_id,epoch,state_json FROM epoch_snapshots "
            "ORDER BY run_id,epoch"
        ):
            raw[run_id].append((epoch, json.loads(data)["events"]))
        events_by_label = {}
        for label, (run_id, initial) in runs.items():
            prev = len(initial["events"])
            events = []
            for i, arr in raw[run_id]:
                if i != len(events):
                    raise ValueError("missing epoch in exploratory auditor")
                events.append(arr[prev:])
                prev = len(arr)
            events_by_label[label] = events
        categories = Counter()
        cases = []
        sampled_worlds = defaultdict(set)
        exposed_worlds = set()
        missing_worlds = set()
        for seed in range(analyze_study25.FIRST, analyze_study25.FIRST + 96):
            baseline = events_by_label[f"seed={seed}/control"]
            mutated = events_by_label[f"seed={seed}/mutate-g2"]
            for epoch in range(24):
                old, ambiguous = _targeted_first_outputs(baseline[epoch], 2)
                for actor, cell in sorted(old):
                    if (actor, cell) in ambiguous:
                        continue
                    exposed_worlds.add(seed)
                    result = classify(baseline[epoch], mutated[epoch], actor, cell)
                    category = result["category"]
                    categories[category] += 1
                    sampled_worlds[category].add(seed)
                    if category in ("changed_read_no_output", "no_changed_read_no_output"):
                        missing_worlds.add(seed)
                    cases.append({
                        "seed": seed,
                        "epoch": epoch,
                        "actor": actor,
                        "cell": cell,
                        **result,
                    })
        if len(missing_worlds) != 3:
            raise ValueError("known strict g2 missing-output world count could not be reproduced")
        return {
            "study": "CL-1 Study 26",
            "scope": "post-outcome exploratory mechanism classification",
            "source_revision": verified["source_revision"],
            "new_independent_worlds": 0,
            "reused_seed_worlds": 96,
            "replayed_matched_runs": 864,
            "replayed_hashed_snapshots": 20736,
            "previously_known_g2_dropout_worlds": 3,
            "qualified_control_exposed_worlds": len(exposed_worlds),
            "categorized_unique_seed_worlds": {
                k: len(sampled_worlds[k]) for k in CATEGORY
            },
            "categorized_actor_epoch_cell_cases": {
                k: categories[k] for k in CATEGORY
            },
            "control_source_read_output_cases": cases,
            "scientific_conclusion": "EXPLORATORY ONLY; no preregistered code-dependent output-value recovery",
            "autonomous_repair": "NOT EVALUABLE",
            "life_claim": "NONE",
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
    print(json.dumps({k: report[k] for k in (
        "study", "source_revision", "qualified_control_exposed_worlds",
        "categorized_unique_seed_worlds", "autonomous_repair",
    )}))


if __name__ == "__main__":
    main()
