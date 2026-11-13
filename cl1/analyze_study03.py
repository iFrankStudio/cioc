#!/usr/bin/env python3
"""CL-1 Study 03: independent read-only edge-restoration provenance analysis."""
from __future__ import annotations

import argparse
import json
import sqlite3
from collections import Counter
from pathlib import Path

import analyze
from analyze_study02 import wilson_95

FIRST = 830000
SEEDS = 128
LABELS = ("sham", "lesion-only", "external-restore", "disable-p1", "zero-credit")
OFFSETS = (4, 24, 48, 72)
HORIZON = 96


def node_next(world: dict, process_id: int = 1, node_index: int = 8) -> int | None:
    nodes = world["processes"][str(process_id)]["code"]["nodes"]
    op = nodes[node_index]
    if isinstance(op, dict) and len(op) == 1:
        fields = next(iter(op.values()))
        return fields.get("next") if isinstance(fields, dict) else None
    return None


def active_after_lesion(world: dict) -> bool:
    observed_lesion = False
    for event in world["events"]:
        kind = event["kind"]
        if (
            event["origin"] == "External"
            and isinstance(kind, dict)
            and "Intervened" in kind
            and "PatchEdge" in kind["Intervened"]
            and "node: 8" in kind["Intervened"]
            and "next: 0" in kind["Intervened"]
        ):
            observed_lesion = True
        elif observed_lesion and analyze.internal_process(event) == 1:
            return True
    return False


def valid_control_actions(condition: str, actions: list[dict]) -> bool:
    if condition == "zero-credit":
        return len(actions) == 3 and all(
            a["after_attempts"] == 0 and "SetCredits" in a["action"]
            for a in actions
        )
    if len(actions) != 4 or [a["after_attempts"] for a in actions] != list(OFFSETS):
        return False
    if condition == "lesion-only":
        return "PatchEdge" in actions[0]["action"] and all(
            "Disable" in x["action"] for x in actions[1:]
        )
    if condition == "external-restore":
        return (
            "PatchEdge" in actions[0]["action"]
            and "PatchEdge" in actions[1]["action"]
            and all("Disable" in x["action"] for x in actions[2:])
        )
    if condition == "disable-p1":
        return "Disable" in actions[0]["action"]
    return condition == "sham" and all("Disable" in x["action"] for x in actions)


