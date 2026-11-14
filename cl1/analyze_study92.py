#!/usr/bin/env python3
"""Study92 original-only retrospective diagnosis, zero new native worlds.

Re-verify original Study91 source genotypes, 1536 records and 27648 hashes
before any classification. No seed remapping, output-based selection or re-run.
"""
import argparse
import json
import sqlite3
from collections import Counter
from pathlib import Path

import analyze_study91 as stage
import analyze_study82 as codegen

ORIGINAL_SHA = "83515e9231c73e00cc0fb727b5414b90d4c60c24"
ORIGINAL_CI = 43598229363
ARMS = (
    "intact", "sham", "lesion-native",
    "lesion-edit-disabled", "lesion-host-restore", "lesion-draw-ablated",
)


def require(cond, message):
    if not cond:
        raise ValueError(message)


def source_run(conn, seed, source, arm):
    label = f"seed={seed}/source={source}/{arm}"
    rows = conn.execute(
        "SELECT initial_json,final_json,actions_json,attempts_json,source_revision "
        "FROM epoch_runs WHERE condition=?", (label,)
    ).fetchall()
    require(len(rows) == 1, "missing exact original source/arm")
    init_raw, end_raw, actions_raw, counts_raw, revision = rows[0]
    require(revision == ORIGINAL_SHA, "not source-pinned original Study91 data")
    initial, end = json.loads(init_raw), json.loads(end_raw)
    actions, charges = json.loads(actions_raw), json.loads(counts_raw)
    code = codegen.expected_code(seed)
    prefix = stage.trace_native(initial, code, source)
    trace = stage.trace_native(end, code, source)
    events = end["events"][len(initial["events"]):]
    natives = [e for e in events if e["origin"] == {"Internal": 1}]
    patch_attempts = [
        e for e in natives if codegen.variant(code[e["pc"]])[0] == "PatchNext"
    ]
    edits = [
        e["kind"]["ExecutableEdgePatched"]
        for e in natives
        if isinstance(e["kind"], dict) and "ExecutableEdgePatched" in e["kind"]
    ]
    patched_pc8_from_detour = [
        edit for edit in edits if edit["node"] == 8 and edit["before"] == 0
    ]
    return {
        "initial": initial,
        "final": end,
        "charges": charges,
        "actions": actions,
        "trace": trace,
        "pre_attempts": prefix["attempts"],
        "pre_witnesses": stage.pre_exposure(initial, code, source),
        "post_native_attempts": len(natives),
        "post_native_pc8_attempts": sum(e["pc"] == 8 for e in natives),
        "post_native_non_pc8_attempts": sum(e["pc"] != 8 for e in natives),
        "post_native_patchnext_attempts": len(patch_attempts),
        "post_native_effective_pc8_edits": edits,
        "post_native_changes_from_damaged_pc8_edge": patched_pc8_from_detour,
        "post_native_traps": sum(
            isinstance(e["kind"], dict) and "Trapped" in e["kind"]
            for e in natives
        ),
        "post_native_draw_deferred": sum(
            isinstance(e["kind"], dict) and "DrawDeferred" in e["kind"]
            for e in natives
        ),
    }


