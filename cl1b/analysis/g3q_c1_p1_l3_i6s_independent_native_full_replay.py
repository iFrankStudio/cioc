#!/usr/bin/env python3
"""I6S — independent original-CIL interpreter for native two-patch worlds.

This observer reconstructs every charged CIL instruction and feedback event
for 64 freshly initialized PID1 genomes and 3 ablations. It uses the frozen
179-node source, NOT Rust instruction execution, as its program.
The second (middle3->0) repair was authored in advance, so the experiment is
a falsifiable two-edit and witness-isolation baseline, not evidence of
second-edit policy invention or learned mutation.
"""
import json
import os
import sys
from pathlib import Path

FREEZE=(Path(__file__).resolve().parents[1]/
    "studies/B56-F1d1k-G3Q-C1-P1-L3-I6S-frozen-180-node-two-edits-isolated-true-task-service-v1.json")
NAME="B56-F1d1k-G3Q-C1-P1-L3-I6S"
KEYS=(7,8,9,11,20,21,32,33,50,52,56,62,63,64)
CREDITS=8000
TASK_GOALS=(0,0,2,2)

def truth_modes(w):
    output=[]
    for t in range(3):
        value=0
        for i,raw in enumerate(w):
            opcode=(raw+t)%4 if i==1 else raw
            value=0 if opcode==3 else (value+opcode)%3
        output.append(value)
    return output

