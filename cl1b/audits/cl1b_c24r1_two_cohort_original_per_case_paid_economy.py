#!/usr/bin/env python3
"""C24-R1: exact ORIGINAL two-cohort full-paid-CIL per-case policy economics.
Replay independently all 372 already executed native worlds; NO new worlds.
Task-conditioned F8-vs-different-source F6 differences are economics, NOT a
causal proof of individual source-instruction effect or PID-accessible oracle.
"""
import collections
import hashlib
import json
import os
import statistics
import subprocess
import sys
import tempfile
import zipfile
from pathlib import Path

ROOT=Path(__file__).resolve().parents[1]
COHORTS={
    "C24":{
        "sha":"22c800a4c80362e48963366b83821a174b2be5d9",
        "zip":"7d6ad6711c9fd780bf93dabac1ba0be2b9e6119ae0a4a116e461ba7520c7d3a9",
        "study":"CL1B-C24-F8-PAID-CUMULATIVE-SERVICE-PROXY-EDIT-GATE",
        "native":"cl1b_c24_original_native_186_worlds.json",
        "replay":"cl1b_c24_independent_paid_replay.json",
        "script":ROOT/"audits/cl1b_c24_paid_service_attempt_gate_independent_replay.py",
    },
    "C24B":{
        "sha":"e9d90a03d965aad0f4d767ecef733f86fcfc10b0",
        "zip":"5482b83fe2c881aa855424678dca71149376c13afadab5c4ef0b643241808bf2",
        "study":"CL1B-C24B-F8-PAID-SERVICE-COUNT-DISJOINT-HELDOUT-REPLICATION",
        "native":"cl1b_c24b_original_native_186_worlds.json",
        "replay":"cl1b_c24b_independent_paid_replay.json",
        "script":ROOT/"audits/cl1b_c24b_unchanged_f8_paid_counter_independent_replay.py",
    }
}

def sha256(path):
    h=hashlib.sha256()
    with Path(path).open("rb") as f:
        for blob in iter(lambda:f.read(1048576),b""):h.update(blob)
    return h.hexdigest()

def load_worlds(name,zpath,cfg):
    assert sha256(zpath)==cfg["zip"],("wrong original ZIP",name)
    with zipfile.ZipFile(zpath) as z:
        names=z.namelist()
        assert z.testzip() is None and len(set(names))==len(names)==190
        assert sum(n.endswith(".jsonl") for n in names)==186
        assert all("/" not in n and ".." not in n for n in names)
        native=json.loads(z.read(cfg["native"]))
        original_replay=json.loads(z.read(cfg["replay"]))
        assert native["study"]==original_replay["study"]==cfg["study"]
        assert native["source_sha"]==original_replay["original_native_sha"]==cfg["sha"]
        assert original_replay["first_divergence"] is None
        assert native["counts"]["worlds"]==original_replay["original_native_worlds_exactly_replayed"]==186
        assert native["counts"]["truthful_worlds"]==180 and native["counts"]["adversarial"]==6
        assert len(native["records"])==45 and len(native["controls"])==6
        assert len(original_replay["same_source_first_true_physical_patchnext_effect"])==45
        with tempfile.TemporaryDirectory(prefix="cl1b-c24r1-"+name+"-") as tmp:
            z.extractall(tmp)
            env=dict(os.environ,GITHUB_SHA=cfg["sha"])
            rerun=subprocess.run([sys.executable,str(cfg["script"]),str(Path(tmp)/cfg["native"]),tmp],
                capture_output=True,text=True,check=True,env=env)
            replay=json.loads(rerun.stdout)
            assert replay==original_replay,("Independent full CIL replay changed",name)
        return native,replay

def countcase(rows,tag):
    p=collections.Counter()
    per=[]
    family={}
    for i,record in enumerate(rows):
        on,off,old,simple=(record[x] for x in ("f8_on","f8_off","f6_original","f5_original"))
        assert on["source"]==off["source"]=="F6"
        assert old["source"]=="F6BASE" and simple["source"]=="F5"
        assert on["goals"]==off["goals"]==old["goals"]==simple["goals"]==record["goals"]
        assert all(x["truthful_matches"]==x["completed_stages"]==24 for x in (on,off,old,simple))
        assert len(on["paid_attempt_reads"])==1
        attempts=on["paid_attempt_reads"][0]["attempts"]
        assert attempts in (2,3,4,5,6)
        assert len(on["paid_counter_writes"])==len(on["services"])
        assert [x["value"] for x in on["paid_counter_writes"]]==list(range(1,len(on["services"])+1))
        edit_count=sum(x["pc"]==82 for x in on["edge_edits"])
        assert edit_count==int(attempts in (2,3))
        d_old=old["steps"]-on["steps"]
        d_same=off["steps"]-on["steps"]
        d_simple=simple["steps"]-on["steps"]
        entry={
            "cohort":tag,"case":record["goals"],
            "regime":record["regime"],"name":on["name"],"attempts_at_first_paid_edit":attempts,
            "PC82_real_paid_executions":edit_count,
            "F8_on_full_paid":on["steps"],"F8_off_full_paid":off["steps"],
            "F6_original_full_paid":old["steps"],"F5_original_full_paid":simple["steps"],
            "F8_saves_vs_old_F6_different_source":d_old,
            "F8_saves_vs_F5_different_source":d_simple,
            "F8_on_same_source_true_PatchNext_effect_saving_vs_off":d_same,
            "F8_on_real_services":len(on["services"]),
            "F8_off_real_services":len(off["services"]),
            "F6_original_real_services":len(old["services"]),
            "F5_original_real_services":len(simple["services"]),
            "F8_on_real_PatchConst_edits":len(on["code_edits"]),
            "F8_on_real_PatchNext_effect_edits":sum(not x["suppressed"] for x in on["edge_edits"]),
        }
        assert record["F8_on_minus_F6_original_paid_CIL"]==-d_old
        assert record["F8_on_minus_F5_original_paid_CIL"]==-d_simple
        assert record["same_source_bundle_paid_saving"]==d_same
        for k,d in (("vs_F6",d_old),("vs_F5",d_simple),("vs_identical_initial_F8_off",d_same)):
            p[k+"_" + ("win" if d>0 else "tie" if d==0 else "lose")]+=1
        p["first_attempt_"+str(attempts)]+=1
        if edit_count:p["original_PC82_paid"]+=1
        p["F8_on_paid"]+=on["steps"];p["F8_off_paid"]+=off["steps"]
        p["F6_original_paid"]+=old["steps"];p["F5_original_paid"]+=simple["steps"]
        p["F8_on_true_stages"]+=on["truthful_matches"]
        f=family.setdefault(record["regime"],{"cases":0,"F8_on":0,"F6_original":0,
            "F8_same_source_suppressed":0,"F5_original":0,"first_PC82":0})
        f["cases"]+=1
        f["F8_on"]+=on["steps"];f["F6_original"]+=old["steps"]
        f["F8_same_source_suppressed"]+=off["steps"];f["F5_original"]+=simple["steps"]
        f["first_PC82"]+=edit_count
        per.append(entry)
    assert len(per)==45
    assert all(g["cases"]==9 for g in family.values()) and len(family)==5
    return p,family,per

