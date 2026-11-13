//! CL-1 Study 32: independent native compute-pool transfer experiment.
//! Researcher-endowed finite pool and initial code are NOT energy autonomy.
use cl0_lab::{EpochAction, EpochCondition, Laboratory};
use cl0_world::{EventKind, Intervention, Op, Origin, Process, World};
use std::{env, error::Error};

const FIRST_SEED: u64 = 2_500_000;
const REPLICAS: u64 = 192;
const PRE_ATTEMPTS: usize = 27;
const EPOCHS: usize = 24;
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
                0 => Op::GraphCopyFrom {
                    target: 3,
                    source: 1,
                    node: 2,
                    next,
                },
                1 => Op::GraphCopyNode {
                    graph: 3,
                    source: 2,
                    dst: 2,
                    next,
                },
                2 => Op::DrawCompute { amount: 1, next },
                3 => Op::GraphReadTag {
                    dst: 0,
                    graph: 3,
                    node: 2,
                    next,
                },
                4 => Op::GraphReadTag {
                    dst: 2,
                    graph: 1,
                    node: 2,
                    next,
                },
                5 => Op::Add {
                    dst: 0,
                    lhs: 0,
                    rhs: 2,
                    next,
                },
                6 | 7 => Op::Write {
                    address: 0,
                    value: 3,
                    next,
                },
                8 => Op::Const {
                    dst: 2,
                    value: 0,
                    next,
                },
                9 => Op::Branch {
                    test: 1,
                    nonzero: index - 2,
                    zero: next,
                },
                10 => Op::ReturnCompute { amount: 1, next },
                _ => Op::GraphGrantRead {
                    graph: 3,
                    reader: 1,
                    next,
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
#[cfg(test)]
fn first_nonempty_graph(world: &World) -> Option<(u64, u64)> {
    (1..=64).find_map(|id| {
        let object = world.graph_object(id)?;
        (!object.nodes.is_empty()).then_some((id, object.owner))
    })
}

fn prefork_world(seed: u64) -> World {
    let mut world = seeded_world(seed);
    world
        .intervene(Intervention::SetComputePool { credits: 48 })
        .expect("registered researcher-endowed world pool");
    assert_eq!(world.run(PRE_ATTEMPTS), PRE_ATTEMPTS);
    assert_eq!(world.available_compute_pool(), 48);
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

fn conditions(seed: u64, _world: &World) -> Vec<EpochCondition> {
    let at = |epoch, action| EpochAction { epoch, action };
    let zero_pool = at(0, Intervention::SetComputePool { credits: 0 });
    let refill = || {
        [8_usize, 16_usize]
            .into_iter()
            .flat_map(|epoch| {
                (1..=3).map(move |process| {
                    at(
                        epoch,
                        Intervention::SetCredits {
                            process,
                            credits: 47,
                        },
                    )
                })
            })
            .collect::<Vec<EpochAction>>()
    };
    let zero_credits = (1..=3)
        .map(|process| {
            at(
                0,
                Intervention::SetCredits {
                    process,
                    credits: 0,
                },
            )
        })
        .collect();
    let mut empty_pool_refill = vec![zero_pool.clone()];
    empty_pool_refill.extend(refill());
    vec![
        EpochCondition {
            label: format!("seed={seed}/control"),
            actions: vec![],
        },
        EpochCondition {
            label: format!("seed={seed}/sham"),
            actions: vec![at(0, Intervention::DropGraph { graph: 0 })],
        },
        EpochCondition {
            label: format!("seed={seed}/empty-pool"),
            actions: vec![zero_pool],
        },
        EpochCondition {
            label: format!("seed={seed}/refill"),
            actions: refill(),
        },
        EpochCondition {
            label: format!("seed={seed}/empty-pool-refill"),
            actions: empty_pool_refill,
        },
        EpochCondition {
            label: format!("seed={seed}/zero-credit"),
            actions: zero_credits,
        },
    ]
}

fn main() -> Result<(), Box<dyn Error>> {
    let path = env::args()
        .nth(1)
        .unwrap_or_else(|| "cl1-study32.sqlite".into());
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
        for graph in 1..=3 {
            let object = common
                .graph_object(graph)
                .ok_or("missing scaffold object")?;
            if object.nodes.is_empty() || object.owner != graph {
                return Err(format!("incorrect prefork scaffold for seed {seed}").into());
            }
        }
        for process in 1..=3 {
            if common.process(process).unwrap().credits != 47 {
                return Err(format!("incorrect prefork remaining budget for seed {seed}").into());
            }
        }
        eligible += 1;
        let matched = conditions(seed, &common);
        let runs = lab.run_epochs(&common, &matched, EPOCHS, QUOTA, &revision)?;
        if runs.len() != 6
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
            if run.label.ends_with("/sham")
                && !run.world.events().iter().any(|event| {
                    event.origin == Origin::External
                        && matches!(&event.kind, EventKind::InterventionRejected(s)
                            if s.contains("DropGraph"))
                })
            {
                return Err(format!("missing rejected sham: {}", run.label).into());
            }
            if run.label.contains("/empty-pool")
                && !run.world.events().iter().any(|event| {
                    event.origin == Origin::External
                        && matches!(&event.kind, EventKind::Intervened(s)
                            if s.contains("SetComputePool"))
                })
            {
                return Err(format!("missing explicit emptied pool: {}", run.label).into());
            }
            replayed += 1;
        }
    }
    println!(
        "CL-1 Study 32: {REPLICAS} seed-worlds, {eligible} eligible internally generated objects, \
         {replayed} matched runs and exact epoch replays; path={path}"
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn independent_native_code_mechanism_fixture_remains_replayable() {
        let mut fixture = World::with_strict_capacity(2, 16, 48);
        fixture
            .load(Process::new(
                1,
                vec![
                    Op::GraphNew { dst: 0, next: 1 },
                    Op::Const {
                        dst: 1,
                        value: 2,
                        next: 2,
                    },
                    Op::GraphGrantRead {
                        graph: 0,
                        reader: 1,
                        next: 3,
                    },
                    Op::GraphCopyNode {
                        graph: 0,
                        source: 2,
                        dst: 2,
                        next: 4,
                    },
                    Op::Halt,
                ],
                4,
            ))
            .unwrap();
        fixture
            .load(Process::new(
                2,
                vec![
                    Op::Const {
                        dst: 0,
                        value: 1,
                        next: 1,
                    },
                    Op::Const {
                        dst: 1,
                        value: 0,
                        next: 2,
                    },
                    Op::GraphReadTag {
                        dst: 2,
                        graph: 0,
                        node: 1,
                        next: 3,
                    },
                    Op::Const {
                        dst: 3,
                        value: 7,
                        next: 4,
                    },
                    Op::Write {
                        address: 2,
                        value: 3,
                        next: 5,
                    },
                    Op::Halt,
                ],
                0,
            ))
            .unwrap();
        assert_eq!(fixture.run(4), 4);
        let original = fixture.graph_object(1).unwrap().clone();
        assert!(matches!(original.nodes[0], Op::GraphNew { .. }));
        for changed in [false, true] {
            let mut world = fixture.clone();
            if changed {
                let mut replacement = original.clone();
                replacement.nodes[0] = Op::Halt;
                world
                    .intervene(Intervention::DropGraph { graph: 1 })
                    .unwrap();
                world
                    .intervene(Intervention::PutGraph {
                        graph: 1,
                        object: replacement,
                    })
                    .unwrap();
            } else {
                // Exact original-content reinjection is a separate negative control.
                world
                    .intervene(Intervention::DropGraph { graph: 1 })
                    .unwrap();
                world
                    .intervene(Intervention::PutGraph {
                        graph: 1,
                        object: original.clone(),
                    })
                    .unwrap();
            }
            world
                .intervene(Intervention::SetCredits {
                    process: 2,
                    credits: 5,
                })
                .unwrap();
            assert_eq!(world.run(5), 5);
            let target = if changed { 19 } else { 11 };
            assert_eq!(world.cell(target), Some(7));
            assert_eq!(world.cell(if changed { 11 } else { 19 }), None);
            assert!(world.events().iter().any(|event| {
                event.origin == Origin::Internal(2)
                    && matches!(&event.kind, EventKind::GraphTagRead { graph: 1, owner: 1, node: 0, tag }
                        if *tag == target)
            }));
        }
    }

    #[test]
    fn fork_is_deterministic_without_study_outcome_selection() {
        let a = prefork_world(FIRST_SEED);
        let b = prefork_world(FIRST_SEED);
        assert_eq!(a, b);
        assert_eq!(first_nonempty_graph(&a), first_nonempty_graph(&b));
    }

    #[test]
    fn preregistered_pool_manipulations_and_sham_are_fixed() {
        let common = prefork_world(FIRST_SEED);
        let selected = first_nonempty_graph(&common);
        let treatments = conditions(FIRST_SEED, &common);
        assert_eq!(treatments.len(), 6);
        assert!(treatments[0].actions.is_empty());
        assert!(matches!(
            treatments[1].actions[0].action,
            Intervention::DropGraph { graph: 0 }
        ));
        assert_eq!(treatments[2].actions[0].epoch, 0);
        assert_eq!(treatments[3].actions[0].epoch, 8);
        assert_eq!(treatments[4].actions[1].epoch, 8);
        assert_eq!(selected, Some((1, 1)));
    }

    #[test]
    fn full_small_seed_native_pool_experiment_is_replayable() {
        let mut lab = Laboratory::in_memory().unwrap();
        let common = prefork_world(FIRST_SEED);
        let treatments = conditions(FIRST_SEED, &common);
        let outcomes = lab
            .run_epochs(&common, &treatments, EPOCHS, QUOTA, "study32-test")
            .unwrap();
        assert_eq!(outcomes.len(), 6);
        assert!(outcomes
            .iter()
            .all(|r| lab.verify_epoch_replay(r.id).unwrap()));
        assert!(outcomes[5].attempts_by_epoch.iter().all(|&n| n == 0));
    }
}
