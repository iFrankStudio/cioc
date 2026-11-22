#!/usr/bin/env python3
"""C5 original-native matched-prefix semantic-feedback and actor gate audit.

RUN ONLY AFTER complete source-independent 814-world original-CIL replay.
Do not treat rotated-tail cases as independent environments or favorable holdouts.
"""
import json
import sys
from pathlib import Path
from collections import Counter, defaultdict

ROOT=Path(__file__).resolve().parents[1]
MANIFEST=ROOT/"studies/CL1B-G2-ACTOR-GATE-POSTPREFIX-C5-frozen-814-worlds-v1.json"
ORDER=["G2_actor_gate","G2_gate_always_edit","G2_gate_always_abstain",
       "G2_gate_pc28_suppressed","G2_gate_placebo180","G2_source_never"]
TAILS=["identity","swap_0_1_after24","swap_0_2_after24"]

def analyze(summary,trace_dir):
    m=json.loads(MANIFEST.read_text(encoding="utf-8"))
    n=json.loads(Path(summary).read_text(encoding="utf-8"))
    assert m["status"]=="PRE_NATIVE_FROZEN"
    assert n["study"]==m["study"]=="CL1B-G2-ACTOR-GATE-POSTPREFIX-C5"
    assert n["total_original_native_worlds"]==814
    assert len(m["cases"])==135 and len(m["arms"])==810 and len(n["truthful_records"])==810
    assert len(n["adversarial_records"])==4
    assert n["incomplete_truthful_world_names"]==[]
    assert [v["name"] for v in m["arms"]]==[v["name"] for v in n["truthful_records"]]
    assert all([x["source"] for x in n["truthful_records"][6*i:6*i+6]]==ORDER for i in range(135))
    labels=Counter();senses=Counter();count_streak=Counter()
    total_gate_events=0;last_success_guess=0;last_success_bit=0
    joint=[];cases=[];per_tail=defaultdict(lambda:{"n":0,"costs":{key:0 for key in ORDER},
       "gate_allow":0,"gate_abstain":0,"gate_better_than_always":0,
       "gate_better_than_abstain":0,"gate_worse_than_abstain":0})
    for i,case in enumerate(m["cases"]):
        gate,always,abstain,off,placebo,never=n["truthful_records"][6*i:6*i+6]
        assert all(x["goals"]==case["goals"] for x in [gate,always,abstain,off,placebo,never])
        assert all((x["completed_stages"],x["truthful_matches"],x["claim"],x["budget_exhausted"])==(48,48,1,False)
                   for x in (gate,always,abstain,off,placebo,never))
        assert gate["steps"]==placebo["steps"]
        assert gate["paid_gate_records"]==placebo["paid_gate_records"]
        assert gate["paid_gate_allow"]==len(gate["parameter_patches"])
        assert abstain["parameter_patches"]==[]
        assert always["paid_gate_abstain"]==0 and abstain["paid_gate_allow"]==0
        path=Path(trace_dir)/("cl1b_g2_actor_gate_c5_original_"+gate["name"]+".jsonl")
        trace=[json.loads(line) for line in path.read_text(encoding="utf-8").splitlines() if line.strip()]
        first_stage24=next(j for j,line in enumerate(trace) if line[0]=="feedback" and line[2]==24)
        prefix=trace[:first_stage24]
        assert prefix,"no common paid native prefix"
        feedback=[]
        gates=[]
        for line in trace:
            if line[0]=="feedback":
                feedback.append(line)
            elif line[0]=="cil" and line[2]==67:
                step,after,regs=line[1],line[3],line[4]
                assert after in (27,125)
                r3=regs[3]
                assert (after==27)==(r3!=0),"actor gate paid CIL branch must track nonzero r3"
                assert feedback,"paid PC67 actor gate without past feedback"
                last=feedback[-1]
                assert last[1]<step
                labels[str(r3)]+=1
                if r3==last[4]:last_success_guess+=1
                if last[5]==1 and last[6]==1:last_success_bit+=1
                # count consecutive good feedback bits in the last five paid
                # actual external Write11 services, not researcher true goals.
                streak=0
                for event in reversed(feedback):
                    if event[6]!=1:break
                    streak+=1
                count_streak[str(min(streak,10))]+=1
                senses[str(last[6])]+=1
                gates.append({"step":step,"r3":r3,"branch":after,"stage_before_or_after24":len(feedback)})
                total_gate_events+=1
        assert len(gates)==len(gate["paid_gate_records"])
        assert sum(x["branch"]==27 for x in gates)==gate["paid_gate_allow"]
        assert sum(x["branch"]==125 for x in gates)==gate["paid_gate_abstain"]
        f=per_tail[case["tail_mode"]]
        f["n"]+=1
        for x in (gate,always,abstain,off,placebo,never):f["costs"][x["source"]]+=x["steps"]
        f["gate_allow"]+=gate["paid_gate_allow"]
        f["gate_abstain"]+=gate["paid_gate_abstain"]
        f["gate_better_than_always"]+=gate["steps"]<always["steps"]
        f["gate_better_than_abstain"]+=gate["steps"]<abstain["steps"]
        f["gate_worse_than_abstain"]+=gate["steps"]>abstain["steps"]
        cases.append({"name":case["name"],"parent_case":case["parent_case"],
            "mode":case["tail_mode"],"gate_cost":gate["steps"],
            "always_cost":always["steps"],"abstain_cost":abstain["steps"],
            "pc28_effect_suppressed_cost":off["steps"],
            "gate_minus_abstain":gate["steps"]-abstain["steps"],
            "gate_minus_always":gate["steps"]-always["steps"],
            "paid_gates":gates,"native_prefix_length":len(prefix)})
        joint.append({"case":case,"prefix":prefix,"gates":gates,
            "delta_vs_abstain":abstain["steps"]-gate["steps"]})
    anchors=[]
    for i in range(45):
        group=joint[3*i:3*i+3]
        assert [x["case"]["tail_mode"] for x in group]==TAILS
        assert len({x["case"]["parent_case"] for x in group})==1
        assert all(x["case"]["goals"][:24]==group[0]["case"]["goals"][:24] for x in group)
        assert group[0]["case"]["goals"]!=group[1]["case"]["goals"]
        assert group[0]["case"]["goals"]!=group[2]["case"]["goals"]
        assert group[1]["case"]["goals"]!=group[2]["case"]["goals"]
        assert all(x["prefix"]==group[0]["prefix"] for x in group[1:]), (
            "original-native paid CIL/feedback histories differ before stage24",group[0]["case"]["parent_case"])
        decisions=[tuple(x["branch"] for x in g["gates"]) for g in group]
        deltas=[g["delta_vs_abstain"] for g in group]
        anchors.append({"parent_case":group[0]["case"]["parent_case"],
            "native_prefix_event_rows":len(group[0]["prefix"]),
            "pre_stage24_gate_events":sum(x[0]=="cil" and x[2]==67 for x in group[0]["prefix"]),
            "decision_sequences":decisions,
            "decision_sequence_changed_under_future_label_swap":len(set(decisions))>1,
            "abstain_minus_gate_per_tail":deltas,
            "gate_net_benefit_sign_changed":any(x>0 for x in deltas) and any(x<0 for x in deltas)})
    totals={k:sum(f["costs"][k] for f in per_tail.values()) for k in ORDER}
    assert totals==n["costs"]
    assert len(per_tail)==3 and all(x["n"]==45 for x in per_tail.values())
    assert totals["G2_actor_gate"]==totals["G2_gate_placebo180"]
    assert sum(x["gate_allow"] for x in per_tail.values())==n["actor_gate_allow_events"]
    assert sum(x["gate_abstain"] for x in per_tail.values())==n["actor_gate_abstain_events"]
    assert total_gate_events==n["actor_gate_allow_events"]+n["actor_gate_abstain_events"]
    assert totals["G2_gate_always_edit"]-totals["G2_actor_gate"]==n["net_gate_saving_vs_always"]
    assert totals["G2_gate_always_abstain"]-totals["G2_actor_gate"]==n["net_gate_saving_vs_abstain"]
    # C4 identity is a direct original-native reproduction, not a new holdout.
    assert per_tail["identity"]["costs"]=={
       "G2_actor_gate":132760,"G2_gate_always_edit":132957,
       "G2_gate_always_abstain":131154,"G2_gate_pc28_suppressed":131216,
       "G2_gate_placebo180":132760,"G2_source_never":125205
    },"identity did not exactly reproduce frozen C4 original-native costs"
    return {"study":m["study"],"status":"C5_ORIGINAL_NATIVE_POST_REPLAY_PAID_GATE_FEEDBACK_SEMANTICS",
       "native_worlds":814,"true_complete_worlds":810,
       "matched_shared_prefix_anchors":45,
       "identical_complete_native_histories_before_stage24":45,
       "total_paid_actor_gate_events":total_gate_events,
       "last_feedback_guess_equals_r3_events":last_success_guess,
       "last_feedback_was_true_and_reported_success_events":last_success_bit,
       "last_received_feedback_bit_distribution":dict(senses),
       "actor_gate_r3_labels":dict(labels),
       "last_feedback_success_streak_bounded10":dict(count_streak),
       "anchors_with_post24_changed_gate_decision_sequences":sum(a["decision_sequence_changed_under_future_label_swap"] for a in anchors),
       "anchors_with_opposite_gate_vs_abstain_future_sign":sum(a["gate_net_benefit_sign_changed"] for a in anchors),
       "net_gate_saving_vs_always":n["net_gate_saving_vs_always"],
       "net_gate_saving_vs_abstain":n["net_gate_saving_vs_abstain"],
       "main_efficiency_accepted":n["main_efficiency_accepted"],
       "truth_arm_paid_costs":totals,"tail_costs":dict(per_tail),
       "anchors":anchors,"cases":cases,
       "nonclaims":[
         "The 45 C4 task prefixes, including entire identity worlds, are reused; not 135 independent new worlds.",
         "r3 is a researcher-authored branch signal, not an actor-learned causal estimate of mutation's future payoff.",
         "Changed goal mapping does not simultaneously rename all fixed actor internal states; not a general label symmetry theorem.",
         "No authenticated external feedback, actor-invented mutation rule, or CL1B B3/B4."
       ]}
def main():
    if len(sys.argv)!=3:
        raise SystemExit("usage: cl1b_g2_actor_gate_c5_feedback_semantics_audit.py summary.json original_native_dir")
    print(json.dumps(analyze(sys.argv[1],sys.argv[2]),indent=2,ensure_ascii=False))
if __name__=="__main__":main()
