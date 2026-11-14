#!/usr/bin/env python3
"""Study95 retrospective original-only source/host writer route diagnosis.

Revalidates all Study94 original genomes/epochs before inspecting the unique
preregistered E/L case. Adds ZERO ecological worlds and no revised R gate.
"""
import argparse
import hashlib
import json
import sqlite3
from pathlib import Path

import analyze_study94 as original
import analyze_study82 as original_module

SOURCE_SHA = "b59ce0cd2b7a960d0157ca3deb54e2de9bc861ae"
ORIGINAL_RUN = 43599692512
ARMS = (
    "intact", "sham", "lesion-native", "lesion-edit-disabled",
    "lesion-host-restore", "lesion-draw-ablated",
)


def require(ok, message):
    if not ok:
        raise ValueError(message)


def native(event):
    return event["origin"] == {"Internal": 1}


def event_value(event, name):
    kind = event["kind"]
    return kind.get(name) if isinstance(kind, dict) else None


def edge_at(state, pc):
    tag, fields = original.variant(state["processes"]["1"]["code"]["nodes"][pc])
    require(tag not in ("Halt", "Branch") and "next" in fields,
            "target native predecessor has no concrete single edge")
    return fields["next"]


def read_arm(conn, seed, source, arm, site):
    label = f"seed={seed}/source={source}/{arm}"
    rows = conn.execute(
        "SELECT id,source_revision,initial_sha256,initial_json,actions_json,"
        "attempts_json,final_sha256,final_json FROM epoch_runs WHERE condition=?",
        (label,),
    ).fetchall()
    require(len(rows) == 1, "original source/arm missing or duplicated")
    run_id, revision, init_hash, raw_init, raw_plan, raw_charges, end_hash, raw_end = rows[0]
    require(revision == SOURCE_SHA, "original ecological revision differs")
    require(hashlib.sha256(raw_init.encode()).hexdigest() == init_hash
            and hashlib.sha256(raw_end.encode()).hexdigest() == end_hash,
            "original initial/final hash invalid")
    initial = json.loads(raw_init)
    final = json.loads(raw_end)
    plan = json.loads(raw_plan)
    charges = json.loads(raw_charges)
    rows = conn.execute(
        "SELECT epoch,state_sha256,state_json FROM epoch_snapshots "
        "WHERE run_id=? ORDER BY epoch",
        (run_id,),
    ).fetchall()
    require(len(rows) == 18 and len(charges) == 18, "original epochs missing")
    states = []
    preceding = initial
    events_per_epoch = []
    for expected, (epoch, state_hash, raw_state) in enumerate(rows):
        require(epoch == expected and hashlib.sha256(raw_state.encode()).hexdigest() == state_hash,
                "original SHA256 epoch hash mismatch")
        state = json.loads(raw_state)
        events = state["events"]
        require(events[:len(preceding["events"])] == preceding["events"],
                "original native event prefix violated")
        events_per_epoch.append(events[len(preceding["events"]):])
        states.append(state)
        preceding = state
    require(states[-1] == final, "original end state deviates from verified last epoch")
    p, writer, continuation, offset = site
    # Exact original CIL def-use trace, not value-coincidence: a native Write
    # of the expected number can be untainted after Const or a different path.
    original_trace = original.trace_native(final, original_module.expected_code(seed), source)
    true_function_ordinals = {
        record["attempt"] for record in original_trace["writes"]
        if record["tainted"] and record["pc"] == writer and record["offset"] == offset
    }
    original_native_ordinal = 0
    true_function_ticks = set()
    for event in final["events"]:
        if native(event):
            original_native_ordinal += 1
            if original_native_ordinal in true_function_ordinals:
                true_function_ticks.add(event["tick"])
    native_by_phase = {
        "early_0_3": [e for events in events_per_epoch[:4] for e in events if native(e)],
        "late_4_9": [e for events in events_per_epoch[4:10] for e in events if native(e)],
        "late_10_17": [e for events in events_per_epoch[10:] for e in events if native(e)],
    }

    def native_function_outputs(events):
        return [
            {"pc": e["pc"], "value": event_value(e, "CellWritten")["value"],
             "tick": e["tick"]}
            for e in events
            if (e["tick"] in true_function_ticks and e["pc"] == writer
                and (record := event_value(e, "CellWritten")) is not None
                and record["key"] == 9
                and record["value"] == source + offset)
        ]

    def state(epoch):
        obj = states[epoch]
        actor = obj["processes"]["1"]
        return {
            "pc": actor["pc"], "credits": actor["credits"],
            "stopped": actor["stopped"], "r0": actor["registers"][0],
            "p_next": edge_at(obj, p),
            "public_pool": obj["compute_pool"],
        }

    patch_events = [
        {
            "epoch": idx, "tick": e["tick"], "origin": e["origin"],
            "text": event_value(e, "Intervened"),
        }
        for idx, events in enumerate(events_per_epoch)
        for e in events
        if e["origin"] == "External" and
        (text := event_value(e, "Intervened")) is not None and text.startswith("PatchEdge")
    ]
    first_two_charged = [
        e for events in events_per_epoch for e in events if native(e)
    ][:2]
    report = {
        "source": source,
        "arm": arm,
        "first_postfork_two_native_pcs": [e["pc"] for e in first_two_charged],
        "first_postfork_two_native_writes": native_function_outputs(first_two_charged),
        "original_postfork_epochs": len(states),
        "original_postfork_charged_attempts": sum(charges),
        "immediately_before_external_host_repair": state(3),
        "after_external_host_repair_epoch": state(4),
        "original_final_actor_state": state(17),
        "exact_host_patch_events": patch_events,
        "phase_counts": {},
    }
    for phase, events in native_by_phase.items():
        report["phase_counts"][phase] = {
            "native_attempts": len(events),
            "native_visit_predecessor_p": sum(e["pc"] == p for e in events),
            "native_visit_writer_w": sum(e["pc"] == writer for e in events),
            "native_actual_successful_source_writer_events": native_function_outputs(events),
            "native_traps": [event_value(e, "Trapped") for e in events
                             if event_value(e, "Trapped") is not None],
            "native_draw_deferrals": sum(event_value(e, "DrawDeferred") is not None
                                         for e in events),
            "native_code_edit_events": [
                event_value(e, "ExecutableEdgePatched") for e in events
                if event_value(e, "ExecutableEdgePatched") is not None
            ],
            "all_distinct_native_pcs": sorted({e["pc"] for e in events}),
        }
    if arm == "lesion-host-restore":
        require(
            len(patch_events) == 2
            and [event["epoch"] for event in patch_events] == [0, 4]
            and patch_events[1]["origin"] == "External"
            and report["after_external_host_repair_epoch"]["p_next"] == writer,
            "host restoration not at registered epoch4/P/W with actual code edge",
        )
    return report


