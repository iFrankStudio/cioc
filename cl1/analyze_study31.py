#!/usr/bin/env python3
"""Study 31: post-result, exact native CIL register def-use lineage audit.

Read-only use of the frozen Study-27 world corpus, never a new experiment.
"""
from __future__ import annotations

import argparse
import json
import sqlite3
from collections import Counter
from pathlib import Path

import analyze
import analyze_study27


# Explicit opcode-to-written-register mapping. All other opcodes are either
# non-writing or invalidate the witness if their effects cannot be classified.
DST_FIELD = {
    "Const": "dst",
    "Read": "dst",
    "ReadNext": "dst",
    "CloneNode": "dst",
    "GraphNew": "dst",
    "GraphCopyNode": "dst",
    "GraphCopyFrom": "node",
    "GraphReadNext": "dst",
    "GraphReadField": "field",
    "GraphLaunch": "dst",
    "GraphEncode": "dst",
}
NO_REGISTER_WRITE = {
    "Write", "Branch", "PatchNext", "PatchConst", "Delete",
    "GraphSetConst", "GraphSetNext", "GraphGrantRead",
    "GraphRevokeRead", "GraphDrop", "GraphTransfer", "GraphReplace",
    "DrawCompute", "ReturnCompute", "TransferCompute",
}
SUCCESS_KINDS = {
    "Executed", "CellRead", "CellWritten",
    "GraphCreated", "GraphNodeCopied", "GraphCopied", "GraphNodeEdited",
    "GraphNodeRewired", "GraphOwnerTransferred", "GraphReadTag",
    "GraphTagRead", "GraphFieldRead", "GraphLaunched",
    "GraphEncoded", "GraphReplaced", "GraphDropped", "GraphReaderGranted",
    "GraphReaderRevoked", "GraphReadNext",
}


def trace_native_address(events: list[dict], program: list, tag: int,
                         address: int) -> dict:
    """Conservatively follow graph2 source label into native Write.address.

    A successful native opcode may clear or propagate register labels. Native
    graphs and process code are distinct: graph edits do not change this
    prefork program. Unknown register semantics fail closed.
    """
    labels: dict[int, frozenset[str]] = {}
    tagged_outputs: list[dict] = []
    reads = 0
    last_tick = -1
    for event in events:
        if analyze.internal_process(event) != 1:
            continue
        if event.get("process") != 1:
            raise ValueError("internal origin and process mismatch")
        tick = event["tick"]
        if not isinstance(tick, int) or tick < last_tick:
            raise ValueError("native process event ticks invalid")
        last_tick = tick
        pc = event["pc"]
        if not isinstance(pc, int) or pc < 0 or pc >= len(program):
            raise ValueError("executed PC was not in prefork source code")
        node = program[pc]
        if node == "Halt":
            continue
        if not isinstance(node, dict) or len(node) != 1:
            raise ValueError("invalid prefork opcode descriptor")
        opname, params = next(iter(node.items()))
        kind = event["kind"]
        if isinstance(kind, dict) and "Trapped" in kind:
            continue
        if kind == "Halted":
            continue
        if not (kind == "Executed" or (isinstance(kind, dict)
                                       and len(kind) == 1)):
            raise ValueError("unknown event kind")
        if opname == "GraphReadTag":
            if not isinstance(kind, dict) or "GraphTagRead" not in kind:
                raise ValueError("successful graph tag read has no success provenance")
            source = kind["GraphTagRead"]
            if (source.get("graph") == 2 and source.get("owner") == 2
                    and source.get("node") == 0 and source.get("tag") == tag):
                labels[params["dst"]] = frozenset(("graph2-tag",))
                reads += 1
            else:
                labels[params["dst"]] = frozenset()
        elif opname in ("Add", "Sub"):
            if kind != "Executed":
                raise ValueError("native arithmetic without executed-success event")
            # Capture source labels BEFORE overwriting destination.
            labels[params["dst"]] = labels.get(params["lhs"], frozenset()) | labels.get(
                params["rhs"], frozenset()
            )
        elif opname == "Write":
            if not isinstance(kind, dict) or "CellWritten" not in kind:
                raise ValueError("native write lacks successful CellWritten")
            written = kind["CellWritten"]
            if (written.get("key") == address
                    and "graph2-tag" in labels.get(params["address"], frozenset())):
                tagged_outputs.append({
                    "tick": tick,
                    "pc": pc,
                    "address_register": params["address"],
                    "cell": written["key"],
                    "value": written["value"],
                })
        elif opname in DST_FIELD:
            # Explicitly clear old graph2 label, including GraphCopyFrom.node.
            index = params[DST_FIELD[opname]]
            if not isinstance(index, int) or not 0 <= index < 4:
                raise ValueError("unexpected destination register")
            labels[index] = frozenset()
        elif opname not in NO_REGISTER_WRITE:
            raise ValueError(f"unknown successful register semantics: {opname}")
    return {
        "tag_read_count": reads,
        "qualified_writes": tagged_outputs,
        "qualified_unique": len(tagged_outputs) == 1,
    }


