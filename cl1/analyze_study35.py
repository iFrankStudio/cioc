#!/usr/bin/env python3
"""Study35 preregistered independent native-source address-shift replication.

The native code/pool are initially researcher endowed. Source mutation is external.
A positive shift is NOT native life, autonomous repair, or energy production.
"""
from __future__ import annotations

import argparse
import bisect
import json
import sqlite3
from collections import Counter, defaultdict
from pathlib import Path

import analyze
import analyze_study33 as baseline
from analyze_study02 import wilson_95

FIRST = 2_700_000
SEEDS = 192
EPOCHS = 24
LABELS = baseline.LABELS
WINDOWS = ((0, 8), (8, 16), (16, 24))


def independent_baseline(path: Path) -> dict:
    """Reuse the pre-existing independent exhaustive source/hash/pool audit.

    The Rust runner is a literal frozen-grammar replay on fresh seeds, so the
    original independent auditor's program grammar and ledger apply unchanged.
    """
    old_first, old_seeds = baseline.FIRST, baseline.SEEDS
    try:
        baseline.FIRST, baseline.SEEDS = FIRST, SEEDS
        result = baseline.audit(path)
    finally:
        baseline.FIRST, baseline.SEEDS = old_first, old_seeds
    if (result["independent_seed_worlds"] != SEEDS
            or result["matched_runs"] != SEEDS * len(LABELS)
            or result["hashed_epoch_snapshots"] != SEEDS * len(LABELS) * EPOCHS):
        raise ValueError("Study35 baseline world/hash counts differ")
    return result


def actor1_native_source_trace(initial: dict, final: dict, arm: str,
                               event_lengths: list[int]) -> dict:
    """Trace native process1's actual graph-source and address register lineage."""
    program = initial["processes"]["1"]["code"]["nodes"]
    target_tag = 19 if arm.startswith("mutate-g2") else 11
    provenance: dict[int, bool] = {}
    source_reads: list[dict] = []
    writes: list[dict] = []
    draws: list[dict] = []
    attempts = 0
    prefix = len(initial["events"])
    if len(event_lengths) != EPOCHS or event_lengths[-1] != len(final["events"]):
        raise ValueError("missing or truncated epoch event chronology")
    if any(b < a for a, b in zip(event_lengths, event_lengths[1:])):
        raise ValueError("non-monotone event-count chronology")
    for index in range(prefix, len(final["events"])):
        event = final["events"][index]
        if analyze.internal_process(event) != 1:
            continue
        if event["process"] != 1:
            raise ValueError("native actor attribution mismatch")
        attempts += 1
        epoch = bisect.bisect_right(event_lengths, index)
        if not 0 <= epoch < EPOCHS:
            raise ValueError("native event not found within registered epochs")
        tick = event["tick"]
        pc, kind = event["pc"], event["kind"]
        if not isinstance(pc, int) or not 0 <= pc < len(program):
            raise ValueError("native process1 execution PC is invalid")
        op = program[pc]
        if op == "Halt" or isinstance(kind, dict) and "Trapped" in kind:
            continue
        if not isinstance(op, dict) or len(op) != 1:
            raise ValueError("invalid native executable instruction")
        name, args = next(iter(op.items()))
        if name == "GraphReadTag":
            if not isinstance(kind, dict) or "GraphTagRead" not in kind:
                raise ValueError("native graph read has no successful resolved event")
            resolved = kind["GraphTagRead"]
            qualifying = (
                resolved.get("graph") == 2
                and resolved.get("owner") == 2
                and resolved.get("node") == 0
                and resolved.get("tag") == target_tag
            )
            provenance[args["dst"]] = qualifying
            if qualifying:
                source_reads.append({
                    "tick": tick, "epoch": epoch, "pc": pc,
                    "tag": target_tag, "dst": args["dst"],
                    "graph_operand_register": args["graph"],
                })
        elif name == "Add":
            if kind != "Executed":
                raise ValueError("native Add has unexpected successful event")
            provenance[args["dst"]] = (
                provenance.get(args["lhs"], False)
                or provenance.get(args["rhs"], False)
            )
        elif name == "Write":
            if not isinstance(kind, dict) or "CellWritten" not in kind:
                raise ValueError("native Write lacks successful output event")
            if provenance.get(args["address"], False):
                written = kind["CellWritten"]
                writes.append({
                    "tick": tick, "epoch": epoch, "pc": pc,
                    "key": written["key"], "value": written["value"],
                    "postfork_native_attempts": attempts,
                    "after_first_native_draw": bool(draws),
                    "same_epoch_prior_draw": any(
                        d["epoch"] == epoch and d["tick"] < tick for d in draws
                    ),
                    "after_original_47_attempts": attempts > 47,
                    "registered_study33_target_address": written["key"] == target_tag,
                })
        elif name == "DrawCompute":
            if kind != "Executed":
                raise ValueError("native DrawCompute lacks success event")
            draws.append({"tick": tick, "epoch": epoch})
        elif name in ("Const", "Read", "GraphNew", "GraphCopyNode"):
            provenance[args["dst"]] = False
        elif name == "GraphCopyFrom":
            provenance[args["node"]] = False
        elif name not in ("GraphGrantRead", "Branch", "ReturnCompute"):
            raise ValueError(f"unregistered provenance opcode: {name}")
    return {
        "source_reads": source_reads,
        "source_address_writes": writes,
        "native_draws": draws,
        "postfork_actor1_attempts": attempts,
    }


