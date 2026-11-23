#!/usr/bin/env python3
"""C26: independent ordinary charged CIL interpreter. No Rust VM imported.

C26 is deliberately a four-world engineering capability gate rather than
a blinded generalization study. It proves the private credit sensor is paid,
program itself measures both trial costs, compares them using only basic CIL,
then pays for the source change if and only if its own measured cost says B.
"""
import copy
import json
import os
import sys
from pathlib import Path

ROOT=Path(__file__).resolve().parents[1]
FROZEN=ROOT/"studies/CL1B-C26-PAID-OWN-CREDIT-METER-CANDIDATE-CHOICE-frozen-4-worlds-v1.json"
KEYS=(11,41,42,43,44,45,50)
CREDIT=3000

def independently_replay(source,arm,trace_file):
    assert len(source)==160
    src=copy.deepcopy(source)
    reg=[0,0,0,0]
    cells={50:arm["mode"]}
    pc=0; paid=0; stopped=False
    samples=[]; edits=[]; true_production=0; trials=0; peak=162
    disabled=arm["patchnext_suppressed"]
    native=(json.loads(line) for line in Path(trace_file).open())
    while paid<CREDIT:
        at=pc
        op=src[at]
        tag=op["kind"]
        paid+=1
        private_after=CREDIT-paid
        kind="executed"
        if tag=="Const":
            reg[op["dst"]]=op["value"]
            pc=op["next"]
        elif tag=="SenseOwnCredits":
            # Ordinary native scheduler has ALREADY charged this instruction,
            # including both the first and last samples; no free credit oracle.
            reg[op["dst"]]=private_after
            pc=op["next"]
            assert at in (0,30,60) and op["dst"]==0
            samples.append({"step":paid,"pc":at,"post_debit_private_credits":private_after})
        elif tag=="Read":
            address=reg[op["address"]]
            reg[op["dst"]]=cells.get(address,0)
            pc=op["next"];kind="read"
        elif tag=="Write":
            address=reg[op["address"]]
            value=reg[op["value"]]
            cells[address]=value
            pc=op["next"];kind="write"
            if at in (107,137):
                assert address==11 and value==1
                true_production+=1
            if at in (12,42):
                assert address==11 and value==1
                trials+=1
        elif tag in ("Add","Sub"):
            a,b=reg[op["lhs"]],reg[op["rhs"]]
            reg[op["dst"]]=a+b if tag=="Add" else a-b
            assert -2**63<=reg[op["dst"]]<2**63
            pc=op["next"]
        elif tag=="Branch":
            pc=op["nonzero"] if reg[op["test"]]!=0 else op["zero"]
        elif tag=="PatchNext":
            assert at==95 and reg[op["node"]]==100 and reg[op["target"]]==131
            if not disabled:
                assert src[100]["kind"]=="Const"
                src[100]["next"]=131
            else:
                kind="suppressed"
            pc=op["next"]
            edits.append({"step":paid,"pc":95,"node":100,
                          "new_next":src[100]["next"],"suppressed":disabled})
        elif tag=="Halt":
            stopped=True;kind="halt"
        else:
            raise AssertionError(("C26 unregistered CIL operation",tag,at))
        assert 0<=pc<len(src)
        assert src[100]["kind"]=="Const" and src[100]["dst"]==2 and src[100]["value"]==50
        occupied=161+len(cells)
        assert occupied<=185 and len(cells)<=16
        peak=max(peak,occupied)
        row={"charged_step":paid,"pc":at,"next_pc":pc,"registers":reg.copy(),
             "own_credits_after":private_after,
             "cells":[cells.get(k) for k in KEYS],
             "source_pc100_next":src[100]["next"],
             "occupied_units":occupied,
             "original_native_event":kind}
        original=next(native,None)
        if row!=original:
            raise AssertionError({"arm":arm["id"],"step":paid,"actual_native":original,
                                  "independent_paid_CIL":row})
        if stopped:
            assert next(native,None) is None
            break
    assert stopped,("C26 private CIL exhausted before original native halt",arm["id"])
    assert true_production==cells.get(45)==24
    assert cells.get(11)==1 and trials==2
    assert len(samples)==3
    sensed=[s["post_debit_private_credits"] for s in samples]
    assert [cells[i] for i in (41,42,43)]==sensed
    A=sensed[0]-sensed[1]
    B=sensed[1]-sensed[2]
    assert A>0 and B>0
    assert cells[44]==int(B<A)==arm["mode"]
    assert len(edits)==arm["mode"]
    assert src[100]["next"]==(131 if arm["mode"] and not disabled else 101)
    return {"id":arm["id"],"mode":arm["mode"],"patchnext_suppressed":disabled,
            "total_paid_cil":paid,"private_credits_remain":CREDIT-paid,
            "sensor_records":samples,"trial_cost_a_paid":A,"trial_cost_b_paid":B,
            "actor_chosen_candidate":cells[44],
            "actor_paid_real_edit_records":edits,
            "final_executable_next_pc100":src[100]["next"],
            "total_real_production_services":true_production,
            "total_real_trial_services":trials,
            "completed_production_services":cells[45],
            "final_service_output":cells[11],"peak_storage":peak,
            "trapped":False,"original_pid_halted":True}

