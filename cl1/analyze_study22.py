#!/usr/bin/env python3
"""Study 22: independent audit of fresh-seed, matched compute-recharge ecology.

Never infer code/data causality or native repair from energy injections.
"""
from __future__ import annotations

import argparse
import json
import sqlite3
from collections import Counter
from pathlib import Path

import analyze
from analyze_study02 import wilson_95
from analyze_study14 import epoch_graph_events, reciprocal_pairs, simulated_state, newly_populated_graph

FIRST = 2_220_000
SEEDS = 96
EPOCHS = 24
QUOTA = 12
LABELS = (
    "control", "sham-drop", "refill",
    "drop-g1-refill", "drop-g2-refill", "drop-both-refill",
    "restore-g1-refill", "zero-credit",
)
WINDOWS = ((0, 8), (8, 16), (16, 24))
REFILL_ARMS = set(LABELS[2:-1])


def schedule(arm: str, initial: dict) -> list[dict]:
    def epoch0(action: dict) -> dict:
        return {"epoch": 0, "action": action}

    if arm == "control":
        return []
    if arm == "sham-drop":
        return [epoch0({"DropGraph": {"graph": 0}})]
    if arm == "zero-credit":
        return [
            epoch0({"SetCredits": {"process": int(process), "credits": 0}})
            for process in sorted(initial["processes"], key=int)
        ]
    if arm not in REFILL_ARMS:
        raise ValueError(f"unknown arm: {arm}")
    actions: list[dict] = []
    if arm in ("drop-g1-refill", "drop-both-refill", "restore-g1-refill"):
        actions.append(epoch0({"DropGraph": {"graph": 1}}))
    if arm in ("drop-g2-refill", "drop-both-refill"):
        actions.append(epoch0({"DropGraph": {"graph": 2}}))
    if arm == "restore-g1-refill":
        actions.append({
            "epoch": 8,
            "action": {"PutGraph": {
                "graph": 1, "object": initial["graph_objects"]["1"],
            }},
        })
    for epoch in (8, 16):
        for process in (1, 2, 3):
            actions.append({
                "epoch": epoch,
                "action": {"SetCredits": {"process": process, "credits": 47}},
            })
    return actions


def _successes(events: list[dict]) -> tuple[int, int, int]:
    """Count genuine charged, internal cross-owner copies, cell reads, writes."""
    copied = reads = writes = 0
    for event in events:
        kind = event["kind"]
        if not isinstance(kind, dict):
            continue
        actor = analyze.internal_process(event)
        if actor is None or actor != event["process"]:
            continue
        if "GraphCopied" in kind:
            payload = kind["GraphCopied"]
            if actor != payload["target_owner"] or event["pc"] < 9:
                raise ValueError("invalid successful graph-copy provenance")
            copied += int(payload["source_owner"] != payload["target_owner"])
        if "CellRead" in kind:
            reads += 1
        if "CellWritten" in kind:
            writes += 1
    return copied, reads, writes


def _external_count(events: list[dict], label: str) -> int:
    return sum(
        event["origin"] == "External"
        and isinstance(event["kind"], dict)
        and "Intervened" in event["kind"]
        and label in str(event["kind"]["Intervened"])
        for event in events
    )


