#!/usr/bin/env python3
"""Independent C14 source-origin interpreter: source-frozen native CIL, all 274 worlds.
No Rust/CL0 execution imports, no native summary as authority for opcode outcomes.
"""
import json
import sys
from pathlib import Path

ROOT=Path(__file__).resolve().parents[1]
C10=ROOT/"studies/CL1B-G2-ACTOR-PAID-STATE-TRIGGERED-ROLLBACK-C10-frozen-274-worlds-v1.json"
C14=ROOT/"studies/CL1B-C14-NATIVE-FEEDBACK-ATTESTED-SUCCESS-SOURCE-RESTORATION-frozen-274-worlds-v1.json"
C16=ROOT/"studies/CL1B-C16-SINGLE-USE-PAID-SUCCESS-CERTIFICATE-frozen-274-worlds-v1.json"
CELLS=(11,20,21,30,32,33,34,35,36,37,38,39,40,41,42,44,45)
CREDIT=6000

def source(parent,arm):
    ticket=arm=="truth_one_use_reentry" or arm.startswith("same_source_")
    reentry=ticket or arm=="different_source_c15_unlimited_reentry"
    c14=reentry or arm=="different_source_c14c_no_reentry"
    assert c14 or arm=="different_source_c9_fixed_abstain"
    key=("frozen_source_g2_C9_checkpoint_abstain" if arm=="different_source_c9_fixed_abstain"
         else "frozen_source_g2_rollback")
    code=[dict(v) for v in parent[key]]
    assert len(code)==180
    if c14:
        assert code[6]=={"kind":"Branch","test":0,"nonzero":29,"zero":7}
        assert code[99]=={"kind":"Const","dst":2,"value":42,"next":100}
        code[6]["nonzero"]=180
        code[99]["value"]=44
        code.extend([
            {"kind":"Const","dst":2,"value":41,"next":181},
            {"kind":"ReadLastWriter","dst":0,"address":2,"next":182},
            {"kind":"Const","dst":2,"value":1,"next":183},
            {"kind":"Sub","dst":0,"lhs":0,"rhs":2,"next":184},
            {"kind":"Branch","test":0,"nonzero":29,"zero":185},
            {"kind":"Const","dst":2,"value":41,"next":186},
            {"kind":"Read","dst":0,"address":2,"next":187},
            {"kind":"Const","dst":2,"value":44,"next":188},
            {"kind":"Write","address":2,"value":0,"next":29},
        ])
    if reentry:
        assert code[95]=={"kind":"Branch","test":0,"nonzero":99,"zero":86}
        assert code[102]=={"kind":"PatchConst","node":1,"value":0,"next":103}
        code[95]["nonzero"]=189
        code[102]["next"]=173
        code.extend([
            {"kind":"Const","dst":2,"value":44,"next":190},
            {"kind":"ReadLastWriter","dst":0,"address":2,"next":191},
            {"kind":"Const","dst":2,"value":1,"next":192},
            {"kind":"Sub","dst":0,"lhs":0,"rhs":2,"next":193},
            {"kind":"Branch","test":0,"nonzero":103,"zero":99}
        ])
    if ticket:
        assert code[188]=={"kind":"Write","address":2,"value":0,"next":29}
        assert code[95]=={"kind":"Branch","test":0,"nonzero":189,"zero":86}
        assert code[102]=={"kind":"PatchConst","node":1,"value":0,"next":173}
        code[188]["next"]=194
        code[95]["nonzero"]=197
        code[102]["next"]=200
        code.extend([
            {"kind":"Const","dst":2,"value":45,"next":195},
            {"kind":"Const","dst":0,"value":0,"next":196},
            {"kind":"Write","address":2,"value":0,"next":29},
            {"kind":"Const","dst":2,"value":45,"next":198},
            {"kind":"Read","dst":0,"address":2,"next":199},
            {"kind":"Branch","test":0,"nonzero":103,"zero":189},
            {"kind":"Const","dst":2,"value":45,"next":201},
            {"kind":"Const","dst":0,"value":1,"next":202},
            {"kind":"Write","address":2,"value":0,"next":173},
        ])
    assert len(code)==(203 if ticket else 194 if reentry else 189 if c14 else 180)
    return code

