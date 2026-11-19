#!/usr/bin/env python3
"""I5 independent all-original-native step replay and one-byte repair bound.

Uses the prospectively frozen *source* rather than native output as its
instruction stream. For 64 separately initialized original PID1 genotypes and
four adversarial controls, independently recomputes each paid CIL opcode,
register, cell presence/value, private credits, dynamically selected source
word, normalized RESET3 gene fallback, all three fresh certificate witnesses,
and every environment feedback SetCell20 following paid Write11.

The same independent oracle enumerates every possible single-word edit, thus
proving the finite one-edit upper bound of 32/40 initial incomplete rules.
"""
import json
import os
import sys
from pathlib import Path

FROZEN=(Path(__file__).resolve().parents[1]/
    "studies/B56-F1d1k-G3Q-C1-P1-L3-I5-frozen-173-node-optimal-one-byte-repair-v1.json")
NAME="B56-F1d1k-G3Q-C1-P1-L3-I5"
DATA_KEYS=(7,8,9,11,20,21,32,33,50,52,56,62,63,64)
TASK_GOALS=(0,0,2,2)
PRIVATE_BUDGET=8000

def actual_modes(words):
    a,b,c=words
    values=[]
    for t in range(3):
        accum=0
        for idx,initial in enumerate((a,b,c)):
            op=(initial+t)%4 if idx==1 else initial
            accum=0 if op==3 else (accum+op)%3
        values.append(accum)
    return values

def truly_safe(words):
    return len(set(actual_modes(words)))==3

def exhaustive_single_edit_candidates(seed):
    options=[]
    for index in range(3):
        for proposed in range(4):
            if proposed==seed[index]:
                continue
            other=list(seed)
            other[index]=proposed
            if truly_safe(other):
                options.append({"source_word_index":index,"replacement":proposed})
    return options

