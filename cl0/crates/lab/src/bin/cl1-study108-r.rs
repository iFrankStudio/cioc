//! CL-1 Study108-R: POST-OUTCOME exact original Study108 world-state continuation.
//! Never regenerate seed genomes; zero new samples, no host interventions.
use cl0_lab::{hash_bytes, EpochCondition, Laboratory};
use cl0_world::{EventKind, Origin, World};
use rusqlite::{Connection, OpenFlags};
use serde_json::json;
use std::{collections::BTreeMap, env, error::Error, fs};

const ORIGINAL: &str = "406bb3e1160987113671eb9ffdf517c3edf4ad51";
const EPOCHS: usize = 4;
const QUOTA: usize = 500;

fn births(world: &World) -> usize {
    world.events().iter()
        .filter(|e| {
            e.origin == Origin::Internal(e.process)
                && matches!(e.kind, EventKind::GraphLaunched { .. })
        })
        .count()
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<_> = env::args().collect();
    if args.len() != 4 {
        return Err("usage: cl1-study108-r original.sqlite continuation.sqlite summary.json".into());
    }
    let source_revision = env::var("CL1_SOURCE_REVISION")
        .unwrap_or_else(|_| "unverified-local-checkout".into());
    let conn = Connection::open_with_flags(&args[1], OpenFlags::SQLITE_OPEN_READ_ONLY)?;
    let mut lab = Laboratory::open(&args[2])?;
    let mut statement = conn.prepare(
        "SELECT condition,source_revision,final_sha256,final_json,attempts_json          FROM epoch_runs ORDER BY id"
    )?;
    let raw = statement.query_map([], |r| {
        Ok((
            r.get::<_, String>(0)?,
            r.get::<_, String>(1)?,
            r.get::<_, String>(2)?,
            r.get::<_, String>(3)?,
            r.get::<_, String>(4)?,
        ))
    })?;
    let mut by_world: BTreeMap<(u64, String), (usize, usize)> = BTreeMap::new();
    let mut all = Vec::new();
    let mut charged_added = 0;
    for item in raw {
        let (label, revision, original_sha, original_json, attempt_json) = item?;
        if revision != ORIGINAL || hash_bytes(original_json.as_bytes()) != original_sha {
            return Err(format!("original Study108 source/SHA mismatch: {label}").into());
        }
        let (seed_part, mode) = label.strip_prefix("seed=").ok_or("incorrect original label")?
            .split_once('/').ok_or("incorrect original condition")?;
        let seed: u64 = seed_part.parse()?;
        if !(6_200_000..6_200_032).contains(&seed)
            || !["native-edit-source3", "patch-disabled-source3",
                 "native-edit-source8", "patch-disabled-source8"].contains(&mode)
        {
            return Err(format!("unregistered original state: {label}").into());
        }
        let prior_epochs: Vec<usize> = serde_json::from_str(&attempt_json)?;
        if prior_epochs != vec![250; 8] {
            return Err(format!("original run had not reached exact 2000 attempts: {label}").into());
        }
        let original: World = serde_json::from_str(&original_json)?;
        if original.total_compute_inventory() != Some(1400) {
            return Err(format!("wrong original remaining credit inventory: {label}").into());
        }
        let before = births(&original);
        let results = lab.run_epochs(
            &original,
            &[EpochCondition {
                label: format!("resumed/{label}"),
                actions: vec![],
            }],
            EPOCHS,
            QUOTA,
            &source_revision,
        )?;
        let run = results.first().ok_or("missing continuation")?;
        if run.initial_sha256 != original_sha || !lab.verify_epoch_replay(run.id)? {
            return Err(format!("Study108-R original-SHA linked continuation failed: {label}").into());
        }
        let charged: usize = run.attempts_by_epoch.iter().sum();
        if charged > 1400 || run.attempts_by_epoch.last() != Some(&0) {
            return Err(format!("not naturally terminal before final recorded epoch: {label}").into());
        }
        let mut probe = run.world.clone();
        if probe.step() {
            return Err(format!("resource-censored actor remains runnable: {label}").into());
        }
        if run.world.total_compute_inventory() != Some(1400 - charged as u64)
            || run.world.events().iter().filter(|e| e.origin == Origin::External).count() != 5
        {
            return Err(format!("continued resource/host conservation failed: {label}").into());
        }
        let after = births(&run.world);
        if after < before
            || by_world.insert((seed, mode.to_owned()), (before, after)).is_some()
        {
            return Err(format!("invalid/duplicated original child genealogy: {label}").into());
        }
        charged_added += charged;
        all.push(json!({
            "source_label": label, "seed": seed, "mode": mode,
            "original_final_sha256": original_sha,
            "continuation_initial_sha256": run.initial_sha256,
            "continued_final_sha256": run.final_sha256,
            "prior_native_births": before,
            "terminal_native_births": after,
            "additional_charged_native_attempts": charged,
            "final_native_inventory": run.world.total_compute_inventory(),
        }));
    }
    if by_world.len() != 128 {
        return Err("not all original 128 Study108 worlds present".into());
    }
    let mut pairs = Vec::new();
    for seed in 6_200_000..6_200_032 {
        for source in [3, 8] {
            let enabled = by_world.get(&(seed, format!("native-edit-source{source}")))
                .ok_or("missing enabled original run")?;
            let disabled = by_world.get(&(seed, format!("patch-disabled-source{source}")))
                .ok_or("missing disabled original run")?;
            pairs.push(json!({
                "seed": seed, "source": source,
                "original_fixed_2000_birth_delta": enabled.0 as i64 - disabled.0 as i64,
                "terminal_birth_delta": enabled.1 as i64 - disabled.1 as i64,
                "original_enabled_births": enabled.0,
                "original_disabled_births": disabled.0,
                "terminal_enabled_births": enabled.1,
                "terminal_disabled_births": disabled.1,
            }));
        }
    }
    if pairs.iter().filter(|r| r["original_fixed_2000_birth_delta"].as_i64() == Some(1)).count() != 2 {
        return Err("original Study108 +1 original outcome no longer matches archive".into());
    }
    let winner: Vec<_> = pairs.iter().filter(|r| r["seed"].as_u64() == Some(6_200_017)).cloned().collect();
    fs::write(&args[3], serde_json::to_vec_pretty(&json!({
        "status": "POST_OUTCOME_RETROSPECTIVE_NO_NEW_CIL_GENOMES",
        "original_revision": ORIGINAL,
        "continuation_source_revision": source_revision,
        "new_genotype_units": 0,
        "original_worlds": 128,
        "new_exact_replayed_epochs": 512,
        "additional_charged_native_attempts": charged_added,
        "all64_original_paired_birth_differences_at_terminal": pairs,
        "previously_known_positive_seed6200017": winner,
        "all128_verified_world_chains": all,
    }))?)?;
    println!("Study108-R original-world continuation verified: old_worlds=128 new_genotypes=0 continuation_epochs=512 extra_attempts={charged_added} previous_seed6200017_terminal={winner:?} [RETROSPECTIVE NOT ADAPTATION]");
    Ok(())
}
