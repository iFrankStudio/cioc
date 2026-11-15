#!/usr/bin/env python3
"""Study130 PROSPECTIVE new-genotype original native 3-generation functional whole-allele heredity."""
import argparse
import copy
import hashlib
import json
import sqlite3
from collections import Counter, defaultdict
from pathlib import Path

from analyze_study130_world import audit as independent_original_audit, MODES

SOURCE=None  # passed as original GitHub Actions run source SHA
FIRST=7_400_000
N=32

def require(x,message):
    if not x:
        raise ValueError(message)

def sha(s):
    return hashlib.sha256(s.encode("utf-8")).hexdigest()

def hash_code(code):
    return sha(json.dumps(code,sort_keys=True))

def part(ev,key):
    k=ev["kind"]
    return k.get(key) if isinstance(k,dict) else None

def genealogy(initial,final,mode,expected_attempts):
    founders={
        1:"A" if mode.endswith("Afirst") else "B",
        2:"B" if mode.endswith("Afirst") else "A",
    }
    original_code={
        founders[1]:copy.deepcopy(initial["processes"]["1"]["code"]["nodes"]),
        founders[2]:copy.deepcopy(initial["processes"]["2"]["code"]["nodes"]),
    }
    source=3 if "-source3-" in mode else 8
    ablated=mode.startswith("patch-disabled")
    codes={pid:copy.deepcopy(initial["processes"][str(pid)]["code"]["nodes"])
           for pid in (1,2)}
    root=dict(founders)
    depth={1:0,2:0}
    birth_code={}
    birth_full={}
    born_different={}
    born_hash={}
    parents={}
    children=defaultdict(list)
    graphs={}
    reads={}
    canonical=set()
    expressed=set()
    mutation_count=0
    suppressed_count=0
    trapped=0
    native_events=0
    for ev in final["events"]:
        if ev["origin"]=="External":
            continue
        pid=ev["process"]
        require(ev["origin"]=={"Internal":pid} and pid in codes,
                "non-native actor attempted encoded CIL instruction")
        native_events+=1
        pc=ev["pc"]
        require(0<=pc<len(codes[pid]),
                "native actor PC invalid relative to actual current code")
        if pid in birth_code and pc in born_different[pid]:
            if (pc<len(birth_code[pid])
                and codes[pid][pc]==birth_code[pid][pc]
                and part(ev,"Trapped") is None):
                expressed.add(pid)
        read=part(ev,"CellRead")
        if read is not None and pc==2 and read["key"]==7 and read["value"]==source:
            reads[pid]=ev["tick"]
        wr=part(ev,"CellWritten")
        if (wr is not None and pc==3 and wr["key"]==9 and wr["value"]==source
            and pid in reads and reads[pid]<ev["tick"]):
            canonical.add(pid)
        edit=part(ev,"ExecutableEdgePatched")
        if edit is not None:
            require(not ablated and pc==8 and "PatchNext" in codes[pid][8],
                    "real native mutation occurs in disabled world or nonnative PC")
            node=edit["node"]
            require(0<=node<len(codes[pid]),"mutation node out of bound")
            op=codes[pid][node]
            require(len(op)==1,"malformed actor executable CIL code")
            body=next(iter(op.values()))
            require(isinstance(body,dict) and body.get("next")==edit["before"]
                    and 0<=edit["after"]<28 and edit["after"]!=edit["before"],
                    "native edit before/after mismatched actual copied code")
            body["next"]=edit["after"]
            mutation_count+=1
        suppressed=part(ev,"NativeCodeEditSuppressed")
        if suppressed is not None:
            require(ablated and pc==8 and suppressed["opcode_tag"]==6,
                    "unregistered mutation suppression")
            suppressed_count+=1
        created=part(ev,"GraphCreated")
        if created is not None:
            gid=created["graph"]
            require(gid not in graphs and created["owner"]==pid,
                    "graph owner or graph ID re-used illegally")
            graphs[gid]={"owner":pid,"code":[],"source":[]}
        copied=part(ev,"GraphNodeCopied")
        if copied is not None:
            gid=copied["graph"]
            require(gid in graphs and graphs[gid]["owner"]==pid
                    and copied["owner"]==pid and copied["source_process"]==pid
                    and copied["new_node"]==len(graphs[gid]["code"]),
                    "graph code copied from host/foreign source or incorrect index")
            idx=copied["source_node"]
            require(0<=idx<len(codes[pid]),"source node outside current native program")
            graphs[gid]["code"].append(copy.deepcopy(codes[pid][idx]))
            graphs[gid]["source"].append(idx)
        launched=part(ev,"GraphLaunched")
        if launched is not None:
            require(launched["parent_process"]==pid and pc==22,
                    "actor native birth from unregistered parent/PC")
            child=launched["child_process"]
            gid=launched["source_graph"]
            require(gid in graphs and graphs[gid]["owner"]==pid
                    and child not in codes and child not in root and child>=3,
                    "native newborn source graph or process ID invalid")
            baby=copy.deepcopy(graphs[gid]["code"])
            require(0<len(baby)==launched["moved_nodes"]<=28,
                    "native moved graph size incorrect")
            codes[child]=copy.deepcopy(baby)
            root[child]=root[pid]
            parents[child]=pid
            depth[child]=depth[pid]+1
            children[pid].append(child)
            birth_code[child]=baby
            born_hash[child]=hash_code(baby)
            birth_full[child]=(len(baby)==28
                and graphs[gid]["source"]==list(range(28)))
            orig=original_code[root[child]]
            born_different[child]={
                i for i in range(28)
                if i>=len(baby) or baby[i]!=orig[i]
            }
        if part(ev,"Trapped") is not None:
            trapped+=1

    require(native_events==expected_attempts,
            "original whole-world CIL native paid event count not matched")
    require(len(codes)==len(final["processes"]),
            "native process ancestry lost or externally fabricated")
    for pid,code in codes.items():
        require(code==final["processes"][str(pid)]["code"]["nodes"],
                "original typed executable graph not reconstructed from native actor events")
    require(not(ablated and mutation_count) and not(not ablated and suppressed_count),
            "native edit ablation provenance incorrect")
    full_births=Counter()
    functional_full_births=Counter()
    functional_full_by_generation=Counter()
    functional_allele_by_generation=Counter()
    functional_mutant_births=Counter()
    functional_expressed_mutants=Counter()
    carriers=defaultdict(list)
    for pid in parents:
        label=root[pid]
        if not birth_full[pid]:
            continue
        full_births[label]+=1
        if pid not in canonical:
            continue
        functional_full_births[label]+=1
        functional_full_by_generation[depth[pid]]+=1
        functional_allele_by_generation[(depth[pid],born_hash[pid])]+=1
        if born_different[pid]:
            functional_mutant_births[label]+=1
            carriers[(label,born_hash[pid])].append(pid)
            if pid in expressed:
                functional_expressed_mutants[label]+=1
    native_links=[]
    for pid in parents:
        parent=parents[pid]
        if parent<3 or not birth_full[pid] or not birth_full[parent]:
            continue
        if (pid not in canonical or parent not in canonical
            or parent not in expressed
            or not born_different[pid] or not born_different[parent]
            or born_hash[pid]!=born_hash[parent]):
            continue
        native_links.append({
            "root":root[pid],"allele_sha256":born_hash[pid],
            "parent":parent,"child":pid,
            "parent_generation":depth[parent],
            "child_generation":depth[pid],
        })
    persistent_alleles=set((x["root"],x["allele_sha256"]) for x in native_links)
    links_by_parent=defaultdict(list)
    for x in native_links:
        links_by_parent[x["parent"]].append(x["child"])
    three_generation=[]
    for link in native_links:
        for grandchild in links_by_parent[link["child"]]:
            if born_hash[grandchild]==link["allele_sha256"]:
                three_generation.append({
                    "root":link["root"],"allele_sha256":link["allele_sha256"],
                    "first":link["parent"],"second":link["child"],
                    "third":grandchild,
                })
    # Prospectively registered whole-executable birth-cohort frequency gate.
    # Every frequency denominator counts all *functional* full-copy native
    # newborns across BOTH competing true founder ancestry labels, not only
    # the mutated H carriers and not only the retrospective favorable seeds.
    three_gen_growth=[]
    for chain in three_generation:
        h=chain["allele_sha256"]
        first_gen=depth[chain["first"]]
        all_original_h_carriers=[
            p for p in parents if birth_full[p] and p in canonical
            and born_hash[p]==h
        ]
        earliest=min((depth[p] for p in all_original_h_carriers),default=None)
        if earliest is None or earliest!=first_gen:
            continue
        g=(first_gen,first_gen+1,first_gen+2)
        ns=[functional_allele_by_generation[(i,h)] for i in g]
        totals=[functional_full_by_generation[i] for i in g]
        if not all(t>0 and n>0 for t,n in zip(totals,ns)):
            continue
        growing=ns[0]*totals[1]<ns[1]*totals[0]
        nondecreasing=ns[1]*totals[2]<=ns[2]*totals[1]
        if growing and nondecreasing:
            proof={
                "allele_sha256":h,"root_founder_label":chain["root"],
                "three_native_actor_ids":[chain["first"],chain["second"],chain["third"]],
                "original_depths":list(g),
                "functional_allele_born_carriers_at_depth":ns,
                "all_full_functional_born_carriers_at_depth":totals,
            }
            if proof not in three_gen_growth:
                three_gen_growth.append(proof)
    # Direct same-founder, same-real-generation *functional parent* rate.
    # Each parent in the denominator stays even if it later traps or has
    # zero faithfully copied functional immediate children. Do not conflate
    # an allele's total frequency with native offspring produced per bearer.
    rate_matches=[]
    for proof in three_gen_growth:
        allele=proof["allele_sha256"]
        founder_label=proof["root_founder_label"]
        ancestor_hash=hash_code(original_code[founder_label])
        for generation in sorted(set(depth.values())):
            def eligible(pid):
                return (pid in parents and birth_full[pid]
                        and pid in canonical
                        and root[pid]==founder_label
                        and depth[pid]==generation)
            h_parents=[pid for pid in parents if eligible(pid)
                       and born_hash[pid]==allele]
            anc_parents=[pid for pid in parents if eligible(pid)
                         and born_hash[pid]==ancestor_hash]
            if not h_parents or not anc_parents:
                continue
            def faithful_functional_children(parents_list):
                return sum(
                    birth_full.get(child,False) and child in canonical
                    for parent in parents_list for child in children[parent]
                )
            h_children=faithful_functional_children(h_parents)
            ancestor_children=faithful_functional_children(anc_parents)
            higher=h_children*len(anc_parents)>ancestor_children*len(h_parents)
            rate_matches.append({
                "allele_sha256":allele,
                "true_founder_lineage":founder_label,
                "native_birth_depth":generation,
                "mutant_functional_parents":len(h_parents),
                "ancestral_functional_parents":len(anc_parents),
                "mutant_faithful_functional_immediate_children":h_children,
                "ancestral_faithful_functional_immediate_children":ancestor_children,
                "native_mutant_rate_strictly_higher":higher,
                "all_mutant_parent_ids":h_parents,
                "all_ancestral_parent_ids":anc_parents,
            })
    if ablated:
        require(not persistent_alleles and all(
            not born_different[pid] for pid in parents if birth_full[pid]),
            "native edit-disabled original graph produced mutated faithful executable genotype")
    per_allele=[
        {
            "root":label,"allele_sha256":h,"full_functional_carriers":len(pids),
            "native_actor_ids":pids,
            "carrier_generations":[depth[pid] for pid in pids],
            "native_same_allele_functional_parent_child_edges":sum(
                x["root"]==label and x["allele_sha256"]==h for x in native_links),
            "native_three_functional_generation_chains":sum(
                x["root"]==label and x["allele_sha256"]==h
                for x in three_generation),
        }
        for (label,h),pids in sorted(carriers.items())
    ]
    return {
        "original_source":source,
        "original_mutation_disabled":ablated,
        "actor_native_edit_events":mutation_count,
        "actor_native_edit_suppressed_events":suppressed_count,
        "native_births_total":len(parents),
        "native_births_A":sum(root[pid]=="A" for pid in parents),
        "native_births_B":sum(root[pid]=="B" for pid in parents),
        "fully_native_copied_births_A":full_births["A"],
        "fully_native_copied_births_B":full_births["B"],
        "full_code_canonically_functional_births_A":functional_full_births["A"],
        "full_code_canonically_functional_births_B":functional_full_births["B"],
        "functionally_expressed_mutant_births_A":functional_expressed_mutants["A"],
        "functionally_expressed_mutant_births_B":functional_expressed_mutants["B"],
        "original_distinct_changed_functional_alleles":len(carriers),
        "original_functional_changed_allele_native_parent_child_edges":len(native_links),
        "original_persistent_functional_changed_alleles":len(persistent_alleles),
        "original_functional_changed_allele_three_gen_chains":len(three_generation),
        "native_functional_births_by_depth":dict(sorted(functional_full_by_generation.items())),
        "original_frequency_growing_three_generation_alleles":three_gen_growth,
        "original_frequency_growing_three_generation_allele_count":
            len(set(x["allele_sha256"] for x in three_gen_growth)),
        "original_same_founder_depth_rate_control_comparisons":rate_matches,
        "original_same_founder_depth_eligible_control_count":len(rate_matches),
        "original_mutant_alleles_with_rate_advantage":
            len(set(x["allele_sha256"] for x in rate_matches
                    if x["native_mutant_rate_strictly_higher"])),
        "persistent_allele_source_nontargeted_native_edges":native_links,
        "three_functional_generations_same_allele":three_generation,
        "all_original_functional_allele_carrier_descriptions":per_allele,
        "actor_native_traps":trapped,
    }

