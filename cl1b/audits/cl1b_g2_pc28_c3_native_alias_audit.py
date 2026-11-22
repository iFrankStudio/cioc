#!/usr/bin/env python3
"""C3 exact original-native future-tail alias and decision regret audit.

Requires the independently replayed Rust-native 292-world C3 summary + JSONL.
No post-hoc tail selection or embedded expected effect sign.
"""
import json
import sys
from pathlib import Path
from collections import defaultdict

ROOT=Path(__file__).resolve().parents[1]
MANIFEST=ROOT/"studies/CL1B-G2-PC28-DECISION-ALIAS-C3-frozen-292-worlds-v1.json"
ORDER=("G2_on","G2_pc28_suppressed","G2_placebo180","G2_never")

def audit(summary_path,trace_dir):
    m=json.loads(MANIFEST.read_text(encoding="utf-8"))
    n=json.loads(Path(summary_path).read_text(encoding="utf-8"))
    assert m["status"]=="PRE_NATIVE_FROZEN"
    assert n["study"]==m["study"]=="CL1B-G2-PC28-DECISION-ALIAS-C3"
    assert n["total_original_native_worlds"]==m["counts"]["native_worlds"]==292
    truth=n["truthful_records"]
    assert len(truth)==288 and len(n["adversarial_records"])==4
    assert not n["incomplete_truthful_world_names"],"C3 fails full 48-stage feasibility"
    assert len(m["cases"])==72 and len(m["arms"])==288
    assert [x["name"] for x in truth]==[x["name"] for x in m["arms"]]

    def native_trace(record):
        p=Path(trace_dir)/("cl1b_g2_pc28_c3_original_"+record["name"]+".jsonl")
        return [json.loads(x) for x in p.read_text(encoding="utf-8").splitlines() if x]
    by_anchor=defaultdict(list)
    by_mode=defaultdict(lambda:{"cases":0,"on":0,"off":0,"never":0,"delta":0,"positive":0,"negative":0,"tie":0})
    totals={name:0 for name in ORDER}
    rows=[]
    for i,case in enumerate(m["cases"]):
        on,off,placebo,never=truth[4*i:4*i+4]
        assert tuple(x["source"] for x in (on,off,placebo,never))==ORDER
        assert all(x["goals"]==case["goals"] for x in (on,off,placebo,never))
        assert all((x["completed_stages"],x["truthful_matches"],x["claim"],x["budget_exhausted"])==(48,48,1,False)
                   for x in (on,off,placebo,never))
        assert on["steps"]==placebo["steps"] and on["services"]==placebo["services"]
        assert on["decisions"]==placebo["decisions"]
        assert on["parameter_patches"],"C3 must expose a paid first PC28 for every anchor"
        first_step=on["parameter_patches"][0]["step"]
        t=native_trace(on)
        first_idx=next(j for j,line in enumerate(t)
            if line[0]=="cil" and line[1]==first_step and line[2]==28)
        # The frozen tasks differ ONLY after stage23. The first PC28 must be
        # strictly before the next stage24 true task is ever serviced.
        assert all(not (line[0]=="feedback" and line[2]>=24) for line in t[:first_idx+1])
        first=t[first_idx]
        assert first[10]=="paid_parameter_patch"
        # Normalize no fields: the entire original-native CIL and feedback
        # prefix (even more stringent than the actor's actual observation) must
        # be byte-for-byte equivalent across all eight future tails.
        prefix=t[:first_idx+1]
        delta=off["steps"]-on["steps"]
        by_anchor[case["anchor"]].append({
            "mode":case["regime"],"case":case["name"],
            "first_paid_pc28_step":first_step,
            "first_pc28_trace_index":first_idx,
            "native_prefix":prefix,
            "first_parameter_attempt":on["parameter_patches"][0]["attempted"],
            "delta":delta
        })
        family=by_mode[case["regime"]]
        family["cases"]+=1
        family["on"]+=on["steps"]
        family["off"]+=off["steps"]
        family["never"]+=never["steps"]
        family["delta"]+=delta
        family["positive"]+=delta>0
        family["negative"]+=delta<0
        family["tie"]+=delta==0
        for x in (on,off,placebo,never):
            totals[x["source"]]+=x["steps"]
        rows.append({"case":case["name"],"anchor":case["anchor"],"mode":case["regime"],
          "on_cost":on["steps"],"off_cost":off["steps"],
          "never_cost":never["steps"],"delta_off_minus_on":delta,
          "pc28_attempts_on":len(on["parameter_patches"]),
          "changed_parameter_events_on":sum(bool(p["changed"]) for p in on["parameter_patches"]),
          "later_reads_on":len(on["later_mutated_reads"])})
    assert len(by_anchor)==9 and len(by_mode)==8
    assert all(f["cases"]==9 for f in by_mode.values())
    anchor_results=[]
    for anchor,records in by_anchor.items():
        assert len(records)==8
        assert len({r["mode"] for r in records})==8
        base=records[0]
        assert all(r["first_paid_pc28_step"]==base["first_paid_pc28_step"] for r in records)
        assert all(r["first_pc28_trace_index"]==base["first_pc28_trace_index"] for r in records)
        assert all(r["first_parameter_attempt"]==base["first_parameter_attempt"] for r in records)
        assert all(r["native_prefix"]==base["native_prefix"] for r in records), (
            "same frozen first 24 goals did not yield identical original-native history",anchor)
        pos=sum(max(r["delta"],0) for r in records)
        neg=sum(max(-r["delta"],0) for r in records)
        # At the fixed first-PC28 decision point, a permanent policy of
        # always-on incurs regret for harmful updates; always-off incurs
        # regret for beneficial updates. Oracle picks cheaper arm per tail.
        fixed_regret=min(pos,neg)
        anchor_results.append({
           "anchor":anchor,"shared_first_pc28_step":base["first_paid_pc28_step"],
           "shared_exact_native_prefix_events":len(base["native_prefix"]),
           "positive":sum(r["delta"]>0 for r in records),
           "negative":sum(r["delta"]<0 for r in records),
           "tie":sum(r["delta"]==0 for r in records),
           "sum_delta":sum(r["delta"] for r in records),
           "regret_permanently_on_vs_oracle":neg,
           "regret_permanently_off_vs_oracle":pos,
           "minimum_irreversible_prefix_choice_regret":fixed_regret,
           "opposite_sign_same_exact_history":pos>0 and neg>0,
           "tails":[{"mode":r["mode"],"delta":r["delta"]} for r in records]
        })
    anchor_results.sort(key=lambda x:x["anchor"])
    assert len(anchor_results)==9
    assert sum(x["sum_delta"] for x in anchor_results)==sum(r["delta_off_minus_on"] for r in rows)
    assert totals["G2_on"]==totals["G2_placebo180"]
    assert totals["G2_pc28_suppressed"]-totals["G2_on"]==n["paired_suppressed_minus_on_cost"]
    assert sum(r["delta_off_minus_on"]>0 for r in rows)==n["wins_on"]
    assert sum(r["delta_off_minus_on"]<0 for r in rows)==n["losses_on"]
    assert sum(r["delta_off_minus_on"]==0 for r in rows)==n["ties"]
    return {
      "study":m["study"],"status":"C3_POST_INDEPENDENT_REPLAY_ORIGINAL_NATIVE_EXACT_PREFIX_AUDIT",
      "n_original_native_worlds":292,"n_truthful_worlds":288,"n_future_tails":72,
      "shared_prefix_anchors":9,"tails_per_anchor":8,
      "anchors_with_opposite_sign_future_effects":sum(x["opposite_sign_same_exact_history"] for x in anchor_results),
      "minimum_irreversible_fixed_prefix_regret_across_eight_tails":
          sum(x["minimum_irreversible_prefix_choice_regret"] for x in anchor_results),
      "paid_cost_on_minus_off":totals["G2_on"]-totals["G2_pc28_suppressed"],
      "paid_costs":totals,"causal_delta_off_minus_on":n["paired_suppressed_minus_on_cost"],
      "wins_on":n["wins_on"],"losses_on":n["losses_on"],"ties":n["ties"],
      "by_tail":dict(by_mode),"anchors":anchor_results,"cases":rows,
      "nonclaims":[
        "Nine C2 prefixes were reused by construction, not a random cohort of 72 distinct independent environments.",
        "Permanent suppression of all future PC28 effects is a host-only full-lifecycle intervention, not an actor action.",
        "Same exact history with opposite future costs constrains an irreversible early policy; later feedback may allow adaptation.",
        "No autonomous algorithm invention, trusted feedback, or CL1B B3/B4 is established."
      ]
    }

def main():
    if len(sys.argv)!=3:
        raise SystemExit("usage: cl1b_g2_pc28_c3_native_alias_audit.py NATIVE_SUMMARY.json ORIGINAL_NATIVE_TRACE_DIR")
    print(json.dumps(audit(sys.argv[1],sys.argv[2]),indent=2,ensure_ascii=False))

if __name__=="__main__":main()
