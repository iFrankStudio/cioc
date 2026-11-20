#!/usr/bin/env python3
"""P3->P4 prefix-identifiability audit. Source-only, NOT a native CIL experiment.

Run from repository root:
  python3 cl1b/analysis/cl1b_policy_p4_prefix_identifiability.py
No third-party dependencies. Uses frozen P3 cases, never post-hoc generated labels.
"""
import json
from collections import Counter
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
frozen = json.loads((ROOT / "studies/CL1B-POLICY-P3-frozen-eight-stage-regime-transfer-v1.json").read_text())
arms = [a for a in frozen["arms"] if a["feedback"] == "truth" and not a["suppressed"]]
assert len(arms) == 27
by_key = {(a["regime"], a["start"], a["first_delta"]): tuple(a["goals"]) for a in arms}
assert len(by_key) == 27
first_differences = Counter()
for a in range(3):
    for d in range(3):
        s = by_key["stable", a, d]
        w = by_key["switch", a, d]
        c = by_key["scrambled", a, d]
        assert len(s) == len(w) == len(c) == 8
        assert s[:4] == w[:4], (a, d)
        assert s[:2] == c[:2], (a, d)
        for label, other, expected in (("switch", w, 4), ("scrambled", c, 2)):
            first = next((i for i, (x, y) in enumerate(zip(s, other)) if x != y), None)
            assert first == expected, (label, a, d, first)
            first_differences[(label, first)] += 1
assert first_differences == {("switch", 4): 9, ("scrambled", 2): 9}
print(json.dumps({
    "status": "SOURCE_ONLY_PREFIX_AUDIT_NOT_NATIVE",
    "paired_frozen_sequences": 27,
    "same_first_two_goals_across_all_regimes_per_a_d": True,
    "stable_switch_same_first_four_goals_per_a_d": True,
    "first_goal_index_distinguishing_stable_from_switch": 4,
    "first_goal_index_distinguishing_stable_from_scrambled": 2,
    "count_per_comparison": 9,
    "causal_limit": "A paid actor with only the first two truthful task outcomes cannot identify the regime. Later goal feedback is available only after actual paid service; never inject regime identity or future goal into CIL.",
}, indent=2))
