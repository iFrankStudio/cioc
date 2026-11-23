#!/usr/bin/env python3
"""Retrospective exact original C18 unseen-zero + C18b known-exposure 45-pair audit.

Not a new prospective trial and not 90 independent holdout tasks.
Read both immutable source-pinned complete ZIPs and all native CIL event streams.
Verify prior full 274-world independent interpreter parity before making causal claims.
"""
from pathlib import Path
import hashlib
import json
import sys
import zipfile

C18_SHA="810cfcadb2e8912fd998c21d6600401351556a9fbfc3191021019adefaa5fbdc"
C18B_SHA="dd1909075ebf23524ad7ea74f8a1a2f2c542ce52cf53eb996dc85adc6d581aad"
CELLS=[11,20,21,30,32,33,34,35,36,37,38,39,40,41,42,44,45,46,47]
ARMS=[
    "truth_paid_cached_source_difference_gate",
    "same_source_pc225_equal_marker_suppressed",
    "same_source_pc212_veto_effect_suppressed",
    "same_source_pc102_physical_effect_suppressed",
    "different_source_c17_ungated_postedit_veto",
    "different_source_c9_fixed_abstain",
]
FEES_B=[194239,191755,195123,189252,195606,162029]
FEES_UNSEEN=[211379,211379,211379,211379,211379,142998]

def load(path,want):
    assert hashlib.sha256(Path(path).read_bytes()).hexdigest()==want
    z=zipfile.ZipFile(path)
    assert z.testzip() is None
    names=z.namelist()
    assert len(names)==len(set(names))==278
    assert sum(n.endswith(".jsonl") for n in names)==274
    prefix="cl1b_c18b_" if want==C18B_SHA else "cl1b_c18_"
    native=json.loads(z.read(prefix+"original_native_274_worlds.json"))
    replay=json.loads(z.read(prefix+"independent_replay.json"))
    assert replay["first_divergence"] is None
    assert native["native_worlds"]==replay["native_worlds"]==274
    assert native["total_paid_cil"]==replay["exact_private_paid_cil_replayed"]
    assert len(native["records"])==45 and len(native["adversarial_records"])==4
    assert all(len(c["arms"])==6 for c in native["records"])
    assert all(tuple(a["arm"] for a in c["arms"])==tuple(ARMS) for c in native["records"])
    return z,native,prefix

def trace(z,prefix,trial):
    return [json.loads(t) for t in z.read(prefix+"original_"+trial["name"]+".jsonl").splitlines() if t]

