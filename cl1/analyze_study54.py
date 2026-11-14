#!/usr/bin/env python3
"""Study54 post-result copy-source Write opportunity in original Study53 worlds.

Source opcode names below refer to FROZEN INITIAL parent code only. They are
not certified runtime copied code after unknown in-epoch native mutations.
"""
from __future__ import annotations

import argparse
import json
import sqlite3
from collections import Counter, defaultdict
from pathlib import Path

import analyze
import analyze_study53
from analyze_study52 import runtime_source_uncertainty

FIRST = 3_700_000
SEEDS = 128
ARMS = ("ambient-general", "ambient-only", "closed-general",
        "zero-credit-ambient-general")


def event_kind(event: dict) -> str:
    k = event["kind"]
    return next(iter(k)) if isinstance(k, dict) else k


def classify_source_copies(initial: dict, final: dict, snapshots: list[dict] | None = None) -> list[dict]:
    """Validate actual resolved source-node events, not register operands."""
    events = final["events"][len(initial["events"]):]
    copies = []
    for event in events:
        if event_kind(event) != "GraphNodeCopied":
            continue
        actor, k = event["process"], event["kind"]["GraphNodeCopied"]
        if (analyze.internal_process(event) != actor or k["owner"] != actor
                or k["source_process"] != actor):
            raise ValueError("unattributed native source graph copy event")
        # Child-created copies are not part of frozen original-program ancestry.
        if actor not in (1, 2, 3):
            continue
        program = initial["processes"][str(actor)]["code"]["nodes"]
        source = k["source_node"]
        if not isinstance(source, int) or not 0 <= source < len(program):
            raise ValueError("resolved native parent source node out of bounds")
        node = program[source]
        if node == "Halt":
            name = "Halt"
        elif isinstance(node, dict) and len(node) == 1:
            name = next(iter(node))
        else:
            raise ValueError("malformed original CIL source")
        if source == 0:
            continue
        if name == "Write" and snapshots is None:
            exact_runtime = "NOT EVALUABLE: no in-epoch source snapshots"
        elif name == "Write" and runtime_source_uncertainty(
            initial, snapshots, event["tick"], actor
        ):
            exact_runtime = "NOT EVALUABLE: possible native original-code edit"
        else:
            exact_runtime = name
        copies.append({
            "parent": actor, "graph": k["graph"],
            "source_node": source, "initial_source_opcode": name,
            "copy_tick": event["tick"],
            "runtime_source_opcode": exact_runtime,
        })
    return copies


def stages(initial: dict, final: dict, snapshots: list[dict] | None = None) -> dict:
    """Source copy→same-parent graph edit→same-parent graph launch causal gates."""
    native = final["events"][len(initial["events"]):]
    copies = classify_source_copies(initial, final, snapshots)
    edits = []
    launches = []
    for event in native:
        kind = event_kind(event)
        if kind not in ("GraphNodeEdited", "GraphLaunched"):
            continue
        if analyze.internal_process(event) != event["process"]:
            raise ValueError("externally fabricated graph operation")
        fields = event["kind"][kind]
        if kind == "GraphNodeEdited":
            if fields["owner"] != event["process"]:
                raise ValueError("native graph edit owner mismatch")
            edits.append({
                "parent": event["process"], "graph": fields["graph"],
                "tick": event["tick"],
            })
        else:
            if fields["parent_process"] != event["process"]:
                raise ValueError("native graph child launch parent mismatch")
            launches.append({
                "parent": event["process"], "graph": fields["source_graph"],
                "tick": event["tick"], "child": fields["child_process"],
            })
    write_copies = [c for c in copies if c["initial_source_opcode"] == "Write"]
    linked = []
    for c in write_copies:
        later = [
            l for l in launches
            if l["parent"] == c["parent"] and l["graph"] == c["graph"]
            and l["tick"] > c["copy_tick"]
        ]
        for l in later:
            edit_after = any(
                e["parent"] == c["parent"] and e["graph"] == c["graph"]
                and c["copy_tick"] < e["tick"] < l["tick"]
                for e in edits
            )
            linked.append({
                **c, "launch_tick": l["tick"], "child": l["child"],
                "edit_between_copy_and_launch": edit_after,
            })
    return {
        "copies": copies,
        "write_copies": write_copies,
        "write_copy_before_launch": bool(linked),
        "write_copy_edit_launch": any(x["edit_between_copy_and_launch"] for x in linked),
        "linked_witnesses": linked,
        "native_launches": launches,
        "native_edits": edits,
    }


