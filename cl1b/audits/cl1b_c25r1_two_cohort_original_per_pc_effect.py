#!/usr/bin/env python3
"""C25R1: dual-archive original-native paid-CIL PC-path decomposition.

Revalidate both 276-world exact original source-pin ZIP archives by SHA256,
reexecute the independent Python interpreter for all 552 original native
worlds and compare complete paid-CIL/event records to archived results.
Only retrospective observer analysis; actor did not have this free cost oracle.
"""
import collections
import hashlib
import json
import os
import subprocess
import sys
import tempfile
import zipfile
from pathlib import Path

ROOT=Path(__file__).resolve().parents[1]
COHORTS={
  "C25":{
    "source_sha":"03b4dbb7330c8db921096e6512cfa71b0ccb70c5",
    "original_zip_sha":"3b80382e39ff0181afae04d4a946236060a6809e2ee42654d2cc3a60f244a2c2",
    "native":"cl1b_c25_original_native_276_worlds.json",
    "audit":"cl1b_c25_independent_full_paid_replay.json",
    "prefix":"cl1b_c25_original_",
    "audit_script":ROOT/"audits/cl1b_c25_paid_self_quarantine_independent_native_replay.py",
  },
  "C25B":{
    "source_sha":"af692838774b0017049034391cc217aeee7a60bf",
    "original_zip_sha":"2e55a13d8e5416ee5952c7ecd6a08c45e24d794ebe1a1dabc2e36ee4c3c6f022",
    "native":"cl1b_c25b_original_native_276_worlds.json",
    "audit":"cl1b_c25b_independent_full_paid_replay.json",
    "prefix":"cl1b_c25b_original_",
    "audit_script":ROOT/"audits/cl1b_c25b_unchanged_self_quarantine_independent_native_replay.py",
  }
}
def digest(path):
    h=hashlib.sha256()
    with Path(path).open("rb") as f:
        for block in iter(lambda:f.read(1048576),b""):
            h.update(block)
    return h.hexdigest()
def cil_pc_counts(path):
    c=collections.Counter()
    services=0
    with Path(path).open() as f:
        for line in f:
            row=json.loads(line)
            if row[0]=="cil":c[row[2]]+=1
            elif row[0]=="feedback":services+=1
            else:raise AssertionError(("original native unknown trace row",row))
    return c,services

