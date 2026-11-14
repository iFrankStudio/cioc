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
