#!/usr/bin/env python3
"""C13 independent Python execution of every native paid CIL instruction and external intervention.
No CL0 Rust bindings; source is rebuilt from frozen parent and visible C13 opcode specification.
"""
import json
import sys
from pathlib import Path

ROOT=Path(__file__).resolve().parents[1]
C10=ROOT/"studies/CL1B-G2-ACTOR-PAID-STATE-TRIGGERED-ROLLBACK-C10-frozen-274-worlds-v1.json"
C13=ROOT/"studies/CL1B-C13-PAID-COUNTER-CAUSAL-ROLLBACK-GATE-frozen-184-worlds-v1.json"
CELLS=(11,20,21,30,32,33,34,35,36,37,38,39,40,41,42,43)
CREDIT=6000

def source(m,arm):
    gated=arm.startswith("gated_")
    metered=gated or arm=="c12_ungated_meter_reference"
    assert metered or arm=="c9_fixed_abstain_reference"
    key=("frozen_source_g2_C9_checkpoint_abstain" if not metered
         else "frozen_source_g2_rollback")
    code=[dict(x) for x in m[key]]
    assert len(code)==180
    if metered:
        assert code[95]=={"kind":"Branch","test":0,"nonzero":99,"zero":86}
        code[95]["nonzero"]=180
        code.extend([
            {"kind":"Const","dst":2,"value":43,"next":181},
            {"kind":"Read","dst":0,"address":2,"next":182},
            {"kind":"Const","dst":2,"value":1,"next":183},
            {"kind":"Add","dst":0,"lhs":0,"rhs":2,"next":184},
            {"kind":"Const","dst":2,"value":43,"next":185},
            {"kind":"Write","address":2,"value":0,"next":186 if gated else 99}
        ])
    if gated:
        code.extend([
            {"kind":"Const","dst":2,"value":43,"next":187},
            {"kind":"Read","dst":0,"address":2,"next":188},
            {"kind":"Const","dst":2,"value":1,"next":189},
            {"kind":"Sub","dst":0,"lhs":0,"rhs":2,"next":190},
            {"kind":"Branch","test":0,"nonzero":103,"zero":99}
        ])
    assert len(code)==(191 if gated else 186 if metered else 180)
    return code

