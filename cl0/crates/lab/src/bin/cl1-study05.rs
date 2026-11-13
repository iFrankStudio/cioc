//! CL-1 Study 05: investigator-seeded information ecology with independent
//! external environment epochs. No host life oracle or repair.
use cl0_lab::{EpochAction, EpochCondition, Laboratory};
use cl0_world::{EventKind, Intervention, Op, Origin, Process, World};
use std::{env, error::Error};

const FIRST_SEED: u64 = 1_050_000;
const REPLICAS: u64 = 64;
const EPOCHS: usize = 24;
const QUOTA: usize = 8;

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
                    if random(&mut state).is_multiple_of(2) {
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

fn conditions(seed: u64) -> Vec<EpochCondition> {
    let action = |epoch, action| EpochAction { epoch, action };
    let supply = |epoch| action(epoch, Intervention::SetComputePool { credits: 32 });
    vec![
        EpochCondition {
            label: format!("seed={seed}/closed"),
            actions: vec![],
        },
        EpochCondition {
            label: format!("seed={seed}/initial-pool"),
            actions: vec![supply(0)],
        },
        EpochCondition {
            label: format!("seed={seed}/pulse-pool"),
            actions: [0, 4, 8, 12, 16, 20].into_iter().map(supply).collect(),
        },
        EpochCondition {
            label: format!("seed={seed}/zero-credit-pulse"),
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
                .chain([4, 8, 12, 16, 20].into_iter().map(supply))
                .collect(),
        },
    ]
}

fn main() -> Result<(), Box<dyn Error>> {
    let path = env::args()
        .nth(1)
        .unwrap_or_else(|| "cl1-study05.sqlite".into());
    let revision =
        env::var("CL1_SOURCE_REVISION").unwrap_or_else(|_| "unverified-local-checkout".into());
    let mut lab = Laboratory::open(&path)?;
    let mut replayed = 0;
    for seed in FIRST_SEED..FIRST_SEED + REPLICAS {
        let world = seeded_world(seed);
        let runs = lab.run_epochs(&world, &conditions(seed), EPOCHS, QUOTA, &revision)?;
        if runs.len() != 4
            || runs
                .iter()
                .any(|r| r.initial_sha256 != runs[0].initial_sha256)
        {
            return Err(format!("unmatched cohort: seed={seed}").into());
        }
        for run in runs {
            if !lab.verify_epoch_replay(run.id)? {
                return Err(format!("epoch replay mismatch for {}", run.label).into());
            }
            if run.world.occupied_units() > 160 {
                return Err(format!("invalid capacity for {}", run.label).into());
            }
            if run.label.ends_with("/closed") && run.attempts_by_epoch.iter().sum::<usize>() > 84 {
                return Err(format!("closed credit conservation failed for {}", run.label).into());
            }
            if run.label.ends_with("/zero-credit-pulse")
                && run.attempts_by_epoch.iter().sum::<usize>() != 0
            {
                return Err(
                    format!("inert environment control executed CIL: {}", run.label).into(),
                );
            }
            let expected_supply = if run.label.ends_with("/closed") {
                0
            } else if run.label.ends_with("/initial-pool") {
                1
            } else {
                6
            };
            let actual_supply = run
                .world
                .events()
                .iter()
                .filter(|event| {
                    event.origin == Origin::External
                        && matches!(&event.kind,
                    EventKind::Intervened(detail) if detail.contains("SetComputePool"))
                })
                .count();
            if actual_supply != expected_supply {
                return Err(
                    format!("exogenous environment pulses missed for {}", run.label).into(),
                );
            }
            replayed += 1;
        }
    }
    println!(
        "CL-1 Study 05: {REPLICAS} independently seeded worlds, {replayed} epoch runs, \
         {replayed} exact replays, {EPOCHS} epochs/run; path={path}"
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generator_remains_fixed_and_world_is_strict() {
        assert_eq!(seeded_world(FIRST_SEED).occupied_units(), 51);
        assert_eq!(seeded_world(FIRST_SEED), seeded_world(FIRST_SEED));
        assert_ne!(seeded_world(FIRST_SEED), seeded_world(FIRST_SEED + 1));
    }

    #[test]
    fn independent_pulses_continue_after_inert_processes() {
        let mut lab = Laboratory::in_memory().unwrap();
        let runs = lab
            .run_epochs(
                &seeded_world(FIRST_SEED),
                &conditions(FIRST_SEED),
                EPOCHS,
                QUOTA,
                "epoch-test",
            )
            .unwrap();
        assert_eq!(runs.len(), 4);
        assert_eq!(runs[3].attempts_by_epoch, vec![0; EPOCHS]);
        assert!(runs.iter().all(|r| lab.verify_epoch_replay(r.id).unwrap()));
        assert_eq!(runs[2].world.events().iter().filter(|event| {
            matches!(&event.kind, EventKind::Intervened(text) if text.contains("SetComputePool"))
        }).count(), 6);
    }
}
