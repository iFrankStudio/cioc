#!/usr/bin/env python3
"""Read-only Study80 Phase-0 full SQLite + native event ancestry auditor.

Ten HUMAN-scripted calibration runs, zero independently seeded ecological worlds.
Rust replay independently establishes executable determinism; this observer verifies
all original serialized epoch hashes and preregistered control-specific mechanisms.
"""
import argparse
import hashlib
import json
import sqlite3
from collections import defaultdict
from pathlib import Path

EPOCHS = 12
QUOTA = 2
SOURCES = (3, 8)
ARMS = ("intact", "sham", "lesion-edit", "lesion-disabled", "lesion-host-repair")


def require(test, message):
    if not test:
        raise ValueError(message)


def digest(data):
    return hashlib.sha256(data.encode("utf-8")).hexdigest()


def node_edge(state):
    node = state["processes"]["1"]["code"]["nodes"][3]
    require("Read" in node, "function-bearing PC3 is no longer native Read")
    return node["Read"]["next"]


def record_writes(state):
    result = []
    for event in state["events"]:
        kind = event["kind"]
        if event["origin"] != {"Internal": 1}:
            continue
        if isinstance(kind, dict) and "CellWritten" in kind:
            item = kind["CellWritten"]
            if item["key"] == 9:
                require(event["pc"] == 4, "source write from unexpected PC")
                result.append(item["value"])
    return result


def count_suppressed(state):
    return sum(
        1
        for e in state["events"]
        if e["origin"] == {"Internal": 1}
        and isinstance(e["kind"], dict)
        and e["kind"].get("NativeCodeEditSuppressed", {}).get("opcode_tag") == 6
    )


