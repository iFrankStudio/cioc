#!/usr/bin/env python3
"""Study57: source-pinned post-result native changed-Branch exposure diagnostic.

Zero new independent seed pairs. A conservative original-code exposure requires
a successfully executed initial changed-Branch PC with no earlier original-actor
native executable mutator. Uncertain means NOT EVALUABLE, not absence.
"""
from __future__ import annotations

import argparse
import json
import sqlite3
from collections import Counter, defaultdict
from pathlib import Path

import analyze
import analyze_study55
import analyze_study56
from analyze_study50 import trace_assembly

COHORTS = (
    ("55", 3_800_000, "e6ed35766de551a1528279634008288e3d30b18b",
     analyze_study55.audit, (0, 2)),
    ("56", 3_900_000, "8983cd1e402ab181398ba3d13a58065e8d7a426e",
     analyze_study56.audit, (3, 3)),
)
ARMS = ("ambient-general", "ambient-only", "closed-general",
        "zero-credit-ambient-general")
MUTATORS = {"PatchNext", "PatchConst", "CloneNode"}


def op_name(node) -> str:
    if node == "Halt":
        return "Halt"
    if isinstance(node, dict) and len(node) == 1:
        return next(iter(node))
    raise ValueError("invalid frozen native executable opcode")


def changed_branch_sites(local: dict, uniform: dict) -> dict[int, set[int]]:
    """Verify declared initial Branch.nonzero-only paired graph difference."""
    sites = {}
    for actor in (1, 2, 3):
        a = local["processes"][str(actor)]["code"]["nodes"]
        b = uniform["processes"][str(actor)]["code"]["nodes"]
        if len(a) != len(b) or len(a) != 82:
            raise ValueError("incorrect frozen native original program length")
        changed = set()
        for pc, (left, right) in enumerate(zip(a, b)):
            if left == right:
                continue
            if (not 9 <= pc < 81 or op_name(left) != "Branch"
                    or op_name(right) != "Branch"):
                raise ValueError("unregistered non-Branch paired code difference")
            la, ub = left["Branch"], right["Branch"]
            if (la["test"] != ub["test"] or la["zero"] != ub["zero"]
                    or la["nonzero"] != pc - 2 or not 0 <= ub["nonzero"] < pc):
                raise ValueError("invalid paired topology treatment")
            changed.add(pc)
        sites[actor] = changed
    return sites


def classify_native_branch_exposure(initial: dict, events: list[dict],
                                   sites: dict[int, set[int]]) -> list[dict]:
    """Only successful, actor-authenticated events; classify prior mutation."""
    tainted = set()
    exposures = []
    prior_tick = initial["events"][-1]["tick"] if initial["events"] else -1
    for event in events:
        tick = event["tick"]
        if tick <= prior_tick:
            raise ValueError("nonmonotone native event tick")
        prior_tick = tick
        actor = event["process"]
        if actor not in (1, 2, 3):
            continue
        internal = analyze.internal_process(event) == actor
        if not internal:
            continue
        pc = event["pc"]
        original = initial["processes"][str(actor)]["code"]["nodes"]
        if not isinstance(pc, int) or not 0 <= pc < len(original):
            raise ValueError("invalid original actor PC")
        if event["kind"] != "Executed":
            continue
        name = op_name(original[pc])
        if pc in sites[actor]:
            exposures.append({
                "tick": tick, "actor": actor, "pc": pc,
                "unambiguous_initial_branch": actor not in tainted
                and name == "Branch",
            })
        if name in MUTATORS:
            tainted.add(actor)
    return exposures


