#!/usr/bin/env python3
"""C19A known-C17 + C19B unseen original 274x2 native full paid-source audit.

Retroactive audit only; no new native trial and no new prospective cohort.
Independently fetch original permanent Git ZIPs; no source construction or
posthoc goal picking. Verify actual native event history, charged PC173.next
temporary patch, real live Const173 self-execution, paid restore, value
consistency, same-source PC225 first-effect causal prefixes, all 45 pairs.
"""
import hashlib
import json
import sys
import zipfile
from pathlib import Path

COHORT={
 "a": {
   "sha":"4e0eea03df3a6009a94b972539184d96cd76450974f8a1777aff76e2b5ee568b",
   "study":"CL1B-C19A-KNOWN-C17-LIVE-SOURCE-CALIBRATION",
   "tested_sha":"8d2cf7f71ef744b50e59b11ea9a5a75130ff510b",
   "run_id":45127112522, "paid":1129708, "observed":15,
   "same":4,"different":11,"source_edits":11,
   "fee_main":190063,"fee_same_suppressed":191875,"fee_c9":162029,
   "full_cached":45, "cached_stages":2160, "known":True,
 },
 "b": {
   "sha":"04037a8fefc9e59c22986d32aa944814dbc596480576addc8cbaf86c7c25c363",
   "study":"CL1B-C19B-PROSPECTIVE-LIVE-SOURCE-HELDOUT",
   "tested_sha":"ce8a6fdc15711a1645756a1fe871bbde3bd9928b",
   "run_id":45127120747, "paid":1199905, "observed":32,
   "same":20,"different":12,"source_edits":12,
   "fee_main":209189,"fee_same_suppressed":206434,"fee_c9":147096,
   "full_cached":43, "cached_stages":2157, "known":False,
 }
}
ARMS=(
 "truth_paid_live_self_executed_source_difference_gate",
 "same_source_pc225_equal_marker_suppressed",
 "same_source_pc212_veto_effect_suppressed",
 "same_source_pc102_physical_effect_suppressed",
 "different_source_c18b_historical_cached_gate",
 "different_source_c9_fixed_abstain"
)
CELLS=(11,20,21,30,32,33,34,35,36,37,38,39,40,41,42,44,45,46,47,48)
def original(path,c):
    blob=Path(path).read_bytes()
    assert hashlib.sha256(blob).hexdigest()==c["sha"]
    z=zipfile.ZipFile(path)
    names=z.namelist()
    assert z.testzip() is None and len(names)==len(set(names))==278
    assert sum(name.endswith(".jsonl") for name in names)==274
    tag=c["study"][7:10].lower()
    # Study labels start CL1B-C19A/C19B: derive 'a'/'b' only.
    tag="a" if c["known"] else "b"
    d=json.loads(z.read(f"cl1b_c19{tag}_original_native_274_worlds.json"))
    p=json.loads(z.read(f"cl1b_c19{tag}_independent_replay.json"))
    assert d["study"]==p["study"]==c["study"]
    assert p["first_divergence"] is None
    assert d["native_worlds"]==p["native_worlds"]==274
    assert d["total_paid_cil"]==p["exact_private_paid_cil_replayed"]==c["paid"]
    assert len(d["records"])==45 and len(d["adversarial_records"])==4
    provenance=z.read(f"cl1b-c19{tag}-original-provenance.txt").decode()
    assert f"tested_sha={c['tested_sha']}" in provenance
    assert f"run_id={c['run_id']}" in provenance
    return z,d,tag
def trace(z,tag,trial):
    return [json.loads(r) for r in z.read(f"cl1b_c19{tag}_original_{trial['name']}.jsonl").splitlines() if r.strip()]
