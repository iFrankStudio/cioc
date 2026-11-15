//! Study125-R: POST-OUTCOME EXACT original shared-world CIL competitor continuation.
//! Zero new code genotypes; 256 original exact full World terminal states re-run
//! to scheduler ineligibility with absolutely no further host intervention.
use cl0_lab::{hash_bytes, EpochCondition, Laboratory};
use cl0_world::{EventKind, Origin, World};
use rusqlite::{Connection, OpenFlags};
use serde_json::json;
use std::{collections::BTreeMap, env, error::Error, fs};

const ORIGINAL_REVISION:&str="429fd5082693f19b3e3bb25d39b2784e7c8f8d2c";
const FIRST_SEED:u64=7_000_000;
const GENOTYPES:u64=32;
const EPOCHS:usize=8;
const QUOTA:usize=1000;
const MODES:[&str;8]=[
    "native-edit-source3-Afirst","patch-disabled-source3-Afirst",
    "native-edit-source8-Afirst","patch-disabled-source8-Afirst",
    "native-edit-source3-Bfirst","patch-disabled-source3-Bfirst",
    "native-edit-source8-Bfirst","patch-disabled-source8-Bfirst",
];

fn lineage_births(world:&World,a_first:bool)->Result<(usize,usize),Box<dyn Error>> {
    let mut owner=BTreeMap::<u64,bool>::new();
    owner.insert(1,a_first);
    owner.insert(2,!a_first);
    let mut a=0;
    let mut b=0;
    for event in world.events() {
        if let EventKind::GraphLaunched{parent_process,child_process,..}=event.kind {
            if event.origin != Origin::Internal(parent_process) {
                return Err("native birth not attributed to matching actor".into());
            }
            let from_a=*owner.get(&parent_process)
                .ok_or("native child launch has unknown ancestral founder")?;
            if owner.insert(child_process,from_a).is_some() {
                return Err("native child reused process identifier".into());
            }
            if from_a {a+=1;} else {b+=1;}
        }
    }
    Ok((a,b))
}

