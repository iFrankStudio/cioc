//! CL-1 Study47 is a scripted **VM mechanism calibration**. NEVER life evidence.
//! Human-authored graph builder; ZERO stochastic independent ecology worlds.
use cl0_lab::{EpochCondition, Laboratory};
use cl0_world::{EventKind, Op, Origin, Process, World};
use std::{env, error::Error};

const EPOCHS: usize = 8;
const QUOTA: usize = 8;

fn fixture(mode: &str) -> Result<World, Box<dyn Error>> {
    let mut world = World::with_strict_capacity(8, 16, 64);
    let edge_op = match mode {
        "native-local-edge" | "zero-child-credit" => Op::GraphSetNext {
            graph: 3,
            node: 2,
            target: 0,
            next: 4,
        },
        "copied-stale-edge" => Op::Const {
            dst: 2,
            value: 0,
            next: 4,
        },
        _ => return Err("unregistered fixture".into()),
    };
    let child_credit = if mode == "zero-child-credit" { 0 } else { 3 };
    world.load(Process::new(
        1,
        vec![
            Op::GraphNew { dst: 3, next: 1 },
            Op::Const {
                dst: 2,
                value: 6,
                next: 2,
            },
            Op::GraphCopyNode {
                graph: 3,
                source: 2,
                dst: 2,
                next: 3,
            },
            edge_op,
            Op::Const {
                dst: 1,
                value: child_credit,
                next: 5,
            },
            Op::GraphLaunch {
                graph: 3,
                credits: 1,
                dst: 2,
                next: 8,
            },
            Op::Write {
                address: 0,
                value: 3,
                next: 7,
            },
            Op::Halt,
            Op::Halt,
        ],
        30,
    ))?;
    assert_eq!(world.total_compute_inventory(), Some(30));
    Ok(world)
}

fn main() -> Result<(), Box<dyn Error>> {
    let path = env::args()
        .nth(1)
        .unwrap_or_else(|| "cl1-study47-calibration.sqlite".into());
    let revision =
        env::var("CL1_SOURCE_REVISION").unwrap_or_else(|_| "unverified-local-checkout".into());
    let mut lab = Laboratory::open(&path)?;
    for mode in [
        "native-local-edge",
        "copied-stale-edge",
        "zero-child-credit",
    ] {
        let world = fixture(mode)?;
        let outcome = lab.run_epochs(
            &world,
            &[EpochCondition {
                label: mode.to_string(),
                actions: vec![],
            }],
            EPOCHS,
            QUOTA,
            &revision,
        )?;
        let run = outcome.first().ok_or("missing calibration run")?;
        if !lab.verify_epoch_replay(run.id)? {
            return Err(format!("exact graph-edge fixture replay failed: {mode}").into());
        }
        let child_writes = run
            .world
            .events()
            .iter()
            .filter(|e| {
                e.process > 1
                    && e.origin == Origin::Internal(e.process)
                    && matches!(e.kind, EventKind::CellWritten { key: 0, value: 0 })
            })
            .count();
        let child_launches = run
            .world
            .events()
            .iter()
            .filter(|e| {
                e.process == 1
                    && e.origin == Origin::Internal(1)
                    && matches!(e.kind, EventKind::GraphLaunched { .. })
            })
            .count();
        let invalid_edge_traps = run
            .world
            .events()
            .iter()
            .filter(|e| {
                e.process > 1
                    && e.origin == Origin::Internal(e.process)
                    && matches!(&e.kind,
                        EventKind::Trapped(error) if error.contains("invalid edge"))
            })
            .count();
        println!(
            "{mode}: native child launches {child_launches},              child CellWritten(key0,value0) {child_writes}, child invalid-edge traps {invalid_edge_traps}"
        );
        if (mode == "native-local-edge" && (child_writes == 0 || child_launches != 1))
            || (mode != "native-local-edge" && child_writes != 0)
            || (mode == "copied-stale-edge" && invalid_edge_traps == 0)
        {
            return Err(format!("pre-registered scripted mechanism mismatch: {mode}").into());
        }
    }
    println!("Study47: exactly THREE scripted fixtures, ZERO independent ecology worlds; {path}");
    Ok(())
}
