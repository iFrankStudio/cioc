#!/usr/bin/env python3
"""Independent CL-1 Study 01 reader. Engineering diagnostics, never a life classifier."""
from __future__ import annotations

import argparse
import hashlib
import json
import sqlite3
from collections import Counter
from pathlib import Path

FIRST_SEED = 610000
REPLICAS = 64
CONDITIONS = ("control", "zero-credit", "disable-p1", "lesion-cell", "sham-cell")
INITIAL_PROCESSES = 3
INITIAL_NODES = 48
TOTAL_UNITS = 160
LIMIT = 96


def digest(s: str) -> str:
    return hashlib.sha256(s.encode("utf-8")).hexdigest()


def internal_process(event: dict) -> int | None:
    origin = event["origin"]
    if not isinstance(origin, dict) or "Internal" not in origin:
        return None
    return int(origin["Internal"])


def opcode_at_initial_event(initial: dict, event: dict) -> str | None:
    process_id = internal_process(event)
    if process_id is None:
        return None
    process = initial["processes"].get(str(process_id))
    if process is None:
        return None
    pc = event["pc"]
    nodes = process["code"]["nodes"]
    if not isinstance(pc, int) or not 0 <= pc < len(nodes):
        return None
    op = nodes[pc]
    if isinstance(op, dict) and len(op) == 1:
        return next(iter(op))
    if isinstance(op, str):
        return op
    return None


GRAPH_OPS = {
    "CloneNode", "PatchNext", "PatchConst", "GraphNew", "GraphCopyNode",
    "GraphSetNext", "GraphSetConst", "GraphEncode", "GraphReplace",
    "GraphLaunch", "GraphDrop",
}


def occupied_units(state: dict) -> int:
    processes = state["processes"]
    graphs = state["graph_objects"]
    count = len(state["cells"])
    count += sum(len(p["code"]["nodes"]) for p in processes.values())
    count += sum(1 + len(g["nodes"]) for g in graphs.values())
    if state["strict_accounting"]:
        count += len(processes)
        count += sum(len(g["readers"]) for g in graphs.values())
    return count


def semantic_signature(state: dict) -> dict:
    """Remove only the two directly manipulated keys and event history.

    All registers, code, active/stop flags, compute accounts, objects and OTHER
    shared cells remain included. Different externally written keys cannot
    manufacture an apparent indirect effect by themselves.
    """
    return {
        "processes": state["processes"],
        "graphs": state["graph_objects"],
        "compute_pool": state["compute_pool"],
        "cells": {k: v for k, v in state["cells"].items() if k not in {"0", "1000000"}},
    }


def metrics(initial: dict, final: dict, attempts: int) -> dict:
    events = final["events"]
    internal = [event for event in events if internal_process(event) is not None]
    other = sum(1 for event in internal if internal_process(event) in (2, 3))
    mutation_attempts = sum(
        opcode_at_initial_event(initial, event) in GRAPH_OPS for event in internal
    )
    graph_changed = (
        final["graph_objects"] != initial["graph_objects"]
        or any(
            process["code"]["nodes"] != initial["processes"][pid]["code"]["nodes"]
            for pid, process in final["processes"].items()
            if pid in initial["processes"]
        )
        or len(final["processes"]) != len(initial["processes"])
    )
    code_nodes = sum(len(p["code"]["nodes"]) for p in final["processes"].values())
    code_nodes += sum(len(g["nodes"]) for g in final["graph_objects"].values())
    return {
        "attempts": attempts,
        "internal_attempts": len(internal),
        "other_process_attempts": other,
        "external_events": sum(e["origin"] == "External" for e in events),
        "traps": sum(isinstance(e["kind"], dict) and "Trapped" in e["kind"] for e in internal),
        "cells": len(final["cells"]),
        "occupied_units": occupied_units(final),
        "new_processes": max(0, len(final["processes"]) - INITIAL_PROCESSES),
        "graph_objects": len(final["graph_objects"]),
        "node_count_delta": code_nodes - INITIAL_NODES,
        "mutation_attempts": mutation_attempts,
        "endogenous_graph_change_proxy": bool(graph_changed and mutation_attempts),
        "active_late": attempts > 48,
    }


def parse_label(label: str) -> tuple[int, str]:
    if not label.startswith("seed=") or "/" not in label:
        raise ValueError(f"unexpected condition label: {label}")
    prefix, condition = label.split("/", 1)
    seed = int(prefix[len("seed="):])
    if seed not in range(FIRST_SEED, FIRST_SEED + REPLICAS) or condition not in CONDITIONS:
        raise ValueError(f"out-of-protocol condition: {label}")
    return seed, condition


