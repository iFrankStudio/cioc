#!/usr/bin/env python3
"""Study40: retrospective successful native W3 work vs external starter dependence.

This cannot diagnose native life or repair; zero new independent worlds.
"""
from __future__ import annotations

import argparse
import json
import sqlite3
import statistics
from collections import Counter, defaultdict
from pathlib import Path

import analyze
import analyze_study38
import analyze_study39

STUDIES = (
    ("38", "02d0e9a017a26107edeca164c728d73e00998517", 97, 33),
    ("39", "2071049cc8896ef6f1fc6d6e5f004fe152ae28e6", 93, 36),
)
PRIMARY_ARMS = (
    "closed", "closed-general", "ambient", "ambient-draw", "ambient-general"
)


def internal(event: dict) -> bool:
    return analyze.internal_process(event) == event["process"]


def successful(kind) -> bool:
    return kind != "Halted" and not (
        isinstance(kind, dict) and "Trapped" in kind
    )


def native_successes(events: list[dict], code: dict) -> list[dict]:
    """Only internally attributed successful opcodes; reject wrong actor or PC."""
    wins = []
    for event in events:
        if not internal(event):
            raise ValueError("native W3 event lacks valid actor provenance")
        actor = event["process"]
        program = code[str(actor)]["code"]["nodes"]
        pc = event["pc"]
        if not isinstance(pc, int) or not 0 <= pc < len(program):
            raise ValueError("native event references invalid original instruction")
        if successful(event["kind"]):
            wins.append(event)
    return wins


def summarize_run(initial: dict, snapshots: list[dict], actions: list[dict],
                  attempts: list[int]) -> dict:
    """Use 24 ordered native epochs, never final event deltas as fake epochs."""
    if len(snapshots) != 24 or len(attempts) != 24:
        raise ValueError("missing registered observation epochs")
    previous = initial["events"]
    native_by_epoch = {}
    starter_epochs = []
    same_epoch_starter_success = 0
    internal_writes = 0
    internal_draws = 0
    graph_actions = Counter()
    total_success = 0
    total_attempts = 0
    examples = []
    initial_code = initial["processes"]
    for epoch, snapshot in enumerate(snapshots):
        current = snapshot["events"]
        if current[:len(previous)] != previous:
            raise ValueError("native event history dropped or reordered")
        epoch_actions = [action for action in actions if action["epoch"] == epoch]
        events = current[len(previous):]
        if len(events) != attempts[epoch] + len(epoch_actions):
            raise ValueError("external action or native-attempt count mismatch")
        for event in events[:len(epoch_actions)]:
            if event["origin"] != "External":
                raise ValueError("externally issued credit action forged as native")
        native = events[len(epoch_actions):]
        if len(native) != attempts[epoch]:
            raise ValueError("incorrect native event partition")
        # Independently validate native actor/PC for all events, not just winners.
        victories = native_successes(native, initial_code)
        if epoch >= 16:
            starter_ids = [
                a["action"]["SetCredits"]["process"]
                for a in epoch_actions
                if "SetCredits" in a["action"]
                and a["action"]["SetCredits"]["credits"] == 1
            ]
            if len(starter_ids) > 1:
                raise ValueError("more than one environmental starter per epoch")
            if starter_ids:
                actor = starter_ids[0]
                if not native or native[0]["process"] != actor:
                    raise ValueError("environmental starter did not enable matching first native actor")
                starter_epochs.append(epoch)
                if victories:
                    same_epoch_starter_success += 1
            successes = len(victories)
            total_success += successes
            total_attempts += attempts[epoch]
            native_by_epoch[epoch] = {
                "native_attempts": attempts[epoch],
                "native_successes": successes,
                "starter": bool(starter_ids),
            }
            for event in victories:
                kind = event["kind"]
                opcode = initial_code[str(event["process"])]["code"]["nodes"][event["pc"]]
                if (isinstance(opcode, dict) and "DrawCompute" in opcode
                        and kind == "Executed"):
                    internal_draws += 1
                if isinstance(kind, dict):
                    if "CellWritten" in kind:
                        internal_writes += 1
                        if len(examples) < 3:
                            examples.append({
                                "epoch": epoch,
                                "actor": event["process"],
                                "cell": kind["CellWritten"]["key"],
                                "value": kind["CellWritten"]["value"],
                                "same_epoch_external_starter": bool(starter_ids),
                            })
                    for name in ("GraphCreated", "GraphNodeCopied", "GraphTagRead"):
                        if name in kind:
                            graph_actions[name] += 1
        previous = current
    successes_by_epoch = [
        e for e, item in native_by_epoch.items() if item["native_successes"] > 0
    ]
    no_same_epoch_starter = [
        e for e in successes_by_epoch if not native_by_epoch[e]["starter"]
    ]
    return {
        "W3_attempts": total_attempts,
        "W3_successes": total_success,
        "W3_active": total_attempts > 0,
        "W3_successful_world": total_success > 0,
        "W3_native_write_world": internal_writes > 0,
        "W3_native_write_events": internal_writes,
        "W3_native_Draw_world": internal_draws > 0,
        "W3_native_Draw_events": internal_draws,
        "W3_successful_epochs": len(successes_by_epoch),
        "W3_success_2_epochs": len(successes_by_epoch) >= 2,
        "W3_success_all_8_epochs": len(successes_by_epoch) == 8,
        "W3_success_without_starter_2_epochs": len(no_same_epoch_starter) >= 2,
        "W3_success_without_starter_epochs": len(no_same_epoch_starter),
        "W3_external_starter_epochs": starter_epochs,
        "W3_starter_plus_success_epochs": same_epoch_starter_success,
        "W3_native_graph_events": dict(graph_actions),
        "W3_native_write_examples": examples,
    }


