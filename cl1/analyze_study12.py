#!/usr/bin/env python3
"""Read-only CL-1 Study 12: internally generated graph-object causal ablation audit."""
from __future__ import annotations

import argparse
import json
import sqlite3
from pathlib import Path

import analyze
from analyze_study02 import wilson_95

FIRST = 1_810_000
SEEDS = 96
EPOCHS = 16
QUOTA = 12
LABELS = ("control", "sham-drop", "target-drop", "disable-owner", "zero-credit")


def chosen_graph(initial: dict) -> tuple[int, int] | None:
    """Preregistered pre-treatment choice: smallest nonempty graph handle."""
    objects = initial["graph_objects"]
    candidates = sorted(
        (int(key), int(obj["owner"]))
        for key, obj in objects.items()
        if obj["nodes"]
    )
    return candidates[0] if candidates else None


def semantic_state(world: dict) -> dict:
    """Exclude only external events/ticks: no direct object should be an 'indirect' effect."""
    return {
        "processes": world["processes"],
        "graph_objects": world["graph_objects"],
        "cells": world["cells"],
        "compute_pool": world["compute_pool"],
    }


def process_state(world: dict, excluded_owner: int | None = None) -> dict:
    if excluded_owner is None:
        return world["processes"]
    return {
        pid: value for pid, value in world["processes"].items()
        if pid != str(excluded_owner)
    }


def external_ablation_executed(world: dict) -> bool:
    return any(
        event["origin"] == "External"
        and isinstance(event["kind"], dict)
        and "Intervened" in event["kind"]
        and "DropGraph" in event["kind"]["Intervened"]
        for event in world["events"]
    )


def postdrop_nonempty_owner_graph(initial: dict, final: dict, owner: int) -> bool:
    before = set(initial["graph_objects"])
    return any(
        graph_id not in before and obj["owner"] == owner and len(obj["nodes"]) > 0
        for graph_id, obj in final["graph_objects"].items()
    )



def successful_exchange(events: list[dict]) -> tuple[int, int]:
    """Only actual successful VM events, with internal actor and pre-action owners."""
    cross_owner_copies = 0
    different_owner_transfers = 0
    for event in events:
        kind = event["kind"]
        if not isinstance(kind, dict):
            continue
        if "GraphCopied" in kind:
            actor = analyze.internal_process(event)
            info = kind["GraphCopied"]
            if actor is None or info["target_owner"] != actor:
                raise ValueError("invalid successful graph-copy provenance")
            cross_owner_copies += info["source_owner"] != actor
        elif "GraphTransferred" in kind:
            actor = analyze.internal_process(event)
            info = kind["GraphTransferred"]
            if actor is None or info["former_owner"] != actor:
                raise ValueError("invalid successful graph-transfer provenance")
            different_owner_transfers += info["new_owner"] != actor
    return cross_owner_copies, different_owner_transfers


