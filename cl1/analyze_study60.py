#!/usr/bin/env python3
"""Study60 post-result original Study59 native child opcode and source lineage.

All original Stage59 SHA256 snapshots are rechecked by its frozen prospective
independent paired auditor BEFORE this descriptive event classification.
"""
from __future__ import annotations

import argparse
import json
import sqlite3
from collections import Counter, defaultdict
from pathlib import Path

import analyze
import analyze_study59

ORIGINAL_REVISION = "a5a65b2a542780edfb770345eb4d12daaadf9160"
FIRST = 4_000_000
SEEDS = 256
ARMS = ("ambient-general", "ambient-only", "closed-general",
        "zero-credit-ambient-general")


def event_name(event: dict) -> str:
    kind = event["kind"]
    if isinstance(kind, str):
        return kind
    if isinstance(kind, dict) and len(kind) == 1:
        return next(iter(kind))
    raise ValueError("unrecognized native event")


def internal_actor(event: dict) -> bool:
    return analyze.internal_process(event) == event["process"]


def source_history(events: list[dict], launch: dict) -> list[dict]:
    """Only source graph copies genuinely executed by the launching parent."""
    result = []
    for event in events:
        if event["tick"] >= launch["tick"]:
            break
        if not internal_actor(event) or event["process"] != launch["parent"]:
            continue
        name = event_name(event)
        if name not in ("GraphNodeCopied", "GraphNodeCopiedLocal"):
            continue
        details = event["kind"][name]
        if (details["owner"] != launch["parent"]
                or details["source_process"] != launch["parent"]):
            raise ValueError("native source copy actor attribution forged")
        if details["graph"] == launch["graph"]:
            result.append({
                "tick": event["tick"],
                "mode": "local" if name.endswith("Local") else "absolute",
                "parent": event["process"],
                "graph": details["graph"],
                "source_index": details["source_node"],
                "new_graph_node": details["new_node"],
            })
    return result


def child_opcode_at_pc(state_before: dict, state_after: dict,
                       child: int, pc: int) -> dict:
    candidates = []
    for state in (state_before, state_after):
        process = state["processes"].get(str(child))
        if process is None:
            continue
        nodes = process["code"]["nodes"]
        if isinstance(pc, int) and 0 <= pc < len(nodes):
            candidates.append(nodes[pc])
    if not candidates:
        raise ValueError("native child event PC lacks executable code")
    if any(node != candidates[0] for node in candidates):
        return {"runtime_opcode": "NOT EVALUABLE: changed between hashed epochs"}
    op = candidates[0]
    name = next(iter(op)) if isinstance(op, dict) else op
    return {"runtime_opcode": name, "runtime_node": op}


def classify_run(initial: dict, snapshots: list[dict], actions: list[dict]) -> dict:
    if len(snapshots) != 24:
        raise ValueError("historical run missing 24 immutable epochs")
    launches = {}
    initial_event_count = len(initial["events"])
    previous = initial
    source_native_events = []
    counts = Counter()
    witness = []
    successful_child_epochs = defaultdict(set)
    no_starter_child_epochs = defaultdict(set)
    for epoch, state in enumerate(snapshots):
        events = state["events"]
        if events[:len(previous["events"])] != previous["events"]:
            raise ValueError("tampered native cumulative event prefix")
        new = events[len(previous["events"]):]
        planned = [a for a in actions if a["epoch"] == epoch]
        if len(new) < len(planned):
            raise ValueError("truncated external action ledger")
        external = new[:len(planned)]
        native = new[len(planned):]
        if any(e["origin"] != "External" for e in external):
            raise ValueError("host actions disguised as native")
        if any(not internal_actor(e) for e in native):
            raise ValueError("native event falsely attributed")
        starter = any(
            a["action"].get("SetCredits", {}).get("credits") == 1
            for a in planned
        )
        for event in native:
            name = event_name(event)
            actor = event["process"]
            if name == "GraphLaunched":
                details = event["kind"]["GraphLaunched"]
                if details["parent_process"] != actor:
                    raise ValueError("child launch parent/source mismatch")
                child = details["child_process"]
                if child <= 3 or child in launches:
                    raise ValueError("duplicate or wrong child ID")
                launches[child] = {
                    "tick": event["tick"],
                    "parent": actor,
                    "child": child,
                    "graph": details["source_graph"],
                }
            elif actor > 3:
                if actor not in launches or event["tick"] <= launches[actor]["tick"]:
                    raise ValueError("orphan native child event")
                counts[f"native_child_{name}_events"] += 1
                if name in ("Halted", "Trapped", "GraphCreated"):
                    continue
                counts["native_child_success_non_GraphCreated_events"] += 1
                successful_child_epochs[actor].add(epoch)
                if not starter:
                    no_starter_child_epochs[actor].add(epoch)
                observed = child_opcode_at_pc(previous, state, actor, event["pc"])
                if len(witness) < 18:
                    launch = launches[actor]
                    witness.append({
                        "epoch": epoch,
                        "tick": event["tick"],
                        "child": actor,
                        "pc": event["pc"],
                        "kind": name,
                        "runtime": observed,
                        "launch": launch,
                        "same_epoch_external_actor_starter": starter,
                        "parent_prior_source_copies": source_history(
                            source_native_events, launch
                        ),
                    })
            source_native_events.append(event)
        previous = state
    if len(initial["events"]) > initial_event_count:
        raise ValueError("original initial event history unexpectedly changed")
    return {
        "native_child_launches": len(launches),
        "child_success_non_GraphCreated": counts["native_child_success_non_GraphCreated_events"] > 0,
        "child_success_non_GraphCreated_events": counts["native_child_success_non_GraphCreated_events"],
        "child_success_2_epochs": any(len(x) >= 2 for x in successful_child_epochs.values()),
        "child_success_2_epochs_without_same_epoch_starter": any(
            len(x) >= 2 for x in no_starter_child_epochs.values()
        ),
        "native_event_counts": dict(sorted(counts.items())),
        "witnesses": witness,
    }