def audit_one(path: Path, study: str, source_revision: str,
              expected_general: int, expected_draw: int) -> dict:
    # This first reuses the independently designed, previously validated
    # original full-cohort auditor: source-pinned grammar, all hashes, policy,
    # actor attribution, credit flow and exact intervention schedule.
    original = (
        analyze_study38.audit(path, source_revision)
        if study == "38" else analyze_study39.audit(path, source_revision)
    )
    if (original["independent_seed_worlds"] != 128
            or original["matched_runs"] != 1280
            or original["hashed_epoch_snapshots"] != 30720
            or original["registered_primary"]["ambient_general_W3"]["worlds"]
            != expected_general
            or original["registered_primary"]["ambient_draw_W3"]["worlds"]
            != expected_draw):
        raise ValueError("original registered Study38/39 W3 source result changed")

    db = sqlite3.connect(f"{path.resolve().as_uri()}?mode=ro", uri=True)
    try:
        counts = Counter()
        success_distributions = defaultdict(list)
        seeds = defaultdict(dict)
        witnesses = []
        rows = db.execute(
            "SELECT id,condition,source_revision,initial_sha256,initial_json,"
            "actions_json,attempts_json FROM epoch_runs ORDER BY id"
        )
        total = 0
        snapshots_total = 0
        expected_seed_start = 2_900_000 if study == "38" else 3_000_000
        for run_id, label, revision, initial_sha, initial_raw, actions_raw, attempts_raw in rows:
            seed_label, arm = label.split("/", 1)
            seed = int(seed_label.removeprefix("seed="))
            if not (expected_seed_start <= seed < expected_seed_start + 128
                    and arm in analyze_study38.ARMS):
                raise ValueError("unregistered cohort seed or arm")
            if revision != source_revision or analyze.digest(initial_raw) != initial_sha:
                raise ValueError("historical source/initial corruption")
            initial = json.loads(initial_raw)
            actions = json.loads(actions_raw)
            attempts = json.loads(attempts_raw)
            snapshots = []
            for pos, digest, raw in db.execute(
                "SELECT epoch,state_sha256,state_json FROM epoch_snapshots "
                "WHERE run_id=? ORDER BY epoch", (run_id,)
            ):
                if pos != len(snapshots) or analyze.digest(raw) != digest:
                    raise ValueError("W3 observational snapshot corruption")
                snapshots.append(json.loads(raw))
                snapshots_total += 1
            result = summarize_run(initial, snapshots, actions, attempts)
            if arm in seeds[seed]:
                raise ValueError("duplicate paired sample")
            seeds[seed][arm] = result
            for key in (
                "W3_active", "W3_successful_world", "W3_native_write_world",
                "W3_native_Draw_world", "W3_success_2_epochs",
                "W3_success_all_8_epochs", "W3_success_without_starter_2_epochs",
            ):
                counts[f"{arm}.{key}"] += bool(result[key])
            for key in (
                "W3_attempts", "W3_successes", "W3_native_write_events",
                "W3_native_Draw_events", "W3_success_without_starter_epochs",
                "W3_starter_plus_success_epochs",
            ):
                counts[f"{arm}.{key}"] += result[key]
            counts[f"{arm}.W3_external_starter_events"] += len(
                result["W3_external_starter_epochs"]
            )
            for graph_kind, n in result["W3_native_graph_events"].items():
                counts[f"{arm}.native_{graph_kind}_events"] += n
            if result["W3_active"]:
                success_distributions[arm].append(result["W3_successes"])
            if result["W3_native_write_examples"] and len(witnesses) < 16:
                witnesses.append({
                    "seed": seed, "arm": arm,
                    "native_writes": result["W3_native_write_examples"],
                })
            total += 1
        if (total != 1280 or snapshots_total != 30720 or len(seeds) != 128
                or any(set(arms) != set(analyze_study38.ARMS) for arms in seeds.values())):
            raise ValueError("incomplete or unbalanced original matched W3 cohort")
        if (counts["ambient-general.W3_active"] != expected_general
                or counts["ambient-draw.W3_active"] != expected_draw):
            raise ValueError("registered W3 primary count not independently reproduced")
        for arm in ("closed", "closed-draw", "closed-general", "ambient",
                    "ambient-draw", "ambient-general", "sham"):
            counts[f"{arm}.W3_active"] += 0
            counts[f"{arm}.W3_successful_world"] += 0
        distributions = {}
        for arm in analyze_study38.ARMS:
            successes = success_distributions[arm]
            distributions[arm] = ({
                "active_worlds": len(successes),
                "successful_native_attempts_median_when_W3_active": (
                    statistics.median(successes) if successes else None
                ),
                "successful_native_attempts_min_when_W3_active": (
                    min(successes) if successes else None
                ),
                "successful_native_attempts_max_when_W3_active": (
                    max(successes) if successes else None
                ),
            })
        return {
            "original_study": study,
            "source_revision": source_revision,
            "retrospective_new_independent_worlds": 0,
            "reused_original_worlds": 128,
            "matched_runs": total,
            "verified_epoch_snapshots": snapshots_total,
            "metrics": dict(sorted(counts.items())),
            "W3_success_distribution": distributions,
            "sample_native_cell_output_witnesses": witnesses,
            "per_seed": {
                str(seed): {
                    arm: {
                        "W3_successful_epochs": seeds[seed][arm]["W3_successful_epochs"],
                        "W3_native_write_world": seeds[seed][arm]["W3_native_write_world"],
                        "W3_success_without_starter_epochs":
                        seeds[seed][arm]["W3_success_without_starter_epochs"],
                    }
                    for arm in PRIMARY_ARMS
                }
                for seed in sorted(seeds)
            },
            "life_claim": "NONE",
        }
    finally:
        db.close()


