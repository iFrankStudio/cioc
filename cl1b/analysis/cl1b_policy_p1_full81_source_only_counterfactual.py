#!/usr/bin/env python3
"""P1 finite out-of-sample ablation: entire 3^4 task-goal cube.

Both arms use identical frozen source, hidden goals, paid CIL instructions,
feedback and resource accounting. Only native PatchConst effect is disabled
in the counterfactual. This file is SOURCE-ONLY, never label native.
A cherry-picked +47-instruction example does NOT imply improvement over all
81 possible goal schedules. Compare both positive and negative transfer.
"""
import itertools
import json
from collections import Counter
from pathlib import Path

HERE=Path(__file__).resolve().parents[1]
FROZEN=HERE/"studies/CL1B-POLICY-P1-frozen-62-node-feedback-conditioned-executable-search-stride-v1.json"
KEYS=(11,20,21,30,32,33)
SOURCE=json.loads(FROZEN.read_text())["original_native_CIL_source"]
assert len(SOURCE)==62 and SOURCE[45]["kind"]=="PatchConst"

def evaluate(goals,suppress):
    source=[dict(x) for x in SOURCE]
    cells={}
    registers=[0,0,0,0]
    pc=0
    count=0
    probes=[]
    patches=[]
    while count<800:
        count+=1
        op=source[pc]
        kind=op["kind"]
        if kind=="Const":
            registers[op["dst"]]=op["value"]
        elif kind=="Read":
            registers[op["dst"]]=cells.get(registers[op["address"]],0)
        elif kind in ("Add","Sub"):
            a,b=registers[op["lhs"]],registers[op["rhs"]]
            registers[op["dst"]]=a+b if kind=="Add" else a-b
        elif kind=="Write":
            key=registers[op["address"]]
            value=registers[op["value"]]
            cells[key]=value
            if pc==3:
                assert key==11
                episode=cells.get(32,0)
                assert episode<4
                correct=int(value==goals[episode])
                cells[20]=correct
                probes.append((episode,value,correct))
        elif kind=="Branch":
            pc=op["nonzero"] if registers[op["test"]] else op["zero"]
            continue
        elif kind=="PatchConst":
            assert pc==45 and registers[op["node"]]==46
            assert len(patches)==0
            before=source[46]["value"]
            if not suppress:
                source[46]["value"]=registers[op["value"]]
            patches.append((before,source[46]["value"]))
        elif kind=="Halt":
            assert len(cells)<=16
            return {
                "steps":count,
                "claim":cells.get(21),
                "completed":cells.get(32),
                "probes":len(probes),
                "true_hits":sum(t for _,_,t in probes),
                "stride":source[46]["value"],
                "patches":patches,
            }
        else:
            raise AssertionError((kind,pc))
        pc=op["next"]
    raise AssertionError(("source-executed model exceeded private compute",goals))

def main():
    rows=[]
    for goals in itertools.product((0,1,2),repeat=4):
        a=evaluate(goals,False)
        b=evaluate(goals,True)
        assert (a["claim"],a["completed"],a["true_hits"])==(1,4,4)
        assert (b["claim"],b["completed"],b["true_hits"])==(1,4,4)
        rows.append({
            "goals":list(goals),
            "paid_step_saving_from_policy_patch":b["steps"]-a["steps"],
            "task_probe_saving_from_policy_patch":b["probes"]-a["probes"],
            "original_patched_stride":a["stride"],
        })
    histogram=Counter(r["paid_step_saving_from_policy_patch"] for r in rows)
    benefits=[r for r in rows if r["paid_step_saving_from_policy_patch"]>0]
    harms=[r for r in rows if r["paid_step_saving_from_policy_patch"]<0]
    ties=[r for r in rows if r["paid_step_saving_from_policy_patch"]==0]
    assert len(rows)==81
    assert next(r for r in rows if r["goals"]==[0,2,1,0])["paid_step_saving_from_policy_patch"]==47
    assert next(r for r in rows if r["goals"]==[0,1,2,0])["paid_step_saving_from_policy_patch"]==0
    print(json.dumps({
        "study":"CL1B-POLICY-P1",
        "level":"SOURCE_ONLY_81_GOAL_ABLATION_NOT_NATIVE",
        "n_finite_goal_sequences":81,
        "n_benefits":len(benefits),
        "n_harms":len(harms),
        "n_ties":len(ties),
        "total_paid_steps_saved":sum(r["paid_step_saving_from_policy_patch"] for r in rows),
        "total_task_probes_saved":sum(r["task_probe_saving_from_policy_patch"] for r in rows),
        "saving_histogram":dict(sorted(histogram.items())),
        "best_case":max(rows,key=lambda x:x["paid_step_saving_from_policy_patch"]),
        "worst_case":min(rows,key=lambda x:x["paid_step_saving_from_policy_patch"]),
        "all_81_source_only":rows,
        "nonclaims":"No original native VM run for these 81 additional schedules, no evidence of algorithm invention or broad task generalization. Holdout noncyclic schedules can produce unfavorable transfers."
    },indent=2))
if __name__=="__main__":
    main()
