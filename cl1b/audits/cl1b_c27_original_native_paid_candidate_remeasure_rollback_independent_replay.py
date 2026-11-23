#!/usr/bin/env python3
"""C27 independent ordinary charged-CIL interpreter for original 8 native worlds.

Paid private credit sensors, two price tests, source edit, mid-life neutral
environment cell50 change, remeasurement, and actor-paid ReadNext -> PatchNext
rollback are all re-executed from frozen bytes. Not a reward oracle or B3.
"""
import copy
import json
import os
import sys
from pathlib import Path

ROOT=Path(__file__).resolve().parents[1]
FROZEN=ROOT/"studies/CL1B-C27-PAID-CREDIT-REMEASURE-ROLLBACK-frozen-8-worlds-v1.json"
OLD=ROOT/"studies/CL1B-C26-PAID-OWN-CREDIT-METER-CANDIDATE-CHOICE-frozen-4-worlds-v1.json"
KEYS=(11,41,42,43,44,45,50)
INITIAL=3000

def replay(source,arm,trace):
    src=copy.deepcopy(source)
    reg=[0,0,0,0]
    cells={50:arm["initial_mode"]}
    disabled=arm["patchnext_suppressed"]
    change=arm["mode_after_stage12"]
    pc=0;paid=0;halted=False;midpoint=False
    production=0;trials=0
    sensors=[];decisions=[];edits=[];environment=[];peak=172
    native=(json.loads(line) for line in Path(trace).open())
    def matching(row):
        orig=next(native,None)
        if row!=orig:
            raise AssertionError({"arm":arm["id"],"paid":paid,
              "native_original":orig,"independent_CIL":row})
    while paid<INITIAL:
        at=pc
        op=src[at]
        kind=op["kind"]
        paid+=1
        credits=INITIAL-paid
        ev="executed"
        if kind=="SenseOwnCredits":
            reg[op["dst"]]=credits
            pc=op["next"]
            assert at in (0,30,60) and op["dst"]==0
            sensors.append({"step":paid,"pc":at,"own_private_credits_after":credits,
                "mode":cells.get(50),"stage":cells.get(45,0)})
        elif kind=="Const":
            reg[op["dst"]]=op["value"];pc=op["next"]
        elif kind=="Read":
            reg[op["dst"]]=cells.get(reg[op["address"]],0);pc=op["next"];ev="read"
        elif kind=="ReadNext":
            assert at==164 and reg[op["node"]]==100
            reg[op["dst"]]=src[reg[op["node"]]]["next"]
            pc=op["next"]
        elif kind=="Write":
            key=reg[op["address"]];value=reg[op["value"]]
            cells[key]=value;pc=op["next"];ev="write"
            if at in (82,92):
                assert key==44 and value==int(at==92)
                assert value==cells[50]
                decisions.append({"step":paid,"pc":at,"chosen":value,
                    "mode":cells.get(50),"stage":cells.get(45,0)})
            if at in (107,137):
                assert key==11 and value==1
                production+=1
            if at in (12,42):
                assert key==11 and value==1
                trials+=1
        elif kind in ("Add","Sub"):
            a,b=reg[op["lhs"]],reg[op["rhs"]]
            reg[op["dst"]]=a+b if kind=="Add" else a-b
            assert -2**63<=reg[op["dst"]]<2**63
            pc=op["next"]
        elif kind=="Branch":
            pc=op["nonzero"] if reg[op["test"]]!=0 else op["zero"]
        elif kind=="PatchNext":
            assert at in (95,169) and reg[op["node"]]==100
            target=131 if at==95 else 101
            assert reg[op["target"]]==target
            prior=src[100]["next"]
            if not disabled:
                src[100]["next"]=target
            else:
                ev="suppressed"
            edits.append({"step":paid,"pc":at,"node":100,
                "attempted_target":target,"prior":prior,
                "actual_after":src[100]["next"],"suppressed":disabled,
                "mode":cells.get(50),"stage":cells.get(45,0)})
            pc=op["next"]
        elif kind=="Halt":
            halted=True;ev="halt"
        else:
            raise AssertionError(("unregistered original CIL opcode",kind,at))
        assert 0<=pc<len(src)
        assert src[100]["kind"]=="Const" and src[100]["dst"]==2 and src[100]["value"]==50
        used=171+len(cells)
        assert used<=196 and len(cells)<=16
        peak=max(peak,used)
        matching({"charged_step":paid,"pc":at,"next_pc":pc,"registers":reg.copy(),
            "own_credits_after":credits,
            "cells":[cells.get(k) for k in KEYS],
            "source_pc100_next":src[100]["next"],"occupied_units":used,
            "original_native_event":ev})
        if at==155 and cells.get(45)==12 and not midpoint:
            midpoint=True
            if change is not None:
                old=cells[50]
                assert change!=old
                cells[50]=change
                external={"after_charged_step":paid,"stage":12,"key":50,
                   "old_mode":old,"new_mode":change,"occupied_units":used,
                   "source_pc100_next":src[100]["next"]}
                environment.append(external)
                matching({"environment_change":external})
        if halted:
            assert next(native,None) is None
            break
    assert halted and midpoint and production==24 and trials==4
    assert cells.get(45)==24 and cells.get(11)==1
    assert len(sensors)==6 and len(decisions)==2
    assert len(environment)==int(change is not None)
    costs=[]
    for group in [sensors[:3],sensors[3:]]:
        a=group[0]["own_private_credits_after"]-group[1]["own_private_credits_after"]
        b=group[1]["own_private_credits_after"]-group[2]["own_private_credits_after"]
        costs.append({"A":a,"B":b,"actor_chose_B":b<a})
    mode2=change if change is not None else arm["initial_mode"]
    assert decisions[0]["chosen"]==arm["initial_mode"]
    assert decisions[1]["chosen"]==mode2
    assert costs[0]["actor_chose_B"]==bool(arm["initial_mode"])
    assert costs[1]["actor_chose_B"]==bool(mode2)
    assert src[100]["next"]==(131 if mode2 and not disabled else 101)
    rollback=sum(x["pc"]==169 for x in edits)
    assert rollback==int(arm["initial_mode"]==1 and mode2==0 and not disabled)
    return {
     "id":arm["id"],"initial_mode":arm["initial_mode"],
     "mode_after_stage12":change,"patchnext_suppressed":disabled,
     "total_paid_cil":paid,"private_credits_remain":INITIAL-paid,
     "sensor_records":sensors,"cost_comparisons_by_paid_phase":costs,
     "actor_paid_decisions":decisions,"actor_paid_patchnext_edits":edits,
     "actor_paid_live_rollback_pc169_count":rollback,
     "environment_events":environment,
     "final_executable_next_pc100":src[100]["next"],
     "real_production_services":production,"real_trial_services":trials,
     "completed_production_services":cells[45],"final_output":cells[11],
     "peak_occupied":peak,"original_pid_halted":True,"trapped":False
    }