def main() -> None:
    p = argparse.ArgumentParser(description=__doc__)
    p.add_argument("study38", type=Path)
    p.add_argument("study39", type=Path)
    p.add_argument("--json", type=Path, required=True)
    args = p.parse_args()
    cases = {}
    for study, revision, general, draw in STUDIES:
        case = args.study38 if study == "38" else args.study39
        cases[study] = audit_one(case, study, revision, general, draw)
    result = {
        "study": "CL-1 Study40 retrospective native W3 functionality audit",
        "registered_after_source_results": True,
        "new_independent_worlds": 0,
        "reused_independent_cohorts": 2,
        "matched_runs": sum(x["matched_runs"] for x in cases.values()),
        "verified_epoch_sha256_snapshots": sum(
            x["verified_epoch_snapshots"] for x in cases.values()
        ),
        "separate_cohorts": cases,
        "life_claim": "NONE",
    }
    args.json.write_text(json.dumps(result, indent=2) + "\n", encoding="utf-8")
    print(json.dumps({
        "study": result["study"],
        "new_worlds": 0,
        "snapshots": result["verified_epoch_sha256_snapshots"],
        "study38": {
            key: cases["38"]["metrics"].get(f"ambient-general.{key}", 0)
            for key in ("W3_successful_world", "W3_native_write_world",
                        "W3_success_all_8_epochs", "W3_success_without_starter_2_epochs")
        },
        "study39": {
            key: cases["39"]["metrics"].get(f"ambient-general.{key}", 0)
            for key in ("W3_successful_world", "W3_native_write_world",
                        "W3_success_all_8_epochs", "W3_success_without_starter_2_epochs")
        },
        "life_claim": "NONE",
    }, indent=2))


if __name__ == "__main__":
    main()
