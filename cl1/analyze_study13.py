#!/usr/bin/env python3
"""CL-1 Study 13: independent two-window reciprocal graph-information audit.

A successful exchange is an information-processing proxy, never proof of
self-maintenance, restoration, or life.
"""
from __future__ import annotations

import argparse
import json
import sqlite3
from collections import Counter
from pathlib import Path

import analyze
from analyze_study02 import wilson_95

FIRST = 1_930_000
SEEDS = 96
EPOCHS = 16
QUOTA = 12
LABELS = ("control", "sham-drop", "drop-g1", "drop-g2", "drop-both", "zero-credit")


def expected_schedule(label: str, initial: dict) -> list[dict]:
    at = lambda action: {"epoch": 0, "action": action}
    if label == "control":
        return []
    if label == "sham-drop":
        return [at({"DropGraph": {"graph": 0}})]
    if label == "drop-g1":
        return [at({"DropGraph": {"graph": 1}})]
    if label == "drop-g2":
        return [at({"DropGraph": {"graph": 2}})]
    if label == "drop-both":
        return [at({"DropGraph": {"graph": 1}}), at({"DropGraph": {"graph": 2}})]
    if label == "zero-credit":
        return [
            at({"SetCredits": {"process": int(process), "credits": 0}})
            for process in sorted(initial["processes"], key=int)
        ]
    raise ValueError(f"unknown experimental arm: {label}")


def simulated_state(world: dict) -> dict:
    return {
        "processes": world["processes"],
        "graph_objects": world["graph_objects"],
        "cells": world["cells"],
        "compute_pool": world["compute_pool"],
    }


def epoch_graph_events(initial: dict, snapshots: list[dict]) -> tuple[list[list[dict]], list[dict]]:
    previous = len(initial["events"])
    by_epoch = []
    for snapshot in snapshots:
        events = snapshot["events"]
        if len(events) < previous:
            raise ValueError("event log shrank between epochs")
        by_epoch.append(events[previous:])
        previous = len(events)
    return by_epoch, [event for group in by_epoch for event in group]


def reciprocal_pairs(groups: list[list[dict]]) -> tuple[set[tuple[int, int]], Counter]:
    """Success events only: actor must own the target, PC post scaffold."""
    directions = set()
    copies = Counter()
    for events in groups:
        for event in events:
            kind = event["kind"]
            if not isinstance(kind, dict) or "GraphCopied" not in kind:
                continue
            actor = analyze.internal_process(event)
            payload = kind["GraphCopied"]
            source = payload["source_owner"]
            target = payload["target_owner"]
            if actor is None or actor != target or event["pc"] < 9:
                raise ValueError("invalid copy provenance in post-scaffold epoch")
            if source == target:
                continue
            directions.add((source, target))
            copies[(source, target)] += 1
    paired = {
        tuple(sorted((source, target)))
        for source, target in directions
        if (target, source) in directions
    }
    return paired, copies


def newly_populated_graph(initial: dict, final: dict, events: list[dict]) -> bool:
    """Any observed internally sourced new graph later receiving a code node.

    Event ordering is stronger than final-state-only growth, because an
    internally created graph may be dropped before the final snapshot.
    Old graph handles are excluded, and mere header allocation is not growth.
    """
    old_handles = set(initial["graph_objects"])
    created: set[str] = set()
    filled: set[str] = set()
    for event in events:
        actor = analyze.internal_process(event)
        kind = event["kind"]
        if actor is None or not isinstance(kind, dict):
            continue
        if "GraphCreated" in kind:
            graph_id = str(kind["GraphCreated"]["graph"])
            if graph_id not in old_handles:
                created.add(graph_id)
        elif "GraphNodeCopied" in kind:
            graph_id = str(kind["GraphNodeCopied"]["graph"])
            if graph_id in created:
                filled.add(graph_id)
        elif "GraphCopied" in kind:
            graph_id = str(kind["GraphCopied"]["target_graph"])
            if graph_id in created:
                filled.add(graph_id)
    # Other generic graph-code append operations are only visible in state
    # snapshots; retain a conservative surviving final-state fallback.
    filled.update(
        graph_id
        for graph_id, obj in final["graph_objects"].items()
        if graph_id in created and obj["nodes"]
    )
    return bool(filled)


