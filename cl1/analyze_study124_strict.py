#!/usr/bin/env python3
"""Study124 PROSPECTIVE COMPLETE-STATE GENOTYPE AUDIT actual SAME-CIL-actor functional mutant reproduction.

512 NEW prospectively frozen worlds. Independently verifies every original Study124
SHA256 world, then reconstructs source CIL executable at every inherited birth.
"""
import argparse
import copy
import json
import sqlite3
from collections import defaultdict
from pathlib import Path

from analyze_study124 import audit as original_audit

SOURCE = None  # source SHA provided via required invocation
SEEDS = range(6_900_000, 6_900_128)
MODES = ("native-edit-source3","patch-disabled-source3",
         "native-edit-source8","patch-disabled-source8")

def require(condition, message):
    if not condition:
        raise ValueError(message)

def get(event, name):
    k = event["kind"]
    return k.get(name) if isinstance(k,dict) else None

def reconstruct(initial, final, source, ablated):
    original=initial["processes"]["1"]["code"]["nodes"]
    require(initial["processes"]["2"]["code"]["nodes"]==original,
            "initial founder code not matched")
    codes={1:copy.deepcopy(original),2:copy.deepcopy(original)}
    graph={}
    births={}
    child_ids=defaultdict(list)
    seen_read={}
    canonical_writers=set()
    expressed_inherited=set()
    edits=0
    traps=0
    actual_births=0
    for e in final["events"]:
        if e["origin"]=="External":
            continue
        actor=e["process"]
        require(e["origin"]=={"Internal":actor} and actor in codes,
                "unattributed native actor CIL execution")
        pc=e["pc"]
        # Proof of expression is ACTUAL VM execution of a node whose
        # instruction content is identical to a CHANGED code node at birth.
        birth=births.get(actor)
        if (birth and pc in birth["changed_nodes"]
                and pc<len(codes[actor])
                and pc<len(birth["born_code"])
                and codes[actor][pc]==birth["born_code"][pc]):
            expressed_inherited.add(actor)
        read=get(e,"CellRead")
        if read and read["key"]==7 and pc==2:
            if read["value"]==source:
                seen_read[actor]=e["tick"]
        wr=get(e,"CellWritten")
        if (wr and wr["key"]==9 and pc==3 and wr["value"]==source
                and actor in seen_read and seen_read[actor]<e["tick"]):
            canonical_writers.add(actor)
        edit=get(e,"ExecutableEdgePatched")
        if edit:
            require(not ablated, "unregistered effective native code mutation in disabled world")
            loc=edit["node"]
            require(0<=loc<len(codes[actor]), "native code edit out of bounds")
            old_node=codes[actor][loc]
            require(len(old_node)==1, "malformed native executable node")
            body=next(iter(old_node.values()))
            require(isinstance(body,dict) and "next" in body,
                    "unpatchable actual typed CIL instruction mutated")
            require(body["next"]==edit["before"],"true edit previous edge wrong")
            require(edit["before"]!=edit["after"],"no-op edit mislabeled effective")
            body["next"]=edit["after"]
            edits+=1
        created=get(e,"GraphCreated")
        if created is not None:
            gid=created["graph"]
            require(gid not in graph and created["owner"]==actor,"duplicate host or foreign graph")
            graph[gid]={"owner":actor,"code":[],"sources":[]}
        copied=get(e,"GraphNodeCopied")
        if copied is not None:
            gid=copied["graph"]
            require(gid in graph and graph[gid]["owner"]==actor
                    and copied["owner"]==actor
                    and copied["source_process"]==actor,
                    "cross-owner/host graph-source birth")
            idx=copied["source_node"]
            nodes=graph[gid]["code"]
            require(0<=idx<len(codes[actor]) and copied["new_node"]==len(nodes),
                    "wrong native GraphCopyNode provenance")
            nodes.append(copy.deepcopy(codes[actor][idx]))
            graph[gid]["sources"].append(idx)
        launched=get(e,"GraphLaunched")
        if launched is not None:
            require(launched["parent_process"]==actor,"native child parent mismatch")
            gid=launched["source_graph"]
            child=launched["child_process"]
            require(gid in graph and graph[gid]["owner"]==actor
                    and child not in codes and child>=3,
                    "foreign/externally created or reused child CIL code")
            birth_code=copy.deepcopy(graph[gid]["code"])
            require(0 < len(birth_code)==launched["moved_nodes"]<=len(original),
                    "native birth graph byte count differs from real moved code")
            codes[child]=copy.deepcopy(birth_code)
            changed={i for i in range(len(original))
                     if i>=len(birth_code) or original[i]!=birth_code[i]}
            births[child]={
                "id":child,
                "parent":actor,
                "born_tick":e["tick"],
                "changed_nodes":changed,
                "born_code":birth_code,
                "whole_source_copied":
                    graph[gid]["sources"] == list(range(len(original))),
            }
            child_ids[actor].append(child)
            actual_births+=1
        if get(e,"Trapped") is not None:
            traps+=1
    require(len(codes)==len(final["processes"]), "lost process CIL ancestry")
    for actor,code in codes.items():
        require(code==final["processes"][str(actor)]["code"]["nodes"],
                f"final native code does not match actor original event-replay {actor}")
    if ablated:
        require(edits==0 and not any(b["changed_nodes"] for b in births.values()),
                "mutation-only ablation produced altered-code child")
    # Establish every genotype's birth attributes BEFORE walking its later
    # genealogy; otherwise a parent can inspect a grandchild before that later
    # dictionary entry has received its own born_full_length flag.
    for info in births.values():
        info["born_changed"]=bool(info["changed_nodes"])
        info["born_full_length"]=len(info["born_code"])==len(original)
    for info in births.values():
        actor=info["id"]
        info["executed_inherited_changed_code"]=actor in expressed_inherited
        info["canonical_source_function"]=actor in canonical_writers
        info["native_child_count"]=len(child_ids[actor])
        info["same_actor_strict_functional_mutant_reproducer"]=(
            info["born_changed"]
            and info["born_full_length"]
            and info["whole_source_copied"]
            and info["executed_inherited_changed_code"]
            and info["canonical_source_function"]
            and info["native_child_count"]>0
        )
        info["changed_canonical_grandchildren"]=sum(
            bool(births[grandchild]["changed_nodes"])
            and births[grandchild]["born_full_length"]
            and births[grandchild]["whole_source_copied"]
            and grandchild in canonical_writers
            for grandchild in child_ids[actor]
        )
        info["two_generation_functional_mutant_heredity"]=(
            info["same_actor_strict_functional_mutant_reproducer"]
            and info["changed_canonical_grandchildren"]>0
        )
    return {
        "actual_native_births":actual_births,
        "source_function_writers":len(canonical_writers),
        "actual_native_edit_events":edits,
        "changed_code_native_children":sum(b["born_changed"] for b in births.values()),
        "executed_inherited_changed_code_actors":sum(
            b["executed_inherited_changed_code"] for b in births.values()),
        "strict_same_actor_functional_mutant_reproducers":[
            actor for actor,b in births.items()
            if b["same_actor_strict_functional_mutant_reproducer"]
        ],
        "two_generation_functional_mutant_lineages":[
            actor for actor,b in births.items()
            if b["two_generation_functional_mutant_heredity"]
        ],
        "canonical_mutated_nonfounder_writers":sum(
            b["born_changed"] and b["canonical_source_function"]
            for b in births.values()),
        "all_native_descendant_births":[
            {key:(sorted(value) if isinstance(value,set) else value)
             for key,value in info.items() if key!="born_code"}
            for info in births.values()
        ],
        "native_traps":traps,
    }

