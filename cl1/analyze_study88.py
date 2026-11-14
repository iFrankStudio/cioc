#!/usr/bin/env python3
"""Read-only Study88 first-live-event-prefork native lesion/restoration ancestry auditor.

Original 128 genomes * 2 sources * 6 arms; 27648 exact postfork epochs.
This script NEVER calls a CIL world simulator or performs replacement repair.
"""
import argparse
import bisect
import hashlib
import json
import re
import sqlite3
from collections import Counter
from pathlib import Path

import analyze_study82 as original

SOURCE_REVISION_ENV = "CL1_SOURCE_REVISION"
FIRST = 5_900_000
N = 128
EPOCHS = 18
QUOTA = 12
MAX_PRE_ATTEMPTS = 288
ARMS = (
    "intact",
    "sham",
    "lesion-native",
    "lesion-edit-disabled",
    "lesion-host-restore",
    "lesion-draw-ablated",
)
PATCH = re.compile(r"^PatchEdge \{ process: 1, node: 8, next: (\d+) \}$")


def require(ok, msg):
    if not ok:
        raise ValueError(msg)


def sha(raw):
    return hashlib.sha256(raw.encode("utf-8")).hexdigest()


def variant(op):
    return original.variant(op)


def executable_edge(code):
    op, fields = variant(code[8])
    return None if op == "Branch" else fields["next"]


def native(event):
    return event["origin"] == {"Internal": 1}


def trace_native(state, original_code, sensor):
    """Follow true source def-use and all native AND EXTERNAL PC8 edge transitions."""
    old_edge = executable_edge(original_code)
    tainted = False
    offset = 0
    visited8 = False
    attempts = 0
    after_lesion = False
    edits_since_lesion = 0
    draws = 0
    writes = []
    edited = []
    for e in state["events"]:
        kind = e["kind"]
        if e["origin"] == "External":
            if isinstance(kind, dict) and "InterventionRejected" in kind:
                raise ValueError("rejected external intervention")
            text = kind.get("Intervened") if isinstance(kind, dict) else None
            if text is not None and text.startswith("PatchEdge"):
                m = PATCH.fullmatch(text)
                require(m is not None, "external mutation of unregistered code location")
                old_edge = int(m.group(1))
                after_lesion = old_edge == 8
                if after_lesion:
                    edits_since_lesion = 0
            continue
        if not native(e):
            continue
        require(e["process"] == 1, "unexpected source of native actor event")
        attempts += 1
        pc = e["pc"]
        require(0 <= pc < 28, "invalid native execution PC")
        opcode, fields = variant(original_code[pc])
        trapped = original.value_kind(e, "Trapped") is not None
        deferred = original.value_kind(e, "DrawDeferred") is not None
        changed = original.value_kind(e, "ExecutableEdgePatched")
        suppressed = original.value_kind(e, "NativeCodeEditSuppressed")
        if changed is not None:
            require(opcode == "PatchNext"
                    and changed["node"] == 8
                    and changed["before"] == old_edge
                    and changed["before"] != changed["after"]
                    and 0 <= changed["after"] < 28,
                    "forged native executable edit or broken exact code-edge chain")
            old_edge = changed["after"]
            edited.append((attempts, pc, changed["before"], changed["after"]))
            if after_lesion:
                edits_since_lesion += 1
        if suppressed is not None:
            require(opcode == "PatchNext" and suppressed["opcode_tag"] == 6,
                    "unregistered native edit suppression")
        if trapped or deferred or kind == "Halted":
            continue
        if opcode == "DrawCompute":
            require(kind == "Executed", "non-native Draw success claimed")
            draws += 1
        if pc == 8 and opcode != "Write":
            visited8 = True
        if opcode == "Read":
            read = original.value_kind(e, "CellRead")
            require(read is not None and read["key"] == 7 and read["value"] == sensor,
                    "native source read PC/register ancestry invalid")
            tainted, offset, visited8 = True, 0, pc == 8
        elif opcode == "Const" and fields["dst"] == 0:
            require(kind == "Executed", "non-native Const accepted")
            tainted, offset, visited8 = False, fields["value"], False
        elif opcode == "Add":
            offset += 8
        elif opcode == "Sub":
            offset -= 8
        elif opcode == "Write":
            out = original.value_kind(e, "CellWritten")
            require(out is not None and out["key"] == 9
                    and out["value"] == sensor * int(tainted) + offset,
                    "source-dependent native output register ancestry invalid")
            writes.append({
                "pc": pc,
                "offset": offset,
                "value": out["value"],
                "tainted": tainted,
                "visited8": visited8,
                "attempt": attempts,
                "edits_after_lesion_before": edits_since_lesion,
                "draws_before": draws,
            })
    require(executable_edge(state["processes"]["1"]["code"]["nodes"]) == old_edge,
            "final actual code graph is not reconstructed from native and external edits")
    return {"writes": writes, "edits": edited, "attempts": attempts, "draws": draws, "tainted_current": tainted, "offset_current": offset}