def analyze(db_path: Path) -> dict:
    conn = sqlite3.connect(f"{db_path.resolve().as_uri()}?mode=ro", uri=True)
    try:
        rows = conn.execute(
            """SELECT id,source_revision,condition,initial_sha256,initial_json,
                      actions_json,attempt_limit,attempts,final_sha256,final_json
               FROM scheduled_runs ORDER BY id"""
        ).fetchall()
        if len(rows) != REPLICAS * len(CONDITIONS):
            raise ValueError(f"expected 320 conditions; found {len(rows)}")
        seen: set[tuple[int, str]] = set()
        by_seed: dict[int, dict[str, dict]] = {}
        hashes_by_seed: dict[int, str] = {}
        revisions: set[str] = set()
        checkpoints = conn.execute(
            """SELECT run_id,ordinal,after_attempts,state_sha256,state_json
               FROM scheduled_checkpoints ORDER BY run_id,ordinal"""
        ).fetchall()
        checkpoint_count = Counter()
        for run_id, ordinal, offset, sha, snapshot in checkpoints:
            if digest(snapshot) != sha or ordinal < 0 or offset < 0 or offset > LIMIT:
                raise ValueError(f"invalid checkpoint for run {run_id}")
            checkpoint_count[run_id] += 1

        for (run_id, revision, label, initial_hash, initial_json, actions_json,
             limit, attempts, final_hash, final_json) in rows:
            seed, condition = parse_label(label)
            if (seed, condition) in seen:
                raise ValueError(f"duplicate condition: {label}")
            seen.add((seed, condition))
            revisions.add(revision)
            if (
                not revision or digest(initial_json) != initial_hash
                or digest(final_json) != final_hash or limit != LIMIT
                or not 0 <= attempts <= LIMIT
            ):
                raise ValueError(f"invalid hash, revision or attempt limit: {label}")
            if seed in hashes_by_seed and hashes_by_seed[seed] != initial_hash:
                raise ValueError(f"unmatched initial hash for seed {seed}")
            hashes_by_seed[seed] = initial_hash

            initial, final = json.loads(initial_json), json.loads(final_json)
            if not initial.get("strict_accounting") or initial.get("total_storage_limit") != TOTAL_UNITS:
                raise ValueError(f"world configuration mismatch: {label}")
            if occupied_units(initial) != 51 or occupied_units(final) > TOTAL_UNITS:
                raise ValueError(f"storage bound failed: {label}")
            if len(initial["processes"]) != INITIAL_PROCESSES:
                raise ValueError(f"wrong number of initial processes: {label}")
            actions = json.loads(actions_json)
            expected_actions = {"control": 0, "zero-credit": 3, "disable-p1": 1,
                                "lesion-cell": 1, "sham-cell": 1}[condition]
            if len(actions) != expected_actions:
                raise ValueError(f"intervention mismatch: {label}")
            if checkpoint_count[run_id] > expected_actions:
                raise ValueError(f"extra checkpoint: {label}")
            if condition == "zero-credit" and (
                attempts != 0 or any(internal_process(e) is not None for e in final["events"])
            ):
                raise ValueError(f"zero-credit run executed CIL: {label}")
            if condition == "control" and checkpoint_count[run_id] != 0:
                raise ValueError(f"control unexpectedly has checkpoints: {label}")
            m = metrics(initial, final, attempts)
            m["final_state"] = final
            by_seed.setdefault(seed, {})[condition] = m

        if len(revisions) != 1 or len(by_seed) != REPLICAS:
            raise ValueError("source revision mismatch or incomplete seed range")
        if any(set(conditions) != set(CONDITIONS) for conditions in by_seed.values()):
            raise ValueError("missing matched condition")

        summary = {}
        for condition in CONDITIONS:
            group = [by_seed[seed][condition] for seed in sorted(by_seed)]
            summary[condition] = {
                "total_attempts": sum(m["attempts"] for m in group),
                "total_traps": sum(m["traps"] for m in group),
                "total_mutation_attempts": sum(m["mutation_attempts"] for m in group),
                "graph_change_proxy_seeds": sum(m["endogenous_graph_change_proxy"] for m in group),
                "active_late_seeds": sum(m["active_late"] for m in group),
                "total_new_processes": sum(m["new_processes"] for m in group),
            }
        lesion_effects = 0
        other_process_effects = 0
        for seed, groups in by_seed.items():
            if semantic_signature(groups["lesion-cell"]["final_state"]) != semantic_signature(
                groups["sham-cell"]["final_state"]
            ):
                lesion_effects += 1
            if (
                groups["control"]["other_process_attempts"]
                != groups["disable-p1"]["other_process_attempts"]
            ):
                other_process_effects += 1
        return {
            "study": "CL-1 Study 01",
            "status": "descriptive first study; organizational closure and self-maintenance NOT evaluated",
            "revision": next(iter(revisions)),
            "independent_seeds": REPLICAS,
            "matched_conditions": len(rows),
            "hashes_valid": True,
            "checkpoints_hash_valid": True,
            "recorded_checkpoints": len(checkpoints),
            "exact_replay": "verified by Rust study runner before this independent analysis",
            "conditions": summary,
            "paired_nontrivial_cell_lesion_effect_seeds": lesion_effects,
            "paired_other_process_attempt_change_seeds": other_process_effects,
            "organizational_closure": "not evaluable",
            "autonomous_maintenance": "not evaluable",
            "endogenous_novelty_accumulation": "not evaluable",
            "life_claim": "none",
            "caveat": "Same-seed replay does not prove life; seeded structure and interventions are external.",
        }
    finally:
        conn.close()


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("sqlite", type=Path)
    parser.add_argument("--json", type=Path, required=True)
    args = parser.parse_args()
    result = analyze(args.sqlite)
    args.json.write_text(json.dumps(result, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
    print(json.dumps({
        "study": result["study"],
        "seeds": result["independent_seeds"],
        "conditions": result["matched_conditions"],
        "checkpoints": result["recorded_checkpoints"],
        "lesion_effects": result["paired_nontrivial_cell_lesion_effect_seeds"],
        "other_process_attempt_effects": result["paired_other_process_attempt_change_seeds"],
    }))


if __name__ == "__main__":
    main()
