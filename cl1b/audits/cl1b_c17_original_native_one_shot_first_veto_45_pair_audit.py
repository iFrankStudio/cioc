#!/usr/bin/env python3
"""Source-pin C17 retrospective *native* causal audit over the original preserved ZIP.
This is not an additional trial or post-hoc task selection; preserve all 45 pair
and 4 false-feedback denominators. The original independent CIL replay is checked.
"""
import hashlib
import json
import sys
import zipfile
from pathlib import Path

ORIG_SHA="29c295382848fb4fe944221766ae51324ada87726120d97d674b93fed551878c"
ARMS=(
    "truth_paid_postedit_feedback_veto",
    "same_source_pc212_veto_effect_suppressed",
    "same_source_pc102_physical_effect_suppressed",
    "different_source_c16_no_outcome_veto",
    "different_source_c14c_no_reentry",
    "different_source_c9_fixed_abstain",
)
FEES=dict(zip(ARMS,(195606,200014,196341,197866,188317,162029)))
COMPLETED=dict(zip(ARMS,(45,43,45,44,45,45)))
STAGES=dict(zip(ARMS,(2160,2158,2160,2160,2160,2160)))
CAUSAL={
    "c17_coupled_pulses_a0_d0":(1,582,2177),
    "c17_coupled_pulses_a1_d0":(0,34,4513),
    "c17_coupled_pulses_a1_d2":(0,703,2025),
    "c17_coupled_pulses_a2_d2":(0,257,2824),
    "c17_drifting_band_a0_d2":(0,477,3212),
    "c17_drifting_band_a2_d0":(0,971,2149),
    "c17_drifting_band_a2_d1":(0,1154,1286),
    "c17_drifting_band_a2_d2":(1,230,4378),
}
def read_trace(z,record):
    return [json.loads(line) for line in z.read("cl1b_c17_original_"+record["name"]+".jsonl").splitlines()
            if line.strip()]
