#!/usr/bin/env python3
"""Study67 retrospective cause of Study66 W3 inactivity, ZERO new worlds.

Run the untouched original Stage66 complete independent source/ledger auditor
before any descriptive post-result classification.
"""
from __future__ import annotations

import argparse
import json
import sqlite3
from collections import Counter, defaultdict
from pathlib import Path

import analyze
import analyze_study66
from analyze_study65 import op_name

REVISION = "156fd52bea389296b0c9a3b72e4bf8c737e30cf2"
FIRST, N, EPOCHS = 4_400_000, 256, 24
MODES = analyze_study66.MODES


def resource_state(actor: dict, pool: int) -> str:
    if actor["stopped"]:
        return "stopped_with_pool" if pool > 0 else "stopped_pool_empty"
    if actor["credits"] == 0:
        return "zero_credit_deadlock_with_pool" if pool > 0 else "zero_credit_pool_empty"
    return "still_runnable"


def terminal_cause(native_event: dict | None, program: list) -> str:
    if native_event is None:
        return "no_native_terminal"
    kind = native_event["kind"]
    name = op_name(kind)
    if name == "Halted":
        return "native_Halt"
    if name != "Trapped":
        return "no_native_terminal"
    pc = native_event["pc"]
    if not isinstance(pc,int) or not 0 <= pc < len(program):
        raise ValueError("fatal native event does not reference CIL code")
    message = kind["Trapped"]
    if "insufficient world compute pool" in message:
        if op_name(program[pc]) != "DrawCompute":
            raise ValueError("fatal resource trap not from native DrawCompute")
        return "fatal_native_DrawCompute_pool_insufficient"
    return "other_native_trap"


def audit_one(db: sqlite3.Connection, rid: int, initial: dict,
              final: dict, actions: list[dict], attempts: list[int]) -> dict:
    previous = initial
    snapshots = []
    first_terminal = None
    first_native_epoch = None
    last_native_epoch = None
    first_empty_epoch = None
    successful_draws = []
    fatal_draw = None
    native_events = 0
    for epoch, digest, raw in db.execute(
        "SELECT epoch,state_sha256,state_json FROM epoch_snapshots "
        "WHERE run_id=? ORDER BY epoch", (rid,)
    ):
        if epoch != len(snapshots) or analyze.digest(raw) != digest:
            raise ValueError("original Stage66 immutable epoch SHA256 corrupted")
        snap = json.loads(raw)
        if snap["events"][:len(previous["events"])] != previous["events"]:
            raise ValueError("historical native events rewritten")
        external = [a for a in actions if a["epoch"] == epoch]
        new = snap["events"][len(previous["events"]):]
        if len(new) != len(external) + attempts[epoch]:
            raise ValueError("native and external event counts mismatch")
        for e in new[:len(external)]:
            if e["origin"] != "External" or op_name(e["kind"]) != "Intervened":
                raise ValueError("researcher-side pool input misattributed to native")
        native = new[len(external):]
        if native:
            if first_native_epoch is None:
                first_native_epoch = epoch
            last_native_epoch = epoch
        elif first_empty_epoch is None and first_native_epoch is not None:
            first_empty_epoch = epoch
        for e in native:
            if analyze.internal_process(e) != 1 or e["process"] != 1:
                raise ValueError("native actor attribution falsified")
            pc = e["pc"]
            program = initial["processes"]["1"]["code"]["nodes"]
            if not isinstance(pc,int) or not 0 <= pc < len(program):
                raise ValueError("terminal CIL event outside frozen program")
            event_kind = op_name(e["kind"])
            if event_kind in ("Halted","Trapped") and first_terminal is None:
                first_terminal = {
                    "epoch":epoch,"tick":e["tick"],"pc":pc,
                    "cause":terminal_cause(e,program),
                    "message":e["kind"]["Trapped"] if event_kind == "Trapped" else None,
                }
                if op_name(program[pc]) == "DrawCompute" and event_kind == "Trapped":
                    fatal_draw = dict(first_terminal)
            if op_name(program[pc]) == "DrawCompute" and event_kind == "Executed":
                successful_draws.append({"epoch":epoch,"tick":e["tick"],"pc":pc})
            native_events += 1
        previous = snap
        snapshots.append(snap)
    if len(snapshots) != EPOCHS or final != snapshots[-1]:
        raise ValueError("Stage66 original source final not equal to 24th epoch")
    w3 = snapshots[15]
    w3_state = resource_state(w3["processes"]["1"], w3["compute_pool"])
    if sum(attempts[16:]) != 0 or w3_state == "still_runnable":
        raise ValueError("Study66 original W3 zero/native runnable boundary changed")
    final_state = resource_state(final["processes"]["1"], final["compute_pool"])
    return {
        "attempts_by_window":[sum(attempts[i:i+8]) for i in (0,8,16)],
        "first_native_epoch":first_native_epoch,
        "last_native_epoch":last_native_epoch,
        "first_empty_after_activity_epoch":first_empty_epoch,
        "first_terminal":first_terminal,
        "terminal_cause":first_terminal["cause"] if first_terminal else "no_native_terminal",
        "successful_native_DrawCompute_events":len(successful_draws),
        "first_Draw_epoch":successful_draws[0]["epoch"] if successful_draws else None,
        "last_Draw_epoch":successful_draws[-1]["epoch"] if successful_draws else None,
        "fatal_Draw":fatal_draw,
        "W3_entry_state":w3_state,
        "W3_entry_actor_credits":w3["processes"]["1"]["credits"],
        "W3_entry_world_pool":w3["compute_pool"],
        "W3_entry_total_compute_inventory":(
            w3["processes"]["1"]["credits"] + w3["compute_pool"]
        ),
        "final_state":final_state,
        "final_pool":final["compute_pool"],
        "final_actor_credits":final["processes"]["1"]["credits"],
        "charged_native_attempts":native_events,
        "sha256_snapshots_verified":len(snapshots),
    }


