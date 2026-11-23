#!/usr/bin/env python3
"""C22-R1: exact-source-pinned descriptive paid-PC anatomy of the original C22 archive.

No new native worlds. Every original byte and SHA256 is checked, original 141
world CIL independent replayer is rerun with its tested source identity, then
all CIL PC invoices and services are counted from source-pinned JSONL.
PC histograms describe executed paths, not counterfactual deletion savings.
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
TESTED_SHA="3fec55f901b418b81ed06f08c234a78724024977"
ZIP_SHA="ef44677809546195960772e7d7dadc093d0a0cf28d2d1adacd35995f93d8a150"
NATIVE_NAME="cl1b_c22_original_native_141_worlds.json"
REPLAY_NAME="cl1b_c22_independent_paid_replay.json"
AUDIT=ROOT/"audits/cl1b_c22_paid_self_edit_new_heldout_independent_replay.py"

def digest(path):
    h=hashlib.sha256()
    with open(path,"rb") as stream:
        for block in iter(lambda:stream.read(1024*1024),b""):
            h.update(block)
    return h.hexdigest()

def trace_hist(path):
    h=collections.Counter()
    n=0
    services=[]
    for line in Path(path).read_text().splitlines():
        e=json.loads(line)
        if e[0]=="cil":
            assert len(e)==9
            assert isinstance(e[2],int) and 0<=e[2]<180
            assert e[1]==n+1
            n+=1
            h[e[2]]+=1
        elif e[0]=="feedback":
            assert len(e)==10
            services.append({"at_paid_step":e[1],"stage":e[2],
                "truth":e[5],"actual_feedback":e[6]})
        else:
            raise AssertionError(("unrecognized original event",e[0]))
    assert sum(h.values())==n
    return h,services

def top_delta(a,b,count=20):
    diff=collections.Counter({i:a[i]-b[i] for i in range(180)})
    ordered=sorted(diff.items(),key=lambda x:(-x[1],x[0]))
    return [{"pc":pc,"main_minus_same_source_suppressed_paid":v} for pc,v in ordered[:count]]

def run(zip_path,out_path):
    assert digest(zip_path)==ZIP_SHA,("ZIP byte hash mismatch",digest(zip_path))
    with zipfile.ZipFile(zip_path) as z:
        names=z.namelist()
        assert z.testzip() is None and len(names)==len(set(names))==145
        assert sum(x.endswith(".jsonl") for x in names)==141
        native=json.loads(z.read(NATIVE_NAME))
        recorded=json.loads(z.read(REPLAY_NAME))
        prov=z.read("cl1b-c22-native-provenance.txt").decode()
        assert "tested_sha="+TESTED_SHA in prov
        assert "run_id=45156358171" in prov
        assert native["source_sha"]==recorded["source_sha"]==TESTED_SHA
        assert recorded["first_divergence"] is None
        with tempfile.TemporaryDirectory(prefix="cl1b-c22r1-") as tmp:
            for name in names:
                if "/" in name or name.startswith("."):
                    raise AssertionError(("unexpected ZIP member name",name))
            z.extractall(tmp)
            env=dict(os.environ,GITHUB_SHA=TESTED_SHA)
            independent=subprocess.run(
                [sys.executable,str(AUDIT),str(Path(tmp)/NATIVE_NAME),tmp],
                check=True,text=True,capture_output=True,env=env)
            validated=json.loads(independent.stdout)
            assert validated==recorded, "fresh all-141-original independent per-CIL replay differs from preserved successful replay"
            cases=native["records"]
            strict_proofs=recorded["strict_same_source_first_physical_PatchNext_divergences"]
            assert len(cases)==len(strict_proofs)==45
            main_hist=collections.Counter()
            off_hist=collections.Counter()
            f5_hist=collections.Counter()
            groups={}
            results=[]
            for i,case in enumerate(cases):
                on,off,f5=(case[x] for x in ("f6_on","f6_off","f5_on"))
                h_on,services_on=trace_hist(Path(tmp)/("cl1b_c22_original_"+on["name"]+".jsonl"))
                h_off,services_off=trace_hist(Path(tmp)/("cl1b_c22_original_"+off["name"]+".jsonl"))
                h_f5,services_f5=trace_hist(Path(tmp)/("cl1b_c22_original_"+f5["name"]+".jsonl"))
                assert len(services_on)==len(on["services"])
                assert len(services_off)==len(off["services"])
                assert len(services_f5)==len(f5["services"])
                assert sum(h_on.values())==on["steps"]
                assert sum(h_off.values())==off["steps"]
                assert sum(h_f5.values())==f5["steps"]
                main_hist.update(h_on);off_hist.update(h_off);f5_hist.update(h_f5)
                assert all(x["truthful_matches"]==x["completed_stages"]==24 for x in (on,off,f5))
                proof=strict_proofs[i]
                assert proof["case"]==on["name"] and proof["first_effect_exposed"]
                assert proof["first_effect_pc"] in (82,131,136,139,155,177)
                assert proof["identical_all_original_native_paid_and_service_prefix"]
                # All 45 pairs have a real physical difference. Any charged cost
                # delta must happen after the first effective PatchNext event.
                p=proof["first_divergent_paid_event_offset"]
                assert p>=0
                assert case["same_source_bundle_paid_saving"]==off["steps"]-on["steps"]<0
                assert case["cross_source_f6_vs_f5_paid_saving"]==f5["steps"]-on["steps"]
                result={
                    "name":on["name"],"regime":case["regime"],
                    "first_effect_pc":proof["first_effect_pc"],
                    "first_divergent_original_event_offset":p,
                    "first_true_modified_executable_edge_slot":proof["changed_live_edge_slot"],
                    "on_paid_CIL":on["steps"],"off_same_source_paid_CIL":off["steps"],
                    "different_source_F5_paid_CIL":f5["steps"],
                    "main_minus_same_source_suppressed_CIL":on["steps"]-off["steps"],
                    "main_minus_F5_CIL":on["steps"]-f5["steps"],
                    "on_paid_services":len(services_on),"off_paid_services":len(services_off),
                    "F5_paid_services":len(services_f5),
                    "on_minus_off_paid_services":len(services_on)-len(services_off),
                    "on_patchconst_edits":len(on["code_edits"]),
                    "on_paid_patchnext_physical_edits":sum(not e["suppressed"] for e in on["edge_edits"]),
                    "on_core_0_152_paid":sum(h_on[k] for k in range(153)),
                    "off_core_0_152_paid":sum(h_off[k] for k in range(153)),
                    "on_F6_refill_153_179_paid":sum(h_on[k] for k in range(153,180)),
                    "off_F6_refill_153_179_paid":sum(h_off[k] for k in range(153,180)),
                }
                assert result["on_core_0_152_paid"]+result["on_F6_refill_153_179_paid"]==on["steps"]
                assert result["off_core_0_152_paid"]+result["off_F6_refill_153_179_paid"]==off["steps"]
                results.append(result)
                g=groups.setdefault(case["regime"],{"cases":0,"on_total":0,"off_total":0,
                    "F5_total":0,"on_paid_services":0,"off_paid_services":0})
                g["cases"]+=1;g["on_total"]+=on["steps"];g["off_total"]+=off["steps"]
                g["F5_total"]+=f5["steps"];g["on_paid_services"]+=len(services_on)
                g["off_paid_services"]+=len(services_off)
            # Six preregistered false-feedback original worlds must stay in denominator.
            assert len(native["controls"])==6
            for case in native["controls"]:
                h,services=trace_hist(Path(tmp)/("cl1b_c22_original_"+case["name"]+".jsonl"))
                assert sum(h.values())==case["steps"]
                assert len(services)==len(case["services"])
            assert sum(main_hist.values())==83335 and sum(off_hist.values())==56766 and sum(f5_hist.values())==73586
            assert sum(x["main_minus_same_source_suppressed_CIL"] for x in results)==26569
            assert sum(x["main_minus_F5_CIL"] for x in results)==9749
            assert all(x["main_minus_same_source_suppressed_CIL"]>0 for x in results)
            assert sum(main_hist.values())-sum(off_hist.values())==26569
            assert sum(main_hist[k]-off_hist[k] for k in range(180))==26569
            core_diff=sum(main_hist[k]-off_hist[k] for k in range(153))
            refill_diff=sum(main_hist[k]-off_hist[k] for k in range(153,180))
            assert core_diff+refill_diff==26569
            group_rollup={k:{**v,"paid_main_minus_same_source_off":v["on_total"]-v["off_total"],
                "extra_main_services":v["on_paid_services"]-v["off_paid_services"]}
                for k,v in sorted(groups.items())}
            assert sorted(g["cases"] for g in groups.values())==[9]*5
            report={
                "study":"CL1B-C22R1-ORIGINAL-NATIVE-FULL-PAID-PC-ROI-TRACE-FORENSICS",
                "source":"C22 first run original ZIP SHA256 exact, original 141 native worlds reinterpreted independently",
                "tested_source_sha":TESTED_SHA,"original_zip_sha256":ZIP_SHA,
                "original_native_worlds_reinterpreted":141,"source_is_reused_F6":True,
                "B3_B4":"NOT_MET",
                "PC_decomposition_is_descriptive_NOT_deletion_causality":True,
                "arm_CIL":{"F6_on":sum(main_hist.values()),"F6_off_same_initial_source":sum(off_hist.values()),
                    "F5_different_source":sum(f5_hist.values())},
                "paid_cost_excess_main_minus_same_source_off":26569,
                "paid_cost_excess_main_minus_F5_different_source":9749,
                "all_45_pairs_first_effect_physical_prefix_proven":True,
                "first_effect_pc_frequencies":dict(sorted(collections.Counter(x["first_effect_pc"] for x in results).items())),
                "per_regime":group_rollup,
                "PC_0_152_extra_paid":core_diff,
                "PC_153_179_extra_paid":refill_diff,
                "main_PC_histogram":{str(k):main_hist[k] for k in range(180)},
                "same_source_suppressed_PC_histogram":{str(k):off_hist[k] for k in range(180)},
                "different_source_F5_PC_histogram":{str(k):f5_hist[k] for k in range(153)},
                "highest_positive_PC_cost_differences":top_delta(main_hist,off_hist),
                "sum_extra_services_main_minus_same_source_off":sum(x["on_minus_off_paid_services"] for x in results),
                "per_case":results,
                "scientific_limits":"Original F6 finite predefined grammar; suppresses all PatchNext physical effects, not a single patch; do not infer deletions safe or B3",
            }
            Path(out_path).write_text(json.dumps(report,indent=2,ensure_ascii=False)+"\n")
            print(json.dumps({k:report[k] for k in (
                "arm_CIL","PC_0_152_extra_paid","PC_153_179_extra_paid",
                "first_effect_pc_frequencies","sum_extra_services_main_minus_same_source_off",
                "highest_positive_PC_cost_differences","per_regime")},ensure_ascii=False))
            return report

if __name__=="__main__":
    assert len(sys.argv)==3,"usage: C22R1.py original-archive.zip output-full-forensics.json"
    run(sys.argv[1],sys.argv[2])
