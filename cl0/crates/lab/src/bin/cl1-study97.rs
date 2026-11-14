//! CL-1 Study97: prospective random CHILD-PAYLOAD phenotype under an explicitly
//! investigator-scripted generic native CIL assembler. NOT spontaneous reproduction.
use cl0_lab::{hash_bytes, EpochCondition, Laboratory};
use cl0_world::{EventKind, Intervention, Op, Origin, Process, World};
use std::{collections::BTreeSet, env, error::Error};

const FIRST_SEED: u64 = 6_200_000;
const SEEDS: u64 = 256;
const EPOCHS: usize = 12;
const QUOTA: usize = 12;
const MODES: [&str; 3] = ["local", "absolute-stale", "zero-credit"];

fn random(state: &mut u64) -> u64 {
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

/// Fixed interface/loop scaffolding is NOT emerged; these twelve algorithmic
/// opcodes and their Const payloads are genuinely fresh per original seed.
fn random_child_payload(seed: u64) -> Vec<Op> {
    let mut state = mix_seed(seed);
    let mut child = vec![
        Op::Const { dst: 2, value: 7, next: 1 },
        Op::Const { dst: 3, value: 9, next: 2 },
        Op::Const { dst: 1, value: 1, next: 3 },
    ];
    for pc in 3..15 {
        let next = pc + 1;
        let op = match random(&mut state) % 5 {
            0 => Op::Read { dst: 0, address: 2, next },
            1 => Op::Write { address: 3, value: 0, next },
            2 => Op::Add { dst: 0, lhs: 0, rhs: 1, next },
            3 => Op::Sub { dst: 0, lhs: 0, rhs: 1, next },
            _ => Op::Const { dst: 0, value: (random(&mut state) % 8) as i64, next },
        };
        child.push(op);
    }
    child.push(Op::Branch { test: 2, nonzero: 3, zero: 3 });
    assert_eq!(child.len(), 16);
    child
}

/// Source code at source parent node100..115 uses absolute parent edges;
/// GraphCopyNodeLocal generically relocates its final Branch to native child3.
fn parent_template(op: &Op, source_index: usize) -> Op {
    let next = source_index + 1;
    match op {
        Op::Const { dst, value, .. } => Op::Const { dst: *dst, value: *value, next },
        Op::Read { dst, address, .. } => Op::Read { dst: *dst, address: *address, next },
        Op::Write { address, value, .. } => Op::Write { address: *address, value: *value, next },
        Op::Add { dst, lhs, rhs, .. } => {
            Op::Add { dst: *dst, lhs: *lhs, rhs: *rhs, next }
        }
        Op::Sub { dst, lhs, rhs, .. } => {
            Op::Sub { dst: *dst, lhs: *lhs, rhs: *rhs, next }
        }
        Op::Branch { .. } => Op::Branch { test: 2, nonzero: 103, zero: 103 },
        _ => unreachable!("frozen sixteen-node child payload grammar"),
    }
}

/// This constant parent is an openly human-programmed blind assembler.
/// Child functional opcodes are generated independently and never inspected
/// to decide whether to copy them or how to assemble them.
fn scripted_parent(payload: &[Op], mode: &str) -> Result<Vec<Op>, Box<dyn Error>> {
    if !MODES.contains(&mode) || payload.len() != 16 {
        return Err("unknown Study97 mode or child payload length".into());
    }
    let mut code = vec![Op::GraphNew { dst: 0, next: 1 }];
    for index in 0..16 {
        let pc = code.len();
        code.push(Op::Const {
            dst: 1,
            value: i64::try_from(100 + index)?,
            next: pc + 1,
        });
        let copy = if mode == "absolute-stale" {
            Op::GraphCopyNode { graph: 0, source: 1, dst: 2, next: pc + 2 }
        } else {
            Op::GraphCopyNodeLocal { graph: 0, source: 1, dst: 2, next: pc + 2 }
        };
        code.push(copy);
    }
    assert_eq!(code.len(), 33);
    for index in 0..15 {
        let pc = code.len();
        code.push(Op::Const { dst: 1, value: i64::try_from(index)?, next: pc + 1 });
        code.push(Op::Const { dst: 3, value: i64::try_from(index + 1)?, next: pc + 2 });
        code.push(Op::GraphSetNext { graph: 0, node: 1, target: 3, next: pc + 3 });
    }
    assert_eq!(code.len(), 78);
    code.push(Op::Const {
        dst: 1,
        value: if mode == "zero-credit" { 0 } else { 48 },
        next: 79,
    });
    code.push(Op::GraphLaunch { graph: 0, credits: 1, dst: 2, next: 80 });
    code.push(Op::Halt);
    while code.len() < 100 {
        code.push(Op::Halt);
    }
    for (index, op) in payload.iter().enumerate() {
        code.push(parent_template(op, 100 + index));
    }
    if code.len() != 116 {
        return Err("parent/child assembly source template size changed".into());
    }
    Ok(code)
}

fn initial_world(payload: &[Op], mode: &str, source: i64) -> Result<World, Box<dyn Error>> {
    let code = scripted_parent(payload, mode)?;
    let mut world = World::with_strict_capacity(4, 128, 320);
    world.load(Process::new(1, code, 192))?;
    world.intervene(Intervention::SetCell { key: 7, value: source })?;
    Ok(world)
}

fn main() -> Result<(), Box<dyn Error>> {
    let path = env::args().nth(1)
        .unwrap_or_else(|| "cl1-study97-random-child-payload-ecology.sqlite".into());
    let revision = env::var("CL1_SOURCE_REVISION")
        .unwrap_or_else(|_| "unverified-local-checkout".into());
    let mut lab = Laboratory::open(&path)?;
    let mut unique_payloads = BTreeSet::new();
    let mut worlds = 0_u64;
    let mut launched = 0_u64;
    for seed in FIRST_SEED..FIRST_SEED + SEEDS {
        let payload = random_child_payload(seed);
        unique_payloads.insert(hash_bytes(serde_json::to_string(&payload)?.as_bytes()));
        for source in [3, 8] {
            for mode in MODES {
                let original = initial_world(&payload, mode, source)?;
                let expected_code = scripted_parent(&payload, mode)?;
                if original.process(1).ok_or("parent missing")?.code.nodes != expected_code {
                    return Err("original stochastic payload matched-arm code mismatch".into());
                }
                let result = lab.run_epochs(
                    &original,
                    &[EpochCondition {
                        label: format!("seed={seed}/source={source}/{mode}"),
                        actions: vec![],
                    }],
                    EPOCHS,
                    QUOTA,
                    &revision,
                )?;
                let run = result.first().ok_or("original native child run missing")?;
                if !lab.verify_epoch_replay(run.id)? {
                    return Err(format!("original full epoch replay mismatch: {seed}/{source}/{mode}").into());
                }
                let attempts = run.attempts_by_epoch.iter().sum::<usize>() as u64;
                if run.world.total_compute_inventory() != Some(192 - attempts)
                    || run.world.events().iter().any(|e| matches!(e.kind, EventKind::InterventionRejected(_)))
                {
                    return Err(format!("native compute conservation/intervention invalid: {seed}/{source}/{mode}").into());
                }
                let launches = run.world.events().iter().filter(|e| {
                    e.origin == Origin::Internal(1)
                        && matches!(e.kind, EventKind::GraphLaunched {
                            moved_nodes: 16, ..
                        })
                }).count();
                if launches != 1 {
                    return Err(format!("scripted blind child assembler did not launch exactly one child: {seed}/{source}/{mode}").into());
                }
                launched += launches as u64;
                worlds += 1;
            }
        }
        if (seed - FIRST_SEED + 1).is_multiple_of(32) {
            println!("Study97 original random child payloads: {} / {SEEDS} seeds", seed - FIRST_SEED + 1);
        }
    }
    if worlds != SEEDS * 6 || launched != worlds {
        return Err("incomplete preregistered source-paired native child constructor cohort".into());
    }
    println!(
        "Study97 ORIGINAL: {SEEDS} independent sampled payload labels, {} distinct payload bytes, {worlds} native source-paired child worlds, {} original SHA256 epochs; builder is HUMAN-authored and no life claim; {path}",
        unique_payloads.len(), worlds * EPOCHS as u64
    );
    Ok(())
}
