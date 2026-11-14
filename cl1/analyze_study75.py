#!/usr/bin/env python3
"""Study75 read-only resource and functional opportunity audit of original Study74-P.

Post-result historical classification ONLY; zero new independent worlds.
Reruns the complete immutable prospective Study74-P auditor before any diagnosis.
"""
from __future__ import annotations
import argparse
import json
import sqlite3
from collections import Counter, defaultdict
from pathlib import Path

import analyze
import analyze_study74_poolpaid as prior

SOURCE = "2d97f1ed60bdb1d638cd8bfacfacb5e7dde68119"
BOUNDARIES = {7: "W2_entry", 15: "W3_entry", 23: "final"}
N = 128
EPOCHS = 24


def opname(node) -> str:
    return next(iter(node)) if isinstance(node, dict) else node


def stopped_by_native_pool_trap(state: dict, actor: int = 1) -> bool:
    """Only last successful actor transition from genuine native process PC."""
    for e in reversed(state["events"]):
        if e["process"] != actor or analyze.internal_process(e) != actor:
            continue
        if opname(e["kind"]) != "Trapped":
            return False
        nodes = state["processes"][str(actor)]["code"]["nodes"]
        pc = e["pc"]
        if not isinstance(pc, int) or not 0 <= pc < len(nodes):
            raise ValueError("native trap original PC out of range")
        return (opname(nodes[pc]) == "DrawCompute"
                and "insufficient world compute pool" in e["kind"]["Trapped"])
    return False


def classify(state: dict) -> str:
    process = state["processes"]["1"]
    code = process["code"]["nodes"]
    pc = process["pc"]
    if not isinstance(pc, int) or not 0 <= pc < len(code):
        raise ValueError("invalid native actor PC")
    credits = process["credits"]
    pool = state["compute_pool"]
    if pool < 0 or credits < 0:
        raise ValueError("negative original actor or common pool")
    if process["stopped"]:
        return ("stopped-after-pool-Draw-trap"
                if stopped_by_native_pool_trap(state)
                else "other-stopped")
    if credits > 0:
        return "live"
    if pool == 0:
        return "zero/local-empty-pool"
    if opname(code[pc]) != "DrawCompute":
        return "zero/local-nonDraw"
    return ("zero/local-pool-paid-eligible" if pool >= 9
            else "zero/local-Draw-uncapitalized")


def functional_flags(audited: dict) -> dict:
    flags = {
        "native_Draw_any": bool(audited["draws"]),
        "native_Draw_W2": any(x["window"] == 1 for x in audited["draws"]),
        "native_Draw_W3": any(x["window"] == 2 for x in audited["draws"]),
        "native_Read_any": audited["native_source_Read_events"] > 0,
        "native_pool_paid_attempt": bool(audited["pool_paid_events"]),
        "native_W3_activity": audited["native_attempts_by_window"][2] > 0,
    }
    for w in range(3):
        flags[f"native_tainted_Write_W{w+1}"] = any(
            x["window"] == w and x["source_tainted"]
            and x["latest_native_source_read_epoch"] is not None
            and x["latest_native_source_read_epoch"]//8 == w
            for x in audited["writes"]
        )
    flags["native_tainted_Write_W3_and_Draw_W3"] = (
        flags["native_tainted_Write_W3"] and flags["native_Draw_W3"]
    )
    flags["native_tainted_Write_W2W3_and_Draw_W3"] = (
        flags["native_tainted_Write_W2"] and flags["native_tainted_Write_W3"]
        and flags["native_Draw_W3"]
    )
    return flags


