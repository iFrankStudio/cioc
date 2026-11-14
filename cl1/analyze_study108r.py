#!/usr/bin/env python3
"""Read-only original Study108 -> post-outcome natural-terminal continuation audit."""
import argparse
import hashlib
import json
import sqlite3
from pathlib import Path

from analyze_study108 import audit as original_study108_audit, audit_events

ORIGINAL = "406bb3e1160987113671eb9ffdf517c3edf4ad51"
MODES = ("native-edit-source3", "patch-disabled-source3",
         "native-edit-source8", "patch-disabled-source8")


def require(ok, text):
    if not ok:
        raise ValueError(text)


def hash_text(value):
    return hashlib.sha256(value.encode("utf-8")).hexdigest()


def native_births(world):
    return sum(
        isinstance(e["kind"], dict) and "GraphLaunched" in e["kind"]
        and e["origin"] == {"Internal": e["process"]}
        for e in world["events"]
    )


def audit(original_path, continuation_path, revision, summary_path):
    audit_old = original_study108_audit(original_path, ORIGINAL)
    require(audit_old["native_original_world_runs"] == 128
            and audit_old["verified_full_epoch_sha256"] == 1024,
            "original Study108 corpus independent verification failed")
    originals = sqlite3.connect(original_path)
    original_rows = originals.execute(
        "SELECT condition,final_json,final_sha256 FROM epoch_runs"
    ).fetchall()
    old = {label: (json_raw, sha) for label, json_raw, sha in original_rows}
    require(len(old) == 128, "invalid complete original Study108 run denominator")

    continued = sqlite3.connect(continuation_path)
    rows = continued.execute(
        "SELECT id,condition,source_revision,initial_json,initial_sha256,"
        "attempts_json,final_json,final_sha256 FROM epoch_runs ORDER BY id"
    ).fetchall()
    require(len(rows) == 128, "missing originally registered 128 continuation worlds")
    summary = json.loads(summary_path.read_text())
    require(summary["original_worlds"] == 128
            and summary["new_genotype_units"] == 0
            and summary["new_exact_replayed_epochs"] == 512
            and summary["original_revision"] == ORIGINAL
            and summary["continuation_source_revision"] == revision,
            "runner summary provenance incorrect")
    claimed = {entry["source_label"]: entry
               for entry in summary["all128_verified_world_chains"]}
    require(len(claimed) == 128, "duplicate runner chain")

    seen = set()
    per_world = {}
    total_snapshots = 0
    total_added_attempts = 0
    for run_id, label, src_rev, initial_json, initial_sha, attempts_json, final_json, final_sha in rows:
        require(src_rev == revision and len(revision) == 40, "continuation source revision differs")
        require(label.startswith("resumed/seed="), "unexpected continuation label")
        old_label = label.removeprefix("resumed/")
        seed_s, mode = old_label.removeprefix("seed=").split("/", 1)
        seed = int(seed_s)
        require(seed in range(6200000, 6200032) and mode in MODES and old_label not in seen,
                "unexpected original seed/arm or duplicate in continuation")
        seen.add(old_label)
        require(old_label in old and old_label in claimed, "original linked run absent")
        original_full, original_sha = old[old_label]
        require(initial_json == original_full
                and initial_sha == original_sha
                and hash_text(initial_json) == initial_sha
                and hash_text(final_json) == final_sha,
                "original final to continuation initial exact SHA256 linkage failure")
        attempts = json.loads(attempts_json)
        added = sum(attempts)
        require(len(attempts) == 4 and attempts[-1] == 0
                and all(0 <= a <= 500 for a in attempts) and added <= 1400,
                "unnatural termination or additional-credit attempts")
        original = json.loads(initial_json)
        final = json.loads(final_json)
        require(final["compute_pool"] + sum(
            p["credits"] for p in final["processes"].values()
        ) == 1400 - added, "native continuation resource accounting mismatch")
        require(sum(e["origin"] == "External" for e in final["events"]) == 5,
                "unexpected post-fork host action")
        natives = [e for e in final["events"] if isinstance(e["origin"], dict)]
        require(len(natives) == 2000 + added, "native event count not original+added charged attempts")
        for ev in natives:
            require(ev["origin"] == {"Internal": ev["process"]}, "wrong process source")
        before = native_births(original)
        after = native_births(final)
        require(after >= before, "native child history vanished after original cutoff")
        given = claimed[old_label]
        require(given["prior_native_births"] == before
                and given["terminal_native_births"] == after
                and given["additional_charged_native_attempts"] == added
                and given["continuation_initial_sha256"] == initial_sha
                and given["continued_final_sha256"] == final_sha,
                "Rust runner birth totals or SHA summaries not reproduced")
        source = 3 if mode.endswith("source3") else 8
        disabled = mode.startswith("patch-disabled")
        grant = original["processes"]["1"]["code"]["nodes"][20]["Const"]["value"]
        final["_actual_attempts"] = 2000 + added
        audit_events(final, source, disabled, grant)
        final.pop("_actual_attempts")
        states = continued.execute(
            "SELECT epoch,state_sha256,state_json FROM epoch_snapshots WHERE run_id=? ORDER BY epoch",
            (run_id,),
        ).fetchall()
        require(len(states) == 4, "expected four originally frozen continuation epochs")
        for index, (epoch, hs, body) in enumerate(states):
            require(epoch == index and hash_text(body) == hs,
                    "original native continuation epoch SHA256 mismatch")
            total_snapshots += 1
        require(states[-1][2] == final_json, "last native continuation snapshot not actual final")
        per_world[(seed, mode)] = (before, after)
        total_added_attempts += added

    require(len(per_world) == 128 and total_snapshots == 512,
            "not all original worlds fully audited to natural terminal")
    comparisons = []
    for seed in range(6200000, 6200032):
        for source in (3, 8):
            enabled = per_world[(seed, f"native-edit-source{source}")]
            disabled = per_world[(seed, f"patch-disabled-source{source}")]
            comparisons.append({
                "seed": seed, "source": source,
                "old_birth_delta": enabled[0] - disabled[0],
                "terminal_birth_delta": enabled[1] - disabled[1],
            })
    require(len(comparisons) == 64
            and sum(x["old_birth_delta"] == 1 for x in comparisons) == 2
            and all((x["seed"] == 6200017) == (x["old_birth_delta"] == 1)
                    for x in comparisons),
            "known original Study108 positive cohort altered")
    recorded = summary["all64_original_paired_birth_differences_at_terminal"]
    require(all(
        a["seed"] == b["seed"] and a["source"] == b["source"]
        and a["original_fixed_2000_birth_delta"] == b["old_birth_delta"]
        and a["terminal_birth_delta"] == b["terminal_birth_delta"]
        for a, b in zip(recorded, comparisons)
    ), "paired comparison inconsistent with original SQLite")
    require(total_added_attempts == summary["additional_charged_native_attempts"],
            "independent total continuation charges mismatch")
    return {
        "status": "VERIFIED_POST_OUTCOME_ORIGINAL_STUDY108_CONTINUATIONS_NO_NEW_SAMPLES",
        "source_original": ORIGINAL,
        "source_continuation": revision,
        "new_genotypes": 0,
        "verified_original_epochs": 1024,
        "verified_continuation_epochs": total_snapshots,
        "original_worlds": 128,
        "post_cutoff_charged_native_attempts": total_added_attempts,
        "terminal_paired_positive": sum(x["terminal_birth_delta"] > 0 for x in comparisons),
        "terminal_paired_negative": sum(x["terminal_birth_delta"] < 0 for x in comparisons),
        "terminal_paired_ties": sum(x["terminal_birth_delta"] == 0 for x in comparisons),
        "original_seed6200017_source_pairs_at_natural_terminal": [
            x for x in comparisons if x["seed"] == 6200017
        ],
        "cyberlife_or_natural_selection_proven": False,
    }


def main():
    p = argparse.ArgumentParser()
    p.add_argument("original", type=Path)
    p.add_argument("continuation", type=Path)
    p.add_argument("summary", type=Path)
    p.add_argument("--expected-revision", required=True)
    p.add_argument("--json", type=Path, required=True)
    a = p.parse_args()
    result = audit(a.original, a.continuation, a.expected_revision, a.summary)
    a.json.write_text(json.dumps(result, sort_keys=True, indent=2) + "\n")
    print(json.dumps(result, sort_keys=True))


if __name__ == "__main__":
    main()
