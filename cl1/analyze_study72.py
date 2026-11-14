#!/usr/bin/env python3
"""Study72 prospective stochastic native feedback, independent source/epoch audit.

One frozen random genome per seed. No prewritten Read→Write sequence, no external
scheduler starts, no organism survival/repair inference from finite starting credit.
"""
from __future__ import annotations

import argparse
import hashlib
import json
import sqlite3
from collections import Counter, defaultdict
from pathlib import Path

import analyze
from analyze_study02 import wilson_95

FIRST, N, EPOCHS, QUOTA = 4_800_000, 256, 24, 12
A, B, MASK = 6364136223846793005, 1442695040888963407, (1 << 64) - 1
MODES = (
    "broad-dynamic",
    "broad-static-low",
    "broad-static-high",
    "broad-ablated-dynamic",
    "broad-ablated-static-low",
    "broad-ablated-static-high",
    "short-dynamic",
    "short-static-low",
    "short-static-high",
    "short-ablated-dynamic",
    "short-ablated-static-low",
    "short-ablated-static-high",
)


def random_state(state: int) -> int:
    return (state * A + B) & MASK


def mix_seed(seed: int) -> int:
    """One-time SplitMix64 seed whitening, identical to the native Rust runner."""
    x = (seed + 0x9E3779B97F4A7C15) & MASK
    x = ((x ^ (x >> 30)) * 0xBF58476D1CE4E5B9) & MASK
    x = ((x ^ (x >> 27)) * 0x94D049BB133111EB) & MASK
    return (x ^ (x >> 31)) & MASK


def expected_code(seed: int, mode: str) -> list:
    if not FIRST <= seed < FIRST + N or mode not in MODES:
        raise ValueError("unregistered independent seed or arm")
    ablated = "-ablated-" in mode
    state = mix_seed(seed)
    code = [
        {"Const": {"dst": 1, "value": 1, "next": 1}},
        {"Const": {"dst": 2, "value": 7, "next": 2}},
        {"Const": {"dst": 3, "value": 9, "next": 3}},
    ]
    for pc in range(3, 27):
        nxt = pc + 1
        state = random_state(state)
        choice = state % 6
        if choice == 0:
            op = ({"Const": {"dst": 0, "value": 0, "next": nxt}}
                  if ablated else
                  {"Read": {"dst": 0, "address": 2, "next": nxt}})
        elif choice == 1:
            op = {"Write": {"address": 3, "value": 0, "next": nxt}}
        elif choice == 2:
            op = {"Add": {"dst": 0, "lhs": 0, "rhs": 1, "next": nxt}}
        elif choice == 3:
            op = {"Sub": {"dst": 0, "lhs": 0, "rhs": 1, "next": nxt}}
        elif choice == 4:
            state = random_state(state)
            op = {"Const": {"dst": 0, "value": state % 8, "next": nxt}}
        else:
            state = random_state(state)  # identical branch draw in both paired arms
            target = 3 if pc == 3 else (
                3 + state % (pc - 3) if mode.startswith("broad") else pc - 1
            )
            op = {"Branch": {"test": 0, "nonzero": target, "zero": nxt}}
        code.append(op)
    code.append("Halt")  # no prewired cyclic connector in EITHER regime
    if len(code) != 28:
        raise ValueError("frozen random program length changed")
    return code


def op_name(op) -> str:
    return next(iter(op)) if isinstance(op, dict) else op


def propagate_r0(op, r0: int, tainted: bool, sensor: int) -> tuple[int, bool]:
    """Independent CIL register0 def-use oracle; Const ablates inherited source."""
    name = op_name(op)
    if name == "Read":
        return sensor, True
    if name == "Const" and op["Const"]["dst"] == 0:
        return op["Const"]["value"], False
    if name == "Add" and op["Add"]["dst"] == 0:
        return r0 + 1, tainted
    if name == "Sub" and op["Sub"]["dst"] == 0:
        return r0 - 1, tainted
    return r0, tainted


