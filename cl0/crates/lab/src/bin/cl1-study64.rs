//! CL-1 Study64: stochastic native information-feedback opportunity, NOT life.
//! 128 independent seed GENOMES, eight explicitly matched topology/source/read variants.
//! Unlike Study63, no useful Read->Write sequence is authored by the researcher.
use cl0_lab::{EpochCondition, Laboratory};
use cl0_world::{Intervention, Op, Process, World};
use std::{env, error::Error};

const FIRST_SEED: u64 = 4_200_000;
const SEEDS: u64 = 128;
const EPOCHS: usize = 24;
const QUOTA: usize = 12;

const MODES: [&str; 8] = [
    "cyclic-low",
    "cyclic-high",
    "cyclic-ablated-low",
    "cyclic-ablated-high",
    "linear-low",
    "linear-high",
    "linear-ablated-low",
    "linear-ablated-high",
];

fn random(state: &mut u64) -> u64 {
    *state = state
        .wrapping_mul(6_364_136_223_846_793_005)
        .wrapping_add(1_442_695_040_888_963_407);
    *state
}

fn seeded_world(seed: u64, mode: &str) -> Result<World, Box<dyn Error>> {
    if !MODES.contains(&mode) {
        return Err("unknown preregistered ecology mode".into());
    }
    let cyclic = mode.starts_with("cyclic");
    let ablated = mode.contains("-ablated-");
    let sensor = if mode.ends_with("-low") { 3 } else { 8 };
    let mut state = seed;
    let mut nodes = vec![
        Op::Const {
            dst: 1,
            value: 1,
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
        let op = match random(&mut state) % 6 {
            0 => {
                if ablated {
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
            _ => Op::Branch {
                test: 0,
                nonzero: if index == 3 { 3 } else { index - 1 },
                zero: next,
            },
        };
        nodes.push(op);
    }
    if cyclic {
        nodes.push(Op::Branch {
            test: 1,
            nonzero: 3,
            zero: 3,
        });
    } else {
        nodes.push(Op::Halt);
    }
    let mut world = World::with_strict_capacity(4, 32, 64);
    world.load(Process::new(1, nodes, 300))?;
    // Sensor data is explicitly researcher-provided, not native organism content.
    world.intervene(Intervention::SetCell {
        key: 7,
        value: sensor,
    })?;
    Ok(world)
}

fn main() -> Result<(), Box<dyn Error>> {
    let path = env::args()
        .nth(1)
        .unwrap_or_else(|| "cl1-study64-stochastic-feedback.sqlite".into());
    let revision =
        env::var("CL1_SOURCE_REVISION").unwrap_or_else(|_| "unverified-local-checkout".into());
    let mut lab = Laboratory::open(&path)?;
    let mut total_runs = 0_u64;
    for seed in FIRST_SEED..FIRST_SEED + SEEDS {
        for mode in MODES {
            let world = seeded_world(seed, mode)?;
            let run = lab.run_epochs(
                &world,
                &[EpochCondition {
                    label: format!("seed={seed}/{mode}"),
                    actions: vec![],
                }],
                EPOCHS,
                QUOTA,
                &revision,
            )?;
            let recorded = run.first().ok_or("missing stochastic feedback run")?;
            if !lab.verify_epoch_replay(recorded.id)? {
                return Err(
                    format!("exact native Study64 replay failed: seed={seed}/{mode}").into(),
                );
            }
            let attempts: usize = recorded.attempts_by_epoch.iter().sum();
            if recorded.world.total_compute_inventory() != Some(300 - attempts as u64) {
                return Err(format!("compute conservation failed seed={seed}/{mode}").into());
            }
            total_runs += 1;
        }
        if (seed + 1 - FIRST_SEED) % 32 == 0 {
            println!(
                "Study64 exact-replayed {} of {} seed genomes",
                seed + 1 - FIRST_SEED,
                SEEDS
            );
        }
    }
    if total_runs != SEEDS * MODES.len() as u64 {
        return Err("incomplete preregistered original matched ecology".into());
    }
    println!(
        "Study64: {SEEDS} independent stochastic genomes; {total_runs} matched runs; {} hashed epochs; no life claim; {path}",
        total_runs * EPOCHS as u64
    );
    Ok(())
}
