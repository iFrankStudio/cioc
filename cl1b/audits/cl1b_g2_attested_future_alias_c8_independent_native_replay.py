#!/usr/bin/env python3
"""Independent per-instruction C8 same-original-native-CIL exact future alias after actor-paid real code attestation."""
import json
import sys
from pathlib import Path

ROOT=Path(__file__).resolve().parents[1]
MANIFEST=ROOT/"studies/CL1B-G2-ATTESTED-VALUE-FUTURE-ALIAS-C8-frozen-274-worlds-v1.json"
SOURCES={
    "G2_readback_attest":"frozen_source_g2_readback_attest",
    "G2_readback_always":"frozen_source_g2_readback_always",
    "G2_readback_abstain":"frozen_source_g2_readback_abstain",
    "G2_readback_pc28_suppressed":"frozen_source_g2_readback_attest",
    "G2_readback_placebo180":"frozen_source_g2_readback_attest",
    "G2_intent_mirror_C6":"frozen_source_g2_intent_mirror_C6"
}
CELLS=(11,20,21,30,32,33,34,35,36,37,38,39,40,41)
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
    decisions=[]
    refills=[]
    patches=[]
    edits=[]
    later_reads=[]
    last_changed=None
    failed_halt=False
    suppressed_pc28=[]
    path=Path(folder)/("cl1b_g2_attested_future_alias_c8_original_"+arm["name"]+".jsonl")
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
                if not (at==28 and arm["source"]=="G2_readback_pc28_suppressed"):
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
                suppressed=at==28 and arm["source"]=="G2_readback_pc28_suppressed"
                assert code[target]["value"]==(pre_value if suppressed else value)
                if at==28:
                    assert value in (0,1,2)
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
            if at==52 and arm["source"]!="G2_intent_mirror_C6":
                assert kind=="Write" and memory[41]==regs[2]==theta()
                assert last173 is not None and last173<paid
                last52=paid
                readback_writes+=1
                if arm["source"]=="G2_readback_pc28_suppressed" and (
                    last_suppressed is not None and last173>last_suppressed
                ):
                    attested_after_suppression+=1
                readback_records.append({
                    "step":paid,"last_pc173":last173,
                    "actual_theta":theta(),"written_cell41":memory[41]
                })
            if at==67:
                assert kind=="Branch" and pc in (27,125)
                legacy=arm["source"]=="G2_intent_mirror_C6"
                key=40 if legacy else 41
                witnessed=memory.get(key,0)
                actual_theta=theta()
                if not legacy:
                    attested_checks+=1
                    assert last173 is not None and last52 is not None
                    assert last173<last52<paid
                    assert witnessed==actual_theta,"native CIL readback differs from actually executed source value"
                assert regs[0]==regs[3]-witnessed
                if witnessed!=actual_theta:attested_mismatches+=1
                expected_route={
                    "G2_readback_attest":27 if regs[0]!=0 else 125,
                    "G2_readback_always":27,
                    "G2_readback_abstain":125,
                    "G2_readback_pc28_suppressed":27 if regs[0]!=0 else 125,
                    "G2_readback_placebo180":27 if regs[0]!=0 else 125,
                    "G2_intent_mirror_C6":27 if regs[0]!=0 else 125
                }[arm["source"]]
                assert pc==expected_route
                if pc==27:gate_allow+=1
                else:gate_abstain+=1
                paid_gate_records.append({
                    "step":paid,"r3":regs[3],"r0":regs[0],
                    "route":pc,"witness":witnessed,
                    "actual_theta":actual_theta,"evidence_key":key,
                    "last_pc173":last173,"last_write":last52,
                    "decision":"EDIT" if pc==27 else "ABSTAIN"
                })
            if at==173:
                last173=paid
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
                "suppressed_pc28" if at==28 and arm["source"]=="G2_readback_pc28_suppressed"
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
        "final_attested_theta":memory.get(41,0)}