def audit_one(path: Path, first: int, revision: str,
              site_by_seed: dict[int, dict[int, set[int]]],
              topology: str) -> dict:
    db = sqlite3.connect(f"{path.resolve().as_uri()}?mode=ro", uri=True)
    try:
        counts = Counter()
        seen = defaultdict(set)
        evidence = []
        total = 0
        for condition, source, raw_initial, raw_final in db.execute(
            "SELECT condition,source_revision,initial_json,final_json "
            "FROM epoch_runs ORDER BY id"
        ):
            label, arm = condition.split("/", 1)
            seed = int(label.removeprefix("seed="))
            if (source != revision or seed not in range(first, first + 256)
                    or arm not in ARMS or arm in seen[seed]):
                raise ValueError("wrong historical source, seed, or duplicated arm")
            seen[seed].add(arm)
            initial, final = json.loads(raw_initial), json.loads(raw_final)
            original_events = initial["events"]
            events = final["events"]
            if events[:len(original_events)] != original_events:
                raise ValueError("original event prefix changed")
            suffix = events[len(original_events):]
            sites = site_by_seed[seed]
            exposures = classify_native_branch_exposure(initial, suffix, sites)
            launches = trace_assembly(suffix, final)
            strict = [x for x in launches if x["copy_before_edit_before_launch"]]
            first_launch_tick = min((x["tick"] for x in launches), default=None)
            first_strict_tick = min((x["tick"] for x in strict), default=None)
            earliest_strict = min(strict, key=lambda x: x["tick"]) if strict else None
            earlier_parent_exposure = (
                [x for x in exposures
                 if x["unambiguous_initial_branch"]
                 and x["actor"] == earliest_strict["parent"]
                 and x["tick"] < first_strict_tick]
                if earliest_strict else []
            )
            for name, yes in {
                "worlds_with_changed_branch_site": any(sites.values()),
                "worlds_with_candidate_branch_exposure": bool(exposures),
                "worlds_with_confirmed_branch_exposure": any(
                    e["unambiguous_initial_branch"] for e in exposures),
                "worlds_with_mutation_uncertain_branch_exposure": any(
                    not e["unambiguous_initial_branch"] for e in exposures),
                "worlds_with_exposure_before_first_launch": any(
                    x["unambiguous_initial_branch"] and
                    first_launch_tick is not None and x["tick"] < first_launch_tick
                    for x in exposures
                ),
                "worlds_with_native_child_launch": bool(launches),
                "worlds_with_strict_native_assembly": bool(strict),
                "worlds_with_strict_assembly_and_confirmed_earlier_parent_branch":
                    bool(earlier_parent_exposure),
                "worlds_with_strict_assembly_without_confirmed_earlier_parent_branch":
                    bool(strict) and not earlier_parent_exposure,
            }.items():
                counts[f"{arm}.{name}"] += int(yes)
            counts[f"{arm}.candidate_exposure_events"] += len(exposures)
            counts[f"{arm}.confirmed_initial_branch_exposure_events"] += sum(
                e["unambiguous_initial_branch"] for e in exposures
            )
            counts[f"{arm}.mutation_uncertain_exposure_events"] += sum(
                not e["unambiguous_initial_branch"] for e in exposures
            )
            if strict and len(evidence) < 24:
                evidence.append({
                    "seed": seed, "topology": topology, "arm": arm,
                    "first_strict_launch": {
                        "tick": first_strict_tick,
                        "parent": earliest_strict["parent"],
                        "child": earliest_strict["child"],
                        "source_graph": earliest_strict["source_graph"],
                    },
                    "parent_confirmed_early_branch_exposures": earlier_parent_exposure[:5],
                    "candidate_native_branch_exposures": exposures[:12],
                })
            total += 1
        if (total != 1024 or len(seen) != 256
                or any(arms != set(ARMS) for arms in seen.values())):
            raise ValueError("truncated original four-arm matched cohort")
        return {
            "source_revision": revision,
            "topology": topology,
            "original_pairs_reused": 256,
            "runs": total,
            "stage_counts_by_arm": dict(sorted(counts.items())),
            "first_strict_graph_path_exposure_witnesses": evidence,
        }
    finally:
        db.close()


