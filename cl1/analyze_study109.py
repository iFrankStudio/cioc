#!/usr/bin/env python3
"""Study109: POST-OUTCOME, original Study108 event-level mutation/offspring ancestry.

No simulated worlds, no seed fitting. Original SQLite only, provenance audited first
by the already source-frozen Study108 auditor. Descriptive original lineage audit.
"""
import argparse
import json
import sqlite3
from collections import Counter
from pathlib import Path

from analyze_study108 import audit as original_study108_audit

ORIGINAL_REVISION = "406bb3e1160987113671eb9ffdf517c3edf4ad51"
SOURCES = (3, 8)
SEEDS = range(6200000, 6200032)


def require(ok, explanation):
    if not ok:
        raise ValueError(explanation)


def payload(event, name):
    kind = event["kind"]
    return kind.get(name) if isinstance(kind, dict) else None


def real_ancestry(state):
    """Reconstruct executable edge at EACH birth from preceding native PC8 events."""
    edge = {1: 24, 2: 24}
    first_faster_event = None
    mutations = []
    births = []
    first_route = {}
    last_pc4 = {}
    first_actor_read = set()
    first_actor_write = set()
    all_actor_pc = {}
    native_attempts = 0
    for event in state["events"]:
        origin = event["origin"]
        if not isinstance(origin, dict) or "Internal" not in origin:
            require(origin == "External", "unregistered event origin")
            continue
        actor = event["process"]
        require(origin == {"Internal": actor}, "native event process mismatch")
        native_attempts += 1
        pc = event["pc"]
        tick = event["tick"]
        if actor in last_pc4 and actor not in first_route and pc in (5, 24):
            first_route[actor] = {"next_pc": pc, "tick": tick}
        if pc == 4:
            last_pc4[actor] = tick
        all_actor_pc.setdefault(actor, Counter())[pc] += 1
        read = payload(event, "CellRead")
        if read is not None and read["key"] == 7:
            first_actor_read.add(actor)
        written = payload(event, "CellWritten")
        if written is not None and written["key"] == 9:
            require(actor in first_actor_read, "child native source write missing earlier actor read")
            first_actor_write.add(actor)
        edit = payload(event, "ExecutableEdgePatched")
        if edit is not None:
            require(pc == 8 and edit["node"] == 4, "different source or native code site")
            require(actor in edge and edit["before"] == edge[actor], "invalid edited code ancestry")
            require({edit["before"], edit["after"]} == {5, 24}, "unregistered mutation allele")
            edge[actor] = edit["after"]
            mutations.append({
                "tick": tick, "actor": actor, "before": edit["before"], "after": edit["after"],
            })
            if first_faster_event is None and edit["before"] == 24 and edit["after"] == 5:
                first_faster_event = {"tick": tick, "actor": actor}
        launch = payload(event, "GraphLaunched")
        if launch is not None:
            require(actor == launch["parent_process"] and pc == 22, "false native birth")
            child = launch["child_process"]
            require(child not in edge and child >= 3, "duplicate or externally created child")
            edge[child] = edge[actor]
            births.append({
                "tick": tick, "parent": actor, "child": child,
                "inherited_edge": edge[child],
            })
    for actor, expected in edge.items():
        process = state["processes"][str(actor)]
        require(process["code"]["nodes"][4]["Const"]["next"] == expected,
                "final child code not predicted by original event mutation ancestry")
    for b in births:
        child = b["child"]
        route = first_route.get(child)
        if route is not None:
            # The child may later self-mutate; its first functional route, reached
            # before its first PC8, must be the edge actually copied at birth.
            require(route["next_pc"] == b["inherited_edge"],
                    "child's first paid executable path not identical to inherited allele")
        b["first_expressed_route"] = route["next_pc"] if route else None
        b["first_expressed_tick"] = route["tick"] if route else None

    return {
        "native_attempts": native_attempts,
        "first_fast_mutation": first_faster_event,
        "mutations": mutations,
        "native_births": births,
        "fast_births": sum(b["inherited_edge"] == 5 for b in births),
        "slow_births": sum(b["inherited_edge"] == 24 for b in births),
        "fast_expressed_children": sum(b["first_expressed_route"] == 5 for b in births),
        "slow_expressed_children": sum(b["first_expressed_route"] == 24 for b in births),
        "functional_writers": len(first_actor_write),
        "last_native_birth_tick": births[-1]["tick"] if births else None,
        "remaining_public_pool": state["compute_pool"],
        "residual_process_credits": sum(p["credits"] for p in state["processes"].values()),
        "native_attempts_pc4": sum(c[4] for c in all_actor_pc.values()),
        "native_attempts_slow_delay": sum(sum(c[k] for k in range(24, 28))
                                          for c in all_actor_pc.values()),
    }


