#!/usr/bin/env python3
"""Postresult zero-new-world repair-reachability audit of original Study88 SQLite.

Run only with the exact source-pinned original Study88 CI archive. The original
prospective E/L/H/R counts remain unchanged; this studies interpretation.
"""
import argparse
import hashlib
import json
import sqlite3
from pathlib import Path

import analyze_study88 as original
import analyze_study82 as codegen

SOURCE_SHA = "6863443229608760ad33f17e72c5a08657c997c9"
EXPECTED_SHA256 = 27_648
ARMS = ("intact", "sham", "lesion-native", "lesion-edit-disabled", "lesion-host-restore")


def require(ok, message):
    if not ok:
        raise ValueError(message)


def kind(event, name):
    typ = event["kind"]
    return typ.get(name) if isinstance(typ, dict) else None


def internal(event):
    return event["origin"] == {"Internal": 1}


def native_kind_and_edge(op):
    name, args = codegen.variant(op)
    return name, args.get("next")


def arm_trace(conn, seed, source, arm):
    label = f"seed={seed}/source={source}/{arm}"
    rows = conn.execute(
        "SELECT id,source_revision,initial_json,final_json,actions_json,attempts_json "
        "FROM epoch_runs WHERE condition=?", (label,)
    ).fetchall()
    require(len(rows) == 1, "missing unique original registered Study88 arm")
    run_id, revision, initial_raw, final_raw, actions_raw, attempts_raw = rows[0]
    require(revision == SOURCE_SHA, "not original Study88 CI source revision")
    pre = json.loads(initial_raw)
    final = json.loads(final_raw)
    actions = json.loads(actions_raw)
    charges = json.loads(attempts_raw)
    snaps = conn.execute(
        "SELECT epoch,state_sha256,state_json FROM epoch_snapshots WHERE run_id=? ORDER BY epoch",
        (run_id,),
    ).fetchall()
    require(len(snaps) == 18 and len(charges) == 18,
            "incomplete original postfork epoch records")
    previous = pre
    for expected, (epoch, h, raw) in enumerate(snaps):
        require(epoch == expected, "original epoch numbers not sequential")
        require(hashlib.sha256(raw.encode()).hexdigest() == h,
                "original snapshot SHA256 mismatch")
        state = json.loads(raw)
        require(state["events"][:len(previous["events"])] == previous["events"],
                "original event stream not prefix-preserving")
        previous = state
    require(previous == final, "original final state differs from terminal epoch")
    after = final["events"][len(pre["events"]):]
    natives = [e for e in after if internal(e)]
    external_edits = [
        e for e in after
        if e["origin"] == "External"
        and (message := kind(e, "Intervened")) is not None
        and message.startswith("PatchEdge")
    ]
    mutation_events = [
        e for e in natives
        if kind(e, "ExecutableEdgePatched") is not None
    ]
    output_events = [
        e for e in natives
        if (value := kind(e, "CellWritten")) is not None
        and value["key"] == 9
    ]
    trapped = [e for e in natives if kind(e, "Trapped") is not None]
    deferred = [e for e in natives if kind(e, "DrawDeferred") is not None]
    pre_actor = pre["processes"]["1"]
    final_actor = final["processes"]["1"]
    prefix_tag, prefix_edge = native_kind_and_edge(pre_actor["code"]["nodes"][8])
    final_tag, final_edge = native_kind_and_edge(final_actor["code"]["nodes"][8])
    return {
        "run_id": run_id,
        "seed": seed,
        "source": source,
        "arm": arm,
        "prefork_actor_pc": pre_actor["pc"],
        "prefork_actor_credits": pre_actor["credits"],
        "prefork_PC8_opcode": prefix_tag,
        "prefork_PC8_next": prefix_edge,
        "final_PC8_opcode": final_tag,
        "final_PC8_next": final_edge,
        "original_external_patch_events": [kind(e, "Intervened") for e in external_edits],
        "original_native_pc_attempts": [e["pc"] for e in natives],
        "native_attempt_count": len(natives),
        "native_successful_edge_edits": [kind(e, "ExecutableEdgePatched") for e in mutation_events],
        "native_source_writes": [{"pc": e["pc"], "value": kind(e, "CellWritten")["value"]} for e in output_events],
        "traps": [kind(e, "Trapped") for e in trapped],
        "deferred_draw_requests": len(deferred),
        "postfork_attempt_charges": sum(charges),
        "original_postfork_epoch_snapshots": len(snaps),
        "original_epoch0_edge": native_kind_and_edge(json.loads(snaps[0][2])["processes"]["1"]["code"]["nodes"][8])[1],
        "original_plan_patch_actions": [
            a for a in actions if "PatchEdge" in a["action"]
        ],
    }


