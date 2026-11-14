#!/usr/bin/env python3
"""Read-only Study112 ORIGINAL SQLite audit: scripted finite-chassis seeded variants.

Full original SHA256 state snapshots, independent actor-PC/event-source, actual native
code mutation and child-launch ancestry. No life, spontaneous genesis or fitness oracle.
"""
import argparse
import hashlib
import json
import math
import sqlite3
from collections import Counter, defaultdict
from pathlib import Path

FIRST = 6_500_000
GENOTYPES = 32
EPOCHS = 16
QUOTA = 250
MODES = (
    "native-edit-source3-ambient-on",
    "patch-disabled-source3-ambient-on",
    "native-edit-source3-ambient-off",
    "patch-disabled-source3-ambient-off",
    "native-edit-source8-ambient-on",
    "patch-disabled-source8-ambient-on",
    "native-edit-source8-ambient-off",
    "patch-disabled-source8-ambient-off",
)


def require(cond, message):
    if not cond:
        raise ValueError(message)


def sha(text):
    return hashlib.sha256(text.encode("utf-8")).hexdigest()


def part(event, name):
    kind = event["kind"]
    return kind.get(name) if isinstance(kind, dict) else None


def actor(event):
    origin = event["origin"]
    if isinstance(origin, dict) and "Internal" in origin:
        require(origin["Internal"] == event["process"], "origin actor != event actor")
        return event["process"]
    return None


def edge(state, pid):
    node = state["processes"][str(pid)]["code"]["nodes"][4]
    require("Const" in node, "native functional code site no longer Const")
    return node["Const"]["next"]


def exact_sign_p(deltas):
    pos = sum(x > 0 for x in deltas)
    neg = sum(x < 0 for x in deltas)
    n = pos + neg
    if n == 0:
        return 1.0
    tail = sum(math.comb(n, k) for k in range(min(pos, neg) + 1))
    return min(1.0, 2 * tail / 2**n)


