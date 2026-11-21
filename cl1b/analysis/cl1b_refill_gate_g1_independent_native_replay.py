#!/usr/bin/env python3
"""CL1B-REFILL-GATE-G1 fully independent original-native CIL trace replay.

Reads frozen exact 180-node loaded CIL sources and 184 native JSONL events.
No Rust-derived source/program logic; no future-goal oracle for the actor.
"""
import json
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
FROZEN = ROOT / "studies/CL1B-REFILL-GATE-G1-frozen-180-node-native-184-worlds-v1.json"
CELLS = (11,20,21,30,32,33,34,35,36,37,38,39)
SOURCE_MAP = {
    "F6_parent": "source_f6_original",
    "G1_conditional": "source_g1_conditional",
    "G1_always": "source_g1_always",
    "G1_never": "source_g1_never",
}
CREDIT = 6000

def original_trace_path(folder, name):
    return Path(folder) / ("cl1b_refill_gate_g1_original_" + name + ".jsonl")

def replay(m, arm, folder):
    program = [dict(x) for x in m[SOURCE_MAP[arm["source"]]]]
    assert len(program) == 180
    goals = arm["goals"]
    assert len(goals) == 24 and all(x in (0,1,2) for x in goals)
    mode = arm["feedback"]
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
    path=original_trace_path(folder,arm["name"])
    with path.open(encoding="utf-8") as file:
        iterator=(json.loads(line) for line in file if line.strip())
        def current_edges():
            return [program[12]["next"],program[34]["next"],program[46]["value"],program[133]["next"]]
        def assert_row(expected):
            row=next(iterator,None)
            if row!=expected:
                raise AssertionError(json.dumps({
                    "name":arm["name"],"paid":paid,"expected":expected,"native":row
                },ensure_ascii=False)[:4500])
        while paid < CREDIT:
            at=pc
            op=program[pc]
            kind=op["kind"]
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
                address=regs[op["address"]]
                value=regs[op["value"]]
                memory[address]=value
                pc=op["next"]
                if (arm["source"]=="F6_parent" and at==174) or (arm["source"]!="F6_parent" and at==175):
                    assert address==38 and value==0
                    refills.append({"step":paid,"pc":at})
            elif kind=="Branch":
                pc=op["nonzero"] if regs[op["test"]] else op["zero"]
                if at==173:
                    assert arm["source"]!="F6_parent"
                    expected={
                        "G1_conditional":174 if regs[3]!=0 else 176,
                        "G1_always":174,"G1_never":176
                    }[arm["source"]]
                    assert pc==expected
                    decisions.append({"step":paid,"guess":regs[3],"next":pc})
            elif kind=="PatchConst":
                target=regs[op["node"]]
                value=regs[op["value"]]
                assert target==46
                program[target]["value"]=value
                pc=op["next"]
            elif kind=="PatchNext":
                target=regs[op["node"]]
                destination=regs[op["target"]]
                assert target in (12,34,133)
                assert 0<=destination<len(program)
                program[target]["next"]=destination
                pc=op["next"]
            elif kind=="Halt":
                pc=at
            else:
                raise AssertionError("unknown native CIL opcode "+kind)
            if at in (45,88,102,122,82,131,136,139,155,177,178):
                if (arm["source"]=="F6_parent" and at!=178) or (arm["source"]!="F6_parent" and at!=177):
                    patches.append({"step":paid,"pc":at})
            occupied=181+len(memory)
            peak=max(peak,occupied)
            assert len(memory)<=16 and occupied<=300
            assert 0<=pc<180
            assert_row(["cil",paid,at,pc,list(regs),CREDIT-paid,
                        [memory.get(k) for k in CELLS],occupied,current_edges()])
            if at==3:
                assert kind=="Write" and regs[op["value"]]==memory[11]
                guess=memory[11]
                stage=memory.get(32,0)
                assert 0<=stage<24
                truth=int(guess==goals[stage])
                feedback=truth if mode=="truth" else 1-truth if mode=="inverted" else 0
                services.append({"step":paid,"stage":stage,"guess":guess,"truth":truth,"feedback":feedback})
                memory[20]=feedback
                occupied=181+len(memory)
                peak=max(peak,occupied)
                assert_row(["feedback",paid,stage,goals[stage],guess,truth,feedback,
                            [memory.get(k) for k in CELLS],occupied,current_edges()])
            if kind=="Halt":
                break
        else:
            raise AssertionError("original paid CIL credit exhausted before Halt: "+arm["name"])
        assert next(iterator,None) is None, "extra native trace events"
    if mode=="truth":
        assert memory.get(32)==24 and sum(s["truth"] for s in services)==24
        assert memory.get(21)==1
    return {
        "name":arm["name"],"source":arm["source"],"regime":arm["regime"],
        "feedback":mode,"goals":goals,"steps":paid,"services":services,
        "decisions":decisions,"refills":refills,"patches":patches,
        "completed_stages":memory.get(32,0),
        "truthful_matches":sum(s["truth"] for s in services),
        "claim":memory.get(21,0),"final_edges":current_edges(),
        "peak_occupied":peak,"host_source_edits":0
    }

def main():
    if len(sys.argv)!=3:
        raise SystemExit("usage: replay.py /path/to/native_184_worlds.json /path/to/original-trace-dir")
    summary=json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
    m=json.loads(FROZEN.read_text(encoding="utf-8"))
    assert summary["study"]==m["study"]=="CL1B-REFILL-GATE-G1"
    assert summary["status"]=="ORIGINAL_NATIVE_PROSPECTIVE"
    assert summary["worlds"]==m["counts"]["total_native_worlds"]==184
    expected_cases=m["arms"]+m["controls"]
    native_cases=summary["truthful_records"]+summary["adversarial_records"]
    assert len(expected_cases)==len(native_cases)==184
    total_paid=0
    gate_rows=0
    cost_by_regime={}
    for arm,native in zip(expected_cases,native_cases):
        assert native["name"]==arm["name"]
        actual=replay(m,arm,sys.argv[2])
        assert actual==native, ("summary mismatch",arm["name"],actual,native)
        total_paid+=actual["steps"]
        if arm["source"]=="G1_conditional" and arm["feedback"]=="truth":
            gate_rows+=len(actual["decisions"])
        if arm["feedback"]=="truth":
            cost_by_regime.setdefault(arm["regime"],{})
            cost_by_regime[arm["regime"]].setdefault(arm["source"],0)
            cost_by_regime[arm["regime"]][arm["source"]]+=actual["steps"]
    assert total_paid==summary["total_private_paid_cil"]
    assert gate_rows==summary["actor_conditional_gates"]
    assert len(cost_by_regime)==5
    print(json.dumps({
        "study":"CL1B-REFILL-GATE-G1",
        "status":"VERIFIED_INDEPENDENT_ORIGINAL_NATIVE_CIL_REPLAY",
        "source_manifest":"CL1B-REFILL-GATE-G1-frozen-180-node-native-184-worlds-v1.json",
        "verified_worlds":184,"verified_charged_instructions":total_paid,
        "first_divergence":None,"actor_conditional_gates":gate_rows,
        "cost_by_regime":cost_by_regime,
        "nonclaims":[
            "All G1 initial conditional policies were researcher authored.",
            "Not a test of autonomous policy invention or open-ended evolution."
        ]},ensure_ascii=False,indent=2))
if __name__=="__main__":
    main()
