#!/usr/bin/env python3
"""CL1B-POLICY-P0 independent original CIL source replay, exact 8 native worlds.

This observer does NOT read the Rust internal opcode interpreter or native
register state to generate its forecast. It executes the prospectively frozen
43-node CIL itself; every native paid PC, register, memory occupancy/presence,
private credit, original executable task-policy Const and post-Write11 external
feedback line must match. No success is inferred from actor's claim alone.

Finite researcher-designed task-policy candidate cycling. NOT proof that
the original digital individual invented its adaptation algorithm.
"""
import json
import os
import sys
from pathlib import Path

FREEZE=Path(__file__).resolve().parents[1]/"studies/CL1B-POLICY-P0-frozen-43-node-task-feedback-paid-executable-patch-v1.json"
STUDY="CL1B-POLICY-P0"
KEYS=(11,20,21,32,33)
PRIVATE_CREDITS=800

def original_actor_source_interpreter(source,case,trace=None):
    code=[dict(x) for x in source]
    goals=case["goals"]
    assert len(goals)==4 and set(goals)<=set((0,1,2))
    feedback=case.get("feedback","truth")
    disabled=case.get("suppressed",False)
    r=[0,0,0,0]
    cells={}
    pc=0
    paid=0
    peak=44
    patches=[]
    services=[]
    reads=[]
    success_writes=[]
    attempts=[]
    claims=[]

    def records():
        if trace is None:
            while True:
                yield None
        else:
            with Path(trace).open() as f:
                for line in f:
                    yield json.loads(line)
    rows=records()
    def exact_native_equivalence(row):
        native=next(rows,None)
        if trace is not None and native!=row:
            raise AssertionError({
                "first_divergence_at_original_paid_step":paid,
                "arm":case["name"],"native_original_step":native,
                "independent_original_CIL_replay":row,
            })

    while paid<PRIVATE_CREDITS:
        inst=code[pc]
        at=pc
        paid+=1
        kind=inst["kind"]
        if kind=="Const":
            r[inst["dst"]]=inst["value"]
            pc=inst["next"]
        elif kind in ("Add","Sub"):
            a,b=r[inst["lhs"]],r[inst["rhs"]]
            r[inst["dst"]]=a+b if kind=="Add" else a-b
            pc=inst["next"]
        elif kind=="Read":
            key=r[inst["address"]]
            r[inst["dst"]]=cells.get(key,0)
            if at==4:
                assert key==20
                reads.append({"step":paid,"value":r[inst["dst"]]})
            pc=inst["next"]
        elif kind=="Write":
            key=r[inst["address"]]
            value=r[inst["value"]]
            cells[key]=value
            if at==11:
                assert key==33
                attempts.append({"step":paid,"value":value})
            if at==32:
                assert key==32 and 1<=value<=4
                success_writes.append({"step":paid,"value":value})
            if at in (25,41):
                assert key==21 and value==(0 if at==25 else 1)
                claims.append({"step":paid,"value":value})
            pc=inst["next"]
        elif kind=="Branch":
            pc=inst["nonzero"] if r[inst["test"]] else inst["zero"]
        elif kind=="PatchConst":
            assert at==22 and inst["kind"]=="PatchConst"
            target=r[inst["node"]]
            assert target==1 and code[1]["kind"]=="Const"
            assert reads and reads[-1]["value"]==0, "no genuine original paid failed-task feedback prior to patch"
            before=code[1]["value"]
            proposed=r[inst["value"]]
            assert proposed in (0,1,2)
            if not disabled:
                assert proposed!=before
                code[1]["value"]=proposed
            patches.append({"step":paid,"pc":at,"node":target,"before":before,
                "after":code[1]["value"],"edit_suppressed":disabled})
            pc=inst["next"]
        elif kind=="Halt":
            assert at in (26,42)
            pc=at
        else:
            raise AssertionError(("unknown CIL original opcode",at,kind))
        peak=max(peak,44+len(cells))
        assert len(cells)<=16 and peak<=300
        exact_native_equivalence(["cil",paid,at,pc,list(r),PRIVATE_CREDITS-paid,
            [cells.get(key) for key in KEYS],44+len(cells),code[1]["value"]])

        if at==2:
            assert kind=="Write" and key==11
            assert value==code[1]["value"]
            episode=cells.get(32,0)
            assert 0<=episode<4
            goal=goals[episode]
            truth=int(value==goal)
            assert feedback in ("truth","inverted","forced0")
            signal=truth if feedback=="truth" else 1-truth if feedback=="inverted" else 0
            services.append({"step":paid,"episode":episode,"output":value,
                "truth":truth,"feedback":signal})
            cells[20]=signal
            peak=max(peak,44+len(cells))
            exact_native_equivalence(["feedback",paid,episode,goal,value,truth,signal,
                [cells.get(key) for key in KEYS],44+len(cells),code[1]["value"]])
        if kind=="Halt":
            break
    else:
        raise AssertionError("original PID did not Halt under private charged compute limit")

    if trace is not None:
        assert next(rows,None) is None,"extra native line after original paid Halt"
    assert len(reads)==len(services)
    assert [v["value"] for v in reads]==[v["feedback"] for v in services]
    assert all(patch["step"]>reads[0]["step"] for patch in patches)
    assert len(claims)==1
    expected=case["expected"]
    result={
        "paid_steps":paid,"halt_pc":pc,
        "credits_remaining":PRIVATE_CREDITS-paid,
        "final_executable_guess":code[1]["value"],
        "actor_claimed_success":cells.get(21),
        "actor_completed_episode":cells.get(32),
        "original_paid_source_patches":patches,
        "original_paid_task_services":services,
        "original_paid_feedback_reads":reads,
        "original_paid_success_writes":success_writes,
        "original_paid_attempt_writes":attempts,
        "terminal_claim_writes":claims,
        "occupied_data_cells":len(cells),
        "peak_occupied":peak,
    }
    for key,value in result.items():
        if expected[key]!=value:
            raise AssertionError(("precommitted original source prediction mismatch",
                case["name"],key,expected[key],value))
    return result

