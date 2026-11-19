#!/usr/bin/env python3
"""Independent source-level CIL replay for G3Q-C1-P0; no Rust/World imports.

This is a second interpreter of the frozen source, NOT an independently
recorded native event trace. Compare exact-SHA native evidence separately.
"""
import copy
import json
from collections import Counter
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
SOURCE = ROOT / "studies/B56-F1d1k-G3Q-C1-P0-frozen-180-node-paid-bank-deployment-founder-v1.json"
ANCESTOR = ROOT / "studies/B56-F1d1k-G3Q-C0-frozen-exact-164-node-original-CIL-founder-v1.json"

def replay(code):
    registers = [0] * 4
    cells = {}
    pc = 0
    counts = Counter()
    pcs = Counter()
    bank_reads = []
    active_writes = []
    steps = 0
    while steps < 32000:
        assert 0 <= pc < len(code), ("invalid PC", pc)
        op = code[pc]
        kind = op["kind"]
        pcs[pc] += 1
        counts[kind] += 1
        steps += 1
        if kind == "Halt":
            break
        if kind == "Const":
            registers[op["dst"]] = op["value"]
        elif kind in ("Add", "Sub"):
            a, b = registers[op["lhs"]], registers[op["rhs"]]
            registers[op["dst"]] = a + b if kind == "Add" else a - b
        elif kind == "Read":
            key = registers[op["address"]]
            value = cells.get(key, 0)
            registers[op["dst"]] = value
            if pc in (164, 168, 172):
                bank_reads.append([key, value])
        elif kind == "Write":
            key = registers[op["address"]]
            value = registers[op["value"]]
            cells[key] = value
            if pc in (166, 170, 174):
                active_writes.append([key, value])
        elif kind == "Branch":
            pc = op["nonzero"] if registers[op["test"]] != 0 else op["zero"]
            continue
        else:
            raise AssertionError(("unsupported instruction", pc, kind))
        pc = op["next"]
    else:
        raise AssertionError("compute limit exhausted without Halt")
    return dict(steps=steps, counts=dict(counts), pcs=dict(pcs),
                bank_reads=bank_reads, active_writes=active_writes,
                cells=cells, halted_pc=pc, credits_remaining=32000-steps)

def main():
    frozen = json.loads(SOURCE.read_text())
    ancestor = json.loads(ANCESTOR.read_text())
    code = frozen["exact_native_founder_code"]
    original = ancestor["exact_native_founder_code"]
    assert len(code) == 180 and len(original) == 164
    assert code[:163] == original[:163]
    assert frozen["allowed_cells"] == ancestor["allowed_cells"]
    result = replay(code)
    assert result["steps"] == 24066, result["steps"]
    assert result["credits_remaining"] == 7934
    assert result["halted_pc"] == 179
    assert result["pcs"].get(106) == 64
    assert sum(result["pcs"].get(pc, 0) for pc in (113, 117, 121)) == 72
    assert result["bank_reads"] == [[51, 1], [52, 3], [53, 2]]
    assert result["active_writes"] == [[62, 1], [63, 3], [64, 2]]
    assert [result["cells"][key] for key in (51, 52, 53, 62, 63, 64, 11)] == [1,3,2,1,3,2,2]
    assert len(result["cells"]) <= 16
    # Causal sensitivity: redirect the first bank read from cell51 to cell52.
    # The independent replay must detect changed active descriptor, not just
    # accept a hardcoded expected output.
    mutant = copy.deepcopy(code)
    assert mutant[163] == {"kind": "Const", "dst": 2, "value": 51, "next": 164}
    mutant[163]["value"] = 52
    changed = replay(mutant)
    assert changed["active_writes"] == [[62, 3], [63, 3], [64, 2]]
    assert changed["active_writes"] != result["active_writes"]
    print(json.dumps({"level":"INDEPENDENT_SOURCE_REPLAY_NOT_NATIVE_EVENT_REPLAY",
                      "study":"B56-F1d1k-G3Q-C1-P0",
                      "baseline":{k:v for k,v in result.items() if k != "pcs"},
                      "mutation_control":{"pc":163,"from":51,"to":52,
                                          "active_writes":changed["active_writes"],
                                          "detected":True}}, indent=2, sort_keys=True))

if __name__ == "__main__":
    main()
