#!/usr/bin/env python3
"""Study59 paired native graph portability causal audit (new-seed prospective).

Native child Write is required, not merely code existence; VM-assisted
edge relocation is external world semantics, not endogenous life.
"""
from __future__ import annotations

import argparse
import json
import sqlite3
from collections import Counter
from copy import deepcopy
from pathlib import Path

import analyze
import analyze_study59_absolute as absolute
import analyze_study59_local as local
from analyze_study02 import wilson_95

FIRST = 4_000_000
SEEDS = 256
ARMS = ("ambient-general", "ambient-only", "closed-general",
        "zero-credit-ambient-general")


def original_seed_records(path: Path, source_revision: str) -> dict[int, dict]:
    db = sqlite3.connect(f"{path.resolve().as_uri()}?mode=ro", uri=True)
    try:
        worlds = {}
        for condition, revision, raw in db.execute(
            "SELECT condition,source_revision,initial_json "
            "FROM epoch_runs ORDER BY id"
        ):
            pre, arm = condition.split("/", 1)
            seed = int(pre.removeprefix("seed="))
            if revision != source_revision or not FIRST <= seed < FIRST + SEEDS:
                raise ValueError("wrong run source/seed revision")
            if arm not in ARMS:
                raise ValueError("unregistered environment condition")
            initial = json.loads(raw)
            if seed in worlds:
                if worlds[seed]["world"] != initial:
                    raise ValueError("matched four environmental arms differ at fork")
                if arm in worlds[seed]["arms"]:
                    raise ValueError("duplicate within-seed environment arm")
                worlds[seed]["arms"].add(arm)
            else:
                worlds[seed] = {"world": initial, "arms": {arm}}
        if (len(worlds) != SEEDS or any(w["arms"] != set(ARMS) for w in worlds.values())):
            raise ValueError("missing original seed or environment arm")
        return {k: v["world"] for k, v in worlds.items()}
    finally:
        db.close()


def normalize_local(nodes: list) -> list:
    """Only registered choice1/20 opcode names, never change scaffold or other fields."""
    result = deepcopy(nodes)
    for pc in range(9, 81):
        node = result[pc]
        if isinstance(node, dict) and "GraphCopyNodeLocal" in node:
            if node["GraphCopyNodeLocal"] != {
                "graph": 3, "source": 2, "dst": 2, "next": pc+1,
            }:
                raise ValueError("undeclared local opcode arguments")
            result[pc] = {"GraphCopyNode": node["GraphCopyNodeLocal"]}
    return result


def paired_audit(absolute_db: Path, local_db: Path, revision: str) -> dict:
    original = absolute.audit(absolute_db, revision)
    treatment = local.audit(local_db, revision)
    for name, data in (("absolute", original), ("local", treatment)):
        if (data["source_revision"] != revision or
                data["independent_worlds"] != SEEDS or
                data["matched_runs"] != SEEDS*len(ARMS) or
                data["hashed_epoch_snapshots"] != SEEDS*len(ARMS)*24):
            raise ValueError("original full source-pinned independent audit invalid: "+name)
    a_worlds = original_seed_records(absolute_db, revision)
    b_worlds = original_seed_records(local_db, revision)
    for seed in range(FIRST, FIRST+SEEDS):
        a = a_worlds[seed]
        b = deepcopy(b_worlds[seed])
        for actor in (1,2,3):
            p = b["processes"][str(actor)]
            p["code"]["nodes"] = normalize_local(p["code"]["nodes"])
        if a != b:
            raise ValueError("initial native CIL program differs beyond preregistered generic copy law")
    pair = Counter()
    mechanisms = {}
    for seed in range(FIRST, FIRST+SEEDS):
        aa = original["all_seed_outcomes"][str(seed)]["ambient-general"]
        bb = treatment["all_seed_outcomes"][str(seed)]["ambient-general"]
        # Source original auditors record strict child output per seed.
        a = bool(aa["native_child_shared_cell_write"])
        b = bool(bb["native_child_shared_cell_write"])
        pair["local_only"] += int(b and not a)
        pair["absolute_only"] += int(a and not b)
        pair["both"] += int(a and b)
        pair["neither"] += int(not a and not b)
    n_a = original["primary_native_child_shared_cell_write"]["worlds"]
    n_b = treatment["primary_native_child_shared_cell_write"]["worlds"]
    if n_a != pair["absolute_only"] + pair["both"] or n_b != pair["local_only"] + pair["both"]:
        raise ValueError("paired child Write does not match independent original audits")
    return {
        "study": "CL-1 Study59 prospectively paired local native graph edge law",
        "source_revision": revision,
        "independent_seed_pairs": SEEDS,
        "matched_native_runs": 2*SEEDS*len(ARMS),
        "SHA256_verified_snapshots": 2*SEEDS*len(ARMS)*24,
        "primary_functional_native_child_CellWritten": {
            "absolute_worlds": n_a,
            "local_worlds": n_b,
            "denominator_per_regime": SEEDS,
            "absolute_wilson95": [round(x,6) for x in wilson_95(n_a,SEEDS)],
            "local_wilson95": [round(x,6) for x in wilson_95(n_b,SEEDS)],
            "paired_discordance": dict(sorted(pair.items())),
        },
        "independent_secondary": {
            "absolute": original["secondary"],
            "local": treatment["secondary"],
        },
        "source_of_local_edge_closure": "EXTERNAL VM-DEFINED GENERIC GRAPH LOCALITY",
        "native_self_repair": "NOT DEMONSTRATED",
        "life_claim": "NONE",
    }


def main() -> None:
    p=argparse.ArgumentParser(description=__doc__)
    p.add_argument("absolute",type=Path)
    p.add_argument("local",type=Path)
    p.add_argument("--expected-revision",required=True)
    p.add_argument("--json",type=Path,required=True)
    args=p.parse_args()
    report=paired_audit(args.absolute,args.local,args.expected_revision)
    args.json.write_text(json.dumps(report,indent=2)+"\n",encoding="utf-8")
    print(json.dumps({
        "study":report["study"],
        "seed_pairs":report["independent_seed_pairs"],
        "primary":report["primary_functional_native_child_CellWritten"],
        "life_claim":"NONE",
    },indent=2))


if __name__=="__main__":
    main()
