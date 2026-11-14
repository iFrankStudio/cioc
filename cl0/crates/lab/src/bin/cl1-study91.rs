//! Study91 prospective 128-genotype original native executable-code edit ecology.
//! The untouched eight-way grammar permits native PatchNext.
//! A host damage redirects PC8 to PC0 rather than trapping the actor on PC8.
//! This is NOT a preprogrammed repair or a claim of life.
use cl0_lab::{hash_bytes, EpochAction, EpochCondition, Laboratory};
use cl0_world::{EventKind, Intervention, Op, Origin, Process, World};
use std::{collections::BTreeSet, env, error::Error};

const FIRST_SEED: u64 = 6_000_000;
const SEEDS: u64 = 128;
const MAX_PRE_ATTEMPTS: usize = 288;
const POST_EPOCHS: usize = 18;
const QUOTA: usize = 12;
const GENOME_END: usize = 27;
const POST_MODES: [&str; 6] = [
    "intact",
    "sham",
    "lesion-native",
    "lesion-edit-disabled",
    "lesion-host-restore",
    "lesion-draw-ablated",
];

fn random(state: &mut u64) -> u64 {
    // A fresh fully scrambled 64-bit SplitMix64 output for EACH draw.
    // Unlike Study81's low-bit LCG binning, modulo 8 now sees mixed bits.
    *state = state.wrapping_add(0x9e3779b97f4a7c15);
    let mut z = *state;
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
    z ^ (z >> 31)
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


fn initial_world(genotype: &[Op], source: i64) -> Result<World, Box<dyn Error>> {
    let mut world = World::with_strict_capacity(4, 32, 64)
        .with_pool_paid_draw()
        .with_recoverable_draw_shortage()
        .with_native_executable_edit_trace();
    world.load(Process::new(1, genotype.to_vec(), 64))?;
    world.intervene(Intervention::SetCell { key: 7, value: source })?;
    world.intervene(Intervention::SetComputePool { credits: 96 })?;
    Ok(world)
}

fn source_state(world: &World, source: i64) -> (BTreeSet<(usize, i64)>, bool, i64) {
    let mut records = BTreeSet::new();
    let Some(actor) = world.process(1) else { return (records, false, 0) };
    let mut tainted = false;
    let mut offset = 0_i64;
    let mut traversed8 = false;
    for e in world.events() {
        if e.origin != Origin::Internal(1) { continue; }
        let Some(op) = actor.code.nodes.get(e.pc) else { continue };
        if matches!(e.kind, EventKind::Trapped(_) | EventKind::DrawDeferred { .. } | EventKind::Halted) {
            continue;
        }
        if e.pc == 8 && !matches!(op, Op::Write { .. }) {
            traversed8 = true;
        }
        match (op, &e.kind) {
            (Op::Read { dst: 0, .. }, EventKind::CellRead { key: 7, value })
                if *value == source =>
            {
                tainted = true;
                offset = 0;
                traversed8 = e.pc == 8;
            }
            (Op::Const { dst: 0, value, .. }, EventKind::Executed) => {
                tainted = false;
                offset = *value;
                traversed8 = false;
            }
            (Op::Add { dst: 0, .. }, EventKind::Executed) => {
                offset = offset.wrapping_add(8);
            }
            (Op::Sub { dst: 0, .. }, EventKind::Executed) => {
                offset = offset.wrapping_sub(8);
            }
            (Op::Write { .. }, EventKind::CellWritten { key: 9, value })
                if tainted && traversed8 && *value == source.wrapping_add(offset) =>
            {
                records.insert((e.pc, offset));
            }
            _ => {}
        }
    }
    (records, tainted, offset)
}

fn patchable_next(world: &World) -> Option<usize> {
    match world.process(1)?.code.nodes.get(8)? {
        Op::Const { next, .. }
        | Op::Read { next, .. }
        | Op::Write { next, .. }
        | Op::Add { next, .. }
        | Op::Sub { next, .. }
        | Op::DrawCompute { next, .. }
        | Op::PatchNext { next, .. } => Some(*next),
        _ => None,
    }
}

/// The first joint live native-PC8, two-source and R0-def-use-qualified
/// checkpoint is selected without ever inspecting future lesion outcomes.
fn first_live_trigger(
    mut low: World,
    mut high: World,
) -> Result<(World, World, bool, usize), Box<dyn Error>> {
    for attempted_pair in 0..MAX_PRE_ATTEMPTS {
        if attempted_pair > 0 && attempted_pair.is_multiple_of(QUOTA) {
            for world in [&mut low, &mut high] {
                let credits = world.available_compute_pool()
                    .checked_add(8).ok_or("native prefork environmental input overflow")?;
                world.intervene(Intervention::SetComputePool { credits })?;
            }
        }
        low.run(1);
        high.run(1);
        let (low_writers, low_tainted, low_offset) = source_state(&low, 3);
        let (high_writers, high_tainted, high_offset) = source_state(&high, 8);
        let Some(low_actor) = low.process(1) else { continue };
        let Some(high_actor) = high.process(1) else { continue };
        let valid = low_actor.pc == 8 && high_actor.pc == 8
            && !low_actor.stopped && !high_actor.stopped
            && low_actor.credits > 0 && high_actor.credits > 0
            && low_actor.registers[1..] == [8, 7, 9]
            && high_actor.registers[1..] == [8, 7, 9]
            && low_tainted && high_tainted
            && low_offset == high_offset
            && low_actor.registers[0] == 3_i64.wrapping_add(low_offset)
            && high_actor.registers[0] == 8_i64.wrapping_add(high_offset)
            && patchable_next(&low).is_some_and(|next| next != 0)
            && patchable_next(&high).is_some_and(|next| next != 0)
            && !low_writers.is_disjoint(&high_writers);
        if valid {
            return Ok((low, high, true, attempted_pair + 1));
        }
    }
    Ok((low, high, false, MAX_PRE_ATTEMPTS))
}

fn post_plan(
    prefix: &World,
    mode: &str,
    exposed: bool,
    original_edge: Option<usize>,
) -> Result<(Vec<EpochAction>, World, i128), Box<dyn Error>> {
    let mut predicted = prefix.clone();
    let mut actions = Vec::new();
    let mut net_input = 0_i128;
    for epoch in 0..POST_EPOCHS {
        let before = predicted.available_compute_pool();
        let credits = if mode == "lesion-draw-ablated" {
            0
        } else {
            before.checked_add(8).ok_or("post-fork environment overflow")?
        };
        net_input += i128::from(credits) - i128::from(before);
        let energy = EpochAction {
            epoch,
            action: Intervention::SetComputePool { credits },
        };
        predicted.intervene(energy.action.clone())?;
        actions.push(energy);
        if exposed && epoch == 0 && mode != "intact" {
            let next = if mode == "sham" {
                original_edge.ok_or("eligible function lacked patchable edge")?
            } else {
                // A traversable detour through startup; NOT a self-loop
                // that mathematically prevents all later native edits.
                0
            };
            let lesion = EpochAction {
                epoch,
                action: Intervention::PatchEdge { process: 1, node: 8, next },
            };
            predicted.intervene(lesion.action.clone())?;
            actions.push(lesion);
        }
        if exposed && epoch == 4 && mode == "lesion-host-restore" {
            let repair = EpochAction {
                epoch,
                action: Intervention::PatchEdge {
                    process: 1,
                    node: 8,
                    next: original_edge.ok_or("eligible function lacked restoration target")?,
                },
            };
            predicted.intervene(repair.action.clone())?;
            actions.push(repair);
        }
        predicted.run(QUOTA);
    }
    Ok((actions, predicted, net_input))
}

fn main() -> Result<(), Box<dyn Error>> {
    let path = env::args().nth(1)
        .unwrap_or_else(|| "cl1-study91-restricted-native-restoration.sqlite".into());
    let revision = env::var("CL1_SOURCE_REVISION")
        .unwrap_or_else(|_| "unverified-local-checkout".into());
    let mut lab = Laboratory::open(&path)?;
    let mut unique_genomes = BTreeSet::new();
    let mut exposed_seeds = 0_u64;
    let mut worlds = 0_u64;
    for seed in FIRST_SEED..FIRST_SEED + SEEDS {
        let genotype = code_for_seed(seed);
        let hash = hash_bytes(serde_json::to_string(&genotype)?.as_bytes());
        if !unique_genomes.insert(hash) {
            return Err(format!("Study91 duplicated original genotype: {seed}").into());
        }
        let (low, high, eligible, prefork_paired_attempts) = first_live_trigger(
            initial_world(&genotype, 3)?,
            initial_world(&genotype, 8)?,
        )?;
        if eligible {
            if low.process(1).ok_or("eligible actor missing")?.pc != 8
                || high.process(1).ok_or("eligible actor missing")?.pc != 8
                || prefork_paired_attempts > MAX_PRE_ATTEMPTS
            {
                return Err("invalid first live trigger".into());
            }
        }
        if eligible { exposed_seeds += 1; }
        for (source, prefix) in [(3, &low), (8, &high)] {
            for mode in POST_MODES {
                let mut world = prefix.clone();
                if mode == "lesion-edit-disabled" || mode == "lesion-host-restore" {
                    // The law is disabled only AFTER the identical pre-treatment history.
                    world = world.with_native_code_edits_disabled();
                }
                let (actions, predicted, net_input) =
                    post_plan(&world, mode, eligible, patchable_next(prefix))?;
                let result = lab.run_epochs(
                    &world,
                    &[EpochCondition {
                        label: format!("seed={seed}/source={source}/{mode}"),
                        actions,
                    }],
                    POST_EPOCHS,
                    QUOTA,
                    &revision,
                )?;
                let run = result.first().ok_or("missing Study91 original run")?;
                if run.world != predicted || !lab.verify_epoch_replay(run.id)? {
                    return Err(format!("Study91 full replay mismatch: {seed}/{source}/{mode}").into());
                }
                let attempts = run.attempts_by_epoch.iter().sum::<usize>() as i128;
                let expected_inventory =
                    i128::from(world.total_compute_inventory().ok_or("starting inventory overflow")?)
                    + net_input - attempts;
                let actual_inventory =
                    i128::from(run.world.total_compute_inventory().ok_or("ending inventory overflow")?);
                if actual_inventory != expected_inventory {
                    return Err(format!("Study91 resource ledger invalid: {seed}/{source}/{mode}").into());
                }
                if run.world.events().iter().any(|e| {
                    matches!(e.kind, EventKind::InterventionRejected(_))
                }) {
                    return Err(format!("Study91 host intervention rejected: {seed}/{source}/{mode}").into());
                }
                worlds += 1;
            }
        }
        if (seed - FIRST_SEED + 1).is_multiple_of(32) {
            println!("Study91 original event-triggered native lesion ecology: {} / {SEEDS} unique genomes", seed - FIRST_SEED + 1);
        }
    }
    if unique_genomes.len() != SEEDS as usize || worlds != 2 * 6 * SEEDS {
        return Err("incomplete prospective fixed Stage91 cohort".into());
    }
    println!("Study91 ORIGINAL: {SEEDS} code genotypes, {exposed_seeds} live native PC8 trigger source functions, {worlds} matched native post-fork worlds, {} exact hashed epochs; source-pinned only, no life claim; {path}", worlds * POST_EPOCHS as u64);
    Ok(())
}
