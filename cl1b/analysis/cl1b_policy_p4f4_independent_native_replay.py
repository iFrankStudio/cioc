#!/usr/bin/env python3
"""P4F4 independent original-CIL replayer: every paid instruction and actual
environment feedback in 326 original native PID1 worlds, not Rust-derived truth.
"""
import json
import os
import sys
from pathlib import Path

ROOT=Path(__file__).resolve().parents[1]
FROZEN=ROOT/"studies/CL1B-POLICY-P4F4-frozen-actor-paid-temporary-success-monitor-326-world-v1.json"
CELLS=(11,20,21,30,32,33,34,35,36,37)
CREDIT=6000

def replay(source,arm,trace):
    code=[dict(x) for x in source]
    expected_nodes=142 if arm["source"]=="F4" else 132
    assert len(code)==expected_nodes
    goals=arm["goals"]
    mode=arm["feedback"]
    disabled=arm["patchnext_suppressed"]
    assert len(goals)==24 and all(g in (0,1,2) for g in goals)
    assert mode in ("truth","inverted","forced0")
    reg=[0,0,0,0]
    cells={}
    pc=0
    paid=0
    observed=0
    services=[]
    edits=[]
    edges=[]
    writes37=[]
    reads37=[]
    peak=expected_nodes+1
    rows=(json.loads(line) for line in Path(trace).open())
    def edges_now():
        return [code[12]["next"],code[34]["next"],code[46]["value"]]
    def check(row):
        actual=next(rows,None)
        if actual!=row:
            raise AssertionError({"arm":arm["name"],"paid":paid,
               "native":actual,"independent":row})
    while paid<CREDIT:
        op=code[pc]
        at=pc
        kind=op["kind"]
        paid+=1
        if kind=="Const":
            reg[op["dst"]]=op["value"];pc=op["next"]
        elif kind in ("Add","Sub"):
            a,b=reg[op["lhs"]],reg[op["rhs"]]
            reg[op["dst"]]=a+b if kind=="Add" else a-b
            pc=op["next"]
        elif kind=="Read":
            address=reg[op["address"]]
            reg[op["dst"]]=cells.get(address,0)
            if at==5:
                assert address==20
                observed+=1
            if at==110:
                assert address==37 and writes37
                assert writes37[-1]["value"]==reg[op["dst"]]
                reads37.append({"step":paid,"value":reg[op["dst"]]})
            pc=op["next"]
        elif kind=="Write":
            address=reg[op["address"]]
            value=reg[op["value"]]
            cells[address]=value
            if at in (128,124,133):
                assert arm["source"] in ("F4","F3") and address==37
                assert value==cells.get(30,0)
                writes37.append({"step":paid,"pc":at,"value":value})
            pc=op["next"]
        elif kind=="Branch":
            pc=op["nonzero"] if reg[op["test"]] else op["zero"]
        elif kind=="PatchConst":
            assert at in (45,88,102,122)
            target=reg[op["node"]]
            value=reg[op["value"]]
            assert target==46 and value in (0,1,2)
            if at==122:
                assert arm["source"] in ("F4","F3") and code[34]["next"]==106
                assert reads37
            before=code[target]["value"]
            code[target]["value"]=value
            edits.append({"step":paid,"pc":at,"before":before,"after":value})
            pc=op["next"]
        elif kind=="PatchNext":
            assert at in (82,131,136,139)
            target=reg[op["node"]]
            value=reg[op["target"]]
            expected={
              ("F4",82):(12,137),("F3",82):(12,83),
              ("F4",131):(34,132),("F3",131):(34,106),
              ("F4",136):(34,132),("F4",139):(34,106),
            }
            assert (arm["source"],at) in expected
            assert (target,value)==expected[(arm["source"],at)]
            before=code[target]["next"]
            if not disabled:code[target]["next"]=value
            edges.append({"step":paid,"pc":at,"target":target,
                "before":before,"after":code[target]["next"],"suppressed":disabled})
            pc=op["next"]
        elif kind=="Halt":
            pc=at
        else:
            raise AssertionError(("unknown original CIL",kind,at))
        occupied=expected_nodes+1+len(cells)
        assert len(cells)<=16 and occupied<=300
        peak=max(peak,occupied)
        check(["cil",paid,at,pc,list(reg),CREDIT-paid,
            [cells.get(k) for k in CELLS],occupied,edges_now()])
        if at==3:
            guess=reg[3]
            assert cells[11]==guess
            stage=cells.get(32,0)
            assert 0<=stage<24
            truth=int(guess==goals[stage])
            feedback=truth if mode=="truth" else 1-truth if mode=="inverted" else 0
            services.append({"step":paid,"stage":stage,"guess":guess,
              "truth":truth,"feedback":feedback})
            cells[20]=feedback
            occupied=expected_nodes+1+len(cells)
            peak=max(peak,occupied)
            check(["feedback",paid,stage,goals[stage],guess,truth,feedback,
                [cells.get(k) for k in CELLS],occupied,edges_now()])
        if kind=="Halt":
            break
    else:
        raise AssertionError(("exhausted original paid compute",arm["name"]))
    assert next(rows,None) is None
    assert observed==len(services)
    return {"name":arm["name"],"source":arm["source"],"regime":arm["regime"],
      "goals":goals,"feedback":mode,"suppressed":disabled,"steps":paid,
      "services":services,"read20_count":observed,
      "truthful_matches":sum(x["truth"] for x in services),
      "completed_stages":cells.get(32),"claim":cells.get(21),
      "code_edits":edits,"edge_edits":edges,
      "write37":writes37,"read37":reads37,"final_edges":edges_now(),
      "peak_occupied":peak,"original_pid_halted":True,"host_source_edits":0}

