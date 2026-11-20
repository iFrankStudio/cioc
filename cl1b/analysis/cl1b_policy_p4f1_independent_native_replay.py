#!/usr/bin/env python3
"""CL1B-POLICY-P4F1 separately implemented original CIL opcode replayer.
No Rust step result or native fitness flag is used to compute predicted source
transitions. All 110 original PID1 world traces are compared line-for-line.
"""
import json
import os
import sys
from pathlib import Path

ROOT=Path(__file__).resolve().parents[1]
MANIFEST=ROOT/"studies/CL1B-POLICY-P4F1-frozen-recoverable-source-policy-with-new-heldout-v1.json"
KEYS=(11,20,21,30,32,33,34,35,36)
CREDITS=6000

def execute(source,arm,trace_path):
    code=[dict(x) for x in source]
    goals=arm["goals"]
    assert len(code)==(106 if arm['source']=='F1' else 91) and len(goals)==24
    assert all(x in (0,1,2) for x in goals)
    disabled=arm["patchnext_suppressed"]
    mode=arm["feedback"]
    assert mode in ("truth","inverted","forced0")
    reg=[0,0,0,0]
    cells={}
    pc=0
    paid=0
    services=[]
    reads=[]
    edits=[]
    edges=[]
    previous_writes=[]
    previous_reads=[]
    fail_writes=[]
    retained_writes=[]
    retained_reads=[]
    mode_reads=[]
    mode_writes=[]
    delivered=[]
    base_occupancy=len(code)+1
    peak=base_occupancy
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
            if at==94:
                assert arm["source"]=="F1" and address==36 and reg[op["dst"]] in (0,1)
                mode_reads.append({"step":paid,"value":reg[op["dst"]]})
            if at==100:
                assert arm["source"]=="F1" and address==35 and len(retained_writes)==1
                assert reg[op["dst"]]==retained_writes[0]["value"]
                retained_reads.append({"step":paid,"value":reg[op["dst"]]})
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
            if at==92:
                assert arm["source"]=="F1" and address==35 and not retained_writes
                assert value==code[46]["value"]
                retained_writes.append({"step":paid,"value":value})
            if at in (98,105):
                assert arm["source"]=="F1" and address==36
                assert value==(1 if at==98 else 0)
                mode_writes.append({"step":paid,"value":value})
            pc=op["next"]
        elif kind=="Branch":
            pc=op["nonzero"] if reg[op["test"]] else op["zero"]
        elif kind=="PatchConst":
            target=reg[op["node"]]
            requested=reg[op["value"]]
            assert target==46 and requested in (0,1,2)
            assert at in (45,88,102)
            if at==45:
                assert cells.get(32)==2 and len(previous_reads)==1
            if at==88:
                assert cells.get(33)==2 and code[12]["next"]==83 and requested==1
                if arm["source"]=="F1":assert cells.get(36,0)==0
            if at==102:
                assert arm["source"]=="F1" and cells.get(33)==2
                assert code[12]["next"]==83 and cells.get(36)==1
                assert requested==cells[35]
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
        peak=max(peak,base_occupancy+len(cells))
        assert len(cells)<=16 and peak<=300
        validate(["cil",paid,at,pc,list(reg),CREDITS-paid,
         [cells.get(k) for k in KEYS],base_occupancy+len(cells),code[12]["next"],code[46]["value"]])
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
            peak=max(peak,base_occupancy+len(cells))
            validate(["feedback",paid,stage,goals[stage],guess,truth,feedback,
                [cells.get(k) for k in KEYS],base_occupancy+len(cells),
                code[12]["next"],code[46]["value"]])
        if kind=="Halt":break
    else:
        raise AssertionError(("original paid credit exhaustion",arm["name"]))
    assert next(recorded,None) is None
    assert len(reads)==len(services)
    assert [x["value"] for x in reads]==delivered
    assert len(edges)<=1
    result={"name":arm["name"],"regime":arm["regime"],"goals":goals,
      "source_family":arm["source"],
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
      "paid_retained_policy_writes":retained_writes,
      "paid_retained_policy_reads":retained_reads,
      "paid_active_mode_reads":mode_reads,
      "paid_active_mode_writes":mode_writes,
      "peak_occupied":peak,"original_PID1_halted":True,"host_postload_source_edits":0}
    return result

