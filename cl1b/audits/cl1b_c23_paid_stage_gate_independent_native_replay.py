#!/usr/bin/env python3
"""C23 fully independent charged-CIL original native interpreter on 186 worlds, with true PID1 paid stage-maturity Read32/Branch gating. No earned ROI prediction or B3.
"""
import json
import os
import sys
from pathlib import Path

ROOT=Path(__file__).resolve().parents[1]
FROZEN=ROOT/"studies/CL1B-C23-PAID-STAGE-GATE-frozen-186-worlds-v1.json"
PARENT=ROOT/"studies/CL1B-POLICY-P4F6-frozen-actor-paid-clean-success-refill-542-world-v1.json"
C22=ROOT/"studies/CL1B-C22-PAID-PATCHNEXT-SELF-EDIT-RETURN-frozen-141-worlds-v1.json"
CELLS=(11,20,21,30,32,33,34,35,36,37,38,39)
CREDIT=6000

def replay(source,arm,trace):
    code=[dict(x) for x in source]
    expected_nodes=184 if arm["source"]=="F6" else 180 if arm["source"]=="F6BASE" else 153
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
    reads38=[]
    writes38=[]
    budget_gates=[]
    reads39=[]
    writes39=[]
    clean_streak_gates=[]
    paid_stage_reads=[]
    paid_stage_branches=[]
    peak=expected_nodes+1
    rows=(json.loads(line) for line in Path(trace).open())
    def edges_now():
        return [code[12]["next"],code[34]["next"],code[46]["value"],code[133]["next"]]
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
            if at==181:
                assert arm["source"]=="F6" and address==32
                assert reg[op["dst"]]==cells.get(32,0)
                paid_stage_reads.append({"step":paid,"stage":reg[op["dst"]]})
            if at==143:
                assert arm["source"] in ("F6","F6BASE","F5") and address==38
                assert reg[op["dst"]] in (0,1)
                reads38.append({"step":paid,"value":reg[op["dst"]]})
            if at==163:
                assert arm["source"] in ("F6","F6BASE") and address==39
                assert reg[op["dst"]] in (0,1,2,3)
                reads39.append({"step":paid,"value":reg[op["dst"]]})
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
                assert arm["source"] in ("F6","F6BASE","F5") and address==37
                assert value==cells.get(30,0)
                writes37.append({"step":paid,"pc":at,"value":value})
            if at==152:
                assert arm["source"] in ("F6","F6BASE","F5") and address==38 and value==1
                assert sum(e["pc"]==122 for e in edits)==1+sum(x["value"]==1 for x in writes38)
                writes38.append({"step":paid,"value":value})
            if at==174:
                assert arm["source"] in ("F6","F6BASE") and address==38 and value==0
                assert cells.get(39)==0
                writes38.append({"step":paid,"value":value})
            if at in (161,167,172):
                assert arm["source"] in ("F6","F6BASE") and address==39
                assert (1<=value<=4) if at==167 else value==0
                writes39.append({"step":paid,"pc":at,"value":value})
            pc=op["next"]
        elif kind=="Branch":
            pc=op["nonzero"] if reg[op["test"]] else op["zero"]
            if at==182:
                assert arm["source"]=="F6" and reg[0]==cells.get(32,0)
                assert pc==(183 if reg[0]!=0 else 127)
                paid_stage_branches.append({"step":paid,"stage":reg[0],"next":pc})
            if at==146:
                assert arm["source"] in ("F6","F6BASE","F5")
                meter=cells.get(38,0)
                assert pc==(140 if meter==1 else 138)
                budget_gates.append({"step":paid,"meter":meter,"next":pc})
            if at in (158,170):
                assert arm["source"] in ("F6","F6BASE")
                branch_value=cells.get(33,0) if at==158 else cells.get(39,0)
                expected_jump=(159 if branch_value!=0 else 162) if at==158 else (171 if branch_value==4 else 62)
                assert pc==expected_jump
                clean_streak_gates.append({"step":paid,"pc":at,"value":branch_value,"next":pc})
        elif kind=="PatchConst":
            assert at in (45,88,102,122)
            target=reg[op["node"]]
            value=reg[op["value"]]
            assert target==46 and value in (0,1,2)
            if at==122:
                assert arm["source"] in ("F6","F6BASE","F5") and code[34]["next"]==106
                assert reads37
            before=code[target]["value"]
            code[target]["value"]=value
            edits.append({"step":paid,"pc":at,"before":before,"after":value})
            pc=op["next"]
        elif kind=="PatchNext":
            assert at in (82,131,136,139,155,177)
            target=reg[op["node"]]
            value=reg[op["target"]]
            expected={
              ("F6",82):(12,137),("F6BASE",82):(12,137),("F5",82):(12,137),
              ("F6",131):(34,132),("F6BASE",131):(34,132),("F5",131):(34,132),
              ("F6",136):(34,132),("F6BASE",136):(34,132),("F5",136):(34,132),
              ("F6",139):(34,106),("F6BASE",139):(34,106),("F5",139):(34,106),
              ("F6",155):(133,156),("F6BASE",155):(133,156),("F6",177):(133,62),("F6BASE",177):(133,62),
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
    online=sum(e["pc"]==122 for e in edits)
    if arm["source"] in ("F6","F6BASE"):
        ups=sum(x["value"]==1 for x in writes38)
        downs=sum(x["value"]==0 for x in writes38)
        assert online==ups
        assert downs<=ups<=downs+1
        assert cells.get(38,0)==ups-downs
        assert sum(e["pc"]==155 for e in edges)==ups
        assert sum(e["pc"]==177 for e in edges)==downs
    else:
        assert online==len(writes38)<=1
        assert cells.get(38,0)==len(writes38)
        assert not reads39 and not writes39 and not clean_streak_gates
    return {"name":arm["name"],"source":arm["source"],"regime":arm["regime"],
      "goals":goals,"feedback":mode,"suppressed":disabled,"steps":paid,
      "services":services,"read20_count":observed,
      "truthful_matches":sum(x["truth"] for x in services),
      "completed_stages":cells.get(32),"claim":cells.get(21),
      "code_edits":edits,"edge_edits":edges,
      "write37":writes37,"read37":reads37,
      "write38":writes38,"read38":reads38,"budget_gates":budget_gates,
      "write39":writes39,"read39":reads39,"clean_streak_gates":clean_streak_gates,
      "paid_stage_reads":paid_stage_reads,"paid_stage_branches":paid_stage_branches,
      "online_edit_budget_meter":cells.get(38,0),"final_edges":edges_now(),
      "peak_occupied":peak,"original_pid_halted":True,"host_source_edits":0}

def main():
    if len(sys.argv)!=3:
        raise SystemExit("usage: script SUMMARY.json ORIGINAL_TRACE_DIR")
    summary=json.loads(Path(sys.argv[1]).read_text())
    manifest=json.loads(FROZEN.read_text())
    assert summary["study"]==manifest["study"]=="CL1B-POLICY-P4F6"
    assert summary["level"]=="ORIGINAL_NATIVE_542_WORLD_ACTOR_PAID_FOUR_CLEAN_SUCCESS_EDIT_REFILL"
    assert summary["source_sha"] not in ("","local")
    if os.environ.get("GITHUB_SHA"):
        assert summary["source_sha"]==os.environ["GITHUB_SHA"]
    assert len(manifest["arms"])==540 and len(summary["records"])==180
    total=0
    worlds=0
    groups={}
    def check(arm,native):
        nonlocal total,worlds
        source=manifest["gate_source_F7_184_nodes"] if arm["source"]=="F6" else parent["original_native_CIL_source_f6"] if arm["source"]=="F6BASE" else parent["baseline_source_f5"]
        path=Path(sys.argv[2])/f"cl1b_policy_p4f6_original_{arm['name']}.jsonl"
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
        on=check(a,record["f6_on"])
        off=check(b,record["f6_off"])
        base=check(c,record["f5_on"])
        assert all(x["truthful_matches"]==x["completed_stages"]==24
                    and x["claim"]==1 for x in (on,off,base))
        d=off["steps"]-on["steps"]
        p=len(off["services"])-len(on["services"])
        x=base["steps"]-on["steps"]
        xp=len(base["services"])-len(on["services"])
        assert record["same_source_bundle_paid_saving"]==d
        assert record["same_source_bundle_probe_saving"]==p
        assert record["cross_source_f6_vs_f5_paid_saving"]==x
        assert record["cross_source_f6_vs_f5_probe_saving"]==xp
        group=groups.setdefault(g["regime"],{"cases":0,"same_source_paid":0,"cross_source_paid":0,"cross_source_probes_saved":0})
        group["cases"]+=1
        group["same_source_paid"]+=d
        group["cross_source_paid"]+=x
        group["cross_source_probes_saved"]+=xp
    for arm,native in zip(manifest["controls"],summary["controls"]):
        check(arm,native)
    assert worlds==542
    assert total==summary["counts"]["total_original_paid_instructions"]
    assert summary["counts"]["worlds"]==542
    print(json.dumps({"study":"CL1B-POLICY-P4F6",
      "original_native_worlds_exactly_replayed":worlds,
      "original_charged_CIL_instruction_count_exactly_replayed":total,
      "first_divergence":None,"source_sha":summary["source_sha"],
      "regimes":groups,"limits":"No self-invented algorithm or cross-world lineage"},indent=2))

if __name__=="__main__":
    main()
