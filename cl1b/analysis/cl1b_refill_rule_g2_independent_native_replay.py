#!/usr/bin/env python3
"""Independent source-frozen CIL opcode-by-opcode replay of G2 native PID1."""
import json
import sys
from pathlib import Path

ROOT=Path(__file__).resolve().parents[1]
MANIFEST=ROOT/"studies/CL1B-REFILL-RULE-G2-frozen-180-node-self-modifying-184-world-v1.json"
SOURCES={"G2_changed_allow":"source_g2_changed_allow",
         "G2_changed_deny":"source_g2_changed_deny",
         "G2_always":"source_g2_always","G2_never":"source_g2_never"}
CELLS=(11,20,21,30,32,33,34,35,36,37,38,39)
CREDIT=6000

def replay(m,arm,folder):
    code=[dict(x) for x in m[SOURCES[arm["source"]]]]
    assert len(code)==180
    goals=arm["goals"]
    assert len(goals)==24 and all(x in (0,1,2) for x in goals)
    mode=arm["feedback"]
    assert mode in ("truth","inverted","forced0")
    regs=[0,0,0,0]
    memory={}
    pc=0
    paid=0
    peak=181
    services=[]
    decisions=[]
    refills=[]
    patches=[]
    edits=[]
    later_reads=[]
    last_changed=None
    failed_halt=False
    path=Path(folder)/("cl1b_refill_rule_g2_original_"+arm["name"]+".jsonl")
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
                assert code[target]["value"]==value
                if at==28:
                    assert value in (0,1,2)
                    patches.append({"step":paid,"pc":28,"target":173,
                        "before":pre_value,"after":value,"changed":pre_value!=value})
                    if pre_value!=value:last_changed=value
                edits.append({"step":paid,"pc":at,"target":target,
                    "before":pre_value,"after":value,"kind":"PatchConst"})
            if at in (82,131,136,139,155,26):
                target=regs[1]
                value=regs[0]
                assert target in (12,34,133)
                if at==26:assert target==133 and value==62
                edits.append({"step":paid,"pc":at,"target":target,
                    "before":pre_next,"after":value,"kind":"PatchNext"})
            if at==173:
                assert regs[2]==theta()
                if last_changed is not None:
                    assert last_changed==regs[2]
                    later_reads.append({"step":paid,"theta":last_changed})
                    last_changed=None
            if at==175:
                delta=regs[0]
                expected={
                    "G2_changed_allow":176 if delta else 179,
                    "G2_changed_deny":179 if delta else 176,
                    "G2_always":176,"G2_never":179,
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
                "paid_parameter_patch" if at==28 else "normal"])
            if at==3:
                assert kind=="Write" and memory[11]==regs[op["value"]]
                stage=memory.get(32,0)
                assert 0<=stage<24
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
    if mode=="truth":assert (completed,true_count,claim)==(24,24,1)
    if mode=="forced0":assert failed_halt and completed==0 and claim==0
    if mode=="inverted":assert claim==1 and true_count<len(services)
    return {"name":arm["name"],"source":arm["source"],"regime":arm["regime"],
        "feedback":mode,"goals":goals,"steps":paid,"services":services,
        "decisions":decisions,"refills":refills,"parameter_patches":patches,
        "paid_code_edits":edits,"later_mutated_reads":later_reads,
        "completed_stages":completed,"truthful_matches":true_count,
        "claim":claim,"final_theta":theta(),"final_edges":edges(),
        "peak_occupied":peak,"failed_third_probe_halt":failed_halt,
        "host_source_edits":0}

def main():
    if len(sys.argv)!=3:
        raise SystemExit("usage: replayer.py native_summary.json original_trace_dir")
    m=json.loads(MANIFEST.read_text(encoding="utf-8"))
    native=json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
    assert m["study"]==native["study"]=="CL1B-REFILL-RULE-G2"
    assert m["status"]=="PRE_NATIVE_FROZEN"
    assert native["status"]=="ORIGINAL_NATIVE_FROZEN"
    assert native["original_worlds"]==184
    cases=m["arms"]+m["controls"]
    expected=native["truthful_records"]+native["adversarial_records"]
    assert len(cases)==len(expected)==184
    paid_total=mutation_count=later_count=gate_count=0
    costs={}
    for arm,old in zip(cases,expected):
        assert arm["name"]==old["name"]
        new=replay(m,arm,sys.argv[2])
        assert new==old,("original native result differs from interpreter",arm["name"])
        paid_total+=new["steps"]
        if arm["feedback"]=="truth":
            costs.setdefault(arm["regime"],{}).setdefault(arm["source"],0)
            costs[arm["regime"]][arm["source"]]+=new["steps"]
        if arm["feedback"]=="truth" and arm["source"]=="G2_changed_allow":
            mutation_count+=sum(x["changed"] for x in new["parameter_patches"])
            later_count+=len(new["later_mutated_reads"])
            gate_count+=len(new["decisions"])
    assert paid_total==native["total_private_paid_cil"]
    assert mutation_count==native["conditional_effective_source_mutations"]
    assert later_count==native["conditional_later_mutated_parameter_reads"]
    assert gate_count==native["conditional_paid_decision_gates"]
    assert mutation_count>0 and later_count>0 and gate_count>0
    print(json.dumps({"study":"CL1B-REFILL-RULE-G2",
        "status":"VERIFIED_ORIGINAL_NATIVE_CIL_PER_INSTRUCTION",
        "worlds":184,"total_native_paid_cil":paid_total,"first_divergence":None,
        "conditional_effective_source_mutations":mutation_count,
        "conditional_later_mutated_parameter_reads":later_count,
        "conditional_paid_decision_gates":gate_count,
        "per_regime_total_charged_cil":costs,
        "nonclaims":["Source mutator authored externally","B3/B4 not met"]},
        ensure_ascii=False,indent=2))

if __name__=="__main__":main()
