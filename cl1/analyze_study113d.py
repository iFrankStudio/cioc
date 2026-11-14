#!/usr/bin/env python3
"""Study113-D: POST-OUTCOME read-only structure of original 64 unselected CIL variants.

Input is ONLY the already audited original Study113 JSON from CI43612353922.
No new worlds, no filtered genotype denominator, no confirmatory p-values.
"""
import argparse
import json
from collections import Counter, defaultdict
from pathlib import Path

SOURCE = "eee5e9902f3f2eba30a088975b2aa56bf02bca73"
SEEDS = range(6600000, 6600064)
MODES = ("native-edit-source3", "patch-disabled-source3",
         "native-edit-source8", "patch-disabled-source8")

def require(x, message):
    if not x:
        raise ValueError(message)

def region(site):
    if 0 <= site <= 3:
        return "sensor_and_output_0_3"
    if 4 <= site <= 8:
        return "mutator_control_4_8"
    if 9 <= site <= 12:
        return "feedback_9_12"
    if 13 <= site <= 23:
        return "copy_and_reproduction_13_23"
    if 24 <= site <= 27:
        return "delay_24_27"
    raise ValueError(f"unregistered code index {site}")

def audit(path):
    raw = json.loads(path.read_text(encoding="utf-8"))
    require(raw["source_revision"] == SOURCE, "wrong pinned original Study113 source")
    require(raw["matched_worlds"] == 256 and raw["exact_original_SHA256_epochs"] == 4096,
            "not all original Study113 worlds independently SHA256 verified")
    require(raw["initial_distinct_parameter_genomes"] == 64,
            "missing initial same-genome source cohort")
    require(raw["spontaneous_replicator_origins"] == 0 and not raw["cyberlife_proven"],
            "incorrect original science claim")
    rows = {}
    for row in raw["all_unfiltered_original_world_results"]:
        seed = row["seed"]
        mode = row["arm"]
        require(seed in SEEDS and mode in MODES and (seed, mode) not in rows,
                "missing, duplicated or unregistered original seed/arm")
        require(0 <= row["patch_site"] < 28 and 0 <= row["initial_edge_target"] < 28,
                "invalid original full code-site or edge-target")
        rows[(seed, mode)] = row
    require(len(rows) == 256, "missing fixed complete original cohort")

    cohorts = []
    summaries = {}
    by_site = {}
    for source in (3, 8):
        source_rows = []
        aggregate = defaultdict(list)
        aggregate_site = defaultdict(list)
        for seed in SEEDS:
            enabled = rows[(seed, f"native-edit-source{source}")]
            denied = rows[(seed, f"patch-disabled-source{source}")]
            require(enabled["initial_code_hash"] == denied["initial_code_hash"]
                    and enabled["patch_site"] == denied["patch_site"]
                    and enabled["initial_edge_target"] == denied["initial_edge_target"],
                    "initial identical code genotype or mutation target violated")
            delta = enabled["births"]-denied["births"]
            record = {
                "seed": seed, "source": source,
                "site": enabled["patch_site"], "target": enabled["initial_edge_target"],
                "region": region(enabled["patch_site"]),
                "births_enabled": enabled["births"], "births_disabled": denied["births"],
                "birth_delta": delta,
                "actual_edit_events": enabled["changed_native_edit_events"],
                "changed_child_births": enabled["changed_code_births"],
                "expressed_inherited_changed_actors": enabled["expressed_changed_actors"],
                "canonical_source_writers_enabled": enabled["native_function_writers"],
                "canonical_source_writers_disabled": denied["native_function_writers"],
                "invalid_source_outputs_enabled": enabled["invalid_function_outputs"],
                "traps_enabled": enabled["traps"],
                "traps_disabled": denied["traps"],
            }
            source_rows.append(record)
            cohorts.append(record)
            aggregate[record["region"]].append(record)
            aggregate_site[record["site"]].append(record)
        original = raw["comparison_by_source"][f"source{source}"]
        require([r["birth_delta"] for r in source_rows]
                == original["full_64_seed_signed_birth_deltas"],
                "retro breakdown did not reproduce untouched original 64-seed outcome")
        def counts(group):
            return {
                "original_genotypes": len(group),
                "positive": sum(r["birth_delta"] > 0 for r in group),
                "negative": sum(r["birth_delta"] < 0 for r in group),
                "ties": sum(r["birth_delta"] == 0 for r in group),
                "births_enabled": sum(r["births_enabled"] for r in group),
                "births_disabled": sum(r["births_disabled"] for r in group),
                "true_edit_exposure": sum(r["actual_edit_events"] > 0 for r in group),
                "inherited_changed_code_seeds": sum(r["changed_child_births"] > 0 for r in group),
                "executed_inherited_changed_code_seeds": sum(
                    r["expressed_inherited_changed_actors"] > 0 for r in group
                ),
                "trapped_enabled_seeds": sum(r["traps_enabled"] > 0 for r in group),
                "trapped_disabled_seeds": sum(r["traps_disabled"] > 0 for r in group),
                "total_invalid_source_outputs_enabled": sum(
                    r["invalid_source_outputs_enabled"] for r in group
                ),
            }
        totals = counts(source_rows)
        require(all(totals[k] == original[v] for k,v in (
            ("positive","positive"),("negative","negative"),("ties","ties"),
            ("births_enabled","native_enabled_births"),
            ("births_disabled","native_disabled_births"),
            ("true_edit_exposure","actual_edit_exposure_seeds"),
            ("inherited_changed_code_seeds","changed_code_child_seeds"),
            ("executed_inherited_changed_code_seeds","inherited_changed_site_expression_seeds")
        )), "independent retrospective totals differ from pre-existing source report")
        summaries[f"source{source}"] = {
            "all64": totals,
            "full_64_birth_deltas": [r["birth_delta"] for r in source_rows],
            "by_anatomical_code_region_POST_HOC_DESCRIPTIVE": {
                key:counts(value) for key,value in sorted(aggregate.items())
            },
            "by_original_mutation_site_POST_HOC_DESCRIPTIVE": {
                str(key):counts(value) for key,value in sorted(aggregate_site.items())
            },
        }
    require(len(cohorts) == 128, "dependent two-source cohort missing")
    require(len({r["seed"] for r in cohorts}) == 64,
            "source3 and source8 reused same genotype cohort incorrectly")
    return {
        "status":"POST_OUTCOME_DESCRIPTIVE_NATIVE_ALL_SITE_FRAGILITY_NOT_EVOLUTION",
        "source_revision":SOURCE,
        "new_ecological_worlds":0,
        "new_genotype_units":0,
        "original_seed_genotypes":64,
        "original_SHA256_epoch_records_reused":4096,
        "source_dependent_summary":summaries,
        "complete_original_64_by_2_source_genotype_rows":cohorts,
        "cyberlife_proven":False,
    }

def main():
    p=argparse.ArgumentParser()
    p.add_argument("original_audited_json",type=Path)
    p.add_argument("--json",required=True,type=Path)
    a=p.parse_args()
    result=audit(a.original_audited_json)
    a.json.write_text(json.dumps(result,indent=2,sort_keys=True)+"\n",encoding="utf-8")
    print(json.dumps({
        "status":result["status"],
        "original_seed_genotypes":result["original_seed_genotypes"],
        "by_source":{
            s:{
                "totals":report["all64"],
                "code_regions":report["by_anatomical_code_region_POST_HOC_DESCRIPTIVE"],
            } for s,report in result["source_dependent_summary"].items()
        },
    },sort_keys=True))

if __name__=="__main__":
    main()
