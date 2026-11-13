#!/usr/bin/env python3
"""CL-1 Study 20: independent read-only two-graph functional calibration audit.

One externally scripted fixture, nine matched arms; NEVER a cyberlife cohort.
"""
from __future__ import annotations

import argparse
import copy
import json
import sqlite3
from pathlib import Path

import analyze

LABELS = ("control", "sham", "drop1", "drop2", "drop-both",
          "restore1", "restore2", "mutate2", "zero-credit")
OUTPUT = {"control": 11, "sham": 11, "restore1": 11,
          "restore2": 11, "mutate2": 30}
EPOCHS = 3
QUOTA = 12


def schedule(label: str, initial: dict) -> list[dict]:
    def at(action: dict) -> dict:
        return {"epoch": 0, "action": action}
    actions = [
        at({"SetCredits": {"process": 1, "credits": 0}}),
        at({"SetCredits": {"process": 2, "credits": 0}}),
    ]
    if label == "sham":
        actions.append(at({"DropGraph": {"graph": 0}}))
    if label in ("drop1", "drop-both", "restore1"):
        actions.append(at({"DropGraph": {"graph": 1}}))
    if label in ("drop2", "drop-both", "restore2", "mutate2"):
        actions.append(at({"DropGraph": {"graph": 2}}))
    if label == "restore1":
        actions.append(at({"PutGraph": {
            "graph": 1, "object": initial["graph_objects"]["1"],
        }}))
    if label in ("restore2", "mutate2"):
        obj = copy.deepcopy(initial["graph_objects"]["2"])
        if label == "mutate2":
            obj["nodes"][0] = "Halt"
        actions.append(at({"PutGraph": {"graph": 2, "object": obj}}))
    actions.append(at({"SetCredits": {
        "process": 3,
        "credits": 0 if label == "zero-credit" else 9,
    }}))
    return actions


def simulated(world: dict) -> dict:
    return {key: world[key] for key in
            ("processes", "graph_objects", "cells", "compute_pool")}


