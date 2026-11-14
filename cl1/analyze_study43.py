#!/usr/bin/env python3
"""Study43: POST-HOC read-only native child source/function ancestry analysis.

Study41/42 already revealed GraphCreated-only successful child events in a
preliminary inspection. This is explanatory, not prospective life evidence.
"""
from __future__ import annotations

import argparse
import json
import sqlite3
from collections import Counter, defaultdict
from pathlib import Path

import analyze
import analyze_study41
import analyze_study42

HISTORY = (
    ("41", 3_100_000, "d52840573e4ee0ebee889ab008d964f5d1272e62"),
    ("42", 3_200_000, "e2b74edb2f3b39f895f4d69410dde15431bc6309"),
)
ARMS = ("ambient-general", "ambient-only", "closed-general",
        "zero-credit-ambient-general")


def key(kind) -> str:
    return next(iter(kind)) if isinstance(kind, dict) else kind


def internal(event: dict) -> bool:
    return analyze.internal_process(event) == event["process"]


def scan_child_events(state: dict, initial: dict) -> dict:
    """Read all native activity; reject children with no valid parent launch."""
    launched = {}
    kind_counts = Counter()
    child_epochs = defaultdict(set)
    child_unassisted_epochs = defaultdict(set)
    parent_output_prelaunch = set()
    child_output = set()
    parent_cell_keys = defaultdict(set)
    children_having_shared_parent_output = set()
    child_codes = []
    seen_child_native_events = defaultdict(int)
    previous_len = len(initial["events"])
    all_native_child_count = 0
    all_actions = defaultdict(list)
    # Exact epoch snapshots are handled in audit_one; state contains final
    # event stream only. Caller enriches per-event epoch and starter flag.
    for event in state["events"][previous_len:]:
        typ = key(event["kind"])
        actor = event["process"]
        if typ == "GraphLaunched" and internal(event):
            info = event["kind"]["GraphLaunched"]
            child = info["child_process"]
            if (info["parent_process"] != actor or child in launched
                    or child <= 3 or str(child) not in state["processes"]):
                raise ValueError("invalid child launch identity or parent provenance")
            launched[child] = {
                "parent": actor, "tick": event["tick"],
                "source_graph": info["source_graph"],
                "source_owner": info.get("source_owner"),
                "moved_nodes": info["moved_nodes"],
                "code_nodes": state["processes"][str(child)]["code"]["nodes"],
            }
            child_codes.append({
                "child": child, "parent": actor,
                "source_graph": info["source_graph"],
                "code_size": len(state["processes"][str(child)]["code"]["nodes"]),
                "first_operators": [
                    key(op) for op in state["processes"][str(child)]["code"]["nodes"][:4]
                ],
            })
        if internal(event) and actor <= 3 and typ == "CellWritten":
            parent_cell_keys[actor].add(event["kind"]["CellWritten"]["key"])
        if actor > 3 and internal(event):
            if actor not in launched or event["tick"] <= launched[actor]["tick"]:
                raise ValueError("native child event before valid GraphLaunched ancestry")
            all_native_child_count += 1
            seen_child_native_events[actor] += 1
            kind_counts[typ] += 1
            if typ not in ("Halted", "Trapped"):
                child_epochs[actor].add(event["_study43_epoch"])
                if not event["_study43_starter"]:
                    child_unassisted_epochs[actor].add(event["_study43_epoch"])
            if typ == "CellWritten":
                child_output.add(actor)
                parent = launched[actor]["parent"]
                address = event["kind"]["CellWritten"]["key"]
                if address in parent_cell_keys[parent]:
                    children_having_shared_parent_output.add(actor)
    successful_kinds = sum(n for k, n in kind_counts.items()
                           if k not in ("Halted", "Trapped"))
    return {
        "worlds_with_launch": bool(launched),
        "children_launched": len(launched),
        "worlds_with_child_success": successful_kinds > 0,
        "native_child_successful_events": successful_kinds,
        "worlds_with_child_cell_write": bool(child_output),
        "children_with_cell_write": len(child_output),
        "worlds_child_reuses_parent_output_key": bool(children_having_shared_parent_output),
        "worlds_with_child_success_2_epochs": any(
            len(epochs) >= 2 for epochs in child_epochs.values()
        ),
        "worlds_with_child_no_starter_2_W3_epochs": any(
            len([e for e in epochs if e >= 16]) >= 2
            for epochs in child_unassisted_epochs.values()
        ),
        "worlds_with_parent_write_before_launch": any(
            True for child, entry in launched.items()
            for e in state["events"] if internal(e)
            and e["process"] == entry["parent"]
            and key(e["kind"]) == "CellWritten"
            and e["tick"] < entry["tick"]
        ),
        "native_child_event_counts": dict(kind_counts),
        "first_child_code_descriptors": child_codes[:8],
    }