def replay_once(source,arm,logfile=None):
    seed=arm["seed"]
    code=[dict(x) for x in source]
    for at,value in zip((0,3,6),seed):
        code[at]["value"]=value
    p=0
    reg=[0,0,0,0]
    cells={}
    paid=0
    peak=len(code)+1
    meta=0
    feedback_reads=[]
    witnesses=[]
    resets=[]
    passes=[]
    services=[]
    patches=[]
    branches=[]
    guards=[]
    guard_writes=[]
    feedback_mode=arm.get("feedback","truth")
    suppressed=arm.get("suppressed",False)

    def rows():
        if logfile is None:
            while True:
                yield None
        else:
            with Path(logfile).open() as stream:
                for line in stream:
                    yield json.loads(line)
    events=rows()
    def verify(predicted):
        observed=next(events,None)
        if logfile is not None and observed!=predicted:
            raise AssertionError({
                "original_arm":seed,
                "first_native_divergence_at_paid_step":paid,
                "native_original":observed,
                "source_independent":predicted,
            })

    while paid<CREDITS:
        op=code[p]
        at=p
        paid+=1
        kind=op["kind"]
        if kind=="Const":
            reg[op["dst"]]=op["value"]
            p=op["next"]
        elif kind in ("Add","Sub"):
            a,b=reg[op["lhs"]],reg[op["rhs"]]
            reg[op["dst"]]=a+b if kind=="Add" else a-b
            p=op["next"]
        elif kind=="Read":
            key=reg[op["address"]]
            reg[op["dst"]]=cells.get(key,0)
            if at==24:
                assert key in (62,63,64)
                meta+=1
            if at==62:
                assert key==20
                feedback_reads.append(reg[op["dst"]])
            if at==149:
                assert key==11
                guards.append({"step":paid,"value":reg[0]})
            p=op["next"]
        elif kind=="Write":
            key=reg[op["address"]]
            value=reg[op["value"]]
            cells[key]=value
            if at==107:
                assert key in (7,8,9) and value==1
                witnesses.append({"step":paid,"key":key,"value":value})
            if at in (155,157,159):
                assert (at,key,value) in ((155,7,0),(157,8,0),(159,9,0))
                resets.append({"step":paid,"key":key,"value":value})
            if at==147:
                assert key==21 and value==1
                assert [cells.get(k) for k in (7,8,9)]==[1,1,1]
                passes.append(paid)
            if at in (152,177):
                assert key==11 and value==(1 if at==152 else 2)
                guard_writes.append({"step":paid,"key":11,"value":value})
            p=op["next"]
        elif kind=="Branch":
            if at in (150,162,168,174,178):
                cond=reg[op["test"]]
                target=op["nonzero"] if cond else op["zero"]
                branches.append({"step":paid,"pc":at,"condition":cond,"target":target})
            p=op["nonzero"] if reg[op["test"]] else op["zero"]
        elif kind=="PatchConst":
            assert at in (169,171)
            assert len(patches)<2
            node=reg[op["node"]]
            requested=reg[op["value"]]
            assert node in (3,6) and code[node]["kind"]=="Const"
            assert len(resets)==3*(len(patches)+1), (
                "MISSING FULL PAID FLAG RESET BEFORE ORIGINAL SOURCE EDIT",seed,paid,at)
            assert [cells.get(k) for k in (7,8,9)]==[0,0,0], (
                "STALE ORIGINAL CERTIFICATE EVIDENCE",seed,paid,at)
            before=code[node]["value"]
            if not suppressed:
                code[node]["value"]=requested
            patches.append({"step":paid,"pc":at,"node":node,
                "before":before,"after":code[node]["value"],
                "edit_suppressed":suppressed})
            p=op["next"]
        elif kind=="Halt":
            assert code[at]["kind"]=="Halt"
            p=at
        else:
            raise AssertionError(("unknown original frozen CIL kind",at,kind))

        peak=max(peak,len(code)+1+len(cells))
        assert len(cells)<=16 and peak<=300
        verify(["cil",paid,at,p,list(reg),CREDITS-paid,
            [cells.get(k) for k in KEYS],len(code)+1+len(cells),
            code[3]["value"],code[6]["value"]])

        if at==54:
            assert kind=="Write" and key==11 and len(passes)==1
            episode=cells.get(32,0)
            assert 0<=episode<4
            goal=TASK_GOALS[episode]
            truth=int(value==goal)
            assert feedback_mode in ("truth","inverted")
            delivered=truth if feedback_mode=="truth" else 1-truth
            middle=cells[63]
            services.append({"step":paid,"episode":episode,
                "middle":middle,"output":value,
                "truth":truth,"feedback":delivered})
            cells[20]=delivered
            peak=max(peak,len(code)+1+len(cells))
            verify(["feedback",paid,episode,goal,middle,
                value,truth,delivered,[cells.get(k) for k in KEYS],
                len(code)+1+len(cells),code[3]["value"],code[6]["value"]])
        if kind=="Halt":
            break
    else:
        raise AssertionError(("PID1 never paid for Halt before credit budget",seed))

    if logfile is not None:
        assert next(events,None) is None,"native record contains rows past paid Halt"
    assert len(patches)<=2
    assert len(passes)<=1
    assert len(resets)==3*len(patches)
    assert len(witnesses)==3*(1+len(patches))
    assert meta==3*(len(witnesses)+len(services))
    assert feedback_reads==[event["feedback"] for event in services]
    assert len(guards)==1+len(patches)-int(bool(passes))
    assert [v["value"] for v in guard_writes]==list(range(1,len(patches)+1))

    final=[code[0]["value"],code[3]["value"],code[6]["value"]]
    modes=truth_modes(final)
    covered=len(set(modes))==3
    assert (len(passes)==1)==covered,("false semantic structural certificate",seed)
    result={
        "paid_steps":paid,"halt_pc":p,"credits_remaining":CREDITS-paid,
        "source_after":final,"semantic_three_modes":modes,
        "semantic_structural_coverage":covered,
        "witness_marks":witnesses,"witness_resets":resets,
        "certificate_pass_steps":passes,"real_task_services":services,
        "paid_source_edits":patches,"actor_decisions":branches,
        "retry_guard_writes":guard_writes,"paid_guard_reads":guards,
        "paid_meta_reads":meta,"paid_feedback_reads":feedback_reads,
        "actor_claimed_success":cells.get(21),
        "actor_completed_episodes":cells.get(32),
        "used_cells":len(cells),"peak_occupied":peak,
    }
    expected=arm["expected"]
    for k,v in result.items():
        prediction_keys={
            "source_after":"final_rule",
            "semantic_three_modes":"semantic_modes",
            "semantic_structural_coverage":"semantic_coverage",
            "witness_marks":"paid_witnesses",
            "witness_resets":"paid_resets",
            "certificate_pass_steps":"certificate",
            "real_task_services":"task_service",
            "paid_source_edits":"source_edits",
            "actor_decisions":"paid_decisions",
            "retry_guard_writes":"paid_retry_writes",
            "paid_meta_reads":"meta_Read_count",
            "actor_claimed_success":"actor_claim",
            "actor_completed_episodes":"actor_episode",
        }
        if expected[prediction_keys.get(k,k)]!=v:
            raise AssertionError(("frozen I6S prediction differs from independent source interpreter",seed,k,expected[prediction_keys.get(k,k)],v))
    return result

