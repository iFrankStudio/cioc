#!/usr/bin/env python3
"""Study65 prospective stochastic native feedback, independent source/epoch audit.

One frozen random genome per seed. No prewritten Read→Write sequence, no external
scheduler starts, no organism survival/repair inference from finite starting credit.
"""
from __future__ import annotations

import argparse
import json
import sqlite3
from collections import Counter, defaultdict
from pathlib import Path

import analyze
from analyze_study02 import wilson_95

FIRST, N, EPOCHS, QUOTA = 4_300_000, 256, 24, 12
A, B, MASK = 6364136223846793005, 1442695040888963407, (1 << 64) - 1
MODES = (
    "cyclic-low", "cyclic-high",
    "cyclic-ablated-low", "cyclic-ablated-high",
    "linear-low", "linear-high",
    "linear-ablated-low", "linear-ablated-high",
)


def random_state(state: int) -> int:
    return (state * A + B) & MASK


def expected_code(seed: int, mode: str) -> list:
    if not FIRST <= seed < FIRST + N or mode not in MODES:
        raise ValueError("unregistered independent seed or arm")
    ablated = "-ablated-" in mode
    cyclic = mode.startswith("cyclic")
    state = seed
    code = [
        {"Const": {"dst": 1, "value": 1, "next": 1}},
        {"Const": {"dst": 2, "value": 7, "next": 2}},
        {"Const": {"dst": 3, "value": 9, "next": 3}},
    ]
    for pc in range(3, 27):
        nxt = pc + 1
        state = random_state(state)
        choice = state % 6
        if choice == 0:
            op = ({"Const": {"dst": 0, "value": 0, "next": nxt}}
                  if ablated else
                  {"Read": {"dst": 0, "address": 2, "next": nxt}})
        elif choice == 1:
            op = {"Write": {"address": 3, "value": 0, "next": nxt}}
        elif choice == 2:
            op = {"Add": {"dst": 0, "lhs": 0, "rhs": 1, "next": nxt}}
        elif choice == 3:
            op = {"Sub": {"dst": 0, "lhs": 0, "rhs": 1, "next": nxt}}
        elif choice == 4:
            state = random_state(state)
            op = {"Const": {"dst": 0, "value": state % 8, "next": nxt}}
        else:
            op = {"Branch": {
                "test": 0, "nonzero": 3 if pc == 3 else pc - 1,
                "zero": nxt,
            }}
        code.append(op)
    code.append({"Branch": {"test": 1, "nonzero": 3, "zero": 3}}
                if cyclic else "Halt")
    if len(code) != 28:
        raise ValueError("frozen random program length changed")
    return code


def op_name(op) -> str:
    return next(iter(op)) if isinstance(op, dict) else op


def propagate_r0(op, r0: int, tainted: bool, sensor: int) -> tuple[int, bool]:
    """Independent CIL register0 def-use oracle; Const ablates inherited source."""
    name = op_name(op)
    if name == "Read":
        return sensor, True
    if name == "Const" and op["Const"]["dst"] == 0:
        return op["Const"]["value"], False
    if name == "Add" and op["Add"]["dst"] == 0:
        return r0 + 1, tainted
    if name == "Sub" and op["Sub"]["dst"] == 0:
        return r0 - 1, tainted
    return r0, tainted


