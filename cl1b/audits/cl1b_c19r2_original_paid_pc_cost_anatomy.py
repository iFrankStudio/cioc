#!/usr/bin/env python3
"""CL-1B C19-R2: source-pinned *descriptive* paid-PC cost anatomy.

Read both permanent C19A/B original native ZIPs, independently verify all 548
worlds through the pre-existing auditor, then inspect every truthful main,
same-source PC225-effect-suppressed and different-source C9 trajectory.
This is NOT a new native experiment. PC-index bins are bookkeeping, not causal
ablation: original source nodes <=179 are not necessarily semantically equal.
"""
import json
import sys
from collections import Counter
from pathlib import Path

from cl1b_c19_original_live_source_47_paid_edge_reads_dual_cohort_audit import (
    ARMS, COHORT, co, original, trace,
)

BUCKETS = (
    ("base_PC000_179", 0, 179),
    ("C14_PC180_188", 180, 188),
    ("C15_PC189_193", 189, 193),
    ("C16_PC194_202", 194, 202),
    ("C17_PC203_215", 203, 215),
    ("C18_PC216_228", 216, 228),
    ("C19_PC229_240", 229, 240),
)
assert all(BUCKETS[i][2] + 1 == BUCKETS[i + 1][1] for i in range(len(BUCKETS) - 1))


def paid_histogram(z, tag, trial):
    h = Counter()
    events = trace(z, tag, trial)
    for event in events:
        if event[0] == "cil":
            assert len(event) >= 10, ("incomplete original CIL event", trial["name"])
            assert isinstance(event[2], int) and 0 <= event[2] <= 240
            h[event[2]] += 1
    assert sum(h.values()) == trial["steps"], (
        "charged event count mismatch", trial["name"], sum(h.values()), trial["steps"]
    )
    return h


def bins(hist):
    return {name: sum(hist[i] for i in range(lo, hi + 1)) for name, lo, hi in BUCKETS}


def top_paid(hist, count=12):
    return [{"PC": pc, "paid_hits": n} for pc, n in
            sorted(hist.items(), key=lambda p: (-p[1], p[0]))[:count]]


def audit_cohort(zip_path, cfg):
    z, native, tag = original(zip_path, cfg)
    verified = co(cfg, z, native, tag)
    all_hists = [Counter(), Counter(), Counter()]
    records = []
    for case in native["records"]:
        main, suppressed, *_, fixed = case["arms"]
        assert (main["arm"], suppressed["arm"], fixed["arm"]) == (
            ARMS[0], ARMS[1], ARMS[5]
        )
        assert all(t["feedback"] == "truth" and t["completed_stages"] == 48
                   and t["true_matches"] == 48 for t in (main, suppressed, fixed))
        assert main["goals"] == suppressed["goals"] == fixed["goals"]
        h0, h1, h5 = (paid_histogram(z, tag, trial)
                       for trial in (main, suppressed, fixed))
        assert max(h5, default=0) < 180, "C9 code cannot execute C14-C19 nodes"
        assert max(h0, default=0) < 241 and max(h1, default=0) < 241
        for out, histogram in zip(all_hists, (h0, h1, h5)):
            out.update(histogram)
        b0, b1, b5 = (bins(h) for h in (h0, h1, h5))
        no_live = main["paid_live_source_const_executed"] == 0
        assert main["paid_live_source_const_executed"] == main["paid_live_source_edge_restored"]
        assert sum(b0.values()) == main["steps"]
        assert sum(b1.values()) == suppressed["steps"]
        assert b5["base_PC000_179"] == fixed["steps"]
        records.append({
            "case": case["name"],
            "charged_live_source_reads": main["paid_live_source_const_executed"],
            "no_paid_live_source_read": no_live,
            "main_CIL": main["steps"],
            "same_source_PC225_suppressed_CIL": suppressed["steps"],
            "different_source_C9_CIL": fixed["steps"],
            "main_minus_C9": main["steps"] - fixed["steps"],
            "main_minus_PC225_suppressed": main["steps"] - suppressed["steps"],
            "main_base_PC000_179_minus_C9_base_PC000_179":
                b0["base_PC000_179"] - fixed["steps"],
            "main_appended_PC180_240": main["steps"] - b0["base_PC000_179"],
            "main_bins": b0,
            "PC225_suppressed_bins": b1,
            "C9_bins": b5,
        })
    assert len(records) == len({r["case"] for r in records}) == 45
    totals = [sum(h.values()) for h in all_hists]
    assert totals == [cfg["fee_main"], cfg["fee_same_suppressed"], cfg["fee_c9"]]
    assert verified["original_source_zip_sha256"] == cfg["sha"]
    assert verified["paid_temporary_edge_armed"] == cfg["observed"]
    assert verified["same_source_first_native_effect_has_identical_paid_prefix"]
    main_bins = bins(all_hists[0])
    suppression_bins = bins(all_hists[1])
    fixed_bins = bins(all_hists[2])
    zero = [r for r in records if r["no_paid_live_source_read"]]
    observed = [r for r in records if not r["no_paid_live_source_read"]]
    assert len(zero) + len(observed) == 45
    assert sum(r["charged_live_source_reads"] for r in records) == cfg["observed"]
    assert sum(r["main_minus_C9"] for r in records) == cfg["fee_main"] - cfg["fee_c9"]
    assert sum(r["main_appended_PC180_240"] for r in records) == totals[0] - main_bins["base_PC000_179"]
    by_exposure = {}
    for key, group in (("zero_paid_source_reads", zero), ("at_least_one_paid_source_read", observed)):
        by_exposure[key] = {
            "worlds": len(group),
            "true_stages_each_world": 48,
            "main_CIL": sum(r["main_CIL"] for r in group),
            "C9_CIL": sum(r["different_source_C9_CIL"] for r in group),
            "main_excess_vs_C9_CIL": sum(r["main_minus_C9"] for r in group),
            "paid_appended_PC180_240": sum(r["main_appended_PC180_240"] for r in group),
        }
    return {
        "cohort": "KNOWN_CALIBRATION" if cfg["known"] else "NEW_UNSEEN_HELDOUT",
        "source_zip_sha256": cfg["sha"],
        "tested_source_sha": cfg["tested_sha"],
        "original_native_run": cfg["run_id"],
        "original_native_worlds_independently_reverified": 274,
        "charged_main_CIL": totals[0],
        "charged_same_source_PC225_suppressed_CIL": totals[1],
        "charged_different_source_C9_CIL": totals[2],
        "main_minus_C9_CIL": totals[0] - totals[2],
        "main_paid_PC_index_buckets": main_bins,
        "same_source_PC225_suppressed_PC_index_buckets": suppression_bins,
        "different_source_C9_PC_index_buckets": fixed_bins,
        "main_appended_PC180_240_paid": totals[0] - main_bins["base_PC000_179"],
        "main_base_PC000_179_minus_C9_base_PC000_179":
            main_bins["base_PC000_179"] - totals[2],
        "main_minus_same_source_suppressed_PC_paid_by_bucket":
            {label: main_bins[label] - suppression_bins[label] for label, _, _ in BUCKETS},
        "main_most_executed_pc_indices": top_paid(all_hists[0]),
        "C9_most_executed_pc_indices": top_paid(all_hists[2]),
        "zero_vs_nonzero_source_read_groups": by_exposure,
        "cases": records,
    }


