#!/usr/bin/env python3
"""Re-audit original C16 274-world native ZIP and the exact 45 same-source pairs.
Retrospective evidence analysis, NOT a new holdout or new paid CIL execution.
"""
import hashlib
import json
import sys
import zipfile
from pathlib import Path

ORIGINAL_SHA = "c5a37e69dd885db9aed29caa2107bdb65ee05c78ea3f8b2283ce90d5d9316479"
ARMS = (
    "truth_one_use_reentry",
    "same_source_pc102_effect_suppressed",
    "same_source_pc202_ticket_consumption_suppressed",
    "different_source_c15_unlimited_reentry",
    "different_source_c14c_no_reentry",
    "different_source_c9_fixed_abstain",
)
FEES = dict(zip(ARMS, (195446, 195087, 196159, 194665, 191608, 132701)))
COMPLETED = dict(zip(ARMS, (45, 45, 43, 43, 45, 45)))
STAGES = dict(zip(ARMS, (2160, 2160, 2106, 2106, 2160, 2160)))

def audit(path):
    assert hashlib.sha256(Path(path).read_bytes()).hexdigest() == ORIGINAL_SHA
    with zipfile.ZipFile(path) as z:
        assert z.testzip() is None
        names = z.namelist()
        assert len(names) == len(set(names)) == 278
        assert sum(name.endswith(".jsonl") for name in names) == 274
        native = json.loads(z.read("cl1b_c16_original_native_274_worlds.json"))
        other = json.loads(z.read("cl1b_c16_independent_replay.json"))
        assert native["study"] == other["study"] == "CL1B-C16-SINGLE-USE-PAID-SUCCESS-CERTIFICATE"
        assert native["native_worlds"] == other["native_worlds"] == 274
        assert native["total_paid_cil"] == other["exact_private_paid_cil_replayed"] == 1112914
        assert other["first_divergence"] is None
        assert len(native["records"]) == 45 and len(native["adversarial_records"]) == 4
        assert native["arm_fees"] == FEES
        assert native["arm_completed_worlds"] == COMPLETED

        def trace(record):
            b = z.read("cl1b_c16_original_" + record["name"] + ".jsonl")
            return [json.loads(line) for line in b.splitlines() if line.strip()]

        sums = {arm: {"fee":0, "stages":0, "full":0, "minted":0,
                      "spent":0, "blocked":0, "physical":0} for arm in ARMS}
        changed_ticket = []
        changed_physical = []
        pair_rows = []
        paid_ticket_invariants = 0
        for case in native["records"]:
            row = case["arms"]
            assert tuple(v["arm"] for v in row) == ARMS
            assert all(v["feedback"] == "truth" and v["goals"] == row[0]["goals"] for v in row)
            for arm,v in zip(ARMS,row):
                a = sums[arm]
                a["fee"] += v["steps"]
                a["stages"] += v["completed_stages"]
                a["full"] += int(v["completed_stages"] == 48 and v["claim"] == 1
                                 and not v["budget_exhausted"])
                a["minted"] += v["paid_success_certificates_minted"]
                a["spent"] += v["paid_ticket_spend_writes"]
                a["blocked"] += v["paid_ticket_consumed_gate_rejections"]
                a["physical"] += v["effective_pc173_restorations"]

            live, physical, no_ticket = row[:3]
            assert live["completed_stages"] == physical["completed_stages"] == 48
            original = trace(live)
            fresh = False
            for e in original:
                if e[0] != "cil":
                    continue
                if e[2] == 196:
                    fresh = True
                    assert e[6][-1] == 0
                if e[2] == 202:
                    assert fresh, ("duplicate physical certificate spend",case["name"],e[1])
                    assert e[6][-1] == 1
                    fresh = False
                    paid_ticket_invariants += 1
                if e[2] == 199 and e[3] == 103 and e[6][-1] == 1:
                    assert not fresh
            assert sum(e[0] == "cil" and e[2] == 196 for e in original) == live["paid_success_certificates_minted"]
            assert sum(e[0] == "cil" and e[2] == 202 for e in original) == live["paid_ticket_spend_writes"]

            ticket_diff = live["steps"] != no_ticket["steps"] or live["services"] != no_ticket["services"]
            physical_diff = live["steps"] != physical["steps"] or live["services"] != physical["services"]
            if ticket_diff:
                assert live["completed_stages"] == 48 and no_ticket["budget_exhausted"]
                control = trace(no_ticket)
                step = next(e[1] for e in original if e[0]=="cil" and e[2]==202)
                prefix_live = [e for e in original if e[1] < step]
                prefix_control = [e for e in control if e[1] < step]
                assert prefix_live == prefix_control, ("different paid native state before first ticket write",case["name"])
                at = next(e for e in original if e[0]=="cil" and e[1]==step)
                suppressed = next(e for e in control if e[0]=="cil" and e[1]==step)
                assert at[2] == suppressed[2] == 202
                assert at[3:6] == suppressed[3:6]
                assert at[6][:-1] == suppressed[6][:-1]
                assert (at[6][-1],suppressed[6][-1]) == (1,0)
                assert (at[9],suppressed[9]) == ("normal","suppressed_pc202")
                changed_ticket.append(dict(name=case["name"], first_paid_PC202_step=step,
                    identical_pre_effect_events=len(prefix_live),
                    stages_live=live["completed_stages"], stages_without_ticket_state=no_ticket["completed_stages"],
                    stages_saved=live["completed_stages"]-no_ticket["completed_stages"],
                    credits_live=live["steps"],credits_without_ticket_state=no_ticket["steps"],
                    ticket_spends_live=live["paid_ticket_spend_writes"],
                    ticket_spends_without_ticket_state=no_ticket["paid_ticket_spend_writes"],
                    ticket_double_use_blocks_live=live["paid_ticket_consumed_gate_rejections"]))
            else:
                assert live["steps"] == no_ticket["steps"] and live["services"] == no_ticket["services"]
            if physical_diff:
                assert live["completed_stages"] == physical["completed_stages"] == 48
                changed_physical.append(dict(name=case["name"],
                    live_paid=live["steps"], suppressed_paid=physical["steps"],
                    live_real_physical_edits=live["effective_pc173_restorations"],
                    suppressed_real_physical_edits=physical["effective_pc173_restorations"]))
            pair_rows.append(dict(name=case["name"], live_stages=live["completed_stages"],
                ticket_control_stages=no_ticket["completed_stages"], physical_control_stages=physical["completed_stages"],
                live_fee=live["steps"],ticket_control_fee=no_ticket["steps"],physical_control_fee=physical["steps"]))

        assert len(changed_ticket)==2 and len(changed_physical)==2
        assert {v["name"]:v["stages_saved"] for v in changed_ticket} == {
            "c16_pulse_trains_a2_d0":22, "c16_irregular_switchback_a2_d2":32,
        }
        assert sum(v["stages_saved"] for v in changed_ticket)==54
        assert sum(v["credits_without_ticket_state"]-v["credits_live"] for v in changed_ticket)==713
        assert paid_ticket_invariants==35
        assert {arm:v["fee"] for arm,v in sums.items()}==FEES
        assert {arm:v["full"] for arm,v in sums.items()}==COMPLETED
        assert {arm:v["stages"] for arm,v in sums.items()}==STAGES
        assert sums[ARMS[0]]["minted"]==78 and sums[ARMS[0]]["spent"]==35
        assert sums[ARMS[0]]["blocked"]==4 and sums[ARMS[0]]["physical"]==33
        assert sums[ARMS[1]]["physical"]==0
        assert sums[ARMS[2]]["spent"]==sums[ARMS[2]]["physical"]==73
        assert native["same_source_live_vs_suppressed_paid_or_feedback_difference_cases"]==2
        assert native["same_source_live_vs_ticket_spend_effect_difference_cases"]==2
        assert all(v["paid_success_certificates_minted"]==0 for v in native["adversarial_records"])
        assert all(v["completed_stages"]==48 and v["true_matches"]==41 and v["claim"]==1
                   for v in native["adversarial_records"] if v["feedback"]=="inverted")
        return dict(study="CL1B-C16-NATIVE-PAID-SINGLE-USE-CERTIFICATE-FIRST-CAUSAL-EFFECT",
            status="RETROSPECTIVE_SOURCE_PINNED_COMPLETE_45_PAIRS",
            original_zip_sha256=ORIGINAL_SHA, full_original_274_worlds_replay_first_divergence=None,
            original_paid_cil=1112914, forty_five_strict_same_source_true_pairs=45,
            paid_ticket_PC202_single_use_invariant_events=35, true_ticket_effect_cases=2,
            both_affected_cases_identical_native_pre_first_ticket_write=True,
            stages_saved_by_ticket_actual_write_effect=54, CIL_saved_by_ticket_effect=713,
            main_physical_source_edit_stage_gain_vs_same_source_suppression=0,
            main_physical_effect_CIL_disadvantage=359,
            arm_metrics=sums, ticket_causal_cases=changed_ticket,
            physical_effect_cases=changed_physical, complete_45_true_pairs=pair_rows,
            deceptive_controls=[{k:v[k] for k in ("name","feedback","true_matches","completed_stages","claim","steps","paid_success_certificates_minted")}
                                for v in native["adversarial_records"]],
            scientific_status="B3_B4_NOT_MET")

if __name__ == "__main__":
    assert len(sys.argv)==3, "usage: audit.py ORIGINAL_NATIVE_ZIP output.json"
    result=audit(sys.argv[1])
    Path(sys.argv[2]).write_text(json.dumps(result,indent=2,ensure_ascii=False)+"\n",encoding="utf-8")
    print(json.dumps({k:v for k,v in result.items()
                      if k not in ("complete_45_true_pairs","deceptive_controls")},indent=2,ensure_ascii=False))
