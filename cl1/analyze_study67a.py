#!/usr/bin/env python3
"""Study67-A: read-only Stage66 native Draw-only loan eligibility (no new worlds).

Earlier frozen Stage68-labeled read-only protocol was administratively
renumbered because a distinct prospective Study68 was already in flight.
"""
from __future__ import annotations

import argparse
import json
import sqlite3
from collections import Counter, defaultdict
from pathlib import Path

import analyze
import analyze_study67

REVISION = analyze_study67.REVISION
FIRST, N = 4_400_000, 256
MODES = analyze_study67.MODES


def current_cil_opcode(op: object) -> str:
    if isinstance(op, dict) and len(op) == 1:
        return next(iter(op))
    if op == "Halt":
        return "Halt"
    raise ValueError("not a supported original CIL opcode")


def native_eligibility(state: dict) -> dict:
    if set(state["processes"]) != {"1"}:
        raise ValueError("wrong source process population for original Study66")
    actor = state["processes"]["1"]
    program = actor["code"]["nodes"]
    pc = actor["pc"]
    if not isinstance(pc, int) or not 0 <= pc < len(program):
        raise ValueError("invalid native original W3 program counter")
    opcode = current_cil_opcode(program[pc])
    pool, credit, stopped = state["compute_pool"], actor["credits"], actor["stopped"]
    if pool < 0 or credit < 0:
        raise ValueError("negative persisted compute inventory")
    amount = None
    if opcode == "DrawCompute":
        amount_reg = program[pc]["DrawCompute"]["amount"]
        if not isinstance(amount_reg, int) or not 0 <= amount_reg < len(actor["registers"]):
            raise ValueError("native DrawCompute requests an invalid amount register")
        amount = actor["registers"][amount_reg]
        if not isinstance(amount, int) or amount < 0:
            raise ValueError("invalid negative resolved Draw quantity")
    pending = not stopped and credit == 0 and pool > 0 and opcode == "DrawCompute"
    eligible = pending and amount is not None and pool >= amount + 1
    if stopped:
        category = "stopped"
    elif credit > 0:
        category = "runnable_positive_credit"
    elif pool == 0:
        category = "zero_credit_empty_pool"
    elif opcode == "DrawCompute":
        category = "zero_credit_positive_pool_next_Draw"
    else:
        category = "zero_credit_positive_pool_next_other"
    return {
        "category": category,
        "current_pc": pc,
        "current_opcode": opcode,
        "actor_credits": credit,
        "world_pool": pool,
        "actor_stopped": stopped,
        "native_draw_requested": amount,
        "pending_Draw": pending,
        "pending_Draw_pool_sufficient_for_toll_and_transfer": eligible,
        "pending_Draw_pool_insufficient": pending and not eligible,
    }


