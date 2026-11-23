#!/usr/bin/env python3
"""Retrospective exact-source 270-pair native C14b OFF vs C14c ON audit.
Requires both original Git-archived Actions ZIPs, refuses stale/missing/mismatched bytes.
This is a cross-run causal cost decomposition, NOT new native CIL / B3 evidence.
"""
import hashlib
import json
import sys
import zipfile
from pathlib import Path

OFF_SHA = "d35dfa5dd01bb0d0443a839ebbd35c99d29458ee9acac6c2ff740ebe0e60e941"
ON_SHA = "9dbedd5624de349299da434677f6a8de6e6cf5a7df6784ba00d689d9ab8207be"
STUDY_OFF="CL1B-C14B-PAID-WRITER-VERIFIED-SUCCESS-SOURCE-RESTORATION"
STUDY_ON="CL1B-C14C-PAID-WRITER-ON-SUCCESS-SOURCE-RESTORATION"
ARMS=(
    "truth_success_restore",
    "same_source_pc102_effect_suppressed",
    "same_source_host_force_success_value_0",
    "same_source_host_force_success_value_2",
    "different_source_c10_parent_restore",
    "different_source_c9_fixed_abstain",
)

def load_zip(path,sha,native,independent,study):
    data=Path(path).read_bytes()
    assert hashlib.sha256(data).hexdigest()==sha,("original ZIP SHA mismatch",path)
    with zipfile.ZipFile(path) as z:
        assert z.testzip() is None
        names=z.namelist()
        assert len(names)==len(set(names))==278,len(names)
        assert sum(n.endswith(".jsonl") for n in names)==274
        n=json.loads(z.read(native))
        i=json.loads(z.read(independent))
        assert n["study"]==i["study"]==study
        assert n["native_worlds"]==i["native_worlds"]==274
        assert i["first_divergence"] is None
        assert n["total_paid_cil"]==i["exact_private_paid_cil_replayed"]
        assert len(n["records"])==45
        assert len(n["adversarial_records"])==4
        return n

