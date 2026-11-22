#!/usr/bin/env python3
"""Independent per-instruction C6 Rust-native original actor-maintained self-state actor-paid PC67 gate and PC28 exact-effect replay."""
import json
import sys
from pathlib import Path

ROOT=Path(__file__).resolve().parents[1]
MANIFEST=ROOT/"studies/CL1B-G2-ACTOR-SELFSTATE-MIRROR-C6-frozen-274-worlds-v1.json"
SOURCES={
    "G2_mirror":"frozen_source_g2_mirror",
    "G2_mirror_always":"frozen_source_g2_mirror_always",
    "G2_mirror_abstain":"frozen_source_g2_mirror_abstain",
    "G2_mirror_pc28_suppressed":"frozen_source_g2_mirror",
    "G2_mirror_placebo180":"frozen_source_g2_mirror",
    "G2_legacy_label_gate":"frozen_source_g2_legacy_label_gate"
}
CELLS=(11,20,21,30,32,33,34,35,36,37,38,39,40)
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
    mirror_writes=mirror_checks=mirror_mismatches=mirror_noops=0
    decisions=[]
    refills=[]
    patches=[]
    edits=[]
    later_reads=[]
    last_changed=None
    failed_halt=False
    suppressed_pc28=[]
    path=Path(folder)/("cl1b_g2_selfstate_mirror_c6_original_"+arm["name"]+".jsonl")
    with path.open(encoding="utf-8") as stream:
        lines=(json.loads(line) for line in stream if line.strip())
        def edges():
            return [code[12]["next"],code[34]["next"],code[46]["value"],code[133]["next"]]
        def theta():
            n=code[173]
            assert n["kind"]=="Const" and n["dst"]==2 and n["next"]==174
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
                if not (at==28 and arm["source"]=="G2_mirror_pc28_suppressed"):
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
                assert target==(173 if at==28 else 46)
                suppressed=at==28 and arm["source"]=="G2_mirror_pc28_suppressed"
                assert code[target]["value"]==(pre_value if suppressed else value)
                if at==28:
                    assert value in (0,1,2)
                    patches.append({"step":paid,"pc":28,"target":173,
                        "before":pre_value,"attempted":value,
                        "after":pre_value if suppressed else value,
                        "changed":not suppressed and pre_value!=value,
                        "suppressed":suppressed})
                    if suppressed:
                        suppressed_pc28.append({"step":paid,"pc":28,"target":173,
                            "attempted":value,"before":pre_value,"after":pre_value})
                    elif pre_value!=value:
                        last_changed=value
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
            if at==52 and arm["source"]!="G2_legacy_label_gate":
                assert kind=="Write" and memory[40]==regs[3]
                mirror_writes+=1
            if at==67:
                assert kind=="Branch" and pc in (27,125)
                shadow=memory.get(40,0)
                actual_theta=theta()
                if arm["source"]!="G2_legacy_label_gate":
                    mirror_checks+=1
                    assert regs[0]==regs[3]-shadow
                    if shadow!=actual_theta:
                        mirror_mismatches+=1
                        assert arm["source"]=="G2_mirror_pc28_suppressed"
                expected_route={
                    "G2_mirror":27 if regs[0]!=0 else 125,
                    "G2_mirror_always":27,
                    "G2_mirror_abstain":125,
                    "G2_mirror_pc28_suppressed":27 if regs[0]!=0 else 125,
                    "G2_mirror_placebo180":27 if regs[0]!=0 else 125,
                    "G2_legacy_label_gate":27 if regs[3]!=0 else 125
                }[arm["source"]]
                assert pc==expected_route
                if pc==27:gate_allow+=1
                else:gate_abstain+=1
                if arm["source"]=="G2_mirror" and regs[3]==shadow:
                    assert pc==125
                    mirror_noops+=1
                paid_gate_records.append({
                    "step":paid,"r3":regs[3],"r0":regs[0],
                    "route":pc,"shadow":shadow,"actual_theta":actual_theta,
                    "shadow_mismatch":shadow!=actual_theta,
                    "decision":"EDIT" if pc==27 else "ABSTAIN"
                })
            if at==173:
                assert regs[2]==theta()
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
                "suppressed_pc28" if at==28 and arm["source"]=="G2_mirror_pc28_suppressed"
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
        "paid_mirror_writes":mirror_writes,
        "mirror_gate_checks":mirror_checks,
        "mirror_gate_mismatches":mirror_mismatches,
        "mirror_noop_skips":mirror_noops,
        "final_shadow":memory.get(40,0)}

