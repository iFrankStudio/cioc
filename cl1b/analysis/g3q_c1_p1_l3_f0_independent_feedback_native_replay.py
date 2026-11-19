#!/usr/bin/env python3
"""Independent every-paid-step CL0 CIL replay with interleaved task feedback.

The CIL source is the prospectively frozen 122-node researcher-authored F0
founder. Native trace rows are observations, not executable instructions.
All hidden goals remain in the task harness; the original PID1 sees only its
own Write11 outcomes and externally authored feedback cell20.
"""
import json
import os
import sys
from collections import Counter
from pathlib import Path

ROOT=Path(__file__).resolve().parents[1]
SOURCE=ROOT/"studies/B56-F1d1k-G3Q-C1-P1-L3-F0-frozen-122-node-feedback-relearning-founder-v1.json"
KEYS=(11,20,21,32,33,50,51,52,53,56,62,63,64)
STUDY="B56-F1d1k-G3Q-C1-P1-L3-F0"

def run(source,arm,trace=None):
    registers=[0,0,0,0]
    cells={}
    pc=0
    steps=0
    attempts=Counter()
    services=[]
    feedback_reads=[]
    meta_fetches=0
    checkpoints=0
    true_matches=0
    feed_count=0
    def rows():
        if trace is None:
            while True:yield None
        else:
            with Path(trace).open() as stream:
                for line in stream:yield json.loads(line)
    records=rows()
    def accept(expected):
        got=next(records,None)
        if trace is not None and got != expected:
            raise AssertionError({"native_divergence_at_paid_instruction":steps,
              "observed":got,"independently_expected":expected})
    while steps < 8000:
        here=pc
        op=source[pc]
        kind=op["kind"]
        steps+=1
        if kind=="Const":
            registers[op["dst"]]=op["value"]
            pc=op["next"]
        elif kind in ("Add","Sub"):
            a,b=registers[op["lhs"]],registers[op["rhs"]]
            registers[op["dst"]]=a+b if kind=="Add" else a-b
            pc=op["next"]
        elif kind=="Read":
            key=registers[op["address"]]
            val=cells.get(key,0)
            registers[op["dst"]]=val
            if here==79:
                assert key==20
                feedback_reads.append(val)
            if here==37:
                assert 62<=key<=64
                meta_fetches+=1
            pc=op["next"]
        elif kind=="Write":
            key=registers[op["address"]]
            val=registers[op["value"]]
            cells[key]=val
            if here==58:
                assert key==56
                checkpoints+=1
            pc=op["next"]
        elif kind=="Branch":
            pc=op["nonzero"] if registers[op["test"]] else op["zero"]
        elif kind=="Halt":
            assert here in (98,121)
        else:
            raise AssertionError((here,kind))
        expected=["cil",steps,here,pc,list(registers),8000-steps,
                  [cells.get(key) for key in KEYS],123+len(cells)]
        accept(expected)
        if kind=="Write" and here==71:
            assert key==11 and 0<=val<=2
            episode=cells[32]
            assert 0<=episode<4
            goal=arm["goals"][episode]
            trial=cells[63]
            assert 0<=trial<=2
            truth=int(val==goal)
            delivered={"truth":truth,"blind":0,"inverted":1-truth}[arm["feedback"]]
            attempts[episode]+=1
            assert attempts[episode]<=3
            true_matches+=truth
            services.append([episode,goal,trial,val,truth,delivered,steps])
            assert 20 not in cells or cells[20] in (0,1)
            cells[20]=delivered
            feed_count+=1
            feedback=["feedback",steps,episode,goal,trial,val,truth,delivered,
                      [cells.get(key) for key in KEYS],123+len(cells)]
            accept(feedback)
        if kind=="Halt":
            break
    else:
        raise AssertionError("founder exhausted private instruction budget")
    if trace is not None:
        try:
            extra=next(records)
        except StopIteration:
            extra=None
        assert extra is None,("extra native evidence after source Halt",extra)
    assert pc in (98,121)
    assert cells[21] in (0,1)
    assert len(cells)==13
    assert meta_fetches==checkpoints==3*len(services)
    assert feedback_reads==[x[5] for x in services]
    result={
        "paid_steps":steps,"service_count":len(services),
        "services":services,"true_matches":true_matches,
        "feedback_reads":feedback_reads,
        "meta_fetches":meta_fetches,"checkpoints":checkpoints,
        "episode_probes":[attempts[i] for i in range(4)],
        "final_episode":cells[32],"final_trial_byte":cells[63],
        "actor_success_claim":cells[21],
        "occupied":123+len(cells),
        "credits_remain":8000-steps,
        "feedback_interventions":feed_count,
        "natural_halt_pc":pc}
    assert result["paid_steps"]==arm["paid_steps"]
    assert result["service_count"]==arm["services"]
    assert result["true_matches"]==arm["truth_matches"]
    assert [x for x in result["episode_probes"] if x>0]==arm["episode_probes"]
    assert result["final_episode"]==arm["final_episode"]
    assert result["final_trial_byte"]==arm["final_trial_byte"]
    assert result["actor_success_claim"]==arm["success"]
    return result

