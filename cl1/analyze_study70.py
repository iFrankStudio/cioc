#!/usr/bin/env python3
"""Study70: retrospective source-pinned stochastic CIL genotype independence audit.

ZERO new native worlds. Genotype uniqueness does NOT imply statistical
independence of adjacent/offset seeds or open-ended digital life.
"""
from __future__ import annotations

import argparse
import hashlib
import json
import sqlite3
import statistics
from collections import Counter, defaultdict
from pathlib import Path

import analyze_study68
import analyze_study69

REV = {
    "68": "49a74113de513b32984f3656733087e9ae3a8b74",
    "69": "f06e71294c055e4ebff6bf543a5f0f324e3718a6",
}
FIRST = {"68": 4_500_000, "69": 4_600_000}
N = 256
TOPOLOGIES = ("short", "broad")
MODES = (
    "short-low", "short-high", "short-ablated-low", "short-ablated-high",
    "broad-low", "broad-high", "broad-ablated-low", "broad-ablated-high",
)


def genome_hash(nodes: list) -> str:
    """Canonical exact CIL instructions, ignoring seed label and external sensor."""
    body = json.dumps(nodes, sort_keys=True, separators=(",", ":"), ensure_ascii=True)
    return hashlib.sha256(body.encode("utf-8")).hexdigest()


def op_name(node) -> str:
    return next(iter(node)) if isinstance(node, dict) else node


def program_distance(a: list, b: list) -> dict[str, int]:
    if len(a) != 28 or len(b) != 28 or a[27] != "Halt" or b[27] != "Halt":
        raise ValueError("nonregistered native CIL genome length or tail")
    same_opcode = sum(op_name(a[pc]) == op_name(b[pc]) for pc in range(3, 27))
    same_full = sum(a[pc] == b[pc] for pc in range(3, 27))
    same_branch_target = sum(
        a[pc]["Branch"]["nonzero"] == b[pc]["Branch"]["nonzero"]
        for pc in range(3, 27)
        if op_name(a[pc]) == op_name(b[pc]) == "Branch"
    )
    both_branches = sum(
        op_name(a[pc]) == op_name(b[pc]) == "Branch"
        for pc in range(3, 27)
    )
    return {
        "identical_instructions_of24": same_full,
        "same_opcode_tags_of24": same_opcode,
        "same_native_branch_targets": same_branch_target,
        "both_branch_opcodes": both_branches,
    }


def dist_summary(values: list[int]) -> dict:
    if len(values) != N and len(values) != N-1 and len(values) != N-2:
        raise ValueError("original deterministic similarity pairing incomplete")
    return {
        "pairs": len(values),
        "minimum": min(values),
        "median": statistics.median(values),
        "mean": round(statistics.mean(values), 6),
        "maximum": max(values),
        "histogram": {str(k): v for k, v in sorted(Counter(values).items())},
    }


def verify_topology_pair(short: list, broad: list) -> None:
    if (len(short) != 28 or len(broad) != 28
            or short[-1] != "Halt" or broad[-1] != "Halt"):
        raise ValueError("registered CIL code size/tail mismatch")
    if short[:3] != broad[:3]:
        raise ValueError("researcher I/O interface differs outside registered topology")
    for pc in range(3, 27):
        a, b = short[pc], broad[pc]
        if op_name(a) != op_name(b):
            raise ValueError("program opcode was resampled across topology arms")
        if op_name(a) != "Branch":
            if a != b:
                raise ValueError("random source/value opcode differs across paired topologies")
        else:
            x, y = a["Branch"], b["Branch"]
            if (x["test"] != y["test"] or x["zero"] != y["zero"]
                    or x["nonzero"] != (3 if pc == 3 else pc-1)
                    or not (3 <= y["nonzero"] < pc if pc > 3 else y["nonzero"] == 3)):
                raise ValueError("random broad Branch changed more than its nonzero target")