def main():
    if len(sys.argv)!=3:
        raise SystemExit("usage: cl1b_g2_selfstate_mirror_c6_independent_native_replay.py summary.json original_jsonl_dir")
    m=json.loads(MANIFEST.read_text(encoding="utf-8"))
    native=json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
    assert m["study"]==native["study"]=="CL1B-G2-ACTOR-SELFSTATE-MIRROR-C6"
    assert m["status"]=="PRE_NATIVE_FROZEN"
    assert native["status"]=="ORIGINAL_NATIVE_ACTOR_PAID_STATE_MIRROR"
    assert native["total_original_native_worlds"]==274
    all_arms=m["arms"]+m["controls"]
    expected=native["truthful_records"]+native["adversarial_records"]
    assert len(all_arms)==len(expected)==274
    costs={name:0 for name in SOURCES}
    families={}
    paid_total=0
    for arm,from_rust in zip(all_arms,expected):
        assert arm["name"]==from_rust["name"]
        independent=replay(m,arm,sys.argv[2])
        assert independent==from_rust,("full native world summary does not match independent original CIL",arm["name"])
        paid_total+=independent["steps"]
        if arm["feedback"]=="truth":
            costs[arm["source"]]+=independent["steps"]
            families.setdefault(arm["regime"],{}).setdefault(arm["source"],0)
            families[arm["regime"]][arm["source"]]+=independent["steps"]
    assert costs==native["costs"]
    assert paid_total==native["total_private_paid_cil"]
    noops=allow=abstain=writes=mismatch=effects=first_prefix=no_exposure=0
    individual=[]
    for i in range(45):
        gate,always,fixed_off,poison,placebo,legacy=expected[6*i:6*i+6]
        assert [x["source"] for x in [gate,always,fixed_off,poison,placebo,legacy]]==list(SOURCES)
        assert all(x["goals"]==gate["goals"] for x in [always,fixed_off,poison,placebo,legacy])
        assert all((x["completed_stages"],x["truthful_matches"],x["claim"],x["budget_exhausted"])==(48,48,1,False) for x in [gate,always,fixed_off,poison,placebo,legacy])
        assert gate["mirror_gate_mismatches"]==0 and always["mirror_gate_mismatches"]==0
        assert fixed_off["mirror_gate_mismatches"]==0 and placebo["mirror_gate_mismatches"]==0
        assert gate["paid_gate_allow"]==gate["paid_mirror_writes"]
        assert always["paid_gate_allow"]==always["paid_mirror_writes"]
        assert fixed_off["paid_gate_allow"]==fixed_off["paid_mirror_writes"]==0
        assert gate["mirror_noop_skips"]==gate["paid_gate_abstain"]
        assert gate["steps"]==placebo["steps"]
        assert gate["paid_gate_records"]==placebo["paid_gate_records"]
        noops+=gate["mirror_noop_skips"]
        allow+=gate["paid_gate_allow"];abstain+=gate["paid_gate_abstain"]
        writes+=gate["paid_mirror_writes"]
        mismatch+=poison["mirror_gate_mismatches"]
        effects+=len(poison["suppressed_pc28"])
        def trace(x):
            path=Path(sys.argv[2])/("cl1b_g2_selfstate_mirror_c6_original_"+x["name"]+".jsonl")
            return [json.loads(line) for line in path.read_text(encoding="utf-8").splitlines() if line.strip()]
        tgate=trace(gate);tplacebo=trace(placebo);tpoison=trace(poison)
        assert tgate==tplacebo,"native placebo mutated original CIL or feedback history"
        if not poison["suppressed_pc28"]:
            no_exposure+=1
            assert tgate==tpoison,"unexposed identical-source world diverged"
        else:
            first=poison["suppressed_pc28"][0]["step"]
            j=next(k for k,x in enumerate(tpoison) if x[0]=="cil" and x[1]==first and x[2]==28)
            assert tgate[:j]==tpoison[:j],"pre-intervention original CIL/feedback history must be identical"
            assert tpoison[j][10]=="suppressed_pc28"
            first_prefix+=1
        individual.append({"name":m["cases"][i]["name"],"regime":m["cases"][i]["regime"],
            "mirror_cost":gate["steps"],"always_cost":always["steps"],
            "never_cost":fixed_off["steps"],"poisoned_cost_NOT_CAUSAL":poison["steps"],
            "legacy_different_source_cost":legacy["steps"],
            "actual_paid_mirror_writes":gate["paid_mirror_writes"],
            "actual_skipped_redundant_edits":gate["mirror_noop_skips"],
            "source_edits_differ_from_comparison":gate["steps"]-min(always["steps"],fixed_off["steps"])})
    assert allow==writes and abstain==noops
    assert (allow,abstain,writes,noops)==(
       native["actor_mirror_gate_allow"],native["actor_mirror_gate_abstain"],
       native["actual_mirror_parameter_writes"],native["skipped_paid_same_parameter_edits"])
    assert mismatch==native["poisoned_shadow_mismatch_gate_events"]
    assert effects==native["poisoned_suppressed_pc28_attempts"]
    assert first_prefix==native["first_native_suppression_prefix_witnesses"]
    assert no_exposure==native["unexposed_poison_pairs"]
    assert costs["G2_mirror_always"]-costs["G2_mirror"]==native["mirror_net_saving_vs_always"]
    assert costs["G2_mirror_abstain"]-costs["G2_mirror"]==native["mirror_net_saving_vs_abstain"]
    assert (costs["G2_mirror"]<costs["G2_mirror_always"] and
            costs["G2_mirror"]<costs["G2_mirror_abstain"])==native["main_efficiency_accepted"]
    print(json.dumps({
      "study":"CL1B-G2-ACTOR-SELFSTATE-MIRROR-C6",
      "status":"INDEPENDENT_FULL_ORIGINAL_NATIVE_PAID_SELFSTATE_MIRROR_REPLAY",
      "source_manifest":"cl1b/studies/CL1B-G2-ACTOR-SELFSTATE-MIRROR-C6-frozen-274-worlds-v1.json",
      "verified_original_native_worlds":274,
      "verified_charged_original_cil_instructions":paid_total,
      "first_divergence":None,
      "actor_mirror_gate_allow":allow,"actor_mirror_gate_abstain":abstain,
      "paid_mirror_writes":writes,"skipped_same_value_edits":noops,
      "poisoned_shadow_mismatch_gate_events":mismatch,
      "poisoned_paid_pc28_effect_suppressions":effects,
      "original_native_first_suppression_prefixes":first_prefix,
      "full_fee_mirror_vs_always_saving":native["mirror_net_saving_vs_always"],
      "full_fee_mirror_vs_abstain_saving":native["mirror_net_saving_vs_abstain"],
      "main_efficiency_accepted":native["main_efficiency_accepted"],
      "arm_costs":costs,
      "family_arm_costs":families,
      "per_case":individual,
      "nonclaims":[
         "Actor's paid data mirror proves self-state memory and no-op edit rejection, not prediction of future net rewards.",
         "Mirror host PC28 suppression poisons cell40 truth; NOT a pure causal mutation-effect comparison.",
         "Historical legacy r3 gate uses different source; cost difference is descriptive only.",
         "No original actor-invented algorithm, authenticated environmental feedback, CL1B B3/B4."
      ]
    },indent=2,ensure_ascii=False))
if __name__=="__main__":main()
