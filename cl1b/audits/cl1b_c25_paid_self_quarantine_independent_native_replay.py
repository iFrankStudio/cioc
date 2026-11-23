#!/usr/bin/env python3
"""C25 independently replays 276 frozen true original native PID-paid CIL self-quarantine worlds, including real service-count Read40/Write40 and conditional PatchNext after paid attempt proxy. Fixed authored rule, NOT B3.
"""
import json
import os
import sys
from pathlib import Path

ROOT=Path(__file__).resolve().parents[1]
FROZEN=ROOT/"studies/CL1B-C25-PAID-SELF-QUARANTINE-frozen-276-worlds-v1.json"
OLD_C24=ROOT/"studies/CL1B-C24-PAID-SERVICE-COUNT-EDIT-GATE-frozen-186-worlds-v1.json"
C23=ROOT/"studies/CL1B-C23-PAID-STAGE-GATE-frozen-186-worlds-v1.json"
PARENT=ROOT/"studies/CL1B-POLICY-P4F6-frozen-actor-paid-clean-success-refill-542-world-v1.json"
C22=ROOT/"studies/CL1B-C22-PAID-PATCHNEXT-SELF-EDIT-RETURN-frozen-141-worlds-v1.json"
CELLS=(11,20,21,30,32,33,34,35,36,37,38,39,40)
CREDIT=6000

def replay(source,arm,trace):
    code=[dict(x) for x in source]
    expected_nodes=210 if arm["source"] in ("F9","F10") else 195 if arm["source"]=="F8BASE" else 153
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
    paid_quarantine_self_edits=[]
    peak=expected_nodes+1
    rows=(json.loads(line) for line in Path(trace).open())
    def edges_now():
        return [code[12]["next"],code[34]["next"],code[46]["value"],code[133]["next"]]+[code[j]["next"] if len(code)>j else -1 for j in (128,134,138,153,175)]
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
                assert arm["source"] in ("F9","F10","F8BASE") and address==40
                assert reg[op["dst"]]==cells.get(40,0)==len(services)
                paid_attempt_reads.append({"step":paid,"attempts":reg[op["dst"]]})
            if at==185:
                assert arm["source"] in ("F9","F10","F8BASE") and address==40
                assert reg[op["dst"]]==cells.get(40,0)==len(services)-1
                paid_counter_reads.append({"step":paid,"value":reg[op["dst"]]})
            if at==143:
                assert arm["source"] in ("F9","F10","F8BASE","F5") and address==38
                assert reg[op["dst"]] in (0,1)
                reads38.append({"step":paid,"value":reg[op["dst"]]})
            if at==163:
                assert arm["source"] in ("F9","F10","F8BASE") and address==39
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
                assert arm["source"] in ("F9","F10","F8BASE") and address==40
                assert value==len(services)
                paid_counter_writes.append({"step":paid,"value":value})
            if at in (128,124,133):
                assert arm["source"] in ("F9","F10","F8BASE","F5") and address==37
                assert value==cells.get(30,0)
                writes37.append({"step":paid,"pc":at,"value":value})
            if at==152:
                assert arm["source"] in ("F9","F10","F8BASE","F5") and address==38 and value==1
                assert sum(e["pc"]==122 for e in edits)==1+sum(x["value"]==1 for x in writes38)
                writes38.append({"step":paid,"value":value})
            if at==174:
                assert arm["source"] in ("F9","F10","F8BASE") and address==38 and value==0
                assert cells.get(39)==0
                writes38.append({"step":paid,"value":value})
            if at in (161,167,172):
                assert arm["source"] in ("F9","F10","F8BASE") and address==39
                assert (1<=value<=4) if at==167 else value==0
                writes39.append({"step":paid,"pc":at,"value":value})
            pc=op["next"]
        elif kind=="Branch":
            pc=op["nonzero"] if reg[op["test"]] else op["zero"]
            if at in (190,194):
                assert arm["source"] in ("F9","F10","F8BASE")
                attempts=cells.get(40,0)
                if at==190:
                    assert reg[0]==attempts-2
                    expected_next=(195 if arm["source"]=="F10" else 191) if attempts==2 else 192
                else:
                    assert reg[0]==attempts-3
                    expected_next=195 if arm["source"]=="F10" else (191 if attempts==3 else (195 if arm["source"]=="F9" else 127))
                assert pc==expected_next
                paid_attempt_decisions.append({"step":paid,"pc":at,"attempts":attempts,"next":pc})
            if at==146:
                assert arm["source"] in ("F9","F10","F8BASE","F5")
                meter=cells.get(38,0)
                assert pc==(140 if meter==1 else 138)
                budget_gates.append({"step":paid,"meter":meter,"next":pc})
            if at in (158,170):
                assert arm["source"] in ("F9","F10","F8BASE")
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
                assert arm["source"] in ("F9","F10","F8BASE","F5") and code[34]["next"]==106
                assert reads37
            before=code[target]["value"]
            code[target]["value"]=value
            edits.append({"step":paid,"pc":at,"before":before,"after":value})
            pc=op["next"]
        elif kind=="PatchNext":
            assert at in (82,131,136,139,155,177,197,200,203,206,209)
            target=reg[op["node"]]
            value=reg[op["target"]]
            expected={
              ("F9",82):(12,137),("F10",82):(12,137),("F8BASE",82):(12,137),("F5",82):(12,137),
              ("F9",131):(34,132),("F10",131):(34,132),("F8BASE",131):(34,132),("F5",131):(34,132),
              ("F9",136):(34,132),("F10",136):(34,132),("F8BASE",136):(34,132),("F5",136):(34,132),
              ("F9",139):(34,106),("F10",139):(34,106),("F8BASE",139):(34,106),("F5",139):(34,106),
              ("F9",155):(133,156),("F10",155):(133,156),("F8BASE",155):(133,156),("F9",177):(133,62),("F10",177):(133,62),("F8BASE",177):(133,62),
            }
            if at in (197,200,203,206,209):
                assert arm["source"] in ("F9","F10")
                assert (target,value)=={197:(128,46),200:(134,123),203:(138,140),206:(153,134),209:(175,178)}[at]
                assert code[target]["kind"] in ("Const","Write")
                before=code[target]["next"]
                if not disabled:code[target]["next"]=value
                paid_quarantine_self_edits.append({"step":paid,"pc":at,"target":target,
                    "before":before,"after":code[target]["next"],"suppressed":disabled})
            else:
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
        assert len(cells)<=17 and occupied<=340
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
    if arm["source"] in ("F9","F10","F8BASE"):
        ups=sum(x["value"]==1 for x in writes38)
        downs=sum(x["value"]==0 for x in writes38)
        assert online==ups
        assert downs<=ups<=downs+1
        assert cells.get(38,0)==ups-downs
        if arm["source"]=="F8BASE":
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
      "paid_quarantine_self_edits":paid_quarantine_self_edits,
      "online_edit_budget_meter":cells.get(38,0),"final_edges":edges_now(),
      "peak_occupied":peak,"original_pid_halted":True,"host_source_edits":0}


