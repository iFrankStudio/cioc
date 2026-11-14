#!/usr/bin/env python3
"""Study117 unselected all-target native activation of PREAUTHORED latent CIL function.

Independent original-SQLite SHA256 and charged resource audit. Reuses only the
original Study114 event-level code-copy reconstructor for the 56-node mechanism,
not its study outcomes, while adding *birth-allele and child-specific* module assay.
"""
import argparse
import hashlib
import json
import sqlite3
from collections import Counter, defaultdict
from pathlib import Path

from analyze_study114 import actual_actor_history
from analyze_study115r import genealogy

FIRST=7_000_000
N=64
EPOCHS=16
QUOTA=250
MODULES=(29,35,41,47)
MODES=(
    "native-cap4-source3","disabled-cap4-source3",
    "native-cap5-source3","disabled-cap5-source3",
    "native-cap4-source8","disabled-cap4-source8",
    "native-cap5-source8","disabled-cap5-source8",
)

def check(value, explanation):
    if not value:
        raise ValueError(explanation)

def sha(value):
    return hashlib.sha256(value.encode("utf-8")).hexdigest()

def part(event,name):
    k=event["kind"]
    return k.get(name) if isinstance(k,dict) else None

def latent_nodes(nodes, latent):
    check(len(nodes)==56, "not complete source 56-node CIL")
    check(nodes[7]=={"Const":{"dst":1,"value":4,"next":28}},
          "actor mutator did not enter original one-shot permission gate")
    check(nodes[8]=={"PatchNext":{"node":1,"target":0,"next":55}},
          "genuine native code edit must rejoin single-shot target-shutdown route")
    check(nodes[28]=={"Const":{"dst":2,"value":13,"next":53}},
          "one-shot gate address constant missing")
    check(nodes[9]=={"Const":{"dst":1,"value":0,"next":10}}
          and nodes[10]=={"Sub":{"dst":3,"lhs":3,"rhs":3,"next":11}}
          and nodes[11]=={"Write":{"address":2,"value":1,"next":12}},
          "actor native gate shutdown writes not frozen")
    check(nodes[14]=={"Const":{"dst":3,"value":56,"next":15}},
          "native code copy limit not 56 nodes")
    for s in MODULES:
        if latent:
            expected=[
                {"Const":{"dst":2,"value":7,"next":s+1}},
                {"Read":{"dst":0,"address":2,"next":s+2}},
                {"Const":{"dst":3,"value":10,"next":s+3}},
                {"Const":{"dst":2,"value":1,"next":s+4}},
                {"Add":{"dst":0,"lhs":0,"rhs":2,"next":s+5}},
                {"Write":{"address":3,"value":0,"next":5}},
            ]
        else:
            expected=[{"Const":{"dst":0,"value":0,"next":i}} for i in range(s,s+6)]
        check(nodes[s:s+6]==expected,
              f"latent vs inert source module {s} was not frozen")
    check(nodes[53]=={"Read":{"dst":3,"address":2,"next":54}}
          and nodes[54]=={"Branch":{"test":3,"nonzero":8,"zero":55}}
          and nodes[55]=={"Const":{"dst":2,"value":13,"next":9}},
          "native gate switch and actor-paid branch not original")

