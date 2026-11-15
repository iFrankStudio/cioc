#!/usr/bin/env python3
"""Read-only original Study125 -> terminal Study125-R native shared competition audit.

Never generate new CIL genomes or change host inputs. Check all originals,
all linked terminal states and recompute genuine founder lineage differences.
"""
import argparse
import hashlib
import json
import sqlite3
from pathlib import Path

from analyze_study125 import audit as original_audit, trace, MODES

ORIGINAL="429fd5082693f19b3e3bb25d39b2784e7c8f8d2c"
FIRST=7_000_000
N=32
EPOCHS=8
QUOTA=1000

def require(x,message):
    if not x:
        raise ValueError(message)

def sha(s):
    return hashlib.sha256(s.encode("utf-8")).hexdigest()

def audit(original_path,continuation_path,summary_path,revision):
    prior=original_audit(original_path,ORIGINAL)
    require(prior["original_competitor_pairs"]==32
            and prior["original_native_competition_worlds"]==256
            and prior["original_complete_sha256_epochs"]==4096,
            "original preregistered CIL competitive ecology not fully verified")
    old=db_original=sqlite3.connect(original_path)
    original_rows=old.execute(
        "SELECT condition,initial_json,final_json,final_sha256,attempts_json "
        "FROM epoch_runs"
    ).fetchall()
    require(len(original_rows)==256,"missing original seed world denominator")
    original={r[0]:r[1:] for r in original_rows}
    continued=sqlite3.connect(continuation_path)
    rows=continued.execute(
        "SELECT id,condition,source_revision,initial_json,initial_sha256,"
        "attempts_json,final_json,final_sha256 FROM epoch_runs ORDER BY id"
    ).fetchall()
    require(len(rows)==256,"incomplete original 256 world continuations")
    report=json.loads(summary_path.read_text(encoding="utf-8"))
    require(report["new_genotypes"]==0
            and report["original_competitor_pairs"]==32
            and report["original_worlds"]==256
            and report["continuation_full_sha256_epochs"]==2048
            and report["original_source_revision"]==ORIGINAL
            and report["continuation_source_revision"]==revision,
            "original-vs-terminal runner source/protocol provenance mismatch")
    runner={r["source_label"]:r
            for r in report["full_256_verified_source_native_competitor_state_chains"]}
    require(len(runner)==256,"duplicate original-terminal runner chain")
    full_results={}
    extra_charged_total=0
    rehashed=0
    for run_id,label,rev,initial,initial_sha,attempts_raw,final,final_sha in rows:
        require(rev==revision and len(revision)==40,"incorrect continuation code source")
        require(label.startswith("resumed/seed="),"unexpected resumed original competitor label")
        old_label=label.removeprefix("resumed/")
        require(old_label in original and old_label in runner
                and old_label not in full_results,
                "original world replaced, missing or duplicated")
        original_initial,original_final,original_sha,original_attempts=original[old_label]
        require(initial==original_final and initial_sha==original_sha
                and sha(initial)==initial_sha and sha(final)==final_sha,
                "original full-state SHA256 not exactly linked to continuation")
        seedpart,mode=old_label.removeprefix("seed=").split("/",1)
        seed=int(seedpart)
        require(seed in range(FIRST,FIRST+N) and mode in MODES,
                "invalid complete original ancestor seed or control arm")
        attempts=json.loads(attempts_raw)
        require(len(attempts)==EPOCHS and attempts[-1]==0
                and all(0<=x<=QUOTA for x in attempts),
                "continuation did not extend original to real native terminal")
        extra_paid=sum(attempts)
        original_paid=sum(json.loads(original_attempts))
        require(original_paid+extra_paid<=7800,
                "native compute expenditure exceeds supplied environmental credits")
        final_state=json.loads(final)
        require(final_state["compute_pool"]+sum(
            p["credits"] for p in final_state["processes"].values())
            ==7800-original_paid-extra_paid,
            "native terminal actor resource + shared pool not conserved")
        require(sum(e["origin"]=="External" for e in final_state["events"])==20,
                "post-outcome host intervention or ambient resource contamination")
        require(sum(isinstance(e["origin"],dict) for e in final_state["events"])
                ==original_paid+extra_paid,
                "true native charged continuation event ledger mismatch")
        for e in final_state["events"]:
            if isinstance(e["origin"],dict):
                require(e["origin"]=={"Internal":e["process"]},
                        "actor-native original continuation event not source-attributed")
        entries=continued.execute(
            "SELECT epoch,state_sha256,state_json FROM epoch_snapshots "
            "WHERE run_id=? ORDER BY epoch",(run_id,)
        ).fetchall()
        require(len(entries)==EPOCHS,"missing original linked continuation epoch")
        for i,(epoch,digest,raw) in enumerate(entries):
            require(epoch==i and sha(raw)==digest,
                    "continuation full native deterministic epoch SHA256 invalid")
            rehashed+=1
        require(entries[-1][2]==final,"continuation final not last audited state")
        original_at_cutoff=trace(json.loads(original_initial),
                                 json.loads(original_final),mode,original_paid)
        terminal=trace(json.loads(original_initial),final_state,mode,
                       original_paid+extra_paid)
        for founder in ("A","B"):
            key=f"original_native_births_{founder}"
            require(terminal[key]>=original_at_cutoff[key],
                    "original native descendants disappeared from terminal world")
        by_rust=runner[old_label]
        require(by_rust["original_final_sha256"]==original_sha
                and by_rust["continuation_initial_sha256"]==initial_sha
                and by_rust["continuation_final_sha256"]==final_sha
                and by_rust["additional_native_paid_attempts"]==extra_paid
                and by_rust["original_native_paid_attempts"]==original_paid
                and all(
                    by_rust[f"births_{founder}_original"]==
                        original_at_cutoff[f"original_native_births_{founder}"]
                    and by_rust[f"births_{founder}_terminal"]==
                        terminal[f"original_native_births_{founder}"]
                    for founder in ("A","B")),
                "independent original-to-terminal native lineage birth count mismatch")
        full_results[old_label]={
            "seed":seed,"arm":mode,"original_paid":original_paid,
            "extra_paid":extra_paid,
            "original":original_at_cutoff,
            "terminal":terminal,
            "original_sha256":original_sha,
            "terminal_sha256":final_sha,
        }
        extra_charged_total+=extra_paid
    require(len(full_results)==256 and rehashed==2048,
            "original full 256-world continuation denominator not verified")
    require(extra_charged_total==report["additional_original_native_paid_attempts"],
            "independent original terminal credit expenditure mismatch")
    comparisons=[]
    rust_pairs=report["all_128_original_vs_terminal_source_order_edit_interactions"]
    require(len(rust_pairs)==128,"runner original/terminal pair controls missing")
    original_pairs={
        (r["seed"],r["source"],r["order"]):r
        for r in prior["full_128_original_source_order_paired_editing_interactions"]
    }
    require(len(original_pairs)==128,"registered original competition vectors missing")
    for seed in range(FIRST,FIRST+N):
        for source in (3,8):
            for order in ("Afirst","Bfirst"):
                yes=full_results[f"seed={seed}/native-edit-source{source}-{order}"]
                no=full_results[f"seed={seed}/patch-disabled-source{source}-{order}"]
                def diff(world):
                    return world["original_native_births_A"]-world["original_native_births_B"]
                original_diff=diff(yes["original"])-diff(no["original"])
                terminal_diff=diff(yes["terminal"])-diff(no["terminal"])
                baseline=original_pairs[(seed,source,order)]
                require(original_diff==baseline["editing_interaction_difference"],
                        "independent original editing-effect comparison changed")
                saved=next(r for r in rust_pairs if r["seed"]==seed
                           and r["source"]==source and r["order"]==order)
                require(saved["original_edit_interaction"]==original_diff
                        and saved["terminal_edit_interaction"]==terminal_diff,
                        "terminal Rust vs Python source-lineage sign mismatch")
                comparisons.append({
                    "seed":seed,"source":source,"order":order,
                    "original_edit_interaction":original_diff,
                    "terminal_edit_interaction":terminal_diff,
                    "original_mutation_positive":original_diff>0,
                    "terminal_mutation_positive":terminal_diff>0,
                    "terminal_mutation_negative":terminal_diff<0,
                    "terminal_mutation_tie":terminal_diff==0,
                    "original_rank_reversal":baseline["rank_reversal"],
                })
    stats={}
    for source in (3,8):
        for order in ("Afirst","Bfirst"):
            group=[x for x in comparisons
                   if x["source"]==source and x["order"]==order]
            require(len(group)==32
                    and sum(x["original_edit_interaction"]>0 for x in group)==6
                    and sum(x["original_edit_interaction"]<0 for x in group)==21,
                    "original Study125 6:21:5 preregistered cohort altered")
            stats[f"source{source}-{order}"]={
                "original_positive":6,"original_negative":21,"original_ties":5,
                "terminal_positive":sum(x["terminal_edit_interaction"]>0 for x in group),
                "terminal_negative":sum(x["terminal_edit_interaction"]<0 for x in group),
                "terminal_ties":sum(x["terminal_edit_interaction"]==0 for x in group),
                "original_positive_retained":sum(
                    x["original_edit_interaction"]>0
                    and x["terminal_edit_interaction"]>0 for x in group),
                "original_positive_lost_to_tie":sum(
                    x["original_edit_interaction"]>0
                    and x["terminal_edit_interaction"]==0 for x in group),
                "original_positive_reversed_negative":sum(
                    x["original_edit_interaction"]>0
                    and x["terminal_edit_interaction"]<0 for x in group),
            }
    return {
        "status":"VERIFIED_RETROSPECTIVE_ORIGINAL_256_SAMEWORLD_NATIVE_TERMINAL_NOT_INDEPENDENT_GENOTYPES",
        "original_revision":ORIGINAL,
        "continuation_revision":revision,
        "new_genotypes":0,"original_pair_genotypes":32,
        "original_full_epoch_sha256_reverified":4096,
        "original_worlds_linked":256,
        "continuation_full_epoch_sha256_reverified":rehashed,
        "additional_native_paid_attempts":extra_charged_total,
        "source_order_terminal_competition_effects":stats,
        "complete_original_128_source_order_competitor_comparisons":comparisons,
        "full_256_original_world_terminal_lineages":[
            full_results[f"seed={seed}/{mode}"]
            for seed in range(FIRST,FIRST+N) for mode in MODES
        ],
        "cyberlife_or_open_ended_evolution_proven":False,
    }

def main():
    p=argparse.ArgumentParser()
    p.add_argument("original",type=Path)
    p.add_argument("continuation",type=Path)
    p.add_argument("rust_summary",type=Path)
    p.add_argument("--expected-revision",required=True)
    p.add_argument("--json",type=Path,required=True)
    a=p.parse_args()
    result=audit(a.original,a.continuation,a.rust_summary,a.expected_revision)
    a.json.write_text(json.dumps(result,sort_keys=True,indent=2)+"\n")
    print(json.dumps({
        "status":result["status"],
        "new_genotypes":result["new_genotypes"],
        "original_pair_genotypes":result["original_pair_genotypes"],
        "original_worlds_linked":result["original_worlds_linked"],
        "continuation_full_epoch_sha256_reverified":
            result["continuation_full_epoch_sha256_reverified"],
        "additional_native_paid_attempts":result["additional_native_paid_attempts"],
        "source_order_terminal_competition_effects":
            result["source_order_terminal_competition_effects"],
    },sort_keys=True))

if __name__=="__main__":
    main()
