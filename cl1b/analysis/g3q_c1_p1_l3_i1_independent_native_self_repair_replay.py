#!/usr/bin/env python3
"""Original-native one-PID1 self-PatchConst repair source/event reexecution.

I1 is a prospective 178-node author-seeded CIL founder. We read original
Rust step-by-step snapshots and independently reconstruct every source
instruction, the one paid executable literal edit, register/cell/compute
states, and every external task feedback transition. The modified-physics
ablation suppresses native code edits and must fail to make progress.
This proves a narrowly preprogrammed repair, NOT generic creative evolution.
"""
import json
import os
import sys
from pathlib import Path

SOURCE=(Path(__file__).resolve().parents[1]/
    "studies/B56-F1d1k-G3Q-C1-P1-L3-I1-frozen-178-node-certificate-driven-native-self-repair-v1.json")
KEYS=(7,8,9,11,20,21,32,33,50,51,52,53,56,62,63,64)
STUDY="B56-F1d1k-G3Q-C1-P1-L3-I1"

def replay(frozen_code, arm, trace=None, suppress=False):
    code=[dict(x) for x in frozen_code]
    if arm["preload_mutation"]:
        assert code[6]["value"]==2
        code[6]["value"]=3
    reg=[0,0,0,0]
    cells={}
    pc=0
    paid=0
    patch=[]
    patch_attempts=0
    witness=[]
    cert_pass=[]
    service=[]
    meta_reads=0
    repaired_flag_zero=0
    stopped=False
    feedback_reads=[]
    peak=len(code)+1
    def records():
        if trace is None:
            while True:yield None
        else:
            with Path(trace).open() as stream:
                for line in stream:yield json.loads(line)
    rows=records()
    def check(expected):
        observed=next(rows,None)
        if trace is not None and observed!=expected:
            fields=("type","paid step","before PC","after PC","registers",
                    "private credits","16 cell presence/values","occupied units",
                    "self-edited executable Const6 literal")
            raise AssertionError({"first_native_divergence":paid,
                "native":observed,"independent":expected})
    while paid<8000:
        op=code[pc]
        here=pc
        paid+=1
        kind=op["kind"]
        if kind=="Const":
            reg[op["dst"]]=op["value"]
            pc=op["next"]
        elif kind in ("Add","Sub"):
            lhs,rhs=reg[op["lhs"]],reg[op["rhs"]]
            reg[op["dst"]]=lhs+rhs if kind=="Add" else lhs-rhs
            pc=op["next"]
        elif kind=="Read":
            key=reg[op["address"]]
            reg[op["dst"]]=cells.get(key,0)
            if here==37:
                assert key in (62,63,64)
                meta_reads+=1
            if here==79:
                assert key==20
                feedback_reads.append(reg[op["dst"]])
            pc=op["next"]
        elif kind=="Write":
            key=reg[op["address"]]
            val=reg[op["value"]]
            cells[key]=val
            if here==130:
                assert key in (7,8,9) and val==1
                witness.append({"step":paid,"key":key,"value":val})
            if here==170:
                assert key==21 and val==1
                assert [cells.get(k) for k in (7,8,9)]==[1,1,1]
                cert_pass.append(paid)
            if here==173:
                assert key==7 and val==0
                repaired_flag_zero+=1
            pc=op["next"]
        elif kind=="Branch":
            pc=op["nonzero"] if reg[op["test"]] else op["zero"]
        elif kind=="PatchConst":
            assert here==176
            index=reg[op["node"]]
            value=reg[op["value"]]
            assert index==6 and value==2 and code[index]["kind"]=="Const"
            prior=code[index]["value"]
            patch_attempts+=1
            if not suppress:
                code[index]["value"]=value
                patch.append({"step":paid,"pc":here,"node":index,"before":prior,"after":value,"suppressed":False})
            pc=op["next"]
        elif kind=="Halt":
            stopped=True
        else:
            raise AssertionError(("invalid source opcode",here,kind))
        peak=max(peak,len(code)+1+len(cells))
        assert len(cells)<=16 and peak<=300
        check(["cil",paid,here,pc,list(reg),8000-paid,
            [cells.get(k) for k in KEYS],len(code)+1+len(cells),code[6]["value"]])
        if here==71:
            assert cert_pass and kind=="Write" and key==11
            epoch=cells[32]
            assert epoch<4
            goal=arm["goals"][epoch]
            truth=int(val==goal)
            fb=truth if arm["feedback"]=="truth" else 1-truth
            rec={"step":paid,"epoch":epoch,"middle":cells[63],
                 "output":val,"truth":truth,"feedback":fb}
            service.append(rec)
            cells[20]=fb
            check(["feedback",paid,epoch,goal,cells[63],val,truth,fb,
                [cells.get(k) for k in KEYS],len(code)+1+len(cells),code[6]["value"]])
        if stopped:
            break
    if trace is not None:
        assert next(rows,None) is None,"native trace contains trailing records"
    result={
        "steps":paid,"halted":stopped,"halt_pc":pc,"credits_remaining":8000-paid,
        "source_literal6":code[6]["value"],"source_edits":patch,
        "attempts":patch_attempts,"certPass":cert_pass,
        "cert_witnesses":witness,"services":service,
        "meta":meta_reads,"flag7_reset_events":repaired_flag_zero,
        "used_cells":len(cells),"peak_occupied":peak,
        "original_actor_claim":cells.get(21),"paid_feedback_reads":feedback_reads}
    assert feedback_reads==[s["feedback"] for s in service]
    if suppress:
        e=arm["expected"]
        assert not stopped and paid==e["steps"]==8000
        assert pc==e["pc"]
        assert code[6]["value"]==3
        assert patch_attempts==e["source_edits"]
        assert repaired_flag_zero==e["repair_failures"]
        assert len(service)==e["source_task_service_events"]==0
        assert len(cert_pass)==e["cert_pass_steps"]==0
        assert len(cells)==e["used_cells"] and len(code)+1+len(cells)==e["occupied"]
    else:
        e=arm["expected"]
        assert stopped and paid==e["steps"] and pc==e["halt_pc"]
        assert code[6]["value"]==e["original_Pc6"]
        assert patch==e["source_edits"]
        assert cert_pass==e["cert_pass_steps"]
        assert witness==e["cert_witnesses"]
        assert service==e["source_task_service_events"]
        assert meta_reads==e["paid_metacode_reads"]
        assert len(cells)==e["used_cells"] and peak==len(code)+1+len(cells)
    return result

