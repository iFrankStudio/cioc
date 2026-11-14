#!/usr/bin/env python3
"""Study34 read-only initial-code reachability audit of the archived Study33 cohort.

This diagnostic is retrospective; it does not certify absence of dynamic code edits.
"""
from __future__ import annotations

import argparse
import json
import sqlite3
from collections import Counter, defaultdict
from pathlib import Path

from analyze_study33 import FIRST, LABELS, SEEDS, expected_program_continuations


def eligible_initial_read(nodes: list) -> bool:
    return any(
        isinstance(op, dict)
        and isinstance(op.get("GraphReadTag"), dict)
        and op["GraphReadTag"].get("graph") == 2
        and op["GraphReadTag"].get("node") == 0
        for op in nodes
    )


def audit(path: Path) -> dict:
    connection = sqlite3.connect(f"{path.resolve().as_uri()}?mode=ro", uri=True)
    try:
        rows = connection.execute(
            "SELECT condition,initial_json,source_revision FROM epoch_runs"
        ).fetchall()
        if len(rows) != SEEDS * len(LABELS):
            raise ValueError("expected all 128 seeds x nine paired arms")
        arms = defaultdict(set)
        seen_initial = {}
        revisions = set()
        by_actor = Counter()
        for condition, raw, revision in rows:
            if not condition.startswith("seed=") or "/" not in condition:
                raise ValueError("unregistered arm")
            prefix, arm = condition.split("/", 1)
            seed = int(prefix.removeprefix("seed="))
            if not FIRST <= seed < FIRST + SEEDS or arm not in LABELS:
                raise ValueError("unregistered seed or arm")
            if arm in arms[seed]:
                raise ValueError("duplicate paired arm")
            arms[seed].add(arm)
            if seed in seen_initial and seen_initial[seed] != raw:
                raise ValueError("initial state differs between paired arms")
            seen_initial[seed] = raw
            revisions.add(revision)
            state = json.loads(raw)
            expected = expected_program_continuations(seed)
            for actor in (1, 2, 3):
                nodes = state["processes"][str(actor)]["code"]["nodes"]
                if len(nodes) != 28 or nodes[9:27] != expected[actor]:
                    raise ValueError("initial code does not match Study33 grammar")
                if eligible_initial_read(nodes):
                    by_actor[actor] += 1
        if len(arms) != SEEDS or any(a != set(LABELS) for a in arms.values()):
            raise ValueError("incomplete paired cohort")
        if len(revisions) != 1 or not next(iter(revisions)):
            raise ValueError("mixed or missing source revisions")
        if by_actor:
            raise ValueError("source2/node0 initial read exists; static null invalid")
        return {
            "study": "CL-1 Study34 retrospective initial-code diagnostic",
            "independent_new_worlds": 0,
            "reused_study33_worlds": SEEDS,
            "matched_arms": len(rows),
            "source_revision": next(iter(revisions)),
            "initial_processes_with_source2_node0_read": 0,
            "initial_read_gate": "STRUCTURALLY UNREACHABLE",
            "executed_read_gate": "NOT EVALUABLE WITHOUT DYNAMIC CODE ANCESTRY",
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
