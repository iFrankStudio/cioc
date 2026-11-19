#!/usr/bin/env python3
"""Independent execution of frozen 66-node CIL source; native aggregate crosscheck.

Not a native event-by-event replay. No hidden-task or evolution claim.
"""
import json
import sys
from pathlib import Path

SOURCE=Path(__file__).resolve().parents[1]/"studies/B56-F1d1k-G3Q-C1-P1-L2-M0-frozen-66-node-full-three-word-executor-v1.json"

def replay(code):
    reg=[0,0,0,0];cell={};pc=0;steps=0
    bank_reads=[];active_writes=[];fetch=[];acc=[];outputs=[]
    while steps<1000:
        here=pc
        op=code[pc];kind=op["kind"];steps+=1
        if kind=="Halt":break
        if kind=="Const":reg[op["dst"]]=op["value"]
        elif kind in ("Add","Sub"):
            a,b=reg[op["lhs"]],reg[op["rhs"]]
            reg[op["dst"]]=a+b if kind=="Add" else a-b
        elif kind=="Read":
            address=reg[op["address"]]
            value=cell.get(address,0)
            reg[op["dst"]]=value
            if here in (10,14,18):bank_reads.append([address,value])
            if here==30:fetch.append([address,value])
        elif kind=="Write":
            address=reg[op["address"]];value=reg[op["value"]]
            cell[address]=value
            if here in (12,16,20):active_writes.append([address,value])
            if here==51:acc.append(value)
            if here==64:outputs.append(value)
        elif kind=="Branch":
            pc=op["nonzero"] if reg[op["test"]] else op["zero"]
            continue
        else:raise AssertionError((here,kind))
        pc=op["next"]
    else:raise AssertionError("Halt not reached")
    assert pc==65
    return {"steps":steps,"bankReads":bank_reads,"activeWrites":active_writes,
            "fetch":fetch,"accWrites":acc,"out":outputs,
            "output":cell[11],"cellCount":len(cell),"credits_remaining":1000-steps}

def main():
    manifest=json.loads(SOURCE.read_text())
    src=manifest["exact_native_founder_code"]
    assert len(src)==66
    arms=manifest["pre_registered_arms"]
    assert len(arms)==4
    native=None
    if len(sys.argv)==2:
        native=json.loads(Path(sys.argv[1]).read_text())
        assert native["study"]==manifest["study"]
        assert native["level"]=="NATIVE_SINGLE_PID_THREE_WORD_BOUNDED_INTERPRETER_NOT_CERTIFIED_LEARNING"
        assert native["founder_nodes"]==66
        assert native["world_caps"]==[16,180,300]
        assert len(native["observations"])==4
    elif len(sys.argv)!=1:
        raise SystemExit("usage: python3 g3q_c1_p1_l2_m0_source_replay.py [NATIVE_SUMMARY.json]")
    summary=[]
    for ix,arm in enumerate(arms):
        changed=[dict(op) for op in src]
        if arm["mutated_pc"] is not None:
            pc=arm["mutated_pc"]
            assert pc in (0,3,6)
            changed[pc]["value"]=arm["mutated_value"]
        measured=replay(changed)
        for key in ("steps","bankReads","activeWrites","fetch","accWrites","out","output","cellCount"):
            assert measured[key]==arm[key],(arm["name"],key,measured[key],arm[key])
        assert len(measured["fetch"])==len(measured["accWrites"])==3
        if native is not None:
            observed=native["observations"][ix]
            assert observed["name"]==arm["name"]
            match={"steps":"steps","bank_reads":"bankReads","active_writes":"activeWrites",
                   "meta_fetches":"fetch","meta_accumulators":"accWrites","service":"out",
                   "credits_remaining":"credits_remaining"}
            for k,v in match.items():
                assert observed[k]==measured[v],(arm["name"],k)
            assert observed["peak"]<=76
        summary.append({"arm":arm["name"],"steps":measured["steps"],
                        "output":measured["output"],"fetch":measured["fetch"],
                        "accumulators":measured["accWrites"]})
    assert [r["output"] for r in summary]==[0,2,2,2]
    print(json.dumps({"study":manifest["study"],
      "level":"NATIVE_AGGREGATE_CROSSCHECK_NOT_NATIVE_EVENT_REPLAY" if native else "INDEPENDENT_SOURCE_REPLAY_ONLY",
      "arms":summary,"first_field_divergence":None,
      "limitations":"Researcher-preloaded bytecode grammar; no same-PID certification, hidden tasks, or learning."},indent=2))

if __name__=="__main__":main()
