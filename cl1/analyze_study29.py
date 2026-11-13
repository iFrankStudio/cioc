#!/usr/bin/env python3
"""Study 29 independent audit: pairwise two-source native CIL address arithmetic.

A strictly observed externally induced address change is NOT endogenous life.
"""
from __future__ import annotations

import argparse
import copy
import json
import sqlite3
from collections import Counter, defaultdict
from pathlib import Path

import analyze
from analyze_study02 import wilson_95
from analyze_study14 import simulated_state

FIRST = 2_400_000
SEEDS = 256
EPOCHS = 8
QUOTA = 12
LABELS = (
    "control", "sham", "mutate-g1", "mutate-g2", "mutate-both",
    "exact-original-g2", "zero-credit",
)


def schedule(arm: str, initial: dict) -> list[dict]:
    def at(action: dict) -> dict:
        return {"epoch": 0, "action": action}

    def edited(graphs: tuple[int, ...], identical: bool = False) -> list[dict]:
        actions = []
        for graph in graphs:
            obj = copy.deepcopy(initial["graph_objects"][str(graph)])
            if not identical:
                obj["nodes"][0] = "Halt"
            actions.extend([
                at({"DropGraph": {"graph": graph}}),
                at({"PutGraph": {"graph": graph, "object": obj}}),
            ])
        return actions
    return {
        "control": [],
        "sham": [at({"DropGraph": {"graph": 0}})],
        "mutate-g1": edited((1,)),
        "mutate-g2": edited((2,)),
        "mutate-both": edited((1, 2)),
        "exact-original-g2": edited((2,), identical=True),
        "zero-credit": [
            at({"SetCredits": {"process": id, "credits": 0}})
            for id in (1, 2, 3)
        ],
    }[arm]


def strict_dual_output(
    events: list[dict], tag1: int, tag2: int, address: int
) -> dict:
    """Strict same-actor successful graph1+graph2 reads before native output."""
    reads = {1: [], 2: []}
    outputs = []
    for event in events:
        kind = event["kind"]
        if not isinstance(kind, dict) or analyze.internal_process(event) != 1:
            continue
        if event["process"] != 1:
            raise ValueError("actor/origin mismatch")
        payload = kind.get("GraphTagRead")
        if isinstance(payload, dict):
            gid = payload.get("graph")
            if gid in (1, 2):
                tag = tag1 if gid == 1 else tag2
                if (payload.get("owner") == gid and payload.get("node") == 0
                        and payload.get("tag") == tag):
                    reads[gid].append(int(event["tick"]))
        write = kind.get("CellWritten")
        if isinstance(write, dict) and write.get("key") == address:
            outputs.append((int(event["tick"]), int(write["value"])))
    good = (
        len(reads[1]) == 1
        and len(reads[2]) == 1
        and len(outputs) == 1
        and reads[1][0] < outputs[0][0]
        and reads[2][0] < outputs[0][0]
    )
    return {
        "qualified": bool(good),
        "source1_reads": len(reads[1]),
        "source2_reads": len(reads[2]),
        "target_writes": len(outputs),
        "value": outputs[0][1] if good else None,
    }


