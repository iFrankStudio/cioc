#!/usr/bin/env python3
"""C2 native reuse accounting; call only AFTER full independent 184-world replay.

This reads the native summary, not future goals outside the frozen source.
No benefit or exposure count is hard-coded. Negative outcomes remain visible.
"""
import json
import sys
from collections import defaultdict
from pathlib import Path

MANIFEST=Path(__file__).resolve().parents[1]/"studies/CL1B-G2-PC28-LIFECYCLE-C2-frozen-184-worlds-v1.json"

def main():
    if len(sys.argv)!=2:raise SystemExit("usage: c2_native_audit.py native_summary.json")
    m=json.loads(MANIFEST.read_text(encoding="utf-8"))
    n=json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
    assert m["status"]=="PRE_NATIVE_FROZEN"
    assert m["study"]==n["study"]=="CL1B-G2-PC28-LIFECYCLE-C2"
    assert n["total_original_native_worlds"]==184
    a=n["truthful_records"]
    assert len(a)==180 and len(n["adversarial_records"])==4
    assert len(m["arms"])==180
    assert all(x["name"]==y["name"] for x,y in zip(a,m["arms"]))
    assert n["incomplete_truthful_world_names"]==[], "predeclared completion gate failed"
    costs={x:0 for x in ["G2_on","G2_pc28_suppressed","G2_placebo180","G2_never"]}
    families=defaultdict(lambda:{"pairs":0,"on_cost":0,"off_cost":0,"never_cost":0,"off_minus_on_cost":0,"changed_worlds":0,"reused_worlds":0})
    rows=[]
    for i in range(45):
        on,off,placebo,never=a[i*4:i*4+4]
        assert [x["source"] for x in (on,off,placebo,never)]==list(costs)
        assert all((x["completed_stages"],x["truthful_matches"],x["claim"],x["budget_exhausted"])==(48,48,1,False) for x in (on,off,placebo,never))
        assert on["goals"]==off["goals"]==placebo["goals"]==never["goals"]
        assert on["steps"]==placebo["steps"]
        assert on["services"]==placebo["services"] and on["decisions"]==placebo["decisions"]
        for x in (on,off,placebo,never):costs[x["source"]]+=x["steps"]
        delta=off["steps"]-on["steps"]
        changed=sum(1 for p in on["parameter_patches"] if p["changed"])
        reads=len(on["later_mutated_reads"])
        assert reads<=changed and len(off["suppressed_pc28"])==len(off["parameter_patches"])
        fam=families[on["regime"]]
        fam["pairs"]+=1
        fam["on_cost"]+=on["steps"];fam["off_cost"]+=off["steps"];fam["never_cost"]+=never["steps"]
        fam["off_minus_on_cost"]+=delta;fam["changed_worlds"]+=changed>0;fam["reused_worlds"]+=reads>0
        rows.append({"case":on["name"].removesuffix("_G2_on"),"regime":on["regime"],
          "on_cost":on["steps"],"off_cost":off["steps"],"never_cost":never["steps"],"off_minus_on_cost":delta,
          "on_pc28_attempts":len(on["parameter_patches"]),"on_effective_parameter_changes":changed,
          "on_later_mutated_reads":reads,"on_real_task_probes":len(on["services"]),
          "off_real_task_probes":len(off["services"])})
    assert len(families)==5 and all(f["pairs"]==9 for f in families.values())
    assert costs["G2_on"]==costs["G2_placebo180"]
    assert sum(x["off_minus_on_cost"] for x in rows)==n["paired_suppressed_minus_on_cost"]
    assert sum(x["on_pc28_attempts"] for x in rows)>0, "no paid modification exposure"
    assert (sum(x["off_minus_on_cost"]>0 for x in rows),
            sum(x["off_minus_on_cost"]<0 for x in rows),
            sum(x["off_minus_on_cost"]==0 for x in rows))==(n["wins_on"],n["losses_on"],n["ties"])
    print(json.dumps({"study":m["study"],"status":"C2_ORIGINAL_NATIVE_POST_REPLAY_REUSE_AUDIT",
      "total_truthful_worlds":180,"true_goals_per_world":48,"total_original_native_worlds":184,
      "primary_off_minus_on":n["paired_suppressed_minus_on_cost"],"arm_costs":costs,
      "on_wins":n["wins_on"],"on_losses":n["losses_on"],"ties":n["ties"],
      "worlds_with_paid_pc28":sum(x["on_pc28_attempts"]>0 for x in rows),
      "worlds_with_changed_parameter":sum(x["on_effective_parameter_changes"]>0 for x in rows),
      "worlds_with_later_mutated_read":sum(x["on_later_mutated_reads"]>0 for x in rows),
      "paid_pc28_executions":sum(x["on_pc28_attempts"] for x in rows),
      "effective_parameter_change_events":sum(x["on_effective_parameter_changes"] for x in rows),
      "later_mutated_read_events":sum(x["on_later_mutated_reads"] for x in rows),
      "families":dict(families),"cases":rows,
      "nonclaims":["No C1-vs-C2 length causal comparison; source and tasks differ.",
        "No actor-invented 48-stage horizon or mutation algorithm.",
        "No distribution-general net benefit, authenticated feedback or B3/B4."]},
      indent=2,ensure_ascii=False))
if __name__=="__main__":main()
