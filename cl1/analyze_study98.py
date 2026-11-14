#!/usr/bin/env python3
"""Retrospective static-only predictor for original Study97 random child phenotype.

Zero new worlds: full original archive is reverified before comparing an
initial-genotype-only, fixed-loop affine register def-use prediction with the
already-registered native repeated source function F for all original seeds.
"""
import argparse
import json
from pathlib import Path

import analyze_study97 as original
from analyze_study82 import variant

SOURCE_SHA = "8b4ecc329773f090d5bd6273114794d28b382994"


def require(ok, message):
    if not ok:
        raise ValueError(message)


def predict_original_child(seed, source):
    code = original.expected_payload(seed)
    pc = 0
    registers = [0, 0, 0, 0]
    tainted = False
    offset = 0
    first = set()
    second = set()
    writer_events = []
    for attempt in range(1, 49):
        require(0 <= pc < 16, "static child code escaped 16-node bounds")
        opcode, fields = variant(code[pc])
        executed_pc = pc
        if opcode == "Const":
            dst = fields["dst"]
            registers[dst] = fields["value"]
            if dst == 0:
                tainted = False
                offset = fields["value"]
            pc = fields["next"]
        elif opcode == "Read":
            require(fields["dst"] == 0 and registers[fields["address"]] == 7,
                    "random CIL read not based on source7 ABI")
            registers[0] = source
            tainted = True
            offset = 0
            pc = fields["next"]
        elif opcode == "Write":
            require(registers[fields["address"]] == 9 and fields["value"] == 0,
                    "random CIL writer not based on destination9 ABI")
            actual_value = registers[0]
            if tainted:
                require(actual_value == source + offset, "tainted source affine formula")
                (first if attempt <= 24 else second).add((executed_pc, offset))
            writer_events.append({
                "attempt": attempt,
                "writer_pc": executed_pc,
                "source_tainted": tainted,
                "value": actual_value,
                "offset": offset,
            })
            pc = fields["next"]
        elif opcode == "Add":
            require((fields["dst"], fields["lhs"], fields["rhs"]) == (0, 0, 1),
                    "unregistered random arithmetic syntax")
            registers[0] += registers[1]
            offset += registers[1]
            pc = fields["next"]
        elif opcode == "Sub":
            require((fields["dst"], fields["lhs"], fields["rhs"]) == (0, 0, 1),
                    "unregistered random arithmetic syntax")
            registers[0] -= registers[1]
            offset -= registers[1]
            pc = fields["next"]
        elif opcode == "Branch":
            require(executed_pc == 15 and fields == {
                "test": 2, "nonzero": 3, "zero": 3,
            } and registers[2] == 7, "undisclosed environmental loop control")
            pc = 3
        else:
            raise ValueError("unexpected stochastic native child opcode")
    return {
        "window_first_keys": sorted(first),
        "window_second_keys": sorted(second),
        "two_window_function_keys": sorted(first & second),
        "writer_events": writer_events,
    }


def audit(sqlite):
    original_result = original.audit(sqlite, SOURCE_SHA)
    require((
        original_result["sampled_original_seed_labels"],
        original_result["distinct_child_payload_hashes"],
        original_result["original_source_arms"],
        original_result["verified_original_epoch_sha256"],
        original_result["source_paired_repeated_child_function_local_F"],
        original_result["source_paired_repeated_child_function_absolute_stale_F"],
        original_result["source_paired_repeated_child_function_zero_credit_F"],
    ) == (256, 256, 1536, 18432, 151, 0, 0),
        "original registered Study97 denominator/F outcomes changed")
    rows = original_result["per_original_payload_outcome"]
    require(len(rows) == 256, "original auditor did not return all 256 genotypes")
    predicted_yes = 0
    comparison = []
    for index, seed in enumerate(range(original.FIRST, original.FIRST + original.SEEDS)):
        original_row = rows[index]
        require(original_row["seed"] == seed, "original seed ordering changed")
        source3 = predict_original_child(seed, 3)
        source8 = predict_original_child(seed, 8)
        source3_keys = set(tuple(key) for key in source3["two_window_function_keys"])
        source8_keys = set(tuple(key) for key in source8["two_window_function_keys"])
        predicted = sorted(source3_keys & source8_keys)
        phenotype = original_row["source_paired_results"]["local"]
        native_observed = sorted(
            tuple(key) for key in phenotype["paired_child_writer_pc_and_affine_offsets"]
        )
        require(predicted == native_observed
                and bool(predicted) == phenotype["paired_inherited_F"],
                f"static, preregistered random-CIL prediction mismatch at original seed {seed}")
        if predicted:
            predicted_yes += 1
        comparison.append({
            "seed": seed,
            "static_predicted_two_window_source_writers": predicted,
            "original_matched_native_F": phenotype["paired_inherited_F"],
        })
    require(predicted_yes == 151, "original 151 F not exactly explained by static source code")
    return {
        "study": 98,
        "status": "EXACT_STATIC_CODE_NULL_MATCH_RETROSPECTIVE_ZERO_NEW_WORLDS",
        "original_revision": SOURCE_SHA,
        "original_ci": 43602170104,
        "reverified_original_payload_seeds": 256,
        "reverified_original_native_worlds": 1536,
        "reverified_original_SHA256_epochs": 18432,
        "statically_predicted_local_F": predicted_yes,
        "native_observed_local_F": 151,
        "statically_unexplained_positive_F": 0,
        "original_stale_control_F": 0,
        "original_no_child_credit_control_F": 0,
        "human_authored_assembler_and_finite_loop": True,
        "adaptive_endogenous_constructor_or_reproduction": False,
        "individual_original_seed_prediction": comparison,
        "cl1_closed": False,
    }


def main():
    p = argparse.ArgumentParser()
    p.add_argument("sqlite", type=Path)
    p.add_argument("--json", type=Path, required=True)
    args = p.parse_args()
    result = audit(args.sqlite)
    args.json.write_text(json.dumps(result, indent=2, sort_keys=True) + "\n")
    print(json.dumps({
        key: result[key]
        for key in (
            "study", "status", "reverified_original_payload_seeds",
            "statically_predicted_local_F", "native_observed_local_F",
            "statically_unexplained_positive_F",
        )
    }, sort_keys=True))


if __name__ == "__main__":
    main()
