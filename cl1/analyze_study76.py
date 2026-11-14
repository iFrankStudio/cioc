#!/usr/bin/env python3
"""Study76 independent SHA256/native-PC and conserved energy audit.

Six researcher-scripted resource/VM calibration fixtures, NO ecological worlds.
"""
from __future__ import annotations
import argparse
import json
import sqlite3
from pathlib import Path
import analyze

EPOCHS, QUOTA = 24, 12
MODES = (
    "legacy-fatal-ambient",
    "recoverable-ambient",
    "legacy-pool-fee-early",
    "recoverable-pool-fee-reserve",
    "recoverable-no-pool",
    "recoverable-invalid-operand",
)
EXPECTED = {
    "legacy-fatal-ambient":(2,0,0,1,0,[0,1],12,0,True),
    "recoverable-ambient":(6,1,1,0,1,[0,1,1,2,3,4],3,5,True),
    "legacy-pool-fee-early":(1,0,0,1,1,[0],15,0,True),
    "recoverable-pool-fee-reserve":(4,1,0,0,1,[0,1,2,3],7,5,True),
    "recoverable-no-pool":(0,0,0,0,0,[],0,0,False),
    "recoverable-invalid-operand":(1,0,0,1,0,[0],16,0,True),
}

def name(kind) -> str:
    return next(iter(kind)) if isinstance(kind,dict) else kind


def program(mode: str) -> list:
    if mode in ("recoverable-no-pool","recoverable-invalid-operand"):
        return [{"DrawCompute":{"amount":1,"next":0}}]
    if mode in ("legacy-fatal-ambient","recoverable-ambient"):
        return [
            {"Const":{"dst":1,"value":8,"next":1}},
            {"DrawCompute":{"amount":1,"next":2}},
            {"Const":{"dst":0,"value":7,"next":3}},
            {"Write":{"address":2,"value":0,"next":4}},
            "Halt",
        ]
    return [
        {"DrawCompute":{"amount":1,"next":1}},
        {"Const":{"dst":0,"value":7,"next":2}},
        {"Write":{"address":2,"value":0,"next":3}},
        "Halt",
    ]


