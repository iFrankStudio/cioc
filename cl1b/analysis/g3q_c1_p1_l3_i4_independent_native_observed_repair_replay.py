#!/usr/bin/env python3
"""I4 — source-independent original PID1 finite-CIL interpreter and full trace auditor.

Re-executes all 64 frozen three-word founder genotypes plus 3 preregistered
controls. It sees the original native charged Rust snapshots only as evidence,
never as instructions. All instruction PCs, registers, private compute,
cell presence/values, selected executable patch address, edited Const3/Const6,
and externally injected task feedback are independently reconstructed.

Structural truth is separately calculated from the actual final 3-word
meta-rule grammar; a successful actor certificate is not treated as an oracle.
The algorithm is author-written, bounded and not claimed to have been invented
by the digital individual.
"""
import json
import os
import sys
from pathlib import Path

SOURCE=(Path(__file__).resolve().parents[1]/
    "studies/B56-F1d1k-G3Q-C1-P1-L3-I4-frozen-180-node-observation-selected-repair-v1.json")
STUDY="B56-F1d1k-G3Q-C1-P1-L3-I4"
KEYS=(7,8,9,11,20,21,32,33,50,52,56,62,63,64)
CREDITS=8000
GOALS=(0,0,2,2)

def semantic_modes(genes):
    a,b,c=genes
    result=[]
    for t in range(3):
        acc=0
        for idx,original in enumerate((a,b,c)):
            opcode=(original+t)%4 if idx==1 else original
            acc=0 if opcode==3 else (acc+opcode)%3
        result.append(acc)
    return result

def paid_original_pid_replay(template,seed,feedback_regime,suppressed,expected,trace=None):
    code=[dict(inst) for inst in template]
    for pc,val in ((0,seed[0]),(3,seed[1]),(6,seed[2])):
        code[pc]["value"]=val
    r=[0,0,0,0]
    cells={}
    pc=0
    paid=0
    peak=181
    witnesses=[]
    resets=[]
    certificates=[]
    services=[]
    edits=[]
    feedback_reads=[]
    meta_fetches=0
    guard_writes=0
    chosen=None

    def native_rows():
        if trace is None:
            while True: yield None
        else:
            with Path(trace).open() as fh:
                for line in fh:
                    yield json.loads(line)
    rows=native_rows()

    def audit(row):
        observed=next(rows,None)
        if trace is not None and observed!=row:
            raise AssertionError({
                "first_native_divergence_at_paid_step":paid,
                "expected_original_PC":row[2] if len(row)>2 else None,
                "observed_native":observed,
                "independent_source_replay":row,
            })

    while paid<CREDITS:
        op=code[pc]
        here=pc
        paid+=1
        kind=op["kind"]
        if kind=="Const":
            r[op["dst"]]=op["value"]
            pc=op["next"]
        elif kind in ("Add","Sub"):
            lhs,rhs=r[op["lhs"]],r[op["rhs"]]
            r[op["dst"]]=lhs+rhs if kind=="Add" else lhs-rhs
            pc=op["next"]
        elif kind=="Read":
            address=r[op["address"]]
            value=cells.get(address,0)
            r[op["dst"]]=value
            if here==25:
                assert address in (62,63,64)
                meta_fetches+=1
            if here==67:
                assert address==20
                feedback_reads.append(value)
            pc=op["next"]
        elif kind=="Write":
            address=r[op["address"]]
            value=r[op["value"]]
            cells[address]=value
            if here==118:
                assert address in (7,8,9) and value==1
                witnesses.append({"step":paid,"key":address,"value":value})
            if here in (166,168,170):
                assert (here,address,value) in ((166,7,0),(168,8,0),(170,9,0))
                resets.append({"step":paid,"key":address,"value":value})
            if here==158:
                assert address==21 and value==1
                assert [cells.get(k) for k in (7,8,9)]==[1,1,1]
                certificates.append(paid)
            if here==163:
                assert address==11 and value==1
                guard_writes+=1
            pc=op["next"]
        elif kind=="Branch":
            if here==173:
                assert chosen is None
                assert r[1] in (0,1)
                target=3 if r[1] else 6
                chosen={"step":paid,"register1":r[1],"chosen_node":target}
                assert not certificates,"original PID did not need patch after success"
                assert len(witnesses)==3
                seen=len({v["key"] for v in witnesses})
                assert seen in (1,2)
                assert (seen==1 and target==6) or (seen==2 and target==3), (
                    "native repair selection was not causally justified by witness marks",
                    seen,target)
            pc=op["nonzero"] if r[op["test"]] else op["zero"]
        elif kind=="PatchConst":
            assert here in (177,179)
            assert chosen is not None and not edits
            node=r[op["node"]]
            wanted=r[op["value"]]
            assert node==chosen["chosen_node"] and node in (3,6)
            assert node==(3 if here==179 else 6)
            assert code[node]["kind"]=="Const"
            prior=code[node]["value"]
            if not suppressed:
                code[node]["value"]=wanted
            edits.append({"step":paid,"pc":here,"node":node,
                "before":prior,"after":code[node]["value"],
                "edit_suppressed":suppressed})
            pc=op["next"]
        elif kind=="Halt":
            assert here in (94,109,175)
            pc=here
        else:
            raise AssertionError(("source has unsupported CIL opcode",here,kind))

        peak=max(peak,181+len(cells))
        assert len(cells)<=16 and peak<=300
        audit(["cil",paid,here,pc,list(r),CREDITS-paid,
            [cells.get(key) for key in KEYS],181+len(cells),
            code[3]["value"],code[6]["value"]])

        if here==59:
            assert kind=="Write" and address==11
            assert len(certificates)==1, "native task service without original paid certificate"
            episode=cells.get(32,0)
            assert 0<=episode<len(GOALS)
            truth=int(value==GOALS[episode])
            assert feedback_regime in ("truth","inverted")
            bit=truth if feedback_regime=="truth" else 1-truth
            middle=cells[63]
            services.append({"step":paid,"episode":episode,"middle":middle,
                "output":value,"truth":truth,"feedback":bit})
            cells[20]=bit
            peak=max(peak,181+len(cells))
            audit(["feedback",paid,episode,GOALS[episode],middle,
                value,truth,bit,[cells.get(key) for key in KEYS],
                181+len(cells),code[3]["value"],code[6]["value"]])

        if kind=="Halt":
            break
    else:
        raise AssertionError("original source never reached paid Halt before private budget exhausted")
    if trace is not None:
        assert next(rows,None) is None, "original native trace has extra rows after Halt"

    assert len(certificates)<=1 and len(edits)<=1
    assert len(edits)==guard_writes
    assert len(resets)==3*len(edits)
    assert len(witnesses)==3*(1+int(bool(edits)))
    assert meta_fetches==3*(len(witnesses)+len(services))
    assert feedback_reads==[v["feedback"] for v in services]

    final=[code[0]["value"],code[3]["value"],code[6]["value"]]
    modes=semantic_modes(final)
    safe=len(set(modes))==3
    assert safe==(len(certificates)==1), "false-positive or false-negative coverage certification"
    result={
        "paid_steps":paid,"halt_pc":pc,
        "private_credits_remaining":CREDITS-paid,
        "source_rule_after":final,"actual_mode_sequence":modes,
        "actual_three_mode_coverage":safe,
        "certificate_pass_steps":certificates,
        "paid_witness_marks":witnesses,
        "paid_witness_resets":resets,
        "original_source_edits":edits,
        "chosen_repair_provenance":chosen,
        "original_paid_meta_fetches":meta_fetches,
        "original_paid_feedback_reads":feedback_reads,
        "original_paid_services":services,
        "original_paid_retry_guard_writes":guard_writes,
        "actor_declared_success":cells.get(21),
        "actor_final_episode":cells.get(32),
        "occupied_data_cells":len(cells),
        "peak_occupied_units":peak,
    }
    for k,v in result.items():
        if v!=expected[k]:
            raise AssertionError(("precommitted source prediction mismatch",seed,k,v,expected[k]))
    return result

