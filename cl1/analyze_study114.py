#!/usr/bin/env python3
"""Precommitted Study114: all-target actor-native CIL mutation and graph-copy audit.

The CIL self-copy strategy is human-designed. Original random target sites and
destinations are unselected; all 64 seeds and even structurally invalid edits count.
"""
import argparse
import copy
import hashlib
import json
import math
import sqlite3
from collections import Counter, defaultdict
from pathlib import Path

FIRST = 6_700_000
N = 32
EPOCHS = 16
QUOTA = 250
MODES = (
    "native-core-source3", "disabled-core-source3",
    "native-full-source3", "disabled-full-source3",
    "native-core-source8", "disabled-core-source8",
    "native-full-source8", "disabled-full-source8",
)

def require(x, message):
    if not x:
        raise ValueError(message)

def sha(text):
    return hashlib.sha256(text.encode("utf-8")).hexdigest()

def event_part(event, key):
    value = event["kind"]
    return value.get(key) if isinstance(value, dict) else None

def sign_p(deltas):
    a = sum(x > 0 for x in deltas)
    b = sum(x < 0 for x in deltas)
    n = a + b
    if not n:
        return 1.0
    return min(1.0, 2 * sum(math.comb(n, k)
                           for k in range(min(a, b) + 1)) / 2**n)

