#!/usr/bin/env python3
"""Independent per-instruction C7 Rust-native original actor-paid actual executed code value readback and suppression replay."""
import json
import sys
from pathlib import Path

ROOT=Path(__file__).resolve().parents[1]
MANIFEST=ROOT/"studies/CL1B-G2-ACTOR-PAID-STATE-TRIGGERED-ROLLBACK-C10-frozen-274-worlds-v1.json"
SOURCES={
    "G2_rollback":"frozen_source_g2_rollback",
    "G2_rollback_pc102_suppressed":"frozen_source_g2_rollback",
    "G2_rollback_pc28_suppressed":"frozen_source_g2_rollback",
    "G2_rollback_placebo180":"frozen_source_g2_rollback",
    "G2_C9_checkpoint_no_rollback":"frozen_source_g2_C9_checkpoint_no_rollback",
    "G2_C9_checkpoint_abstain":"frozen_source_g2_C9_checkpoint_abstain"
}
CELLS=(11,20,21,30,32,33,34,35,36,37,38,39,40,41,42)
CREDIT=6000

def replay(m,arm,folder):
    code=[dict(x) for x in m[SOURCES[arm["source"]]]]
    assert len(code)==180
    goals=arm["goals"]
    assert len(goals)==48 and all(x in (0,1,2) for x in goals)
    mode=arm["feedback"]
    assert mode in ("truth","inverted","forced0")
    regs=[0,0,0,0]
    memory={}
    pc=0
    paid=0
    peak=181
    services=[]
    paid_gate_records=[]
    gate_allow=gate_abstain=0
    readback_writes=attested_checks=attested_mismatches=attested_after_suppression=0
    last173=last52=last_suppressed=None
    readback_records=[]
    checkpoints=[]
    most_recent_checkpoint=None
    checkpoint_count=0
    rollback_attempt_records=[]
    genuine_restores=0
    rollback_readback_records=[]
    last_rollback=None
    last_positive_cell36_trigger=None
    suppressed_pc102_count=0
    decisions=[]
    refills=[]
    patches=[]
    edits=[]
    later_reads=[]
    last_changed=None
    failed_halt=False
    suppressed_pc28=[]
    path=Path(folder)/("cl1b_g2_state_rollback_c10_original_"+arm["name"]+".jsonl")
    with path.open(encoding="utf-8") as stream:
        lines=(json.loads(line) for line in stream if line.strip())
        def edges():
            return [code[12]["next"],code[34]["next"],code[46]["value"],code[133]["next"]]
        def theta():
            n=code[173]
            assert n["kind"]=="Const" and n["dst"]==2 and n["next"] in (24,174)
            return n["value"]
        def check(wanted):
            actual=next(lines,None)
            if actual!=wanted:
                raise AssertionError(json.dumps({"name":arm["name"],"paid":paid,
                    "actual_native":actual,"independent":wanted},ensure_ascii=False)[:5000])
        while paid<CREDIT:
            at=pc
            op=code[at]
            kind=op["kind"]
            pre_value=code[regs[op["node"]]]["value"] if kind=="PatchConst" else None
            pre_next=code[regs[op["node"]]]["next"] if kind=="PatchNext" else None
            paid+=1
            if kind=="Const":
                regs[op["dst"]]=op["value"]
                pc=op["next"]
            elif kind in ("Add","Sub"):
                a,b=regs[op["lhs"]],regs[op["rhs"]]
                regs[op["dst"]]=a+b if kind=="Add" else a-b
                pc=op["next"]
            elif kind=="Read":
                regs[op["dst"]]=memory.get(regs[op["address"]],0)
                pc=op["next"]
            elif kind=="Write":
                memory[regs[op["address"]]]=regs[op["value"]]
                pc=op["next"]
            elif kind=="Branch":
                pc=op["nonzero"] if regs[op["test"]] else op["zero"]
            elif kind=="PatchConst":
                target=regs[op["node"]]
                assert 0<=target<180 and code[target]["kind"]=="Const"
                if not ((at==28 and arm["source"]=="G2_rollback_pc28_suppressed") or
                        (at==102 and arm["source"]=="G2_rollback_pc102_suppressed")):
                    code[target]["value"]=regs[op["value"]]
                pc=op["next"]
            elif kind=="PatchNext":
                target=regs[op["node"]]
                destination=regs[op["target"]]
                assert 0<=target<180 and 0<=destination<180
                assert code[target]["kind"]=="Write"
                code[target]["next"]=destination
                pc=op["next"]
            elif kind=="Halt":
                pc=at
            else:
                raise AssertionError(("unsupported original CIL opcode",kind,at))
            if at in (45,88,102,122,28):
                target=regs[1]
                value=regs[3] if at==28 else regs[0]
                actor_rollback=at==102 and arm["source"].startswith("G2_rollback")
                assert target==(173 if at==28 or actor_rollback else 46)
                suppressed=(at==28 and arm["source"]=="G2_rollback_pc28_suppressed") or (
                    actor_rollback and arm["source"]=="G2_rollback_pc102_suppressed")
                assert code[target]["value"]==(pre_value if suppressed else value)
                if at==28:
                    assert value in (0,1,2)
                    if True: # C10 all PC28 pathways are actor-paid checkpointed
                        assert most_recent_checkpoint is not None
                        assert paid-most_recent_checkpoint[0]==2
                        assert most_recent_checkpoint[1]==pre_value==memory.get(42)

                    patches.append({"step":paid,"pc":28,"target":173,
                        "before":pre_value,"attempted":value,
                        "after":pre_value if suppressed else value,
                        "changed":not suppressed and pre_value!=value,
                        "suppressed":suppressed})
                    if suppressed:
                        last_suppressed=paid
                        suppressed_pc28.append({"step":paid,"pc":28,"target":173,
                            "attempted":value,"before":pre_value,"after":pre_value})
                    elif pre_value!=value:
                        last_changed=value
                        last_rollback=None
                if actor_rollback:
                    assert last_positive_cell36_trigger is not None
                    assert paid-last_positive_cell36_trigger==4
                    assert memory.get(36,0)!=0
                    assert value==memory.get(42,0)
                    if checkpoint_count:
                        assert most_recent_checkpoint is not None
                        assert value==most_recent_checkpoint[1]
                    if suppressed:suppressed_pc102_count+=1
                    physical=checkpoint_count>0 and not suppressed and pre_value!=value
                    if physical:
                        genuine_restores+=1
                        last_changed=value
                        last_rollback=(paid,value)
                    rollback_attempt_records.append({
                        "step":paid,"pc":102,
                        "paid_history_trigger_step":last_positive_cell36_trigger,
                        "checkpoint_exists":checkpoint_count>0,
                        "parent_from_actor_cell42":value,
                        "before":pre_value,
                        "after":pre_value if suppressed else value,
                        "suppressed":bool(suppressed),
                        "actual_physical_parent_restoration":bool(physical)
                    })
                edits.append({"step":paid,"pc":at,"target":target,
                    "before":pre_value,"attempted":value,
                    "after":pre_value if suppressed else value,
                    "kind":"PatchConst","suppressed":suppressed})
            if at in (82,131,136,139,155,26):
                target=regs[1]
                value=regs[0]
                assert target in (12,34,133)
                if at==26:assert target==133 and value==62
                edits.append({"step":paid,"pc":at,"target":target,
                    "before":pre_next,"after":value,"kind":"PatchNext"})
            if at==52:
                assert kind=="Write" and memory[41]==regs[2]==theta()
                assert last173 is not None and last173<paid
                last52=paid
                readback_writes+=1
                if arm["source"]=="G2_rollback_pc28_suppressed" and (
                    last_suppressed is not None and last173>last_suppressed
                ):
                    attested_after_suppression+=1
                readback_records.append({
                    "step":paid,"last_pc173":last173,
                    "actual_theta":theta(),"written_cell41":memory[41]
                })
            if at==67:
                assert kind=="Branch" and pc in (1,27,125)
                witnessed=memory.get(41,0)
                actual_theta=theta()
                attested_checks+=1
                assert last173 is not None and last52 is not None
                assert last173<last52<paid
                assert witnessed==actual_theta
                assert regs[0]==regs[3]-witnessed
                if witnessed!=actual_theta:attested_mismatches+=1
                expected_route={
                    "G2_rollback":1 if regs[0]!=0 else 125,
                    "G2_rollback_pc102_suppressed":1 if regs[0]!=0 else 125,
                    "G2_rollback_pc28_suppressed":1 if regs[0]!=0 else 125,
                    "G2_rollback_placebo180":1 if regs[0]!=0 else 125,
                    "G2_C9_checkpoint_no_rollback":1 if regs[0]!=0 else 125,
                    "G2_C9_checkpoint_abstain":125
                }[arm["source"]]
                assert pc==expected_route,("C9 paid checkpoint gate unexpectedly routed",arm["name"],pc,expected_route)
                if pc==125:gate_abstain+=1
                else:gate_allow+=1
                paid_gate_records.append({
                    "step":paid,"r3":regs[3],"r0":regs[0],"route":pc,
                    "witness":witnessed,"actual_theta":actual_theta,
                    "evidence_key":41,"last_pc173":last173,"last_write":last52,
                    "decision":"ABSTAIN" if pc==125 else "EDIT"
                })
            if at==95 and arm["source"].startswith("G2_rollback") and pc==99:
                assert memory.get(36,0)!=0
                last_positive_cell36_trigger=paid
            if at==97:
                assert kind=="Write" and pc==27
                assert memory[42]==regs[0]==theta()==memory[41]
                checkpoint_count+=1
                most_recent_checkpoint=(paid,theta())
                checkpoints.append({"step":paid,"previous_physical_theta":theta(),
                    "cell42":memory[42],"last_verified_readback":last52})
            if at==173:
                last173=paid
                assert regs[2]==theta()
                if last_rollback is not None:
                    rp,rtheta=last_rollback
                    assert paid>rp and regs[2]==rtheta
                    rollback_readback_records.append({
                        "step":paid,"rollback_step":rp,"actual_readback_theta":rtheta
                    })
                    last_rollback=None
                if last_changed is not None:
                    assert last_changed==regs[2]
                    later_reads.append({"step":paid,"theta":last_changed})
                    last_changed=None
            if at==175:
                delta=regs[0]
                expected=176 if delta else 179
                assert pc==expected
                decisions.append({"step":paid,"difference":delta,"next":pc,
                    "theta":theta(),"last_guess":regs[3]})
            if at==178:
                assert memory[38]==0
                refills.append({"step":paid,"pc":178})
            if at==15 and pc==61:
                assert regs[0]==0
                failed_halt=True
            occupied=181+len(memory)
            assert len(memory)<=16 and occupied<=300 and 0<=pc<180
            peak=max(peak,occupied)
            check(["cil",paid,at,pc,list(regs),CREDIT-paid,
                [memory.get(k) for k in CELLS],occupied,edges(),theta(),
                "suppressed_pc28" if at==28 and arm["source"]=="G2_rollback_pc28_suppressed"
                else "suppressed_pc102" if at==102 and arm["source"]=="G2_rollback_pc102_suppressed"
                else "paid_parameter_patch" if at==28 else "normal"])
            if at==3:
                assert kind=="Write" and memory[11]==regs[op["value"]]
                stage=memory.get(32,0)
                assert 0<=stage<48
                guess=memory[11]
                truth=int(guess==goals[stage])
                feedback=truth if mode=="truth" else 1-truth if mode=="inverted" else 0
                services.append({"step":paid,"stage":stage,"guess":guess,
                    "truth":truth,"feedback":feedback})
                memory[20]=feedback
                occupied=181+len(memory)
                peak=max(peak,occupied)
                check(["feedback",paid,stage,goals[stage],guess,truth,feedback,
                    [memory.get(k) for k in CELLS],occupied,edges(),theta()])
            if kind=="Halt":break
        else:
            raise AssertionError(("CIL private credits exhausted",arm["name"]))
        assert next(lines,None) is None,"extra original-native trace events"
    true_count=sum(x["truth"] for x in services)
    completed=memory.get(32,0)
    claim=memory.get(21,0)
    if mode=="truth":assert (completed,true_count,claim)==(48,48,1)
    if mode=="forced0":assert failed_halt and completed==0 and claim==0
    if mode=="inverted":assert claim==1 and true_count<len(services)
    return {"name":arm["name"],"source":arm["source"],"regime":arm["regime"],
        "feedback":mode,"goals":goals,"steps":paid,"services":services,
        "decisions":decisions,"refills":refills,"parameter_patches":patches,
        "paid_code_edits":edits,"suppressed_pc28":suppressed_pc28,
        "later_mutated_reads":later_reads,
        "completed_stages":completed,"truthful_matches":true_count,
        "claim":claim,"final_theta":theta(),"final_edges":edges(),
        "peak_occupied":peak,"failed_third_probe_halt":failed_halt,
        "host_source_edits":0,"budget_exhausted":False,
        "paid_gate_allow":gate_allow,"paid_gate_abstain":gate_abstain,
        "paid_gate_records":paid_gate_records,
        "actor_readback_writes":readback_writes,
        "attested_gate_checks":attested_checks,
        "attested_gate_mismatches":attested_mismatches,
        "attested_after_suppressed_patch":attested_after_suppression,
        "actor_readback_records":readback_records,
        "final_attested_theta":memory.get(41,0),
        "checkpoint_records":checkpoints,
        "actor_paid_checkpoint_writes":checkpoint_count,
        "final_parent_checkpoint":memory.get(42,0),
        "rollback_attempt_records":rollback_attempt_records,
        "genuine_paid_physical_parent_restores":genuine_restores,
        "later_actual_rollback_readbacks":rollback_readback_records,
        "suppressed_pc102_count":suppressed_pc102_count}


