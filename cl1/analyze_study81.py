#!/usr/bin/env python3
"""Independent read-only native ancestry and all-epoch-hash audit for Study81.

128 untouched seeded genotypes x 4 paired source/edit-law worlds.
Never interprets stochastic PatchNext execution as endogenous repair or life.
"""
import argparse
import hashlib
import json
import sqlite3
from collections import Counter, defaultdict
from pathlib import Path

FIRST = 5_400_000
SEEDS = 128
EPOCHS = 24
QUOTA = 12
ARMS = (
    "source-low-native-edit",
    "source-high-native-edit",
    "source-low-edit-disabled",
    "source-high-edit-disabled",
)
OPS = ("Read", "Write", "Add", "Sub", "Const", "Branch", "DrawCompute", "PatchNext")
A = 6_364_136_223_846_793_005
B = 1_442_695_040_888_963_407
MASK = (1 << 64) - 1


def random_state(state):
    return (state * A + B) & MASK


def mix_seed(seed):
    x = (seed + 0x9e3779b97f4a7c15) & MASK
    x = ((x ^ (x >> 30)) * 0xbf58476d1ce4e5b9) & MASK
    x = ((x ^ (x >> 27)) * 0x94d049bb133111eb) & MASK
    return (x ^ (x >> 31)) & MASK


def expected_code(seed):
    state = mix_seed(seed)
    code = [
        {"Const": {"dst": 1, "value": 8, "next": 1}},
        {"Const": {"dst": 2, "value": 7, "next": 2}},
        {"Const": {"dst": 3, "value": 9, "next": 3}},
    ]
    for pc in range(3, 27):
        nxt = pc + 1
        state = random_state(state)
        choice = state % 8
        if choice == 0:
            op = {"Read": {"dst": 0, "address": 2, "next": nxt}}
        elif choice == 1:
            op = {"Write": {"address": 3, "value": 0, "next": nxt}}
        elif choice == 2:
            op = {"Add": {"dst": 0, "lhs": 0, "rhs": 1, "next": nxt}}
        elif choice == 3:
            op = {"Sub": {"dst": 0, "lhs": 0, "rhs": 1, "next": nxt}}
        elif choice == 4:
            state = random_state(state)
            op = {"Const": {"dst": 0, "value": state % 8, "next": nxt}}
        elif choice == 5:
            state = random_state(state)
            back = 3 if pc == 3 else 3 + state % (pc - 3)
            op = {"Branch": {"test": 0, "nonzero": back, "zero": nxt}}
        elif choice == 6:
            op = {"DrawCompute": {"amount": 1, "next": nxt}}
        else:
            op = {"PatchNext": {"node": 1, "target": 0, "next": nxt}}
        code.append(op)
    code.append("Halt")
    return code


def require(ok, message):
    if not ok:
        raise ValueError(message)


def hash_text(s):
    return hashlib.sha256(s.encode("utf-8")).hexdigest()


def variant(op):
    if op == "Halt":
        return "Halt", {}
    require(isinstance(op, dict) and len(op) == 1, "malformed CIL node")
    return next(iter(op.items()))


def internal(event):
    return event["origin"] == {"Internal": 1}


def value_kind(event, name):
    kind = event["kind"]
    return kind.get(name) if isinstance(kind, dict) else None


