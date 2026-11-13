//! CL-1 Study 02: preregistered reciprocal causal-sensitivity audit.
//! The seeded generator is an exact copy of Study 01, with fresh seeds.
use cl0_lab::{Laboratory, ScheduledAction, ScheduledCondition};
use cl0_world::{Intervention, Op, Process, World};
use std::{env, error::Error};

const FIRST_SEED: u64 = 720_000;
const REPLICAS: u64 = 128;
const HORIZON: usize = 96;

fn random(state: &mut u64) -> u64 {
    *state = state
        .wrapping_mul(6_364_136_223_846_793_005)
        .wrapping_add(1_442_695_040_888_963_407);
    *state
}

/// Initial CIL is externally authored with a fixed uniform 12-way choice.
/// The generator does not inspect any candidate's subsequent performance.
fn seeded_world(seed: u64) -> World {
    let mut state = seed;
    let mut world = World::with_strict_capacity(32, 32, 160);
    for id in 1..=3 {
        let key = (random(&mut state) % 5) as i64;
        let value = (random(&mut state) % 101) as i64;
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
                value: 1,
                next: 3,
            },
            Op::Const {
                dst: 3,
                value: 1,
                next: 4,
            },
        ];
        for index in 4..15 {
            let next = index + 1;
            let op = match random(&mut state) % 12 {
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
                5 => Op::CloneNode {
                    dst: 3,
                    source: 2,
                    next,
                },
                6 => Op::PatchConst {
                    node: 2,
                    value: 1,
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
            };
            nodes.push(op);
        }
        nodes.push(Op::Halt);
        world
            .load(Process::new(id, nodes, 28))
            .expect("preregistered initial allocation must fit");
    }
    world
}

fn conditions(seed: u64) -> Vec<ScheduledCondition> {
    let action = |after_attempts, action| ScheduledAction {
        after_attempts,
        action,
    };
    let mut cases = vec![ScheduledCondition {
        label: format!("seed={seed}/control"),
        actions: vec![],
    }];
    for id in 1..=3 {
        cases.push(ScheduledCondition {
            label: format!("seed={seed}/disable-p{id}"),
            actions: vec![action(12, Intervention::Disable { process: id })],
        });
    }
    cases.push(ScheduledCondition {
        label: format!("seed={seed}/sham-disable"),
        actions: vec![action(12, Intervention::Disable { process: 9999 })],
    });
    cases.push(ScheduledCondition {
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
    });
    cases
}

fn main() -> Result<(), Box<dyn Error>> {
    let path = env::args()
        .nth(1)
        .unwrap_or_else(|| "cl1-study02.sqlite".to_owned());
    let revision =
        env::var("CL1_SOURCE_REVISION").unwrap_or_else(|_| "unverified-local-checkout".into());
    let mut lab = Laboratory::open(&path)?;
    let mut replayed = 0_usize;
    for seed in FIRST_SEED..FIRST_SEED + REPLICAS {
        let base = seeded_world(seed);
        let runs = lab.run_scheduled(&base, &conditions(seed), HORIZON, &revision)?;
        if runs.len() != 6 {
            return Err(format!("incorrect condition count for seed {seed}").into());
        }
        if runs
            .iter()
            .any(|run| run.initial_sha256 != runs[0].initial_sha256)
        {
            return Err(format!("unmatched initial hash for seed {seed}").into());
        }
        for run in &runs {
            if !lab.verify_scheduled_replay(run.id)? {
                return Err(format!("replay failed for seed {seed}: {}", run.label).into());
            }
            if run.world.occupied_units() > 160 {
                return Err(format!("storage exceeded for seed {seed}").into());
            }
            if run.label.ends_with("/zero-credit") && run.attempts != 0 {
                return Err(format!("zero-credit control executed for seed {seed}").into());
            }
            replayed += 1;
        }
    }
    println!(
        "CL-1 Study 02: {REPLICAS} independent seeded worlds,          {replayed} conditions and {replayed} exact replays; path={path}"
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn independent_seeded_worlds_have_fixed_storage() {
        assert_eq!(seeded_world(FIRST_SEED).occupied_units(), 51);
        assert_eq!(seeded_world(FIRST_SEED), seeded_world(FIRST_SEED));
        assert_ne!(seeded_world(FIRST_SEED), seeded_world(FIRST_SEED + 1));
    }

    #[test]
    fn all_ablation_controls_are_present() {
        let cases = conditions(FIRST_SEED);
        assert_eq!(cases.len(), 6);
        assert_eq!(cases[0].actions.len(), 0);
        assert_eq!(cases[4].actions[0].after_attempts, 12);
        assert!(cases[5].actions.iter().all(|a| a.after_attempts == 0));
    }
}