def main():
    frozen=json.loads(SOURCE.read_text())
    source=frozen["self_modified_original_CIL_source"]
    cases=frozen["precommitted_all64_original_pid1_arms"]
    controls=frozen["precommitted_controls"]
    assert len(source)==180 and len(cases)==64 and len(controls)==3
    assert source[177]==source[179]=={"kind":"PatchConst","node":1,"value":3,"next":0}
    native=None
    if len(sys.argv)==3:
        native=json.loads(Path(sys.argv[1]).read_text())
        assert native["study"]==STUDY
        assert native["level"]=="NATIVE_ORIGINAL_PID_EVIDENCE_CONDITIONED_EDIT_TARGET_EXHAUSTIVE64"
        assert native["source_sha"] not in ("","local")
        if os.environ.get("GITHUB_SHA"):
            assert native["source_sha"]==os.environ["GITHUB_SHA"]
        assert len(native["original64_native_arms"])==64
        assert len(native["matched_native_controls"])==3
        directory=Path(sys.argv[2])
    elif len(sys.argv)!=1:
        raise SystemExit("usage: python3 I4_REPLAY.py [NATIVE_SUMMARY.json EVIDENCE_FOLDER]")

    total=0
    rescued=0
    targets={3:0,6:0}
    outcomes=[]
    for ordinal,case in enumerate(cases):
        seed=case["seed"]
        assert ordinal==(seed[0]*4+seed[1])*4+seed[2]
        name=f"a{seed[0]}_b{seed[1]}_c{seed[2]}"
        trace=(directory/f"b56_f1d1k_g3q_c1_p1_l3_i4_original_paid_{name}.jsonl") if native else None
        r=paid_original_pid_replay(source,seed,"truth",False,case["expected"],trace)
        total+=r["paid_steps"]
        rescued+=int(not case["initial_structurally_safe"] and r["actual_three_mode_coverage"])
        for edit in r["original_source_edits"]:
            targets[edit["node"]]+=1
        if native:
            observed=native["original64_native_arms"][ordinal]
            expected_native={
                "arm":name,"initial_rule":seed,
                "original_paid_steps":r["paid_steps"],
                "original_task_services":r["original_paid_services"],
                "original_paid_source_edits":r["original_source_edits"],
                "native_original_PID1_Halted":True,
                "native_certified":r["actual_three_mode_coverage"],
                "semantic_rule_modes":r["actual_mode_sequence"],
                "choice_provenance":r["chosen_repair_provenance"],
                "original_paid_witness_resets":r["paid_witness_resets"],
                "peak_occupied":r["peak_occupied_units"],
                "postload_host_source_edits":0,
                "true_task_matches":sum(v["truth"] for v in r["original_paid_services"]),
                "actor_declared_success":r["actor_declared_success"],
            }
            for k,v in expected_native.items():
                assert observed[k]==v,(name,"native summary divergence",k,observed[k],v)
        if seed in ([0,0,3],[0,2,0],[0,2,3],[1,1,0],[2,0,3],[3,3,3]):
            outcomes.append({"seed":seed,"original_paid_steps":r["paid_steps"],
                "selected_source_node":r["original_source_edits"][0]["node"] if r["original_source_edits"] else None,
                "final_rule":r["source_rule_after"],
                "original_semantic_coverage":r["actual_three_mode_coverage"],
                "actor_certificate":len(r["certificate_pass_steps"])==1,
                "first_divergence":None})

    assert rescued==30 and targets=={3:24,6:16}
    control_results=[]
    for ordinal,control in enumerate(controls):
        name=control["name"]
        seed=[control["first"],control["middle"],control["tail"]]
        trace=(directory/f"b56_f1d1k_g3q_c1_p1_l3_i4_original_paid_{name}.jsonl") if native else None
        r=paid_original_pid_replay(source,seed,control["feedback"],
            control["disabled"],control["expected"],trace)
        total+=r["paid_steps"]
        if native:
            observed=native["matched_native_controls"][ordinal]
            assert observed["arm"]==name and observed["initial_rule"]==seed
            assert observed["original_paid_steps"]==r["paid_steps"]
            assert observed["original_paid_source_edits"]==r["original_source_edits"]
            assert observed["native_certified"]==r["actual_three_mode_coverage"]
            assert observed["choice_provenance"]==r["chosen_repair_provenance"]
            assert observed["original_task_services"]==r["original_paid_services"]
            assert observed["actor_declared_success"]==r["actor_declared_success"]
            assert observed["native_original_PID1_Halted"] is True
        if control["disabled"]:
            assert not r["actual_three_mode_coverage"] and len(r["original_paid_services"])==0
        else:
            assert r["actual_three_mode_coverage"]
            assert r["actor_declared_success"]==1
        control_results.append({"arm":name,
            "paid_original_instructions":r["paid_steps"],
            "patch":r["original_source_edits"],
            "paid_task_probes":len(r["original_paid_services"]),
            "true_task_matches":sum(x["truth"] for x in r["original_paid_services"]),
            "actor_claimed_success":r["actor_declared_success"],
            "first_divergence":None})

    if native:
        assert native["counts"]=={
            "initial_safe":24,"initial_unsafe":40,"self_edits":40,
            "chosen_tail":16,"chosen_middle":24,"repaired_to_safe":30,
            "final_safe":54,"false_certificates":0,"false_rejections":0}
    print(json.dumps({
        "study":STUDY,
        "evidence_level":"NATIVE_67_ORIGINAL_WORLDS_EVERY_CHARGED_CIL_STEP_SOURCE_CHOICE_AND_FEEDBACK_REPLAY" if native else "FROZEN_SOURCE_ONLY_EXHAUSTIVE_PREDICTIONS",
        "source_sha":native["source_sha"] if native else None,
        "original_paid_steps_independently_checked":total if native else 0,
        "original_genotypes":64,"initially_unsafe":40,
        "original_actor_paid_edits":40,
        "conditional_edit_target_counts":{"source_PC3_middle":24,"source_PC6_tail":16},
        "actual_successful_repairs":rescued,
        "verified_I3_native_rescue_baseline":6,
        "rescue_multiplier":5.0,
        "false_certificates":0,
        "false_rejections":0,
        "native_first_divergence":None,
        "critical_counterexamples":outcomes,
        "adversarial_controls":control_results,
        "limits":"Two-branch researcher-authored evidence-conditioned source edit policy; no novel repair-algorithm invention, persistent lineage or semantically trusted task oracle."
    },indent=2))

if __name__=="__main__":
    main()
