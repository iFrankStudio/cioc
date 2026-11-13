//! CL-1 Study 01: strictly preregistered synthetic information ecology.
//! See cl1/studies/01-preregistered-random-ecology.md.
//! Seeded programs are investigator-generated and do NOT demonstrate life.
use cl0_lab::{Laboratory, ScheduledAction, ScheduledCondition};
use cl0_world::{EventKind, Intervention, Op, Origin, Process, World};
use std::{env, error::Error};

const FIRST_SEED: u64 = 610_000;
const REPLICAS: u64 = 64;
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
    vec![
        ScheduledCondition {
            label: format!("seed={seed}/control"),
            actions: vec![],
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
        ScheduledCondition {
            label: format!("seed={seed}/disable-p1"),
            actions: vec![action(12, Intervention::Disable { process: 1 })],
        },
        ScheduledCondition {
            label: format!("seed={seed}/lesion-cell"),
            actions: vec![action(
                12,
                Intervention::SetCell {
                    key: 0,
                    value: 9999,
                },
            )],
        },
        ScheduledCondition {
            label: format!("seed={seed}/sham-cell"),
            actions: vec![action(
                12,
                Intervention::SetCell {
                    key: 1_000_000,
                    value: 9999,
                },
            )],
        },
    ]
}

fn main() -> Result<(), Box<dyn Error>> {
    let path = env::args()
        .nth(1)
        .unwrap_or_else(|| "cl1-study01.sqlite".to_owned());
    let revision =
        env::var("CL1_SOURCE_REVISION").unwrap_or_else(|_| "unverified-local-checkout".into());
    let mut lab = Laboratory::open(&path)?;
    let mut replays = 0_usize;
    for seed in FIRST_SEED..FIRST_SEED + REPLICAS {
        let baseline = seeded_world(seed);
        let runs = lab.run_scheduled(&baseline, &conditions(seed), HORIZON, &revision)?;
        if runs.len() != 5 {
            return Err("study condition count invalid".into());
        }
        let same_baseline = runs
            .iter()
            .all(|r| r.initial_sha256 == runs[0].initial_sha256);
        if !same_baseline {
            return Err(format!("matched initial worlds differ for seed {seed}").into());
        }
        for run in &runs {
            if !lab.verify_scheduled_replay(run.id)? {
                return Err(format!("replay failed for seed {seed}: {}", run.label).into());
            }
            if run.world.occupied_units() > 160 {
                return Err(format!("storage exceeded for seed {seed}").into());
            }
            if run.label.ends_with("/zero-credit")
                && (run.attempts != 0
                    || run
                        .world
                        .events()
                        .iter()
                        .any(|event| matches!(event.origin, Origin::Internal(_))))
            {
                return Err(format!("zero-credit control violated for seed {seed}").into());
            }
            if run
                .world
                .events()
                .iter()
                .any(|event| matches!(&event.kind, EventKind::Trapped(_)))
            {
                // Failures remain in the record and the denominator.
            }
            replays += 1;
        }
    }
    println!(
        "CL-1 Study 01: {REPLICAS} independent seeded worlds,          {replays} matched runs, {replays} exact replays passed; path={path}"
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seeded_inputs_are_reproducible_but_independent() {
        assert_eq!(seeded_world(FIRST_SEED), seeded_world(FIRST_SEED));
        assert_ne!(seeded_world(FIRST_SEED), seeded_world(FIRST_SEED + 1));
        assert_eq!(seeded_world(FIRST_SEED).occupied_units(), 51);
    }

    #[test]
    fn five_conditions_are_fixed_and_timed() {
        let cases = conditions(FIRST_SEED);
        assert_eq!(cases.len(), 5);
        assert_eq!(cases[0].actions.len(), 0);
        assert_eq!(cases[1].actions.len(), 3);
        assert!(cases[1].actions.iter().all(|a| a.after_attempts == 0));
        assert!(cases[2..]
            .iter()
            .all(|case| case.actions[0].after_attempts == 12));
    }

    #[test]
    fn original_control_is_replayable_without_host_selection() {
        let mut lab = Laboratory::in_memory().unwrap();
        let baseline = seeded_world(FIRST_SEED);
        let runs = lab
            .run_scheduled(
                &baseline,
                &conditions(FIRST_SEED),
                HORIZON,
                "cl1-study01-test",
            )
            .unwrap();
        assert_eq!(runs.len(), 5);
        assert!(runs
            .iter()
            .all(|r| lab.verify_scheduled_replay(r.id).unwrap()));
        assert_eq!(runs[1].attempts, 0);
    }
}