def paired_provenance(folder,a,b):
    def rows(r):
        return [json.loads(v) for v in (folder/f"cl1b_c27_original_{r['id']}.jsonl").open()]
    assert a["initial_mode"]==b["initial_mode"]
    assert a["mode_after_stage12"]==b["mode_after_stage12"]
    assert a["patchnext_suppressed"] is False and b["patchnext_suppressed"] is True
    x,y=rows(a),rows(b)
    first=next((i for i,(aa,bb) in enumerate(zip(x,y)) if aa!=bb),None)
    if first is None:
        assert len(x)==len(y)
        return {"case":a["id"],"first_physical_source_change_exposure":None,
            "full_original_native_trace_identical":True}
    assert first is not None and x[:first]==y[:first]
    xx,yy=x[first],y[first]
    assert xx.get("pc")==yy.get("pc")==95
    assert xx["charged_step"]==yy["charged_step"]
    assert xx["registers"]==yy["registers"]
    assert xx["own_credits_after"]==yy["own_credits_after"]
    assert xx["cells"]==yy["cells"]
    assert xx["source_pc100_next"]==131 and yy["source_pc100_next"]==101
    assert xx["original_native_event"]=="executed" and yy["original_native_event"]=="suppressed"
    return {"case":a["id"],"first_physical_source_change_exposure":{
        "original_native_trace_events_identical_prior":first,
        "first_source_edit_paid_PC":95,"charged_step":xx["charged_step"],
        "real_on_next":131,"off_suppressed_next":101},
        "full_original_native_trace_identical":False}

