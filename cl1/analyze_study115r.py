#!/usr/bin/env python3
"""Study115-R POST OUTCOME original native CIL trait persistence by exact genealogy.

No new worlds; original 512 worlds and 8192 SHA256 epochs must be independently
audited in analyze_study115 before any germline conclusion is computed.
"""
import argparse
import json
import sqlite3
from collections import defaultdict
from pathlib import Path

from analyze_study115 import audit as audit_original

REVISION="e1cbb2f5364dccd20afcce1a232d62e46018147c"
FIRST=6_800_000
N=64
MODULES={29,35,41,47}
MODES=(
    "native-latent-source3","disabled-latent-source3",
    "native-inert-source3","disabled-inert-source3",
    "native-latent-source8","disabled-latent-source8",
    "native-inert-source8","disabled-inert-source8",
)

def require(condition,reason):
    if not condition:
        raise ValueError(reason)

def kind(event,name):
    k=event["kind"]
    return k.get(name) if isinstance(k,dict) else None

def genealogy(final, qualified):
    """Reconstruct native child-birth PC4 specifically at original graph-copy-time."""
    origin_edge={1:24,2:24}
    generation={1:0,2:0}
    births={}
    graphs={}
    mutations={}
    for event in final["events"]:
        if event["origin"]=="External":
            continue
        actor=event["process"]
        require(event["origin"]=={"Internal":actor} and actor in origin_edge,
                "orphan native actor in original event stream")
        change=kind(event,"ExecutableEdgePatched")
        graph_new=kind(event,"GraphCreated")
        copied=kind(event,"GraphNodeCopied")
        launched=kind(event,"GraphLaunched")
        if change is not None and change["node"]==4:
            require(change["before"]==origin_edge[actor],
                    "actual actor native code mutation does not match its original ancestry")
            origin_edge[actor]=change["after"]
            mutations.setdefault(actor,[]).append({
                "tick":event["tick"],"from":change["before"],"to":change["after"]
            })
        if graph_new is not None:
            gid=graph_new["graph"]
            require(gid not in graphs and graph_new["owner"]==actor,
                    "native graph id reused or created by other actor")
            graphs[gid]={"owner":actor,"copied":0,"copied_pc4_edge":None}
        if copied is not None:
            gid=copied["graph"]
            require(gid in graphs and graphs[gid]["owner"]==actor
                    and copied["source_process"]==actor,
                    "graph-copy event lacks same native actor source")
            g=graphs[gid]
            require(copied["new_node"]==g["copied"],
                    "native copied node not sequential in full genealogy")
            if copied["source_node"]==4:
                # This is the child's ACTUAL birth-edge, not its parent's later
                # current code after mutations while copying other graph nodes.
                g["copied_pc4_edge"]=origin_edge[actor]
            g["copied"]+=1
        if launched is not None:
            gid=launched["source_graph"]
            child=launched["child_process"]
            require(event["pc"]==22 and launched["parent_process"]==actor
                    and child not in origin_edge and child>=3
                    and gid in graphs and graphs[gid]["owner"]==actor,
                    "native child not descended from original parent-owned graph")
            g=graphs[gid]
            copied_count=launched["moved_nodes"]
            require(copied_count==g["copied"] and copied_count>0,
                    "actual child code length not grounded in copied graph")
            code_edge=g["copied_pc4_edge"] if copied_count>4 else None
            generation[child]=generation[actor]+1
            births[child]={
                "child":child,"parent":actor,"generation":generation[child],
                "birth_tick":event["tick"],
                "complete":copied_count==56,
                "copied_nodes":copied_count,
                "birth_PC4_edge":code_edge,
            }
            # Short-code children can still run, but cannot qualify as a full
            # 56-node trait carrier; their original copied PC4 may be absent.
            origin_edge[child]=code_edge
    require(len(final["processes"])==len(births)+2,
            "originally resident process inventory disagrees with native births")
    strict_child={
        w["actor"]:w for w in qualified
        if w["full_56_node_child_with_inherited_active_edge"]
    }
    for child,w in strict_child.items():
        require(child in births and births[child]["complete"]
                and births[child]["birth_PC4_edge"]==w["module"]
                and births[child]["birth_tick"]<w["start_tick"],
                "first independent strict trait witness contradicts copied PC4 gene at birth")
    source_function_grandchildren=[
        child for child,b in births.items()
        if b["generation"]>=2 and child in strict_child
    ]
    original_firstgen=[
        child for child,b in births.items()
        if b["generation"]==1 and child in strict_child
    ]
    children_of_qualified_firstgen=[
        child for child,b in births.items() if b["parent"] in original_firstgen
    ]
    return {
        "total_native_births":len(births),
        "first_generation_qualified_functional_native_children":len(original_firstgen),
        "second_or_later_generation_qualified_functional_native_children":
            len(source_function_grandchildren),
        "grandchildren_of_first_generation_qualified_native_children":
            len(children_of_qualified_firstgen),
        "grandchildren_complete_56_code":
            sum(births[c]["complete"] for c in children_of_qualified_firstgen),
        "grandchildren_inheriting_module_entry_at_birth":sum(
            births[c]["complete"] and births[c]["birth_PC4_edge"] in MODULES
            for c in children_of_qualified_firstgen
        ),
        "grandchildren_qualifying_expressed_source_plus_one":sum(
            c in strict_child for c in children_of_qualified_firstgen
        ),
        "functional_firstgen_children_with_own_native_PC4_edit":
            sum(bool(mutations.get(child)) for child in original_firstgen),
        "functional_firstgen_children_who_native_launch":sum(
            any(b["parent"]==child for b in births.values()) for child in original_firstgen
        ),
        "original_all_true_descendant_births":list(births.values()),
        "firstgen_functional_actors":sorted(original_firstgen),
        "higher_generation_strict_trait_actors":sorted(source_function_grandchildren),
        "native_code_allele_mutation_events_by_actor":mutations,
    }