def co(a,z,d,tag):
    world_records=[]
    effects=[]
    temporary_arm=temporary_restore=live_const=live_value_copied=0
    false_pos=false_neg=0
    equal=different=0
    main_sources=set()
    changed_proof=[]
    source_restore_failure=[]
    for case in d["records"]:
        row=case["arms"]
        assert len(row)==6 and tuple(r["arm"] for r in row)==ARMS
        assert all(r["feedback"]=="truth" and r["goals"]==row[0]["goals"] for r in row)
        main,ctrl=row[0],row[1]
        main_sources.add(case["name"])
        assert main["completed_stages"]==48 and main["true_matches"]==48
        assert main["final_live_PC173_next"]==24
        assert not main["unrestored_live_PC173_edge_at_termination"]
        assert main["paid_live_source_edge_armed"]==main["paid_live_source_edge_restored"]
        assert main["paid_live_source_const_executed"]==main["paid_live_source_edge_restored"]
        assert main["paid_live_source_register_copied"]==main["paid_live_source_edge_restored"]
        assert main["paid_live_candidate_cell48_writes"]==main["paid_live_source_edge_restored"]
        assert main["paid_live_candidate_cell48_reads"]==main["paid_live_source_edge_restored"]
        temporary_arm+=main["paid_live_source_edge_armed"]
        temporary_restore+=main["paid_live_source_edge_restored"]
        live_const+=main["paid_live_source_const_executed"]
        live_value_copied+=main["paid_live_source_register_copied"]
        false_pos+=main["observer_cache_false_equal"]
        false_neg+=main["observer_cache_false_different"]
        equal+=main["observer_true_prepatch_equal"]
        different+=main["observer_true_prepatch_different"]
        assert main["paid_cached_same_probe_bypasses"]==main["observer_true_prepatch_equal"]
        assert main["paid_cached_different_probe_activations"]==main["observer_true_prepatch_different"]
        tr=trace(z,tag,main)
        source_samples=main["observer_source_cache_truth_samples"]
        assert len(source_samples)==main["paid_live_source_edge_armed"]
        samples_read=0
        for idx,event in enumerate(tr):
            if event[0]!="cil":continue
            if event[2]==233:
                assert event[3]==173 and event[8][6]==234
                assert idx+1<len(tr) and tr[idx+1][0]=="cil" and tr[idx+1][2]==173
                e173=tr[idx+1]
                assert e173[3]==234 and e173[8][6]==234
                assert e173[4][2]==event[8][4]  # true charged live Const literal
                # 234,235 paid, then 236 paid restore to original next24
                controls=[x for x in tr[idx+2:idx+6] if x[0]=="cil"]
                assert [x[2] for x in controls]==[234,235,236,237]
                assert controls[2][8][6]==24 and controls[2][3]==237
                assert controls[3][8][6]==24
                samples_read+=1
                changed_proof.append([case["name"],event[1],e173[4][2],event[8][4],controls[2][1]])
        assert samples_read==main["paid_live_source_edge_armed"]
        for p in source_samples:
            paid,before,proposed,live,actor_equal,real_equal,suppressed=p
            assert not suppressed
            assert live==before and actor_equal==real_equal==(before==proposed)
        if main["paid_live_source_edge_armed"]!=main["paid_live_source_edge_restored"]:
            source_restore_failure.append(case["name"])
        effect=(main["steps"]!=ctrl["steps"] or main["services"]!=ctrl["services"] or main["final_signature"]!=ctrl["final_signature"])
        if effect:
            tc=trace(z,tag,ctrl)
            first=next(e[1] for e in tr if e[0]=="cil" and e[2]==225)
            before_main=[e for e in tr if e[1]<first]
            before_ctrl=[e for e in tc if e[1]<first]
            assert before_main==before_ctrl,("DIVERGENCE BEFORE PAID PC225 EFFECT",case["name"])
            em=next(e for e in tr if e[0]=="cil" and e[1]==first)
            ec=next(e for e in tc if e[0]=="cil" and e[1]==first)
            assert em[2]==ec[2]==225 and em[:6]==ec[:6]
            i=CELLS.index(47)
            assert em[6][i]==1 and ec[6][i]==0
            assert em[7]==ec[7] and em[8]==ec[8]
            assert em[9]=="normal" and ec[9]=="suppressed_pc225"
            effects.append({"case":case["name"],"first_paid_PC225_effect_step":first,
                "identical_original_native_pre_effect_events":len(before_main),
                "main_private_paid_CIL":main["steps"],"same_source_effect_suppressed_private_CIL":ctrl["steps"],
                "actor_cost_minus_suppression":main["steps"]-ctrl["steps"],
                "true_stage_delta":main["completed_stages"]-ctrl["completed_stages"]})
        world_records.append({"case":case["name"],"paid_real_PC173_reads":main["paid_live_source_const_executed"],
            "real_source_changes":main["effective_pc173_restorations"],
            "true_same":main["observer_true_prepatch_equal"],
            "true_different":main["observer_true_prepatch_different"],
            "false_equal":main["observer_cache_false_equal"],
            "false_different":main["observer_cache_false_different"],
            "main_paid":main["steps"],"same_source_PC225_suppressed_paid":ctrl["steps"],
            "main_completed":main["completed_stages"],"suppress_completed":ctrl["completed_stages"],
            "same_source_effect":effect})
    assert len(main_sources)==45
    assert temporary_arm==temporary_restore==live_const==live_value_copied==a["observed"]
    assert false_pos==false_neg==0
    assert equal==a["same"] and different==a["different"]
    assert len(changed_proof)==a["observed"] and not source_restore_failure
    assert sum(r["effective_pc173_restorations"] for c in d["records"] for r in c["arms"][:1])==a["source_edits"]
    assert d["arm_fees"][ARMS[0]]==a["fee_main"]
    assert d["arm_fees"][ARMS[1]]==a["fee_same_suppressed"]
    assert d["arm_fees"][ARMS[5]]==a["fee_c9"]
    assert d["arm_completed_worlds"][ARMS[4]]==a["full_cached"]
    assert sum(c["arms"][4]["completed_stages"] for c in d["records"])==a["cached_stages"]
    assert len(effects)==4
    assert sum(v["actor_cost_minus_suppression"] for v in effects)==a["fee_main"]-a["fee_same_suppressed"]
    assert all(x["true_stage_delta"]==0 for x in effects)
    return {"cohort":"KNOWN_NOT_HOLDOUT" if a["known"] else "NEW_HELDOUT",
        "original_source_zip_sha256":a["sha"],
        "original_native_paid_CIL":a["paid"],
        "known_C17_prior_exposure":a["known"],
        "full_45_case_source_matched_pairs":45,
        "paid_temporary_edge_armed":temporary_arm,
        "paid_temporary_edge_restored":temporary_restore,
        "actually_executed_live_Const173":live_const,
        "paid_live_parameter_copied":live_value_copied,
        "actor_correct_equal_source_cases":equal,"actor_correct_different_source_cases":different,
        "actor_false_equal":false_pos,"actor_false_different":false_neg,
        "unrestored_live_source_edge_after_trial":source_restore_failure,
        "real_physical_source_edits":a["source_edits"],
        "same_source_PC225_gate_causal_cases":len(effects),
        "same_source_first_native_effect_has_identical_paid_prefix":True,
        "main_CIL":a["fee_main"],
        "same_source_PC225_effect_suppressed_CIL":a["fee_same_suppressed"],
        "main_minus_suppressed_CIL":a["fee_main"]-a["fee_same_suppressed"],
        "strong_fixed_C9_CIL":a["fee_c9"],
        "original_complete_45_case_rows":world_records,
        "all_real_paid_live_self_execution_events":changed_proof,
        "strict_four_PC225_first_effect_cases":effects,
        "four_false_feedback_controls":[{"name":v["name"],"claimed":v["claim"],
            "true_matches":v["true_matches"],"completed_stages":v["completed_stages"]}
            for v in d["adversarial_records"]]}