def audit(path: Path) -> dict:
    db = sqlite3.connect(f"{path.resolve().as_uri()}?mode=ro", uri=True)
    try:
        rows = db.execute(
            "SELECT id,source_revision,condition,initial_sha256,initial_json,"
            "actions_json,epoch_count,quota,attempts_json,final_sha256,final_json "
            "FROM epoch_runs ORDER BY id"
        ).fetchall()
        if len(rows) != SEEDS * len(LABELS):
            raise ValueError(f"incomplete cohort: {len(rows)} runs")
        checkpoints = {}
        for run_id, epoch, sha, serialized in db.execute(
            "SELECT run_id,epoch,state_sha256,state_json "
            "FROM epoch_snapshots ORDER BY run_id,epoch"
        ):
            if not 0 <= epoch < EPOCHS or analyze.digest(serialized) != sha:
                raise ValueError(f"damaged checkpoint run={run_id},epoch={epoch}")
            checkpoints.setdefault(run_id, []).append((epoch, sha, serialized))
        by_seed = {}
        initial_sha_by_seed = {}
        revisions = set()
        for (
            run_id, revision, label, initial_sha, initial_json, actions_json,
            epochs, quota, attempts_json, final_sha, final_json,
        ) in rows:
            if not label.startswith("seed=") or "/" not in label:
                raise ValueError(f"bad condition: {label}")
            prefix, arm = label.split("/", 1)
            seed = int(prefix[5:])
            if seed not in range(FIRST, FIRST + SEEDS) or arm not in LABELS:
                raise ValueError(f"nonregistered seed/arm: {label}")
            if arm in by_seed.get(seed, {}):
                raise ValueError(f"duplicate experimental arm: {label}")
            if (
                not revision or epochs != EPOCHS or quota != QUOTA
                or analyze.digest(initial_json) != initial_sha
                or analyze.digest(final_json) != final_sha
            ):
                raise ValueError(f"invalid source revision or snapshot digest: {label}")
            revisions.add(revision)
            if seed in initial_sha_by_seed and initial_sha_by_seed[seed] != initial_sha:
                raise ValueError(f"unmatched initial world: {seed}")
            initial_sha_by_seed[seed] = initial_sha
            initial = json.loads(initial_json)
            final = json.loads(final_json)
            if (
                initial["total_storage_limit"] != 192
                or not initial["strict_accounting"]
                or analyze.occupied_units(initial) > 192
                or analyze.occupied_units(final) > 192
            ):
                raise ValueError(f"world violates strict storage: {label}")
            for graph_id in ("1", "2", "3"):
                obj = initial["graph_objects"].get(graph_id)
                if obj is None or not obj["nodes"] or obj["owner"] != int(graph_id):
                    raise ValueError(f"scaffold not exposed: {label}")
            scheduled = json.loads(actions_json)
            if scheduled != expected_schedule(arm, initial):
                raise ValueError(f"unregistered intervention schedule: {label}")
            attempts_by_epoch = json.loads(attempts_json)
            if len(attempts_by_epoch) != EPOCHS or any(
                not isinstance(n, int) or n < 0 or n > QUOTA for n in attempts_by_epoch
            ):
                raise ValueError(f"unregistered attempt allocation: {label}")
            snaps = checkpoints.get(run_id, [])
            if len(snaps) != EPOCHS or any(i != epoch for i, (epoch, _, _) in enumerate(snaps)):
                raise ValueError(f"missing/duplicate/out-of-order epoch state: {label}")
            if snaps[-1][1] != final_sha or snaps[-1][2] != final_json:
                raise ValueError(f"last epoch differs from final state: {label}")
            actual = [json.loads(serialized) for _, _, serialized in snaps]
            epochs_events, postfork = epoch_graph_events(initial, actual)
            if arm == "zero-credit" and (
                any(attempts_by_epoch)
                or any(analyze.internal_process(event) is not None for event in postfork)
            ):
                raise ValueError(f"zero credit process unexpectedly executed: {label}")
            expected_damage = {
                "control": 0, "sham-drop": 0, "drop-g1": 1,
                "drop-g2": 1, "drop-both": 2, "zero-credit": 0,
            }[arm]
            delivered = sum(
                event["origin"] == "External"
                and isinstance(event["kind"], dict)
                and "Intervened" in event["kind"]
                and "DropGraph" in event["kind"]["Intervened"]
                for event in postfork
            )
            if delivered != expected_damage:
                raise ValueError(f"external ablation exposure failed: {label}")
            w1_pairs, w1_edges = reciprocal_pairs(epochs_events[:8])
            w2_pairs, w2_edges = reciprocal_pairs(epochs_events[8:])
            by_seed.setdefault(seed, {})[arm] = {
                "initial": initial, "final": final,
                "w1": w1_pairs, "w2": w2_pairs,
                "w1_copies": sum(w1_edges.values()),
                "w2_copies": sum(w2_edges.values()),
                "postfork": postfork,
                "new_nonempty": newly_populated_graph(initial, final, postfork),
            }
        if len(revisions) != 1 or len(by_seed) != SEEDS or any(
            set(arms) != set(LABELS) for arms in by_seed.values()
        ):
            raise ValueError("missing independent seed, arm or stable source revision")
        counts = Counter()
        direction_pairs = Counter()
        for seed in sorted(by_seed):
            arms = by_seed[seed]
            control = arms["control"]
            if simulated_state(control["final"]) != simulated_state(arms["sham-drop"]["final"]):
                raise ValueError(f"invalid sham equivalence for seed {seed}")
            counts["bidirectional_w1"] += bool(control["w1"])
            counts["bidirectional_w2"] += bool(control["w2"])
            counts["bidirectional_both_windows"] += bool(control["w1"] and control["w2"])
            counts["control_copy_events_w1"] += control["w1_copies"]
            counts["control_copy_events_w2"] += control["w2_copies"]
            for pair in control["w1"] | control["w2"]:
                direction_pairs[str(pair)] += 1
            if control["w1"]:
                counts["w1_exposed"] += 1
                lost1 = not bool(control["w1"] & arms["drop-g1"]["w1"])
                lost2 = not bool(control["w1"] & arms["drop-g2"]["w1"])
                counts["drop_g1_function_loss"] += lost1
                counts["drop_g2_function_loss"] += lost2
                counts["single_target_loss_any"] += lost1 or lost2
                counts["joint_damage_increment"] += (
                    not bool(control["w1"] & arms["drop-both"]["w1"])
                    and not lost1 and not lost2
                )
            for arm in ("drop-g1", "drop-g2", "drop-both"):
                outcome = arms[arm]
                counts[f"{arm}_new_nonempty"] += outcome["new_nonempty"]
                has_new_header = any(
                    isinstance(event["kind"], dict) and "GraphCreated" in event["kind"]
                    for event in outcome["postfork"]
                    if analyze.internal_process(event) is not None
                )
                has_node_copy = any(
                    isinstance(event["kind"], dict) and "GraphNodeCopied" in event["kind"]
                    for event in outcome["postfork"]
                    if analyze.internal_process(event) is not None
                )
                counts[f"{arm}_late_reappearance_candidate"] += (
                    not bool(outcome["w1"]) and bool(outcome["w2"])
                    and has_new_header and has_node_copy
                )
        binary_names = [
            "bidirectional_w1", "bidirectional_w2", "bidirectional_both_windows",
            "w1_exposed", "drop_g1_function_loss", "drop_g2_function_loss",
            "single_target_loss_any", "joint_damage_increment",
            "drop-g1_new_nonempty", "drop-g2_new_nonempty", "drop-both_new_nonempty",
            "drop-g1_late_reappearance_candidate",
            "drop-g2_late_reappearance_candidate",
            "drop-both_late_reappearance_candidate",
        ]
        metrics = {}
        for name in binary_names:
            n = counts[name]
            lo, hi = wilson_95(n, SEEDS)
            metrics[name] = {
                "seed_worlds": n,
                "denominator": SEEDS,
                "wilson_95": [round(lo, 6), round(hi, 6)],
            }
        return {
            "study": "CL-1 Study 13",
            "status": "two-window reciprocal information-flow causal probe; not proof of closure",
            "source_revision": next(iter(revisions)),
            "independent_seed_worlds": SEEDS,
            "matched_runs": len(rows),
            "hashed_epoch_snapshots": sum(map(len, checkpoints.values())),
            "exact_replay": "Rust laboratory full world/event replays, every epoch",
            "metrics": metrics,
            "control_copy_events_w1": counts["control_copy_events_w1"],
            "control_copy_events_w2": counts["control_copy_events_w2"],
            "owner_pairs_with_both_directions": dict(direction_pairs),
            "scientific_recovery": "NOT EVALUABLE",
            "organizational_closure": "NOT EVALUABLE",
            "life_claim": "NONE",
            "caveat": "Observer-defined reciprocal graph copying may reflect initially randomized code, not internally initiated functional regeneration.",
        }
    finally:
        db.close()


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("sqlite", type=Path)
    parser.add_argument("--json", type=Path, required=True)
    args = parser.parse_args()
    report = audit(args.sqlite)
    args.json.write_text(json.dumps(report, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
    print(json.dumps({
        "study": report["study"],
        "worlds": report["independent_seed_worlds"],
        "runs": report["matched_runs"],
        "w1": report["metrics"]["bidirectional_w1"]["seed_worlds"],
        "w2": report["metrics"]["bidirectional_w2"]["seed_worlds"],
        "conditional_exposed": report["metrics"]["w1_exposed"]["seed_worlds"],
    }))


if __name__ == "__main__":
    main()
