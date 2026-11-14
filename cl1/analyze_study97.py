#!/usr/bin/env python3
"""Study97: independent original random payload/code/inherited function auditor.

This verifies researcher-scripted native assembler vs random child algorithm code.
A positive inherited phenotype is NOT autonomous emergence or reproduction.
"""
import argparse
import hashlib
import json
import sqlite3
from collections import Counter
from pathlib import Path

from analyze_study82 import mix_seed, random_draw, variant

FIRST = 6_200_000
SEEDS = 256
EPOCHS = 12
QUOTA = 12
ARMS = ("local", "absolute-stale", "zero-credit")


def require(ok, message):
    if not ok:
        raise ValueError(message)


def h(raw):
    return hashlib.sha256(raw.encode("utf-8")).hexdigest()


def node(tag, **fields):
    return {tag: fields}


def const(dst, value, nxt):
    return node("Const", dst=dst, value=value, next=nxt)


def expected_payload(seed):
    state = mix_seed(seed)
    code = [
        const(2, 7, 1),
        const(3, 9, 2),
        const(1, 1, 3),
    ]
    for pc in range(3, 15):
        nxt = pc + 1
        state, sample = random_draw(state)
        choice = sample % 5
        if choice == 0:
            code.append(node("Read", dst=0, address=2, next=nxt))
        elif choice == 1:
            code.append(node("Write", address=3, value=0, next=nxt))
        elif choice == 2:
            code.append(node("Add", dst=0, lhs=0, rhs=1, next=nxt))
        elif choice == 3:
            code.append(node("Sub", dst=0, lhs=0, rhs=1, next=nxt))
        else:
            state, sample = random_draw(state)
            code.append(const(0, sample % 8, nxt))
    code.append(node("Branch", test=2, nonzero=3, zero=3))
    require(len(code) == 16, "random original payload size changed")
    return code


def source_template(op, source_pc):
    kind, fields = variant(op)
    if kind == "Branch":
        return node("Branch", test=2, nonzero=103, zero=103)
    return node(kind, **{**fields, "next": source_pc + 1})


def expected_parent(seed, mode):
    payload = expected_payload(seed)
    code = [node("GraphNew", dst=0, next=1)]
    for index in range(16):
        pc = len(code)
        code.append(const(1, 100 + index, pc + 1))
        copy = "GraphCopyNode" if mode == "absolute-stale" else "GraphCopyNodeLocal"
        code.append(node(copy, graph=0, source=1, dst=2, next=pc + 2))
    require(len(code) == 33, "native copy assembly program length changed")
    for index in range(15):
        pc = len(code)
        code.append(const(1, index, pc + 1))
        code.append(const(3, index + 1, pc + 2))
        code.append(node("GraphSetNext", graph=0, node=1, target=3, next=pc + 3))
    require(len(code) == 78, "native generic linker scaffold changed")
    code.append(const(1, 0 if mode == "zero-credit" else 48, 79))
    code.append(node("GraphLaunch", graph=0, credits=1, dst=2, next=80))
    code.append("Halt")
    while len(code) < 100:
        code.append("Halt")
    code.extend(source_template(op, 100 + i) for i, op in enumerate(payload))
    require(len(code) == 116, "original native parent program size changed")
    return code


def event_kind(e, kind):
    raw = e["kind"]
    return raw.get(kind) if isinstance(raw, dict) else None


def actor_native(e, actor):
    return e["origin"] == {"Internal": actor} and e["process"] == actor


def evaluate_child(event_stream, source, payload):
    """Native child def-use, not numeric coincidence, under the original ABI."""
    attempts = 0
    tainted = False
    offset = 0
    records = []
    successful_reads = 0
    trapped = []
    ordered_pcs = []
    for event in event_stream:
        if not actor_native(event, 2):
            continue
        attempts += 1
        pc = event["pc"]
        require(0 <= pc < len(payload), "native child executed invalid PC")
        ordered_pcs.append(pc)
        opcode, fields = variant(payload[pc])
        trap = event_kind(event, "Trapped")
        if trap is not None:
            trapped.append({"pc": pc, "reason": trap})
            continue
        kind = event["kind"]
        if opcode == "Const":
            require(kind == "Executed", "child Const did not execute natively")
            if fields["dst"] == 0:
                tainted = False
                offset = fields["value"]
        elif opcode == "Read":
            read = event_kind(event, "CellRead")
            require(read == {"key": 7, "value": source},
                    "child cannot claim source Read without native CellRead event")
            tainted = True
            offset = 0
            successful_reads += 1
        elif opcode == "Write":
            write = event_kind(event, "CellWritten")
            require(write is not None and write["key"] == 9
                    and write["value"] == (source if tainted else 0) + offset,
                    "child Write output is not explained by native R0 def-use")
            records.append({
                "native_attempt": attempts,
                "pc": pc,
                "offset": offset,
                "tainted": tainted,
                "value": write["value"],
            })
        elif opcode == "Add":
            require(kind == "Executed", "unregistered child arithmetic event")
            offset += 1
        elif opcode == "Sub":
            require(kind == "Executed", "unregistered child arithmetic event")
            offset -= 1
        elif opcode == "Branch":
            require(kind == "Executed", "child Branch did not execute normally")
        else:
            raise ValueError("unexpected native child instruction opcode")
    return {
        "native_child_attempts": attempts,
        "child_native_pc_sequence": ordered_pcs,
        "successful_source_reads": successful_reads,
        "child_native_writes": records,
        "child_traps": trapped,
    }


