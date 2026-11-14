//! CL-1 Study66: stochastic native sensor feedback and voluntary DrawCompute coupling.
//! This is NOT native self-maintenance: ring, sensor port and ambient energy are external.
use cl0_lab::{EpochAction, EpochCondition, Laboratory};
use cl0_world::{Intervention, Op, Process, World};
use std::{env, error::Error};

const FIRST_SEED: u64 = 4_400_000;
const SEEDS: u64 = 256;
const EPOCHS: usize = 24;
const QUOTA: usize = 12;
const MODES: [&str; 8] = [
    "ambient-low",
    "ambient-high",
    "closed-low",
    "closed-high",
    "ambient-draw-ablated-low",
    "ambient-draw-ablated-high",
    "ambient-read-ablated-low",
    "ambient-read-ablated-high",
];

fn random(state: &mut u64) -> u64 {
    *state = state
        .wrapping_mul(6_364_136_223_846_793_005)
        .wrapping_add(1_442_695_040_888_963_407);
    *state
}

fn seeded_world(seed: u64, mode: &str) -> Result<World, Box<dyn Error>> {
    if !MODES.contains(&mode) {
        return Err("unregistered Study66 mode".into());
    }
    let mut state = seed;
    let read_ablated = mode.contains("read-ablated");
    let draw_ablated = mode.contains("draw-ablated");
    let sensor = if mode.ends_with("-low") { 3 } else { 8 };
    let pool = if mode.starts_with("ambient") { 48 } else { 0 };
    let mut nodes = vec![
        Op::Const {
            dst: 1,
            value: 8,
            next: 1,
        },
        Op::Const {
            dst: 2,
            value: 7,
            next: 2,
        },
        Op::Const {
            dst: 3,
            value: 9,
            next: 3,
        },
    ];
    for index in 3..27 {
        let next = index + 1;
        let op = match random(&mut state) % 7 {
            0 => {
                if read_ablated {
                    Op::Const {
                        dst: 0,
                        value: 0,
                        next,
                    }
                } else {
                    Op::Read {
                        dst: 0,
                        address: 2,
                        next,
                    }
                }
            }
            1 => Op::Write {
                address: 3,
                value: 0,
                next,
            },
            2 => Op::Add {
                dst: 0,
                lhs: 0,
                rhs: 1,
                next,
            },
            3 => Op::Sub {
                dst: 0,
                lhs: 0,
                rhs: 1,
                next,
            },
            4 => Op::Const {
                dst: 0,
                value: (random(&mut state) % 8) as i64,
                next,
            },
            5 => Op::Branch {
                test: 0,
                nonzero: if index == 3 { 3 } else { index - 1 },
                zero: next,
            },
            _ => {
                if draw_ablated {
                    Op::Const {
                        dst: 1,
                        value: 8,
                        next,
                    }
                } else {
                    Op::DrawCompute { amount: 1, next }
                }
            }
        };
        nodes.push(op);
    }
    nodes.push(Op::Branch {
        test: 1,
        nonzero: 3,
        zero: 3,
    });
    let mut world = World::with_strict_capacity(4, 32, 64);
    world.load(Process::new(1, nodes, 16))?;
    // External source information and ambient pool setup; not native organism content.
    world.intervene(Intervention::SetCell {
        key: 7,
        value: sensor,
    })?;
    world.intervene(Intervention::SetComputePool { credits: pool })?;
    Ok(world)
}

/// Fixed environmental +8 pool input per epoch, NEVER a process-targeted starter.
/// SetComputePool is absolute; its value is computed from the prior replayed pool.
fn ambient_plan(original: &World) -> Result<(Vec<EpochAction>, World), Box<dyn Error>> {
    let mut predicted = original.clone();
    let mut actions = Vec::new();
    for epoch in 0..EPOCHS {
        if epoch >= 1 {
            let amount = predicted
                .available_compute_pool()
                .checked_add(8)
                .ok_or("native world pool overflow")?;
            let intervention = EpochAction {
                epoch,
                action: Intervention::SetComputePool { credits: amount },
            };
            predicted.intervene(intervention.action.clone())?;
            actions.push(intervention);
        }
        predicted.run(QUOTA);
    }
    Ok((actions, predicted))
}

fn main() -> Result<(), Box<dyn Error>> {
    let path = env::args()
        .nth(1)
        .unwrap_or_else(|| "cl1-study66-feedback-energy-ecology.sqlite".into());
    let revision =
        env::var("CL1_SOURCE_REVISION").unwrap_or_else(|_| "unverified-local-checkout".into());
    let mut lab = Laboratory::open(&path)?;
    let mut total_runs = 0_u64;
    for seed in FIRST_SEED..FIRST_SEED + SEEDS {
        for mode in MODES {
            let world = seeded_world(seed, mode)?;
            let ambient = mode.starts_with("ambient");
            let (actions, predicted) = if ambient {
                ambient_plan(&world)?
            } else {
                let mut predicted = world.clone();
                for _epoch in 0..EPOCHS {
                    predicted.run(QUOTA);
                }
                (vec![], predicted)
            };
            let result = lab.run_epochs(
                &world,
                &[EpochCondition {
                    label: format!("seed={seed}/{mode}"),
                    actions,
                }],
                EPOCHS,
                QUOTA,
                &revision,
            )?;
            let run = result.first().ok_or("missing original native seed run")?;
            if run.world != predicted || !lab.verify_epoch_replay(run.id)? {
                return Err(format!("native replay/ambient schedule mismatch: {seed}/{mode}").into());
            }
            let attempts: usize = run.attempts_by_epoch.iter().sum();
            let initial = 16_u64 + if ambient { 48 } else { 0 };
            let input = if ambient { 8_u64 * (EPOCHS as u64 - 1) } else { 0 };
            if run.world.total_compute_inventory() != Some(initial + input - attempts as u64) {
                return Err(format!("native compute inventory mismatch: {seed}/{mode}").into());
            }
            total_runs += 1;
        }
        if (seed + 1 - FIRST_SEED).is_multiple_of(32) {
            println!(
                "Study66 source-pinned native-run replay: {} / {} random genomes",
                seed + 1 - FIRST_SEED,
                SEEDS
            );
        }
    }
    if total_runs != SEEDS * MODES.len() as u64 {
        return Err("missing original registered eight-mode genome samples".into());
    }
    println!(
        "Study66 complete: {SEEDS} independent genomes, {total_runs} native runs, {} hashed epochs; no life claims; {path}",
        total_runs * EPOCHS as u64
    );
    Ok(())
}