def main():
    if len(sys.argv) not in (1,3):
        raise SystemExit("usage: python3 I1_REPLAY.py [NATIVE_SUMMARY.json EVIDENCE_DIR]")
    frozen=json.loads(SOURCE.read_text())
    code=frozen["exact_native_founder_code"]
    arms=frozen["prespecified_original_arms"]
    assert len(code)==178 and len(arms)==3
    assert code[176]=={"kind":"PatchConst","node":1,"value":3,"next":0}
    native=None
    if len(sys.argv)==3:
        native=json.loads(Path(sys.argv[1]).read_text())
        assert native["study"]==STUDY
        assert native["level"]=="NATIVE_ONE_PID_PROGRAMMED_CERTIFICATE_DRIVEN_SOURCE_REPAIR_AND_TASK_RELEARNING"
        assert len(native["arms"])==3
        assert native["source_sha"] not in ("","local")
        if os.environ.get("GITHUB_SHA"):
            assert native["source_sha"]==os.environ["GITHUB_SHA"]
        evidence=Path(sys.argv[2])
    results=[]
    total=0
    for idx,arm in enumerate(arms):
        trace=(evidence/f"b56_f1d1k_g3q_c1_p1_l3_i1_original_paid_{arm['name']}.jsonl") if native else None
        result=replay(code,arm,trace)
        total+=result["steps"]
        if native:
            n=native["arms"][idx]
            assert n["arm"]==arm["name"]
            assert n["original_paid_steps"]==result["steps"]
            assert n["credits_remaining"]==result["credits_remaining"]
            assert n["original_pid1_halted"] is True
            assert n["source_edits"]==result["source_edits"]
            assert n["structural_certificate_success_count"]==len(result["certPass"])
            assert n["paid_witnesses"]==result["cert_witnesses"]
            assert n["paid_task_services"]==result["services"]
            assert n["paid_metacode_fetches"]==result["meta"]
            assert n["paid_feedback_reads"]==result["paid_feedback_reads"]
            assert n["final_executable_pc6"]==result["source_literal6"]
            assert n["peak_occupied"]==result["peak_occupied"]
            assert n["postload_external_source_or_rule_writes"]==0
        results.append({"arm":arm["name"],"paid_original_steps":result["steps"],
                        "self_patch_count":len(result["source_edits"]),
                        "postrepair_certification_count":len(result["certPass"]),
                        "real_task_services":len(result["services"]),
                        "true_task_matches":sum(x["truth"] for x in result["services"]),
                        "actor_claimed_success":result["original_actor_claim"],
                        "first_divergence":None})
    control=frozen["pre_registered_patch_suppressed_negative_world"]
    control_arm={"preload_mutation":True,"feedback":"truth","goals":[0,0,2,2],
                 "expected":control["expected"]}
    trace=(evidence/"b56_f1d1k_g3q_c1_p1_l3_i1_original_paid_unsafe_seed_edit_suppressed.jsonl") if native else None
    negative=replay(code,control_arm,trace,suppress=True)
    total+=negative["steps"]
    assert [x["paid_original_steps"] for x in results]==[1357,1724,1236]
    assert [x["self_patch_count"] for x in results]==[0,1,1]
    assert [x["true_task_matches"] for x in results]==[4,4,0]
    assert negative["steps"]==8000 and not negative["halted"] and negative["attempts"]==21
    print(json.dumps({
      "study":STUDY,
      "evidence_level":"NATIVE_EVERY_CIL_STEP_SOURCE_MUTATION_AND_ENV_FEEDBACK_REPLAY" if native else "INDEPENDENT_SOURCE_PREDICTION_ONLY",
      "source_sha":native["source_sha"] if native else None,
      "original_paid_steps_checked":total if native else 0,
      "original_arms":results,
      "code_edit_suppressed_control":{"charged_steps":negative["steps"],
          "attempted_patchconst_count":negative["attempts"],
          "source_const6_remains":negative["source_literal6"],
          "natural_halt":negative["halted"],
          "no_task_services":len(negative["services"])==0},
      "first_divergence":None,
      "limits":"One fixed author-designed repair from known unsafe3 to safe2, not general rule-invention, not verified semantic fitness under deceptive feedback."},indent=2))

if __name__=="__main__":main()