def first_patchnext_physical_effect(folder, on, off):
    assert on["source"]==off["source"] and on["source"] in ("F9","F10")
    assert on["goals"]==off["goals"] and on["feedback"]==off["feedback"]=="truth"
    assert on["suppressed"] is False and off["suppressed"] is True
    def events(r):
        return [json.loads(line) for line in (Path(folder)/f"cl1b_c25_original_{r['name']}.jsonl").read_text().splitlines()]
    aa,bb=events(on),events(off)
    i=next((i for i,(a,b) in enumerate(zip(aa,bb)) if a!=b),None)
    if i is None:
        assert len(aa)==len(bb)
        return {"case":on["name"],"physical_patchnext_exposed":False,"original_native_full_trace_identical":True}
    assert aa[:i]==bb[:i]
    a,b=aa[i],bb[i]
    assert a[:8]==b[:8] and a[0]==b[0]=="cil"
    assert a[2] in (82,131,136,139,155,177,197,200,203,206,209),"First native difference must be charged PatchNext physical effect"
    differences=[j for j in range(9) if a[8][j]!=b[8][j]]
    assert len(differences)==1
    return {"case":on["name"],"physical_patchnext_exposed":True,
        "identical_all_original_paid_and_feedback_prior_events":i,
        "first_effect_paid_PC":a[2],"changed_live_edge_slot":differences[0],
        "active_edge":a[8][differences[0]],"suppressed_edge":b[8][differences[0]]}

