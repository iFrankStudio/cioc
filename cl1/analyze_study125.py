#!/usr/bin/env python3
"""Study125: independent read-only audit of original same-world CIL competitor genealogy.

All 256 original source/edit/slot-order paired worlds. No genotype/postselection.
"""
import argparse
import copy
import hashlib
import json
import sqlite3
from collections import Counter, defaultdict
from pathlib import Path

FIRST=7_000_000
N=32
EPOCHS=16
QUOTA=250
MODES=tuple(
    f"{mutation}-source{source}-{order}"
    for mutation in ("native-edit","patch-disabled")
    for source in (3,8)
    for order in ("Afirst","Bfirst")
)

def require(x,message):
    if not x:
        raise ValueError(message)

def sha(data):
    return hashlib.sha256(data.encode("utf-8")).hexdigest()

def part(event,key):
    value=event["kind"]
    return value.get(key) if isinstance(value,dict) else None

def codehash(code):
    return sha(json.dumps(code,sort_keys=True))

def trace(initial,final,mode,attempts):
    first=initial["processes"]["1"]["code"]["nodes"]
    second=initial["processes"]["2"]["code"]["nodes"]
    require(len(first)==len(second)==28,"original unequal code length")
    require(first!=second,"original distinct competitor code missing")
    require(all(first[i]==second[i] for i in range(28) if i!=24),
            "founders differ beyond registered PC24 cost")
    d1=first[24]["Const"]["value"]
    d2=second[24]["Const"]["value"]
    a_first=mode.endswith("Afirst")
    delay_a=d1 if a_first else d2
    delay_b=d2 if a_first else d1
    require(1<=delay_a<=16 and delay_b==delay_a+16,
            "competitor delay advantage not original registered 16")
    require(first[7]==second[7] and first[20]==second[20],
            "shared original site/grant differs")
    source=3 if "-source3-" in mode else 8
    disabled=mode.startswith("patch-disabled")
    require(initial["cells"]["7"]==source and 0<=initial["cells"]["11"]<28,
            "source or mutation target initial state invalid")
    require(init_bool(initial,"native_patchnext_disabled")==disabled
            and initial["trace_native_executable_edits"] is True,
            "native mutation gate or executable edit provenance missing")
    require(initial["compute_pool"]==800
            and initial["processes"]["1"]["credits"]==500
            and initial["processes"]["2"]["credits"]==500,
            "initial competitor resource inequality")
    founders={1:"A" if a_first else "B",2:"B" if a_first else "A"}
    codes={1:copy.deepcopy(first),2:copy.deepcopy(second)}
    born_code={}
    born_changed={}
    born_full={}
    lineage=dict(founders)
    depth={1:0,2:0}
    graphs={}
    births_by=Counter()
    complete_birth_by=Counter()
    canon_by=Counter()
    mutant_function_by=Counter()
    actor_function=set()
    actor_read={}
    expressed=set()
    child_by=defaultdict(list)
    edits=0
    suppressed=0
    traps=0
    mutation_birth_count=0
    incomplete_births=0
    recorded_native=0
    source8_and_3_regardless=0
    for ev in final["events"]:
        origin=ev["origin"]
        if origin=="External":
            continue
        actor=ev["process"]
        require(origin=={"Internal":actor} and actor in codes,
                "source-native event missing valid known actor")
        recorded_native+=1
        pc=ev["pc"]
        require(0<=pc<len(codes[actor]),"native PC outside real actor executable")
        # Mark expression from original immutable code image at the exact time
        # of executing the inherited mutant site, not a later rewritten node.
        if (actor in born_changed and pc in born_changed[actor]
            and part(ev,"Trapped") is None and (
            pc<len(born_code[actor]) and codes[actor][pc]==born_code[actor][pc]
        ):
            expressed.add(actor)
        read=part(ev,"CellRead")
        wr=part(ev,"CellWritten")
        edit=part(ev,"ExecutableEdgePatched")
        supp=part(ev,"NativeCodeEditSuppressed")
        new=part(ev,"GraphCreated")
        copied=part(ev,"GraphNodeCopied")
        launch=part(ev,"GraphLaunched")
        if read is not None and read["key"]==7 and pc==2:
            if read["value"]==source:
                actor_read[actor]=ev["tick"]
        if wr is not None and wr["key"]==9 and pc==3 and wr["value"]==source:
            if actor in actor_read and actor_read[actor]<ev["tick"]:
                actor_function.add(actor)
        if edit is not None:
            require(not disabled and pc==8 and "PatchNext" in codes[actor][8],
                    "unregistered native code edit provenance")
            idx=edit["node"]
            require(0<=idx<len(codes[actor]),"mutation outside executable")
            op=codes[actor][idx]
            require(len(op)==1,"malformed executable node")
            body=next(iter(op.values()))
            require(isinstance(body,dict) and body.get("next")==edit["before"]
                    and edit["before"]!=edit["after"] and 0<=edit["after"]<28,
                    "real native mutation old/next control mismatch")
            body["next"]=edit["after"]
            edits+=1
        if supp is not None:
            require(disabled and pc==8 and supp["opcode_tag"]==6,
                    "unexpected native edit suppression")
            suppressed+=1
        if new is not None:
            gid=new["graph"]
            require(new["owner"]==actor and gid not in graphs,
                    "nonowned graph or graph reuse")
            graphs[gid]={"owner":actor,"nodes":[],"source":[]}
        if copied is not None:
            gid=copied["graph"]
            require(pc==16 and copied["source_process"]==actor
                    and copied["owner"]==actor and gid in graphs
                    and graphs[gid]["owner"]==actor,
                    "invalid native graph-copy ownership")
            seq=graphs[gid]["source"]
            require(copied["new_node"]==len(seq)
                    and 0<=copied["source_node"]<len(codes[actor]),
                    "graph code copy source or destination incorrect")
            seq.append(copied["source_node"])
            graphs[gid]["nodes"].append(copy.deepcopy(codes[actor][copied["source_node"]]))
        if launch is not None:
            require(pc==22 and launch["parent_process"]==actor,
                    "birth wrong native parent/PC")
            gid=launch["source_graph"]
            child=launch["child_process"]
            require(gid in graphs and graphs[gid]["owner"]==actor
                    and child not in lineage and child>=3,
                    "child not born from owned CIL graph")
            body=copy.deepcopy(graphs[gid]["nodes"])
            require(0<len(body)==launch["moved_nodes"]<=28,
                    "native child graph size differs from owned copied nodes")
            require(0<=launch["transferred_credits"]<=7800,
                    "invalid native local credit transfer")
            codes[child]=body
            lineage[child]=lineage[actor]
            depth[child]=depth[actor]+1
            child_by[actor].append(child)
            births_by[lineage[child]]+=1
            parent_founder=first if lineage[child]==lineage[1] else second
            original_founder=first if lineage[child]==founders[1] else second
            # Prefer explicit ancestry mapping to founders across order swaps.
            original_founder=(first if lineage[child]==lineage[1] else second)
            changed={i for i in range(28)
                     if i>=len(body) or original_founder[i]!=body[i]}
            born_code[child]=copy.deepcopy(body)
            born_changed[child]=changed
            full=(len(body)==28 and graphs[gid]["source"]==list(range(28)))
            born_full[child]=full
            if full:
                complete_birth_by[lineage[child]]+=1
            else:
                incomplete_births+=1
            if changed:
                mutation_birth_count+=1
        if part(ev,"Trapped") is not None:
            traps+=1

    require(recorded_native==attempts,
            "native event count differs from paid CIL attempts")
    require(len(codes)==len(final["processes"]),
            "reconstructed native child actor count differs from actual final actors")
    for pid,body in codes.items():
        require(final["processes"][str(pid)]["code"]["nodes"]==body,
                f"native VM current executable image is not source-replay actor {pid}")
    require(not(disabled and edits) and not(not disabled and suppressed),
            "wrong mutation-only ablation law")
    # Compute actor-level functional descendant counts from actual charged
    # PC2 source reads preceding same-actor PC3 writes. Never infer from code.
    functional_by=Counter()
    functional_mutant_by=Counter()
    reproduce_functional_mutants_by=Counter()
    for pid in lineage:
        if pid<3:
            continue
        label=lineage[pid]
        if pid in actor_function and born_full[pid]:
            functional_by[label]+=1
        if (pid in actor_function and pid in expressed and born_changed[pid]
            and born_full[pid]):
            functional_mutant_by[label]+=1
            if child_by[pid]:
                reproduce_functional_mutants_by[label]+=1
    require(all(v>=0 for v in births_by.values()),"negative native births")
    saturation=sum(len(final["processes"])>=64 for _ in range(1))
    return {
        "source":source,"mutations_disabled":disabled,
        "founder_order":"Afirst" if a_first else "Bfirst",
        "initial_A_code_hash":codehash(first if a_first else second),
        "initial_B_code_hash":codehash(second if a_first else first),
        "short_delay":delay_a,"long_delay":delay_b,
        "original_native_births_A":births_by["A"],
        "original_native_births_B":births_by["B"],
        "native_full_copied_births_A":complete_birth_by["A"],
        "native_full_copied_births_B":complete_birth_by["B"],
        "functional_descendants_A":functional_by["A"],
        "functional_descendants_B":functional_by["B"],
        "functional_inherited_mutants_A":functional_mutant_by["A"],
        "functional_inherited_mutants_B":functional_mutant_by["B"],
        "functional_inherited_mutant_reproducers_A":reproduce_functional_mutants_by["A"],
        "functional_inherited_mutant_reproducers_B":reproduce_functional_mutants_by["B"],
        "deepest_reproducing_generation_A":max(
            [depth[p] for p,children in child_by.items()
             if lineage[p]=="A" and children],default=0),
        "deepest_reproducing_generation_B":max(
            [depth[p] for p,children in child_by.items()
             if lineage[p]=="B" and children],default=0),
        "native_mutations":edits,"suppressed_native_edits":suppressed,
        "real_changed_code_births":mutation_birth_count,
        "incomplete_native_code_births":incomplete_births,
        "native_traps":traps,
        "process_cap_saturated":bool(saturation),
        "native_paid_attempts":attempts,
        "final_shared_pool":final["compute_pool"],
    }

def init_bool(world,key):
    return world.get(key,False)

def audit(path,revision):
    db=sqlite3.connect(path)
    entries=db.execute(
        "SELECT id,condition,source_revision,initial_json,initial_sha256,"
        "actions_json,attempts_json,final_json,final_sha256 "
        "FROM epoch_runs ORDER BY id"
    ).fetchall()
    require(len(entries)==N*len(MODES)==256,"missing original 256 same-world CIL competition arms")
    results={}
    pair_genotypes=defaultdict(set)
    snapshots=0
    paid_total=0
    for rid,label,rev,initial_raw,initial_hash,actions_raw,attempts_raw,final_raw,final_hash in entries:
        require(rev==revision and len(revision)==40,
                "original source revision invalid or changed")
        require(label.startswith("seed="),"unknown original seed label")
        seed_str,arm=label[5:].split("/",1)
        seed=int(seed_str)
        require(FIRST<=seed<FIRST+N and arm in MODES and (seed,arm) not in results,
                "missing/duplicated original paired experimental condition")
        require(sha(initial_raw)==initial_hash and sha(final_raw)==final_hash,
                "original full-world initial/final SHA256 invalid")
        init=json.loads(initial_raw)
        end=json.loads(final_raw)
        src=3 if "-source3-" in arm else 8
        code1=init["processes"]["1"]["code"]["nodes"]
        code2=init["processes"]["2"]["code"]["nodes"]
        a_first=arm.endswith("Afirst")
        source_a=code1 if a_first else code2
        source_b=code2 if a_first else code1
        pair_genotypes[seed].add((
            codehash(source_a),codehash(source_b),init["cells"]["11"]))
        actions=json.loads(actions_raw)
        require(len(actions)==15,"not exactly 15 unconditional ambient supply interventions")
        ss=db.execute(
            "SELECT epoch,state_sha256,state_json FROM epoch_snapshots "
            "WHERE run_id=? ORDER BY epoch",(rid,)
        ).fetchall()
        require(len(ss)==EPOCHS,"all16 exact original full-world states required")
        for i,(ep,digest,raw) in enumerate(ss):
            require(ep==i and sha(raw)==digest,
                    "original exact full-world epoch sha256 verification failed")
            snapshots+=1
        require(ss[-1][2]==final_raw,"original last epoch not actual saved final")
        for i,act in enumerate(actions,1):
            previous=json.loads(ss[i-1][2])
            require(act["epoch"]==i
                    and act["action"]=={"SetComputePool":{
                        "credits":previous["compute_pool"]+400}},
                    "public compute pulses must be unconditional, not actor targeted")
        attempts=json.loads(attempts_raw)
        require(len(attempts)==EPOCHS and all(0<=a<=QUOTA for a in attempts),
                "native CIL charged attempt quotas outside registered bounds")
        cost=sum(attempts)
        paid_total+=cost
        require(sum(e["origin"]=="External" for e in end["events"])==20,
                "external code injections or incorrect public compute pulse count")
        require(end["compute_pool"]+sum(p["credits"] for p in end["processes"].values())
                ==7800-cost,"original shared compute conservation failed")
        result=trace(init,end,arm,cost)
        require(result["source"]==src,"environment source mismatch")
        results[(seed,arm)]={"seed":seed,"arm":arm,**result}
    require(len(results)==256 and len(pair_genotypes)==32
            and all(len(x)==1 for x in pair_genotypes.values()),
            "not all original seed competitors and slot controls matched")
    full=[]
    for seed in range(FIRST,FIRST+N):
        for source in (3,8):
            for order in ("Afirst","Bfirst"):
                yes=results[(seed,f"native-edit-source{source}-{order}")]
                no=results[(seed,f"patch-disabled-source{source}-{order}")]
                d_yes=yes["original_native_births_A"]-yes["original_native_births_B"]
                d_no=no["original_native_births_A"]-no["original_native_births_B"]
                full.append({
                    "seed":seed,"source":source,"order":order,
                    "enabled_native_birth_difference_A_minus_B":d_yes,
                    "disabled_native_birth_difference_A_minus_B":d_no,
                    "editing_interaction_difference":d_yes-d_no,
                    "enabled_canonical_functional_A_minus_B":
                        yes["functional_descendants_A"]-yes["functional_descendants_B"],
                    "disabled_canonical_functional_A_minus_B":
                        no["functional_descendants_A"]-no["functional_descendants_B"],
                    "enabled_functional_mutant_reproducers_A":
                        yes["functional_inherited_mutant_reproducers_A"],
                    "enabled_functional_mutant_reproducers_B":
                        yes["functional_inherited_mutant_reproducers_B"],
                    "rank_reversal":d_yes*d_no<0,
                })
    require(len(full)==128,"lost original genotype/source/order paired ecology")
    high=[]
    for seed in range(FIRST,FIRST+N):
        cells=[next(x for x in full if x["seed"]==seed and x["source"]==source
                    and x["order"]==order)
               for source in (3,8) for order in ("Afirst","Bfirst")]
        if all(
            x["editing_interaction_difference"]>0
            and (x["enabled_canonical_functional_A_minus_B"]-
                 x["disabled_canonical_functional_A_minus_B"])>0
            and x["enabled_functional_mutant_reproducers_A"]>0
            for x in cells
        ):
            high.append(seed)
    return {
        "status":"ORIGINAL_SAME_WORLD_COMPETITION_NATIVE_CIL_NOT_LIFE",
        "source_revision":revision,
        "original_competitor_pairs":N,
        "original_native_competition_worlds":len(entries),
        "original_complete_sha256_epochs":snapshots,
        "paid_native_attempts":paid_total,
        "distinct_competitor_pair_states":len(set(next(iter(v)) for v in pair_genotypes.values())),
        "full_256_original_world_results":[
            results[(seed,mode)] for seed in range(FIRST,FIRST+N) for mode in MODES
        ],
        "full_128_original_source_order_paired_editing_interactions":full,
        "source_order_robust_strict_functional_A_advantage_seeds":high,
        "primary_editing_effect_signs": {
            f"source{source}-{order}":{
                "positive":sum(x["editing_interaction_difference"]>0 for x in full
                    if x["source"]==source and x["order"]==order),
                "negative":sum(x["editing_interaction_difference"]<0 for x in full
                    if x["source"]==source and x["order"]==order),
                "ties":sum(x["editing_interaction_difference"]==0 for x in full
                    if x["source"]==source and x["order"]==order),
                "rank_reversals":sum(x["rank_reversal"] for x in full
                    if x["source"]==source and x["order"]==order),
            } for source in (3,8) for order in ("Afirst","Bfirst")
        },
        "original_saturated_worlds":sum(r["process_cap_saturated"] for r in results.values()),
        "organismic_origin_or_open_ended_evolution_proven":False,
    }

def main():
    p=argparse.ArgumentParser()
    p.add_argument("original_sqlite",type=Path)
    p.add_argument("--expected-revision",required=True)
    p.add_argument("--json",required=True,type=Path)
    args=p.parse_args()
    result=audit(args.original_sqlite,args.expected_revision)
    args.json.write_text(json.dumps(result,indent=2,sort_keys=True)+"\n")
    print(json.dumps({
        "status":result["status"],
        "original_competitor_pairs":result["original_competitor_pairs"],
        "original_native_competition_worlds":result["original_native_competition_worlds"],
        "original_complete_sha256_epochs":result["original_complete_sha256_epochs"],
        "original_saturated_worlds":result["original_saturated_worlds"],
        "primary_editing_effect_signs":result["primary_editing_effect_signs"],
        "source_order_robust_strict_functional_A_advantage_seeds":
            result["source_order_robust_strict_functional_A_advantage_seeds"],
    },sort_keys=True))

if __name__=="__main__":
    main()
