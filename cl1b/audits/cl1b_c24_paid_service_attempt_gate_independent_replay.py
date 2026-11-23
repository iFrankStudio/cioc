#!/usr/bin/env python3
"""C24 independently replays all 186 original native PID-paid CIL worlds, including real service-count Read40/Write40 and conditional PatchNext after paid attempt proxy. Fixed authored rule, NOT B3.
"""
import json
import os
import sys
from pathlib import Path

ROOT=Path(__file__).resolve().parents[1]
FROZEN=ROOT/"studies/CL1B-C24-PAID-SERVICE-COUNT-EDIT-GATE-frozen-186-worlds-v1.json"
C23=ROOT/"studies/CL1B-C23-PAID-STAGE-GATE-frozen-186-worlds-v1.json"
PARENT=ROOT/"studies/CL1B-POLICY-P4F6-frozen-actor-paid-clean-success-refill-542-world-v1.json"
C22=ROOT/"studies/CL1B-C22-PAID-PATCHNEXT-SELF-EDIT-RETURN-frozen-141-worlds-v1.json"
CELLS=(11,20,21,30,32,33,34,35,36,37,38,39,40)
CREDIT=6000

def replay(source,arm,trace):
    code=[dict(x) for x in source]
    expected_nodes=195 if arm["source"]=="F6" else 180 if arm["source"]=="F6BASE" else 153
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
    paid_attempt_reads=[]
    paid_attempt_decisions=[]
    paid_counter_reads=[]
    paid_counter_writes=[]
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
                assert arm["source"]=="F6" and address==40
                assert reg[op["dst"]]==cells.get(40,0)==len(services)
                paid_attempt_reads.append({"step":paid,"attempts":reg[op["dst"]]})
            if at==185:
                assert arm["source"]=="F6" and address==40
                assert reg[op["dst"]]==cells.get(40,0)==len(services)-1
                paid_counter_reads.append({"step":paid,"value":reg[op["dst"]]})
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
            if at==189:
                assert arm["source"]=="F6" and address==40
                assert value==len(services)
                paid_counter_writes.append({"step":paid,"value":value})
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
            if at in (190,194):
                assert arm["source"]=="F6"
                attempts=cells.get(40,0)
                if at==190:
                    assert reg[0]==attempts-2
                    assert pc==(191 if attempts==2 else 192)
                else:
                    assert reg[0]==attempts-3
                    assert pc==(191 if attempts==3 else 127)
                paid_attempt_decisions.append({"step":paid,"pc":at,"attempts":attempts,"next":pc})
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
        assert len(cells)<=17 and occupied<=320
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
      "paid_attempt_reads":paid_attempt_reads,"paid_attempt_decisions":paid_attempt_decisions,
      "paid_counter_reads":paid_counter_reads,"paid_counter_writes":paid_counter_writes,
      "online_edit_budget_meter":cells.get(38,0),"final_edges":edges_now(),
      "peak_occupied":peak,"original_pid_halted":True,"host_source_edits":0}


def first_patchnext_physical_effect(folder, on, off):
    assert on["source"]==off["source"]=="F6"
    assert on["goals"]==off["goals"] and on["feedback"]==off["feedback"]=="truth"
    assert on["suppressed"] is False and off["suppressed"] is True
    def events(r):
        return [json.loads(line) for line in (Path(folder)/f"cl1b_c24_original_{r['name']}.jsonl").read_text().splitlines()]
    aa,bb=events(on),events(off)
    i=next((i for i,(a,b) in enumerate(zip(aa,bb)) if a!=b),None)
    if i is None:
        assert len(aa)==len(bb)
        return {"case":on["name"],"physical_patchnext_exposed":False,"original_native_full_trace_identical":True}
    assert aa[:i]==bb[:i]
    a,b=aa[i],bb[i]
    assert a[:8]==b[:8] and a[0]==b[0]=="cil"
    assert a[2] in (82,131,136,139,155,177),"First native difference must be charged PatchNext effect"
    differences=[j for j in range(4) if a[8][j]!=b[8][j]]
    assert len(differences)==1
    return {"case":on["name"],"physical_patchnext_exposed":True,
        "identical_all_original_paid_and_feedback_prior_events":i,
        "first_effect_paid_PC":a[2],"changed_live_edge_slot":differences[0],
        "active_edge":a[8][differences[0]],"suppressed_edge":b[8][differences[0]]}

