#!/usr/bin/env python3
"""Study 25: independent source-content-perturbation audit in fresh random CIL worlds.

Successful code read + different same-actor later cell write is only externally
induced sensitivity, NEVER internal design autonomy or regeneration.
"""
from __future__ import annotations

import argparse
import copy
import json
import sqlite3
from collections import Counter, defaultdict
from pathlib import Path

import analyze
from analyze_study02 import wilson_95
from analyze_study14 import epoch_graph_events, reciprocal_pairs, simulated_state
from analyze_study24 import epoch_exposure

FIRST = 2_250_000
SEEDS = 96
EPOCHS = 24
QUOTA = 12
WINDOWS = ((0, 8), (8, 16), (16, 24))
LABELS = (
    "control", "sham", "mutate-g1", "mutate-g2", "mutate-both",
    "exact-original-g1", "refill", "mutate-g1-refill", "zero-credit",
)


def schedule(label: str, initial: dict) -> list[dict]:
    def at(epoch: int, operation: dict) -> dict:
        return {"epoch": epoch, "action": operation}

    def mutate(graphs: tuple[int, ...], identical: bool = False) -> list[dict]:
        events = []
        for gid in graphs:
            obj = copy.deepcopy(initial["graph_objects"][str(gid)])
            if not identical:
                obj["nodes"][0] = "Halt"
            events.extend([
                at(0, {"DropGraph": {"graph": gid}}),
                at(0, {"PutGraph": {"graph": gid, "object": obj}}),
            ])
        return events

    def refill() -> list[dict]:
        return [
            at(epoch, {"SetCredits": {"process": actor, "credits": 47}})
            for epoch in (8, 16) for actor in (1, 2, 3)
        ]

    actions = {
        "control": [],
        "sham": [at(0, {"DropGraph": {"graph": 0}})],
        "mutate-g1": mutate((1,)),
        "mutate-g2": mutate((2,)),
        "mutate-both": mutate((1, 2)),
        "exact-original-g1": mutate((1,), identical=True),
        "refill": refill(),
        "mutate-g1-refill": mutate((1,)) + refill(),
        "zero-credit": [
            at(0, {"SetCredits": {"process": actor, "credits": 0}})
            for actor in (1, 2, 3)
        ],
    }
    return actions[label]


def _targeted_first_outputs(events: list[dict], graph: int) -> tuple[dict, set]:
    """Per (actor, cell) first output after first successful targeted tag read.

    Keyed by same actor+cell in same epoch; multiple outputs at identical key are
    not used for a positive sensitivity claim.
    """
    seen_reads: dict[int, tuple[int, int]] = {}
    outputs: dict[tuple[int, int], list[tuple[int, int]]] = defaultdict(list)
    tags: dict[tuple[int, int], int] = {}
    for event in events:
        kind = event["kind"]
        if not isinstance(kind, dict):
            continue
        actor = analyze.internal_process(event)
        if actor is None or actor != event["process"]:
            continue
        read = kind.get("GraphTagRead")
        if read is not None and int(read["graph"]) == graph and int(read["owner"]) != actor:
            seen_reads[actor] = (int(event["tick"]), int(read["tag"]))
        write = kind.get("CellWritten")
        if write is not None and actor in seen_reads:
            read_tick, tag = seen_reads[actor]
            if int(event["tick"]) > read_tick:
                key = actor, int(write["key"])
                outputs[key].append((int(event["tick"]), int(write["value"])))
                tags[key] = tag
    qualified = {
        key: {"value": values[0][1], "tag": tags[key]}
        for key, values in outputs.items()
        if len(values) == 1
    }
    ambiguous = {key for key, values in outputs.items() if len(values) != 1}
    return qualified, ambiguous