def main():
    if len(sys.argv)!=3:raise SystemExit("usage: C24_NATIVE_AUDIT SUMMARY.json ORIGINAL_TRACE_DIR")
    summary=json.loads(Path(sys.argv[1]).read_text())
    manifest=json.loads(FROZEN.read_text())
    parent=json.loads(PARENT.read_text())
    original_c24=json.loads(OLD_C24.read_text())
    assert original_c24["study"]=="CL1B-C24-F8-PAID-CUMULATIVE-SERVICE-PROXY-EDIT-GATE"
    c22=json.loads(C22.read_text())
    c23=json.loads(C23.read_text())
    study="CL1B-C24B-F8-PAID-SERVICE-COUNT-DISJOINT-HELDOUT-REPLICATION"
    assert summary["study"]==manifest["study"]==study
    assert summary["level"]=="ORIGINAL_NATIVE_C24B_186_WORLD_UNCHANGED_F8_PAID_SERVICE_COUNT_REPLICATION"
    assert manifest["status"]=="PRE_NATIVE_FROZEN_NEW_DISJOINT_STRESS_45_GOALS"
    assert parent["study"]=="CL1B-POLICY-P4F6"
    assert summary["source_sha"] not in ("","local")
    if os.environ.get("GITHUB_SHA"):assert summary["source_sha"]==os.environ["GITHUB_SHA"]
    f8=manifest["exact_frozen_F8_source_195_nodes"]
    assert f8==original_c24["exact_frozen_F8_source_195_nodes"],"C24B original F8 changed compared to C24"
    f6=parent["original_native_CIL_source_f6"]
    f5=parent["baseline_source_f5"]
    assert len(f8)==195 and len(f6)==180 and len(f5)==153
    assert f8[3]==dict(f6[3],next=184)
    assert f8[81]==dict(f6[81],next=180)
    assert all(f8[j]==f6[j] for j in range(180) if j not in (3,81))
    assert f8[180]=={"kind":"Const","dst":2,"value":40,"next":181}
    assert f8[181]=={"kind":"Read","dst":0,"address":2,"next":182}
    assert f8[182]=={"kind":"Const","dst":2,"value":2,"next":183}
    assert f8[183]=={"kind":"Sub","dst":0,"lhs":0,"rhs":2,"next":190}
    assert f8[184]=={"kind":"Const","dst":2,"value":40,"next":185}
    assert f8[185]=={"kind":"Read","dst":0,"address":2,"next":186}
    assert f8[189]=={"kind":"Write","address":2,"value":0,"next":4}
    assert f8[190]=={"kind":"Branch","test":0,"nonzero":192,"zero":191}
    assert f8[191]=={"kind":"Const","dst":0,"value":137,"next":82}
    assert f8[194]=={"kind":"Branch","test":0,"nonzero":127,"zero":191}
    old={tuple(x["goals"]) for x in parent["goal_cases"]+c22["goal_cases"]+c23["goal_cases"]+original_c24["goal_cases"]}
    new={tuple(x["goals"]) for x in manifest["goal_cases"]}
    assert len(new)==45 and not old.intersection(new)
    assert len(manifest["arms"])==180 and len(summary["records"])==45
    assert len(manifest["controls"])==len(summary["controls"])==6
    total=0
    worlds=0
    proofs=[]
    group={}
    totals={"f8_on":0,"f8_off":0,"f6_original":0,"f5_original":0}
    correct={k:0 for k in totals}
    gate={"charged_counter_increments":0,"charged_counter_reads_before_increment":0,
          "charged_attempt_reads_before_first_edit":0,"gate_allow_attempt_2":0,
          "gate_allow_attempt_3":0,"gate_skip_all_other_attempt_counts":0,
          "paid_original_PC82_patchnext_executions":0}
    def check(arm,native):
        nonlocal total,worlds
        source=f8 if arm["source"] in ("F9","F10","F8BASE") else f6 if arm["source"]=="F6BASE" else f5
        path=Path(sys.argv[2])/f"cl1b_c25_original_{arm['name']}.jsonl"
        expected=replay(source,arm,path)
        assert expected==native,(arm["name"],next((k for k in expected if expected[k]!=native.get(k)),None))
        if arm["source"] in ("F9","F10","F8BASE"):
            count=len(expected["services"])
            assert len(expected["paid_counter_reads"])==len(expected["paid_counter_writes"])==count
            assert [e["value"] for e in expected["paid_counter_reads"]]==list(range(count))
            assert [e["value"] for e in expected["paid_counter_writes"]]==list(range(1,count+1))
            assert all(e["attempts"]>=2 for e in expected["paid_attempt_reads"])
            for e in expected["paid_attempt_decisions"]:
                n=e["attempts"]
                assert e["next"]==((191 if n==2 else 192) if e["pc"]==190 else (191 if n==3 else 127))
        else:
            assert (expected["paid_counter_reads"]==expected["paid_counter_writes"]==
                    expected["paid_attempt_reads"]==expected["paid_attempt_decisions"]==[])
        total+=expected["steps"]
        worlds+=1
        return expected
    for idx,g in enumerate(manifest["goal_cases"]):
        a,b,c,d=manifest["arms"][4*idx:4*idx+4]
        n=summary["records"][idx]
        assert a["goals"]==b["goals"]==c["goals"]==d["goals"]==g["goals"]
        assert a["source"]==b["source"]=="F6" and c["source"]=="F6BASE" and d["source"]=="F5"
        assert n["regime"]==g["regime"] and n["goals"]==g["goals"]
        x=check(a,n["f8_on"])
        y=check(b,n["f8_off"])
        z=check(c,n["f6_original"])
        w=check(d,n["f5_original"])
        for label,outcome in (("f8_on",x),("f8_off",y),("f6_original",z),("f5_original",w)):
            assert 0<=outcome["truthful_matches"]<=24 and 0<=outcome["completed_stages"]<=24
            totals[label]+=outcome["steps"]
            correct[label]+=outcome["truthful_matches"]
        proofs.append(first_patchnext_physical_effect(sys.argv[2],x,y))
        gate["charged_counter_increments"]+=len(x["paid_counter_writes"])
        gate["charged_counter_reads_before_increment"]+=len(x["paid_counter_reads"])
        gate["charged_attempt_reads_before_first_edit"]+=len(x["paid_attempt_reads"])
        gate["paid_original_PC82_patchnext_executions"]+=sum(z["pc"]==82 for z in x["edge_edits"])
        attempts=[v["attempts"] for v in x["paid_attempt_reads"]]
        for value in attempts:
            gate["gate_allow_attempt_2"]+=int(value==2)
            gate["gate_allow_attempt_3"]+=int(value==3)
            gate["gate_skip_all_other_attempt_counts"]+=int(value not in (2,3))
        assert n["same_source_bundle_paid_saving"]==y["steps"]-x["steps"]
        assert n["same_source_bundle_probe_saving"]==len(y["services"])-len(x["services"])
        assert n["F8_on_minus_F6_original_paid_CIL"]==x["steps"]-z["steps"]
        assert n["F8_on_minus_F5_original_paid_CIL"]==x["steps"]-w["steps"]
        assert n["cross_source_f6_vs_f5_paid_saving"]==w["steps"]-x["steps"]
        assert n["cross_source_f6_vs_f5_probe_saving"]==len(w["services"])-len(x["services"])
        g_row=group.setdefault(g["regime"],{"cases":0,"same_source_bundle_paid_saving":0,
                "F8_minus_original_F6_paid":0,"F8_minus_old_F5_paid":0})
        g_row["cases"]+=1
        g_row["same_source_bundle_paid_saving"]+=y["steps"]-x["steps"]
        g_row["F8_minus_original_F6_paid"]+=x["steps"]-z["steps"]
        g_row["F8_minus_old_F5_paid"]+=x["steps"]-w["steps"]
    for arm,native in zip(manifest["controls"],summary["controls"]):
        check(arm,native)
    assert worlds==186
    assert len(proofs)==45
    assert sum(g["cases"] for g in group.values())==45
    assert summary["counts"]["worlds"]==186
    assert summary["counts"]["total_original_paid_instructions"]==total
    print(json.dumps({
      "study":study,
      "status":"C24_ORIGINAL_NATIVE_186_FULL_PAID_SERVICE_COUNTER_AND_GATE_INDEPENDENT_REPLAY",
      "original_native_worlds_exactly_replayed":worlds,
      "original_charged_CIL_instruction_count_exactly_replayed":total,
      "original_native_sha":summary["source_sha"],"first_divergence":None,
      "arm_fees":totals,"arm_true_matched_stages":correct,
      "actor_paid_service_count_gate_evidence":gate,
      "same_source_first_true_physical_patchnext_effect":proofs,
      "same_source_paid_savings_sum":totals["f8_off"]-totals["f8_on"],
      "F8_on_minus_original_F6_paid":totals["f8_on"]-totals["f6_original"],
      "F8_on_minus_original_F5_paid":totals["f8_on"]-totals["f5_original"],
      "regimes":group,
      "limitations":"Researcher-authored two/three-attempt cost proxy; PID paid actual service counters and edits. Not a self-discovered ROI rule, B3/B4 NOT_MET."},indent=2))
if __name__=="__main__":
    main()