def audit(db: Path) -> dict:
    con = sqlite3.connect(f"{db.resolve().as_uri()}?mode=ro", uri=True)
    try:
        rows = con.execute(
            "SELECT id,source_revision,condition,initial_sha256,initial_json,"
            "actions_json,epoch_count,quota,attempts_json,final_sha256,final_json "
            "FROM epoch_runs ORDER BY id"
        ).fetchall()
        if len(rows) != SEEDS * len(LABELS):
            raise ValueError(f"unregistered or incomplete run count: {len(rows)}")
        checkpoints = {}
        for run_id, epoch, digest, snapshot in con.execute(
            "SELECT run_id,epoch,state_sha256,state_json FROM epoch_snapshots "
            "ORDER BY run_id,epoch"
        ):
            if analyze.digest(snapshot) != digest or epoch < 0 or epoch >= EPOCHS:
                raise ValueError(f"invalid epoch snapshot hash or index: {run_id}")
            checkpoints.setdefault(run_id, []).append((epoch, digest, snapshot))

        by_seed = {}
        hash_by_seed = {}
        revisions = set()
        for rid, revision, label, initial_hash, initial_json, actions_json, epochs, quota, attempts_json, final_hash, final_json in rows:
            if not label.startswith("seed=") or "/" not in label:
                raise ValueError(f"invalid label: {label}")
            prefix, condition = label.split("/", 1)
            seed = int(prefix[5:])
            if (
                seed not in range(FIRST, FIRST + SEEDS)
                or condition not in LABELS
                or condition in by_seed.get(seed, {})
            ):
                raise ValueError(f"unexpected or duplicate condition: {label}")
            if (
                not revision or analyze.digest(initial_json) != initial_hash
                or analyze.digest(final_json) != final_hash or epochs != EPOCHS or quota != QUOTA
            ):
                raise ValueError(f"mismatched run hash or experiment parameters: {label}")
            if seed in hash_by_seed and hash_by_seed[seed] != initial_hash:
                raise ValueError(f"unmatched prefork initial world: seed={seed}")
            hash_by_seed[seed] = initial_hash
            revisions.add(revision)
            initial, final = json.loads(initial_json), json.loads(final_json)
            if (
                not initial["strict_accounting"] or initial["total_storage_limit"] != 192
                or analyze.occupied_units(initial) != 99
                or analyze.occupied_units(final) > 192 or len(initial["processes"]) < 3
            ):
                raise ValueError(f"invalid capacity: {label}")
            attempts = json.loads(attempts_json)
            if len(attempts) != EPOCHS or any(not 0 <= x <= QUOTA for x in attempts):
                raise ValueError(f"invalid CIL count per epoch: {label}")
            snapshots = checkpoints.get(rid, [])
            if (
                len(snapshots) != EPOCHS
                or any(index != epoch for index, (epoch, _, _) in enumerate(snapshots))
                or snapshots[-1][2] != final_json or snapshots[-1][1] != final_hash
            ):
                raise ValueError(f"incomplete or mismatched epoch states: {label}")
            target = chosen_graph(initial)
            prefix_events = initial["events"]
            created = [e for e in prefix_events if isinstance(e["kind"], dict)
                       and "GraphCreated" in e["kind"]]
            node_copies = [e for e in prefix_events if isinstance(e["kind"], dict)
                           and "GraphNodeCopied" in e["kind"]]
            if len(created) != 3 or len(node_copies) != 3:
                raise ValueError(f"expected three externally scaffolded graph origins: {label}")
            for event in created:
                actor = analyze.internal_process(event)
                if actor is None or event["kind"]["GraphCreated"]["owner"] != actor:
                    raise ValueError(f"graph creation lacks actor source: {label}")
            for event in node_copies:
                actor = analyze.internal_process(event)
                item = event["kind"]["GraphNodeCopied"]
                if actor is None or item["owner"] != actor or item["source_process"] != actor:
                    raise ValueError(f"process-code node copy attribution invalid: {label}")
            # This is an externally engineered, symmetrical three-process
            # opportunity scaffold, not spontaneous object creation.
            for pid in (1, 2, 3):
                graph = initial["graph_objects"].get(str(pid))
                if (
                    graph is None
                    or graph["owner"] != pid
                    or len(graph["nodes"]) != 1
                    or ((pid + 1) % 3 + 1) not in graph["readers"]
                    or initial["cells"].get(str(pid - 1)) != pid
                ):
                    raise ValueError(f"scaffold calibration failed: {label}")
            if target != (1, 1):
                raise ValueError(f"target not structurally exposed: {label}")
            action_list = json.loads(actions_json)
            if condition == "control":
                if action_list:
                    raise ValueError(f"control has external intervention: {label}")
            elif condition == "zero-credit":
                if (
                    len(action_list) != len(initial["processes"])
                    or sum(attempts) != 0
                    or any(analyze.internal_process(e) is not None
                           for e in final["events"][len(initial["events"]):])
                ):
                    raise ValueError(f"zero-credit control violation: {label}")
                if any(
                    a["epoch"] != 0 or "SetCredits" not in a["action"]
                    or a["action"]["SetCredits"]["credits"] != 0
                    for a in action_list
                ):
                    raise ValueError(f"incorrect external zero-credit schedule: {label}")
            else:
                if len(action_list) != 1 or action_list[0]["epoch"] != 0:
                    raise ValueError(f"incorrect intervention: {label}")
                action = action_list[0]["action"]
                if condition == "sham-drop" and action != {"DropGraph": {"graph": 0}}:
                    raise ValueError(f"unexpected sham target: {label}")
                if condition == "target-drop" and action != {
                    "DropGraph": {"graph": target[0] if target else 0}
                }:
                    raise ValueError(f"unexpected precommitted target: {label}")
                if condition == "disable-owner" and action != {
                    "Disable": {"process": target[1] if target else 9999}
                }:
                    raise ValueError(f"unexpected owner: {label}")
            by_seed.setdefault(seed, {})[condition] = {
                "initial": initial,
                "final": final,
                "attempts": sum(attempts),
                "attempts_by_epoch": attempts,
                "target": target,
            }
        if len(by_seed) != SEEDS or len(revisions) != 1 or any(
            set(arms) != set(LABELS) for arms in by_seed.values()
        ):
            raise ValueError("missing independent seed or source revision mismatch")

        counts = {
            "eligible_target": 0,
            "drop_exposed": 0,
            "indirect_process_impact": 0,
            "nonowner_process_impact": 0,
            "postdrop_object_creation": 0,
            "sham_mismatch": 0,
            "no_target": 0,
        }
        for seed in sorted(by_seed):
            arms = by_seed[seed]
            initial = arms["control"]["initial"]
            control = arms["control"]["final"]
            sham = arms["sham-drop"]["final"]
            dropped = arms["target-drop"]["final"]
            target = arms["control"]["target"]
            if semantic_state(control) != semantic_state(sham):
                counts["sham_mismatch"] += 1
            if target is None:
                counts["no_target"] += 1
                if external_ablation_executed(dropped):
                    raise ValueError(f"unregistered target deletion in no-target seed {seed}")
                continue
            counts["eligible_target"] += 1
            if not external_ablation_executed(dropped):
                raise ValueError(f"eligible graph was not deleted: seed={seed}")
            counts["drop_exposed"] += 1
            owner = target[1]
            counts["indirect_process_impact"] += process_state(control) != process_state(dropped)
            counts["nonowner_process_impact"] += (
                process_state(control, owner) != process_state(dropped, owner)
            )
            counts["postdrop_object_creation"] += postdrop_nonempty_owner_graph(initial, dropped, owner)
        if counts["sham_mismatch"]:
            raise ValueError(f"sham changed simulated state in {counts['sham_mismatch']} worlds")
        postfork_created_graph_events = 0
        postfork_process_code_copy_events = 0
        postfork_node_const_edits = 0
        postfork_node_next_edits = 0
        worlds_with_new_graph = 0
        worlds_with_process_code_copy = 0
        worlds_with_graph_edit = 0
        worlds_with_intergraph_copy_and_edit = 0
        successful_copies = 0
        successful_transfers = 0
        seeds_with_exchange = 0
        seeds_with_random_copy = 0
        seeds_with_random_transfer = 0
        seeds_with_modified_graph = 0
        seeds_with_late_activity = 0
        postfork_traps_in_control = 0
        for seed, arms in by_seed.items():
            control = arms["control"]
            prefix_length = len(control["initial"]["events"])
            new_events = control["final"]["events"][prefix_length:]
            # The first 9 CIL nodes never contain GraphCopyFrom/GraphTransfer.
            # These successful events are sourced from externally seeded
            # RANDOM continuation code, not an autonomous goal or invention.
            for event in new_events:
                kind = event["kind"]
                if isinstance(kind, dict) and (
                    "GraphCopied" in kind or "GraphTransferred" in kind
                ) and event["pc"] < 9:
                    raise ValueError(f"unexpected prewritten graph exchange: seed={seed}")
            def transitions(kind_name):
                return [e for e in new_events if isinstance(e["kind"], dict)
                        and kind_name in e["kind"]]
            new_graphs = transitions("GraphCreated")
            code_copies = transitions("GraphNodeCopied")
            edits = transitions("GraphNodeEdited")
            for event in new_graphs:
                actor = analyze.internal_process(event)
                item = event["kind"]["GraphCreated"]
                if actor is None or item["owner"] != actor:
                    raise ValueError(f"invalid internal graph creation: seed={seed}")
                if str(item["graph"]) in control["initial"]["graph_objects"]:
                    raise ValueError(f"graph creation reused an existing handle: {seed}")
            for event in code_copies:
                actor = analyze.internal_process(event)
                item = event["kind"]["GraphNodeCopied"]
                if actor is None or item["owner"] != actor or item["source_process"] != actor:
                    raise ValueError(f"invalid process code origin: seed={seed}")
            for event in edits:
                actor = analyze.internal_process(event)
                item = event["kind"]["GraphNodeEdited"]
                if actor is None or item["owner"] != actor or item["edit"] not in ("next", "const"):
                    raise ValueError(f"invalid generic code edit: seed={seed}")
            postfork_created_graph_events += len(new_graphs)
            postfork_process_code_copy_events += len(code_copies)
            postfork_node_const_edits += sum(e["kind"]["GraphNodeEdited"]["edit"] == "const" for e in edits)
            postfork_node_next_edits += sum(e["kind"]["GraphNodeEdited"]["edit"] == "next" for e in edits)
            worlds_with_new_graph += len(new_graphs) > 0
            worlds_with_process_code_copy += len(code_copies) > 0
            worlds_with_graph_edit += len(edits) > 0
            copied, transferred = successful_exchange(new_events)
            worlds_with_intergraph_copy_and_edit += copied > 0 and len(edits) > 0
            successful_copies += copied
            successful_transfers += transferred
            seeds_with_exchange += copied > 0 or transferred > 0
            seeds_with_random_copy += copied > 0
            seeds_with_random_transfer += transferred > 0
            seeds_with_late_activity += any(
                attempts > 0 for attempts in control["attempts_by_epoch"][8:]
            )
            postfork_traps_in_control += sum(
                isinstance(event["kind"], dict) and "Trapped" in event["kind"]
                for event in new_events
                if analyze.internal_process(event) is not None
            )
            initial_graphs = control["initial"]["graph_objects"]
            final_graphs = control["final"]["graph_objects"]
            seeds_with_modified_graph += any(
                key in final_graphs and final_graphs[key]["nodes"] != obj["nodes"]
                for key, obj in initial_graphs.items()
            )
            zero_credit = arms["zero-credit"]
            credit_prefix = len(zero_credit["initial"]["events"])
            if successful_exchange(zero_credit["final"]["events"][credit_prefix:]) != (0, 0):
                raise ValueError(f"zero-credit arm produced graph transition: seed={seed}")
        lo_exchange, hi_exchange = wilson_95(seeds_with_exchange, SEEDS)
        metrics = {}
        for key, value in counts.items():
            lo, hi = wilson_95(value, SEEDS)
            metrics[key] = {
                "seeds": value,
                "denominator": SEEDS,
                "wilson_95": [round(lo, 6), round(hi, 6)],
            }
        return {
            "study": "CL-1 Study 12",
            "status": "observer-only graph origin and code edit provenance; NOT autonomous maintenance evidence",
            "source_revision": next(iter(revisions)),
            "independent_seed_worlds": SEEDS,
            "matched_runs": len(rows),
            "hashed_epoch_snapshots": sum(map(len, checkpoints.values())),
            "exact_replay": "verified by Rust lab for each full run and each epoch",
            "metrics": metrics,
            "successful_cross_owner_copy_events": successful_copies,
            "successful_different_owner_transfer_events": successful_transfers,
            "seeds_with_successful_cross_owner_exchange": {
                "seeds": seeds_with_exchange, "denominator": SEEDS,
                "wilson_95": [round(lo_exchange, 6), round(hi_exchange, 6)],
            },
            "scaffold_opportunity_source": "EXTERNAL NINE-NODE PREFIX; NO PREWIRED COPY OR TRANSFER",
            "successful_random_cross_owner_copy_events": successful_copies,
            "successful_random_graph_transfer_events": successful_transfers,
            "seeds_with_random_copy": seeds_with_random_copy,
            "seeds_with_random_transfer": seeds_with_random_transfer,
            "seeds_with_original_graph_mutation": seeds_with_modified_graph,
            "seeds_with_activity_in_late_epochs": seeds_with_late_activity,
            "postfork_trapped_attempts_in_control": postfork_traps_in_control,
            "graph_lineage_observer_coverage": "GraphNew, GraphCopyNode, GraphCopyFrom, GraphSetNext, GraphSetConst; not GraphEncode/Replace/Launch",
            "postfork_graph_created_events": postfork_created_graph_events,
            "postfork_process_code_copy_events": postfork_process_code_copy_events,
            "postfork_graph_const_edits": postfork_node_const_edits,
            "postfork_graph_next_edits": postfork_node_next_edits,
            "worlds_with_graph_created": worlds_with_new_graph,
            "worlds_with_process_code_copy": worlds_with_process_code_copy,
            "worlds_with_graph_edit": worlds_with_graph_edit,
            "worlds_with_intergraph_copy_and_edit": worlds_with_intergraph_copy_and_edit,
            "organizational_closure": "NOT EVALUABLE",
            "functional_recovery": "NOT EVALUABLE",
            "life_claim": "NONE",
            "caveat": "Lineage covers selected graph operations only; process code was initially investigator generated. These observations do not demonstrate semantic novelty, maintenance or life.",
        }
    finally:
        con.close()


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("database", type=Path)
    parser.add_argument("--json", type=Path, required=True)
    args = parser.parse_args()
    report = audit(args.database)
    args.json.write_text(json.dumps(report, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
    print(json.dumps({
        "study": report["study"],
        "worlds": report["independent_seed_worlds"],
        "runs": report["matched_runs"],
        "eligible": report["metrics"]["eligible_target"]["seeds"],
        "indirect_impact": report["metrics"]["indirect_process_impact"]["seeds"],
        "cross_owner_exchange_seeds": report["seeds_with_successful_cross_owner_exchange"]["seeds"],
    }))


if __name__ == "__main__":
    main()
