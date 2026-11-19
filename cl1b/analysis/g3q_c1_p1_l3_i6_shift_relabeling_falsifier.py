#!/usr/bin/env python3
"""I6 pre-native shift falsifier. Synthetic relabelings, NOT native CIL worlds.

Enumerates all 4^3 independent cyclic relabelings of I5's frozen safety table.
The relabeling is an explicit researcher-imposed environment change; the actor
has no mechanism to infer it yet. Compare fixed, shift-aware oracle, and
best-in-hindsight fixed two-edit policies without no-op edits or early safety.
"""
import itertools
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
SOURCE = ROOT / "studies/B56-F1d1k-G3Q-C1-P1-L3-I5-frozen-173-node-optimal-one-byte-repair-v1.json"
OPS = tuple(itertools.product(range(3), range(4)))
SHIFTS = tuple(itertools.product(range(4), repeat=3))

def relabel(g, shift):
    return tuple((x + y) % 4 for x, y in zip(g, shift))

def edit(g, op):
    index, value = op
    if g[index] == value:
        return None
    result = list(g)
    result[index] = value
    return tuple(result)

def successful(seed, safe, first, second):
    intermediate = edit(seed, first)
    if intermediate is None or intermediate in safe:
        return False
    final = edit(intermediate, second)
    return final is not None and final in safe

def main():
    frozen = json.loads(SOURCE.read_text(encoding="utf-8"))
    entries = frozen["pre_registered_64_original_genotypes"]
    assert len(entries) == 64
    safe0 = {tuple(e["seed"]) for e in entries if e["initial_safe"]}
    hard0 = tuple(map(tuple, frozen["finite_one_byte_bound_proof"]["impossible_genotypes"]))
    assert len(safe0) == 24 and len(hard0) == 8
    assert all(h not in safe0 and all(edit(h, op) not in safe0 for op in OPS) for h in hard0)
    policies = tuple(itertools.product(OPS, repeat=2))
    baseline = ((2, 0), (1, 0))
    totals = {policy: 0 for policy in policies}
    baseline_by_shift = {}
    oracle_winners_per_shift = {}
    for shift in SHIFTS:
        safe = {relabel(g, shift) for g in safe0}
        hard = tuple(relabel(g, shift) for g in hard0)
        successes = {}
        for policy in policies:
            score = sum(successful(h, safe, *policy) for h in hard)
            successes[policy] = score
            totals[policy] += score
        winners = [p for p, n in successes.items() if n == len(hard)]
        assert len(winners) == 6, (shift, winners)
        baseline_by_shift[str(shift)] = successes[baseline]
        oracle_winners_per_shift[str(shift)] = len(winners)
    best_fixed = max(totals.values())
    assert totals[baseline] == 192
    assert best_fixed == 192
    assert len(SHIFTS) * len(hard0) == 512
    assert sum(1 for n in baseline_by_shift.values() if n == 8) == 12
    assert sum(1 for n in baseline_by_shift.values() if n == 0) == 16
    print(json.dumps({
        "status": "PRE_NATIVE_SYNTHETIC_SEMANTIC_RELABELING_ONLY",
        "shifts": len(SHIFTS),
        "cases": len(SHIFTS) * len(hard0),
        "policies": len(policies),
        "fixed_original_policy_successes": totals[baseline],
        "best_global_fixed_policy_successes": best_fixed,
        "shift_aware_oracle_successes": 512,
        "fully_successful_fixed_policy_count_per_shift": 6,
        "baseline_shift_success_histogram": {
            str(n): list(baseline_by_shift.values()).count(n) for n in (0, 2, 4, 8)
        },
        "limitations": [
            "Relabelings are synthetic oracle transformations, not implemented native semantics",
            "Shift-aware oracle is a privileged host upper bound, not an original PID1 algorithm",
            "No actor has observed a shift or executed two paid PatchConst operations",
            "A native blinded shift experiment must first implement paid evidence that identifies the changed semantics",
            "Fixed-policy score counts only actual byte-changing edits and checks unsafe intermediate state"
        ]
    }, indent=2))

if __name__ == "__main__":
    main()