def audit(path, revision):
    conn = sqlite3.connect(path)
    rows = conn.execute(
        "SELECT id,condition,source_revision,initial_json,initial_sha256,"
        "attempts_json,final_json,final_sha256 FROM epoch_runs ORDER BY id"
    ).fetchall()
    require(len(rows) == len(SOURCES) * len(ARMS), "not exactly 10 original scripted worlds")
    seen = set()
    code_by_source = defaultdict(set)
    signature = {}
    run_report = []
    hashed = 0
    for run_id, label, source_rev, init_json, init_sha, attempts_json, final_json, final_sha in rows:
        require(source_rev == revision and source_rev != "unverified-local-checkout", "source revision mismatch")
        require(label.startswith("study80-phase0/source="), "unregistered run label")
        token = label.removeprefix("study80-phase0/source=")
        source_label, arm = token.split("/", 1)
        source = int(source_label)
        require(source in SOURCES and arm in ARMS, "unexpected source/arm")
        require((source, arm) not in seen, "duplicate original control arm")
        seen.add((source, arm))
        require(digest(init_json) == init_sha and digest(final_json) == final_sha, "initial/final SHA mismatch")
        attempts = json.loads(attempts_json)
        require(attempts == [QUOTA] * EPOCHS, "incomplete native instruction attempts")
        initial = json.loads(init_json)
        code = initial["processes"]["1"]["code"]["nodes"]
        code_by_source[source].add(json.dumps(code, sort_keys=True, separators=(",", ":")))
        require(initial["processes"]["1"]["credits"] == 48, "different initial process credits")
        require(initial["cells"]["4"] == source, "source cell not initialized externally")
        require(initial["native_code_edits_disabled"] is True if arm in ("lesion-disabled", "lesion-host-repair") else "native_code_edits_disabled" not in initial, "incorrect serialized edit-ablation flag")

        snap_rows = conn.execute(
            "SELECT epoch,state_sha256,state_json FROM epoch_snapshots WHERE run_id=? ORDER BY epoch",
            (run_id,),
        ).fetchall()
        require(len(snap_rows) == EPOCHS, "missing original epoch snapshots")
        states = []
        for expected_epoch, (epoch, sha, raw) in enumerate(snap_rows):
            require(epoch == expected_epoch and digest(raw) == sha, "failed original epoch SHA256")
            states.append(json.loads(raw))
            hashed += 1
        require(snap_rows[-1][2] == final_json, "last epoch state differs from final")
        require(node_edge(states[2]) == 4, "unexpected pre-lesion functional path")
        if arm in ("lesion-edit", "lesion-disabled", "lesion-host-repair"):
            require(node_edge(states[3]) == 5, "lesion was not present at the post-intervention boundary")
        else:
            require(node_edge(states[3]) == 4, "intact or sham edge unexpectedly changed")
        if arm in ("lesion-edit", "lesion-host-repair"):
            require(node_edge(states[4]) == 4, "native or external restoration did not restore the edge")
        final_state = states[-1]
        require(final_state["processes"]["1"]["credits"] == 48 - EPOCHS * QUOTA, "native compute charge mismatch")
        require(final_state["compute_pool"] == 0, "unregistered energy injection")
        require(final_state["processes"]["1"]["stopped"] is False, "unexpected native stop/trap")
        native_events = [e for e in final_state["events"] if e["origin"] == {"Internal": 1}]
        require(len(native_events) == EPOCHS * QUOTA, "native event/charged attempts mismatch")
        require(not any(isinstance(e["kind"], dict) and "Trapped" in e["kind"] for e in native_events), "native trapped")
        require(all(e["credits_after"] >= 0 for e in native_events), "negative actor credits")
        values_by_epoch = [record_writes(state) for state in states]
        require(values_by_epoch[2] == [source], "missing unique pre-lesion source read/write")
        require(all(set(values).issubset({source}) for values in values_by_epoch), "output does not match source")
        if arm in ("lesion-edit", "lesion-disabled", "lesion-host-repair"):
            require(values_by_epoch[3] == [source], "lesion failed to suppress first post-lesion output")
            # The first post-lesion two attempts must be a real Read at PC3,
            # followed by the ORIGINAL actor executing the edit opcode at PC5.
            new_native = [
                event for event in states[3]["events"][len(states[2]["events"]):]
                if event["origin"] == {"Internal": 1}
            ]
            require(
                len(new_native) == 2
                and [event["pc"] for event in new_native] == [3, 5],
                "post-lesion source-PC chronology invalid",
            )
            edit_event = new_native[-1]["kind"]
            if arm == "lesion-edit":
                require(edit_event == "Executed", "native code edit did not execute at original PC5")
            else:
                require(
                    isinstance(edit_event, dict)
                    and edit_event.get("NativeCodeEditSuppressed", {}).get("opcode_tag") == 6,
                    "mutation-disabled actor did not execute charged edit suppression",
                )
        if arm == "lesion-edit":
            require(count_suppressed(final_state) == 0, "unexpected edit suppression in native arm")
            require(len(values_by_epoch[4]) > len(values_by_epoch[3]), "no first native functional restoration")
            require(len(values_by_epoch[6]) > len(values_by_epoch[5]), "no second restoration window")
            require(len(values_by_epoch[7]) > len(values_by_epoch[6]), "no third restoration window")
        if arm == "lesion-disabled":
            require(values_by_epoch[-1] == [source], "mutant-disabled control regained output")
            require(count_suppressed(final_state) > 0, "no source-audited native mutation suppression")
        if arm == "lesion-host-repair":
            require(len(values_by_epoch[4]) > len(values_by_epoch[3]), "external rescue calibration not restored")
        if arm == "sham":
            require(len(values_by_epoch[3]) > len(values_by_epoch[2]), "sham unexpectedly interrupted output")
        if arm != "intact":
            external_patches = [
                e for e in final_state["events"]
                if e["origin"] == "External"
                and isinstance(e["kind"], dict)
                and "Intervened" in e["kind"]
                and "PatchEdge" in e["kind"]["Intervened"]
            ]
            require(len(external_patches) == (2 if arm == "lesion-host-repair" else 1), "missing/extra host edits")
        signature[(source, arm)] = {
            "initial_code_hash": digest(json.dumps(code, sort_keys=True)),
            "total_writes": len(values_by_epoch[-1]),
            "edit_suppression_events": count_suppressed(final_state),
        }
        run_report.append({"source": source, "arm": arm, **signature[(source, arm)]})
    require(seen == {(s, a) for s in SOURCES for a in ARMS}, "missing control")
    require(all(len(code_by_source[source]) == 1 for source in SOURCES), "matched arms changed original CIL code")
    require(
        all(signature[(3, arm)]["initial_code_hash"] == signature[(8, arm)]["initial_code_hash"] for arm in ARMS),
        "input sensor changed executable initial genotype",
    )
    return {
        "status": "SCRIPTED_MECHANISM_CALIBRATION_ONLY",
        "source_revision": revision,
        "human_authored_worlds": len(rows),
        "independent_ecological_genomes": 0,
        "verified_epoch_sha256": hashed,
        "all_worlds_source_code_identical": True,
        "autonomous_restoration_or_life": False,
        "runs": run_report,
    }


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("sqlite", type=Path)
    parser.add_argument("--expected-revision", required=True)
    parser.add_argument("--json", type=Path)
    args = parser.parse_args()
    result = audit(args.sqlite, args.expected_revision)
    args.json.write_text(json.dumps(result, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    print(json.dumps({key: result[key] for key in ("status", "human_authored_worlds", "independent_ecological_genomes", "verified_epoch_sha256")}, sort_keys=True))


if __name__ == "__main__":
    main()