def audit(original_db):
    checked=audit_original(original_db,REVISION)
    require(checked["complete_original_native_worlds"]==512
            and checked["complete_original_SHA256_epochs"]==8192,
            "Study115 source independent original full-state replay not verified")
    known={(w["seed"],w["arm"]):w
           for w in checked["all512_native_worlds_and_child_expression"]}
    require(len(known)==512,"duplicate previous audited original world")
    conn=sqlite3.connect(original_db)
    items=conn.execute(
        "SELECT condition,source_revision,final_json FROM epoch_runs ORDER BY id"
    ).fetchall()
    require(len(items)==512,"original source archive no longer complete")
    run={}
    for label,revision,payload in items:
        require(revision==REVISION and label.startswith("seed="),
                "source-original provenance mismatch")
        seed_s,arm=label[5:].split("/",1)
        seed=int(seed_s)
        require(seed in range(FIRST,FIRST+N) and arm in MODES
                and (seed,arm) not in run,"duplicated/unregistered initial target seed")
        already=known[(seed,arm)]
        g=genealogy(json.loads(payload),already["module_witnesses"])
        require(g["total_native_births"]==already["native_births"],
                "new code copied genealogy contradicts original SHA256 audit")
        require(g["first_generation_qualified_functional_native_children"]+
                g["second_or_later_generation_qualified_functional_native_children"]
                ==already["strict_unique_functional_inherited_child_count"],
                "new unique native inherited trait counts differ from original validated source")
        run[(seed,arm)]={"seed":seed,"arm":arm,
                         "initial_target":already["mutator_initial_target"],**g}
    by_source={}
    for source in (3,8):
        all_seed=[]
        for seed in range(FIRST,FIRST+N):
            original=run[(seed,f"native-latent-source{source}")]
            negative=[
                run[(seed,f"disabled-latent-source{source}")],
                run[(seed,f"native-inert-source{source}")],
                run[(seed,f"disabled-inert-source{source}")],
            ]
            require(not any(w["second_or_later_generation_qualified_functional_native_children"]>0
                            for w in negative),
                    "negative control developed unregistered inherited function")
            all_seed.append({
                "seed":seed,"original_mutation_target":original["initial_target"],
                "first_generation_qualified_functional_children":
                    original["first_generation_qualified_functional_native_children"],
                "grandchildren_from_those_functional_children":
                    original["grandchildren_of_first_generation_qualified_native_children"],
                "complete_grandchildren":
                    original["grandchildren_complete_56_code"],
                "grandchildren_with_module_entry_inherited_at_birth":
                    original["grandchildren_inheriting_module_entry_at_birth"],
                "grandchildren_actually_executing_inherited_function":
                    original["grandchildren_qualifying_expressed_source_plus_one"],
                "all_higher_generations_with_strict_function":
                    original["second_or_later_generation_qualified_functional_native_children"],
                "firstgen_native_PC4_mutation_exposure":
                    original["functional_firstgen_children_with_own_native_PC4_edit"],
            })
        by_source[f"source{source}"]={
            "all_original64_target_genotype_intention_to_treat":all_seed,
            "first_generation_seed_positive":sum(x["first_generation_qualified_functional_children"]>0 for x in all_seed),
            "first_generation_strict_functional_children":sum(x["first_generation_qualified_functional_children"] for x in all_seed),
            "grandchildren_born_to_strict_functional_children":sum(x["grandchildren_from_those_functional_children"] for x in all_seed),
            "those_grandchildren_whose_native_birth_code_copies_all56_nodes":sum(x["complete_grandchildren"] for x in all_seed),
            "those_grandchildren_inheriting_PC4_module_start_at_birth":sum(x["grandchildren_with_module_entry_inherited_at_birth"] for x in all_seed),
            "those_grandchildren_executing_original_inherited_function":sum(x["grandchildren_actually_executing_inherited_function"] for x in all_seed),
            "all_second_or_later_generation_strict_functional_actor_count":sum(x["all_higher_generations_with_strict_function"] for x in all_seed),
            "first_generation_strict_functional_children_who_execute_another_native_PC4_edit":sum(x["firstgen_native_PC4_mutation_exposure"] for x in all_seed),
        }
    return {
        "status":"POST_OUTCOME_ORIGINAL_FIRSTGEN_TO_GRANDCHILD_LATENT_CIL_TRAIT_PERSISTENCE_NOT_EVOLUTION",
        "source_revision":REVISION,
        "original_independent_seed_labels":64,
        "original_complete_worlds":512,
        "original_full_epochs_cryptographically_reverified":8192,
        "new_random_genotypes":0,
        "source_worlds_dependent":True,
        "by_source":by_source,
        "all512_copied_native_actor_genealogies":[
            run[(seed,arm)] for seed in range(FIRST,FIRST+N) for arm in MODES
        ],
        "CL1_life_gate":False,
    }

def main():
    parser=argparse.ArgumentParser()
    parser.add_argument("original_sqlite",type=Path)
    parser.add_argument("--json",type=Path,required=True)
    a=parser.parse_args()
    report=audit(a.original_sqlite)
    a.json.write_text(json.dumps(report,indent=2,sort_keys=True)+"\n")
    print(json.dumps({
        "status":report["status"],
        "original_complete_worlds":report["original_complete_worlds"],
        "original_full_epochs_cryptographically_reverified":report["original_full_epochs_cryptographically_reverified"],
        "source_summary":{
            source:{k:v for k,v in results.items() if k!="all_original64_target_genotype_intention_to_treat"}
            for source,results in report["by_source"].items()
        }
    },sort_keys=True))

if __name__=="__main__":
    main()
