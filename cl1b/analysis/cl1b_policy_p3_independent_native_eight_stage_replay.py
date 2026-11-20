#!/usr/bin/env python3
"""CL1B-POLICY-P2 original source-independent exact native replay.

Every charged CIL instruction from the frozen 80-node founder is re-executed
by a separate Python interpreter (NOT inferred from the Rust engine). Audit
all 81 paired goal sequences and six adversity controls: actor-origin
Read20, self-paid previous-goal Write34 / Read34, signed normalisation,
self-paid executable PatchConst at node46, original Write11, private credits,
cells/PC/registers, natural halt and truthful environment feedback.
"""
import json
import os
import sys
from pathlib import Path

ROOT=Path(__file__).resolve().parents[1]
SOURCE_FILE=ROOT/"studies/CL1B-POLICY-P2-frozen-80-node-actor-paid-relative-delta-search-policy-v1.json"
PRIOR_FILE=ROOT/"studies/CL1B-POLICY-P1-H81-frozen-162-original-world-complete-transfer-audit-v1.json"
STUDY="CL1B-POLICY-P2"
KEYS=(11,20,21,30,32,33,34)
CREDIT=1600

def run_source(source,arm,trace=None):
    code=[dict(op) for op in source]
    goals=arm["goals"]
    mode=arm["feedback"]
    disabled=arm["suppressed"]
    assert len(code)==80
    assert len(goals)==8 and set(goals)<=set((0,1,2))
    assert mode in ("truth","inverted","forced0")
    cells={}
    reg=[0,0,0,0]
    pc=0
    paid=0
    peak=81
    outputs=[]
    rewards=[]
    patches=[]
    successes=[]
    failures=[]
    reset=[]
    claims=[]
    prior_writes=[]
    prior_reads=[]

    def native_rows():
        if trace is None:
            while True: yield None
        else:
            with Path(trace).open() as fh:
                for row in fh: yield json.loads(row)
    records=native_rows()

    def validate(row):
        observed=next(records,None)
        if trace is not None and observed!=row:
            raise AssertionError({
                "original_arm":arm["name"],
                "first_native_divergence_paid_instruction":paid,
                "native_original_evidence":observed,
                "independent_original_source_replay":row,
            })

    while paid<CREDIT:
        inst=code[pc]
        at=pc
        kind=inst["kind"]
        paid+=1
        if kind=="Const":
            reg[inst["dst"]]=inst["value"]
            pc=inst["next"]
        elif kind=="Read":
            address=reg[inst["address"]]
            reg[inst["dst"]]=cells.get(address,0)
            if at==5:
                assert address==20
                rewards.append({"step":paid,"value":reg[inst["dst"]]})
            if at==69:
                assert address==34 and len(prior_writes)==1 and not prior_reads
                assert reg[inst["dst"]]==prior_writes[0]["value"]
                prior_reads.append({"step":paid,"value":reg[inst["dst"]]})
            pc=inst["next"]
        elif kind in ("Add","Sub"):
            first,second=reg[inst["lhs"]],reg[inst["rhs"]]
            reg[inst["dst"]]=first+second if kind=="Add" else first-second
            pc=inst["next"]
        elif kind=="Write":
            address=reg[inst["address"]]
            value=reg[inst["value"]]
            cells[address]=value
            if at==12:
                assert address==33 and value in (1,2,3)
                failures.append({"step":paid,"value":value})
            if at==34:
                assert address==32 and 1<=value<=8
                successes.append({"step":paid,"value":value})
            if at==37:
                assert address==33 and value==0
                reset.append({"step":paid,"value":0})
            if at==66:
                assert address==34 and not prior_writes
                assert cells.get(32)==1, "one paid previous-goal store only after first earned task"
                prior_writes.append({"step":paid,"value":value})
            if address==21:
                assert at in (27,60) and value==(0 if at==27 else 1)
                claims.append({"step":paid,"value":value})
            pc=inst["next"]
        elif kind=="Branch":
            pc=inst["nonzero"] if reg[inst["test"]] else inst["zero"]
        elif kind=="PatchConst":
            assert at==45 and reg[inst["node"]]==46
            assert not patches
            assert rewards and rewards[-1]["value"]==1
            assert cells.get(32)==2
            assert len(prior_reads)==len(prior_writes)==1
            assert code[46]["kind"]=="Const"
            desired=reg[inst["value"]]
            assert desired in (0,1,2)
            before=code[46]["value"]
            if not disabled: code[46]["value"]=desired
            patches.append({"step":paid,"source_node":46,"before":before,
                "after":code[46]["value"],"requested":desired,
                "edit_suppressed":disabled})
            pc=inst["next"]
        elif kind=="Halt":
            assert at in (28,61)
            pc=at
        else:
            raise AssertionError(("frozen original opcode unknown",kind,at))

        peak=max(peak,81+len(cells))
        assert len(cells)<=16 and peak<=300
        validate(["cil",paid,at,pc,list(reg),CREDIT-paid,
            [cells.get(k) for k in KEYS],81+len(cells),code[46]["value"]])

        if at==3:
            assert kind=="Write" and address==11
            assert value==reg[3], "task guess not generated by actor"
            episode=cells.get(32,0)
            assert 0<=episode<8
            truthful=int(value==goals[episode])
            delivered=truthful if mode=="truth" else 1-truthful if mode=="inverted" else 0
            outputs.append({"step":paid,"episode":episode,"output":value,
                "truth":truthful,"feedback":delivered})
            cells[20]=delivered
            peak=max(peak,81+len(cells))
            validate(["feedback",paid,episode,goals[episode],value,truthful,
                delivered,[cells.get(k) for k in KEYS],81+len(cells),
                code[46]["value"]])
        if kind=="Halt": break
    else:
        raise AssertionError(("original PID exhausted private paid credits",arm["name"]))

    if trace is not None:
        assert next(records,None) is None, "native evidence continues past paid Halt"
    assert len(claims)==1
    assert len(successes)==len(reset)
    assert len(rewards)==len(outputs)
    assert [x["value"] for x in rewards]==[x["feedback"] for x in outputs]
    assert len(patches)<=1
    assert len(prior_writes)<=1 and len(prior_reads)<=1
    assert code[1]==source[1],"source task-output Const mutated instead of policy stride"
    got={
        "paid_steps":paid,
        "halt_pc":pc,
        "private_credits_left":CREDIT-paid,
        "final_executable_stride":code[46]["value"],
        "actor_claim":cells.get(21),
        "actor_completed_stages":cells.get(32),
        "original_paid_task_services":outputs,
        "original_paid_feedback_reads":rewards,
        "original_paid_policy_source_edits":patches,
        "original_paid_success_writes":successes,
        "original_paid_failure_writes":failures,
        "original_paid_failure_resets":reset,
        "original_paid_terminal_claims":claims,
        "original_paid_previous_goal_writes":prior_writes,
        "original_paid_previous_goal_reads":prior_reads,
        "occupied_data_cells":len(cells),
        "peak_occupied":peak,
    }
    return got


