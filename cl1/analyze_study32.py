#!/usr/bin/env python3
"""Study 32: exact independent audit of native pooled compute uptake and return.

The pool is researcher-endowed; native transfers do not create energy or life.
"""
from __future__ import annotations

import argparse
import json
import sqlite3
from collections import Counter, defaultdict
from pathlib import Path

import analyze
from analyze_study02 import wilson_95
from analyze_study14 import simulated_state

FIRST = 2_500_000
SEEDS = 192
EPOCHS = 24
QUOTA = 12
LABELS = (
    "control", "sham", "empty-pool", "refill",
    "empty-pool-refill", "zero-credit",
)
WINDOWS = ((0, 8), (8, 16), (16, 24))
MASK = (1 << 64) - 1


def expected_program_continuations(seed: int) -> dict[int, list[dict]]:
    """Independently replay the preregistered fixed 64-bit LCG/grammar."""
    state = seed
    programs: dict[int, list[dict]] = {}
    for actor in (1, 2, 3):
        nodes = []
        for index in range(9, 27):
            state = (state * 6_364_136_223_846_793_005
                     + 1_442_695_040_888_963_407) & MASK
            next_pc = index + 1
            choice = state % 12
            if choice == 0:
                op = {"GraphCopyFrom": {
                    "target": 3, "source": 1, "node": 2, "next": next_pc}}
            elif choice == 1:
                op = {"GraphCopyNode": {
                    "graph": 3, "source": 2, "dst": 2, "next": next_pc}}
            elif choice == 2:
                op = {"DrawCompute": {"amount": 1, "next": next_pc}}
            elif choice == 3:
                op = {"GraphReadTag": {
                    "dst": 0, "graph": 3, "node": 2, "next": next_pc}}
            elif choice == 4:
                op = {"GraphReadTag": {
                    "dst": 2, "graph": 1, "node": 2, "next": next_pc}}
            elif choice == 5:
                op = {"Add": {
                    "dst": 0, "lhs": 0, "rhs": 2, "next": next_pc}}
            elif choice in (6, 7):
                op = {"Write": {
                    "address": 0, "value": 3, "next": next_pc}}
            elif choice == 8:
                op = {"Const": {"dst": 2, "value": 0, "next": next_pc}}
            elif choice == 9:
                op = {"Branch": {
                    "test": 1, "nonzero": index - 2, "zero": next_pc}}
            elif choice == 10:
                op = {"ReturnCompute": {"amount": 1, "next": next_pc}}
            else:
                op = {"GraphGrantRead": {
                    "graph": 3, "reader": 1, "next": next_pc}}
            nodes.append(op)
        programs[actor] = nodes
    return programs


def actions(arm: str) -> list[dict]:
    def at(epoch: int, action: dict) -> dict:
        return {"epoch": epoch, "action": action}

    if arm == "control":
        return []
    if arm == "sham":
        return [at(0, {"DropGraph": {"graph": 0}})]
    if arm == "zero-credit":
        return [
            at(0, {"SetCredits": {"process": id, "credits": 0}})
            for id in (1, 2, 3)
        ]
    if arm not in ("empty-pool", "refill", "empty-pool-refill"):
        raise ValueError("unregistered study branch")
    result = []
    if arm.startswith("empty-pool"):
        result.append(at(0, {"SetComputePool": {"credits": 0}}))
    if arm in ("refill", "empty-pool-refill"):
        result.extend(
            at(epoch, {"SetCredits": {"process": id, "credits": 47}})
            for epoch in (8, 16) for id in (1, 2, 3)
        )
    return result


