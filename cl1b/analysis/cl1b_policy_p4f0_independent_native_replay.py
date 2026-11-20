#!/usr/bin/env python3
"""CL1B-POLICY-P4F0 separately implemented original CIL opcode replayer.
No Rust step result or native fitness flag is used to compute predicted source
transitions. All 110 original PID1 world traces are compared line-for-line.
"""
import json
import os
import sys
from pathlib import Path

ROOT=Path(__file__).resolve().parents[1]
MANIFEST=ROOT/"studies/CL1B-POLICY-P4F0-frozen-event-gated-24-stage-110-world-v1.json"
KEYS=(11,20,21,30,32,33,34)
CREDITS=6000

def execute(source,arm,trace_path):
    code=[dict(x) for x in source]
    goals=arm["goals"]
    assert len(code)==91 and len(goals)==24
    assert all(x in (0,1,2) for x in goals)
    disabled=arm["patchnext_suppressed"]
    mode=arm["feedback"]
    assert mode in ("truth","inverted","forced0")
    reg=[0,0,0,0]
    cells={}
    pc=0
    paid=0
    peak=92
    services=[]
    reads=[]
    edits=[]
    edges=[]
    previous_writes=[]
    previous_reads=[]
    fail_writes=[]
    delivered=[]
    recorded=(json.loads(row) for row in Path(trace_path).open())
    def validate(row):
        native=next(recorded,None)
        if native!=row:
            raise AssertionError({"first_divergence_paid_instruction":paid,"arm":arm["name"],
             "native":native,"independent_source":row})
    while paid<CREDITS:
        at=pc
        op=code[pc]
        kind=op["kind"]
        paid+=1
        if kind=="Const":
            reg[op["dst"]]=op["value"];pc=op["next"]
        elif kind in ("Add","Sub"):
            a,b=reg[op["lhs"]],reg[op["rhs"]]
            reg[op["dst"]]=a+b if kind=="Add" else a-b;pc=op["next"]
        elif kind=="Read":
            address=reg[op["address"]]
            reg[op["dst"]]=cells.get(address,0)
            if at==5:
                assert address==20
                reads.append({"step":paid,"value":reg[op["dst"]]})
            if at==69:
                assert address==34 and len(previous_writes)==1 and not previous_reads
                assert reg[op["dst"]]==previous_writes[0]["value"]
                previous_reads.append({"step":paid,"value":reg[op["dst"]]})
            pc=op["next"]
        elif kind=="Write":
            address=reg[op["address"]]
            value=reg[op["value"]]
            cells[address]=value
            if at==12:
                assert address==33 and value in (1,2,3)
                fail_writes.append({"step":paid,"value":value})
            if at==66:
                assert address==34 and not previous_writes
                previous_writes.append({"step":paid,"value":value})
            pc=op["next"]
        elif kind=="Branch":
            pc=op["nonzero"] if reg[op["test"]] else op["zero"]
        elif kind=="PatchConst":
            target=reg[op["node"]]
            requested=reg[op["value"]]
            assert target==46 and requested in (0,1,2)
            assert at in (45,88)
            if at==45:
                assert cells.get(32)==2 and len(previous_reads)==1
            if at==88:
                assert cells.get(33)==2 and code[12]["next"]==83 and requested==1
            before=code[46]["value"]
            code[target]["value"]=requested
            edits.append({"step":paid,"source_pc":at,"target":46,
                "before":before,"after":requested,"requested":requested})
            pc=op["next"]
        elif kind=="PatchNext":
            assert at==82 and len(edges)==0 and cells.get(32)==2
            target=reg[op["node"]]
            desired=reg[op["target"]]
            assert target==12 and desired==83
            before=code[12]["next"]
            assert before==13
            if not disabled: code[12]["next"]=desired
            edges.append({"step":paid,"source_pc":82,"target":12,"before":before,
                "after":code[12]["next"],"requested":desired,"suppressed":disabled})
            pc=op["next"]
        elif kind=="Halt":
            assert at in (28,61)
            pc=at
        else:
            raise AssertionError(("unknown original CIL opcode",kind,at))
        peak=max(peak,92+len(cells))
        assert len(cells)<=16 and peak<=300
        validate(["cil",paid,at,pc,list(reg),CREDITS-paid,
         [cells.get(k) for k in KEYS],92+len(cells),code[12]["next"],code[46]["value"]])
        if at==3:
            guess=reg[3]
            assert cells[11]==guess
            stage=cells.get(32,0)
            assert 0<=stage<24
            truth=int(guess==goals[stage])
            feedback=truth if mode=="truth" else 1-truth if mode=="inverted" else 0
            services.append({"step":paid,"episode":stage,"output":guess,
                "truth":truth,"feedback":feedback})
            delivered.append(feedback)
            cells[20]=feedback
            peak=max(peak,92+len(cells))
            validate(["feedback",paid,stage,goals[stage],guess,truth,feedback,
                [cells.get(k) for k in KEYS],92+len(cells),
                code[12]["next"],code[46]["value"]])
        if kind=="Halt":break
    else:
        raise AssertionError(("original paid credit exhaustion",arm["name"]))
    assert next(recorded,None) is None
    assert len(reads)==len(services)
    assert [x["value"] for x in reads]==delivered
    assert len(edges)<=1
    result={"name":arm["name"],"regime":arm["regime"],"goals":goals,
      "feedback":mode,"patchnext_suppressed":disabled,
      "paid_steps":paid,"task_probes":len(services),
      "actual_truthful_matches":sum(x["truth"] for x in services),
      "completed_stages":cells.get(32),"actor_claim":cells.get(21),
      "final_failure_edge":code[12]["next"],
      "final_executable_stride":code[46]["value"],
      "paid_const_edits":edits,"paid_edge_edits":edges,
      "paid_task_services":services,"paid_feedback_reads":reads,
      "paid_previous_goal_writes":previous_writes,
      "paid_previous_goal_reads":previous_reads,
      "paid_failure_count_writes":fail_writes,
      "peak_occupied":peak,"original_PID1_halted":True,"host_postload_source_edits":0}
    return result

