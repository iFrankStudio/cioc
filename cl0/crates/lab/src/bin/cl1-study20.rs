//! Study 20: two-component CIL input/output mechanism calibration.
//! All initial code and restoration are investigator-authored; NOT cyberlife.
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
                dst: 2,
                value: source_index,
                next: 4,
            },
            Op::GraphCopyNode {
                graph: 0,
                source: 2,
                dst: 2,
                next: 5,
            },
            Op::Halt,
        ],
        6,
    )
}

fn common_world() -> World {
    let mut world = World::with_strict_capacity(4, 16, 64);
    world.load(owner(1, 0)).expect("owner1 loaded");
    world.load(owner(2, 1)).expect("owner2 loaded");
    world
        .load(Process::new(
            3,
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
                    value: 90,
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
        ))
        .expect("reader loaded");
    assert_eq!(world.run(10), 10);
    assert!(world.cell(90).is_none());
    for graph in 1..=2 {
        let object = world.graph_object(graph).expect("graph constructed");
        assert_eq!(object.owner, graph);
        assert_eq!(object.nodes.len(), 1);
        assert!(object.readers.contains(&3));
    }
    assert!(matches!(
        world.graph_object(1).unwrap().nodes[0],
        Op::GraphNew { .. }
    ));
    assert!(matches!(
        world.graph_object(2).unwrap().nodes[0],
        Op::Const { .. }
    ));
    world
}

fn action(action: Intervention) -> EpochAction {
    EpochAction { epoch: 0, action }
}

fn conditions(common: &World) -> Vec<EpochCondition> {
    let saved1: GraphObject = common.graph_object(1).unwrap().clone();
    let saved2: GraphObject = common.graph_object(2).unwrap().clone();
    let mut mutated2 = saved2.clone();
    mutated2.nodes[0] = Op::Halt;
    let scenarios = vec![
        ("control", vec![]),
        ("sham", vec![Intervention::DropGraph { graph: 0 }]),
        ("drop1", vec![Intervention::DropGraph { graph: 1 }]),
        ("drop2", vec![Intervention::DropGraph { graph: 2 }]),
        (
            "drop-both",
            vec![
                Intervention::DropGraph { graph: 1 },
                Intervention::DropGraph { graph: 2 },
            ],
        ),
        (
            "restore1",
            vec![
                Intervention::DropGraph { graph: 1 },
                Intervention::PutGraph {
                    graph: 1,
                    object: saved1,
                },
            ],
        ),
        (
            "restore2",
            vec![
                Intervention::DropGraph { graph: 2 },
                Intervention::PutGraph {
                    graph: 2,
                    object: saved2,
                },
            ],
        ),
        (
            "mutate2",
            vec![
                Intervention::DropGraph { graph: 2 },
                Intervention::PutGraph {
                    graph: 2,
                    object: mutated2,
                },
            ],
        ),
        ("zero-credit", vec![]),
    ];
    scenarios
        .into_iter()
        .map(|(label, graph_actions)| {
            let mut actions = vec![
                action(Intervention::SetCredits {
                    process: 1,
                    credits: 0,
                }),
                action(Intervention::SetCredits {
                    process: 2,
                    credits: 0,
                }),
            ];
            actions.extend(graph_actions.into_iter().map(action));
            actions.push(action(Intervention::SetCredits {
                process: 3,
                credits: if label == "zero-credit" { 0 } else { 9 },
            }));
            EpochCondition {
                label: label.into(),
                actions,
            }
        })
        .collect()
}

fn run(path: &str, revision: &str) -> Result<(), Box<dyn Error>> {
    let common = common_world();
    let mut lab = Laboratory::open(path)?;
    let runs = lab.run_epochs(&common, &conditions(&common), EPOCHS, QUOTA, revision)?;
    if runs.len() != 9
        || runs
            .iter()
            .any(|r| r.initial_sha256 != runs[0].initial_sha256)
    {
        return Err("Study 20 initial states/arms diverged".into());
    }
    for recorded in &runs {
        if !lab.verify_epoch_replay(recorded.id)? || recorded.world.occupied_units() > 64 {
            return Err(format!("invalid replay/occupancy: {}", recorded.label).into());
        }
        let expected = match recorded.label.as_str() {
            "control" | "sham" | "restore1" | "restore2" => Some(11),
            "mutate2" => Some(30),
            "drop1" | "drop2" | "drop-both" | "zero-credit" => None,
            _ => return Err("unexpected arm".into()),
        };
        if recorded.world.cell(90) != expected {
            return Err(format!("wrong CIL functional output: {}", recorded.label).into());
        }
        if let Some(value) = expected {
            if !recorded.world.events().iter().any(|e| {
                e.origin == Origin::Internal(3)
                    && matches!(e.kind, EventKind::CellWritten { key: 90, value: v } if v == value)
            }) {
                return Err(format!("missing internal output event: {}", recorded.label).into());
            }
        }
        if recorded.label == "zero-credit"
            && recorded.attempts_by_epoch.iter().sum::<usize>() != 0
        {
            return Err("zero-credit control executed CIL".into());
        }
    }
    let control = &runs[0].world;
    let sham = &runs[1].world;
    if (1..=3).any(|id| control.process(id) != sham.process(id))
        || (1..=2).any(|id| control.graph_object(id) != sham.graph_object(id))
        || control.cell(90) != sham.cell(90)
        || control.available_compute_pool() != sham.available_compute_pool()
    {
        return Err("sham invalidated simulated state".into());
    }
    println!("CL-1 Study 20: 9 paired mechanism arms, 27 epoch snapshots, full exact replay; not evidence of life; path={path}");
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let path = env::args()
        .nth(1)
        .unwrap_or_else(|| "cl1-study20.sqlite".into());
    let revision =
        env::var("CL1_SOURCE_REVISION").unwrap_or_else(|_| "unverified-local-checkout".into());
    run(&path, &revision)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn common_prefork_graphs_are_internal_executions_but_preprogrammed() {
        let world = common_world();
        assert_eq!(world.graph_count(), 2);
        assert_eq!(world.process(3).unwrap().credits, 0);
        assert_eq!(conditions(&world).len(), 9);
    }

    #[test]
    fn complete_mechanism_fixture_replays() {
        let path = std::env::temp_dir().join(format!(
            "cioc-study20-test-{}.sqlite",
            std::process::id()
        ));
        let _ = std::fs::remove_file(&path);
        run(path.to_str().unwrap(), "study20-test").unwrap();
        let _ = std::fs::remove_file(path);
    }
}