def audit(original_sqlite, revision):
    verified=original_audit(original_sqlite,revision)
    require(verified["matched_worlds"]==512
            and verified["exact_original_SHA256_epochs"]==8192,
            "original whole-population Study124 audit not complete")
    original_by_pair={
        (x["seed"],x["arm"]):x
        for x in verified["all_unfiltered_original_world_results"]
    }
    require(len(original_by_pair)==512,"original source report missing seed/arm")
    conn=sqlite3.connect(original_sqlite)
    rows=conn.execute(
        "SELECT condition,initial_json,final_json FROM epoch_runs ORDER BY id"
    ).fetchall()
    require(len(rows)==512,"original Study124 512 complete worlds not found")
    all_rows={}
    for label, initial_json, final_json in rows:
        seed_s,arm=label.removeprefix("seed=").split("/",1)
        seed=int(seed_s)
        require(seed in SEEDS and arm in MODES and (seed,arm) not in all_rows,
                "unexpected source seed or duplicate paired trial")
        source=3 if arm.endswith("source3") else 8
        disabled=arm.startswith("patch-disabled")
        result=reconstruct(json.loads(initial_json),json.loads(final_json),source,disabled)
        prior=original_by_pair[(seed,arm)]
        require(result["actual_native_births"]==prior["births"]
                and result["changed_code_native_children"]==prior["changed_code_births"]
                and result["actual_native_edit_events"]==prior["changed_native_edit_events"]
                and result["source_function_writers"]==prior["native_function_writers"],
                "independent full-actor genealogy differs from original frozen observer")
        all_rows[(seed,arm)]={
            "seed":seed,"arm":arm,"source":source,
            "native_births":result["actual_native_births"],
            "changed_code_native_children":result["changed_code_native_children"],
            "canonical_mutated_nonfounder_writers":
                result["canonical_mutated_nonfounder_writers"],
            "strict_same_actor_functional_mutant_reproducers":
                result["strict_same_actor_functional_mutant_reproducers"],
            "two_generation_functional_mutant_lineages":
                result["two_generation_functional_mutant_lineages"],
            "all_native_descendant_births":result["all_native_descendant_births"],
        }
    paired=[]
    for seed in SEEDS:
        for source in (3,8):
            yes=all_rows[(seed,f"native-edit-source{source}")]
            no=all_rows[(seed,f"patch-disabled-source{source}")]
            delta=yes["native_births"]-no["native_births"]
            verified_delta=verified["comparison_by_source"][f"source{source}"][
                "full_128_seed_signed_birth_deltas"][seed-6_900_000]
            require(delta==verified_delta,"source original prespecified outcome altered")
            paired.append({
                "seed":seed,"source":source,
                "native_enabled_minus_disabled_birth_delta":delta,
                "native_enabled_births":yes["native_births"],
                "native_disabled_births":no["native_births"],
                "enabled_changed_code_native_children":yes["changed_code_native_children"],
                "enabled_canonical_changed_descendant_writers":
                    yes["canonical_mutated_nonfounder_writers"],
                "enabled_same_actor_functional_mutant_reproducer_ids":
                    yes["strict_same_actor_functional_mutant_reproducers"],
                "enabled_two_generation_functional_mutant_lineage_ids":
                    yes["two_generation_functional_mutant_lineages"],
            })
    qualifying = [
        seed for seed in SEEDS
        if all(
            next(r for r in paired if r["seed"]==seed and r["source"]==source)
            ["native_enabled_minus_disabled_birth_delta"]>0
            and next(r for r in paired if r["seed"]==seed and r["source"]==source)
            ["enabled_two_generation_functional_mutant_lineage_ids"]
            for source in (3,8))
    ]
    original_state_genotypes = {}
    for seed in SEEDS:
        item=original_by_pair[(seed,"native-edit-source3")]
        original_state_genotypes[seed]=(item["initial_code_hash"],item["initial_edge_target"])
    distinct_qualifying_genotypes=len(set(original_state_genotypes[seed] for seed in qualifying))
    return {
        "qualifying_complete_state_genotype_count":distinct_qualifying_genotypes,
        "qualified_seed_labels":qualifying,
        "strict_qualifying_seed_states":[
            {"seed":seed,"initial_code_hash":original_state_genotypes[seed][0],
             "initial_mutation_target":original_state_genotypes[seed][1]}
            for seed in qualifying
        ],
        "status":"PROSPECTIVE_UNSELECTED_NATIVE_WHOLE_CODE_FUNCTIONAL_LINEAGE_NOT_LIFE",
        "original_source_revision":revision,
        "new_seed_labels":128,
        "new_complete_state_genotype_units":
            verified["initial_distinct_code_plus_mutation_target_genotype_states"],
        "initial_complete_state_duplicate_groups":
            verified["original_duplicate_code_plus_target_state_seed_groups"],
        "original_seed_labels":128,
        "original_unique_executable_code_hashes":
            verified["initial_distinct_parameter_genomes"],
        "original_complete_state_valid_as_preregistered":
            verified["original_cohort_valid_as_preregistered"],
        "original_distinct_complete_state_genotype_count":
            verified["initial_distinct_code_plus_mutation_target_genotype_states"],
        "original_duplicate_code_hash_seed_groups":
            verified["original_duplicate_code_hash_seed_groups"],
        "original_128_distinct_seeded_chassis":128,
        "original_native_worlds":512,
        "original_complete_sha256_epochs":8192,
        "paired_source_outcomes":paired,
        "all_512_native_ancestry_rows":[
            all_rows[(s,a)] for s in SEEDS for a in MODES
        ],
        "positive_birth_seeds_both_sources":[
            seed for seed in SEEDS
            if all(next(r for r in paired if r["seed"]==seed and r["source"]==source)
                   ["native_enabled_minus_disabled_birth_delta"]>0
                   for source in (3,8))
        ],
        "positive_birth_and_strict_same_actor_both_sources":[
            seed for seed in SEEDS
            if all(
                next(r for r in paired if r["seed"]==seed and r["source"]==source)
                ["native_enabled_minus_disabled_birth_delta"]>0
                and next(r for r in paired if r["seed"]==seed and r["source"]==source)
                ["enabled_same_actor_functional_mutant_reproducer_ids"]
                for source in (3,8))
        ],
        "positive_birth_and_two_generation_heredity_both_sources":[
            seed for seed in SEEDS
            if all(
                next(r for r in paired if r["seed"]==seed and r["source"]==source)
                ["native_enabled_minus_disabled_birth_delta"]>0
                and next(r for r in paired if r["seed"]==seed and r["source"]==source)
                ["enabled_two_generation_functional_mutant_lineage_ids"]
                for source in (3,8))
        ],
        "cyberlife_proven":False,
    }

def main():
    p=argparse.ArgumentParser()
    p.add_argument("original_sqlite",type=Path)
    p.add_argument("--expected-revision",required=True)
    p.add_argument("--json",required=True,type=Path)
    args=p.parse_args()
    result=audit(args.original_sqlite,args.expected_revision)
    args.json.write_text(json.dumps(result,sort_keys=True,indent=2)+"\n")
    print(json.dumps({
        "status":result["status"],
        "original_native_worlds":result["original_native_worlds"],
        "original_complete_sha256_epochs":result["original_complete_sha256_epochs"],
        "positive_birth_seeds_both_sources":result["positive_birth_seeds_both_sources"],
        "positive_birth_and_strict_same_actor_both_sources":
            result["positive_birth_and_strict_same_actor_both_sources"],
        "positive_birth_and_two_generation_heredity_both_sources":
            result["positive_birth_and_two_generation_heredity_both_sources"],
    },sort_keys=True))

if __name__=="__main__":
    main()