def inspect_run(initial: dict, snapshots: list[tuple[int, str, str]],
                attempts: list[int], mode: str) -> dict:
    code = initial["processes"]["1"]["code"]["nodes"]
    sensor = 3 if mode.endswith("-low") else 8
    pc = 0
    halted = False
    r0, tainted, last_read_epoch = 0, False, None
    writes = []
    reads = 0
    native_epoch_counts = [0] * EPOCHS
    previous = initial
    charged = 0
    last_written = None
    for expected_epoch, (epoch, digest, raw) in enumerate(snapshots):
        if epoch != expected_epoch:
            raise ValueError("missing, duplicated or reordered original epoch")
        if analyze.digest(raw) != digest:
            raise ValueError("original native SHA256 epoch payload does not match")
        state = json.loads(raw)
        if state["events"][:len(previous["events"])] != previous["events"]:
            raise ValueError("native event history not a prefix")
        new = state["events"][len(previous["events"]):]
        if len(new) != attempts[epoch]:
            raise ValueError("native attempted CIL count differs from recorded epoch quota")
        native_epoch_counts[epoch] = len(new)
        for e in new:
            if analyze.internal_process(e) != 1 or e["process"] != 1:
                raise ValueError("external output masquerading as a native program")
            if halted or e["pc"] != pc:
                raise ValueError("impossible native PC or executing after Halt")
            op = code[pc]
            name = op_name(op)
            kind = e["kind"]
            actual = op_name(kind)
            if name == "Halt":
                if actual != "Halted":
                    raise ValueError("Halt without native halt")
                halted = True
                continue
            if name == "Read":
                if actual != "CellRead" or kind["CellRead"] != {
                    "key": 7, "value": sensor,
                }:
                    raise ValueError("read source input provenance incorrect")
                reads += 1
                last_read_epoch = epoch
            elif name == "Write":
                if actual != "CellWritten" or kind["CellWritten"]["key"] != 9:
                    raise ValueError("successful native Write did not output to registered cell")
                if kind["CellWritten"]["value"] != r0:
                    raise ValueError("native CIL Write not consistent with source/ALU provenance")
                writes.append({
                    "epoch": epoch,
                    "window": epoch // 8,
                    "pc": pc,
                    "tick": e["tick"],
                    "value": r0,
                    "source_tainted": tainted,
                    "latest_native_source_read_epoch": last_read_epoch if tainted else None,
                })
                last_written = r0
            elif actual != "Executed":
                raise ValueError("native non-I/O opcode failed or trapped")
            r0, tainted = propagate_r0(op, r0, tainted, sensor)
            if name == "Branch":
                branch = op["Branch"]
                cond = r0 if branch["test"] == 0 else 1
                pc = branch["nonzero"] if cond != 0 else branch["zero"]
            else:
                pc = op[name]["next"]
            if not 0 <= pc < len(code):
                raise ValueError("native program edge leaves frozen executable graph")
        charged += len(new)
        p = state["processes"]["1"]
        if (state["compute_pool"] != 0 or p["credits"] != 300 - charged
                or p["code"]["nodes"] != code or state["cells"]["7"] != sensor
                or p["stopped"] != halted
                or (not halted and p["pc"] != pc)
                or (last_written is not None and state["cells"].get("9") != last_written)):
            raise ValueError("native state, source code, credits or I/O replay altered")
        previous = state
    if (charged != sum(attempts) or len(snapshots) != EPOCHS):
        raise ValueError("24-epoch native attempt data incomplete")
    return {
        "native_attempts": charged,
        "native_event_counts_by_epoch": native_epoch_counts,
        "native_Read_events": reads,
        "native_shared_Write_events": len(writes),
        "source_tainted_writes": sum(w["source_tainted"] for w in writes),
        "source_tainted_writes_in_W2": sum(w["source_tainted"] and
                                           w["latest_native_source_read_epoch"] is not None and
                                           8 <= w["latest_native_source_read_epoch"] < 16 and
                                           w["window"] == 1 for w in writes),
        "source_tainted_writes_in_W3": sum(w["source_tainted"] and
                                           w["latest_native_source_read_epoch"] is not None and
                                           16 <= w["latest_native_source_read_epoch"] < 24 and
                                           w["window"] == 2 for w in writes),
        "writes": writes,
        "halted": halted,
    }


