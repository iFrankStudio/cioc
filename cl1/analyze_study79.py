#!/usr/bin/env python3
"""Study79: source-locked POST-RESULT native resource-wait mechanism ancestry.

Rerun original independent Stage77 and Stage78 auditors, then describe native
source PC and chronological actual DrawDeferred vs fatal-shortage events.
NO new ecological genomes and no life/organism-repair inference.
"""
from __future__ import annotations

import argparse
import hashlib
import json
import sqlite3
from collections import Counter, defaultdict
from pathlib import Path

import analyze
import analyze_study77
import analyze_study78

ORIGINAL = (
    ("Study77", analyze_study77,
     "79a9bdf65daccc1c83261b04229b2131dd1cd0b0", 5_100_000),
    ("Study78", analyze_study78,
     "cc9f14319fcd9513b559b5f06c804c10410e88a3", 5_200_000),
)
N = 128
EPOCHS = 24
MODES = ("broad-ambient-retry-intact-low",
         "broad-ambient-retry-intact-high",
         "broad-ambient-legacy-intact-low",
         "broad-ambient-legacy-intact-high",
         "short-ambient-retry-intact-low",
         "short-ambient-retry-intact-high",
         "short-ambient-legacy-intact-low",
         "short-ambient-legacy-intact-high")


def name(kind) -> str:
    if isinstance(kind,str):
        return kind
    if isinstance(kind,dict) and len(kind)==1:
        return next(iter(kind))
    raise ValueError("noncanonical native CIL event kind")


def native_events(initial:dict, final:dict) -> list:
    if final["events"][:len(initial["events"])] != initial["events"]:
        raise ValueError("final original source no longer extends loaded events")
    code=initial["processes"]["1"]["code"]["nodes"]
    events=[]
    for e in final["events"][len(initial["events"]):]:
        if e["origin"]=="External":
            if name(e["kind"])!="Intervened":
                raise ValueError("unregistered external actor action")
            continue
        if analyze.internal_process(e)!=1 or e["process"]!=1:
            raise ValueError("native actor attribution forged")
        pc=e["pc"]
        if not isinstance(pc,int) or not 0<=pc<len(code):
            raise ValueError("native event executable PC out of original bounds")
        opcode=name(code[pc])
        ekind=name(e["kind"])
        if ekind=="DrawDeferred":
            d=e["kind"]["DrawDeferred"]
            if opcode!="DrawCompute" or d["requested"]!=8 or d["available"]>=8:
                raise ValueError("native deferral not at valid source Draw shortage")
        if e.get("pool_paid_attempt",False) and opcode!="DrawCompute":
            raise ValueError("shared-pool-paid event not original native Draw opcode")
        if ekind=="Trapped" and opcode=="DrawCompute" and not isinstance(
            e["kind"]["Trapped"],str
        ):
            raise ValueError("native Draw trap message corrupted")
        events.append({
            "tick":e["tick"],"pc":pc,"source_opcode":opcode,
            "kind":ekind,
            "pool_paid_attempt":bool(e.get("pool_paid_attempt",False)),
            "details":e["kind"] if isinstance(e["kind"],dict) else None,
        })
    return events


def event_cause(retry:list, legacy:list) -> dict:
    """Find first native program divergence, not original external event clock."""
    m=min(len(retry),len(legacy))
    i=0
    while i<m and (
        retry[i]["pc"],retry[i]["kind"],retry[i]["pool_paid_attempt"],
        retry[i]["details"]
    ) == (
        legacy[i]["pc"],legacy[i]["kind"],legacy[i]["pool_paid_attempt"],
        legacy[i]["details"]
    ):
        i+=1
    if i==len(retry)==len(legacy):
        return {"kind":"none","ordinal":None}
    ra=retry[i] if i<len(retry) else None
    le=legacy[i] if i<len(legacy) else None
    observed="other"
    if ra and le and ra["pc"]==le["pc"]:
        if ra["kind"]=="DrawDeferred" and le["kind"]=="Trapped":
            if (ra["source_opcode"]=="DrawCompute"
                    and le["source_opcode"]=="DrawCompute"
                    and "insufficient world compute pool" in le["details"]["Trapped"]):
                observed="deferred-instead-of-legacy-fatal-Draw"
        elif (ra["kind"]=="Executed" and le["kind"]=="Trapped"
                and ra["source_opcode"]=="DrawCompute"
                and le["source_opcode"]=="DrawCompute"
                and "insufficient world compute pool" in le["details"]["Trapped"]):
            observed="reserved-fee-success-instead-of-legacy-fatal-Draw"
    if ra is None or le is None:
        observed="one-arm-terminated-or-no-longer-runnable"
    return {
        "kind":observed,
        "ordinal":i+1,
        "retry":ra,
        "legacy":le,
    }


