#!/usr/bin/env python3
"""POLICY-P1: independent exact 62-node CIL replay for all six native worlds.

Original CIL self-mutates executable candidate transition stride PC46 after
actual paid positive Read20 in stage1. It never patches its output Const.
This separately authored interpreter checks every charged step, registers,
cell presence/values, policy code source byte and genuine external feedback.
It also checks causal advantage against equal-cost suppressed source writes.
"""
import json
import os
import sys
from pathlib import Path

FROZEN=(Path(__file__).resolve().parents[1]/
    "studies/CL1B-POLICY-P1-frozen-62-node-feedback-conditioned-executable-search-stride-v1.json")
STUDY="CL1B-POLICY-P1"
KEYS=(11,20,21,30,32,33)
PRIVATE=800

def replay(original,case,native_trace=None):
    code=[dict(x) for x in original]
    reg=[0,0,0,0]
    cell={}
    pc=0
    paid=0
    peak=63
    services=[]
    reads=[]
    patches=[]
    successes=[]
    failures=[]
    resets=[]
    claims=[]
    goals=case["goals"]
    mode=case["feedback"]
    suppressed=case["suppressed"]
    assert len(code)==62 and len(goals)==4 and set(goals)<=set((0,1,2))
    assert mode in ("truth","inverted","forced0")

    def rows():
        if native_trace is None:
            while True:
                yield None
        else:
            with Path(native_trace).open() as fh:
                for line in fh:
                    yield json.loads(line)
    native=rows()

    def check(expected):
        received=next(native,None)
        if native_trace is not None and received!=expected:
            raise AssertionError({
                "first_divergence_paid_instruction":paid,
                "case":case["name"],
                "original_native":received,
                "independent_frozen_source":expected,
            })

    while paid<PRIVATE:
        op=code[pc]
        kind=op["kind"]
        at=pc
        paid+=1
        if kind=="Const":
            reg[op["dst"]]=op["value"]
            pc=op["next"]
        elif kind=="Read":
            addr=reg[op["address"]]
            reg[op["dst"]]=cell.get(addr,0)
            if at==5:
                assert addr==20
                reads.append({"step":paid,"value":reg[op["dst"]]})
            pc=op["next"]
        elif kind in ("Add","Sub"):
            x,y=reg[op["lhs"]],reg[op["rhs"]]
            reg[op["dst"]]=x+y if kind=="Add" else x-y
            pc=op["next"]
        elif kind=="Write":
            addr=reg[op["address"]]
            value=reg[op["value"]]
            cell[addr]=value
            if at==12:
                assert addr==33 and value in (1,2,3)
                failures.append({"step":paid,"value":value})
            if at==34:
                assert addr==32 and value in (1,2,3,4)
                successes.append({"step":paid,"value":value})
            if at==37:
                assert addr==33 and value==0
                resets.append({"step":paid,"value":value})
            if addr==21:
                assert at in (27,60)
                assert value==(0 if at==27 else 1)
                claims.append({"step":paid,"value":value})
            pc=op["next"]
        elif kind=="Branch":
            pc=op["nonzero"] if reg[op["test"]] else op["zero"]
        elif kind=="PatchConst":
            assert at==45 and reg[op["node"]]==46
            assert len(patches)==0
            assert reads and reads[-1]["value"]==1
            assert cell.get(32)==2, "native policy may not patch before earned calibration"
            before=code[46]["value"]
            desired=reg[op["value"]]
            if not suppressed:
                code[46]["value"]=desired
            patches.append({"step":paid,"source_node":46,"before":before,
                "after":code[46]["value"],"edit_suppressed":suppressed})
            pc=op["next"]
        elif kind=="Halt":
            assert at in (28,61)
            pc=at
        else:
            raise AssertionError(("unsupported source opcode",at,kind))

        peak=max(peak,63+len(cell))
        assert len(cell)<=16 and peak<=300
        check(["cil",paid,at,pc,list(reg),PRIVATE-paid,
            [cell.get(key) for key in KEYS],63+len(cell),code[46]["value"]])

        if at==3:
            assert kind=="Write" and addr==11
            assert value==reg[3], "original paid task Write11 not determined by actor guess register"
            episode=cell.get(32,0)
            assert 0<=episode<4
            truth=int(value==goals[episode])
            signal=truth if mode=="truth" else 1-truth if mode=="inverted" else 0
            services.append({"step":paid,"episode":episode,"output":value,
                "truth":truth,"feedback":signal})
            cell[20]=signal
            peak=max(peak,63+len(cell))
            check(["feedback",paid,episode,goals[episode],value,truth,
                signal,[cell.get(key) for key in KEYS],63+len(cell),
                code[46]["value"]])
        if kind=="Halt":
            break
    else:
        raise AssertionError(("original PID exceeds private compute budget",case["name"]))
    if native_trace is not None:
        assert next(native,None) is None, "native trace continues after original Halt"
    assert len(patches)<=1
    assert len(reads)==len(services)
    assert [x["value"] for x in reads]==[x["feedback"] for x in services]
    assert len(successes)==len(resets)
    assert len(claims)==1
    assert code[1]==original[1], "no source change at fixed original Read3 PC1"
    expected=case["expected"]
    result={
        "paid_steps":paid,"halt_pc":pc,
        "private_credits_left":PRIVATE-paid,
        "executable_policy_stride":code[46]["value"],
        "actor_claimed_success":cell.get(21),
        "actor_completed_stages":cell.get(32),
        "original_paid_task_services":services,
        "original_paid_feedback_reads":reads,
        "original_paid_source_policy_patches":patches,
        "original_paid_success_count_writes":successes,
        "original_paid_fail_count_writes":failures,
        "original_paid_fail_reset_writes":resets,
        "original_paid_terminal_claims":claims,
        "occupied_data_cells":len(cell),
        "peak_occupied":peak,
    }
    for key,value in result.items():
        assert expected[key]==value,(case["name"],key,
            "prospective prediction contradicted",expected[key],value)
    return result

