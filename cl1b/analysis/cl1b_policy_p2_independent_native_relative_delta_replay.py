#!/usr/bin/env python3
"""CL1B-POLICY-P2 original source-independent exact native replay.

Every charged CIL instruction from the frozen 80-node founder is re-executed
by a separate Python interpreter (NOT inferred from the Rust engine). Audit
all 81 paired goal sequences and six adversity controls: actor-origin
Read20, self-paid previous-goal Write34 / Read34, signed normalisation,
self-paid executable PatchConst at node46, original Write11, private credits,
cells/PC/registers, natural halt and truthful environment feedback.
"""
import json
import os
import sys
from pathlib import Path

ROOT=Path(__file__).resolve().parents[1]
SOURCE_FILE=ROOT/"studies/CL1B-POLICY-P2-frozen-80-node-actor-paid-relative-delta-search-policy-v1.json"
PRIOR_FILE=ROOT/"studies/CL1B-POLICY-P1-H81-frozen-162-original-world-complete-transfer-audit-v1.json"
STUDY="CL1B-POLICY-P2"
KEYS=(11,20,21,30,32,33,34)
CREDIT=800

def run_source(source,arm,trace=None):
    code=[dict(op) for op in source]
    goals=arm["goals"]
    mode=arm["feedback"]
    disabled=arm["suppressed"]
    assert len(code)==80
    assert len(goals)==4 and set(goals)<=set((0,1,2))
    assert mode in ("truth","inverted","forced0")
    cells={}
    reg=[0,0,0,0]
    pc=0
    paid=0
    peak=81
    outputs=[]
    rewards=[]
    patches=[]
    successes=[]
    failures=[]
    reset=[]
    claims=[]
    prior_writes=[]
    prior_reads=[]

    def native_rows():
        if trace is None:
            while True: yield None
        else:
            with Path(trace).open() as fh:
                for row in fh: yield json.loads(row)
    records=native_rows()

    def validate(row):
        observed=next(records,None)
        if trace is not None and observed!=row:
            raise AssertionError({
                "original_arm":arm["name"],
                "first_native_divergence_paid_instruction":paid,
                "native_original_evidence":observed,
                "independent_original_source_replay":row,
            })

    while paid<CREDIT:
        inst=code[pc]
        at=pc
        kind=inst["kind"]
        paid+=1
        if kind=="Const":
            reg[inst["dst"]]=inst["value"]
            pc=inst["next"]
        elif kind=="Read":
            address=reg[inst["address"]]
            reg[inst["dst"]]=cells.get(address,0)
            if at==5:
                assert address==20
                rewards.append({"step":paid,"value":reg[inst["dst"]]})
            if at==69:
                assert address==34 and len(prior_writes)==1 and not prior_reads
                assert reg[inst["dst"]]==prior_writes[0]["value"]
                prior_reads.append({"step":paid,"value":reg[inst["dst"]]})
            pc=inst["next"]
        elif kind in ("Add","Sub"):
            first,second=reg[inst["lhs"]],reg[inst["rhs"]]
            reg[inst["dst"]]=first+second if kind=="Add" else first-second
            pc=inst["next"]
        elif kind=="Write":
            address=reg[inst["address"]]
            value=reg[inst["value"]]
            cells[address]=value
            if at==12:
                assert address==33 and value in (1,2,3)
                failures.append({"step":paid,"value":value})
            if at==34:
                assert address==32 and value in (1,2,3,4)
                successes.append({"step":paid,"value":value})
            if at==37:
                assert address==33 and value==0
                reset.append({"step":paid,"value":0})
            if at==66:
                assert address==34 and not prior_writes
                assert cells.get(32)==1, "one paid previous-goal store only after first earned task"
                prior_writes.append({"step":paid,"value":value})
            if address==21:
                assert at in (27,60) and value==(0 if at==27 else 1)
                claims.append({"step":paid,"value":value})
            pc=inst["next"]
        elif kind=="Branch":
            pc=inst["nonzero"] if reg[inst["test"]] else inst["zero"]
        elif kind=="PatchConst":
            assert at==45 and reg[inst["node"]]==46
            assert not patches
            assert rewards and rewards[-1]["value"]==1
            assert cells.get(32)==2
            assert len(prior_reads)==len(prior_writes)==1
            assert code[46]["kind"]=="Const"
            desired=reg[inst["value"]]
            assert desired in (0,1,2)
            before=code[46]["value"]
            if not disabled: code[46]["value"]=desired
            patches.append({"step":paid,"source_node":46,"before":before,
                "after":code[46]["value"],"requested":desired,
                "edit_suppressed":disabled})
            pc=inst["next"]
        elif kind=="Halt":
            assert at in (28,61)
            pc=at
        else:
            raise AssertionError(("frozen original opcode unknown",kind,at))

        peak=max(peak,81+len(cells))
        assert len(cells)<=16 and peak<=300
        validate(["cil",paid,at,pc,list(reg),CREDIT-paid,
            [cells.get(k) for k in KEYS],81+len(cells),code[46]["value"]])

        if at==3:
            assert kind=="Write" and address==11
            assert value==reg[3], "task guess not generated by actor"
            episode=cells.get(32,0)
            assert 0<=episode<4
            truthful=int(value==goals[episode])
            delivered=truthful if mode=="truth" else 1-truthful if mode=="inverted" else 0
            outputs.append({"step":paid,"episode":episode,"output":value,
                "truth":truthful,"feedback":delivered})
            cells[20]=delivered
            peak=max(peak,81+len(cells))
            validate(["feedback",paid,episode,goals[episode],value,truthful,
                delivered,[cells.get(k) for k in KEYS],81+len(cells),
                code[46]["value"]])
        if kind=="Halt": break
    else:
        raise AssertionError(("original PID exhausted private paid credits",arm["name"]))

    if trace is not None:
        assert next(records,None) is None, "native evidence continues past paid Halt"
    assert len(claims)==1
    assert len(successes)==len(reset)
    assert len(rewards)==len(outputs)
    assert [x["value"] for x in rewards]==[x["feedback"] for x in outputs]
    assert len(patches)<=1
    assert len(prior_writes)<=1 and len(prior_reads)<=1
    assert code[1]==source[1],"source task-output Const mutated instead of policy stride"
    expected=arm["expected"]
    got={
        "paid_steps":paid,
        "halt_pc":pc,
        "private_credits_left":CREDIT-paid,
        "final_executable_stride":code[46]["value"],
        "actor_claim":cells.get(21),
        "actor_completed_stages":cells.get(32),
        "original_paid_task_services":outputs,
        "original_paid_feedback_reads":rewards,
        "original_paid_policy_source_edits":patches,
        "original_paid_success_writes":successes,
        "original_paid_failure_writes":failures,
        "original_paid_failure_resets":reset,
        "original_paid_terminal_claims":claims,
        "original_paid_previous_goal_writes":prior_writes,
        "original_paid_previous_goal_reads":prior_reads,
        "occupied_data_cells":len(cells),
        "peak_occupied":peak,
    }
    for key,value in got.items():
        if expected[key]!=value:
            raise AssertionError(("precommitted prediction contradicted",arm["name"],
                key,expected[key],value))
    return got

