#!/usr/bin/env python3
"""I6 pre-native falsifier: I5 first paid edit followed by a FIXED middle→0 edit.

Uses frozen I5 initial_safe labels, NOT Rust/CIL native execution. This is a
strong baseline against claiming that two-edit success proves adaptive choice.
"""
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
SOURCE = ROOT / "studies/B56-F1d1k-G3Q-C1-P1-L3-I5-frozen-173-node-optimal-one-byte-repair-v1.json"

def main():
    manifest = json.loads(SOURCE.read_text(encoding="utf-8"))
    entries = manifest["pre_registered_64_original_genotypes"]
    assert len(entries) == 64
    safe = {tuple(e["seed"]) for e in entries if e["initial_safe"]}
    assert len(safe) == 24
    hard = [e for e in entries if not e["initial_safe"] and not e["one_edit_safe_candidates"]]
    assert len(hard) == 8
    cases = []
    for e in hard:
        seed = tuple(e["seed"])
        first = tuple(e["expected"]["source_after"])
        patches = e["expected"]["original_paid_source_patches"]
        assert len(patches) == 1 and patches[0]["node"] == 6
        assert first != seed and first not in safe
        assert sum(a != b for a,b in zip(seed,first)) == 1
        fixed = (first[0], 0, first[2])
        assert first[1] != 0, "Fixed second edit must be an actual mutation"
        assert fixed in safe, (seed, first, fixed)
        alternatives = []
        for i in range(3):
            for v in range(4):
                if v == first[i]:
                    continue
                candidate = list(first)
                candidate[i] = v
                if tuple(candidate) in safe:
                    alternatives.append({"word": i, "value": v})
        cases.append({"seed": list(seed), "I5_first_edit": list(first),
                      "fixed_middle_zero": list(fixed),
                      "safe_second_edits": alternatives})
    result = {
        "status": "PRE_NATIVE_FROZEN_SEMANTIC_ORACLE_ONLY",
        "study": "I6 fixed-policy counterexample",
        "hard_cases": cases,
        "fixed_middle_zero_success": len(cases),
        "fixed_middle_zero_total": 8,
        "scientific_consequence": "After the actual I5 first-edit policy, a single preauthored second edit PC3→0 repairs all eight hard seeds in the frozen semantic oracle. Success alone cannot distinguish evidence-conditioned selection from a fixed two-edit script.",
        "limitations": ["Not an original PID1 run", "Not native Rust or independent CIL interpretation", "No second-edit cost, witness reset or budget feasibility established"]
    }
    print(json.dumps(result, indent=2))
if __name__ == "__main__":
    main()