def inspect_run(initial: dict, snapshots: list[tuple[int, str, str]],
                attempts: list[int], mode: str) -> dict:
    code = initial["processes"]["1"]["code"]["nodes"]
    sensor = 8 if mode.endswith("-static-high") else 3
    pc = 0
    halted = False
    r0, tainted, last_read_epoch = 0, False, None
    writes = []
    changed_edge_taken = []
    reads = 0
    native_epoch_counts = [0] * EPOCHS
    previous = initial
    charged = 0
    last_written = None
    for expected_epoch, (epoch, digest, raw) in enumerate(snapshots):
        if epoch != expected_epoch:
            raise ValueError("missing, duplicated or reordered original epoch")
        if analyze.digest(raw) != digest:
            raise ValueError("original native SHA256 epoch payload does not match")
        state = json.loads(raw)
        if state["events"][:len(previous["events"])] != previous["events"]:
            raise ValueError("native event history not a prefix")
        new = state["events"][len(previous["events"]):]
        event_sensor = 8 if mode.endswith("-dynamic") and epoch >= 12 else sensor
        switched = mode.endswith("-dynamic") and epoch == 12
        external_count = int(switched)
        if len(new) != attempts[epoch] + external_count:
            raise ValueError("native attempts or source-switch external action count incorrect")
        if switched:
            intervention = new.pop(0)
            if (intervention["origin"] != "External" or
                    op_name(intervention["kind"]) != "Intervened"):
                raise ValueError("source switch not attributed as external")
        native_epoch_counts[epoch] = len(new)
        for e in new:
            if analyze.internal_process(e) != 1 or e["process"] != 1:
                raise ValueError("external output masquerading as a native program")
            if halted or e["pc"] != pc:
                raise ValueError("impossible native PC or executing after Halt")
            op = code[pc]
            name = op_name(op)
            kind = e["kind"]
            actual = op_name(kind)
            if name == "Halt":
                if actual != "Halted":
                    raise ValueError("Halt without native halt")
                halted = True
                continue
            if name == "Read":
                if actual != "CellRead" or kind["CellRead"] != {
                    "key": 7, "value": event_sensor,
                }:
                    raise ValueError("read source input provenance incorrect")
                reads += 1
                last_read_epoch = epoch
            elif name == "Write":
                if actual != "CellWritten" or kind["CellWritten"]["key"] != 9:
                    raise ValueError("successful native Write did not output to registered cell")
                if kind["CellWritten"]["value"] != r0:
                    raise ValueError("native CIL Write not consistent with source/ALU provenance")
                writes.append({
                    "epoch": epoch,
                    "window": epoch // 8,
                    "pc": pc,
                    "tick": e["tick"],
                    "value": r0,
                    "source_tainted": tainted,
                    "latest_native_source_read_epoch": last_read_epoch if tainted else None,
                })
                last_written = r0
            elif actual != "Executed":
                raise ValueError("native non-I/O opcode failed or trapped")
            r0, tainted = propagate_r0(op, r0, tainted, event_sensor)
            if name == "Branch":
                branch = op["Branch"]
                if (mode.startswith("broad") and pc > 3
                        and r0 != 0 and branch["nonzero"] != pc-1):
                    changed_edge_taken.append({
                        "epoch":epoch,"tick":e["tick"],"pc":pc,
                        "target":branch["nonzero"]
                    })
                cond = r0 if branch["test"] == 0 else 1
                pc = branch["nonzero"] if cond != 0 else branch["zero"]
            else:
                pc = op[name]["next"]
            if not 0 <= pc < len(code):
                raise ValueError("native program edge leaves frozen executable graph")
        charged += len(new)
        p = state["processes"]["1"]
        if (state["compute_pool"] != 0 or p["credits"] != 300 - charged
                or p["code"]["nodes"] != code or state["cells"]["7"] != event_sensor
                or p["stopped"] != halted
                or (not halted and p["pc"] != pc)
                or (last_written is not None and state["cells"].get("9") != last_written)):
            raise ValueError("native state, source code, credits or I/O replay altered")
        previous = state
    if (charged != sum(attempts) or len(snapshots) != EPOCHS):
        raise ValueError("24-epoch native attempt data incomplete")
    return {
        "native_attempts": charged,
        "native_event_counts_by_epoch": native_epoch_counts,
        "native_Read_events": reads,
        "native_shared_Write_events": len(writes),
        "source_tainted_writes": sum(w["source_tainted"] for w in writes),
        "source_tainted_writes_in_W2": sum(w["source_tainted"] and
                                           w["latest_native_source_read_epoch"] is not None and
                                           8 <= w["latest_native_source_read_epoch"] < 16 and
                                           w["window"] == 1 for w in writes),
        "source_tainted_writes_in_W3": sum(w["source_tainted"] and
                                           w["latest_native_source_read_epoch"] is not None and
                                           16 <= w["latest_native_source_read_epoch"] < 24 and
                                           w["window"] == 2 for w in writes),
        "writes": writes,
        "changed_random_backedge_taken": changed_edge_taken,
        "halted": halted,
    }


