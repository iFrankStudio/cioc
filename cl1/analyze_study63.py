#!/usr/bin/env python3
"""Study63 independent audit: six HUMAN-SCRIPTED feedback/energy mechanisms.

No stochastic worlds, no endogenous organization or life claim.
"""
from __future__ import annotations

import argparse
import json
import sqlite3
from collections import defaultdict
from pathlib import Path

import analyze

MODES = (
    "sensor-low-native-pool",
    "sensor-high-native-pool",
    "sensor-switch-native-pool",
    "sensor-high-native-pool-read-ablated",
    "sensor-high-empty-pool",
    "sensor-high-zero-process-credit",
)
EPOCHS, QUOTA = 8, 12


def expected_program(mode: str) -> list:
    read = ({"Const": {"dst": 2, "value": 0, "next": 4}}
            if mode == "sensor-high-native-pool-read-ablated"
            else {"Read": {"dst": 2, "address": 0, "next": 4}})
    return [
        {"Const": {"dst": 0, "value": 7, "next": 1}},
        {"Const": {"dst": 1, "value": 8, "next": 2}},
        {"DrawCompute": {"amount": 1, "next": 3}},
        read,
        {"Const": {"dst": 0, "value": 9, "next": 5}},
        {"Add": {"dst": 3, "lhs": 2, "rhs": 1, "next": 6}},
        {"Write": {"address": 0, "value": 3, "next": 7}},
        {"Const": {"dst": 0, "value": 7, "next": 2}},
    ]


def event_kind(e: dict) -> str:
    kind = e["kind"]
    return next(iter(kind)) if isinstance(kind, dict) else kind


