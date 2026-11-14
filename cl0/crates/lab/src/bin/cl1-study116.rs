//! Study116: preregistered, entirely HUMAN-authored neutral-code buffer mechanism.
//! Same 56-node whole-copy genome within every paired experimental condition.
use cl0_lab::{hash_bytes, EpochAction, EpochCondition, Laboratory};
use cl0_world::{Intervention, Op, Process, World};
use std::{collections::BTreeSet, env, error::Error};

const FIRST_SEED: u64 = 6_900_000;
const GENOTYPES: u64 = 64;
const EPOCHS: usize = 16;
const QUOTA: usize = 250;
const MODES: [&str; 8] = [
    "native-latent-source3", "disabled-latent-source3",
    "native-inert-source3", "disabled-inert-source3",
    "native-latent-source8", "disabled-latent-source8",
    "native-inert-source8", "disabled-inert-source8",
];

fn splitmix64(state: &mut u64) -> u64 {
    *state = state.wrapping_add(0x9e3779b97f4a7c15);
    let mut z = *state;
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
    z ^ (z >> 31)
}

fn genome(seed: u64, latent: bool) -> (Vec<Op>, i64) {
    let mut rng = seed;
    // All 56 mutation destinations are used without retrospective selection.
    let target = (splitmix64(&mut rng) % 56) as i64;
    let delay = 1 + splitmix64(&mut rng) % 32;
    let grant = 320 + splitmix64(&mut rng) % 161;
    let mut nodes = vec![
        Op::Const { dst: 2, value: 7, next: 1 },
        Op::Const { dst: 3, value: 9, next: 2 },
        Op::Read { dst: 0, address: 2, next: 3 },
        Op::Write { address: 3, value: 0, next: 4 },
        Op::Const { dst: 2, value: 1, next: 24 },
        Op::Const { dst: 2, value: 11, next: 6 },
        Op::Read { dst: 0, address: 2, next: 7 },
        Op::Const { dst: 1, value: 4, next: 28 },
        Op::PatchNext { node: 1, target: 0, next: 55 },
        Op::Const { dst: 1, value: 0, next: 10 },
        Op::Sub { dst: 3, lhs: 3, rhs: 3, next: 11 },
        Op::Write { address: 2, value: 1, next: 12 },
        Op::Const { dst: 2, value: 1, next: 13 },
        Op::Const { dst: 1, value: 0, next: 14 },
        Op::Const { dst: 3, value: 56, next: 15 },
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
        Op::Const { dst: 2, value: 13, next: 53 },
    ];
    for start in [29_usize, 35, 41, 47] {
        if latent {
            // Human-authored latent processing module, NOT newly synthesized code.
            nodes.extend([
                Op::Const { dst: 2, value: 7, next: start + 1 },
                Op::Read { dst: 0, address: 2, next: start + 2 },
                Op::Const { dst: 3, value: 10, next: start + 3 },
                Op::Const { dst: 2, value: 1, next: start + 4 },
                Op::Add { dst: 0, lhs: 0, rhs: 2, next: start + 5 },
                Op::Write { address: 3, value: 0, next: 5 },
            ]);
        } else {
            // Equal-copy-cost, equal-length neutral no-op code control.
            for index in start..start + 6 {
                nodes.push(Op::Const { dst: 0, value: 0, next: index });
            }
        }
    }
    // Actor-authored one-shot mutation authorization and subsequent disable.
    nodes.push(Op::Read { dst: 3, address: 2, next: 54 });
    nodes.push(Op::Branch { test: 3, nonzero: 8, zero: 55 });
    nodes.push(Op::Const { dst: 2, value: 13, next: 9 });
    assert_eq!(nodes.len(), 56);
    (nodes, target)
}

