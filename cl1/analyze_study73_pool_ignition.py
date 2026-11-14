#!/usr/bin/env python3
"""Study73 six SCRIPTED native pool-paid Draw mechanism fixtures, NOT life."""
from __future__ import annotations

import argparse
import json
import sqlite3
from pathlib import Path

import analyze

MODES = (
    "pool-paid-draw",
    "legacy-zero-credit-draw",
    "pool-paid-non-draw",
    "pool-paid-empty-pool",
    "pool-paid-insufficient-pool",
    "pool-paid-stopped-draw",
)
EXPECTED = {
    "pool-paid-draw": (5, 1, 1, 0, 2),
    "legacy-zero-credit-draw": (1, 0, 0, 0, 6),
    "pool-paid-non-draw": (1, 0, 0, 0, 6),
    "pool-paid-empty-pool": (1, 0, 0, 0, 0),
    "pool-paid-insufficient-pool": (2, 1, 0, 1, 2),
    "pool-paid-stopped-draw": (0, 0, 0, 0, 6),
}


def kind(event: dict) -> str:
    x = event["kind"]
    return next(iter(x)) if isinstance(x, dict) else x


def program(mode: str) -> list:
    if mode == "pool-paid-stopped-draw":
        return [{"DrawCompute": {"amount": 1, "next": 0}}]
    return [
        {"Const": {"dst": 1, "value": 3, "next": 1}},
        ({"Const": {"dst": 0, "value": 8, "next": 2}}
         if mode == "pool-paid-non-draw" else
         {"DrawCompute": {"amount": 1, "next": 2}}),
        {"Const": {"dst": 0, "value": 7, "next": 3}},
        {"Write": {"address": 2, "value": 0, "next": 4}},
        "Halt",
    ]


