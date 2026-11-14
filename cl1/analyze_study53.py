#!/usr/bin/env python3
"""Study53 preregistered native graph register-alias causal paired comparison.

The only prospective program difference is source/result register topology.
External compute and neutral starter are NOT endogenous maintenance or life.
"""
from __future__ import annotations

import argparse
import json
import sqlite3
from collections import Counter, defaultdict
from copy import deepcopy
from pathlib import Path

import analyze
import analyze_study53_alias
import analyze_study53_separate
from analyze_study02 import wilson_95
from analyze_study50 import trace_assembly
from analyze_study52 import runtime_source_uncertainty

FIRST = 3_700_000
SEEDS = 128
ARMS = ("ambient-general", "ambient-only", "closed-general",
        "zero-credit-ambient-general")


def normalize_program(nodes: list) -> list:
    """Convert the *declared treatment* register topology to alias baseline."""
    result = deepcopy(nodes)
    for index in range(9, 81):
        op = result[index]
        if not isinstance(op, dict):
            continue
        if "GraphCopyNode" in op:
            c = op["GraphCopyNode"]
            if (c["graph"], c["source"], c["dst"]) != (3, 2, 0):
                raise ValueError("unknown treatment GraphCopyNode operand change")
            c["dst"] = 2
        if "GraphSetNext" in op:
            e = op["GraphSetNext"]
            if (e["graph"], e["node"], e["target"]) != (3, 0, 0):
                raise ValueError("unknown treatment GraphSetNext operand change")
            e["node"] = 2
    return result


def ancestry(initial: dict, final: dict) -> dict:
    """Strict native CIL graph path and initial Write source ancestry."""
    events = final["events"][len(initial["events"]):]
    launched = trace_assembly(events, final)
    complete = [a for a in launched if a["copy_before_edit_before_launch"]]
    copy_write = False
    potentially_mutated = False
    examples = []
    for path in complete:
        parent = path["parent"]
        program = initial["processes"][str(parent)]["code"]["nodes"]
        for copy in path["copies_nonzero"]:
            if not any(copy["tick"] < edit["tick"] < path["tick"]
                       for edit in path["edits"]):
                continue
            source = copy["source_node"]
            if not isinstance(source, int) or not 0 <= source < len(program):
                raise ValueError("native source instruction index invalid")
            op = program[source]
            name = next(iter(op)) if isinstance(op, dict) else op
            if name == "Write":
                copy_write = True
                if len(examples) < 3:
                    examples.append({
                        "parent": parent, "child": path["child"],
                        "graph": path["source_graph"],
                        "source_index": source, "copy_tick": copy["tick"],
                        "launch_tick": path["tick"],
                        "initial_source_opcode": name,
                    })
        # Exact runtime opcode attribution remains conservative where later
        # native mutators have changed code before the copy; not a life witness.
    child_write = any(a["child_successful_CellWritten"] for a in launched)
    return {
        "strict_path": bool(complete),
        "initial_Write_source_strict_path": copy_write,
        "native_functional_child_write": child_write,
        "native_launch": bool(launched),
        "child_Write_in_code": any(a["has_Write"] for a in launched),
        "child_non_GraphNew": any(a["has_non_GraphNew_opcode"] for a in launched),
        "examples": examples,
    }


def audit_mode(path: Path, mode: str, revision: str) -> dict:
    original = (
        analyze_study53_alias.audit(path, revision)
        if mode == "alias"
        else analyze_study53_separate.audit(path, revision)
    )
    if (original["independent_worlds"] != SEEDS
            or original["matched_runs"] != SEEDS * len(ARMS)
            or original["hashed_epoch_snapshots"] != SEEDS * len(ARMS) * 24):
        raise ValueError("invalid Study53 original independent auditor summary")
    db = sqlite3.connect(f"{path.resolve().as_uri()}?mode=ro", uri=True)
    try:
        outcomes = defaultdict(dict)
        samples = {}
        counts = Counter()
        examples = []
        total = 0
        for label, source, raw_initial, raw_final in db.execute(
            "SELECT condition,source_revision,initial_json,final_json "
            "FROM epoch_runs ORDER BY id"
        ):
            seed_txt, arm = label.split("/", 1)
            seed = int(seed_txt.removeprefix("seed="))
            if (not FIRST <= seed < FIRST + SEEDS or arm not in ARMS
                    or arm in outcomes[seed] or source != revision):
                raise ValueError("wrong seed, duplicate arm, or mixed revision")
            initial, final = json.loads(raw_initial), json.loads(raw_final)
            if seed not in samples:
                samples[seed] = initial
            elif samples[seed] != initial:
                raise ValueError("unmatched initial world within topology regime")
            val = ancestry(initial, final)
            outcomes[seed][arm] = val
            for key, v in val.items():
                if key != "examples":
                    counts[f"{arm}.{key}"] += int(v)
            if val["examples"] and len(examples) < 14:
                examples.append({"seed": seed, "arm": arm, "witnesses": val["examples"]})
            total += 1
        if (total != SEEDS * len(ARMS) or len(samples) != SEEDS
                or any(set(a) != set(ARMS) for a in outcomes.values())):
            raise ValueError("incomplete independently seeded topology cohort")
        if (counts["ambient-general.native_functional_child_write"] !=
                original["primary_native_child_shared_cell_write"]["worlds"]):
            raise ValueError("native child output contradicts original independent audit")
        return {
            "mode": mode,
            "source_revision": revision,
            "independent_seed_worlds": SEEDS,
            "matched_native_runs": total,
            "original_epoch_sha256_verified": SEEDS * len(ARMS) * 24,
            "registered_by_arm": dict(sorted(counts.items())),
            "per_seed": outcomes,
            "original_state_by_seed": samples,
            "Write_ancestry_witness_examples": examples,
        }
    finally:
        db.close()


