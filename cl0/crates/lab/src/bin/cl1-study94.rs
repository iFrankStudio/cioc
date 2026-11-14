//! Study94 prospective 128-genotype original native executable-code edit ecology.
//! The untouched eight-way grammar permits native PatchNext.
//! A source-witnessed live predecessor bypasses the actual next Writer to its existing continuation.
//! This is NOT a preprogrammed repair or a claim of life.
use cl0_lab::{hash_bytes, EpochAction, EpochCondition, Laboratory};
use cl0_world::{EventKind, Intervention, Op, Origin, Process, World};
use std::{collections::BTreeSet, env, error::Error};

const FIRST_SEED: u64 = 6_100_000;
const SEEDS: u64 = 512;
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

/// The first qualifying pair is chosen entirely from actual *pretreatment*
/// native actor event ancestry and current code, before any future lesion.
fn directly_functional_writer_site(low: &World, high: &World) -> Option<(usize, usize, usize, i64)> {
    let (low_writes, low_tainted, low_offset) = source_state(low, 3);
    let (high_writes, high_tainted, high_offset) = source_state(high, 8);
    let a = low.process(1)?;
    let b = high.process(1)?;
    let site = a.pc;
    if a.pc != b.pc || !(3..27).contains(&site)
        || a.stopped || b.stopped || a.credits < 2 || b.credits < 2
        || a.registers[1..] != [8, 7, 9]
        || b.registers[1..] != [8, 7, 9]
        || !low_tainted || !high_tainted || low_offset != high_offset
        || a.registers[0] != 3_i64.wrapping_add(low_offset)
        || b.registers[0] != 8_i64.wrapping_add(high_offset)
    {
        return None;
    }
    let step = |op: &Op, old: i64| -> Option<(usize, i64)> {
        match op {
            Op::Read { dst: 0, address: 2, next } => Some((*next, 0)),
            Op::Add { dst: 0, lhs: 0, rhs: 1, next } => {
                Some((*next, old.wrapping_add(8)))
            }
            Op::Sub { dst: 0, lhs: 0, rhs: 1, next } => {
                Some((*next, old.wrapping_sub(8)))
            }
            _ => None,
        }
    };
    let (writer, predicted_offset) = step(a.code.nodes.get(site)?, low_offset)?;
    if step(b.code.nodes.get(site)?, high_offset)? != (writer, predicted_offset)
        || writer == site || writer >= a.code.nodes.len()
        || !low_writes.contains(&(writer, predicted_offset))
        || !high_writes.contains(&(writer, predicted_offset))
    {
        return None;
    }
    let continuation = match (a.code.nodes.get(writer)?, b.code.nodes.get(writer)?) {
        (
            Op::Write { address: 3, value: 0, next: ca },
            Op::Write { address: 3, value: 0, next: cb },
        ) if ca == cb => *ca,
        _ => return None,
    };
    if continuation >= a.code.nodes.len() || continuation == writer
        || continuation == site
        || matches!(a.code.nodes.get(continuation)?, Op::Halt | Op::Write { .. })
        || matches!(b.code.nodes.get(continuation)?, Op::Halt | Op::Write { .. })
    {
        return None;
    }
    Some((site, writer, continuation, predicted_offset))
}

fn first_live_trigger(
    mut low: World,
    mut high: World,
) -> Result<(World, World, Option<(usize, usize, usize, i64)>, usize), Box<dyn Error>> {
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
        if let Some(site) = directly_functional_writer_site(&low, &high) {
            return Ok((low, high, Some(site), attempted_pair + 1));
        }
    }
    Ok((low, high, None, MAX_PRE_ATTEMPTS))
}

fn post_plan(
    prefix: &World,
    mode: &str,
    site: Option<(usize, usize, usize, i64)>,
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
        if let Some((predecessor, writer, continuation, _)) = site {
            if epoch == 0 && mode != "intact" {
                let next = if mode == "sham" { writer } else { continuation };
                let lesion = EpochAction {
                    epoch,
                    action: Intervention::PatchEdge { process: 1, node: predecessor, next },
                };
                predicted.intervene(lesion.action.clone())?;
                actions.push(lesion);
            }
            if epoch == 4 && mode == "lesion-host-restore" {
                let restore = EpochAction {
                    epoch,
                    action: Intervention::PatchEdge {
                        process: 1, node: predecessor, next: writer,
                    },
                };
                predicted.intervene(restore.action.clone())?;
                actions.push(restore);
            }
        }
        predicted.run(QUOTA);
    }
    Ok((actions, predicted, net_input))
}

fn main() -> Result<(), Box<dyn Error>> {
    let path = env::args().nth(1)
        .unwrap_or_else(|| "cl1-study94-restricted-native-restoration.sqlite".into());
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
            return Err(format!("Study94 duplicated original genotype: {seed}").into());
        }
        let (low, high, site, prefork_paired_attempts) = first_live_trigger(
            initial_world(&genotype, 3)?,
            initial_world(&genotype, 8)?,
        )?;
        if let Some((predecessor, writer, _, _)) = site {
            if low.process(1).ok_or("eligible actor missing")?.pc != predecessor
                || high.process(1).ok_or("eligible actor missing")?.pc != predecessor
                || predecessor == writer || prefork_paired_attempts > MAX_PRE_ATTEMPTS
                || directly_functional_writer_site(&low, &high) != site
            {
                return Err("invalid first direct-native-writer trigger".into());
            }
            exposed_seeds += 1;
        }
        for (source, prefix) in [(3, &low), (8, &high)] {
            for mode in POST_MODES {
                let mut world = prefix.clone();
                if mode == "lesion-edit-disabled" || mode == "lesion-host-restore" {
                    // The law is disabled only AFTER the identical pre-treatment history.
                    world = world.with_native_code_edits_disabled();
                }
                let (actions, predicted, net_input) =
                    post_plan(&world, mode, site)?;
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
                let run = result.first().ok_or("missing Study94 original run")?;
                if run.world != predicted || !lab.verify_epoch_replay(run.id)? {
                    return Err(format!("Study94 full replay mismatch: {seed}/{source}/{mode}").into());
                }
                let attempts = run.attempts_by_epoch.iter().sum::<usize>() as i128;
                let expected_inventory =
                    i128::from(world.total_compute_inventory().ok_or("starting inventory overflow")?)
                    + net_input - attempts;
                let actual_inventory =
                    i128::from(run.world.total_compute_inventory().ok_or("ending inventory overflow")?);
                if actual_inventory != expected_inventory {
                    return Err(format!("Study94 resource ledger invalid: {seed}/{source}/{mode}").into());
                }
                if run.world.events().iter().any(|e| {
                    matches!(e.kind, EventKind::InterventionRejected(_))
                }) {
                    return Err(format!("Study94 host intervention rejected: {seed}/{source}/{mode}").into());
                }
                worlds += 1;
            }
        }
        if (seed - FIRST_SEED + 1).is_multiple_of(32) {
            println!("Study94 original event-triggered native lesion ecology: {} / {SEEDS} unique genomes", seed - FIRST_SEED + 1);
        }
    }
    if unique_genomes.len() != SEEDS as usize || worlds != 2 * 6 * SEEDS {
        return Err("incomplete prospective fixed Stage94 cohort".into());
    }
    println!("Study94 ORIGINAL: {SEEDS} code genotypes, {exposed_seeds} first-live direct writer-predecessor functions, {worlds} matched native post-fork worlds, {} exact hashed epochs; source-pinned only, no life claim; {path}", worlds * POST_EPOCHS as u64);
    Ok(())
}