def compare(off,on):
    assert off["total_paid_cil"]==1050989
    assert on["total_paid_cil"]==1054669
    assert on["total_paid_cil"]-off["total_paid_cil"]==3680
    pair_rows=[]
    same_services=valid_fee_identity=final_code_changes=0
    off_writer=on_writer=on_missing=on_stale=0
    on_records=off_records=0
    off_peak={}
    on_peak={}
    for old_case,new_case in zip(off["records"],on["records"]):
        assert old_case["name"]==new_case["name"]
        assert old_case["family"]==new_case["family"]
        assert len(old_case["arms"])==len(new_case["arms"])==6
        for arm,a,b in zip(ARMS,old_case["arms"],new_case["arms"]):
            assert a["name"]==b["name"]==old_case["name"]+"_"+arm
            assert a["arm"]==b["arm"]==arm
            assert a["feedback"]==b["feedback"]=="truth"
            assert a["goals"]==b["goals"]
            assert a["services"]==b["services"],("native service behavior difference",a["name"])
            same_services+=1
            delta=b["steps"]-a["steps"]
            assert delta==4*b["claimed_success_records"],(a["name"],delta,b["claimed_success_records"])
            valid_fee_identity+=1
            assert a["claimed_success_records"]==0
            if arm in ARMS[:4]:
                assert b["truly_correct_source_records"]==b["claimed_success_records"]
                assert len(b["paid_writer_checks"])==len(a["paid_writer_checks"])==48
                off_writer+=sum(w[1]==-1 for w in a["paid_writer_checks"])
                on_writer+=sum(w[1]==1 for w in b["paid_writer_checks"])
                on_missing+=sum(w[1]==-1 for w in b["paid_writer_checks"])
                on_stale+=b["stale_success_code_samples"]
            off_records+=a["claimed_success_records"]
            on_records+=b["claimed_success_records"]
            if a["final_signature"]!=b["final_signature"]:
                final_code_changes+=1
            off_peak[arm]=max(off_peak.get(arm,0),a["peak_occupied"])
            on_peak[arm]=max(on_peak.get(arm,0),b["peak_occupied"])
            pair_rows.append({
                "case":old_case["name"],"family":old_case["family"],"arm":arm,
                "off_paid":a["steps"],"on_paid":b["steps"],"delta_paid":delta,
                "on_paid_success_source_writes":b["claimed_success_records"],
                "off_peak":a["peak_occupied"],"on_peak":b["peak_occupied"],
                "final_code_changed":a["final_signature"]!=b["final_signature"],
            })
    assert same_services==valid_fee_identity==270
    assert off_records==0 and on_records==230*4
    assert off_writer==2160*4 and on_writer==230*4 and on_missing==1930*4
    assert on_stale==23*4
    assert final_code_changes==6
    assert off_peak["truth_success_restore"]==204
    assert on_peak["truth_success_restore"]==220
    for a,b in zip(off["adversarial_records"],on["adversarial_records"]):
        assert a["name"]==b["name"] and a["arm"]==b["arm"] and a["goals"]==b["goals"]
        assert a["services"]==b["services"]
        assert a["steps"]==b["steps"]
    for arm in ARMS[:4]:
        assert on["arm_fees"][arm]-off["arm_fees"][arm]==920
    for arm in ARMS[4:]:
        assert on["arm_fees"][arm]==off["arm_fees"][arm]
    assert all(len(c["arms"][0]["subsequent_physical_pc173_readbacks"])==0 for c in on["records"])
    assert off["main_actual_physical_pc173_restores"]==on["main_actual_physical_pc173_restores"]==5
    return {
        "study":"CL1B-C14B-C14C-PROVENANCE-OPTIN-SAME-SOURCE-COST-AUDIT",
        "analysis_type":"RETROSPECTIVE_FULL_DENOMINATOR_SOURCE_PINNED",
        "C14b_off_zip_sha256":OFF_SHA,
        "C14c_on_zip_sha256":ON_SHA,
        "truthful_pairs":270,"false_feedback_worlds_per_revision":4,
        "full_service_sequences_equal_pairs":same_services,
        "all_pairs_fee_delta_4x_original_paid_write44":valid_fee_identity,
        "total_native_CIL_off":off["total_paid_cil"],
        "total_native_CIL_on":on["total_paid_cil"],
        "total_native_CIL_on_minus_off":3680,
        "main_arm_CIL_on_minus_off":920,
        "off_true_successes_missing_writer":2160,
        "on_true_successes_missing_writer":1930,
        "on_authentic_success_source_memories":230,
        "on_stale_record_against_current_physical_code":23,
        "physical_restores_without_subsequent_executed_code_readback":5,
        "all_arm_final_code_differences":final_code_changes,
        "main_peak_storage_units_off":204,
        "main_peak_storage_units_on":220,
        "arm_off_peak":off_peak,"arm_on_peak":on_peak,
        "all_pairs":pair_rows,
        "B3_B4":"NOT_MET",
    }

def main():
    assert len(sys.argv)==4,"usage: audit.py off_original.zip on_original.zip result.json"
    off=load_zip(sys.argv[1],OFF_SHA,
        "cl1b_c14b_original_native_274_worlds.json","cl1b_c14b_independent_replay.json",STUDY_OFF)
    on=load_zip(sys.argv[2],ON_SHA,
        "cl1b_c14c_original_native_274_worlds.json","cl1b_c14c_independent_replay.json",STUDY_ON)
    result=compare(off,on)
    Path(sys.argv[3]).write_text(json.dumps(result,indent=2,ensure_ascii=False)+"\n")
    print(json.dumps({k:v for k,v in result.items() if k not in ("all_pairs","arm_off_peak","arm_on_peak")},
                     ensure_ascii=False,indent=2))

if __name__=="__main__":
    main()
