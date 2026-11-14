#!/usr/bin/env python3
"""Study77 independently audit random source feedback plus native pool harvesting.

External +8 world-pool input is not native credit generation. Native DrawCompute
is an actor-executed operation, but does not establish endogenous cyberlife.
"""
from __future__ import annotations

import argparse
import json
import sqlite3
from collections import Counter, defaultdict
from pathlib import Path

import analyze
from analyze_study02 import wilson_95
from analyze_study65 import op_name

FIRST, N, EPOCHS, QUOTA = 5_100_000, 128, 24, 12
A, B, MASK = 6364136223846793005, 1442695040888963407, (1 << 64) - 1
MODES = (
    "broad-ambient-retry-intact-low",
    "broad-ambient-retry-intact-high",
    "broad-ambient-legacy-intact-low",
    "broad-ambient-legacy-intact-high",
    "short-ambient-retry-intact-low",
    "short-ambient-retry-intact-high",
    "short-ambient-legacy-intact-low",
    "short-ambient-legacy-intact-high",
    "broad-ambient-retry-draw-ablated-low",
    "broad-ambient-retry-draw-ablated-high",
    "broad-ambient-legacy-draw-ablated-low",
    "broad-ambient-legacy-draw-ablated-high",
    "broad-closed-retry-intact-low",
    "broad-closed-retry-intact-high",
    "short-closed-retry-intact-low",
    "short-closed-retry-intact-high",
)


def random_state(state: int) -> int:
    return (state * A + B) & MASK


def mix_seed(seed: int) -> int:
    x = (seed + 0x9E3779B97F4A7C15) & MASK
    x = ((x ^ (x >> 30)) * 0xBF58476D1CE4E5B9) & MASK
    x = ((x ^ (x >> 27)) * 0x94D049BB133111EB) & MASK
    return (x ^ (x >> 31)) & MASK


def expected_code(seed: int, mode: str) -> list:
    if not FIRST <= seed < FIRST + N or mode not in MODES:
        raise ValueError("unregistered Study77 seed or intervention arm")
    draw_ablated = "draw-ablated" in mode
    state = mix_seed(seed)
    code = [
        {"Const": {"dst": 1, "value": 8, "next": 1}},
        {"Const": {"dst": 2, "value": 7, "next": 2}},
        {"Const": {"dst": 3, "value": 9, "next": 3}},
    ]
    for pc in range(3, 27):
        nxt = pc + 1
        state = random_state(state)
        choice = state % 7
        if choice == 0:
            op = {"Read": {"dst": 0, "address": 2, "next": nxt}}
        elif choice == 1:
            op = {"Write": {"address": 3, "value": 0, "next": nxt}}
        elif choice == 2:
            op = {"Add": {"dst": 0, "lhs": 0, "rhs": 1, "next": nxt}}
        elif choice == 3:
            op = {"Sub": {"dst": 0, "lhs": 0, "rhs": 1, "next": nxt}}
        elif choice == 4:
            state = random_state(state)
            op = {"Const": {"dst": 0, "value": state % 8, "next": nxt}}
        elif choice == 5:
            state = random_state(state)
            target = 3 if pc == 3 else (
                3 + state % (pc - 3) if mode.startswith("broad") else pc - 1
            )
            op = {"Branch": {"test": 0, "nonzero": target, "zero": nxt}}
        else:
            op = ({"Const": {"dst": 1, "value": 8, "next": nxt}}
                  if draw_ablated
                  else {"DrawCompute": {"amount": 1, "next": nxt}})
        code.append(op)
    code.append("Halt")
    if len(code) != 28:
        raise ValueError("Study77 original code length changed")
    return code


def propagate_r0(op, value: int, tainted: bool, sensor: int) -> tuple[int, bool]:
    name = op_name(op)
    if name == "Read":
        return sensor, True
    if name == "Const" and op["Const"]["dst"] == 0:
        return op["Const"]["value"], False
    if name == "Add" and op["Add"]["dst"] == 0:
        return value + 8, tainted
    if name == "Sub" and op["Sub"]["dst"] == 0:
        return value - 8, tainted
    return value, tainted


