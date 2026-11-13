//! CL-1 Study 03: preregistered random code-damage/reversal audit.
//! No host repair instruction or life classification. See frozen preregistration.
use cl0_lab::{Laboratory, ScheduledAction, ScheduledCondition};
use cl0_world::{Intervention, Op, Process, World};
use std::{env, error::Error};

const FIRST_SEED: u64 = 830_000;
const REPLICAS: u64 = 128;
const HORIZON: usize = 96;

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
        let target_node = (random(&mut state) % 15) as i64;
        let target_edge = (random(&mut state) % 16) as i64;
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
                value: target_node,
                next: 3,
            },
            Op::Const {
                dst: 3,
                value: target_edge,
                next: 4,
            },
        ];
        for index in 4..15 {
            let next = index + 1;
            let op = if index == 8 {
                // Patchable lesion target; no planted repair controller.
                match random(&mut state) % 5 {
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
                    _ => Op::DrawCompute { amount: 2, next },
                }
            } else {
                match random(&mut state) % 12 {
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
                    4 => Op::Branch {
                        test: 1,
                        nonzero: next,
                        zero: index - 1,
                    },
                    5 => Op::PatchNext {
                        node: 2,
                        target: 3,
                        next,
                    },
                    6 => Op::CloneNode {
                        dst: 3,
                        source: 2,
                        next,
                    },
                    7 => Op::GraphNew { dst: 3, next },
                    8 => Op::GraphCopyNode {
                        graph: 3,
                        source: 2,
                        dst: 2,
                        next,
                    },
                    9 => Op::GraphDrop { graph: 3, next },
                    10 => Op::DrawCompute { amount: 2, next },
                    _ => Op::Branch {
                        test: 2,
                        nonzero: index - 2,
                        zero: next,
                    },
                }
            };
            nodes.push(op);
        }
        nodes.push(Op::Halt);
        world
            .load(Process::new(id, nodes, 28))
            .expect("preregistered world must fit strict resource bounds");
    }
    world
}

fn conditions(seed: u64) -> Vec<ScheduledCondition> {
    let noop = |at| ScheduledAction {
        after_attempts: at,
        action: Intervention::Disable { process: 9999 },
    };
    let action = |at, action| ScheduledAction {
        after_attempts: at,
        action,
    };
    vec![
        ScheduledCondition {
            label: format!("seed={seed}/sham"),
            actions: [4, 24, 48, 72].into_iter().map(noop).collect(),
        },
        ScheduledCondition {
            label: format!("seed={seed}/lesion-only"),
            actions: vec![
                action(
                    4,
                    Intervention::PatchEdge {
                        process: 1,
                        node: 8,
                        next: 0,
                    },
                ),
                noop(24),
                noop(48),
                noop(72),
            ],
        },
        ScheduledCondition {
            label: format!("seed={seed}/external-restore"),
            actions: vec![
                action(
                    4,
                    Intervention::PatchEdge {
                        process: 1,
                        node: 8,
                        next: 0,
                    },
                ),
                action(
                    24,
                    Intervention::PatchEdge {
                        process: 1,
                        node: 8,
                        next: 9,
                    },
                ),
                noop(48),
                noop(72),
            ],
        },
        ScheduledCondition {
            label: format!("seed={seed}/disable-p1"),
            actions: vec![
                action(4, Intervention::Disable { process: 1 }),
                noop(24),
                noop(48),
                noop(72),
            ],
        },
        ScheduledCondition {
            label: format!("seed={seed}/zero-credit"),
            actions: (1..=3)
                .map(|process| {
                    action(
                        0,
                        Intervention::SetCredits {
                            process,
                            credits: 0,
                        },
                    )
                })
                .collect(),
        },
    ]
}

fn main() -> Result<(), Box<dyn Error>> {
    let path = env::args()
        .nth(1)
        .unwrap_or_else(|| "cl1-study03.sqlite".to_owned());
    let revision =
        env::var("CL1_SOURCE_REVISION").unwrap_or_else(|_| "unverified-local-checkout".into());
    let mut laboratory = Laboratory::open(&path)?;
    let mut replays = 0_usize;
    for seed in FIRST_SEED..FIRST_SEED + REPLICAS {
        let world = seeded_world(seed);
        let outcomes = laboratory.run_scheduled(&world, &conditions(seed), HORIZON, &revision)?;
        if outcomes.len() != 5 {
            return Err(format!("incorrect condition count for seed {seed}").into());
        }
        if outcomes
            .iter()
            .any(|r| r.initial_sha256 != outcomes[0].initial_sha256)
        {
            return Err(format!("initial snapshot mismatch for seed {seed}").into());
        }
        for run in &outcomes {
            if !laboratory.verify_scheduled_replay(run.id)? {
                return Err(format!("invalid replay for seed {seed}: {}", run.label).into());
            }
            if run.world.occupied_units() > 160 {
                return Err(format!("storage bound exceeded for seed {seed}").into());
            }
            if run.label.ends_with("/zero-credit") && run.attempts != 0 {
                return Err(format!("zero-credit control failed for seed {seed}").into());
            }
            replays += 1;
        }
    }
    println!(
        "CL-1 Study 03: {REPLICAS} independent seeded worlds,          {replays} conditions and {replays} exact replays; path={path}"
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn next_edge(node: &Op) -> Option<usize> {
        match node {
            Op::Add { next, .. }
            | Op::Sub { next, .. }
            | Op::Read { next, .. }
            | Op::Write { next, .. }
            | Op::DrawCompute { next, .. } => Some(*next),
            _ => None,
        }
    }

    #[test]
    fn independent_seeds_have_exogenous_patchable_target() {
        for seed in FIRST_SEED..FIRST_SEED + REPLICAS {
            let world = seeded_world(seed);
            assert_eq!(world.occupied_units(), 51);
            assert_eq!(
                next_edge(&world.process(1).unwrap().code.nodes[8]),
                Some(9)
            );
        }
    }

    #[test]
    fn schedule_preserves_all_matched_negative_and_positive_controls() {
        let cases = conditions(FIRST_SEED);
        assert_eq!(cases.len(), 5);
        assert_eq!(cases[0].actions.len(), 4);
        assert_eq!(cases[1].actions[0].after_attempts, 4);
        assert_eq!(cases[2].actions[1].after_attempts, 24);
        assert_eq!(cases[4].actions.len(), 3);
    }

    #[test]
    fn constructed_small_replica_replays_all_conditions() {
        let mut lab = Laboratory::in_memory().unwrap();
        let outcomes = lab
            .run_scheduled(&seeded_world(FIRST_SEED), &conditions(FIRST_SEED), HORIZON, "test")
            .unwrap();
        assert_eq!(outcomes.len(), 5);
        assert!(outcomes
            .iter()
            .all(|r| lab.verify_scheduled_replay(r.id).unwrap()));
    }
}
