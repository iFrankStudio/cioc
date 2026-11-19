#!/usr/bin/env python3
"""Independently reconstruct every original paid native CIL step, including PatchNext.

P1-L2-M2 source is precommitted. Real native JSONL is evidence input only.
Neither this program nor the native test establishes task feedback learning.
"""
import json
import os
import sys
from pathlib import Path

ROOT=Path(__file__).resolve().parents[1]
FROZEN=ROOT/"studies/B56-F1d1k-G3Q-C1-P1-L2-M2-frozen-179-node-self-terminal-certified-meta-cycle-v1.json"
ANCESTOR=ROOT/"studies/B56-F1d1k-G3Q-C0-frozen-exact-164-node-original-CIL-founder-v1.json"
KEYS=(7,11,20,21,32,33,50,51,52,53,56,61,62,63,64,65)
STUDY="B56-F1d1k-G3Q-C1-P1-L2-M2"

def interpret(code, trace_path=None):
    code=[dict(x) for x in code]
    reg=[0,0,0,0]
    cells={}
    pc=0
    ordinal=0
    bank_reads=[]
    active_writes=[]
    meta_fetches=[]
    checkpoint=[]
    modes=[]
    patches=[]
    verdicts=0
    retained=0
    halted=False
    def steps():
        if trace_path is None:
            while True:yield None
        else:
            with Path(trace_path).open() as stream:
                for line in stream:yield json.loads(line)
    for row in steps():
        if ordinal>=32000:raise AssertionError("compute budget exceeded")
        here=pc
        op=code[pc]
        kind=op["kind"]
        ordinal+=1
        if kind=="Const":reg[op["dst"]]=op["value"]
        elif kind in ("Add","Sub"):
            a,b=reg[op["lhs"]],reg[op["rhs"]]
            reg[op["dst"]]=a+b if kind=="Add" else a-b
        elif kind=="Read":
            address=reg[op["address"]]
            value=cells.get(address,0)
            reg[op["dst"]]=value
            if here in (164,168,172):bank_reads.append([address,value])
            if here==29 and patches:meta_fetches.append([address,value])
        elif kind=="Write":
            address=reg[op["address"]]
            value=reg[op["value"]]
            cells[address]=value
            if here==106:verdicts+=1
            if here in (113,117,121):retained+=1
            if here in (166,170,174):active_writes.append([address,value])
            if here==46 and patches:checkpoint.append(value)
            if here==59 and patches:modes.append(value)
        elif kind=="Branch":
            pc=op["nonzero"] if reg[op["test"]] else op["zero"]
        elif kind=="PatchNext":
            index=reg[op["node"]]
            target=reg[op["target"]]
            if not (0<=index<len(code) and 0<=target<len(code)):raise AssertionError("invalid paid patch")
            previous=code[index].get("next")
            if previous is None:raise AssertionError("node without patchable edge")
            code[index]["next"]=target
            patches.append({"step":ordinal,"executed_pc":here,"node":index,
                           "before":previous,"after":target})
        elif kind=="Halt":
            halted=True
        else:
            raise AssertionError(("unsupported CIL source",here,kind))
        if kind not in ("Branch","Halt"):
            pc=op["next"]
        expected=[ordinal,here,pc,list(reg),32000-ordinal,
                  [cells.get(k) for k in KEYS],180+len(cells),code[59]["next"]]
        if row is not None and row!=expected:
            names=("ordinal","old PC","next PC","registers","private credits",
                   "16 native cell states","occupied units","original CIL PC59.next")
            for name,actual,wanted in zip(names,row,expected):
                if actual!=wanted:raise AssertionError({
                    "first_original_native_divergence":ordinal,
                    "field":name,"source_opcode":op,
                    "native_observed":actual,"independent_expected":wanted})
            raise AssertionError({"first_original_native_divergence":ordinal,
                                  "native_row":row,"expected":expected})
        if halted:break
    if not halted:raise AssertionError("original PID did not naturally Halt")
    return {
        "paid_steps":ordinal,"credits_remaining":32000-ordinal,
        "certificate_writes":verdicts,"bank_retention_writes":retained,
        "bank_reads":bank_reads,"active_writes":active_writes,
        "dynamic_meta_reads":meta_fetches,"accumulator_checkpoints":checkpoint,
        "mode_outputs":modes,"patches":patches,
        "final_halt_pc":pc,"source_edged_PC59":code[59]["next"],
        "data_bank":[cells[k] for k in (51,52,53)],
        "occupied":180+len(cells),"used_cells":len(cells),
        "external_postload_data_injections_observed_by_model":0}

