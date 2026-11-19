#!/usr/bin/env python3
"""I6 pre-native finite two-edit reachability, not a native actor or CIL replay.

Reads the frozen I5 64-case semantic oracle, independently constructs all
single-word mutation edges and breadth-first shortest paths. This oracle is
derived from I5's frozen predictions, not from Rust traces; no claim of
independent verification of CIL semantics or of an implemented I6 actor.
"""
import itertools
import json
from collections import deque
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
MANIFEST = ROOT / "studies/B56-F1d1k-G3Q-C1-P1-L3-I5-frozen-173-node-optimal-one-byte-repair-v1.json"
GENES = tuple(itertools.product(range(4), repeat=3))

def neighbors(g):
    for i in range(3):
        for v in range(4):
            if v != g[i]:
                h = list(g)
                h[i] = v
                yield tuple(h)

def main():
    data = json.loads(MANIFEST.read_text(encoding="utf-8"))
    entries = data["pre_registered_64_original_genotypes"]
    assert len(entries) == 64
    table = {tuple(e["seed"]): e for e in entries}
    assert set(table) == set(GENES)
    safe = {g for g, e in table.items() if e["initial_safe"]}
    assert len(safe) == 24
    assert all(table[g]["initial_safe"] == (g in safe) for g in GENES)
    distances = {}
    paths = {}
    for start in GENES:
        q = deque([(start, (start,))])
        seen = {start}
        while q:
            node, path = q.popleft()
            if node in safe:
                distances[start] = len(path) - 1
                paths[start] = path
                break
            for nxt in neighbors(node):
                if nxt not in seen:
                    seen.add(nxt)
                    q.append((nxt, path + (nxt,)))
        assert start in distances
    hard = sorted(g for g, d in distances.items() if d == 2)
    assert len(hard) == 8, hard
    assert sorted(distances.values()).count(0) == 24
    assert sorted(distances.values()).count(1) == 32
    assert sorted(distances.values()).count(2) == 8
    assert hard == sorted(map(tuple, data["finite_one_byte_bound_proof"]["impossible_genotypes"]))
    for g in hard:
        assert not any(n in safe for n in neighbors(g))
        assert paths[g][1] not in safe and paths[g][2] in safe
    result = {
        "status": "PRE_NATIVE_FINITE_SEMANTIC_GRAPH_ONLY",
        "source": str(MANIFEST.relative_to(ROOT)),
        "distance_histogram": {"0": 24, "1": 32, "2": 8},
        "hard_cases": [
            {"seed": list(g), "lexicographic_shortest_path": [list(x) for x in paths[g]],
             "two_step_path_count": sum(1 for a in neighbors(g) for b in neighbors(a) if b in safe)}
            for g in hard
        ],
        "limitations": [
            "Semantic labels come from frozen I5 predictions, not independently recomputed Rust/CIL",
            "Graph paths are host-selected; PID1 has not executed two source edits",
            "No compute, node, memory, feedback, or reset costs modeled",
        ],
    }
    print(json.dumps(result, ensure_ascii=False, indent=2))

if __name__ == "__main__":
    main()