def corpus(path:Path,source:str,first:int,prior:dict) -> dict:
    db=sqlite3.connect(f"{path.resolve().as_uri()}?mode=ro",uri=True)
    try:
        cohort=defaultdict(dict)
        totals=Counter()
        witnesses=[]
        for condition, revision, raw_initial, raw_final in db.execute(
            "SELECT condition,source_revision,initial_json,final_json "
            "FROM epoch_runs ORDER BY id"
        ):
            pre,mode=condition.split("/",1)
            if mode not in MODES:
                continue
            seed=int(pre.removeprefix("seed="))
            if revision!=source or not first<=seed<first+N or mode in cohort[seed]:
                raise ValueError("historical native cohort source or mode mismatch")
            initial,final=json.loads(raw_initial),json.loads(raw_final)
            events=native_events(initial,final)
            cohort[seed][mode]=events
            totals[f"{mode}.native_DrawDeferred_events"]+=sum(
                x["kind"]=="DrawDeferred" for x in events)
            totals[f"{mode}.worlds_with_native_DrawDeferred"]+=int(any(
                x["kind"]=="DrawDeferred" for x in events))
            totals[f"{mode}.native_successful_Draw_events"]+=sum(
                x["source_opcode"]=="DrawCompute" and x["kind"]=="Executed"
                for x in events)
            totals[f"{mode}.native_fatal_Draw_shortage_events"]+=sum(
                x["source_opcode"]=="DrawCompute" and x["kind"]=="Trapped"
                and "insufficient world compute pool" in x["details"]["Trapped"]
                for x in events)
        if len(cohort)!=N or any(set(arms)!=set(MODES) for arms in cohort.values()):
            raise ValueError("historical 128-world native source-cohort missing mode")
        cross=Counter()
        for seed in range(first,first+N):
            original=prior["all_original_seed_outcomes"][str(seed)]
            for topo in ("broad","short"):
                r=original[f"{topo}-retry"]
                old=original[f"{topo}-legacy"]
                new_positive=bool(r["native_W2_W3_source_feedback_and_W3_Draw"])
                old_positive=bool(old["native_W2_W3_source_feedback_and_W3_Draw"])
                if new_positive and not old_positive:
                    cross[f"{topo}.retry_only_primary_worlds"]+=1
                    for sensor in ("low","high"):
                        retry=cohort[seed][f"{topo}-ambient-retry-intact-{sensor}"]
                        legacy=cohort[seed][f"{topo}-ambient-legacy-intact-{sensor}"]
                        cause=event_cause(retry,legacy)
                        cross[f"{topo}.retry_only_{sensor}.first_divergence.{cause['kind']}"]+=1
                        cross[f"{topo}.retry_only_{sensor}.native_DrawDeferred_present"]+=int(
                            any(x["kind"]=="DrawDeferred" for x in retry))
                        cross[f"{topo}.retry_only_{sensor}.legacy_fatal_Draw_present"]+=int(
                            any(x["source_opcode"]=="DrawCompute" and
                                x["kind"]=="Trapped" and
                                "insufficient world compute pool" in x["details"]["Trapped"]
                                for x in legacy))
                        if len(witnesses)<30:
                            witnesses.append({
                                "seed":seed,"topology":topo,"sensor":sensor,
                                "paired_first_native_difference":cause,
                                "all_retry_native_deferrals":[x for x in retry if
                                    x["kind"]=="DrawDeferred"][:8],
                                "all_legacy_Draw_insufficient_traps":[x for x in legacy
                                    if x["source_opcode"]=="DrawCompute" and
                                    x["kind"]=="Trapped" and
                                    "insufficient world compute pool" in x["details"]["Trapped"]][:8],
                            })
        expected=prior["primary_A_W2W3_native_feedback_with_W3_Draw"]
        for topo in ("broad","short"):
            if cross[f"{topo}.retry_only_primary_worlds"] != expected[topo][
                "paired_discordance"].get("retry_only",0):
                raise ValueError("retrospective source event witnesses don't match original primary")
        return {
            "original_independent_seed_genomes_reused":N,
            "source_revision":source,
            "original_original_modes_source_verified":N*len(MODES),
            "native_event_classification_counts":dict(sorted(totals.items())),
            "paired_retry_only_first_event_causes":dict(sorted(cross.items())),
            "first_original_native_ancestry_witnesses":witnesses,
        }
    finally:
        db.close()


