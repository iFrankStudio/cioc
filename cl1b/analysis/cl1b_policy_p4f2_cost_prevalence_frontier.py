#!/usr/bin/env python3
"""P4F2 exact cost/prevalence frontier. SYNTHETIC ONLY, not Rust/CIL evidence."""
import itertools
import json

FAMILIES = {
    "transient": (0, 0, 1, 1, 1, 1),
    "persistent": (0, 0, 0, 0, 0, 0),
    "delayed": (0, 0, 0, 1, 1, 1),
}

def loss(seq, patch, mismatch, observe, restore_at):
    return observe + (patch if restore_at is not None else 0) + sum(
        mismatch * (int(bit == 1) if restore_at is not None and t >= restore_at else int(bit == 0))
        for t, bit in enumerate(seq) if t >= 2
    )

def totals(patch, mismatch, observe, weights):
    out = {}
    for mode, choices in [("immediate", ()), ("never", ())] + [
        ("observe", b) for b in itertools.product((False, True), repeat=2)
    ]:
        name = mode if mode != "observe" else "observe_if0%d_if1%d" % choices
        out[name] = sum(weights[n] * loss(
            seq, patch, mismatch, observe if mode == "observe" else 0,
            2 if mode == "immediate" else None if mode == "never"
            else 3 if choices[seq[2]] else None
        ) for n, seq in FAMILIES.items())
    return out

def audit():
    assert len({v[:2] for v in FAMILIES.values()}) == 1
    assert FAMILIES["persistent"][:3] == FAMILIES["delayed"][:3]
    strict = 0
    for p, m, o in itertools.product(range(9), range(1, 9), range(9)):
        v = totals(p, m, o, dict.fromkeys(FAMILIES, 1))
        if min(x for k, x in v.items() if k.startswith("observe")) < min(v["immediate"], v["never"]):
            strict += 1
    baseline = totals(3, 4, 2, dict.fromkeys(FAMILIES, 1))
    weighted = totals(0, 1, 0, {"transient": 1, "persistent": 2, "delayed": 1})
    assert strict == 0
    assert (baseline["immediate"], baseline["never"], min(
        x for k, x in baseline.items() if k.startswith("observe"))) == (37, 20, 26)
    assert (weighted["immediate"], weighted["never"], min(
        x for k, x in weighted.items() if k.startswith("observe"))) == (7, 9, 6)
    return {
        "status": "SYNTHETIC_NOT_NATIVE",
        "equal_weight_cost_grid": {"cases": 648, "observation_strictly_better": strict},
        "original_assumptions": {"patch": 3, "mismatch": 4, "observe": 2, "costs": baseline},
        "prevalence_counterexample": {
            "weights": {"transient": 1, "persistent": 2, "delayed": 1},
            "patch": 0, "mismatch": 1, "observe": 0, "costs": weighted
        },
        "interpretation": "Third observation has no strict value on the equal-weight three-family grid, but can be useful when persistent shocks are more common and observation is free. Neither result predicts native CIL costs.",
        "native_falsification": "Freeze unseen shock lengths and prevalence before running original Rust/CIL PID1, measure all charged instructions and truthful tasks against same-source immediate, never, masked-feedback, inverted-feedback and patch-suppressed arms.",
        "nonclaims": ["No original Rust/CIL world executed", "Researcher-selected families", "No B3/B4 milestone"]
    }

if __name__ == "__main__":
    print(json.dumps(audit(), indent=2, sort_keys=True))