def audit(path,source_revision):
    parent=independent_original_audit(path,source_revision)
    require(parent["original_native_competition_worlds"]==256
            and parent["original_complete_sha256_epochs"]==12288
            and parent["original_competitor_pairs"]==32,
            "full original archived Study130 source and SHA audit not passed")
    conn=sqlite3.connect(path)
    rows=conn.execute(
        "SELECT condition,initial_json,final_json,attempts_json "
        "FROM epoch_runs ORDER BY id"
    ).fetchall()
    require(len(rows)==256,"entire new native competing genotype cohort missing")
    outcomes={}
    for label,first,last,attempts in rows:
        require(label.startswith("seed="),"unexpected source code sample")
        seed_s,mode=label.removeprefix("seed=").split("/",1)
        seed=int(seed_s)
        require(seed in range(FIRST,FIRST+N) and mode in MODES
                and (seed,mode) not in outcomes,"unregistered original seed or mode")
        original=genealogy(json.loads(first),json.loads(last),mode,
                           sum(json.loads(attempts)))
        outcomes[(seed,mode)]=original
    require(len(outcomes)==256,"original full denominator not completely classified")
    rows_out=[]
    for seed in range(FIRST,FIRST+N):
        for source in (3,8):
            for order in ("Afirst","Bfirst"):
                positive=outcomes[(seed,f"native-edit-source{source}-{order}")]
                neg=outcomes[(seed,f"patch-disabled-source{source}-{order}")]
                seed_alleles=positive["original_persistent_functional_changed_alleles"]
                require(neg["original_persistent_functional_changed_alleles"]==0,
                        "whole-code allele falsely created in native edit-disabled control")
                rows_out.append({
                    "seed":seed,"source":source,"order":order,
                    "original_mutated_persistent_functional_alleles":seed_alleles,
                    "original_mutated_functional_parent_child_edges":
                        positive["original_functional_changed_allele_native_parent_child_edges"],
                    "original_mutated_three_functional_generations":
                        positive["original_functional_changed_allele_three_gen_chains"],
                    "original_birth_cohort_frequency_growing_alleles":
                        positive["original_frequency_growing_three_generation_allele_count"],
                    "same_founder_depth_mutant_alleles_with_real_parent_rate_advantage":
                        positive["original_mutant_alleles_with_rate_advantage"],
                    "eligible_same_founder_same_depth_allele_rate_controls":
                        positive["original_same_founder_depth_eligible_control_count"],
                    "canonical_functional_expressed_mutants_A":
                        positive["functionally_expressed_mutant_births_A"],
                    "canonical_functional_expressed_mutants_B":
                        positive["functionally_expressed_mutant_births_B"],
                })
    all_4_seeds=[
        seed for seed in range(FIRST,FIRST+N)
        if all(next(row for row in rows_out if row["seed"]==seed
                    and row["source"]==source and row["order"]==order)
               ["original_mutated_persistent_functional_alleles"]>0
               for source in (3,8) for order in ("Afirst","Bfirst"))
    ]
    all_4_three=[
        seed for seed in range(FIRST,FIRST+N)
        if all(next(row for row in rows_out if row["seed"]==seed
                    and row["source"]==source and row["order"]==order)
               ["original_mutated_three_functional_generations"]>0
               for source in (3,8) for order in ("Afirst","Bfirst"))
    ]
    all_4_frequency_growth=[
        seed for seed in range(FIRST,FIRST+N)
        if all(next(row for row in rows_out if row["seed"]==seed
                    and row["source"]==source and row["order"]==order)
               ["original_birth_cohort_frequency_growing_alleles"]>0
               for source in (3,8) for order in ("Afirst","Bfirst"))
    ]
    rate_advantage_all4=[
        seed for seed in range(FIRST,FIRST+N)
        if all(next(row for row in rows_out if row["seed"]==seed
                    and row["source"]==source and row["order"]==order)
               ["same_founder_depth_mutant_alleles_with_real_parent_rate_advantage"]>0
               for source in (3,8) for order in ("Afirst","Bfirst"))
    ]
    return {
        "status":"PROSPECTIVE_SAME_FOUNDER_DEPTH_MUTANT_VS_ANCESTRAL_NATIVE_REPRODUCTIVE_RATE_NOT_CAUSAL_SELECTION",
        "source_revision":source_revision,
        "new_original_pair_genotypes":32,
        "original_worlds":256,
        "original_epoch_sha256_reverified":12288,
        "new_genotype_pair_seed_labels":32,
        "independent_prospective_three_generation_fidelity_gate_passed":
            len(all_4_three)>0,
        "original_seed_pair_labels_with_functional_allele_persistence_all_4_conditions":
            all_4_seeds,
        "original_seed_pair_labels_with_three_generation_functional_allele_chain_all_4_conditions":
            all_4_three,
        "original_seed_pair_labels_with_3gen_primary_pass":all_4_three,
        "original_seed_pair_labels_with_3gen_increasing_allele_frequency_all_4_conditions":
            all_4_frequency_growth,
        "original_seed_pair_labels_with_same_founder_depth_rate_advantage_all_4_conditions":
            rate_advantage_all4,
        "primary_precommitted_same_founder_depth_rate_advantage_passed":
            len(rate_advantage_all4)>0,
        "all_128_original_source_order_eligible_control_comparisons":
            sum(x["eligible_same_founder_same_depth_allele_rate_controls"]
                for x in rows_out),
        "primary_precommitted_birth_cohort_allele_frequency_growth_passed":
            len(all_4_frequency_growth)>0,
        "original_source_order_allele_3gen_success_count":sum(
            row["original_mutated_three_functional_generations"]>0 for row in rows_out
        ),
        "original_full_256_world_native_alleles":[
            {"seed":seed,"arm":arm,**outcomes[(seed,arm)]}
            for seed in range(FIRST,FIRST+N) for arm in MODES
        ],
        "original_128_source_order_allele_persistence_comparisons":rows_out,
        "open_ended_selection_or_cyberlife_proven":False,
    }

