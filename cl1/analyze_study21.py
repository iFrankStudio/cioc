#!/usr/bin/env python3
"""Study 21 read-only, source-pinned, temporal functional control auditor.

This fixture is externally scripted; no endogenous recovery or life claim.
"""
from __future__ import annotations

import argparse
import copy
import json
import sqlite3
from pathlib import Path

import analyze

LABELS = (
    "control", "sham", "drop1", "drop2", "drop-both",
    "restore1", "restore2", "restore-both", "mutate2", "no-repair", "zero-credit",
)
EPOCHS = 3
QUOTA = 12
EXPECTED = {
    "control": [11, 11, 11],
    "sham": [11, 11, 11],
    "restore1": [None, 11, 11],
    "restore2": [None, 11, 11],
    "restore-both": [None, 11, 11],
    "mutate2": [None, 30, 30],
}
RESTORED = {"restore1", "restore2", "restore-both", "mutate2"}


def schedule(label: str, initial: dict) -> list[dict]:
    def at(epoch: int, action: dict) -> dict:
        return {"epoch": epoch, "action": action}

    saved1 = initial["graph_objects"]["1"]
    saved2 = initial["graph_objects"]["2"]
    mutated2 = copy.deepcopy(saved2)
    mutated2["nodes"][0] = "Halt"
    actions: list[dict] = []
    if label == "sham":
        actions.append(at(0, {"DropGraph": {"graph": 0}}))
    if label in ("drop1", "drop-both", "restore1", "restore-both", "no-repair"):
        actions.append(at(0, {"DropGraph": {"graph": 1}}))
    if label in ("drop2", "drop-both", "restore2", "restore-both", "mutate2"):
        actions.append(at(0, {"DropGraph": {"graph": 2}}))
    if label in ("restore1", "restore-both"):
        actions.append(at(1, {"PutGraph": {"graph": 1, "object": saved1}}))
    if label in ("restore2", "restore-both"):
        actions.append(at(1, {"PutGraph": {"graph": 2, "object": saved2}}))
    if label == "mutate2":
        actions.append(at(1, {"PutGraph": {"graph": 2, "object": mutated2}}))
    for epoch in range(EPOCHS):
        actions.append(at(epoch, {
            "SetCredits": {
                "process": epoch + 3,
                "credits": 0 if label == "zero-credit" else 9,
            },
        }))
    return sorted(actions, key=lambda a: a["epoch"])


def expected(label: str) -> list[int | None]:
    return EXPECTED.get(label, [None, None, None])


def state(world: dict) -> dict:
    return {key: world[key] for key in
            ("processes", "graph_objects", "cells", "compute_pool")}


def _external_event(event: dict, operation: str) -> bool:
    return (event["origin"] == "External"
            and isinstance(event["kind"], dict)
            and operation in str(event["kind"].get("Intervened", "")))


