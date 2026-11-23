#!/usr/bin/env python3
"""C23-R1: source-pinned gate-time AVAILABLE PAID sensor values, not a free PID1 oracle.

Retrospective only; replays all existing 186 native worlds with independent CIL.
Distinguishes observer-only native trace snapshots from values PID1 has actually
paid to Read; no new native worlds and no claim of acquired future knowledge.
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
ORIGINAL="934cbe7af9e906f041de0fc3e9d9100c3b29c297"
ZIP_HASH="5d55423803f397a143e941e7c8219dfde8c1d3abe5573a44526ebba0e907c44e"
NATIVE="cl1b_c23_original_native_186_worlds.json"
REPLAY="cl1b_c23_independent_paid_replay.json"
CELLS=(11,20,21,30,32,33,34,35,36,37,38,39)
AUDIT=ROOT/"audits/cl1b_c23_paid_stage_gate_independent_native_replay.py"

def sha256(path):
    h=hashlib.sha256()
    with open(path,"rb") as f:
        for block in iter(lambda:f.read(1048576),b""):h.update(block)
    return h.hexdigest()

def run(zpath,out):
    assert sha256(zpath)==ZIP_HASH
    with zipfile.ZipFile(zpath) as z:
        names=z.namelist()
        assert z.testzip() is None and len(names)==len(set(names))==190
        assert sum(x.endswith(".jsonl") for x in names)==186
        for n in names:assert "/" not in n and ".." not in n
        original=json.loads(z.read(NATIVE))
        saved=json.loads(z.read(REPLAY))
        assert original["source_sha"]==saved["original_native_sha"]==ORIGINAL
        assert original["study"]==saved["study"]=="CL1B-C23-F7-PAID-OBSERVED-STAGE-MATURITY-CODE-EDIT-GATE"
        assert saved["first_divergence"] is None
        with tempfile.TemporaryDirectory(prefix="c23r1-") as tmp:
            z.extractall(tmp)
            env=dict(os.environ,GITHUB_SHA=ORIGINAL)
            p=subprocess.run([sys.executable,str(AUDIT),str(Path(tmp)/NATIVE),tmp],
                check=True,text=True,capture_output=True,env=env)
            reread=json.loads(p.stdout)
            assert reread==saved,"fresh original paid-CIL independent native replay differs from original audited 186 worlds"
            hist={str(k):collections.Counter() for k in CELLS}
            gate_steps=collections.Counter()
            paid_service_before_gate=collections.Counter()
            first_feedback_before_gate=collections.Counter()
            features=[]
            for rec in original["records"]:
                on=rec["f7_on"]
                assert len(on["paid_stage_reads"])==len(on["paid_stage_branches"])==1
                filename=f"cl1b_c23_original_{on['name']}.jsonl"
                data=[json.loads(line) for line in (Path(tmp)/filename).read_text().splitlines()]
                reads=[(i,e) for i,e in enumerate(data) if e[0]=="cil" and e[2]==181]
                branches=[(i,e) for i,e in enumerate(data) if e[0]=="cil" and e[2]==182]
                assert len(reads)==len(branches)==1
                i,e=reads[0]
                j,br=branches[0]
                assert j==i+1 and br[1]==e[1]+1
                assert e[4][0]==e[6][CELLS.index(32)]==2
                assert br[3]==183 and br[4][0]==2
                feedbacks=[a for a in data[:i] if a[0]=="feedback"]
                assert len(feedbacks)>=2
                true_wins=sum(a[5] for a in feedbacks)
                assert true_wins==2
                failures=sum(a[5]==0 for a in feedbacks)
                gate_steps[e[1]]+=1
                paid_service_before_gate[len(feedbacks)]+=1
                first_feedback_before_gate[failures]+=1
                feature={str(k):e[6][p] for p,k in enumerate(CELLS)}
                for key,value in feature.items():hist[key][str(value)]+=1
                features.append({
                    "case":on["name"],"charged_step_of_first_stage_read":e[1],
                    "observed_completed_stages":2,
                    "actually_paid_PID1_sensor":{"Read32":2},
                    "observer_only_other_existing_cells_NOT_yet_PID_read_at_gate":{k:v for k,v in feature.items() if k!="32"},
                    "real_service_attempts_before_gate":len(feedbacks),
                    "real_failures_before_gate":failures,
                    "true_successes_before_gate":true_wins,
                    "complete_lifecycle_F7_on_paid":on["steps"],
                    "complete_lifecycle_original_F6_paid":rec["f6_original"]["steps"],
                    "same_source_F7_off_paid":rec["f7_off"]["steps"],
                })
            assert len(features)==45
            assert hist["32"]=={"2":45}
            assert saved["actor_paid_stage_gate_evidence"]["gate_skip_zero_stage"]==0
            total=original["counts"]["total_original_paid_instructions"]
            assert total==saved["original_charged_CIL_instruction_count_exactly_replayed"]
            report={
                "study":"CL1B-C23R1-ORIGINAL-PAID-GATE-OBSERVABLE-SIGNAL-COVERAGE",
                "original_tested_sha":ORIGINAL,"original_zip_sha256":ZIP_HASH,
                "original_worlds_exactly_replayed":186,"new_native_worlds":0,
                "first_divergence":None,"B3_B4":"NOT_MET",
                "constant_stage_maturity_observation":True,
                "PC181_observed_stage_histogram":dict(hist["32"]),
                "other_cells_original_native_observer_snapshot_NOT_actor_paid_sensor":{k:dict(v) for k,v in hist.items() if k!="32"},
                "original_paid_step_at_read_histogram":dict(gate_steps),
                "prior_real_paid_services_histogram":dict(paid_service_before_gate),
                "prior_real_failed_services_histogram":dict(first_feedback_before_gate),
                "features_considered_discriminative_only_if_multiple_values":{
                    k:len(v)>1 for k,v in hist.items() if k!="32"},
                "individual_exact_native_gate_snapshots":features,
                "limits":"All cells except Read32 are observer-only snapshots here: PID1 would need to pay for any new Read. C23 F7 never used cost or failed service count, no ROI learning, descriptive existing 186 native worlds only."
            }
            Path(out).write_text(json.dumps(report,indent=2,ensure_ascii=False)+"\n")
            print(json.dumps({
                "exact_paid_original_replay":"PASS","original_native_worlds":186,
                "stage_when_gate_executes":dict(hist["32"]),
                "number_of_prereg_frozen_new_goals":45,
                "available_other_cell_diversity":{k:dict(v) for k,v in hist.items() if k!="32"},
                "paid_step_at_gate":dict(gate_steps),
                "real_service_attempts_before_gate":dict(paid_service_before_gate),
                "real_failed_service_attempts_before_gate":dict(first_feedback_before_gate)},
                ensure_ascii=False))
            return report
if __name__=="__main__":
    assert len(sys.argv)==3,"usage: C23R1.py ORIGINAL_NATIVE_ZIP OUTPUT_JSON"
    run(sys.argv[1],sys.argv[2])