def main():
    frozen=json.loads(FROZEN.read_text())
    assert frozen["study"]==STUDY
    source=frozen["original_native_CIL_source"]
    cases=frozen["pre_registered_native_cases"]
    assert len(source)==62 and len(cases)==6
    assert source[45]=={"kind":"PatchConst","node":1,"value":3,"next":46}
    assert source[46]=={"kind":"Const","dst":2,"value":1,"next":47}
    native=None
    if len(sys.argv)==3:
        native=json.loads(Path(sys.argv[1]).read_text())
        assert native["study"]==STUDY
        assert native["level"]=="NATIVE_ORIGINAL_PID_FEEDBACK_CONDITIONED_EXECUTABLE_SEARCH_STRIDE"
        assert native["source_sha"] not in ("","local")
        if os.environ.get("GITHUB_SHA"):
            assert native["source_sha"]==os.environ["GITHUB_SHA"]
        assert len(native["worlds"])==6
        trace_folder=Path(sys.argv[2])
    elif len(sys.argv)!=1:
        raise SystemExit("usage: POLICY_P1.py [NATIVE_SUMMARY.json NATIVE_TRACES_DIR]")

    original=[]
    total=0
    for index,case in enumerate(cases):
        trace=(trace_folder/f"cl1b_policy_p1_original_native_{case['name']}.jsonl") if native else None
        outcome=replay(source,case,trace)
        total+=outcome["paid_steps"]
        assert len(outcome["original_paid_task_services"])<=12
        if native:
            observed=native["worlds"][index]
            mapping={
                "arm":case["name"],"goals":case["goals"],
                "feedback_regime":case["feedback"],
                "source_edit_suppressed":case["suppressed"],
                "paid_original_steps":outcome["paid_steps"],
                "final_original_executable_stride":outcome["executable_policy_stride"],
                "original_paid_policy_source_edits":outcome["original_paid_source_policy_patches"],
                "original_paid_task_services":outcome["original_paid_task_services"],
                "original_paid_feedback_reads":outcome["original_paid_feedback_reads"],
                "actor_claimed_success":outcome["actor_claimed_success"],
                "actor_completed_stages":outcome["actor_completed_stages"],
                "original_PID1_halted":True,
                "peak_occupied":outcome["peak_occupied"],
                "postload_host_source_edits":0,
            }
            for key,value in mapping.items():
                assert observed[key]==value,(case["name"],key,observed[key],value)
        original.append(outcome)
    assert total==856
    assert (original[1]["paid_steps"],original[2]["paid_steps"])==(149,196)
    assert original[2]["paid_steps"]-original[1]["paid_steps"]==47
    assert original[1]["executable_policy_stride"]==2
    assert original[2]["executable_policy_stride"]==1
    assert original[1]["actor_completed_stages"]==original[2]["actor_completed_stages"]==4
    assert original[0]["paid_steps"]==original[3]["paid_steps"]==123
    assert original[4]["actor_claimed_success"]==1
    assert sum(x["truth"] for x in original[4]["original_paid_task_services"])<4
    assert original[5]["actor_claimed_success"]==0
    if native:
        assert native["counts"]=={
            "original_native_worlds":6,"true_feedback_worlds":4,
            "private_original_paid_steps":856,
            "stride2_source_enabled_saved_charged_steps":47,
            "stride2_source_enabled_serviced_probes":5,
            "stride2_edit_disabled_serviced_probes":7,
        }
    print(json.dumps({
        "study":STUDY,
        "evidence_level":"NATIVE_6_WORLD_SOURCE_INDEPENDENT_EVERY_PAID_CIL_AND_FEEDBACK_REPLAY" if native else "PRE_NATIVE_FROZEN_SOURCE_PREDICTIONS",
        "source_sha":native["source_sha"] if native else None,
        "native_paid_instructions_replayed":total if native else 0,
        "source_policy_stride_PC46_mutated_by_original_PID":True,
        "stride2_paid_instructions_with_actor_policy_edit":149,
        "stride2_paid_instructions_when_same_native_patch_is_suppressed":196,
        "charged_instruction_savings":47,
        "genuine_task_service_attempts_saved":2,
        "stride1_noop_edit_negative_control_charged_savings":0,
        "deceptive_inverted_reward_causes_false_success":True,
        "all_zero_sham_causes_failfast":True,
        "first_divergence":None,
        "limits":"The researcher designed candidate stride update and feedback logic; not learned algorithm invention. Cyclic task regimes are preselected, no universal task generalization.",
    },indent=2))

if __name__=="__main__":
    main()