def main():
    f=json.loads(FREEZE.read_text())
    source=f["original_native_CIL"]
    arms=f["original_native_task_cases"]
    controls=f["causal_controls"]
    assert len(source)==43 and len(arms)==4 and len(controls)==4
    assert source[1]=={"kind":"Const","dst":3,"value":0,"next":2}
    assert source[2]=={"kind":"Write","address":2,"value":3,"next":3}
    assert source[22]=={"kind":"PatchConst","node":1,"value":3,"next":0}
    native=None
    if len(sys.argv)==3:
        native=json.loads(Path(sys.argv[1]).read_text())
        assert native["study"]==STUDY
        assert native["level"]=="NATIVE_ORIGINAL_PID_FEEDBACK_CONDITIONED_PAID_EXECUTABLE_CONST"
        assert native["source_sha"] not in ("","local")
        if os.environ.get("GITHUB_SHA"):
            assert native["source_sha"]==os.environ["GITHUB_SHA"]
        assert len(native["original_task_regimes"])==4
        assert len(native["controls"])==4
        traces=Path(sys.argv[2])
    elif len(sys.argv)!=1:
        raise SystemExit("usage: python3 POLICY_P0_REPLAY.py [NATIVE_SUMMARY.json EVIDENCE_DIR]")

    cost=0
    edits=0
    arms_result=[]
    for i,case in enumerate(arms):
        name=case["name"]
        trace=(traces/f"cl1b_policy_p0_native_original_{name}.jsonl") if native else None
        result=original_actor_source_interpreter(source,case,trace)
        cost+=result["paid_steps"]
        edits+=len(result["original_paid_source_patches"])
        assert result["actor_claimed_success"]==1
        assert result["actor_completed_episode"]==4
        assert sum(x["truth"] for x in result["original_paid_task_services"])==4
        assert len(result["original_paid_source_patches"])>0
        if native:
            observed=native["original_task_regimes"][i]
            expected_native={
                "arm":name,"goals":case["goals"],
                "paid_original_steps":result["paid_steps"],
                "final_original_executable_const1":result["final_executable_guess"],
                "actor_claimed_success":result["actor_claimed_success"],
                "actor_completed_episode":result["actor_completed_episode"],
                "original_paid_source_patches":result["original_paid_source_patches"],
                "original_paid_task_services":result["original_paid_task_services"],
                "original_paid_feedback_reads":result["original_paid_feedback_reads"],
                "original_PID1_Halted":True,
                "peak_occupied":result["peak_occupied"],
                "host_source_edit_events_after_load":0,
            }
            for k,v in expected_native.items():
                assert observed[k]==v,(name,"native original summary differs",k,observed[k],v)
        arms_result.append({
            "arm":name,"goals":case["goals"],
            "paid_native_source_edit_events":len(result["original_paid_source_patches"]),
            "source_edit_provenance":result["original_paid_source_patches"],
            "true_task_attempts":len(result["original_paid_task_services"]),
            "true_task_completions":sum(y["truth"] for y in result["original_paid_task_services"]),
            "original_paid_instructions":result["paid_steps"],
            "full_four_stage_task_complete":True,
            "first_divergence":None,
        })
    assert edits==20
    ctrls=[]
    for i,case in enumerate(controls):
        name=case["name"]
        trace=(traces/f"cl1b_policy_p0_native_original_{name}.jsonl") if native else None
        result=original_actor_source_interpreter(source,case,trace)
        cost+=result["paid_steps"]
        if case["suppressed"] or case["feedback"]=="forced0":
            assert result["actor_claimed_success"]!=1
            assert result["actor_completed_episode"]!=4
        else:
            assert result["actor_claimed_success"]==1
            assert sum(x["truth"] for x in result["original_paid_task_services"])<4
        if native:
            observed=native["controls"][i]
            assert observed["arm"]==name and observed["goals"]==case["goals"]
            assert observed["paid_original_steps"]==result["paid_steps"]
            assert observed["original_paid_source_patches"]==result["original_paid_source_patches"]
            assert observed["original_paid_task_services"]==result["original_paid_task_services"]
            assert observed["actor_claimed_success"]==result["actor_claimed_success"]
            assert observed["original_PID1_Halted"] is True
        ctrls.append({
            "arm":name,"feedback_regime":case["feedback"],
            "edit_suppressed":case["suppressed"],
            "paid_original_instructions":result["paid_steps"],
            "source_edit_attempts":len(result["original_paid_source_patches"]),
            "true_task_matches":sum(x["truth"] for x in result["original_paid_task_services"]),
            "actor_success_claim":result["actor_claimed_success"],
            "first_divergence":None,
        })
    assert cost==f["expected_counts"]["total_paid"]==1094
    if native:
        assert native["counts"]=={
            "original_worlds":4,"causal_controls":4,
            "paid_original_cil_steps":cost,
            "actual_source_mutations":edits,
            "full_truthful_four_stage_task_completions":4}
    print(json.dumps({
        "study":STUDY,
        "evidence_level":"NATIVE_8_ORIGINAL_WORLDS_ALL_PAID_EXECUTABLE_SOURCE_PATCH_STEPS_REPLAY" if native else "SOURCE_ONLY_FROZEN_PRE_NATIVE_FORECAST",
        "source_sha":native["source_sha"] if native else None,
        "original_paid_steps_independently_checked":cost if native else 0,
        "native_original_worlds":8 if native else 0,
        "source_nodes":43,
        "peak_world_occupied":f["expected_counts"]["peak"],
        "charged_task_feedback_driven_executable_source_mutations":edits,
        "honest_regime_full_task_completions":4,
        "first_divergence":None,
        "honest_task_regimes":arms_result,
        "causal_controls":ctrls,
        "source_only_81_four_task_sequences":f["finite_goal_alphabet_projection"],
        "limits":"Executable output-guess mutation is contingent on real paid feedback. However the mutation rule was pre-authored, not discovered by the individual; no general code synthesis, semantic reward authentication or open-ended evolution is proven."
    },indent=2))

if __name__=="__main__":
    main()
