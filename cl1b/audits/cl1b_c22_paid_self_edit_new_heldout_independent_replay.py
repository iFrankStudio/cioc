#!/usr/bin/env python3
"""C22 independent original-CIL replay for 141 new task original PID1 worlds.
F6 paid self-modification physical effects ON vs identical F6 source PatchNext OFF.
"""
import json
import os
import sys
from pathlib import Path

ROOT=Path(__file__).resolve().parents[1]
FROZEN=ROOT/"studies/CL1B-C22-PAID-PATCHNEXT-SELF-EDIT-RETURN-frozen-141-worlds-v1.json"
PARENT=ROOT/"studies/CL1B-POLICY-P4F6-frozen-actor-paid-clean-success-refill-542-world-v1.json"
CELLS=(11,20,21,30,32,33,34,35,36,37,38,39)
CREDIT=6000

def replay(source,arm,trace):
    code=[dict(x) for x in source]
    expected_nodes=180 if arm["source"]=="F6" else 153
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
            if at==143:
                assert arm["source"] in ("F6","F5") and address==38
                assert reg[op["dst"]] in (0,1)
                reads38.append({"step":paid,"value":reg[op["dst"]]})
            if at==163:
                assert arm["source"]=="F6" and address==39
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
                assert arm["source"] in ("F6","F5") and address==37
                assert value==cells.get(30,0)
                writes37.append({"step":paid,"pc":at,"value":value})
            if at==152:
                assert arm["source"] in ("F6","F5") and address==38 and value==1
                assert sum(e["pc"]==122 for e in edits)==1+sum(x["value"]==1 for x in writes38)
                writes38.append({"step":paid,"value":value})
            if at==174:
                assert arm["source"]=="F6" and address==38 and value==0
                assert cells.get(39)==0
                writes38.append({"step":paid,"value":value})
            if at in (161,167,172):
                assert arm["source"]=="F6" and address==39
                assert (1<=value<=4) if at==167 else value==0
                writes39.append({"step":paid,"pc":at,"value":value})
            pc=op["next"]
        elif kind=="Branch":
            pc=op["nonzero"] if reg[op["test"]] else op["zero"]
            if at==146:
                assert arm["source"] in ("F6","F5")
                meter=cells.get(38,0)
                assert pc==(140 if meter==1 else 138)
                budget_gates.append({"step":paid,"meter":meter,"next":pc})
            if at in (158,170):
                assert arm["source"]=="F6"
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
                assert arm["source"] in ("F6","F5") and code[34]["next"]==106
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
              ("F6",82):(12,137),("F5",82):(12,137),
              ("F6",131):(34,132),("F5",131):(34,132),
              ("F6",136):(34,132),("F5",136):(34,132),
              ("F6",139):(34,106),("F5",139):(34,106),
              ("F6",155):(133,156),("F6",177):(133,62),
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
    if arm["source"]=="F6":
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
      "online_edit_budget_meter":cells.get(38,0),"final_edges":edges_now(),
      "peak_occupied":peak,"original_pid_halted":True,"host_source_edits":0}


def first_patchnext_effect_proof(folder, on_arm, off_arm):
    """Native full original-CIL event prefix matches until first actual
    suppressed PatchNext physical edge difference. This is NOT single-PC ablation.
    """
    def rows(arm):
        path=Path(folder)/f"cl1b_c22_original_{arm['name']}.jsonl"
        return [json.loads(line) for line in path.read_text().splitlines() if line.strip()]
    assert on_arm["source"]==off_arm["source"]=="F6"
    assert not on_arm["patchnext_suppressed"] and off_arm["patchnext_suppressed"]
    assert on_arm["goals"]==off_arm["goals"] and on_arm["feedback"]==off_arm["feedback"]=="truth"
    aa,bb=rows(on_arm),rows(off_arm)
    first=next((i for i,(a,b) in enumerate(zip(aa,bb)) if a!=b),None)
    if first is None:
        assert len(aa)==len(bb)
        return {"case":on_arm["name"],"first_effect_exposed":False,"all_native_events_identical":True}
    a,b=aa[first],bb[first]
    assert a[0]==b[0]=="cil",(on_arm["name"],"first delta not paid CIL",first,a,b)
    assert a[2]==b[2] and a[2] in (82,131,136,139,155,177),(on_arm["name"],"not PatchNext at first effect",a,b)
    assert a[:8]==b[:8],(on_arm["name"],"other physical state before first edit",first,a,b)
    assert len(a[8])==len(b[8])==4
    different=[i for i in range(4) if a[8][i]!=b[8][i]]
    assert len(different)==1,(on_arm["name"],"first effect not exactly single live executable edge",different,a[8],b[8])
    return {"case":on_arm["name"],"first_effect_exposed":True,
            "first_divergent_paid_event_offset":first,"first_effect_pc":a[2],
            "changed_live_edge_slot":different[0],
            "on_edge_value":a[8][different[0]],"suppressed_edge_value":b[8][different[0]],
            "identical_all_original_native_paid_and_service_prefix":True}

