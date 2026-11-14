//! CL-1 Study73 pool-paid Draw calibration: 6 fixed HUMAN-AUTHORED fixtures.
//! Generic substrate feasibility ONLY, no spontaneous organisms.
use cl0_lab::{EpochCondition, Laboratory};
use cl0_world::{EventKind, Intervention, Op, Origin, Process, World};
use std::{env, error::Error};

const MODES: [&str; 6] = [
    "pool-paid-draw",
    "legacy-zero-credit-draw",
    "pool-paid-non-draw",
    "pool-paid-empty-pool",
    "pool-paid-insufficient-pool",
    "pool-paid-stopped-draw",
];
const EPOCHS: usize = 24;
const QUOTA: usize = 12;

fn original(mode: &str) -> Result<World, Box<dyn Error>> {
    if !MODES.contains(&mode) {
        return Err("unknown preregistered fixture".into());
    }
    let mut world = World::with_strict_capacity(4, 8, 18);
    if mode != "legacy-zero-credit-draw" {
        world = world.with_pool_paid_draw();
    }
    let process = if mode == "pool-paid-stopped-draw" {
        let mut process = Process::new(1, vec![Op::DrawCompute { amount: 1, next: 0 }], 0);
        process.registers[1] = 3;
        process.stopped = true;
        process
    } else {
        Process::new(
            1,
            vec![
                Op::Const {
                    dst: 1,
                    value: 3,
                    next: 1,
                },
                if mode == "pool-paid-non-draw" {
                    Op::Const {
                        dst: 0,
                        value: 8,
                        next: 2,
                    }
                } else {
                    Op::DrawCompute { amount: 1, next: 2 }
                },
                Op::Const {
                    dst: 0,
                    value: 7,
                    next: 3,
                },
                Op::Write {
                    address: 2,
                    value: 0,
                    next: 4,
                },
                Op::Halt,
            ],
            1,
        )
    };
    world.load(process)?;
    let pool = match mode {
        "pool-paid-empty-pool" => 0,
        "pool-paid-insufficient-pool" => 3,
        _ => 6,
    };
    world.intervene(Intervention::SetComputePool { credits: pool })?;
    Ok(world)
}

fn main() -> Result<(), Box<dyn Error>> {
    let path = env::args()
        .nth(1)
        .unwrap_or_else(|| "cl1-study73-pool-ignition-calibration.sqlite".into());
    let revision = env::var("CL1_SOURCE_REVISION").unwrap_or_else(|_| "local-unverified".into());
    let mut lab = Laboratory::open(&path)?;
    for mode in MODES {
        let world = original(mode)?;
        let total = world.total_compute_inventory().ok_or("compute overflow")?;
        let runs = lab.run_epochs(
            &world,
            &[EpochCondition {
                label: mode.into(),
                actions: vec![],
            }],
            EPOCHS,
            QUOTA,
            &revision,
        )?;
        let run = runs.first().ok_or("missing frozen fixture")?;
        if !lab.verify_epoch_replay(run.id)? {
            return Err(format!("original SQLite native replay failed: {mode}").into());
        }
        let native: Vec<_> = run
            .world
            .events()
            .iter()
            .filter(|e| e.origin == Origin::Internal(1))
            .collect();
        let paid: Vec<_> = native.iter().filter(|e| e.pool_paid_attempt).collect();
        let writes = native
            .iter()
            .filter(|e| matches!(e.kind, EventKind::CellWritten { key: 0, value: 7 }))
            .count();
        let traps = native
            .iter()
            .filter(|e| matches!(e.kind, EventKind::Trapped(_)))
            .count();
        let expected = match mode {
            "pool-paid-draw" => (5, 1, 1, 0),
            "legacy-zero-credit-draw" => (1, 0, 0, 0),
            "pool-paid-non-draw" => (1, 0, 0, 0),
            "pool-paid-empty-pool" => (1, 0, 0, 0),
            "pool-paid-insufficient-pool" => (2, 1, 0, 1),
            "pool-paid-stopped-draw" => (0, 0, 0, 0),
            _ => unreachable!("registered fixed mode"),
        };
        if (native.len(), paid.len(), writes, traps) != expected
            || paid.iter().any(|e| e.pc != 1)
            || run.world.total_compute_inventory() != Some(total - native.len() as u64)
            || run.attempts_by_epoch.iter().sum::<usize>() != native.len()
        {
            return Err(format!("native provenance/credit invariant failed: {mode}").into());
        }
        println!(
            "{mode}: {} native attempts, {} pool-paid, {writes} cell writes, {traps} traps",
            native.len(),
            paid.len()
        );
    }
    println!(
        "Study73 calibration: six scripted conditions, ZERO independent ecological worlds; {path}"
    );
    Ok(())
}