def audit(unseen_path,known_path):
    zu,nu,up=load(unseen_path,C18_SHA)
    zk,nk,kp=load(known_path,C18B_SHA)
    assert nu["study"]=="CL1B-C18-ACTOR-PAID-CACHED-SOURCE-DIFF-POSTEDIT-GATE"
    assert nk["study"]=="CL1B-C18B-KNOWN-C17-EXPOSURE-CALIBRATION"
    assert nu["total_paid_cil"]==1205337 and nk["total_paid_cil"]==1135094
    assert list(nu["arm_fees"].values()) and list(nk["arm_fees"].values())
    for n,fees in [(nu,FEES_UNSEEN),(nk,FEES_B)]:
        assert n["arm_fees"]==dict(zip(ARMS,fees))
        assert n["arm_completed_worlds"]==dict.fromkeys(ARMS,45)
        assert len(n["incomplete_true_world_names"])==0
        assert all(a["true_matches"]==48 and a["completed_stages"]==48
                   for c in n["records"] for a in c["arms"])
    assert {c["name"] for c in nu["records"]}.isdisjoint({c["name"] for c in nk["records"]})
    assert all(c["name"].startswith("c18_") for c in nu["records"])
    assert all(c["name"].startswith("c17_") for c in nk["records"])

    # Unseen holdout negative is a real no-exposure result, not a failed test.
    for c in nu["records"]:
        for a in c["arms"]:
            for k in (
                "claimed_success_records","paid_pc102_target173_attempts",
                "effective_pc173_restorations",
                "paid_gate_unconditional_clear47","paid_cached_source_read41",
                "paid_cached_equal_marker_writes","paid_cached_equal_marker_effect_suppressed",
                "paid_cached_same_probe_bypasses","paid_cached_different_probe_activations",
                "paid_postedit_probe_armed","paid_postedit_probe_disarmed",
                "paid_postedit_veto_written","observer_true_prepatch_equal",
                "observer_true_prepatch_different","observer_cache_false_equal",
                "observer_cache_false_different",
            ):
                assert a[k]==0,(c["name"],a["arm"],k,a[k])

    counts={"actor_equal_actual_equal":0,"actor_equal_actual_different":0,
            "actor_different_actual_equal":0,"actor_different_actual_different":0}
    all_45=[]
    first_effect=[]
    total_exposed=0
    total_nonnative_source=0
    total_true_changed=0
    main_first_noop_monitor=0
    for c in nk["records"]:
        main,disable_equal,disable_veto,disable_physical,old_c17,old_c9=c["arms"]
        t=trace(zk,kp,main)
        proposals=main["observer_source_cache_truth_samples"]
        patch_events=[e for e in t if e[0]=="cil" and e[2]==102]
        assert len(proposals)==len(patch_events)==main["paid_pc102_target173_attempts"]
        assert main["paid_gate_unconditional_clear47"]==len(proposals)
        assert main["paid_cached_source_read41"]==len(proposals)
        assert main["paid_cached_equal_marker_writes"]==main["paid_cached_same_probe_bypasses"]
        assert main["paid_cached_different_probe_activations"]==main["paid_postedit_probe_armed"]
        for p,e in zip(proposals,patch_events):
            paid,before,candidate,cached,actor_equal,physical_equal,suppressed=p
            assert not suppressed and e[1]==paid and e[4][0]==candidate
            assert e[8][4]==candidate
            assert e[6][CELLS.index(41)]==cached
            assert e[6][CELLS.index(47)]==int(actor_equal)
            assert actor_equal==(cached==candidate)
            assert physical_equal==(before==candidate)
            k=("actor_equal_" if actor_equal else "actor_different_")
            k+=("actual_equal" if physical_equal else "actual_different")
            counts[k]+=1
            if not physical_equal:total_true_changed+=1
        total_exposed+=len(proposals)
        total_nonnative_source+=main["paid_cached_equal_marker_writes"]
        main_first_noop_monitor+=main["paid_cached_different_probe_activations"]

        diff=(main["steps"]!=disable_equal["steps"] or
              main["services"]!=disable_equal["services"] or
              main["final_signature"]!=disable_equal["final_signature"])
        if diff:
            m225=next(e for e in t if e[0]=="cil" and e[2]==225)
            step=m225[1]
            control_trace=trace(zk,kp,disable_equal)
            prefix1=[e for e in t if e[1]<step]
            prefix2=[e for e in control_trace if e[1]<step]
            assert prefix1==prefix2,("native pre-first paid PC225 effect differs",c["name"])
            ce=next(e for e in control_trace if e[0]=="cil" and e[1]==step)
            assert ce[2]==m225[2]==225
            assert ce[:6]==m225[:6] and ce[7:9]==m225[7:9]
            assert (m225[6][-1],ce[6][-1])==(1,0)
            assert (m225[9],ce[9])==("normal","suppressed_pc225")
            first_effect.append({
                "name":c["name"],
                "first_real_paid_PC225_step":step,
                "identical_pre_effect_native_event_count":len(prefix1),
                "main_true_stages":main["completed_stages"],
                "same_source_equal_marker_suppressed_stages":disable_equal["completed_stages"],
                "main_private_CIL":main["steps"],
                "same_source_equal_marker_suppressed_private_CIL":disable_equal["steps"],
                "main_minus_suppressed_CIL":main["steps"]-disable_equal["steps"],
                "actual_physical_edits_in_main":main["effective_pc173_restorations"]
            })
        all_45.append({
            "name":c["name"],"source_comparison_count":len(proposals),
            "cache_true_same":main["observer_true_prepatch_equal"],
            "cache_true_changed":main["observer_true_prepatch_different"],
            "cache_false_equal":main["observer_cache_false_equal"],
            "cache_false_different":main["observer_cache_false_different"],
            "main_stages":main["completed_stages"],
            "main_paid":main["steps"],
            "same_source_suppressed_PC225_paid":disable_equal["steps"],
            "same_source_suppressed_PC212_paid":disable_veto["steps"],
            "same_source_suppressed_PC102_paid":disable_physical["steps"],
            "different_source_C17_paid":old_c17["steps"],
            "different_source_C9_paid":old_c9["steps"],
            "affected_by_equal_marker":diff
        })
    assert total_exposed==65 and total_true_changed==33
    assert total_nonnative_source==64 and main_first_noop_monitor==1
    assert counts=={
        "actor_equal_actual_equal":31,
        "actor_equal_actual_different":33,
        "actor_different_actual_equal":1,
        "actor_different_actual_different":0
    }
    assert len(first_effect)==nk["same_source_live_vs_cached_equal_marker_effect_difference_cases"]==8
    assert sum(x["main_minus_suppressed_CIL"] for x in first_effect)==194239-191755==2484
    assert all(x["main_true_stages"]==x["same_source_equal_marker_suppressed_stages"]==48
               for x in first_effect)
    assert sum(a["effective_pc173_restorations"] for c in nk["records"] for a in c["arms"][:1])==33
    assert sum(a["paid_postedit_probe_armed"] for c in nk["records"] for a in c["arms"][:1])==1
    assert sum(a["paid_postedit_feedback_zero"] for c in nk["records"] for a in c["arms"][:1])==1
    assert sum(a["paid_postedit_veto_written"] for c in nk["records"] for a in c["arms"][:1])==1
    assert sum(a["paid_veto_gate_blocks"] for c in nk["records"] for a in c["arms"][:1])==12
    assert nk["arm_fees"][ARMS[2]]==195123
    assert nk["arm_fees"][ARMS[3]]==189252
    return dict(
        study="CL1B-C18-C18B-ORIGINAL-274-WORLD-SOURCE-PINNED-NOEXPOSURE-AND-CACHED-FALSE-EQUAL-AUDIT",
        original_C18_unseen_zip_sha256=C18_SHA,
        original_C18b_known_zip_sha256=C18B_SHA,
        independent_original_CIL_replay_first_divergence=None,
        unseen_C18_all_45_prospective_cases_have_zero_exposure=True,
        unseen_C18_true_worlds=270,
        unseen_C18_all_274_worlds=274,
        known_C18b_45_case_exposure_cohort_IS_PREVIOUS_C17_NOT_HOLDOUT=True,
        known_C18b_original_worlds=274,
        paid_comparisons=total_exposed,
        original_cache_vs_physical_confusion=counts,
        actual_physical_edits_all_misclassified_equal=total_true_changed,
        actor_stored_same_marker=total_nonnative_source,
        only_one_monitor_activated_on_actually_unchanged_source=main_first_noop_monitor,
        same_source_real_PC225_effect_cases=len(first_effect),
        exact_pre_first_PC225_native_event_prefix_equal=True,
        paid_CIL_C18b_main=194239,
        paid_CIL_C18b_strict_equal_marker_suppressed=191755,
        main_minus_suppressed_CIL_disadvantage=2484,
        paid_CIL_C18b_strong_fixed_C9=162029,
        C18b_all_45_true_worlds_complete_for_all_arms=True,
        known_C18b_effect_cases=first_effect,
        known_C18b_complete_45_original_pairs=all_45,
        scientific_status="NOT_HOLDOUT_CACHE_STALE_33_OUT_OF_33_CODE_CHANGES_MISSED_B3_B4_NOT_MET"
    )
if __name__=="__main__":
    assert len(sys.argv)==4,"usage C18_original.zip C18b_known_original.zip out.json"
    d=audit(sys.argv[1],sys.argv[2])
    Path(sys.argv[3]).write_text(json.dumps(d,ensure_ascii=False,indent=2)+"\n",encoding="utf-8")
    print(json.dumps({k:v for k,v in d.items() if k not in (
        "known_C18b_complete_45_original_pairs","known_C18b_effect_cases")},
        ensure_ascii=False,indent=2))