fn main()->Result<(),Box<dyn Error>> {
    let args:Vec<_>=env::args().collect();
    if args.len()!=4 {
        return Err("usage: cl1-study125-r original.sqlite continuation.sqlite report.json".into());
    }
    let continuation_revision=env::var("CL1_SOURCE_REVISION")
        .unwrap_or_else(|_|"unverified-local-checkout".into());
    let input=Connection::open_with_flags(&args[1],OpenFlags::SQLITE_OPEN_READ_ONLY)?;
    let mut lab=Laboratory::open(&args[2])?;
    let mut stmt=input.prepare("SELECT condition,source_revision,final_sha256,final_json,attempts_json FROM epoch_runs ORDER BY id")?;
    let entries=stmt.query_map([],|row| Ok((
        row.get::<_,String>(0)?,row.get::<_,String>(1)?,
        row.get::<_,String>(2)?,row.get::<_,String>(3)?,
        row.get::<_,String>(4)?
    )))?;
    let mut all=Vec::new();
    let mut by_pair=BTreeMap::<(u64,String),((usize,usize),(usize,usize))>::new();
    let mut total_extra=0_usize;
    for row in entries {
        let (label,revision,original_sha,original_json,attempt_json)=row?;
        if revision!=ORIGINAL_REVISION
            || hash_bytes(original_json.as_bytes())!=original_sha {
            return Err(format!("invalid immutable original Study125 source/hash: {label}").into());
        }
        let (seedpart,mode)=label.strip_prefix("seed=")
            .ok_or("invalid registered original label")?
            .split_once('/').ok_or("invalid original seed/condition")?;
        let seed:u64=seedpart.parse()?;
        if !(FIRST_SEED..FIRST_SEED+GENOTYPES).contains(&seed)
            || !MODES.contains(&mode) {
            return Err(format!("unregistered original competitor world: {label}").into());
        }
        let earlier:Vec<usize>=serde_json::from_str(&attempt_json)?;
        if earlier.len()!=16 || earlier.iter().any(|x|*x>250) {
            return Err(format!("original Study125 incomplete 16 epochs: {label}").into());
        }
        let charged_original:usize=earlier.iter().sum();
        let original:World=serde_json::from_str(&original_json)?;
        let original_remaining=original.total_compute_inventory()
            .ok_or("original native compute inventory overflow")?;
        if original_remaining!=7800-charged_original as u64 {
            return Err(format!("original shared compute ledger mismatch: {label}").into());
        }
        let a_first=mode.ends_with("Afirst");
        let before=lineage_births(&original,a_first)?;
        let continuation=lab.run_epochs(
            &original,
            &[EpochCondition{
                label:format!("resumed/{label}"),
                actions:vec![],
            }],
            EPOCHS,QUOTA,&continuation_revision,
        )?;
        let result=continuation.first().ok_or("continuation original run missing")?;
        if result.initial_sha256 != original_sha || !lab.verify_epoch_replay(result.id)? {
            return Err(format!("continuation did not link original full SHA256: {label}").into());
        }
        let paid:usize=result.attempts_by_epoch.iter().sum();
        if paid as u64>original_remaining
            || result.attempts_by_epoch.last()!=Some(&0) {
            return Err(format!("original CIL world not naturally terminal: {label}").into());
        }
        let mut probe=result.world.clone();
        if probe.step() {
            return Err(format!("CIL actor remains schedulable after continuation: {label}").into());
        }
        if result.world.total_compute_inventory()!=Some(original_remaining-paid as u64)
            || result.world.events().iter()
                .filter(|e|e.origin==Origin::External).count()!=20 {
            return Err(format!("terminal CIL credits or host origin conservation failed: {label}").into());
        }
        let after=lineage_births(&result.world,a_first)?;
        if after.0<before.0||after.1<before.1
            || by_pair.insert((seed,mode.into()),(before,after)).is_some() {
            return Err(format!("native original founder ancestry disappeared: {label}").into());
        }
        total_extra+=paid;
        all.push(json!({
            "source_label":label,"seed":seed,"mode":mode,
            "original_final_sha256":original_sha,
            "continuation_initial_sha256":result.initial_sha256,
            "continuation_final_sha256":result.final_sha256,
            "births_A_original":before.0,"births_B_original":before.1,
            "births_A_terminal":after.0,"births_B_terminal":after.1,
            "original_native_paid_attempts":charged_original,
            "additional_native_paid_attempts":paid,
            "terminal_native_inventory":result.world.total_compute_inventory(),
        }));
    }
    if by_pair.len()!=256 {return Err("incomplete immutable 256 shared competitor worlds".into());}
    let mut compared=Vec::new();
    for seed in FIRST_SEED..FIRST_SEED+GENOTYPES {
        for source in [3,8] {
            for order in ["Afirst","Bfirst"] {
                let yes=by_pair.get(&(seed,format!("native-edit-source{source}-{order}")))
                    .ok_or("missing original native-edit competing world")?;
                let no=by_pair.get(&(seed,format!("patch-disabled-source{source}-{order}")))
                    .ok_or("missing original suppressed-edit competing world")?;
                let difference=|state:(usize,usize)| state.0 as i64-state.1 as i64;
                let previous=difference(yes.0)-difference(no.0);
                let terminal=difference(yes.1)-difference(no.1);
                compared.push(json!({
                    "seed":seed,"source":source,"order":order,
                    "original_edit_interaction":previous,
                    "terminal_edit_interaction":terminal,
                    "terminal_enabled_births_A":yes.1.0,
                    "terminal_enabled_births_B":yes.1.1,
                    "terminal_disabled_births_A":no.1.0,
                    "terminal_disabled_births_B":no.1.1,
                }));
            }
        }
    }
    if compared.len()!=128 {return Err("not all original source/order comparisons".into());}
    for source in [3,8] {
        for order in ["Afirst","Bfirst"] {
            let subgroup:Vec<_>=compared.iter().filter(|x|
                x["source"].as_i64()==Some(source)
                && x["order"].as_str()==Some(order)).collect();
            let npos=subgroup.iter().filter(|x|
                x["original_edit_interaction"].as_i64().unwrap()>0).count();
            let nneg=subgroup.iter().filter(|x|
                x["original_edit_interaction"].as_i64().unwrap()<0).count();
            if subgroup.len()!=32 || npos!=6 || nneg!=21 {
                return Err("original Study125 positive/negative denominator altered".into());
            }
        }
    }
    let report=json!({
        "status":"POST_OUTCOME_ALL256_EXACT_ORIGINAL_SHARED_COMPETITOR_STATES_TO_NATIVE_TERMINAL",
        "original_source_revision":ORIGINAL_REVISION,
        "continuation_source_revision":continuation_revision,
        "new_genotypes":0,
        "original_competitor_pairs":32,
        "original_worlds":256,
        "continuation_full_sha256_epochs":256*EPOCHS,
        "additional_original_native_paid_attempts":total_extra,
        "full_256_verified_source_native_competitor_state_chains":all,
        "all_128_original_vs_terminal_source_order_edit_interactions":compared,
        "spontaneous_life_or_open_ended_evolution_proven":false,
    });
    fs::write(&args[3],serde_json::to_vec_pretty(&report)?)?;
    println!("Study125-R original exact-world terminal: old_worlds=256 extra_epoch_hashes={} extra_paid_attempts={total_extra}, no_new_genotypes=1 [POST_OUTCOME]",256*EPOCHS);
    for source in [3,8] {for order in ["Afirst","Bfirst"] {
        let group:Vec<_>=compared.iter().filter(|x|x["source"].as_i64()==Some(source)
            &&x["order"].as_str()==Some(order)).collect();
        let terminal=(group.iter().filter(|x|x["terminal_edit_interaction"].as_i64().unwrap()>0).count(),
                      group.iter().filter(|x|x["terminal_edit_interaction"].as_i64().unwrap()<0).count(),
                      group.iter().filter(|x|x["terminal_edit_interaction"].as_i64().unwrap()==0).count());
        println!("Study125-R source{source}-{order} exact terminal native editing interaction: positive={} negative={} ties={} [original6_21_5; NOT_INDEPENDENT]",terminal.0,terminal.1,terminal.2);
    }}
    Ok(())
}