def audit(path: Path) -> dict:
    db = sqlite3.connect(f"{path.resolve().as_uri()}?mode=ro", uri=True)
    try:
        rows = db.execute(
            "SELECT id,source_revision,condition,initial_sha256,initial_json,"
            "actions_json,epoch_count,quota,attempts_json,final_sha256,final_json "
            "FROM epoch_runs ORDER BY id"
        ).fetchall()
        if len(rows) != SEEDS * len(LABELS):
            raise ValueError(f"missing preregistered paired Study25 arms: {len(rows)}")
        snapshots: dict[int, list[tuple[int, str, str]]] = defaultdict(list)
        for run_id, epoch, sha, raw in db.execute(
            "SELECT run_id,epoch,state_sha256,state_json "
            "FROM epoch_snapshots ORDER BY run_id,epoch"
        ):
            if not 0 <= epoch < EPOCHS or analyze.digest(raw) != sha:
                raise ValueError("invalid epoch digest/index")
            snapshots[run_id].append((epoch, sha, raw))
        seeds: dict[int, dict] = defaultdict(dict)
        initial_hashes: dict[int, str] = {}
        revisions: set[str] = set()
        counts = Counter()
        source_pairs = Counter()
        for (run_id, revision, condition, initial_sha, initial_raw, actions_raw,
             epochs, quota, attempts_raw, final_sha, final_raw) in rows:
            prefix, arm = condition.split("/", 1)
            seed = int(prefix.removeprefix("seed="))
            if seed not in range(FIRST, FIRST + SEEDS) or arm not in LABELS:
                raise ValueError("unregistered seed/arm")
            if arm in seeds[seed]:
                raise ValueError("duplicate arm")
            if not revision or epochs != EPOCHS or quota != QUOTA:
                raise ValueError("source/epoch/quota mismatch")
            if (analyze.digest(initial_raw) != initial_sha
                    or analyze.digest(final_raw) != final_sha):
                raise ValueError("initial or final digest mismatch")
            if seed in initial_hashes and initial_hashes[seed] != initial_sha:
                raise ValueError("different initial state for paired branches")
            initial_hashes[seed] = initial_sha
            revisions.add(revision)
            initial, final = json.loads(initial_raw), json.loads(final_raw)
            if (initial["total_storage_limit"] != 192
                    or initial["max_nodes_per_process"] != 40
                    or initial["storage_limit"] != 32
                    or not initial["strict_accounting"]):
                raise ValueError("world capacities differ from registration")
            for gid in (1, 2, 3):
                graph = initial["graph_objects"].get(str(gid))
                if (graph is None or graph["owner"] != gid or len(graph["nodes"]) != 1
                        or initial["processes"][str(gid)]["credits"] != 47):
                    raise ValueError("invalid prefork graph/credit opportunity")
            if json.loads(actions_raw) != schedule(arm, initial):
                raise ValueError(f"modified or mismatched source perturbation schedule: {condition}")
            attempts = json.loads(attempts_raw)
            if len(attempts) != EPOCHS or any(
                not isinstance(n, int) or not 0 <= n <= QUOTA for n in attempts
            ):
                raise ValueError("invalid execution attempt schedule")
            frames = snapshots[run_id]
            if len(frames) != EPOCHS or any(e != pos for pos, (e, _, _) in enumerate(frames)):
                raise ValueError("missing or duplicated epoch snapshot")
            if frames[-1][1:] != (final_sha, final_raw):
                raise ValueError("last epoch differs from final state")
            previous = initial
            epoch_events = []
            for epoch, sha, raw in frames:
                state = json.loads(raw)
                if state["events"][:len(previous["events"])] != previous["events"]:
                    raise ValueError("event ancestry broken between epochs")
                if analyze.occupied_units(state) > 192:
                    raise ValueError("strict storage exceeded")
                epoch_events.append(state["events"][len(previous["events"]):])
                previous = state
            postfork = [e for events in epoch_events for e in events]
            if arm == "zero-credit" and (any(attempts) or any(
                analyze.internal_process(e) is not None for e in postfork
            )):
                raise ValueError("zero-credit negative control executed native VM code")
            expected_modified = 2 if arm == "mutate-both" else int(
                arm in ("mutate-g1", "mutate-g2", "exact-original-g1", "mutate-g1-refill")
            )
            for name in ("DropGraph", "PutGraph"):
                n = sum(
                    e["origin"] == "External" and isinstance(e["kind"], dict)
                    and name in str(e["kind"].get("Intervened", ""))
                    for e in postfork
                )
                if n != expected_modified:
                    raise ValueError("external code intervention provenance mismatch")
            credits = sum(
                e["origin"] == "External" and isinstance(e["kind"], dict)
                and "SetCredits" in str(e["kind"].get("Intervened", ""))
                for e in postfork
            )
            if credits != (3 if arm == "zero-credit" else
                           6 if arm in ("refill", "mutate-g1-refill") else 0):
                raise ValueError("external credits not as registered")
            if arm == "sham" and not any(
                e["origin"] == "External" and isinstance(e["kind"], dict)
                and "InterventionRejected" in e["kind"] for e in postfork
            ):
                raise ValueError("sham did not reject")
            observations = []
            for start, stop in WINDOWS:
                group = epoch_events[start:stop]
                joined = [e for evs in group for e in evs]
                read_counts = Counter()
                write_counts = 0
                samples = []
                for ei, evs in enumerate(group):
                    measured = epoch_exposure(evs)
                    read_counts.update(measured["counts"])
                    write_counts += measured["counts"]["cell_writes"]
                    for (owner, reader), n in measured["pairs"].items():
                        source_pairs[(arm, start // 8 + 1, owner, reader)] += n
                    samples.append({
                        "epoch": start + ei,
                        "g1": _targeted_first_outputs(evs, 1),
                        "g2": _targeted_first_outputs(evs, 2),
                    })
                observations.append({
                    "active": sum(attempts[start:stop]) > 0,
                    "foreign_read": read_counts["foreign_graph_reads"] > 0,
                    "foreign_reads_total": read_counts["foreign_graph_reads"],
                    "native_write": write_counts > 0,
                    "native_writes_total": write_counts,
                    "read_then_write": read_counts["same_actor_read_then_write"] > 0,
                    "read_then_write_total": read_counts["same_actor_read_then_write"],
                    "cross_owner_copy": any(
                        isinstance(e["kind"], dict)
                        and "GraphCopied" in e["kind"]
                        and analyze.internal_process(e) == e["kind"]["GraphCopied"]["target_owner"]
                        and e["kind"]["GraphCopied"]["source_owner"]
                            != e["kind"]["GraphCopied"]["target_owner"]
                        for e in joined
                    ),
                    "samples": samples,
                })
            seeds[seed][arm] = {
                "final": final, "window": observations,
            }
        if (len(seeds) != SEEDS or len(revisions) != 1
                or any(set(arms) != set(LABELS) for arms in seeds.values())):
            raise ValueError("unbalanced independent source-pinned seed cohort")
        paired = Counter()
        examples = []
        for seed, arms in sorted(seeds.items()):
            if simulated_state(arms["control"]["final"]) != simulated_state(arms["sham"]["final"]):
                raise ValueError("sham affected native simulation state")
            if simulated_state(arms["control"]["final"]) != simulated_state(
                arms["exact-original-g1"]["final"]
            ):
                raise ValueError("byte-identical source content reinjection altered simulated state")
            for arm in LABELS:
                for wi, observation in enumerate(arms[arm]["window"], 1):
                    for key in ("active", "foreign_read", "native_write",
                                "read_then_write", "cross_owner_copy"):
                        counts[f"{arm}.W{wi}.{key}"] += bool(observation[key])
                    for key in ("foreign_reads_total", "native_writes_total",
                                "read_then_write_total"):
                        counts[f"raw.{arm}.W{wi}.{key}"] += observation[key]
            for altered, target in (("mutate-g1", 1), ("mutate-g2", 2)):
                observed_any = False
                changed_any = False
                loss_any = False
                for wi in range(3):
                    base = arms["control"]["window"][wi]["samples"]
                    mutated = arms[altered]["window"][wi]["samples"]
                    for b, m in zip(base, mutated):
                        original, bad = b[f"g{target}"]
                        replacement, bad2 = m[f"g{target}"]
                        for k, item in original.items():
                            if k in bad or k in bad2 or k not in replacement:
                                continue
                            counterpart = replacement[k]
                            if item["tag"] != counterpart["tag"]:
                                observed_any = True
                                if item["value"] != counterpart["value"]:
                                    changed_any = True
                                    if len(examples) < 15:
                                        examples.append({
                                            "seed": seed, "arm": altered,
                                            "epoch": b["epoch"], "actor": k[0],
                                            "cell": k[1], "old": item,
                                            "new": counterpart,
                                        })
                        if any(k not in replacement for k in original):
                            loss_any = True
                paired[f"{altered}.paired_source_tag_changed"] += observed_any
                paired[f"{altered}.paired_source_sensitive_output_value_changed"] += changed_any
                paired[f"{altered}.output_missing_after_exposure"] += loss_any
        metrics = {}
        for key, n in sorted(counts.items()):
            if key.startswith("raw."):
                continue
            lo, hi = wilson_95(n, SEEDS)
            metrics[key] = {
                "seed_worlds": n, "denominator": SEEDS,
                "wilson_95": [round(lo, 6), round(hi, 6)],
            }
        return {
            "study": "CL-1 Study 25",
            "source_revision": next(iter(revisions)),
            "independent_seed_worlds": SEEDS,
            "matched_runs": len(rows),
            "hashed_epoch_snapshots": sum(len(v) for v in snapshots.values()),
            "source_tag_and_value_paired_contrasts": dict(sorted(paired.items())),
            "registered_metrics": metrics,
            "observed_raw_event_counts": {
                k: v for k, v in sorted(counts.items()) if k.startswith("raw.")
            },
            "source_owner_reader_pairs": {
                "/".join(map(str, k)): n for k, n in sorted(source_pairs.items())
            },
            "strict_paired_examples": examples,
            "external_content_sensitive_output": (
                "EXPOSED" if any(
                    key.endswith(".paired_source_sensitive_output_value_changed") and n > 0
                    for key, n in paired.items()
                ) else "NOT EVALUABLE: no matched changed read-and-write output"
            ),
            "autonomous_reconstruction": "NOT EVALUABLE",
            "organizational_closure": "NOT EVALUABLE",
            "life_claim": "NONE",
        }
    finally:
        db.close()


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("sqlite", type=Path)
    parser.add_argument("--json", type=Path, required=True)
    args = parser.parse_args()
    report = audit(args.sqlite)
    args.json.write_text(json.dumps(report, indent=2) + "\n", encoding="utf-8")
    print(json.dumps({
        "study": report["study"], "seed_worlds": report["independent_seed_worlds"],
        "runs": report["matched_runs"], "snapshots": report["hashed_epoch_snapshots"],
        "paired_contrasts": report["source_tag_and_value_paired_contrasts"],
        "functional_readout": report["external_content_sensitive_output"],
    }))


if __name__ == "__main__":
    main()