def audit_native_events(state, code, source):
    events = state["events"]
    edited_node = code[8]
    tag, fields = variant(edited_node)
    known_edge = None if tag == "Branch" else fields["next"]
    actual_edits = []
    attempted_edits = 0
    suppressed = 0
    successful_draws = 0
    native_attempts = 0
    output_records = []
    coefficient, offset = 0, 0
    for e in events:
        if not internal(e):
            continue
        native_attempts += 1
        pc = e["pc"]
        require(0 <= pc < 28, "native event has invalid program counter")
        op_name, instruction = variant(code[pc])
        k = e["kind"]
        trapped = value_kind(e, "Trapped") is not None
        deferred = value_kind(e, "DrawDeferred") is not None
        if op_name == "PatchNext":
            attempted_edits += 1
            suppression = value_kind(e, "NativeCodeEditSuppressed")
            edit = value_kind(e, "ExecutableEdgePatched")
            if suppression is not None:
                require(suppression["opcode_tag"] == 6, "wrong native edit opcode attribution")
                suppressed += 1
            if edit is not None:
                require(edit["node"] == 8, "unexpected edited executable node")
                require(known_edge is not None, "effective edit of unpatchable branch node")
                require(edit["before"] == known_edge, "native before-edge chain broken")
                require(edit["after"] != known_edge, "no-op edit incorrectly tagged effective")
                require(0 <= edit["after"] < 28, "new native edge invalid")
                known_edge = edit["after"]
                actual_edits.append({"pc": pc, "at_native_attempt": native_attempts, **edit})
            if not trapped:
                require(
                    k == "Executed" or suppression is not None or edit is not None,
                    "PatchNext success missing observable native transition",
                )
        else:
            require(value_kind(e, "ExecutableEdgePatched") is None, "native edge-edit event from non-edit opcode")
            require(value_kind(e, "NativeCodeEditSuppressed") is None, "suppression from non-edit opcode")
        if op_name == "Read" and not trapped:
            read = value_kind(e, "CellRead")
            require(read is not None and read["key"] == 7 and read["value"] == source,
                    "native source-port Read/def-use mismatch")
            coefficient, offset = 1, 0
        elif op_name == "Const" and not trapped:
            if instruction["dst"] == 0:
                coefficient, offset = 0, instruction["value"]
        elif op_name == "Add" and not trapped:
            require(instruction["dst"] == 0 and instruction["lhs"] == 0 and instruction["rhs"] == 1,
                    "unregistered seeded arithmetic opcode")
            offset += 8
        elif op_name == "Sub" and not trapped:
            require(instruction["dst"] == 0 and instruction["lhs"] == 0 and instruction["rhs"] == 1,
                    "unregistered seeded arithmetic opcode")
            offset -= 8
        elif op_name == "DrawCompute" and not trapped and not deferred:
            require(k == "Executed", "Draw success not native Executed")
            successful_draws += 1
        elif op_name == "Write" and not trapped:
            write = value_kind(e, "CellWritten")
            require(write is not None and write["key"] == 9,
                    "native output-port write missing or wrong key")
            require(write["value"] == coefficient * source + offset,
                    "native output differs from independently traced source/arith ancestry")
            output_records.append({
                "pc": pc,
                "value": write["value"],
                "source_coefficient": coefficient,
                "offset": offset,
                "at_native_attempt": native_attempts,
                "edits_before": len(actual_edits),
                "draws_before": successful_draws,
            })
    final_node = state["processes"]["1"]["code"]["nodes"][8]
    ftag, ffields = variant(final_node)
    require(ftag == tag, "native opcode was replaced outside registered grammar")
    require((None if ftag == "Branch" else ffields["next"]) == known_edge,
            "final executable edge inconsistent with native edit event lineage")
    return {
        "native_attempts": native_attempts,
        "attempted_edits": attempted_edits,
        "effective_edits": actual_edits,
        "suppressed_edits": suppressed,
        "native_draws": successful_draws,
        "writes": output_records,
    }