def main():
    if len(sys.argv) not in (1,4):
        raise SystemExit("usage: python3 SCRIPT [NATIVE_SUMMARY ORIGINAL_NATIVE.jsonl MUTANT_NATIVE.jsonl]")
    frozen=json.loads(FROZEN.read_text())
    code=frozen["exact_native_founder_code"]
    original=json.loads(ANCESTOR.read_text())["exact_native_founder_code"]
    assert len(code)==179 and len(original)==164 and code[:163]==original[:163]
    assert code[177]=={"kind":"PatchNext","node":2,"target":3,"next":21}
    assert code[178]=={"kind":"Halt"}
    arms=frozen["expected_frozen_arms"]
    assert len(arms)==2
    native=None
    if len(sys.argv)==4:
        native=json.loads(Path(sys.argv[1]).read_text())
        assert native["study"]==STUDY
        assert native["level"]=="NATIVE_SINGLE_PID_PROGRAMMED_SELF_PATCHED_TERMINAL_META_CYCLE"
        assert len(native["arms"])==2
        assert native["source_sha"] not in ("","local")
        if os.environ.get("GITHUB_SHA"):
            assert native["source_sha"]==os.environ["GITHUB_SHA"]
    records=[]
    for i,arm in enumerate(arms):
        source=[dict(x) for x in code]
        mutation=arm["preload_source_mutation"]
        if mutation is not None:
            assert mutation=={"pc":171,"field":"value","from":53,"to":51}
            assert source[171]["value"]==53
            source[171]["value"]=51
        replay=interpret(source,sys.argv[i+2] if native else None)
        exp=arm["expected"]
        checks={
            "paid_steps":exp["paid_steps"],
            "credits_remaining":exp["credits_remaining"],
            "certificate_writes":exp["coverage_certificates"],
            "bank_retention_writes":exp["paid_safe_bank_retention"],
            "bank_reads":exp["retained_bank_reads"],
            "active_writes":exp["active_writes"],
            "dynamic_meta_reads":exp["dynamic_meta_reads"],
            "accumulator_checkpoints":exp["accumulator_checkpoints"],
            "mode_outputs":[exp["meta_mode_output"]],
            "final_halt_pc":exp["final_halt_pc"],
            "source_edged_PC59":178,
            "data_bank":exp["bank"],
            "occupied":exp["occupied"],
            "used_cells":exp["used_cells"]}
        for k,v in checks.items():
            if replay[k]!=v:raise AssertionError((arm["name"],k,replay[k],v))
        assert replay["patches"]==[{"step":24064,"executed_pc":177,
            "node":59,"before":60,"after":178}]
        if native:
            received=native["arms"][i]
            assert received["arm"]==arm["name"]
            mapping={
                "native_paid_steps":"paid_steps",
                "credits_remaining":"credits_remaining",
                "peak_storage":"occupied",
                "original_certificates":"certificate_writes",
                "paid_bank_retention":"bank_retention_writes",
                "bank_reads":"bank_reads",
                "active_writes":"active_writes",
                "meta_fetches":"dynamic_meta_reads",
                "accumulator_checkpoints":"accumulator_checkpoints",
                "computed_modes":"mode_outputs"}
            for actual,expected_key in mapping.items():
                assert received[actual]==replay[expected_key],(arm["name"],actual)
            assert received["source_patch_count"]==1
            assert received["source_patch_step"]==24064
            assert received["source_patch_opcode_pc"]==177
            assert received["edited_original_node"]==59
            assert received["edited_edge_from"]==60
            assert received["edited_edge_to"]==178
            assert received["halt_pc"]==178
            assert received["original_pid_self_halted"] is True
            assert received["postload_external_writes"]==0
        records.append({"arm":arm["name"],"paid_native_or_interpreted_steps":replay["paid_steps"],
            "first_divergence":None,"self_patch":replay["patches"][0],
            "output":replay["mode_outputs"][0],"halted":replay["final_halt_pc"]==178})
    assert [r["output"] for r in records]==[2,1]
    print(json.dumps({
        "study":STUDY,
        "evidence_level":"EXACT_NATIVE_PID_INSTRUCTION_REPLAY" if native else "SOURCE_ONLY_PROSPECTIVE_PREDICTION",
        "verified_rows":48298 if native else 0,
        "source_sha":native["source_sha"] if native else None,
        "results":records,"first_divergence":None,
        "limitations":"Preauthored PatchNext behavior; no hidden objective, task feedback, evolving fitness or novel ISA."},
        indent=2))

if __name__=="__main__":main()