def main():
    if len(sys.argv)!=3:
        raise SystemExit("usage: P4F1_NATIVE_REPLAY.py NATIVE_SUMMARY.json TRACES")
    manifest=json.loads(MANIFEST.read_text())
    native=json.loads(Path(sys.argv[1]).read_text())
    assert manifest["study"]==native["study"]=="CL1B-POLICY-P4F1"
    assert manifest["status"]=="PRE_NATIVE_FROZEN_ACTOR_PAID_RETAINED_POLICY_ROLLBACK_WITH_NEW_HELDOUT"
    assert native["level"]=="ORIGINAL_NATIVE_182_WORLD_ACTOR_PAID_POLICY_RESTORE24"
    assert native["source_sha"] not in ("","local")
    if os.environ.get("GITHUB_SHA"):
        assert native["source_sha"]==os.environ["GITHUB_SHA"]
    source_f1=manifest["original_native_CIL_source_f1"]
    source_f0=manifest["baseline_source_f0"]
    assert len(source_f1)==106 and len(source_f0)==91
    assert source_f1[45]=={"kind":"PatchConst","node":1,"value":0,"next":91}
    assert source_f1[82]=={"kind":"PatchNext","node":1,"target":0,"next":46}
    assert source_f1[92]=={"kind":"Write","address":2,"value":0,"next":80}
    assert source_f1[102]=={"kind":"PatchConst","node":1,"value":0,"next":103}
    arms=manifest["arms"]
    goals=manifest["goal_cases"]
    assert len(arms)==180 and len(goals)==72
    assert len(manifest["controls"])==2
    assert len(native["f1_pairs"])==72
    assert len(native["new_heldout_f0_native_pairs"])==18
    assert len(native["controls"])==2
    traces=Path(sys.argv[2])
    total=0
    stats={}
    cross={}
    def audit(arm,actual):
        nonlocal total
        code=source_f1 if arm["source"]=="F1" else source_f0
        path=traces/f"cl1b_policy_p4f1_original_{arm['name']}.jsonl"
        observed=execute(code,arm,path)
        total+=observed["paid_steps"]
        if observed!=actual:
            diffs={k:{"native":actual.get(k),"independent":v}
                   for k,v in observed.items() if actual.get(k)!=v}
            raise AssertionError({"arm":arm["name"],"aggregate_divergence":diffs})
        return observed
    cursor=0
    first=0
    for index,g in enumerate(goals):
        a,b=arms[cursor:cursor+2]
        cursor+=2
        assert a["source"]==b["source"]=="F1"
        assert a["goals"]==b["goals"]==g["goals"]
        on_record=native["f1_pairs"][index]
        assert on_record["regime"]==g["regime"]
        on=audit(a,on_record["f1_detector_on"])
        off=audit(b,on_record["f1_patchnext_suppressed"])
        for x in (on,off):
            assert x["actor_claim"]==1 and x["completed_stages"]==24
            assert x["actual_truthful_matches"]==24
            assert len(x["paid_retained_policy_writes"])==1
            assert len(x["paid_edge_edits"])==1
        assert on["final_failure_edge"]==83 and off["final_failure_edge"]==13
        d=off["paid_steps"]-on["paid_steps"]
        p=off["task_probes"]-on["task_probes"]
        assert on_record["native_paid_saving"]==d
        assert on_record["native_probe_saving"]==p
        reg=stats.setdefault(g["regime"],{"pairs":0,"net_paid_instruction_saving":0,
          "net_task_probe_saving":0,"beneficial":0,"harmful":0,"neutral":0})
        reg["pairs"]+=1
        reg["net_paid_instruction_saving"]+=d
        reg["net_task_probe_saving"]+=p
        reg["beneficial"]+=int(d>0)
        reg["harmful"]+=int(d<0)
        reg["neutral"]+=int(d==0)
        if g["classification"]=="NEW_P4F1_HELDOUT":
            f0a,f0b=arms[cursor:cursor+2]
            cursor+=2
            assert f0a["source"]==f0b["source"]=="F0"
            evidence=native["new_heldout_f0_native_pairs"][first]
            first+=1
            assert evidence["regime"]==g["regime"]
            f0_on=audit(f0a,evidence["f0_native_on"])
            f0_off=audit(f0b,evidence["f0_native_off"])
            assert f0_on["actual_truthful_matches"]==f0_off["actual_truthful_matches"]==24
            delta=f0_on["paid_steps"]-on["paid_steps"]
            probe=f0_on["task_probes"]-on["task_probes"]
            assert evidence["f1_on_vs_f0_on_absolute_source_charged_saving"]==delta
            assert evidence["f1_on_vs_f0_on_actual_task_probe_saving"]==probe
            group=cross.setdefault(g["regime"],{
             "pairs":0,"absolute_f1_vs_f0_paid_instruction_saving":0,
             "absolute_f1_vs_f0_task_probe_saving":0})
            group["pairs"]+=1
            group["absolute_f1_vs_f0_paid_instruction_saving"]+=delta
            group["absolute_f1_vs_f0_task_probe_saving"]+=probe
    assert cursor==180 and first==18
    for arm,record in zip(manifest["controls"],native["controls"]):
        observed=audit(arm,record)
        if arm["feedback"]=="inverted":
            assert observed["actor_claim"]==1
            assert observed["actual_truthful_matches"]<24
        else:
            assert arm["feedback"]=="forced0" and observed["actor_claim"]==0
    counts=native["counts"]
    assert counts["native_worlds"]==182
    assert counts["f1_paired_worlds"]==144
    assert counts["fresh_heldout_f0_baseline_worlds"]==36
    assert counts["adversarial_controls"]==2
    assert counts["all_truthful_native_worlds_finish_24"]==180
    assert counts["total_original_paid_instructions"]==total
    assert counts["regimes"]==stats
    assert counts["fresh_heldout_cross_source"]==cross
    print(json.dumps({"study":"CL1B-POLICY-P4F1",
      "evidence_level":"182_ORIGINAL_NATIVE_WORLDS_INDEPENDENT_EXACT_CIL_REPLAY",
      "source_sha":native["source_sha"],
      "native_worlds":182,"original_paid_instructions_replayed":total,
      "f1_same_source_regimes":stats,
      "fresh_heldout_f1_vs_f0_cross_source":cross,
      "first_native_divergence":None,
      "limitations":"Author-authored remembered policy, mode flag and thresholds; source-distinct F1/F0 comparisons are not same-source causal controls."},indent=2))

if __name__=="__main__":
    main()