def trace_run(initial: dict, snapshots: list[tuple[int, str, str]],
              actions: list[dict], attempts: list[int], mode: str) -> dict:
    code = initial["processes"]["1"]["code"]["nodes"]
    ambient = "-ambient-" in mode
    sensor = 3 if mode.endswith("-low") else 8
    initial_pool = 96 if ambient else 0
    current_pool = initial_pool
    current_credits = 64
    r0, tainted, last_read_epoch = 0, False, None
    pc, stopped = 0, False
    drawn = []
    pool_paid_events = []
    deferred = []
    writes = []
    reads = 0
    count_by_window = [0, 0, 0]
    cumulative_attempts = 0
    ambient_events = 0
    previous = initial
    for expected_epoch, (epoch, digest, raw) in enumerate(snapshots):
        if epoch != expected_epoch or analyze.digest(raw) != digest:
            raise ValueError("native Study77 snapshot hash/order corrupted")
        state = json.loads(raw)
        if state["events"][:len(previous["events"])] != previous["events"]:
            raise ValueError("native event chronology does not extend original prefix")
        new = state["events"][len(previous["events"]):]
        scheduled = [x for x in actions if x["epoch"] == epoch]
        if (len(new) != len(scheduled) + attempts[epoch]
                or len(scheduled) != int(ambient and epoch >= 1)):
            raise ValueError("unregistered external input or attempted native instruction")
        for a, event in zip(scheduled, new[:len(scheduled)]):
            if (event["origin"] != "External"
                    or op_name(event["kind"]) != "Intervened"
                    or set(a["action"]) != {"SetComputePool"}
                    or a["action"]["SetComputePool"]["credits"] != current_pool + 8):
                raise ValueError("external environment pool action changed")
            current_pool += 8
            ambient_events += 1
        native = new[len(scheduled):]
        for event in native:
            if (analyze.internal_process(event) != 1
                    or event["process"] != 1
                    or stopped or event["pc"] != pc):
                raise ValueError("native process was falsely woken or PC does not match")
            op = code[pc]
            name = op_name(op)
            kind = event["kind"]
            result = op_name(kind)
            # Provenance of an opt-in world-pool-paid native Draw attempt is
            # explicit. Never infer organism-created energy from this subsidy.
            pool_paid = event.get("pool_paid_attempt", False)
            if pool_paid:
                if (name != "DrawCompute" or current_credits != 0 or current_pool < 1):
                    raise ValueError("forged pool-paid Draw/native zero-credit entry")
                if "-retry-" in mode and current_pool < 9:
                    raise ValueError("recoverable policy paid native Draw without fee+8 reserve")
                current_pool -= 1
                pool_paid_events.append({
                    "epoch": epoch, "window": epoch//8,
                    "tick": event["tick"], "pc": pc,
                })
            else:
                if current_credits <= 0:
                    raise ValueError("unfunded native non-Draw instruction executed")
                current_credits -= 1
            cumulative_attempts += 1
            count_by_window[epoch // 8] += 1
            if name == "DrawCompute":
                if result == "Executed":
                    if current_pool < 8:
                        raise ValueError("native DrawCompute succeeded without pool energy")
                    current_pool -= 8
                    current_credits += 8
                    drawn.append({"epoch": epoch, "window": epoch//8,
                                  "tick": event["tick"], "pc": pc})
                elif result == "DrawDeferred":
                    if ("-retry-" not in mode or current_pool >= 8
                            or kind["DrawDeferred"] != {
                                "requested": 8, "available": current_pool
                            }):
                        raise ValueError("fake native DrawDeferred or resource wasn't short")
                    deferred.append({"epoch":epoch,"window":epoch//8,
                                     "tick":event["tick"],"pc":pc})
                elif result == "Trapped":
                    if ("-retry-" in mode or current_pool >= 8
                            or "insufficient world compute pool" not in kind["Trapped"]):
                        raise ValueError("native invalid resource trap in retry world")
                    stopped = True
                else:
                    raise ValueError("incorrect native Draw success, defer or trap provenance")
            elif name == "Halt":
                if result != "Halted":
                    raise ValueError("native Halt event not correctly accounted")
                stopped = True
            elif name == "Read":
                if result != "CellRead" or kind["CellRead"] != {
                    "key": 7, "value": sensor,
                }:
                    raise ValueError("source input was not natively sensed")
                last_read_epoch = epoch
                reads += 1
            elif name == "Write":
                if (result != "CellWritten"
                        or kind["CellWritten"] != {"key": 9, "value": r0}):
                    raise ValueError("native Write violates reconstructed CIL dataflow")
                writes.append({
                    "epoch":epoch, "window":epoch//8, "tick":event["tick"],
                    "pc":pc, "value":r0, "source_tainted":tainted,
                    "native_attempt_ordinal": cumulative_attempts,
                    "latest_native_source_read_epoch":
                        last_read_epoch if tainted else None,
                })
            elif result != "Executed":
                raise ValueError("unregistered native failure or invented instruction")
            if stopped or result == "DrawDeferred":
                # Deferred Draw is charged, but retains PC for the next native retry.
                continue
            r0, tainted = propagate_r0(op, r0, tainted, sensor)
            if name == "Branch":
                branch = op["Branch"]
                test = r0 if branch["test"] == 0 else 8
                pc = branch["nonzero"] if test != 0 else branch["zero"]
            else:
                pc = op[name]["next"]
        if (state["compute_pool"] != current_pool
                or state["processes"]["1"]["credits"] != current_credits
                or state["processes"]["1"]["stopped"] != stopped
                or not 0 <= pc < len(code)
                or state["processes"]["1"]["pc"] != pc
                or state["processes"]["1"]["code"]["nodes"] != code
                or not state.get("pool_paid_draw", False)
                or state.get("recoverable_draw_shortage",False) != ("-retry-" in mode)
                or state["cells"]["7"] != sensor
                or (writes and state["cells"].get("9") != writes[-1]["value"])):
            raise ValueError("native actor state or energy pool not accounted for")
        if (current_credits + current_pool
                != 64 + initial_pool + ambient_events*8 - cumulative_attempts):
            raise ValueError("native compute inventory is not resource-conserving")
        previous = state
    if cumulative_attempts != sum(attempts) or ambient_events != (23 if ambient else 0):
        raise ValueError("unaccounted registered native attempts or external ambient supply")
    return {
        "native_attempts": cumulative_attempts,
        "native_attempts_by_window": count_by_window,
        "native_source_Read_events": reads,
        "native_CellWritten_events": len(writes),
        "native_successful_DrawCompute_events": len(drawn),
        "native_DrawCompute_by_window": [
            sum(d["window"] == i for d in drawn) for i in range(3)
        ],
        "draws": drawn,
        "pool_paid_events": pool_paid_events,
        "deferred": deferred,
        "writes": writes,
        "final_positive_pool_but_actor_not_runnable": (
            current_pool > 0 and (current_credits == 0 or stopped)
        ),
    }


def source_pairs(low: dict, high: dict, window: int) -> list[tuple[dict, dict]]:
    a = [w for w in low["writes"]
         if w["window"] == window and w["source_tainted"]
         and w["latest_native_source_read_epoch"] is not None
         and w["latest_native_source_read_epoch"] // 8 == window]
    b = [w for w in high["writes"]
         if w["window"] == window and w["source_tainted"]
         and w["latest_native_source_read_epoch"] is not None
         and w["latest_native_source_read_epoch"] // 8 == window]
    return [
        (x, y) for x in a for y in b
        if x["pc"] == y["pc"] and y["value"] - x["value"] == 5
    ]


def evaluate_pair(low: dict, high: dict) -> dict:
    pairs2 = source_pairs(low, high, 1)
    pairs3 = source_pairs(low, high, 2)
    basic = bool(pairs2 and pairs3)

    def has_draw_before(run: dict, write: dict) -> bool:
        return any(d["window"] == 2 and d["tick"] < write["tick"]
                   for d in run["draws"])

    # Frozen primary requires successful native resource uptake *before* the
    # qualifying W3 output AND an actual native attempt ordinal >64 in each
    # low/high world, not merely a W3 calendar label. Choose the first
    # chronologically eligible original pair, not a post-result favored seed.
    qualified3 = sorted((
        (x, y) for x, y in pairs3
        if x["native_attempt_ordinal"] > 64
        and y["native_attempt_ordinal"] > 64
        and has_draw_before(low, x)
        and has_draw_before(high, y)
    ), key=lambda a: (max(a[0]["tick"], a[1]["tick"]), a[0]["pc"]))
    first3 = qualified3[0] if qualified3 else None
    primary_a = basic and first3 is not None

    def has_draw_same_window_before(run: dict, write: dict, w: int) -> bool:
        return any(d["window"] == w and d["tick"] < write["tick"]
                   for d in run["draws"])

    stricter_b = primary_a and all(
        any(has_draw_same_window_before(low, x, w)
            and has_draw_same_window_before(high, y, w)
            for x, y in matches)
        for w, matches in ((1, pairs2), (2, pairs3))
    )
    return {
        "paired_source_sensitive_both_windows": basic,
        "primary_A_coupled_native_pool_and_feedback": primary_a,
        "secondary_B_fresh_native_pool_each_late_window": stricter_b,
        "first_W2_source_pair": pairs2[0] if pairs2 else None,
        "first_W3_source_pair": first3,
    }


def audit(path: Path, revision: str) -> dict:
    db = sqlite3.connect(f"{path.resolve().as_uri()}?mode=ro", uri=True)
    try:
        worlds = defaultdict(dict)
        counts = Counter()
        witnesses = []
        n_runs = 0
        sha_count = 0
        for (rid, label, original_revision, initial_sha, raw_initial,
             raw_actions, epochs, quota, raw_attempts, final_sha, raw_final) in db.execute(
            "SELECT id,condition,source_revision,initial_sha256,initial_json,"
            "actions_json,epoch_count,quota,attempts_json,final_sha256,final_json "
            "FROM epoch_runs ORDER BY id"
        ):
            prefix, mode = label.split("/", 1)
            seed = int(prefix.removeprefix("seed="))
            if (not FIRST <= seed < FIRST + N or mode not in MODES
                    or mode in worlds[seed] or original_revision != revision):
                raise ValueError("wrong source revision or original eight-mode seed population")
            if (analyze.digest(raw_initial) != initial_sha
                    or analyze.digest(raw_final) != final_sha
                    or epochs != EPOCHS or quota != QUOTA):
                raise ValueError("frozen original run hash, epoch or quota changed")
            initial = json.loads(raw_initial)
            ambient = "-ambient-" in mode
            sensor = 3 if mode.endswith("-low") else 8
            if (set(initial["processes"]) != {"1"}
                    or initial["processes"]["1"]["code"]["nodes"] != expected_code(seed,mode)
                    or initial["processes"]["1"]["credits"] != 64
                    or initial["compute_pool"] != (96 if ambient else 0)
                    or initial["cells"] != {"7": sensor}
                    or initial["storage_limit"] != 4
                    or initial["max_nodes_per_process"] != 32
                    or initial["total_storage_limit"] != 64
                    or not initial["strict_accounting"]
                or not initial.get("pool_paid_draw", False)
                or initial.get("recoverable_draw_shortage",False) != ("-retry-" in mode)):
                raise ValueError("native frozen random CIL or world inputs changed")
            if (len(initial["events"]) != 3
                    or [op_name(x["kind"]) for x in initial["events"]]
                    != ["Loaded","Intervened","Intervened"]
                    or any(x["origin"] != "External" for x in initial["events"])):
                raise ValueError("initial external world setup provenance invalid")
            actions = json.loads(raw_actions)
            expected_epochs = list(range(1,EPOCHS)) if ambient else []
            if [x["epoch"] for x in actions] != expected_epochs:
                raise ValueError("missing or unregistered +8 ambient schedule")
            if any(set(x["action"]) != {"SetComputePool"} for x in actions):
                raise ValueError("host process-specific assistance was introduced")
            attempts = json.loads(raw_attempts)
            if len(attempts) != EPOCHS or any(not 0 <= x <= QUOTA for x in attempts):
                raise ValueError("native attempt accounting outside protocol")
            snapshots = db.execute(
                "SELECT epoch,state_sha256,state_json FROM epoch_snapshots "
                "WHERE run_id=? ORDER BY epoch", (rid,)
            ).fetchall()
            if len(snapshots) != EPOCHS or [row[0] for row in snapshots] != list(range(EPOCHS)):
                raise ValueError("invalid source-pinned original SHA256 epoch set")
            data = trace_run(initial, snapshots, actions, attempts, mode)
            if json.loads(snapshots[-1][2]) != json.loads(raw_final):
                raise ValueError("original final CIL state does not match snapshot24")
            worlds[seed][mode] = data
            sha_count += EPOCHS
            n_runs += 1
            counts[f"{mode}.W3_native_attempt_worlds"] += int(data["native_attempts_by_window"][2] > 0)
            counts[f"{mode}.W3_native_source_tainted_Write_worlds"] += int(any(
                w["window"] == 2 and w["source_tainted"]
                and w["latest_native_source_read_epoch"] is not None
                and w["latest_native_source_read_epoch"] // 8 == 2
                for w in data["writes"]
            ))
            counts[f"{mode}.native_DrawCompute_worlds"] += int(bool(data["draws"]))
            counts[f"{mode}.native_DrawCompute_events"] += len(data["draws"])
            counts[f"{mode}.pool_paid_Draw_worlds"] += int(bool(data["pool_paid_events"]))
            counts[f"{mode}.pool_paid_Draw_attempts"] += len(data["pool_paid_events"])
            counts[f"{mode}.native_DrawDeferred_events"] += len(data["deferred"])
            counts[f"{mode}.native_DrawDeferred_worlds"] += int(bool(data["deferred"]))
            counts[f"{mode}.native_Read_events"] += data["native_source_Read_events"]
            counts[f"{mode}.native_Write_events"] += data["native_CellWritten_events"]
            counts[f"{mode}.final_positive_pool_deadlock_worlds"] += int(
                data["final_positive_pool_but_actor_not_runnable"]
            )
        if (n_runs != N*len(MODES) or len(worlds) != N
                or sha_count != N*len(MODES)*EPOCHS
                or any(set(modes) != set(MODES) for modes in worlds.values())):
            raise ValueError("incomplete independent seed ecology or native SHA256 cohort")
        per_seed = {}
        primary = Counter()
        w3 = Counter()
        witness = []
        paired_a = {t: Counter() for t in ("broad", "short")}
        paired_b = {t: Counter() for t in ("broad", "short")}
        for seed in range(FIRST,FIRST+N):
            arms = worlds[seed]
            per_seed[str(seed)] = {}
            for topo in ("broad", "short"):
                props = {}
                for policy in ("retry", "legacy"):
                    low = arms[f"{topo}-ambient-{policy}-intact-low"]
                    high = arms[f"{topo}-ambient-{policy}-intact-high"]
                    candidate = evaluate_pair(low,high)
                    has_a = candidate["primary_A_coupled_native_pool_and_feedback"]
                    has_b = low["native_attempts_by_window"][2] > 0
                    props[policy] = (has_a,has_b)
                    primary[f"{topo}-{policy}"] += int(has_a)
                    w3[f"{topo}-{policy}"] += int(has_b)
                    per_seed[str(seed)][f"{topo}-{policy}"] = {
                        "native_W2_W3_source_feedback_and_W3_Draw": has_a,
                        "native_W3_charged_activity": has_b,
                        "native_pool_paid_Draw_events": len(low["pool_paid_events"]),
                        "native_DrawDeferred_events": len(low["deferred"]),
                    }
                    if policy == "legacy" and (low["deferred"] or high["deferred"]):
                        raise ValueError("legacy shortage unexpectedly executed a nonfatal retry")
                    if has_a and len(witness) < 18:
                        witness.append({"seed":seed,"topology":topo,"policy":policy,**candidate})
                for index, result in enumerate(("a","b")):
                    old = props["legacy"][index]
                    new = props["retry"][index]
                    counts_map = paired_a[topo] if result=="a" else paired_b[topo]
                    counts_map["retry_only"] += int(new and not old)
                    counts_map["legacy_only"] += int(old and not new)
                    counts_map["both"] += int(old and new)
                    counts_map["neither"] += int(not old and not new)
                for sensor in ("low", "high"):
                    closed = arms[f"{topo}-closed-retry-intact-{sensor}"]
                    if closed["draws"] or closed["pool_paid_events"]:
                        raise ValueError("closed pool created native Draw energy")
            for policy in ("retry","legacy"):
                for sensor in ("low","high"):
                    ablated = arms[f"broad-ambient-{policy}-draw-ablated-{sensor}"]
                    if (ablated["draws"] or ablated["pool_paid_events"]
                            or ablated["native_attempts"] > 64):
                        raise ValueError("Draw-disabled actor exceeded starting 64 local credits")
        def wilson(value):
            return [round(x,6) for x in wilson_95(value,N)]
        primary_a = {
            t: {
                "retry_worlds":primary[f"{t}-retry"],
                "legacy_worlds":primary[f"{t}-legacy"],
                "denominator_each":N,
                "retry_wilson95":wilson(primary[f"{t}-retry"]),
                "legacy_wilson95":wilson(primary[f"{t}-legacy"]),
                "paired_discordance":dict(paired_a[t])
            } for t in ("broad","short")
        }
        primary_b = {
            t: {
                "retry_W3_active_worlds":w3[f"{t}-retry"],
                "legacy_W3_active_worlds":w3[f"{t}-legacy"],
                "denominator_each":N,
                "paired_discordance":dict(paired_b[t]),
                "retry_worlds_with_actual_pool_paid_Draw":counts[
                    f"{t}-ambient-retry-intact-low.pool_paid_Draw_worlds"
                ],
            } for t in ("broad","short")
        }
        return {
            "study":"CL-1 Study77 prospective generic native nonfatal Draw causal ecology",
            "source_revision":revision,
            "original_independent_seed_genomes":N,
            "matched_native_runs":n_runs,
            "SHA256_verified_original_snapshots":sha_count,
            "primary_A_W2W3_native_feedback_with_W3_Draw":primary_a,
            "primary_B_W3_resource_access":primary_b,
            "registered_secondary":dict(sorted(counts.items())),
            "all_original_seed_outcomes":per_seed,
            "native_joint_witnesses":witness,
            "host_actor_specific_starter":0,
            "world_pool_paid_entry_is":"EXTERNAL GENERAL SUBSTRATE LAW",
            "recoverable_resource_retry_is":"EXTERNAL GENERAL SUBSTRATE LAW, NOT NATIVE REPAIR",
            "life_claim":"NONE",
        }
    finally:
        db.close()


def main() -> None:
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument("sqlite",type=Path)
    parser.add_argument("--expected-revision",required=True)
    parser.add_argument("--json",type=Path,required=True)
    args=parser.parse_args()
    report=audit(args.sqlite,args.expected_revision)
    args.json.write_text(json.dumps(report,indent=2)+"\n",encoding="utf-8")
    print(json.dumps({
        "study":report["study"],
        "seed_genomes":report["original_independent_seed_genomes"],
        "SHA256_epochs":report["SHA256_verified_original_snapshots"],
        "primary_A":report["primary_A_W2W3_native_feedback_with_W3_Draw"],
        "primary_B":report["primary_B_W3_resource_access"],
        "life_claim":report["life_claim"],
    },indent=2))


if __name__=="__main__":
    main()
