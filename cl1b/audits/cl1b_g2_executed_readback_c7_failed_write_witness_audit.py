#!/usr/bin/env python3
"""C7 post-result original-native failed-write/readback witness chronology.

This is post hoc mechanistic attribution for the already frozen C7 worlds.
All original CIL, source, cell and feedback history must have been separately
verified before this audit. This program never supplies an actor observation.
"""
import json
import statistics
import sys
from collections import defaultdict
from pathlib import Path

MANIFEST=Path(__file__).resolve().parents[1]/"studies/CL1B-G2-ACTOR-EXECUTED-CODE-READBACK-C7-frozen-274-worlds-v1.json"

def analyze(native_summary):
    m=json.loads(MANIFEST.read_text(encoding="utf-8"))
    d=json.loads(Path(native_summary).read_text(encoding="utf-8"))
    assert m["study"]==d["study"]=="CL1B-G2-ACTOR-EXECUTED-CODE-READBACK-C7"
    assert m["status"]=="PRE_NATIVE_FROZEN"
    assert d["status"]=="ORIGINAL_NATIVE_ACTOR_PAID_EXECUTED_CONST_SELF_READBACK"
    assert len(m["cases"])==45 and len(d["truthful_records"])==270
    assert d["incomplete_truthful_world_names"]==[]
    base=m["frozen_source_g2_intent_mirror_C6"]
    live=m["frozen_source_g2_readback_attest"]
    assert [i for i in range(180) if base[i]!=live[i]]==[24,28,52,79,173]
    assert live[173]["next"]==24 and live[24]=={"kind":"Const","dst":1,"value":41,"next":52}
    assert live[52]=={"kind":"Write","address":1,"value":2,"next":174}
    assert live[28]["next"]==125 and live[79]["value"]==41
    per_case=[];by_regime=defaultdict(lambda:{"cases":0,"paid_effect_suppressions":0,
        "successful_physical_readbacks_after_suppression":0,
        "readbacks_witnessing_a_failed_intended_write":0})
    all_delays=[];events=0;exposures=0;contradictions=0
    for i,c in enumerate(m["cases"]):
        gate,always,abstain,supp,placebo,legacy=d["truthful_records"][6*i:6*i+6]
        assert supp["source"]=="G2_readback_pc28_suppressed"
        assert gate["source"]=="G2_readback_attest"
        assert gate["actor_readback_records"]==placebo["actor_readback_records"]
        assert gate["paid_gate_records"]==placebo["paid_gate_records"]
        assert gate["steps"]==placebo["steps"]
        assert gate["attested_gate_mismatches"]==0
        assert supp["attested_gate_mismatches"]==0
        attempts=supp["suppressed_pc28"]
        rb=supp["actor_readback_records"]
        gate_events=supp["paid_gate_records"]
        assert len(rb)==len(gate_events)==supp["actor_readback_writes"]
        seen=0;count=0;delays=[]
        for j,(read,decision) in enumerate(zip(rb,gate_events)):
            assert read["step"]<decision["step"]
            assert decision["witness"]==read["actual_theta"]==read["written_cell41"]
            assert decision["evidence_key"]==41
            assert decision["last_pc173"]==read["last_pc173"]
            assert decision["last_write"]==read["step"]
            prior=[x for x in attempts if x["step"]<read["step"]]
            if prior:
                last=prior[-1]
                seen+=1
                assert read["last_pc173"]>last["step"]
                assert read["actual_theta"]==last["after"],(
                    "physical executed code differs from retained source",c["name"])
                if read["actual_theta"]!=last["attempted"]:
                    count+=1
                    lag=read["step"]-last["step"]
                    assert lag>0
                    delays.append(lag);all_delays.append(lag)
            # If this is the last readback, no guarantee it was followed by edit.
        assert seen==supp["attested_after_suppressed_patch"]
        assert count==seen,(
            "a suppressed paid attempted write did not produce a countervailing real source-value witness",c["name"])
        events+=len(attempts)
        exposures+=seen;contradictions+=count
        f=by_regime[c["regime"]];f["cases"]+=1
        f["paid_effect_suppressions"]+=len(attempts)
        f["successful_physical_readbacks_after_suppression"]+=seen
        f["readbacks_witnessing_a_failed_intended_write"]+=count
        per_case.append({"case":c["name"],"regime":c["regime"],
            "paid_suppressed_code_patch_attempts":len(attempts),
            "later_paid_physical_code_readback_events":seen,
            "actually_different_from_intended_write":count,
            "paid_instruction_lag_since_latest_suppressed_patch":delays,
            "actor_gate_readbacks_exact_and_fresh":True,
            "gate_cost":gate["steps"],"suppressed_cost":supp["steps"],
            "intent_mirror_different_source_cost":legacy["steps"]})
    assert events==d["actual_paid_pc28_effect_suppressions"]
    assert exposures==d["true_physical_attestations_after_effect_suppression"]
    assert exposures==contradictions
    assert d["actor_attested_gate_mismatches_all_matching_arms"]==0
    assert sum(x["cases"] for x in by_regime.values())==45
    return {
        "study":m["study"],"evidence_class":"POST_HOC_SOURCE_PINNED_NATIVE_ACTOR_EXECUTION_WITNESS",
        "truthful_frozen_new_cases":45,"total_original_native_worlds":274,
        "paid_suppressed_pc28_attempts":events,
        "actor_paid_PC173_to_Write41_readbacks_after_suppression":exposures,
        "actual_scalar_disagrees_with_last_failed_write_intent":contradictions,
        "source_theta_mismatch_at_all_original_attested_paid_gates":0,
        "readback_lag_in_paid_CIL":{
            "minimum":min(all_delays),"median":statistics.median(all_delays),
            "maximum":max(all_delays)
        },
        "C7_overhead_vs_old_C6_different_source":d["costs"]["G2_readback_attest"]-
                                                     d["costs"]["G2_intent_mirror_C6"],
        "C7_overhead_vs_same_skeleton_abstain":-d["readback_net_saving_vs_abstain"],
        "families":dict(by_regime),"cases":per_case,
        "nonclaims":[
            "19 source-readback contradictions are after an explicit researcher effect suppression, not naturally detected cyberattacks.",
            "Observed value exists only after paid PC173 execution. This is not a direct read-constant opcode or a cryptographic attestation.",
            "C6 legacy mirror is different source and charge profile, not a same-source causal ablation.",
            "No independently learned mutation-profit predictor, feedback authenticity or CL1B B3/B4."
        ]
    }

if __name__=="__main__":
    if len(sys.argv)!=2:
        raise SystemExit("usage: cl1b_g2_executed_readback_c7_failed_write_witness_audit.py native_C7_summary.json")
    print(json.dumps(analyze(sys.argv[1]),ensure_ascii=False,indent=2))
