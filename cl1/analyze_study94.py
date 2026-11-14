#!/usr/bin/env python3
"""Read-only Study94 first-live-event-prefork native lesion/restoration ancestry auditor.

Original 512 genomes * 2 sources * 6 arms; 110592 exact postfork epochs.
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
FIRST = 6_100_000
N = 512
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
PATCH = re.compile(r"^PatchEdge \{ process: 1, node: (\d+), next: (\d+) \}$")


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
    """Independently trace native source output and generic PC8 edits."""
    old_edge = executable_edge(original_code)
    tainted = False
    offset = 0
    visited8 = False
    attempts = 0
    attempted_pcs = []
    after_lesion = False
    external_patches = 0
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
                node, new_edge = int(m.group(1)), int(m.group(2))
                if node == 8:
                    old_edge = new_edge
                external_patches += 1
                after_lesion = external_patches == 1
                if after_lesion:
                    edits_since_lesion = 0
            continue
        if not native(e):
            continue
        require(e["process"] == 1, "unexpected source of native actor event")
        attempts += 1
        pc = e["pc"]
        attempted_pcs.append((attempts, pc))
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
    return {"writes": writes, "edits": edited, "attempts": attempts, "draws": draws, "attempted_pcs": attempted_pcs, "tainted_current": tainted, "offset_current": offset}


def pre_exposure(initial, original_code, source):
    native_summary = trace_native(initial, original_code, source)
    witnesses = {
        (w["pc"], w["offset"])
        for w in native_summary["writes"]
        if w["tainted"] and w["visited8"]
    }
    return witnesses


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


def select_direct_writer(low, high, low_trace, high_trace, witnesses):
    """Source-only rewrite of Rust first-live direct writer P,W,C selection."""
    a = low["processes"]["1"]
    b = high["processes"]["1"]
    p = a["pc"]
    if not (p == b["pc"] and 3 <= p < 27
            and not a["stopped"] and not b["stopped"]
            and min(a["credits"], b["credits"]) >= 2
            and a["registers"][1:] == b["registers"][1:] == [8, 7, 9]
            and low_trace["tainted_current"] and high_trace["tainted_current"]
            and low_trace["offset_current"] == high_trace["offset_current"]
            and a["registers"][0] == 3 + low_trace["offset_current"]
            and b["registers"][0] == 8 + high_trace["offset_current"]):
        return None
    code_a = a["code"]["nodes"]
    code_b = b["code"]["nodes"]

    def step(op, offset):
        name, fields = variant(op)
        if name == "Read" and fields["dst"] == 0 and fields["address"] == 2:
            return (fields["next"], 0)
        if name in ("Add", "Sub") and (
            fields["dst"], fields["lhs"], fields["rhs"]
        ) == (0, 0, 1):
            return (fields["next"], offset + (8 if name == "Add" else -8))
        return None

    low_next = step(code_a[p], low_trace["offset_current"])
    high_next = step(code_b[p], high_trace["offset_current"])
    if low_next is None or low_next != high_next:
        return None
    w, offset = low_next
    if w == p or not 0 <= w < len(code_a) or (w, offset) not in witnesses:
        return None
    writer_a, wf_a = variant(code_a[w])
    writer_b, wf_b = variant(code_b[w])
    if writer_a != writer_b or writer_a != "Write":
        return None
    if (wf_a.get("address"), wf_a.get("value")) != (3, 0) or wf_a != wf_b:
        return None
    c = wf_a["next"]
    if (c in (p, w) or not 0 <= c < len(code_a)
            or variant(code_a[c])[0] in ("Write", "Halt")
            or variant(code_b[c])[0] in ("Write", "Halt")):
        return None
    return (p, w, c, offset)


def auditor(path, expected_rev):
    database = sqlite3.connect(path)
    rows = database.execute(
        "SELECT id,condition,source_revision,initial_sha256,initial_json,actions_json,"
        "attempts_json,final_sha256,final_json FROM epoch_runs ORDER BY id"
    ).fetchall()
    require(len(rows) == N * 2 * len(ARMS), "incomplete registered 6144 native-world cohort")
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
                "unexpected/duplicate Study94 genome arm")
        require(sha(pre_raw) == pre_sha and sha(end_raw) == end_sha,
                "original before/after SHA256 mismatch")
        initial = json.loads(pre_raw)
        original_code = original.expected_code(seed)
        observed_code = initial["processes"]["1"]["code"]["nodes"]
        require(len(observed_code) == len(original_code) == 28, "wrong native genotype size")
        # Native PatchNext can change ONLY PC8's next edge under this grammar.
        # Everything else must remain byte-identical to the independently
        # regenerated SplitMix64 genotype, including opcode tags and operands.
        for pc in range(28):
            if pc == 8:
                observed_tag, observed_fields = variant(observed_code[pc])
                expected_tag, expected_fields = variant(original_code[pc])
                require(observed_tag == expected_tag, "changed native PC8 instruction tag")
                require(
                    {k: v for k, v in observed_fields.items() if k != "next"}
                    == {k: v for k, v in expected_fields.items() if k != "next"},
                    "non-edge functional genotype edit escaped trace",
                )
            else:
                require(observed_code[pc] == original_code[pc],
                        "native checkpoint code differs from source-regenerated genotype outside PC8")
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
                            and 3 <= spec["PatchEdge"]["node"] < 27,
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
        code = original.expected_code(seed)
        low_pre, high_pre = low["initial"], high["initial"]
        low_actor, high_actor = low_pre["processes"]["1"], high_pre["processes"]["1"]
        low_trace = trace_native(low_pre, code, 3)
        high_trace = trace_native(high_pre, code, 8)
        witnesses = low["pre_witnesses"] & high["pre_witnesses"]
        site = select_direct_writer(low_pre, high_pre, low_trace, high_trace, witnesses)
        E = site is not None
        require(low_trace["attempts"] <= MAX_PRE_ATTEMPTS
                and high_trace["attempts"] <= MAX_PRE_ATTEMPTS,
                "pretrigger exceeded preregistered source-pair horizon")
        if E:
            counts["pre_fork_source_function_exposed"] += 1
        for source in (3, 8):
            for arm in ARMS:
                run = runs[(seed, source, arm)]
                patch_actions = [
                    (a["epoch"], a["action"]["PatchEdge"]["node"],
                     a["action"]["PatchEdge"]["next"])
                    for a in run["actions"] if "PatchEdge" in a["action"]
                ]
                expected = []
                if E and arm != "intact":
                    P, W, C, offset = site
                    expected.append((0, P, W if arm == "sham" else C))
                    if arm == "lesion-host-restore":
                        expected.append((4, P, W))
                require(patch_actions == expected,
                        "registered lesion site/target or host repair deviated from blind source gate")
                if arm in ("lesion-edit-disabled", "lesion-host-restore"):
                    require(not any(
                        attempt > run["pre_attempts"]
                        for attempt, _, _, _ in run["trace"]["edits"]
                    ), "mutation-disabled original arm edited actual native executable code")

        def shared_keys(arm, start, end, need_edit=False):
            return (
                post_window_writes(runs[(seed, 3, arm)], start, end, need_edit)
                & post_window_writes(runs[(seed, 8, arm)], start, end, need_edit)
            ) & witnesses

        def first_two_outputs(source, arm):
            run = runs[(seed, source, arm)]
            return {
                (w["pc"], w["offset"])
                for w in run["trace"]["writes"]
                if (w["tainted"] and
                    1 <= w["attempt"] - run["pre_attempts"] <= 2)
            }

        source_first = {
            (source, arm): first_two_outputs(source, arm)
            for source in (3, 8)
            for arm in ("sham", "lesion-native")
        }
        loss_witness = set()
        if E:
            P, W, C, offset = site
            witness = (W, offset)
            if (witness in source_first[(3, "sham")]
                    and witness in source_first[(8, "sham")]
                    and witness not in source_first[(3, "lesion-native")]
                    and witness not in source_first[(8, "lesion-native")]):
                loss_witness.add(witness)
        L = bool(loss_witness)
        if L:
            counts["effective_source_function_loss"] += 1

        A = E and all(
            any(
                attempt > runs[(seed, source, "lesion-native")]["pre_attempts"]
                and pc != site[0]
                for attempt, pc in runs[(seed, source, "lesion-native")]["trace"]["attempted_pcs"]
            )
            for source in (3, 8)
        )
        if A:
            counts["paired_repair_path_reachable_A"] += 1
        patch_opportunities = [
            sum(
                1 for attempt, pc
                in runs[(seed, source, "lesion-native")]["trace"]["attempted_pcs"]
                if attempt > runs[(seed, source, "lesion-native")]["pre_attempts"]
                and variant(code[pc])[0] == "PatchNext"
            )
            for source in (3, 8)
        ]
        if E and all(x > 0 for x in patch_opportunities):
            counts["paired_native_patchnext_attempted_after_lesion"] += 1

        recovered_a = shared_keys("lesion-native", 4, 9, True)
        recovered_b = shared_keys("lesion-native", 10, 17, True)
        host_a = shared_keys("lesion-host-restore", 4, 9)
        host_b = shared_keys("lesion-host-restore", 10, 17)
        host_recovered = L and bool(loss_witness & host_a & host_b)
        if host_recovered:
            counts["explicit_external_host_recovery"] += 1

        # A generic random CIL PatchNext edits node8, but the selected damaged
        # predecessor P is data-derived and may differ from8. Do NOT count a
        # random unrelated node8 edit as functional repair. A non-P direct edit
        # would require a separately proved altered causal path before R.
        causal_edit_p = E and any(
            any(attempt > runs[(seed, source, "lesion-native")]["pre_attempts"]
                for attempt, _, before, after
                in runs[(seed, source, "lesion-native")]["trace"]["edits"]
                if (site[0] == 8 and before == site[2] and after != before))
            for source in (3, 8)
        )
        eligible_recovered = loss_witness & recovered_a & recovered_b
        mutation_negative = all(
            (W, offset) not in post_window_writes(
                runs[(seed, source, "lesion-edit-disabled")], start, end
            )
            for source in (3, 8)
            for start, end in ((4, 9), (10, 17))
        ) if E else False
        R = bool(host_recovered and eligible_recovered and mutation_negative
                 and causal_edit_p and site[0] == 8)
        if R:
            counts["strict_native_function_restoration"] += 1
        if E and not L:
            counts["exposed_but_lesion_not_effective"] += 1
        detail.append({
            "seed": seed,
            "eligible_E": E,
            "selected_p_w_c_offset": list(site) if site is not None else None,
            "loss_L": L,
            "actor_native_reachable_A": bool(A),
            "native_patchnext_attempts_low_high": patch_opportunities,
            "host_control_H": bool(host_recovered),
            "strict_native_repair_R": R,
            "late_function_reappeared_after_pc8_edit_candidate": bool(
                host_recovered and eligible_recovered and not R
            ),
        })
    require(sum(1 for x in detail if x["eligible_E"]) == counts["pre_fork_source_function_exposed"],
            "wrong preselection denominator")
    return {
        "study": 94,
        "status": "PROSPECTIVE_LIVE_NATIVE_WRITER_BYPASS_CAUSAL_SURVEY",
        "source_revision": expected_rev,
        "original_distinct_genotypes": N,
        "original_matched_worlds": len(runs),
        "verified_original_postfork_epoch_sha256": all_hashes,
        "pre_fork_exposed_E": counts["pre_fork_source_function_exposed"],
        "effective_lesion_L": counts["effective_source_function_loss"],
        "post_lesion_reachable_A": counts["paired_repair_path_reachable_A"],
        "paired_post_lesion_patchnext_opportunity": counts["paired_native_patchnext_attempted_after_lesion"],
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