def original_programs(path: Path, cohort: str) -> dict[str, dict[int, list]]:
    """Independent full archived population and exact original source program checks."""
    db = sqlite3.connect(f"{path.resolve().as_uri()}?mode=ro", uri=True)
    try:
        seeds: dict[int, dict[str,list]] = defaultdict(dict)
        count = 0
        for condition, revision, raw in db.execute(
            "SELECT condition,source_revision,initial_json FROM epoch_runs ORDER BY id"
        ):
            s, mode = condition.split("/", 1)
            seed = int(s.removeprefix("seed="))
            if (revision != REV[cohort] or not FIRST[cohort] <= seed < FIRST[cohort]+N
                    or mode not in MODES or mode in seeds[seed]):
                raise ValueError("duplicate, missing, wrong-source or unregistered historical run")
            initial = json.loads(raw)
            if set(initial["processes"]) != {"1"}:
                raise ValueError("more than the frozen one native actor")
            nodes = initial["processes"]["1"]["code"]["nodes"]
            expected = (
                analyze_study68.expected_code(seed, mode)
                if cohort == "68" else analyze_study69.expected_code(seed, mode)
            )
            if nodes != expected:
                raise ValueError("historical CIL initial code not independently reconstructed")
            seeds[seed][mode] = nodes
            count += 1
        if (count != N * len(MODES) or len(seeds) != N
                or any(set(v) != set(MODES) for v in seeds.values())):
            raise ValueError("source-pinned original eight-arm genome cohort incomplete")
        result = {top: {} for top in TOPOLOGIES}
        for seed in range(FIRST[cohort], FIRST[cohort] + N):
            arms = seeds[seed]
            for top in TOPOLOGIES:
                low = arms[f"{top}-low"]
                if arms[f"{top}-high"] != low:
                    raise ValueError("paired high/low code differs beyond sensor data")
                for suffix in ("low", "high"):
                    ablated = arms[f"{top}-ablated-{suffix}"]
                    for a, b in zip(low, ablated):
                        if op_name(a) == "Read":
                            if b != {"Const": {"dst":0, "value":0, "next":a["Read"]["next"]}}:
                                raise ValueError("Read-negative is not exact CIL ablation")
                        elif a != b:
                            raise ValueError("ablation modified unrelated native opcode")
                result[top][seed] = low
            verify_topology_pair(result["short"][seed], result["broad"][seed])
        return result
    finally:
        db.close()


def uniqueness(programs: dict[int, list]) -> dict:
    by_fingerprint: dict[str,list[int]] = defaultdict(list)
    for seed, code in programs.items():
        by_fingerprint[genome_hash(code)].append(seed)
    duplicates = [seeds for seeds in by_fingerprint.values() if len(seeds) > 1]
    return {
        "labelled_genomes": len(programs),
        "unique_exact_instruction_genotypes": len(by_fingerprint),
        "duplicate_labels": sum(len(x)-1 for x in duplicates),
        "largest_exact_collision_class": max(map(len, by_fingerprint.values())),
        "first_12_exact_collision_classes": duplicates[:12],
    }


def comparison(a: dict[int, list], b: dict[int, list]) -> dict:
    values = {"matched_offset": defaultdict(list),
              "permuted_fixed_offset": defaultdict(list)}
    first_matches = []
    same_exact = 0
    for i in range(N):
        x = a[FIRST["68"] + i]
        y = b[FIRST["69"] + i]
        z = b[FIRST["69"] + ((73*i + 19) % N)]
        for name, v in (("matched_offset", y), ("permuted_fixed_offset", z)):
            distances = program_distance(x, v)
            for k, count in distances.items():
                values[name][k].append(count)
        if genome_hash(x) == genome_hash(y):
            same_exact += 1
            first_matches.append(i)
    return {
        "same_seed_offset_exact_genotype_collisions": same_exact,
        "first_12_same_offset_collision_indices": first_matches[:12],
        "fixed_permutation": "j=(73*i+19)%256 (NOT a statistical independence test)",
        **{k: {metric: dist_summary(v) for metric, v in d.items()}
           for k, d in values.items()},
    }


