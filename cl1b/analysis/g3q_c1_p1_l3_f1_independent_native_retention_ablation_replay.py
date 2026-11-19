#!/usr/bin/env python3
"""Independent per-native-step F1 ON/OFF causal control replay.

Reuses only the precommitted F0 SECOND CIL interpreter's pure run() mechanism;
reads the *two different* frozen sources and interleaved original native
task-feedback JSONL. Does not trust terminal native summary as an oracle.
"""
import json
import os
import sys
from pathlib import Path
from g3q_c1_p1_l3_f0_independent_feedback_native_replay import run as replay_original

ROOT=Path(__file__).resolve().parents[1]
F0=ROOT/"studies/B56-F1d1k-G3Q-C1-P1-L3-F0-frozen-122-node-feedback-relearning-founder-v1.json"
F1=ROOT/"studies/B56-F1d1k-G3Q-C1-P1-L3-F1-frozen-122-node-matched-retention-ablation-v1.json"
STUDY="B56-F1d1k-G3Q-C1-P1-L3-F1"

def scenario(raw,policy):
    x=raw[policy]
    goal=raw["goals"][-1]
    succeeded=x["true_successes"]==4
    last=( {0:2,1:0,2:1}[goal] if succeeded else 2)
    return {
        "name":policy+"_"+raw["name"],
        "goals":raw["goals"],"feedback":"truth",
        "paid_steps":x["steps"],"services":sum(x["probes"]),
        "final_episode":4 if succeeded else 0,"success":int(succeeded),
        "episode_probes":x["probes"],
        "truth_matches":x["true_successes"],
        "final_trial_byte":last}

def main():
    f0=json.loads(F0.read_text())
    f1=json.loads(F1.read_text())
    on=f0["exact_native_founder_code"]
    off=f1["exact_preload_control_source"]
    assert len(on)==len(off)==122
    assert [i for i,(a,b) in enumerate(zip(on,off)) if a!=b]==[93,94]
    assert on[93]=={"kind":"Const","dst":1,"value":1,"next":94}
    assert on[94]=={"kind":"Branch","test":1,"nonzero":28,"zero":28}
    assert off[93]=={"kind":"Const","dst":2,"value":63,"next":94}
    assert off[94]=={"kind":"Write","address":2,"value":3,"next":28}
    native=None
    if len(sys.argv)==3:
        native=json.loads(Path(sys.argv[1]).read_text())
        assert native["study"]==STUDY
        assert native["level"]=="NATIVE_EQUAL_LENGTH_PRELOAD_RETENTION_MEMORY_ABLATION"
        assert native["source_sha"] not in ("","local")
        if os.environ.get("GITHUB_SHA"):
            assert native["source_sha"]==os.environ["GITHUB_SHA"]
        assert len(native["records"])==8
        trace_dir=Path(sys.argv[2])
    elif len(sys.argv)!=1:
        raise SystemExit("usage: python3 F1_REPLAY.py [NATIVE_SUMMARY.json NATIVE_TRACE_DIR]")
    results={}
    index=0
    for policy,source in (("on",on),("off",off)):
        for raw in f1["goal_tasks_and_source_model_predictions"]:
            expected=scenario(raw,policy)
            trace=(trace_dir/f"b56_f1d1k_g3q_c1_p1_l3_f1_native_{policy}_{raw['name']}.jsonl") if native else None
            got=replay_original(source,expected,trace)
            if native:
                record=native["records"][index]
                assert record["arm"]==expected["name"]
                assert record["policy"]==policy and record["case"]==raw["name"]
                assert record["source_nodes"]==122
                assert record["paid_steps"]==got["paid_steps"]
                assert record["credits_remaining"]==got["credits_remain"]
                assert record["service_events"]==got["services"]
                assert record["episode_probes"]==got["episode_probes"]
                assert record["true_task_matches"]==got["true_matches"]
                assert record["original_paid_feedback_reads"]==got["feedback_reads"]
                assert record["meta_fetches"]==got["meta_fetches"]
                assert record["accumulator_checkpoints"]==got["checkpoints"]
                assert record["actor_claimed_success"]==got["actor_success_claim"]
                assert record["occupied_units"]==got["occupied"]==136
                assert record["native_original_pid1_halted"] is True
            index+=1
            results.setdefault(raw["name"],{})[policy]={
                "paid_steps":got["paid_steps"],
                "probes":got["episode_probes"],
                "true_successes":got["true_matches"]}
    expected_delta={"shift0_to2":(234,2),"shift1_to0":(237,2),
                    "shift2_to1":(-121,-1),"unreachable":(0,0)}
    rows=[]
    for name,(credit_delta,probe_delta) in expected_delta.items():
        a,b=results[name]["on"],results[name]["off"]
        actual_credit_delta=b["paid_steps"]-a["paid_steps"]
        actual_probe_delta=sum(b["probes"])-sum(a["probes"])
        assert (actual_credit_delta,actual_probe_delta)==(credit_delta,probe_delta)
        assert a["true_successes"]==b["true_successes"]
        rows.append({"case":name,"ON":a,"OFF":b,
                     "OFF_minus_ON_charged_steps":actual_credit_delta,
                     "OFF_minus_ON_probes":actual_probe_delta})
    assert [row["OFF_minus_ON_probes"] for row in rows]==[2,2,-1,0]
    print(json.dumps({
        "study":STUDY,
        "evidence_level":"ALL_ORIGINAL_NATIVE_STEP_REPLAY_WITH_TASK_FEEDBACK" if native else "SOURCE_ONLY_MIXED_RESULT",
        "source_sha":native["source_sha"] if native else None,
        "paid_native_steps_checked":sum(v["paid_steps"] for arms in results.values() for v in arms.values()) if native else 0,
        "source_researcher_mutated_pcs":[93,94],
        "same_world_costs":True,
        "cases":rows,"first_divergence":None,
        "conclusion":"Retaining the last successful candidate gives benefits in two schedules but a counterexample in the third."},indent=2))

if __name__=="__main__":main()