def audit(path: Path, source_revision: str) -> dict:
    db = sqlite3.connect(f"{path.resolve().as_uri()}?mode=ro", uri=True)
    try:
        rows = db.execute(
            "SELECT id,condition,source_revision,initial_sha256,initial_json,"
            "actions_json,epoch_count,quota,attempts_json,final_sha256,final_json "
            "FROM epoch_runs ORDER BY id"
        ).fetchall()
        if len(rows) != len(MODES):
            raise ValueError("Study63 did not execute exactly six scripted fixtures")
        report = {}
        hash_count = 0
        for (rid, mode, revision, initial_sha, raw_initial, raw_actions,
             n_epochs, quota, raw_attempts, final_sha, raw_final) in rows:
            if mode not in MODES or mode in report or revision != source_revision:
                raise ValueError("unregistered scripted mode or wrong source revision")
            if (analyze.digest(raw_initial) != initial_sha
                    or analyze.digest(raw_final) != final_sha
                    or n_epochs != EPOCHS or quota != QUOTA):
                raise ValueError("source-pinned Study63 original run corrupted")
            initial, final = json.loads(raw_initial), json.loads(raw_final)
            initial_credits = 0 if mode == "sensor-high-zero-process-credit" else 3
            initial_pool = 0 if mode == "sensor-high-empty-pool" else 200
            initial_value = (
                3 if mode in ("sensor-low-native-pool",
                             "sensor-switch-native-pool") else 8
            )
            if (set(initial["processes"]) != {"1"}
                    or initial["processes"]["1"]["code"]["nodes"] != expected_program(mode)
                    or initial["processes"]["1"]["credits"] != initial_credits
                    or initial["cells"] != {"7": initial_value}
                    or initial["compute_pool"] != initial_pool
                    or initial["storage_limit"] != 16
                    or initial["max_nodes_per_process"] != 12
                    or initial["total_storage_limit"] != 40
                    or not initial["strict_accounting"]):
                raise ValueError("scripted world differs from registered instructions/caps")
            initial_events = initial["events"]
            if (len(initial_events) != 3
                    or initial_events[0]["origin"] != "External"
                    or event_kind(initial_events[0]) != "Loaded"
                    or any(
                        e["origin"] != "External" or event_kind(e) != "Intervened"
                        for e in initial_events[1:]
                    )):
                raise ValueError("original load/sensor/compute were not explicitly External")
            actions = json.loads(raw_actions)
            expected_actions = ([{
                "epoch": 4,
                "action": {"SetCell": {"key": 7, "value": 8}},
            }] if mode == "sensor-switch-native-pool" else [])
            if actions != expected_actions:
                raise ValueError("unregistered host intervention in native feedback trial")
            attempts = json.loads(raw_attempts)
            if len(attempts) != EPOCHS or any(x > QUOTA or x < 0 for x in attempts):
                raise ValueError("native attempt quota violated")
            sensor_value = initial_value
            last_read = None
            draws = 0
            writes = []
            reads = []
            traps = []
            previous = initial
            for epoch, snapshot_sha, raw_snapshot in db.execute(
                "SELECT epoch,state_sha256,state_json FROM epoch_snapshots "
                "WHERE run_id=? ORDER BY epoch", (rid,)
            ):
                if epoch != hash_count % EPOCHS or analyze.digest(raw_snapshot) != snapshot_sha:
                    raise ValueError("Study63 snapshot SHA or epoch order wrong")
                state = json.loads(raw_snapshot)
                if state["events"][:len(previous["events"])] != previous["events"]:
                    raise ValueError("native event history nonmonotone")
                new = state["events"][len(previous["events"]):]
                scheduled = [a for a in actions if a["epoch"] == epoch]
                if len(new) != len(scheduled) + attempts[epoch]:
                    raise ValueError("native/external event counts disagree")
                for host_event in new[:len(scheduled)]:
                    if host_event["origin"] != "External":
                        raise ValueError("sensor stimulus falsely native")
                for action in scheduled:
                    sensor_value = action["action"]["SetCell"]["value"]
                native = new[len(scheduled):]
                if any(analyze.internal_process(e) != 1 or e["process"] != 1
                       or e["pc"] not in range(8) for e in native):
                    raise ValueError("native Read/Write/Draw actor or PC invalid")
                for e in native:
                    kind = event_kind(e)
                    pc = e["pc"]
                    if pc == 2 and kind == "Executed":
                        draws += 1
                    if kind == "CellRead":
                        if pc != 3 or e["kind"]["CellRead"] != {
                            "key": 7, "value": sensor_value,
                        }:
                            raise ValueError("not a true native environment sensor read")
                        last_read = (e["kind"]["CellRead"]["value"], epoch, e["tick"])
                        reads.append({"epoch": epoch, "value": last_read[0], "tick": e["tick"]})
                    if kind == "CellWritten":
                        d = e["kind"]["CellWritten"]
                        if pc != 6 or d["key"] != 9:
                            raise ValueError("successful native output not from frozen Write")
                        if mode == "sensor-high-native-pool-read-ablated":
                            if d["value"] != 8:
                                raise ValueError("ablated sensor still appears informative")
                        elif last_read is None or d["value"] != last_read[0] + 8:
                            raise ValueError("output lacks causal native Read→Add provenance")
                        writes.append({
                            "epoch": epoch, "value": d["value"], "tick": e["tick"],
                            "source_read": last_read,
                        })
                    if kind == "Trapped":
                        traps.append(e["kind"]["Trapped"])
                charged = sum(attempts[:epoch + 1])
                expected_pool = initial_pool - 8 * draws
                expected_credits = initial_credits + 8 * draws - charged
                if (state["compute_pool"] != expected_pool
                        or state["processes"]["1"]["credits"] != expected_credits
                        or state["compute_pool"] + expected_credits
                        != initial_pool + initial_credits - charged
                        or state["cells"]["7"] != sensor_value):
                    raise ValueError("charged CIL compute or researcher sensor provenance inconsistent")
                previous = state
                hash_count += 1
            if hash_count % EPOCHS != 0 or previous != final:
                raise ValueError("Study63 incomplete exact native epoch state replay")
            write_epochs = defaultdict(set)
            for event in writes:
                write_epochs[event["value"]].add(event["epoch"])
            if mode == "sensor-low-native-pool":
                passed = len(write_epochs[11]) >= 3 and len(writes) > 0 and draws > 0
                passed &= set(write_epochs) == {11}
            elif mode == "sensor-high-native-pool":
                passed = len(write_epochs[16]) >= 3 and draws > 0 and set(write_epochs) == {16}
            elif mode == "sensor-switch-native-pool":
                passed = (any(w["value"] == 11 and w["epoch"] < 4 for w in writes)
                          and any(w["value"] == 16 and w["epoch"] >= 4 for w in writes)
                          and draws > 0 and set(write_epochs) == {11, 16})
            elif mode == "sensor-high-native-pool-read-ablated":
                passed = not reads and set(write_epochs) == {8} and draws > 0
            elif mode == "sensor-high-empty-pool":
                passed = not writes and draws == 0 and any(
                    "insufficient world compute pool" in error for error in traps
                )
            else:
                passed = not writes and not reads and draws == 0 and sum(attempts) == 0
            if not passed:
                raise ValueError(f"registered feedback positive/negative gate FAILED: {mode}")
            report[mode] = {
                "scripted_researcher_construct": True,
                "native_attempts": sum(attempts),
                "successful_native_DrawCompute": draws,
                "native_sensor_reads": len(reads),
                "native_output_events": len(writes),
                "native_output_values": sorted(write_epochs),
                "distinct_native_output_epochs_by_value": {
                    str(value): sorted(epochs) for value, epochs in write_epochs.items()
                },
                "native_traps": traps,
                "native_output_first_witness": writes[:3],
                "registered_gate_pass": True,
            }
        if set(report) != set(MODES) or hash_count != len(MODES)*EPOCHS:
            raise ValueError("missing source-pinned Study63 scripted mode or hash")
        return {
            "study": "CL-1 Study63 human-scripted feedback and native compute feasibility",
            "source_revision": source_revision,
            "stochastic_independent_worlds": 0,
            "human_authored_fixtures": len(MODES),
            "original_SHA256_snapshots": hash_count,
            "scripted_mechanism_gates_pass": True,
            "fixtures": report,
            "endogenous_organization": "NOT TESTED",
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
    outcome = audit(args.sqlite, args.expected_revision)
    args.json.write_text(json.dumps(outcome, indent=2) + "\n", encoding="utf-8")
    print(json.dumps(outcome, indent=2))


if __name__ == "__main__":
    main()