def replay_founder(code_template,site,name,seed,feedback_regime,disable_edit,expected,trace=None):
    code=[dict(instr) for instr in code_template]
    for pc,val in ((0,seed[0]),(3,seed[1]),(6,seed[2])):
        code[pc]["value"]=val
    reg=[0,0,0,0]
    cells={}
    pc=0
    paid=0
    peak=len(code)+1
    witnesses=[]
    resets=[]
    certificates=[]
    patches=[]
    decisions=[]
    services=[]
    meta_reads=0
    feedback_reads=[]
    retry_writes=0

    def observed():
        if trace is None:
            while True:
                yield None
        else:
            with Path(trace).open() as stream:
                for line in stream:
                    yield json.loads(line)
    records=observed()
    def audit(expected_row):
        original=next(records,None)
        if trace is not None and original!=expected_row:
            raise AssertionError({
                "first_divergence_paid_step":paid,
                "arm":name,
                "native_original":original,
                "independent_CIL_source_replay":expected_row,
            })

    while paid<PRIVATE_BUDGET:
        op=code[pc]
        here=pc
        kind=op["kind"]
        paid+=1
        if kind=="Const":
            reg[op["dst"]]=op["value"]
            pc=op["next"]
        elif kind in ("Add","Sub"):
            lhs,rhs=reg[op["lhs"]],reg[op["rhs"]]
            reg[op["dst"]]=lhs+rhs if kind=="Add" else lhs-rhs
            pc=op["next"]
        elif kind=="Read":
            key=reg[op["address"]]
            reg[op["dst"]]=cells.get(key,0)
            if here==site["paid_meta_Read"]:
                assert key in (62,63,64)
                meta_reads+=1
            if here==site["task_feedback_Read20"]:
                assert key==20
                feedback_reads.append(reg[op["dst"]])
            pc=op["next"]
        elif kind=="Write":
            address=reg[op["address"]]
            value=reg[op["value"]]
            cells[address]=value
            if here==site["paid_witness_Write"]:
                assert address in (7,8,9) and value==1
                witnesses.append({"step":paid,"key":address,"value":value})
            if here in (site["paid_reset7"],site["paid_reset8"],site["paid_reset9"]):
                assert address in (7,8,9) and value==0
                resets.append({"step":paid,"key":address,"value":value})
            if here==site["paid_certificate_pass"]:
                assert address==21 and value==1
                assert [cells.get(k) for k in (7,8,9)]==[1,1,1]
                certificates.append(paid)
            if here==site["paid_retry_guard_Write11"]:
                assert address==11 and value==1
                retry_writes+=1
            pc=op["next"]
        elif kind=="Branch":
            if here==site["witness_count_target_chooser"]:
                assert reg[1] in (0,1)
                chosen=3 if reg[1] else 6
                assert not decisions and not certificates
                assert len(witnesses)==3
                distinct=len({x["key"] for x in witnesses})
                assert (distinct==1 and chosen==6) or (distinct==2 and chosen==3)
                decisions.append({"step":paid,"decision":"witness_count",
                    "register1":reg[1],"node_from_witness":chosen})
            if here==site["normalized_FIRST3_branch"]:
                assert len(decisions)==1 and decisions[0]["node_from_witness"]==6
                assert reg[0]==seed[0]-3
                decisions.append({"step":paid,
                    "decision":"first_gene_reset_normalization",
                    "first_gene":seed[0],"register0":reg[0],
                    "use_zero_fallback":reg[0]==0})
            pc=op["nonzero"] if reg[op["test"]] else op["zero"]
        elif kind=="PatchConst":
            assert here in (site["tail_patch_PatchConst"],
                site["middle_or_normalized_tail_PatchConst"])
            assert len(patches)==0 and len(decisions)>=1
            node=reg[op["node"]]
            desired=reg[op["value"]]
            assert node in (3,6)
            assert node==decisions[0]["node_from_witness"]
            if here==site["tail_patch_PatchConst"]:
                assert node==6 and seed[0]!=3 and desired==seed[0]
            else:
                assert desired==0
                assert (node==3 and len(decisions)==1) or (
                    node==6 and len(decisions)==2 and seed[0]==3)
            before=code[node]["value"]
            if not disable_edit:
                code[node]["value"]=desired
            patches.append({"step":paid,"pc":here,"node":node,
                "before":before,"after":code[node]["value"],
                "edit_suppressed":disable_edit})
            pc=op["next"]
        elif kind=="Halt":
            assert code[here]["kind"]=="Halt" # never stale hardcoded PCs
            pc=here
        else:
            raise AssertionError(("unsupported frozen original CIL",here,kind))

        peak=max(peak,len(code)+1+len(cells))
        assert len(cells)<=16 and peak<=300
        audit(["cil",paid,here,pc,list(reg),PRIVATE_BUDGET-paid,
            [cells.get(k) for k in DATA_KEYS],len(code)+1+len(cells),
            code[3]["value"],code[6]["value"]])

        if here==site["task_Write11"]:
            assert kind=="Write" and address==11 and len(certificates)==1
            episode=cells.get(32,0)
            assert 0<=episode<4
            actual=int(value==TASK_GOALS[episode])
            assert feedback_regime in ("truth","inverted")
            signal=actual if feedback_regime=="truth" else 1-actual
            middle=cells[63]
            services.append({"step":paid,"episode":episode,
                "middle":middle,"output":value,"truth":actual,"feedback":signal})
            cells[20]=signal
            peak=max(peak,len(code)+1+len(cells))
            audit(["feedback",paid,episode,TASK_GOALS[episode],middle,value,
                actual,signal,[cells.get(k) for k in DATA_KEYS],
                len(code)+1+len(cells),code[3]["value"],code[6]["value"]])

        if kind=="Halt":
            break
    else:
        raise AssertionError(("original PID failed to Halt under fixed compute budget",name))
    if trace is not None:
        assert next(records,None) is None,("extra native trace after original Halt",name)
    assert len(certificates)<=1 and len(patches)<=1
    assert len(patches)==retry_writes
    assert len(resets)==3*len(patches)
    assert len(witnesses)==3*(1+len(patches))
    assert meta_reads==3*(len(witnesses)+len(services))
    assert feedback_reads==[x["feedback"] for x in services]

    final=[code[0]["value"],code[3]["value"],code[6]["value"]]
    semantic=actual_modes(final)
    assert truly_safe(final)==(len(certificates)==1)
    result={
        "paid_steps":paid,"halt_pc":pc,"credits_remaining":PRIVATE_BUDGET-paid,
        "source_after":final,"semantic_three_modes":semantic,
        "semantic_structural_coverage":len(set(semantic))==3,
        "original_paid_witnesses":witnesses,
        "original_paid_full_reset":resets,
        "original_paid_certification":certificates,
        "original_paid_source_patches":patches,
        "original_evidence_decisions":decisions,
        "original_paid_task_services":services,
        "original_paid_meta_reads":meta_reads,
        "original_paid_feedback_reads":feedback_reads,
        "original_paid_retry_writes":retry_writes,
        "actor_claimed_success":cells.get(21),
        "actor_completed_episodes":cells.get(32),
        "occupied_data_cells":len(cells),"peak_occupied":peak,
    }
    for field,val in result.items():
        if val!=expected[field]:
            raise AssertionError(("precommitted I5 forecast mismatch",name,field,val,expected[field]))
    return result