def audit_one(path: Path, study: str, seed_start: int, revision: str) -> dict:
    original = (
        analyze_study41.audit(path, revision)
        if study == "41" else analyze_study42.audit(path, revision)
    )
    if original["independent_worlds"] != 128 or original["hashed_epoch_snapshots"] != 12288:
        raise ValueError("historical original auditor failed expected cohort checks")
    db = sqlite3.connect(f"{path.resolve().as_uri()}?mode=ro", uri=True)
    try:
        counts = Counter()
        samples = []
        per_seed = defaultdict(dict)
        total_snapshots = 0
        for (run_id, condition, raw, actions_raw, attempts_raw, final_raw, src) in db.execute(
            "SELECT id,condition,initial_json,actions_json,attempts_json,"
            "final_json,source_revision FROM epoch_runs ORDER BY id"
        ):
            if src != revision:
                raise ValueError("archived source revision mismatch")
            label, arm = condition.split("/", 1)
            seed = int(label.removeprefix("seed="))
            if (not seed_start <= seed < seed_start + 128
                    or arm not in ARMS or arm in per_seed[seed]):
                raise ValueError("unregistered seed/duplicate arm")
            initial, final = json.loads(raw), json.loads(final_raw)
            actions, attempts = json.loads(actions_raw), json.loads(attempts_raw)
            if len(attempts) != 24:
                raise ValueError("wrong native epoch count")
            prev = initial["events"]
            event_epochs = {}
            starters = set()
            snapshot_count = 0
            for epoch, digest, raw_state in db.execute(
                "SELECT epoch,state_sha256,state_json FROM epoch_snapshots "
                "WHERE run_id=? ORDER BY epoch", (run_id,)
            ):
                if epoch != snapshot_count or analyze.digest(raw_state) != digest:
                    raise ValueError("missing original epoch hash or ordering")
                state = json.loads(raw_state)
                if state["events"][:len(prev)] != prev:
                    raise ValueError("native event log prefix rewritten")
                epoch_actions = [a for a in actions if a["epoch"] == epoch]
                new_events = state["events"][len(prev):]
                if len(new_events) != len(epoch_actions) + attempts[epoch]:
                    raise ValueError("unregistered external action / native ledger")
                for e in new_events[:len(epoch_actions)]:
                    if e["origin"] != "External":
                        raise ValueError("host intervention falsely native")
                has_starter = any(
                    "SetCredits" in action["action"]
                    and action["action"]["SetCredits"]["credits"] == 1
                    for action in epoch_actions
                )
                if has_starter:
                    starters.add(epoch)
                for e in new_events:
                    event_epochs[(e["tick"], e["process"], e["pc"], json.dumps(e["kind"]))] = (
                        epoch, has_starter
                    )
                prev = state["events"]
                snapshot_count += 1
                total_snapshots += 1
            if prev != final["events"] or snapshot_count != 24:
                raise ValueError("missing final native event/snapshot correspondence")
            # Enrich temporary local parsed events with epoch without changing
            # any archived original SQLite bytes or hash inputs.
            enriched = json.loads(json.dumps(final))
            for event in enriched["events"][len(initial["events"]):]:
                item = (event["tick"], event["process"], event["pc"],
                        json.dumps(event["kind"]))
                if item not in event_epochs:
                    raise ValueError("native child event lacks epoch chronology")
                epoch, starter = event_epochs[item]
                event["_study43_epoch"] = epoch
                event["_study43_starter"] = starter
            result = scan_child_events(enriched, initial)
            per_seed[seed][arm] = result
            for key_name, value in result.items():
                if isinstance(value, bool):
                    counts[f"{arm}.{key_name}"] += value
            for name, value in result["native_child_event_counts"].items():
                counts[f"{arm}.child_{name}"] += value
            counts[f"{arm}.native_children_launched"] += result["children_launched"]
            counts[f"{arm}.native_child_successful_events"] += result["native_child_successful_events"]
            if result["worlds_with_child_success"] and len(samples) < 8:
                samples.append({
                    "seed": seed, "arm": arm,
                    "children": result["first_child_code_descriptors"][:4],
                    "events": result["native_child_event_counts"],
                })
        if (len(per_seed) != 128 or any(set(v) != set(ARMS) for v in per_seed.values())
                or total_snapshots != 12288):
            raise ValueError("incomplete original paired child population")
        original_secondary = original["secondary"]
        for arm in ARMS:
            if counts[f"{arm}.worlds_with_launch"] != original_secondary.get(
                f"{arm}.child_launched", 0
            ):
                raise ValueError("historical native child launch count differs")
            if counts[f"{arm}.worlds_with_child_success"] != original_secondary.get(
                f"{arm}.child_success", 0
            ):
                raise ValueError("historical native child success count differs")
        return {
            "cohort": study,
            "source_revision": revision,
            "new_independent_worlds": 0,
            "reused_independent_worlds": 128,
            "matched_runs": 512,
            "validated_original_epoch_hashes": total_snapshots,
            "metrics": dict(sorted(counts.items())),
            "sample_child_ancestry": samples,
            "life_claim": "NONE",
        }
    finally:
        db.close()


def main() -> None:
    p = argparse.ArgumentParser(description=__doc__)
    p.add_argument("study41", type=Path)
    p.add_argument("study42", type=Path)
    p.add_argument("--json", type=Path, required=True)
    args = p.parse_args()
    cases = {}
    for study, start, revision in HISTORY:
        path = args.study41 if study == "41" else args.study42
        cases[study] = audit_one(path, study, start, revision)
    report = {
        "study": "CL-1 Study43 post-hoc child function/ancestry audit",
        "new_independent_worlds": 0,
        "reused_distinct_worlds": 256,
        "matched_runs": 1024,
        "validated_epoch_hashes": 24576,
        "prior_child_GraphCreated_only_observation_disclosed": True,
        "cohorts": cases,
        "life_claim": "NONE",
    }
    args.json.write_text(json.dumps(report, indent=2) + "\n", encoding="utf-8")
    print(json.dumps({
        "study": report["study"],
        "new_worlds": 0,
        "study41": {k: v for k, v in cases["41"]["metrics"].items()
                    if k.startswith("ambient-general.")},
        "study42": {k: v for k, v in cases["42"]["metrics"].items()
                    if k.startswith("ambient-general.")},
        "life_claim": "NONE",
    }, indent=2))


if __name__ == "__main__":
    main()