def audit(path: Path) -> dict:
    db = sqlite3.connect(f"{path.resolve().as_uri()}?mode=ro", uri=True)
    try:
        rows = db.execute(
            "SELECT id,source_revision,condition,initial_sha256,initial_json,"
            "actions_json,epoch_count,quota,attempts_json,final_sha256,final_json "
            "FROM epoch_runs ORDER BY id"
        ).fetchall()
        if len(rows) != SEEDS * len(LABELS):
            raise ValueError(f"incomplete preregistered ecology: {len(rows)} paired runs")
        frames: dict[int, list[tuple[int, str, str]]] = {}
        for run_id, epoch, sha, raw in db.execute(
            "SELECT run_id,epoch,state_sha256,state_json "
            "FROM epoch_snapshots ORDER BY run_id,epoch"
        ):
            if not 0 <= epoch < EPOCHS or analyze.digest(raw) != sha:
                raise ValueError(f"invalid hashed epoch run={run_id} epoch={epoch}")
            frames.setdefault(run_id, []).append((epoch, sha, raw))
        by_seed: dict[int, dict] = {}
        initial_hashes = {}
        source_revisions: set[str] = set()
        for (run_id, revision, condition, initial_sha, initial_raw,
             actions_raw, epochs, quota, attempts_raw, final_sha, final_raw) in rows:
            if not condition.startswith("seed=") or "/" not in condition:
                raise ValueError(f"unregistered condition format {condition}")
            prefix, arm = condition.split("/", 1)
            seed = int(prefix[5:])
            if seed not in range(FIRST, FIRST + SEEDS) or arm not in LABELS:
                raise ValueError(f"unregistered random ecology seed/branch {condition}")
            if arm in by_seed.get(seed, {}):
                raise ValueError(f"duplicate matched arm {condition}")
            if not revision or epochs != EPOCHS or quota != QUOTA:
                raise ValueError(f"invalid source or time scope for {condition}")
            if analyze.digest(initial_raw) != initial_sha or analyze.digest(final_raw) != final_sha:
                raise ValueError(f"bad initial/final digest {condition}")
            if seed in initial_hashes and initial_hashes[seed] != initial_sha:
                raise ValueError(f"unmatched prefork state for seed {seed}")
            initial_hashes[seed] = initial_sha
            source_revisions.add(revision)
            initial, final = json.loads(initial_raw), json.loads(final_raw)
            if (initial["total_storage_limit"] != 192 or not initial["strict_accounting"]
                    or initial["storage_limit"] != 32
                    or initial["max_nodes_per_process"] != 40
                    or analyze.occupied_units(initial) > 192
                    or analyze.occupied_units(final) > 192):
                raise ValueError(f"strict world limits violated by {condition}")
            for id in (1, 2, 3):
                if initial["processes"][str(id)]["credits"] != 47:
                    raise ValueError("unexpected pre-fork normal remaining credit")
                obj = initial["graph_objects"].get(str(id))
                if obj is None or obj["owner"] != id or len(obj["nodes"]) != 1:
                    raise ValueError("missing prespecified neutral graph opportunity")
            if json.loads(actions_raw) != schedule(arm, initial):
                raise ValueError(f"unregistered intervention / changed control: {condition}")
            attempts = json.loads(attempts_raw)
            if len(attempts) != EPOCHS or any(
                not isinstance(value, int) or value < 0 or value > QUOTA
                for value in attempts
            ):
                raise ValueError(f"invalid attempted execution counts: {condition}")
            records = frames.get(run_id, [])
            if (len(records) != EPOCHS
                    or any(epoch != pos for pos, (epoch, _, _) in enumerate(records))):
                raise ValueError(f"missing/duplicate epoch snapshot: {condition}")
            if records[-1][1:] != (final_sha, final_raw):
                raise ValueError(f"last snapshot differs from final: {condition}")
            snapshots = [json.loads(raw) for _, _, raw in records]
            previous_events = initial["events"]
            for snapshot in snapshots:
                if snapshot["events"][: len(previous_events)] != previous_events:
                    raise ValueError(f"event ancestry diverged for {condition}")
                if analyze.occupied_units(snapshot) > 192:
                    raise ValueError(f"capacity overflow at an intermediate epoch: {condition}")
                previous_events = snapshot["events"]
            epochs_events, postfork = epoch_graph_events(initial, snapshots)
            if arm == "zero-credit" and (
                any(attempts)
                or any(analyze.internal_process(event) is not None for event in postfork)
            ):
                raise ValueError(f"zero-credit branch executed CIL: {condition}")
            expected_drop = {
                "control": 0, "sham-drop": 0, "refill": 0,
                "drop-g1-refill": 1, "drop-g2-refill": 1,
                "drop-both-refill": 2, "restore-g1-refill": 1,
                "zero-credit": 0,
            }[arm]
            if _external_count(postfork, "DropGraph") != expected_drop:
                raise ValueError(f"actual graph ablation differed: {condition}")
            if _external_count(postfork, "PutGraph") != int(arm == "restore-g1-refill"):
                raise ValueError(f"externally restored graph missing: {condition}")
            expected_credits = (6 if arm in REFILL_ARMS
                                else 3 if arm == "zero-credit" else 0)
            if _external_count(postfork, "SetCredits") != expected_credits:
                raise ValueError(f"external credit subsidy not as preregistered: {condition}")
            activities = []
            copy_counts = []
            read_counts = []
            write_counts = []
            reciprocal = []
            for start, stop in WINDOWS:
                group = epochs_events[start:stop]
                flat = [event for events in group for event in events]
                copied, read, written = _successes(flat)
                pairs, _ = reciprocal_pairs(group)
                reciprocal.append(bool(pairs))
                copy_counts.append(copied)
                read_counts.append(read)
                write_counts.append(written)
                activities.append(sum(attempts[start:stop]) > 0)
            by_seed.setdefault(seed, {})[arm] = {
                "initial": initial, "final": final, "attempts": attempts,
                "activities": activities,
                "cross_owner_copies": copy_counts,
                "native_reads": read_counts,
                "native_writes": write_counts,
                "reciprocal": reciprocal,
                "new_nonempty_graph": newly_populated_graph(initial, final, postfork),
            }
        if (len(by_seed) != SEEDS or len(source_revisions) != 1
                or any(set(arms) != set(LABELS) for arms in by_seed.values())):
            raise ValueError("incomplete source-pinned, balanced 96-world ecology")
        seed_counts: Counter = Counter()
        total_events: Counter = Counter()
        for seed in sorted(by_seed):
            matched = by_seed[seed]
            if simulated_state(matched["control"]["final"]) != simulated_state(matched["sham-drop"]["final"]):
                raise ValueError(f"sham changed simulated world: seed {seed}")
            for arm in LABELS:
                record = matched[arm]
                for w in range(3):
                    seed_counts[f"{arm}.W{w + 1}.active"] += record["activities"][w]
                    seed_counts[f"{arm}.W{w + 1}.cross_owner_copy"] += (
                        record["cross_owner_copies"][w] > 0
                    )
                    seed_counts[f"{arm}.W{w + 1}.native_read"] += (
                        record["native_reads"][w] > 0
                    )
                    seed_counts[f"{arm}.W{w + 1}.native_write"] += (
                        record["native_writes"][w] > 0
                    )
                    seed_counts[f"{arm}.W{w + 1}.both_io"] += (
                        record["native_reads"][w] > 0
                        and record["native_writes"][w] > 0
                    )
                    seed_counts[f"{arm}.W{w + 1}.reciprocal_copy"] += record["reciprocal"][w]
                    total_events[f"{arm}.W{w + 1}.cross_owner_copies"] += record["cross_owner_copies"][w]
                    total_events[f"{arm}.W{w + 1}.native_reads"] += record["native_reads"][w]
                    total_events[f"{arm}.W{w + 1}.native_writes"] += record["native_writes"][w]
                seed_counts[f"{arm}.new_nonempty_graph"] += record["new_nonempty_graph"]
            for endpoint, fn in (
                ("late_active", lambda x: x["activities"][2]),
                ("late_copy", lambda x: x["cross_owner_copies"][2] > 0),
            ):
                refill_only = fn(matched["refill"]) and not fn(matched["control"])
                control_only = fn(matched["control"]) and not fn(matched["refill"])
                seed_counts[f"paired.{endpoint}.refill_only"] += refill_only
                seed_counts[f"paired.{endpoint}.control_only"] += control_only
        metrics = {}
        for name, n in sorted(seed_counts.items()):
            lo, hi = wilson_95(n, SEEDS)
            metrics[name] = {
                "seed_worlds": n,
                "denominator": SEEDS,
                "wilson_95": [round(lo, 6), round(hi, 6)],
            }
        return {
            "study": "CL-1 Study 22",
            "status": "fresh-seed compute-subsidy feasibility, not autonomous maintenance",
            "source_revision": next(iter(source_revisions)),
            "independent_seed_worlds": SEEDS,
            "matched_runs": len(rows),
            "hashed_epoch_snapshots": sum(map(len, frames.values())),
            "exact_epoch_replay": "Rust full-state/epoch source-pinned CI",
            "registered_metrics": metrics,
            "charged_event_counts": dict(sorted(total_events.items())),
            "late_refill_comparison": {
                k: seed_counts[k] for k in (
                    "paired.late_active.refill_only",
                    "paired.late_active.control_only",
                    "paired.late_copy.refill_only",
                    "paired.late_copy.control_only",
                )
            },
            "external_recharge_is_not_internal_energy_acquisition": True,
            "external_graph_restoration_is_not_native_repair": True,
            "source_sensitive_functional_dependency": "NOT EVALUABLE",
            "endogenous_functional_recovery": "NOT EVALUABLE",
            "organizational_closure": "NOT EVALUABLE",
            "life_claim": "NONE",
            "caveat": "Fresh independent seeds but externally initialized programs and supplied credits. Copying, native cell I/O or late activity do not establish closed causal maintenance.",
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
        "snapshots": report["hashed_epoch_snapshots"],
        "paired_late": report["late_refill_comparison"],
        "recovery": report["endogenous_functional_recovery"],
    }))


if __name__ == "__main__":
    main()
