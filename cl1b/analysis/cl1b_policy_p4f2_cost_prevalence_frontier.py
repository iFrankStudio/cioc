#!/usr/bin/env python3
"""P4F2 synthetic cost frontier with explicit third-task probe charge.

This is a finite counterfactual model, NOT original Rust/CIL evidence.
The previous revision incorrectly double-counted the third task mismatch:
the sum over t>=2 already includes the probe task. This version separates
ordinary service (shared) from a genuinely extra diagnostic task (optional).
This is a synthetic model, not native CIL evidence.
"""
import itertools
import json

FAMILIES = {
    "transient": (0, 0, 1, 1, 1, 1),
    "persistent": (0, 0, 0, 0, 0, 0),
    "delayed": (0, 0, 0, 1, 1, 1),
}


def loss(seq, patch, mismatch, observe, restore_at, charge_probe):
    # Decisions start after the common first two failed tasks.
    # Observation at t=2 must actually run the OLD policy to yield its bit.
    # The normal t=2 task is ALREADY included in the sum below.
    # charge_probe models a distinct EXTRA diagnostic attempt, not that task.
    return (
        observe
        + (patch if restore_at is not None else 0)
        + (mismatch * int(seq[2] == 0) if charge_probe else 0)
        + sum(
            mismatch * (
                int(bit == 1) if restore_at is not None and t >= restore_at
                else int(bit == 0)
            )
            for t, bit in enumerate(seq)
            if t >= 2
        )
    )


def totals(patch, mismatch, observe, weights, charge_probe):
    out = {}
    for mode, choices in [("immediate", ()), ("never", ())] + [
        ("observe", b) for b in itertools.product((False, True), repeat=2)
    ]:
        name = mode if mode != "observe" else "observe_if0%d_if1%d" % choices
        out[name] = sum(
            weights[n] * loss(
                seq, patch, mismatch,
                observe if mode == "observe" else 0,
                2 if mode == "immediate" else None if mode == "never"
                else 3 if choices[seq[2]] else None,
                charge_probe and mode == "observe",
            )
            for n, seq in FAMILIES.items()
        )
    return out


def minimum_observe(costs):
    return min(v for k, v in costs.items() if k.startswith("observe"))


def audit():
    assert len({v[:2] for v in FAMILIES.values()}) == 1
    assert FAMILIES["persistent"][:3] == FAMILIES["delayed"][:3]
    equal = dict.fromkeys(FAMILIES, 1)
    weighted = {"transient": 1, "persistent": 2, "delayed": 1}
    old = totals(0, 1, 0, weighted, False)
    corrected = totals(0, 1, 0, weighted, True)
    assert (old["immediate"], old["never"], minimum_observe(old)) == (7, 9, 6)
    assert (corrected["immediate"], corrected["never"], minimum_observe(corrected)) == (7, 9, 9)
    old_baseline = totals(3, 4, 2, equal, False)
    corrected_baseline = totals(3, 4, 2, equal, True)
    assert (old_baseline["immediate"], old_baseline["never"], minimum_observe(old_baseline)) == (37, 20, 26)
    assert (corrected_baseline["immediate"], corrected_baseline["never"], minimum_observe(corrected_baseline)) == (37, 20, 34)
    grid = list(itertools.product(range(9), range(1, 9), range(9)))
    strict = sum(
        minimum_observe(v) < min(v["immediate"], v["never"])
        for p, m, o in grid
        for v in [totals(p, m, o, equal, True)]
    )
    assert len(grid) == 648 and strict == 0
    return {
        "status": "SYNTHETIC_NOT_NATIVE",
        "correction": "The ordinary third task is already counted in every arm; the former extra charge is valid only if a separate diagnostic task is performed. The favorable counterexample survives with shared service and disappears under the stipulated extra diagnostic charge.",
        "shared_service_weighted": {"immediate": 7, "never": 9, "best_observe": 6},
        "extra_diagnostic_weighted": {"immediate": 7, "never": 9, "best_observe": 9},
        "shared_service_equal_costs": {"immediate": 37, "never": 20, "best_observe": 26},
        "extra_diagnostic_equal_costs": {"immediate": 37, "never": 20, "best_observe": 34},
        "extra_diagnostic_equal_weight_grid": {"cases": len(grid), "observe_strictly_better": strict},
        "limitations": [
            "Synthetic bit sequences and assumed costs, not native CIL traces",
            "The t=2 normal task mismatch is already included in all arms; adding its cost again double-counts unless a distinct extra attempt actually occurs",
            "The extra diagnostic case is a hypothetical distinct attempt with the same bit; native implementation must demonstrate that this attempt exists",
            "No B3/B4 or open-ended evolution claim",
        ],
        "native_falsification": "Run same-source original Rust/CIL matched shock worlds and account for incremental probe attempts, information access, patches, and all resource debits. Compare immediate, never, masked, inverted and patch-suppressed arms.",
    }


if __name__ == "__main__":
    print(json.dumps(audit(), indent=2, sort_keys=True))