def recurrent_pair(low: dict, high: dict) -> dict:
    witnesses = []
    for window in (1, 2):
        a = [w for w in low["writes"]
             if w["window"] == window and w["source_tainted"]
             and w["latest_native_source_read_epoch"] is not None
             and w["latest_native_source_read_epoch"] // 8 == window]
        b = [w for w in high["writes"]
             if w["window"] == window and w["source_tainted"]
             and w["latest_native_source_read_epoch"] is not None
             and w["latest_native_source_read_epoch"] // 8 == window]
        match = next((
            (x, y) for x in a for y in b
            if x["pc"] == y["pc"] and y["value"] - x["value"] == 5
        ), None)
        witnesses.append({
            "window": "W2" if window == 1 else "W3",
            "paired_source_sensitive": match is not None,
            "low": match[0] if match else None,
            "high": match[1] if match else None,
        })
    return {
        "sustained_source_sensitive_2_windows": all(
            w["paired_source_sensitive"] for w in witnesses
        ),
        "paired_window_witnesses": witnesses,
    }


def audit(path: Path, revision: str) -> dict:
    db = sqlite3.connect(f"{path.resolve().as_uri()}?mode=ro", uri=True)
    try:
        run_rows = db.execute(
            "SELECT id,condition,source_revision,initial_sha256,initial_json,"
            "actions_json,epoch_count,quota,attempts_json,final_sha256,final_json "
            "FROM epoch_runs ORDER BY id"
        )
        worlds = defaultdict(dict)
        totals = Counter()
        source_witnesses = []
        hashed_epochs = 0
        num_runs = 0
        for (rid, condition, source, initial_sha, raw_initial, raw_actions,
             epochs, quota, raw_attempts, final_sha, raw_final) in run_rows:
            stem, mode = condition.split("/", 1)
            seed = int(stem.removeprefix("seed="))
            if (not FIRST <= seed < FIRST + N or mode not in MODES
                    or mode in worlds[seed] or source != revision):
                raise ValueError("unregistered population, duplicates or source revision")
            if (analyze.digest(raw_initial) != initial_sha
                    or analyze.digest(raw_final) != final_sha
                    or epochs != EPOCHS or quota != QUOTA
                    or json.loads(raw_actions) != []):
                raise ValueError("original source SHA, actions or quota invalid")
            initial = json.loads(raw_initial)
            if (set(initial["processes"]) != {"1"}
                    or initial["processes"]["1"]["code"]["nodes"] != expected_code(seed, mode)
                    or initial["processes"]["1"]["credits"] != 300
                    or initial["cells"] != {
                        "7": 3 if mode.endswith("-low") else 8,
                    }
                    or initial["compute_pool"] != 0
                    or initial["storage_limit"] != 4
                    or initial["max_nodes_per_process"] != 32
                    or initial["total_storage_limit"] != 64
                    or not initial["strict_accounting"]):
                raise ValueError("random CIL grammar or frozen initial world differs")
            if (len(initial["events"]) != 2
                    or op_name(initial["events"][0]["kind"]) != "Loaded"
                    or op_name(initial["events"][1]["kind"]) != "Intervened"
                    or any(e["origin"] != "External" for e in initial["events"])):
                raise ValueError("initial human-set input not explicitly external")
            attempts = json.loads(raw_attempts)
            if len(attempts) != EPOCHS or any(not 0 <= x <= QUOTA for x in attempts):
                raise ValueError("invalid native epoch quota")
            snapshots = db.execute(
                "SELECT epoch,state_sha256,state_json FROM epoch_snapshots "
                "WHERE run_id=? ORDER BY epoch", (rid,)
            ).fetchall()
            if len(snapshots) != EPOCHS or [row[0] for row in snapshots] != list(range(EPOCHS)):
                raise ValueError("missing native original epoch")
            data = inspect_run(initial, snapshots, attempts, mode)
            if json.loads(snapshots[-1][2]) != json.loads(raw_final):
                raise ValueError("native final state differs from original 24th epoch")
            worlds[seed][mode] = data
            hashed_epochs += EPOCHS
            num_runs += 1
            totals[f"{mode}.worlds_any_native_W3_attempt"] += int(any(
                data["native_event_counts_by_epoch"][16:]
            ))
            totals[f"{mode}.worlds_tainted_W2_write"] += int(
                data["source_tainted_writes_in_W2"] > 0)
            totals[f"{mode}.worlds_tainted_W3_write"] += int(
                data["source_tainted_writes_in_W3"] > 0)
            totals[f"{mode}.native_Read_events"] += data["native_Read_events"]
            totals[f"{mode}.native_Write_events"] += data["native_shared_Write_events"]
            totals[f"{mode}.native_tainted_Write_events"] += data["source_tainted_writes"]
        if (num_runs != N * len(MODES)
                or hashed_epochs != N * len(MODES) * EPOCHS
                or len(worlds) != N
                or any(set(arm) != set(MODES) for arm in worlds.values())):
            raise ValueError("incomplete, unbalanced or duplicated Study65 population")
        for seed, modes in worlds.items():
            for topology in ("cyclic", "linear"):
                low = modes[f"{topology}-low"]
                high = modes[f"{topology}-high"]
                ablated_low = modes[f"{topology}-ablated-low"]
                ablated_high = modes[f"{topology}-ablated-high"]
                if (ablated_low["native_Read_events"] != 0
                        or ablated_high["native_Read_events"] != 0):
                    raise ValueError("negative control performed an undeclared Read")
                sig = lambda d: [
                    (w["tick"], w["pc"], w["value"]) for w in d["writes"]
                ]
                if sig(ablated_low) != sig(ablated_high):
                    raise ValueError("read-ablated outputs depend on source input")
                result = recurrent_pair(low, high)
                totals[f"{topology}.recurrent_paired_source_sensitive_worlds"] += int(
                    result["sustained_source_sensitive_2_windows"])
                if result["sustained_source_sensitive_2_windows"] and len(source_witnesses) < 20:
                    source_witnesses.append({
                        "seed": seed,
                        "topology": topology,
                        **result,
                    })
        pair = Counter()
        for seed, modes in worlds.items():
            c = recurrent_pair(modes["cyclic-low"], modes["cyclic-high"])[
                "sustained_source_sensitive_2_windows"
            ]
            l = recurrent_pair(modes["linear-low"], modes["linear-high"])[
                "sustained_source_sensitive_2_windows"
            ]
            pair["cyclic_only"] += int(c and not l)
            pair["linear_only"] += int(l and not c)
            pair["both"] += int(c and l)
            pair["neither"] += int(not c and not l)
        n_c = totals["cyclic.recurrent_paired_source_sensitive_worlds"]
        n_l = totals["linear.recurrent_paired_source_sensitive_worlds"]
        if n_c != pair["cyclic_only"] + pair["both"] or n_l != pair["linear_only"] + pair["both"]:
            raise ValueError("original paired seed response totals inconsistent")
        return {
            "study": "CL-1 Study65 preregistered stochastic CIL source-feedback opportunity",
            "source_revision": revision,
            "original_independent_genomes": N,
            "matched_native_runs": num_runs,
            "SHA256_verified_epoch_snapshots": hashed_epochs,
            "original_external_compute_endowment_per_run": 300,
            "native_compute_acquisition": "NOT TESTED IN THIS ISOLATED FEEDBACK ECOLOGY",
            "registered_primary_recurrent_source_sensitive_W2_and_W3": {
                "cyclic_worlds": n_c,
                "linear_worlds": n_l,
                "denominator_each": N,
                "cyclic_wilson95": [round(v, 6) for v in wilson_95(n_c, N)],
                "linear_wilson95": [round(v, 6) for v in wilson_95(n_l, N)],
                "paired_discordance": dict(sorted(pair.items())),
                "registered_directional_topology_criterion": pair["cyclic_only"] > pair["linear_only"],
            },
            "registered_secondary": dict(sorted(totals.items())),
            "first_native_source_feedback_witnesses": source_witnesses,
            "organization_endogenous": "NOT SHOWN; generic I/O interface and cycle topology are researcher-supplied",
            "life_claim": "NONE",
        }
    finally:
        db.close()


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("sqlite", type=Path)
    parser.add_argument("--expected-revision", required=True)
    parser.add_argument("--json", type=Path, required=True)
    args = parser.parse_args()
    report = audit(args.sqlite, args.expected_revision)
    args.json.write_text(json.dumps(report, indent=2) + "\n", encoding="utf-8")
    print(json.dumps({
        "study": report["study"],
        "independent_random_genomes": report["original_independent_genomes"],
        "matched_runs": report["matched_native_runs"],
        "hashed_epochs": report["SHA256_verified_epoch_snapshots"],
        "primary": report["registered_primary_recurrent_source_sensitive_W2_and_W3"],
        "life_claim": report["life_claim"],
    }, indent=2))


if __name__ == "__main__":
    main()