def expression_at_native_child_birth(state, source, latent):
    """Chronological native-code allele and six-step module witness for each child."""
    edge={1:24,2:24}
    birth={}
    source_read_tick={}
    module_state={}
    outputs=[]
    all_pc8_edits=Counter()
    correct_source_function=set()
    incorrect_source_outputs=0
    for e in state["events"]:
        if e["origin"]=="External":
            continue
        actor=e["process"]
        check(e["origin"]=={"Internal":actor} and actor in edge,
              "native CIL actor has no grounded ancestor")
        pc=e["pc"]
        read=part(e,"CellRead")
        write=part(e,"CellWritten")
        edit=part(e,"ExecutableEdgePatched")
        launch=part(e,"GraphLaunched")
        if read is not None and read["key"]==7 and pc==2 and read["value"]==source:
            source_read_tick.setdefault(actor,e["tick"])
        if write is not None and write["key"]==9:
            if (pc==3 and write["value"]==source and actor in source_read_tick
                    and e["tick"]>source_read_tick[actor]):
                correct_source_function.add(actor)
            else:
                incorrect_source_outputs+=1

        # Verify exact six native events in the actor's own sequence.
        current=module_state.get(actor)
        if current is not None:
            s,index,start=current
            if pc==s+index:
                good=True
                if index==1:
                    good=read is not None and read["key"]==7 and read["value"]==source
                elif index==4:
                    good=e["kind"]=="Executed"
                elif index==5:
                    good=write is not None and write["key"]==10 and write["value"]==source+1
                else:
                    good=e["kind"]=="Executed"
                if good:
                    if index==5:
                        newborn=actor in birth
                        child=birth.get(actor)
                        qualifying=(
                            newborn and child["complete"]
                            and child["birth_edge"]==s
                            and child["born_tick"]<start
                            and not child["changed_pc4_after_birth"]
                        )
                        outputs.append({
                            "actor":actor,"module":s,"start_tick":start,
                            "write_tick":e["tick"],"native_child":newborn,
                            "source":source,"derived_output":source+1,
                            "full_56_node_child_with_inherited_active_edge":bool(qualifying),
                        })
                        module_state.pop(actor,None)
                    else:
                        module_state[actor]=(s,index+1,start)
                    current=None
                else:
                    module_state.pop(actor,None)
            else:
                module_state.pop(actor,None)
        if pc in MODULES and current is not None:
            # A completed module is not accidentally reinterpreted twice.
            pass
        if pc in MODULES and e["kind"]=="Executed":
            module_state[actor]=(pc,1,e["tick"])

        if edit is not None:
            check(pc==8, "mutated executable not a native PatchNext instruction")
            if edit["node"]==4:
                check(edit["before"]==edge[actor],
                      "native PC4 code-edge previous allele contradicts reconstructed source")
                edge[actor]=edit["after"]
                if actor in birth:
                    birth[actor]["changed_pc4_after_birth"]=True
            all_pc8_edits[actor]+=1
        if launch is not None:
            child=launch["child_process"]
            check(pc==22 and launch["parent_process"]==actor
                  and child not in edge and child>=3,
                  "native launch has invalid parent-child ancestry")
            edge[child]=edge[actor]
            birth[child]={
                "parent":actor,"born_tick":e["tick"],
                "birth_edge":edge[child],
                "code_nodes":launch["moved_nodes"],
                "complete":launch["moved_nodes"]==56,
                "changed_pc4_after_birth":False,
            }
    return {
        "all_verified_six_step_source_plus_one_native_executions":len(outputs),
        "founder_six_step_module_executions":sum(not r["native_child"] for r in outputs),
        "child_six_step_module_executions":sum(r["native_child"] for r in outputs),
        "strict_complete_inherited_active_module_child_executions":sum(
            r["full_56_node_child_with_inherited_active_edge"] for r in outputs
        ),
        "strict_unique_functional_inherited_child_count":len({
            r["actor"] for r in outputs if r["full_56_node_child_with_inherited_active_edge"]
        }),
        "strict_functional_inherited_child_which_later_reproduced":len({
            r["actor"] for r in outputs if r["full_56_node_child_with_inherited_active_edge"]
            and any(b["parent"]==r["actor"] for b in birth.values())
        }),
        "native_birth_count_from_genuine_event_ancestry":len(birth),
        "complete_birth_count":sum(b["complete"] for b in birth.values()),
        "native_births_with_latent_PC4_edge":sum(
            b["complete"] and b["birth_edge"] in MODULES for b in birth.values()
        ),
        "canonical_source_function_actor_count":len(correct_source_function),
        "noncanonical_source9_output_events":incorrect_source_outputs,
        "actual_mutation_events_per_actor":dict(all_pc8_edits),
        "module_witnesses":outputs,
    }