def main():
    if len(sys.argv)!=3:
        raise SystemExit("usage: script SUMMARY.json ORIGINAL_TRACE_DIR")
    summary=json.loads(Path(sys.argv[1]).read_text())
    manifest=json.loads(FROZEN.read_text())
    assert summary["study"]==manifest["study"]=="CL1B-POLICY-P4F4"
    assert summary["level"]=="ORIGINAL_NATIVE_326_WORLD_TEMPORARY_PAID_SUCCESS_MONITOR"
    assert summary["source_sha"] not in ("","local")
    if os.environ.get("GITHUB_SHA"):
        assert summary["source_sha"]==os.environ["GITHUB_SHA"]
    assert len(manifest["arms"])==324 and len(summary["records"])==108
    total=0
    worlds=0
    groups={}
    def check(arm,native):
        nonlocal total,worlds
        source=manifest["original_native_CIL_source_f4"] if arm["source"]=="F4" else manifest["baseline_source_f3"]
        path=Path(sys.argv[2])/f"cl1b_policy_p4f4_original_{arm['name']}.jsonl"
        independently=replay(source,arm,path)
        assert independently==native,(arm["name"],next((k for k in independently if independently[k]!=native.get(k)),None))
        total+=independently["steps"]
        worlds+=1
        return independently
    for i,g in enumerate(manifest["goal_cases"]):
        a,b,c=manifest["arms"][3*i:3*i+3]
        record=summary["records"][i]
        assert a["goals"]==b["goals"]==c["goals"]==g["goals"]
        assert record["regime"]==g["regime"] and record["goals"]==g["goals"]
        on=check(a,record["f4_on"])
        off=check(b,record["f4_off"])
        base=check(c,record["f3_on"])
        assert all(x["truthful_matches"]==x["completed_stages"]==24
                    and x["claim"]==1 for x in (on,off,base))
        d=off["steps"]-on["steps"]
        p=len(off["services"])-len(on["services"])
        x=base["steps"]-on["steps"]
        xp=len(base["services"])-len(on["services"])
        assert record["same_source_bundle_paid_saving"]==d
        assert record["same_source_bundle_probe_saving"]==p
        assert record["cross_source_f4_vs_f3_paid_saving"]==x
        assert record["cross_source_f4_vs_f3_probe_saving"]==xp
        group=groups.setdefault(g["regime"],{"cases":0,"same_source_paid":0,"cross_source_paid":0,"cross_source_probes_saved":0})
        group["cases"]+=1
        group["same_source_paid"]+=d
        group["cross_source_paid"]+=x
        group["cross_source_probes_saved"]+=xp
    for arm,native in zip(manifest["controls"],summary["controls"]):
        check(arm,native)
    assert worlds==326
    assert total==summary["counts"]["total_original_paid_instructions"]
    assert summary["counts"]["worlds"]==326
    print(json.dumps({"study":"CL1B-POLICY-P4F4",
      "original_native_worlds_exactly_replayed":worlds,
      "original_charged_CIL_instruction_count_exactly_replayed":total,
      "first_divergence":None,"source_sha":summary["source_sha"],
      "regimes":groups,"limits":"No self-invented algorithm or cross-world lineage"},indent=2))

if __name__=="__main__":
    main()