fn initial_world(code: Vec<Op>, mode: &str, target: i64)
    -> Result<World, Box<dyn Error>>
{
    if !MODES.contains(&mode) {
        return Err("unregistered Study116 source/latent/ablation arm".into());
    }
    let mut world = World::with_strict_capacity(4, 128, 4096)
        .with_pool_paid_draw()
        .with_recoverable_draw_shortage()
        .with_native_executable_edit_trace();
    if mode.starts_with("disabled-") {
        world = world.with_native_patchnext_disabled();
    }
    world.load(Process::new(1, code.clone(), 500))?;
    world.load(Process::new(2, code, 500))?;
    let source = if mode.ends_with("source3") { 3 } else { 8 };
    world.intervene(Intervention::SetCell { key: 7, value: source })?;
    world.intervene(Intervention::SetCell { key: 11, value: target })?;
    world.intervene(Intervention::SetCell { key: 13, value: 1 })?;
    world.intervene(Intervention::SetComputePool { credits: 800 })?;
    Ok(world)
}

fn main() -> Result<(), Box<dyn Error>> {
    let path = env::args().nth(1)
        .unwrap_or_else(|| "cl1-study116-original-native-dormant-buffer.sqlite".into());
    let revision = env::var("CL1_SOURCE_REVISION")
        .unwrap_or_else(|_| "unverified-local-checkout".into());
    let mut lab = Laboratory::open(&path)?;
    let mut code_hashes = BTreeSet::new();
    let mut target_seeds = BTreeSet::new();
    let mut worlds = 0_usize;
    let mut snapshots = 0_usize;
    let mut paid = 0_usize;
    for seed in FIRST_SEED..FIRST_SEED + GENOTYPES {
        for mode in MODES {
            let latent = mode.contains("-latent-");
            let (code, target) = genome(seed, latent);
            target_seeds.insert((seed, target));
            code_hashes.insert(hash_bytes(serde_json::to_string(&code)?.as_bytes()));
            let world = initial_world(code, mode, target)?;
            if world.process(1).unwrap().code != world.process(2).unwrap().code {
                return Err(format!("initial founder code bytes differ: {seed}/{mode}").into());
            }
            let mut expected = world.clone();
            let mut ambient_actions = Vec::new();
            for epoch in 0..EPOCHS {
                if epoch > 0 {
                    let credits = expected.available_compute_pool()
                        .checked_add(400).ok_or("ambient public supply overflow")?;
                    let action = EpochAction {
                        epoch,
                        action: Intervention::SetComputePool { credits },
                    };
                    expected.intervene(action.action.clone())?;
                    ambient_actions.push(action);
                }
                expected.run(QUOTA);
            }
            let actual = lab.run_epochs(
                &world,
                &[EpochCondition {
                    label: format!("seed={seed}/{mode}"),
                    actions: ambient_actions,
                }],
                EPOCHS,
                QUOTA,
                &revision,
            )?;
            let outcome = actual.first().ok_or("missing native neutral-buffer run")?;
            if outcome.world != expected || !lab.verify_epoch_replay(outcome.id)? {
                return Err(format!("exact Study116 epoch replay mismatch: {seed}/{mode}").into());
            }
            let charged: usize = outcome.attempts_by_epoch.iter().sum();
            if outcome.world.total_compute_inventory() != Some(7800 - charged as u64) {
                return Err(format!("native energy ledger corrupted: {seed}/{mode}").into());
            }
            if outcome.attempts_by_epoch.len() != EPOCHS {
                return Err("incomplete registered CIL resource epochs".into());
            }
            worlds += 1;
            snapshots += EPOCHS;
            paid += charged;
        }
    }
    if worlds != 512 || snapshots != 8192 || target_seeds.len() != GENOTYPES as usize {
        return Err("missing original full 64-genotype 8-arm latent module worlds".into());
    }
    println!("Study116 ORIGINAL one-shot native CIL mutagen: original_seed_labels={GENOTYPES} distinct_source_code_hashes={} native_worlds={worlds} exactly_replayed_SHA256_epochs={snapshots} total_native_charges={paid} [HUMAN-WRITTEN MUTATOR GATE, NOT LIFE]; {path}",code_hashes.len());
    Ok(())
}