def audit_events(state, source, patch_disabled, grant):
    writes = set()
    reads = {}
    code_edge = {1: 24, 2: 24}
    birth_edges = {}
    draws = Counter()
    launched = Counter()
    copied = defaultdict(set)
    launches = 0
    mutations = 0
    suppressed = 0
    native_events = []
    for event in state["events"]:
        a = actor(event)
        if a is None:
            require(event["origin"] == "External", "unknown host event provenance")
            continue
        native_events.append(event)
        rd = part(event, "CellRead")
        wr = part(event, "CellWritten")
        edit = part(event, "ExecutableEdgePatched")
        denied = part(event, "NativeCodeEditSuppressed")
        cloned = part(event, "GraphNodeCopied")
        launched_event = part(event, "GraphLaunched")
        if rd is not None and rd["key"] == 7:
            require(event["pc"] == 2 and rd["value"] == source, "native source read mismatch")
            reads[a] = source
        if wr is not None and wr["key"] == 9:
            require(event["pc"] == 3 and wr["value"] == source, "native source writer mismatch")
            require(reads.get(a) == source, "source-written without same-actor read")
            writes.add(a)
        if rd is not None and rd["key"] == 11:
            require(event["pc"] == 6 and rd["value"] in (5, 24), "unknown mutation input")
        if wr is not None and wr["key"] == 11:
            require(event["pc"] == 11 and wr["value"] in (5, 24), "unknown next mutation input")
        if edit is not None:
            require(not patch_disabled, "real code mutation in patch-disabled control")
            require(event["pc"] == 8 and edit["node"] == 4, "wrong mutated code node/PC")
            require(edit["before"] == code_edge[a], "mutation event ancestry before mismatch")
            require({edit["before"], edit["after"]} == {5, 24}, "mutation outside registered paths")
            code_edge[a] = edit["after"]
            mutations += 1
        if denied is not None:
            require(patch_disabled and event["pc"] == 8 and denied["opcode_tag"] == 6,
                    "wrong suppressed instruction provenance")
            suppressed += 1
        if event["pc"] == 21 and event["kind"] == "Executed":
            draws[a] += 1
        if cloned is not None:
            require(cloned["source_process"] == a and event["pc"] == 16,
                    "child executable copied from non-actor")
            copied[a].add(cloned["source_node"])
        if launched_event is not None:
            require(event["pc"] == 22 and launched_event["parent_process"] == a, "bad launch PC")
            child = launched_event["child_process"]
            require(child not in code_edge and child >= 3, "duplicate or host-origin child")
            require(launched_event["moved_nodes"] == 28, "incomplete native code copy")
            require(launched_event["transferred_credits"] == grant, "bad native child resources")
            require(draws[a] > launched[a], "native child without prior successful actor Draw")
            require(copied[a] == set(range(28)), "not all 28 native source nodes copied")
            copied[a] = set()
            launched[a] += 1
            code_edge[child] = code_edge[a]
            birth_edges[child] = code_edge[a]
            launches += 1

    for pid, observed in code_edge.items():
        require(edge(state, pid) == observed, f"actor {pid} final executable allele differs from event ancestry")
    if patch_disabled:
        require(mutations == 0 and all(v == 24 for v in birth_edges.values()),
                "native-only patch ablation changed inherited executable genotype")
    else:
        require(suppressed == 0, "native enabled run has intervention suppression")
    require(writes.issuperset({1, 2}), "original native founders failed source-sensitive function")
    require(len(native_events) == state["_actual_attempts"], "native event count != total charged attempts")

    # Check birth-time code expression, not merely a stored inherited allele.
    first_pc = {}
    for e in native_events:
        key = (e["process"], e["pc"])
        first_pc.setdefault(key, e["tick"])
    expressed = Counter()
    for child, inherited in birth_edges.items():
        start = first_pc.get((child, 4))
        join = first_pc.get((child, 5))
        if start is not None and join is not None:
            require(join > start, "child functional path did not precede mutation route")
            delay = first_pc.get((child, 24))
            if inherited == 24:
                require(delay is not None and start < delay < join,
                        "slow-born child did not execute charged slow path")
                expressed["slow"] += 1
            else:
                require(delay is None or delay > join,
                        "fast-born child executed slow path before first mutation")
                expressed["fast"] += 1
    return {
        "native_births": launches,
        "mutations": mutations,
        "suppressed": suppressed,
        "fast_births": sum(v == 5 for v in birth_edges.values()),
        "slow_births": sum(v == 24 for v in birth_edges.values()),
        "fast_expressed": expressed["fast"],
        "slow_expressed": expressed["slow"],
        "source_writers": len(writes),
    }