def audit(path: Path, source: str) -> dict:
    db=sqlite3.connect(f"{path.resolve().as_uri()}?mode=ro",uri=True)
    try:
        rows=db.execute(
            "SELECT id,condition,source_revision,initial_sha256,initial_json,actions_json,"
            "epoch_count,quota,attempts_json,final_sha256,final_json "
            "FROM epoch_runs ORDER BY id"
        ).fetchall()
        if len(rows)!=6:
            raise ValueError("scripted calibration has wrong number of conditions")
        results={}
        total_snapshots=0
        for (rid,mode,revision,initial_sha,raw_initial,raw_actions,epochs,quota,
             raw_attempts,final_sha,raw_final) in rows:
            if (mode not in MODES or mode in results or revision!=source
                    or epochs!=EPOCHS or quota!=QUOTA
                    or analyze.digest(raw_initial)!=initial_sha
                    or analyze.digest(raw_final)!=final_sha):
                raise ValueError("source-pinned original run not the registered mode")
            initial=json.loads(raw_initial)
            actual_code=initial["processes"]["1"]["code"]["nodes"]
            if actual_code!=program(mode):
                raise ValueError("human scripted program was modified")
            recovering=mode.startswith("recoverable")
            pool_paid=mode not in ("legacy-fatal-ambient","recoverable-invalid-operand")
            initial_pool = (4 if mode.endswith("ambient")
                            else 8 if mode in ("legacy-pool-fee-early","recoverable-pool-fee-reserve")
                            else 16 if mode=="recoverable-invalid-operand"
                            else 0)
            start_credits= (2 if mode.endswith("ambient")
                            else 1 if mode=="recoverable-invalid-operand" else 0)
            expected_registers=[0,-1 if mode=="recoverable-invalid-operand" else 8,9,0]
            if (initial.get("recoverable_draw_shortage",False)!=recovering
                    or initial.get("pool_paid_draw",False)!=pool_paid
                    or initial["processes"]["1"]["registers"]!=expected_registers
                    or initial["processes"]["1"]["credits"]!=start_credits
                    or initial["compute_pool"]!=initial_pool
                    or initial["storage_limit"]!=4 or initial["max_nodes_per_process"]!=8
                    or initial["total_storage_limit"]!=18
                    or not initial["strict_accounting"]
                    or initial["cells"]):
                raise ValueError("frozen initial VM flags, source ports or energy modified")
            actions=json.loads(raw_actions)
            refilled=mode in (
                "legacy-fatal-ambient","recoverable-ambient",
                "legacy-pool-fee-early","recoverable-pool-fee-reserve",
            )
            expected_actions=([
                {"epoch":1,"action":{"SetComputePool":{"credits":12 if mode.endswith("ambient") else
                                                       15 if mode=="legacy-pool-fee-early" else 16}}}
            ] if refilled else [])
            if actions!=expected_actions:
                raise ValueError("external researcher pool source was altered")
            snapshots=db.execute(
                "SELECT epoch,state_sha256,state_json FROM epoch_snapshots "
                "WHERE run_id=? ORDER BY epoch",(rid,)
            ).fetchall()
            if len(snapshots)!=EPOCHS or [x[0] for x in snapshots]!=list(range(EPOCHS)):
                raise ValueError("original 24-epoch archive incomplete")
            attempts=json.loads(raw_attempts)
            if len(attempts)!=EPOCHS:
                raise ValueError("original native attempt quota record missing")
            pc=0
            reg=expected_registers.copy()
            local=start_credits
            pool=initial_pool
            stopped=False
            writes=deferred=traps=fees=0
            original_events=initial["events"]
            if (len(original_events)!=2 or
                    [name(x["kind"]) for x in original_events]!=["Loaded","Intervened"] or
                    any(x["origin"]!="External" for x in original_events)):
                raise ValueError("original setup was not explicitly external")
            original_total=pool+local
            steps=[]
            previous=initial
            for epoch,digest,raw_state in snapshots:
                if analyze.digest(raw_state)!=digest:
                    raise ValueError("source-pinned native epoch SHA256 mismatch")
                state=json.loads(raw_state)
                if state["events"][:len(previous["events"])]!=previous["events"]:
                    raise ValueError("native ledger is not append-only")
                events=state["events"][len(previous["events"]):]
                external=events[:1] if refilled and epoch==1 else []
                if external:
                    e=external[0]
                    target=expected_actions[0]["action"]["SetComputePool"]["credits"]
                    if e["origin"]!="External" or name(e["kind"])!="Intervened":
                        raise ValueError("host pool injection misattributed as native")
                    original_total+=target-pool
                    pool=target
                native=events[len(external):]
                if len(native)!=attempts[epoch] or attempts[epoch]>QUOTA:
                    raise ValueError("registered native attempt quota differs")
                for e in native:
                    if (e["origin"]!={"Internal":1} or e["process"]!=1
                            or e["pc"]!=pc or stopped):
                        raise ValueError("native process or original PC fake")
                    op=actual_code[pc]
                    op_name=name(op)
                    event_name=name(e["kind"])
                    if e.get("pool_paid_attempt",False):
                        if not pool_paid or local!=0 or pool<=0 or op_name!="DrawCompute":
                            raise ValueError("forged world-paid native attempt")
                        pool-=1
                        fees+=1
                    else:
                        if local<=0:
                            raise ValueError("native instruction used no local compute")
                        local-=1
                    steps.append(e)
                    if event_name=="Trapped":
                        if op_name!="DrawCompute" or (
                            mode!="recoverable-invalid-operand"
                            and not (reg[1]>pool and not recovering)
                        ):
                            raise ValueError("unregistered fatal native trap")
                        stopped=True
                        traps+=1
                    elif event_name=="DrawDeferred":
                        val=e["kind"]["DrawDeferred"]
                        if (not recovering or op_name!="DrawCompute" or
                                val!={"requested":reg[1],"available":pool}
                                or reg[1]<=pool):
                            raise ValueError("deferred Draw forged or unexpectedly affordable")
                        deferred+=1
                    elif op_name=="DrawCompute":
                        if event_name!="Executed" or pool<reg[1]:
                            raise ValueError("native Draw succeeded without available common energy")
                        pool-=reg[1]
                        local+=reg[1]
                        pc=op["DrawCompute"]["next"]
                    elif op_name=="Const":
                        if event_name!="Executed":
                            raise ValueError("native Const did not execute")
                        item=op["Const"]
                        reg[item["dst"]]=item["value"]
                        pc=item["next"]
                    elif op_name=="Write":
                        if event_name!="CellWritten" or e["kind"]["CellWritten"] != {
                            "key":reg[op["Write"]["address"]],
                            "value":reg[op["Write"]["value"]],
                        }:
                            raise ValueError("native functional output not from original source")
                        writes+=1
                        pc=op["Write"]["next"]
                    elif op_name=="Halt":
                        if event_name!="Halted":
                            raise ValueError("native Halt faked")
                        stopped=True
                    else:
                        raise ValueError("unregistered original native opcode")
                    if e["credits_after"]!=local:
                        raise ValueError("native per-attempt actor balance falsified")
                process=state["processes"]["1"]
                if (state.get("recoverable_draw_shortage",False)!=recovering
                        or state.get("pool_paid_draw",False)!=pool_paid
                        or process["code"]["nodes"]!=actual_code
                        or process["registers"]!=reg
                        or process["pc"]!=pc or process["stopped"]!=stopped
                        or process["credits"]!=local or state["compute_pool"]!=pool
                        or pool+local != original_total-len(steps)):
                    raise ValueError("native world source, attempt charge, or state replay inconsistent")
                previous=state
                total_snapshots+=1
            if previous!=json.loads(raw_final):
                raise ValueError("recorded final native world differs from last hashed epoch")
            expected=EXPECTED[mode]
            observed=(len(steps),writes,deferred,traps,fees,
                      [x["pc"] for x in steps],pool,local,stopped)
            if observed!=expected:
                raise ValueError(f"scripted native mode violated frozen control: {mode}: {observed}")
            results[mode]={
                "original_charged_attempts":len(steps),
                "native_shared_outputs":writes,
                "native_deferred_resource_requests":deferred,
                "native_fatal_traps":traps,
                "world_pool_paid_native_attempts":fees,
                "final_pool":pool,
                "final_local_credits":local,
            }
        if set(results)!=set(MODES) or total_snapshots!=144:
            raise ValueError("incomplete exact six-fixture original snapshot cohort")
        return {
            "study":"CL-1 Study76 human-scripted native resource retry VM calibration",
            "source_revision":source,
            "human_scripted_fixtures":6,
            "new_independent_ecological_worlds":0,
            "matched_native_runs":6,
            "SHA256_verified_original_epochs":total_snapshots,
            "modes":results,
            "researcher_provided_physics":"RECOVERABLE SHORTAGE AND ATOMIC POOL FEE",
            "host_actor_specific_rescue":0,
            "life_claim":"NONE",
        }
    finally:
        db.close()


def main():
    p=argparse.ArgumentParser(description=__doc__)
    p.add_argument("sqlite",type=Path)
    p.add_argument("--expected-revision",required=True)
    p.add_argument("--json",type=Path,required=True)
    a=p.parse_args()
    report=audit(a.sqlite,a.expected_revision)
    a.json.write_text(json.dumps(report,indent=2)+"\n",encoding="utf-8")
    print(json.dumps(report,indent=2))


if __name__=="__main__":
    main()
