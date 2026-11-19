#!/usr/bin/env python3
"""I6: privileged one-bit policy-selection ceiling, synthetic only.

For each visible genotype, compare best single fixed two-edit action with
the best union of TWO fixed actions. An oracle bit may select which action
succeeds. This is an information-theoretic upper bound, NOT an implementable
paid probe, because the oracle partition depends on hidden semantics.
"""
import itertools
import json
from collections import Counter
from pathlib import Path

root = Path(__file__).resolve().parents[1]
frozen = json.loads((root / "studies/B56-F1d1k-G3Q-C1-P1-L3-I5-frozen-173-node-optimal-one-byte-repair-v1.json").read_text())
safe0 = [tuple(e["seed"]) for e in frozen["pre_registered_64_original_genotypes"] if e["initial_safe"]]
hard0 = [tuple(x) for x in frozen["finite_one_byte_bound_proof"]["impossible_genotypes"]]
assert len(safe0) == 24 and len(hard0) == 8
V = range(4)
ops = tuple(itertools.product(range(3), V))
cases = {seed: [] for seed in itertools.product(V, repeat=3)}
for shift in itertools.product(V, repeat=3):
    safe = {tuple((v + d) % 4 for v, d in zip(g, shift)) for g in safe0}
    for hard in hard0:
        seed = tuple((v + d) % 4 for v, d in zip(hard, shift))
        cases[seed].append(safe)

def edit(g, op):
    i, v = op
    if g[i] == v:
        return None
    out = list(g)
    out[i] = v
    return tuple(out)

def masks(seed, environments):
    for first, second in itertools.product(ops, repeat=2):
        middle = edit(seed, first)
        mask = 0
        if middle is not None:
            end = edit(middle, second)
            if end is not None:
                for k, safe in enumerate(environments):
                    if middle not in safe and end in safe:
                        mask |= 1 << k
        yield mask

single = 0
oracle_bit = 0
hist = Counter()
for seed, environments in cases.items():
    assert len(environments) == 8
    available = set(masks(seed, environments))
    one = max(m.bit_count() for m in available)
    two = max((a | b).bit_count() for a in available for b in available)
    assert one == 6 and two == 8, (seed, one, two)
    single += one
    oracle_bit += two
    hist[(one, two)] += 1
assert single == 384 and oracle_bit == 512 and len(cases) == 64
print(json.dumps({
    "evidence_level": "SYNTHETIC_ENUMERATION_NOT_NATIVE",
    "visible_genotypes": len(cases),
    "hidden_contexts_per_genotype": 8,
    "total_cases": 512,
    "fixed_two_edit_actions_per_genotype": 144,
    "seed_only_best": single,
    "privileged_one_bit_two_policy_oracle": oracle_bit,
    "per_seed_distribution": {str(k): v for k, v in hist.items()},
    "critical_nonclaim": "One oracle bit is sufficient to select between two seed-specific policies, but no paid CIL observation implementing that bit is demonstrated. A probe must distinguish the necessary partitions without privileged semantic access and must obey native resource accounting.",
    "next_falsifier": "Freeze an implementable paid probe and compare its induced partition with the privileged oracle partition, including equal-cost sham and inverted-probe controls."
}, indent=2))