def audit(path, expected_revision):
    conn = sqlite3.connect(path)
    records = conn.execute(
        "SELECT id,condition,source_revision,initial_json,initial_sha256,actions_json,attempts_json,"
        "final_json,final_sha256 FROM epoch_runs ORDER BY id"
    ).fetchall()
    require(len(records) == GENOTYPES * len(MODES), "not 256 registered original Study112 factorial worlds")
    rows = {}
    genome_hashes = {}
    snapshots = 0
    for run_id, label, revision, original, initial_sha, actions_json, attempts_json, final_json, final_sha in records:
        require(revision == expected_revision and len(revision) == 40, "source revision mismatch")
        require(label.startswith("seed="), "unexpected original label")
        seed_label, mode = label.removeprefix("seed=").split("/", 1)
        seed = int(seed_label)
        require(FIRST <= seed < FIRST + GENOTYPES and mode in MODES, "unregistered world")
        require((seed, mode) not in rows, "duplicate original world")
        require(sha(original) == initial_sha and sha(final_json) == final_sha,
                "initial/final original full-state SHA mismatch")
        initial = json.loads(original)
        source = 3 if "-source3-" in mode else 8
        disabled = mode.startswith("patch-disabled-")
        require(initial["cells"]["7"] == source and initial["cells"]["11"] == 5,
                "unregistered original environmental inputs")
        require(initial["compute_pool"] == 800, "wrong original initial public compute")
        require(initial["processes"]["1"]["credits"] == initial["processes"]["2"]["credits"] == 500,
                "founder credit mismatch")
        require(len(initial["processes"]) == 2, "researcher injected extra initial actor")
        require(initial["processes"]["1"]["code"] == initial["processes"]["2"]["code"],
                "initial founder genotypes not byte identical")
        require(initial.get("native_patchnext_disabled", False) == disabled,
                "wrong serialized narrow mutation ablation law")
        require(initial["trace_native_executable_edits"] is True, "missing native provenance tracing")
        original_code_hash = sha(json.dumps(initial["processes"]["1"]["code"], sort_keys=True))
        genome_hashes.setdefault(seed, set()).add(original_code_hash)
        source_code = initial["processes"]["1"]["code"]["nodes"]
        require(source_code[4] == {"Const": {"dst": 2, "value": 1, "next": 24}},
                "founder initial functional PC4 must be the prospectively slow path")
        grant = source_code[20]["Const"]["value"]
        delay = source_code[24]["Const"]["value"]
        require(320 <= grant <= 480 and 1 <= delay <= 32, "unregistered genotype parameters")
        attempts = json.loads(attempts_json)
        require(len(attempts) == EPOCHS and all(0 <= v <= QUOTA for v in attempts),
                "invalid native charged epoch attempts")
        total = sum(attempts)
        snapshots_rows = conn.execute(
            "SELECT epoch,state_sha256,state_json FROM epoch_snapshots WHERE run_id=? ORDER BY epoch",
            (run_id,),
        ).fetchall()
        require(len(snapshots_rows) == EPOCHS, "missing original epoch")
        actions = json.loads(actions_json)
        require(len(actions) == EPOCHS - 1,
                "missing or additional researcher-side ambient input epochs")
        for index, action in enumerate(actions, start=1):
            require(action["epoch"] == index and "SetComputePool" in action["action"],
                    "unexpected ambient input epoch, actor rescue or other host intervention")
            before_pool = json.loads(snapshots_rows[index - 1][2])["compute_pool"]
            require(action["action"]["SetComputePool"]["credits"] == before_pool + (400 if mode.endswith("-ambient-on") else 0),
                    "externally supplied public credits not fixed +400 over preceding epoch")
        for i, (epoch, digest, raw) in enumerate(snapshots_rows):
            require(epoch == i and sha(raw) == digest, "invalid original epoch SHA256")
            snapshots += 1
        require(snapshots_rows[-1][2] == final_json, "last epoch differs from final recorded world")
        state = json.loads(final_json)
        inventory = state["compute_pool"] + sum(
            p["credits"] for p in state["processes"].values()
        )
        require(inventory == (7800 if mode.endswith("-ambient-on") else 1800) - total,
                "native pool/actor/15ambient factorial compute conservation failed")
        require(sum(e["origin"] == "External" for e in state["events"]) == 20,
                "unexpected host intervention")
        state["_actual_attempts"] = total
        outcome = audit_events(state, source, disabled, grant)
        per_epoch_native_births = []
        previous_births = 0
        for _, _, snapshot in snapshots_rows:
            world = json.loads(snapshot)
            seen = sum(
                isinstance(e["kind"], dict) and "GraphLaunched" in e["kind"]
                and isinstance(e["origin"], dict)
                for e in world["events"]
            )
            per_epoch_native_births.append(seen - previous_births)
            previous_births = seen
        require(sum(per_epoch_native_births) == outcome["native_births"],
                "original epoch birth counts disagree with full actor genealogy")
        outcome["per_epoch_native_births"] = per_epoch_native_births
        del state["_actual_attempts"]
        rows[(seed, mode)] = {
            "source": source,
            "disabled": disabled,
            "genotype_hash": original_code_hash,
            "grant": grant,
            "slow_delay_count": delay,
            "attempts": total,
            **outcome,
        }
    require(len(rows) == GENOTYPES * len(MODES), "registered world labels not complete")
    require(all(len(h) == 1 for h in genome_hashes.values()), "matched arms have differing initial code")

    paired = {}
    exposures = 0
    for seed in range(FIRST, FIRST + GENOTYPES):
        paired[seed] = {}
        genotype_hash = None
        for source in (3, 8):
            for ambient in ("on", "off"):
                yes = rows[(seed, f"native-edit-source{source}-ambient-{ambient}")]
                no = rows[(seed, f"patch-disabled-source{source}-ambient-{ambient}")]
                require(yes["genotype_hash"] == no["genotype_hash"],
                        "same initial code genotype corrupted within factorial match")
                if genotype_hash is None:
                    genotype_hash = yes["genotype_hash"]
                require(yes["genotype_hash"] == genotype_hash,
                        "factorial world source/genome differs across matching conditions")
                require(no["mutations"] == 0 and no["fast_births"] == 0 and no["suppressed"] > 0,
                        "native mutation-only disabled arm lacks charged non-mutating PatchNext")
                paired[seed][f"source{source}-{ambient}"] = (
                    yes["native_births"] - no["native_births"]
                )
        a3 = rows[(seed, "native-edit-source3-ambient-on")]
        a8 = rows[(seed, "native-edit-source8-ambient-on")]
        if (a3["mutations"] > 0 and a8["mutations"] > 0
                and a3["fast_expressed"] > 0 and a8["fast_expressed"] > 0):
            exposures += 1

    source_stats = {}
    for source in (3, 8):
        on_deltas = [paired[seed][f"source{source}-on"] for seed in range(FIRST, FIRST + GENOTYPES)]
        off_deltas = [paired[seed][f"source{source}-off"] for seed in range(FIRST, FIRST + GENOTYPES)]
        interactions = [a - b for a, b in zip(on_deltas, off_deltas)]
        source_stats[f"source{source}"] = {
            "mutation_effect_ambient_on_by_seed": on_deltas,
            "mutation_effect_ambient_off_by_seed": off_deltas,
            "ambient_by_mutation_interaction_by_seed": interactions,
            "positive": sum(d > 0 for d in interactions),
            "negative": sum(d < 0 for d in interactions),
            "ties": sum(d == 0 for d in interactions),
            "paired_two_sided_interaction_sign_p": exact_sign_p(interactions),
        }
    return {
        "status": "HUMAN_ENGINEERED_CIL_AMBIENT_BY_NATIVE_MUTATION_FACTORIAL_NOT_LIFE",
        "source_revision": expected_revision,
        "seeded_template_genotype_labels": GENOTYPES,
        "distinct_code_genotypes": len(set(next(iter(h)) for h in genome_hashes.values())),
        "native_original_world_runs": len(records),
        "factorial_mode_count": len(MODES),
        "verified_full_epoch_sha256": snapshots,
        "exposure_E_genotypes": exposures,
        "primary_comparisons": source_stats,
        "worlds": [
            {"seed": seed, "arm": mode, **rows[(seed, mode)]}
            for seed in range(FIRST, FIRST + GENOTYPES) for mode in MODES
        ],
        "cyberlife_proven": False,
    }


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("sqlite", type=Path)
    parser.add_argument("--expected-revision", required=True)
    parser.add_argument("--json", type=Path)
    args = parser.parse_args()
    result = audit(args.sqlite, args.expected_revision)
    args.json.write_text(json.dumps(result, sort_keys=True, indent=2) + "\n", encoding="utf-8")
    print(json.dumps({
        key: result[key] for key in (
            "status", "native_original_world_runs", "verified_full_epoch_sha256",
            "exposure_E_genotypes", "distinct_code_genotypes", "primary_comparisons"
        )
    }, sort_keys=True))


if __name__ == "__main__":
    main()
