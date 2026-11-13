//! CL-1 Study 12: preregistered ablation of first internally created
//! nonempty graph object after an identical prefork history.
use cl0_lab::{EpochAction, EpochCondition, Laboratory};
use cl0_world::{EventKind, Intervention, Op, Origin, Process, World};
use std::{env, error::Error};

const FIRST_SEED: u64 = 1_810_000;
const REPLICAS: u64 = 96;
const PRE_ATTEMPTS: usize = 27;
const EPOCHS: usize = 16;
const QUOTA: usize = 12;

fn random(state: &mut u64) -> u64 {
    *state = state
        .wrapping_mul(6_364_136_223_846_793_005)
        .wrapping_add(1_442_695_040_888_963_407);
    *state
}

fn seeded_world(seed: u64) -> World {
    let mut state = seed;
    let mut world = World::with_strict_capacity(32, 40, 192);
    for id in 1..=3 {
        let predecessor = (id + 1) % 3 + 1;
        let mut nodes = vec![
            Op::GraphNew { dst: 3, next: 1 },
            Op::GraphCopyNode {
                graph: 3,
                source: 2,
                dst: 2,
                next: 2,
            },
            Op::Const {
                dst: 0,
                value: id as i64 - 1,
                next: 3,
            },
            Op::Write {
                address: 0,
                value: 3,
                next: 4,
            },
            Op::Const {
                dst: 1,
                value: predecessor as i64,
                next: 5,
            },
            Op::GraphGrantRead {
                graph: 3,
                reader: 1,
                next: 6,
            },
            Op::Const {
                dst: 0,
                value: (id % 3) as i64,
                next: 7,
            },
            Op::Read {
                dst: 1,
                address: 0,
                next: 8,
            },
            Op::Const {
                dst: 2,
                value: 0,
                next: 9,
            },
        ];
        for index in 9..27 {
            let next = index + 1;
            let op = match random(&mut state) % 12 {
                0 | 1 => Op::GraphCopyFrom {
                    target: 3,
                    source: 1,
                    node: 2,
                    next,
                },
                2 => Op::GraphCopyNode {
                    graph: 3,
                    source: 2,
                    dst: 2,
                    next,
                },
                3 => Op::GraphSetConst {
                    graph: 3,
                    node: 2,
                    value: 0,
                    next,
                },
                4 => Op::GraphSetNext {
                    graph: 3,
                    node: 2,
                    target: 2,
                    next,
                },
                5 => Op::GraphReadTag {
                    dst: 0,
                    graph: 1,
                    node: 2,
                    next,
                },
                6 => Op::GraphGrantRead {
                    graph: 3,
                    reader: 1,
                    next,
                },
                7 => Op::GraphTransfer {
                    graph: 3,
                    recipient: 1,
                    next,
                },
                8 => Op::GraphNew { dst: 3, next },
                9 => Op::Const {
                    dst: 2,
                    value: 0,
                    next,
                },
                10 => Op::Write {
                    address: 0,
                    value: 3,
                    next,
                },
                _ => Op::Branch {
                    test: 1,
                    nonzero: index - 2,
                    zero: next,
                },
            };
            nodes.push(op);
        }
        nodes.push(Op::Halt);
        world
            .load(Process::new(id, nodes, 56))
            .expect("registered initial graph fits");
    }
    world
}

/// Lowest *nonempty* internally constructed object after the common history.
fn first_nonempty_graph(world: &World) -> Option<(u64, u64)> {
    (1..=64).find_map(|id| {
        let object = world.graph_object(id)?;
        (!object.nodes.is_empty()).then_some((id, object.owner))
    })
}

fn prefork_world(seed: u64) -> World {
    let mut world = seeded_world(seed);
    assert_eq!(world.run(PRE_ATTEMPTS), PRE_ATTEMPTS);
    for id in 1..=3 {
        let graph = world
            .graph_object(id)
            .expect("scaffold must allocate graph");
        assert_eq!(graph.owner, id);
        assert_eq!(graph.nodes.len(), 1);
        assert!(graph.readers.contains(&((id + 1) % 3 + 1)));
        assert_eq!(world.cell(id as i64 - 1), Some(id as i64));
    }
    world
}

fn conditions(seed: u64, selected: Option<(u64, u64)>, world: &World) -> Vec<EpochCondition> {
    let (handle, owner) = selected.unwrap_or((0, 9999));
    let zero_credit = (1..=64)
        .filter(|&id| world.process(id).is_some())
        .map(|process| EpochAction {
            epoch: 0,
            action: Intervention::SetCredits {
                process,
                credits: 0,
            },
        })
        .collect();
    vec![
        EpochCondition {
            label: format!("seed={seed}/control"),
            actions: vec![],
        },
        EpochCondition {
            label: format!("seed={seed}/sham-drop"),
            actions: vec![EpochAction {
                epoch: 0,
                action: Intervention::DropGraph { graph: 0 },
            }],
        },
        EpochCondition {
            label: format!("seed={seed}/target-drop"),
            actions: vec![EpochAction {
                epoch: 0,
                action: Intervention::DropGraph { graph: handle },
            }],
        },
        EpochCondition {
            label: format!("seed={seed}/disable-owner"),
            actions: vec![EpochAction {
                epoch: 0,
                action: Intervention::Disable { process: owner },
            }],
        },
        EpochCondition {
            label: format!("seed={seed}/zero-credit"),
            actions: zero_credit,
        },
    ]
}