def per_seed(conn, seed):
    worlds = {
        (source, arm): source_run(conn, seed, source, arm)
        for source in (3, 8) for arm in ARMS
    }
    baseline = worlds[(3, "intact")]["pre_witnesses"] & worlds[(8, "intact")]["pre_witnesses"]
    require(bool(baseline), "registered live exposed E has no paired prefork source witnesses")
    early = {
        arm: {
            source: stage.post_window_writes(worlds[(source, arm)], 0, 3)
            for source in (3, 8)
        }
        for arm in ARMS
    }
    class_per_witness = {}
    for witness in sorted(baseline):
        sham2 = witness in early["sham"][3] and witness in early["sham"][8]
        lesion_low = witness in early["lesion-native"][3]
        lesion_high = witness in early["lesion-native"][8]
        if not sham2:
            label = "sham_function_dropped_out_before_loss_window"
        elif lesion_low or lesion_high:
            label = "detour_failed_to_interrupt_original_function_in_both_sources"
        else:
            label = "genuine_early_function_loss"
        class_per_witness[str(witness)] = {
            "classification": label,
            "sham_two_source": sham2,
            "lesion_low": lesion_low,
            "lesion_high": lesion_high,
        }
    groups = {}
    for arm in ARMS:
        groups[arm] = {}
        for source in (3, 8):
            run = worlds[(source, arm)]
            groups[arm][str(source)] = {
                "actual_original_prefork_native_attempts": run["pre_attempts"],
                "post_native_attempts": run["post_native_attempts"],
                "post_native_pc8_attempts": run["post_native_pc8_attempts"],
                "post_native_non_pc8_attempts": run["post_native_non_pc8_attempts"],
                "native_patchnext_attempts": run["post_native_patchnext_attempts"],
                "native_effective_edge_edits": run["post_native_effective_pc8_edits"],
                "native_actual_pc8_detour_restorations": run["post_native_changes_from_damaged_pc8_edge"],
                "native_traps": run["post_native_traps"],
                "native_draw_deferred": run["post_native_draw_deferred"],
                "early_pretreatment_witnesses": sorted(
                    run["pre_witnesses"] & early[arm][source]
                ),
                "late4_9_pretreatment_witnesses": sorted(
                    run["pre_witnesses"] & stage.post_window_writes(run, 4, 9)
                ),
                "late10_17_pretreatment_witnesses": sorted(
                    run["pre_witnesses"] & stage.post_window_writes(run, 10, 17)
                ),
            }
    reachable_both = all(
        worlds[(source, "lesion-native")]["post_native_non_pc8_attempts"] > 0
        for source in (3, 8)
    )
    patch_both = all(
        worlds[(source, "lesion-native")]["post_native_patchnext_attempts"] > 0
        for source in (3, 8)
    )
    return {
        "seed": seed,
        "original_first_live_witnesses": sorted(baseline),
        "true_post_lesion_actor_route_reachable_both_sources": reachable_both,
        "actual_native_patchnext_attempted_both_sources": patch_both,
        "witness_classification": class_per_witness,
        "original_matched_arm_native_event_diagnostics": groups,
    }


def audit(sqlite):
    original = stage.auditor(sqlite, ORIGINAL_SHA)
    require(original["original_distinct_genotypes"] == 128
            and original["original_matched_worlds"] == 1536
            and original["verified_original_postfork_epoch_sha256"] == 27648,
            "original Study91 world/hash identity mismatch")
    require((
        original["pre_fork_exposed_E"],
        original["effective_lesion_L"],
        original["explicit_host_restoration_controls"],
        original["strict_native_repair_R"],
    ) == (2, 0, 0, 0), "original registered Study91 outcomes differ")
    seeds = [
        item["seed"] for item in original["seed_gate_outcomes"]
        if item["eligible_E"]
    ]
    require(len(seeds) == 2, "not exactly the two original pretrigger-exposed seeds")
    with sqlite3.connect(sqlite) as conn:
        details = [per_seed(conn, seed) for seed in seeds]
    reasons = Counter(
        row["classification"]
        for item in details for row in item["witness_classification"].values()
    )
    return {
        "study": 92,
        "status": "RETROSPECTIVE_ORIGINAL_ONLY_ZERO_NEW_WORLDS",
        "original_ci_run": ORIGINAL_CI,
        "original_revision": ORIGINAL_SHA,
        "reverified_original_genotypes": 128,
        "reverified_original_worlds": 1536,
        "reverified_original_sha256_epochs": 27648,
        "independent_new_genotypes": 0,
        "original_E": 2,
        "original_L": 0,
        "original_H": 0,
        "original_R": 0,
        "paired_actor_route_reachable_A": sum(
            item["true_post_lesion_actor_route_reachable_both_sources"] for item in details
        ),
        "paired_actual_native_patchnext_opportunity": sum(
            item["actual_native_patchnext_attempted_both_sources"] for item in details
        ),
        "pre_witness_mechanism_classifications": dict(reasons),
        "original_two_E_genome_details": details,
        "cl1_closed": False,
    }


def main():
    p = argparse.ArgumentParser()
    p.add_argument("sqlite", type=Path)
    p.add_argument("--json", type=Path, required=True)
    args = p.parse_args()
    result = audit(args.sqlite)
    args.json.write_text(json.dumps(result, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    print(json.dumps({
        key: result[key]
        for key in (
            "study", "original_E", "original_L", "original_H", "original_R",
            "paired_actor_route_reachable_A",
            "paired_actual_native_patchnext_opportunity",
            "pre_witness_mechanism_classifications",
        )
    }, sort_keys=True))


if __name__ == "__main__":
    main()
