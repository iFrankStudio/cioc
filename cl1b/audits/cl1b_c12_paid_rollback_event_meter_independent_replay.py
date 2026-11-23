#!/usr/bin/env python3
"""Independent complete paid CIL replay for C12 original 182 native metering worlds.
No imports from CL-0 Rust/world libraries or native-run output-derived expected code.
"""
import json
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
SOURCE = ROOT / "studies/CL1B-G2-ACTOR-PAID-STATE-TRIGGERED-ROLLBACK-C10-frozen-274-worlds-v1.json"
CELLS = (11, 20, 21, 30, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43)
CREDIT = 6000

def source(m, metered):
    code = [dict(x) for x in m["frozen_source_g2_rollback"]]
    assert len(code) == 180 and code[95] == {"kind":"Branch","test":0,"nonzero":99,"zero":86}
    if metered:
        code[95]["nonzero"] = 180
        code.extend([
            {"kind":"Const","dst":2,"value":43,"next":181},
            {"kind":"Read","dst":0,"address":2,"next":182},
            {"kind":"Const","dst":2,"value":1,"next":183},
            {"kind":"Add","dst":0,"lhs":0,"rhs":2,"next":184},
            {"kind":"Const","dst":2,"value":43,"next":185},
            {"kind":"Write","address":2,"value":0,"next":99},
        ])
    assert len(code) == (186 if metered else 180)
    return code

def replay(m, record, folder):
    name, metered = record["name"], record["metered"]
    suppress = record["suppress_restore"]
    mode, goals = record["feedback"], record["goals"]
    assert mode in ("truth", "inverted", "forced0") and len(goals) == 48
    code = source(m, metered)
    regs = [0,0,0,0]
    memory = {}
    services = []
    pc = paid = meter_writes = rollback_attempts = reverts = 0
    peak = len(code) + 1

    def sig():
        assert code[173]["kind"] == "Const"
        assert all(code[i]["kind"] == "Write" for i in (12,34,133))
        assert code[46]["kind"] == "Const"
        return [code[12]["next"],code[34]["next"],code[46]["value"],
                code[133]["next"],code[173]["value"]]

    def occupied():
        return len(code)+1+len(memory)

    path = Path(folder)/("cl1b_c12_original_"+name+".jsonl")
    with path.open(encoding="utf-8") as f:
        rows=(json.loads(line) for line in f)
        def check(wanted):
            actual=next(rows,None)
            assert actual==wanted, json.dumps({"name":name,"paid":paid,
                "actual":actual,"independent":wanted},ensure_ascii=False)[:5000]

        while paid<CREDIT:
            at=pc
            op=code[at]
            kind=op["kind"]
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
                assert code[target]["kind"]=="Const"
                old=code[target]["value"]
                new=regs[op["value"]]
                if not (at==102 and suppress):
                    code[target]["value"]=new
                pc=op["next"]
                if at==102:
                    assert target==173
                    rollback_attempts+=1
                    if not suppress and old!=new:
                        reverts+=1
            elif kind=="PatchNext":
                target=regs[op["node"]]
                assert code[target]["kind"]=="Write"
                code[target]["next"]=regs[op["target"]]
                pc=op["next"]
            elif kind=="Halt":
                pc=at
            else:
                raise AssertionError(("unknown opcode",kind,at))

            if at==185:
                meter_writes+=1
                assert memory[43]==rollback_attempts+1
            assert 0<=pc<len(code) and len(memory)<=16 and occupied()<=300
            peak=max(peak,occupied())
            check(["cil",paid,at,pc,list(regs),CREDIT-paid,
                   [memory.get(k) for k in CELLS],occupied(),sig(),
                   "suppressed_pc102" if at==102 and suppress
                   else "paid_meter_write" if at==185 else "normal"])
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
        else:
            raise AssertionError(("private CIL exhausted",name))
        assert next(rows,None) is None, ("excess native trace rows",name)

    assert meter_writes==(rollback_attempts if metered else 0)
    assert memory.get(43)==(rollback_attempts if metered else None)
    correct=sum(x[2] for x in services)
    completed,claim=memory.get(32,0),memory.get(21,0)
    if mode=="truth":
        assert (completed,correct,claim)==(48,48,1)
    elif mode=="forced0":
        assert (completed,claim)==(0,0)
    else:
        assert claim==1 and correct<len(services)
    assert record["paid"]==paid
    assert record["service_attempts"]==len(services)
    assert record["meter_writes"]==meter_writes
    assert record["meter_value"]==memory.get(43)
    assert record["peak_occupied"]==peak
    assert record["final_signature"]==sig()
    assert record["completed_stages"]==completed
    assert record["true_matches"]==correct
    assert record["claim"]==claim
    assert record["rollback_attempts"]==rollback_attempts
    assert record["physical_value_changes_at_pc102"]==reverts
    return paid,len(services),rollback_attempts

def main(summary_path,folder):
    m=json.loads(SOURCE.read_text())
    s=json.loads(Path(summary_path).read_text())
    assert s["study"]=="CL1B-C12-PAID-ROLLBACK-EVENT-METER"
    assert len(s["all_cases"])==45 and len(s["deceptive_records"])==2
    total=fee=ref_saving=measured_saving=0
    for case in s["all_cases"]:
        a,b,c,d=[case[k] for k in (
            "metered_live","metered_suppressed","reference_live","reference_suppressed")]
        values=[replay(m,x,folder) for x in (a,b,c,d)]
        total+=sum(x[0] for x in values)
        assert values[0][1]==values[2][1] and values[1][1]==values[3][1]
        assert values[0][2]==values[2][2] and values[1][2]==values[3][2]
        assert values[0][0]==values[2][0]+6*values[2][2]
        assert values[1][0]==values[3][0]+6*values[3][2]
        fee+=6*(values[0][2]+values[1][2])
        dr=values[3][0]-values[2][0]
        dm=values[1][0]-values[0][0]
        assert case["reference_same_source_saving"]==dr
        assert case["metered_same_source_saving"]==dm
        ref_saving+=dr
        measured_saving+=dm
    for record in s["deceptive_records"]:
        total+=replay(m,record,folder)[0]
    assert s["native_worlds"]==182
    assert s["total_native_private_cil"]==total
    assert s["total_meter_fee_across_truthful_metered_arms"]==fee
    assert s["reference_same_source_PC102_effect_net_saving"]==ref_saving==1840
    assert s["metered_same_source_PC102_effect_net_saving"]==measured_saving
    return {"status":"INDEPENDENT_COMPLETE_CIL_PARITY","first_divergence":None,
        "native_worlds":182,"all_exact_replayed_private_CIL":total,
        "C12_true_arms_meter_fee":fee,
        "C10_same_source_PC102_saving":ref_saving,
        "C12_metered_same_source_PC102_saving":measured_saving,
        "B3_B4":"NOT_MET"}

if __name__=="__main__":
    assert len(sys.argv)==3,"usage: auditor summary.json evidence_folder"
    print(json.dumps(main(sys.argv[1],sys.argv[2]),ensure_ascii=False,indent=2))
