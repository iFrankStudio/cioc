#!/usr/bin/env python3
"""Study61 paired native graph portability causal audit (new-seed prospective).

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
import analyze_study61_absolute as absolute
import analyze_study61_local as local
from analyze_study02 import wilson_95

FIRST = 4_100_000
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
        a = bool(aa["child_native_CellWritten"])
        b = bool(bb["child_native_CellWritten"])
        pair["local_only"] += int(b and not a)
        pair["absolute_only"] += int(a and not b)
        pair["both"] += int(a and b)
        pair["neither"] += int(not a and not b)
    n_a = original["primary_native_child_shared_cell_write"]["worlds"]
    n_b = treatment["primary_native_child_shared_cell_write"]["worlds"]
    if n_a != pair["absolute_only"] + pair["both"] or n_b != pair["local_only"] + pair["both"]:
        raise ValueError("paired child Write does not match independent original audits")

    # Independently inspect actual child successful non-GraphCreated events
    # from the original source-pinned event chronicles. The original source
    # auditors have independently checked all 49,152 SHA256 snapshots.
    def child_qualifiers(path: Path) -> dict[int, bool]:
        db = sqlite3.connect(f"{path.resolve().as_uri()}?mode=ro", uri=True)
        try:
            result = {}
            for label,raw_initial,raw_final in db.execute(
                "SELECT condition,initial_json,final_json FROM epoch_runs "
                "WHERE condition LIKE '%/ambient-general' ORDER BY id"
            ):
                seed = int(label.split("/",1)[0].removeprefix("seed="))
                if seed in result or seed not in range(FIRST,FIRST+SEEDS):
                    raise ValueError("unregistered child exposure seed")
                initial,final=json.loads(raw_initial),json.loads(raw_final)
                launch={}
                good=False
                for event in final["events"][len(initial["events"]):]:
                    kind=event["kind"]
                    actor=event["process"]
                    if (analyze.internal_process(event)!=actor):
                        continue
                    if isinstance(kind,dict) and "GraphLaunched" in kind:
                        detail=kind["GraphLaunched"]
                        child=detail["child_process"]
                        if (detail["parent_process"]!=actor or child<=3 or child in launch):
                            raise ValueError("invalid original native child launch")
                        launch[child]=event["tick"]
                    if actor in launch and event["tick"]>launch[actor]:
                        typ=(next(iter(kind)) if isinstance(kind,dict) else kind)
                        if typ not in ("GraphCreated","Trapped","Halted"):
                            good=True
                result[seed]=good
            if len(result)!=SEEDS:
                raise ValueError("incomplete independently sampled child outcome seeds")
            return result
        finally:
            db.close()
    child_a=child_qualifiers(absolute_db)
    child_b=child_qualifiers(local_db)
    paired_child=Counter()
    for seed in range(FIRST,FIRST+SEEDS):
        x,y=child_a[seed],child_b[seed]
        paired_child["local_only"]+=int(y and not x)
        paired_child["absolute_only"]+=int(x and not y)
        paired_child["both"]+=int(x and y)
        paired_child["neither"]+=int(not x and not y)
    n_child_a=sum(child_a.values())
    n_child_b=sum(child_b.values())
    if (n_child_a != original["secondary"].get(
            "ambient-general.child_non_GraphCreated_success",0)
            or n_child_b != treatment["secondary"].get(
                "ambient-general.child_non_GraphCreated_success",0)):
        raise ValueError("independently audited native child opcode count differs")
    return {
        "study": "CL-1 Study61 prospectively paired local native graph edge law",
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
        "primary_B_executable_Write_graph_at_native_launch": {
            "status": "NOT EVALUABLE AT INSTRUCTION-LEVEL FROM EPOCH-BOUNDARY SNAPSHOTS",
            "reason": "A later code graph alone does not independently establish the child executable bytes at the exact GraphLaunched tick, nor its source-node provenance when same-epoch edits are possible.",
            "measured_as_primary_success": False,
        },
        "primary_B_successful_native_child_non_GraphCreated": {
            "absolute_worlds": n_child_a,
            "local_worlds": n_child_b,
            "denominator_per_regime": SEEDS,
            "absolute_wilson95": [round(x,6) for x in wilson_95(n_child_a, SEEDS)],
            "local_wilson95": [round(x,6) for x in wilson_95(n_child_b, SEEDS)],
            "paired_discordance": dict(sorted(paired_child.items())),
            "prospective_direction_local_more": paired_child["local_only"] > paired_child["absolute_only"],
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
        "primary_A":report["primary_functional_native_child_CellWritten"],
        "primary_B":report["primary_B_successful_native_child_non_GraphCreated"],
        "life_claim":"NONE",
    },indent=2))


if __name__=="__main__":
    main()
