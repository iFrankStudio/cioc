#!/usr/bin/env python3
"""Study87: retrospective original Study86 E11/L0 sham and lesion exposure audit.

Zero new genomes, no CIL simulation; full Study86 hashes and sources audited first.
"""
import argparse
import json
import sqlite3
from collections import Counter
from pathlib import Path

import analyze_study86 as original

ORIGINAL_REV = "f162b70e06296f14a22288a69b9ecf6d8fd58fd8"
# The original Study86 CI was source-pinned to its actual original runner source
# revision, which precedes the subsequent metadata-only auditor label fix.
ORIGINAL_RUN_REV = "bfce0210c2e5aeabbdfe5165bcd7fdbe286cb1eb"
ARMS = (
    "intact",
    "sham",
    "lesion-native",
    "lesion-edit-disabled",
    "lesion-host-restore",
    "lesion-draw-ablated",
)


def require(condition, message):
    if not condition:
        raise ValueError(message)


def load_run(database, seed, source, arm):
    label = f"seed={seed}/source={source}/{arm}"
    rows = database.execute(
        "SELECT initial_json,attempts_json,final_json,id FROM epoch_runs WHERE condition=?",
        (label,),
    ).fetchall()
    require(len(rows) == 1, "wrong source/arm/run count")
    before, attempts, after, run_id = rows[0]
    initial = json.loads(before)
    final = json.loads(after)
    code = original.original.expected_code(seed)
    run = {
        "initial": initial,
        "final": final,
        "pre_attempts": sum(
            e["origin"] == {"Internal": 1} for e in initial["events"]
        ),
        "charges": json.loads(attempts),
        "trace": original.trace_native(final, code, source),
    }
    pre_events = len(initial["events"])
    earlier = initial["events"]
    early_snapshots = database.execute(
        "SELECT epoch,state_json FROM epoch_snapshots WHERE run_id=? AND epoch<=3 ORDER BY epoch",
        (run_id,),
    ).fetchall()
    require(len(early_snapshots) == 4, "missing four first postfork epoch records")
    native = []
    for epoch, raw in early_snapshots:
        state = json.loads(raw)
        new = state["events"][len(earlier):]
        native.extend(e for e in new if e["origin"] == {"Internal": 1})
        earlier = state["events"]
    run["native_early_pc8"] = sum(e["pc"] == 8 for e in native)
    run["native_early_source_reads"] = sum(
        isinstance(e["kind"], dict) and "CellRead" in e["kind"]
        and e["kind"]["CellRead"]["key"] == 7 for e in native
    )
    run["native_early_source_writes"] = sum(
        isinstance(e["kind"], dict) and "CellWritten" in e["kind"]
        and e["kind"]["CellWritten"]["key"] == 9 for e in native
    )
    state3 = json.loads(early_snapshots[-1][1])
    run["epoch3_actor"] = {
        "pc": state3["processes"]["1"]["pc"],
        "stopped": state3["processes"]["1"]["stopped"],
        "credits": state3["processes"]["1"]["credits"],
        "pool": state3["compute_pool"],
    }
    run["node8_opcode"] = original.original.variant(code[8])[0]
    run["checkpoint_edge8"] = original.executable_edge(initial["processes"]["1"]["code"]["nodes"])
    return run


