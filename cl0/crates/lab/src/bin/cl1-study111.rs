//! Study111: PRE-REGISTERED fixed 32 CIL chassis genotypes, 4 same-genome arms.
//! Every entire self-replication/mutator strategy is human-designed, NOT life genesis.
use cl0_lab::{hash_bytes, EpochAction, EpochCondition, Laboratory};
use cl0_world::{Intervention, Op, Process, World};
use std::{collections::BTreeSet, env, error::Error};

const FIRST_SEED: u64 = 6_400_000;
const GENOTYPES: u64 = 32;
const EPOCHS: usize = 16;
const QUOTA: usize = 250;
const MODES: [&str; 4] = [
    "native-edit-source3",
    "patch-disabled-source3",
    "native-edit-source8",
    "patch-disabled-source8",
];

fn splitmix64(state: &mut u64) -> u64 {
    *state = state.wrapping_add(0x9e3779b97f4a7c15);
    let mut z = *state;
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
    z ^ (z >> 31)
}

fn genome(seed: u64) -> Vec<Op> {
    let mut state = seed;
    let delay = 1 + splitmix64(&mut state) % 32;
    let grant = 320 + splitmix64(&mut state) % 161;
    let nodes = vec![
        Op::Const { dst: 2, value: 7, next: 1 },
        Op::Const { dst: 3, value: 9, next: 2 },
        Op::Read { dst: 0, address: 2, next: 3 },
        Op::Write { address: 3, value: 0, next: 4 },
        Op::Const { dst: 2, value: 1, next: 24 },
        Op::Const { dst: 2, value: 11, next: 6 },
        Op::Read { dst: 0, address: 2, next: 7 },
        Op::Const { dst: 1, value: 4, next: 8 },
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
        return Err("unregistered Study111 world arm".into());
    }
    let mut world = World::with_strict_capacity(4, 64, 2048)
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
    world.intervene(Intervention::SetComputePool { credits: 800 })?;
    Ok(world)
}

fn main() -> Result<(), Box<dyn Error>> {
    let path = env::args().nth(1)
        .unwrap_or_else(|| "cl1-study111-native-mutation-only-causal-ecology.sqlite".into());
    let revision = env::var("CL1_SOURCE_REVISION")
        .unwrap_or_else(|_| "unverified-local-checkout".into());
    let mut lab = Laboratory::open(&path)?;
    let mut distinct = BTreeSet::new();
    let mut runs = 0_usize;
    let mut snapshots = 0_usize;
    let mut total_attempts = 0_usize;
    for seed in FIRST_SEED..FIRST_SEED + GENOTYPES {
        let source_code = genome(seed);
        distinct.insert(hash_bytes(serde_json::to_string(&source_code)?.as_bytes()));
        for mode in MODES {
            let world = initial_world(source_code.clone(), mode)?;
            if world.process(1).unwrap().code != world.process(2).unwrap().code {
                return Err(format!("nonidentical initial parent genotype: {seed}/{mode}").into());
            }
            let mut predicted = world.clone();
            let mut ambient_actions = Vec::new();
            for epoch in 0..EPOCHS {
                if epoch > 0 {
                    let credits = predicted.available_compute_pool()
                        .checked_add(400)
                        .ok_or("prespecified ambient compute input overflow")?;
                    let action = EpochAction {
                        epoch,
                        action: Intervention::SetComputePool { credits },
                    };
                    predicted.intervene(action.action.clone())?;
                    ambient_actions.push(action);
                }
                predicted.run(QUOTA);
            }
            let outcome = lab.run_epochs(
                &world,
                &[EpochCondition {
                    label: format!("seed={seed}/{mode}"),
                    actions: ambient_actions,
                }],
                EPOCHS,
                QUOTA,
                &revision,
            )?;
            let run = outcome.first().ok_or("missing original Study111 CIL run")?;
            if run.world != predicted || !lab.verify_epoch_replay(run.id)? {
                return Err(format!("full original SHA256 epoch replay mismatch: {seed}/{mode}").into());
            }
            let attempts: usize = run.attempts_by_epoch.iter().sum();
            if run.world.total_compute_inventory() != Some(7800 - attempts as u64) {
                return Err(format!("total native compute inventory mismatch: {seed}/{mode}").into());
            }
            if run.attempts_by_epoch.len() != EPOCHS {
                return Err("missing registered CIL epoch".into());
            }
            runs += 1;
            snapshots += EPOCHS;
            total_attempts += attempts;
        }
    }
    if runs != (GENOTYPES as usize) * MODES.len() || snapshots != runs * EPOCHS {
        return Err("incomplete registered 32-genotype four-arm slow-parent beneficial mutation experiment".into());
    }
    println!("Study111 ORIGINAL complete: genotype_labels={GENOTYPES} distinct_initial_code_hashes={} native_world_runs={runs} fully_replayed_SHA256_epochs={snapshots} charged_attempts={total_attempts} [HUMAN-DESIGNED CIL WITH EXTERNALLY REPLENISHED POOL, NOT EVOLUTION]; {path}", distinct.len());
    Ok(())
}