def main():
    p=argparse.ArgumentParser()
    p.add_argument("original",type=Path)
    p.add_argument("--expected-revision",required=True)
    p.add_argument("--json",type=Path,required=True)
    a=p.parse_args()
    result=audit(a.original,a.expected_revision)
    a.json.write_text(json.dumps(result,sort_keys=True,indent=2)+"\n")
    print(json.dumps({
        "status":result["status"],
        "new_original_pair_genotypes":result["new_original_pair_genotypes"],
        "original_worlds":result["original_worlds"],
        "original_epoch_sha256_reverified":result["original_epoch_sha256_reverified"],
        "all4_persistent_functional_allele_seeds":
            result["original_seed_pair_labels_with_functional_allele_persistence_all_4_conditions"],
        "all4_three_generation_functional_allele_seeds":
            result["original_seed_pair_labels_with_three_generation_functional_allele_chain_all_4_conditions"],
        "independent_prospective_three_generation_fidelity_gate_passed":
            result["independent_prospective_three_generation_fidelity_gate_passed"],
        "original_seed_pair_labels_with_3gen_increasing_allele_frequency_all_4_conditions":
            result["original_seed_pair_labels_with_3gen_increasing_allele_frequency_all_4_conditions"],
        "primary_precommitted_birth_cohort_allele_frequency_growth_passed":
            result["primary_precommitted_birth_cohort_allele_frequency_growth_passed"],
        "same_founder_depth_rate_advantage_all4_seeds":
            result["original_seed_pair_labels_with_same_founder_depth_rate_advantage_all_4_conditions"],
        "primary_precommitted_same_founder_depth_rate_advantage_passed":
            result["primary_precommitted_same_founder_depth_rate_advantage_passed"],
        "all_128_eligible_control_comparisons":
            result["all_128_original_source_order_eligible_control_comparisons"],
    },sort_keys=True))

if __name__=="__main__":
    main()
