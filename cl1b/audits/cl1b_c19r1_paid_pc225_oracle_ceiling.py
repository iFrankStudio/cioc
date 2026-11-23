#!/usr/bin/env python3
"""C19-R1 hindsight-only two-policy PC225 cost envelope.

Reinspect original C19A/B native ZIPs with existing independent 548-world
source-pinned auditor. No new native worlds, no actor-run learning, no
free decision capability bestowed on the organism.
"""
import json
import sys
from pathlib import Path

from cl1b_c19_original_live_source_47_paid_edge_reads_dual_cohort_audit import (
    ARMS, COHORT, co, original,
)


def evaluate(path, cfg):
    z, native, tag = original(path, cfg)
    checked = co(cfg, z, native, tag)
    assert len(native["records"]) == 45
    cases = []
    for case in native["records"]:
        main, suppressed, *_, c9 = case["arms"]
        assert (main["arm"], suppressed["arm"], c9["arm"]) == (ARMS[0], ARMS[1], ARMS[5])
        assert main["feedback"] == suppressed["feedback"] == c9["feedback"] == "truth"
        assert main["goals"] == suppressed["goals"] == c9["goals"]
        # No case may be dropped, and optimizing paid CIL must not trade away stages.
        assert main["completed_stages"] == suppressed["completed_stages"] == c9["completed_stages"] == 48
        x, y, fixed = main["steps"], suppressed["steps"], c9["steps"]
        assert all(isinstance(v, int) and v >= 0 for v in (x, y, fixed))
        cases.append({
            "case": case["name"],
            "main_live_gate_paid_CIL": x,
            "same_source_suppress_PC225_effect_paid_CIL": y,
            "different_source_C9_fixed_paid_CIL": fixed,
            "main_minus_same_source_suppressed": x - y,
            "free_hindsight_choice": "main" if x < y else ("suppressed" if y < x else "tie"),
            "free_hindsight_minimum_paid_CIL": min(x, y),
            "true_completed_stages_each": 48,
        })
    assert len(cases) == len({r["case"] for r in cases}) == 45
    main_total = sum(r["main_live_gate_paid_CIL"] for r in cases)
    suppressed_total = sum(r["same_source_suppress_PC225_effect_paid_CIL"] for r in cases)
    c9_total = sum(r["different_source_C9_fixed_paid_CIL"] for r in cases)
    oracle_total = sum(r["free_hindsight_minimum_paid_CIL"] for r in cases)
    assert (main_total, suppressed_total, c9_total) == (
        cfg["fee_main"], cfg["fee_same_suppressed"], cfg["fee_c9"]
    )
    assert checked["main_CIL"] == main_total
    assert checked["same_source_PC225_effect_suppressed_CIL"] == suppressed_total
    assert checked["same_source_first_native_effect_has_identical_paid_prefix"]
    assert checked["actor_false_equal"] == checked["actor_false_different"] == 0
    assert checked["original_source_zip_sha256"] == cfg["sha"]
    assert sum(r["main_minus_same_source_suppressed"] for r in cases) == main_total - suppressed_total
    assert sum(r["free_hindsight_choice"] != "tie" for r in cases) == 4
    assert oracle_total < min(main_total, suppressed_total)
    return {
        "cohort": "KNOWN_C17_CALIBRATION" if cfg["known"] else "UNSEEN_HELDOUT",
        "original_zip_sha256": cfg["sha"],
        "tested_original_sha": cfg["tested_sha"],
        "original_native_run": cfg["run_id"],
        "original_native_worlds": 274,
        "strict_same_source_truthful_pairs": 45,
        "main_paid_CIL": main_total,
        "same_source_PC225_suppressed_paid_CIL": suppressed_total,
        "different_source_C9_paid_CIL": c9_total,
        "zero_cost_hindsight_oracle_paid_CIL": oracle_total,
        "oracle_saves_vs_main_CIL": main_total - oracle_total,
        "oracle_still_more_expensive_than_C9_CIL": oracle_total - c9_total,
        "cases_main_cheaper": sum(r["free_hindsight_choice"] == "main" for r in cases),
        "cases_suppression_cheaper": sum(r["free_hindsight_choice"] == "suppressed" for r in cases),
        "cases_equal": sum(r["free_hindsight_choice"] == "tie" for r in cases),
        "true_stages_each_arm": 45 * 48,
        "cases": cases,
    }


def run(a_zip, b_zip):
    a = evaluate(a_zip, COHORT["a"])
    b = evaluate(b_zip, COHORT["b"])
    assert (a["zero_cost_hindsight_oracle_paid_CIL"], b["zero_cost_hindsight_oracle_paid_CIL"]) == (190037, 206111)
    assert (a["oracle_still_more_expensive_than_C9_CIL"], b["oracle_still_more_expensive_than_C9_CIL"]) == (28008, 59015)
    oracle = a["zero_cost_hindsight_oracle_paid_CIL"] + b["zero_cost_hindsight_oracle_paid_CIL"]
    fixed = a["different_source_C9_paid_CIL"] + b["different_source_C9_paid_CIL"]
    assert (oracle, fixed) == (396148, 309125)
    return {
        "study": "CL1B-C19R1-RETROSPECTIVE-PC225-FREE-ORACLE-CEILING",
        "classification": "RETROSPECTIVE_REANALYSIS_NO_NEW_NATIVE_TRIAL",
        "input_integrity": "TWO_SHA256_PINNED_ORIGINAL_NATIVE_274_ZIPS",
        "source_matched_comparison": "ONLY_LIVE_PC225_EFFECT_VS_SAME_241_NODE_SOURCE_PC225_EFFECT_SUPPRESSION",
        "oracle_assumption": "UNREALISTIC_PER_TASK_CLAIRVOYANCE_ZERO_DECISION_COST_NO_ADAPTIVE_ACTOR",
        "fixed_C9_comparator": "DIFFERENT_SOURCE_ECONOMIC_BENCHMARK_NOT_SINGLE_FACTOR_CAUSAL_CONTROL",
        "combined_oracle_paid_CIL": oracle,
        "combined_different_source_C9_paid_CIL": fixed,
        "combined_oracle_excess_vs_C9_CIL": oracle - fixed,
        "combined_oracle_saving_vs_executed_main_CIL": (
            a["main_paid_CIL"] + b["main_paid_CIL"] - oracle
        ),
        "known_not_heldout": a,
        "prospective_heldout": b,
        "B3_B4": "NOT_MET",
        "inference_limit": "Only the PC225 gate on two frozen cohorts: even a free hindsight selector cannot erase the C9 cost gap. Not a general bound on all programs, tasks, or future evolution.",
    }


if __name__ == "__main__":
    assert len(sys.argv) == 4, "usage: oracle.py C19A.zip C19B.zip output.json"
    report = run(sys.argv[1], sys.argv[2])
    Path(sys.argv[3]).write_text(json.dumps(report, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
    print(json.dumps({k: v for k, v in report.items() if k not in ("known_not_heldout", "prospective_heldout")}, indent=2, ensure_ascii=False))