def main():
    if len(sys.argv)!=3:
        raise SystemExit("usage: script SUMMARY.json ORIGINAL_TRACE_DIR")
    summary=json.loads(Path(sys.argv[1]).read_text())
    manifest=json.loads(FROZEN.read_text())
    assert summary["study"]==manifest["study"]=="CL1B-C22-PAID-PATCHNEXT-SELF-EDIT-RETURN-ON-COMPUTE-NEW-HELDOUT"
    parent=json.loads(PARENT.read_text())
    assert parent["study"]=="CL1B-POLICY-P4F6"
    assert manifest["status"]=="PRE_NATIVE_FROZEN_NEW_24_STAGE_GOALS"
    assert summary["level"]=="ORIGINAL_NATIVE_141_WORLD_P4F6_REUSED_SOURCE_NEW_GOAL_PATCHNEXT_EFFECT_ROI"
    assert summary["source_sha"] not in ("","local")
    if os.environ.get("GITHUB_SHA"):
        assert summary["source_sha"]==os.environ["GITHUB_SHA"]
    assert len(manifest["arms"])==135 and len(summary["records"])==45 and len(manifest["controls"])==len(summary["controls"])==6
    old={tuple(x["goals"]) for x in parent["goal_cases"]}
    fresh={tuple(x["goals"]) for x in manifest["goal_cases"]}
    assert len(fresh)==45 and not (fresh & old) and len(old)==168
    total=0
    worlds=0
    groups={}
    def check(arm,native):
        nonlocal total,worlds
        source=parent["original_native_CIL_source_f6"] if arm["source"]=="F6" else parent["baseline_source_f5"]
        path=Path(sys.argv[2])/f"cl1b_c22_original_{arm['name']}.jsonl"
        independently=replay(source,arm,path)
        assert independently==native,(arm["name"],next((k for k in independently if independently[k]!=native.get(k)),None))
        total+=independently["steps"]
        worlds+=1
        return independently
    first_divergence_proofs=[]
    roi={"wins":0,"ties":0,"losses":0,"total_same_source_paid_saving":0,"total_cross_source_F5_minus_F6":0,"on_true_stages":0,"off_true_stages":0,"F5_true_stages":0,"actor_real_patchnext_edits":0,"actor_real_patchconst_edits":0}
    for i,g in enumerate(manifest["goal_cases"]):
        a,b,c=manifest["arms"][3*i:3*i+3]
        record=summary["records"][i]
        assert a["goals"]==b["goals"]==c["goals"]==g["goals"]
        assert record["regime"]==g["regime"] and record["goals"]==g["goals"]
        on=check(a,record["f6_on"])
        off=check(b,record["f6_off"])
        base=check(c,record["f5_on"])
        assert all(0<=x["truthful_matches"]<=24 and 0<=x["completed_stages"]<=24 for x in (on,off,base))
        first_divergence_proofs.append(first_patchnext_effect_proof(sys.argv[2],a,b))
        roi["on_true_stages"]+=on["truthful_matches"]
        roi["off_true_stages"]+=off["truthful_matches"]
        roi["F5_true_stages"]+=base["truthful_matches"]
        roi["actor_real_patchnext_edits"]+=sum(not e["suppressed"] for e in on["edge_edits"])
        roi["actor_real_patchconst_edits"]+=len(on["code_edits"])
        d=off["steps"]-on["steps"]
        roi["total_same_source_paid_saving"]+=d
        roi["wins" if d>0 else "losses" if d<0 else "ties"]+=1
        p=len(off["services"])-len(on["services"])
        x=base["steps"]-on["steps"]
        roi["total_cross_source_F5_minus_F6"]+=x
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
    assert worlds==141
    assert total==summary["counts"]["total_original_paid_instructions"]
    assert summary["counts"]["worlds"]==141
    assert roi["wins"]+roi["ties"]+roi["losses"]==45
    assert len(first_divergence_proofs)==45
    print(json.dumps({"study":"CL1B-C22-PAID-PATCHNEXT-SELF-EDIT-RETURN-ON-COMPUTE-NEW-HELDOUT",
      "original_native_worlds_exactly_replayed":worlds,
      "new_goal_cases":45,"economic_observations":roi,
      "strict_same_source_first_physical_PatchNext_divergences":first_divergence_proofs,
      "original_charged_CIL_instruction_count_exactly_replayed":total,
      "first_divergence":None,"source_sha":summary["source_sha"],
      "regimes":groups,"limits":"Original F6 researcher-authored finite policy reused. Same-source OFF suppresses ALL PatchNext physical effects but charges CIL; not self-invented B3 or life."},indent=2))

if __name__=="__main__":
    main()