def audit(path: Path) -> dict:
    db = sqlite3.connect(f"{path.resolve().as_uri()}?mode=ro", uri=True)
    try:
        rows = db.execute(
            "SELECT id,source_revision,condition,initial_sha256,initial_json,"
            "actions_json,epoch_count,quota,attempts_json,final_sha256,final_json "
            "FROM epoch_runs ORDER BY id"
        ).fetchall()
        if len(rows) != 9:
            raise ValueError(f"expected exactly nine control branches, got {len(rows)}")
        snapshots = {}
        for run_id, epoch, sha, data in db.execute(
            "SELECT run_id,epoch,state_sha256,state_json "
            "FROM epoch_snapshots ORDER BY run_id,epoch"
        ):
            if not 0 <= epoch < EPOCHS or analyze.digest(data) != sha:
                raise ValueError("invalid archived epoch snapshot")
            snapshots.setdefault(run_id, []).append((epoch, sha, data))
        outcomes = {}
        revisions = set()
        initial_hashes = set()
        for (run_id, revision, label, initial_sha, initial_json, actions_json,
             epochs, quota, attempts_json, final_sha, final_json) in rows:
            if label not in LABELS or label in outcomes:
                raise ValueError(f"invalid duplicate/unregistered arm: {label}")
            if not revision or epochs != EPOCHS or quota != QUOTA:
                raise ValueError("incomplete study parameters")
            if (analyze.digest(initial_json) != initial_sha
                    or analyze.digest(final_json) != final_sha):
                raise ValueError(f"initial/final digest mismatch: {label}")
            revisions.add(revision)
            initial_hashes.add(initial_sha)
            initial, final = json.loads(initial_json), json.loads(final_json)
            if initial["total_storage_limit"] != 64 or not initial["strict_accounting"]:
                raise ValueError("world capacity differs from frozen specification")
            if (analyze.occupied_units(initial) > 64
                    or analyze.occupied_units(final) > 64):
                raise ValueError("strict storage exceeded")
            if initial["cells"].get("90") is not None:
                raise ValueError("external prefork functional output")
            for graph_id, owner, tag in ((1, 1, "GraphNew"), (2, 2, "Const")):
                obj = initial["graph_objects"].get(str(graph_id))
                if obj is None or obj["owner"] != owner or 3 not in obj["readers"]:
                    raise ValueError("missing prefork internal graph opportunity")
                if len(obj["nodes"]) != 1 or tag not in obj["nodes"][0]:
                    raise ValueError("unexpected prefork source code")
            if json.loads(actions_json) != schedule(label, initial):
                raise ValueError(f"unregistered intervention schedule: {label}")
            attempts = json.loads(attempts_json)
            if len(attempts) != EPOCHS or any(
                not isinstance(n, int) or not 0 <= n <= QUOTA for n in attempts
            ):
                raise ValueError("invalid epoch attempt counts")
            snaps = snapshots.get(run_id, [])
            if len(snaps) != EPOCHS or any(
                epoch != i for i, (epoch, _, _) in enumerate(snaps)
            ):
                raise ValueError(f"incomplete epoch snapshots for {label}")
            if snaps[-1][1:] != (final_sha, final_json):
                raise ValueError("last epoch not equal to final world")
            if final["events"][:len(initial["events"])] != initial["events"]:
                raise ValueError("initial event ancestry altered")
            later = final["events"][len(initial["events"]):]
            output_events = [e for e in later if (
                analyze.internal_process(e) == 3
                and isinstance(e["kind"], dict)
                and e["kind"].get("CellWritten", {}).get("key") == 90
            )]
            expected = OUTPUT.get(label)
            final_value = final["cells"].get("90")
            if expected is None:
                if final_value is not None or output_events:
                    raise ValueError(f"unregistered output in {label}")
            elif final_value != expected or len(output_events) != 1 or (
                output_events[0]["kind"]["CellWritten"]["value"] != expected
            ):
                raise ValueError(f"functional output mismatch: {label}")
            if label == "zero-credit" and (
                any(attempts) or any(analyze.internal_process(e) is not None for e in later)
            ):
                raise ValueError("zero-credit branch executed")
            if label in ("drop1", "drop2", "drop-both") and not any(
                analyze.internal_process(e) == 3
                and isinstance(e["kind"], dict) and "Trapped" in e["kind"]
                for e in later
            ):
                raise ValueError("missing CIL lesion consequence")
            for expected_kind, expected_n in (
                ("PutGraph", 1 if label in ("restore1", "restore2", "mutate2") else 0),
                ("DropGraph", 2 if label == "drop-both"
                 else 1 if label in ("drop1", "drop2", "restore1", "restore2", "mutate2") else 0),
            ):
                n = sum(
                    e["origin"] == "External"
                    and isinstance(e["kind"], dict)
                    and "Intervened" in e["kind"]
                    and expected_kind in e["kind"]["Intervened"]
                    for e in later
                )
                if n != expected_n:
                    raise ValueError(f"external provenance mismatch: {label}, {expected_kind}")
            if label == "sham" and not any(
                e["origin"] == "External"
                and isinstance(e["kind"], dict)
                and "InterventionRejected" in e["kind"]
                for e in later
            ):
                raise ValueError("sham action was not rejected")
            outcomes[label] = final
        if set(outcomes) != set(LABELS) or len(revisions) != 1 or len(initial_hashes) != 1:
            raise ValueError("incomplete or unpaired cohort")
        if simulated(outcomes["control"]) != simulated(outcomes["sham"]):
            raise ValueError("sham and untouched simulated states differ")
        for label, graph_id in (("restore1", "1"), ("restore2", "2")):
            if outcomes[label]["graph_objects"][graph_id] != outcomes["control"]["graph_objects"][graph_id]:
                raise ValueError(f"same-object external restoration failed: {label}")
        return {
            "study": "CL-1 Study 20",
            "kind": "externally authored functional assay calibration",
            "source_revision": next(iter(revisions)),
            "paired_arms": len(outcomes),
            "independent_ecological_seed_worlds": 0,
            "epoch_snapshots": sum(map(len, snapshots.values())),
            "outputs": {label: outcomes[label]["cells"].get("90") for label in LABELS},
            "sham_equivalent": True,
            "functional_two_graph_lesion_control": "PASSED",
            "same_object_external_restoration_control": "PASSED",
            "source_content_mutation_control": "PASSED",
            "longitudinal_autonomous_recovery": "NOT EVALUABLE",
            "organizational_closure": "NOT EVALUABLE",
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
    print(json.dumps(report))


if __name__ == "__main__":
    main()
