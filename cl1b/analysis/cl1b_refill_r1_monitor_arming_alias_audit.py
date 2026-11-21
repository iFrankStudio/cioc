#!/usr/bin/env python3
"""Source-pinned CL1B-R1 PC155 arming-time observational-alias audit.

Reads the original 472-world native summary and the original *native* JSONL
traces from evidence run 44906052116, NOT a generated replay or a toy VM.
Only truthful cases with an actual PC174 suppression exposure are eligible.

Usage:
  python3 cl1b/analysis/cl1b_refill_r1_monitor_arming_alias_audit.py \
    --native-json /path/to/cl1b_refill_causal_r1_native_472_worlds.json \
    --trace-dir /path/to/run-records/.../ \
    --output /path/to/alias-audit.json

Without --trace-dir this script FAILS CLOSED: summary-only service-prefix
collisions do not certify exact original native trace equivalence.
"""
import argparse
import json
from collections import defaultdict
from pathlib import Path

RUN_ID = 44906052116
SOURCE_SHA = "10bb882f92644c975ab9d61115ba476dccf73154"
FIRST_MONITOR_ARMING_PC = 155
CAUSAL_REFILL_PC = 174


def native_rows(folder: Path, name: str) -> list:
    path = folder / f"cl1b_refill_causal_r1_original_{name}.jsonl"
    with path.open(encoding="utf-8") as stream:
        return [json.loads(line) for line in stream if line.strip()]


def first_pc(rows: list, pc: int) -> int:
    positions = [i for i, event in enumerate(rows)
                 if event[0] == "cil" and event[2] == pc]
    assert positions, f"PC{pc} absent from original native trace"
    return positions[0]


def first_divergence(left: list, right: list) -> int:
    for index, (a, b) in enumerate(zip(left, right)):
        if a != b:
            return index
    return min(len(left), len(right))


def audit(native: dict, trace_dir: Path) -> dict:
    assert native["study"] == "CL1B-REFILL-CAUSAL-R1"
    assert native["source_sha"] == SOURCE_SHA
    assert native["counts"]["worlds"] == 472
    assert native["counts"]["actual_suppressed_pc174_exposures"] == 99
    assert native["counts"]["total_original_paid_instructions"] == 677122

    groups = defaultdict(list)
    eligible = 0
    for item in native["fresh_records"] + native["exploratory_records"]:
        if not item["actual_suppressed_pc174_exposures"]:
            continue
        eligible += 1
        actor = item["f6_on"]
        ablated = item["f6_selective_refill_disabled"]
        assert actor["source"] == "F6"
        assert actor["feedback"] == ablated["feedback"] == "truth"
        assert actor["completed_stages"] == ablated["completed_stages"] == 24
        armings = [e["step"] for e in actor["edge_edits"]
                   if e["pc"] == FIRST_MONITOR_ARMING_PC]
        assert armings, actor["name"]
        arming = armings[0]
        first_refill = ablated["suppressed_refill"][0]["step"]
        assert arming < first_refill, actor["name"]
        observations = tuple((e["step"], e["stage"], e["guess"], e["feedback"])
                             for e in actor["services"] if e["step"] < arming)
        key = (arming, observations)
        groups[key].append({
            "name": actor["name"],
            "regime": item["regime"],
            "classification": item["classification"],
            "saving": item["same_source_refill_causal_paid_saving"],
            "arming_step": arming,
            "first_refill_step": first_refill,
        })
    assert eligible == 99

    witness_groups = []
    for (arming, observations), items in groups.items():
        positive = [e for e in items if e["saving"] > 0]
        negative = [e for e in items if e["saving"] < 0]
        if not positive or not negative:
            continue
        # The evidence version under audit has exactly two cases per
        # mixed-sign class. Future changes require a new protocol, not
        # silently changing the witness pairing or regret denominator.
        assert len(items) == len(positive) + len(negative) == 2
        p, n = positive[0], negative[0]
        left, right = native_rows(trace_dir, p["name"]), native_rows(trace_dir, n["name"])
        pi, ni = first_pc(left, FIRST_MONITOR_ARMING_PC), first_pc(right, FIRST_MONITOR_ARMING_PC)
        assert pi == ni, (p["name"], n["name"], pi, ni)
        assert left[pi] == right[ni]
        assert left[pi][1] == arming  # charged-CIL index, not trace-row index
        divergence = first_divergence(left, right)
        assert pi < divergence < min(len(left), len(right)), (p["name"], n["name"])
        first_left, first_right = left[divergence], right[divergence]
        assert first_left[0] == first_right[0] == "feedback"
        assert first_left[1] == first_right[1] > arming
        assert first_left[2] == first_right[2]
        assert first_left[6] != first_right[6]  # real visible feedback differs
        assert len(observations) > 0
        regret = min(p["saving"], -n["saving"])
        witness_groups.append({
            "arming_step": arming,
            "observed_services_before_arming": len(observations),
            "positive": p,
            "negative": n,
            "identical_original_native_trace_rows": divergence,
            "arming_trace_row_index": pi,
            "first_divergence_kind": first_left[0],
            "first_divergence_paid_step": first_left[1],
            "first_divergence_task_stage": first_left[2],
            "minimum_regret_for_early_irrevocable_binary_choice": regret,
        })
    witness_groups.sort(key=lambda x: (x["arming_step"], x["positive"]["name"]))
    assert len(groups) == 66 and len(witness_groups) == 7
    assert sum(g["minimum_regret_for_early_irrevocable_binary_choice"]
               for g in witness_groups) == 889
    assert all(g["arming_trace_row_index"] < g["identical_original_native_trace_rows"]
               for g in witness_groups)
    return {
        "status": "VERIFIED_ORIGINAL_NATIVE_TRACE_PREFIX",
        "study": "CL1B-REFILL-CAUSAL-R1-PC155-ALIAS",
        "source_sha": SOURCE_SHA,
        "evidence_run_id": RUN_ID,
        "original_worlds": native["counts"]["worlds"],
        "original_paid_cil_replayed_in_parent_evidence": 677122,
        "truthful_first_pc174_exposed_cases": eligible,
        "distinct_pc155_service_history_classes": len(groups),
        "opposite_sign_alias_classes": len(witness_groups),
        "matched_original_native_trace_witnesses": len(witness_groups),
        "minimum_regret_on_14_selected_cases_for_early_irrevocable_binary_choice":
            sum(g["minimum_regret_for_early_irrevocable_binary_choice"]
                for g in witness_groups),
        "witness_groups": witness_groups,
        "interpretation_limits": [
            "Existing frozen retrospective cases, mostly prior exposed; NOT new heldout evaluation.",
            "Both original-arm continuations retain identical paid monitoring; this compares "
            "future PC174 write-effect permission, not turning monitoring itself off.",
            "Lower bound holds only for an irrevocable arm-time binary choice within these "
            "14 selected cases; a policy that waits for new observations may do better.",
            "No proof that first-PC174 states themselves are aliased or that no online learner exists.",
            "Researcher-only PC174 suppression is not an actor permission or an evolved policy.",
        ],
    }


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--native-json", required=True, type=Path)
    parser.add_argument("--trace-dir", required=True, type=Path)
    parser.add_argument("--output", type=Path)
    args = parser.parse_args()
    native = json.loads(args.native_json.read_text(encoding="utf-8"))
    result = audit(native, args.trace_dir)
    rendered = json.dumps(result, ensure_ascii=False, indent=2) + "\n"
    if args.output:
        args.output.write_text(rendered, encoding="utf-8")
    else:
        print(rendered, end="")


if __name__ == "__main__":
    main()
