#!/usr/bin/env python3
"""Study34 retrospective native graph2 source exposure, not life certification.

IMPORTANT: GraphReadTag.graph and .node are register *indices* in the CIL VM.
The resolved graph id and node id are recorded in successful GraphTagRead events.
A static equality comparison between an opcode register index and graph id is invalid.
"""
from __future__ import annotations

import argparse
import json
import sqlite3
from collections import Counter, defaultdict
from pathlib import Path

import analyze
from analyze_study33 import FIRST, LABELS, SEEDS, expected_program_continuations


def source2_read_events(state: dict) -> list[dict]:
    """Independently select successful, attributed, resolved native source events."""
    found = []
    processes = state["processes"]
    for event in state["events"]:
        kind = event["kind"]
        if not isinstance(kind, dict) or "GraphTagRead" not in kind:
            continue
        read = kind["GraphTagRead"]
        actor = event["process"]
        if (read.get("graph"), read.get("owner"), read.get("node")) != (2, 2, 0):
            continue
        if event.get("origin") != {"Internal": actor}:
            raise ValueError("unattributed source2 event")
        process = processes[str(actor)]
        pc = event["pc"]
        if not isinstance(pc, int) or not 0 <= pc < len(process["code"]["nodes"]):
            raise ValueError("read has invalid code PC")
        opcode = process["code"]["nodes"][pc]
        if not isinstance(opcode, dict) or "GraphReadTag" not in opcode:
            raise ValueError("native source event lacks corresponding GraphReadTag")
        if read["tag"] not in (11, 19):
            raise ValueError("unexpected source2 opcode tag")
        found.append({
            "actor": actor, "tick": event["tick"], "pc": pc,
            "resolved_graph": 2, "resolved_node": 0,
            "opcode_graph_register": opcode["GraphReadTag"]["graph"],
            "opcode_node_register": opcode["GraphReadTag"]["node"],
            "tag": read["tag"],
        })
    return found


def native_address_lineage(initial: dict, final: dict, arm: str) -> dict:
    """Read/Draw/Write exposure through native actor1 register def-use.

    This diagnostic is broader than Study33's strict target-address endpoint.
    A successful non-target write does NOT satisfy that registered endpoint.
    """
    program = initial["processes"]["1"]["code"]["nodes"]
    provenance: dict[int, bool] = {}
    seen_draw = False
    first_draw = None
    reads_before_draw = []
    reads_after_draw = []
    writes = []
    actor_attempts = 0
    target = 19 if arm.startswith("mutate-g2") else 11
    for event in final["events"][len(initial["events"]):]:
        if analyze.internal_process(event) != 1:
            continue
        actor_attempts += 1
        tick = event["tick"]
        pc = event["pc"]
        if not isinstance(pc, int) or not 0 <= pc < len(program):
            raise ValueError("actor1 native PC out of registered initial code")
        op = program[pc]
        kind = event["kind"]
        if op == "Halt" or isinstance(kind, dict) and "Trapped" in kind:
            continue
        if not isinstance(op, dict) or len(op) != 1:
            raise ValueError("unrecognized native source opcode")
        name, args = next(iter(op.items()))
        if name == "DrawCompute":
            if kind != "Executed":
                raise ValueError("unexpected native DrawCompute result")
            seen_draw = True
            if first_draw is None:
                first_draw = tick
        elif name == "GraphReadTag":
            if not isinstance(kind, dict) or "GraphTagRead" not in kind:
                raise ValueError("successful GraphReadTag without resolved event")
            read = kind["GraphTagRead"]
            qualifies = (
                read["graph"] == 2 and read["owner"] == 2
                and read["node"] == 0 and read["tag"] == target
            )
            provenance[args["dst"]] = qualifies
            if qualifies:
                (reads_after_draw if seen_draw else reads_before_draw).append(tick)
        elif name == "Add":
            if kind != "Executed":
                raise ValueError("unexpected Add success event")
            provenance[args["dst"]] = (
                provenance.get(args["lhs"], False)
                or provenance.get(args["rhs"], False)
            )
        elif name == "Write":
            if not isinstance(kind, dict) or "CellWritten" not in kind:
                raise ValueError("unexpected successful native Write")
            if provenance.get(args["address"], False):
                written = kind["CellWritten"]
                writes.append({
                    "tick": tick, "key": written["key"], "value": written["value"],
                    "after_draw": seen_draw,
                    "after_original_47_attempts": actor_attempts > 47,
                    "matches_registered_source_address": written["key"] == target,
                })
        elif name in ("Const", "Read", "GraphNew", "GraphCopyNode"):
            provenance[args["dst"]] = False
        elif name == "GraphCopyFrom":
            provenance[args["node"]] = False
        elif name not in ("GraphGrantRead", "Branch", "ReturnCompute"):
            raise ValueError(f"unhandled Study34 actor1 source effect: {name}")
    return {
        "first_draw_tick": first_draw,
        "source_reads_before_first_draw": reads_before_draw,
        "source_reads_after_first_draw": reads_after_draw,
        "native_source_address_writes": writes,
        "registered_source_address_writes": [
            w for w in writes if w["matches_registered_source_address"]
        ],
    }