def audit_cohort(local_path: Path, uniform_path: Path, study: str, first: int,
                 revision: str, original_auditor, known: tuple[int, int]) -> dict:
    original = original_auditor(local_path, uniform_path, revision)
    if (original["independent_seed_pairs"] != 256
            or original["matched_native_runs"] != 2048
            or original["SHA256_verified_original_epochs"] != 49152
            or original["primary_A_spontaneous_strict_graph_assembly"]["local_worlds"] != known[0]
            or original["primary_A_spontaneous_strict_graph_assembly"]["uniform_worlds"] != known[1]):
        raise ValueError("frozen original Study55/56 full auditor changed")
    code_by_seed = defaultdict(dict)
    for mode, path in (("local", local_path), ("uniform", uniform_path)):
        db = sqlite3.connect(f"{path.resolve().as_uri()}?mode=ro", uri=True)
        try:
            for label, raw in db.execute(
                "SELECT condition,initial_json FROM epoch_runs "
                "WHERE condition LIKE '%/ambient-general' ORDER BY id"
            ):
                seed = int(label.split("/", 1)[0].removeprefix("seed="))
                if seed in code_by_seed and mode in code_by_seed[seed]:
                    raise ValueError("duplicate original topology seed")
                code_by_seed[seed][mode] = json.loads(raw)
        finally:
            db.close()
    if len(code_by_seed) != 256:
        raise ValueError("missing original paired source programs")
    changed = {}
    for seed in range(first, first + 256):
        if set(code_by_seed[seed]) != {"local", "uniform"}:
            raise ValueError("missing original code topology mode")
        changed[seed] = changed_branch_sites(
            code_by_seed[seed]["local"], code_by_seed[seed]["uniform"]
        )
    modes = {
        mode: audit_one(path, first, revision, changed, mode)
        for mode, path in (("local", local_path), ("uniform", uniform_path))
    }
    for mode, expected in (("local", known[0]), ("uniform", known[1])):
        if (modes[mode]["stage_counts_by_arm"].get(
            "ambient-general.worlds_with_strict_native_assembly", 0) != expected):
            raise ValueError("native assembly output differs from registered result")
    return {
        "study": study,
        "source_revision": revision,
        "original_independent_seed_pairs_reused": 256,
        "original_run_count_reverified": 2048,
        "original_sha256_epochs_verified_by_original_auditor": 49152,
        "modes": modes,
        "life_claim": "NONE",
    }


def main() -> None:
    p = argparse.ArgumentParser(description=__doc__)
    for study in ("55", "56"):
        for mode in ("local", "uniform"):
            p.add_argument(f"study{study}_{mode}", type=Path)
    p.add_argument("--json", type=Path, required=True)
    args = p.parse_args()
    results = {}
    for study, first, revision, original_auditor, known in COHORTS:
        results[study] = audit_cohort(
            getattr(args, f"study{study}_local"),
            getattr(args, f"study{study}_uniform"),
            study, first, revision, original_auditor, known,
        )
    report = {
        "study": "CL-1 Study57 retrospective actual native changed-Branch exposure",
        "new_independent_seed_pairs": 0,
        "original_independent_seed_pairs_reused": 512,
        "historical_native_runs": 4096,
        "original_SHA256_snapshots_reverified": 98304,
        "cohorts": results,
        "life_claim": "NONE",
    }
    args.json.write_text(json.dumps(report, indent=2) + "\n", encoding="utf-8")
    print(json.dumps({
        "study": report["study"],
        "new_seed_pairs": 0,
        "study55": {
            m: {k:v for k,v in x["stage_counts_by_arm"].items()
                if k.startswith("ambient-general.")}
            for m,x in results["55"]["modes"].items()
        },
        "study56": {
            m: {k:v for k,v in x["stage_counts_by_arm"].items()
                if k.startswith("ambient-general.")}
            for m,x in results["56"]["modes"].items()
        },
        "life_claim": "NONE",
    }, indent=2))


if __name__ == "__main__":
    main()