def adjacent(programs: dict[int, list], first: int) -> dict:
    result = {}
    for offset in (1, 2):
        d = defaultdict(list)
        for i in range(N-offset):
            x = programs[first+i]
            y = programs[first+i+offset]
            for name, value in program_distance(x, y).items():
                d[name].append(value)
        result[f"adjacent_seed_step{offset}"] = {
            k: dist_summary(v) for k,v in d.items()
        }
    return result


def audit(source68: Path, source69: Path) -> dict:
    original68 = analyze_study68.audit(source68, REV["68"])
    original69 = analyze_study69.audit(source69, REV["69"])
    for name, original, expected in (("68", original68, 79), ("69", original69, 81)):
        primary = original["registered_primary_recurrent_source_sensitive_W2_and_W3"]
        if (original["original_independent_genomes"] != N
                or original["matched_native_runs"] != N * len(MODES)
                or original["SHA256_verified_epoch_snapshots"] != N * len(MODES)*24
                or primary["broad_worlds"] != expected
                or primary["short_worlds"] != 0):
            raise ValueError("original frozen primary feedback result did not independently replay")
    programs = {"68":original_programs(source68,"68"),
                "69":original_programs(source69,"69")}
    result = {
        "study": "CL-1 Study70 retrospective CIL genotype uniqueness and seed dependence",
        "registered_AFTER_results": True,
        "new_independent_worlds": 0,
        "original_seed_labels_reused": 512,
        "original_matched_runs_rechecked": 4096,
        "original_SHA256_epoch_snapshots_reverified": 98304,
        "original_registered_primary_by_cohort": {
            "68": original68["registered_primary_recurrent_source_sensitive_W2_and_W3"],
            "69": original69["registered_primary_recurrent_source_sensitive_W2_and_W3"],
        },
        "source_revisions": REV,
        "unique_genotypes": {},
        "cross_cohort_exact_overlap": {},
        "cohort_offset_dependence": {},
        "near_seed_pattern_diagnostics": {},
        "life_claim": "NONE",
        "independence_caveat": "Distinct seeds and unique full CIL genomes do not establish independent random samples; LCG modulo6 can correlate instruction patterns",
    }
    for top in TOPOLOGIES:
        a, b = programs["68"][top], programs["69"][top]
        hashes_a = {genome_hash(x) for x in a.values()}
        hashes_b = {genome_hash(x) for x in b.values()}
        result["unique_genotypes"][top] = {
            "68": uniqueness(a), "69": uniqueness(b),
        }
        result["cross_cohort_exact_overlap"][top] = {
            "genotypes_shared_by_both_cohorts": len(hashes_a & hashes_b),
            "distinct_full_genotypes_among_512_labels": len(hashes_a | hashes_b),
        }
        result["cohort_offset_dependence"][top] = comparison(a, b)
        result["near_seed_pattern_diagnostics"][top] = {
            "68": adjacent(a, FIRST["68"]),
            "69": adjacent(b, FIRST["69"]),
        }
    return result


def main() -> None:
    p = argparse.ArgumentParser(description=__doc__)
    p.add_argument("study68", type=Path)
    p.add_argument("study69", type=Path)
    p.add_argument("--json", type=Path, required=True)
    args = p.parse_args()
    report = audit(args.study68, args.study69)
    args.json.write_text(json.dumps(report, indent=2) + "\n", encoding="utf-8")
    print(json.dumps({
        "study":report["study"],
        "new_worlds":0,
        "unique_genotypes":report["unique_genotypes"],
        "overlap":report["cross_cohort_exact_overlap"],
        "same_offset_comparison": {
            top:{
                "matched_full": report["cohort_offset_dependence"][top][
                    "matched_offset"]["identical_instructions_of24"]["mean"],
                "permuted_full": report["cohort_offset_dependence"][top][
                    "permuted_fixed_offset"]["identical_instructions_of24"]["mean"],
            } for top in TOPOLOGIES
        },
        "life_claim":"NONE",
    },indent=2))


if __name__ == "__main__":
    main()