def audit(original_sqlite):
    verified = original_study108_audit(original_sqlite, ORIGINAL_REVISION)
    require(verified["native_original_world_runs"] == 128
            and verified["verified_full_epoch_sha256"] == 1024,
            "Study108 exact original corpus audit failed")
    conn = sqlite3.connect(original_sqlite)
    data = {}
    for label, state_json in conn.execute(
        "SELECT condition,final_json FROM epoch_runs ORDER BY id"
    ):
        seed_s, arm = label.removeprefix("seed=").split("/", 1)
        seed = int(seed_s)
        require(seed in SEEDS, "unregistered initial world")
        assert arm.startswith(("native-edit-", "patch-disabled-"))
        require((seed, arm) not in data, "duplicate Study108 original world")
        data[(seed, arm)] = real_ancestry(json.loads(state_json))

    comparisons = []
    winners = []
    for source in SOURCES:
        for seed in SEEDS:
            yes = data[(seed, f"native-edit-source{source}")]
            no = data[(seed, f"patch-disabled-source{source}")]
            require(yes["native_attempts"] == no["native_attempts"] == 2000,
                    "unexpected original fixed-horizon attempt imbalance")
            require(not no["mutations"] and no["fast_births"] == 0,
                    "false mutation in charged ablation world")
            enabled_births = len(yes["native_births"])
            disabled_births = len(no["native_births"])
            difference = enabled_births - disabled_births
            require(difference == verified["primary_comparisons"][f"source{source}"]
                    ["mutation_enabled_minus_disabled_births_by_seed"][seed - 6200000],
                    "retrospective native genealogy disagrees with frozen pre-result auditor")
            row = {
                "seed": seed, "source": source, "birth_delta": difference,
                "enabled_births": enabled_births, "disabled_births": disabled_births,
                "fast_births_enabled": yes["fast_births"],
                "slow_births_enabled": yes["slow_births"],
                "fast_expressed_children_enabled": yes["fast_expressed_children"],
                "first_fast_mutation": yes["first_fast_mutation"],
                "last_birth_tick_enabled": yes["last_native_birth_tick"],
                "last_birth_tick_disabled": no["last_native_birth_tick"],
                "remaining_public_pool_enabled": yes["remaining_public_pool"],
                "remaining_public_pool_disabled": no["remaining_public_pool"],
                "slow_delay_attempts_enabled": yes["native_attempts_slow_delay"],
                "slow_delay_attempts_disabled": no["native_attempts_slow_delay"],
                "native_mutation_count_enabled": len(yes["mutations"]),
                "native_source_writers_enabled": yes["functional_writers"],
                "native_source_writers_disabled": no["functional_writers"],
            }
            comparisons.append(row)
            if difference > 0:
                winners.append({
                    **row,
                    "complete_enabled_birth_chronology": yes["native_births"],
                    "complete_disabled_birth_chronology": no["native_births"],
                    "native_mutation_chronology": yes["mutations"],
                })

    require(len(comparisons) == 64, "missing all original paired source contrasts")
    require(len(winners) == 2 and {(r["seed"], r["source"]) for r in winners}
            == {(6200017, 3), (6200016, 8)},
            "retrospective output differs from already known positive cases")
    return {
        "study": 109,
        "status": "POST_OUTCOME_RETROSPECTIVE_ORIGINAL_NATIVE_LINEAGE_ONLY",
        "original_study108_revision": ORIGINAL_REVISION,
        "original_independent_seed_labels": 32,
        "new_ecological_samples": 0,
        "original_verified_sha256_epochs": verified["verified_full_epoch_sha256"],
        "original_paired_source_conditions": 64,
        "all_64_original_paired_genotype_outcomes": comparisons,
        "two_previously_known_positive_cases": winners,
        "native_adaptation_or_life_proven": False,
    }


def main():
    p = argparse.ArgumentParser()
    p.add_argument("sqlite", type=Path)
    p.add_argument("--json", type=Path, required=True)
    args = p.parse_args()
    result = audit(args.sqlite)
    args.json.write_text(json.dumps(result, indent=2, sort_keys=True) + "\n")
    print(json.dumps({
        "status": result["status"],
        "original_paired_source_conditions": 64,
        "original_verified_sha256_epochs": result["original_verified_sha256_epochs"],
        "previously_known_positive_cases": [
            {k: v for k, v in row.items()
             if k in ("seed", "source", "birth_delta", "fast_births_enabled",
                      "fast_expressed_children_enabled", "slow_delay_attempts_enabled",
                      "slow_delay_attempts_disabled", "remaining_public_pool_enabled",
                      "remaining_public_pool_disabled", "last_birth_tick_enabled",
                      "last_birth_tick_disabled")}
            for row in result["two_previously_known_positive_cases"]
        ],
    }, sort_keys=True))


if __name__ == "__main__":
    main()