def audit(path: Path) -> dict:
    prior = analyze_study27.audit(path)
    if (prior["primary_address_relocation"]["seed_worlds"] != 2
            or prior["independent_seed_worlds"] != 192
            or prior["matched_runs"] != 1344
            or prior["hashed_epoch_snapshots"] != 10752):
        raise ValueError("not the frozen verified Study27 cohort")
    db = sqlite3.connect(f"{path.resolve().as_uri()}?mode=ro", uri=True)
    try:
        runs = {}
        for run_id, condition, initial_raw in db.execute(
            "SELECT id,condition,initial_json FROM epoch_runs"
        ):
            initial = json.loads(initial_raw)
            runs[run_id] = {
                "condition": condition,
                "program": initial["processes"]["1"]["code"]["nodes"],
                "prior_event_count": len(initial["events"]),
            }
        observations = {}
        for run_id, raw in db.execute(
            "SELECT run_id,state_json FROM epoch_snapshots WHERE epoch=0"
        ):
            run = runs[run_id]
            branch = run["condition"]
            if branch.endswith("/control") or branch.endswith("/mutate-g2"):
                events = json.loads(raw)["events"][run["prior_event_count"]:]
                mutated = branch.endswith("/mutate-g2")
                observations[branch] = trace_native_address(
                    events, run["program"], 19 if mutated else 11,
                    19 if mutated else 11,
                )
        if len(observations) != 384:
            raise ValueError("missing source-control matched epoch0 events")
        counts = Counter()
        strict = []
        known = {w["seed"] for w in prior["strict_matched_witnesses"]}
        for seed in range(2_300_000, 2_300_192):
            plain = observations[f"seed={seed}/control"]
            altered = observations[f"seed={seed}/mutate-g2"]
            counts["control_graph2_native_read"] += plain["tag_read_count"] > 0
            counts["mutated_graph2_native_read"] += altered["tag_read_count"] > 0
            counts["control_register_qualified_output"] += plain["qualified_unique"]
            counts["mutant_register_qualified_output"] += altered["qualified_unique"]
            qualifies = plain["qualified_unique"] and altered["qualified_unique"]
            if qualifies:
                old, new = plain["qualified_writes"][0], altered["qualified_writes"][0]
                qualifies = old["value"] == new["value"]
            counts["paired_register_lineage_function"] += qualifies
            if qualifies:
                strict.append({
                    "seed": seed,
                    "control": plain["qualified_writes"][0],
                    "mutation": altered["qualified_writes"][0],
                })
        for witness in strict:
            if witness["seed"] not in known:
                raise ValueError("unexpected register-lineage witness outside Study27 outcome")
        return {
            "study": "CL-1 Study 31",
            "source_revision": prior["source_revision"],
            "kind": "post-result native register def-use proof, not new independent ecology",
            "new_independent_worlds": 0,
            "reused_independent_seed_worlds": 192,
            "matched_runs_replayed": 1344,
            "hashed_snapshots_replayed": 10752,
            "known_original_paired_source_address_worlds": len(known),
            "register_lineage_counts": dict(sorted(counts.items())),
            "register_lineage_matched_witnesses": strict,
            "previously_known_but_unverified_by_register_trace": sorted(
                known - {w["seed"] for w in strict}
            ),
            "source_sensitive_register_address_lineage": (
                "CONFIRMED POST-RESULT" if len(strict) else "NOT DEMONSTRATED"
            ),
            "native_multicomponent_function": "NOT EVALUABLE",
            "autonomous_functional_maintenance": "NOT EVALUABLE",
            "life_claim": "NONE",
        }
    finally:
        db.close()


def main() -> None:
    p = argparse.ArgumentParser(description=__doc__)
    p.add_argument("sqlite", type=Path)
    p.add_argument("--json", type=Path, required=True)
    args = p.parse_args()
    report = audit(args.sqlite)
    args.json.write_text(json.dumps(report, indent=2) + "\n", encoding="utf-8")
    print(json.dumps({
        "study": report["study"],
        "known": report["known_original_paired_source_address_worlds"],
        "register_lineage": report["register_lineage_counts"].get(
            "paired_register_lineage_function", 0
        ),
        "new_worlds": report["new_independent_worlds"],
        "life_claim": report["life_claim"],
    }))


if __name__ == "__main__":
    main()
