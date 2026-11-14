#!/usr/bin/env python3
"""CL-1 Study61-absolute independent native executable plasticity and source-audited ecology.

No claimed life: external environment supplies ambient compute and neutral ignition.
Only unequivocally attributed native code changes count; reverted mutation remains
NOT EVALUABLE with epoch-level snapshots.
"""
from __future__ import annotations

import argparse
import json
import sqlite3
from collections import Counter, defaultdict
from pathlib import Path

import analyze
from analyze_study02 import wilson_95

FIRST = 4_100_000
SEEDS = 256
EPOCHS = 24
ARMS = ("ambient-general", "ambient-only", "closed-general", "zero-credit-ambient-general")
AMBIENT = {"ambient-general", "ambient-only", "zero-credit-ambient-general"}
IGNITION = {"ambient-general", "closed-general", "zero-credit-ambient-general"}
MASK = (1 << 64) - 1
MUTATORS = {"PatchNext", "PatchConst", "CloneNode"}


def expected_continuations(seed: int) -> dict[int, list]:
    """Reconstruct 72 exact native CIL random instructions independently of Rust."""
    state = seed
    result = {}
    for actor in (1, 2, 3):
        nodes = []
        for index in range(9, 81):
            state = (state * 6_364_136_223_846_793_005 +
                     1_442_695_040_888_963_407) & MASK
            n, choice = index + 1, state % 24
            if choice == 0:
                op = {"GraphCopyFrom": {"target": 3, "source": 1, "node": 2, "next": n}}
            elif choice == 1:
                op = {"GraphCopyNode": {"graph": 3, "source": 2, "dst": 2, "next": n}}
            elif choice == 2:
                op = {"DrawCompute": {"amount": 1, "next": n}}
            elif choice == 3:
                op = {"GraphReadTag": {"dst": 0, "graph": 3, "node": 2, "next": n}}
            elif choice == 4:
                op = {"GraphReadTag": {"dst": 2, "graph": 1, "node": 2, "next": n}}
            elif choice == 5:
                op = {"Add": {"dst": 0, "lhs": 0, "rhs": 2, "next": n}}
            elif choice in (6, 7):
                op = {"Write": {"address": 0, "value": 3, "next": n}}
            elif choice == 8:
                state = (state * 6_364_136_223_846_793_005 +
                         1_442_695_040_888_963_407) & MASK
                op = {"Const": {"dst": 2, "value": state % 81, "next": n}}
            elif choice == 9:
                op = {"Branch": {"test": 1, "nonzero": index - 2, "zero": n}}
            elif choice == 10:
                op = {"ReturnCompute": {"amount": 1, "next": n}}
            elif choice == 11:
                op = {"GraphGrantRead": {"graph": 3, "reader": 1, "next": n}}
            elif choice == 12:
                op = {"CloneNode": {"dst": 2, "source": 2, "next": n}}
            elif choice == 13:
                op = {"PatchNext": {"node": 2, "target": 0, "next": n}}
            elif choice == 14:
                op = {"PatchConst": {"node": 2, "value": 0, "next": n}}
            elif choice == 15:
                op = {"ReadNext": {"dst": 2, "node": 0, "next": n}}
            elif choice == 16:
                op = {"GraphSetNext": {"graph": 3, "node": 2, "target": 0, "next": n}}
            elif choice == 17:
                op = {"GraphSetConst": {"graph": 3, "node": 2, "value": 0, "next": n}}
            elif choice == 18:
                op = {"GraphLaunch": {"graph": 3, "credits": 1, "dst": 2, "next": n}}
            elif choice == 19:
                op = {"GraphEncode": {"graph": 3, "address": 0, "dst": 2, "next": n}}
            elif choice == 20:
                op = {"GraphCopyNode": {"graph": 3, "source": 2, "dst": 2, "next": n}}
            elif choice in (21, 22):
                op = {"GraphSetNext": {"graph": 3, "node": 2, "target": 0, "next": n}}
            else:
                op = {"GraphLaunch": {"graph": 3, "credits": 1, "dst": 2, "next": n}}
            nodes.append(op)
        result[actor] = nodes
    return result


def inventory(state: dict) -> int:
    return state["compute_pool"] + sum(p["credits"] for p in state["processes"].values())