def replay(parent,original,folder):
    arm=original["arm"]
    mode=original["feedback"]
    goals=original["goals"]
    name=original["name"]
    assert mode in ("truth","forced0","inverted")
    assert len(goals)==48 and all(g in (0,1,2) for g in goals)
    gated=arm.startswith("gated_")
    metered=gated or arm=="c12_ungated_meter_reference"
    suppress=arm=="gated_pc102_effect_suppressed"
    clamp=(1 if arm=="gated_host_clamp_one" else
           2 if arm=="gated_host_force_two" else None)
    code=source(parent,arm)
    regs=[0,0,0,0]
    memory={}
    pc=0
    paid=0
    peak=len(code)+1
    services=[]
    edits=[]
    decisions=[]
    writes=candidate_events=pc102_paid=changed=allowed=skipped=tampers=0
    last95=None
    last185=None
    def sig():
        assert code[12]["kind"]==code[34]["kind"]==code[133]["kind"]=="Write"
        assert code[46]["kind"]==code[173]["kind"]=="Const"
        return [code[12]["next"],code[34]["next"],code[46]["value"],
                code[133]["next"],code[173]["value"]]
    def occupied():
        return len(code)+1+len(memory)
    path=Path(folder)/f"cl1b_c13_original_{name}.jsonl"
    with path.open(encoding="utf8") as f:
        rows=(json.loads(line) for line in f)
        def check(wanted):
            actual=next(rows,None)
            if actual!=wanted:
                raise AssertionError(json.dumps({"name":name,"paid":paid,
                    "native":actual,"independent":wanted},ensure_ascii=False)[:6000])
        while paid<CREDIT:
            at=pc
            op=code[at]
            kind=op["kind"]
            paid+=1
            if kind=="Const":
                regs[op["dst"]]=op["value"]
                pc=op["next"]
            elif kind in ("Add","Sub"):
                a=regs[op["lhs"]]
                b=regs[op["rhs"]]
                regs[op["dst"]]=a+b if kind=="Add" else a-b
                pc=op["next"]
            elif kind=="Read":
                regs[op["dst"]]=memory.get(regs[op["address"]],0)
                pc=op["next"]
            elif kind=="Write":
                memory[regs[op["address"]]]=regs[op["value"]]
                pc=op["next"]
            elif kind=="Branch":
                pc=op["nonzero"] if regs[op["test"]]!=0 else op["zero"]
            elif kind=="PatchConst":
                target=regs[op["node"]]
                assert code[target]["kind"]=="Const"
                before=code[target]["value"]
                value=regs[op["value"]]
                skipped_effect=at==102 and suppress
                if not skipped_effect:
                    code[target]["value"]=value
                pc=op["next"]
                assert at in (45,88,102,122,28)
                edits.append([at,target,before,value,skipped_effect])
                if at==102:
                    assert target==173 and value==memory.get(42,0)
                    pc102_paid+=1
                    if not skipped_effect and before!=value:
                        changed+=1
                    if gated: assert last185 is not None and last185<paid
            elif kind=="PatchNext":
                target=regs[op["node"]]
                value=regs[op["target"]]
                assert code[target]["kind"]=="Write"
                before=code[target]["next"]
                code[target]["next"]=value
                pc=op["next"]
                assert at in (26,82,131,136,139,155)
                edits.append([at,target,before,value,False])
            elif kind=="Halt":
                pc=at
            else:
                raise AssertionError(("unsupported native opcode",at,kind))
            if at==95 and metered and pc==180:
                last95=paid
            if at==185:
                assert metered and kind=="Write"
                candidate_events+=1
                writes+=1
                last185=paid
                assert last95 is not None and last95<paid
                if clamp is None:
                    assert memory[43]==candidate_events
            if at==190:
                assert gated
                assert last185 is not None and last185<paid
                expected=99 if regs[0]==0 else 103
                assert pc==expected
                if expected==99:allowed+=1
                else:skipped+=1
                decisions.append({
                    "step":paid,"count_read_from_paid_cell43":regs[0]+1,
                    "route":"ALLOW_PC102" if expected==99 else "SKIP_PC102",
                    "pc":190,"next_pc":expected,"candidate_index":candidate_events
                })
            assert 0<=pc<len(code) and len(memory)<=16 and occupied()<=300
            peak=max(peak,occupied())
            check(["cil",paid,at,pc,list(regs),CREDIT-paid,
                   [memory.get(k) for k in CELLS],occupied(),sig(),
                   "suppressed_pc102" if at==102 and suppress else
                   "paid_meter_write" if at==185 else
                   "paid_decision" if at==190 else "normal"])
            if at==185 and clamp is not None:
                before=memory[43]
                memory[43]=clamp
                tampers+=1
                peak=max(peak,occupied())
                check(["tamper",paid,43,before,clamp,
                       [memory.get(k) for k in CELLS],occupied(),sig()])
            if at==3:
                stage=memory.get(32,0)
                assert 0<=stage<48
                guess=memory[11]
                truth=int(guess==goals[stage])
                feedback=truth if mode=="truth" else 1-truth if mode=="inverted" else 0
                services.append([stage,guess,truth,feedback])
                memory[20]=feedback
                peak=max(peak,occupied())
                check(["feedback",paid,stage,goals[stage],guess,truth,feedback,
                       [memory.get(k) for k in CELLS],occupied(),sig()])
            if kind=="Halt":
                break
        assert next(rows,None) is None,"native file has extra unaccounted events"
    assert writes==candidate_events
    if gated:
        assert candidate_events==allowed+skipped and pc102_paid==allowed
    if metered and not gated:
        assert candidate_events==pc102_paid
    if clamp is not None:
        assert tampers==candidate_events
        assert memory.get(43)==(clamp if candidate_events>0 else None)
        if clamp==1:assert skipped==0
        if clamp==2:assert allowed==0
    else:
        assert tampers==0
        assert memory.get(43)==(candidate_events if metered and candidate_events>0 else None)
    if gated and clamp is None:
        assert allowed==int(candidate_events>0)
    correct=sum(x[2] for x in services)
    completed=memory.get(32,0)
    claim=memory.get(21,0)
    if mode=="forced0":assert completed==0
    if mode=="inverted":assert correct<len(services)
    comparable={
        "name":name,"arm":arm,"feedback":mode,"goals":goals,
        "steps":paid,"services":services,"paid_code_edits":edits,
        "paid_decisions":decisions,"candidate_events":candidate_events,
        "actor_paid_meter_writes":writes,"actor_paid_pc102":pc102_paid,
        "physical_pc102_value_changes":changed,
        "allow_pc102":allowed,"skip_pc102":skipped,
        "externally_tampered_cell43":tampers,
        "meter_value":memory.get(43),"completed_stages":completed,
        "true_matches":correct,"claim":claim,"peak_occupied":peak,
        "final_signature":sig(),"budget_exhausted":paid==CREDIT
    }
    assert comparable==original, json.dumps({"name":name,
        "native_summary":original,"independent":comparable},ensure_ascii=False)[:6000]
    return comparable