def main():
    if len(sys.argv)!=3:
        raise SystemExit("usage: C27_ORIGINAL_NATIVE_SUMMARY.json ORIGINAL_NATIVE_JSONL_DIR")
    native=json.loads(Path(sys.argv[1]).read_text())
    m=json.loads(FROZEN.read_text())
    c26=json.loads(OLD.read_text())
    study="CL1B-C27-INTRA-LIFECYCLE-PAID-RESOURCE-REMEASUREMENT-SOURCE-ROLLBACK"
    assert native["study"]==m["study"]==study
    assert native["level"]=="ORIGINAL_NATIVE_C27_EIGHT_WORLD_PAID_RESOURCE_REMEASURE_AND_SOURCE_ROLLBACK"
    assert m["status"]=="PRE_NATIVE_FROZEN_8_ORIGINAL_WORLDS_IN_LIFECYCLE_ADAPT_AND_REVERT"
    assert c26["study"]=="CL1B-C26-PAID-OWN-CREDIT-METROLOGY-CANDIDATE-CHOICE"
    if os.environ.get("GITHUB_SHA"):
        assert native["source_sha"]==os.environ["GITHUB_SHA"]
    src=m["original_full_frozen_170_node_CIL_source"]
    old=c26["original_program_160_node_CIL"]
    assert len(src)==170 and len(old)==160
    assert all(src[i]==old[i] for i in range(160) if i not in (82,158))
    assert src[82]==dict(old[82],next=163)
    assert src[158]==dict(old[158],nonzero=160)
    assert src[164]=={"kind":"ReadNext","dst":0,"node":1,"next":165}
    assert src[169]=={"kind":"PatchNext","node":1,"target":0,"next":100}
    assert len(m["arms"])==len(native["results"])==8
    results=[]
    for a,r in zip(m["arms"],native["results"]):
        ans=replay(src,a,Path(sys.argv[2])/f"cl1b_c27_original_{a['id']}.jsonl")
        assert ans==r,(a["id"],next((k for k in ans if ans[k]!=r.get(k)),None))
        results.append(ans)
    assert all(r["real_production_services"]==24 and r["real_trial_services"]==4 for r in results)
    assert native["counts"]["original_native_worlds"]==8
    assert native["counts"]["production_services"]==192
    assert native["counts"]["trial_services"]==32
    assert native["counts"]["paid_private_sensor_executions"]==48
    assert sum(r["total_paid_cil"] for r in results)==native["counts"]["total_paid_CIL"]
    comparisons=[]
    paired=[]
    for i in range(4):
        on,off=results[2*i:2*i+2]
        delta=off["total_paid_cil"]-on["total_paid_cil"]
        assert delta==native["same_initial_source_paid_effect_by_regime"][i]["same_initial_source_paid_self_edit_effect_saving"]
        comparisons.append({"cohort":on["id"],"on_paid_CIL":on["total_paid_cil"],
                            "off_paid_CIL":off["total_paid_cil"],
                            "same_source_physical_effect_paid_saving":delta,
                            "actor_choices":[x["chosen"] for x in on["actor_paid_decisions"]],
                            "rollback_PC169_count":on["actor_paid_live_rollback_pc169_count"],
                            "environment_external_mode_changes":on["environment_events"]})
        paired.append(paired_provenance(Path(sys.argv[2]),on,off))
    print(json.dumps({
       "study":study,
       "tested_original_native_sha":native["source_sha"],
       "original_native_worlds_exactly_replayed":8,
       "first_divergence":None,
       "actual_private_credit_sensors_paid":48,
       "actual_production_services":192,
       "actual_candidate_trial_services":32,
       "total_original_paid_CIL":native["counts"]["total_paid_CIL"],
       "arm_full_paid_CIL":{r["id"]:r["total_paid_cil"] for r in results},
       "within_single_actor_lifetime_paid_candidate_price_comparisons":{
         r["id"]:r["cost_comparisons_by_paid_phase"] for r in results},
       "native_actor_paid_real_edits_and_rollback":{r["id"]:r["actor_paid_patchnext_edits"] for r in results},
       "exact_first_physical_edit_prefix":paired,
       "paired_same_initial_source_full_paid_cost_and_actor_decisions":comparisons,
       "scope":"Only two authored candidate programs under externally switched neutral data; PID paid measures, chooses, edits and rolls back code. Not invented candidates, B3 or B4."
    },indent=2))
if __name__=="__main__":
    main()
