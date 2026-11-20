#!/usr/bin/env python3
"""P1-H81 independent original-CIL every-step native 162-world cube audit.

Reuses the separately written 62-op source interpreter, NOT Rust engine state,
and verifies every original paid instruction register/memory/PC/credit plus
environmental reward event. The complete 3^4 goal cube is paired at identical
source and compute with a disabled native PatchConst control.
"""
import json
import os
import sys
from pathlib import Path
from cl1b_policy_p1_independent_native_search_stride_replay import replay

ROOT=Path(__file__).resolve().parents[1]
P1=ROOT/"studies/CL1B-POLICY-P1-frozen-62-node-feedback-conditioned-executable-search-stride-v1.json"
H81=ROOT/"studies/CL1B-POLICY-P1-H81-frozen-162-original-world-complete-transfer-audit-v1.json"
STUDY="CL1B-POLICY-P1-H81"

def main():
    p1=json.loads(P1.read_text())
    h81=json.loads(H81.read_text())
    source=p1["original_native_CIL_source"]
    cases=h81["all162_pre_registered_native_cases"]
    assert h81["study"]==STUDY and len(source)==62 and len(cases)==162
    original=None
    if len(sys.argv)==3:
        original=json.loads(Path(sys.argv[1]).read_text())
        assert original["study"]==STUDY
        assert original["level"]=="NATIVE_ORIGINAL_PID_162_WORLD_COMPLETE_FINITE_POLICY_TRANSFER_CUBE"
        assert original["source_sha"] not in ("","local")
        if os.environ.get("GITHUB_SHA"):
            assert original["source_sha"]==os.environ["GITHUB_SHA"]
        assert len(original["all81_paired_native_worlds"])==81
        folder=Path(sys.argv[2])
    elif len(sys.argv)!=1:
        raise SystemExit("usage: python3 P1_H81.py [NATIVE_SUMMARY.json TRACE_DIR]")
    positive=negative=neutral=0
    total_saved=0
    probes_saved=0
    total_paid=0
    examples=[]
    for i in range(81):
        on,off=cases[2*i:2*i+2]
        assert on["goals"]==off["goals"]
        assert on["feedback"]==off["feedback"]=="truth"
        assert not on["suppressed"] and off["suppressed"]
        on_path=(folder/f"cl1b_policy_p1_original_native_{on['name']}.jsonl") if original else None
        off_path=(folder/f"cl1b_policy_p1_original_native_{off['name']}.jsonl") if original else None
        active=replay(source,on,on_path)
        static=replay(source,off,off_path)
        assert active["actor_claimed_success"]==static["actor_claimed_success"]==1
        assert active["actor_completed_stages"]==static["actor_completed_stages"]==4
        assert sum(x["truth"] for x in active["original_paid_task_services"])==4
        assert sum(x["truth"] for x in static["original_paid_task_services"])==4
        saved=static["paid_steps"]-active["paid_steps"]
        probe_difference=len(static["original_paid_task_services"])-len(active["original_paid_task_services"])
        positive+=int(saved>0)
        negative+=int(saved<0)
        neutral+=int(saved==0)
        total_saved+=saved
        probes_saved+=probe_difference
        total_paid+=active["paid_steps"]+static["paid_steps"]
        if original:
            row=original["all81_paired_native_worlds"][i]
            assert row["goal_sequence"]==on["goals"]
            assert row["source_patch_charged_step_savings"]==saved
            assert row["source_patch_paid_task_probe_savings"]==probe_difference
            for case,check in ((on,row["policy_on"]),(off,row["policy_off"])):
                expected={
                    "arm":case["name"],"goals":case["goals"],
                    "feedback_regime":"truth",
                    "source_edit_suppressed":case["suppressed"],
                    "paid_original_steps":case["expected"]["paid_steps"],
                    "final_original_executable_stride":case["expected"]["executable_policy_stride"],
                    "original_paid_policy_source_edits":case["expected"]["original_paid_source_policy_patches"],
                    "original_paid_task_services":case["expected"]["original_paid_task_services"],
                    "original_paid_feedback_reads":case["expected"]["original_paid_feedback_reads"],
                    "actor_claimed_success":case["expected"]["actor_claimed_success"],
                    "actor_completed_stages":case["expected"]["actor_completed_stages"],
                    "original_PID1_halted":True,
                    "peak_occupied":case["expected"]["peak_occupied"],
                    "postload_host_source_edits":0,
                }
                for k,v in expected.items():
                    assert check[k]==v,(case["name"],"native aggregate mismatch",k,check[k],v)
        if on["goals"] in ([0,2,1,0],[0,1,2,0],[0,2,0,1],[0,0,0,0]):
            examples.append({"goals":on["goals"],"original_native_charged_source_policy_on":active["paid_steps"],
                "charged_patch_disabled":static["paid_steps"],
                "source_policy_savings":saved,
                "first_trace_divergence":None})
    assert (positive,negative,neutral)==(27,27,27)
    assert (total_saved,probes_saved,total_paid)==(-108,0,35748)
    source_counts=h81["pre_registered_source_projection"]
    assert source_counts["positive_policy_transfer"]==positive
    assert source_counts["negative_policy_transfer"]==negative
    assert source_counts["neutral_policy_transfer"]==neutral
    assert source_counts["net_paid_instruction_savings"]==total_saved
    if original:
        assert original["counts"]=={
            "task_goal_sequences":81,"native_original_worlds":162,
            "positive_policy_transfer":positive,
            "negative_policy_transfer":negative,
            "neutral_policy_transfer":neutral,
            "net_paid_instruction_savings":total_saved,
            "net_paid_task_probe_savings":probes_saved,
            "total_paid_original_CIL_instructions":total_paid,
            "actor_true_four_task_completions":162,
        }
    print(json.dumps({
        "study":STUDY,
        "evidence_level":"NATIVE_162_WORLD_ALL_CHARGED_CIL_STEPS_AND_FEEDBACK_REPLAY" if original else "PREREGISTERED_SOURCE_ONLY_FINITE_PROJECTIONS",
        "source_sha":original["source_sha"] if original else None,
        "native_original_paid_instructions_replayed":total_paid if original else 0,
        "native_worlds":162,"goals_exhaustive":81,
        "policy_patch_helped":positive,
        "policy_patch_harmed":negative,
        "policy_patch_tied":neutral,
        "all_worlds_truthfully_completed_four_tasks":True,
        "total_net_paid_instruction_savings":total_saved,
        "total_net_paid_task_probe_savings":probes_saved,
        "first_divergence":None,
        "causal_examples":examples,
        "limits":"Complete finite 81-goal alphabet cube only. Researcher-authored policy mutation can negatively transfer; this does not establish self-invented mutation rules or open-ended adaptive benefit."
    },indent=2))

if __name__=="__main__":
    main()
