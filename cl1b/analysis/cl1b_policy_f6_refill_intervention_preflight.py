#!/usr/bin/env python3
"""Static fail-closed source gate for the CL-1B F6 selective-refill falsifier.

No native experiment, no causal estimate, and no claim of new holdout.
Usage: python3 cl1b/analysis/cl1b_policy_f6_refill_intervention_preflight.py
       python3 cl1b/analysis/cl1b_policy_f6_refill_intervention_preflight.py --self-test
"""
import argparse
import copy
import hashlib
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
STUDY = ROOT / "cl1b/studies/CL1B-POLICY-P4F6-frozen-actor-paid-clean-success-refill-542-world-v1.json"


def require(condition, message):
    if not condition:
        raise ValueError(message)


def check(frozen):
    source = frozen["original_native_CIL_source_f6"]
    require(len(source) == 180, "F6 must fill exactly 180 original source slots")
    require(frozen["limits"]["strict_data_cells"] == 16, "changed data-cell limit")
    require(frozen["limits"]["strict_occupied_units"] == 300, "changed occupancy limit")
    require(frozen["limits"]["private_pid1_compute_credits"] == 6000, "changed private compute")
    require(len(frozen["goal_cases"]) == 180, "changed case count")
    require(all(len(c["goals"]) == 24 for c in frozen["goal_cases"]), "changed stage count")
    require(len({c["name"] for c in frozen["goal_cases"]}) == 180, "duplicate case name")
    require(len(frozen["arms"]) == 540 and len(frozen["controls"]) == 2, "changed baseline arms")

    # This is a structural PC-local proof, not an execution-path proof.
    expected = {
        133: {"kind": "Const", "next": 134},  # checked separately below: editable success edge
        152: {"kind": "Write", "address": 2, "value": 0, "next": 153},
        153: {"kind": "Const", "dst": 1, "value": 133, "next": 154},
        154: {"kind": "Const", "dst": 0, "value": 156, "next": 155},
        155: {"kind": "PatchNext", "node": 1, "target": 0, "next": 134},
        170: {"kind": "Branch", "test": 0, "nonzero": 62, "zero": 171},
        171: {"kind": "Const", "dst": 2, "value": 39, "next": 172},
        172: {"kind": "Write", "address": 2, "value": 0, "next": 173},
        173: {"kind": "Const", "dst": 2, "value": 38, "next": 174},
        174: {"kind": "Write", "address": 2, "value": 0, "next": 175},
        175: {"kind": "Const", "dst": 1, "value": 133, "next": 176},
        176: {"kind": "Const", "dst": 0, "value": 62, "next": 177},
        177: {"kind": "PatchNext", "node": 1, "target": 0, "next": 178},
    }
    for pc, fields in expected.items():
        node = source[pc]
        for key, value in fields.items():
            require(node.get(key) == value, f"PC{pc}.{key}: expected {value!r}, got {node.get(key)!r}")

    require(source[133]["next"] == 62, "original monitor exit is not PC62")
    require(source[151] == {"kind": "Const", "dst": 2, "value": 38, "next": 152},
            "PC152 is no longer preceded by budget-address setup")
    require(source[147] == {"kind": "Const", "dst": 2, "value": 38, "next": 148},
            "budget read setup changed")
    require(source[166] == {"kind": "Const", "dst": 2, "value": 39, "next": 167},
            "streak increment write setup changed")
    require(source[167]["kind"] == "Write" and source[167]["next"] == 168,
            "streak increment write changed")
    # Only PC174 is eligible for suppression; never intercept all Write38=0,
    # all writes of value 0, or all writes to cell 38.
    return {
        "status": "STATIC_SOURCE_SHAPE_ONLY",
        "target_pc": 174,
        "intercept_only_when": "PC==174 AND opcode Write AND resolved address==38 AND resolved value==0",
        "charge": "identical paid Write attempt in both arms",
        "untouched_budget_consumption_pc": 152,
        "untouched_streak_reset_pc": 172,
        "original_source_nodes": len(source),
        "goal_cases": len(frozen["goal_cases"]),
        "fresh_holdout": False,
        "native_execution": False,
        "dynamic_path_verified": False,
        "causal_effect_estimated": False,
    }


def self_test(frozen):
    check(frozen)
    for pc, field, replacement in [
        (174, "next", 62), (173, "value", 39),
        (170, "zero", 62), (152, "next", 62),
        (177, "target", 2),
    ]:
        mutant = copy.deepcopy(frozen)
        mutant["original_native_CIL_source_f6"][pc][field] = replacement
        try:
            check(mutant)
        except ValueError:
            continue
        raise AssertionError(f"source mutation PC{pc}.{field} escaped fail-closed check")
    return "PASS: five structurally invalid source mutants rejected"


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--self-test", action="store_true")
    args = parser.parse_args()
    raw = STUDY.read_bytes()
    frozen = json.loads(raw)
    result = check(frozen)
    result["frozen_file_sha256"] = hashlib.sha256(raw).hexdigest()
    if args.self_test:
        result["self_test"] = self_test(frozen)
    print(json.dumps(result, ensure_ascii=False, indent=2, sort_keys=True))


if __name__ == "__main__":
    main()
