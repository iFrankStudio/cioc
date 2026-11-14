#!/usr/bin/env python3
"""Precommitted Study113: all-target actor-native CIL mutation and graph-copy audit.

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

FIRST = 6_600_000
N = 64
EPOCHS = 16
QUOTA = 250
MODES = (
    "native-edit-source3", "patch-disabled-source3",
    "native-edit-source8", "patch-disabled-source8",
)
PATCHABLE = set(range(28)) - {12, 19, 27, 16}

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
    expressed_changed = set()
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
        require(0 <= pc < 28, "invalid native executable PC")
        if actor in child_birth_tick and pc == site and codes[actor][site] != original_nodes[site]:
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
            require(pc == 2, "source sensor read at non-native PC")
            actor_reads[actor] = read["value"]
        if written is not None and written["key"] == 9:
            require(pc == 3, "source function writer at non-native PC")
            if written["value"] == source and actor_reads.get(actor) == source:
                source_writers.add(actor)
            else:
                invalid_outputs += 1

        if edited is not None:
            require(not ablated and pc == 8 and edited["node"] == site,
                    "unregistered source of native CIL executable mutation")
            source_op = codes[actor][site]
            op_payload = next(iter(source_op.values())) if isinstance(source_op, dict) else None
            require(isinstance(op_payload, dict) and "next" in op_payload,
                    "reported native edit on nonpatchable opcode")
            require(op_payload["next"] == edited["before"],
                    "native code edit has wrong previous executable edge")
            require(edited["before"] != edited["after"]
                    and 0 <= edited["after"] < 28,
                    "invalid native edit target")
            op_payload["next"] = edited["after"]
            edits.append({"actor": actor, "site": site, "from": edited["before"],
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
            require(len(graph_nodes) == launched["moved_nodes"] == 28,
                    "child code not fully copied from original native CIL graph")
            require(launched["transferred_credits"] == grant,
                    "child transfer mismatch")
            require(native_draws[actor] > actor_launches[actor],
                    "CIL newborn not funded by preceding native DrawCompute")
            actor_launches[actor] += 1
            code_birth = copy.deepcopy(graph_nodes)
            codes[child] = code_birth
            child_birth_tick[child] = e["tick"]
            mutated = code_birth != original_nodes
            if mutated:
                changed_births += 1
            births.append({
                "actor": actor, "child": child, "tick": e["tick"],
                "born_changed": mutated, "born_site_next": (
                    next(iter(code_birth[site].values())).get("next")
                    if isinstance(code_birth[site], dict) else None
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
    require(len(entries) == N * len(MODES), "wrong complete 256 original world denominator")
    results = {}
    genome_by_seed = defaultdict(set)
    target_pairs = {}
    snaps = 0
    charged_total = 0
    for (rid, label, rev, initial_json, initial_sha, actions_json,
         attempts_json, final_json, final_sha) in entries:
        require(rev == revision and len(revision) == 40, "unregistered original source commit")
        require(label.startswith("seed="), "wrong original genotype label")
        seed_s, arm = label[5:].split("/", 1)
        seed = int(seed_s)
        require(FIRST <= seed < FIRST + N and arm in MODES and (seed, arm) not in results,
                "unregistered duplicate original seed/arm")
        require(sha(initial_json) == initial_sha and sha(final_json) == final_sha,
                "original recorded SHA256 initial/final mismatch")
        init = json.loads(initial_json)
        source = 3 if arm.endswith("source3") else 8
        ablated = arm.startswith("patch-disabled-")
        require(init["compute_pool"] == 800 and init["cells"]["7"] == source,
                "incorrect public world pool or source")
        require(len(init["processes"]) == 2
                and init["processes"]["1"]["code"] == init["processes"]["2"]["code"],
                "original CIL founder code not byte identical")
        require(all(init["processes"][str(pid)]["credits"] == 500 for pid in (1, 2)),
                "original founder credits mismatch")
        require(init.get("native_patchnext_disabled", False) == ablated
                and init["trace_native_executable_edits"] is True,
                "world law or edit origin observer incorrect")
        genotype = init["processes"]["1"]["code"]
        nodes = genotype["nodes"]
        site = nodes[7]["Const"]["value"]
        target = init["cells"]["11"]
        delay = nodes[24]["Const"]["value"]
        grant = nodes[20]["Const"]["value"]
        require(0 <= site < 28 and 0 <= target < 28
                and 1 <= delay <= 32 and 320 <= grant <= 480,
                "unregistered full-site and full-target genome")
        require(nodes[9] == {"Const": {"dst": 1, "value": 27, "next": 10}},
                "mutation target complement not frozen")
        require(nodes[4] == {"Const": {"dst": 2, "value": 1, "next": 24}},
                "original executable function not slow")
        genotype_hash = sha(json.dumps(genotype, sort_keys=True))
        genome_by_seed[seed].add(genotype_hash)
        target_pairs.setdefault(seed, set()).add((site, target))
        snaps_raw = conn.execute(
            "SELECT epoch,state_sha256,state_json FROM epoch_snapshots "
            "WHERE run_id=? ORDER BY epoch", (rid,)
        ).fetchall()
        require(len(snaps_raw) == EPOCHS, "missing native exact full epoch snapshots")
        actions = json.loads(actions_json)
        require(len(actions) == EPOCHS-1, "incorrect host external action count")
        for i, action in enumerate(actions, 1):
            require(action["epoch"] == i and "SetComputePool" in action["action"],
                    "unregistered targeted host action")
            earlier = json.loads(snaps_raw[i-1][2])
            require(action["action"]["SetComputePool"]["credits"] ==
                    earlier["compute_pool"] + 400,
                    "public ambient supply was not exactly unconditional +400")
        for i, (ep, digest, raw) in enumerate(snaps_raw):
            require(ep == i and sha(raw) == digest,
                    "full original native CIL epoch SHA256 invalid")
            snaps += 1
        require(snaps_raw[-1][2] == final_json, "last saved world not final")
        attempts = json.loads(attempts_json)
        require(len(attempts) == EPOCHS and all(0 <= a <= QUOTA for a in attempts),
                "out-of-contract charged CIL attempts")
        actual = sum(attempts)
        charged_total += actual
        final = json.loads(final_json)
        require(sum(e["origin"] == "External" for e in final["events"]) == 20,
                "unregistered host action in complete native event history")
        require(final["compute_pool"] + sum(
            p["credits"] for p in final["processes"].values()) == 7800-actual,
            "native public and actor charged compute conservation failed")
        result = actual_actor_history(init, final, site, ablated, source, grant, actual)
        results[(seed, arm)] = {
            "seed": seed, "arm": arm, "source": source,
            "patch_site": site, "initial_edge_target": target,
            "initial_code_hash": genotype_hash,
            "attempts": actual,
            "pool_final": final["compute_pool"],
            **result,
        }

    require(len(results) == N * len(MODES) and len(genome_by_seed) == N,
            "not all prespecified native site/target combinations run")
    require(all(len(v) == 1 for v in genome_by_seed.values())
            and len(set(next(iter(v)) for v in genome_by_seed.values())) == N,
            "initial seeded code genomes not distinct or matched across arms")
    require(all(len(v) == 1 for v in target_pairs.values()),
            "site/target changed across matched source/mutation arms")
    by_source = {}
    for source in (3, 8):
        deltas = [
            results[(seed, f"native-edit-source{source}")]["births"] -
            results[(seed, f"patch-disabled-source{source}")]["births"]
            for seed in range(FIRST, FIRST+N)
        ]
        by_source[f"source{source}"] = {
            "full_64_seed_signed_birth_deltas": deltas,
            "positive": sum(x>0 for x in deltas),
            "negative": sum(x<0 for x in deltas),
            "ties": sum(x==0 for x in deltas),
            "descriptive_two_sided_sign_p": sign_p(deltas),
            "actual_edit_exposure_seeds": sum(
                results[(seed, f"native-edit-source{source}")]["changed_native_edit_events"] > 0
                for seed in range(FIRST, FIRST+N)),
            "changed_code_child_seeds": sum(
                results[(seed, f"native-edit-source{source}")]["changed_code_births"] > 0
                for seed in range(FIRST, FIRST+N)),
            "inherited_changed_site_expression_seeds": sum(
                results[(seed, f"native-edit-source{source}")]["expressed_changed_actors"] > 0
                for seed in range(FIRST, FIRST+N)),
            "native_enabled_births": sum(
                results[(seed, f"native-edit-source{source}")]["births"]
                for seed in range(FIRST, FIRST+N)),
            "native_disabled_births": sum(
                results[(seed, f"patch-disabled-source{source}")]["births"]
                for seed in range(FIRST, FIRST+N)),
        }
    return {
        "status": "UNSELECTED_NATIVE_ALL_SITE_TARGET_HUMAN_CHASSIS_SURVEY_NOT_EVOLUTION",
        "source_revision": revision,
        "initial_distinct_parameter_genomes": N,
        "spontaneous_replicator_origins": 0,
        "matched_worlds": len(results),
        "exact_original_SHA256_epochs": snaps,
        "total_native_charged_attempts": charged_total,
        "all_seed_parameters": [
            {"seed": s, "site": next(iter(target_pairs[s]))[0],
             "target": next(iter(target_pairs[s]))[1]}
            for s in range(FIRST, FIRST+N)
        ],
        "comparison_by_source": by_source,
        "all_unfiltered_original_world_results": [
            results[(s, arm)] for s in range(FIRST, FIRST+N) for arm in MODES
        ],
        "cyberlife_proven": False,
    }

def main():
    p = argparse.ArgumentParser()
    p.add_argument("sqlite", type=Path)
    p.add_argument("--expected-revision", required=True)
    p.add_argument("--json", type=Path, required=True)
    a = p.parse_args()
    result = audit(a.sqlite, a.expected_revision)
    a.json.write_text(json.dumps(result, sort_keys=True, indent=2)+"\n")
    print(json.dumps({
        "status": result["status"],
        "matched_worlds": result["matched_worlds"],
        "initial_distinct_parameter_genomes": result["initial_distinct_parameter_genomes"],
        "exact_original_SHA256_epochs": result["exact_original_SHA256_epochs"],
        "comparison_by_source": result["comparison_by_source"],
    }, sort_keys=True))

if __name__ == "__main__":
    main()
