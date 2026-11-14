#!/usr/bin/env python3
"""Study83: READ-ONLY retrospective paired code-edit functional causality analysis.

Study82 is source-pinned, verified here first, and NOT counted as new ecology.
"""
import argparse
import bisect
import json
import math
import sqlite3
from collections import Counter
from pathlib import Path

import analyze_study82 as original

ORIGINAL_REVISION = "1819d0f961e9e70ba78df9bf60def22f7451f243"
LOW_ON = "source-low-native-edit"
HIGH_ON = "source-high-native-edit"
LOW_OFF = "source-low-edit-disabled"
HIGH_OFF = "source-high-edit-disabled"
ARMS = (LOW_ON, HIGH_ON, LOW_OFF, HIGH_OFF)


def require(condition, message):
    if not condition:
        raise ValueError(message)


def native_events(state):
    return [e for e in state["events"] if e["origin"] == {"Internal": 1}]


def epoch_of_attempt(attempt, charges):
    cumulative = []
    total = 0
    for count in charges:
        total += count
        cumulative.append(total)
    require(1 <= attempt <= total, "invalid charged native attempt ordinal")
    return bisect.bisect_left(cumulative, attempt)


def source_w2(low, high, filter_fn=lambda w: True):
    left = {
        (write["pc"], write["offset"])
        for write in low["stats"]["writes"]
        if write["source_coefficient"] == 1 and filter_fn(write)
    }
    right = {
        (write["pc"], write["offset"])
        for write in high["stats"]["writes"]
        if write["source_coefficient"] == 1 and filter_fn(write)
    }
    return left & right


def late_w2(low, high):
    eligible = None
    for start, end in ((12, 17), (18, 23)):
        lset = source_w2(
            low, high,
            lambda write: start <= epoch_of_attempt(
                write["at_native_attempt"], low["charges"] if write in low["stats"]["writes"] else high["charges"]
            ) <= end,
        )
        # Rebuild each input's own source-to-writer windows separately;
        # do not use equality of structurally identical dicts for arm identity.
        lo = {
            (w["pc"], w["offset"])
            for w in low["stats"]["writes"]
            if w["source_coefficient"] == 1
            and start <= epoch_of_attempt(w["at_native_attempt"], low["charges"]) <= end
        }
        hi = {
            (w["pc"], w["offset"])
            for w in high["stats"]["writes"]
            if w["source_coefficient"] == 1
            and start <= epoch_of_attempt(w["at_native_attempt"], high["charges"]) <= end
        }
        common = lo & hi
        eligible = common if eligible is None else eligible & common
    return eligible if eligible is not None else set()


def causal_native_pc_divergence(enabled, disabled):
    on = native_events(enabled["state"])
    off = native_events(disabled["state"])
    on_pcs = [e["pc"] for e in on]
    off_pcs = [e["pc"] for e in off]
    common = 0
    for pc_on, pc_off in zip(on_pcs, off_pcs):
        if pc_on != pc_off:
            break
        common += 1
    if common == len(on_pcs) == len(off_pcs):
        return {"diverged": False, "changed_edge_preceded": False, "last_shared_pc8": False}
    divergence = common + 1
    preceding_edits = any(
        original.value_kind(e, "ExecutableEdgePatched") is not None
        for e in on[:common]
    )
    common_after_pc8 = common > 0 and on_pcs[common - 1] == 8 and off_pcs[common - 1] == 8
    require(
        preceding_edits and common_after_pc8,
        f"unattributed first PC divergence: ordinal={divergence}, last common={on_pcs[common-1] if common else None}",
    )
    return {
        "diverged": True,
        "at_native_ordinal": divergence,
        "changed_edge_preceded": preceding_edits,
        "last_shared_pc8": common_after_pc8,
        "on_next_pc": on_pcs[common] if common < len(on_pcs) else None,
        "off_next_pc": off_pcs[common] if common < len(off_pcs) else None,
    }


def paired_bucket(on, off):
    return (
        "both" if on and off else
        "enabled_only" if on else
        "disabled_only" if off else "neither"
    )


def exact_two_sided_discordant(enabled_only, disabled_only):
    n = enabled_only + disabled_only
    if not n:
        return 1.0
    tail = sum(math.comb(n, k) for k in range(min(enabled_only, disabled_only) + 1))
    return min(1.0, 2.0 * tail / (2 ** n))


