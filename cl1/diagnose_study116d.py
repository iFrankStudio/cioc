#!/usr/bin/env python3
"""Retrospective ONLY, Study116 original SQLite event/PC diagnostics, 0 new worlds."""
import argparse
import json
import sqlite3
from collections import defaultdict
from pathlib import Path

SEEDS={
    6900019:41,6900029:47,6900030:35,6900034:29,
    6900037:35,6900047:35,6900063:47
}
REVISION="6a30420743ebc0f198925ba1eadf1d76b4fc30b9"

def typed(k):
    return next(iter(k)) if isinstance(k,dict) else k

def study(db):
    conn=sqlite3.connect(db)
    output=[]
    for seed,target in SEEDS.items():
        label=f"seed={seed}/native-latent-source3"
        rows=conn.execute(
            "SELECT source_revision,final_json FROM epoch_runs WHERE condition=?",(label,)
        ).fetchall()
        if len(rows)!=1 or rows[0][0]!=REVISION:
            raise ValueError(f"missing exact original source world {label}")
        w=json.loads(rows[0][1])
        births=[]
        edits=[]
        native_events=defaultdict(list)
        written10=[]
        for ev in w["events"]:
            if ev["origin"]=="External":
                continue
            pid=ev["process"]
            if ev["origin"]!={"Internal":pid}:
                raise ValueError("non-native event attribution")
            k=typed(ev["kind"])
            e={
                "tick":ev["tick"],"pc":ev["pc"],"kind":k,
                "credits_after":ev.get("credits_after"),
            }
            native_events[pid].append(e)
            if isinstance(ev["kind"],dict) and "GraphLaunched" in ev["kind"]:
                v=ev["kind"]["GraphLaunched"]
                births.append({
                    "parent":v["parent_process"],"child":v["child_process"],
                    "tick":ev["tick"],"code_nodes":v["moved_nodes"]
                })
            if isinstance(ev["kind"],dict) and "ExecutableEdgePatched" in ev["kind"]:
                v=ev["kind"]["ExecutableEdgePatched"]
                if v["node"]==4:
                    edits.append({"actor":pid,"tick":ev["tick"],**v})
            if isinstance(ev["kind"],dict) and "CellWritten" in ev["kind"]:
                v=ev["kind"]["CellWritten"]
                if v["key"] in (10,13):
                    written10.append({"actor":pid,"tick":ev["tick"],"pc":ev["pc"],**v})
        sample_children=[b["child"] for b in births[:4]]
        report={
            "seed":seed,"initial_target":target,"birth_count":len(births),
            "early_births":births[:8],"actual_pc4_mutations":edits[:16],
            "all_original_internal_key10_and_key13_writes":written10[:24],
            "first_four_native_children":{
                str(child):{
                    "first40actor_paid_PC_events":native_events[child][:40],
                    "final_PC":w["processes"][str(child)].get("pc"),
                    "final_credits":w["processes"][str(child)].get("credits"),
                    "final_stopped":w["processes"][str(child)].get("stopped"),
                    "final_node4":w["processes"][str(child)]["code"]["nodes"][4],
                    "module_PC_hits":[ev for ev in native_events[child] if 29<=ev["pc"]<=52][:25],
                } for child in sample_children
            },
            "founder1_first40PC":native_events[1][:40],
            "founder2_first40PC":native_events[2][:40],
            "all_real_module_written_key10_count":sum(x["key"]==10 for x in written10),
        }
        output.append(report)
    return {
        "status":"POST_OUTCOME_DIAGNOSTIC_ORIGINAL_SOURCE_NO_NEW_WORLDS",
        "original_revision":REVISION,
        "original_unselected_module_entry_target_seeds":len(SEEDS),
        "original_native_worlds_reopened":len(output),
        "original_genomes_or_laws_modified":0,
        "case_records":output,
    }

def main():
    p=argparse.ArgumentParser()
    p.add_argument("original_sqlite",type=Path)
    p.add_argument("--json",type=Path,required=True)
    a=p.parse_args()
    result=study(a.original_sqlite)
    a.json.write_text(json.dumps(result,indent=2,sort_keys=True)+"\n")
    print(json.dumps({
        "status":result["status"],
        "cases":[{
            "seed":x["seed"],"initial_target":x["initial_target"],
            "birth_count":x["birth_count"],
            "module_outputs_key10":x["all_real_module_written_key10_count"],
            "first_child_pc_trace":next(iter(x["first_four_native_children"].values()))["first40actor_paid_PC_events"][:20]
            if x["first_four_native_children"] else [],
            "early_births":x["early_births"][:2],
        } for x in result["case_records"]
        ]},sort_keys=True))
if __name__=="__main__":
    main()