def main():
    frozen=json.loads(FREEZE.read_text())
    source=frozen["original_native_CIL_source"]
    cases=frozen["all64_original_world_cases"]
    controls=frozen["controls"]
    assert len(source)==180 and len(cases)==64 and len(controls)==3
    assert source[150]["nonzero"]==172
    assert source[171]=={"kind":"PatchConst","node":1,"value":0,"next":0}
    native=None
    if len(sys.argv)==3:
        native=json.loads(Path(sys.argv[1]).read_text())
        assert native["study"]==NAME
        assert native["level"]=="NATIVE_ORIGINAL_PID_180_NODE_BOUNDED_TWO_EDITS_FULL_WITNESS_ISOLATION"
        assert native["source_sha"] not in ("","local")
        if os.environ.get("GITHUB_SHA"):
            assert native["source_sha"]==os.environ["GITHUB_SHA"]
        assert len(native["original64_worlds"])==64
        assert len(native["controls"])==3
        directory=Path(sys.argv[2])
    elif len(sys.argv)!=1:
        raise SystemExit("usage: python3 I6S_REPLAY.py [NATIVE_SUMMARY.json EVIDENCE_DIR]")

    total_paid=0
    source_edit_one=0
    source_edit_two=0
    incomplete=0
    repairs=[]
    for i,arm in enumerate(cases):
        seed=arm["seed"]
        assert i==(seed[0]*4+seed[1])*4+seed[2]
        name=f"a{seed[0]}_b{seed[1]}_c{seed[2]}"
        trace=(directory/f"b56_f1d1k_g3q_c1_p1_l3_i6s_original_{name}.jsonl") if native else None
        result=replay_once(source,arm,trace)
        total_paid+=result["paid_steps"]
        if not arm["initial_structurally_safe"]:
            incomplete+=1
            source_edit_one+=int(len(result["paid_source_edits"])==1)
            source_edit_two+=int(len(result["paid_source_edits"])==2)
            assert result["semantic_structural_coverage"]
        if native:
            n=native["original64_worlds"][i]
            expected_native={
                "arm":name,"seed":seed,"paid_original_steps":result["paid_steps"],
                "original_PID1_naturally_halted":True,
                "original_PID1_source_edits":result["paid_source_edits"],
                "native_certified":result["semantic_structural_coverage"],
                "semantic_modes":result["semantic_three_modes"],
                "paid_retry_guard_events":result["retry_guard_writes"],
                "paid_full_witness_reset_events":result["witness_resets"],
                "real_task_services":result["real_task_services"],
                "original_meta_reads":result["paid_meta_reads"],
                "peak_occupied":result["peak_occupied"],
                "final_source_rule":result["source_after"],
                "postload_host_source_writes":0,
                "actor_claimed_success":result["actor_claimed_success"],
            }
            for k,v in expected_native.items():
                assert n[k]==v,(name,"native summary contradiction",k,n[k],v)
        if len(result["paid_source_edits"])==2:
            assert result["paid_source_edits"][1]["node"]==3
            assert [e["value"] for e in result["retry_guard_writes"]]==[1,2]
            assert len(result["witness_resets"])==6
            assert len(result["witness_marks"])==9
            assert result["real_task_services"]
            repairs.append({
                "seed":seed,"first_charged_patch":result["paid_source_edits"][0],
                "second_charged_patch":result["paid_source_edits"][1],
                "full_paid_certificate_pass_step":result["certificate_pass_steps"][0],
                "paid_task_service_events":len(result["real_task_services"]),
                "first_native_divergence":None,
            })
    assert (incomplete,source_edit_one,source_edit_two)==(40,32,8)
    assert len(repairs)==8
    control_results=[]
    for i,ctrl in enumerate(controls):
        name=ctrl["name"]
        trace=(directory/f"b56_f1d1k_g3q_c1_p1_l3_i6s_original_{name}.jsonl") if native else None
        result=replay_once(source,ctrl,trace)
        total_paid+=result["paid_steps"]
        if native:
            n=native["controls"][i]
            assert n["arm"]==name and n["seed"]==ctrl["seed"]
            assert n["paid_original_steps"]==result["paid_steps"]
            assert n["original_PID1_source_edits"]==result["paid_source_edits"]
            assert n["native_certified"]==result["semantic_structural_coverage"]
            assert n["real_task_services"]==result["real_task_services"]
            assert n["original_PID1_naturally_halted"] is True
        if ctrl["suppressed"]:
            assert not result["semantic_structural_coverage"]
            assert not result["real_task_services"]
        control_results.append({"arm":name,
            "paid_original_steps":result["paid_steps"],
            "actual_executable_patches":result["paid_source_edits"],
            "structural_certificate":result["semantic_structural_coverage"],
            "task_probes":len(result["real_task_services"]),
            "semantic_task_successes":sum(x["truth"] for x in result["real_task_services"]),
            "actor_success_claim":result["actor_claimed_success"],
            "first_divergence":None})
    if native:
        assert native["counts"]=={
            "initial_safe":24,"initial_unsafe":40,
            "one_edit_rescued":32,"two_edit_rescued":8,
            "all64_structurally_certified":64,
            "false_certifications":0,"false_rejections":0,
        }
    print(json.dumps({
        "study":NAME,
        "evidence_level":"NATIVE_67_ORIGINAL_WORLDS_ALL_CHARGED_CIL_STEPS_AND_FEEDBACK_REPLAY" if native else "FROZEN_SOURCE_ONLY_PRE_NATIVE_PREDICTIONS",
        "source_sha":native["source_sha"] if native else None,
        "original_paid_steps_independently_replayed":total_paid if native else 0,
        "original_genotype_cases":64,"original_actor_one_edit_success":32,
        "original_actor_two_edit_success":8,
        "all64_structurally_certified":True,
        "all64_false_certificates":0,"all64_false_rejections":0,
        "single_original_PID_per_run":True,
        "first_divergence":None,
        "hard_cases":repairs,"controls":control_results,
        "limits":"The second edit is a pre-authored fixed middle word3->0. This does not demonstrate selecting the second source repair from new evidence, inventing a repair policy, or open-ended evolution.",
    },indent=2))

if __name__=="__main__":
    main()