def audit(path: Path) -> dict:
    connection = sqlite3.connect(f"{path.resolve().as_uri()}?mode=ro", uri=True)
    try:
        rows = connection.execute(
            "SELECT condition,initial_sha256,initial_json,final_sha256,final_json,"
            "source_revision FROM epoch_runs ORDER BY id"
        )
        arms: dict[int, set[str]] = defaultdict(set)
        initial_hashes = {}
        revisions = set()
        actor_worlds = Counter()
        actor_events = Counter()
        source_tags = Counter()
        changed_codes = 0
        count = 0
        gates = Counter()
        witness_examples = []
        per_seed = defaultdict(dict)
        for condition, initial_sha, raw, final_sha, final_raw, revision in rows:
            if not condition.startswith("seed=") or "/" not in condition:
                raise ValueError("unregistered arm")
            prefix, arm = condition.split("/", 1)
            seed = int(prefix.removeprefix("seed="))
            if not FIRST <= seed < FIRST + SEEDS or arm not in LABELS:
                raise ValueError("unregistered seed or arm")
            if arm in arms[seed]:
                raise ValueError("duplicate arm")
            arms[seed].add(arm)
            if analyze.digest(raw) != initial_sha or analyze.digest(final_raw) != final_sha:
                raise ValueError("initial/final snapshot hash mismatch")
            if seed in initial_hashes and initial_hashes[seed] != initial_sha:
                raise ValueError("paired initial state mismatch")
            initial_hashes[seed] = initial_sha
            revisions.add(revision)
            initial, final = json.loads(raw), json.loads(final_raw)
            expected = expected_program_continuations(seed)
            for actor in (1, 2, 3):
                initial_nodes = initial["processes"][str(actor)]["code"]["nodes"]
                final_nodes = final["processes"][str(actor)]["code"]["nodes"]
                if len(initial_nodes) != 28 or initial_nodes[9:27] != expected[actor]:
                    raise ValueError("original code grammar differs")
                if final_nodes != initial_nodes:
                    changed_codes += 1
            lineage = native_address_lineage(initial, final, arm)
            per_seed[seed][arm] = {
                "native_draw": lineage["first_draw_tick"] is not None,
                "source_read_before_draw": bool(lineage["source_reads_before_first_draw"]),
                "source_read_after_draw": bool(lineage["source_reads_after_first_draw"]),
                "source_address_write": bool(lineage["native_source_address_writes"]),
                "registered_target_write": bool(lineage["registered_source_address_writes"]),
            }
            if lineage["native_source_address_writes"] and len(witness_examples) < 12:
                witness_examples.append({
                    "seed": seed, "arm": arm,
                    "writes": lineage["native_source_address_writes"][:3],
                })
            for name, value in per_seed[seed][arm].items():
                gates[f"{arm}.{name}"] += bool(value)
            events = source2_read_events(final)
            for actor in {e["actor"] for e in events}:
                actor_worlds[f"{arm}.actor{actor}"] += 1
            for e in events:
                actor_events[f"{arm}.actor{e['actor']}"] += 1
                source_tags[str(e["tag"])] += 1
            count += 1
        if count != SEEDS * len(LABELS) or len(arms) != SEEDS:
            raise ValueError("incomplete 128 x 9 cohort")
        if any(a != set(LABELS) for a in arms.values()):
            raise ValueError("unbalanced matched arms")
        if len(revisions) != 1 or not next(iter(revisions)):
            raise ValueError("mixed source revisions")
        if changed_codes:
            raise ValueError("unexpected code changes between initial/final states")
        # The complete epoch event hashes, scheduled interventions, native
        # resource ledger and strict joint endpoints are audited independently
        # by analyze_study33.py before interpreting this retrospective report.
        return {
            "study": "CL-1 Study34 resolved native source exposure",
            "independent_new_worlds": 0,
            "reused_study33_worlds": SEEDS,
            "matched_arms": count,
            "source_revision": next(iter(revisions)),
            "initial_code_changes_at_final": changed_codes,
            "source2_native_read_worlds_by_actor_and_arm": dict(sorted(actor_worlds.items())),
            "source2_native_read_events_by_actor_and_arm": dict(sorted(actor_events.items())),
            "source2_tag_events": dict(sorted(source_tags.items())),
            "source2_process1_control_exposure": actor_worlds["control.actor1"],
            "gate_counts": dict(sorted(gates.items())),
            "retrospective_native_address_witnesses": witness_examples,
            "per_seed_gates": {
                str(seed): per_seed[seed] for seed in sorted(per_seed)
            },
            "graph_operand_interpretation": "REGISTER INDEX, NOT DIRECT GRAPH OBJECT ID",
            "strict_joint_source_and_compute_outcome": "SEE INDEPENDENT STUDY33 AUDIT",
            "life_claim": "NONE",
        }
    finally:
        connection.close()


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("sqlite", type=Path)
    parser.add_argument("--json", type=Path)
    args = parser.parse_args()
    result = audit(args.sqlite)
    if args.json:
        args.json.write_text(json.dumps(result, indent=2) + "\n", encoding="utf-8")
    print(json.dumps(result, indent=2))


if __name__ == "__main__":
    main()
