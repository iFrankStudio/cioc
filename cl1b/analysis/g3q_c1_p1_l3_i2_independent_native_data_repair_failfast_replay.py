#!/usr/bin/env python3
"""I2: independent source-authoritative every-charged-original-CIL replay.

Reconstructs the exact 180-node CIL interpreter with data-conditioned native
PatchConst at PC178, real actor-paid structural certificate, one-repair guard,
and external hidden-task SetCell20 transitions. The Rust trace and summary are
observations rather than executable instructions. A suppressed-edit control
must fail closed after one retry without task service.

Proof boundary: original actor copies a researcher-supplied first-word gene,
not a discovery of an algorithmic search rule or semantic truth.
"""
import json
import os
import sys
from pathlib import Path

FROZEN = (Path(__file__).resolve().parents[1] /
    "studies/B56-F1d1k-G3Q-C1-P1-L3-I2-frozen-180-node-data-conditioned-repair-and-failfast-v1.json")
STUDY="B56-F1d1k-G3Q-C1-P1-L3-I2"
KEYS=(7,8,9,11,20,21,32,33,50,51,52,53,56,62,63,64)
GOALS=(0,0,2,2)
CREDITS=8000

def replay_source(template,arm,trace=None,disable_edit=False):
    code=[dict(op) for op in template]
    for pc,field in ((0,"first"),(3,"middle"),(6,"tail")):
        code[pc]["value"]=arm[field]
    r=[0,0,0,0]
    cell={}
    paid=0
    pc=0
    peak=len(code)+1
    witnesses=[]
    certified=[]
    repairs=[]
    services=[]
    meta_fetches=0
    feedback_reads=[]
    guard_checks=0
    def records():
        if trace is None:
            while True:
                yield None
        else:
            with Path(trace).open() as stream:
                for line in stream:
                    yield json.loads(line)
    stream=records()
    def audit(expected):
        actual=next(stream,None)
        if trace is not None and actual!=expected:
            raise AssertionError({"first_original_paid_step_divergence":paid,
                "native":actual,"independently_replayed":expected})
    while paid<CREDITS:
        op=code[pc]
        at=pc
        paid+=1
        kind=op["kind"]
        if kind=="Const":
            r[op["dst"]]=op["value"]
            pc=op["next"]
        elif kind=="Add" or kind=="Sub":
            lhs,rhs=r[op["lhs"]],r[op["rhs"]]
            r[op["dst"]]=lhs+rhs if kind=="Add" else lhs-rhs
            pc=op["next"]
        elif kind=="Read":
            addr=r[op["address"]]
            v=cell.get(addr,0)
            r[op["dst"]]=v
            if at==33:
                assert addr in (62,63,64)
                meta_fetches+=1
            if at==75:
                assert addr==20
                feedback_reads.append(v)
            pc=op["next"]
        elif kind=="Write":
            addr=r[op["address"]]
            val=r[op["value"]]
            cell[addr]=val
            if at==126:
                assert addr in (7,8,9) and val==1
                witnesses.append({"step":paid,"key":addr,"value":val})
            if at==166:
                assert addr==21 and val==1
                assert [cell.get(k) for k in (7,8,9)]==[1,1,1]
                certified.append(paid)
            if at==171:
                assert addr==11 and val==1
                guard_checks+=1
            pc=op["next"]
        elif kind=="Branch":
            pc=op["nonzero"] if r[op["test"]] else op["zero"]
        elif kind=="PatchConst":
            assert at==178 and op["node"]==1 and op["value"]==3
            node=r[op["node"]]
            requested=r[op["value"]]
            assert node==6 and requested==arm["first"]
            assert code[node]["kind"]=="Const"
            prior=code[node]["value"]
            assert len(repairs)==0
            if not disable_edit:
                code[node]["value"]=requested
            repairs.append({"step":paid,"pc":at,"node":node,
                "before":prior,"after":code[node]["value"],
                "edit_suppressed":disable_edit})
            pc=op["next"]
        elif kind=="Halt":
            assert at in (94,117,179)
            pc=at
        else:
            raise AssertionError(("unknown CIL instruction",at,kind))
        peak=max(peak,181+len(cell))
        assert len(cell)<=16 and peak<=300
        audit(["cil",paid,at,pc,list(r),CREDITS-paid,
            [cell.get(k) for k in KEYS],181+len(cell),code[6]["value"]])
        if kind=="Halt":
            break
        if at==67:
            assert kind=="Write" and addr==11 and len(certified)==1
            episode=cell.get(32,0)
            assert episode<4
            truth=int(val==GOALS[episode])
            mode=arm["feedback"]
            assert mode in ("truth","inverted")
            feedback=truth if mode=="truth" else 1-truth
            middle=cell[63]
            event={"step":paid,"episode":episode,"middle":middle,
                "output":val,"truth":truth,"feedback":feedback}
            services.append(event)
            cell[20]=feedback
            peak=max(peak,181+len(cell))
            audit(["feedback",paid,episode,GOALS[episode],middle,
                val,truth,feedback,[cell.get(k) for k in KEYS],
                181+len(cell),code[6]["value"]])
    else:
        raise AssertionError("no original native Halt within frozen 8000 credits")
    assert at in (94,117,179)
    if trace is not None:
        assert next(stream,None) is None, "additional uncharged native trace records"
    assert len(repairs)<=1
    assert guard_checks==len(repairs)
    assert len(certified)<=1
    assert feedback_reads==[x["feedback"] for x in services]
    assert meta_fetches==3*(len(witnesses)+len(services))
    result={
      "steps":paid,"halt_pc":at,"credits_remaining":CREDITS-paid,
      "certificate_witnesses":witnesses,
      "cert_pass_steps":certified,"services":services,
      "source_edits":repairs,"source_tail_final":code[6]["value"],
      "episode_final":cell.get(32),"task_success_claim":cell.get(21),
      "used_cells":len(cell),"peak_occupied":peak,
      "paid_meta_fetches":meta_fetches,"paid_feedback_reads":feedback_reads}
    for field in result:
        if result[field]!=arm["expected"][field]:
            raise AssertionError(("frozen source prediction differs",arm["name"],
              field,result[field],arm["expected"][field]))
    return result