def main():
    if len(sys.argv)!=3:
        raise SystemExit("usage: P3_REPLAY.py NATIVE_SUMMARY.json NATIVE_TRACES_DIR")
    native=json.loads(Path(sys.argv[1]).read_text())
    manifest=json.loads((ROOT/"studies/CL1B-POLICY-P3-frozen-eight-stage-regime-transfer-v1.json").read_text())
    ancestor=json.loads(SOURCE_FILE.read_text())
    assert native["study"]==manifest["study"]=="CL1B-POLICY-P3"
    assert native["level"]=="NATIVE_ORIGINAL_PID_80_NODE_EIGHT_STAGE_56_WORLD_CAUSAL_TRANSFER"
    assert native["source_sha"] not in ("local","")
    if os.environ.get("GITHUB_SHA"):
        assert native["source_sha"]==os.environ["GITHUB_SHA"]
    source=[dict(x) for x in ancestor["original_native_CIL_source"]]
    assert source[38]=={"kind":"Const","dst":2,"value":4,"next":39}
    source[38]["value"]=8
    assert source[45]=={"kind":"PatchConst","node":1,"value":0,"next":46}
    assert len(manifest["arms"])==54 and len(manifest["controls"])==2
    assert len(native["pairs"])==27 and len(native["controls"])==2
    folder=Path(sys.argv[2])
    total=0
    by_group={}
    def audit(arm,original):
        nonlocal total
        trace=folder/f"cl1b_policy_p3_original_{arm['name']}.jsonl"
        result=run_source(source,arm,trace)
        total+=result["paid_steps"]
        fields={
          "original_paid_steps":"paid_steps",
          "final_executable_stride":"final_executable_stride",
          "actor_claim":"actor_claim",
          "actor_completed_stages":"actor_completed_stages",
          "original_paid_policy_edits":"original_paid_policy_source_edits",
          "original_paid_task_services":"original_paid_task_services",
          "original_paid_feedback_reads":"original_paid_feedback_reads",
          "original_paid_previous_goal_writes":"original_paid_previous_goal_writes",
          "original_paid_previous_goal_reads":"original_paid_previous_goal_reads",
          "peak_occupied":"peak_occupied"}
        for original_key,ours in fields.items():
            assert original[original_key]==result[ours],(arm["name"],original_key)
        assert original["original_PID1_halted"] is True
        return result
    for i in range(27):
        a,b=manifest["arms"][2*i:2*i+2]
        record=native["pairs"][i]
        assert a["goals"]==b["goals"]==record["goals"]
        assert a["regime"]==record["regime"]
        on=audit(a,record["source_patch_on"])
        off=audit(b,record["source_patch_off"])
        assert on["actor_claim"]==off["actor_claim"]==1
        assert on["actor_completed_stages"]==off["actor_completed_stages"]==8
        assert sum(s["truth"] for s in on["original_paid_task_services"])==8
        assert sum(s["truth"] for s in off["original_paid_task_services"])==8
        diff=off["paid_steps"]-on["paid_steps"]
        pdiff=len(off["original_paid_task_services"])-len(on["original_paid_task_services"])
        assert record["native_paid_instruction_saving"]==diff
        assert record["native_paid_task_probe_saving"]==pdiff
        group=by_group.setdefault(a["regime"],{"pairs":0,"net_paid_instruction_saving":0,"net_task_probe_saving":0})
        group["pairs"]+=1
        group["net_paid_instruction_saving"]+=diff
        group["net_task_probe_saving"]+=pdiff
    for arm,record in zip(manifest["controls"],native["controls"]):
        result=audit(arm,record)
        if arm["feedback"]=="inverted":
            assert result["actor_claim"]==1
            assert sum(s["truth"] for s in result["original_paid_task_services"])<8
        else:
            assert arm["feedback"]=="forced0" and result["actor_claim"]==0
    assert native["counts"]["regimes"]==by_group
    assert native["counts"]["total_original_paid_instructions"]==total
    assert native["counts"]["original_worlds"]==56
    assert set(by_group)=={"stable","switch","scrambled"}
    print(json.dumps({"study":"CL1B-POLICY-P3",
      "source_sha":native["source_sha"],
      "evidence_level":"56_ORIGINAL_NATIVE_WORLD_EVERY_CHARGED_CIL_INSTRUCTION_INDEPENDENT_REPLAY",
      "native_original_paid_instructions_independently_replayed":total,
      "native_worlds":56,"regimes":by_group,
      "first_native_divergence":None,
      "nonclaims":"Host preloaded 8-stage horizon, researcher-authored one-time policy, no rollback/strategy inference."},indent=2))

if __name__=="__main__":
    main()