def audit(path: Path, revision: str) -> dict:
    conn = sqlite3.connect(f"{path.resolve().as_uri()}?mode=ro", uri=True)
    try:
        rows = conn.execute(
            "SELECT id,condition,source_revision,initial_sha256,initial_json,"
            "actions_json,epoch_count,quota,attempts_json,final_sha256,final_json "
            "FROM epoch_runs ORDER BY id"
        ).fetchall()
        if len(rows) != len(MODES):
            raise ValueError("not six and only six scripted original worlds")
        outcomes = {}
        snapshots_count = 0
        for rid, name, src, init_sha, init_raw, actions, epochs, quota, att_raw, end_sha, end_raw in rows:
            if name not in MODES or name in outcomes or src != revision:
                raise ValueError("source version, mode or duplicate script wrong")
            if (analyze.digest(init_raw) != init_sha or analyze.digest(end_raw) != end_sha
                    or epochs != 24 or quota != 12 or json.loads(actions) != []):
                raise ValueError("original source hashes, mode epochs or actions wrong")
            initial = json.loads(init_raw)
            if (list(initial["processes"]) != ["1"]
                    or initial["processes"]["1"]["code"]["nodes"] != program(name)
                    or initial["processes"]["1"]["credits"] != (
                        0 if name == "pool-paid-stopped-draw" else 1)
                    or initial["processes"]["1"]["stopped"] != (
                        name == "pool-paid-stopped-draw")
                    or initial.get("pool_paid_draw", False) != (
                        name != "legacy-zero-credit-draw")
                    or initial["storage_limit"] != 4
                    or initial["max_nodes_per_process"] != 8
                    or initial["total_storage_limit"] != 18
                    or not initial["strict_accounting"]
                    or initial["compute_pool"] != (
                        0 if name == "pool-paid-empty-pool"
                        else 3 if name == "pool-paid-insufficient-pool"
                        else 6)
                    or initial["cells"]):
                raise ValueError("scripted source/rules differ from registered original")
            first = initial["events"]
            if (len(first) != 2 or first[0]["origin"] != "External"
                    or kind(first[0]) != "Loaded"
                    or first[1]["origin"] != "External"
                    or kind(first[1]) != "Intervened"
                    or any(e.get("pool_paid_attempt", False) for e in first)):
                raise ValueError("incorrect initial EXTERNAL credit endowment ledger")
            previous = initial
            steps = []
            attempts = json.loads(att_raw)
            if len(attempts) != 24:
                raise ValueError("missing charged attempt list")
            for epoch, digest, snapshot_raw in conn.execute(
                "SELECT epoch,state_sha256,state_json FROM epoch_snapshots "
                "WHERE run_id=? ORDER BY epoch", (rid,)
            ):
                if epoch != snapshots_count % 24 or analyze.digest(snapshot_raw) != digest:
                    raise ValueError("missing or corrupted original snapshot")
                state = json.loads(snapshot_raw)
                if state["events"][:len(previous["events"])] != previous["events"]:
                    raise ValueError("native event history not append-only")
                fresh = state["events"][len(previous["events"]):]
                if len(fresh) != attempts[epoch]:
                    raise ValueError("outside native/external action bookkeeping")
                for e in fresh:
                    if e["origin"] != {"Internal": 1} or e["process"] != 1:
                        raise ValueError("an external event falsely credited as native")
                    if e.get("pool_paid_attempt", False) and (
                        not initial.get("pool_paid_draw") or e["pc"] != 1
                        or program(name)[1] != {"DrawCompute": {"amount": 1, "next": 2}}
                    ):
                        raise ValueError("false pool-paid native attempt provenance")
                    steps.append(e)
                if (state["processes"]["1"]["code"]["nodes"] != program(name)
                        or state.get("pool_paid_draw", False)
                        != initial.get("pool_paid_draw", False)):
                    raise ValueError("native code or the substrate rule unexpectedly changed")
                original_total = initial["compute_pool"] + initial["processes"]["1"]["credits"]
                if (state["compute_pool"] + state["processes"]["1"]["credits"]
                        != original_total - len(steps)):
                    raise ValueError("native computation minted or lost unexplained pool/actor credits")
                previous = state
                snapshots_count += 1
            if previous != json.loads(end_raw) or sum(attempts) != len(steps):
                raise ValueError("original final state or attempt history differs")
            # Native executable PC ancestry must match the fixed source code,
            # including the zero-credit scheduler's single pool-paid Draw.
            expected_pc = {
                "pool-paid-draw": [0, 1, 2, 3, 4],
                "legacy-zero-credit-draw": [0],
                "pool-paid-non-draw": [0],
                "pool-paid-empty-pool": [0],
                "pool-paid-insufficient-pool": [0, 1],
                "pool-paid-stopped-draw": [],
            }[name]
            if [e["pc"] for e in steps] != expected_pc:
                raise ValueError("native CIL sequence was not the frozen original program")
            if any(e.get("pool_paid_attempt", False) for e in steps if e["pc"] != 1):
                raise ValueError("non-Draw code improperly accessed pooled ignition")
            writes = [e for e in steps if kind(e) == "CellWritten"]
            traps = [e for e in steps if kind(e) == "Trapped"]
            fees = [e for e in steps if e.get("pool_paid_attempt", False)]
            observed = (
                len(steps),len(fees),len(writes),len(traps),previous["compute_pool"]
            )
            if observed != EXPECTED[name]:
                raise ValueError(f"scripted mode failed frozen outcome: {name}: {observed}")
            if any(e["pc"] != 3 or e["kind"]["CellWritten"] != {"key": 0,"value":7}
                   for e in writes):
                raise ValueError("fabricated or wrong native cell output")
            if len(fees) == 1 and (
                fees[0]["pc"] != 1 or
                (name == "pool-paid-draw" and kind(fees[0]) != "Executed") or
                (name == "pool-paid-insufficient-pool" and kind(fees[0]) != "Trapped")
            ):
                raise ValueError("wrong native resource acquisition success/trap gate")
            outcomes[name] = {
                "native_attempts":len(steps),
                "world_pool_paid_native_attempts":len(fees),
                "native_shared_writes":len(writes),
                "native_traps":len(traps),
                "final_world_pool":previous["compute_pool"],
                "final_local_credits":previous["processes"]["1"]["credits"],
            }
        if set(outcomes) != set(MODES) or snapshots_count != 144:
            raise ValueError("incomplete six-fixture exact world snapshots")
        return {
            "study":"CL-1 Study73 opt-in pool-paid Draw HUMAN-SCRIPTED mechanism calibration",
            "source_revision":revision,
            "independent_stochastic_worlds":0,
            "scripted_mechanism_fixtures":6,
            "matched_native_runs":6,
            "original_SHA256_snapshots":snapshots_count,
            "registered_modes":outcomes,
            "energy_creation":"NONE",
            "host_actor_targeted_rescue":"NONE",
            "life_claim":"NONE",
        }
    finally:
        conn.close()


def main():
    p=argparse.ArgumentParser(description=__doc__)
    p.add_argument("sqlite",type=Path)
    p.add_argument("--expected-revision",required=True)
    p.add_argument("--json",type=Path,required=True)
    args=p.parse_args()
    report=audit(args.sqlite,args.expected_revision)
    args.json.write_text(json.dumps(report,indent=2)+"\n",encoding="utf-8")
    print(json.dumps(report,indent=2))


if __name__=="__main__":
    main()