def recurrent_pair(low: dict, high: dict) -> dict:
    witnesses = []
    for window in (1, 2):
        a = [w for w in low["writes"]
             if w["window"] == window and w["source_tainted"]
             and w["latest_native_source_read_epoch"] is not None
             and w["latest_native_source_read_epoch"] // 8 == window]
        b = [w for w in high["writes"]
             if w["window"] == window and w["source_tainted"]
             and w["latest_native_source_read_epoch"] is not None
             and w["latest_native_source_read_epoch"] // 8 == window]
        match = next((
            (x, y) for x in a for y in b
            if x["pc"] == y["pc"] and y["value"] - x["value"] == 5
        ), None)
        witnesses.append({
            "window": "W2" if window == 1 else "W3",
            "paired_source_sensitive": match is not None,
            "low": match[0] if match else None,
            "high": match[1] if match else None,
        })
    return {
        "sustained_source_sensitive_2_windows": all(
            w["paired_source_sensitive"] for w in witnesses
        ),
        "paired_window_witnesses": witnesses,
    }





def first_tainted_write_by_pc(run: dict, first_epoch: int, last_epoch: int) -> dict[int, dict]:
    """First successful native Write per writer PC with a fresh same-window Read."""
    hits = {}
    for w in run["writes"]:
        src = w["latest_native_source_read_epoch"]
        if (first_epoch <= w["epoch"] <= last_epoch and w["source_tainted"]
                and src is not None and first_epoch <= src <= w["epoch"]):
            hits.setdefault(w["pc"], w)
    return hits


def within_world_pair(dynamic: dict, low: dict, high: dict) -> dict:
    """Frozen PRE+POST sensor-switch output with complete matched controls."""
    pred = {}
    for win, (start, end) in {
        "PRE": (8, 11), "POST": (12, 15)
    }.items():
        pred[win] = {
            "dynamic": first_tainted_write_by_pc(dynamic, start, end),
            "low": first_tainted_write_by_pc(low, start, end),
            "high": first_tainted_write_by_pc(high, start, end),
        }
    all_pcs = sorted(set(pred["PRE"]["dynamic"]) & set(pred["POST"]["dynamic"]))
    for pc in all_pcs:
        if any(pc not in pred[window][name]
               for window in ("PRE", "POST")
               for name in ("dynamic", "low", "high")):
            continue
        a = pred["PRE"]
        b = pred["POST"]
        if (a["dynamic"][pc]["value"] != a["low"][pc]["value"]
                or b["dynamic"][pc]["value"] != b["high"][pc]["value"]
                or a["high"][pc]["value"] - a["low"][pc]["value"] != 5
                or b["high"][pc]["value"] - b["low"][pc]["value"] != 5):
            continue
        return {
            "qualified": True,
            "writer_pc": pc,
            "PRE": {k: a[k][pc] for k in ("dynamic","low","high")},
            "POST": {k: b[k][pc] for k in ("dynamic","low","high")},
        }
    return {"qualified": False, "writer_pc": None, "PRE": None, "POST": None}


def sign_test_two_sided(positive: int, negative: int) -> float:
    """Exact two-sided paired sign test, ignoring ties, no SciPy dependency."""
    from math import comb
    total = positive + negative
    if total == 0:
        return 1.0
    return min(1.0, (2.0 * sum(comb(total, k) for k in range(min(positive, negative)+1))) / (2**total))


def audit(path: Path, revision: str) -> dict:
    db = sqlite3.connect(f"{path.resolve().as_uri()}?mode=ro",uri=True)
    try:
        rows = db.execute(
            "SELECT id,condition,source_revision,initial_sha256,initial_json,"
            "actions_json,epoch_count,quota,attempts_json,final_sha256,final_json "
            "FROM epoch_runs ORDER BY id"
        )
        worlds = defaultdict(dict)
        program_hashes = {}
        totals = Counter()
        first_witnesses = []
        snapshots_count = 0
        run_count = 0
        for (rid, condition, source, sha_initial, raw_initial, raw_actions, epochs,
             quota, raw_attempts, sha_final, raw_final) in rows:
            label, mode = condition.split("/", 1)
            seed = int(label.removeprefix("seed="))
            if (not FIRST <= seed < FIRST+N or mode not in MODES
                    or mode in worlds[seed] or source != revision):
                raise ValueError("unexpected original seed, mode or source revision")
            if (analyze.digest(raw_initial) != sha_initial
                    or analyze.digest(raw_final) != sha_final
                    or epochs != EPOCHS or quota != QUOTA):
                raise ValueError("original source hashes, epoch count or quota differ")
            planned = ([{"epoch": 12, "action": {"SetCell": {"key": 7, "value": 8}}}]
                       if mode.endswith("-dynamic") else [])
            if json.loads(raw_actions) != planned:
                raise ValueError("external sensor-change intervention schedule modified")
            initial = json.loads(raw_initial)
            sensor = 8 if mode.endswith("-static-high") else 3
            if (set(initial["processes"]) != {"1"}
                    or initial["processes"]["1"]["code"]["nodes"] != expected_code(seed, mode)
                    or initial["processes"]["1"]["credits"] != 300
                    or initial["cells"] != {"7": sensor}
                    or initial["compute_pool"] != 0
                    or initial["storage_limit"] != 4
                    or initial["max_nodes_per_process"] != 32
                    or initial["total_storage_limit"] != 64
                    or not initial["strict_accounting"]):
                raise ValueError("frozen native program, controls or initial world modified")
            if (len(initial["events"]) != 2
                    or op_name(initial["events"][0]["kind"]) != "Loaded"
                    or op_name(initial["events"][1]["kind"]) != "Intervened"
                    or any(e["origin"] != "External" for e in initial["events"])):
                raise ValueError("researcher-provided initial sensor not explicitly external")
            attempts = json.loads(raw_attempts)
            if len(attempts) != EPOCHS or any(not 0<=count<=QUOTA for count in attempts):
                raise ValueError("invalid native attempt trace")
            snapshots = db.execute(
                "SELECT epoch,state_sha256,state_json FROM epoch_snapshots "
                "WHERE run_id=? ORDER BY epoch",(rid,)
            ).fetchall()
            if len(snapshots) != EPOCHS or [x[0] for x in snapshots] != list(range(EPOCHS)):
                raise ValueError("incomplete original 24-epoch source state")
            data = inspect_run(initial,snapshots,attempts,mode)
            if json.loads(snapshots[-1][2]) != json.loads(raw_final):
                raise ValueError("original final world differs from last SHA256 snapshot")
            worlds[seed][mode] = data
            if mode=="broad-dynamic":
                code=json.dumps(initial["processes"]["1"]["code"]["nodes"],
                                sort_keys=True,separators=(",",":")).encode()
                program_hashes[seed]=hashlib.sha256(code).hexdigest()
            totals[f"{mode}.worlds_native_PRE_Read"] += int(
                any(w["epoch"] in range(8,12) for w in data["writes"]
                    if w["source_tainted"] and w["latest_native_source_read_epoch"] in range(8,12)))
            totals[f"{mode}.worlds_native_POST_Read_and_write"] += int(bool(
                first_tainted_write_by_pc(data,12,15)))
            totals[f"{mode}.worlds_native_W3"] += int(any(data["native_event_counts_by_epoch"][16:]))
            totals[f"{mode}.native_Read_events"] += data["native_Read_events"]
            totals[f"{mode}.native_Write_events"] += data["native_shared_Write_events"]
            snapshots_count += EPOCHS
            run_count += 1
        if (len(worlds) != N or run_count != N * len(MODES)
                or snapshots_count != N * len(MODES) * EPOCHS
                or len(program_hashes) != N
                or len(set(program_hashes.values())) != N
                or any(set(x) != set(MODES) for x in worlds.values())):
            raise ValueError("incomplete or non-unique native Study72 genotype cohort")
        pair=Counter()
        witnessed={}
        for seed,modes in worlds.items():
            observed={}
            for topo in ("broad","short"):
                dyn=modes[f"{topo}-dynamic"]
                low=modes[f"{topo}-static-low"]
                high=modes[f"{topo}-static-high"]
                # A dynamic source change may only appear as EXTERNAL SetCell, never
                # Read ablation or an actor-specific process refill.
                abdyn=modes[f"{topo}-ablated-dynamic"]
                ablow=modes[f"{topo}-ablated-static-low"]
                abhigh=modes[f"{topo}-ablated-static-high"]
                for ab in (abdyn,ablow,abhigh):
                    if ab["native_Read_events"] != 0:
                        raise ValueError("Read-ablated control executed a native sensor Read")
                # The dynamic branch contains exactly one extra *external*
                # SetCell at epoch12. It advances global event ticks by one,
                # without advancing native PC, attempt order or register state.
                # Compare aligned native output traces, not raw global clocks.
                signature=lambda run,dynamic:[(
                    w["epoch"],
                    w["tick"] - int(dynamic and w["epoch"] >= 12),
                    w["pc"],w["value"],
                ) for w in run["writes"]]
                if (signature(abdyn,True)!=signature(ablow,False)
                        or signature(abdyn,True)!=signature(abhigh,False)):
                    raise ValueError("Read-ablated native output changed under a sensor switch")
                # Matched programs were identical before dynamic intervention.
                if (dyn["native_event_counts_by_epoch"][:12] != low["native_event_counts_by_epoch"][:12]
                        or any((w["epoch"],w["tick"],w["pc"],w["value"])
                               not in [(v["epoch"],v["tick"],v["pc"],v["value"])
                                       for v in low["writes"]] for w in dyn["writes"]
                               if w["epoch"] < 12)):
                    raise ValueError("dynamic program diverged before external intervention")
                result=within_world_pair(dyn,low,high)
                observed[topo]=result["qualified"]
                totals[f"{topo}.worlds_primary_within_world_switch"]+=int(result["qualified"])
                if result["qualified"] and len(first_witnesses)<25:
                    first_witnesses.append({"seed":seed,"topology":topo,**result})
            pair["broad_only"]+=int(observed["broad"] and not observed["short"])
            pair["short_only"]+=int(observed["short"] and not observed["broad"])
            pair["both"]+=int(observed["broad"] and observed["short"])
            pair["neither"]+=int(not observed["broad"] and not observed["short"])
            witnessed[str(seed)] = observed
        b=totals["broad.worlds_primary_within_world_switch"]
        s=totals["short.worlds_primary_within_world_switch"]
        if b!=pair["broad_only"]+pair["both"] or s!=pair["short_only"]+pair["both"]:
            raise ValueError("paired response population inconsistent")
        return {
            "study":"CL-1 Study72 within-world prospective dynamic sensor tracking",
            "source_revision":revision,
            "original_independent_genomes":N,
            "matched_native_runs":run_count,
            "SHA256_verified_epoch_snapshots":snapshots_count,
            "registered_primary_within_world_dynamic_tracking":{
                "broad_worlds":b,"short_worlds":s,"denominator_each":N,
                "broad_Wilson95":[round(v,6) for v in wilson_95(b,N)],
                "short_Wilson95":[round(v,6) for v in wilson_95(s,N)],
                "paired_discordance":dict(sorted(pair.items())),
                "two_sided_paired_sign_p":sign_test_two_sided(pair["broad_only"],pair["short_only"]),
                "directional_broad_vs_short":pair["broad_only"]>pair["short_only"],
            },
            "original_genotype_uniqueness":{
                "unique_broad_native_genotypes":len(set(program_hashes.values())),
                "mixed_initial_state":"SplitMix64 then unchanged CIL LCG",
            },
            "registered_reachability_and_controls":dict(sorted(totals.items())),
            "first_native_source_switch_witnesses":first_witnesses,
            "per_seed_qualifications":witnessed,
            "organism_self_maintained":"NOT DEMONSTRATED; all external sensor and initial compute",
            "life_claim":"NONE",
        }
    finally:
        db.close()


def main() -> None:
    p=argparse.ArgumentParser(description=__doc__)
    p.add_argument("sqlite",type=Path)
    p.add_argument("--expected-revision",required=True)
    p.add_argument("--json",type=Path,required=True)
    args=p.parse_args()
    report=audit(args.sqlite,args.expected_revision)
    args.json.write_text(json.dumps(report,indent=2)+"\n",encoding="utf-8")
    print(json.dumps({
        "study":report["study"],
        "new_genomes":report["original_independent_genomes"],
        "snapshots":report["SHA256_verified_epoch_snapshots"],
        "primary":report["registered_primary_within_world_dynamic_tracking"],
        "life_claim":"NONE",
    },indent=2))


if __name__=="__main__":
    main()