def audit(path):
    # Re-run the original completely independent source/genome, SHA and native
    # trace auditor before interpreting any retrospective comparison.
    original_summary = original.audit(path, ORIGINAL_REVISION)
    require(original_summary["original_genotypes"] == 128
            and original_summary["original_worlds"] == 512
            and original_summary["verified_original_epoch_sha256"] == 12288,
            "unverified original Study82 population or snapshots")
    connection = sqlite3.connect(path)
    rows = connection.execute(
        "SELECT condition,initial_json,attempts_json,final_json FROM epoch_runs"
    ).fetchall()
    records = {}
    for label, init_raw, charges_raw, final_raw in rows:
        seed_part, arm = label.removeprefix("seed=").split("/", 1)
        seed = int(seed_part)
        initial = json.loads(init_raw)
        state = json.loads(final_raw)
        source = 3 if "-low-" in arm else 8
        records[(seed, arm)] = {
            "initial": initial,
            "state": state,
            "charges": json.loads(charges_raw),
            "stats": original.audit_native_events(
                state, initial["processes"]["1"]["code"]["nodes"], source
            ),
        }
    require(len(records) == 512, "incomplete original matched Study82 worlds")
    counts = Counter()
    w2_buckets = Counter()
    late_buckets = Counter()
    causal_pc = Counter()
    seed_results = []
    for seed in range(5_500_000, 5_500_128):
        low_on = records[(seed, LOW_ON)]
        high_on = records[(seed, HIGH_ON)]
        low_off = records[(seed, LOW_OFF)]
        high_off = records[(seed, HIGH_OFF)]
        on_w2 = source_w2(low_on, high_on)
        off_w2 = source_w2(low_off, high_off)
        on_late = late_w2(low_on, high_on)
        off_late = late_w2(low_off, high_off)
        bucket = paired_bucket(bool(on_w2), bool(off_w2))
        late_bucket = paired_bucket(bool(on_late), bool(off_late))
        w2_buckets[bucket] += 1
        late_buckets[late_bucket] += 1
        for name, on, off in (
            ("low", low_on, low_off),
            ("high", high_on, high_off),
        ):
            diff = causal_native_pc_divergence(on, off)
            if diff["diverged"]:
                causal_pc[f"{name}_diverged_seeds"] += 1
            if on["stats"]["effective_edits"]:
                counts[f"{name}_native_effective_edit_seeds"] += 1
            if on["stats"]["writes"] != off["stats"]["writes"]:
                counts[f"{name}_different_native_output_trace_seeds"] += 1
        if on_w2:
            counts["source_W2_enabled"] += 1
        if off_w2:
            counts["source_W2_disabled"] += 1
        if on_late:
            counts["late_W2_enabled"] += 1
        if off_late:
            counts["late_W2_disabled"] += 1
        if on_w2 and all(
            any(
                (w["pc"], w["offset"]) in on_w2
                and w["draws_before"] > 0
                and w["at_native_attempt"] > 64
                for w in run["stats"]["writes"]
            )
            for run in (low_on, high_on)
        ):
            counts["enabled_W2_actual_draw_after_64"] += 1
        seed_results.append({
            "seed": seed,
            "w2_edited": bool(on_w2),
            "w2_edit_disabled": bool(off_w2),
            "w2_bucket": bucket,
            "late_w2_edited": bool(on_late),
            "late_w2_edit_disabled": bool(off_late),
            "late_bucket": late_bucket,
            "low_native_effective_edits": len(low_on["stats"]["effective_edits"]),
            "high_native_effective_edits": len(high_on["stats"]["effective_edits"]),
        })
    require(sum(w2_buckets.values()) == 128
            and sum(late_buckets.values()) == 128,
            "invalid complete fixed-genotype paired denominator")
    require(counts["source_W2_enabled"] == 17,
            "retrospective enabled W2 no longer matches archived original Study82")
    return {
        "study": 83,
        "status": "RETROSPECTIVE_NO_NEW_ECOLOGY",
        "original_source_revision": ORIGINAL_REVISION,
        "original_archive_run_id": 43592985831,
        "original_genotypes_reverified": 128,
        "original_worlds_reverified": 512,
        "original_sha256_epoch_snapshots_reverified": 12288,
        "new_ecological_worlds": 0,
        "w2_paired": dict(w2_buckets),
        "w2_two_sided_discordant_p": exact_two_sided_discordant(
            w2_buckets["enabled_only"], w2_buckets["disabled_only"]),
        "late_w2_paired": dict(late_buckets),
        "late_w2_two_sided_discordant_p": exact_two_sided_discordant(
            late_buckets["enabled_only"], late_buckets["disabled_only"]),
        "counts": dict(counts),
        "causal_path_first_divergence": dict(causal_pc),
        "seed_results": seed_results,
        "cl1_closed": False,
    }


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("sqlite", type=Path)
    parser.add_argument("--json", required=True, type=Path)
    args = parser.parse_args()
    result = audit(args.sqlite)
    args.json.write_text(json.dumps(result, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    print(json.dumps({
        name: result[name] for name in (
            "status", "w2_paired", "w2_two_sided_discordant_p",
            "late_w2_paired", "late_w2_two_sided_discordant_p",
            "counts", "causal_path_first_divergence",
        )
    }, sort_keys=True))


if __name__ == "__main__":
    main()
