#!/usr/bin/env python3
"""G3Q-C1-P1-L1 independent frozen-source replay vs native Rust aggregate output.

Limited to aggregate fields; does not independently replay native event traces.
"""
import json
import sys
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1]
SOURCE=ROOT/"studies/B56-F1d1k-G3Q-C1-P1-L1-frozen-179-node-original-rule-dependent-arithmetic-v1.json"

def run(code):
    pc=0; registers=[0]*4; cells={}; steps=0; counts={}
    reads=[]; writes=[]
    while steps<32000:
        op=code[pc]; at=pc; name=op["kind"]
        steps+=1;counts[pc]=counts.get(pc,0)+1
        if name=="Halt": break
        if name=="Const": registers[op["dst"]]=op["value"]
        elif name in ("Add","Sub"):
            a,b=registers[op["lhs"]],registers[op["rhs"]]
            registers[op["dst"]]=a+b if name=="Add" else a-b
        elif name=="Read":
            key=registers[op["address"]]; val=cells.get(key,0)
            registers[op["dst"]]=val
            if at in (162,166,170): reads.append([key,val])
        elif name=="Write":
            key=registers[op["address"]]; val=registers[op["value"]]
            cells[key]=val
            if at in (164,168,172): writes.append([key,val])
        elif name=="Branch":
            pc=op["nonzero"] if registers[op["test"]] else op["zero"]
            continue
        else: raise AssertionError((pc,name))
        pc=op["next"]
    else: raise AssertionError("source does not Halt within budget")
    assert pc==178
    assert counts[106]==64
    assert sum(counts[i] for i in (111,115,119))==72
    return dict(steps=steps, credits_remaining=32000-steps,
        bank_reads=reads, active_writes=writes, endogenous_input=cells[65],
        arithmetic_output=cells[11], source_nodes=len(code), cells=cells)

def main():
    frozen=json.loads(SOURCE.read_text())
    code=frozen["exact_native_founder_code"]
    assert len(code)==179
    baseline=run(code)
    assert baseline["steps"]==23939
    assert baseline["bank_reads"]==[[51,1],[52,3],[53,2]]
    assert baseline["active_writes"]==[[62,1],[63,3],[64,2]]
    assert baseline["arithmetic_output"]==65
    assert code[169]=={"kind":"Const","dst":2,"value":53,"next":170}
    mutant=[dict(x) for x in code];mutant[169]["value"]=51
    alternative=run(mutant)
    assert alternative["arithmetic_output"]==64
    assert alternative["active_writes"]==[[62,1],[63,3],[64,1]]
    if len(sys.argv)==1:
        print(json.dumps({"level":"SOURCE_REPLAY_ONLY","baseline":baseline["arithmetic_output"],
            "mutant":alternative["arithmetic_output"],"steps":baseline["steps"]},indent=2))
        return
    if len(sys.argv)!=2: raise SystemExit("usage: python3 SCRIPT [native_report.json]")
    report=json.loads(Path(sys.argv[1]).read_text())
    assert report["study"]=="B56-F1d1k-G3Q-C1-P1-L1"
    assert report["level"]=="NATIVE_BOUNDED_RULE_DEPENDENT_ARITHMETIC_NO_HIDDEN_TASK"
    assert len(report["observations"])==2
    for data,expected,name in zip(report["observations"],(baseline,alternative),("frozen_original","preload_address_mutant")):
        assert data["label"]==name
        for key in ("steps","credits_remaining","bank_reads","active_writes",
                    "endogenous_input","arithmetic_output","source_nodes"):
            assert data[key]==expected[key],(name,key,data[key],expected[key])
        assert data["peak"]<=196
    print(json.dumps({"study":report["study"],"level":"NATIVE_AGGREGATE_VS_INDEPENDENT_SOURCE_REPLAY",
        "native_sha":report["source_sha"],"steps":baseline["steps"],
        "original_output":65,"preload_control_output":64,"first_field_divergence":None,
        "limits":"Not native event-by-event replay; no hidden task, regime shift or feedback."},indent=2))
if __name__=="__main__":main()