def audit(sqlite):
    validated = original.auditor(sqlite, SOURCE_SHA)
    require((validated["original_distinct_genotypes"],
             validated["original_matched_worlds"],
             validated["verified_original_postfork_epoch_sha256"])
            == (512, 6144, 110592), "original Study94 cohort or hashes differ")
    require((validated["pre_fork_exposed_E"],
             validated["effective_lesion_L"],
             validated["explicit_host_restoration_controls"],
             validated["strict_native_repair_R"]) == (1, 1, 0, 0),
            "original Study94 registered outcome changed")
    eligible = [row for row in validated["seed_gate_outcomes"]
                if row["eligible_E"] and row["loss_L"]]
    require(len(eligible) == 1, "not exactly one original registered E/L genotype")
    selection = eligible[0]
    seed = selection["seed"]
    site = tuple(selection["selected_p_w_c_offset"])
    require(len(site) == 4, "original registered source-writer P/W/C missing")
    with sqlite3.connect(sqlite) as conn:
        details = {
            f"source={source}/{arm}": read_arm(conn, seed, source, arm, site)
            for source in (3, 8) for arm in ARMS
        }
    first_two = {
        source: {
            "sham": details[f"source={source}/sham"]["first_postfork_two_native_writes"],
            "lesion": details[f"source={source}/lesion-native"]["first_postfork_two_native_writes"],
        } for source in (3, 8)
    }
    require(all(
        len(first_two[source]["sham"]) >= 1 and len(first_two[source]["lesion"]) == 0
        for source in (3, 8)
    ), "original registered L did not survive event-level first-two recheck")
    sham_late = {
        source: [
            len(details[f"source={source}/sham"]["phase_counts"][phase]
                ["native_actual_successful_source_writer_events"]) for phase in ("late_4_9", "late_10_17")
        ] for source in (3, 8)
    }
    host_late = {
        source: [
            len(details[f"source={source}/lesion-host-restore"]["phase_counts"][phase]
                ["native_actual_successful_source_writer_events"]) for phase in ("late_4_9", "late_10_17")
        ] for source in (3, 8)
    }
    host_pc_reencounters = {
        source: sum(
            details[f"source={source}/lesion-host-restore"]["phase_counts"][phase]
                ["native_visit_predecessor_p"]
            for phase in ("late_4_9", "late_10_17")
        ) for source in (3, 8)
    }
    return {
        "study": 95,
        "status": "RETROSPECTIVE_REPLAY_OF_ORIGINAL_DATA_ZERO_NEW_WORLDS",
        "source_revision": SOURCE_SHA,
        "source_ci": ORIGINAL_RUN,
        "reverified_original_genotypes": 512,
        "reverified_original_matched_worlds": 6144,
        "reverified_original_epoch_sha256": 110592,
        "independent_new_genotypes": 0,
        "registered_E_L_H_R": [1, 1, 0, 0],
        "selected_original_EL_seed": seed,
        "original_unchanged_p_writer_w_continuation_c_offset": list(site),
        "original_sham_late_window_outputs_by_source": sham_late,
        "original_host_restored_late_window_outputs_by_source": host_late,
        "host_restored_actor_revisits_original_predecessor_P_by_source": host_pc_reencounters,
        "matched_source_arm_event_state_details": details,
        "cl1_closed": False,
    }


def main():
    p = argparse.ArgumentParser()
    p.add_argument("sqlite", type=Path)
    p.add_argument("--json", type=Path, required=True)
    args = p.parse_args()
    result = audit(args.sqlite)
    args.json.write_text(json.dumps(result, indent=2, sort_keys=True) + "\n")
    print(json.dumps({key: result[key] for key in (
        "study", "selected_original_EL_seed",
        "original_unchanged_p_writer_w_continuation_c_offset",
        "original_sham_late_window_outputs_by_source",
        "original_host_restored_late_window_outputs_by_source",
        "host_restored_actor_revisits_original_predecessor_P_by_source",
    )}, sort_keys=True))


if __name__ == "__main__":
    main()
