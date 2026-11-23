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
    if len(sys.argv)!=3:
        raise SystemExit("usage: C25_SOURCE_PINNED_AUDIT ORIGINAL_NATIVE_SUMMARY.json ORIGINAL_JSONL_DIR")
    summary=json.loads(Path(sys.argv[1]).read_text())
    m=json.loads(FROZEN.read_text())
    old=json.loads(OLD_C24.read_text())
    p=json.loads(PARENT.read_text())
    c23=json.loads(C23.read_text())
    c22=json.loads(C22.read_text())
    c24b=json.loads(ROOT.joinpath("studies/CL1B-C24B-F8-PAID-SERVICE-COUNT-HELDOUT-REPLICATION-frozen-186-worlds-v1.json").read_text())
    study="CL1B-C25-F9F10-ACTOR-PAID-PATCHNEXT-SELF-QUARANTINE"
    assert summary["study"]==m["study"]==study
    assert summary["level"]=="ORIGINAL_NATIVE_C25_276_WORLD_ACTOR_PAID_SELF_QUARANTINE"
    assert m["status"]=="PRE_NATIVE_FROZEN_SIX_ARM_276_NATIVE_WORLDS_NEW45_GOALS"
    assert summary["source_sha"] not in ("","local")
    if os.environ.get("GITHUB_SHA"):assert summary["source_sha"]==os.environ["GITHUB_SHA"]
    f8=old["exact_frozen_F8_source_195_nodes"]
    f9=m["actor_paid_source_F9_selective_210_nodes"]
    f10=m["actor_paid_source_F10_full_210_nodes"]
    f5=p["baseline_source_f5"]
    assert len(f8)==195 and len(f9)==len(f10)==210 and len(f5)==153
    assert f9[194]==dict(f8[194],nonzero=195)
    assert f10[190]==dict(f8[190],zero=195)
    assert f10[194]==dict(f8[194],nonzero=195,zero=195)
    assert all(f9[i]==f8[i] for i in range(195) if i!=194)
    assert all(f10[i]==f8[i] for i in range(195) if i not in (190,194))
    assert f9[195:]==f10[195:]
    for i,(node,next_pc) in enumerate([(128,46),(134,123),(138,140),(153,134),(175,178)]):
        pc=195+i*3
        assert f9[pc:pc+3]==[
          {"kind":"Const","dst":1,"value":node,"next":pc+1},
          {"kind":"Const","dst":0,"value":next_pc,"next":pc+2},
          {"kind":"PatchNext","node":1,"target":0,"next":127 if i==4 else pc+3}
        ]
    oldgoals={tuple(v["goals"]) for x in [p,c22,c23,old,c24b] for v in x["goal_cases"]}
    newgoals={tuple(v["goals"]) for v in m["goal_cases"]}
    assert len(newgoals)==45 and not oldgoals.intersection(newgoals)
    assert len(m["arms"])==270 and len(summary["records"])==45
    assert len(m["controls"])==len(summary["controls"])==6
    sources={"F9":f9,"F10":f10,"F8BASE":f8,"F5":f5}
    labels=["f9_on","f9_off","f10_on","f10_off","f8_original","f5_original"]
    types=["F9","F9","F10","F10","F8BASE","F5"]
    total=0
    worlds=0
    fees={k:0 for k in labels}
    stage={k:0 for k in labels}
    negative_fees=0
    paired_effects={"f9":[],"f10":[]}
    quarantine={"f9_on":0,"f9_off":0,"f10_on":0,"f10_off":0,
                "f9_selected_execution_cases":0,"f10_all_execution_cases":0}
    groups={}
    per_case=[]
    def check(arm,native):
        nonlocal total,worlds
        source=sources[arm["source"]]
        trace=Path(sys.argv[2])/f"cl1b_c25_original_{arm['name']}.jsonl"
        independently=replay(source,arm,trace)
        assert independently==native,(arm["name"],next((k for k in independently if independently[k]!=native.get(k)),None))
        if arm["source"] in ("F9","F10","F8BASE"):
            count=len(independently["services"])
            assert len(independently["paid_counter_reads"])==len(independently["paid_counter_writes"])==count
            assert [x["value"] for x in independently["paid_counter_reads"]]==list(range(count))
            assert [x["value"] for x in independently["paid_counter_writes"]]==list(range(1,count+1))
            assert len(independently["paid_attempt_reads"])==1
        else:
            assert independently["paid_counter_reads"]==independently["paid_counter_writes"]==[]
            assert independently["paid_attempt_reads"]==independently["paid_attempt_decisions"]==[]
        if arm["source"] in ("F9","F10"):
            attempts=independently["paid_attempt_reads"][0]["attempts"]
            assert attempts>=2
            expected=5 if arm["source"]=="F10" or attempts>=4 else 0
            assert len(independently["paid_quarantine_self_edits"])==expected
            assert [x["pc"] for x in independently["paid_quarantine_self_edits"]]==([197,200,203,206,209] if expected else [])
            assert all(x["suppressed"]==arm["patchnext_suppressed"] for x in independently["paid_quarantine_self_edits"])
            assert all((x["after"]==x["before"])==bool(arm["patchnext_suppressed"]) for x in independently["paid_quarantine_self_edits"])
        else:
            assert independently["paid_quarantine_self_edits"]==[]
        total+=independently["steps"]
        worlds+=1
        return independently
    for i,g in enumerate(m["goal_cases"]):
        arms=m["arms"][6*i:6*i+6]
        native=summary["records"][i]
        assert native["goals"]==g["goals"] and native["regime"]==g["regime"]
        assert len(arms)==6
        outputs=[]
        for j,arm in enumerate(arms):
            assert arm["source"]==types[j]
            assert arm["feedback"]=="truth" and arm["goals"]==g["goals"]
            assert arm["patchnext_suppressed"]==(j in (1,3))
            x=check(arm,native[labels[j]])
            assert 0<=x["truthful_matches"]<=24 and 0<=x["completed_stages"]<=24
            outputs.append(x)
            fees[labels[j]]+=x["steps"]
            stage[labels[j]]+=x["truthful_matches"]
        x0,x1,x2,x3,x4,x5=outputs
        a=first_patchnext_physical_effect(sys.argv[2],x0,x1)
        b=first_patchnext_physical_effect(sys.argv[2],x2,x3)
        paired_effects["f9"].append(a)
        paired_effects["f10"].append(b)
        for label,x in (("f9_on",x0),("f9_off",x1),("f10_on",x2),("f10_off",x3)):
            quarantine[label]+=len(x["paid_quarantine_self_edits"])
        quarantine["f9_selected_execution_cases"]+=int(len(x0["paid_quarantine_self_edits"])==5)
        quarantine["f10_all_execution_cases"]+=int(len(x2["paid_quarantine_self_edits"])==5)
        assert native["F9_same_source_physical_edit_net_paid_saving"]==x1["steps"]-x0["steps"]
        assert native["F10_same_source_physical_edit_net_paid_saving"]==x3["steps"]-x2["steps"]
        assert native["F9_vs_old_F8_full_program_paid_saving"]==x4["steps"]-x0["steps"]
        assert native["F10_vs_old_F8_full_program_paid_saving"]==x4["steps"]-x2["steps"]
        assert native["F10_vs_old_F5_full_program_paid_saving"]==x5["steps"]-x2["steps"]
        assert native["F9_original_real_paid_quarantine_edits"]==len(x0["paid_quarantine_self_edits"])
        assert native["F10_original_real_paid_quarantine_edits"]==len(x2["paid_quarantine_self_edits"])
        row=groups.setdefault(g["regime"],{"cases":0,"f9_effect_paid_saving":0,"f10_effect_paid_saving":0,"f9_old_f8_paid_saving":0,
                         "f10_old_f8_paid_saving":0,"f10_old_f5_paid_saving":0})
        row["cases"]+=1
        row["f9_effect_paid_saving"]+=x1["steps"]-x0["steps"]
        row["f10_effect_paid_saving"]+=x3["steps"]-x2["steps"]
        row["f9_old_f8_paid_saving"]+=x4["steps"]-x0["steps"]
        row["f10_old_f8_paid_saving"]+=x4["steps"]-x2["steps"]
        row["f10_old_f5_paid_saving"]+=x5["steps"]-x2["steps"]
        per_case.append({"name":x0["name"],"goals":g["goals"],"regime":g["regime"],
             "f9_on_paid":x0["steps"],"f9_off_paid":x1["steps"],
             "f10_on_paid":x2["steps"],"f10_off_paid":x3["steps"],
             "f8_original_paid":x4["steps"],"f5_original_paid":x5["steps"],
             "f9_true_stages":x0["truthful_matches"],"f10_true_stages":x2["truthful_matches"],
             "f9_same_source_edit_saving":x1["steps"]-x0["steps"],
             "f10_same_source_edit_saving":x3["steps"]-x2["steps"],
             "first_paid_service_attempts":x0["paid_attempt_reads"][0]["attempts"],
             "f9_original_paid_quarantine_edits":len(x0["paid_quarantine_self_edits"]),
             "f10_original_paid_quarantine_edits":len(x2["paid_quarantine_self_edits"])})
    for arm,native in zip(m["controls"],summary["controls"]):
        x=check(arm,native)
        negative_fees+=x["steps"]
    assert len(per_case)==45 and worlds==276
    assert summary["counts"]["worlds"]==276 and summary["counts"]["truthful_worlds"]==270
    assert summary["counts"]["total_original_paid_instructions"]==total
    assert sum(fees.values())+negative_fees==total
    assert all(len(paired_effects[k])==45 for k in paired_effects)
    print(json.dumps({
      "study":study,"level":"C25_276_NATIVE_WORLDS_INDEPENDENT_EXACT_PAID_CIL_REPLAY",
      "tested_source_sha":summary["source_sha"],"first_divergence":None,
      "original_worlds_exactly_replayed":276,"true_new_task_schedules":45,
      "total_original_paid_CIL_including_negatives":total,
      "negative_control_paid_CIL":negative_fees,
      "arm_paid_CIL":fees,"arm_true_matched_stages":stage,
      "paid_PID1_quarantine_witness":quarantine,
      "F9_same_source_full_paid_physical_edit_net_saving":fees["f9_off"]-fees["f9_on"],
      "F10_same_source_full_paid_physical_edit_net_saving":fees["f10_off"]-fees["f10_on"],
      "F9_on_vs_original_F8_paid_saving":fees["f8_original"]-fees["f9_on"],
      "F10_on_vs_original_F8_paid_saving":fees["f8_original"]-fees["f10_on"],
      "F10_on_vs_original_F5_paid_saving":fees["f5_original"]-fees["f10_on"],
      "F9_same_source_first_physical_edit_effect":paired_effects["f9"],
      "F10_same_source_first_physical_edit_effect":paired_effects["f10"],
      "groups":groups,"per_case":per_case,
      "limits":"Researcher-authored native paid self-quarantine program. Same-source tests bundle all PatchNext physical effects including old editing; no individual-PatchNext isolated attribution. No learned ROI policy or B3/B4."},indent=2))
if __name__=="__main__":
    main()
