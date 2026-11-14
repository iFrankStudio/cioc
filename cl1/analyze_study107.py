#!/usr/bin/env python3
"""Read-only Study107 native 28-site census observer; zero spontaneous ecology genotypes."""
import argparse
import hashlib
import json
import math
import sqlite3
from collections import Counter, defaultdict
from pathlib import Path

SITES = range(28)
MODES = (
    "native-edit-source3", "patch-disabled-source3",
    "native-edit-source8", "patch-disabled-source8",
)
EPOCHS = 8
QUOTA = 250


def require(value, message):
    if not value:
        raise ValueError(message)


def sha(s):
    return hashlib.sha256(s.encode("utf-8")).hexdigest()


def event_field(event, key):
    k = event["kind"]
    return k.get(key) if isinstance(k, dict) else None


def audit_state(state, source, site, ablated, attempts):
    require(state["compute_pool"] + sum(
        p["credits"] for p in state["processes"].values()
    ) == 3400 - attempts, "native resource ledger mismatch")
    external = [e for e in state["events"] if e["origin"] == "External"]
    require(len(external) == 5, "unregistered external action")
    natives = [e for e in state["events"] if isinstance(e["origin"], dict)]
    require(len(natives) == attempts, "native event count not charged attempt count")

    read = {}
    writers = set()
    copied = defaultdict(set)
    draws = Counter()
    launches = Counter()
    mutation_events = []
    suppressed = 0
    traps = []
    birth_parent = {}
    birth_code_sites = {}
    for e in natives:
        actor = e["process"]
        require(e["origin"] == {"Internal": actor}, "false native origin")
        rd = event_field(e, "CellRead")
        wr = event_field(e, "CellWritten")
        edit = event_field(e, "ExecutableEdgePatched")
        denied = event_field(e, "NativeCodeEditSuppressed")
        cloned = event_field(e, "GraphNodeCopied")
        launch = event_field(e, "GraphLaunched")
        trapped = event_field(e, "Trapped")
        if rd and rd["key"] == 7:
            require(e["pc"] == 2 and rd["value"] == source, "bad true source read")
            read[actor] = source
        if wr and wr["key"] == 9:
            require(e["pc"] == 3 and wr["value"] == source, "bad native source write")
            require(read.get(actor) == source, "native write missing earlier actor read")
            writers.add(actor)
        if edit is not None:
            require(not ablated and e["pc"] == 8 and edit["node"] == site,
                    "wrong original actor edit node or origin")
            require(edit["before"] != edit["after"] and edit["after"] in (5, 24),
                    "native edit not registered live edge")
            mutation_events.append((actor, edit))
        if denied is not None:
            require(ablated and e["pc"] == 8 and denied["opcode_tag"] == 6,
                    "invalid source of mutation suppression")
            suppressed += 1
        if trapped is not None:
            traps.append((actor, e["pc"], trapped))
        if cloned is not None:
            require(e["pc"] == 16 and cloned["source_process"] == actor,
                    "child copying sourced to wrong CIL actor")
            copied[actor].add(cloned["source_node"])
        if e["pc"] == 21 and e["kind"] == "Executed":
            draws[actor] += 1
        if launch is not None:
            require(e["pc"] == 22 and launch["parent_process"] == actor,
                    "incorrect native child launch attribution")
            child = launch["child_process"]
            require(child not in birth_parent and child >= 3, "duplicate or host child")
            require(launch["moved_nodes"] == 28 and launch["transferred_credits"] == 400,
                    "incorrect copied code volume or compute credit grant")
            require(draws[actor] > launches[actor], "native launch not preceded by true Draw")
            require(copied[actor] == set(range(28)), "not full28 node CIL code copy")
            copied[actor] = set()
            launches[actor] += 1
            birth_parent[child] = actor
            newborn = state["processes"][str(child)]["code"]["nodes"]
            require(len(newborn) == 28, "child not full original size")
            require(newborn[7]["Const"]["value"] == site, "native child lacks inherited mutation-site parameter")
            birth_code_sites[child] = site

    require(writers.issuperset({1, 2}), "native founder source function absent")
    if ablated:
        require(not mutation_events, "native mutation despite narrow disable")
    else:
        require(suppressed == 0, "suppression in enabled world")
    return {
        "native_births": len(birth_parent),
        "native_mutation_events": len(mutation_events),
        "native_suppression_events": suppressed,
        "native_source_writers": len(writers),
        "trap_count": len(traps),
        "trap_pcs": sorted(Counter(pc for _, pc, _ in traps).items()),
        "trap_reasons": sorted(Counter(reason for _, _, reason in traps).items()),
        "native_grandchildren": sum(1 for a in birth_parent.values() if a >= 3),
    }


def exact_sign_p(deltas):
    a = sum(v > 0 for v in deltas)
    b = sum(v < 0 for v in deltas)
    n = a + b
    if n == 0:
        return 1.0
    tail = sum(math.comb(n, k) for k in range(min(a, b) + 1))
    return min(1.0, 2 * tail / 2**n)