def main():
    frozen=json.loads(FROZEN.read_text())
    source=frozen["exact_source_CIL"]
    sites=frozen["critical_source_pcs"]
    arms=frozen["pre_registered_64_original_genotypes"]
    controls=frozen["pre_registered_controls"]
    assert len(source)==173 and len(arms)==64 and len(controls)==4
    native=None
    if len(sys.argv)==3:
        native=json.loads(Path(sys.argv[1]).read_text())
        assert native["study"]==NAME
        assert native["level"]=="NATIVE_173_NODE_GENOME_NORMALIZED_ONE_EDIT_EXHAUSTIVE64_OPTIMAL_REPAIR"
        assert native["source_sha"] not in ("","local")
        if os.environ.get("GITHUB_SHA"):
            assert native["source_sha"]==os.environ["GITHUB_SHA"]
        assert len(native["original64_worlds"])==64
        assert len(native["controls"])==4
        trace_dir=Path(sys.argv[2])
    elif len(sys.argv)!=1:
        raise SystemExit("Usage: python3 I5_REPLAY.py [NATIVE_SUMMARY.json NATIVE_TRACE_DIR]")
    total_original_steps=0
    rescued=0
    edited=0
    target_counts={3:0,6:0}
    high_value=[]
    theoretical_possible=0

    for index,arm in enumerate(arms):
        seed=arm["seed"]
        assert index==(seed[0]*4+seed[1])*4+seed[2]
        name=f"a{seed[0]}_b{seed[1]}_c{seed[2]}"
        assert arm["initial_safe"]==truly_safe(seed)
        possible=exhaustive_single_edit_candidates(seed)
        assert possible==arm["one_edit_safe_candidates"],(
            "precommitted independent one-edit upper bound mismatch",seed)
        theoretical_possible+=int(not arm["initial_safe"] and bool(possible))
        path=(trace_dir/f"b56_f1d1k_g3q_c1_p1_l3_i5_native_{name}.jsonl") if native else None
        result=replay_founder(source,sites,name,seed,"truth",False,arm["expected"],path)
        total_original_steps+=result["paid_steps"]
        rescued+=int(not arm["initial_safe"] and result["semantic_structural_coverage"])
        edited+=len(result["original_paid_source_patches"])
        for x in result["original_paid_source_patches"]:
            target_counts[x["node"]]+=1
        if native:
            n=native["original64_worlds"][index]
            expected_native={
                "arm":name,"initial_rule":seed,
                "paid_original_instructions":result["paid_steps"],
                "original_paid_source_edits":result["original_paid_source_patches"],
                "native_certified":result["semantic_structural_coverage"],
                "independent_semantic_modes":result["semantic_three_modes"],
                "original_paid_task_services":result["original_paid_task_services"],
                "repair_decisions":result["original_evidence_decisions"],
                "witness_resets":result["original_paid_full_reset"],
                "peak_occupied":result["peak_occupied"],
                "native_original_PID1_halted":True,
                "actor_claimed_success":result["actor_claimed_success"],
                "true_task_matches":sum(x["truth"] for x in result["original_paid_task_services"]),
                "host_source_edits_after_load":0,
            }
            for key,val in expected_native.items():
                assert n[key]==val,(name,key,"native differs",n[key],val)
        if seed in ([3,0,3],[3,1,3],[0,2,0],[0,2,3],[1,1,3],[3,2,3]):
            high_value.append({"seed":seed,
                "original_paid_steps":result["paid_steps"],
                "observed_native_patch":result["original_paid_source_patches"],
                "final_rule":result["source_after"],
                "true_structural_coverage":result["semantic_structural_coverage"],
                "actor_certified":bool(result["original_paid_certification"]),
                "first_divergence":None})

    assert (theoretical_possible,rescued,edited,target_counts[3],target_counts[6])==(32,32,40,24,16)
    ctrl_results=[]
    for index,arm in enumerate(controls):
        seed=[arm["a"],arm["b"],arm["c"]]
        name=arm["name"]
        trace=(trace_dir/f"b56_f1d1k_g3q_c1_p1_l3_i5_native_{name}.jsonl") if native else None
        result=replay_founder(source,sites,name,seed,
            arm["feedback"],arm["disabled"],arm["expected"],trace)
        total_original_steps+=result["paid_steps"]
        if native:
            n=native["controls"][index]
            assert n["arm"]==name and n["initial_rule"]==seed
            assert n["paid_original_instructions"]==result["paid_steps"]
            assert n["original_paid_source_edits"]==result["original_paid_source_patches"]
            assert n["native_certified"]==result["semantic_structural_coverage"]
            assert n["repair_decisions"]==result["original_evidence_decisions"]
            assert n["original_paid_task_services"]==result["original_paid_task_services"]
            assert n["native_original_PID1_halted"] is True
        if arm["disabled"]:
            assert result["original_paid_task_services"]==[]
            assert not result["semantic_structural_coverage"]
        else:
            assert result["semantic_structural_coverage"]
            assert result["actor_claimed_success"]==1
        ctrl_results.append({
            "name":name,"paid_steps":result["paid_steps"],
            "source_patch":result["original_paid_source_patches"],
            "certified":result["semantic_structural_coverage"],
            "true_task_matches":sum(s["truth"] for s in result["original_paid_task_services"]),
            "actor_claimed_success":result["actor_claimed_success"],
            "first_divergence":None,
        })
    if native:
        assert native["counts"]=={
            "original_genotypes":64,"initially_safe":24,"initially_unsafe":40,
            "one_edit_feasible":32,"one_edit_impossible":8,
            "original_pid_paid_edits":40,"source_middle_edits":24,
            "source_tail_edits":16,"actual_successful_repairs":32,
            "false_certificates":0,"false_rejections":0,
        }
    print(json.dumps({
        "study":NAME,
        "evidence_level":"NATIVE_68_ORIGINAL_WORLD_ALL_PAID_CIL_STEPS_AND_FEEDBACK_REPLAY" if native else "PRECOMMITTED_SOURCE_ONLY_EXHAUSTIVE_PREDICTIONS",
        "source_sha":native["source_sha"] if native else None,
        "native_paid_original_steps_independently_checked":total_original_steps if native else 0,
        "all_genotypes":64,"initially_unsafe":40,
        "semantically_one_edit_repairable":theoretical_possible,
        "original_actor_successful_repairs":rescued,
        "reached_proven_finite_one_edit_upper_bound":rescued==theoretical_possible,
        "initially_unrepairable_by_any_one_byte_edit":8,
        "original_actor_paid_edit_count":edited,
        "original_actor_chosen_targets":{"middle_PC3":target_counts[3],"tail_PC6":target_counts[6]},
        "false_certificates":0,"false_rejections":0,
        "first_divergence":None,
        "counterexamples":high_value,
        "causal_controls":ctrl_results,
        "limits":"Author-written one-repair target selection and candidate normalization, not a learned repair grammar or open-ended evolution."
    },indent=2))

if __name__=="__main__":
    main()