def audit(alias_path: Path, separate_path: Path, expected_revision: str) -> dict:
    baseline = audit_mode(alias_path, "alias", expected_revision)
    treatment = audit_mode(separate_path, "separate", expected_revision)
    for seed in range(FIRST, FIRST + SEEDS):
        a = baseline["original_state_by_seed"][seed]
        b = deepcopy(treatment["original_state_by_seed"][seed])
        for actor in (1, 2, 3):
            b["processes"][str(actor)]["code"]["nodes"] = normalize_program(
                b["processes"][str(actor)]["code"]["nodes"]
            )
        if a != b:
            raise ValueError("unregistered additional initial differences between topology modes")
    discordance = Counter()
    for seed in range(FIRST, FIRST + SEEDS):
        a = baseline["per_seed"][seed]["ambient-general"]["initial_Write_source_strict_path"]
        b = treatment["per_seed"][seed]["ambient-general"]["initial_Write_source_strict_path"]
        discordance["separate_only"] += int(b and not a)
        discordance["alias_only"] += int(a and not b)
        discordance["both"] += int(a and b)
        discordance["neither"] += int(not a and not b)
    a_count = baseline["registered_by_arm"].get(
        "ambient-general.initial_Write_source_strict_path", 0
    )
    b_count = treatment["registered_by_arm"].get(
        "ambient-general.initial_Write_source_strict_path", 0
    )
    for cohort in (baseline, treatment):
        del cohort["original_state_by_seed"]
        del cohort["per_seed"]
    return {
        "study": "CL-1 Study53 prospective paired native graph source-register alias",
        "source_revision": expected_revision,
        "independent_seed_pairs": SEEDS,
        "matched_native_runs": 2 * SEEDS * len(ARMS),
        "SHA256_verified_original_epochs": 2 * SEEDS * len(ARMS) * 24,
        "primary_A_initial_Write_source_in_strict_assembly": {
            "alias_worlds": a_count,
            "separate_worlds": b_count,
            "denominator_per_mode": SEEDS,
            "alias_wilson95": [round(v, 6) for v in wilson_95(a_count, SEEDS)],
            "separate_wilson95": [round(v, 6) for v in wilson_95(b_count, SEEDS)],
            "paired_discordance": dict(sorted(discordance.items())),
            "treatment_only_mechanistic_replication":
                discordance["separate_only"] > 0,
        },
        "primary_B_functional_native_child": {
            "alias_worlds": baseline["registered_by_arm"].get(
                "ambient-general.native_functional_child_write", 0
            ),
            "separate_worlds": treatment["registered_by_arm"].get(
                "ambient-general.native_functional_child_write", 0
            ),
            "denominator_per_mode": SEEDS,
        },
        "original_modes": {"alias": baseline, "separate": treatment},
        "external_world_energy": "EXPERIMENTER-ENDOWED OR AMBIENT, NEVER AUTONOMOUS",
        "life_claim": "NONE",
    }


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("alias", type=Path)
    parser.add_argument("separate", type=Path)
    parser.add_argument("--expected-revision", required=True)
    parser.add_argument("--json", type=Path, required=True)
    args = parser.parse_args()
    result = audit(args.alias, args.separate, args.expected_revision)
    args.json.write_text(json.dumps(result, indent=2) + "\n", encoding="utf-8")
    print(json.dumps({
        "study": result["study"],
        "primary_A": result["primary_A_initial_Write_source_in_strict_assembly"],
        "primary_B": result["primary_B_functional_native_child"],
        "life_claim": result["life_claim"],
    }, indent=2))


if __name__ == "__main__":
    main()
