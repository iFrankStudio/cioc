#!/usr/bin/env python3
"""Independent 66-original-world every-paid-step replay for G3Q-C1-I3.

Prospectively frozen native CIL source only; all64 initial finite-grammar
genotypes are reexecuted independently from published original Rust trace.
Source-paid 7/8/9 witness writes are checked with paid FULL reset before any
new certificate, and final structural truth is recomputed directly from the
three native meta-words, not from the actor's certificate or researcher summary.

This is a bounded 176-node one-attempt repair, not general mutation learning.
"""
import json
import os
import sys
from pathlib import Path

FROZEN=(Path(__file__).resolve().parents[1]/
    "studies/B56-F1d1k-G3Q-C1-P1-L3-I3-frozen-176-node-exhaustive-certificate-isolation-v1.json")
KEYS=(7,8,9,11,20,21,32,33,50,52,56,62,63,64)
CREDITS=8000
STUDY="B56-F1d1k-G3Q-C1-P1-L3-I3"
GOALS=(0,0,2,2)

def true_modes(a,b,c):
    outputs=[]
    for t in range(3):
        acc=0
        for i,raw in enumerate((a,b,c)):
            word=(raw+t)%4 if i==1 else raw
            acc=0 if word==3 else (acc+word)%3
        outputs.append(acc)
    return outputs

def replay_one(source,seed,feedback,disabled,expected,trace=None):
    a,b,c=seed
    original=[dict(x) for x in source]
    for pc,v in ((0,a),(3,b),(6,c)):
        original[pc]["value"]=v
    registers=[0,0,0,0]
    cells={}
    pc=0
    paid=0
    peak=len(source)+1
    witnesses=[]
    resets=[]
    passes=[]
    services=[]
    repairs=[]
    feedback_reads=[]
    meta_fetches=0
    flag_attempt_writes=0
    def native_rows():
        if trace is None:
            while True:
                yield None
        else:
            with Path(trace).open() as fp:
                for line in fp:
                    yield json.loads(line)
    rows=native_rows()
    def check(expected_row):
        actual=next(rows,None)
        if trace is not None and actual!=expected_row:
            raise AssertionError({"first_native_divergence_original_step":paid,
                "original_PC":expected_row[2] if len(expected_row)>2 else None,
                "native_row":actual,"independent_row":expected_row})
    while paid<CREDITS:
        inst=original[pc]
        at=pc
        paid+=1
        kind=inst["kind"]
        if kind=="Const":
            registers[inst["dst"]]=inst["value"]
            pc=inst["next"]
        elif kind in ("Add","Sub"):
            lhs,rhs=registers[inst["lhs"]],registers[inst["rhs"]]
            registers[inst["dst"]]=lhs+rhs if kind=="Add" else lhs-rhs
            pc=inst["next"]
        elif kind=="Read":
            addr=registers[inst["address"]]
            v=cells.get(addr,0)
            registers[inst["dst"]]=v
            if at==25:
                assert addr in (62,63,64)
                meta_fetches+=1
            if at==67:
                assert addr==20
                feedback_reads.append(v)
            pc=inst["next"]
        elif kind=="Write":
            key=registers[inst["address"]]
            value=registers[inst["value"]]
            cells[key]=value
            if at==118:
                assert key in (7,8,9) and value==1
                witnesses.append({"step":paid,"key":key,"value":value})
            if at in (166,168,170):
                assert value==0 and (at,key) in ((166,7),(168,8),(170,9))
                resets.append({"step":paid,"key":key,"value":value})
            if at==158:
                assert key==21 and value==1
                assert [cells.get(k) for k in (7,8,9)]==[1,1,1]
                passes.append(paid)
            if at==163:
                assert key==11 and value==1
                flag_attempt_writes+=1
            pc=inst["next"]
        elif kind=="Branch":
            pc=inst["nonzero"] if registers[inst["test"]] else inst["zero"]
        elif kind=="PatchConst":
            assert at==174
            node=registers[inst["node"]]
            desired=registers[inst["value"]]
            assert node==6 and desired==a
            assert not repairs and original[6]["kind"]=="Const"
            before=original[6]["value"]
            if not disabled:
                original[6]["value"]=desired
            repairs.append({"step":paid,"pc":174,"node":6,
                "before":before,"after":original[6]["value"],
                "edit_suppressed":disabled})
            pc=inst["next"]
        elif kind=="Halt":
            assert at in (86,109,175)
            pc=at
        else:
            raise AssertionError(("invalid frozen CIL opcode",at,kind))
        peak=max(peak,177+len(cells))
        assert len(cells)<=16 and peak<=300
        check(["cil",paid,at,pc,list(registers),CREDITS-paid,
            [cells.get(key) for key in KEYS],
            177+len(cells),original[6]["value"]])
        if at==59:
            assert kind=="Write" and key==11
            assert len(passes)==1,"actor served task before real paid certificate"
            episode=cells.get(32,0)
            assert 0<=episode<len(GOALS)
            truth=int(value==GOALS[episode])
            bit=truth if feedback=="truth" else 1-truth
            active_middle=cells[63]
            services.append({"step":paid,"episode":episode,"middle":active_middle,
                "output":value,"truth":truth,"feedback":bit})
            cells[20]=bit
            peak=max(peak,177+len(cells))
            check(["feedback",paid,episode,GOALS[episode],active_middle,
                value,truth,bit,
                [cells.get(key) for key in KEYS],177+len(cells),
                original[6]["value"]])
        if kind=="Halt":
            break
    else:
        raise AssertionError("original PID did not Halt before credit exhausted")
    if trace is not None:
        assert next(rows,None) is None,"native evidence contains trailing uncharged data"
    assert len(repairs)<=1
    assert len(passes)<=1
    assert len(repairs)==flag_attempt_writes
    assert len(resets)==3*len(repairs)
    assert feedback_reads==[x["feedback"] for x in services]
    assert meta_fetches==3*(len(witnesses)+len(services))
    modes=true_modes(a,b,original[6]["value"])
    truly_safe=len(set(modes))==3
    assert truly_safe==(len(passes)==1),"false structural certificate or false rejection"
    result={
      "steps":paid,"halt_pc":pc,"private_credits_remaining":CREDITS-paid,
      "final_source_rule":[a,b,original[6]["value"]],
      "actual_three_mode_outputs":modes,"structurally_safe":truly_safe,
      "certificate_passes":passes,"witnesses":witnesses,
      "reset_events":resets,"paid_meta_fetches":meta_fetches,
      "paid_feedback_reads":feedback_reads,
      "task_services":services,"source_edits":repairs,
      "actor_success":cells.get(21),"final_episode":cells.get(32),
      "used_cells":len(cells),"peak_occupied":peak}
    for k,v in result.items():
        if expected[k]!=v:
            raise AssertionError(("frozen prereg failed",seed,k,expected[k],v))
    return result

