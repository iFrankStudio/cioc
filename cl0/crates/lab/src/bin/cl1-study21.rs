//! Study 21: observed loss followed by externally performed temporal restoration.
//! Entire mechanism is investigator-authored. Never evidence of native repair.
use cl0_lab::{EpochAction, EpochCondition, Laboratory};
use cl0_world::{EventKind, GraphObject, Intervention, Op, Origin, Process, World};
use std::{env, error::Error};

const EPOCHS: usize = 3;
const QUOTA: usize = 12;

fn owner(id: u64, source_index: i64) -> Process {
    Process::new(
        id,
        vec![
            Op::GraphNew { dst: 0, next: 1 },
            Op::Const {
                dst: 1,
                value: 3,
                next: 2,
            },
            Op::GraphGrantRead {
                graph: 0,
                reader: 1,
                next: 3,
            },
            Op::Const {
                dst: 1,
                value: 4,
                next: 4,
            },
            Op::GraphGrantRead {
                graph: 0,
                reader: 1,
                next: 5,
            },
            Op::Const {
                dst: 1,
                value: 5,
                next: 6,
            },
            Op::GraphGrantRead {
                graph: 0,
                reader: 1,
                next: 7,
            },
            Op::Const {
                dst: 2,
                value: source_index,
                next: 8,
            },
            Op::GraphCopyNode {
                graph: 0,
                source: 2,
                dst: 2,
                next: 9,
            },
            Op::Halt,
        ],
        9,
    )
}

fn reader(id: u64) -> Process {
    Process::new(
        id,
        vec![
            Op::Const {
                dst: 0,
                value: 1,
                next: 1,
            },
            Op::Const {
                dst: 1,
                value: 2,
                next: 2,
            },
            Op::Const {
                dst: 2,
                value: 0,
                next: 3,
            },
            Op::GraphReadTag {
                dst: 3,
                graph: 0,
                node: 2,
                next: 4,
            },
            Op::GraphReadTag {
                dst: 2,
                graph: 1,
                node: 2,
                next: 5,
            },
            Op::Add {
                dst: 3,
                lhs: 3,
                rhs: 2,
                next: 6,
            },
            Op::Const {
                dst: 0,
                value: 90 + (id - 3) as i64,
                next: 7,
            },
            Op::Write {
                address: 0,
                value: 3,
                next: 8,
            },
            Op::Halt,
        ],
        0,
    )
}

fn common_world() -> World {
    let mut world = World::with_strict_capacity(4, 16, 96);
    world.load(owner(1, 0)).expect("owner1");
    world.load(owner(2, 1)).expect("owner2");
    for id in 3..=5 {
        world.load(reader(id)).expect("measurement process");
    }
    assert_eq!(world.run(18), 18);
    for graph in 1..=2 {
        let object = world.graph_object(graph).expect("prefork graph");
        assert_eq!(object.owner, graph);
        assert_eq!(object.nodes.len(), 1);
        for reader in 3..=5 {
            assert!(object.readers.contains(&reader));
        }
    }
    assert!(matches!(
        world.graph_object(1).unwrap().nodes[0],
        Op::GraphNew { .. }
    ));
    assert!(matches!(
        world.graph_object(2).unwrap().nodes[0],
        Op::Const { .. }
    ));
    for id in 3..=5 {
        assert_eq!(world.process(id).unwrap().credits, 0);
        assert!(world.cell(90 + (id - 3) as i64).is_none());
    }
    world
}

fn at(epoch: usize, action: Intervention) -> EpochAction {
    EpochAction { epoch, action }
}

fn conditions(common: &World) -> Vec<EpochCondition> {
    let saved1: GraphObject = common.graph_object(1).unwrap().clone();
    let saved2: GraphObject = common.graph_object(2).unwrap().clone();
    let mut mutated2 = saved2.clone();
    mutated2.nodes[0] = Op::Halt;
    let plans: Vec<(&str, Vec<EpochAction>)> = vec![
        ("control", vec![]),
        ("sham", vec![at(0, Intervention::DropGraph { graph: 0 })]),
        ("drop1", vec![at(0, Intervention::DropGraph { graph: 1 })]),
        ("drop2", vec![at(0, Intervention::DropGraph { graph: 2 })]),
        (
            "drop-both",
            vec![
                at(0, Intervention::DropGraph { graph: 1 }),
                at(0, Intervention::DropGraph { graph: 2 }),
            ],
        ),
        (
            "restore1",
            vec![
                at(0, Intervention::DropGraph { graph: 1 }),
                at(
                    1,
                    Intervention::PutGraph {
                        graph: 1,
                        object: saved1,
                    },
                ),
            ],
        ),
        (
            "restore2",
            vec![
                at(0, Intervention::DropGraph { graph: 2 }),
                at(
                    1,
                    Intervention::PutGraph {
                        graph: 2,
                        object: saved2.clone(),
                    },
                ),
            ],
        ),
        (
            "restore-both",
            vec![
                at(0, Intervention::DropGraph { graph: 1 }),
                at(0, Intervention::DropGraph { graph: 2 }),
                at(
                    1,
                    Intervention::PutGraph {
                        graph: 1,
                        object: common.graph_object(1).unwrap().clone(),
                    },
                ),
                at(
                    1,
                    Intervention::PutGraph {
                        graph: 2,
                        object: saved2,
                    },
                ),
            ],
        ),
        (
            "mutate2",
            vec![
                at(0, Intervention::DropGraph { graph: 2 }),
                at(
                    1,
                    Intervention::PutGraph {
                        graph: 2,
                        object: mutated2,
                    },
                ),
            ],
        ),
        (
            "no-repair",
            vec![at(0, Intervention::DropGraph { graph: 1 })],
        ),
        ("zero-credit", vec![]),
    ];
    plans
        .into_iter()
        .map(|(label, mut actions)| {
            for epoch in 0..EPOCHS {
                actions.push(at(
                    epoch,
                    Intervention::SetCredits {
                        process: epoch as u64 + 3,
                        credits: if label == "zero-credit" { 0 } else { 9 },
                    },
                ));
            }
            // The laboratory only accepts chronologically sorted schedules.
            actions.sort_by_key(|action| action.epoch);
            EpochCondition {
                label: label.into(),
                actions,
            }
        })
        .collect()
}