def audit_one(path: Path, mode: str) -> dict:
    db=sqlite3.connect(f"{path.resolve().as_uri()}?mode=ro",uri=True)
    try:
        seen=defaultdict(set)
        counts=Counter()
        witness_worlds=[]
        epoch_hashes=0
        for rid, condition, rev, raw_initial, raw_actions in db.execute(
            "SELECT id,condition,source_revision,initial_json,actions_json "
            "FROM epoch_runs ORDER BY id"
        ):
            label,arm=condition.split("/",1)
            seed=int(label.removeprefix("seed="))
            if rev!=ORIGINAL_REVISION or not FIRST<=seed<FIRST+SEEDS or arm not in ARMS or arm in seen[seed]:
                raise ValueError("historical source or arm mismatch")
            seen[seed].add(arm)
            previous=json.loads(raw_initial)
            snapshots=[]
            for epoch,digest,raw in db.execute(
                "SELECT epoch,state_sha256,state_json FROM epoch_snapshots "
                "WHERE run_id=? ORDER BY epoch",(rid,)
            ):
                if epoch!=len(snapshots) or analyze.digest(raw)!=digest:
                    raise ValueError("historical hash corrupted")
                snapshots.append(json.loads(raw))
                epoch_hashes+=1
            result=classify_run(previous,snapshots,json.loads(raw_actions))
            counts[f"{arm}.worlds_non_GraphCreated_child_success"]+=int(result["child_success_non_GraphCreated"])
            counts[f"{arm}.native_child_non_GraphCreated_success_events"]+=result["child_success_non_GraphCreated_events"]
            counts[f"{arm}.worlds_native_child_success_two_epochs"]+=int(result["child_success_2_epochs"])
            counts[f"{arm}.worlds_native_child_success_no_same_epoch_starter_two_epochs"]+=int(
                result["child_success_2_epochs_without_same_epoch_starter"])
            for name,n in result["native_event_counts"].items():
                counts[f"{arm}.{name}"]+=n
            if result["child_success_non_GraphCreated"] and len(witness_worlds)<16:
                witness_worlds.append({
                    "seed":seed,"arm":arm,"mode":mode,
                    "native_non_GraphCreated_child_events":result["child_success_non_GraphCreated_events"],
                    "first_native_successes":result["witnesses"][:12],
                })
        if len(seen)!=SEEDS or any(a!=set(ARMS) for a in seen.values()) or epoch_hashes!=24576:
            raise ValueError("incomplete historical source-pinned four-arm corpus")
        return {
            "mode":mode,
            "original_seed_pairs_reused":SEEDS,
            "original_epoch_SHA256_verified":epoch_hashes,
            "counts":dict(sorted(counts.items())),
            "native_child_nonGraphCreated_witnesses":witness_worlds,
        }
    finally:
        db.close()


def audit(absolute_path: Path,local_path: Path) -> dict:
    prior=analyze_study59.paired_audit(absolute_path,local_path,ORIGINAL_REVISION)
    if (prior["independent_seed_pairs"]!=SEEDS or prior["matched_native_runs"]!=2048
            or prior["SHA256_verified_snapshots"]!=49152):
        raise ValueError("original predeclared experimental result did not replay")
    result={
        "study":"CL-1 Study60 historical native local-copy child opcode ancestry",
        "registered_after_known_nonGraphCreated_child_result":True,
        "new_independent_worlds":0,
        "source_revision":ORIGINAL_REVISION,
        "original_seed_pairs_reused":SEEDS,
        "matched_runs":2048,
        "original_SHA256_snapshots_verified":49152,
        "modes":{
            "absolute":audit_one(absolute_path,"absolute"),
            "local":audit_one(local_path,"local"),
        },
        "original_registered_Study59_primary":prior["primary_functional_native_child_CellWritten"],
        "life_claim":"NONE",
    }
    for mode,expected in (("absolute",0),("local",1)):
        observed=result["modes"][mode]["counts"].get(
            "ambient-general.worlds_non_GraphCreated_child_success",0
        )
        if observed!=expected:
            raise ValueError("known original native child non-GraphCreated outcome changed")
    return result


def main() -> None:
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument("absolute",type=Path)
    parser.add_argument("local",type=Path)
    parser.add_argument("--json",type=Path,required=True)
    args=parser.parse_args()
    report=audit(args.absolute,args.local)
    args.json.write_text(json.dumps(report,indent=2)+"\n",encoding="utf-8")
    print(json.dumps({
        "study":report["study"],
        "absolute":report["modes"]["absolute"]["counts"],
        "local":report["modes"]["local"]["counts"],
        "first_local_witness":report["modes"]["local"]["native_child_nonGraphCreated_witnesses"][:1],
        "life_claim":report["life_claim"],
    },indent=2))


if __name__=="__main__":
    main()
