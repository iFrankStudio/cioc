#!/usr/bin/env python3
"""CL1B-REFILL-CAUSAL-R1 independent paid-original-CIL interpreter; 
environment feedback in 472 original PID1 native worlds, not Rust-derived truth.
"""
import json
import os
import sys
from pathlib import Path

ROOT=Path(__file__).resolve().parents[1]
FROZEN=ROOT/"studies/CL1B-REFILL-CAUSAL-R1-frozen-source-identical-pc174-ablation-472-worlds.json"
CELLS=(11,20,21,30,32,33,34,35,36,37,38,39)
CREDIT=6000

def replay(source,arm,trace):
    code=[dict(x) for x in source]
    expected_nodes=180 if arm["source"]=="F6" else 153
    assert len(code)==expected_nodes
    goals=arm["goals"]
    mode=arm["feedback"]
    suppression=arm["suppression"]
    assert suppression in ("none","refill174","placebo_unreachable180")
    assert arm["source"]=="F6" or suppression=="none"
    disabled=False
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
    suppressed_refill=[]
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
            intercepted=(at==174 and suppression=="refill174")
            if intercepted:
                assert arm["source"]=="F6" and address==38 and value==0
                suppressed_refill.append({"step":paid,"pc":174,"key":38,"value":0})
            else:
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
                if not intercepted:writes38.append({"step":paid,"value":value})
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
            [cells.get(k) for k in CELLS],occupied,edges_now(),
            "suppressed_refill" if kind=="Write" and at==174 and suppression=="refill174" else "normal"])
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
        assert sum(e["pc"]==177 for e in edges)==downs+len(suppressed_refill)
        assert (ups<=1 and downs==0) if suppression=="refill174" else not suppressed_refill
    else:
        assert online==len(writes38)<=1
        assert cells.get(38,0)==len(writes38)
        assert not reads39 and not writes39 and not clean_streak_gates
        assert not suppressed_refill
    return {"name":arm["name"],"source":arm["source"],"regime":arm["regime"],
      "goals":goals,"feedback":mode,"suppression":suppression,"steps":paid,
      "services":services,"read20_count":observed,
      "truthful_matches":sum(x["truth"] for x in services),
      "completed_stages":cells.get(32),"claim":cells.get(21),
      "code_edits":edits,"edge_edits":edges,
      "write37":writes37,"read37":reads37,
      "write38":writes38,"read38":reads38,"budget_gates":budget_gates,
      "write39":writes39,"read39":reads39,"clean_streak_gates":clean_streak_gates,
      "suppressed_refill":suppressed_refill,
      "online_edit_budget_meter":cells.get(38,0),"final_edges":edges_now(),
      "peak_occupied":peak,"original_pid_halted":True,"host_source_edits":0}


def original_trace_path(folder,arm):
    return Path(folder)/f"cl1b_refill_causal_r1_original_{arm['name']}.jsonl"

def prefix_and_placebo(folder,a,b,p):
    """First actual targeted refill is the earliest allowable paired divergence."""
    import itertools
    exposed=False
    with original_trace_path(folder,a).open() as normal,original_trace_path(folder,b).open() as ablated:
        for index,(x,y) in enumerate(itertools.zip_longest(normal,ablated)):
            if x is None or y is None:
                assert exposed
                break
            n=json.loads(x);s=json.loads(y)
            if s[0]=="cil" and s[-1]=="suppressed_refill":
                assert n[0]=="cil" and n[2]==s[2]==174
                assert n[1]==s[1] and n[3:6]==s[3:6] and n[7:9]==s[7:9]
                # At the first paid intervention, only the actual cell38 value
                # and provenance flag differ. It does not mint compute.
                assert n[6][10]==0 and s[6][10]==1
                assert all(n[6][i]==s[6][i] for i in range(len(n[6])) if i!=10)
                exposed=True
                break
            assert n==s,{"problem":"first divergence before paid PC174 suppression",
                         "arm":a["name"],"row":index,"normal":n,"ablated":s}
    with original_trace_path(folder,a).open() as normal,original_trace_path(folder,p).open() as placebo:
        for index,(x,y) in enumerate(itertools.zip_longest(normal,placebo)):
            assert x==y,{"problem":"unreachable PC180 placebo changed original trace",
                         "arm":a["name"],"row":index}
    return exposed

