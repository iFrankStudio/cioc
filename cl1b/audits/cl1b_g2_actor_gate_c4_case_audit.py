#!/usr/bin/env python3
"""C4 native case-level paid actor-gate selection audit (post independent replay).

All 45 pre-frozen cases and six charged original-native arms remain in denominator.
The per-case hindsight best fixed arm is a descriptive oracle, not the PID1's
available action and not an expectation or heldout deployment guarantee.
"""
import json
import sys
from collections import defaultdict
from pathlib import Path

MANIFEST=Path(__file__).resolve().parents[1]/"studies/CL1B-G2-ACTOR-PAID-GATE-C4-frozen-274-worlds-v1.json"
ORDER=["G2_actor_gate","G2_gate_always_edit","G2_gate_always_abstain",
       "G2_gate_pc28_suppressed","G2_gate_placebo180","G2_source_never"]

def analyze(path):
    m=json.loads(MANIFEST.read_text(encoding="utf-8"))
    n=json.loads(Path(path).read_text(encoding="utf-8"))
    assert m["study"]==n["study"]=="CL1B-G2-ACTOR-PAID-GATE-C4"
    assert m["status"]=="PRE_NATIVE_FROZEN" and n["status"]=="ORIGINAL_NATIVE_ACTOR_PAID_GATED_PC28"
    assert n["total_original_native_worlds"]==274
    assert len(m["cases"])==45
    assert len(n["truthful_records"])==270 and len(n["adversarial_records"])==4
    assert n["incomplete_truthful_world_names"]==[]
    source=m["frozen_source_g2_actor_gate"]
    assert len(source)==180
    for name in ("frozen_source_g2_gate_always_edit","frozen_source_g2_gate_always_abstain"):
        ref=m[name]
        assert len(ref)==180
        assert all(ref[i]==source[i] for i in range(180) if i!=67)
        assert ref[67]["kind"]=="Branch" and ref[67]["test"]==3
    assert source[67]=={"kind":"Branch","test":3,"nonzero":27,"zero":125}
    assert m["frozen_source_g2_gate_always_edit"][67]["zero"]==27
    assert m["frozen_source_g2_gate_always_abstain"][67]["nonzero"]==125
    rows=[];totals={name:0 for name in ORDER}
    families=defaultdict(lambda:{"n":0,"gate":0,"always":0,"abstain":0,
                                 "suppressed":0,"gate_paid_edit":0,"gate_paid_abstain":0,
                                 "regret_vs_per_case_two_fixed_oracle":0})
    for i,case in enumerate(m["cases"]):
        group=n["truthful_records"][6*i:6*i+6]
        assert [x["source"] for x in group]==ORDER
        assert [x["name"] for x in group]==[a["name"] for a in m["arms"][6*i:6*i+6]]
        assert all(x["goals"]==case["goals"] for x in group)
        assert all((x["completed_stages"],x["truthful_matches"],x["claim"],x["budget_exhausted"])==(48,48,1,False) for x in group)
        gate,always,abstain,off,placebo,never=group
        for x in group:totals[x["source"]]+=x["steps"]
        assert gate["steps"]==placebo["steps"]
        assert gate["paid_gate_records"]==placebo["paid_gate_records"]
        assert gate["paid_gate_allow"]==len(gate["parameter_patches"])
        assert always["paid_gate_abstain"]==0
        assert abstain["paid_gate_allow"]==0
        assert abstain["parameter_patches"]==[]
        delta=off["steps"]-gate["steps"]
        regretted=gate["steps"]-min(always["steps"],abstain["steps"])
        f=families[case["regime"]];f["n"]+=1
        f["gate"]+=gate["steps"];f["always"]+=always["steps"]
        f["abstain"]+=abstain["steps"];f["suppressed"]+=off["steps"]
        f["gate_paid_edit"]+=gate["paid_gate_allow"]
        f["gate_paid_abstain"]+=gate["paid_gate_abstain"]
        f["regret_vs_per_case_two_fixed_oracle"]+=regretted
        rows.append({"case":case["name"],"regime":case["regime"],
            "gate":gate["steps"],"always":always["steps"],
            "abstain":abstain["steps"],"suppressed":off["steps"],
            "source_never":never["steps"],
            "same_source_suppressed_minus_gate":delta,
            "paid_gate_edit":gate["paid_gate_allow"],
            "paid_gate_abstain":gate["paid_gate_abstain"],
            "gate_beats_both":gate["steps"]<always["steps"] and gate["steps"]<abstain["steps"],
            "gate_vs_per_case_two_fixed_oracle":regretted})
    assert totals==n["costs"]
    assert len(families)==5 and all(f["n"]==9 for f in families.values())
    assert sum(x["paid_gate_edit"] for x in rows)==n["actor_gate_allow_events"]
    assert sum(x["paid_gate_abstain"] for x in rows)==n["actor_gate_abstain_events"]
    assert totals["G2_gate_pc28_suppressed"]-totals["G2_actor_gate"]==n["paired_suppressed_minus_gate_cost"]
    assert totals["G2_gate_always_edit"]-totals["G2_actor_gate"]==n["net_gate_saving_vs_always"]
    assert totals["G2_gate_always_abstain"]-totals["G2_actor_gate"]==n["net_gate_saving_vs_abstain"]
    assert totals["G2_actor_gate"]==totals["G2_gate_placebo180"]
    return {
      "study":m["study"],"status":"C4_POST_ORIGINAL_NATIVE_AND_INDEPENDENT_REPLAY_CASE_LEVEL_AUDIT",
      "all_true_worlds_completed":270,"all_original_native_worlds":274,
      "paid_gate_allow":n["actor_gate_allow_events"],
      "paid_gate_abstain":n["actor_gate_abstain_events"],
      "net_gate_saving_vs_always":n["net_gate_saving_vs_always"],
      "net_gate_saving_vs_abstain":n["net_gate_saving_vs_abstain"],
      "true_effect_suppression_minus_gate":n["paired_suppressed_minus_gate_cost"],
      "gate_better_than_both_per_case":sum(x["gate_beats_both"] for x in rows),
      "gate_better_than_always_per_case":sum(x["gate"]<x["always"] for x in rows),
      "gate_better_than_abstain_per_case":sum(x["gate"]<x["abstain"] for x in rows),
      "gate_equal_to_abstain_per_case":sum(x["gate"]==x["abstain"] for x in rows),
      "gate_worse_than_abstain_per_case":sum(x["gate"]>x["abstain"] for x in rows),
      "total_gate_regret_vs_per_case_two_fixed_arm_oracle":sum(x["gate_vs_per_case_two_fixed_oracle"] for x in rows),
      "arm_costs":totals,"families":dict(families),"cases":rows,
      "nonclaims":[
        "Best per-case two-fixed-arm hindsight oracle is not native actor knowledge.",
        "Gate/always/abstain differ only by paid source PC67 branch target but are different CIL source programs.",
        "A positive gate selection over always is not autonomous future-reward forecast, proof of learning, or CL1B B3/B4.",
        "No post-frozen task rejection, no privileged gate input, no free patched code."
      ]
    }

def main():
    if len(sys.argv)!=2:
        raise SystemExit("usage: cl1b_g2_actor_gate_c4_case_audit.py native_summary.json")
    print(json.dumps(analyze(sys.argv[1]),indent=2,ensure_ascii=False))

if __name__=="__main__":main()