def pre_exposure(initial, original_code, source):
    native_summary = trace_native(initial, original_code, source)
    witnesses = {
        (w["pc"], w["offset"])
        for w in native_summary["writes"]
        if w["tainted"] and w["visited8"]
    }
    return witnesses if executable_edge(initial["processes"]["1"]["code"]["nodes"]) is not None else set()


def post_window_writes(run, start_epoch, end_epoch, require_post_lesion_edit=False):
    pre_attempts = run["pre_attempts"]
    totals = []
    cursor = 0
    for count in run["charges"]:
        cursor += count
        totals.append(cursor)
    values = set()
    for write in run["trace"]["writes"]:
        if not write["tainted"] or write["attempt"] <= pre_attempts:
            continue
        post_attempt = write["attempt"] - pre_attempts
        local_epoch = bisect.bisect_left(totals, post_attempt)
        require(0 <= local_epoch < EPOCHS, "postfork output ordinal outside epoch ledger")
        if not start_epoch <= local_epoch <= end_epoch:
            continue
        if require_post_lesion_edit and write["edits_after_lesion_before"] == 0:
            continue
        values.add((write["pc"], write["offset"]))
    return values


def auditor(path, expected_rev):
    database = sqlite3.connect(path)
    rows = database.execute(
        "SELECT id,condition,source_revision,initial_sha256,initial_json,actions_json,"
        "attempts_json,final_sha256,final_json FROM epoch_runs ORDER BY id"
    ).fetchall()
    require(len(rows) == N * 2 * len(ARMS), "incomplete registered 1536 native-world cohort")
    runs = {}
    genotype_hashes = {}
    all_hashes = 0
    for run_id, label, revision, pre_sha, pre_raw, plan_raw, charges_raw, end_sha, end_raw in rows:
        require(revision == expected_rev and revision != "unverified-local-checkout",
                "source revision mismatch")
        parts = label.split("/")
        require(len(parts) == 3 and parts[0].startswith("seed=")
                and parts[1] in ("source=3", "source=8")
                and parts[2] in ARMS, "unregistered source/arm label")
        seed = int(parts[0].removeprefix("seed="))
        source = int(parts[1].removeprefix("source="))
        arm = parts[2]
        require(FIRST <= seed < FIRST + N and (seed, source, arm) not in runs,
                "unexpected/duplicate Study88 genome arm")
        require(sha(pre_raw) == pre_sha and sha(end_raw) == end_sha,
                "original before/after SHA256 mismatch")
        initial = json.loads(pre_raw)
        original_code = original.expected_code(seed)
        observed_code = initial["processes"]["1"]["code"]["nodes"]
        original_code_hash = sha(json.dumps(original_code, sort_keys=True))
        if seed in genotype_hashes:
            require(genotype_hashes[seed] == original_code_hash,
                    "seed changed original preregistered genotype")
        else:
            genotype_hashes[seed] = original_code_hash
        require(initial["cells"]["7"] == source and initial["trace_native_executable_edits"],
                "wrong original external source or missing native edit observer")
        require(initial["pool_paid_draw"] and initial["recoverable_draw_shortage"],
                "changed substrate resource law")
        pre_history = dict(initial)
        pre_history.pop("native_code_edits_disabled", None)
        pre_history["native_code_edits_disabled"] = False
        disabled = arm in ("lesion-edit-disabled", "lesion-host-restore")
        require(initial.get("native_code_edits_disabled", False) == disabled,
                "edit-disabled arm changed before the fork")
        pre_key = (seed, source)
        normalized = json.dumps(pre_history, sort_keys=True)
        if not hasattr(auditor, "_prefixes"):
            auditor._prefixes = {}
        if pre_key in auditor._prefixes:
            require(auditor._prefixes[pre_key] == normalized,
                    "matched arms do not share byte-identical pre-fork world history")
        else:
            auditor._prefixes[pre_key] = normalized
        pre_trace = trace_native(initial, original_code, source)
        pre_attempts = pre_trace["attempts"]
        require(pre_attempts <= MAX_PRE_ATTEMPTS, "pretrigger exceeded first-live attempt horizon")
        # Event history of the original pre-fork world includes exactly
        # three external initialization events plus the source-specific
        # ambient pool updates every 12 paired attempts while active.
        require(len(initial["events"]) >= pre_attempts + 3,
                "missing original native pretrigger event ancestry")
        charges = json.loads(charges_raw)
        require(len(charges) == EPOCHS and all(isinstance(x, int) and 0 <= x <= QUOTA for x in charges),
                "invalid postfork charge count")
        actions = json.loads(plan_raw)
        snaps = database.execute(
            "SELECT epoch,state_sha256,state_json FROM epoch_snapshots WHERE run_id=? ORDER BY epoch",
            (run_id,),
        )
        previous = initial
        epochs = 0
        net_environment = 0
        for epoch, h, raw in snaps:
            require(epoch == epochs and sha(raw) == h, "broken epoch order or full snapshot hash")
            state = json.loads(raw)
            prior_events = previous["events"]
            events = state["events"]
            require(events[:len(prior_events)] == prior_events,
                    "original native/host event history not prefix preserving")
            appended = events[len(prior_events):]
            scheduled = [a for a in actions if a["epoch"] == epoch]
            require(len(appended) == len(scheduled) + charges[epoch],
                    "unaccounted native charged attempt or external event")
            old_pool = previous["compute_pool"]
            expected_pool = 0 if arm == "lesion-draw-ablated" else old_pool + 8
            require(scheduled and scheduled[0]["action"] ==
                    {"SetComputePool": {"credits": expected_pool}},
                    "unregistered resource supply or ablation law")
            net_environment += expected_pool - old_pool
            for event, action in zip(appended[:len(scheduled)], scheduled):
                require(event["origin"] == "External"
                        and isinstance(event["kind"], dict)
                        and "Intervened" in event["kind"],
                        "unregistered external intervention rejection")
                spec = action["action"]
                if "PatchEdge" in spec:
                    require(spec["PatchEdge"]["process"] == 1
                            and spec["PatchEdge"]["node"] == 8,
                            "unregistered externally patched code")
            require(all(native(e) and e["process"] == 1 for e in appended[len(scheduled):]),
                    "non-native actor in charged attempts")
            previous = state
            epochs += 1
            all_hashes += 1
        require(epochs == EPOCHS and sha(end_raw) == end_sha
                and json.dumps(previous, separators=(",", ":")) != "",
                "missing last original postfork snapshot")
        require(previous == json.loads(end_raw), "last full state not equal saved original final")
        start_inventory = initial["processes"]["1"]["credits"] + initial["compute_pool"]
        end_inventory = previous["processes"]["1"]["credits"] + previous["compute_pool"]
        require(end_inventory == start_inventory + net_environment - sum(charges),
                "native compute inventory conservation broken")
        trace = trace_native(previous, original_code, source)
        require(trace["attempts"] == pre_attempts + sum(charges),
                "full native attempts do not equal original pre+post ledger")
        runs[(seed, source, arm)] = {
            "initial": initial,
            "final": previous,
            "actions": actions,
            "charges": charges,
            "pre_attempts": pre_attempts,
            "pre_witnesses": pre_exposure(initial, original_code, source),
            "original_edge": executable_edge(observed_code),
            "trace": trace,
        }
    require(len(genotype_hashes) == N and len(set(genotype_hashes.values())) == N,
            "duplicate or missing original prospectively preregistered genotypes")
    # No-treatment postfork starting snapshots must be source paired and the
    # mutation-disabled flag is the ONLY permitted control-law fork difference.
    for seed in range(FIRST, FIRST + N):
        for source in (3, 8):
            for arm in ARMS:
                require(
                    runs[(seed, source, arm)]["pre_attempts"]
                    == runs[(seed, source, "intact")]["pre_attempts"],
                    "matched arm changed original first-live trigger history",
                )
    counts = Counter()
    detail = []
    for seed in range(FIRST, FIRST + N):
        low = runs[(seed, 3, "intact")]
        high = runs[(seed, 8, "intact")]
        witnesses = low["pre_witnesses"] & high["pre_witnesses"]
        # First-live checkpoint is certified strictly by the pre-fork source states.
        low_state = low["initial"]["processes"]["1"]
        high_state = high["initial"]["processes"]["1"]
        low_trace = trace_native(low["initial"], original.expected_code(seed), 3)
        high_trace = trace_native(high["initial"], original.expected_code(seed), 8)
        eligible_time = low_trace["attempts"] == high_trace["attempts"]
        E = bool(
            witnesses and eligible_time
            and 0 < low_trace["attempts"] <= MAX_PRE_ATTEMPTS
            and low["original_edge"] not in (None, 8)
            and high["original_edge"] not in (None, 8)
            and low_state["pc"] == high_state["pc"] == 8
            and not low_state["stopped"] and not high_state["stopped"]
            and low_state["credits"] > 0 and high_state["credits"] > 0
            and low_state["registers"][1:] == [8, 7, 9]
            and high_state["registers"][1:] == [8, 7, 9]
            and low_trace["tainted_current"] and high_trace["tainted_current"]
            and low_trace["offset_current"] == high_trace["offset_current"]
            and low_state["registers"][0] == 3 + low_trace["offset_current"]
            and high_state["registers"][0] == 8 + high_trace["offset_current"]
        )
        # The source code must not silently reinterpret a source-specific
        # mismatch as a nonexposed sample. We independently test consistency.
        require(eligible_time, "low/high original pretrigger native attempt mismatch")
        require(low_trace["attempts"] <= MAX_PRE_ATTEMPTS,
                "exceeded preregistered paired trigger horizon")
        if E:
            counts["pre_fork_source_function_exposed"] += 1
        for source in (3, 8):
            edge = runs[(seed, source, "intact")]["original_edge"]
            for arm in ARMS:
                run = runs[(seed, source, arm)]
                patch_actions = [
                    (a["epoch"], a["action"]["PatchEdge"]["next"])
                    for a in run["actions"] if "PatchEdge" in a["action"]
                ]
                expected = []
                if E and arm != "intact":
                    expected.append((0, edge if arm == "sham" else 8))
                    if arm == "lesion-host-restore":
                        expected.append((4, edge))
                require(patch_actions == expected,
                        "selection or lesion schedule violates blinded pre-treatment gate")
                if arm == "lesion-edit-disabled" or arm == "lesion-host-restore":
                    require(not any(
                        attempt > run["pre_attempts"]
                        for attempt, _, _, _ in run["trace"]["edits"]
                    ), "mutation-disabled postfork arm performed real native edits")
        def shared_keys(arm, start, end, require_edit=False):
            return (
                post_window_writes(runs[(seed, 3, arm)], start, end, require_edit)
                & post_window_writes(runs[(seed, 8, arm)], start, end, require_edit)
            ) & witnesses
        sham_early = shared_keys("sham", 0, 3)
        lesion_early = shared_keys("lesion-native", 0, 3)
        # Require BOTH input-specific lesion streams lack the eligible function.
        absent_both = {
            key for key in witnesses
            if key not in post_window_writes(runs[(seed, 3, "lesion-native")], 0, 3)
            and key not in post_window_writes(runs[(seed, 8, "lesion-native")], 0, 3)
        }
        loss_witnesses = sham_early & absent_both if E else set()
        L = bool(loss_witnesses)
        if L:
            counts["effective_source_function_loss"] += 1
        recovered_a = shared_keys("lesion-native", 4, 9, True)
        recovered_b = shared_keys("lesion-native", 10, 17, True)
        disabled_a = shared_keys("lesion-edit-disabled", 4, 9)
        disabled_b = shared_keys("lesion-edit-disabled", 10, 17)
        host_recovered = bool(loss_witnesses
                              & shared_keys("lesion-host-restore", 4, 9)
                              & shared_keys("lesion-host-restore", 10, 17)) if L else False
        restored = bool(
            loss_witnesses
            & recovered_a
            & recovered_b
            - (disabled_a | disabled_b)
        ) if L and host_recovered else False
        # Confirm each disabled INPUT stream is absent, not merely the paired W2.
        if restored:
            candidate = loss_witnesses & recovered_a & recovered_b
            confirmed = {
                key for key in candidate
                if all(
                    key not in post_window_writes(
                        runs[(seed, source, "lesion-edit-disabled")], window[0], window[1]
                    )
                    for source in (3, 8)
                    for window in ((4, 9), (10, 17))
                )
            }
            restored = bool(confirmed)
        if restored:
            counts["strict_native_function_restoration"] += 1
        if host_recovered:
            counts["explicit_external_host_recovery"] += 1
        if E and not L:
            counts["exposed_but_lesion_not_effective"] += 1
        detail.append({
            "seed": seed,
            "eligible_E": E,
            "pre_source_witness_count": len(witnesses),
            "loss_L": L,
            "restoration_R": restored,
            "external_restore_positive": host_recovered,
            "native_recovery_candidate_a": bool(recovered_a),
            "native_recovery_candidate_b": bool(recovered_b),
        })
    require(sum(1 for x in detail if x["eligible_E"]) == counts["pre_fork_source_function_exposed"],
            "wrong preselection denominator")
    return {
        "study": 88,
        "status": "PROSPECTIVE_LIVE_PC8_NATIVE_CODE_LESION_CAUSAL_SURVEY",
        "source_revision": expected_rev,
        "original_distinct_genotypes": N,
        "original_matched_worlds": len(runs),
        "verified_original_postfork_epoch_sha256": all_hashes,
        "pre_fork_exposed_E": counts["pre_fork_source_function_exposed"],
        "effective_lesion_L": counts["effective_source_function_loss"],
        "strict_native_repair_R": counts["strict_native_function_restoration"],
        "native_repair_causally_evaluable": counts["explicit_external_host_recovery"] > 0,
        "explicit_host_restoration_controls": counts["explicit_external_host_recovery"],
        "counts": dict(counts),
        "seed_gate_outcomes": detail,
        "multi_component_cl1_closed": False,
    }


def main():
    p = argparse.ArgumentParser()
    p.add_argument("sqlite", type=Path)
    p.add_argument("--expected-revision", required=True)
    p.add_argument("--json", type=Path, required=True)
    args = p.parse_args()
    result = auditor(args.sqlite, args.expected_revision)
    args.json.write_text(json.dumps(result, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    print(json.dumps({k: result[k] for k in (
        "study", "status", "original_distinct_genotypes", "original_matched_worlds",
        "verified_original_postfork_epoch_sha256", "pre_fork_exposed_E",
        "effective_lesion_L", "strict_native_repair_R",
        "explicit_host_restoration_controls")}, sort_keys=True))


if __name__ == "__main__":
    main()
