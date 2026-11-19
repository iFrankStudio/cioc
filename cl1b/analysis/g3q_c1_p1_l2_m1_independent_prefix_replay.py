#!/usr/bin/env python3
"""Independent exact-frozen CIL interpreter for G3Q-C1-P1-L2-M1.

Source-derived finite prefix and optional same-SHA native aggregate crosscheck.
This is NOT per-native-event reexecution or a hidden task test.
"""
import json
import os
import sys
from pathlib import Path

SOURCE=(Path(__file__).resolve().parents[1]/
    "studies/B56-F1d1k-G3Q-C1-P1-L2-M1-frozen-180-node-paid-certified-bank-reexecution-v1.json")
ANCESTOR=(Path(__file__).resolve().parents[1]/
    "studies/B56-F1d1k-G3Q-C0-frozen-exact-164-node-original-CIL-founder-v1.json")

def replay(code):
    reg=[0,0,0,0];cells={};pc=0;steps=0
    bank_reads=[];active_writes=[];fetch=[];acc=[]
    cert=0;retain=0
    while steps<32000:
        here=pc
        op=code[pc];kind=op["kind"];steps+=1
        if kind=="Halt":raise AssertionError(("premature Halt",here,steps))
        if kind=="Const":reg[op["dst"]]=op["value"]
        elif kind=="Add":reg[op["dst"]]=reg[op["lhs"]]+reg[op["rhs"]]
        elif kind=="Sub":reg[op["dst"]]=reg[op["lhs"]]-reg[op["rhs"]]
        elif kind=="Read":
            key=reg[op["address"]];value=cells.get(key,0)
            reg[op["dst"]]=value
            if here in (164,168,172):bank_reads.append([key,value])
            if here==29 and len(active_writes)==3:fetch.append([key,value])
        elif kind=="Write":
            key=reg[op["address"]];value=reg[op["value"]]
            cells[key]=value
            if here==106:cert+=1
            if here in (113,117,121):retain+=1
            if here in (166,170,174):active_writes.append([key,value])
            if here==46 and len(active_writes)==3:acc.append(value)
            if here==59 and len(active_writes)==3:
                assert key==33
                assert len(cells)==16
                return dict(steps=steps,credits_remaining=32000-steps,
                    bank=bank_reads,active=active_writes,
                    fetch=fetch,checkpoint=acc,mode=value,cert=cert,ret=retain,
                    bankFinal=[cells[k] for k in (51,52,53)],
                    activeFinal=[cells[k] for k in (62,63,64)],
                    live=True)
        elif kind=="Branch":
            pc=op["nonzero"] if reg[op["test"]] else op["zero"]
            continue
        else:raise AssertionError(("invalid opcode",here,kind))
        pc=op["next"]
    raise AssertionError("no postdeployment interpreter output within 32000 paid instructions")

def main():
    frozen=json.loads(SOURCE.read_text())
    source=frozen["exact_native_founder_code"]
    ancestor=json.loads(ANCESTOR.read_text())["exact_native_founder_code"]
    assert len(source)==180 and len(ancestor)==164 and source[:163]==ancestor[:163]
    assert source[178]["next"]==21
    assert source[179]=={"kind":"Halt"}
    assert len(frozen["matched_preload_controls"])==2
    native=None
    if len(sys.argv)==2:
        native=json.loads(Path(sys.argv[1]).read_text())
        assert native["study"]==frozen["study"]
        assert native["level"]=="NATIVE_SINGLE_PID_PAID_FULL_META_REEXECUTION_PREFIX_NOT_TASK_SERVICE"
        assert len(native["arms"])==2
        assert native["source_sha"] not in ("", "local")
        if os.environ.get("GITHUB_SHA"):
            assert native["source_sha"]==os.environ["GITHUB_SHA"],"wrong CI source SHA"
    elif len(sys.argv)!=1:
        raise SystemExit("usage: python3 g3q_c1_p1_l2_m1_independent_prefix_replay.py [NATIVE_SUMMARY.json]")
    rows=[]
    for idx,arm in enumerate(frozen["matched_preload_controls"]):
        records=[dict(op) for op in source]
        mutation=arm["source_mutation"]
        if mutation is not None:
            pc=mutation["pc"]
            assert pc==171 and mutation["field"]=="value"
            assert records[pc]["value"]==mutation["before"]
            records[pc]["value"]=mutation["after"]
        result=replay(records)
        expected=arm["expected"]
        for field in ("steps","bank","active","fetch","checkpoint","mode","cert","ret","remaining"):
            check="credits_remaining" if field=="remaining" else field
            assert result[check]==expected[field],(arm["name"],field,result[check],expected[field])
        assert result["bankFinal"]==[1,3,2]
        assert result["live"] and result["ret"]==72 and result["cert"]==64
        if native:
            observed=native["arms"][idx]
            assert observed["arm"]==arm["name"]
            for field,native_field in (
                ("steps","paid_steps_until_first_postdeployment_meta_output"),
                ("credits_remaining","credits_remaining"),
                ("bank","bank_reads"),("active","active_writes"),
                ("fetch","actual_paid_meta_opcode_fetches"),
                ("checkpoint","actual_paid_accumulator_writes"),
                ("mode","mode_output"),
                ("cert","certificate_writes_before_deployment"),
                ("ret","safe_bank_writes_before_deployment")):
                assert result[field]==observed[native_field],(arm["name"],field)
            assert observed["source_nodes"]==180 and observed["pid"]==1
            assert observed["postdeployment_pid_still_alive"] is True
            assert observed["peak_occupied"]==197
        rows.append({"arm":arm["name"],"steps":result["steps"],
            "bank":result["bank"],"active":result["active"],
            "fetch":result["fetch"],"accumulators":result["checkpoint"],
            "result":result["mode"]})
    assert [x["result"] for x in rows]==[2,1]
    print(json.dumps({"study":frozen["study"],
        "level":"NATIVE_AGGREGATE_CROSSCHECK_NOT_NATIVE_EVENT_REPLAY" if native else "INDEPENDENT_FROZEN_SOURCE_PREFIX_REPLAY_ONLY",
        "rows":rows,"first_divergence":None,
        "limits":"Original PID is still running at the sampled output; no full lifecycle termination, hidden task or task feedback."},indent=2))

if __name__=="__main__":main()