def neutral_candidate(state: dict) -> tuple[int, int] | None:
    processes = state["processes"]
    if any(not p["stopped"] and p["credits"] > 0 for p in processes.values()):
        return None
    pool = state["compute_pool"]
    if pool < 1:
        return None
    ids = sorted(int(k) for k in processes)
    for actor in ids:
        p = processes[str(actor)]
        if not p["stopped"] and p["credits"] == 0:
            return actor, pool
    return None


def expected_actions(previous: dict, arm: str, epoch: int) -> list[dict]:
    at = lambda action: {"epoch": epoch, "action": action}
    acts = []
    copy = {**previous, "processes": {
        k: {**v} for k, v in previous["processes"].items()
    }}
    if arm == "zero-credit-ambient-general" and epoch == 0:
        for actor in (1, 2, 3):
            acts.append(at({"SetCredits": {"process": actor, "credits": 0}}))
            copy["processes"][str(actor)]["credits"] = 0
    if arm in AMBIENT and epoch > 0:
        pool = copy["compute_pool"] + 8
        acts.append(at({"SetComputePool": {"credits": pool}}))
        copy["compute_pool"] = pool
    if arm in IGNITION:
        found = neutral_candidate(copy)
        if found is not None:
            actor, pool = found
            acts.extend([
                at({"SetComputePool": {"credits": pool - 1}}),
                at({"SetCredits": {"process": actor, "credits": 1}}),
            ])
    return acts


def op_name(op) -> str | None:
    if isinstance(op, dict) and len(op) == 1:
        return next(iter(op))
    return None


def verify_native_child_write(
    event: dict, previous: dict, state: dict, launch_tick: dict[int, int]
) -> dict:
    """Reject a successful child cell write without native launch and Write PC."""
    actor = event["process"]
    kind = event["kind"]
    if (actor <= 3 or actor not in launch_tick
            or event["tick"] <= launch_tick[actor]
            or analyze.internal_process(event) != actor
            or not isinstance(kind, dict) or "CellWritten" not in kind):
        raise ValueError("forged or out-of-order child output")
    pc = event["pc"]
    if not isinstance(pc, int) or pc < 0:
        raise ValueError("child output PC invalid")
    op_candidates = []
    for snapshot in (previous, state):
        p = snapshot["processes"].get(str(actor))
        if p is not None and pc < len(p["code"]["nodes"]):
            op_candidates.append(p["code"]["nodes"][pc])
    if not any(isinstance(op, dict) and "Write" in op for op in op_candidates):
        raise ValueError("child native CellWritten lacks executable Write ancestry")
    return kind["CellWritten"]


def classify_mutation(before: dict, after: dict, native: list[dict]) -> dict:
    """Conservative per-epoch code diff + native successful mutator attribution.

    One successful original actor mutator with a code-diff in the same epoch is
    unequivocal native attribution because no experiment arm can alter code.
    """
    successful = []
    for event in native:
        if event["process"] not in (1, 2, 3) or event["kind"] != "Executed":
            continue
        actor = str(event["process"])
        pc = event["pc"]
        before_nodes = before["processes"][actor]["code"]["nodes"]
        after_nodes = after["processes"][actor]["code"]["nodes"]
        choices = [nodes[pc] for nodes in (before_nodes, after_nodes)
                   if isinstance(pc, int) and 0 <= pc < len(nodes)]
        if any(op_name(op) in MUTATORS for op in choices):
            successful.append((int(actor), pc, event["tick"]))
    changed = []
    for actor in (1, 2, 3):
        a = str(actor)
        if before["processes"][a]["code"]["nodes"] != after["processes"][a]["code"]["nodes"]:
            changed.append(actor)
    # Every code difference must have a native successful mutator in this epoch.
    for actor in changed:
        if not any(a == actor for a, _, _ in successful):
            raise ValueError("native executable changed without attributed successful mutator")
    single = [(a, pc, tick) for a, pc, tick in successful
              if a in changed and sum(x == a for x, _, _ in successful) == 1]
    return {"successful_mutators": successful, "changed_actors": changed,
            "unambiguous_actors": single}


