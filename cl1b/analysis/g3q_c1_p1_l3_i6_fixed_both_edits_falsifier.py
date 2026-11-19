#!/usr/bin/env python3
"""I6 pre-native falsifier: exhaustive genotype-independent TWO-edit scripts.

Reads I5's frozen initial_safe oracle, not Rust traces. A fixed script consists
of two ordered (gene_index, replacement_byte) operations. Both must change a
byte in EVERY hard case; certification is evaluated only after both edits.
This is a finite semantic counterexample, not original-PID execution.
"""
import itertools
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
SOURCE = ROOT / "studies/B56-F1d1k-G3Q-C1-P1-L3-I5-frozen-173-node-optimal-one-byte-repair-v1.json"

def main():
    frozen = json.loads(SOURCE.read_text(encoding="utf-8"))
    entries = frozen["pre_registered_64_original_genotypes"]
    assert len(entries) == 64
    safe = {tuple(x["seed"]) for x in entries if x["initial_safe"]}
    assert len(safe) == 24
    hard = [tuple(x) for x in frozen["finite_one_byte_bound_proof"]["impossible_genotypes"]]
    assert len(hard) == 8
    operations = tuple(itertools.product(range(3), range(4)))
    results = []
    for first, second in itertools.product(operations, repeat=2):
        passed = []
        for seed in hard:
            state = list(seed)
            if state[first[0]] == first[1]:
                continue  # No-op is not a paid actual source mutation.
            state[first[0]] = first[1]
            assert tuple(state) not in safe, "I5 hard case unexpectedly one-edit safe"
            if state[second[0]] == second[1]:
                continue
            state[second[0]] = second[1]
            if tuple(state) in safe:
                passed.append(list(seed))
        results.append({"first": list(first), "second": list(second), "successes": len(passed)})
    optimum = max(r["successes"] for r in results)
    winners = [r for r in results if r["successes"] == optimum]
    assert optimum == 8
    assert len(winners) == 6, winners
    assert any(r["first"] == [2, 0] and r["second"] == [1, 0] for r in winners)
    print(json.dumps({
        "status": "PRE_NATIVE_I5_ORACLE_ONLY",
        "tested_fixed_two_edit_scripts": len(results),
        "hard_cases": [list(x) for x in hard],
        "maximum_hard_cases_rescued_without_any_branching": optimum,
        "number_of_optimal_fixed_scripts": len(winners),
        "optimal_scripts": winners,
        "interpretation": "8/8 repair after two edits cannot distinguish a fresh-evidence-driven second decision from a genotype-independent fixed script; even the FIRST edit can be fixed.",
        "limitations": [
            "Safety oracle is frozen I5 prediction, not independently implemented CIL semantics",
            "No original PID1 executed these scripts; no source patch, witness reset, resource or CI evidence",
            "Real native feasibility of any two-edit script under 16/180/300 remains untested",
            "To distinguish evidence-based decisions, prospectively introduce a blinded semantic/task shift where a fixed script fails while paid evidence remains informative"
        ]
    }, indent=2))

if __name__ == "__main__":
    main()
