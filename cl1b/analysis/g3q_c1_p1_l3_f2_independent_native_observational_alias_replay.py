#!/usr/bin/env python3
"""Independent full original-native F2 feedback-truth observational alias.

Reexecutes all original native CIL steps via F0's separate source interpreter,
then compares every actor-visible native step and feedback transition while
excluding experimenter-only hidden goal / truth labels. Full World byte-equality
is separately checked by the original Rust native integration test.
"""
import hashlib
import json
import os
import sys
from pathlib import Path
from g3q_c1_p1_l3_f0_independent_feedback_native_replay import run as source_replay

ROOT=Path(__file__).resolve().parents[1]
SOURCE=ROOT/"studies/B56-F1d1k-G3Q-C1-P1-L3-F0-frozen-122-node-feedback-relearning-founder-v1.json"
PREREG=ROOT/"studies/B56-F1d1k-G3Q-C1-P1-L3-F2-frozen-feedback-truth-observational-alias-preregistration.json"

def actor_visible_projection(path):
    visible=[]
    with Path(path).open() as stream:
        for line in stream:
            record=json.loads(line)
            if record[0]=="cil":
                assert len(record)==8
                visible.append(record)
            elif record[0]=="feedback":
                assert len(record)==10
                # Ignore ONLY researcher-held hidden target and actual fitness:
                # names, source registers/cells, paid outcome & delivered bit stay.
                visible.append(["feedback_received",record[1],record[2],
                    record[4],record[5],record[7],record[8],record[9]])
            else:
                raise AssertionError(("unexpected trace row",record))
    return visible

def main():
    if len(sys.argv)!=3:
        raise SystemExit("usage: python3 F2_REPLAY.py NATIVE_SUMMARY.json NATIVE_TRACE_DIR")
    frozen=json.loads(SOURCE.read_text())
    source=frozen["exact_native_founder_code"]
    prereg=json.loads(PREREG.read_text())
    native=json.loads(Path(sys.argv[1]).read_text())
    assert len(source)==122 and len(prereg["pairs"])==2
    assert native["study"]==prereg["study"]
    assert native["level"]=="NATIVE_FULL_WORLD_AND_EVENT_HISTORY_OBSERVATIONAL_ALIAS"
    assert native["source_sha"] not in ("","local")
    if os.environ.get("GITHUB_SHA"):
        assert native["source_sha"]==os.environ["GITHUB_SHA"]
    assert native["original_world_structurally_equal"] is True
    assert native["complete_serialized_world_bytes_equal"] is True
    assert native["original_paid_actor_visible_trace_equal"] is True
    assert len(native["arms"])==2
    projected=[]
    outputs=[]
    for i,arm in enumerate(prereg["pairs"]):
        name=arm["name"]
        path=(Path(sys.argv[2])/f"b56_f1d1k_g3q_c1_p1_l3_f2_native_paid_{name}.jsonl")
        exp=arm["native_expected"]
        config={"name":name,"goals":arm["hidden_goals"],
            "feedback":arm["feedback_regime"],
            "paid_steps":495,"services":4,"truth_matches":exp["actual_goal_matches"],
            "episode_probes":[1,1,1,1],"final_episode":4,
            "final_trial_byte":0,"success":1}
        independently=source_replay(source,config,path)
        assert independently["paid_steps"]==495
        assert independently["true_matches"]==exp["actual_goal_matches"]
        assert independently["actor_success_claim"]==1
        result=native["arms"][i]
        assert result["arm"]==name
        assert result["hidden_goals"]==arm["hidden_goals"]
        assert result["feedback_regime"]==arm["feedback_regime"]
        assert result["true_task_matches"]==independently["true_matches"]
        assert result["original_paid_instructions"]==495
        assert result["task_outputs"]==[1,1,1,1]
        assert result["visible_feedback"]==[1,1,1,1]
        assert result["original_paid_feedback_reads"]==[1,1,1,1]
        assert result["actor_declared_success"]==1
        assert result["world_occupied_units"]==136
        assert result["original_process_halted"] is True
        p=actor_visible_projection(path)
        assert len(p)==499
        projected.append(p)
        outputs.append(independently["true_matches"])
    assert projected[0]==projected[1],"first original actor-observable divergence"
    assert outputs==[4,0]
    digest=hashlib.sha256(json.dumps(projected[0],sort_keys=True,separators=(",",":")).encode()).hexdigest()
    print(json.dumps({
        "study":prereg["study"],"level":"ORIGINAL_NATIVE_EVERY_STEP_AND_FEEDBACK_ALIAS_INDEPENDENTLY_REPLAYED",
        "source_sha":native["source_sha"],
        "paid_native_steps_each":495,
        "full_actor_visible_records_each":499,
        "fully_equal_actor_visible_sha256":digest,
        "independent_native_first_divergence":None,
        "same_visible_feedback":[1,1,1,1],
        "same_visible_task_output":[1,1,1,1],
        "actual_goal_matches":[4,0],
        "world_with_all_original_events_equal":True,
        "conclusion":"A single feedback bit cannot determine semantic fitness: identical original native World and actor history can correspond to incompatible external hidden objectives.",
        "limits":"Experimenter goals outside the World, not native PID's own truth sensor; no general threat resistance."},indent=2))

if __name__=="__main__":main()
