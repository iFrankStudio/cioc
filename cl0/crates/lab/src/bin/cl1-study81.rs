//! Study81 prospective 128-genotype original native executable-code edit ecology.
//! The seeded grammar intentionally contains generic native PatchNext; this study
//! does not introduce lesions and cannot be interpreted as endogenous repair/life.
use cl0_lab::{hash_bytes, EpochAction, EpochCondition, Laboratory};
use cl0_world::{Intervention, Op, Process, World};
use std::{env, error::Error};

const FIRST_SEED: u64 = 5_400_000;
const SEEDS: u64 = 128;
const EPOCHS: usize = 24;
const QUOTA: usize = 12;
const GENOME_END: usize = 27;
const MODES: [&str; 4] = [
    "source-low-native-edit",
    "source-high-native-edit",
    "source-low-edit-disabled",
    "source-high-edit-disabled",
];

fn random(state: &mut u64) -> u64 {
    *state = state
        .wrapping_mul(6_364_136_223_846_793_005)
        .wrapping_add(1_442_695_040_888_963_407);
    *state
}

fn mix_seed(seed: u64) -> u64 {
    let mut x = seed.wrapping_add(0x9e3779b97f4a7c15);
    x = (x ^ (x >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
    x = (x ^ (x >> 27)).wrapping_mul(0x94d049bb133111eb);
    x ^ (x >> 31)
}

fn code_for_seed(seed: u64) -> Vec<Op> {
    let mut state = mix_seed(seed);
    let mut nodes = vec![
        Op::Const { dst: 1, value: 8, next: 1 },
        Op::Const { dst: 2, value: 7, next: 2 },
        Op::Const { dst: 3, value: 9, next: 3 },
    ];
    for index in 3..GENOME_END {
        let next = index + 1;
        let op = match random(&mut state) % 8 {
            0 => Op::Read { dst: 0, address: 2, next },
            1 => Op::Write { address: 3, value: 0, next },
            2 => Op::Add { dst: 0, lhs: 0, rhs: 1, next },
            3 => Op::Sub { dst: 0, lhs: 0, rhs: 1, next },
            4 => Op::Const {
                dst: 0,
                value: (random(&mut state) % 8) as i64,
                next,
            },
            5 => {
                let sample = random(&mut state);
                let back = if index == 3 {
                    3
                } else {
                    3 + (sample % (index - 3) as u64) as usize
                };
                Op::Branch { test: 0, nonzero: back, zero: next }
            }
            6 => Op::DrawCompute { amount: 1, next },
            _ => Op::PatchNext { node: 1, target: 0, next },
        };
        nodes.push(op);
    }
    nodes.push(Op::Halt);
    nodes
}

fn seeded_world(seed: u64, mode: &str) -> Result<World, Box<dyn Error>> {
    if !MODES.contains(&mode) {
        return Err("unregistered Study81 arm".into());
    }
    let mut world = World::with_strict_capacity(4, 32, 64)
        .with_pool_paid_draw()
        .with_recoverable_draw_shortage()
        .with_native_executable_edit_trace();
    if mode.contains("disabled") {
        world = world.with_native_code_edits_disabled();
    }
    world.load(Process::new(1, code_for_seed(seed), 64))?;
    world.intervene(Intervention::SetCell {
        key: 7,
        value: if mode.contains("-low-") { 3 } else { 8 },
    })?;
    world.intervene(Intervention::SetComputePool { credits: 96 })?;
    Ok(world)
}

fn ambient_plan(initial: &World) -> Result<(Vec<EpochAction>, World), Box<dyn Error>> {
    let mut predicted = initial.clone();
    let mut plan = Vec::new();
    for epoch in 0..EPOCHS {
        if epoch > 0 {
            let supply = predicted.available_compute_pool()
                .checked_add(8)
                .ok_or("ambient pool addition overflow")?;
            let action = EpochAction {
                epoch,
                action: Intervention::SetComputePool { credits: supply },
            };
            predicted.intervene(action.action.clone())?;
            plan.push(action);
        }
        predicted.run(QUOTA);
    }
    Ok((plan, predicted))
}

fn main() -> Result<(), Box<dyn Error>> {
    let path = env::args().nth(1).unwrap_or_else(|| "cl1-study81-stochastic-native-edits.sqlite".into());
    let revision = env::var("CL1_SOURCE_REVISION")
        .unwrap_or_else(|_| "unverified-local-checkout".into());
    let mut lab = Laboratory::open(&path)?;
    let mut worlds = 0_u64;
    let mut unique_code = std::collections::BTreeSet::new();
    for seed in FIRST_SEED..FIRST_SEED + SEEDS {
        let genotype = code_for_seed(seed);
        let genome_hash = hash_bytes(serde_json::to_string(&genotype)?.as_bytes());
        if !unique_code.insert(genome_hash) {
            return Err(format!("duplicate seeded code genome: {seed}").into());
        }
        for mode in MODES {
            let world = seeded_world(seed, mode)?;
            if world.process(1).ok_or("initial actor missing")?.code.nodes != genotype {
                return Err(format!("matched original genome mismatch: {seed}/{mode}").into());
            }
            let (actions, predicted) = ambient_plan(&world)?;
            let output = lab.run_epochs(
                &world,
                &[EpochCondition {
                    label: format!("seed={seed}/{mode}"),
                    actions,
                }],
                EPOCHS,
                QUOTA,
                &revision,
            )?;
            let run = output.first().ok_or("missing original Study81 run")?;
            if run.world != predicted || !lab.verify_epoch_replay(run.id)? {
                return Err(format!("full original native snapshot replay failed: {seed}/{mode}").into());
            }
            let actual_attempts: usize = run.attempts_by_epoch.iter().sum();
            if run.world.total_compute_inventory()
                != Some(64_u64 + 96 + 8 * (EPOCHS as u64 - 1) - actual_attempts as u64)
            {
                return Err(format!("conserved compute inventory mismatch: {seed}/{mode}").into());
            }
            worlds += 1;
        }
        if (seed - FIRST_SEED + 1).is_multiple_of(32) {
            println!("Study81 native source replay: {} / {SEEDS} untouched genomes", seed - FIRST_SEED + 1);
        }
    }
    if worlds != SEEDS * MODES.len() as u64 || unique_code.len() != SEEDS as usize {
        return Err("incomplete original registered seeded population".into());
    }
    println!("Study81 ORIGINAL: {SEEDS} seeded code genomes; {worlds} matched native worlds; {} SHA256 epochs; no lesion/repair/life claims; {path}", worlds * EPOCHS as u64);
    Ok(())
}