def original_pairs(dir,results):
    def rows(record):
        path=dir/f"cl1b_c26_original_{record['id']}.jsonl"
        return [json.loads(line) for line in path.open()]
    by_id={r["id"]:r for r in results}
    output=[]
    for mode in (0,1):
        on=by_id[f"m{mode}_on"]
        off=by_id[f"m{mode}_off"]
        a,b=rows(on),rows(off)
        assert a[0]==b[0]
        first=next((i for i,(x,y) in enumerate(zip(a,b)) if x!=y),None)
        if mode==0:
            assert first is None and len(a)==len(b)
            output.append({"mode":0,"first_physical_edit_exposure":None,
                           "full_same_source_paid_CIL_native_traces_equal":True})
        else:
            assert first is not None and a[:first]==b[:first]
            x,y=a[first],b[first]
            assert x["charged_step"]==y["charged_step"] and x["pc"]==y["pc"]==95
            assert x["registers"]==y["registers"] and x["own_credits_after"]==y["own_credits_after"]
            assert x["cells"]==y["cells"] and x["occupied_units"]==y["occupied_units"]
            assert x["source_pc100_next"]==131 and y["source_pc100_next"]==101
            assert x["original_native_event"]=="executed" and y["original_native_event"]=="suppressed"
            output.append({"mode":1,"first_physical_edit_exposure":{
                "native_paid_step":x["charged_step"],"paid_PC":95,
                "fully_identical_prefix_original_events":first,
                "on_actual_next":131,"off_suppressed_next":101
            },"full_same_source_paid_CIL_native_traces_equal":False})
    return output

def main():
    if len(sys.argv)!=3:
        raise SystemExit("usage: C26_ORIGINAL_NATIVE_SUMMARY.json ORIGINAL_NATIVE_JSONL_DIR")
    original=json.loads(Path(sys.argv[1]).read_text())
    frozen=json.loads(FROZEN.read_text())
    study="CL1B-C26-PAID-OWN-CREDIT-METROLOGY-CANDIDATE-CHOICE"
    assert original["study"]==frozen["study"]==study
    assert frozen["status"]=="PRE_NATIVE_FROZEN_FOUR_WORLDS_GENERIC_ABI_TAG35"
    assert original["level"]=="ORIGINAL_NATIVE_C26_PAID_OWN_COMPUTE_PRICE_COMPARISON_FOUR_WORLDS"
    if os.environ.get("GITHUB_SHA"):
        assert original["source_sha"]==os.environ["GITHUB_SHA"]
    source=frozen["original_program_160_node_CIL"]
    assert len(source)==160 and len(original["results"])==len(frozen["arms"])==4
    assert sum(x["kind"]=="SenseOwnCredits" for x in source)==3
    assert source[95]=={"kind":"PatchNext","node":1,"target":0,"next":100}
    assert source[100]=={"kind":"Const","dst":2,"value":50,"next":101}
    results=[]
    for arm,n in zip(frozen["arms"],original["results"]):
        r=independently_replay(source,arm,Path(sys.argv[2])/f"cl1b_c26_original_{arm['id']}.jsonl")
        assert r==n,(arm["id"],r,n)
        results.append(r)
    assert all(r["total_real_production_services"]==24 for r in results)
    assert original["counts"]["original_native_worlds"]==4
    assert original["counts"]["true_production_services"]==96
    assert original["counts"]["true_trial_services"]==8
    assert sum(r["total_paid_cil"] for r in results)==original["counts"]["actual_paid_cil_total"]
    mode0=results[1]["total_paid_cil"]-results[0]["total_paid_cil"]
    mode1=results[3]["total_paid_cil"]-results[2]["total_paid_cil"]
    assert mode0==original["same_initial_source_mode0_patchnext_effect_net_paid_saving"]
    assert mode1==original["same_initial_source_mode1_patchnext_effect_net_paid_saving"]
    print(json.dumps({
       "study":study,
       "original_native_source_sha":original["source_sha"],
       "original_native_worlds_exactly_replayed":4,
       "first_divergence":None,
       "native_original_full_paid_CIL_per_world":{r["id"]:r["total_paid_cil"] for r in results},
       "paid_private_credit_sensors":{r["id"]:r["sensor_records"] for r in results},
       "actor_paid_cost_A_and_B_and_chosen_candidate":{r["id"]:{
           "cost_A":r["trial_cost_a_paid"],"cost_B":r["trial_cost_b_paid"],
           "choice":r["actor_chosen_candidate"]} for r in results},
       "actor_source_self_edit_witness":{r["id"]:r["actor_paid_real_edit_records"] for r in results},
       "same_initial_source_mode0_effect_net_paid_saving":mode0,
       "same_initial_source_mode1_effect_net_paid_saving":mode1,
       "original_pair_causal_first_effect_prefix":original_pairs(Path(sys.argv[2]),results),
       "true_production_services":96,
       "scientific_scope":"Finite authored two-candidate metrology capability only; NOT自主发明候选、B3、B4"
    },ensure_ascii=False,indent=2))
if __name__=="__main__":
    main()
