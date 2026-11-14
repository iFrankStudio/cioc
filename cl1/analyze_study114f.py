#!/usr/bin/env python3
"""Post-outcome Study114-F: source-function and 2-generation health of original native offspring.

No simulation, no new genomes. First re-verify all Study114 independent original
SHA256 and code-copy ancestry using the original read-only auditor.
"""
import argparse
import json
import sqlite3
from collections import defaultdict
from pathlib import Path

from analyze_study114 import audit as verify_original

ORIGINAL_REVISION = "dd0a1ee78765fbdcdf625ac08069198625d0f84b"
FIRST = 6_700_000
COUNT = 32
MODES = (
    "native-core-source3", "disabled-core-source3",
    "native-full-source3", "disabled-full-source3",
    "native-core-source8", "disabled-core-source8",
    "native-full-source8", "disabled-full-source8",
)

def require(condition, explanation):
    if not condition:
        raise ValueError(explanation)

def event_part(e, name):
    k = e["kind"]
    return k.get(name) if isinstance(k, dict) else None

def native_genetic_lineage(state, source):
    """Count complete native children with actual independent source-function execution."""
    births = {}
    first_reads = {}
    functional = set()
    source_outside_function = 0
    child_born_after_last_epoch = 0
    for event in state["events"]:
        native = event["origin"]
        if native == "External":
            continue
        a = event["process"]
        require(native == {"Internal": a}, "ambiguous actor origin")
        read = event_part(event, "CellRead")
        write = event_part(event, "CellWritten")
        launch = event_part(event, "GraphLaunched")
        if read is not None and read["key"] == 7 and event["pc"] == 2 and read["value"] == source:
            first_reads.setdefault(a, event["tick"])
        if write is not None and write["key"] == 9:
            if (event["pc"] == 3 and write["value"] == source
                    and a in first_reads and first_reads[a] < event["tick"]):
                functional.add(a)
            else:
                source_outside_function += 1
        if launch is not None:
            require(event["pc"] == 22 and launch["parent_process"] == a,
                    "child not created by verified native parent PC22")
            child = launch["child_process"]
            require(child >= 3 and child not in births, "original/external or duplicate child")
            require(1 <= launch["moved_nodes"] <= 56, "invalid born code node count")
            births[child] = {
                "child": child,
                "parent": a,
                "birth_tick": event["tick"],
                "complete": launch["moved_nodes"] == 56,
                "code_nodes": launch["moved_nodes"],
            }
    require(len(births)+2 == len(state["processes"]),
            "number of actual native children not consistent with resident process genealogy")
    for child, b in births.items():
        require(str(child) in state["processes"], "child missing final resident process code")
        require(len(state["processes"][str(child)]["code"]["nodes"]) == b["code_nodes"],
                "real native launched code not same length as resident child code")

    full_functional = {
        pid for pid,b in births.items()
        if b["complete"] and pid in functional
    }
    reproducing = {
        pid for pid in full_functional
        if any(b["parent"] == pid for b in births.values())
    }
    grandchildren = {
        pid for pid,b in births.items() if b["parent"] >= 3
    }
    full_functional_grandchildren = {
        pid for pid in full_functional
        if births[pid]["parent"] in full_functional
    }
    true_function_source_both_founders = {1,2}.issubset(functional)
    # Original function-inheritance assay is only valid if original founders
    # were actually observed to execute its source read/write before mutation.
    require(true_function_source_both_founders,
            "both original founder CIL actors failed source function even before mutation")
    return {
        "raw_native_births":len(births),
        "complete_births":sum(b["complete"] for b in births.values()),
        "incomplete_births":sum(not b["complete"] for b in births.values()),
        "source_correct_child_writers":sum(pid in functional for pid in births),
        "complete_functional_child_births":len(full_functional),
        "complete_functional_reproducing_child_count":len(reproducing),
        "grandchildren_of_any_native_child":len(grandchildren),
        "complete_functional_grandchildren_from_complete_functional_parents":len(
            full_functional_grandchildren
        ),
        "noncanonical_or_source_wrong_output_events":source_outside_function,
        "terminal_functional_child_ids":sorted(full_functional),
        "terminal_reproducing_functional_child_ids":sorted(reproducing),
        "actual_native_child_genealogy":list(births.values()),
    }

