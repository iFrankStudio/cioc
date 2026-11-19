#!/usr/bin/env python3
"""Independent native-PID step-by-step trace replay for G3Q-C1-P1-L2-M1.

Recomputes source control flow, all four registers, cell presence/values,
private credits, PC and occupied units from a prospectively frozen CIL graph.
The native event stream is a read-only observation input, never an oracle.
"""
import json
import os
import sys
from pathlib import Path

ROOT=Path(__file__).resolve().parents[1]
FROZEN=ROOT/"studies/B56-F1d1k-G3Q-C1-P1-L2-M1-frozen-180-node-paid-certified-bank-reexecution-v1.json"
KEYS=(7,11,20,21,32,33,50,51,52,53,56,61,62,63,64,65)
NAME="B56-F1d1k-G3Q-C1-P1-L2-M1"

def replay_native_trace(code,path):
    pc=0
    registers=[0,0,0,0]
    cells={}
    steps=0
    last_pc=None
    with path.open() as stream:
        for raw in stream:
            row=json.loads(raw)
            if not isinstance(row,list) or len(row)!=7:
                raise AssertionError(("malformed native row",steps+1,row))
            observed_step,observed_pc,after_pc,observed_regs,observed_credits,observed_cells,occupied=row
            here=pc
            steps+=1
            if observed_step!=steps or observed_pc!=here:
                raise AssertionError(("native/source PC divergence",steps,here,observed_pc))
            op=code[pc]
            kind=op["kind"]
            if kind=="Const":
                registers[op["dst"]]=op["value"]
            elif kind in ("Add","Sub"):
                a,b=registers[op["lhs"]],registers[op["rhs"]]
                registers[op["dst"]]=a+b if kind=="Add" else a-b
            elif kind=="Read":
                address=registers[op["address"]]
                registers[op["dst"]]=cells.get(address,0)
            elif kind=="Write":
                address=registers[op["address"]]
                cells[address]=registers[op["value"]]
            elif kind=="Branch":
                pc=op["nonzero"] if registers[op["test"]] else op["zero"]
            else:
                raise AssertionError(("unexpected opcode in paid prefix",steps,pc,kind))
            if kind!="Branch":
                pc=op["next"]
            expected_cells=[cells.get(k) for k in KEYS]
            if (after_pc!=pc or observed_regs!=registers or
                observed_credits!=32000-steps or observed_cells!=expected_cells or
                occupied!=181+len(cells)):
                raise AssertionError({
                    "divergent_native_original_step":steps,
                    "source_pc":here,"opcode":op,"expected_after_pc":pc,
                    "native_after_pc":after_pc,"expected_registers":registers,
                    "native_registers":observed_regs,
                    "expected_credits":32000-steps,"native_credits":observed_credits,
                    "expected_cells":expected_cells,"native_cells":observed_cells,
                    "expected_occupied":181+len(cells),"native_occupied":occupied
                })
            last_pc=here
    assert steps==24149 and last_pc==59
    assert pc==60
    assert len(cells)==16
    return {"paid_native_steps":steps,"last_original_CIL_PC":last_pc,
        "next_PC":pc,"credits_remaining":32000-steps,
        "bank":[cells[k] for k in (51,52,53)],
        "active":[cells[k] for k in (62,63,64)],
        "meta_checkpoint":cells[56],
        "meta_mode":cells[33],"occupied":181+len(cells)}

def main():
    if len(sys.argv)!=4:
        raise SystemExit("usage: python3 SCRIPT NATIVE_SUMMARY.json BASELINE_NATIVE.jsonl MUTANT_NATIVE.jsonl")
    frozen=json.loads(FROZEN.read_text())
    source=frozen["exact_native_founder_code"]
    assert len(source)==180 and source[178]["next"]==21
    summary=json.loads(Path(sys.argv[1]).read_text())
    assert summary["study"]==NAME
    assert summary["level"]=="NATIVE_SINGLE_PID_PAID_FULL_META_REEXECUTION_PREFIX_NOT_TASK_SERVICE"
    assert len(summary["arms"])==2
    assert summary["source_sha"] not in ("","local")
    if os.environ.get("GITHUB_SHA"):
        assert summary["source_sha"]==os.environ["GITHUB_SHA"]
    paths=[Path(sys.argv[2]),Path(sys.argv[3])]
    results=[]
    for i,arm in enumerate(frozen["matched_preload_controls"]):
        code=[dict(x) for x in source]
        m=arm["source_mutation"]
        if m:
            assert m["pc"]==171 and m["field"]=="value"
            assert code[171]["value"]==m["before"]
            code[171]["value"]=m["after"]
        found=replay_native_trace(code,paths[i])
        expected=arm["expected"]
        assert found["paid_native_steps"]==expected["steps"]
        assert found["meta_mode"]==expected["mode"]
        assert found["bank"]==expected["bankFinal"]
        assert found["active"]==expected["activeFinal"]
        native_arm=summary["arms"][i]
        assert native_arm["arm"]==arm["name"]
        assert native_arm["paid_steps_until_first_postdeployment_meta_output"]==found["paid_native_steps"]
        assert native_arm["mode_output"]==found["meta_mode"]
        assert native_arm["credits_remaining"]==found["credits_remaining"]
        assert native_arm["postdeployment_pid_still_alive"]
        assert native_arm["peak_occupied"]==found["occupied"]==197
        results.append({"arm":arm["name"],**found,"first_divergence":None})
    assert [x["meta_mode"] for x in results]==[2,1]
    print(json.dumps({"study":NAME,
        "level":"INDEPENDENT_EVERY_ORIGINAL_NATIVE_PID_STEP_REPLAY",
        "source_sha":summary["source_sha"],
        "results":results,
        "note":"Source-parsed, instruction-by-instruction replay of native original PID snapshots. Sample ends with PID still alive, not a hidden task service."},indent=2))

if __name__=="__main__":main()
