#!/usr/bin/env python3
"""P4F2 decision boundary: exact finite enumeration, NOT Rust/CIL native evidence.

A counterfactual cost model, deliberately separate from the original CL0 world.
Observations are truthful synthetic bits, 1=old policy works, 0=old policy fails.
Decision at t=2: restore immediately or pay to observe t=3 before deciding.
After decision, policy remains fixed for the remaining stages; no future oracle.
"""
import itertools
import json

FAMILIES = {
    "transient": (0, 0, 1, 1, 1, 1),
    "persistent": (0, 0, 0, 0, 0, 0),
    "delayed": (0, 0, 0, 1, 1, 1),
}
# Restoration makes the alternative work on failures of the old policy.
# A source edit costs PATCH; each wrong-policy task costs MISMATCH.
PATCH = 3
MISMATCH = 4
OBSERVE = 2

def loss(bits, restore_at, observation_cost=0):
    """Restoration occurs before the indexed stage; None means never."""
    return observation_cost + (PATCH if restore_at is not None else 0) + sum(
        MISMATCH * (int(bit == 1) if restore_at is not None and t >= restore_at
                    else int(bit == 0))
        for t, bit in enumerate(bits)
        if t >= 2
    )

def audit():
    # A prefix-only deterministic policy must make one decision for all worlds.
    assert len({v[:2] for v in FAMILIES.values()}) == 1
    immediate = {name: loss(seq, 2) for name, seq in FAMILIES.items()}
    never = {name: loss(seq, None) for name, seq in FAMILIES.items()}
    # One extra truthful observation, then a deterministic decision by bit.
    conditional = {}
    for mapping in itertools.product((None, 3), repeat=2):
        key = f"if0={'restore' if mapping[0] == 3 else 'keep'},if1={'restore' if mapping[1] == 3 else 'keep'}"
        conditional[key] = {
            name: loss(seq, mapping[seq[2]], OBSERVE)
            for name, seq in FAMILIES.items()
        }
    policies = {"immediate": immediate, "never": never, **conditional}
    totals = {name: sum(values.values()) for name, values in policies.items()}
    optimum = min(totals.values())
    best = sorted(k for k, v in totals.items() if v == optimum)
    # Inability of a 2-observation decision to distinguish any pair is structural.
    # At t=3 transient is distinguishable; persistent and delayed remain identical.
    classes = {}
    for name, seq in FAMILIES.items():
        classes.setdefault("".join(map(str, seq[:3])), []).append(name)
    assert classes == {"001": ["transient"], "000": ["persistent", "delayed"]}
    return {
        "status": "SYNTHETIC_COUNTERFACTUAL_NOT_NATIVE",
        "frozen_sequences": FAMILIES,
        "cost_assumptions": {"patch": PATCH, "mismatch": MISMATCH, "third_observation": OBSERVE},
        "post_two_failure_policy_costs": policies,
        "total_equal_family_weight_cost": totals,
        "minimum_cost": optimum,
        "optimal_policies": best,
        "three_observation_equivalence_classes": classes,
        "falsifiable_native_hypothesis": (
            "An actor that pays for one additional truthful task observation can "
            "avoid some premature rollbacks on transient shocks; it cannot identify "
            "persistent versus delayed recovery at stage three. Native total net "
            "benefit may be negative once actual CIL observation and patch charges count."
        ),
        "strong_controls": [
            "same-source immediate rollback after two failures",
            "same-budget observation with result masked",
            "third feedback bit inverted after truthful task service",
            "patch suppressed but its charge retained",
            "unseen shock lengths and recovery timings frozen before native execution"
        ],
        "nonclaims": [
            "No original Rust world was run by this script",
            "Costs are hypothetical units, not measured CIL instructions",
            "The 3 families are hand selected, not a population sample",
            "No B3/B4 or cyberlife milestone is established"
        ]
    }

if __name__ == "__main__":
    print(json.dumps(audit(), indent=2, sort_keys=True))