def actual_actor_history(initial, final, site, ablated, source, grant, attempts):
    initial_nodes = initial["processes"]["1"]["code"]["nodes"]
    codes = {1: copy.deepcopy(initial_nodes), 2: copy.deepcopy(initial_nodes)}
    original_nodes = copy.deepcopy(initial_nodes)
    active_graphs = {}
    actor_reads = {}
    source_writers = set()
    invalid_outputs = 0
    births = []
    edits = []
    suppressed = 0
    traps = 0
    native_draws = Counter()
    actor_launches = Counter()
    changed_births = 0
    incomplete_copy_births = 0
    expressed_changed = set()
    # For each child remember EXACT modified instructions copied at its birth;
    # do not confuse later actor-self mutation with inheritance expression.
    inherited_differences = {}
    child_birth_tick = {}
    native_events = 0

    for e in final["events"]:
        origin = e["origin"]
        if origin == "External":
            continue
        actor = e["process"]
        require(origin == {"Internal": actor}, "non-actor internal event")
        require(actor in codes, "CIL executing before native original/child load")
        native_events += 1
        pc = e["pc"]
        require(0 <= pc < 56, "invalid native executable PC")
        if (actor in inherited_differences
                and pc in inherited_differences[actor]
                and codes[actor][pc] == inherited_differences[actor][pc]):
            expressed_changed.add(actor)
        read = event_part(e, "CellRead")
        written = event_part(e, "CellWritten")
        edited = event_part(e, "ExecutableEdgePatched")
        disabled = event_part(e, "NativeCodeEditSuppressed")
        created = event_part(e, "GraphCreated")
        copied = event_part(e, "GraphNodeCopied")
        launched = event_part(e, "GraphLaunched")
        trapped = event_part(e, "Trapped")

        if read is not None and read["key"] == 7:
            # Untargeted edge edits can skip the Const address setup, causing
            # the OTHER existing Read at PC6 to read key7. That is a genuine
            # but noncanonical read, not evidence of the original PC2 sensor.
            require("Read" in codes[actor][pc],
                    "native source read not from an executable Read opcode")
            if pc == 2:
                actor_reads[actor] = read["value"]
        if written is not None and written["key"] == 9:
            require("Write" in codes[actor][pc],
                    "native source output not from an executable Write opcode")
            if pc == 3 and written["value"] == source and actor_reads.get(actor) == source:
                source_writers.add(actor)
            else:
                invalid_outputs += 1

        if edited is not None:
            # Initial mutator site was preregistered, but native mutations can
            # subsequently alter the CIL's own control/data flow and therefore
            # dynamically change register R1 before PC8. This is a REAL
            # descendant code-state consequence, not a researcher-resampled site.
            # Verify the actor executes original native PatchNext instruction
            # and edit-before matches its reconstructed current executable node.
            require(not ablated and pc == 8 and 0 <= edited["node"] < 56
                    and "PatchNext" in codes[actor][8],
                    "unregistered source of native CIL executable mutation")
            source_op = codes[actor][edited["node"]]
            op_payload = next(iter(source_op.values())) if isinstance(source_op, dict) else None
            require(isinstance(op_payload, dict) and "next" in op_payload,
                    "reported native edit on nonpatchable opcode")
            require(op_payload["next"] == edited["before"],
                    "native code edit has wrong previous executable edge")
            require(edited["before"] != edited["after"]
                    and 0 <= edited["after"] < 56,
                    "invalid native edit target")
            op_payload["next"] = edited["after"]
            edits.append({"actor": actor, "site": edited["node"], "from": edited["before"],
                          "to": edited["after"], "tick": e["tick"]})
        if disabled is not None:
            require(ablated and pc == 8 and disabled["opcode_tag"] == 6,
                    "unregistered ablated native edit or incorrect PC")
            suppressed += 1
        if trapped is not None:
            traps += 1
        if created is not None:
            require(pc == 15 and created["owner"] == actor,
                    "graph was not made by native CIL actor")
            graph_id = created["graph"]
            require(graph_id not in active_graphs,
                    "native graph identifier unexpectedly reused")
            active_graphs[graph_id] = {"owner": actor, "nodes": []}
        if copied is not None:
            require(pc == 16 and copied["owner"] == actor
                    and copied["source_process"] == actor,
                    "native child code copied from wrong process")
            graph_id = copied["graph"]
            require(graph_id in active_graphs
                    and active_graphs[graph_id]["owner"] == actor,
                    "CIL copied node to a graph not owned by actor")
            parts = active_graphs[graph_id]["nodes"]
            require(copied["new_node"] == len(parts), "graph code copy out of registered order")
            require(0 <= copied["source_node"] < len(codes[actor]),
                    "graph code source outside current executable")
            parts.append(copy.deepcopy(codes[actor][copied["source_node"]]))
        if pc == 21 and e["kind"] == "Executed":
            native_draws[actor] += 1
        if launched is not None:
            require(pc == 22 and launched["parent_process"] == actor,
                    "native birth not sourced to actual parent PC")
            child = launched["child_process"]
            graph = launched["source_graph"]
            require(child not in codes and child >= 3, "host injected or duplicate CIL child")
            require(graph in active_graphs
                    and active_graphs[graph]["owner"] == actor,
                    "birth not sourced to parent-owned native graph")
            graph_nodes = active_graphs[graph]["nodes"]
            # An unselected mutation may corrupt the copy loop itself and
            # launch a SHORTER genuinely copied child. Count that native birth,
            # but do not falsely declare 56-node faithful inheritance.
            require(0 < len(graph_nodes) == launched["moved_nodes"] <= 56,
                    "native child moved-node count disagrees with actual copied graph")
            require(launched["transferred_credits"] == grant,
                    "child transfer mismatch")
            require(native_draws[actor] > actor_launches[actor],
                    "CIL newborn not funded by preceding native DrawCompute")
            actor_launches[actor] += 1
            code_birth = copy.deepcopy(graph_nodes)
            codes[child] = code_birth
            child_birth_tick[child] = e["tick"]
            inherited_differences[child] = {
                i: copy.deepcopy(node)
                for i, node in enumerate(code_birth)
                if node != original_nodes[i]
            }
            incomplete = len(code_birth) != 56
            if incomplete:
                incomplete_copy_births += 1
            mutated = incomplete or bool(inherited_differences[child])
            if mutated:
                changed_births += 1
            births.append({
                "actor": actor, "child": child, "tick": e["tick"],
                "born_changed": mutated,
                "born_incomplete_copy": incomplete,
                "born_code_nodes": len(code_birth),
                "born_site_next": (
                    next(iter(code_birth[site].values())).get("next")
                    if site < len(code_birth) and isinstance(code_birth[site], dict)
                    else None
                ),
            })
            # Parent can create a new graph on a subsequent PC15; old graph is
            # never given more nodes or reinterpreted after its launch.

    require(native_events == attempts, "native event count differs from paid instructions")
    require({1, 2}.issubset(source_writers),
            "original CIL founders did not execute full source-responsive function")
    require(not (ablated and edits) and not (not ablated and suppressed),
            "mutation-only ablation event or native edit ancestry incorrect")
    require(len(final["processes"]) == len(codes),
            "actual stored native CIL actors do not match genealogy")
    for actor, code in codes.items():
        require(final["processes"][str(actor)]["code"]["nodes"] == code,
                f"original native event-reconstructed child code mismatch actor{actor}")
    return {
        "births": len(births),
        "changed_code_births": changed_births,
        "incomplete_copy_births": incomplete_copy_births,
        "expressed_changed_actors": len(expressed_changed),
        "changed_native_edit_events": len(edits),
        "suppressed_native_edits": suppressed,
        "traps": traps,
        "native_function_writers": len(source_writers),
        "invalid_function_outputs": invalid_outputs,
        "native_grandchildren": sum(b["actor"] >= 3 for b in births),
        "example_mutations": edits[:6],
        "example_births": births[:6],
    }

