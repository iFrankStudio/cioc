#!/usr/bin/env python3
"""I6 information-identifiability bound (synthetic, not native execution).

Condition a two-edit policy on the visible initial three-word genotype, but NOT
on the hidden cyclic semantic relabeling. Exhaustively maximize over 144 fixed
two-edit actions separately for each of the 64 visible genotypes. This is an
oracle upper bound for any deterministic no-new-evidence policy that selects
both edits from the seed alone. It is NOT a PID1 implementation or a native
performance measurement.
"""
import itertools
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
FROZEN = ROOT / "studies/B56-F1d1k-G3Q-C1-P1-L3-I5-frozen-173-node-optimal-one-byte-repair-v1.json"
V = range(4)
OPS = tuple(itertools.product(range(3), V))
POLICIES = tuple(itertools.product(OPS, repeat=2))
SHIFTS = tuple(itertools.product(V, repeat=3))

def shift_genotype(g, shift):
    return tuple((a + b) % 4 for a, b in zip(g, shift))

def edit(g, op):
    i, v = op
    if g[i] == v:
        return None
    new = list(g)
    new[i] = v
    return tuple(new)

def success(seed, safe, policy):
    middle = edit(seed, policy[0])
    if middle is None or middle in safe:
        return False
    final = edit(middle, policy[1])
    return final is not None and final in safe

def main():
    frozen = json.loads(FROZEN.read_text(encoding="utf-8"))
    safe0 = {tuple(e["seed"]) for e in frozen["pre_registered_64_original_genotypes"] if e["initial_safe"]}
    hard0 = tuple(map(tuple, frozen["finite_one_byte_bound_proof"]["impossible_genotypes"]))
    assert len(safe0) == 24 and len(hard0) == 8
    cases = {seed: [] for seed in itertools.product(V, repeat=3)}
    for shift in SHIFTS:
        safe = {shift_genotype(x, shift) for x in safe0}
        for hard in hard0:
            seed = shift_genotype(hard, shift)
            cases[seed].append(safe)
    assert sum(map(len, cases.values())) == 512
    assert all(len(v) == 8 for v in cases.values())
    best_per_seed = {}
    for seed, safe_sets in cases.items():
        best = max(sum(success(seed, safe, p) for safe in safe_sets) for p in POLICIES)
        best_per_seed[str(seed)] = best
    assert set(best_per_seed.values()) == {6}
    assert sum(best_per_seed.values()) == 384
    print(json.dumps({
        "evidence_level": "SYNTHETIC_FROZEN_SEMANTIC_ENUMERATION_NOT_NATIVE",
        "shift_count": 64,
        "observed_seed_count": 64,
        "cases": 512,
        "candidate_two_edit_policies_per_seed": 144,
        "best_seed_conditioned_no_new_evidence_success": 384,
        "unavoidable_failures_without_shift_information": 128,
        "best_per_visible_seed": "6/8 for every one of the 64 seeds",
        "interpretation": "Even a privileged policy allowed to memorize the best two-edit script for each visible seed cannot achieve 512/512 without extra semantic evidence.",
        "limitations": [
            "Uniform synthetic shifts and hard-case sampling are researcher-defined",
            "The bound applies only to two actual changing edits, unsafe intermediate state, and no additional semantic observations",
            "Paid intermediate probes or history-dependent observations could exceed this bound and must be modeled explicitly",
            "No native Rust world, CIL patch, PID1 learning, or Actions result is claimed"
        ]
    }, indent=2))

if __name__ == "__main__":
    main()