fn expected(label: &str) -> Result<[Option<i64>; 3], Box<dyn Error>> {
    match label {
        "control" | "sham" => Ok([Some(11), Some(11), Some(11)]),
        "restore1" | "restore2" | "restore-both" => Ok([None, Some(11), Some(11)]),
        "mutate2" => Ok([None, Some(30), Some(30)]),
        "drop1" | "drop2" | "drop-both" | "no-repair" | "zero-credit" => Ok([None, None, None]),
        _ => Err("unregistered condition label".into()),
    }
}

fn run(path: &str, revision: &str) -> Result<(), Box<dyn Error>> {
    let common = common_world();
    let mut lab = Laboratory::open(path)?;
    let runs = lab.run_epochs(&common, &conditions(&common), EPOCHS, QUOTA, revision)?;
    if runs.len() != 11
        || runs
            .iter()
            .any(|record| record.initial_sha256 != runs[0].initial_sha256)
    {
        return Err("missing matched arms or diverged prefork state".into());
    }
    for record in &runs {
        if !lab.verify_epoch_replay(record.id)? || record.world.occupied_units() > 96 {
            return Err(format!("snapshot/resource/replay failure: {}", record.label).into());
        }
        let expected_values = expected(&record.label)?;
        for (epoch, value) in expected_values.iter().enumerate() {
            let key = epoch as i64 + 90;
            if record.world.cell(key) != *value {
                return Err(
                    format!("unregistered outcome at {} epoch {epoch}", record.label).into(),
                );
            }
            let matching_writes = record
                .world
                .events()
                .iter()
                .filter(|e| {
                    e.origin == Origin::Internal(epoch as u64 + 3)
                        && matches!(
                            e.kind,
                            EventKind::CellWritten { key: k, value: v }
                                if k == key && Some(v) == *value
                        )
                })
                .count();
            if matching_writes != usize::from(value.is_some()) {
                return Err(format!(
                    "missing or extra actor-attributed output at {} epoch {epoch}",
                    record.label
                )
                .into());
            }
        }
        if record.label == "zero-credit" && record.attempts_by_epoch.iter().sum::<usize>() != 0 {
            return Err("zero-credit branch executed an internal CIL instruction".into());
        }
        if record.label.starts_with("restore") || record.label == "mutate2" {
            // First window must trap the original (never restarted) reader.
            let reader3 = record.world.process(3).unwrap();
            if !reader3.stopped
                || !record.world.events().iter().any(|e| {
                    e.origin == Origin::Internal(3) && matches!(e.kind, EventKind::Trapped(_))
                })
            {
                return Err(format!("unobserved true loss for {}", record.label).into());
            }
        }
    }
    let untreated = &runs[0].world;
    let sham = &runs[1].world;
    if (1..=5).any(|id| untreated.process(id) != sham.process(id))
        || (1..=2).any(|id| untreated.graph_object(id) != sham.graph_object(id))
        || (90..=92).any(|key| untreated.cell(key) != sham.cell(key))
        || untreated.available_compute_pool() != sham.available_compute_pool()
    {
        return Err("sham changed the simulated state".into());
    }
    println!(
        "Study 21: 11 matched mechanism arms; 33 epoch snapshots; observed loss and externally restored function, NOT endogenous life; path={path}"
    );
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let path = env::args()
        .nth(1)
        .unwrap_or_else(|| "cl1-study21.sqlite".into());
    let revision =
        env::var("CL1_SOURCE_REVISION").unwrap_or_else(|_| "unverified-local-checkout".into());
    run(&path, &revision)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prefork_components_are_internal_execution_of_external_code() {
        let world = common_world();
        assert_eq!(world.graph_count(), 2);
        assert_eq!(conditions(&world).len(), 11);
    }
}
