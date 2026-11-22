#!/usr/bin/env python3
"""C6 post-result mechanistic decomposition, exact original-native summaries.

This is a supplementary explanatory check, NOT a new preregistered win or a
new natural-task holdout. Original-native events were independently replayed
in cl1b_g2_selfstate_mirror_c6_independent_native_replay.py before this audit.
"""
import json
import sys
from pathlib import Path

ROOT=Path(__file__).resolve().parents[1]
MANIFEST=ROOT/"studies/CL1B-G2-ACTOR-SELFSTATE-MIRROR-C6-frozen-274-worlds-v1.json"
ORDER=("G2_mirror","G2_mirror_always","G2_mirror_abstain",
       "G2_mirror_pc28_suppressed","G2_mirror_placebo180","G2_legacy_label_gate")

def audit(path):
    m=json.loads(MANIFEST.read_text(encoding="utf-8"))
    n=json.loads(Path(path).read_text(encoding="utf-8"))
    assert n["study"]==m["study"]=="CL1B-G2-ACTOR-SELFSTATE-MIRROR-C6"
    assert len(m["cases"])==45 and len(n["truthful_records"])==270
    assert n["incomplete_truthful_world_names"]==[]
    rows=[]
    delta_total=skip_total=0
    for i,c in enumerate(m["cases"]):
        gate,always,never,poison,placebo,legacy=n["truthful_records"][6*i:6*i+6]
        assert tuple(x["source"] for x in (gate,always,never,poison,placebo,legacy))==ORDER
        assert all(x["goals"]==c["goals"] for x in (gate,always,never,poison,placebo,legacy))
        assert gate["steps"]==placebo["steps"]
        assert gate["mirror_gate_mismatches"]==always["mirror_gate_mismatches"]==0
        skips=gate["mirror_noop_skips"]
        assert skips==gate["paid_gate_abstain"]
        assert gate["paid_gate_allow"]==gate["paid_mirror_writes"]
        assert always["paid_gate_allow"]==always["paid_mirror_writes"]
        assert always["paid_gate_abstain"]==0
        assert never["paid_gate_allow"]==never["paid_mirror_writes"]==0
        assert always["paid_gate_allow"]-gate["paid_gate_allow"]==skips
        assert always["paid_mirror_writes"]-gate["paid_mirror_writes"]==skips
        # At each mirror-equality guard, "always" pays 4 redundant CIL
        # instructions: PC27 Const target, PC28 PatchConst (same value),
        # PC24 Const cell40, PC52 Write (same value or creates absent cell).
        # All fees are paid and the original native PID1 service path remains
        # identical at the task/feedback semantic level.
        diff=always["steps"]-gate["steps"]
        assert diff==4*skips,("not a pure paid-noop saving",c["name"],diff,skips)
        def normalize(events):
            return [(e["stage"],e["guess"],e["truth"],e["feedback"]) for e in events]
        assert normalize(always["services"])==normalize(gate["services"]),(
            "actual task guesses/true feedback differ",c["name"])
        assert gate["final_theta"]==always["final_theta"]
        assert gate["final_edges"]==always["final_edges"]
        assert gate["completed_stages"]==always["completed_stages"]==48
        delta_total+=diff;skip_total+=skips
        rows.append({
          "name":c["name"],"regime":c["regime"],
          "skipped_same_parameter_update":skips,
          "normal_mirror_paid":gate["steps"],"always_mirror_paid":always["steps"],
          "same_workload_total_CIL_saved":diff,
          "identical_semantic_goal_probe_and_true_feedback_sequence":True,
          "same_final_source_theta":True,
          "normal_actor_peak_occupied":gate["peak_occupied"],
          "always_actor_peak_occupied":always["peak_occupied"]
        })
    assert delta_total==n["mirror_net_saving_vs_always"]==52
    assert skip_total==n["skipped_paid_same_parameter_edits"]==13
    return {
      "study":m["study"],
      "evidence_class":"POST_HOC_NATIVE_ACCOUNTING_IDENTITY_NOT_A_PROSPECTIVELY_ACCEPTED_LEARNING_RESULT",
      "all_45_new_cases_verified":45,
      "skipped_semantically_redundant_paid_code_edits":skip_total,
      "exact_CIL_per_skipped_edit":4,
      "total_4x_charge_saved":delta_total,
      "all_45_task_feedback_sequences_identical_between_mirror_and_always":True,
      "all_45_final_theta_and_mutable_edges_identical":True,
      "original_native_actor_peak_occupied_differences":sum(x["normal_actor_peak_occupied"]!=x["always_actor_peak_occupied"] for x in rows),
      "did_beating_never_edit_policy_occur":n["main_efficiency_accepted"],
      "mirror_more_costly_than_best_paired_never_by":-n["mirror_net_saving_vs_abstain"],
      "cases":rows,
      "nonclaims":[
        "Mirror and always may differ in instruction counter, private credits and materialized cell40; do not claim full physical trace equivalence.",
        "This 52-CIL reduction is exactly removal of redundant paid instructions, NOT future-goal prediction or better task selection.",
        "This retrospective decomposition did not select, remove or reweight any of the 45 frozen task cases.",
        "Poisoned mirror case is not pure editing-effect causal ablation; no CL1B B3/B4."
      ]
    }

if __name__=="__main__":
    if len(sys.argv)!=2:raise SystemExit("usage: cl1b_g2_selfstate_mirror_c6_exact_noop_audit.py rust_original_native_summary.json")
    print(json.dumps(audit(sys.argv[1]),indent=2,ensure_ascii=False))