fn main() -> Result<(), Box<dyn Error>> {
    let path = env::args()
        .nth(1)
        .unwrap_or_else(|| "cl1-study12.sqlite".into());
    let revision =
        env::var("CL1_SOURCE_REVISION").unwrap_or_else(|_| "unverified-local-checkout".into());
    let mut lab = Laboratory::open(&path)?;
    let mut replayed = 0_usize;
    let mut eligible = 0_usize;
    for seed in FIRST_SEED..FIRST_SEED + REPLICAS {
        let common = prefork_world(seed);
        let prefix: Vec<&EventKind> = common
            .events()
            .iter()
            .filter(|e| e.origin == Origin::Internal(e.process))
            .map(|e| &e.kind)
            .collect();
        if prefix
            .iter()
            .filter(|kind| matches!(kind, EventKind::GraphCreated { .. }))
            .count()
            != 3
            || prefix
                .iter()
                .filter(|kind| matches!(kind, EventKind::GraphNodeCopied { .. }))
                .count()
                != 3
        {
            return Err(format!("scaffold lineage not observed for seed {seed}").into());
        }
        let selected = first_nonempty_graph(&common);
        eligible += usize::from(selected.is_some());
        let matched = conditions(seed, selected, &common);
        let runs = lab.run_epochs(&common, &matched, EPOCHS, QUOTA, &revision)?;
        if runs.len() != 5
            || runs
                .iter()
                .any(|r| r.initial_sha256 != runs[0].initial_sha256)
        {
            return Err(format!("mismatched prefork world: seed={seed}").into());
        }
        for run in runs {
            if !lab.verify_epoch_replay(run.id)? {
                return Err(format!("full epoch replay failed for {}", run.label).into());
            }
            if run.world.occupied_units() > 192 {
                return Err(format!("storage limit violated for {}", run.label).into());
            }
            if run.label.ends_with("/zero-credit")
                && run.attempts_by_epoch.iter().sum::<usize>() != 0
            {
                return Err(format!("zero-credit control executed CIL: {}", run.label).into());
            }
            if run.label.ends_with("/target-drop") {
                let has_success = run.world.events().iter().any(|event| {
                    event.origin == Origin::External
                        && matches!(
                            &event.kind,
                            EventKind::Intervened(text) if text.contains("DropGraph")
                        )
                });
                if has_success != selected.is_some() {
                    return Err(format!("graph exposure invalid for {}", run.label).into());
                }
            }
            replayed += 1;
        }
    }
    println!(
        "CL-1 Study 12: {REPLICAS} seed-worlds, {eligible} eligible internally generated objects, \
         {replayed} matched runs and exact epoch replays; path={path}"
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fork_is_deterministic_without_study_outcome_selection() {
        let a = prefork_world(FIRST_SEED);
        let b = prefork_world(FIRST_SEED);
        assert_eq!(a, b);
        assert_eq!(first_nonempty_graph(&a), first_nonempty_graph(&b));
    }

    #[test]
    fn matched_target_and_sham_conditions_are_precommitted() {
        let common = prefork_world(FIRST_SEED);
        let selected = first_nonempty_graph(&common);
        let treatments = conditions(FIRST_SEED, selected, &common);
        assert_eq!(treatments.len(), 5);
        assert!(treatments[0].actions.is_empty());
        assert!(matches!(
            treatments[1].actions[0].action,
            Intervention::DropGraph { graph: 0 }
        ));
        assert_eq!(treatments[2].actions[0].epoch, 0);
        assert_eq!(selected, Some((1, 1)));
    }

    #[test]
    fn full_small_seed_experiment_is_replayable() {
        let mut lab = Laboratory::in_memory().unwrap();
        let common = prefork_world(FIRST_SEED);
        let treatments = conditions(FIRST_SEED, first_nonempty_graph(&common), &common);
        let outcomes = lab
            .run_epochs(&common, &treatments, EPOCHS, QUOTA, "study12-test")
            .unwrap();
        assert_eq!(outcomes.len(), 5);
        assert!(outcomes
            .iter()
            .all(|r| lab.verify_epoch_replay(r.id).unwrap()));
        assert!(outcomes[4].attempts_by_epoch.iter().all(|&n| n == 0));
    }
}
