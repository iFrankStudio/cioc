#!/usr/bin/env python3
"""Study85: retrospective original Study84 failed host positive-control anatomy.

No new native worlds. Executes the exact original source and epoch-hash auditor.
"""
import argparse
import json
import sqlite3
from pathlib import Path

import analyze_study84 as original

ORIGINAL_REVISION = "85087980474e3885f41576f3ae50f3c925edba83"
ARMS = ("sham", "lesion-native", "lesion-edit-disabled", "lesion-host-restore", "lesion-draw-ablated")


def require(ok, message):
    if not ok:
        raise ValueError(message)


def kind_name(event):
    kind = event["kind"]
    return next(iter(kind)) if isinstance(kind, dict) else kind


def original_arm_record(database, seed, source, arm):
    label = f"seed={seed}/source={source}/{arm}"
    rows = database.execute(
        "SELECT id,initial_json,actions_json,attempts_json,final_json FROM epoch_runs WHERE condition=?",
        (label,),
    ).fetchall()
    require(len(rows) == 1, f"original arm missing or duplicate: {label}")
    run_id, init_raw, actions_raw, attempts_raw, final_raw = rows[0]
    states = [
        json.loads(raw)
        for epoch, raw in database.execute(
            "SELECT epoch,state_json FROM epoch_snapshots WHERE run_id=? ORDER BY epoch",
            (run_id,),
        )
    ]
    require(len(states) == 18, "original full postfork epoch history missing")
    return {
        "initial": json.loads(init_raw),
        "actions": json.loads(actions_raw),
        "charges": json.loads(attempts_raw),
        "final": json.loads(final_raw),
        "epochs": states,
    }


def analyze_arm(original_run, seed, source, arm):
    initial = original_run["initial"]
    states = original_run["epochs"]
    code8 = initial["processes"]["1"]["code"]["nodes"][8]
    pre_edge = original.executable_edge(initial["processes"]["1"]["code"]["nodes"])
    require(pre_edge is not None, "known source exposure has unpatchable code edge")
    before_host = states[3] if arm == "lesion-host-restore" else None
    end_host = states[4] if arm == "lesion-host-restore" else None
    pre_host_actor = before_host["processes"]["1"] if before_host else None
    host_after_edge = original.executable_edge(end_host["processes"]["1"]["code"]["nodes"]) if end_host else None
    host_restore_action = [
        a for a in original_run["actions"]
        if a["epoch"] == 4 and "PatchEdge" in a["action"]
    ]
    if arm == "lesion-host-restore":
        require(
            host_restore_action == [
                {"epoch": 4, "action": {"PatchEdge": {"process": 1, "node": 8, "next": pre_edge}}}
            ],
            "external restoration target does not match original pre-fork code edge",
        )
        require(host_after_edge == pre_edge,
                "external restoration was not reflected in real executable code")
    entries = []
    previous = initial
    for epoch, state in enumerate(states):
        events = state["events"][len(previous["events"]):]
        native = [e for e in events if e["origin"] == {"Internal": 1}]
        outputs = []
        reads = []
        edits = []
        draws = 0
        stalls = 0
        stops = []
        for e in native:
            kind = kind_name(e)
            details = e["kind"]
            if kind == "CellWritten" and details["CellWritten"]["key"] == 9:
                outputs.append({"pc": e["pc"], "value": details["CellWritten"]["value"]})
            elif kind == "CellRead" and details["CellRead"]["key"] == 7:
                reads.append({"pc": e["pc"], "value": details["CellRead"]["value"]})
            elif kind == "ExecutableEdgePatched":
                edits.append({"pc": e["pc"], **details["ExecutableEdgePatched"]})
            elif kind == "DrawDeferred":
                stalls += 1
            elif kind in ("Trapped", "Halted"):
                stops.append({"pc": e["pc"], "reason": details.get(kind, kind) if isinstance(details, dict) else kind})
            elif kind == "Executed":
                opcode, _ = original.variant(
                    original.original.expected_code(seed)[e["pc"]]
                )
                if opcode == "DrawCompute":
                    draws += 1
        actor = state["processes"]["1"]
        entries.append({
            "postfork_epoch": epoch,
            "global_epoch": 6 + epoch,
            "charged_attempts": original_run["charges"][epoch],
            "actor_pc": actor["pc"],
            "actor_credits": actor["credits"],
            "actor_stopped": actor["stopped"],
            "available_common_pool": state["compute_pool"],
            "source_reads": reads,
            "source_writes": outputs,
            "real_native_code_edge_edits": edits,
            "successful_native_draws": draws,
            "deferred_draws": stalls,
            "stops": stops,
        })
        previous = state
    if arm == "lesion-host-restore":
        if pre_host_actor["stopped"]:
            classification = "actor_irreversibly_stopped_before_external_restoration"
        elif pre_host_actor["credits"] == 0 and before_host["compute_pool"] == 0:
            classification = "zero_compute_before_external_restoration"
        elif not any(epoch["source_writes"] for epoch in entries[4:]):
            classification = "no_native_output_after_host_restoration"
        else:
            classification = "some_native_output_but_strict_repeated_function_not_restored"
    else:
        classification = "non_host_control"
    return {
        "seed": seed,
        "source": source,
        "arm": arm,
        "pre_fork_edge_pc8_next": pre_edge,
        "pc8_source_opcode": original.variant(code8)[0],
        "host_restore_action": host_restore_action if arm == "lesion-host-restore" else [],
        "host_restored_edge_after_epoch4": host_after_edge,
        "actor_stopped_before_host_action": pre_host_actor["stopped"] if pre_host_actor else None,
        "actor_credits_before_host_action": pre_host_actor["credits"] if pre_host_actor else None,
        "classification": classification,
        "epochs": entries,
    }