def main():
    if len(sys.argv)!=3:
        raise SystemExit("usage: C23_AUDIT SUMMARY.json ORIGINAL_TRACE_DIR")
    summary=json.loads(Path(sys.argv[1]).read_text())
    manifest=json.loads(FROZEN.read_text())
    parent=json.loads(PARENT.read_text())
    c22=json.loads(C22.read_text())
    assert summary["study"]==manifest["study"]=="CL1B-C23-F7-PAID-OBSERVED-STAGE-MATURITY-CODE-EDIT-GATE"
    assert summary["level"]=="ORIGINAL_NATIVE_C23_186_WORLD_PID_PAID_STAGE_MATURITY_GATE"
    assert manifest["status"]=="PRE_NATIVE_FULL_FROZEN_TRUE_NEW_45_GOALS"
    assert parent["study"]=="CL1B-POLICY-P4F6"
    assert summary["source_sha"] not in ("","local")
    if os.environ.get("GITHUB_SHA"):
        assert summary["source_sha"]==os.environ["GITHUB_SHA"]
    f7=manifest["gate_source_F7_184_nodes"]
    f6=parent["original_native_CIL_source_f6"]
    f5=parent["baseline_source_f5"]
    assert len(f7)==184 and len(f6)==180 and len(f5)==153
    assert f7[81]==dict(f6[81],next=180)
    assert f7[:81]==f6[:81] and f7[82:180]==f6[82:]
    assert f7[180:]==[
        {"kind":"Const","dst":2,"value":32,"next":181},
        {"kind":"Read","dst":0,"address":2,"next":182},
        {"kind":"Branch","test":0,"nonzero":183,"zero":127},
        {"kind":"Const","dst":0,"value":137,"next":82}]
    old={tuple(x["goals"]) for x in parent["goal_cases"]+c22["goal_cases"]}
    frozen={tuple(x["goals"]) for x in manifest["goal_cases"]}
    assert len(frozen)==45 and not old.intersection(frozen)
    assert len(manifest["arms"])==180 and len(summary["records"])==45
    assert len(manifest["controls"])==len(summary["controls"])==6
    total=0
    worlds=0
    groups={}
    proofs=[]
    totals={"f7_on":0,"f7_off":0,"f6_original":0,"f5_original":0}
    correct={k:0 for k in totals}
    actor_gate={"paid_stage_reads":0,"gate_skip_zero_stage":0,"gate_allow_after_stage":0,
                "paid_PC82_executions_after_allowed_gate":0}
    def check(arm,native):
        nonlocal total,worlds
        source=f7 if arm["source"]=="F6" else f6 if arm["source"]=="F6BASE" else f5
        path=Path(sys.argv[2])/f"cl1b_c24_original_{arm['name']}.jsonl"
        independent=replay(source,arm,path)
        assert independent==native,(arm["name"],next((k for k in independent if independent[k]!=native.get(k)),None))
        if arm["source"]=="F6":
            assert len(independent["paid_stage_reads"])==len(independent["paid_stage_branches"])
            assert all(x["stage"]==y["stage"] and x["step"]+1==y["step"]
                for x,y in zip(independent["paid_stage_reads"],independent["paid_stage_branches"]))
            assert all(x["next"]==(183 if x["stage"] else 127)
                for x in independent["paid_stage_branches"])
        else:
            assert independent["paid_stage_reads"]==independent["paid_stage_branches"]==[]
        total+=independent["steps"]
        worlds+=1
        return independent
    for i,g in enumerate(manifest["goal_cases"]):
        a,b,c,d=manifest["arms"][4*i:4*i+4]
        n=summary["records"][i]
        assert a["goals"]==b["goals"]==c["goals"]==d["goals"]==g["goals"]
        assert a["source"]==b["source"]=="F6" and c["source"]=="F6BASE" and d["source"]=="F5"
        assert n["regime"]==g["regime"] and n["goals"]==g["goals"]
        x=check(a,n["f7_on"])
        y=check(b,n["f7_off"])
        z=check(c,n["f6_original"])
        w=check(d,n["f5_original"])
        for label,outcome in (("f7_on",x),("f7_off",y),("f6_original",z),("f5_original",w)):
            assert 0<=outcome["truthful_matches"]<=24 and 0<=outcome["completed_stages"]<=24
            totals[label]+=outcome["steps"]
            correct[label]+=outcome["truthful_matches"]
        proofs.append(first_patchnext_physical_effect(sys.argv[2],x,y))
        actor_gate["paid_stage_reads"]+=len(x["paid_stage_reads"])
        actor_gate["gate_skip_zero_stage"]+=sum(e["stage"]==0 for e in x["paid_stage_branches"])
        actor_gate["gate_allow_after_stage"]+=sum(e["stage"]!=0 for e in x["paid_stage_branches"])
        actor_gate["paid_PC82_executions_after_allowed_gate"]+=sum(e["pc"]==82 for e in x["edge_edits"])
        assert n["same_source_bundle_paid_saving"]==y["steps"]-x["steps"]
        assert n["same_source_bundle_probe_saving"]==len(y["services"])-len(x["services"])
        assert n["F7_on_minus_F6_original_paid_CIL"]==x["steps"]-z["steps"]
        assert n["F7_on_minus_F5_original_paid_CIL"]==x["steps"]-w["steps"]
        assert n["cross_source_f6_vs_f5_paid_saving"]==w["steps"]-x["steps"]
        assert n["cross_source_f6_vs_f5_probe_saving"]==len(w["services"])-len(x["services"])
        gp=groups.setdefault(g["regime"],{"cases":0,"same_source_paid_saving":0,
              "new_F7_minus_original_F6_paid":0,"new_F7_minus_F5_paid":0})
        gp["cases"]+=1
        gp["same_source_paid_saving"]+=y["steps"]-x["steps"]
        gp["new_F7_minus_original_F6_paid"]+=x["steps"]-z["steps"]
        gp["new_F7_minus_F5_paid"]+=x["steps"]-w["steps"]
    for arm,native in zip(manifest["controls"],summary["controls"]):
        check(arm,native)
    assert worlds==186 and len(proofs)==45
    assert sum(g["cases"] for g in groups.values())==45
    assert summary["counts"]["worlds"]==186
    assert summary["counts"]["total_original_paid_instructions"]==total
    print(json.dumps({
        "study":manifest["study"],"status":"C23_ORIGINAL_NATIVE_186_COMPLETE_INDEPENDENT_PAID_CIL_REPLAY",
        "original_native_worlds_exactly_replayed":worlds,
        "original_charged_CIL_instruction_count_exactly_replayed":total,
        "original_native_sha":summary["source_sha"],"first_divergence":None,
        "arm_fees":totals,"arm_true_matched_stages":correct,
        "actor_paid_stage_gate_evidence":actor_gate,
        "same_source_first_true_physical_patchnext_effect":proofs,
        "same_source_paid_savings_sum":totals["f7_off"]-totals["f7_on"],
        "F7_on_minus_original_F6_paid":totals["f7_on"]-totals["f6_original"],
        "F7_on_minus_original_F5_paid":totals["f7_on"]-totals["f5_original"],
        "regimes":groups,
        "limitations":"Paid PID1 stage maturity observation externally authored. No true ROI model, no learned search algorithm, no B3/B4."},indent=2))

if __name__=="__main__":
    main()