def main():
    if len(sys.argv)!=3:
        raise SystemExit("usage: P4F0_NATIVE_REPLAY.py NATIVE_SUMMARY.json ORIGINAL_NATIVE_TRACES_DIR")
    manifest=json.loads(MANIFEST.read_text())
    native=json.loads(Path(sys.argv[1]).read_text())
    assert manifest["study"]==native["study"]=="CL1B-POLICY-P4F0"
    assert manifest["status"]=="PRE_NATIVE_FROZEN_EVENT_DRIVEN_DETECTOR_AND_HELDOUT_24_STAGE_PROTOCOL"
    assert native["level"]=="ORIGINAL_NATIVE_110_WORLD_EVENT_ARMED_PATCHNEXT24_EXACT_CIL"
    assert native["source_sha"] not in ("","local")
    if os.environ.get("GITHUB_SHA"):
        assert native["source_sha"]==os.environ["GITHUB_SHA"]
    source=manifest["original_native_CIL_source"]
    assert len(source)==91 and source[12]=={"kind":"Write","address":2,"value":1,"next":13}
    assert source[45]=={"kind":"PatchConst","node":1,"value":0,"next":80}
    assert source[82]=={"kind":"PatchNext","node":1,"target":0,"next":46}
    assert source[88]=={"kind":"PatchConst","node":1,"value":0,"next":89}
    arms=manifest["arms"]
    controls=manifest["controls"]
    assert len(arms)==108 and len(controls)==2 and len(native["pairs"])==54
    assert len(native["controls"])==2
    traces=Path(sys.argv[2])
    total=0
    by_regime={}
    def audit(arm,actual):
        nonlocal total
        path=traces/f"cl1b_policy_p4f0_original_{arm['name']}.jsonl"
        result=execute(source,arm,path)
        total+=result["paid_steps"]
        if result!=actual:
            mismatches={k:{"native":actual.get(k),"independent":v}
              for k,v in result.items() if actual.get(k)!=v}
            raise AssertionError({"arm":arm["name"],"native_summary_divergence":mismatches})
        return result
    for i in range(54):
        on_arm,off_arm=arms[2*i:2*i+2]
        n=native["pairs"][i]
        assert n["goals"]==on_arm["goals"]==off_arm["goals"]
        assert n["regime"]==on_arm["regime"]
        assert not on_arm["patchnext_suppressed"] and off_arm["patchnext_suppressed"]
        on=audit(on_arm,n["detector_on"])
        off=audit(off_arm,n["patchnext_suppressed"])
        assert on["actor_claim"]==off["actor_claim"]==1
        assert on["completed_stages"]==off["completed_stages"]==24
        assert on["actual_truthful_matches"]==off["actual_truthful_matches"]==24
        assert on["final_failure_edge"]==83 and off["final_failure_edge"]==13
        d=off["paid_steps"]-on["paid_steps"]
        p=off["task_probes"]-on["task_probes"]
        assert n["native_paid_saving"]==d and n["native_probe_saving"]==p
        r=by_regime.setdefault(on_arm["regime"],{
          "pairs":0,"net_paid_instruction_saving":0,"net_task_probe_saving":0,
          "beneficial":0,"harmful":0,"neutral":0})
        r["pairs"]+=1;r["net_paid_instruction_saving"]+=d;r["net_task_probe_saving"]+=p
        r["beneficial"]+=int(d>0);r["harmful"]+=int(d<0);r["neutral"]+=int(d==0)
    for arm,actual in zip(controls,native["controls"]):
        result=audit(arm,actual)
        if arm["feedback"]=="inverted":
            assert result["actor_claim"]==1
            assert result["actual_truthful_matches"]<24
        else:
            assert arm["feedback"]=="forced0" and result["actor_claim"]==0
    c=native["counts"]
    assert c["regimes"]==by_regime
    assert c["native_worlds"]==110 and c["paired_worlds"]==108
    assert c["native_controls"]==2 and c["all_truthful_paired_worlds_complete_24"]==108
    assert c["total_paid_original_instructions"]==total
    print(json.dumps({"study":"CL1B-POLICY-P4F0",
      "source_sha":native["source_sha"],
      "evidence_level":"110_ORIGINAL_NATIVE_WORLDS_EVERY_CHARGED_CIL_INSTRUCTION_REPLAY",
      "native_worlds":110,"paid_original_instructions_replayed":total,
      "regimes":by_regime,"first_native_divergence":None,
      "limits":"Author-selected threshold2 and fallback stride1; no self-invented policy, no open-ended organism."},indent=2))
if __name__=="__main__":
    main()
