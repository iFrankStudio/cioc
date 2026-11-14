//! CL-1 Study 39: independent native compute-pool transfer experiment.
//! Researcher-endowed finite pool and initial code are NOT energy autonomy.
use cl0_lab::{EpochAction, EpochCondition, Laboratory};
use cl0_world::{Intervention, Op, Process, World};
use std::{env, error::Error};

const FIRST_SEED: u64 = 3_000_000;
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

/// This is a generic EXTERNAL environment access policy, not native repair.
#[derive(Clone, Copy)]
enum Ignition {
    None,
    Draw,
    General,
}

fn ignition_candidate(world: &World, mode: Ignition) -> Option<(u64, u64)> {
    if matches!(mode, Ignition::None)
        || (1..=3).any(|id| {
            world
                .process(id)
                .is_some_and(|p| !p.stopped && p.credits > 0)
        })
    {
        return None;
    }
    let pool = world.available_compute_pool();
    for id in 1..=3 {
        let Some(p) = world.process(id) else {
            continue;
        };
        if p.stopped || p.credits != 0 {
            continue;
        }
        match mode {
            Ignition::General if pool >= 1 => return Some((id, pool)),
            Ignition::Draw => {
                let Some(Op::DrawCompute { amount, .. }) = p.code.nodes.get(p.pc) else {
                    continue;
                };
                let Some(&raw) = p.registers.get(*amount) else {
                    continue;
                };
                let Ok(q) = u64::try_from(raw) else {
                    continue;
                };
                if q > 0 && pool > q {
                    return Some((id, pool));
                }
            }
            _ => {}
        }
    }
    None
}

fn base_conditions(seed: u64) -> Vec<(String, bool, Ignition, Vec<EpochAction>)> {
    let at = |epoch, action| EpochAction { epoch, action };
    let empty = || vec![at(0, Intervention::SetComputePool { credits: 0 })];
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
    let refill = || {
        [8, 16]
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
            .collect::<Vec<_>>()
    };
    [
        ("closed", false, Ignition::None, vec![]),
        ("closed-draw", false, Ignition::Draw, vec![]),
        ("closed-general", false, Ignition::General, vec![]),
        ("ambient", true, Ignition::None, vec![]),
        ("ambient-draw", true, Ignition::Draw, vec![]),
        ("ambient-general", true, Ignition::General, vec![]),
        (
            "ambient-general-empty-pool",
            true,
            Ignition::General,
            empty(),
        ),
        (
            "ambient-general-zero-credit",
            true,
            Ignition::General,
            zero(),
        ),
        ("external-refill", false, Ignition::None, refill()),
        (
            "sham",
            false,
            Ignition::None,
            vec![at(0, Intervention::DropGraph { graph: 0 })],
        ),
    ]
    .into_iter()
    .map(|(arm, ambient, ignition, actions)| {
        (format!("seed={seed}/{arm}"), ambient, ignition, actions)
    })
    .collect()
}

/// Construct an exact, state-dependent plan from no fitness/goal information.
/// The existing laboratory executes and independently replays this plan.
fn make_plan(
    world: &World,
    base: &[EpochAction],
    ambient: bool,
    ignition: Ignition,
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
        if let Some((id, pool)) = ignition_candidate(&predicted, ignition) {
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
        predicted.run(QUOTA);
    }
    Ok((plan, predicted))
}