def audit(path):
    # Read all original worlds, complete 27,648 epochs, Rust-source-pinned
    # native opcode and resource/event ancestry before taking an exposure subset.
    base = original.auditor(path, ORIGINAL_RUN_REV)
    require(base["original_distinct_genotypes"] == 128
            and base["original_matched_worlds"] == 1536
            and base["verified_original_postfork_epoch_sha256"] == 27648
            and base["pre_fork_exposed_E"] == 11
            and base["effective_lesion_L"] == 0
            and base["strict_native_repair_R"] == 0,
            "fixed original Study86 source-exposure result changed")
    seeds = [x["seed"] for x in base["seed_gate_outcomes"] if x["eligible_E"]]
    require(len(seeds) == 11 and len(set(seeds)) == 11, "invalid original source exposure subset")
    db = sqlite3.connect(path)
    reason_counts = Counter()
    exposed_detail = []
    for seed in seeds:
        arms = {
            (source, arm): load_run(db, seed, source, arm)
            for source in (3, 8) for arm in ARMS
        }
        wlo = original.pre_exposure(
            arms[(3, "intact")]["initial"],
            original.original.expected_code(seed), 3
        )
        whi = original.pre_exposure(
            arms[(8, "intact")]["initial"],
            original.original.expected_code(seed), 8
        )
        witnesses = wlo & whi
        require(witnesses, "invalid original eligible pretreatment source witness")
        def window(source, arm, first, last):
            return original.post_window_writes(arms[(source, arm)], first, last)
        early = {
            f"{arm}_low": len(window(3, arm, 0, 3) & witnesses)
            for arm in ARMS
        }
        early.update({
            f"{arm}_high": len(window(8, arm, 0, 3) & witnesses)
            for arm in ARMS
        })
        sham = bool(window(3, "sham", 0, 3)
                    & window(8, "sham", 0, 3) & witnesses)
        lesion_low = bool(window(3, "lesion-native", 0, 3) & witnesses)
        lesion_high = bool(window(8, "lesion-native", 0, 3) & witnesses)
        if not sham:
            classification = "sham_missing_original_early_two_source_function"
        elif not lesion_low and not lesion_high:
            classification = "genuine_two_source_lesion_loss"
        elif not lesion_low or not lesion_high:
            classification = "partial_one_source_lesion_loss"
        else:
            classification = "lesion_no_original_early_function_loss"
        reason_counts[classification] += 1
        exposed_detail.append({
            "seed": seed,
            "classification": classification,
            "original_source_writer_pc_offset": sorted(
                [{"pc": pc, "offset": offset} for pc, offset in witnesses],
                key=lambda p: (p["pc"], p["offset"]),
            ),
            "early_matching_source_function_counts": early,
            "native_first4_epoch_pc8_counts": {
                f"{arm}/{source}": arms[(source, arm)]["native_early_pc8"]
                for source in (3, 8) for arm in ARMS
            },
            "early_original_native_source_reads": {
                f"{arm}/{source}": arms[(source, arm)]["native_early_source_reads"]
                for source in (3, 8) for arm in ARMS
            },
            "early_original_native_source_writes": {
                f"{arm}/{source}": arms[(source, arm)]["native_early_source_writes"]
                for source in (3, 8) for arm in ARMS
            },
            "source3_pc8_opcode": arms[(3, "intact")]["node8_opcode"],
            "source8_pc8_opcode": arms[(8, "intact")]["node8_opcode"],
            "initial_pc8_edge": {
                str(source): arms[(source, "intact")]["checkpoint_edge8"]
                for source in (3, 8)
            },
            "epoch3_actor_state": {
                f"{arm}/{source}": arms[(source, arm)]["epoch3_actor"]
                for source in (3, 8) for arm in ARMS
            },
        })
    require(sum(reason_counts.values()) == 11,
            "not all pretreatment exposed seeds classified")
    return {
        "study": 87,
        "status": "RETROSPECTIVE_STUDY86_E11_SHAM_AND_LESION_ANNOTATION",
        "original_study86_source_revision": ORIGINAL_RUN_REV,
        "original_genotypes_reaudited": 128,
        "original_runs_reaudited": 1536,
        "original_epoch_sha256_reaudited": 27648,
        "new_genotypes": 0,
        "pretreatment_E_count": 11,
        "original_effective_L_count": 0,
        "reasons": dict(reason_counts),
        "exposed_seed_details": exposed_detail,
        "cl1_closed": False,
    }


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("sqlite", type=Path)
    parser.add_argument("--json", type=Path, required=True)
    args = parser.parse_args()
    result = audit(args.sqlite)
    args.json.write_text(json.dumps(result, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    print(json.dumps({
        key: result[key] for key in (
            "study", "status", "pretreatment_E_count",
            "original_effective_L_count", "reasons",
            "original_runs_reaudited", "original_epoch_sha256_reaudited",
        )
    }, sort_keys=True))


if __name__ == "__main__":
    main()