def audit(path: Path) -> dict:
    db = sqlite3.connect(f"{path.resolve().as_uri()}?mode=ro", uri=True)
    try:
        rows = db.execute(
            "SELECT id,source_revision,condition,initial_sha256,initial_json,"
            "actions_json,epoch_count,quota,attempts_json,final_sha256,final_json "
            "FROM epoch_runs ORDER BY id"
        ).fetchall()
        if len(rows) != len(LABELS):
            raise ValueError(f"Study-21 arm count differs: {len(rows)}")
        snapshots: dict[int, list[tuple[int, str, str]]] = {}
        for run_id, epoch, digest, raw in db.execute(
            "SELECT run_id,epoch,state_sha256,state_json "
            "FROM epoch_snapshots ORDER BY run_id,epoch"
        ):
            if analyze.digest(raw) != digest:
                raise ValueError("corrupt epoch state")
            snapshots.setdefault(run_id, []).append((epoch, digest, raw))
        outcomes: dict[str, dict] = {}
        revisions: set[str] = set()
        initial_hashes: set[str] = set()
        replayed_epoch_count = 0
        for (run_id, revision, label, initial_sha, initial_raw,
             actions_raw, epochs, quota, attempts_raw, final_sha, final_raw) in rows:
            if label not in LABELS or label in outcomes:
                raise ValueError(f"unregistered or repeated label {label}")
            if not revision or epochs != EPOCHS or quota != QUOTA:
                raise ValueError("invalid registered time budget or revision")
            if analyze.digest(initial_raw) != initial_sha or analyze.digest(final_raw) != final_sha:
                raise ValueError("initial/final hash mismatch")
            revisions.add(revision)
            initial_hashes.add(initial_sha)
            initial = json.loads(initial_raw)
            final = json.loads(final_raw)
            if (initial["total_storage_limit"] != 96 or not initial["strict_accounting"]
                    or initial["storage_limit"] != 4
                    or initial["max_nodes_per_process"] != 16):
                raise ValueError("world's resource cap differs from protocol")
            for gid, owner, tag in ((1, 1, "GraphNew"), (2, 2, "Const")):
                obj = initial["graph_objects"].get(str(gid))
                if (obj is None or obj["owner"] != owner
                        or len(obj["nodes"]) != 1
                        or tag not in obj["nodes"][0]
                        or not all(reader in obj["readers"] for reader in (3, 4, 5))):
                    raise ValueError("unregistered graph opportunity")
            for idx in range(3):
                if initial["processes"][str(idx + 3)]["credits"] != 0:
                    raise ValueError("prefork probe has externally minted credit")
                if str(idx + 90) in initial["cells"]:
                    raise ValueError("prefork output contamination")
            if json.loads(actions_raw) != schedule(label, initial):
                raise ValueError(f"unregistered action order or graph data: {label}")
            attempts = json.loads(attempts_raw)
            if (len(attempts) != EPOCHS
                    or any(not isinstance(n, int) or n < 0 or n > QUOTA for n in attempts)):
                raise ValueError("invalid attempted instruction counts")
            frames = snapshots.get(run_id, [])
            if (len(frames) != EPOCHS
                    or any(frame[0] != epoch for epoch, frame in enumerate(frames))):
                raise ValueError("missing epoch snapshot")
            replayed_epoch_count += len(frames)
            if frames[-1][1:] != (final_sha, final_raw):
                raise ValueError("last snapshot != final state")
            if final["events"][:len(initial["events"])] != initial["events"]:
                raise ValueError("initial provenance prefix differs")
            prev = initial
            previous_events = len(prev["events"])
            expected_outputs = expected(label)
            for epoch, (_, _, snap_raw) in enumerate(frames):
                world = json.loads(snap_raw)
                if world["events"][:previous_events] != prev["events"]:
                    raise ValueError("events changed across epoch boundary")
                events = world["events"][previous_events:]
                key = str(90 + epoch)
                output = world["cells"].get(key)
                if output != expected_outputs[epoch]:
                    raise ValueError(f"unexpected post-lesion output: {label} W{epoch}: {output}")
                writes = [
                    event for event in events
                    if (analyze.internal_process(event) == epoch + 3
                        and isinstance(event["kind"], dict)
                        and event["kind"].get("CellWritten", {}).get("key") == 90 + epoch)
                ]
                if (len(writes) != int(output is not None)
                        or (writes and writes[0]["kind"]["CellWritten"]["value"] != output)):
                    raise ValueError(f"missing native output actor: {label} W{epoch}")
                if label in RESTORED and epoch == 0 and not any(
                    analyze.internal_process(event) == 3
                    and isinstance(event["kind"], dict)
                    and "Trapped" in event["kind"]
                    for event in events
                ):
                    raise ValueError("external restoration occurred without observed loss")
                if label == "zero-credit" and (
                    attempts[epoch] != 0
                    or any(analyze.internal_process(event) is not None for event in events)
                ):
                    raise ValueError("unfunded control unexpectedly executed")
                for other_epoch in range(epoch + 1, EPOCHS):
                    if str(90 + other_epoch) in world["cells"]:
                        raise ValueError("future output appeared ahead of corresponding probe")
                if analyze.occupied_units(world) > 96:
                    raise ValueError("strict storage cap exceeded")
                prev, previous_events = world, len(world["events"])
            all_new = final["events"][len(initial["events"]):]
            for kind, count in (
                ("PutGraph", {
                    "restore1": 1, "restore2": 1,
                    "restore-both": 2, "mutate2": 1,
                }.get(label, 0)),
                ("DropGraph", {
                    "sham": 0, "drop1": 1, "drop2": 1, "drop-both": 2,
                    "restore1": 1, "restore2": 1, "restore-both": 2,
                    "mutate2": 1, "no-repair": 1,
                }.get(label, 0)),
            ):
                if sum(_external_event(event, kind) for event in all_new) != count:
                    raise ValueError(f"external intervention provenance mismatch: {label} {kind}")
            if label == "sham" and not any(
                event["origin"] == "External"
                and isinstance(event["kind"], dict)
                and "InterventionRejected" in event["kind"]
                for event in all_new
            ):
                raise ValueError("sham was not the rejected intervention")
            outcomes[label] = final
        if set(outcomes) != set(LABELS) or len(revisions) != 1 or len(initial_hashes) != 1:
            raise ValueError("not a complete same-prefork calibration")
        if state(outcomes["control"]) != state(outcomes["sham"]):
            raise ValueError("sham does not match control")
        return {
            "study": "CL-1 Study 21",
            "source_revision": next(iter(revisions)),
            "kind": "externally designed longitudinal functional measurement apparatus",
            "independent_seed_worlds": 0,
            "matched_conditions": len(LABELS),
            "epoch_snapshots": replayed_epoch_count,
            "observed_epoch_outputs": {label: expected(label) for label in LABELS},
            "observed_failure_before_external_restoration": True,
            "subsequent_separated_external_restoration_windows": [1, 2],
            "external_structural_and_functional_positive_control": "PASSED",
            "internal_autonomous_restoration": "NOT EVALUABLE",
            "endogenous_design": "NOT EVALUABLE",
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
    print(json.dumps(report, ensure_ascii=False))


if __name__ == "__main__":
    main()