def main():
    frozen=json.loads(FROZEN.read_text())
    code=frozen["complete_original_CIL_source"]
    assert len(code)==180
    assert code[178]=={"kind":"PatchConst","node":1,"value":3,"next":0}
    arms=frozen["pre_registered_arms"]+[frozen["pre_registered_control"]]
    assert len(arms)==9
    native=None
    if len(sys.argv)==3:
        native=json.loads(Path(sys.argv[1]).read_text())
        assert native["study"]==STUDY
        assert native["level"]=="NATIVE_180_NODE_ORIGINAL_PID_DATA_CONDITIONED_EXECUTABLE_REPAIR_AND_BOUND_FAILFAST"
        assert len(native["arms"])==9
        assert native["source_sha"] not in ("","local")
        if os.environ.get("GITHUB_SHA"):
            assert native["source_sha"]==os.environ["GITHUB_SHA"]
        trace_folder=Path(sys.argv[2])
    elif len(sys.argv)!=1:
        raise SystemExit("Usage: python3 I2_REPLAY.py [NATIVE_SUMMARY.json NATIVE_TRACES_DIR]")
    evidence=[]
    total_paid=0
    for idx,arm in enumerate(arms):
        disabled=arm["name"]==frozen["pre_registered_control"]["name"]
        trace=(trace_folder/f"b56_f1d1k_g3q_c1_p1_l3_i2_native_paid_{arm['name']}.jsonl") if native else None
        result=replay_source(code,arm,trace,disable_edit=disabled)
        total_paid+=result["steps"]
        if native:
            n=native["arms"][idx]
            assert n["arm"]==arm["name"]
            expected_native={
              "original_paid_steps":result["steps"],
              "private_credits_remaining":result["credits_remaining"],
              "original_executable_source_nodes":180,
              "certified":len(result["cert_pass_steps"])==1,
              "certificate_witnesses":result["certificate_witnesses"],
              "self_edits":result["source_edits"],
              "paid_task_services":result["services"],
              "paid_feedback_reads":result["paid_feedback_reads"],
              "paid_meta_fetches":result["paid_meta_fetches"],
              "final_executable_tail":result["source_tail_final"],
              "actor_success_claim":result["task_success_claim"],
              "final_halt_pc":result["halt_pc"],
              "peak_occupied_units":result["peak_occupied"],
              "single_original_pid1":True,"native_halted":True,
              "host_source_edits_after_load":0}
            for key,value in expected_native.items():
                if n[key]!=value:
                    raise AssertionError(("native summary provenance contradiction",arm["name"],key,
                        n[key],value))
        evidence.append({"arm":arm["name"],
          "paid_original_instructions":result["steps"],
          "certified":bool(result["cert_pass_steps"]),
          "original_actor_self_edit":result["source_edits"],
          "task_probes":len(result["services"]),
          "true_goal_matches":sum(x["truth"] for x in result["services"]),
          "actor_success_claim":result["task_success_claim"],
          "native_first_divergence":None})
    assert len({i["original_actor_self_edit"][0]["after"] for i in evidence[1:4]})==3
    assert [i["true_goal_matches"] for i in evidence[:4]]==[4,4,4,4]
    assert all(i["task_probes"]==0 and not i["certified"] for i in evidence[5:7])
    assert evidence[7]["actor_success_claim"]==1 and evidence[7]["true_goal_matches"]==0
    assert evidence[8]["task_probes"]==0 and evidence[8]["paid_original_instructions"]<8000
    print(json.dumps({"study":STUDY,
        "evidence_level":"NATIVE_EVERY_ORIGINAL_PAID_STEP_PLUS_FEEDBACK_AND_SOURCE_EDIT_REPLAY" if native else "INDEPENDENT_FROZEN_SOURCE_PREDICTION_ONLY",
        "source_sha":native["source_sha"] if native else None,
        "original_paid_instructions_checked":total_paid if native else 0,
        "first_divergence":None,"arms":evidence,
        "conclusion":"An actor can copy different bank-derived patch literals and stop quickly when one bounded repair fails; this is not a discovered general repair grammar."},indent=2))

if __name__=="__main__":
    main()