def audit(path):
    # Full read-only revalidation using the SOURCE-FROZEN independent Study88
    # Python auditor. All original hashes and 128 original genotype labels.
    verified = original.auditor(path, SOURCE_SHA)
    require(verified["original_distinct_genotypes"] == 128
            and verified["original_matched_worlds"] == 1536
            and verified["verified_original_postfork_epoch_sha256"] == EXPECTED_SHA256,
            "original Study88 cohort or hash count mismatch")
    require((verified["pre_fork_exposed_E"],
             verified["effective_lesion_L"],
             verified["explicit_host_restoration_controls"],
             verified["strict_native_repair_R"]) == (1, 1, 1, 0),
            "original registered outcomes unexpectedly changed")
    selected = [
        d["seed"] for d in verified["seed_gate_outcomes"]
        if d["eligible_E"] and d["loss_L"] and d["external_restore_positive"]
    ]
    require(len(selected) == 1, "expected exactly one original E/L/H case")
    seed = selected[0]
    conn = sqlite3.connect(path)
    records = {
        f"source={source}/{arm}": arm_trace(conn, seed, source, arm)
        for source in (3, 8)
        for arm in ARMS
    }
    blocked = []
    for source in (3, 8):
        native = records[f"source={source}/lesion-native"]
        disabled = records[f"source={source}/lesion-edit-disabled"]
        host = records[f"source={source}/lesion-host-restore"]
        condition = (
            native["prefork_actor_pc"] == 8
            and native["prefork_actor_credits"] > 0
            and native["prefork_PC8_opcode"] != "PatchNext"
            and native["prefork_PC8_opcode"] not in ("Branch", "Halt")
            and native["original_epoch0_edge"] == 8
            and native["final_PC8_next"] == 8
            and native["final_PC8_opcode"] == native["prefork_PC8_opcode"]
            and native["native_attempt_count"] > 0
            and set(native["original_native_pc_attempts"]) == {8}
            and not native["native_successful_edge_edits"]
            and not native["native_source_writes"]
            and disabled["original_epoch0_edge"] == 8
            and host["final_PC8_next"] == host["prefork_PC8_next"]
            and bool(host["native_source_writes"])
            and len(native["original_external_patch_events"]) == 1
        )
        blocked.append(condition)
    return {
        "study": 89,
        "status": "RETROSPECTIVE_ZERO_NEW_ECOLOGICAL_WORLDS",
        "original_source_revision": SOURCE_SHA,
        "original_workflow_run": 43597236032,
        "verified_original_postfork_epoch_sha256": EXPECTED_SHA256,
        "original_exposed_E": 1,
        "original_effective_lesion_L": 1,
        "original_host_positive_H": 1,
        "original_native_restoration_R": 0,
        "independent_new_genotypes": 0,
        "selected_original_E_L_H_seed": seed,
        "strict_structural_edit_inaccessibility_both_sources": all(blocked),
        "interpretation": (
            "ACTOR_STRANDED_AT_NONEDIT_PC8_NO_NATIVE_REPAIR_REACHABILITY"
            if all(blocked) else "STRUCTURAL_BLOCK_NOT_PROVEN"
        ),
        "native_actor_details": records,
        "cl1_closed": False,
    }


def main():
    p = argparse.ArgumentParser()
    p.add_argument("sqlite", type=Path)
    p.add_argument("--json", type=Path, required=True)
    args = p.parse_args()
    result = audit(args.sqlite)
    args.json.write_text(json.dumps(result, ensure_ascii=False, indent=2, sort_keys=True) + "\n")
    print(json.dumps(
        {k: result[k] for k in (
            "study", "status", "selected_original_E_L_H_seed",
            "strict_structural_edit_inaccessibility_both_sources",
            "interpretation", "original_native_restoration_R",
        )}, sort_keys=True,
    ))


if __name__ == "__main__":
    main()