def audit(db_path, revision):
    con = sqlite3.connect(db_path)
    rows = con.execute(
        "SELECT id,condition,source_revision,initial_sha256,initial_json,"
        "attempts_json,final_sha256,final_json FROM epoch_runs ORDER BY id"
    ).fetchall()
    require(len(rows) == 512, "missing registered seed x matched world population")
    genotype_by_seed = {}
    records = {}
    all_hashes = 0
    for run_id, label, recorded_rev, init_sha, init_raw, attempts_raw, final_sha, final_raw in rows:
        require(recorded_rev == revision and revision != "unverified-local-checkout",
                "source pin mismatch")
        require(label.startswith("seed="), "bad original seeded label")
        seed_part, arm = label.removeprefix("seed=").split("/", 1)
        seed = int(seed_part)
        require(FIRST <= seed < FIRST + SEEDS and arm in ARMS, "out-of-registry ecological world")
        require((seed, arm) not in records, "duplicate condition for same genome")
        require(hash_text(init_raw) == init_sha and hash_text(final_raw) == final_sha,
                "broken original initial/final SHA256")
        init = json.loads(init_raw)
        code = init["processes"]["1"]["code"]["nodes"]
        require(
            code == expected_code(seed),
            "original CIL genome differs from independently regenerated seeded grammar",
        )
        require(len(code) == 28 and init["processes"]["1"]["credits"] == 64,
                "unexpected original native genotype or starter")
        require(init["cells"]["7"] == (3 if "-low-" in arm else 8), "wrong external input")
        require(init["trace_native_executable_edits"] is True, "native edit provenance not enabled")
        require(init["pool_paid_draw"] is True and init["recoverable_draw_shortage"] is True,
                "mismatched preregistered native energy physics")
        disabled = "disabled" in arm
        require(
            init.get("native_code_edits_disabled", False) == disabled,
            "incorrect registered edit-permission law",
        )
        genome_sha = hash_text(json.dumps(code, sort_keys=True, separators=(",", ":")))
        if seed in genotype_by_seed:
            require(genotype_by_seed[seed] == genome_sha,
                    "matched arm initial native code is not byte-identical")
        else:
            genotype_by_seed[seed] = genome_sha
        attempts = json.loads(attempts_raw)
        require(
            len(attempts) == EPOCHS
            and all(isinstance(value, int) and 0 <= value <= QUOTA for value in attempts),
            "invalid epoch charge ledger",
        )
        snaps = con.execute(
            "SELECT epoch,state_sha256,state_json FROM epoch_snapshots WHERE run_id=? ORDER BY epoch",
            (run_id,),
        )
        final_state = None
        last_count = 0
        for epoch, sha, raw in snaps:
            require(epoch == last_count, "missing or unordered epoch")
            require(hash_text(raw) == sha, "original snapshot SHA256 discrepancy")
            final_state = json.loads(raw)
            require(final_state["format_version"] == init["format_version"],
                    "unexpected native world format migration")
            last_count += 1
            all_hashes += 1
        require(last_count == EPOCHS and final_state is not None,
                "incomplete epoch snapshots")
        require(hash_text(final_raw) == final_sha and final_raw == raw,
                "final world state mismatch with last original epoch")
        stats = audit_native_events(final_state, code, init["cells"]["7"])
        attempts = json.loads(attempts_raw)
        require(stats["native_attempts"] == sum(attempts),
                "native actor events not equal to charged attempts")
        require(
            final_state["processes"]["1"]["credits"] + final_state["compute_pool"]
            == 64 + 96 + 8 * (EPOCHS - 1) - sum(attempts),
            "conserved original native compute inventory mismatch",
        )
        require(
            sum(
                e["origin"] == "External"
                and isinstance(e["kind"], dict)
                and "Intervened" in e["kind"]
                and "SetComputePool" in e["kind"]["Intervened"]
                for e in final_state["events"]
            ) == EPOCHS,
            "unexpected ambient epoch schedule or host recharging",
        )
        require(
            not any(
                e["origin"] == "External"
                and isinstance(e["kind"], dict)
                and "Intervened" in e["kind"]
                and ("PatchEdge" in e["kind"]["Intervened"]
                     or "SetCredits" in e["kind"]["Intervened"])
                for e in final_state["events"]
            ),
            "host altered native code or per-process endowment",
        )
        if disabled:
            require(not stats["effective_edits"], "disabled code-edit arm changed executable graph")
        else:
            require(stats["suppressed_edits"] == 0, "enabled source arm suppressed real native edit")
        records[(seed, arm)] = stats
    require(
        set(genotype_by_seed) == set(range(FIRST, FIRST + SEEDS))
        and len(set(genotype_by_seed.values())) == SEEDS,
        "nonunique or missing original code genomes",
    )
    counts = Counter()
    paired_edit = Counter()
    seed_outcomes = []
    for seed in range(FIRST, FIRST + SEEDS):
        lo = records[(seed, "source-low-native-edit")]
        hi = records[(seed, "source-high-native-edit")]
        lo_off = records[(seed, "source-low-edit-disabled")]
        hi_off = records[(seed, "source-high-edit-disabled")]
        for arm, run in (("low_on", lo), ("high_on", hi), ("low_off", lo_off), ("high_off", hi_off)):
            if run["attempted_edits"]:
                counts[f"{arm}_attempted_seeds"] += 1
            if run["effective_edits"]:
                counts[f"{arm}_effective_seeds"] += 1
            if run["writes"]:
                counts[f"{arm}_writer_seeds"] += 1
        paired_edit[("yes" if lo["effective_edits"] else "no") +
                    "/" + ("yes" if hi["effective_edits"] else "no")] += 1
        candidates_lo = {
            (w["pc"], w["offset"]) for w in lo["writes"]
            if w["source_coefficient"] == 1
        }
        candidates_hi = {
            (w["pc"], w["offset"]) for w in hi["writes"]
            if w["source_coefficient"] == 1
        }
        w2 = bool(candidates_lo & candidates_hi)
        if w2:
            counts["w2_source_tainted_seed_pairs"] += 1
        def eligible(run):
            return {
                (w["pc"], w["offset"])
                for w in run["writes"]
                if w["source_coefficient"] == 1
                and w["edits_before"] > 0
                and w["draws_before"] > 0
                and w["at_native_attempt"] > 64
            }
        joint = bool(eligible(lo) & eligible(hi))
        if joint:
            counts["w2_w3_effective_edit_coexistence_seed_pairs"] += 1
        seed_outcomes.append({
            "seed": seed,
            "low_effective_edits": len(lo["effective_edits"]),
            "high_effective_edits": len(hi["effective_edits"]),
            "low_off_suppressed_edits": lo_off["suppressed_edits"],
            "high_off_suppressed_edits": hi_off["suppressed_edits"],
            "strict_source_w2": w2,
            "strict_source_w2_draw_and_native_edit": joint,
        })
    return {
        "study": 81,
        "status": "PROSPECTIVE_STOCHASTIC_NATIVE_EDIT_SURVEY_ONLY",
        "source_revision": revision,
        "original_genotypes": len(genotype_by_seed),
        "original_worlds": len(records),
        "verified_original_epoch_sha256": all_hashes,
        "zero_lesion_or_repair_claims": True,
        "counts": dict(counts),
        "paired_low_high_effective_edit_seeds": dict(paired_edit),
        "seed_outcomes": seed_outcomes,
    }


def main():
    p = argparse.ArgumentParser()
    p.add_argument("sqlite", type=Path)
    p.add_argument("--expected-revision", required=True)
    p.add_argument("--json", type=Path, required=True)
    args = p.parse_args()
    result = audit(args.sqlite, args.expected_revision)
    args.json.write_text(json.dumps(result, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    print(json.dumps(
        {key: result[key] for key in (
            "study", "status", "source_revision", "original_genotypes",
            "original_worlds", "verified_original_epoch_sha256", "counts",
            "paired_low_high_effective_edit_seeds",
        )},
        sort_keys=True,
    ))


if __name__ == "__main__":
    main()