def run(a_path,b_path):
    Za,Da,ta=original(a_path,COHORT["a"])
    Zb,Db,tb=original(b_path,COHORT["b"])
    A=co(COHORT["a"],Za,Da,ta)
    B=co(COHORT["b"],Zb,Db,tb)
    return {
      "study":"CL1B-C19-ORIGINAL-SOURCE-PINNED-LIVE-SELF-READ-DUAL-COHORT",
      "status":"ORIGINAL_274_KNOWN_PLUS_274_PROSPECTIVE_NATIVE_CAUSAL_AUDIT",
      "independent_274_full_CIL_replay_first_divergence":None,
      "total_paid_original_CIL":COHORT["a"]["paid"]+COHORT["b"]["paid"],
      "total_paid_live_active_source_read_and_restored":A["paid_temporary_edge_armed"]+B["paid_temporary_edge_armed"],
      "all_true_live_source_comparisons_are_exact":True,
      "no_live_source_edge_unrestored":True,
      "heldout_gate_not_economically_preferred_to_same_source_ablation":True,
      "original_known_calibration":A,
      "original_new_heldout":B,
      "B3_B4":"NOT_MET",
      "important_limit":"Accurate paid active-source observation proved; autonomous cumulative optimization and superiority to C9 not demonstrated."
    }
if __name__=="__main__":
    assert len(sys.argv)==4,"usage: audit.py C19a-original.zip C19b-original.zip output.json"
    d=run(sys.argv[1],sys.argv[2])
    Path(sys.argv[3]).write_text(json.dumps(d,ensure_ascii=False,indent=2)+"\n",encoding="utf-8")
    print(json.dumps({k:v for k,v in d.items() if k not in ("original_known_calibration","original_new_heldout")},indent=2,ensure_ascii=False))
