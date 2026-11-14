#!/usr/bin/env python3
"""Study47 independent auditor: THREE scripted VM fixtures, ZERO ecology worlds."""
from __future__ import annotations

import argparse
import json
import sqlite3
from pathlib import Path

import analyze

MODES = ("native-local-edge", "copied-stale-edge", "zero-child-credit")
EPOCHS = 8
QUOTA = 8


def expected_nodes(mode: str) -> list:
    link = (
        {"GraphSetNext": {"graph": 3, "node": 2, "target": 0, "next": 4}}
        if mode != "copied-stale-edge"
        else {"Const": {"dst": 2, "value": 0, "next": 4}}
    )
    return [
        {"GraphNew": {"dst": 3, "next": 1}},
        {"Const": {"dst": 2, "value": 6, "next": 2}},
        {"GraphCopyNode": {"graph": 3, "source": 2, "dst": 2, "next": 3}},
        link,
        {"Const": {"dst": 1, "value": 0 if mode == "zero-child-credit" else 3,
                   "next": 5}},
        {"GraphLaunch": {"graph": 3, "credits": 1, "dst": 2, "next": 8}},
        {"Write": {"address": 0, "value": 3, "next": 7}},
        "Halt", "Halt",
    ]


def edge_valid(code: list) -> bool:
    n = len(code)
    for op in code:
        if op == "Halt":
            continue
        if not isinstance(op, dict) or len(op) != 1:
            return False
        name, fields = next(iter(op.items()))
        targets = ("nonzero", "zero") if name == "Branch" else ("next",)
        if any(not 0 <= fields[k] < n for k in targets):
            return False
    return True


def audit(path: Path, expected_revision: str) -> dict:
    db = sqlite3.connect(f"{path.resolve().as_uri()}?mode=ro", uri=True)
    try:
        rows = db.execute(
            "SELECT id,condition,source_revision,initial_sha256,initial_json,"
            "actions_json,attempts_json,final_sha256,final_json FROM epoch_runs"
        ).fetchall()
        if len(rows) != 3:
            raise ValueError("scripted fixture population not exactly three")
        results = {}
        hashes = 0
        for (rid, mode, revision, start_sha, raw, actions, attempts_raw,
             end_sha, end_raw) in rows:
            if mode not in MODES or mode in results or revision != expected_revision:
                raise ValueError("unregistered scripted mode or source SHA")
            if analyze.digest(raw) != start_sha or analyze.digest(end_raw) != end_sha:
                raise ValueError("scripted fixture SHA mismatch")
            if json.loads(actions) != []:
                raise ValueError("unexpected host intervention in native scripted run")
            start, end = json.loads(raw), json.loads(end_raw)
            if (set(start["processes"]) != {"1"}
                    or start["processes"]["1"]["code"]["nodes"] != expected_nodes(mode)
                    or start["processes"]["1"]["credits"] != 30
                    or start["compute_pool"] != 0):
                raise ValueError("changed human-authored mechanism fixture")
            attempts = json.loads(attempts_raw)
            if len(attempts) != EPOCHS:
                raise ValueError("wrong epoch ledger")
            prev_events = start["events"]
            observed = 0
            final_snap = None
            for epoch, digest, snap_raw in db.execute(
                "SELECT epoch,state_sha256,state_json FROM epoch_snapshots "
                "WHERE run_id=? ORDER BY epoch", (rid,)
            ):
                if epoch != observed or analyze.digest(snap_raw) != digest:
                    raise ValueError("missing or corrupted native mechanism epoch")
                state = json.loads(snap_raw)
                if state["events"][:len(prev_events)] != prev_events:
                    raise ValueError("event chronology mutation")
                new = state["events"][len(prev_events):]
                if len(new) != attempts[epoch] or any(
                    analyze.internal_process(e) != e["process"] for e in new
                ):
                    raise ValueError("native actor identity or attempt ledger corrupted")
                if state["compute_pool"] + sum(
                    p["credits"] for p in state["processes"].values()
                ) != 30 - sum(attempts[:epoch + 1]):
                    raise ValueError("native compute conservation violated")
                prev_events = state["events"]
                observed += 1
                hashes += 1
                final_snap = state
            if observed != EPOCHS or final_snap != end:
                raise ValueError("incomplete scripted program replay")
            launches = [
                e for e in end["events"]
                if analyze.internal_process(e) == 1 and e["process"] == 1
                and isinstance(e["kind"], dict) and "GraphLaunched" in e["kind"]
            ]
            if mode != "zero-child-credit" and len(launches) != 1:
                raise ValueError("no native process-created child")
            if any(
                e["kind"]["GraphLaunched"]["child_process"] != 2
                or e["kind"]["GraphLaunched"]["parent_process"] != 1
                or e["kind"]["GraphLaunched"]["source_graph"] != 1
                for e in launches
            ):
                raise ValueError("wrong source graph or child lineage")
            children = end["processes"].get("2")
            child_writes = [
                e for e in end["events"]
                if e["process"] == 2 and analyze.internal_process(e) == 2
                and isinstance(e["kind"], dict) and "CellWritten" in e["kind"]
            ]
            invalid_traps = [
                e for e in end["events"]
                if e["process"] == 2 and analyze.internal_process(e) == 2
                and isinstance(e["kind"], dict) and "Trapped" in e["kind"]
                and "invalid edge" in e["kind"]["Trapped"]
            ]
            if mode == "native-local-edge":
                if (children is None
                        or children["code"]["nodes"] != [
                            {"Write": {"address": 0, "value": 3, "next": 0}}
                        ]
                        or not edge_valid(children["code"]["nodes"])
                        or not child_writes
                        or any(e["kind"]["CellWritten"] != {"key": 0, "value": 0}
                               for e in child_writes)):
                    raise ValueError("native edge closure did NOT produce child Write")
            elif mode == "copied-stale-edge":
                if (children is None
                        or edge_valid(children["code"]["nodes"])
                        or not invalid_traps or child_writes):
                    raise ValueError("stale copied edge did not trap child")
            elif child_writes:
                raise ValueError("zero-credit child unexpectedly performed Write")
            results[mode] = {
                "native_launches": len(launches),
                "child_code": children["code"]["nodes"] if children else None,
                "child_native_shared_writes": len(child_writes),
                "child_invalid_edge_traps": len(invalid_traps),
                "source_of_builder": "HUMAN-AUTHORED INITIAL CIL",
            }
        if set(results) != set(MODES) or hashes != 3 * EPOCHS:
            raise ValueError("scripted fixture missing from independent audit")
        return {
            "study": "CL-1 Study47 scripted native graph child-edge calibration",
            "source_revision": expected_revision,
            "independent_stochastic_worlds": 0,
            "human_authored_calibrations": 3,
            "hashed_epoch_snapshots": hashes,
            "mechanism_works": True,
            "fixtures": results,
            "life_claim": "NONE",
        }
    finally:
        db.close()


def main() -> None:
    p = argparse.ArgumentParser(description=__doc__)
    p.add_argument("sqlite", type=Path)
    p.add_argument("--expected-revision", required=True)
    p.add_argument("--json", type=Path, required=True)
    args = p.parse_args()
    report = audit(args.sqlite, args.expected_revision)
    args.json.write_text(json.dumps(report, indent=2) + "\n", encoding="utf-8")
    print(json.dumps(report, indent=2))


if __name__ == "__main__":
    main()