def audit(path: Path) -> dict:
    original = analyze_study66.audit(path, REVISION)
    if (original["new_independent_seed_genomes"] != N
            or original["SHA256_verified_original_snapshots"] != N*len(MODES)*EPOCHS
            or original["primary_A_coupled_random_native_read_write_and_DrawCompute"]["worlds"] != 0
            or original["secondary_B_recurrent_W2_W3_native_Draw_and_feedback"]["worlds"] != 0):
        raise ValueError("frozen original complete Study66 negative result changed")
    db=sqlite3.connect(f"{path.resolve().as_uri()}?mode=ro",uri=True)
    try:
        seed_arms=defaultdict(dict)
        counts=Counter()
        examples=[]
        snapshots_count=0
        for rid, condition, revision, raw_initial, raw_final, raw_actions, raw_attempts in db.execute(
            "SELECT id,condition,source_revision,initial_json,final_json,"
            "actions_json,attempts_json FROM epoch_runs ORDER BY id"
        ):
            label, mode = condition.split("/",1)
            seed=int(label.removeprefix("seed="))
            if (revision != REVISION or not FIRST <= seed < FIRST+N
                    or mode not in MODES or mode in seed_arms[seed]):
                raise ValueError("historical Stage66 original source/arm identity changed")
            seed_arms[seed][mode]=True
            d=audit_one(db,rid,json.loads(raw_initial),json.loads(raw_final),
                        json.loads(raw_actions),json.loads(raw_attempts))
            snapshots_count+=d["sha256_snapshots_verified"]
            for w, name in enumerate(("W1","W2","W3")):
                counts[f"{mode}.{name}_native_attempt_worlds"]+=int(d["attempts_by_window"][w]>0)
                counts[f"{mode}.{name}_native_attempts"]+=d["attempts_by_window"][w]
            counts[f"{mode}.worlds_with_any_native_Draw"]+=int(d["successful_native_DrawCompute_events"]>0)
            counts[f"{mode}.successful_native_Draw_events"]+=d["successful_native_DrawCompute_events"]
            counts[f"{mode}.W3_entry_{d['W3_entry_state']}"]+=1
            counts[f"{mode}.final_{d['final_state']}"]+=1
            counts[f"{mode}.terminal_{d['terminal_cause']}"]+=1
            counts[f"{mode}.last_native_epoch_{d['last_native_epoch']}"]+=1
            if d["first_empty_after_activity_epoch"] is not None:
                counts[f"{mode}.became_inactive_by_W2"]+=int(
                    d["first_empty_after_activity_epoch"] <= 8
                )
            if d["last_Draw_epoch"] is not None:
                counts[f"{mode}.last_native_Draw_in_W{d['last_Draw_epoch']//8+1}"]+=1
            if ((d["terminal_cause"] == "fatal_native_DrawCompute_pool_insufficient"
                 or d["W3_entry_state"] == "zero_credit_deadlock_with_pool")
                 and len(examples)<24):
                examples.append({"seed":seed,"mode":mode,**d})
        if (len(seed_arms)!=N or any(set(x)!=set(MODES) for x in seed_arms.values())
                or snapshots_count!=N*len(MODES)*EPOCHS):
            raise ValueError("original Stage66 historical source or hashes not complete")
        for mode in MODES:
            if counts[f"{mode}.W3_native_attempt_worlds"]!=0:
                raise ValueError("Stage66 original preregistered W3 zero changed")
        return {
            "study":"CL-1 Study67 retrospective native actor credit and fatal resource trap diagnosis",
            "post_result_registration":True,
            "new_independent_worlds":0,
            "source_original_study":"Study66",
            "source_revision":REVISION,
            "original_seed_genomes_reused":N,
            "original_matched_runs":N*len(MODES),
            "original_SHA256_snapshots_rechecked":snapshots_count,
            "original_stage66_primary_was":original[
                "primary_A_coupled_random_native_read_write_and_DrawCompute"
            ],
            "per_arm_retrospective_resource_failure_counts":dict(sorted(counts.items())),
            "first_original_native_failure_witnesses":examples,
            "life_claim":"NONE",
        }
    finally:
        db.close()


def main() -> None:
    p=argparse.ArgumentParser(description=__doc__)
    p.add_argument("sqlite",type=Path)
    p.add_argument("--json",type=Path,required=True)
    args=p.parse_args()
    report=audit(args.sqlite)
    args.json.write_text(json.dumps(report,indent=2)+"\n",encoding="utf-8")
    print(json.dumps({
        "study":report["study"],
        "new_worlds":0,
        "hashes":report["original_SHA256_snapshots_rechecked"],
        "ambient_low":{
            k:v for k,v in report["per_arm_retrospective_resource_failure_counts"].items()
            if k.startswith("ambient-low.")
        },
        "ambient_high":{
            k:v for k,v in report["per_arm_retrospective_resource_failure_counts"].items()
            if k.startswith("ambient-high.")
        },
        "life_claim":report["life_claim"],
    },indent=2))


if __name__=="__main__":
    main()