def main():
    manifest=json.loads(SOURCE.read_text())
    code=manifest["exact_native_founder_code"]
    assert len(code)==122
    arms=manifest["prespecified_arms"]
    assert len(arms)==6
    native=None
    if len(sys.argv)==3:
        native=json.loads(Path(sys.argv[1]).read_text())
        assert native["study"]==STUDY
        assert native["level"]=="NATIVE_ONE_PID_FEEDBACK_CONDITIONED_FINITE_META_SEARCH_NOT_EARNED_C0"
        assert native["source_sha"] not in ("","local")
        if os.environ.get("GITHUB_SHA"):
            assert native["source_sha"]==os.environ["GITHUB_SHA"]
        trace_dir=Path(sys.argv[2])
        assert len(native["arms"])==6
    elif len(sys.argv)!=1:
        raise SystemExit("usage: python3 SCRIPT [NATIVE_SUMMARY.json NATIVE_TRACES_DIR]")
    verdicts=[]
    total_paid=0
    for index,arm in enumerate(arms):
        file=(trace_dir/f"b56_f1d1k_g3q_c1_p1_l3_f0_paid_trace_{arm['name']}.jsonl") if native else None
        result=run(code,arm,file)
        total_paid+=result["paid_steps"]
        if native:
            observed=native["arms"][index]
            assert observed["arm"]==arm["name"]
            fields={
                "paid_steps":"paid_steps","true_service_matches":"true_matches",
                "original_paid_feedback_reads":"feedback_reads",
                "meta_fetches":"meta_fetches","accumulator_checkpoints":"checkpoints",
                "actor_success_claim":"actor_success_claim",
                "final_episode":"final_episode","final_active_middle":"final_trial_byte",
                "credits_remain":"credits_remain",
                "postload_environment_feedback_writes":"feedback_interventions"}
            for native_k,our_k in fields.items():
                assert observed[native_k]==result[our_k],(arm["name"],native_k)
            assert observed["services"]==result["services"]
            assert observed["peak_occupied"]==136
            assert observed["single_original_pid1"] is True
            assert observed["self_halted"] is True
            assert observed["episode_probes"]==result["episode_probes"]
        verdicts.append({"arm":arm["name"],"paid_steps":result["paid_steps"],
            "true_matches":result["true_matches"],
            "actor_success_claim":result["actor_success_claim"],
            "probes":result["episode_probes"],"first_divergence":None})
    assert [v["paid_steps"] for v in verdicts]==[971,732,852,377,377,495]
    assert verdicts[-1]["actor_success_claim"]==1 and verdicts[-1]["true_matches"]==0
    print(json.dumps({
      "study":STUDY,
      "evidence_level":"SAME_SHA_EVERY_ORIGINAL_CIL_STEP_PLUS_ENV_FEEDBACK_REPLAY" if native else "SOURCE_ONLY_PREDICTION",
      "source_sha":native["source_sha"] if native else None,
      "total_native_paid_steps_rechecked":total_paid if native else 0,
      "all_arms":verdicts,"first_divergence":None,
      "limits":"Founder has preauthored finite rule search, not earned C0 certification or open-ended adaptation."},indent=2))

if __name__=="__main__":main()
