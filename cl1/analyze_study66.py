#!/usr/bin/env python3
"""Study66 independently audit random source feedback plus native pool harvesting.

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

FIRST, N, EPOCHS, QUOTA = 4_400_000, 256, 24, 12
A, B, MASK = 6364136223846793005, 1442695040888963407, (1 << 64) - 1
MODES = (
    "ambient-low", "ambient-high", "closed-low", "closed-high",
    "ambient-draw-ablated-low", "ambient-draw-ablated-high",
    "ambient-read-ablated-low", "ambient-read-ablated-high",
)


def random_state(state: int) -> int:
    return (state * A + B) & MASK


def expected_code(seed: int, mode: str) -> list:
    if not FIRST <= seed < FIRST + N or mode not in MODES:
        raise ValueError("unregistered Study66 seed or intervention arm")
    read_ablated = "read-ablated" in mode
    draw_ablated = "draw-ablated" in mode
    state = seed
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
            op = ({"Const": {"dst": 0, "value": 0, "next": nxt}}
                  if read_ablated
                  else {"Read": {"dst": 0, "address": 2, "next": nxt}})
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
            op = {"Branch": {
                "test": 0, "nonzero": 3 if pc == 3 else pc - 1,
                "zero": nxt,
            }}
        else:
            op = ({"Const": {"dst": 1, "value": 8, "next": nxt}}
                  if draw_ablated
                  else {"DrawCompute": {"amount": 1, "next": nxt}})
        code.append(op)
    code.append({"Branch": {"test": 1, "nonzero": 3, "zero": 3}})
    if len(code) != 28:
        raise ValueError("Study66 original code length changed")
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
    ambient = mode.startswith("ambient")
    sensor = 3 if mode.endswith("-low") else 8
    initial_pool = 48 if ambient else 0
    current_pool = initial_pool
    current_credits = 16
    r0, tainted, last_read_epoch = 0, False, None
    pc, stopped = 0, False
    drawn = []
    writes = []
    reads = 0
    count_by_window = [0, 0, 0]
    cumulative_attempts = 0
    ambient_events = 0
    previous = initial
    for expected_epoch, (epoch, digest, raw) in enumerate(snapshots):
        if epoch != expected_epoch or analyze.digest(raw) != digest:
            raise ValueError("native Study66 snapshot hash/order corrupted")
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
                elif result == "Trapped":
                    if (current_pool >= 8
                            or "insufficient world compute pool" not in kind["Trapped"]):
                        raise ValueError("native DrawCompute failed for an undeclared reason")
                    stopped = True
                else:
                    raise ValueError("incorrect successful/trapped native compute provenance")
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
                    "latest_native_source_read_epoch":
                        last_read_epoch if tainted else None,
                })
            elif result != "Executed":
                raise ValueError("unregistered native failure or invented instruction")
            if stopped:
                # A trapped Draw is charged and permanently stops actor; no PC change.
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
                or state["cells"]["7"] != sensor
                or (writes and state["cells"].get("9") != writes[-1]["value"])):
            raise ValueError("native actor state or energy pool not accounted for")
        if (current_credits + current_pool
                != 16 + initial_pool + ambient_events*8 - cumulative_attempts):
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
    first3 = min(pairs3, key=lambda a: max(a[0]["tick"], a[1]["tick"])) if pairs3 else None
    def has_draw_before(run: dict, write: dict) -> bool:
        return any(d["tick"] <= write["tick"] for d in run["draws"])
    primary_a = basic and first3 is not None and (
        has_draw_before(low, first3[0]) and has_draw_before(high, first3[1])
    )
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
            ambient = mode.startswith("ambient")
            sensor = 3 if mode.endswith("-low") else 8
            if (set(initial["processes"]) != {"1"}
                    or initial["processes"]["1"]["code"]["nodes"] != expected_code(seed,mode)
                    or initial["processes"]["1"]["credits"] != 16
                    or initial["compute_pool"] != (48 if ambient else 0)
                    or initial["cells"] != {"7": sensor}
                    or initial["storage_limit"] != 4
                    or initial["max_nodes_per_process"] != 32
                    or initial["total_storage_limit"] != 64
                    or not initial["strict_accounting"]):
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
            counts[f"{mode}.native_Read_events"] += data["native_source_Read_events"]
            counts[f"{mode}.native_Write_events"] += data["native_CellWritten_events"]
            counts[f"{mode}.final_positive_pool_deadlock_worlds"] += int(
                data["final_positive_pool_but_actor_not_runnable"]
            )
        if (n_runs != N*len(MODES) or len(worlds) != N
                or sha_count != N*len(MODES)*EPOCHS
                or any(set(modes) != set(MODES) for modes in worlds.values())):
            raise ValueError("incomplete independent seed ecology or native SHA256 cohort")
        for seed, arms in worlds.items():
            for ablation in ("ambient-read-ablated", "ambient-draw-ablated"):
                low = arms[f"{ablation}-low"]
                high = arms[f"{ablation}-high"]
                if ablation.endswith("read-ablated"):
                    if low["native_source_Read_events"] or high["native_source_Read_events"]:
                        raise ValueError("Read-ablation negative still has native source events")
                    ltrace = [(w["tick"],w["pc"],w["value"]) for w in low["writes"]]
                    htrace = [(w["tick"],w["pc"],w["value"]) for w in high["writes"]]
                    if ltrace != htrace:
                        raise ValueError("Read-ablated outputs falsely depend on sensor input")
                if ablation.endswith("draw-ablated") and (
                    low["draws"] or high["draws"]
                ):
                    raise ValueError("Draw-ablation negative has successful native DrawCompute")
            for name in ("closed-low","closed-high"):
                if arms[name]["draws"]:
                    raise ValueError("closed pool supported a native DrawCompute")
            pair = evaluate_pair(arms["ambient-low"], arms["ambient-high"])
            counts["ambient.primary_A_coupled_native_feedback_worlds"] += int(
                pair["primary_A_coupled_native_pool_and_feedback"])
            counts["ambient.secondary_B_native_pool_each_W2_W3_worlds"] += int(
                pair["secondary_B_fresh_native_pool_each_late_window"])
            counts["ambient.source_sensitive_W2_and_W3_without_energy_gate"] += int(
                pair["paired_source_sensitive_both_windows"])
            if (pair["primary_A_coupled_native_pool_and_feedback"]
                    and len(witnesses) < 20):
                witnesses.append({
                    "seed":seed,
                    **pair,
                    "native_low_draws":arms["ambient-low"]["draws"][:6],
                    "native_high_draws":arms["ambient-high"]["draws"][:6],
                })
        a = counts["ambient.primary_A_coupled_native_feedback_worlds"]
        b = counts["ambient.secondary_B_native_pool_each_W2_W3_worlds"]
        if b > a:
            raise ValueError("strict resource/feedback gate exceeds broader primary A")
        return {
            "study":"CL-1 Study66 native stochastic sensor feedback and compute uptake",
            "source_revision":revision,
            "new_independent_seed_genomes":N,
            "matched_native_runs":n_runs,
            "SHA256_verified_original_snapshots":sha_count,
            "primary_A_coupled_random_native_read_write_and_DrawCompute":{
                "worlds":a,"denominator":N,
                "wilson95":[round(x,6) for x in wilson_95(a,N)],
            },
            "secondary_B_recurrent_W2_W3_native_Draw_and_feedback":{
                "worlds":b,"denominator":N,
                "wilson95":[round(x,6) for x in wilson_95(b,N)],
            },
            "registered_secondary":dict(sorted(counts.items())),
            "source_and_credit_joint_witnesses":witnesses,
            "external_ambient_pool_energy":"EXTERNAL +8 per epoch1..23; native Draw merely transfers it",
            "host_actor_starter_actions":0,
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
        "independent_seed_genomes":report["new_independent_seed_genomes"],
        "hashes":report["SHA256_verified_original_snapshots"],
        "primary":report["primary_A_coupled_random_native_read_write_and_DrawCompute"],
        "strict_secondary":report["secondary_B_recurrent_W2_W3_native_Draw_and_feedback"],
        "life_claim":"NONE",
    },indent=2))


if __name__=="__main__":
    main()