def main(a,b,out):
    result={}
    allcases=[]
    allgoals=set()
    counters=collections.Counter()
    for tag,path in (("C24",a),("C24B",b)):
        native,replay=load_worlds(tag,path,COHORTS[tag])
        m,reg,rows=countcase(native["records"],tag)
        assert len(rows)==45
        assert all(tuple(x["case"]) not in allgoals for x in rows)
        allgoals.update(tuple(x["case"]) for x in rows)
        assert m["F8_on_paid"]==replay["arm_fees"]["f8_on"]
        assert m["F8_off_paid"]==replay["arm_fees"]["f8_off"]
        assert m["F6_original_paid"]==replay["arm_fees"]["f6_original"]
        assert m["F5_original_paid"]==replay["arm_fees"]["f5_original"]
        assert m["F8_on_true_stages"]==1080
        assert m["original_PC82_paid"]==replay["actor_paid_service_count_gate_evidence"]["paid_original_PC82_patchnext_executions"]
        for k,v in m.items():counters[k]+=v
        result[tag]={"original_native_worlds_fully_replayed":186,
            "original_tested_sha":COHORTS[tag]["sha"],
            "original_zip_sha256":COHORTS[tag]["zip"],
            "case_economy_counts":dict(m),"by_regime":reg}
        allcases.extend(rows)
    assert len(allcases)==len(allgoals)==90
    assert counters["F8_on_paid"]==160913
    assert counters["F8_off_paid"]==144897
    assert counters["F6_original_paid"]==173978
    assert counters["F5_original_paid"]==155843
    assert counters["F8_on_true_stages"]==2160
    assert counters["vs_F6_win"]+counters["vs_F6_tie"]+counters["vs_F6_lose"]==90
    assert counters["vs_identical_initial_F8_off_win"]+counters["vs_identical_initial_F8_off_tie"]+counters["vs_identical_initial_F8_off_lose"]==90
    report={
        "study":"CL1B-C24R1-TWO-COHORT-ORIGINAL-PAID-SELF-EDIT-PER-CASE-ECONOMY",
        "status":"RETROSPECTIVE_TWO_ALREADY_EXECUTED_NATIVE_COHORTS_FULL_SOURCE_PINNED_PYTHON_REPLAY",
        "source_identical_initial_F8_between_cohorts":True,
        "source_economics_not_self_invented_threshold_or_learned_ROI":True,
        "native_worlds_fully_independently_replayed":372,
        "true_new_goal_schedules_total":90,
        "first_divergence":None,"B3_B4":"NOT_MET",
        "cohort_evidence":result,
        "combined_case_economy_counts":dict(counters),
        "combined_F8_vs_old_F6_different_source_paid_saving":173978-160913,
        "combined_F8_true_edit_vs_same_initial_source_effect_suppressed_cost_excess":160913-144897,
        "combined_F8_vs_F5_different_source_cost_excess":160913-155843,
        "per_case_full_paid_lifecycle_and_real_source_edits":allcases,
        "scientific_limit":"All 372 original native worlds already existed. F8-versus-original-F6/F5 is different-source economics, not individual source edit causal effect. First-attempt service count is paid by PID, but cohort analysis is observer-only retrospective and cannot be free future actor knowledge."
    }
    Path(out).write_text(json.dumps(report,indent=2,ensure_ascii=False)+"\n")
    print(json.dumps({"worlds":372,"cases":90,"combined_counters":dict(counters),
      "cohort_counts":{k:v["case_economy_counts"] for k,v in result.items()},
      "net_savings_vs_old_F6":13065,"remaining_same_source_effect_cost_excess":16016},ensure_ascii=False))
if __name__=="__main__":
    assert len(sys.argv)==4,"usage: audit.py C24_original.zip C24B_original.zip result.json"
    main(*sys.argv[1:])