def replay(parent,record,folder):
    name,arm,mode,goals=(record[k] for k in ("name","arm","feedback","goals"))
    ticket=arm=="truth_one_use_reentry" or arm.startswith("same_source_")
    reentry=ticket or arm=="different_source_c15_unlimited_reentry"
    c14=reentry or arm=="different_source_c14c_no_reentry"
    suppress=arm=="same_source_pc102_effect_suppressed"
    suppress_ticket=arm=="same_source_pc202_ticket_consumption_suppressed"
    clamp=(0 if arm=="same_source_host_force_success_value_0" else
           2 if arm=="same_source_host_force_success_value_2" else None)
    assert mode in ("truth","inverted","forced0")
    assert len(goals)==48
    code=source(parent,arm)
    regs=[0,0,0,0]
    memory={}
    last_writer={}
    pc=paid=0
    peak=len(code)+1
    services=[]
    edits=[]
    memories=[]
    restores=[]
    later_reads=[]
    success_records=correct_records=tampers=attempted=effective_restores=0
    last173=last52=last_service=pending_restored=None
    writer_checks=[]
    skipped_unverified_positive=0
    stale_success_code_samples=0
    certificates_minted=ticket_read_attempts=tickets_spent=ticket_gate_rejections=0
    suppressed_ticket_writes=0
    writer_auth_checks=writer_auth_allowed=writer_auth_blocked=0
    pending_reentry=False
    pending_pc52=False
    reentry_executions=post_reentry_pc52_writes=0

    def sig():
        assert all(code[i]["kind"]=="Write" for i in (12,34,133))
        assert code[46]["kind"]==code[173]["kind"]=="Const"
        return [code[12]["next"],code[34]["next"],code[46]["value"],
                code[133]["next"],code[173]["value"]]
    def occupied():
        # Source-independent CL0 provenance-enabled physics: value + last-writer slot.
        return len(code)+1+2*len(memory)
    path=Path(folder)/f"cl1b_c16_original_{name}.jsonl"
    with path.open(encoding="utf-8") as f:
        rows=(json.loads(line) for line in f if line.strip())
        def verify(wanted):
            got=next(rows,None)
            if got!=wanted:
                raise AssertionError(json.dumps({"name":name,"paid":paid,"native":got,
                    "independent":wanted},ensure_ascii=False)[:6000])
        while paid<CREDIT:
            at=pc
            op=code[pc]
            kind=op["kind"]
            paid+=1
            if pending_reentry:
                assert at==173,("C15 paid original source reentry failed",name,paid,at)
                reentry_executions+=1
                pending_reentry=False
                pending_pc52=True
            if kind=="Const":
                regs[op["dst"]]=op["value"]
                pc=op["next"]
            elif kind in ("Add","Sub"):
                l,r=regs[op["lhs"]],regs[op["rhs"]]
                regs[op["dst"]]=l+r if kind=="Add" else l-r
                pc=op["next"]
            elif kind=="Read":
                regs[op["dst"]]=memory.get(regs[op["address"]],0)
                pc=op["next"]
            elif kind=="ReadLastWriter":
                regs[op["dst"]]=last_writer.get(regs[op["address"]],-1)
                pc=op["next"]
            elif kind=="Write":
                key=regs[op["address"]]
                val=regs[op["value"]]
                if not (at==202 and suppress_ticket):
                    memory[key]=val
                    last_writer[key]=1
                else:
                    assert key==45 and val==1
                pc=op["next"]
            elif kind=="Branch":
                pc=op["nonzero"] if regs[op["test"]] else op["zero"]
            elif kind=="PatchConst":
                target=regs[op["node"]]
                assert code[target]["kind"]=="Const"
                before=code[target]["value"]
                val=regs[op["value"]]
                suppressed=at==102 and suppress
                if not suppressed:
                    code[target]["value"]=val
                pc=op["next"]
                assert at in (28,45,88,102,122)
                if at==102 and target==173:
                    attempted+=1
                    if ticket:
                        assert pc==200,"C16 PC102 must mark ticket spent before executable PC173 reentry"
                    elif reentry:
                        assert pc==173,"C15 reference PC102 must directly reroute to original PC173"
                        pending_reentry=True
                    assert val==memory.get(44 if c14 else 42,0)
                    effective=not suppressed and before!=val
                    if effective:
                        effective_restores+=1
                        pending_restored=(paid,val)
                    restores.append([paid,before,val,suppressed,effective])
                edits.append([at,target,before,val,suppressed])
            elif kind=="PatchNext":
                target=regs[op["node"]]
                dest=regs[op["target"]]
                assert code[target]["kind"]=="Write"
                before=code[target]["next"]
                code[target]["next"]=dest
                pc=op["next"]
                assert at in (26,82,131,136,139,155)
                edits.append([at,target,before,dest,False])
            elif kind=="Halt":
                pc=at
            else:
                raise AssertionError(("unknown CIL node",at,kind))

            if at==173:
                assert regs[2]==sig()[4]
                last173=paid
                if pending_restored is not None:
                    rp,rv=pending_restored
                    assert rp<paid and regs[2]==rv
                    later_reads.append([rp,paid,rv])
                    pending_restored=None
            if at==52:
                if pending_pc52:
                    post_reentry_pc52_writes+=1
                    pending_pc52=False
                assert last173 is not None and last173<paid and memory[41]==regs[2]
                last52=paid
            if at==181 and c14:
                assert regs[0] in (-1,0,1)
                writer_checks.append([paid,regs[0]])
            if at==184 and c14:
                assert pc in (29,185)
                if pc==29:
                    skipped_unverified_positive+=1
                else:
                    assert last173 is not None and last52 is not None
            if at==196 and ticket:
                assert memory[45]==0 and last_writer[45]==1
                certificates_minted+=1
            if at==198 and ticket:
                assert regs[0]==memory.get(45,0)
                ticket_read_attempts+=1
            if at==199 and ticket:
                if pc==103:
                    assert memory[45]==1
                    ticket_gate_rejections+=1
                else:
                    assert pc==189 and memory.get(45,0)==0
            if at==202 and ticket:
                assert pc==173
                if suppress_ticket:
                    assert memory[45]==0
                    suppressed_ticket_writes+=1
                else:
                    assert memory[45]==1 and last_writer[45]==1
                tickets_spent+=1
                pending_reentry=True
            if at==190 and reentry:
                writer_auth_checks+=1
                assert last_writer.get(44,-1)==regs[0]
            if at==193 and reentry:
                if pc==99:
                    writer_auth_allowed+=1
                    assert last_writer.get(44)==1
                else:
                    assert pc==103
                    writer_auth_blocked+=1
            if at==188:
                assert c14 and last_service is not None
                step,truth,feedback=last_service
                assert step<paid and feedback==1
                assert last173<last52<step
                assert memory[44]==memory[41]==regs[0]
                success_records+=1
                correct_records+=int(truth==1)
                if memory[41]!=sig()[4]:
                    stale_success_code_samples+=1
                memories.append([paid,step,truth,feedback,memory[41],sig()[4]])
            assert 0<=pc<len(code) and len(memory)<=16 and occupied()<=300
            peak=max(peak,occupied())
            verify(["cil",paid,at,pc,list(regs),CREDIT-paid,
                    [memory.get(k) for k in CELLS],occupied(),sig(),
                    "suppressed_pc102" if at==102 and suppress
                    else "suppressed_pc202" if at==202 and suppress_ticket
                    else "paid_success_source_write" if at==188 else "normal"])
            if at==188 and clamp is not None:
                before=memory[44]
                memory[44]=clamp
                last_writer[44]=0
                tampers+=1
                peak=max(peak,occupied())
                verify(["tamper",paid,44,before,clamp,
                        [memory.get(k) for k in CELLS],occupied(),sig()])
            if at==3:
                stage=memory.get(32,0)
                assert 0<=stage<48
                guess=memory[11]
                truth=int(guess==goals[stage])
                feedback=truth if mode=="truth" else 1-truth if mode=="inverted" else 0
                services.append([stage,guess,truth,feedback])
                last_service=(paid,truth,feedback)
                memory[20]=feedback
                last_writer[20]=0
                peak=max(peak,occupied())
                verify(["feedback",paid,stage,goals[stage],guess,truth,feedback,
                        [memory.get(k) for k in CELLS],occupied(),sig()])
            if kind=="Halt":
                break
        assert next(rows,None) is None, ("trailing native source events",name)
    assert success_records==len(memories)
    assert tampers==(success_records if clamp is not None else 0)
    if not c14: assert success_records==0
    if mode=="truth" and c14:assert success_records==correct_records
    correct=sum(x[2] for x in services)
    completed=memory.get(32,0)
    claim=memory.get(21,0)
    if mode=="forced0":assert completed==0
    if mode=="inverted":assert correct<len(services)
    out={
        "name":name,"arm":arm,"feedback":mode,"goals":goals,
        "steps":paid,"services":services,"source_edits":edits,
        "success_source_memories":memories,"physical_parent_restorations":restores,
        "subsequent_physical_pc173_readbacks":later_reads,
        "claimed_success_records":success_records,
        "paid_writer_checks":writer_checks,
        "skipped_unverified_positive":skipped_unverified_positive,
        "stale_success_code_samples":stale_success_code_samples,
        "paid_source_writer_auth_checks":writer_auth_checks,
        "paid_source_writer_auth_allowed":writer_auth_allowed,
        "paid_source_writer_auth_blocked":writer_auth_blocked,
        "paid_pc102_reentry_pc173_actual_executions":reentry_executions,
        "paid_pc52_after_reentry_actual_writes":post_reentry_pc52_writes,
        "pending_reentry_when_credits_exhausted":pending_reentry,
        "paid_success_certificates_minted":certificates_minted,
        "paid_ticket_read_attempts":ticket_read_attempts,
        "paid_ticket_spend_writes":tickets_spent,
        "paid_ticket_consumed_gate_rejections":ticket_gate_rejections,
        "paid_ticket_spend_write_effect_suppressed":suppressed_ticket_writes,
        "final_ticket_cell45":memory.get(45),
        "truly_correct_source_records":correct_records,
        "external_cell44_tampers":tampers,
        "paid_pc102_target173_attempts":attempted,
        "effective_pc173_restorations":effective_restores,
        "final_cell44":memory.get(44),
        "completed_stages":completed,"true_matches":correct,"claim":claim,
        "peak_occupied":peak,"final_signature":sig(),
        "budget_exhausted":paid==CREDIT
    }
    if out!=record:
        raise AssertionError(json.dumps({"name":name,"native_summary":record,
            "independent":out},ensure_ascii=False)[:6000])
    return out