def eligible_keys(child):
    return {
        (writer["pc"], writer["offset"])
        for writer in child["child_native_writes"]
        if writer["tainted"] and 1 <= writer["native_attempt"] <= 24
    } & {
        (writer["pc"], writer["offset"])
        for writer in child["child_native_writes"]
        if writer["tainted"] and 25 <= writer["native_attempt"] <= 48
    }


def audit(path, revision):
    with sqlite3.connect(path) as conn:
        rows = conn.execute(
            "SELECT id,condition,source_revision,initial_sha256,initial_json,"
            "actions_json,attempts_json,final_sha256,final_json "
            "FROM epoch_runs ORDER BY id"
        ).fetchall()
        require(len(rows) == SEEDS * 2 * len(ARMS),
                "original registered 1536 world denominator incomplete")
        runs = {}
        payload_hashes = {}
        full_epoch_hashes = 0
        for run_id, label, source_sha, init_hash, raw_init, raw_actions, raw_charges, final_hash, raw_final in rows:
            require(source_sha == revision and source_sha != "unverified-local-checkout",
                    "source SHA mismatch or unpinned original world")
            parts = label.split("/")
            require(len(parts) == 3 and parts[0].startswith("seed=")
                    and parts[1] in ("source=3", "source=8")
                    and parts[2] in ARMS, "unregistered original genotype/source/mode")
            seed = int(parts[0].removeprefix("seed="))
            source = int(parts[1].removeprefix("source="))
            mode = parts[2]
            require(FIRST <= seed < FIRST + SEEDS and (seed, source, mode) not in runs,
                    "original seed overlap or repeated matched arm")
            require(h(raw_init) == init_hash and h(raw_final) == final_hash,
                    "original initial/final canonical SHA256 incorrect")
            initial = json.loads(raw_init)
            final = json.loads(raw_final)
            payload = expected_payload(seed)
            parent_code = expected_parent(seed, mode)
            actual_original = initial["processes"]["1"]
            require(actual_original["code"]["nodes"] == parent_code
                    and actual_original["credits"] == 192,
                    "original executable parent differs from independent generator")
            require(initial["cells"]["7"] == source and initial["compute_pool"] == 0,
                    "source variable/compute starting physics changed")
            require(initial["strict_accounting"] and initial["total_storage_limit"] == 320,
                    "researcher preset storage law changed")
            require(json.loads(raw_actions) == [], "undeclared after-fork external action")
            payload_hashes[seed] = h(json.dumps(payload, sort_keys=True))
            charges = json.loads(raw_charges)
            require(len(charges) == EPOCHS and all(
                type(n) is int and 0 <= n <= QUOTA for n in charges
            ), "wrong native charged attempts")
            snaps = conn.execute(
                "SELECT epoch,state_sha256,state_json FROM epoch_snapshots "
                "WHERE run_id=? ORDER BY epoch",
                (run_id,),
            )
            previous = initial
            count = 0
            for epoch, state_hash, raw_state in snaps:
                require(epoch == count and h(raw_state) == state_hash,
                        "original native full snapshot SHA256/order invalid")
                state = json.loads(raw_state)
                old_events = previous["events"]
                new_events = state["events"]
                require(new_events[:len(old_events)] == old_events,
                        "native event history prefix changed between epochs")
                require(len(new_events) - len(old_events) == charges[count],
                        "unlogged host action or missing native charged instruction")
                require(all(e["origin"] in ({"Internal": 1}, {"Internal": 2})
                            for e in new_events[len(old_events):]),
                        "external host actor/goal provenance appeared during ecology")
                previous = state
                count += 1
                full_epoch_hashes += 1
            require(count == EPOCHS and previous == final,
                    "missing exact full terminal SQLite world snapshot")
            native_charges = sum(charges)
            remaining = final["compute_pool"] + sum(p["credits"]
                for p in final["processes"].values())
            require(remaining == 192 - native_charges,
                    "native shared resource/child transfer conservation violated")

            parent_events = [e for e in final["events"] if actor_native(e, 1)]
            child_events = [e for e in final["events"] if actor_native(e, 2)]
            require(not any(event_kind(e, "CellWritten") is not None for e in parent_events),
                    "researcher-coded assembler itself made claimed native child output")
            copy_tag = "GraphNodeCopied" if mode == "absolute-stale" else "GraphNodeCopiedLocal"
            actual_copies = [event_kind(e, copy_tag) for e in parent_events
                             if event_kind(e, copy_tag) is not None]
            require(len(actual_copies) == 16,
                    "not exactly sixteen actor-native copied template CIL nodes")
            links = [event_kind(e, "GraphNodeEdited") for e in parent_events
                     if event_kind(e, "GraphNodeEdited") is not None]
            require(len(links) == 15, "not exactly fifteen native generic child edge links")
            launched = [event_kind(e, "GraphLaunched") for e in parent_events
                        if event_kind(e, "GraphLaunched") is not None]
            require(len(launched) == 1, "native child must be launched by true parent")
            launch = launched[0]
            require(
                launch["parent_process"] == 1
                and launch["child_process"] == 2
                and launch["moved_nodes"] == 16
                and launch["transferred_credits"] == (0 if mode == "zero-credit" else 48),
                "invalid native child lineage or credit transfer",
            )
            child = final["processes"].get("2")
            require(child is not None and len(child["code"]["nodes"]) == 16,
                    "original native executable child graph missing")
            expected_child = expected_payload(seed)
            if mode == "absolute-stale":
                expected_child[-1] = node("Branch", test=2, nonzero=103, zero=103)
            require(child["code"]["nodes"] == expected_child,
                    "child executable code differs from independently generated inherited payload")
            phenotype = evaluate_child(child_events, source, expected_child)
            if mode == "local":
                require(phenotype["native_child_attempts"] == 48,
                        "local child not funded with the registered finite48 credits")
            elif mode == "zero-credit":
                require(not child_events and phenotype["native_child_attempts"] == 0,
                        "zero-credited child executed native instruction")
            elif mode == "absolute-stale":
                require(phenotype["native_child_attempts"] <= 16,
                        "stale copied child unexpectedly ran beyond first pass")
                require(not eligible_keys(phenotype),
                        "invalid stale Branch yielded repeated native functional child output")
            runs[(seed, source, mode)] = {
                "payload_hash": payload_hashes[seed],
                "child": phenotype,
                "launch": launch,
                "attempts": native_charges,
            }

    require(len(payload_hashes) == SEEDS and len(runs) == SEEDS * 6,
            "missing original random payload labels or original matched native worlds")
    statuses = Counter()
    outcomes = []
    for seed in range(FIRST, FIRST + SEEDS):
        per_arm = {}
        for arm in ARMS:
            low = eligible_keys(runs[(seed, 3, arm)]["child"])
            high = eligible_keys(runs[(seed, 8, arm)]["child"])
            inherited = sorted(low & high)
            F = bool(inherited)
            if F:
                statuses[f"native_source_function_{arm}"] += 1
            per_arm[arm] = {
                "paired_inherited_F": F,
                "paired_child_writer_pc_and_affine_offsets": inherited,
                "native_child_attempts_source3": runs[(seed, 3, arm)]["child"]["native_child_attempts"],
                "native_child_attempts_source8": runs[(seed, 8, arm)]["child"]["native_child_attempts"],
            }
        require(not per_arm["absolute-stale"]["paired_inherited_F"]
                and not per_arm["zero-credit"]["paired_inherited_F"],
                "negative copy/child-energy control gained repeated source function")
        if per_arm["local"]["paired_inherited_F"]:
            statuses["local_only_paired_source_function"] += 1
        outcomes.append({"seed": seed, "source_paired_results": per_arm})
    return {
        "study": 97,
        "status": "PROSPECTIVE_STOCHASTIC_PAYLOAD_CONDITIONAL_ON_HUMAN_SCRIPTED_ASSEMBLER",
        "source_revision": revision,
        "sampled_original_seed_labels": SEEDS,
        "distinct_child_payload_hashes": len(set(payload_hashes.values())),
        "original_source_arms": len(runs),
        "verified_original_epoch_sha256": full_epoch_hashes,
        "source_paired_repeated_child_function_local_F": statuses["native_source_function_local"],
        "source_paired_repeated_child_function_absolute_stale_F": statuses["native_source_function_absolute-stale"],
        "source_paired_repeated_child_function_zero_credit_F": statuses["native_source_function_zero-credit"],
        "paired_only_local_inherited_source_function": statuses["local_only_paired_source_function"],
        "human_scripted_parent_assembler": True,
        "spontaneous_reproduction_or_open_evolution": False,
        "per_original_payload_outcome": outcomes,
        "cl1_closed": False,
    }


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("sqlite", type=Path)
    parser.add_argument("--expected-revision", required=True)
    parser.add_argument("--json", type=Path, required=True)
    args = parser.parse_args()
    result = audit(args.sqlite, args.expected_revision)
    args.json.write_text(json.dumps(result, indent=2, sort_keys=True) + "\n")
    print(json.dumps({
        key: result[key] for key in (
            "study", "status", "sampled_original_seed_labels",
            "distinct_child_payload_hashes", "original_source_arms",
            "verified_original_epoch_sha256",
            "source_paired_repeated_child_function_local_F",
            "source_paired_repeated_child_function_absolute_stale_F",
            "source_paired_repeated_child_function_zero_credit_F",
        )
    }, sort_keys=True))


if __name__ == "__main__":
    main()