fn main() -> Result<(), Box<dyn Error>> {
    let path = env::args()
        .nth(1)
        .unwrap_or_else(|| "cl1-study39.sqlite".into());
    let revision =
        env::var("CL1_SOURCE_REVISION").unwrap_or_else(|_| "unverified-local-checkout".into());
    let mut lab = Laboratory::open(&path)?;
    let mut total = 0_usize;
    let mut external_starters = 0_usize;
    for seed in FIRST_SEED..FIRST_SEED + REPLICAS {
        let initial = prefork_world(seed);
        assert_eq!(initial.total_compute_inventory(), Some(189));
        let mut conditions = Vec::new();
        let mut predicted = Vec::new();
        for (label, ambient, ignition, base) in base_conditions(seed) {
            let (actions, expected) = make_plan(&initial, &base, ambient, ignition)?;
            let ambient_count = if ambient { EPOCHS - 1 } else { 0 };
            external_starters += (actions.len() - base.len() - ambient_count) / 2;
            conditions.push(EpochCondition { label, actions });
            predicted.push(expected);
        }
        let runs = lab.run_epochs(&initial, &conditions, EPOCHS, QUOTA, &revision)?;
        if runs.len() != 10 {
            return Err(format!("wrong matched arm count: seed={seed}").into());
        }
        for (run, expected) in runs.into_iter().zip(predicted) {
            if run.world != expected {
                return Err(
                    format!("predicted environment differs from replay: {}", run.label).into(),
                );
            }
            if !lab.verify_epoch_replay(run.id)? {
                return Err(format!("failed full native epoch replay: {}", run.label).into());
            }
            if run.world.occupied_units() > 192 {
                return Err(format!("storage ceiling violated: {}", run.label).into());
            }
            total += 1;
        }
    }
    println!(
        "CL-1 Study39: {REPLICAS} independent worlds, {total} matched runs,          {} hashed replayed epochs, {external_starters} external ignition transfers; {path}",
        total * EPOCHS
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture(next: Op, credits: u64, pool: u64, stopped: bool) -> World {
        let mut w = World::with_strict_capacity(1, 5, 32);
        let mut process = Process::new(1, vec![next, Op::Halt], credits);
        process.registers[1] = 2;
        w.load(process).unwrap();
        w.intervene(Intervention::SetComputePool { credits: pool })
            .unwrap();
        if stopped {
            w.intervene(Intervention::Disable { process: 1 }).unwrap();
        }
        w
    }

    #[test]
    fn general_environment_ignites_zero_credit_actor_without_inspecting_code() {
        let w = fixture(
            Op::Const {
                dst: 0,
                value: 1,
                next: 1,
            },
            0,
            7,
            false,
        );
        assert_eq!(ignition_candidate(&w, Ignition::General), Some((1, 7)));
        assert!(ignition_candidate(&w, Ignition::Draw).is_none());
        let (plan, after) = make_plan(&w, &[], false, Ignition::General).unwrap();
        assert_eq!(plan.len(), 4);
        assert_eq!(after.total_compute_inventory(), Some(5));
    }

    #[test]
    fn draw_gated_access_requires_a_real_native_draw_and_pool_headroom() {
        let w = fixture(Op::DrawCompute { amount: 1, next: 1 }, 0, 7, false);
        assert_eq!(ignition_candidate(&w, Ignition::Draw), Some((1, 7)));
        assert!(ignition_candidate(
            &fixture(Op::DrawCompute { amount: 1, next: 1 }, 0, 2, false),
            Ignition::Draw
        )
        .is_none());
        assert!(ignition_candidate(
            &fixture(Op::DrawCompute { amount: 1, next: 1 }, 0, 7, true),
            Ignition::General
        )
        .is_none());
    }

    #[test]
    fn ambient_input_occurs_before_external_ignition_and_is_observable() {
        let w = fixture(
            Op::Const {
                dst: 0,
                value: 1,
                next: 1,
            },
            0,
            0,
            false,
        );
        let (plan, after) = make_plan(&w, &[], true, Ignition::General).unwrap();
        assert_eq!(plan[0].epoch, 1);
        assert!(matches!(
            plan[0].action,
            Intervention::SetComputePool { credits: 8 }
        ));
        assert_eq!(plan[1].epoch, 1);
        assert_eq!(after.total_compute_inventory(), Some(23 * 8 - 2));
    }

    #[test]
    fn ten_arms_are_frozen_and_do_not_mutate_common_prefork() {
        let w = prefork_world(FIRST_SEED);
        let arms = base_conditions(FIRST_SEED);
        assert_eq!(arms.len(), 10);
        for (_, ambient, ignition, base) in arms {
            let _ = make_plan(&w, &base, ambient, ignition).unwrap();
            assert_eq!(w.total_compute_inventory(), Some(189));
        }
    }
}