def audit(path: Path) -> dict:
    original = prior.audit(path, SOURCE)
    if (original["source_revision"] != SOURCE
        or original["original_independent_seed_genomes"] != N
        or original["matched_native_runs"] != N*len(prior.MODES)
        or original["SHA256_verified_original_snapshots"] != N*len(prior.MODES)*EPOCHS
        or original["primary_A_W2W3_native_feedback_with_W3_Draw"]["broad"]["on_worlds"] != 3):
        raise ValueError("original primary and 49152 hashes did not reproduce")
    counts=Counter()
    per_seed=defaultdict(dict)
    hash_count=0
    db=sqlite3.connect(f"{path.resolve().as_uri()}?mode=ro",uri=True)
    try:
        for (rid,condition,revision,raw_initial,raw_actions,raw_attempts) in db.execute(
            "SELECT id,condition,source_revision,initial_json,actions_json,attempts_json "
            "FROM epoch_runs ORDER BY id"
        ):
            seedstr,mode=condition.split("/",1)
            seed=int(seedstr.removeprefix("seed="))
            if (revision!=SOURCE or seed not in range(5_000_000,5_000_000+N)
                    or mode not in prior.MODES or mode in per_seed[seed]):
                raise ValueError("unexpected original source/condition")
            initial=json.loads(raw_initial)
            actions=json.loads(raw_actions)
            attempts=json.loads(raw_attempts)
            snapshots=db.execute(
                "SELECT epoch,state_sha256,state_json FROM epoch_snapshots "
                "WHERE run_id=? ORDER BY epoch",(rid,)
            ).fetchall()
            if len(snapshots)!=EPOCHS:
                raise ValueError("historical epoch archive truncated")
            original_trace=prior.trace_run(initial,snapshots,actions,attempts,mode)
            flags=functional_flags(original_trace)
            for name,hit in flags.items():
                counts[f"{mode}.{name}"]+=int(hit)
            eligible_epoch_seen=False
            labels={}
            for epoch,sha,raw in snapshots:
                if epoch not in range(EPOCHS) or analyze.digest(raw)!=sha:
                    raise ValueError("retrospective full SHA256 epoch trace failed")
                state=json.loads(raw)
                label=classify(state)
                if label=="zero/local-pool-paid-eligible":
                    eligible_epoch_seen=True
                if epoch in BOUNDARIES:
                    labels[BOUNDARIES[epoch]]=label
                    counts[f"{mode}.{BOUNDARIES[epoch]}.{label}"]+=1
                hash_count+=1
            counts[f"{mode}.ever_epoch_boundary_pool_paid_eligible"]+=int(eligible_epoch_seen)
            per_seed[seed][mode]={"boundaries":labels,"functional":flags}
        if (hash_count!=49152 or len(per_seed)!=N or
                any(set(x)!=set(prior.MODES) for x in per_seed.values())):
            raise ValueError("original 128x16x24 cohort incomplete")
        for mode in prior.MODES:
            for boundary in BOUNDARIES.values():
                n=sum(v for k,v in counts.items() if k.startswith(f"{mode}.{boundary}."))
                if n!=N:
                    raise ValueError("boundary states not a 128-world partition")
        paired=Counter()
        for seed,arms in per_seed.items():
            for sensor in ("low","high"):
                on=arms[f"broad-ambient-on-intact-{sensor}"]["boundaries"]["W3_entry"]
                off=arms[f"broad-ambient-off-intact-{sensor}"]["boundaries"]["W3_entry"]
                paired[f"{sensor}.same_W3_entry_class"]+=int(on==off)
                paired[f"{sensor}.different_W3_entry_class"]+=int(on!=off)
        return {
            "study":"CL-1 Study75 original Study74-P source-pinned post-result gates",
            "scientific_status":"RETROSPECTIVE, ZERO NEW WORLDS",
            "original_revision":SOURCE,
            "original_independent_genomes_reused":N,
            "original_matched_runs":N*len(prior.MODES),
            "original_SHA256_epochs_verified_twice":hash_count,
            "original_primary":original["primary_A_W2W3_native_feedback_with_W3_Draw"],
            "mode_counts":dict(sorted(counts.items())),
            "paired_on_off_W3_entry_state":dict(sorted(paired.items())),
            "one_genome_only_responded_to_pool_paid_policy":
                original["primary_B_W3_resource_access"]["broad"]["on_worlds_with_actual_pool_paid_Draw"],
            "life_claim":"NONE",
        }
    finally:
        db.close()


def main() -> None:
    p=argparse.ArgumentParser(description=__doc__)
    p.add_argument("sqlite",type=Path)
    p.add_argument("--json",required=True,type=Path)
    args=p.parse_args()
    result=audit(args.sqlite)
    args.json.write_text(json.dumps(result,indent=2)+"\n",encoding="utf-8")
    print(json.dumps({
        "study":result["study"],
        "new_worlds":0,
        "original_hashes":result["original_SHA256_epochs_verified_twice"],
        "broad_ambient_on_low": {
            k:v for k,v in result["mode_counts"].items()
            if k.startswith("broad-ambient-on-intact-low.")
            and ("W3_entry." in k or "native_tainted_Write_W3" in k
                 or "native_Draw_W3" in k)
        },
        "life_claim":"NONE",
    },indent=2))


if __name__=="__main__":
    main()