def main():
    if len(sys.argv)!=3:
        raise SystemExit("usage: interpreter SUMMARY.json ORIGINAL_TRACE_DIR")
    summary=json.loads(Path(sys.argv[1]).read_text())
    m=json.loads(FROZEN.read_text())
    assert summary["study"]==m["study"]=="CL1B-REFILL-CAUSAL-R1"
    assert summary["level"]=="ORIGINAL_NATIVE_472_WORLD_SOURCE_IDENTICAL_PC174_PAID_WRITE_SUPPRESSION"
    assert summary["source_sha"] not in ("","local")
    if os.environ.get("GITHUB_SHA"):
        assert summary["source_sha"]==os.environ["GITHUB_SHA"]
    assert len(m["fresh_cases"])==45 and len(m["exploratory_prior_cases"])==144
    assert len(m["arms_new"])==180 and len(m["arms_old"])==288 and len(m["controls"])==4
    assert len(summary["fresh_records"])==45 and len(summary["exploratory_records"])==144
    total=worlds=exposures=fresh_exposures=fresh_paid=old_paid=first_exposures=0
    groups={}
    def check(arm,native):
        nonlocal total,worlds
        source=m["original_loaded_CIL_source_f6"] if arm["source"]=="F6" else m["baseline_loaded_CIL_source_f5"]
        independent=replay(source,arm,original_trace_path(sys.argv[2],arm))
        assert independent==native,(arm["name"],next((k for k in independent if independent[k]!=native.get(k)),None))
        total+=independent["steps"];worlds+=1
        return independent
    for i,g in enumerate(m["fresh_cases"]):
        a,b,p,c=m["arms_new"][4*i:4*i+4]
        record=summary["fresh_records"][i]
        assert all(x["goals"]==g["goals"] for x in (a,b,p,c))
        on=check(a,record["f6_on"])
        off=check(b,record["f6_selective_refill_disabled"])
        placebo=check(p,record["f6_unreachable_placebo"])
        base=check(c,record["f5_distinct_source"])
        assert all(x["truthful_matches"]==x["completed_stages"]==24 and x["claim"]==1
                   for x in (on,off,placebo,base))
        assert prefix_and_placebo(sys.argv[2],a,b,p)==bool(off["suppressed_refill"])
        first_exposures+=int(bool(off["suppressed_refill"]))
        cost=off["steps"]-on["steps"]
        probes=len(off["services"])-len(on["services"])
        source_cost=base["steps"]-on["steps"]
        assert record["same_source_refill_causal_paid_saving"]==cost
        assert record["same_source_refill_causal_probe_saving"]==probes
        assert record["distinct_source_f5_minus_f6_paid_saving"]==source_cost
        assert record["actual_suppressed_pc174_exposures"]==len(off["suppressed_refill"])
        assert on["suppressed_refill"]==placebo["suppressed_refill"]==[]
        n=len(off["suppressed_refill"])
        exposures+=n;fresh_exposures+=n;fresh_paid+=cost
        group=groups.setdefault(g["regime"],{"cases":0,"causal_paid_saving":0,
          "causal_probe_saving":0,"f5_distinct_source_saving":0,
          "suppressions":0,"wins":0,"no_exposure":0})
        group["cases"]+=1;group["causal_paid_saving"]+=cost
        group["causal_probe_saving"]+=probes
        group["f5_distinct_source_saving"]+=source_cost
        group["suppressions"]+=n;group["wins"]+=int(cost>0)
        group["no_exposure"]+=int(n==0)
        assert group["cases"]<=9
    for i,g in enumerate(m["exploratory_prior_cases"]):
        a,b=m["arms_old"][2*i:2*i+2]
        record=summary["exploratory_records"][i]
        on=check(a,record["f6_on"])
        off=check(b,record["f6_selective_refill_disabled"])
        assert on["truthful_matches"]==off["truthful_matches"]==24
        assert on["completed_stages"]==off["completed_stages"]==24
        cost=off["steps"]-on["steps"]
        assert cost==record["same_source_refill_causal_paid_saving"]
        assert len(off["services"])-len(on["services"])==record["same_source_refill_causal_probe_saving"]
        n=len(off["suppressed_refill"])
        assert n==record["actual_suppressed_pc174_exposures"]
        exposures+=n;old_paid+=cost
    for arm,native in zip(m["controls"],summary["controls"]):
        check(arm,native)
    assert worlds==summary["counts"]["worlds"]==472
    assert total==summary["counts"]["total_original_paid_instructions"]
    assert exposures==summary["counts"]["actual_suppressed_pc174_exposures"]
    assert fresh_exposures==summary["counts"]["fresh_suppressed_exposures"]
    for key,g in groups.items():
        reported=summary["counts"]["fresh_regimes"][key]
        assert g["cases"]==reported["cases"]
        assert g["causal_paid_saving"]==reported["same_source_refill_causal_paid_saving"]
        assert g["causal_probe_saving"]==reported["same_source_refill_causal_probe_saving"]
        assert g["f5_distinct_source_saving"]==reported["distinct_source_f5_minus_f6_paid_saving"]
        assert g["suppressions"]==reported["actual_suppressed_pc174_exposures"]
        assert g["wins"]==reported["original_refill_win_cases"]
    print(json.dumps({"study":"CL1B-REFILL-CAUSAL-R1",
        "original_native_worlds_exactly_replayed":worlds,
        "original_charged_CIL_instruction_count_exactly_replayed":total,
        "first_divergence":None,"source_sha":summary["source_sha"],
        "fresh_cases":45,"previously_exposed_cases":144,
        "fresh_same_source_paid_saving":fresh_paid,
        "old_exploratory_same_source_paid_saving":old_paid,
        "actual_suppressed_pc174_exposures":exposures,
        "fresh_exposures":fresh_exposures,
        "fresh_pairs_with_actual_causal_exposure":first_exposures,
        "fresh_regimes":groups,
        "nonclaims":"Exact same-CIL-source researcher-only PC174 intervention, not actor invention"},indent=2))

if __name__=="__main__":
    main()