def main(summary_path,folder):
    m=json.loads(C14.read_text())
    policy=json.loads(C16.read_text())
    p=json.loads(C10.read_text())
    assert policy["study"]=="CL1B-C16-SINGLE-USE-PAID-SUCCESS-CERTIFICATE"
    assert policy["status"]=="PRE_NATIVE_FROZEN"
    assert len(source(p,"truth_one_use_reentry"))==203
    n=json.loads(Path(summary_path).read_text())
    assert m["status"]=="PRE_NATIVE_FROZEN"
    assert m["study"]=="CL1B-C14-NATIVE-FEEDBACK-ATTESTED-SUCCESS-SOURCE-RESTORATION"
    assert n["study"]=="CL1B-C16-SINGLE-USE-PAID-SUCCESS-CERTIFICATE"
    assert len(policy["cases"])==45 and len(policy["arms"])==6 and len(policy["false_feedback_controls"])==4
    assert len(n["records"])==45 and len(n["adversarial_records"])==4
    old={tuple(v["goals"]) for v in p["cases"]}
    old.update(tuple(v["goals"]) for v in m["cases"])
    new=set()
    charged=success=restores=effects=contaminated=0
    incomplete=[]
    fees={}
    completed={}
    physical={}
    visited=set()
    for case,record in zip(policy["cases"],n["records"]):
        assert case["name"]==record["name"] and case["family"]==record["family"]
        key=tuple(case["goals"])
        assert key not in old and key not in new
        new.add(key)
        assert len(record["arms"])==6
        row=[]
        for arm,trial in zip(policy["arms"],record["arms"]):
            assert trial["name"]==case["name"]+"_"+arm
            assert trial["arm"]==arm and trial["feedback"]=="truth"
            assert trial["goals"]==case["goals"]
            assert trial["name"] not in visited
            visited.add(trial["name"])
            out=replay(p,trial,folder)
            charged+=out["steps"]
            fees[arm]=fees.get(arm,0)+out["steps"]
            physical[arm]=physical.get(arm,0)+out["effective_pc173_restorations"]
            ok=(out["completed_stages"]==48 and out["true_matches"]==48
                and out["claim"]==1 and not out["budget_exhausted"])
            if ok:completed[arm]=completed.get(arm,0)+1
            else:incomplete.append(out["name"])
            row.append(out)
        if row[0]["steps"]!=row[1]["steps"] or row[0]["services"]!=row[1]["services"]:
            effects+=1
        if (row[0]["final_signature"]!=row[2]["final_signature"] or
            row[0]["final_signature"]!=row[3]["final_signature"] or
            row[0]["services"]!=row[2]["services"] or
            row[0]["services"]!=row[3]["services"]):
            contaminated+=1
        success+=row[0]["claimed_success_records"]
        restores+=row[0]["effective_pc173_restorations"]
    for control,record in zip(policy["false_feedback_controls"],n["adversarial_records"]):
        assert control["name"]==record["name"] and control["arm"]==record["arm"]
        assert control["feedback"]==record["feedback"]
        case=next(x for x in policy["cases"] if x["name"]==control["case"])
        assert case["goals"]==record["goals"]
        assert record["name"] not in visited
        visited.add(record["name"])
        charged+=replay(p,record,folder)["steps"]
    assert len(visited)==274
    assert n["native_worlds"]==274 and n["true_worlds"]==270
    assert n["false_feedback_worlds"]==4 and n["frozen_new_cases"]==45
    assert n["total_paid_cil"]==charged
    assert n["main_actor_paid_success_records"]==success
    assert n["main_actual_physical_pc173_restores"]==restores
    assert n["same_source_live_vs_suppressed_paid_or_feedback_difference_cases"]==effects
    assert n["same_source_live_vs_tampered_state_or_feedback_difference_cases"]==contaminated
    assert n["arm_fees"]==fees and n["arm_completed_worlds"]==completed
    assert n["arm_physical_restore_changes"]==physical
    assert n["incomplete_true_world_names"]==incomplete
    return {
        "study":n["study"],"status":"INDEPENDENT_C16_274_NATIVE_SINGLE_USE_SOURCE_CERTIFICATE_PARITY",
        "first_divergence":None,"native_worlds":274,
        "exact_private_paid_cil_replayed":charged,
        "new_goal_schedules":45,"main_paid_success_source_records":success,
        "main_physical_success_source_restores":restores,
        "same_initial_source_physical_effect_cases":effects,
        "same_initial_source_contamination_cases":contaminated,
        "arm_fees":fees,"arm_full_true_completion":completed,
        "incomplete_true_worlds":len(incomplete),"B3_B4":"NOT_MET"
    }
if __name__=="__main__":
    assert len(sys.argv)==3,"usage C14 script native_summary.json native_traces_dir"
    print(json.dumps(main(sys.argv[1],sys.argv[2]),ensure_ascii=False,indent=2))