def audit(original_alias: Path, original_separate: Path, source_rev: str) -> dict:
    prior = analyze_study53.audit(original_alias, original_separate, source_rev)
    if (prior["independent_seed_pairs"] != SEEDS
            or prior["SHA256_verified_original_epochs"] != 24576):
        raise ValueError("Study53 original full audit was not reproducible")
    modes = {}
    for mode, path in (("alias", original_alias), ("separate", original_separate)):
        db = sqlite3.connect(f"{path.resolve().as_uri()}?mode=ro", uri=True)
        try:
            counts = Counter()
            witnesses = []
            seen = defaultdict(set)
            for rid, label, rev, raw_initial, raw_final in db.execute(
                "SELECT id,condition,source_revision,initial_json,final_json "
                "FROM epoch_runs ORDER BY id"
            ):
                seed_str, arm = label.split("/", 1)
                seed = int(seed_str.removeprefix("seed="))
                if (rev != source_rev or not FIRST <= seed < FIRST + SEEDS
                        or arm not in ARMS or arm in seen[seed]):
                    raise ValueError("wrong historical seed pair or arm")
                seen[seed].add(arm)
                initial, final = json.loads(raw_initial), json.loads(raw_final)
                # Only load epoch JSON if a frozen Write-copy opcode is encountered.
                first_pass = stages(initial, final)
                if first_pass["write_copies"]:
                    snapshots = [
                        json.loads(s)
                        for _, s in db.execute(
                            "SELECT epoch,state_json FROM epoch_snapshots "
                            "WHERE run_id=? ORDER BY epoch", (rid,)
                        )
                    ]
                    if len(snapshots) != 24:
                        raise ValueError("missing exact initial-source history")
                    observed = stages(initial, final, snapshots)
                else:
                    observed = first_pass
                copies = observed["copies"]
                for c in copies:
                    counts[f"{arm}.source_op.{c['initial_source_opcode']}"] += 1
                for name, val in {
                    "worlds_any_original_nonzero_source_copy": bool(copies),
                    "worlds_original_Write_source_copied": bool(observed["write_copies"]),
                    "worlds_original_Write_copy_before_launch": observed["write_copy_before_launch"],
                    "worlds_original_Write_copy_edit_launch": observed["write_copy_edit_launch"],
                    "worlds_native_graph_launch": bool(observed["native_launches"]),
                    "worlds_native_graph_edit": bool(observed["native_edits"]),
                }.items():
                    counts[f"{arm}.{name}"] += int(val)
                counts[f"{arm}.native_original_nonzero_copy_events"] += len(copies)
                counts[f"{arm}.native_original_Write_copy_events"] += len(observed["write_copies"])
                counts[f"{arm}.write_copy_before_launch_events"] += len(observed["linked_witnesses"])
                if (observed["write_copies"] or observed["linked_witnesses"]) and len(witnesses) < 20:
                    witnesses.append({
                        "seed": seed, "arm": arm,
                        "Write_copies": observed["write_copies"][:4],
                        "linked": observed["linked_witnesses"][:4],
                    })
            if (len(seen) != SEEDS
                    or any(arms != set(ARMS) for arms in seen.values())):
                raise ValueError("missing original four-arm historical cohort")
            modes[mode] = {
                "source_revision": source_rev,
                "original_independent_seed_pairs_reused": SEEDS,
                "matched_runs": SEEDS * len(ARMS),
                "counts": dict(sorted(counts.items())),
                "native_Write_source_copy_witnesses": witnesses,
            }
        finally:
            db.close()
    return {
        "study": "CL-1 Study54 retrospective native parent Write-source copy",
        "new_independent_worlds": 0,
        "original_independent_seed_pairs_reused": SEEDS,
        "source_revision": source_rev,
        "original_hashed_epochs_reverified_by_Study53": 24576,
        "modes": modes,
        "life_claim": "NONE",
    }


def main() -> None:
    p = argparse.ArgumentParser(description=__doc__)
    p.add_argument("alias", type=Path)
    p.add_argument("separate", type=Path)
    p.add_argument("--expected-revision", required=True)
    p.add_argument("--json", type=Path, required=True)
    args = p.parse_args()
    result = audit(args.alias, args.separate, args.expected_revision)
    args.json.write_text(json.dumps(result, indent=2) + "\n", encoding="utf-8")
    print(json.dumps({
        "study": result["study"],
        "alias": {k:v for k,v in result["modes"]["alias"]["counts"].items()
                  if k.startswith("ambient-general.")},
        "separate": {k:v for k,v in result["modes"]["separate"]["counts"].items()
                     if k.startswith("ambient-general.")},
        "life_claim": "NONE",
    }, indent=2))


if __name__ == "__main__":
    main()