def audit(path: Path) -> dict:
    # Independently reuse the full Stage67 source-auditor and its Stage66 original.
    previous = analyze_study67.audit(path)
    histogram = previous["per_arm_retrospective_resource_failure_counts"]
    if previous["original_SHA256_snapshots_rechecked"] != N * len(MODES) * 24:
        raise ValueError("full original native state SHA256 audit did not pass")
    for mode in ("ambient-low", "ambient-high"):
        if (histogram.get(f"{mode}.W3_entry_zero_credit_deadlock_with_pool") != 207
                or histogram.get(f"{mode}.W3_entry_stopped_with_pool") != 49):
            raise ValueError("Stage67 historical original resource classes mutated")
    db = sqlite3.connect(f"{path.resolve().as_uri()}?mode=ro", uri=True)
    try:
        per_seed = defaultdict(dict)
        counts = Counter()
        first = []
        hashes = 0
        for label, revision, epoch, digest, raw in db.execute(
            "SELECT r.condition, r.source_revision, s.epoch, s.state_sha256, s.state_json "
            "FROM epoch_runs r JOIN epoch_snapshots s ON r.id=s.run_id "
            "WHERE s.epoch=15 ORDER BY r.id"
        ):
            prefix, mode = label.split("/", 1)
            seed = int(prefix.removeprefix("seed="))
            if (revision != REVISION or not FIRST <= seed < FIRST + N
                    or mode not in MODES or mode in per_seed[seed]
                    or epoch != 15 or analyze.digest(raw) != digest):
                raise ValueError("unregistered, duplicated or corrupted original W3 state")
            data = native_eligibility(json.loads(raw))
            per_seed[seed][mode] = data
            hashes += 1
            counts[f"{mode}.state_{data['category']}"] += 1
            if not data["actor_stopped"] and data["actor_credits"] == 0:
                counts[f"{mode}.zero_credit_next_opcode_{data['current_opcode']}"] += 1
            for key in ("pending_Draw", "pending_Draw_pool_sufficient_for_toll_and_transfer",
                        "pending_Draw_pool_insufficient"):
                counts[f"{mode}.{key}_worlds"] += int(data[key])
            if data["pending_Draw"] and len(first) < 24:
                first.append({"seed": seed, "arm": mode, **data})
        if (len(per_seed) != N or hashes != N * len(MODES)
                or any(set(arms) != set(MODES) for arms in per_seed.values())):
            raise ValueError("incomplete correlated eight-arm original native population")
        for mode in ("ambient-low", "ambient-high"):
            if (counts[f"{mode}.state_stopped"] != 49
                    or counts[f"{mode}.state_zero_credit_positive_pool_next_Draw"]
                    + counts[f"{mode}.state_zero_credit_positive_pool_next_other"] != 207):
                raise ValueError("source-pinned Stage67 W3 class count discrepancy")
        pair = Counter()
        for seed in range(FIRST, FIRST + N):
            low = per_seed[seed]["ambient-low"][
                "pending_Draw_pool_sufficient_for_toll_and_transfer"]
            high = per_seed[seed]["ambient-high"][
                "pending_Draw_pool_sufficient_for_toll_and_transfer"]
            pair["both"] += int(low and high)
            pair["low_only"] += int(low and not high)
            pair["high_only"] += int(high and not low)
            pair["neither"] += int(not low and not high)
        return {
            "study": "CL-1 Study67-A original W3 native Draw credit-loan reachability",
            "status": "READ ONLY / AFTER ORIGINAL RESULT / NO COUNTERFACTUAL RUN",
            "original_Study68_labeled_preregistration_retained": True,
            "source_revision": REVISION,
            "new_independent_worlds": 0,
            "source_independent_genomes_reused": N,
            "historical_matched_runs_reaudited": N*len(MODES),
            "source_SHA256_epoch_snapshots_reaudited": N*len(MODES)*24,
            "extra_original_epoch15_SHA256_checks": hashes,
            "per_mode_class_and_next_CIL_opcode_counts": dict(sorted(counts.items())),
            "paired_low_high_eligible_at_W3": dict(sorted(pair.items())),
            "first_pending_native_Draw_historical_states": first,
            "native_credit_advance": "HYPOTHETICAL WORLD LAW NOT IMPLEMENTED",
            "life_claim": "NONE",
        }
    finally:
        db.close()


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("sqlite", type=Path)
    parser.add_argument("--json", type=Path, required=True)
    args = parser.parse_args()
    result = audit(args.sqlite)
    args.json.write_text(json.dumps(result, indent=2) + "\n", encoding="utf-8")
    print(json.dumps({
        "study": result["study"],
        "new_worlds":0,
        "classes": {k:v for k,v in result["per_mode_class_and_next_CIL_opcode_counts"].items()
                    if k.startswith("ambient-low.") or k.startswith("ambient-high.")},
        "paired":result["paired_low_high_eligible_at_W3"],
        "life_claim":"NONE",
    },indent=2))


if __name__ == "__main__":
    main()