def main(summary_file,folder):
    p=json.loads(C10.read_text())
    m=json.loads(C13.read_text())
    native=json.loads(Path(summary_file).read_text())
    assert m["status"]=="PRE_NATIVE_FROZEN"
    assert m["study"]==native["study"]=="CL1B-C13-PAID-COUNTER-CAUSAL-ROLLBACK-GATE"
    assert len(m["cases"])==30 and len(m["arms"])==6 and len(m["controls"])==4
    assert len(native["records"])==30 and len(native["adversarial_records"])==4
    old={tuple(x["goals"]) for x in p["cases"]}
    all_new=set()
    total=0
    incomplete=[]
    totals={}
    completed={}
    by_candidate={}
    by_pc102={}
    total_tamper=0
    cases_differ=0
    skips=0
    seen=set()
    for case,record in zip(m["cases"],native["records"]):
        assert case["name"]==record["name"] and case["family"]==record["family"]
        goal_key=tuple(case["goals"])
        assert goal_key not in old and goal_key not in all_new
        all_new.add(goal_key)
        assert len(record["arms"])==6
        row=[]
        for arm,entry in zip(m["arms"],record["arms"]):
            assert entry["arm"]==arm and entry["feedback"]=="truth"
            assert entry["goals"]==case["goals"]
            assert entry["name"]==case["name"]+"_"+arm
            assert entry["name"] not in seen
            seen.add(entry["name"])
            v=replay(p,entry,folder)
            row.append(v)
            total+=v["steps"]
            totals[arm]=totals.get(arm,0)+v["steps"]
            by_candidate[arm]=by_candidate.get(arm,0)+v["candidate_events"]
            by_pc102[arm]=by_pc102.get(arm,0)+v["actor_paid_pc102"]
            total_tamper+=v["externally_tampered_cell43"]
            ok=(v["completed_stages"]==48 and v["true_matches"]==48 and
                v["claim"]==1 and not v["budget_exhausted"])
            if ok:completed[arm]=completed.get(arm,0)+1
            else:incomplete.append(v["name"])
        skips+=row[0]["skip_pc102"]
        if (row[0]["actor_paid_pc102"]!=row[2]["actor_paid_pc102"]
                or row[0]["actor_paid_pc102"]!=row[3]["actor_paid_pc102"]):
            cases_differ+=1
    for control,record in zip(m["controls"],native["adversarial_records"]):
        assert control["name"]==record["name"]
        assert control["arm"]==record["arm"]
        assert control["feedback"]==record["feedback"]
        case=next(x for x in m["cases"] if x["name"]==control["case"])
        assert record["goals"]==case["goals"]
        assert record["name"] not in seen
        seen.add(record["name"])
        v=replay(p,record,folder)
        total+=v["steps"]
        total_tamper+=v["externally_tampered_cell43"]
    assert len(seen)==184 and total==native["total_private_paid_cil"]
    assert native["native_worlds"]==184
    assert native["truthful_worlds"]==180 and native["adversarial_worlds"]==4
    assert skips==native["gated_live_skipped_actual_pc102"]>0
    assert cases_differ==native["cases_with_live_vs_tampered_paid_pc102_difference"]>0
    assert total_tamper==native["external_tamper_actions"]
    assert totals==native["arm_total_paid_cil"]
    assert completed==native["arm_completed_truthful_worlds"]
    assert by_candidate==native["arm_actor_written_candidate_events"]
    assert by_pc102==native["arm_actual_paid_pc102"]
    assert incomplete==native["incomplete_truthful_world_names"]
    return {
        "study":native["study"],"status":"INDEPENDENT_ORIGINAL_NATIVE_PAID_CIL_PARITY",
        "first_divergence":None,"native_worlds":184,
        "fully_replayed_private_paid_CIL":total,
        "new_frozen_true_cases":30,
        "gated_live_skipped_paid_PC102":skips,
        "causal_tamper_differentiated_cases":cases_differ,
        "externally_tampered_events":total_tamper,
        "arm_total_paid_CIL":totals,
        "arm_completed_truthful_worlds":completed,
        "incomplete_truthful_worlds":len(incomplete),
        "B3_B4":"NOT_MET",
    }
if __name__=="__main__":
    assert len(sys.argv)==3,"usage: python replay.py summary.json native_folder"
    print(json.dumps(main(sys.argv[1],sys.argv[2]),ensure_ascii=False,indent=2))
