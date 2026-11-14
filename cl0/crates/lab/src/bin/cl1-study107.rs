//! Study107: PRE-REGISTERED fixed 32 CIL chassis genotypes, 4 same-genome arms.
//! Every entire self-replication/mutator strategy is human-designed, NOT life genesis.
use cl0_lab::{hash_bytes, EpochCondition, Laboratory};
use cl0_world::{Intervention, Op, Process, World};
use std::{collections::BTreeSet, env, error::Error};

const SITES: usize = 28;
const EPOCHS: usize = 8;
const QUOTA: usize = 250;
const MODES: [&str; 4] = [
    "native-edit-source3",
    "patch-disabled-source3",
    "native-edit-source8",
    "patch-disabled-source8",
];

fn genome(site: usize) -> Vec<Op> {
    assert!(site < SITES);
    let delay = 20;
    let grant = 400;
    let nodes = vec![
        Op::Const { dst: 2, value: 7, next: 1 },
        Op::Const { dst: 3, value: 9, next: 2 },
        Op::Read { dst: 0, address: 2, next: 3 },
        Op::Write { address: 3, value: 0, next: 4 },
        Op::Const { dst: 2, value: 1, next: 5 },
        Op::Const { dst: 2, value: 11, next: 6 },
        Op::Read { dst: 0, address: 2, next: 7 },
        Op::Const { dst: 1, value: site as i64, next: 8 },
        Op::PatchNext { node: 1, target: 0, next: 9 },
        Op::Const { dst: 1, value: 29, next: 10 },
        Op::Sub { dst: 1, lhs: 1, rhs: 0, next: 11 },
        Op::Write { address: 2, value: 1, next: 12 },
        Op::Const { dst: 2, value: 1, next: 13 },
        Op::Const { dst: 1, value: 0, next: 14 },
        Op::Const { dst: 3, value: 28, next: 15 },
        Op::GraphNew { dst: 0, next: 16 },
        Op::GraphCopyNode { graph: 0, source: 1, dst: 1, next: 17 },
        Op::Add { dst: 1, lhs: 1, rhs: 2, next: 18 },
        Op::Sub { dst: 3, lhs: 3, rhs: 2, next: 19 },
        Op::Branch { test: 3, nonzero: 16, zero: 20 },
        Op::Const { dst: 2, value: grant as i64, next: 21 },
        Op::DrawCompute { amount: 2, next: 22 },
        Op::GraphLaunch { graph: 0, credits: 2, dst: 3, next: 23 },
        Op::Const { dst: 1, value: 0, next: 4 },
        Op::Const { dst: 1, value: delay as i64, next: 25 },
        Op::Const { dst: 2, value: 1, next: 26 },
        Op::Sub { dst: 1, lhs: 1, rhs: 2, next: 27 },
        Op::Branch { test: 1, nonzero: 26, zero: 5 },
    ];
    assert_eq!(nodes.len(), 28);
    nodes
}

fn initial_world(code: Vec<Op>, mode: &str) -> Result<World, Box<dyn Error>> {
    if !MODES.contains(&mode) {
        return Err("unregistered Study107 world arm".into());
    }
    let mut world = World::with_strict_capacity(4, 64, 1024)
        .with_pool_paid_draw()
        .with_recoverable_draw_shortage()
        .with_native_executable_edit_trace();
    if mode.starts_with("patch-disabled-") {
        world = world.with_native_patchnext_disabled();
    }
    world.load(Process::new(1, code.clone(), 500))?;
    world.load(Process::new(2, code, 500))?;
    let source = if mode.ends_with("source3") { 3 } else { 8 };
    world.intervene(Intervention::SetCell { key: 7, value: source })?;
    world.intervene(Intervention::SetCell { key: 11, value: 5 })?;
    world.intervene(Intervention::SetComputePool { credits: 2400 })?;
    Ok(world)
}

fn main() -> Result<(), Box<dyn Error>> {
    let path = env::args().nth(1)
        .unwrap_or_else(|| "cl1-study107-native-mutation-only-causal-ecology.sqlite".into());
    let revision = env::var("CL1_SOURCE_REVISION")
        .unwrap_or_else(|_| "unverified-local-checkout".into());
    let mut lab = Laboratory::open(&path)?;
    let mut distinct = BTreeSet::new();
    let mut runs = 0_usize;
    let mut snapshots = 0_usize;
    let mut total_attempts = 0_usize;
    for site in 0..SITES {
        let source_code = genome(site);
        distinct.insert(hash_bytes(serde_json::to_string(&source_code)?.as_bytes()));
        for mode in MODES {
            let world = initial_world(source_code.clone(), mode)?;
            if world.process(1).unwrap().code != world.process(2).unwrap().code {
                return Err(format!("nonidentical initial parent genotype: {site}/{mode}").into());
            }
            let mut predicted = world.clone();
            for _ in 0..EPOCHS {
                predicted.run(QUOTA);
            }
            let outcome = lab.run_epochs(
                &world,
                &[EpochCondition {
                    label: format!("site={site}/{mode}"),
                    actions: vec![],
                }],
                EPOCHS,
                QUOTA,
                &revision,
            )?;
            let run = outcome.first().ok_or("missing original Study107 CIL run")?;
            if run.world != predicted || !lab.verify_epoch_replay(run.id)? {
                return Err(format!("full original SHA256 epoch replay mismatch: {site}/{mode}").into());
            }
            let attempts: usize = run.attempts_by_epoch.iter().sum();
            if run.world.total_compute_inventory() != Some(3400 - attempts as u64) {
                return Err(format!("total native compute inventory mismatch: {site}/{mode}").into());
            }
            if run.attempts_by_epoch.len() != EPOCHS {
                return Err("missing registered CIL epoch".into());
            }
            runs += 1;
            snapshots += EPOCHS;
            total_attempts += attempts;
        }
    }
    if runs != (SITES) * MODES.len() || snapshots != runs * EPOCHS {
        return Err("incomplete registered 28-site four-arm systematic census".into());
    }
    println!("Study107 ORIGINAL complete: exhaustive_target_sites={SITES} distinct_initial_site_code_hashes={} native_world_runs={runs} fully_replayed_SHA256_epochs={snapshots} charged_attempts={total_attempts} [HUMAN-DESIGNED CHASSIS, NOT LIFE]; {path}", distinct.len());
    Ok(())
}
