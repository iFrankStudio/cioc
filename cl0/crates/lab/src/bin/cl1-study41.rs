//! CL-1 Study 41: independent native compute-pool transfer experiment.
//! Researcher-endowed finite pool and initial code are NOT energy autonomy.
use cl0_lab::{EpochAction, EpochCondition, Laboratory};
use cl0_world::{Intervention, Op, Process, World};
use std::{env, error::Error};

const FIRST_SEED: u64 = 3_100_000;
const REPLICAS: u64 = 128;
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
    let mut world = World::with_strict_capacity(32, 64, 256);
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
        for index in 9..36 {
            let next = index + 1;
            let op = match random(&mut state) % 20 {
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
                11 => Op::GraphGrantRead {
                    graph: 3,
                    reader: 1,
                    next,
                },
                12 => Op::CloneNode {
                    dst: 2,
                    source: 2,
                    next,
                },
                13 => Op::PatchNext {
                    node: 2,
                    target: 0,
                    next,
                },
                14 => Op::PatchConst {
                    node: 2,
                    value: 0,
                    next,
                },
                15 => Op::ReadNext {
                    dst: 2,
                    node: 0,
                    next,
                },
                16 => Op::GraphSetNext {
                    graph: 3,
                    node: 2,
                    target: 0,
                    next,
                },
                17 => Op::GraphSetConst {
                    graph: 3,
                    node: 2,
                    value: 0,
                    next,
                },
                18 => Op::GraphLaunch {
                    graph: 3,
                    credits: 1,
                    dst: 2,
                    next,
                },
                _ => Op::GraphEncode {
                    graph: 3,
                    address: 0,
                    dst: 2,
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

/// Neutral external scheduler; no checks of native CIL opcode or fitness.
fn neutral_candidate(world: &World) -> Option<(u64, u64)> {
    // The strict world storage capacity is 256 units. Each child costs >=1 unit
    // and ProcessId is monotone from 1, so an existing ID cannot exceed 256.
    if (1..=256).any(|id| {
        world
            .process(id)
            .is_some_and(|p| !p.stopped && p.credits > 0)
    }) {
        return None;
    }
    let pool = world.available_compute_pool();
    if pool == 0 {
        return None;
    }
    for id in 1..=256 {
        if world
            .process(id)
            .is_some_and(|p| !p.stopped && p.credits == 0)
        {
            return Some((id, pool));
        }
    }
    None
}

fn base_conditions(seed: u64) -> Vec<(String, bool, bool, Vec<EpochAction>)> {
    let at = |epoch, action| EpochAction { epoch, action };
    let zero = || {
        (1..=3)
            .map(|process| {
                at(
                    0,
                    Intervention::SetCredits {
                        process,
                        credits: 0,
                    },
                )
            })
            .collect::<Vec<_>>()
    };
    [
        ("ambient-general", true, true, vec![]),
        ("ambient-only", true, false, vec![]),
        ("closed-general", false, true, vec![]),
        ("zero-credit-ambient-general", true, true, zero()),
    ]
    .into_iter()
    .map(|(arm, ambient, ignition, actions)| {
        (format!("seed={seed}/{arm}"), ambient, ignition, actions)
    })
    .collect()
}

fn make_plan(
    world: &World,
    base: &[EpochAction],
    ambient: bool,
    ignition: bool,
) -> Result<(Vec<EpochAction>, World), Box<dyn Error>> {
    let mut predicted: World = serde_json::from_str(&serde_json::to_string(world)?)?;
    let mut plan = Vec::new();
    for epoch in 0..EPOCHS {
        for action in base.iter().filter(|a| a.epoch == epoch) {
            let _ = predicted.intervene(action.action.clone());
            plan.push(action.clone());
        }
        if ambient && epoch >= 1 {
            let before = predicted.available_compute_pool();
            let after = before
                .checked_add(8)
                .ok_or("ambient compute pool overflow")?;
            let input = EpochAction {
                epoch,
                action: Intervention::SetComputePool { credits: after },
            };
            predicted.intervene(input.action.clone())?;
            plan.push(input);
        }
        if ignition {
            if let Some((id, pool)) = neutral_candidate(&predicted) {
                let debit = EpochAction {
                    epoch,
                    action: Intervention::SetComputePool { credits: pool - 1 },
                };
                let credit = EpochAction {
                    epoch,
                    action: Intervention::SetCredits {
                        process: id,
                        credits: 1,
                    },
                };
                predicted.intervene(debit.action.clone())?;
                predicted.intervene(credit.action.clone())?;
                plan.extend([debit, credit]);
            }
        }
        predicted.run(QUOTA);
    }
    Ok((plan, predicted))
}

fn main() -> Result<(), Box<dyn Error>> {
    let path = env::args()
        .nth(1)
        .unwrap_or_else(|| "cl1-study41.sqlite".into());
    let revision =
        env::var("CL1_SOURCE_REVISION").unwrap_or_else(|_| "unverified-local-checkout".into());
    let mut lab = Laboratory::open(&path)?;
    let mut total = 0_usize;
    for seed in FIRST_SEED..FIRST_SEED + REPLICAS {
        let initial = prefork_world(seed);
        assert_eq!(initial.total_compute_inventory(), Some(189));
        let mut conditions = Vec::new();
        let mut predicted = Vec::new();
        for (label, ambient, ignition, base) in base_conditions(seed) {
            let (actions, expected) = make_plan(&initial, &base, ambient, ignition)?;
            conditions.push(EpochCondition { label, actions });
            predicted.push(expected);
        }
        let runs = lab.run_epochs(&initial, &conditions, EPOCHS, QUOTA, &revision)?;
        if runs.len() != 4 {
            return Err(format!("wrong matched arm count: seed={seed}").into());
        }
        for (run, expected) in runs.into_iter().zip(predicted) {
            if run.world != expected {
                return Err(
                    format!("predicted world differs from native replay: {}", run.label).into(),
                );
            }
            if !lab.verify_epoch_replay(run.id)? {
                return Err(format!("full epoch replay failed for {}", run.label).into());
            }
            if run.world.occupied_units() > 256 {
                return Err(format!("storage limit violated: {}", run.label).into());
            }
            total += 1;
        }
    }
    println!(
        "CL-1 Study41: {REPLICAS} new worlds, {total} native matched runs, {} epoch SHA256 snapshots; {path}",
        total * EPOCHS
    );
    Ok(())
}
