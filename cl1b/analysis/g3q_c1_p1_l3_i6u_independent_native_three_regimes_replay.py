#!/usr/bin/env python3
"""I6U — independent original-CIL interpreter for native two-patch worlds.

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
    "studies/B56-F1d1k-G3Q-C1-P1-L3-I6U-frozen-180-node-bounded-task-policy-search-v1.json")
NAME="B56-F1d1k-G3Q-C1-P1-L3-I6U"
KEYS=(7,8,9,11,20,21,32,33,50,52,56,62,63,64)
CREDITS=8000
TASK_GOALS=None  # each preregistered arm owns the hidden task-goal sequence

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
    goals=arm["goals"]
    assert len(goals)==4 and set(goals)<=set((0,1,2))
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
            goal=goals[episode]
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
            raise AssertionError(("frozen I6U prediction differs from independent source interpreter",seed,k,expected[prediction_keys.get(k,k)],v))
    return result

def main():
    frozen=json.loads(FREEZE.read_text())
    source=frozen["exact_original_native_CIL"]
    cases=frozen["pre_registered_original_regime_arms"]
    controls=frozen["pre_registered_controls"]
    regimes=frozen["task_regimes"]
    assert len(source)==180 and len(cases)==192 and len(controls)==3 and len(regimes)==3
    assert source[82]=={"kind":"Const","dst":2,"value":4,"next":83}
    assert source[88]=={"kind":"Sub","dst":3,"lhs":3,"rhs":2,"next":89}
    assert source[89]=={"kind":"Const","dst":2,"value":-1,"next":90}
    assert source[92]=={"kind":"Const","dst":3,"value":2,"next":93}
    parent_path=Path(__file__).resolve().parents[1]/"studies/B56-F1d1k-G3Q-C1-P1-L3-I6S-frozen-180-node-two-edits-isolated-true-task-service-v1.json"
    parent=json.loads(parent_path.read_text())["original_native_CIL_source"]
    assert [i for i,(a,b) in enumerate(zip(source,parent)) if a!=b]==[82,88,89,92]
    assert len(parent)==len(source)==180
    native=None
    if len(sys.argv)==3:
        native=json.loads(Path(sys.argv[1]).read_text())
        assert native["study"]==NAME
        assert native["level"]=="NATIVE_ORIGINAL_PID_180_NODE_TASK_POLICY_3_REGIMES_192_WORLDS"
        assert native["source_sha"] not in ("","local")
        if os.environ.get("GITHUB_SHA"):
            assert native["source_sha"]==os.environ["GITHUB_SHA"]
        assert len(native["original192_worlds"])==192
        assert len(native["controls"])==3
        assert native["regimes"]==regimes
        directory=Path(sys.argv[2])
    elif len(sys.argv)!=1:
        raise SystemExit("usage: python3 I6U_REPLAY.py [NATIVE_SUMMARY.json EVIDENCE_DIR]")

    total_paid=0
    initially_safe=0
    initial_unsafe=0
    one_edit=0
    two_edits=0
    task_attempts=0
    true_outputs=0
    focus=[]
    regime_summary=[]
    for rno,regime in enumerate(regimes):
        subset=cases[rno*64:(rno+1)*64]
        assert all(arm["regime"]==regime["id"] and arm["goals"]==regime["goals"] for arm in subset)
        r_paid=r_attempts=r_true=r_finished=0
        for ordinal,arm in enumerate(subset):
            i=rno*64+ordinal
            seed=arm["seed"]
            assert ordinal==(seed[0]*4+seed[1])*4+seed[2]
            name=f'{regime["id"]}_a{seed[0]}_b{seed[1]}_c{seed[2]}'
            path=(directory/f"b56_f1d1k_g3q_c1_p1_l3_i6u_original_{name}.jsonl") if native else None
            result=replay_once(source,arm,path)
            total_paid+=result["paid_steps"]
            r_paid+=result["paid_steps"]
            assert result["semantic_structural_coverage"] and len(result["certificate_pass_steps"])==1
            assert result["actor_claimed_success"]==1
            assert result["actor_completed_episodes"]==4
            r_finished+=1
            nedit=len(result["paid_source_edits"])
            if arm["initial_structurally_safe"]:
                assert nedit==0
                initially_safe+=1
            else:
                initial_unsafe+=1
                assert nedit in (1,2)
                one_edit+=int(nedit==1)
                two_edits+=int(nedit==2)
            assert len(result["witness_resets"])==3*nedit
            assert len(result["witness_marks"])==3*(1+nedit)
            task_attempts+=len(result["real_task_services"])
            true_outputs+=sum(x["truth"] for x in result["real_task_services"])
            r_attempts+=len(result["real_task_services"])
            r_true+=sum(x["truth"] for x in result["real_task_services"])
            if native:
                n=native["original192_worlds"][i]
                expected_native={
                    "arm":name,"seed":seed,
                    "regime":regime["id"],"goals":regime["goals"],
                    "paid_original_steps":result["paid_steps"],
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
                    "actor_completed_episode":result["actor_completed_episodes"],
                }
                for k,v in expected_native.items():
                    assert n[k]==v,(name,"native summary contradiction",k,n[k],v)
            if seed in ([1,3,0],[1,3,1],[1,3,2],[1,3,3],[0,2,3],[3,3,3]):
                focus.append({"regime":regime["id"],"seed":seed,"paid":result["paid_steps"],
                    "original_source_edits":result["paid_source_edits"],
                    "task_service_attempts":len(result["real_task_services"]),
                    "truthful_task_matches":sum(x["truth"] for x in result["real_task_services"]),
                    "completed_all_four_stages":True,"first_divergence":None})
        expected_regime=frozen["pre_registered_counts"]["per_regime"][rno]
        assert (r_paid,r_attempts,r_true,r_finished)==(
            expected_regime["paid_charged_steps"],
            expected_regime["task_service_probes"],
            expected_regime["true_feedbacks"],
            expected_regime["full_task_completions"])
        regime_summary.append({"regime":regime["id"],"goals":regime["goals"],
            "original64_worlds":64,"all_four_stage_completions":r_finished,
            "actual_paid_task_service_attempts":r_attempts,
            "truthful_task_matches":r_true,"actual_paid_original_instructions":r_paid})

    assert (initially_safe,initial_unsafe,one_edit,two_edits)==(72,120,96,24)
    controls_output=[]
    for i,arm in enumerate(controls):
        name=arm["name"]
        path=(directory/f"b56_f1d1k_g3q_c1_p1_l3_i6u_original_{name}.jsonl") if native else None
        result=replay_once(source,arm,path)
        total_paid+=result["paid_steps"]
        if native:
            n=native["controls"][i]
            assert n["arm"]==name and n["seed"]==arm["seed"]
            assert n["regime"]==arm["regime"] and n["goals"]==arm["goals"]
            assert n["paid_original_steps"]==result["paid_steps"]
            assert n["original_PID1_source_edits"]==result["paid_source_edits"]
            assert n["native_certified"]==result["semantic_structural_coverage"]
            assert n["real_task_services"]==result["real_task_services"]
            assert n["actor_completed_episode"]==result["actor_completed_episodes"]
            assert n["original_PID1_naturally_halted"] is True
        if arm["suppressed"]:
            assert not result["semantic_structural_coverage"]
            assert result["real_task_services"]==[]
        else:
            assert result["actor_claimed_success"]==1
            assert sum(x["truth"] for x in result["real_task_services"])<4
        controls_output.append({
            "name":name,"paid":result["paid_steps"],
            "source_edit_events":result["paid_source_edits"],
            "structurally_certified":result["semantic_structural_coverage"],
            "true_task_matches":sum(x["truth"] for x in result["real_task_services"]),
            "actor_success_claim":result["actor_claimed_success"],
            "first_divergence":None})
    metrics=frozen["pre_registered_counts"]
    assert (total_paid,task_attempts,true_outputs)==(
        metrics["paid_including_controls"],
        metrics["task_service_attempts"],
        metrics["true_task_matches"])
    if native:
        assert native["counts"]=={
            "regimes":3,"original_genotype_worlds":192,
            "initial_safe":72,"one_edit_rescued":96,"two_edit_rescued":24,
            "all_structurally_certified":192,"all_four_goal_task_completed":192,
            "task_service_attempts":task_attempts,"true_task_matches":true_outputs,
            "false_certificates":0,"false_rejections":0}
    print(json.dumps({
        "study":NAME,
        "evidence_level":"NATIVE_195_ORIGINAL_WORLDS_ALL_CHARGED_STEPS_3_TASK_REGIMES" if native else "FROZEN_SOURCE_ONLY_192_WORLDS_3_TASK_REGIMES",
        "source_sha":native["source_sha"] if native else None,
        "original_paid_steps_independently_replayed":total_paid if native else 0,
        "native_original_worlds":195 if native else 0,
        "original_regime_worlds":192,
        "true_paid_task_service_attempts":task_attempts,
        "true_task_matches":true_outputs,
        "full_four_goal_task_completions":192,
        "one_edit_repairs":96,"two_edit_repairs":24,
        "all_structurally_certified":192,
        "false_certificates":0,"false_rejections":0,
        "first_native_divergence":None,
        "regime_summary":regime_summary,
        "hard_genotype_task_recovery":focus,
        "causal_controls":controls_output,
        "source_only_non_native_81_goal_projection":frozen["pre_native_finite_task_projection"],
        "limits":"Finite four-step goals and researcher-selected descending four-attempt task learner. Original PID executes all adaptations but did not invent them. 5184 all-goal source projections are NOT native evidence."
    },indent=2))

if __name__=="__main__":
    main()