def paired_first_shift(control: dict, mutation: dict) -> dict | None:
    """Use first eligible native write per branch; never outcome-select a pair."""
    lefts, rights = control["source_address_writes"], mutation["source_address_writes"]
    if not lefts or not rights:
        return None
    left, right = lefts[0], rights[0]
    if (not control["source_reads"] or not mutation["source_reads"]
            or left["epoch"] != right["epoch"]
            or right["key"] - left["key"] != 8
            or right["value"] != left["value"]):
        return None
    return {"epoch": left["epoch"], "original": left, "mutation": right}


def audit(path: Path) -> dict:
    base = independent_baseline(path)
    connection = sqlite3.connect(f"{path.resolve().as_uri()}?mode=ro", uri=True)
    try:
        rows = connection.execute(
            "SELECT id,condition,initial_sha256,initial_json,final_sha256,"
            "final_json,attempts_json,source_revision FROM epoch_runs ORDER BY id"
        )
        cohorts: dict[int, dict[str, dict]] = defaultdict(dict)
        metrics = Counter()
        revisions = set()
        examples = []
        for (run_id, condition, original_sha, original_raw, final_sha, final_raw,
             attempts_raw, source_revision) in rows:
            seed_txt, arm = condition.split("/", 1)
            seed = int(seed_txt.removeprefix("seed="))
            if seed not in range(FIRST, FIRST + SEEDS) or arm not in LABELS:
                raise ValueError("unexpected independent seed or condition")
            if arm in cohorts[seed]:
                raise ValueError("duplicate native branch")
            if analyze.digest(original_raw) != original_sha:
                raise ValueError("initial SHA256 mismatch")
            if analyze.digest(final_raw) != final_sha:
                raise ValueError("final SHA256 mismatch")
            initial, final = json.loads(original_raw), json.loads(final_raw)
            if any(
                initial["processes"][str(actor)]["code"]["nodes"]
                != final["processes"][str(actor)]["code"]["nodes"]
                for actor in (1, 2, 3)
            ):
                raise ValueError("unregistered native code edits/replacements")
            # Full epoch SHA digests and external schedules were independently
            # checked in independent_baseline(), prior to this stage.
            event_lengths = [
                len(json.loads(snapshot)["events"])
                for (epoch, snapshot) in connection.execute(
                    "SELECT epoch,state_json FROM epoch_snapshots "
                    "WHERE run_id=? ORDER BY epoch", (run_id,)
                )
            ]
            trace = actor1_native_source_trace(
                initial, final, arm, event_lengths
            )
            attempts = json.loads(attempts_raw)
            if len(attempts) != EPOCHS:
                raise ValueError("invalid preregistered epoch ledger")
            revisions.add(source_revision)
            cohorts[seed][arm] = trace
            for label, positive in {
                "native_draw": bool(trace["native_draws"]),
                "native_graph2_read": bool(trace["source_reads"]),
                "source_tainted_address_write": bool(trace["source_address_writes"]),
                "study33_target_address_write": any(
                    w["registered_study33_target_address"]
                    for w in trace["source_address_writes"]
                ),
                "W1_native_activity": sum(attempts[0:8]) > 0,
                "W2_native_activity": sum(attempts[8:16]) > 0,
                "W3_native_activity": sum(attempts[16:24]) > 0,
            }.items():
                metrics[f"{arm}.{label}"] += int(positive)
            if len(examples) < 12 and trace["source_address_writes"]:
                examples.append({
                    "seed": seed, "arm": arm,
                    "first_native_source_write": trace["source_address_writes"][0],
                })
        if (len(cohorts) != SEEDS
                or any(set(arms) != set(LABELS) for arms in cohorts.values())
                or revisions != {base["source_revision"]}):
            raise ValueError("unbalanced or mismatched source-pinned cohort")
        paired = []
        seed_rows = []
        for seed in sorted(cohorts):
            control, mutation = cohorts[seed]["control"], cohorts[seed]["mutate-g2"]
            match = paired_first_shift(control, mutation)
            if match is not None:
                paired.append({"seed": seed, **match})
            seed_rows.append({
                "seed": seed,
                "control_source_reads": len(control["source_reads"]),
                "mutated_source_reads": len(mutation["source_reads"]),
                "control_source_address_writes": len(control["source_address_writes"]),
                "mutated_source_address_writes": len(mutation["source_address_writes"]),
                "primary_paired_shift": match is not None,
            })
        count = len(paired)
        lo, hi = wilson_95(count, SEEDS)
        metrics["paired_source_address_shift"] = count
        metrics["paired_shift_same_epoch_native_draw"] = sum(
            p["original"]["same_epoch_prior_draw"]
            and p["mutation"]["same_epoch_prior_draw"] for p in paired
        )
        metrics["paired_shift_after_original_47_attempts"] = sum(
            p["original"]["after_original_47_attempts"]
            and p["mutation"]["after_original_47_attempts"] for p in paired
        )
        return {
            "study": "CL-1 Study35 prospective native source address replication",
            "source_revision": base["source_revision"],
            "independent_seed_worlds": SEEDS,
            "matched_runs": base["matched_runs"],
            "hashed_epoch_snapshots": base["hashed_epoch_snapshots"],
            "primary_paired_native_source_address_shift": {
                "worlds": count, "denominator": SEEDS,
                "wilson_95": [round(lo, 6), round(hi, 6)],
            },
            "primary_witnesses": paired,
            "registered_arm_metrics": dict(sorted(metrics.items())),
            "all_seed_outcomes": seed_rows,
            "descriptive_witness_examples": examples,
            "native_resource_origin": "RESEARCHER-ENDOWED, NOT AUTONOMOUS",
            "externally_changed_source": True,
            "endogenous_repair": "NOT EVALUABLE",
            "life_claim": "NONE",
        }
    finally:
        connection.close()


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("sqlite", type=Path)
    parser.add_argument("--json", type=Path, required=True)
    args = parser.parse_args()
    report = audit(args.sqlite)
    args.json.write_text(json.dumps(report, indent=2) + "\n", encoding="utf-8")
    print(json.dumps({
        "study": report["study"],
        "independent_worlds": report["independent_seed_worlds"],
        "matched_runs": report["matched_runs"],
        "primary": report["primary_paired_native_source_address_shift"],
        "life_claim": report["life_claim"],
    }))


if __name__ == "__main__":
    main()