def audit(db,revision):
    con=sqlite3.connect(db)
    records=con.execute(
        "SELECT id,condition,source_revision,initial_json,initial_sha256,"
        "actions_json,attempts_json,final_json,final_sha256 "
        "FROM epoch_runs ORDER BY id"
    ).fetchall()
    check(len(records)==N*len(MODES), "original 512 native world denominator incomplete")
    rows={}
    genomes=defaultdict(lambda:defaultdict(set))
    params=defaultdict(set)
    snapshots=0
    charged=0
    for (rid,label,rev,initial_raw,initial_sha,actions_raw,
         attempts_raw,final_raw,final_sha) in records:
        check(rev==revision and len(revision)==40,"original commit pin missing")
        check(label.startswith("seed="),"unregistered world label")
        seed_s,arm=label[5:].split("/",1)
        seed=int(seed_s)
        check(seed in range(FIRST,FIRST+N) and arm in MODES
              and (seed,arm) not in rows,"unregistered genotype/arm")
        check(sha(initial_raw)==initial_sha and sha(final_raw)==final_sha,
              "original full initial/final SHA256 corrupted")
        init=json.loads(initial_raw)
        state=json.loads(final_raw)
        source=3 if arm.endswith("source3") else 8
        latent=True
        capacity=4 if "-cap4-" in arm else 5
        ablated=arm.startswith("disabled-")
        check(len(init["processes"])==2 and
              init["processes"]["1"]["code"]==init["processes"]["2"]["code"],
              "initial code differs between founders")
        check(all(init["processes"][str(pid)]["credits"]==500 for pid in (1,2)),
              "original founder compute incorrectly initialized")
        check(init["cells"]["7"]==source and 0<=init["cells"]["11"]<56
              and init["cells"]["13"]==1
              and init["compute_pool"]==800,
              "original sensor, unselected code target or single-shot gate not initialized")
        check(init.get("native_patchnext_disabled",False)==ablated
              and init["trace_native_executable_edits"] is True,
              "original mutation law not proper narrow ablation")
        original_code=init["processes"]["1"]["code"]
        nodes=original_code["nodes"]
        latent_nodes(nodes,latent)
        check(init["storage_limit"]==capacity and
              init["max_nodes_per_process"]==128 and
              init["total_storage_limit"]==4096,
              "actual same-genome native world storage capacity is not the declared 4-vs-5-only law")
        grant=nodes[20]["Const"]["value"]
        delay=nodes[24]["Const"]["value"]
        check(320<=grant<=480 and 1<=delay<=32,
              "out-of-registration CIL grant/delay")
        code_hash=sha(json.dumps(original_code,sort_keys=True))
        genomes[seed]["source_code"].add(code_hash)
        params[seed].add((init["cells"]["11"],grant,delay))
        states=con.execute(
            "SELECT epoch,state_sha256,state_json FROM epoch_snapshots "
            "WHERE run_id=? ORDER BY epoch",(rid,)
        ).fetchall()
        check(len(states)==EPOCHS,"missing original 16 epochs")
        actions=json.loads(actions_raw)
        check(len(actions)==15,"ambient public supply epoch action count wrong")
        for i,action in enumerate(actions,1):
            check(action["epoch"]==i and "SetComputePool" in action["action"],
                  "unregistered external intervention")
            before=json.loads(states[i-1][2])["compute_pool"]
            check(action["action"]["SetComputePool"]["credits"]==before+400,
                  "public supply was non-generic or reward-targeted")
        for i,(ep,digest,body) in enumerate(states):
            check(ep==i and sha(body)==digest,
                  "original full epoch native world SHA256 mismatch")
            snapshots+=1
        check(states[-1][2]==final_raw,"last original epoch not final world")
        attempts=json.loads(attempts_raw)
        check(len(attempts)==16 and all(0<=v<=QUOTA for v in attempts),
              "unregistered charged attempt horizon")
        attempts_total=sum(attempts)
        charged+=attempts_total
        check(state["compute_pool"]+sum(
            p["credits"] for p in state["processes"].values()
        )==7800-attempts_total,"native charge and all ambient source credits not conserved")
        check(sum(e["origin"]=="External" for e in state["events"])==21,
              "host provided unexpected code or energy intervention")
        source_report=actual_actor_history(init,state,4,ablated,source,grant,attempts_total)
        expression=expression_at_native_child_birth(state,source,latent)
        family=genealogy(state,expression["module_witnesses"])
        gate_reads=[]
        gate_writes=[]
        noncanonical_gate_reads=[]
        noncanonical_gate_writes=[]
        native_edited=[]
        for ev in state["events"]:
            if ev["origin"]=="External":
                continue
            read=part(ev,"CellRead")
            written=part(ev,"CellWritten")
            edit=part(ev,"ExecutableEdgePatched")
            if read is not None and read["key"]==13:
                if ev["pc"]==53 and read["value"] in (0,1):
                    gate_reads.append((ev["tick"],ev["process"],read["value"]))
                else:
                    # Unselected code mutations can corrupt a Read's address
                    # register. These are REAL but NONCANONICAL reads, not
                    # proof that the intended mutagen permission gate ran.
                    noncanonical_gate_reads.append((ev["tick"],ev["process"],ev["pc"],read["value"]))
            if written is not None and written["key"]==13:
                if ev["pc"]==11 and written["value"]==0:
                    gate_writes.append((ev["tick"],ev["process"]))
                else:
                    # Real mutation-damaged paths can write other values to
                    # the permission cell. Never erase these worlds or falsely
                    # treat them as correctly actor-internal mutagen shutdown.
                    noncanonical_gate_writes.append((ev["tick"],ev["process"],ev["pc"],written["value"]))
            if edit is not None and edit["node"]==4:
                native_edited.append((ev["tick"],ev["process"]))
        check(len(gate_reads)>0 and len(gate_writes)>0,
              "actor-paid one-shot mutation gate not actually exercised")
        first_gate_shutdown=min(t for t,_ in gate_writes)
        resumed_mutations_after_shutdown=sum(
            t>first_gate_shutdown for t,_ in native_edited
        )
        # The strict inherited functional GRANDCHILD must belong to a chain
        # in which its parent actually expressed the module before reproducing
        # and the grandparent actually expressed the module as well. A bare
        # inherited edge without the three actor-specific expression witnesses
        # is not a multi-generation functional organism claim.
        birth_by_child={
            item["child"]:item
            for item in family["original_all_true_descendant_births"]
        }
        strict_actors={
            w["actor"] for w in expression["module_witnesses"]
            if w["full_56_node_child_with_inherited_active_edge"]
        }
        all_expressing_actors={
            w["actor"] for w in expression["module_witnesses"]
        }
        strict_three_generation=[]
        for child in strict_actors:
            child_birth=birth_by_child.get(child)
            if child_birth is None or child_birth["generation"]<2:
                continue
            parent=child_birth["parent"]
            parent_birth=birth_by_child.get(parent)
            if parent_birth is None:
                continue
            grandparent=parent_birth["parent"]
            if parent in strict_actors and grandparent in all_expressing_actors:
                strict_three_generation.append(child)
        check(expression["native_birth_count_from_genuine_event_ancestry"]==source_report["births"]
              and expression["complete_birth_count"]==
              source_report["births"]-source_report["incomplete_copy_births"],
              "derived native offspring not consistent with completely reconstructed copied codes")
        check(latent or expression["all_verified_six_step_source_plus_one_native_executions"]==0,
              "inert code unexpectedly expressed a prewritten latent function")
        check(not ablated or expression["strict_unique_functional_inherited_child_count"]==0,
              "narrow mutation disabled generated child with altered PC4")
        check({1,2}.issubset({
            ev["process"] for ev in state["events"]
            if isinstance(ev["kind"],dict) and "CellWritten" in ev["kind"]
            and ev["kind"]["CellWritten"].get("key")==9
            and ev["kind"]["CellWritten"].get("value")==source
            and ev["pc"]==3
        }),"initial founder source function missing")
        rows[(seed,arm)]={
            "seed":seed,"arm":arm,"source":source,
            "cell_capacity":capacity,"latent":latent,
            "native_edit_enabled":not ablated,"mutator_initial_target":init["cells"]["11"],
            "initial_code_hash":code_hash,"grant":grant,"delay":delay,
            "native_paid_attempts":attempts_total,
            "native_births":source_report["births"],
            "full_56_node_births":expression["complete_birth_count"],
            "traps":source_report["traps"],
            "actual_traps_at_module_write_instruction":sum(
                e["origin"]!=="External"
                and e["pc"] in (34,40,46,52)
                and part(e,"Trapped") is not None
                for e in state["events"]
            ),
            "module_write_key10_successful_native_events":sum(
                e["origin"]!="External" and
                e["pc"] in (34,40,46,52) and
                (part(e,"CellWritten") or {}).get("key")==10
                for e in state["events"]
            ),
            "real_edge_edits":source_report["changed_native_edit_events"],
            "source_integrity_writers":source_report["native_function_writers"],
            "native_actor_gate_reads_1":sum(x[2]==1 for x in gate_reads),
            "native_actor_gate_reads_0":sum(x[2]==0 for x in gate_reads),
            "native_actor_shutdown_zero_writes":len(gate_writes),
            "native_PC4_edits_after_first_shutdown":resumed_mutations_after_shutdown,
            "noncanonical_native_permission_reads":len(noncanonical_gate_reads),
            "noncanonical_native_permission_writes":len(noncanonical_gate_writes),
            "grandchildren_with_fully_expressed_3_generation_module_chain":
                len(strict_three_generation),
            "full_three_generation_module_actor_ids":sorted(strict_three_generation),
            "strict_functional_firstgen_children":family["first_generation_qualified_functional_native_children"],
            "strict_functional_grandchildren_or_later":family["second_or_later_generation_qualified_functional_native_children"],
            "grandchildren_born_from_functional_firstgen":family["grandchildren_of_first_generation_qualified_native_children"],
            "grandchildren_inheriting_latent_module_at_real_birth":family["grandchildren_inheriting_module_entry_at_birth"],
            "grandchildren_expr_derived_function":family["grandchildren_qualifying_expressed_source_plus_one"],
            "grandchild_code_complete":family["grandchildren_complete_56_code"],
            **expression,
        }
    check(len(rows)==512 and snapshots==8192,
          "original complete CIL state denominator corrupted")
    check(len(genomes)==N and
          all(len(v["source_code"])==1 for v in genomes.values()),
          "matched capacity/ablation worlds do NOT carry identical initial actor source genome")
    check(all(len(v)==1 for v in params.values()),
          "target, grant or slow-delay changed between matching capacity4 and5 original worlds")
    summary={}
    for source in (3,8):
        original=[]
        for seed in range(FIRST,FIRST+N):
            worlds={
                arm:rows[(seed,f"{arm}-source{source}")]
                for arm in ("native-cap4","disabled-cap4","native-cap5","disabled-cap5")
            }
            check(len({worlds[a]["initial_code_hash"] for a in worlds})==1,
                  "original within-seed source genome not byte-identical across physics")
            check(worlds["native-cap4"]["cell_capacity"]==4
                  and worlds["native-cap5"]["cell_capacity"]==5,
                  "capacity experimental physics reversed")
            record={
                "seed":seed,
                "initial_unselected_destination":worlds["native-cap5"]["mutator_initial_target"],
                "cap4_native_strict_functional_grandchildren":
                    worlds["native-cap4"]["strict_functional_grandchildren_or_later"],
                "cap5_native_strict_functional_grandchildren":
                    worlds["native-cap5"]["strict_functional_grandchildren_or_later"],
                "cap5_full_three_generation_expressed_module_chains":
                    worlds["native-cap5"]["grandchildren_with_fully_expressed_3_generation_module_chain"],
                "cap4_full_three_generation_expressed_module_chains":
                    worlds["native-cap4"]["grandchildren_with_fully_expressed_3_generation_module_chain"],
                "cap5_disabled_strict_functional_grandchildren":
                    worlds["disabled-cap5"]["strict_functional_grandchildren_or_later"],
                "cap4_disabled_strict_functional_grandchildren":
                    worlds["disabled-cap4"]["strict_functional_grandchildren_or_later"],
                "cap5_strict_firstgen_activated_module_children":
                    worlds["native-cap5"]["strict_functional_firstgen_children"],
                "cap4_strict_firstgen_activated_module_children":
                    worlds["native-cap4"]["strict_functional_firstgen_children"],
                "cap4_original_module_final_write_traps":
                    worlds["native-cap4"]["actual_traps_at_module_write_instruction"],
                "cap5_original_module_final_write_traps":
                    worlds["native-cap5"]["actual_traps_at_module_write_instruction"],
                "cap5_original_module_written_key10":
                    worlds["native-cap5"]["module_write_key10_successful_native_events"],
                "cap4_original_module_written_key10":
                    worlds["native-cap4"]["module_write_key10_successful_native_events"],
                "cap5_actual_native_births":
                    worlds["native-cap5"]["native_births"],
                "cap4_actual_native_births":
                    worlds["native-cap4"]["native_births"],
            }
            record["primary_grandchild_delta_capacity5_minus4"]=(
                record["cap5_native_strict_functional_grandchildren"]-
                record["cap4_native_strict_functional_grandchildren"]
            )
            check(record["cap5_disabled_strict_functional_grandchildren"]==0
                  and record["cap4_disabled_strict_functional_grandchildren"]==0,
                  "mutation-disabled original genome has unexplained native inherited module")
            original.append(record)
        deltas=[w["primary_grandchild_delta_capacity5_minus4"] for w in original]
        summary[f"source{source}"]={
            "full64_original_unfiltered_code_target_seed_rows":original,
            "original_target_draws_hitting_module_entry":sum(
                w["initial_unselected_destination"] in MODULES for w in original
            ),
            "primary_capacity5_minus4_strict_grandchildren_by_seed":deltas,
            "primary_positive":sum(v>0 for v in deltas),
            "primary_negative":sum(v<0 for v in deltas),
            "primary_ties":sum(v==0 for v in deltas),
            "strict_grandchildren_capacity5_enabled":sum(
                w["cap5_native_strict_functional_grandchildren"] for w in original
            ),
            "strict_grandchildren_capacity4_enabled":sum(
                w["cap4_native_strict_functional_grandchildren"] for w in original
            ),
            "full_three_generation_functionally_expressed_chains_capacity5":sum(
                w["cap5_full_three_generation_expressed_module_chains"] for w in original
            ),
            "seed_labels_with_full_three_generation_function_capacity5":sum(
                w["cap5_full_three_generation_expressed_module_chains"]>0 for w in original
            ),
            "module_Write10_native_traps_capacity4":sum(
                w["cap4_original_module_final_write_traps"] for w in original
            ),
            "module_Write10_native_traps_capacity5":sum(
                w["cap5_original_module_final_write_traps"] for w in original
            ),
            "actual_native_module_written_key10_capacity5":sum(
                w["cap5_original_module_written_key10"] for w in original
            ),
            "actual_native_module_written_key10_capacity4":sum(
                w["cap4_original_module_written_key10"] for w in original
            ),
            "source_genotypes_with_capacity5_real_strict_firstgen_derived_function":sum(
                w["cap5_strict_firstgen_activated_module_children"]>0 for w in original
            ),
            "all_capacity5_and4_original_code_hashes_match":True,
        }
    check(summary["source3"]["full64_original_unfiltered_code_target_seed_rows"] and
          [r["initial_unselected_destination"] for r in summary["source3"]["full64_original_unfiltered_code_target_seed_rows"]]
          ==[r["initial_unselected_destination"] for r in summary["source8"]["full64_original_unfiltered_code_target_seed_rows"]],
          "source input contexts incorrectly use different originally unselected target seeds")
    return {
        "status":"SAME_HUMAN_ORIGINAL_CIL_GENOME_STORAGE_4_VS_5_NATIVE_GERMLINE_NOT_EVOLUTION",
        "source_revision":revision,
        "original_seed_labels":N,
        "unique_initial_code_hashes":len({next(iter(v["source_code"])) for v in genomes.values()}),
        "complete_original_native_worlds":len(rows),
        "complete_original_SHA256_epochs":snapshots,
        "all_paid_native_attempts":charged,
        "primary_by_source":summary,
        "all512_native_worlds_with_true_code_and_function": [
            rows[(seed,arm)] for seed in range(FIRST,FIRST+N) for arm in MODES
        ],
        "original_code_spontaneously_generated":False,
        "cyberlife_proven":False,
    }

def main():
    p=argparse.ArgumentParser()
    p.add_argument("sqlite",type=Path)
    p.add_argument("--expected-revision",required=True)
    p.add_argument("--json",type=Path,required=True)
    args=p.parse_args()
    out=audit(args.sqlite,args.expected_revision)
    args.json.write_text(json.dumps(out,indent=2,sort_keys=True)+"\n")
    print(json.dumps({
        "status":out["status"],
        "complete_original_native_worlds":out["complete_original_native_worlds"],
        "complete_original_SHA256_epochs":out["complete_original_SHA256_epochs"],
        "unique_initial_code_hashes":out["unique_initial_code_hashes"],
        "primary_by_source":{
            source:{k:v for k,v in val.items()
                    if k!="full64_original_unfiltered_code_target_seed_rows"
                    and k!="primary_capacity5_minus4_strict_grandchildren_by_seed"}
            for source,val in out["primary_by_source"].items()
        },
    },sort_keys=True))

if __name__=="__main__":
    main()
