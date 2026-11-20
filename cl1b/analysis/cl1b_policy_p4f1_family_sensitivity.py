#!/usr/bin/env python3
"""Independent aggregate-level sensitivity audit of published P4F1 native paired totals.

This is NOT native execution or per-world replay. Nine-pair family aggregates
are not nine independent measurements available for resampling.
"""
import json

FAMILIES = [
    ("stable", 0, 0, False),
    ("switch", -432, 0, False),
    ("alternate", 753, 63, False),
    ("scrambled", 1648, 114, False),
    ("revert", -216, 0, False),
    ("rare_shock", -147, 0, False),
    ("heldout_late_switch", 969, 63, True),
    ("heldout_two_shocks", -672, -9, True),
]

def audit():
    assert len(FAMILIES) == 8
    assert sum(row[1] for row in FAMILIES) == 1903
    assert sum(row[2] for row in FAMILIES) == 231
    savings = sorted(row[1] for row in FAMILIES)
    heldout = [row for row in FAMILIES if row[3]]
    leave_one_out = [
        {"omitted": name, "remaining_paid_saving": 1903 - paid,
         "remaining_probes_saved": 231 - probes}
        for name, paid, probes, _ in FAMILIES
    ]
    # Exact sign-flip enumeration at family level. Regimes were purposively
    # chosen, hence this is a diagnostic and must NOT be reported as a p-value.
    magnitudes = [abs(v) for v in savings if v != 0]
    sign_flip_tail = sum(
        sum(v if mask & (1 << i) else -v for i, v in enumerate(magnitudes)) >= 1903
        for mask in range(1 << len(magnitudes))
    )
    return {
        "scope": "secondary analysis of previously verified family aggregates, no new Rust/CIL execution",
        "family_count": 8, "pairs_per_family": 9,
        "paid_saving": 1903, "probes_saved": 231,
        "positive_negative_zero_families": [
            sum(v > 0 for v in savings), sum(v < 0 for v in savings), sum(v == 0 for v in savings)
        ],
        "median_family_paid_saving": (savings[3] + savings[4]) / 2,
        "new_heldout_paid_saving": sum(row[1] for row in heldout),
        "new_heldout_probes_saved": sum(row[2] for row in heldout),
        "leave_one_family_out": leave_one_out,
        "all_leave_one_out_positive": all(row["remaining_paid_saving"] > 0 for row in leave_one_out),
        "family_sign_flip_descriptive_tail": {
            "numerator": sign_flip_tail, "denominator": 1 << len(magnitudes),
            "not_a_p_value": True,
            "reason": "nonrandom, nonexchangeable task families"
        },
        "limitation": "published aggregates do not expose nine individual pair effects; no confidence interval or population generalization"
    }

if __name__ == "__main__":
    print(json.dumps(audit(), ensure_ascii=False, indent=2, sort_keys=True))