def analyze_db(path: Path) -> dict:
    conn = sqlite3.connect(f"{path.resolve().as_uri()}?mode=ro", uri=True)
    try:
        rows = conn.execute(
            "SELECT id,source_revision,condition,initial_sha256,initial_json,"
            "actions_json,attempt_limit,attempts,final_sha256,final_json "
            "FROM scheduled_runs ORDER BY id"
        ).fetchall()
        if len(rows) != SEEDS * len(LABELS):
            raise ValueError(f"expected 640 conditions, received {len(rows)}")

        by_id: dict[int, dict[int, dict]] = {}
        for run_id, ordinal, offset, sha, snapshot in conn.execute(
            "SELECT run_id,ordinal,after_attempts,state_sha256,state_json "
            "FROM scheduled_checkpoints ORDER BY run_id,ordinal"
        ):
            if analyze.digest(snapshot) != sha or ordinal < 0 or offset < 0:
                raise ValueError(f"invalid pre-action snapshot for run {run_id}")
            if run_id not in by_id:
                by_id[run_id] = {}
            if ordinal in by_id[run_id]:
                raise ValueError(f"duplicate checkpoint ordinal for run {run_id}")
            by_id[run_id][ordinal] = {
                "offset": offset,
                "world": json.loads(snapshot),
            }

        grouped: dict[int, dict[str, dict]] = {}
        hashes = {}
        revision_set = set()
        for (run_id, revision, label, initial_hash, initial_json,
             actions_json, attempt_limit, attempts, final_hash, final_json) in rows:
            if not label.startswith("seed=") or "/" not in label:
                raise ValueError(f"bad label: {label}")
            prefix, condition = label.split("/", 1)
            seed = int(prefix[5:])
            if seed not in range(FIRST, FIRST + SEEDS) or condition not in LABELS:
                raise ValueError(f"unregistered seed or condition: {label}")
            if condition in grouped.get(seed, {}):
                raise ValueError(f"duplicate seed and condition: {label}")
            if (
                not revision or analyze.digest(initial_json) != initial_hash
                or analyze.digest(final_json) != final_hash
                or attempt_limit != HORIZON or not 0 <= attempts <= HORIZON
            ):
                raise ValueError(f"hash or horizon error: {label}")
            if seed in hashes and hashes[seed] != initial_hash:
                raise ValueError(f"matched initial worlds differ for seed {seed}")
            hashes[seed] = initial_hash
            revision_set.add(revision)
            initial = json.loads(initial_json)
            final = json.loads(final_json)
            if (
                node_next(initial) != 9
                or analyze.occupied_units(initial) != 51
                or not initial["strict_accounting"]
                or initial["total_storage_limit"] != 160
                or analyze.occupied_units(final) > 160
            ):
                raise ValueError(f"resource or lesion target invalid: {label}")
            actions = json.loads(actions_json)
            if not valid_control_actions(condition, actions):
                raise ValueError(f"wrong intervention protocol: {label}")
            snapshots = by_id.get(run_id, {})
            if len(snapshots) > len(actions):
                raise ValueError(f"extra pre-intervention snapshots: {label}")
            for ordinal, snap in snapshots.items():
                if ordinal >= len(actions) or snap["offset"] != actions[ordinal]["after_attempts"]:
                    raise ValueError(f"unexpected snapshot offset: {label}")
            if condition == "zero-credit" and (
                attempts != 0
                or any(analyze.internal_process(e) is not None for e in final["events"])
            ):
                raise ValueError(f"zero-credit condition violated: {label}")
            grouped.setdefault(seed, {})[condition] = {
                "snapshots": snapshots,
                "world": final,
                "attempts": attempts,
            }

        if len(grouped) != SEEDS or len(revision_set) != 1:
            raise ValueError("seed count or source revision mismatch")
        if any(set(values) != set(LABELS) for values in grouped.values()):
            raise ValueError("incomplete five-condition matched trial")

        count = Counter()
        for seed in sorted(grouped):
            cases = grouped[seed]
            lesion = cases["lesion-only"]
            checkpoints = lesion["snapshots"]
            exposed = 0 in checkpoints
            count["lesion_exposed"] += exposed
            count["checkpoint_24_reached"] += 1 in checkpoints
            count["checkpoint_48_reached"] += 2 in checkpoints
            if exposed:
                by_24 = 1 in checkpoints and node_next(checkpoints[1]["world"]) == 9
                by_48 = 2 in checkpoints and node_next(checkpoints[2]["world"]) == 9
                by_final = node_next(lesion["world"]) == 9
                count["restored_by_24"] += by_24
                count["restored_by_final"] += by_final
                count["persistent_restoration_proxy"] += bool(by_24 and by_48 and by_final)
                count["active_after_damage"] += active_after_lesion(lesion["world"])
            control = cases["external-restore"]
            count["external_restore_reached"] += 1 in control["snapshots"]
            disable = cases["disable-p1"]
            disable_events = disable["world"]["events"]
            stopped = False
            incorrectly_active = False
            for event in disable_events:
                if (
                    event["origin"] == "External"
                    and isinstance(event["kind"], dict)
                    and "Intervened" in event["kind"]
                    and "Disable" in event["kind"]["Intervened"]
                    and "process: 1" in event["kind"]["Intervened"]
                ):
                    stopped = True
                elif stopped and analyze.internal_process(event) == 1:
                    incorrectly_active = True
            if incorrectly_active:
                raise ValueError(f"disabled process resumed CIL execution: seed={seed}")
            count["disable_reached"] += stopped
        fractions = {}
        for name in ("restored_by_24", "restored_by_final", "persistent_restoration_proxy"):
            numerator = count[name]
            lo, hi = wilson_95(numerator, SEEDS)
            fractions[name] = {
                "seeds": numerator,
                "denominator": SEEDS,
                "wilson_95": [round(lo, 6), round(hi, 6)],
            }
        return {
            "study": "CL-1 Study 03",
            "status": "prospective code-edge restoration audit (not functional maintenance)",
            "source_revision": next(iter(revision_set)),
            "independent_worlds": SEEDS,
            "matched_runs": len(rows),
            "all_hashes_and_replays": "hashes independently verified, full replay verified by Rust runner",
            "counts": dict(sorted(count.items())),
            "restoration_proxies": fractions,
            "autonomous_functional_maintenance": "NOT PROVEN",
            "organizational_closure": "NOT PROVEN",
            "life_claim": "NONE",
            "caveat": "A returned edge may be incidental; external restoration controls are not endogenous evidence.",
        }
    finally:
        conn.close()


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("sqlite", type=Path)
    parser.add_argument("--json", type=Path, required=True)
    args = parser.parse_args()
    report = analyze_db(args.sqlite)
    args.json.write_text(
        json.dumps(report, indent=2, ensure_ascii=False) + "\n", encoding="utf-8"
    )
    print(json.dumps({
        "study": report["study"],
        "worlds": report["independent_worlds"],
        "runs": report["matched_runs"],
        "counts": report["counts"],
    }))


if __name__ == "__main__":
    main()