def audit(db, revision):
    conn = sqlite3.connect(db)
    entries = conn.execute(
        "SELECT id,condition,source_revision,initial_json,initial_sha256,"
        "actions_json,attempts_json,final_json,final_sha256 FROM epoch_runs ORDER BY id"
    ).fetchall()
    require(len(entries) == N * len(MODES), "not all 256 frozen policy/input/ablation worlds")
    results = {}
    original_genomes = defaultdict(set)
    params = defaultdict(dict)
    snaps = 0
    paid_total = 0

    for (rid, label, rev, initial_json, initial_sha, actions_json,
         attempts_json, final_json, final_sha) in entries:
        require(rev == revision and len(revision) == 40, "unregistered source revision")
        require(label.startswith("seed="), "invalid registered original seed")
        seed_s, arm = label[5:].split("/", 1)
        seed = int(seed_s)
        require(FIRST <= seed < FIRST+N and arm in MODES and (seed, arm) not in results,
                "duplicate or unregistered original experiment arm")
        require(sha(initial_json) == initial_sha and sha(final_json) == final_sha,
                "invalid original complete recorded state hash")
        init = json.loads(initial_json)
        source = 3 if arm.endswith("source3") else 8
        ablated = arm.startswith("disabled-")
        lottery = "core" if "-core-" in arm else "full"
        require(init["compute_pool"] == 800 and init["cells"]["7"] == source,
                "invalid original public pool or source input")
        site = init["cells"]["12"]
        target = init["cells"]["11"]
        require(0 <= site < (29 if lottery == "core" else 56) and 0 <= target < 56,
                "unregistered original site/target lottery")
        require(len(init["processes"]) == 2
                and init["processes"]["1"]["code"] == init["processes"]["2"]["code"],
                "original byte-identical founder CIL code not present")
        require(all(init["processes"][str(pid)]["credits"] == 500 for pid in (1,2)),
                "incorrect original local actor credits")
        require(init.get("native_patchnext_disabled", False) == ablated
                and init["trace_native_executable_edits"] is True,
                "original mutation negative law/code trace missing")
        genome = init["processes"]["1"]["code"]
        nodes = genome["nodes"]
        require(len(nodes) == 56, "not full original 56-node true-copy CIL genome")
        require(nodes[7] == {"Const": {"dst": 3, "value": 12, "next": 28}}
                and nodes[28] == {"Read": {"dst": 1, "address": 3, "next": 8}},
                "mutator site must be fetched from externally seeded ordinary cell12")
        require(nodes[9] == {"Const": {"dst": 1, "value": 55, "next": 10}}
                and nodes[14] == {"Const": {"dst": 3, "value": 56, "next": 15}},
                "mutator target complement or full-genome copy limit modified")
        require(all(nodes[i] == {"Const": {"dst": 0, "value": 0, "next": i}}
                    for i in range(29,56)),
                "dormant initial CIL buffer not exactly frozen")
        grant = nodes[20]["Const"]["value"]
        delay = nodes[24]["Const"]["value"]
        require(320 <= grant <= 480 and 1 <= delay <= 32,
                "non-frozen replicated CIL functional parameters")
        genome_hash = sha(json.dumps(genome, sort_keys=True))
        original_genomes[seed].add(genome_hash)
        params[seed].setdefault("target", set()).add(target)
        params[seed].setdefault(lottery, set()).add(site)
        states = conn.execute(
            "SELECT epoch,state_sha256,state_json FROM epoch_snapshots "
            "WHERE run_id=? ORDER BY epoch", (rid,)
        ).fetchall()
        require(len(states) == EPOCHS, "missing full original CIL epochs")
        actions = json.loads(actions_json)
        require(len(actions) == EPOCHS - 1, "missing or targeted host actions")
        for i, action in enumerate(actions, 1):
            require(action["epoch"] == i and "SetComputePool" in action["action"],
                    "unregistered environment action")
            prior = json.loads(states[i-1][2])
            require(action["action"]["SetComputePool"]["credits"] ==
                    prior["compute_pool"] + 400,
                    "host supply was not exactly untargeted +400 credits")
        for i, (epoch, hs, raw) in enumerate(states):
            require(epoch == i and sha(raw) == hs,
                    "original 56-node complete SHA256 epoch mismatch")
            snaps += 1
        require(states[-1][2] == final_json, "last epoch not exact final original")
        charged = json.loads(attempts_json)
        require(len(charged) == EPOCHS and all(0 <= c <= QUOTA for c in charged),
                "incorrect registered native attempts quota")
        actual = sum(charged)
        paid_total += actual
        final = json.loads(final_json)
        require(sum(e["origin"] == "External" for e in final["events"]) == 21,
                "host actor/code intervention not frozen")
        require(final["compute_pool"] + sum(
            p["credits"] for p in final["processes"].values()) == 7800-actual,
            "original externally credited compute resource accounting corrupted")
        ancestry = actual_actor_history(init, final, site, ablated, source, grant, actual)
        results[(seed, arm)] = {
            "seed": seed, "arm": arm, "source": source,
            "site_policy": lottery, "selected_site": site,
            "mutation_target": target, "genome_hash": genome_hash,
            "native_attempts": actual, "native_public_pool_left": final["compute_pool"],
            **ancestry,
        }

    require(len(results) == 256 and len(original_genomes) == N,
            "all original 32 site-policy matched-genotype units not included")
    require(all(len(h) == 1 for h in original_genomes.values()),
            "actor source genome differs across core/full and enabled/disabled worlds")
    require(all(len(p["target"]) == len(p["core"]) == len(p["full"]) == 1
                for p in params.values()),
            "original site/target policy differs within matching source arms")
    by_source = {}
    for source in (3,8):
        differences = []
        core_deltas = []
        full_deltas = []
        for seed in range(FIRST,FIRST+N):
            core = results[(seed,f"native-core-source{source}")]["births"] - (
                results[(seed,f"disabled-core-source{source}")]["births"]
            )
            full = results[(seed,f"native-full-source{source}")]["births"] - (
                results[(seed,f"disabled-full-source{source}")]["births"]
            )
            core_deltas.append(core)
            full_deltas.append(full)
            differences.append(full-core)
        by_source[f"source{source}"] = {
            "core_original_32_signed_native_child_effects": core_deltas,
            "full_original_32_signed_native_child_effects": full_deltas,
            "full_minus_core_32_buffer_interaction": differences,
            "interaction_positive": sum(d>0 for d in differences),
            "interaction_negative": sum(d<0 for d in differences),
            "interaction_ties": sum(d==0 for d in differences),
            "descriptive_two_sided_sign_p": sign_p(differences),
            "full_site_dormant_original_seeds": sum(
                next(iter(params[seed]["full"])) >= 29
                for seed in range(FIRST,FIRST+N)
            ),
            "full_site_edited_child_seeds": sum(
                results[(seed,f"native-full-source{source}")]["changed_code_births"] > 0
                for seed in range(FIRST,FIRST+N)),
            "core_site_edited_child_seeds": sum(
                results[(seed,f"native-core-source{source}")]["changed_code_births"] > 0
                for seed in range(FIRST,FIRST+N)),
            "native_core_birth_total": sum(
                results[(seed,f"native-core-source{source}")]["births"]
                for seed in range(FIRST,FIRST+N)),
            "native_full_birth_total": sum(
                results[(seed,f"native-full-source{source}")]["births"]
                for seed in range(FIRST,FIRST+N)),
        }
    return {
        "status": "MATCHED_HUMAN_ENGINEERED_56_NODE_DORMANT_CODE_BUFFER_NOT_EVOLUTION",
        "source_revision": revision,
        "original_seed_labels": N,
        "distinct_initial_genome_hashes": len(set(
            next(iter(v)) for v in original_genomes.values()
        )),
        "original_matched_worlds": len(results),
        "exact_full_state_SHA256_epochs": snaps,
        "original_charged_CIL_attempts": paid_total,
        "all_original_source_parameters": [{
            "seed": seed, "core_site": next(iter(params[seed]["core"])),
            "full_site": next(iter(params[seed]["full"])),
            "target": next(iter(params[seed]["target"])),
        } for seed in range(FIRST,FIRST+N)],
        "primary_buffer_interaction_by_source": by_source,
        "all256_original_world_provenance": [
            results[(seed,arm)] for seed in range(FIRST,FIRST+N) for arm in MODES
        ],
        "spontaneous_CIL_replicator_origins": 0,
        "cyberlife_proven": False,
    }

def main():
    p = argparse.ArgumentParser()
    p.add_argument("sqlite", type=Path)
    p.add_argument("--expected-revision", required=True)
    p.add_argument("--json", type=Path, required=True)
    args = p.parse_args()
    result = audit(args.sqlite, args.expected_revision)
    args.json.write_text(json.dumps(result, indent=2, sort_keys=True)+"\n")
    print(json.dumps({
        "status": result["status"],
        "original_matched_worlds": result["original_matched_worlds"],
        "distinct_initial_genome_hashes": result["distinct_initial_genome_hashes"],
        "exact_full_state_SHA256_epochs": result["exact_full_state_SHA256_epochs"],
        "primary_buffer_interaction_by_source": result["primary_buffer_interaction_by_source"],
    }, sort_keys=True))

if __name__ == "__main__":
    main()