def audit(path):
    full = original.auditor(path, ORIGINAL_REVISION)
    require(
        full["original_distinct_genotypes"] == 128
        and full["original_matched_worlds"] == 1536
        and full["verified_original_postfork_epoch_sha256"] == 27648
        and full["pre_fork_exposed_E"] == 10
        and full["effective_lesion_L"] == 1
        and full["strict_native_repair_R"] == 0,
        "original Stage84 source-pinned causal gate changed",
    )
    affected = [
        record["seed"]
        for record in full["seed_gate_outcomes"] if record["loss_L"]
    ]
    require(len(affected) == 1, "not exactly one known original effective lesion")
    db = sqlite3.connect(path)
    records = []
    for seed in affected:
        for source in (3, 8):
            for arm in ARMS:
                run = original_arm_record(db, seed, source, arm)
                records.append(analyze_arm(run, seed, source, arm))
    host = [r for r in records if r["arm"] == "lesion-host-restore"]
    require(len(host) == 2, "missing host source controls")
    return {
        "study": 85,
        "status": "POST_OUTCOME_READ_ONLY_DIAGNOSIS_NO_NEW_ECOLOGY",
        "source_revision": ORIGINAL_REVISION,
        "full_original_genotypes_reaudited": 128,
        "full_original_worlds_reaudited": 1536,
        "full_original_epoch_sha256_reaudited": 27648,
        "new_stochastic_worlds": 0,
        "known_original_loss_seed_ids": affected,
        "host_restoration_source3_classification": host[0]["classification"],
        "host_restoration_source8_classification": host[1]["classification"],
        "traced_arms": records,
        "cl1_closed": False,
    }


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("sqlite", type=Path)
    parser.add_argument("--json", type=Path, required=True)
    args = parser.parse_args()
    result = audit(args.sqlite)
    args.json.write_text(json.dumps(result, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    print(json.dumps(
        {key: result[key] for key in (
            "study", "status", "source_revision",
            "known_original_loss_seed_ids",
            "host_restoration_source3_classification",
            "host_restoration_source8_classification",
        )},
        sort_keys=True,
    ))


if __name__ == "__main__":
    main()