def main():
    frozen=json.loads(FROZEN.read_text())
    code=frozen["exact_original_native_CIL_source"]
    cases=frozen["all64_original_pid1_cases"]
    controls=frozen["adversarial_controls"]
    assert len(code)==176 and len(cases)==64 and len(controls)==2
    assert code[174]=={"kind":"PatchConst","node":1,"value":3,"next":0}
    native=None
    if len(sys.argv)==3:
        native=json.loads(Path(sys.argv[1]).read_text())
        assert native["study"]==STUDY
        assert native["level"]=="NATIVE_EXHAUSTIVE_ALL64_ORIGINAL_PID_PAID_WITNESS_ISOLATION"
        assert native["source_sha"] not in ("","local")
        if os.environ.get("GITHUB_SHA"):
            assert native["source_sha"]==os.environ["GITHUB_SHA"]
        assert len(native["all64_native_original_worlds"])==64
        assert len(native["adversarial_controls"])==2
        directory=Path(sys.argv[2])
    elif len(sys.argv)!=1:
        raise SystemExit("usage: python3 I3_REPLAY.py [NATIVE_SUMMARY.json EVIDENCE_DIR]")
    total_steps=0
    safely_certified=0
    repaired=0
    all_examples=[]
    for i,case in enumerate(cases):
        seed=case["seed"]
        assert i==(seed[0]*4+seed[1])*4+seed[2]
        name=f"a{seed[0]}_b{seed[1]}_c{seed[2]}"
        trace=(directory/f"b56_f1d1k_g3q_c1_p1_l3_i3_native_{name}.jsonl") if native else None
        result=replay_one(code,seed,"truth",False,case,trace)
        total_steps+=result["steps"]
        repaired+=int(bool(result["source_edits"]))
        safely_certified+=int(bool(result["certificate_passes"]))
        if native:
            n=native["all64_native_original_worlds"][i]
            assert n["arm"]==name and n["seed"]==seed
            assert n["paid_original_instructions"]==result["steps"]
            assert n["certified"]==result["structurally_safe"]
            assert n["independent_semantic_structural_safe"]==result["structurally_safe"]
            assert n["source_patches"]==result["source_edits"]
            assert n["paid_witness_reset_events"]==result["reset_events"]
            assert n["task_service_events"]==result["task_services"]
            assert n["true_goal_matches"]==sum(s["truth"] for s in result["task_services"])
            assert n["actor_declared_success"]==result["actor_success"]
            assert n["peak_occupied"]==result["peak_occupied"]
            assert n["original_PID1_halted"] is True and n["host_rule_source_edits"]==0
        if seed in ([0,2,1],[2,0,3],[1,1,2],[3,3,3]):
            all_examples.append({"seed":seed,"paid_steps":result["steps"],
                "true_rule_safe":result["structurally_safe"],
                "actor_approved":bool(result["certificate_passes"]),
                "repaired_rule":result["final_source_rule"],
                "patches":result["source_edits"]})
    assert safely_certified==30 and repaired==40
    control_results=[]
    for i,c in enumerate(controls):
        seed=[c["a"],c["b"],c["c"]]
        name=c["name"]
        trace=(directory/f"b56_f1d1k_g3q_c1_p1_l3_i3_native_{name}.jsonl") if native else None
        result=replay_one(code,seed,c["feedback"],c["disabled"],c["expected"],trace)
        total_steps+=result["steps"]
        if native:
            n=native["adversarial_controls"][i]
            assert n["arm"]==name and n["seed"]==seed
            assert n["paid_original_instructions"]==result["steps"]
            assert n["certified"]==result["structurally_safe"]
            assert n["source_patches"]==result["source_edits"]
            assert n["paid_witness_reset_events"]==result["reset_events"]
            assert n["task_service_events"]==result["task_services"]
            assert n["true_goal_matches"]==sum(s["truth"] for s in result["task_services"])
            assert n["actor_declared_success"]==result["actor_success"]
            assert n["original_PID1_halted"] is True
        control_results.append({"name":name,"paid_steps":result["steps"],
            "certified":bool(result["certificate_passes"]),
            "true_task_matches":sum(s["truth"] for s in result["task_services"]),
            "actor_success_claim":result["actor_success"]})
    assert control_results[0]["true_task_matches"]==0 and control_results[0]["actor_success_claim"]==1
    assert not control_results[1]["certified"] and control_results[1]["paid_steps"]<8000
    if native:
        assert native["counts"]=={"genotypes":64,"initial_safe":24,"initially_unsafe":40,
         "source_patches":40,"verified_structurally_safe_after_repair":30,
         "false_certificates":0,"false_rejections":0}
    print(json.dumps({
        "study":STUDY,
        "evidence_level":"NATIVE_66_WORLDS_EXACT_CHARGED_CIL_STEP_AND_FEEDBACK_REPLAY" if native else "SOURCE_ONLY_EXHAUSTIVE_64_GENOTYPE_PREDICTION",
        "source_sha":native["source_sha"] if native else None,
        "fully_replayed_original_paid_instructions":total_steps if native else 0,
        "all64_source_rules_scanned":64,
        "all64_actor_certified_count":safely_certified,
        "all64_actor_paid_edit_count":repaired,
        "false_certificates":0,"false_rejections":0,
        "first_divergence":None,
        "high_value_counterexamples":all_examples,
        "adversarial_controls":control_results,
        "limitations":"One actor per original world, researcher authored candidate and bank-copy repair; no general rule invention or semantic-truth oracle."
    },indent=2))

if __name__=="__main__":
    main()
