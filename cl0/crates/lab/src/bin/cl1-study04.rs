//! CL-1 Study 04: frozen long-horizon external-compute ecology.
//! Externally replenished energy is never credited to endogenous discovery.
use cl0_lab::{Laboratory, ScheduledAction, ScheduledCondition};
use cl0_world::{Intervention, Op, Process, World};
use std::{env, error::Error};

const FIRST_SEED: u64 = 940_000;
const REPLICAS: u64 = 128;
const HORIZON: usize = 512;

fn random(state: &mut u64) -> u64 {
    *state = state
        .wrapping_mul(6_364_136_223_846_793_005)
        .wrapping_add(1_442_695_040_888_963_407);
    *state
}

fn seeded_world(seed: u64) -> World {
    let mut state = seed;
    let mut world = World::with_strict_capacity(32, 32, 160);
    for id in 1..=3 {
        let key = (random(&mut state) % 5) as i64;
        let value = (random(&mut state) % 101) as i64;
        let amount = 2 + (random(&mut state) % 3) as i64;
        let target = 1 + (random(&mut state) % 3) as i64;
        let mut nodes = vec![
            Op::Const {
                dst: 0,
                value: key,
                next: 1,
            },
            Op::Const {
                dst: 1,
                value,
                next: 2,
            },
            Op::Const {
                dst: 2,
                value: amount,
                next: 3,
            },
            Op::Const {
                dst: 3,
                value: target,
                next: 4,
            },
        ];
        for index in 4..15 {
            let next = index + 1;
            let opcode = match random(&mut state) % 12 {
                0 => Op::Add {
                    dst: 1,
                    lhs: 1,
                    rhs: 2,
                    next,
                },
                1 => Op::Sub {
                    dst: 1,
                    lhs: 1,
                    rhs: 2,
                    next,
                },
                2 => Op::Write {
                    address: 0,
                    value: 1,
                    next,
                },
                3 => Op::Read {
                    dst: 1,
                    address: 0,
                    next,
                },
                4 => {
                    if random(&mut state) % 2 == 0 {
                        Op::Branch {
                            test: 2,
                            nonzero: index - 2,
                            zero: next,
                        }
                    } else {
                        Op::Branch {
                            test: 1,
                            nonzero: next,
                            zero: index - 1,
                        }
                    }
                }
                5 => Op::DrawCompute { amount: 2, next },
                6 => Op::ReturnCompute { amount: 2, next },
                7 => Op::TransferCompute {
                    recipient: 3,
                    amount: 2,
                    next,
                },
                8 => Op::GraphNew { dst: 3, next },
                9 => Op::GraphCopyNode {
                    graph: 3,
                    source: 2,
                    dst: 2,
                    next,
                },
                10 => Op::GraphLaunch {
                    graph: 3,
                    credits: 2,
                    dst: 1,
                    next,
                },
                _ => Op::CloneNode {
                    dst: 3,
                    source: 2,
                    next,
                },
            };
            nodes.push(opcode);
        }
        nodes.push(Op::Halt);
        world
            .load(Process::new(id, nodes, 28))
            .expect("registered world initialization must fit");
    }
    world
}

fn conditions(seed: u64) -> Vec<ScheduledCondition> {
    let action = |after_attempts, action| ScheduledAction {
        after_attempts,
        action,
    };
    let supply = |at| action(at, Intervention::SetComputePool { credits: 256 });
    vec![
        ScheduledCondition {
            label: format!("seed={seed}/closed"),
            actions: vec![],
        },
        ScheduledCondition {
            label: format!("seed={seed}/initial-pool"),
            actions: vec![supply(0)],
        },
        ScheduledCondition {
            label: format!("seed={seed}/replenished"),
            actions: [0, 128, 256, 384].into_iter().map(supply).collect(),
        },
        ScheduledCondition {
            label: format!("seed={seed}/disable-p1"),
            actions: vec![supply(0), action(32, Intervention::Disable { process: 1 })],
        },
        ScheduledCondition {
            label: format!("seed={seed}/zero-credit-pool"),
            actions: std::iter::once(supply(0))
                .chain((1..=3).map(|process| {
                    action(
                        0,
                        Intervention::SetCredits {
                            process,
                            credits: 0,
                        },
                    )
                }))
                .collect(),
        },
    ]
}

fn main() -> Result<(), Box<dyn Error>> {
    let path = env::args()
        .nth(1)
        .unwrap_or_else(|| "cl1-study04.sqlite".to_owned());
    let revision =
        env::var("CL1_SOURCE_REVISION").unwrap_or_else(|_| "unverified-local-checkout".into());
    let mut lab = Laboratory::open(&path)?;
    let mut replayed = 0_usize;
    for seed in FIRST_SEED..FIRST_SEED + REPLICAS {
        let world = seeded_world(seed);
        let runs = lab.run_scheduled(&world, &conditions(seed), HORIZON, &revision)?;
        if runs.len() != 5
            || runs
                .iter()
                .any(|r| r.initial_sha256 != runs[0].initial_sha256)
        {
            return Err(format!("unmatched study conditions for seed {seed}").into());
        }
        for run in &runs {
            if !lab.verify_scheduled_replay(run.id)? {
                return Err(format!("replay invalid for seed {seed}: {}", run.label).into());
            }
            if run.world.occupied_units() > 160 {
                return Err(format!("resource capacity exceeded for seed {seed}").into());
            }
            if run.label.ends_with("/closed") && run.attempts > 84 {
                return Err(
                    format!("closed-world compute inventory increased: seed {seed}").into(),
                );
            }
            if run.label.ends_with("/zero-credit-pool") && run.attempts != 0 {
                return Err(format!("zero-credit control ran unexpectedly: seed {seed}").into());
            }
            replayed += 1;
        }
    }
    println!(
        "CL-1 Study 04: {REPLICAS} externally seeded worlds,          {replayed} conditions, {replayed} exact replays; path={path}"
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strict_world_and_generator_are_deterministic() {
        assert_eq!(seeded_world(FIRST_SEED).occupied_units(), 51);
        assert_eq!(seeded_world(FIRST_SEED), seeded_world(FIRST_SEED));
        assert_ne!(seeded_world(FIRST_SEED), seeded_world(FIRST_SEED + 1));
    }

    #[test]
    fn resource_supply_is_explicit_and_ordered() {
        let cases = conditions(FIRST_SEED);
        assert_eq!(cases.len(), 5);
        assert!(cases[0].actions.is_empty());
        assert_eq!(
            cases[2]
                .actions
                .iter()
                .map(|a| a.after_attempts)
                .collect::<Vec<_>>(),
            vec![0, 128, 256, 384]
        );
        assert!(cases[4].actions.iter().all(|a| a.after_attempts == 0));
    }

    #[test]
    fn matched_condition_replay_survives_late_inert_offsets() {
        let mut lab = Laboratory::in_memory().unwrap();
        let runs = lab
            .run_scheduled(
                &seeded_world(FIRST_SEED),
                &conditions(FIRST_SEED),
                HORIZON,
                "test",
            )
            .unwrap();
        assert_eq!(runs.len(), 5);
        assert!(runs
            .iter()
            .all(|run| lab.verify_scheduled_replay(run.id).unwrap()));
        assert_eq!(runs[4].attempts, 0);
        assert!(runs[0].attempts <= 84);
    }
}