def audit(db_path, revision):
    conn = sqlite3.connect(db_path)
    rows = conn.execute(
        "SELECT id,condition,source_revision,initial_json,initial_sha256,"
        "attempts_json,final_json,final_sha256 FROM epoch_runs ORDER BY id"
    ).fetchall()
    require(len(rows) == 112, "not exactly 112 original systematic site controls")
    out = {}
    code_by_site = defaultdict(set)
    snapshots = 0
    for rid, label, source_revision, init, init_sha, attempt_js, final, final_sha in rows:
        require(source_revision == revision and len(revision) == 40, "wrong source revision")
        require(label.startswith("site="), "unexpected site label")
        site_text, mode = label.removeprefix("site=").split("/", 1)
        site = int(site_text)
        require(site in SITES and mode in MODES and (site, mode) not in out,
                "duplicated or unregistered original site mode")
        require(sha(init) == init_sha and sha(final) == final_sha, "unverified original run hash")
        first = json.loads(init)
        source = 3 if mode.endswith("source3") else 8
        ablated = mode.startswith("patch-disabled")
        require(first["cells"]["7"] == source and first["cells"]["11"] == 5,
                "wrong external source or mutation input")
        require(first["compute_pool"] == 2400, "incorrect common compute pool")
        require(len(first["processes"]) == 2, "host-loaded nonfounder process")
        require(all(first["processes"][str(i)]["credits"] == 500 for i in (1, 2)),
                "founder credits incorrect")
        require(first["processes"]["1"]["code"] == first["processes"]["2"]["code"],
                "original CIL founders not byte-identical")
        require(first.get("native_patchnext_disabled", False) == ablated,
                "incorrect matched mutation-only VM law")
        code = first["processes"]["1"]["code"]
        require(code["nodes"][7] == {"Const": {"dst": 1, "value": site, "next": 8}},
                "site parameter source differs from preregistration")
        require(code["nodes"][20]["Const"]["value"] == 400
                and code["nodes"][24]["Const"]["value"] == 20,
                "non-site genotype parameter differs from freeze")
        code_by_site[site].add(sha(json.dumps(code, sort_keys=True)))
        steps = json.loads(attempt_js)
        require(len(steps) == EPOCHS and all(0 <= v <= QUOTA for v in steps),
                "invalid charged attempt quota")
        snapshots_rows = conn.execute(
            "SELECT epoch,state_sha256,state_json FROM epoch_snapshots WHERE run_id=? ORDER BY epoch",
            (rid,),
        ).fetchall()
        require(len(snapshots_rows) == EPOCHS, "missing exact epoch")
        for epoch, (i, hs, raw) in enumerate(snapshots_rows):
            require(i == epoch and sha(raw) == hs, "SHA256 mismatch in original native epoch")
            snapshots += 1
        require(snapshots_rows[-1][2] == final, "last epoch not original final")
        outcome = audit_state(json.loads(final), source, site, ablated, sum(steps))
        out[(site, mode)] = {
            "attempts": sum(steps),
            "source": source,
            "patch_disabled": ablated,
            **outcome,
        }
    require(len(out) == 112 and all(len(code_by_site[k]) == 1 for k in SITES),
            "missing site or same-genome matched controls corrupted")
    require(len({next(iter(code_by_site[k])) for k in SITES}) == 28,
            "not exactly28 original distinct site parameter genomes")

    deltas = {}
    for source in (3, 8):
        vals = []
        for site in SITES:
            a = out[(site, f"native-edit-source{source}")]
            b = out[(site, f"patch-disabled-source{source}")]
            vals.append(a["native_births"] - b["native_births"])
        deltas[f"source{source}"] = {
            "all_28_site_birth_differences": vals,
            "positive": sum(x > 0 for x in vals),
            "negative": sum(x < 0 for x in vals),
            "ties": sum(x == 0 for x in vals),
            "exact_sign_p_illustrative_only": exact_sign_p(vals),
            "sites_with_real_native_mutation": [
                site for site in SITES
                if out[(site, f"native-edit-source{source}")]["native_mutation_events"] > 0
            ],
            "enabled_birth_total": sum(out[(site, f"native-edit-source{source}")]["native_births"] for site in SITES),
            "patch_disabled_birth_total": sum(out[(site, f"patch-disabled-source{source}")]["native_births"] for site in SITES),
            "enabled_trap_sites": [
                site for site in SITES
                if out[(site, f"native-edit-source{source}")]["trap_count"] > 0
            ],
            "disabled_trap_sites": [
                site for site in SITES
                if out[(site, f"patch-disabled-source{source}")]["trap_count"] > 0
            ],
        }
    return {
        "status": "EXHAUSTIVE_HUMAN_AUTHORED_NATIVE_EDGE_MUTATION_FRAGILITY_CENSUS_NOT_LIFE",
        "source_revision": revision,
        "original_site_variants": 28,
        "original_native_matched_runs": len(rows),
        "full_SHA256_epoch_snapshots": snapshots,
        "comparison_by_source": deltas,
        "all_results": [
            {"site": k, "mode": m, **out[(k, m)]}
            for k in SITES for m in MODES
        ],
        "spontaneous_origin_genomes": 0,
        "cyberlife_proven": False,
    }


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("sqlite", type=Path)
    parser.add_argument("--expected-revision", required=True)
    parser.add_argument("--json", type=Path, required=True)
    a = parser.parse_args()
    r = audit(a.sqlite, a.expected_revision)
    a.json.write_text(json.dumps(r, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    print(json.dumps({
        "status": r["status"],
        "original_native_matched_runs": r["original_native_matched_runs"],
        "full_SHA256_epoch_snapshots": r["full_SHA256_epoch_snapshots"],
        "comparison_by_source": r["comparison_by_source"],
    }, sort_keys=True))