def audit(path: Path) -> dict:
    db = sqlite3.connect(f"{path.resolve().as_uri()}?mode=ro", uri=True)
    try:
        rows = db.execute(
            "SELECT id,source_revision,condition,initial_sha256,initial_json,"
            "actions_json,epoch_count,quota,attempts_json,final_sha256,final_json "
            "FROM epoch_runs ORDER BY id"
        ).fetchall()
        if len(rows) != SEEDS * len(LABELS):
            raise ValueError("incomplete 192x6 native pool ecology")
        frames = defaultdict(list)
        for run_id, epoch, digest, raw in db.execute(
            "SELECT run_id,epoch,state_sha256,state_json "
            "FROM epoch_snapshots ORDER BY run_id,epoch"
        ):
            if not 0 <= epoch < EPOCHS or analyze.digest(raw) != digest:
                raise ValueError("invalid epoch snapshot SHA256/epoch index")
            frames[run_id].append((epoch, digest, raw))
        worlds = defaultdict(dict)
        hashes = {}
        revisions = set()
        for (run_id, revision, condition, initial_sha, initial_raw, actions_raw,
             count, quota, attempts_raw, final_sha, final_raw) in rows:
            if "/" not in condition or not condition.startswith("seed="):
                raise ValueError("unknown branch label")
            prefix, arm = condition.split("/", 1)
            seed = int(prefix.removeprefix("seed="))
            if seed not in range(FIRST, FIRST + SEEDS) or arm not in LABELS:
                raise ValueError("unregistered independent world")
            if arm in worlds[seed] or not revision or count != EPOCHS or quota != QUOTA:
                raise ValueError("duplicate arm or invalid source/epoch budget")
            if analyze.digest(initial_raw) != initial_sha or analyze.digest(final_raw) != final_sha:
                raise ValueError("source/final hash mismatch")
            if seed in hashes and hashes[seed] != initial_sha:
                raise ValueError("different prefork state across same-seed arms")
            hashes[seed] = initial_sha
            revisions.add(revision)
            initial, final = json.loads(initial_raw), json.loads(final_raw)
            if (initial["compute_pool"] != 48
                    or initial["total_storage_limit"] != 192
                    or initial["max_nodes_per_process"] != 40
                    or initial["storage_limit"] != 32
                    or not initial["strict_accounting"]):
                raise ValueError("unregistered common world resource cap or compute endowment")
            source = expected_program_continuations(seed)
            for actor in (1, 2, 3):
                process = initial["processes"].get(str(actor))
                if process is None or process["credits"] != 47:
                    raise ValueError("invalid original 47-credit process budget")
                if process["registers"][1] != (actor % 3 + 1):
                    raise ValueError("registered stable positive draw amount changed")
                if process["code"]["nodes"][9:27] != source[actor]:
                    raise ValueError("random source code differs from preregistered 54 LCG draws")
                graph = initial["graph_objects"].get(str(actor))
                if graph is None or graph["owner"] != actor or len(graph["nodes"]) != 1:
                    raise ValueError("missing original graph permission scaffold")
            if json.loads(actions_raw) != actions(arm):
                raise ValueError("unregistered external resource schedule")
            attempts = json.loads(attempts_raw)
            if len(attempts) != EPOCHS or any(
                not isinstance(n, int) or not 0 <= n <= QUOTA for n in attempts
            ):
                raise ValueError("invalid native attempt count")
            recorded = frames[run_id]
            if (len(recorded) != EPOCHS
                    or any(epoch != pos for pos, (epoch, _, _) in enumerate(recorded))
                    or recorded[-1][1:] != (final_sha, final_raw)):
                raise ValueError("missing/out of order/altered epoch snapshot")
            pool = 48
            prev_events = initial["events"]
            successes = {"draw": [0] * EPOCHS, "return": [0] * EPOCHS}
            counts = {actor: 0 for actor in (1, 2, 3)}
            native_executed = [0] * EPOCHS
            actor_draws = {actor: 0 for actor in (1, 2, 3)}
            for epoch, _, raw in recorded:
                snapshot = json.loads(raw)
                if snapshot["events"][:len(prev_events)] != prev_events:
                    raise ValueError("native event provenance reordered or dropped")
                if analyze.occupied_units(snapshot) > 192:
                    raise ValueError("strict storage cap violated")
                events = snapshot["events"][len(prev_events):]
                prev_events = snapshot["events"]
                for event in events:
                    kind = event["kind"]
                    actor = analyze.internal_process(event)
                    if actor is None:
                        if (event["origin"] == "External"
                                and isinstance(kind, dict)
                                and "Intervened" in kind
                                and "SetComputePool" in str(kind["Intervened"])):
                            if arm not in ("empty-pool", "empty-pool-refill") or epoch != 0:
                                raise ValueError("unregistered external compute pool reset")
                            pool = 0
                        continue
                    if actor not in (1, 2, 3) or event["process"] != actor:
                        raise ValueError("unregistered actor execution")
                    pc = event["pc"]
                    program = initial["processes"][str(actor)]["code"]["nodes"]
                    if not isinstance(pc, int) or not 0 <= pc < len(program):
                        raise ValueError("invalid native program counter")
                    opcode = program[pc]
                    native_executed[epoch] += 1
                    counts[actor] += 1
                    if isinstance(opcode, dict) and "DrawCompute" in opcode:
                        if kind == "Executed":
                            quantity = (actor % 3 + 1)
                            if quantity > pool:
                                raise ValueError("native DrawCompute issued without resource")
                            pool -= quantity
                            successes["draw"][epoch] += 1
                            actor_draws[actor] += 1
                        elif not (isinstance(kind, dict) and "Trapped" in kind):
                            raise ValueError("native DrawCompute has unrecognized success event")
                    if isinstance(opcode, dict) and "ReturnCompute" in opcode:
                        if kind == "Executed":
                            pool += (actor % 3 + 1)
                            successes["return"][epoch] += 1
                        elif not (isinstance(kind, dict) and "Trapped" in kind):
                            raise ValueError("native ReturnCompute has unrecognized success event")
                if pool != snapshot["compute_pool"] or pool < 0:
                    raise ValueError("independent pool ledger disagrees with exact SQLite snapshot")
                if native_executed[epoch] != attempts[epoch]:
                    raise ValueError("native event count differs from charged attempts")
            if arm == "zero-credit" and (
                any(attempts) or sum(successes["draw"]) or sum(successes["return"])
            ):
                raise ValueError("zero-credit native compute execution occurred")
            prolonged = (
                arm in ("control", "sham", "empty-pool")
                and any(counts[id] > 47 and actor_draws[id] > 0 for id in (1, 2, 3))
            )
            worlds[seed][arm] = {
                "final": final, "attempts": attempts,
                "draw": successes["draw"], "return": successes["return"],
                "prolonged": prolonged, "actor_attempts": counts,
            }
        if (len(worlds) != SEEDS or len(revisions) != 1
                or any(set(arms) != set(LABELS) for arms in worlds.values())):
            raise ValueError("unbalanced preregistered independent seed-world cohort")
        outcomes = Counter()
        raw = Counter()
        for seed in sorted(worlds):
            records = worlds[seed]
            if simulated_state(records["control"]["final"]) != simulated_state(
                records["sham"]["final"]
            ):
                raise ValueError("sham intervention altered native resources/program state")
            for arm in LABELS:
                info = records[arm]
                outcomes[f"{arm}.prolonged_by_internal_pool_draw"] += info["prolonged"]
                for w, (start, stop) in enumerate(WINDOWS, 1):
                    attempts = sum(info["attempts"][start:stop])
                    draws = sum(info["draw"][start:stop])
                    returns = sum(info["return"][start:stop])
                    outcomes[f"{arm}.W{w}.native_activity"] += attempts > 0
                    outcomes[f"{arm}.W{w}.positive_native_draw"] += draws > 0
                    outcomes[f"{arm}.W{w}.native_return"] += returns > 0
                    raw[f"{arm}.W{w}.native_draw_events"] += draws
                    raw[f"{arm}.W{w}.native_return_events"] += returns
            for endpoint, method in (
                ("w1_draw", lambda r: sum(r["draw"][0:8]) > 0),
                ("w3_activity", lambda r: sum(r["attempts"][16:24]) > 0),
            ):
                have = method(records["control"])
                denied = method(records["empty-pool"])
                outcomes[f"paired.{endpoint}.control_only"] += have and not denied
                outcomes[f"paired.{endpoint}.empty_only"] += denied and not have
        metrics = {}
        for name, n in sorted(outcomes.items()):
            lo, hi = wilson_95(n, SEEDS)
            metrics[name] = {
                "worlds": n, "denominator": SEEDS,
                "wilson_95": [round(lo, 6), round(hi, 6)],
            }
        return {
            "study": "CL-1 Study 32",
            "source_revision": next(iter(revisions)),
            "independent_seed_worlds": SEEDS,
            "matched_runs": len(rows),
            "hashed_epoch_snapshots": sum(map(len, frames.values())),
            "registered_metrics": metrics,
            "raw_native_compute_transfer_events": dict(sorted(raw.items())),
            "origin_of_initial_compute_pool": "EXTERNAL 48-unit endowment before prefork",
            "native_resource_uptake": "INTERPRET ONLY AS EXECUTED PROGRAM OPS",
            "endogenous_energy_creation": "NONE",
            "causal_multicomponent_maintenance": "NOT EVALUABLE",
            "self_repair": "NOT EVALUABLE",
            "life_claim": "NONE",
        }
    finally:
        db.close()


def main() -> None:
    p = argparse.ArgumentParser(description=__doc__)
    p.add_argument("sqlite", type=Path)
    p.add_argument("--json", type=Path, required=True)
    args = p.parse_args()
    result = audit(args.sqlite)
    args.json.write_text(json.dumps(result, indent=2) + "\n", encoding="utf-8")
    print(json.dumps({
        "study": result["study"],
        "worlds": result["independent_seed_worlds"],
        "runs": result["matched_runs"],
        "snapshots": result["hashed_epoch_snapshots"],
        "control_w1_draw": result["registered_metrics"]["control.W1.positive_native_draw"],
        "empty_pool_w1_draw": result["registered_metrics"]["empty-pool.W1.positive_native_draw"],
        "prolonged_control": result["registered_metrics"]["control.prolonged_by_internal_pool_draw"],
        "stage": result["life_claim"],
    }))


if __name__ == "__main__":
    main()
