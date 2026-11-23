#!/usr/bin/env python3
"""C14c retrospective native post-restoration reachability audit.
This is a *retrospective* source-pinned test of the original 274-world paid
native trace, NOT a new independent randomized experiment or B3 claim.
"""
import hashlib
import json
import sys
import zipfile
from pathlib import Path

ORIGINAL_SHA256="9dbedd5624de349299da434677f6a8de6e6cf5a7df6784ba00d689d9ab8207be"
ORIGINAL_STUDY="CL1B-C14C-PAID-WRITER-ON-SUCCESS-SOURCE-RESTORATION"

def audit(path):
    raw=Path(path).read_bytes()
    assert hashlib.sha256(raw).hexdigest()==ORIGINAL_SHA256
    with zipfile.ZipFile(path) as z:
        assert z.testzip() is None
        names=z.namelist()
        assert len(names)==len(set(names))==278
        assert sum(n.endswith(".jsonl") for n in names)==274
        native=json.loads(z.read("cl1b_c14c_original_native_274_worlds.json"))
        independently_replayed=json.loads(z.read("cl1b_c14c_independent_replay.json"))
        assert native["study"]==independently_replayed["study"]==ORIGINAL_STUDY
        assert native["native_worlds"]==independently_replayed["native_worlds"]==274
        assert independently_replayed["first_divergence"] is None
        assert native["total_paid_cil"]==independently_replayed["exact_private_paid_cil_replayed"]==1054669
        assert len(native["records"])==45
        restored_worlds=[]
        totals={"paid_cil_after_effective_physical_rollback":0,
                "feedback_service_events_after_effective_physical_rollback":0,
                "true_pc173_executions_after_rollback":0,
                "true_pc28_edits_after_rollback":0,
                "physical_value_restoration_events":0}
        for case in native["records"]:
            assert len(case["arms"])==6
            real,suppressed=case["arms"][:2]
            assert real["arm"]=="truth_success_restore"
            assert suppressed["arm"]=="same_source_pc102_effect_suppressed"
            assert real["steps"]==suppressed["steps"]
            assert real["services"]==suppressed["services"]
            restorations=[r for r in real["physical_parent_restorations"] if r[4]]
            assert len(restorations)==real["effective_pc173_restorations"]
            assert suppressed["effective_pc173_restorations"]==0
            if not restorations:
                continue
            trace_path="cl1b_c14c_original_"+real["name"]+".jsonl"
            events=[json.loads(line) for line in z.read(trace_path).decode().splitlines() if line.strip()]
            cil_events=[e for e in events if e[0]=="cil"]
            feedback=[e for e in events if e[0]=="feedback"]
            assert len(cil_events)==real["steps"]
            assert len(feedback)==len(real["services"])
            for step,before,after,supp,effective in restorations:
                assert effective and not supp and before!=after
                target=[e for e in cil_events if e[1]==step]
                assert len(target)==1 and target[0][2]==102
                assert target[0][8][4]==after # original native observer signature: PC173 physical Const
                subsequent=[e for e in cil_events if e[1]>step]
                services=[e for e in feedback if e[1]>step]
                assert services and len(subsequent)>100
                old_reads=sum(e[2]==173 for e in subsequent)
                later_edits=sum(e[2]==28 for e in subsequent)
                totals["paid_cil_after_effective_physical_rollback"]+=len(subsequent)
                totals["feedback_service_events_after_effective_physical_rollback"]+=len(services)
                totals["true_pc173_executions_after_rollback"]+=old_reads
                totals["true_pc28_edits_after_rollback"]+=later_edits
                totals["physical_value_restoration_events"]+=1
                restored_worlds.append({
                    "case":case["name"],"restoration_paid_step":step,
                    "old_code_literal":before,"new_physical_source_literal":after,
                    "paid_instructions_after_restore":len(subsequent),
                    "real_feedback_services_after_restore":len(services),
                    "executions_of_restored_PC173_after_restore":old_reads,
                    "PC28_paid_mutations_after_restore":later_edits,
                    "immediate_native_post_restore_pc":subsequent[0][2],
                    "last_original_native_pc":cil_events[-1][2],
                    "same_initial_code_effect_suppression_had_same_total_paid":True,
                    "same_initial_code_effect_suppression_had_same_service_sequence":True
                })
        assert totals=={
            "paid_cil_after_effective_physical_rollback":17004,
            "feedback_service_events_after_effective_physical_rollback":395,
            "true_pc173_executions_after_rollback":0,
            "true_pc28_edits_after_rollback":0,
            "physical_value_restoration_events":5
        },totals
        assert len(restored_worlds)==5
        assert all(r["immediate_native_post_restore_pc"]==103 for r in restored_worlds)
        return {
            "study":"CL1B-C14C-PAID-PHYSICAL-ROLLBACK-POST-EXECUTION-REACHABILITY",
            "status":"RETROSPECTIVE_SOURCE_PINNED_NATIVE_AUDIT",
            "source_original_zip_sha256":ORIGINAL_SHA256,
            "full_original_native_paid_cil_independently_replayed":1054669,
            "source_effect_suppressed_same_initial_cil_pairs":45,
            "all_45_pairs_same_paid_lifecycle_and_true_feedback_services":True,
            "totals":totals,
            "all_five_cases":restored_worlds,
            "conclusion":"RESTORED_SOURCE_PHYSICALLY_CHANGED_BUT_NOT_REEXECUTED",
            "does_not_prove":"Generic future restoration unreachability or endogenous net adaptation",
            "B3_B4":"NOT_MET"
        }

if __name__=="__main__":
    assert len(sys.argv)==3,"usage audit.py original_C14c_actions_zip output.json"
    result=audit(sys.argv[1])
    Path(sys.argv[2]).write_text(json.dumps(result,indent=2,ensure_ascii=False)+"\n")
    print(json.dumps(result,indent=2,ensure_ascii=False))
