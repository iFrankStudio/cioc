#!/usr/bin/env python3
"""Study 33: exact independent audit of native pooled compute uptake and return.

The pool is researcher-endowed; native transfers do not create energy or life.
"""
from __future__ import annotations

import argparse
import copy
import json
import sqlite3
from collections import Counter, defaultdict
from pathlib import Path

import analyze
from analyze_study02 import wilson_95
from analyze_study14 import simulated_state

FIRST = 2_600_000
SEEDS = 128
EPOCHS = 24
QUOTA = 12
LABELS = (
    "control", "mutate-g2", "empty-pool", "mutate-g2-empty-pool",
    "refill", "mutate-g2-refill", "sham", "exact-original-g2",
    "zero-credit",
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


def actions(arm: str, initial: dict) -> list[dict]:
    def at(epoch: int, action: dict) -> dict:
        return {"epoch": epoch, "action": action}

    def source_mutation(exact: bool = False) -> list[dict]:
        original = copy.deepcopy(initial["graph_objects"]["2"])
        if not exact:
            original["nodes"][0] = "Halt"
        return [
            at(0, {"DropGraph": {"graph": 2}}),
            at(0, {"PutGraph": {"graph": 2, "object": original}}),
        ]

    def refill() -> list[dict]:
        return [
            at(epoch, {"SetCredits": {"process": pid, "credits": 47}})
            for epoch in (8, 16) for pid in (1, 2, 3)
        ]

    schedules = {
        "control": [],
        "mutate-g2": source_mutation(),
        "empty-pool": [at(0, {"SetComputePool": {"credits": 0}})],
        "mutate-g2-empty-pool": source_mutation()
        + [at(0, {"SetComputePool": {"credits": 0}})],
        "refill": refill(),
        "mutate-g2-refill": source_mutation() + refill(),
        "sham": [at(0, {"DropGraph": {"graph": 0}})],
        "exact-original-g2": source_mutation(exact=True),
        "zero-credit": [
            at(0, {"SetCredits": {"process": pid, "credits": 0}})
            for pid in (1, 2, 3)
        ],
    }
    return schedules[arm]


SOURCE_WRITES = {
    "Const": "dst",
    "Read": "dst",
    "GraphNew": "dst",
    "GraphCopyNode": "dst",
    "GraphCopyFrom": "node",
    "GraphReadTag": "dst",
}
SOURCE_NONWRITERS = {
    "GraphGrantRead", "Write", "Branch",
    "DrawCompute", "ReturnCompute",
}


def trace_source_and_pool(
    events: list[dict], program: list, tag: int,
    target: int, attempt_offset: int,
) -> dict:
    """Conservative native successful provenance and same-actor resource draw."""
    labels: dict[int, bool] = {}
    draws: list[int] = []
    outputs: list[dict] = []
    seen_tag = 0
    process_attempts = attempt_offset
    for event in events:
        if analyze.internal_process(event) != 1:
            continue
        if event["process"] != 1:
            raise ValueError("native attribution mismatched")
        process_attempts += 1
        tick = int(event["tick"])
        pc = event["pc"]
        if not isinstance(pc, int) or not 0 <= pc < len(program):
            raise ValueError("original process source PC absent")
        node = program[pc]
        kind = event["kind"]
        if node == "Halt" or isinstance(kind, dict) and "Trapped" in kind:
            continue
        if not isinstance(node, dict) or len(node) != 1:
            raise ValueError("unexpected code node")
        opcode, params = next(iter(node.items()))
        if opcode == "GraphReadTag":
            if not (isinstance(kind, dict) and "GraphTagRead" in kind):
                raise ValueError("native code read missing success provenance")
            read = kind["GraphTagRead"]
            qualifying = (
                read.get("graph") == 2 and read.get("owner") == 2
                and read.get("node") == 0 and read.get("tag") == tag
            )
            labels[params["dst"]] = qualifying
            seen_tag += qualifying
        elif opcode == "Add":
            if kind != "Executed":
                raise ValueError("native add success event inconsistent")
            labels[params["dst"]] = labels.get(params["lhs"], False) or labels.get(
                params["rhs"], False
            )
        elif opcode == "Write":
            if not (isinstance(kind, dict) and "CellWritten" in kind):
                raise ValueError("native write missing CellWritten")
            written = kind["CellWritten"]
            if (written["key"] == target
                    and labels.get(params["address"], False)):
                outputs.append({
                    "tick": tick,
                    "epoch_value": written["value"],
                    "prior_draw": any(t < tick for t in draws),
                    "attempts_at_write": process_attempts,
                    "native_budget_extension": process_attempts > 47,
                })
        elif opcode == "DrawCompute":
            if kind == "Executed":
                draws.append(tick)
            elif not (isinstance(kind, dict) and "Trapped" in kind):
                raise ValueError("native draw success provenance inconsistent")
        elif opcode in SOURCE_WRITES:
            labels[params[SOURCE_WRITES[opcode]]] = False
        elif opcode not in SOURCE_NONWRITERS:
            raise ValueError(f"unregistered native source effect: {opcode}")
    # Require one unambiguous target write that actually used source2.
    return {
        "source_reads": seen_tag,
        "positive_native_draws": len(draws),
        "unique_source_write": outputs[0] if len(outputs) == 1 else None,
        "qualifying_output_events": len(outputs),
    }



def ensure_registered_nulls(outcomes: Counter) -> None:
    """Every frozen primary endpoint must be explicit even when 0/128."""
    for key in (
        "control.joint_source_and_draw",
        "paired_source_and_draw",
        "paired_source_and_draw_after_budget_exhaustion",
    ):
        outcomes.setdefault(key, 0)



def audit(path: Path) -> dict:
    db = sqlite3.connect(f"{path.resolve().as_uri()}?mode=ro", uri=True)
    try:
        rows = db.execute(
            "SELECT id,source_revision,condition,initial_sha256,initial_json,"
            "actions_json,epoch_count,quota,attempts_json,final_sha256,final_json "
            "FROM epoch_runs ORDER BY id"
        ).fetchall()
        if len(rows) != SEEDS * len(LABELS):
            raise ValueError("incomplete 128x9 native source/pool ecology")
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
            if initial["graph_objects"]["2"]["owner"] != 2:
                raise ValueError("graph2 owner changed before fork")
            if not isinstance(initial["graph_objects"]["2"]["nodes"][0], dict) or (
                "GraphNew" not in initial["graph_objects"]["2"]["nodes"][0]
            ):
                raise ValueError("graph2 tag11 prefork source missing")
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
            if json.loads(actions_raw) != actions(arm, initial):
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
            witnessed = []
            observed_actor1 = 0
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
                trace = trace_source_and_pool(
                    events, initial["processes"]["1"]["code"]["nodes"],
                    19 if arm.startswith("mutate-g2") else 11,
                    19 if arm.startswith("mutate-g2") else 11,
                    observed_actor1,
                )
                observed_actor1 += sum(
                    analyze.internal_process(e) == 1 for e in events
                )
                witnessed.append(trace)
                for event in events:
                    kind = event["kind"]
                    actor = analyze.internal_process(event)
                    if actor is None:
                        if (event["origin"] == "External"
                                and isinstance(kind, dict)
                                and "Intervened" in kind
                                and "SetComputePool" in str(kind["Intervened"])):
                            if arm not in ("empty-pool", "mutate-g2-empty-pool") or epoch != 0:
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
            expected_edits = int(
                arm in ("mutate-g2", "mutate-g2-empty-pool",
                        "mutate-g2-refill", "exact-original-g2")
            )
            # Postfork external source/permission events are accounted for from
            # complete cumulative final events minus the identical initial prefix.
            final_events = final["events"][len(initial["events"]):]
            for name in ("DropGraph", "PutGraph"):
                observed = sum(
                    e["origin"] == "External"
                    and isinstance(e["kind"], dict)
                    and name in str(e["kind"].get("Intervened", ""))
                    for e in final_events
                )
                if observed != expected_edits:
                    raise ValueError("missing or extra external graph intervention")
            if arm == "sham" and not any(
                e["origin"] == "External"
                and isinstance(e["kind"], dict)
                and "InterventionRejected" in e["kind"]
                for e in final_events
            ):
                raise ValueError("rejected sham intervention is missing")
            prolonged = (
                arm in ("control", "sham", "empty-pool")
                and any(counts[id] > 47 and actor_draws[id] > 0 for id in (1, 2, 3))
            )
            worlds[seed][arm] = {
                "final": final, "attempts": attempts,
                "draw": successes["draw"], "return": successes["return"],
                "prolonged": prolonged, "actor_attempts": counts,
                "witnessed": witnessed,
            }
        if (len(worlds) != SEEDS or len(revisions) != 1
                or any(set(arms) != set(LABELS) for arms in worlds.values())):
            raise ValueError("unbalanced preregistered independent seed-world cohort")
        outcomes = Counter()
        raw = Counter()
        witnesses = []
        byarm = {}
        for seed in sorted(worlds):
            records = worlds[seed]
            if simulated_state(records["control"]["final"]) != simulated_state(
                records["sham"]["final"]
            ):
                raise ValueError("rejected sham changed native final state")
            if simulated_state(records["control"]["final"]) != simulated_state(
                records["exact-original-g2"]["final"]
            ):
                raise ValueError("exact graph2 reinjection changed native state")
            for arm in LABELS:
                info = records[arm]
                outputs = []
                for epoch, trace in enumerate(info["witnessed"]):
                    output = trace["unique_source_write"]
                    if output is not None:
                        outputs.append({"epoch": epoch, **output})
                byarm[(seed, arm)] = outputs
                outcomes[f"{arm}.process1_native_draw"] += any(
                    trace["positive_native_draws"] > 0
                    for trace in info["witnessed"]
                )
                outcomes[f"{arm}.source_tag_register_write"] += bool(outputs)
                outcomes[f"{arm}.joint_source_and_draw"] += any(
                    item["prior_draw"] for item in outputs
                )
                outcomes[f"{arm}.extended_joint_source_and_draw"] += any(
                    item["prior_draw"] and item["native_budget_extension"]
                    for item in outputs
                )
                for w, (lo, hi) in enumerate(WINDOWS, 1):
                    activity = sum(info["attempts"][lo:hi]) > 0
                    outcomes[f"{arm}.W{w}.native_activity"] += activity
                    outcomes[f"{arm}.W{w}.native_draw"] += sum(
                        info["draw"][lo:hi]
                    ) > 0
                    outcomes[f"{arm}.W{w}.source_written"] += any(
                        lo <= o["epoch"] < hi for o in outputs
                    )
                    raw[f"{arm}.W{w}.draw_events"] += sum(info["draw"][lo:hi])
                    raw[f"{arm}.W{w}.return_events"] += sum(info["return"][lo:hi])
            left = {
                entry["epoch"]: entry for entry in byarm[(seed, "control")]
                if entry["prior_draw"]
            }
            right = {
                entry["epoch"]: entry for entry in byarm[(seed, "mutate-g2")]
                if entry["prior_draw"]
            }
            matched = sorted(
                epoch for epoch in left.keys() & right.keys()
                if left[epoch]["epoch_value"] == right[epoch]["epoch_value"]
            )
            if matched:
                outcomes["paired_source_and_draw"] += 1
                strong = any(
                    left[e]["native_budget_extension"]
                    and right[e]["native_budget_extension"]
                    for e in matched
                )
                outcomes["paired_source_and_draw_after_budget_exhaustion"] += strong
                if len(witnesses) < 24:
                    witnesses.append({
                        "seed": seed, "matched_epochs": matched,
                        "strong_native_budget_extension": strong,
                        "control": left[matched[0]], "mutation": right[matched[0]],
                    })
            # Source contrasts with initially empty pool are descriptive,
            # because the native ReturnCompute primitive can recycle pool units.
            for arm, empty_arm in (
                ("control", "empty-pool"),
                ("mutate-g2", "mutate-g2-empty-pool"),
            ):
                source_has = bool(byarm[(seed, arm)])
                empty_has = bool(byarm[(seed, empty_arm)])
                outcomes[f"paired.{arm}.source_only"] += source_has and not empty_has
                outcomes[f"paired.{arm}.empty_only"] += empty_has and not source_has
        ensure_registered_nulls(outcomes)
        metrics = {}
        for name, n in sorted(outcomes.items()):
            lo, hi = wilson_95(n, SEEDS)
            metrics[name] = {
                "worlds": n, "denominator": SEEDS,
                "wilson_95": [round(lo, 6), round(hi, 6)],
            }
        return {
            "study": "CL-1 Study 33",
            "source_revision": next(iter(revisions)),
            "independent_seed_worlds": SEEDS,
            "matched_runs": len(rows),
            "hashed_epoch_snapshots": sum(map(len, frames.values())),
            "primary_register_source_and_compute": metrics[
                "control.joint_source_and_draw"
            ],
            "primary_paired_content_address_and_compute": metrics[
                "paired_source_and_draw"
            ],
            "primary_paired_after_native_budget_extension": metrics[
                "paired_source_and_draw_after_budget_exhaustion"
            ],
            "registered_metrics": metrics,
            "raw_resource_events": dict(sorted(raw.items())),
            "strict_matched_witnesses": witnesses,
            "native_resource_origin": "RESEARCHER-ENDOWED finite pool or recycled process credits",
            "endogenous_code_design": "NOT EVALUABLE",
            "autonomous_multicomponent_maintenance": "NOT EVALUABLE",
            "autonomous_repair": "NOT EVALUABLE",
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
        "new_worlds": result["independent_seed_worlds"],
        "runs": result["matched_runs"],
        "snapshots": result["hashed_epoch_snapshots"],
        "joint_source_and_draw": result["primary_register_source_and_compute"],
        "paired_content_and_draw": result["primary_paired_content_address_and_compute"],
        "paired_after_budget_extension": result[
            "primary_paired_after_native_budget_extension"
        ],
        "life_claim": result["life_claim"],
    }))


if __name__ == "__main__":
    main()
