#!/usr/bin/env python3
"""Independent per-instruction C4 Rust-native actor-paid PC67 gate and PC28 exact-effect replay."""
import json
import sys
from pathlib import Path

ROOT=Path(__file__).resolve().parents[1]
MANIFEST=ROOT/"studies/CL1B-G2-ACTOR-PAID-GATE-C4-frozen-274-worlds-v1.json"
SOURCES={
    "G2_actor_gate":"frozen_source_g2_actor_gate",
    "G2_gate_always_edit":"frozen_source_g2_gate_always_edit",
    "G2_gate_always_abstain":"frozen_source_g2_gate_always_abstain",
    "G2_gate_pc28_suppressed":"frozen_source_g2_actor_gate",
    "G2_gate_placebo180":"frozen_source_g2_actor_gate",
    "G2_source_never":"frozen_source_g2_never"
}
CELLS=(11,20,21,30,32,33,34,35,36,37,38,39)
CREDIT=6000

def replay(m,arm,folder):
    code=[dict(x) for x in m[SOURCES[arm["source"]]]]
    assert len(code)==180
    goals=arm["goals"]
    assert len(goals)==48 and all(x in (0,1,2) for x in goals)
    mode=arm["feedback"]
    assert mode in ("truth","inverted","forced0")
    regs=[0,0,0,0]
    memory={}
    pc=0
    paid=0
    peak=181
    services=[]
    paid_gate_records=[]
    gate_allow=gate_abstain=0
    decisions=[]
    refills=[]
    patches=[]
    edits=[]
    later_reads=[]
    last_changed=None
    failed_halt=False
    suppressed_pc28=[]
    path=Path(folder)/("cl1b_g2_actor_gate_c4_original_"+arm["name"]+".jsonl")
    with path.open(encoding="utf-8") as stream:
        lines=(json.loads(line) for line in stream if line.strip())
        def edges():
            return [code[12]["next"],code[34]["next"],code[46]["value"],code[133]["next"]]
        def theta():
            n=code[173]
            assert n["kind"]=="Const" and n["dst"]==2 and n["next"]==174
            return n["value"]
        def check(wanted):
            actual=next(lines,None)
            if actual!=wanted:
                raise AssertionError(json.dumps({"name":arm["name"],"paid":paid,
                    "actual_native":actual,"independent":wanted},ensure_ascii=False)[:5000])
        while paid<CREDIT:
            at=pc
            op=code[at]
            kind=op["kind"]
            pre_value=code[regs[op["node"]]]["value"] if kind=="PatchConst" else None
            pre_next=code[regs[op["node"]]]["next"] if kind=="PatchNext" else None
            paid+=1
            if kind=="Const":
                regs[op["dst"]]=op["value"]
                pc=op["next"]
            elif kind in ("Add","Sub"):
                a,b=regs[op["lhs"]],regs[op["rhs"]]
                regs[op["dst"]]=a+b if kind=="Add" else a-b
                pc=op["next"]
            elif kind=="Read":
                regs[op["dst"]]=memory.get(regs[op["address"]],0)
                pc=op["next"]
            elif kind=="Write":
                memory[regs[op["address"]]]=regs[op["value"]]
                pc=op["next"]
            elif kind=="Branch":
                pc=op["nonzero"] if regs[op["test"]] else op["zero"]
            elif kind=="PatchConst":
                target=regs[op["node"]]
                assert 0<=target<180 and code[target]["kind"]=="Const"
                if not (at==28 and arm["source"]=="G2_gate_pc28_suppressed"):
                    code[target]["value"]=regs[op["value"]]
                pc=op["next"]
            elif kind=="PatchNext":
                target=regs[op["node"]]
                destination=regs[op["target"]]
                assert 0<=target<180 and 0<=destination<180
                assert code[target]["kind"]=="Write"
                code[target]["next"]=destination
                pc=op["next"]
            elif kind=="Halt":
                pc=at
            else:
                raise AssertionError(("unsupported original CIL opcode",kind,at))
            if at in (45,88,102,122,28):
                target=regs[1]
                value=regs[3] if at==28 else regs[0]
                assert target==(173 if at==28 else 46)
                suppressed=at==28 and arm["source"]=="G2_gate_pc28_suppressed"
                assert code[target]["value"]==(pre_value if suppressed else value)
                if at==28:
                    assert value in (0,1,2)
                    patches.append({"step":paid,"pc":28,"target":173,
                        "before":pre_value,"attempted":value,
                        "after":pre_value if suppressed else value,
                        "changed":not suppressed and pre_value!=value,
                        "suppressed":suppressed})
                    if suppressed:
                        suppressed_pc28.append({"step":paid,"pc":28,"target":173,
                            "attempted":value,"before":pre_value,"after":pre_value})
                    elif pre_value!=value:
                        last_changed=value
                edits.append({"step":paid,"pc":at,"target":target,
                    "before":pre_value,"attempted":value,
                    "after":pre_value if suppressed else value,
                    "kind":"PatchConst","suppressed":suppressed})
            if at in (82,131,136,139,155,26):
                target=regs[1]
                value=regs[0]
                assert target in (12,34,133)
                if at==26:assert target==133 and value==62
                edits.append({"step":paid,"pc":at,"target":target,
                    "before":pre_next,"after":value,"kind":"PatchNext"})
            if at==67 and arm["source"]!="G2_source_never":
                assert kind=="Branch"
                cond=regs[3]!=0
                expected_route={
                    "G2_actor_gate":27 if cond else 125,
                    "G2_gate_always_edit":27,
                    "G2_gate_always_abstain":125,
                    "G2_gate_pc28_suppressed":27 if cond else 125,
                    "G2_gate_placebo180":27 if cond else 125
                }[arm["source"]]
                assert pc==expected_route
                if pc==27:gate_allow+=1
                else:gate_abstain+=1
                paid_gate_records.append({"step":paid,"r3":regs[3],"route":pc,
                    "decision":"EDIT" if pc==27 else "ABSTAIN"})
            if at==173:
                assert regs[2]==theta()
                if last_changed is not None:
                    assert last_changed==regs[2]
                    later_reads.append({"step":paid,"theta":last_changed})
                    last_changed=None
            if at==175:
                delta=regs[0]
                expected={
                    "G2_actor_gate":176 if delta else 179,
                    "G2_gate_always_edit":176 if delta else 179,
                    "G2_gate_always_abstain":176 if delta else 179,
                    "G2_gate_pc28_suppressed":176 if delta else 179,
                    "G2_gate_placebo180":176 if delta else 179,
                    "G2_source_never":179,
                }[arm["source"]]
                assert pc==expected
                decisions.append({"step":paid,"difference":delta,"next":pc,
                    "theta":theta(),"last_guess":regs[3]})
            if at==178:
                assert memory[38]==0
                refills.append({"step":paid,"pc":178})
            if at==15 and pc==61:
                assert regs[0]==0
                failed_halt=True
            occupied=181+len(memory)
            assert len(memory)<=16 and occupied<=300 and 0<=pc<180
            peak=max(peak,occupied)
            check(["cil",paid,at,pc,list(regs),CREDIT-paid,
                [memory.get(k) for k in CELLS],occupied,edges(),theta(),
                "suppressed_pc28" if at==28 and arm["source"]=="G2_gate_pc28_suppressed"
                else "paid_parameter_patch" if at==28 else "normal"])
            if at==3:
                assert kind=="Write" and memory[11]==regs[op["value"]]
                stage=memory.get(32,0)
                assert 0<=stage<48
                guess=memory[11]
                truth=int(guess==goals[stage])
                feedback=truth if mode=="truth" else 1-truth if mode=="inverted" else 0
                services.append({"step":paid,"stage":stage,"guess":guess,
                    "truth":truth,"feedback":feedback})
                memory[20]=feedback
                occupied=181+len(memory)
                peak=max(peak,occupied)
                check(["feedback",paid,stage,goals[stage],guess,truth,feedback,
                    [memory.get(k) for k in CELLS],occupied,edges(),theta()])
            if kind=="Halt":break
        else:
            raise AssertionError(("CIL private credits exhausted",arm["name"]))
        assert next(lines,None) is None,"extra original-native trace events"
    true_count=sum(x["truth"] for x in services)
    completed=memory.get(32,0)
    claim=memory.get(21,0)
    if mode=="truth":assert (completed,true_count,claim)==(48,48,1)
    if mode=="forced0":assert failed_halt and completed==0 and claim==0
    if mode=="inverted":assert claim==1 and true_count<len(services)
    return {"name":arm["name"],"source":arm["source"],"regime":arm["regime"],
        "feedback":mode,"goals":goals,"steps":paid,"services":services,
        "decisions":decisions,"refills":refills,"parameter_patches":patches,
        "paid_code_edits":edits,"suppressed_pc28":suppressed_pc28,
        "later_mutated_reads":later_reads,
        "completed_stages":completed,"truthful_matches":true_count,
        "claim":claim,"final_theta":theta(),"final_edges":edges(),
        "peak_occupied":peak,"failed_third_probe_halt":failed_halt,
        "host_source_edits":0,"budget_exhausted":False,
        "paid_gate_allow":gate_allow,"paid_gate_abstain":gate_abstain,
        "paid_gate_records":paid_gate_records}