def main(c25_zip,c25b_zip,out):
    result={}
    combined=collections.Counter()
    f10_net_hist=collections.Counter()
    f9_net_hist=collections.Counter()
    f10_first_edit_hist=collections.Counter()
    pc_differences=collections.Counter()
    per_case=[]
    all_goals=set()
    for tag,zpath in (("C25",c25_zip),("C25B",c25b_zip)):
        cfg=COHORTS[tag]
        assert digest(zpath)==cfg["original_zip_sha"],("original zip changed",tag)
        with zipfile.ZipFile(zpath) as z:
            names=z.namelist()
            assert z.testzip() is None and len(set(names))==len(names)==280
            assert sum(n.endswith(".jsonl") for n in names)==276
            assert all("/" not in n and ".." not in n for n in names)
            with tempfile.TemporaryDirectory(prefix="cl1b-c25r1-original-"+tag+"-") as tmp:
                z.extractall(tmp)
                native=json.loads((Path(tmp)/cfg["native"]).read_text())
                audit=json.loads((Path(tmp)/cfg["audit"]).read_text())
                assert native["source_sha"]==audit["tested_source_sha"]==cfg["source_sha"]
                assert native["counts"]["worlds"]==audit["original_worlds_exactly_replayed"]==276
                assert audit["first_divergence"] is None
                assert len(native["records"])==45 and len(native["controls"])==6
                assert len(audit["F10_same_source_first_physical_edit_effect"])==45
                assert len(audit["F9_same_source_first_physical_edit_effect"])==45
                cmd=[sys.executable,str(cfg["audit_script"]),str(Path(tmp)/cfg["native"]),tmp]
                p=subprocess.run(cmd,check=True,capture_output=True,text=True,
                  env=dict(os.environ,GITHUB_SHA=cfg["source_sha"]))
                rebuilt=json.loads(p.stdout)
                assert rebuilt==audit,("original paid-CIL independent replay diverged",tag)
                c=collections.Counter()
                pc_local=collections.Counter()
                net_hist=collections.Counter()
                f9_hist=collections.Counter()
                for i,row in enumerate(native["records"]):
                    on=row["f10_on"]
                    off=row["f10_off"]
                    s=row["f9_on"]
                    s_off=row["f9_off"]
                    old=row["f8_original"]
                    simple=row["f5_original"]
                    for arm in [on,off,s,s_off,old,simple]:
                        assert arm["completed_stages"]==arm["truthful_matches"]==24
                        assert arm["goals"]==row["goals"]
                    goal=tuple(row["goals"])
                    assert goal not in all_goals
                    all_goals.add(goal)
                    gain=off["steps"]-on["steps"]
                    f9gain=s_off["steps"]-s["steps"]
                    net_hist[gain]+=1
                    f9_hist[f9gain]+=1
                    f10_net_hist[gain]+=1
                    f9_net_hist[f9gain]+=1
                    assert len(on["paid_quarantine_self_edits"])==len(off["paid_quarantine_self_edits"])==5
                    assert len(on["paid_counter_writes"])==len(on["services"])
                    assert len(off["paid_counter_writes"])==len(off["services"])
                    t1=Path(tmp)/(cfg["prefix"]+on["name"]+".jsonl")
                    t2=Path(tmp)/(cfg["prefix"]+off["name"]+".jsonl")
                    cnt1,nserv1=cil_pc_counts(t1)
                    cnt2,nserv2=cil_pc_counts(t2)
                    assert nserv1==len(on["services"]) and nserv2==len(off["services"])
                    assert sum(cnt1.values())==on["steps"] and sum(cnt2.values())==off["steps"]
                    for k in set(cnt1)|set(cnt2):
                        diff=cnt1[k]-cnt2[k]
                        if diff:pc_local[k]+=diff;pc_differences[k]+=diff
                    assert sum(cnt1.values())-sum(cnt2.values())==-gain
                    first=audit["F10_same_source_first_physical_edit_effect"][i]
                    assert first["case"]==on["name"]
                    assert first["physical_patchnext_exposed"] is True
                    f10_first_edit_hist[first["first_effect_paid_PC"]]+=1
                    assert off["steps"]-on["steps"]==row["F10_same_source_physical_edit_net_paid_saving"]
                    assert s_off["steps"]-s["steps"]==row["F9_same_source_physical_edit_net_paid_saving"]
                    assert old["steps"]-on["steps"]==row["F10_vs_old_F8_full_program_paid_saving"]
                    assert simple["steps"]-on["steps"]==row["F10_vs_old_F5_full_program_paid_saving"]
                    c["tasks"]+=1
                    c["F10_on_paid"]+=on["steps"]
                    c["F10_off_paid"]+=off["steps"]
                    c["F9_on_paid"]+=s["steps"]
                    c["F9_off_paid"]+=s_off["steps"]
                    c["F8_original_paid"]+=old["steps"]
                    c["F5_original_paid"]+=simple["steps"]
                    c["F10_saves_paid"]+=gain
                    c["F9_saves_paid"]+=f9gain
                    c["F10_on_real_services"]+=nserv1
                    c["F10_off_real_services"]+=nserv2
                    c["F10_completed_stages"]+=on["truthful_matches"]
                    c["F10_paid_quarantine_edits"]+=len(on["paid_quarantine_self_edits"])
                    per_case.append({
                       "cohort":tag,"case":on["name"],"regime":row["regime"],
                       "full_24_goals":list(goal),
                       "F10_on_paid":on["steps"],"F10_off_paid":off["steps"],
                       "F10_on_minus_off_paid_CIL":-gain,
                       "F9_on_minus_off_paid_CIL":-f9gain,
                       "F10_on_real_services":nserv1,
                       "F10_off_real_services":nserv2,
                       "actor_paid_first_edit_attempts":on["paid_attempt_reads"][0]["attempts"],
                       "first_real_edit_PC":first["first_effect_paid_PC"],
                       "F10_on_paid_quarantine_edits":len(on["paid_quarantine_self_edits"]),
                    })
                assert c["tasks"]==45 and c["F10_completed_stages"]==1080
                assert c["F10_on_paid"]==audit["arm_paid_CIL"]["f10_on"]
                assert c["F10_off_paid"]==audit["arm_paid_CIL"]["f10_off"]
                assert c["F10_saves_paid"]==audit["F10_same_source_full_paid_physical_edit_net_saving"]==135
                assert sum(pc_local.values())==-135,("original per-PC charge decomposition not exact",tag)
                result[tag]={
                  "original_tested_sha":cfg["source_sha"],
                  "original_zip_sha256":cfg["original_zip_sha"],
                  "original_native_worlds_independently_replayed":276,
                  "case_counts":dict(c),
                  "F10_per_task_saving_histogram":dict(net_hist),
                  "F9_per_task_saving_histogram":dict(f9_hist),
                  "F10_original_paid_CIL_pc_execution_delta_on_minus_suppressed":{str(k):v for k,v in sorted(pc_local.items())},
                  "sum_PC_delta":sum(pc_local.values())
                }
                combined.update(c)
    assert len(all_goals)==len(per_case)==90
    assert combined["F10_saves_paid"]==270
    assert combined["F10_completed_stages"]==2160
    assert combined["F10_paid_quarantine_edits"]==450
    assert sum(pc_differences.values())==-270
    data={
      "study":"CL1B-C25R1-ORIGINAL-NATIVE-552-WORLD-DUAL-COHORT-PER-PC-SELF-QUARANTINE-ROI",
      "status":"RETROSPECTIVE_ONLY_SOURCE_PINNED_INDEPENDENT_ORIGINAL_552_WORLD_REPLAY",
      "original_native_worlds_replayed":552,
      "distinct_frozen_full_24_stage_goal_cases":90,
      "new_native_worlds":0,
      "first_divergence":None,
      "B3_B4":"NOT_MET",
      "source_identity":"C25 and C25B both froze and ran identical 210-node F9/F10 CIL for the two independent new cohorts",
      "both_original_cohorts":result,
      "combined_case_counts":dict(combined),
      "combined_F10_same_initial_source_full_paid_CIL_saved":270,
      "combined_F10_net_paid_saving_histogram":{str(k):v for k,v in sorted(f10_net_hist.items())},
      "combined_F9_net_paid_saving_histogram":{str(k):v for k,v in sorted(f9_net_hist.items())},
      "combined_F10_first_real_source_edit_PC_histogram":dict(f10_first_edit_hist),
      "combined_F10_original_paid_PC_execution_delta_on_minus_suppressed":{str(k):v for k,v in sorted(pc_differences.items())},
      "combined_F10_pc_delta_sum":sum(pc_differences.values()),
      "all_90_real_original_task_level_costs":per_case,
      "limits":"Retrospective original observer-only paid-PC path anatomy and strict same-initial-source all-PatchNext-physical-effect-suppressed bundled counterfactual. No free ROI labels available to PID1, no individual-PC causal isolation, no autonomous policy discovery or B3/B4."
    }
    Path(out).write_text(json.dumps(data,ensure_ascii=False,indent=2)+"\n")
    print(json.dumps({"PASS":"552 original native worlds exact full CIL replay",
      "90_unique_task_cases":90,"F10_paid_savings_histogram":data["combined_F10_net_paid_saving_histogram"],
      "F10_all_90_full_paid_saving":270,
      "F10_paid_PC_delta":data["combined_F10_original_paid_PC_execution_delta_on_minus_suppressed"],
      "F10_paid_PC_delta_sum":data["combined_F10_pc_delta_sum"]},ensure_ascii=False))
if __name__=="__main__":
    assert len(sys.argv)==4,"usage: c25r1.py ORIGINAL_C25.zip ORIGINAL_C25B.zip OUTPUT.json"
    main(*sys.argv[1:])