def all_correlations(original_db):
    verified=verify_original(original_db, ORIGINAL_REVISION)
    require(verified["original_matched_worlds"] == 256
            and verified["exact_full_state_SHA256_epochs"] == 4096
            and verified["distinct_initial_genome_hashes"] == 32,
            "untouched 56-node original Study114 full-state audit failed")
    registered={
        (row["seed"],row["arm"]):row
        for row in verified["all256_original_world_provenance"]
    }
    require(len(registered)==256,"original auditor lost complete denominator")
    conn=sqlite3.connect(original_db)
    original=conn.execute("SELECT condition,source_revision,final_json FROM epoch_runs ORDER BY id").fetchall()
    require(len(original)==256,"wrong complete original SQLite denominator")
    worlds={}
    for label,revision,final_json in original:
        require(revision==ORIGINAL_REVISION,"source run mismatch")
        s,arm=label.removeprefix("seed=").split("/",1)
        seed=int(s)
        require(seed in range(FIRST,FIRST+COUNT) and arm in MODES
                and (seed,arm) not in worlds,"original seed or registered arm violated")
        source=3 if arm.endswith("source3") else 8
        observed=native_genetic_lineage(json.loads(final_json),source)
        old=registered[(seed,arm)]
        require(observed["raw_native_births"]==old["births"],
                "functional lineage reconstruction disagrees with already independently audited native birth count")
        require(observed["incomplete_births"]==old["incomplete_copy_births"],
                "incomplete native offspring count violates original verified event provenance")
        worlds[(seed,arm)]={
            "seed":seed,"arm":arm,"source":source,
            "site_policy":"core" if "-core-" in arm else "full",
            "native_patch_enabled":arm.startswith("native-"),
            "initial_genome_hash":old["genome_hash"],
            **observed,
        }
    require(len(worlds)==256,"missing original result rows")
    comparison={}
    for source in (3,8):
        paired=[]
        for seed in range(FIRST,FIRST+COUNT):
            record={"seed":seed,"source":source}
            for metric in ("raw_native_births","complete_births",
                           "complete_functional_child_births",
                           "complete_functional_reproducing_child_count",
                           "complete_functional_grandchildren_from_complete_functional_parents"):
                d_core=(
                    worlds[(seed,f"native-core-source{source}")][metric]
                    -worlds[(seed,f"disabled-core-source{source}")][metric]
                )
                d_full=(
                    worlds[(seed,f"native-full-source{source}")][metric]
                    -worlds[(seed,f"disabled-full-source{source}")][metric]
                )
                record[metric]={
                    "effect_core":d_core,
                    "effect_full":d_full,
                    "interaction_full_minus_core":d_full-d_core,
                }
            paired.append(record)
        def data(metric):
            v=[r[metric]["interaction_full_minus_core"] for r in paired]
            return {
                "signed_full_minus_core":v,
                "positive":sum(x>0 for x in v),
                "negative":sum(x<0 for x in v),
                "ties":sum(x==0 for x in v),
                "raw_4_arm_totals":{
                    arm:sum(worlds[(seed,arm)][metric] for seed in range(FIRST,FIRST+COUNT))
                    for arm in MODES if arm.endswith(f"source{source}")
                },
            }
        expected=verified["primary_buffer_interaction_by_source"][f"source{source}"][
            "full_minus_core_32_buffer_interaction"
        ]
        require(data("raw_native_births")["signed_full_minus_core"]==expected,
                "raw frozen Study114 primary score unexpectedly changed")
        comparison[f"source{source}"]={
            metric:data(metric) for metric in (
                "raw_native_births","complete_births",
                "complete_functional_child_births",
                "complete_functional_reproducing_child_count",
                "complete_functional_grandchildren_from_complete_functional_parents")
        }
        comparison[f"source{source}"]["all_original_32_seed_pairs"]=paired
    return {
        "status":"POST_OUTCOME_ORIGINAL_56_NODE_FUNCTIONAL_CHILD_GENEALOGY_NOT_NATURAL_EVOLUTION",
        "original_revision":ORIGINAL_REVISION,
        "original_worlds":256,
        "full_original_SHA256_epochs_reaudited":4096,
        "new_genotype_samples":0,
        "source3_and_source8_not_independent":True,
        "by_dependent_source":comparison,
        "all256_actual_native_world_functional_genealogies":[
            worlds[(seed,arm)] for seed in range(FIRST,FIRST+COUNT) for arm in MODES
        ],
        "CL1_accepted":False,
    }

def main():
    p=argparse.ArgumentParser()
    p.add_argument("original_sqlite",type=Path)
    p.add_argument("--json",type=Path,required=True)
    args=p.parse_args()
    out=all_correlations(args.original_sqlite)
    args.json.write_text(json.dumps(out,indent=2,sort_keys=True)+"\n",encoding="utf-8")
    print(json.dumps({
        "status":out["status"],"original_worlds":out["original_worlds"],
        "full_original_SHA256_epochs_reaudited":out["full_original_SHA256_epochs_reaudited"],
        "all_metrics_by_source":{
            source:{
                metric:{"positive":value["positive"],"negative":value["negative"],
                        "ties":value["ties"],"totals":value["raw_4_arm_totals"]}
                for metric,value in record.items() if metric!="all_original_32_seed_pairs"
            } for source,record in out["by_dependent_source"].items()
        }
    },sort_keys=True))

if __name__=="__main__":
    main()
