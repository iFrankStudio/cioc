#!/usr/bin/env python3
"""Study62 retrospective native child graph-copy ancestry; no new seed worlds."""
import argparse
import json
import sqlite3
from collections import Counter, defaultdict
from pathlib import Path

import analyze
import analyze_study61

REVISION = "905bc347401fb1a85ae0faef1b01fd57ef8d139e"
FIRST, COUNT = 4_100_000, 256
ARMS = {"ambient-general", "ambient-only", "closed-general",
        "zero-credit-ambient-general"}


def classify(initial, final):
    events = final["events"][len(initial["events"]):]
    launches, graphs, copies, grandchildren = {}, {}, [], []
    for e in events:
        actor = e["process"]
        if analyze.internal_process(e) != actor:
            continue
        k = e["kind"]
        typ = next(iter(k)) if isinstance(k, dict) else k
        if typ == "GraphLaunched":
            d = k[typ]
            child = d["child_process"]
            if d["parent_process"] != actor or child in launches or child <= 3:
                raise ValueError("forged or duplicated native launch")
            launches[child] = {"parent": actor, "graph": d["source_graph"], "tick": e["tick"]}
            if actor in launches:
                grandchildren.append((actor, child))
        if typ == "GraphCreated" and actor in launches:
            d = k[typ]
            if d["owner"] != actor or d["graph"] in graphs:
                raise ValueError("forged native child-owned graph")
            graphs[d["graph"]] = {"child": actor, "tick": e["tick"]}
        if typ != "GraphNodeCopiedLocal" or actor not in launches:
            continue
        d = k[typ]
        graph = graphs.get(d["graph"])
        if (graph is None or graph["child"] != actor or graph["tick"] >= e["tick"]
                or launches[actor]["tick"] >= e["tick"]
                or d["owner"] != actor or d["source_process"] != actor):
            raise ValueError("descendant did not own and create native local-copy target graph")
        code = final["processes"][str(actor)]["code"]["nodes"]
        if (not isinstance(e["pc"], int) or not 0 <= e["pc"] < len(code)
                or not isinstance(d["source_node"], int)
                or not 0 <= d["source_node"] < len(code)):
            raise ValueError("native copied source or executable PC out of range")
        pc_code = code[e["pc"]]
        if not isinstance(pc_code, dict) or "GraphCopyNodeLocal" not in pc_code:
            raise ValueError("child has no local-copy executable opcode at recorded PC")
        copies.append({
            "tick": e["tick"], "child": actor, "original_parent": launches[actor]["parent"],
            "original_source_graph": launches[actor]["graph"],
            "child_owned_target_graph": d["graph"],
            "source_pc": d["source_node"], "source_opcode": code[d["source_node"]],
            "copy_pc": e["pc"], "new_graph_node": d["new_node"]
        })
    per_graph = Counter((x["child"], x["child_owned_target_graph"]) for x in copies)
    return {"copies": copies, "descendant_grandchildren": grandchildren,
            "repeated_copy_to_same_graph": any(v > 1 for v in per_graph.values()),
            "distinct_target_graphs": len(per_graph)}


def one_mode(path, mode):
    db = sqlite3.connect(f"{path.resolve().as_uri()}?mode=ro", uri=True)
    try:
        seen, counts, witnesses = defaultdict(set), Counter(), []
        for condition, source, raw_initial, raw_final in db.execute(
            "SELECT condition,source_revision,initial_json,final_json FROM epoch_runs ORDER BY id"
        ):
            label, arm = condition.split("/", 1)
            seed = int(label.removeprefix("seed="))
            if (source != REVISION or seed not in range(FIRST, FIRST+COUNT)
                    or arm not in ARMS or arm in seen[seed]):
                raise ValueError("historical native source/arm mismatch")
            seen[seed].add(arm)
            d = classify(json.loads(raw_initial), json.loads(raw_final))
            counts[f"{arm}.child_native_local_copy_events"] += len(d["copies"])
            counts[f"{arm}.worlds_with_descendant_local_copy"] += int(bool(d["copies"]))
            counts[f"{arm}.distinct_child_owned_graph_targets"] += d["distinct_target_graphs"]
            counts[f"{arm}.worlds_repeated_local_copy_to_same_graph"] += int(d["repeated_copy_to_same_graph"])
            counts[f"{arm}.native_grandchild_launches"] += len(d["descendant_grandchildren"])
            if d["copies"] and len(witnesses) < 12:
                witnesses.append({"seed": seed, "arm": arm, **d})
        if len(seen) != COUNT or any(s != ARMS for s in seen.values()):
            raise ValueError("original cohort incomplete")
        return {"counts": dict(sorted(counts.items())),
                "original_source_witnesses": witnesses}
    finally:
        db.close()


def main():
    p = argparse.ArgumentParser(description=__doc__)
    p.add_argument("absolute", type=Path)
    p.add_argument("local", type=Path)
    p.add_argument("--json", type=Path, required=True)
    a = p.parse_args()
    original = analyze_study61.paired_audit(a.absolute, a.local, REVISION)
    if original["SHA256_verified_snapshots"] != 49152 or original["independent_seed_pairs"] != 256:
        raise ValueError("original native SQLite independently audited source mismatch")
    modes = {"absolute": one_mode(a.absolute, "absolute"),
             "local": one_mode(a.local, "local")}
    if (modes["absolute"]["counts"].get("ambient-general.child_native_local_copy_events",0) != 0
        or modes["local"]["counts"].get("ambient-general.child_native_local_copy_events",0) != 11):
        raise ValueError("native child graph copying count contradicts original frozen audit")
    r = {"study": "CL-1 Study62 retrospective native descendant owned graph construction",
         "source_revision": REVISION, "new_independent_worlds": 0,
         "reused_original_seed_pairs": 256, "matched_runs": 2048,
         "reverified_original_sha256_snapshots": 49152, "modes": modes,
         "life_claim": "NONE"}
    a.json.write_text(json.dumps(r, indent=2)+"\n", encoding="utf-8")
    print(json.dumps(r, indent=2)[:6000])


if __name__ == "__main__":
    main()