def audit(path: Path) -> dict:
    db = sqlite3.connect(f"{path.resolve().as_uri()}?mode=ro", uri=True)
    try:
        rows = db.execute(
            "SELECT id,source_revision,condition,initial_sha256,initial_json,"
            "actions_json,epoch_count,quota,attempts_json,final_sha256,final_json "
            "FROM epoch_runs ORDER BY id"
        ).fetchall()
        if len(rows) != SEEDS * len(LABELS):
            raise ValueError(f"not the frozen 256x7 matched cohort: {len(rows)}")
        frames = defaultdict(list)
        for run_id, epoch, sha, raw in db.execute(
            "SELECT run_id,epoch,state_sha256,state_json "
            "FROM epoch_snapshots ORDER BY run_id,epoch"
        ):
            if epoch < 0 or epoch >= EPOCHS or analyze.digest(raw) != sha:
                raise ValueError("missing/altered SQLite epoch snapshot")
            frames[run_id].append((epoch, sha, raw))
        worlds = defaultdict(dict)
        initials = {}
        revisions = set()
        for (run_id, revision, condition, initial_sha, initial_raw,
             actions_raw, epochs, quota, attempts_raw, final_sha, final_raw) in rows:
            prefix, arm = condition.split("/", 1)
            seed = int(prefix.removeprefix("seed="))
            if seed not in range(FIRST, FIRST + SEEDS) or arm not in LABELS:
                raise ValueError("unregistered seed-world/arm")
            if arm in worlds[seed]:
                raise ValueError("duplicate arm")
            if not revision or epochs != EPOCHS or quota != QUOTA:
                raise ValueError("unregistered time budget or source revision")
            if (analyze.digest(initial_raw) != initial_sha
                    or analyze.digest(final_raw) != final_sha):
                raise ValueError("initial/final SHA256 differs")
            if seed in initials and initials[seed] != initial_sha:
                raise ValueError("unmatched same-seed initial state")
            initials[seed] = initial_sha
            revisions.add(revision)
            initial, final = json.loads(initial_raw), json.loads(final_raw)
            if (initial["total_storage_limit"] != 192
                    or not initial["strict_accounting"]
                    or initial["storage_limit"] != 32
                    or initial["max_nodes_per_process"] != 40):
                raise ValueError("unregistered storage capacity")
            for gid in (1, 2, 3):
                graph = initial["graph_objects"].get(str(gid))
                if (graph is None or graph["owner"] != gid or len(graph["nodes"]) != 1
                        or initial["processes"][str(gid)]["credits"] != 47):
                    raise ValueError("incorrect common ecology scaffold")
            if not isinstance(initial["graph_objects"]["2"]["nodes"][0], dict) or (
                "GraphNew" not in initial["graph_objects"]["2"]["nodes"][0]
            ):
                raise ValueError("graph2 prefork source is not the registered GraphNew tag11")
            if json.loads(actions_raw) != schedule(arm, initial):
                raise ValueError(f"invalid source mutation/intervention schedule: {condition}")
            attempts = json.loads(attempts_raw)
            if len(attempts) != EPOCHS or any(
                not isinstance(x, int) or not 0 <= x <= QUOTA for x in attempts
            ):
                raise ValueError("bad epoch attempted instruction counts")
            recorded = frames[run_id]
            if len(recorded) != EPOCHS or any(
                epoch != index for index, (epoch, _, _) in enumerate(recorded)
            ):
                raise ValueError("incomplete/out of order epoch snapshots")
            if recorded[-1][1:] != (final_sha, final_raw):
                raise ValueError("last epoch not equal to final state")
            epoch_events = []
            previous = initial
            for epoch, _, raw in recorded:
                snap = json.loads(raw)
                if snap["events"][: len(previous["events"])] != previous["events"]:
                    raise ValueError("native event history mutated")
                if analyze.occupied_units(snap) > 192:
                    raise ValueError("strict resource accounting exceeded")
                epoch_events.append(snap["events"][len(previous["events"]):])
                previous = snap
            postfork = [e for group in epoch_events for e in group]
            if arm == "zero-credit" and (
                any(attempts)
                or any(analyze.internal_process(e) is not None for e in postfork)
            ):
                raise ValueError("zero-credit null unexpectedly executed")
            modified = 2 if arm == "mutate-both" else int(
                arm in ("mutate-g1", "mutate-g2", "exact-original-g2")
            )
            for name in ("DropGraph", "PutGraph"):
                observed = sum(
                    e["origin"] == "External"
                    and isinstance(e["kind"], dict)
                    and name in str(e["kind"].get("Intervened", ""))
                    for e in postfork
                )
                if observed != modified:
                    raise ValueError(f"external source code intervention absent: {condition}")
            if arm == "sham" and not any(
                e["origin"] == "External" and isinstance(e["kind"], dict)
                and "InterventionRejected" in e["kind"] for e in postfork
            ):
                raise ValueError("sham not a rejected intervention")
            worlds[seed][arm] = {
                "final": final,
                "events0": epoch_events[0],
                "attempts": attempts,
            }
        if (len(worlds) != SEEDS or len(revisions) != 1
                or any(set(arms) != set(LABELS) for arms in worlds.values())):
            raise ValueError("cohort incomplete or mixed source revisions")
        counts = Counter()
        witnesses = []
        for seed in sorted(worlds):
            arms = worlds[seed]
            if simulated_state(arms["control"]["final"]) != simulated_state(
                arms["sham"]["final"]
            ):
                raise ValueError("rejected sham altered simulated state")
            if simulated_state(arms["control"]["final"]) != simulated_state(
                arms["exact-original-g2"]["final"]
            ):
                raise ValueError("exact source2 reinjection altered simulated state")
            pattern = {
                "control": (11, 11, 22),
                "mutate-g1": (19, 11, 30),
                "mutate-g2": (11, 19, 30),
                "mutate-both": (19, 19, 38),
            }
            observed = {
                arm: strict_dual_output(arms[arm]["events0"], *params)
                for arm, params in pattern.items()
            }
            for arm in pattern:
                counts[f"{arm}.expected_dual_source_output"] += observed[arm]["qualified"]
                counts[f"{arm}.first_source_read"] += observed[arm]["source1_reads"] > 0
                counts[f"{arm}.second_source_read"] += observed[arm]["source2_reads"] > 0
            full = all(observed[arm]["qualified"] for arm in pattern)
            if full:
                values = {observed[arm]["value"] for arm in pattern}
                full = len(values) == 1
            counts["paired_prespecified_two_graph_additive_output"] += full
            if full:
                witnesses.append({
                    "seed": seed,
                    "actor": 1,
                    "epoch": 0,
                    "source1_tags": [11, 19],
                    "source2_tags": [11, 19],
                    "output_addresses": {
                        arm: target[2] for arm, target in pattern.items()
                    },
                    "same_numeric_value": observed["control"]["value"],
                })
        metrics = {
            key: {
                "seed_worlds": n,
                "denominator": SEEDS,
                "wilson_95": [round(x, 6) for x in wilson_95(n, SEEDS)],
            }
            for key, n in sorted(counts.items())
        }
        return {
            "study": "CL-1 Study 29",
            "source_revision": next(iter(revisions)),
            "independent_seed_worlds": SEEDS,
            "matched_runs": len(rows),
            "hashed_epoch_snapshots": sum(len(v) for v in frames.values()),
            "primary_dual_source_additive_function": metrics[
                "paired_prespecified_two_graph_additive_output"
            ],
            "registered_metrics": metrics,
            "strict_matched_witnesses": witnesses,
            "function_source": "investigator scaffold + stochastic CIL continuation",
            "source_dataflow_limit": "paired content sensitivity, not register taint",
            "endogenous_design": "NOT EVALUABLE",
            "native_self_repair": "NOT EVALUABLE",
            "life_claim": "NONE",
        }
    finally:
        db.close()


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("sqlite", type=Path)
    parser.add_argument("--json", type=Path, required=True)
    args = parser.parse_args()
    report = audit(args.sqlite)
    args.json.write_text(json.dumps(report, indent=2) + "\n", encoding="utf-8")
    print(json.dumps({
        "study": report["study"],
        "worlds": report["independent_seed_worlds"],
        "runs": report["matched_runs"],
        "snapshots": report["hashed_epoch_snapshots"],
        "dual_source": report["primary_dual_source_additive_function"],
        "self_repair": report["native_self_repair"],
    }))


if __name__ == "__main__":
    main()