def audit(path: Path, revision: str) -> dict:
    db = sqlite3.connect(f"{path.resolve().as_uri()}?mode=ro", uri=True)
    try:
        runs = db.execute(
            "SELECT id,condition,source_revision,initial_sha256,initial_json,"
            "actions_json,attempts_json,final_sha256,final_json "
            "FROM epoch_runs ORDER BY id"
        ).fetchall()
        if len(runs) != SEEDS * len(ARMS):
            raise ValueError("registered cohort must contain 512 matched runs")
        worlds = defaultdict(dict)
        metrics = Counter()
        witnesses = []
        hash_count = 0
        initial_hashes = {}
        for (run_id, condition, source, initial_sha, initial_raw, actions_raw,
             attempts_raw, final_sha, final_raw) in runs:
            if source != revision:
                raise ValueError("source revision not pinned")
            if analyze.digest(initial_raw) != initial_sha or analyze.digest(final_raw) != final_sha:
                raise ValueError("initial/final hashes corrupted")
            if not condition.startswith("seed=") or "/" not in condition:
                raise ValueError("unregistered condition")
            left, arm = condition.split("/", 1)
            seed = int(left.removeprefix("seed="))
            if seed not in range(FIRST, FIRST + SEEDS) or arm not in ARMS or arm in worlds[seed]:
                raise ValueError("duplicate/unregistered seed or branch")
            if seed in initial_hashes and initial_hashes[seed] != initial_sha:
                raise ValueError("matched arms differ in initial world")
            initial_hashes[seed] = initial_sha
            initial = json.loads(initial_raw)
            if (inventory(initial) != 189 or initial["compute_pool"] != 48
                    or initial["total_storage_limit"] != 512
                    or initial["max_nodes_per_process"] != 96):
                raise ValueError("initial native world resources differ from frozen grammar")
            expected = expected_continuations(seed)
            for actor in (1, 2, 3):
                p = initial["processes"][str(actor)]
                if p["credits"] != 47 or p["code"]["nodes"][9:81] != expected[actor]:
                    raise ValueError("initial native CIL continuation differs from independent LCG")
            actions = json.loads(actions_raw)
            attempts = json.loads(attempts_raw)
            if len(attempts) != EPOCHS:
                raise ValueError("missing native attempt epoch ledger")
            previous = initial
            action_index = 0
            native_mutator_events = 0
            uniquely_changed = set()
            any_changed = set()
            native_children = set()
            child_continuation_epochs = defaultdict(set)
            launched_by = {}
            launch_tick = {}
            parent_output_before_launch = {}
            original_parent_outputs = defaultdict(set)
            child_native_writes = []
            child_other_ops = Counter()
            child_non_graphcreated = set()
            child_matching_parent_output = set()
            child_write_epochs = defaultdict(set)
            child_w3_unstarted_success_epochs = defaultdict(set)
            later_success_actors = set()
            mutation_ticks = {}
            output_after = set()
            native_graph_events = Counter()
            native_w3 = 0
            native_w3_no_starter_epochs = set()
            for epoch, digest, raw in db.execute(
                "SELECT epoch,state_sha256,state_json FROM epoch_snapshots "
                "WHERE run_id=? ORDER BY epoch", (run_id,)
            ):
                if epoch >= EPOCHS or analyze.digest(raw) != digest:
                    raise ValueError("incorrect native epoch hash/ordering")
                state = json.loads(raw)
                expected_plan = expected_actions(previous, arm, epoch)
                recorded_actions = actions[action_index:action_index + len(expected_plan)]
                if expected_plan != recorded_actions:
                    raise ValueError("unregistered host process credit or ambient injection")
                action_index += len(expected_plan)
                prefix = previous["events"]
                if state["events"][:len(prefix)] != prefix:
                    raise ValueError("cumulative native history changed")
                events = state["events"][len(prefix):]
                if len(events) != len(expected_plan) + attempts[epoch]:
                    raise ValueError("external/native attempt ledger discrepancy")
                for event in events[:len(expected_plan)]:
                    if event["origin"] != "External":
                        raise ValueError("external environmental action misattributed")
                native = events[len(expected_plan):]
                if any(analyze.internal_process(e) != e["process"] for e in native):
                    raise ValueError("false native event provenance")
                amount = 189
                if arm == "zero-credit-ambient-general":
                    amount -= 141
                if arm in AMBIENT:
                    amount += 8 * epoch
                if inventory(state) != amount - sum(attempts[:epoch+1]):
                    raise ValueError("world compute conservation violated")
                result = classify_mutation(previous, state, native)
                native_mutator_events += len(result["successful_mutators"])
                any_changed.update(result["changed_actors"])
                uniquely_changed.update(a for a, _, _ in result["unambiguous_actors"])
                for actor, pc, tick in result["unambiguous_actors"]:
                    mutation_ticks.setdefault(actor, tick)
                    if len(witnesses) < 16:
                        witnesses.append({"seed": seed, "arm": arm, "epoch": epoch,
                                          "actor": actor, "pc": pc, "tick": tick,
                                          "before_nodes": len(previous["processes"][str(actor)]["code"]["nodes"]),
                                          "after_nodes": len(state["processes"][str(actor)]["code"]["nodes"])})
                starter_this_epoch = any(
                    "SetCredits" in action["action"]
                    and action["action"]["SetCredits"]["credits"] == 1
                    for action in expected_plan
                )
                for event in native:
                    kind = event["kind"]
                    actor = event["process"]
                    if isinstance(kind, dict):
                        if "GraphLaunched" in kind:
                            descriptor = kind["GraphLaunched"]
                            child = descriptor["child_process"]
                            if (child in native_children or child <= 3
                                    or descriptor["parent_process"] != actor
                                    or str(child) not in state["processes"]):
                                raise ValueError("invalid native child or launcher ancestry")
                            native_children.add(child)
                            launched_by[child] = actor
                            launch_tick[child] = event["tick"]
                            parent_output_before_launch[child] = set(
                                original_parent_outputs[actor]
                            )
                        for name in ("GraphCreated", "GraphNodeCopied",
                                     "GraphEncoded", "GraphLaunched"):
                            if name in kind:
                                native_graph_events[name] += 1
                        if "CellWritten" in kind:
                            cell = kind["CellWritten"]
                            original_parent_outputs[actor].add(
                                (cell["key"], cell["value"])
                            )
                            if actor in mutation_ticks and event["tick"] > mutation_ticks[actor]:
                                output_after.add(actor)
                    if (actor in mutation_ticks
                            and event["tick"] > mutation_ticks[actor]
                            and kind != "Halted"
                            and not (isinstance(kind, dict) and "Trapped" in kind)):
                        later_success_actors.add(actor)
                    if actor > 3 and analyze.internal_process(event) == actor:
                        if actor not in native_children or event["tick"] <= launch_tick[actor]:
                            raise ValueError("native child instruction predates parent launch")
                        if kind != "Halted" and not (
                            isinstance(kind, dict) and "Trapped" in kind
                        ):
                            child_continuation_epochs[actor].add(epoch)
                            if epoch >= 16 and not starter_this_epoch:
                                child_w3_unstarted_success_epochs[actor].add(epoch)
                            typ = next(iter(kind)) if isinstance(kind, dict) else kind
                            child_other_ops[typ] += 1
                            if typ != "GraphCreated":
                                child_non_graphcreated.add(actor)
                            if isinstance(kind, dict) and "CellWritten" in kind:
                                cell = verify_native_child_write(
                                    event, previous, state, launch_tick
                                )
                                pc = event["pc"]
                                value_pair = (cell["key"], cell["value"])
                                if value_pair in parent_output_before_launch[actor]:
                                    child_matching_parent_output.add(actor)
                                child_write_epochs[actor].add(epoch)
                                if len(child_native_writes) < 5:
                                    child_native_writes.append({
                                        "epoch": epoch, "child": actor,
                                        "parent": launched_by[actor], "pc": pc,
                                        "tick": event["tick"],
                                        "key": cell["key"], "value": cell["value"],
                                        "matched_parent_before_launch":
                                            value_pair in parent_output_before_launch[actor],
                                        "host_starter_in_epoch": starter_this_epoch,
                                    })
                if epoch >= 16 and native:
                    native_w3 += 1
                    if not any("SetCredits" in a["action"]
                               and a["action"]["SetCredits"]["credits"] == 1
                               for a in expected_plan):
                        native_w3_no_starter_epochs.add(epoch)
                previous = state
                hash_count += 1
            if action_index != len(actions) or previous != json.loads(final_raw):
                raise ValueError("unexpected external action or final state not exact epoch23")
            if any(seed < FIRST or seed >= FIRST + SEEDS for seed in worlds):
                raise ValueError("invalid seed")
            final_changed = sum(
                initial["processes"][str(actor)]["code"]["nodes"]
                != previous["processes"][str(actor)]["code"]["nodes"]
                for actor in (1, 2, 3)
            )
            worlds[seed][arm] = {
                "qualifying_native_code_change": bool(uniquely_changed),
                "event_attributed_change": bool(any_changed),
                "native_mutator_events": native_mutator_events,
                "final_code_changed": bool(final_changed),
                "child_launched": bool(native_children),
                "child_success": any(child_continuation_epochs.values()),
                "child_non_GraphCreated_success": bool(child_non_graphcreated),
                "child_native_CellWritten": bool(child_write_epochs),
                "child_matching_parent_value_before_launch": bool(
                    child_matching_parent_output
                ),
                "child_writes_two_distinct_epochs": any(
                    len(epochs) >= 2 for epochs in child_write_epochs.values()
                ),
                "child_two_unstarted_W3_success_epochs": any(
                    len(epochs) >= 2
                    for epochs in child_w3_unstarted_success_epochs.values()
                ),
                "child_two_epochs": any(len(x) >= 2 for x in child_continuation_epochs.values()),
                "mutated_actor_further_success": bool(later_success_actors),
                "native_output_after_plasticity": bool(output_after),
                "W3_active": native_w3 > 0,
                "W3_no_starter_epochs": len(native_w3_no_starter_epochs),
                "native_graph_events": dict(native_graph_events),
                "child_native_successful_opcodes": dict(child_other_ops),
                "first_child_CellWritten_witnesses": child_native_writes,
            }
            for key, value in worlds[seed][arm].items():
                if isinstance(value, bool):
                    metrics[f"{arm}.{key}"] += int(value)
            metrics[f"{arm}.successful_native_mutator_events"] += native_mutator_events
            for opcode, count in child_other_ops.items():
                metrics[f"{arm}.native_child_{opcode}_events"] += count
            for name, value in native_graph_events.items():
                metrics[f"{arm}.native_{name}_events"] += value
        if (len(worlds) != SEEDS or any(set(row) != set(ARMS) for row in worlds.values())
                or hash_count != SEEDS * len(ARMS) * EPOCHS):
            raise ValueError("missing independent seed/world arm or epoch")
        positive = metrics["ambient-general.child_native_CellWritten"]
        lower, upper = wilson_95(positive, SEEDS)
        return {
            "study": "CL-1 Study61-absolute native executable plasticity opportunity",
            "source_revision": revision,
            "independent_worlds": SEEDS,
            "matched_runs": len(runs),
            "hashed_epoch_snapshots": hash_count,
            "primary_native_child_shared_cell_write": {
                "worlds": positive, "denominator": SEEDS,
                "wilson_95": [round(lower, 6), round(upper, 6)],
            },
            "secondary": dict(sorted(metrics.items())),
            "first_native_plasticity_witnesses": witnesses,
            "all_seed_outcomes": {
                str(seed): worlds[seed] for seed in sorted(worlds)
            },
            "source_of_ambient_compute": "EXTERNAL +8 EPOCH1..23",
            "source_of_generic_starter": "EXTERNAL NEUTRAL ONE CREDIT FROM WORLD POOL",
            "new_general_random_operand": "Const register2 LCG value uniform 0..44",
            "native_graph_opportunities": "24-choice grammar with extra generic copy/set-next/launch variants",
            "native_self_repair": "NOT EVALUABLE",
            "hereditary_autonomy": "NOT EVALUABLE",
            "life_claim": "NONE",
        }
    finally:
        db.close()


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("sqlite", type=Path)
    parser.add_argument("--expected-revision", required=True)
    parser.add_argument("--json", type=Path, required=True)
    args = parser.parse_args()
    report = audit(args.sqlite, args.expected_revision)
    args.json.write_text(json.dumps(report, indent=2) + "\n", encoding="utf-8")
    print(json.dumps({
        "study": report["study"],
        "independent_worlds": report["independent_worlds"],
        "primary": report["primary_native_child_shared_cell_write"],
        "life_claim": report["life_claim"],
    }, indent=2))


if __name__ == "__main__":
    main()