def run(a_zip, b_zip):
    a = audit_cohort(a_zip, COHORT["a"])
    b = audit_cohort(b_zip, COHORT["b"])
    assert a["charged_main_CIL"] + b["charged_main_CIL"] == 399252
    assert a["charged_different_source_C9_CIL"] + b["charged_different_source_C9_CIL"] == 309125
    assert a["main_minus_C9_CIL"] + b["main_minus_C9_CIL"] == 90127
    assert a["charged_same_source_PC225_suppressed_CIL"] + b["charged_same_source_PC225_suppressed_CIL"] == 398309
    return {
        "study": "CL1B-C19R2-ORIGINAL-NATIVE-PAID-PC-COST-ANATOMY",
        "scope": "POSTHOC_DESCRIPTION_OF_90_EXISTING_TRUTHFUL_CASES_AND_548_ORIGINAL_WORLDS",
        "never_claim": "PC-index arithmetic cannot identify causal benefit of deleting instructions; differing C9 graph makes between-source differences descriptive only",
        "total_main_paid_CIL": 399252,
        "total_C9_paid_CIL": 309125,
        "total_main_excess_vs_C9": 90127,
        "total_main_appended_PC180_240":
            a["main_appended_PC180_240_paid"] + b["main_appended_PC180_240_paid"],
        "total_main_base_PC000_179_minus_C9_base_PC000_179":
            a["main_base_PC000_179_minus_C9_base_PC000_179"]
            + b["main_base_PC000_179_minus_C9_base_PC000_179"],
        "known_not_heldout": a,
        "prospective_heldout": b,
        "B3_B4": "NOT_MET",
    }


if __name__ == "__main__":
    assert len(sys.argv) == 4, "usage: anatomy.py C19A.zip C19B.zip report.json"
    report = run(sys.argv[1], sys.argv[2])
    Path(sys.argv[3]).write_text(json.dumps(report, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
    for label in ("known_not_heldout", "prospective_heldout"):
        c = report[label]
        print(json.dumps({
            "cohort": label,
            "charged_main_CIL": c["charged_main_CIL"],
            "charged_C9_CIL": c["charged_different_source_C9_CIL"],
            "main_minus_C9": c["main_minus_C9_CIL"],
            "main_bins": c["main_paid_PC_index_buckets"],
            "PC225_derived_same_source_bucket_delta": c["main_minus_same_source_suppressed_PC_paid_by_bucket"],
            "no_live_source_read": c["zero_vs_nonzero_source_read_groups"]["zero_paid_source_reads"],
            "with_live_source_read": c["zero_vs_nonzero_source_read_groups"]["at_least_one_paid_source_read"],
        }, ensure_ascii=False))
