"""I6 idealized ONE membership-query partition bound. SYNTHETIC, NOT NATIVE.
The probe is a privileged query to the hidden shifted safe set, NOT an available CIL instruction.
It is deliberately stronger than a paid structural witness; do not claim feasibility.
"""
import itertools, json
from pathlib import Path

root = Path(__file__).resolve().parents[1]
frozen = json.loads((root / "studies/B56-F1d1k-G3Q-C1-P1-L3-I5-frozen-173-node-optimal-one-byte-repair-v1.json").read_text())
safe0 = {tuple(e["seed"]) for e in frozen["pre_registered_64_original_genotypes"] if e["initial_safe"]}
hard0 = {tuple(x) for x in frozen["finite_one_byte_bound_proof"]["impossible_genotypes"]}
assert len(safe0) == 24 and len(hard0) == 8
V = range(4)
triples = list(itertools.product(V, repeat=3))
ops = list(itertools.product(range(3), V))

def edit(g, op):
    i, v = op
    if g[i] == v:
        return None
    out = list(g)
    out[i] = v
    return tuple(out)

def bitcount(n):
    return n.bit_count()

total = 0
perfect = 0
examples = []
for seed in triples:
    environments = []
    for shift in triples:
        safe = {tuple((x + d) % 4 for x, d in zip(g, shift)) for g in safe0}
        if any(tuple((x + d) % 4 for x, d in zip(h, shift)) == seed for h in hard0):
            environments.append(safe)
    assert len(environments) == 8, (seed, len(environments))
    masks = set()
    for first, second in itertools.product(ops, repeat=2):
        mid = edit(seed, first)
        if mid is None:
            continue
        end = edit(mid, second)
        if end is None:
            continue
        mask = sum(1 << k for k, safe in enumerate(environments) if mid not in safe and end in safe)
        masks.add(mask)
    assert max(map(bitcount, masks)) == 6
    best = -1
    best_query = None
    best_split = None
    for query in triples:
        yes = sum(1 << k for k, safe in enumerate(environments) if query in safe)
        no = 255 ^ yes
        yes_best = max(bitcount(mask & yes) for mask in masks)
        no_best = max(bitcount(mask & no) for mask in masks)
        score = yes_best + no_best
        if score > best:
            best, best_query, best_split = score, query, (bitcount(yes), bitcount(no))
    assert best <= 8
    total += best
    perfect += best == 8
    if len(examples) < 8:
        examples.append({"seed": seed, "privileged_query": best_query, "partition_sizes": best_split, "best_success": best})
assert total == 512 and perfect == 64, (total, perfect)
print(json.dumps({
    "evidence_level": "SYNTHETIC_PRIVILEGED_MEMBERSHIP_QUERY_NOT_NATIVE",
    "cases": 512,
    "genotypes": 64,
    "contexts_each": 8,
    "seed_only_two_edit_ceiling": 384,
    "one_privileged_membership_query_ceiling": total,
    "genotypes_with_perfect_ideal_query": perfect,
    "examples": examples,
    "critical_limitation": "The query returns membership in the hidden shifted safe set. It is an oracle, not a paid original-PID CIL observation; the computation does not establish that any executable probe can reproduce the partition or fit 16/180/300.",
    "next_test": "Specify an actual pre-edit native paid observation and its per-environment outputs; compute its partition and best achievable success, then compare equal-cost sham, inverted feedback, and fixed policies before claiming causal adaptation."
}, indent=2))