def main():
    if len(sys.argv)!=3:
        raise SystemExit("usage: cl1b_g2_attested_future_alias_c8_independent_native_replay.py rust_summary.json original_jsonl_dir")
    m=json.loads(MANIFEST.read_text(encoding="utf-8"))
    native=json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
    assert native["study"]==m["study"]=="CL1B-G2-ATTESTED-VALUE-FUTURE-ALIAS-C8"
    assert m["status"]=="PRE_NATIVE_FROZEN"
    assert native["status"]=="ORIGINAL_NATIVE_ACTOR_PAID_EXECUTED_CONST_SELF_READBACK"
    assert native["total_original_native_worlds"]==274
    all_arms=m["arms"]+m["controls"]
    expected=native["truthful_records"]+native["adversarial_records"]
    assert len(all_arms)==len(expected)==274
    costs={key:0 for key in SOURCES}
    families={}
    paid_total=0
    for arm,from_rust in zip(all_arms,expected):
        assert arm["name"]==from_rust["name"]
        independent=replay(m,arm,sys.argv[2])
        assert independent==from_rust,("original-native world differs from independent original CIL interpreter",arm["name"])
        paid_total+=independent["steps"]
        if arm["feedback"]=="truth":
            costs[arm["source"]]+=independent["steps"]
            families.setdefault(arm["regime"],{}).setdefault(arm["source"],0)
            families[arm["regime"]][arm["source"]]+=independent["steps"]
    assert paid_total==native["total_private_paid_cil"]
    assert costs==native["costs"]
    attest_count=read_count=mismatch=post_supp=allowed=skipped=effects=prefixes=noexposure=0
    cases=[]
    for i in range(45):
        gate,always,never,suppressed,placebo,legacy=expected[6*i:6*i+6]
        assert [x["source"] for x in (gate,always,never,suppressed,placebo,legacy)]==list(SOURCES)
        assert all(x["goals"]==gate["goals"] for x in (always,never,suppressed,placebo,legacy))
        assert all((x["completed_stages"],x["truthful_matches"],x["claim"],x["budget_exhausted"])==(48,48,1,False)
                   for x in (gate,always,never,suppressed,placebo,legacy))
        assert gate["steps"]==placebo["steps"]
        assert gate["services"]==placebo["services"]
        assert gate["actor_readback_records"]==placebo["actor_readback_records"]
        assert gate["paid_gate_records"]==placebo["paid_gate_records"]
        assert always["paid_gate_abstain"]==0
        assert never["paid_gate_allow"]==0
        for x in (gate,always,never,suppressed,placebo):
            assert x["attested_gate_mismatches"]==0
            assert x["actor_readback_writes"]==x["attested_gate_checks"]==x["paid_gate_allow"]+x["paid_gate_abstain"]
            mismatch+=x["attested_gate_mismatches"]
        allowed+=gate["paid_gate_allow"];skipped+=gate["paid_gate_abstain"]
        read_count+=gate["actor_readback_writes"]
        attest_count+=gate["attested_gate_checks"]
        post_supp+=suppressed["attested_after_suppressed_patch"]
        effects+=len(suppressed["suppressed_pc28"])
        def trace(x):
            p=Path(sys.argv[2])/("cl1b_g2_attested_future_alias_c8_original_"+x["name"]+".jsonl")
            return [json.loads(line) for line in p.read_text(encoding="utf-8").splitlines() if line.strip()]
        tgate=trace(gate);tplacebo=trace(placebo);tsupp=trace(suppressed)
        assert tgate==tplacebo,"unreachable PC180 placebo changed native record"
        if not suppressed["suppressed_pc28"]:
            noexposure+=1
            assert tgate==tsupp
        else:
            first=suppressed["suppressed_pc28"][0]["step"]
            k=next(j for j,z in enumerate(tsupp) if z[0]=="cil" and z[1]==first and z[2]==28)
            assert tgate[:k]==tsupp[:k]
            assert tsupp[k][10]=="suppressed_pc28"
            prefixes+=1
        cases.append({
            "case":m["cases"][i]["name"],"regime":m["cases"][i]["regime"],
            "attested_cost":gate["steps"],"always_cost":always["steps"],"abstain_cost":never["steps"],
            "suppressed_cost":suppressed["steps"],"legacy_C6_different_source_cost":legacy["steps"],
            "paid_value_readbacks":gate["actor_readback_writes"],
            "paid_actor_edit":gate["paid_gate_allow"],
            "paid_actor_skip":gate["paid_gate_abstain"],
            "paid_readback_after_researcher_suppressed_edit":suppressed["attested_after_suppressed_patch"],
            "never_misreport_actual_theta_on_paid_gate":True
        })
    assert (read_count,attest_count,mismatch,post_supp,allowed,skipped,effects,prefixes,noexposure)==(
        native["actor_paid_readback_writes"],native["actor_attested_paid_gate_checks"],
        native["actor_attested_gate_mismatches_all_matching_arms"],
        native["true_physical_attestations_after_effect_suppression"],
        native["actor_normal_gate_edits"],native["actor_normal_gate_abstains"],
        native["actual_paid_pc28_effect_suppressions"],
        native["first_paid_suppression_prefix_witnesses"],native["no_suppression_pairs"])
    assert effects>0 and post_supp>0 and attest_count>0 and allowed>0 and skipped>0
    assert costs["G2_readback_always"]-costs["G2_readback_attest"]==native["readback_net_saving_vs_always"]
    assert costs["G2_readback_abstain"]-costs["G2_readback_attest"]==native["readback_net_saving_vs_abstain"]
    assert costs["G2_readback_pc28_suppressed"]-costs["G2_readback_attest"]==native["causal_suppressed_minus_readback_cost"]
    assert (costs["G2_readback_attest"]<costs["G2_readback_always"] and
            costs["G2_readback_attest"]<costs["G2_readback_abstain"])==native["main_efficiency_accepted"]
    prefix_alias_pairs=attested_anchors=patched_anchors=flipped_anchors=exposed_flipped=0
    positive=negative=ties=0
    alias_evidence=[]
    for ai,anchor in enumerate(m["anchors"]):
        assert len(anchor["prefix"])==24
        for k in range(3):
            assert m["cases"][3*ai+k]["goals"][:24]==anchor["prefix"]
        assert len({tuple(m["cases"][3*ai+k]["goals"]) for k in range(3)})==3
        for armidx in range(6):
            a=expected[18*ai+armidx]
            base=trace(a)
            cut=next(i for i,e in enumerate(base) if e[0]=="feedback" and e[2]==24)
            assert any(e[0]=="cil" and e[2]==173 for e in base[:cut])
            for k in (1,2):
                b=expected[18*ai+6*k+armidx]
                new=trace(b)
                cut_b=next(i for i,e in enumerate(new) if e[0]=="feedback" and e[2]==24)
                assert base[:cut]==new[:cut_b],("full paid-CIL same-prefix future violation",anchor["anchor"],a["source"],k)
                prefix_alias_pairs+=1
        base=trace(expected[18*ai])
        cutoff=next(e[1] for e in base if e[0]=="feedback" and e[2]==24)
        c=expected[18*ai]
        was_attested=any(x["step"]<cutoff for x in c["actor_readback_records"])
        was_patched=any(x["step"]<cutoff for x in c["parameter_patches"])
        attested_anchors+=was_attested;patched_anchors+=was_patched
        deltas=[];paid=[]
        for k in range(3):
            case=m["cases"][3*ai+k]
            live=expected[18*ai+6*k]["steps"]
            off=expected[18*ai+6*k+3]["steps"]
            d=off-live
            positive+=(d>0);negative+=(d<0);ties+=(d==0)
            deltas.append(d)
            paid.append({"future":case["tail"],"live":live,"suppressed":off,
                "same_source_suppressed_minus_live":d,
                "privileged_hindsight_choice":"allow_all" if d>0 else "suppress_all" if d<0 else "either"})
        flipped=min(deltas)<0 and max(deltas)>0
        flipped_anchors+=flipped;exposed_flipped+=(flipped and was_patched and was_attested)
        alias_evidence.append({
            "anchor":anchor["anchor"],"regime":anchor["regime"],
            "paid_code_readback_before_fork":bool(was_attested),
            "paid_pc28_patch_before_fork":bool(was_patched),
            "branch_before_future_first_feedback_paid_instruction":cutoff,
            "native_fork_stage":24,
            "exact_complete_original_native_prefix_equal_across_all_six_arms":True,
            "signed_same_source_pc28_effect_full_lifecycle":deltas,
            "strict_positive_and_negative_future_sign_flip":bool(flipped),
            "future_paid_outcomes":paid})
    assert prefix_alias_pairs==180
    assert positive+negative+ties==45
    assert alias_evidence==native["future_alias_records"],"Rust native per-anchor future alias does not match independent CIL outcome calculations"
    assert (prefix_alias_pairs,attested_anchors,patched_anchors,positive,negative,ties,
        flipped_anchors,exposed_flipped)==(
        native["prefix_alias_pairs_exact"],native["anchors_paid_source_readback_before_fork"],
        native["anchors_paid_pc28_before_fork"],native["future_benefit_positive"],
        native["future_benefit_negative"],native["future_benefit_tied"],
        native["anchors_with_both_effect_signs"],
        native["exposed_attested_anchors_with_both_effect_signs"])
    print(json.dumps({
        "study":m["study"],
        "status":"INDEPENDENT_FULL_ORIGINAL_NATIVE_PAID_EXECUTED_CONST_READBACK",
        "verified_original_native_worlds":274,
        "verified_charged_original_cil_instructions":paid_total,
        "first_divergence":None,
        "verified_identical_original_native_history_prefix_pairs":prefix_alias_pairs,
        "verified_frozen_same_prefix_anchor_histories":15,
        "source_attested_before_new_future_anchors":attested_anchors,
        "paid_self_code_modification_before_new_future_anchors":patched_anchors,
        "same_source_PC28_effect_full_run_positive":positive,
        "same_source_PC28_effect_full_run_negative":negative,
        "same_source_PC28_effect_full_run_tied":ties,
        "anchors_both_future_profit_signs":flipped_anchors,
        "attested_exposed_anchors_both_future_profit_signs":exposed_flipped,
        "future_alias_records":alias_evidence,
        "actor_paid_readback_writes":read_count,
        "verified_gate_fresh_physical_theta":attest_count,
        "physical_theta_mismatches_at_all_same-source_gates":mismatch,
        "suppressed_PatchConst_attempts_paid":effects,
        "physical_source_attestations_after_suppressed_write":post_supp,
        "actor_gate_modify_and_skip":(allowed,skipped),
        "source_identical_suppression_minus_live_cost":native["causal_suppressed_minus_readback_cost"],
        "net_attested_cost_saving_vs_always":native["readback_net_saving_vs_always"],
        "net_attested_cost_saving_vs_abstain":native["readback_net_saving_vs_abstain"],
        "main_efficiency_accepted":native["main_efficiency_accepted"],
        "arm_costs":costs,"family_costs":families,"case_records":cases,
        "nonclaims":[
            "This is after executing original PC173, not a generic introspection or trusted VM attestation opcode.",
            "No newly invented policy in C8; identical C7 actor and before-fork evidence, future effects evaluated by researchers after full execution.",
            "C6 old intent-shadow is different source; C8 attested 180-node code is byte-for-byte C7.",
            "No authenticated environmental feedback or CL1B B3/B4."
        ]
    },ensure_ascii=False,indent=2))
if __name__=="__main__":main()
