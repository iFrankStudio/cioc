#!/usr/bin/env python3
"""C1 post-hoc ORIGINAL-NATIVE summary audit of paid mutation reuse.

Reads the C1 Rust-produced JSON summary; CI separately runs the full original
CIL instruction replay before this diagnostic. Does NOT generate a new cohort,
estimate a population effect, or prove endogenous rule invention.
"""
import json
import sys
from collections import Counter, defaultdict
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
MANIFEST = ROOT / "studies/CL1B-G2-PC28-NATIVE-C1-frozen-original-native-184-worlds-v1.json"
SOURCES = ("G2_on", "G2_pc28_suppressed", "G2_placebo180", "G2_never")


def audit(summary_path):
    frozen = json.loads(MANIFEST.read_text(encoding="utf-8"))
    native = json.loads(Path(summary_path).read_text(encoding="utf-8"))
    assert frozen["status"] == "PRE_NATIVE_FROZEN"
    assert native["study"] == frozen["study"] == "CL1B-G2-PC28-NATIVE-C1"
    assert native["status"] == "ORIGINAL_NATIVE_SOURCE_IDENTICAL_PC28_CAUSAL"
    assert native["total_original_native_worlds"] == 184
    records = native["truthful_records"]
    assert len(records) == len(frozen["arms"]) == 180
    assert len(native["adversarial_records"]) == len(frozen["controls"]) == 4
    assert all(a["name"] == b["name"] for a, b in zip(records, frozen["arms"]))
    assert all(a["name"] == b["name"] for a, b in zip(native["adversarial_records"], frozen["controls"]))

    counts = Counter()
    by_family = defaultdict(lambda: {"pairs": 0, "attempted": 0, "changed": 0,
                                     "later_read": 0, "branch_flip": 0,
                                     "paired_off_minus_on_cost": 0})
    pairs = []
    paid = sum(x["steps"] for x in records + native["adversarial_records"])
    assert paid == native["total_private_paid_cil"] == 306432

    for idx in range(45):
        on, off, placebo, never = records[4*idx:4*idx+4]
        assert tuple(x["source"] for x in (on, off, placebo, never)) == SOURCES
        assert all(x["completed_stages"] == 24 and x["truthful_matches"] == 24
                   and x["claim"] == 1 for x in (on, off, placebo, never))
        assert on["goals"] == off["goals"] == placebo["goals"] == never["goals"]
        assert on["regime"] == off["regime"] == placebo["regime"] == never["regime"]
        assert on["steps"] == placebo["steps"]
        assert on["services"] == placebo["services"]
        assert on["decisions"] == placebo["decisions"]
        attempts = len(on["parameter_patches"])
        assert attempts == len(off["suppressed_pc28"])
        assert attempts == len(off["parameter_patches"])
        assert all(p["suppressed"] for p in off["parameter_patches"])
        assert all(not p["suppressed"] for p in on["parameter_patches"])
        actual_changes = sum(bool(p["changed"]) for p in on["parameter_patches"])
        later_reads = len(on["later_mutated_reads"])
        assert later_reads <= actual_changes
        delta = off["steps"] - on["steps"]

        on_decisions = {d["step"]: d["next"] for d in on["decisions"]}
        off_decisions = {d["step"]: d["next"] for d in off["decisions"]}
        branch_flips = [step for step in on_decisions.keys() & off_decisions.keys()
                        if on_decisions[step] != off_decisions[step]]
        # A differing decision at the same paid step is a positive local
        # witness. Divergent later traces cannot be force-aligned by index.
        assert not branch_flips or (actual_changes and later_reads)
        group = ("never_reached_pc28" if not attempts else
                 "pc28_no_actual_change" if not actual_changes else
                 "changed_but_never_reused" if not later_reads else
                 "changed_and_later_reused")
        counts[group] += 1
        counts["pc28_paid_attempts"] += attempts
        counts["actual_patch_events"] += actual_changes
        counts["mutated_const_read_events"] += later_reads
        counts["cost_positive"] += delta > 0
        counts["cost_negative"] += delta < 0
        counts["cost_equal"] += delta == 0
        counts["identical_step_decision_branch_flips"] += len(branch_flips)

        family = by_family[on["regime"]]
        family["pairs"] += 1
        family["attempted"] += bool(attempts)
        family["changed"] += bool(actual_changes)
        family["later_read"] += bool(later_reads)
        family["branch_flip"] += bool(branch_flips)
        family["paired_off_minus_on_cost"] += delta
        pairs.append({
            "case": on["name"].removesuffix("_G2_on"),
            "regime": on["regime"],
            "class": group,
            "paid_pc28_attempts": attempts,
            "effective_parameter_changes": actual_changes,
            "later_reads_of_changed_parameter": later_reads,
            "same_step_native_branch_flips": branch_flips,
            "off_minus_on_cost": delta,
            "on_charged_cil": on["steps"], "off_charged_cil": off["steps"],
            "on_real_probes": len(on["services"]),
            "off_real_probes": len(off["services"]),
        })

    assert sum(counts[k] for k in (
        "never_reached_pc28", "pc28_no_actual_change",
        "changed_but_never_reused", "changed_and_later_reused")) == 45
    assert (counts["never_reached_pc28"], counts["pc28_no_actual_change"],
            counts["changed_but_never_reused"], counts["changed_and_later_reused"]) == (19, 10, 14, 2), (
        "original-native reuse funnel changed; inspect raw source and replay"
    )
    assert (counts["pc28_paid_attempts"], counts["actual_patch_events"],
            counts["mutated_const_read_events"]) == (28, 18, 2)
    assert (counts["cost_positive"], counts["cost_negative"], counts["cost_equal"]) == (0, 1, 44)
    assert counts["identical_step_decision_branch_flips"] == 1
    assert sum(p["off_minus_on_cost"] for p in pairs) == native["paired_suppressed_minus_on_cost"] == -131
    assert len([p for p in pairs if p["later_reads_of_changed_parameter"]]) == 2
    assert [p["case"] for p in pairs if p["off_minus_on_cost"]] == [
        "c1_fast_reverting_pulses_a0_d1"
    ]
    assert [p["case"] for p in pairs if p["same_step_native_branch_flips"]] == [
        "c1_fast_reverting_pulses_a0_d1"
    ]
    return {
        "study": "CL1B-G2-PC28-NATIVE-C1",
        "status": "RETROSPECTIVE_ORIGINAL_NATIVE_SUMMARY_REUSE_DIAGNOSTIC",
        "verified_original_native_worlds_in_input": 184,
        "original_native_charged_instructions_in_input": paid,
        "paired_truthful_cases": 45,
        "funnel": dict(counts),
        "families": dict(by_family),
        "pairs": pairs,
        "causal_cost_off_minus_on": -131,
        "nonclaims": [
            "No new native world or fresh task distribution was created.",
            "This diagnostic consumes the Rust native summary; the separate independent "
            "replayer is required to validate its original native traces.",
            "The stage-24 reuse funnel is retrospective and does not prove a "
            "population probability, autonomous algorithm invention, trusted feedback, or B3/B4.",
        ],
    }


def main():
    if len(sys.argv) != 2:
        raise SystemExit("usage: cl1b_g2_c1_native_reuse_window.py NATIVE_SUMMARY.json")
    print(json.dumps(audit(sys.argv[1]), ensure_ascii=False, indent=2))


if __name__ == "__main__":
    main()