def main():
    manifest=json.loads(SOURCE_FILE.read_text())
    prior=json.loads(PRIOR_FILE.read_text())
    assert manifest["study"]==STUDY
    source=manifest["original_native_CIL_source"]
    pairs=manifest["all162_pre_registered_original_PID_worlds"]
    controls=manifest["adversarial_controls"]
    assert len(source)==80 and len(pairs)==162 and len(controls)==6
    assert source[45]=={"kind":"PatchConst","node":1,"value":0,"next":46}
    assert source[46]=={"kind":"Const","dst":2,"value":1,"next":47}
    assert source[66]=={"kind":"Write","address":2,"value":3,"next":67}
    assert source[69]=={"kind":"Read","dst":0,"address":2,"next":70}

    original=None
    if len(sys.argv)==3:
        original=json.loads(Path(sys.argv[1]).read_text())
        assert original["study"]==STUDY
        assert original["level"]=="NATIVE_ORIGINAL_PID_80_NODE_RELATIVE_DELTA_PAID_STRIDE_COMPLETE_81_PAIR_CUBE"
        assert original["source_sha"] not in ("","local")
        if os.environ.get("GITHUB_SHA"):
            assert original["source_sha"]==os.environ["GITHUB_SHA"]
        assert len(original["all81_paired_native_worlds"])==81
        assert len(original["six_adversarial_native_controls"])==6
        folder=Path(sys.argv[2])
    elif len(sys.argv)!=1:
        raise SystemExit("usage: P2_REPLAY.py [NATIVE_SUMMARY.json ORIGINAL_NATIVE_TRACES]")

    total=0
    help_count=harm_count=tie_count=0
    net_savings=net_probes=relative_prior=0
    cyclic_goals=0
    cases=[]
    prev=prior["all162_pre_registered_native_cases"]

    for i in range(81):
        on_arm,off_arm=pairs[2*i],pairs[2*i+1]
        assert on_arm["goals"]==off_arm["goals"]
        assert not on_arm["suppressed"] and off_arm["suppressed"]
        checked=[]
        for arm in (on_arm,off_arm):
            log=(folder/f"cl1b_policy_p2_original_{arm['name']}.jsonl") if original else None
            result=run_source(source,arm,log)
            total+=result["paid_steps"]
            assert result["actor_claim"]==1 and result["actor_completed_stages"]==4
            assert sum(x["truth"] for x in result["original_paid_task_services"])==4
            checked.append(result)
        on,off=checked
        native_saving=off["paid_steps"]-on["paid_steps"]
        net_savings+=native_saving
        net_probes+=len(off["original_paid_task_services"])-len(on["original_paid_task_services"])
        help_count+=int(native_saving>0)
        harm_count+=int(native_saving<0)
        tie_count+=int(native_saving==0)
        assert prev[2*i]["goals"]==on_arm["goals"]
        relative_prior+=prev[2*i]["expected"]["paid_steps"]-on["paid_steps"]
        a,b,c,d=on_arm["goals"]
        cyclic_goals+=int(c==(2*b-a+3)%3 and d==(3*b-2*a+6)%3)

        if original:
            native_pair=original["all81_paired_native_worlds"][i]
            assert native_pair["goals"]==on_arm["goals"]
            assert native_pair["native_paid_instruction_saving"]==native_saving
            assert native_pair["native_paid_task_probe_saving"]==(
                len(off["original_paid_task_services"])-len(on["original_paid_task_services"]))
            for side,arm,result in zip(("source_patch_on","source_patch_off"),
                                       (on_arm,off_arm),(on,off)):
                n=native_pair[side]
                expected_native={
                    "arm":arm["name"],"goals":arm["goals"],
                    "feedback_regime":arm["feedback"],
                    "source_edit_suppressed":arm["suppressed"],
                    "original_paid_steps":result["paid_steps"],
                    "final_executable_stride":result["final_executable_stride"],
                    "original_paid_policy_edits":result["original_paid_policy_source_edits"],
                    "original_paid_task_services":result["original_paid_task_services"],
                    "original_paid_feedback_reads":result["original_paid_feedback_reads"],
                    "original_paid_previous_goal_writes":result["original_paid_previous_goal_writes"],
                    "original_paid_previous_goal_reads":result["original_paid_previous_goal_reads"],
                    "actor_claim":result["actor_claim"],
                    "actor_completed_stages":result["actor_completed_stages"],
                    "original_PID1_halted":True,
                    "peak_occupied":result["peak_occupied"],
                    "postload_host_source_edits":0,
                }
                for field,value in expected_native.items():
                    assert n[field]==value,(arm["name"],field,n[field],value)
        if i in (0,6,12,40,50,80):
            cases.append({"goals":on_arm["goals"],
                "paid_when_policy_mutates":on["paid_steps"],
                "paid_when_source_edit_suppressed":off["paid_steps"],
                "causal_savings":native_saving,
                "original_source_patch":on["original_paid_policy_source_edits"],
                "independent_native_divergence":None})

    assert (help_count,harm_count,tie_count)==(27,27,27)
    assert (net_savings,net_probes,total,relative_prior,cyclic_goals)==(-54,0,39636,-1917,9)

    causal=[]
    for i,arm in enumerate(controls):
        log=(folder/f"cl1b_policy_p2_original_{arm['name']}.jsonl") if original else None
        result=run_source(source,arm,log)
        total+=result["paid_steps"]
        if original:
            n=original["six_adversarial_native_controls"][i]
            assert n["arm"]==arm["name"]
            assert n["goals"]==arm["goals"]
            assert n["original_paid_steps"]==result["paid_steps"]
            assert n["original_paid_policy_edits"]==result["original_paid_policy_source_edits"]
            assert n["original_paid_task_services"]==result["original_paid_task_services"]
            assert n["original_paid_previous_goal_writes"]==result["original_paid_previous_goal_writes"]
            assert n["original_paid_previous_goal_reads"]==result["original_paid_previous_goal_reads"]
            assert n["original_PID1_halted"] is True
        causal.append(result)
    assert (causal[0]["paid_steps"],causal[1]["paid_steps"])==(174,221)
    assert (causal[2]["paid_steps"],causal[3]["paid_steps"])==(197,242)
    assert causal[4]["actor_claim"]==1
    assert sum(x["truth"] for x in causal[4]["original_paid_task_services"])<4
    assert causal[5]["actor_claim"]==0

    if original:
        assert original["counts"]=={
            "goal_sequences":81,"paired_original_native_worlds":162,
            "causal_native_controls":6,"positive_patch_transfer":27,
            "negative_patch_transfer":27,"neutral_patch_transfer":27,
            "net_paid_instruction_savings":-54,
            "net_task_probe_savings":0,
            "all_pair_worlds_truthfully_completed_four_tasks":162,
            "total_pair_native_paid_steps":39636,
            "absolute_paid_difference_vs_P1_earlier_source_model":-1917,
        }
    print(json.dumps({
        "study":STUDY,
        "evidence_level":"NATIVE_168_ORIGINAL_WORLD_EXACT_CHARGED_STEP_INDEPENDENT_REPLAY" if original else "FROZEN_SOURCE_ONLY_PREDICTIONS",
        "source_sha":original["source_sha"] if original else None,
        "original_private_paid_steps_independently_replayed":total if original else 0,
        "original_worlds":168,"goals":81,
        "paid_previous_goal_Write34_and_Read34":True,
        "actual_original_PID1_owns_relative_policy_PatchConst46":True,
        "causally_helped":help_count,"causally_harmed":harm_count,
        "causally_tied":tie_count,
        "net_paid_savings_on_full_goal_cube":net_savings,
        "relative_absolute_paid_change_vs_native_verified_P1_projection":relative_prior,
        "first_divergence":None,
        "selected_causal_counterexamples":cases,
        "limitations":"Researcher-authored relative-delta heuristic with finite alphabet; expensive extra paid observation cannot predict unrestricted future tasks and is not general policy invention."
    },indent=2))

if __name__=="__main__":
    main()