def audit(path):
    assert hashlib.sha256(Path(path).read_bytes()).hexdigest()==ORIG_SHA
    with zipfile.ZipFile(path) as z:
        assert z.testzip() is None
        names=z.namelist()
        assert len(names)==len(set(names))==278
        assert sum(x.endswith(".jsonl") for x in names)==274
        native=json.loads(z.read("cl1b_c17_original_native_274_worlds.json"))
        py=json.loads(z.read("cl1b_c17_independent_replay.json"))
        assert native["study"]==py["study"]=="CL1B-C17-ONE-SHOT-PAID-POSTEDIT-FEEDBACK-VETO"
        assert native["native_worlds"]==py["native_worlds"]==274
        assert py["first_divergence"] is None
        assert native["total_paid_cil"]==py["exact_private_paid_cil_replayed"]==1147263
        assert len(native["records"])==45 and len(native["adversarial_records"])==4
        assert native["arm_fees"]==FEES and native["arm_completed_worlds"]==COMPLETED
        totals={a:dict(paid=0,stages=0,full=0,probe_armed=0,probe_disarmed=0,
                       followup_success=0,followup_fail=0,veto_real=0,
                       veto_suppressed=0,veto_checks=0,veto_blocks=0,
                       physical_changes=0,real_success_source_memories=0)
                for a in ARMS}
        match_rows=[]
        veto_cases=[]
        physical_cases=[]
        checked_armed=checked_disarmed=checked_feedback=checked_real_veto=0
        for case in native["records"]:
            assert len(case["arms"])==6
            row=case["arms"]
            assert tuple(v["arm"] for v in row)==ARMS
            assert all(v["feedback"]=="truth" and v["goals"]==row[0]["goals"] for v in row)
            for arm,v in zip(ARMS,row):
                t=totals[arm]
                t["paid"]+=v["steps"];t["stages"]+=v["completed_stages"]
                t["full"]+=int(v["completed_stages"]==48 and v["true_matches"]==48
                               and v["claim"]==1 and not v["budget_exhausted"])
                for key,field in [
                    ("probe_armed","paid_postedit_probe_armed"),
                    ("probe_disarmed","paid_postedit_probe_disarmed"),
                    ("followup_success","paid_postedit_feedback_positive"),
                    ("followup_fail","paid_postedit_feedback_zero"),
                    ("veto_real","paid_postedit_veto_written"),
                    ("veto_suppressed","paid_postedit_veto_effect_suppressed"),
                    ("veto_checks","paid_veto_gate_checks"),
                    ("veto_blocks","paid_veto_gate_blocks"),
                    ("physical_changes","effective_pc173_restorations"),
                    ("real_success_source_memories","claimed_success_records")]:
                    t[key]+=v[field]
            live,veto_suppressed,physical_suppressed=row[:3]
            assert live["completed_stages"]==physical_suppressed["completed_stages"]==48
            assert live["paid_postedit_probe_armed"]==live["paid_postedit_probe_disarmed"]
            assert live["paid_postedit_probe_disarmed"]==len(live["postedit_probe_samples"])
            assert live["paid_postedit_feedback_positive"]+live["paid_postedit_feedback_zero"]==len(live["postedit_probe_samples"])
            if live["paid_postedit_probe_armed"]:
                trace=read_trace(z,live)
                for j,e in enumerate(trace):
                    if e[0]!="cil":continue
                    if e[2]==205:
                        assert e[3]==173 and e[8][5]==206
                        checked_armed+=1
                        # This is the NEXT paid original Read20, not an observer injection.
                        next_pc5=next((k for k in range(j+1,len(trace))
                                     if trace[k][0]=="cil" and trace[k][2]==5),None)
                        assert next_pc5 is not None
                        assert trace[next_pc5][3]==206 and trace[next_pc5][8][5]==206
                        assert trace[next_pc5+1][0]=="cil" and trace[next_pc5+1][2]==206
                    elif e[2]==208:
                        assert e[3]==209 and e[8][5]==6
                        checked_disarmed+=1
                    elif e[2]==209:
                        assert e[3] in (6,210)
                        checked_feedback+=1
                    elif e[2]==212:
                        assert e[3]==6 and e[6][-1]==1
                        checked_real_veto+=1
                    elif e[2]==215:
                        assert e[3]==(103 if e[6][-1]==1 else 197)
            effect=(live["services"]!=veto_suppressed["services"] or
                    live["steps"]!=veto_suppressed["steps"] or
                    live["final_signature"]!=veto_suppressed["final_signature"])
            if effect:
                assert case["name"] in CAUSAL
                ctrace=read_trace(z,veto_suppressed)
                assert live["paid_postedit_veto_written"]>0
                first=next(e[1] for e in trace if e[0]=="cil" and e[2]==212)
                lp=[e for e in trace if e[1]<first]
                cp=[e for e in ctrace if e[1]<first]
                assert lp==cp,("native paid history differed before first actual Write46",case["name"])
                a=next(e for e in trace if e[0]=="cil" and e[1]==first)
                b=next(e for e in ctrace if e[0]=="cil" and e[1]==first)
                assert a[:6]==b[:6]
                assert a[6][:-1]==b[6][:-1] and (a[6][-1],b[6][-1])==(1,None)
                assert a[7]==b[7]+2 and a[8]==b[8]
                assert (a[9],b[9])==("normal","suppressed_pc212")
                delta_stages=live["completed_stages"]-veto_suppressed["completed_stages"]
                delta_cil=veto_suppressed["steps"]-live["steps"]
                assert (delta_stages,delta_cil,len(lp))==CAUSAL[case["name"]]
                veto_cases.append(dict(name=case["name"],first_paid_veto_write_step=first,
                    equal_original_native_prefix_events=len(lp),
                    stages_saved=delta_stages,cil_saved=delta_cil,
                    veto_blocks_by_actor=live["paid_veto_gate_blocks"],
                    full_actor_paid=live["steps"],suppressed_paid=veto_suppressed["steps"]))
            else:
                assert live["steps"]==veto_suppressed["steps"]
                assert live["services"]==veto_suppressed["services"]
            physical=(live["steps"]!=physical_suppressed["steps"] or
                      live["services"]!=physical_suppressed["services"] or
                      live["final_signature"]!=physical_suppressed["final_signature"])
            if physical:
                assert live["effective_pc173_restorations"]>0
                assert physical_suppressed["effective_pc173_restorations"]==0
                physical_cases.append(dict(name=case["name"],
                    live_paid=live["steps"],physical_effect_suppressed_paid=physical_suppressed["steps"],
                    stage_gain=live["completed_stages"]-physical_suppressed["completed_stages"],
                    cil_gain=physical_suppressed["steps"]-live["steps"]))
            match_rows.append(dict(name=case["name"],service_stages_live=live["completed_stages"],
                service_stages_no_veto=veto_suppressed["completed_stages"],
                live_paid=live["steps"],no_veto_paid=veto_suppressed["steps"],
                probe_armed=live["paid_postedit_probe_armed"],veto_writes=live["paid_postedit_veto_written"]))
        assert {a:t["paid"] for a,t in totals.items()}==FEES
        assert {a:t["full"] for a,t in totals.items()}==COMPLETED
        assert {a:t["stages"] for a,t in totals.items()}==STAGES
        m=totals[ARMS[0]];vs=totals[ARMS[1]];ps=totals[ARMS[2]]
        assert m["real_success_source_memories"]==287
        assert m["probe_armed"]==m["probe_disarmed"]==checked_armed==checked_disarmed==57
        assert checked_feedback==57
        assert m["followup_success"]==49 and m["followup_fail"]==8
        assert m["veto_real"]==checked_real_veto==8 and vs["veto_suppressed"]==13
        assert m["veto_checks"]==454 and m["veto_blocks"]==56
        assert m["physical_changes"]==39 and vs["physical_changes"]==108 and ps["physical_changes"]==0
        assert len(veto_cases)==len(physical_cases)==8
        assert {v["name"] for v in veto_cases}==set(CAUSAL)
        assert sum(x["stages_saved"] for x in veto_cases)==2
        assert sum(x["cil_saved"] for x in veto_cases)==4408
        assert sum(x["cil_gain"] for x in physical_cases)==735
        assert all(x["stage_gain"]==0 for x in physical_cases)
        assert native["same_source_live_vs_postedit_veto_effect_difference_cases"]==8
        assert native["same_source_live_vs_suppressed_paid_or_feedback_difference_cases"]==8
        assert len({e["name"] for e in native["adversarial_records"]})==4
        return dict(study="CL1B-C17-PAID-SELF-PATCHED-POSTEDIT-FEEDBACK-FIRST-CAUSAL-WRITE",
            classification="RETROSPECTIVE_ORIGINAL_NATIVE_FULL_45_TRUE_PAIRS",
            original_zip_sha256=ORIG_SHA,
            original_274_world_CIL_independent_replay_first_divergence=None,
            original_native_paid_cil=1147263,all_45_true_source_identical_effect_pairs=45,
            actual_actor_self_patch_activated=checked_armed,
            actor_self_patch_restored=checked_disarmed,
            paid_postedit_real_feedback_consumed=checked_feedback,
            paid_risk_veto_writes=checked_real_veto,
            paid_future_veto_branch_blocks=m["veto_blocks"],
            real_veto_causal_cases=len(veto_cases),
            exact_equal_paid_native_event_prefix_on_each_first_veto=True,
            stages_saved_by_paid_veto=2,private_CIL_saved_by_paid_veto=4408,
            true_physical_code_effect_cases=len(physical_cases),
            net_private_CIL_saved_by_real_physical_code_effect=735,
            strong_fixed_C9_paid=FEES[ARMS[-1]],
            full_arm_metrics=totals,all_45_true_pair_cases=match_rows,
            eight_veto_first_causal_write_cases=veto_cases,
            eight_physical_edit_effect_cases=physical_cases,
            four_false_feedback_controls=[{k:v[k] for k in ("name","feedback","true_matches","completed_stages","claim","steps")}
                                           for v in native["adversarial_records"]],
            scientific_status="B3_B4_NOT_MET_PREPROGRAMMED_HEURISTIC")
if __name__=="__main__":
    assert len(sys.argv)==3,"usage: audit.py original_274_native.zip output.json"
    data=audit(sys.argv[1])
    Path(sys.argv[2]).write_text(json.dumps(data,indent=2,ensure_ascii=False)+"\n",encoding="utf-8")
    print(json.dumps({k:v for k,v in data.items() if k not in (
        "all_45_true_pair_cases","four_false_feedback_controls")},
                     indent=2,ensure_ascii=False))