def main():
    if len(sys.argv)!=3:
        raise SystemExit("usage: cl1b_g2_actor_gate_c4_independent_native_replay.py summary.json raw_trace_dir")
    m=json.loads(MANIFEST.read_text(encoding="utf-8"))
    native=json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
    assert m["study"]==native["study"]=="CL1B-G2-ACTOR-PAID-GATE-C4"
    assert m["status"]=="PRE_NATIVE_FROZEN"
    assert native["status"]=="ORIGINAL_NATIVE_ACTOR_PAID_GATED_PC28"
    assert native["total_original_native_worlds"]==274
    cases=m["arms"]+m["controls"]
    expected=native["truthful_records"]+native["adversarial_records"]
    assert len(cases)==len(expected)==274
    paid_total=0
    costs={}
    totals={name:0 for name in SOURCES}
    for arm,from_native in zip(cases,expected):
        assert arm["name"]==from_native["name"]
        reconstructed=replay(m,arm,sys.argv[2])
        assert reconstructed==from_native,("native summary differs from independent original CIL",arm["name"])
        paid_total+=reconstructed["steps"]
        if arm["feedback"]=="truth":
            costs.setdefault(arm["regime"],{}).setdefault(arm["source"],0)
            costs[arm["regime"]][arm["source"]]+=reconstructed["steps"]
            totals[arm["source"]]+=reconstructed["steps"]
    assert paid_total==native["total_private_paid_cil"]
    assert totals==native["costs"]
    wins=losses=ties=exposures=nonexposed=0
    saving_total=0
    gate_allow=gate_abstain=0
    for i in range(45):
        gate,always,abstain,off,placebo,never=expected[6*i:6*i+6]
        assert [x["source"] for x in (gate,always,abstain,off,placebo,never)]==list(SOURCES)
        assert gate["goals"]==always["goals"]==abstain["goals"]==off["goals"]==placebo["goals"]==never["goals"]
        gate_allow+=gate["paid_gate_allow"]
        gate_abstain+=gate["paid_gate_abstain"]
        assert always["paid_gate_abstain"]==0
        assert abstain["paid_gate_allow"]==0
        assert gate["steps"]==placebo["steps"]
        assert gate["services"]==placebo["services"]
        assert gate["paid_gate_records"]==placebo["paid_gate_records"]
        n=len(off["suppressed_pc28"])
        exposures+=n
        if n==0:nonexposed+=1
        saving=off["steps"]-gate["steps"]
        saving_total+=saving
        if saving>0:wins+=1
        elif saving<0:losses+=1
        else:ties+=1
        def trace(x):
            path=Path(sys.argv[2])/("cl1b_g2_actor_gate_c4_original_"+x["name"]+".jsonl")
            return [json.loads(line) for line in path.read_text(encoding="utf-8").splitlines() if line]
        tgate=trace(gate);toff=trace(off);tplacebo=trace(placebo)
        assert tgate==tplacebo,"original-native PC180 placebo differs"
        if n==0:assert tgate==toff,"identical-source no-PC28-exposure worlds differ"
        else:
            first=off["suppressed_pc28"][0]["step"]
            idx=next(k for k,x in enumerate(toff)
                     if x[0]=="cil" and x[1]==first and x[2]==28)
            assert tgate[:idx]==toff[:idx],"original paid CIL prefix diverges before suppression"
            assert toff[idx][10]=="suppressed_pc28"
    assert gate_allow==native["actor_gate_allow_events"] and gate_abstain==native["actor_gate_abstain_events"]
    assert gate_allow>0 and gate_abstain>0
    assert saving_total==native["paired_suppressed_minus_gate_cost"]
    assert (wins,losses,ties)==(native["causal_wins"],native["causal_losses"],native["causal_ties"])
    assert exposures==native["actual_suppressed_pc28_attempts"]
    assert nonexposed==native["no_exposure_pairs"]
    assert totals["G2_gate_always_edit"]-totals["G2_actor_gate"]==native["net_gate_saving_vs_always"]
    assert totals["G2_gate_always_abstain"]-totals["G2_actor_gate"]==native["net_gate_saving_vs_abstain"]
    assert (totals["G2_actor_gate"]<totals["G2_gate_always_edit"] and
            totals["G2_actor_gate"]<totals["G2_gate_always_abstain"])==native["main_efficiency_accepted"]
    print(json.dumps({
      "study":"CL1B-G2-ACTOR-PAID-GATE-C4",
      "status":"VERIFIED_INDEPENDENT_ORIGINAL_NATIVE_ACTOR_PAID_CIL_GATE",
      "source_manifest":"cl1b/studies/CL1B-G2-ACTOR-PAID-GATE-C4-frozen-274-worlds-v1.json",
      "verified_original_native_worlds":274,
      "verified_charged_original_cil_instructions":paid_total,
      "first_divergence":None,
      "actor_paid_gate_edit_events":gate_allow,
      "actor_paid_gate_abstain_events":gate_abstain,
      "suppressed_pc28_effects_paid":exposures,
      "causal_suppressed_minus_gate_cost":saving_total,
      "causal_gate_wins":wins,"causal_gate_losses":losses,"causal_gate_ties":ties,
      "net_gate_saving_vs_always":native["net_gate_saving_vs_always"],
      "net_gate_saving_vs_abstain":native["net_gate_saving_vs_abstain"],
      "main_efficiency_accepted":native["main_efficiency_accepted"],
      "arm_costs":totals,
      "family_charged_cil_totals":costs,
      "nonclaims":["Researcher-designed r3 proxy; no novel actor-invented rule.",
          "Gate/always/abstain source programs differ by paid PC67 route, not identical-source causal ablation.",
          "Identical-source effect-suppressed and placebo are only host research interventions.",
          "No authenticated feedback, open-ended evolution or CL1B B3/B4."]
    },indent=2,ensure_ascii=False))
if __name__=="__main__":main()
