#!/usr/bin/env python3
"""C15 exact-source retrospective native physical-edit effect audit.
Read *original* archived 274-world native traces and prior exact Python replay.
No new native CIL, no posthoc chosen task subset; full 45 matched true worlds.
"""
import hashlib,json,sys,zipfile
from pathlib import Path

ORIG="d01f8527f8dcc795176d8f18e50b4deaaa41c2590c0dd2f80f5a7e32de4a0b5e"
STUDY="CL1B-C15-AUTHENTIC-SOURCE-PAID-REENTRY"

def read_trace(z, record):
    name="cl1b_c15_original_"+record["name"]+".jsonl"
    return [json.loads(s) for s in z.read(name).decode().splitlines() if s.strip()]

def audit(path):
    raw=Path(path).read_bytes()
    assert hashlib.sha256(raw).hexdigest()==ORIG
    with zipfile.ZipFile(path) as z:
        assert z.testzip() is None
        names=z.namelist()
        assert len(names)==len(set(names))==278
        assert sum(x.endswith(".jsonl") for x in names)==274
        m=json.loads(z.read("cl1b_c15_original_native_274_worlds.json"))
        p=json.loads(z.read("cl1b_c15_independent_replay.json"))
        assert m["study"]==p["study"]==STUDY
        assert m["native_worlds"]==p["native_worlds"]==274
        assert p["first_divergence"] is None
        assert m["total_paid_cil"]==p["exact_private_paid_cil_replayed"]==1085881
        assert len(m["records"])==45
        assert len(m["adversarial_records"])==4
        total_stages={"live":0,"suppressed":0,"baseline_c14c":0,"baseline_c9":0}
        all_incomplete=[]
        effect_rows=[]
        reentries=restores=readbacks=post52=0
        difference_cases=harm_cases=tie_difference_cases=benefit_cases=0
        entire_fee_equal_pairs=0
        all_case_prefix_verified=0
        every_case=[]
        for case in m["records"]:
            assert len(case["arms"])==6
            live,sup,force0,force2,old,c9=case["arms"]
            assert live["arm"]=="truth_success_restore"
            assert sup["arm"]=="same_source_pc102_effect_suppressed"
            assert old["arm"]=="different_source_c14c_no_reentry"
            assert c9["arm"]=="different_source_c9_fixed_abstain"
            assert live["goals"]==sup["goals"]==old["goals"]==c9["goals"]
            assert live["external_cell44_tampers"]==sup["external_cell44_tampers"]==0
            assert sup["effective_pc173_restorations"]==0
            assert live["steps"]==sup["steps"]
            entire_fee_equal_pairs+=1
            for a in case["arms"]:
                if not (a["completed_stages"]==48 and a["claim"]==1 and not a["budget_exhausted"]):
                    all_incomplete.append(a["name"])
            for key,v in [("live",live),("suppressed",sup),("baseline_c14c",old),("baseline_c9",c9)]:
                total_stages[key]+=v["completed_stages"]
            reentries+=live["paid_pc102_reentry_pc173_actual_executions"]
            restores+=live["effective_pc173_restorations"]
            readbacks+=len(live["subsequent_physical_pc173_readbacks"])
            post52+=live["paid_pc52_after_reentry_actual_writes"]
            first_effect=next((e[0] for e in live["physical_parent_restorations"] if e[4]),None)
            service_different=live["services"]!=sup["services"]
            fee_different=live["steps"]!=sup["steps"]
            if service_different or fee_different:
                difference_cases+=1
                assert first_effect is not None
                ltrace=read_trace(z,live)
                strace=read_trace(z,sup)
                # Up through the first source-effective native PC102 event, the
                # starting code, all paid operations, credits, cells, and feedback
                # must be identical. Suppressed-PC102 observer annotations can
                # differ even for no-op operations, so omit *only* that label.
                def before(trace):
                    return [e[:-1] if e[0]=="cil" else e
                            for e in trace if e[1]<first_effect]
                lp=before(ltrace)
                sp=before(strace)
                assert lp==sp,(case["name"],"first real physical code effect has unequal charged prehistory")
                all_case_prefix_verified+=1
                assert live["budget_exhausted"] and sup["budget_exhausted"]
                completed_delta=live["completed_stages"]-sup["completed_stages"]
                if completed_delta<0:harm_cases+=1
                elif completed_delta>0:benefit_cases+=1
                else:tie_difference_cases+=1
                effect_rows.append({
                    "case":case["name"],
                    "first_source_effect_cil_step":first_effect,
                    "identical_pre_effect_raw_event_count":len(lp),
                    "live_paid":live["steps"],"suppressed_paid":sup["steps"],
                    "live_completed_stages":live["completed_stages"],
                    "suppressed_completed_stages":sup["completed_stages"],
                    "difference_completed_stages":completed_delta,
                    "live_actual_source_changes":live["effective_pc173_restorations"],
                    "live_native_reexecutions":live["paid_pc102_reentry_pc173_actual_executions"],
                    "live_actual_postrestore_readbacks":len(live["subsequent_physical_pc173_readbacks"]),
                    "causally_different_true_service_sequence":service_different
                })
            else:
                assert live["completed_stages"]==sup["completed_stages"]
            every_case.append({
                "name":case["name"],
                "same_source_paid_effect_live":live["steps"],
                "same_source_paid_effect_suppressed":sup["steps"],
                "same_source_diff_completed":live["completed_stages"]-sup["completed_stages"],
                "live_real_source_changes":live["effective_pc173_restorations"],
                "suppressed_real_source_changes":sup["effective_pc173_restorations"],
                "live_48_stage_complete":live["completed_stages"]==48,
                "suppressed_48_stage_complete":sup["completed_stages"]==48
            })
        assert entire_fee_equal_pairs==45
        assert difference_cases==all_case_prefix_verified==6
        assert (harm_cases,tie_difference_cases,benefit_cases)==(3,3,0)
        assert total_stages=={
            "live":1987,"suppressed":2037,"baseline_c14c":2160,"baseline_c9":2160
        }
        assert (reentries,restores,readbacks,post52)==(252,165,164,252)
        assert len(all_incomplete)==12
        assert m["arm_fees"]=={
            "truth_success_restore":192550,
            "same_source_pc102_effect_suppressed":192550,
            "same_source_host_force_success_value_0":183666,
            "same_source_host_force_success_value_2":183666,
            "different_source_c14c_no_reentry":183235,
            "different_source_c9_fixed_abstain":142838
        }
        assert m["arm_completed_worlds"]=={
            "truth_success_restore":39,
            "same_source_pc102_effect_suppressed":39,
            "same_source_host_force_success_value_0":45,
            "same_source_host_force_success_value_2":45,
            "different_source_c14c_no_reentry":45,
            "different_source_c9_fixed_abstain":45
        }
        return {
            "study":"CL1B-C15-ORIGINAL-NATIVE-PAID-PHYSICAL-SOURCE-FIRST-EFFECT",
            "classification":"RETROSPECTIVE_FULL_45_PAIR_SOURCE_PINNED",
            "original_native_zip_sha256":ORIG,
            "prior_274_world_independent_CIL_parity":True,
            "original_native_paid_CIL":1085881,
            "full_45_true_case_same_source_fee_matched":entire_fee_equal_pairs,
            "full_45_case_affected_service_or_paid_lifecycle":difference_cases,
            "six_affected_cases_identical_native_pre_first_physical_edit":all_case_prefix_verified,
            "physical_source_changes_main":restores,
            "paid_native_PC173_reentries_main":reentries,
            "real_source_changes_later_readback":readbacks,
            "post_reentry_paid_PC52_memories_main":post52,
            "affected_cases_worse":harm_cases,
            "affected_cases_equal_completed_but_service_different":tie_difference_cases,
            "affected_cases_better":benefit_cases,
            "true_completed_stages":total_stages,
            "live_vs_physical_suppression_total_stage_delta":-50,
            "all_truthful_incomplete_world_names":all_incomplete,
            "all_45_case_effects":every_case,
            "all_six_physical_causal_effect_examples":effect_rows,
            "B3_B4":"NOT_MET"
        }

if __name__=="__main__":
    assert len(sys.argv)==3,"usage: audit original_native.zip output.json"
    d=audit(sys.argv[1])
    Path(sys.argv[2]).write_text(json.dumps(d,indent=2,ensure_ascii=False)+"\n")
    print(json.dumps({k:v for k,v in d.items() if k not in (
        "all_45_case_effects","all_six_physical_causal_effect_examples",
        "all_truthful_incomplete_world_names")},indent=2,ensure_ascii=False))