def code_fingerprint(source_code:list) -> str:
    return hashlib.sha256(json.dumps(
        source_code,sort_keys=True,separators=(",",":")).encode()).hexdigest()


def audit(paths:list[Path]) ->dict:
    if len(paths)!=len(ORIGINAL):
        raise ValueError("Stage79 requires both original source-pinned native SQLite files")
    reports={}
    source_codes={}
    for (label,analyzer,revision,first),path in zip(ORIGINAL,paths):
        original=analyzer.audit(path,revision)
        if (original["source_revision"]!=revision
                or original["original_independent_seed_genomes"]!=N
                or original["matched_native_runs"]!=2048
                or original["SHA256_verified_original_snapshots"]!=49152):
            raise ValueError("entire prospective original SQLite audit not reproduced")
        reports[label]={
            **corpus(path,revision,first,original),
            "prospective_primary_frozen": original[
                "primary_A_W2W3_native_feedback_with_W3_Draw"],
            "prospective_W3_activity_frozen": original["primary_B_W3_resource_access"],
        }
        for seed in range(first,first+N):
            key=code_fingerprint(analyzer.expected_code(
                seed,"broad-ambient-retry-intact-low"))
            if key in source_codes:
                raise ValueError("same entire initial source code genotype repeated across study cohorts")
            source_codes[key]=(label,seed)
    if len(source_codes)!=2*N:
        raise ValueError("original full CIL source genome fingerprints not distinct")
    return {
        "study":"CL-1 Study79 after-result native resource-wait exact causal ancestry",
        "independent_NEW_genomes":0,
        "original_independent_genomes_REUSED":2*N,
        "original_matched_native_runs_REAUDITED":4096,
        "original_SHA256_epoch_states_REAUDITED":98304,
        "unique_full_initial_broad_CIL_genotypes":len(source_codes),
        "seed_sampling_warning":"distinct hashes do not imply independent PRNG effective samples",
        "cohorts":reports,
        "source_of_nonfatal_resource_retry":"EXTERNAL WORLD LAW",
        "native_self_maintenance_or_repair":"NOT DEMONSTRATED",
        "life_claim":"NONE",
    }


def main() ->None:
    p=argparse.ArgumentParser(description=__doc__)
    p.add_argument("study77",type=Path)
    p.add_argument("study78",type=Path)
    p.add_argument("--json",type=Path,required=True)
    a=p.parse_args()
    result=audit([a.study77,a.study78])
    a.json.write_text(json.dumps(result,indent=2)+"\n",encoding="utf-8")
    print(json.dumps({
        "study":result["study"],
        "new_genomes":0,
        "audited_epochs":result["original_SHA256_epoch_states_REAUDITED"],
        "distinct_genotypes":result["unique_full_initial_broad_CIL_genotypes"],
        "paired_native_divergence":{
            k:v["paired_retry_only_first_event_causes"]
            for k,v in result["cohorts"].items()
        },
        "life_claim":"NONE",
    },indent=2))


if __name__=="__main__":
    main()
