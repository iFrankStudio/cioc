//! CL-1 Study76: six HUMAN-AUTHORED native Draw shortage/retry physics fixtures.
//! Generic substrate feasibility, ZERO stochastic ecological worlds or life.
use cl0_lab::{EpochAction, EpochCondition, Laboratory};
use cl0_world::{EventKind, Intervention, Op, Origin, Process, World};
use std::{env, error::Error};

const EPOCHS: usize = 24;
const QUOTA: usize = 12;
const MODES: [&str; 6] = [
    "legacy-fatal-ambient",
    "recoverable-ambient",
    "legacy-pool-fee-early",
    "recoverable-pool-fee-reserve",
    "recoverable-no-pool",
    "recoverable-invalid-operand",
];

fn original(mode: &str) -> Result<(World, Vec<EpochAction>), Box<dyn Error>> {
    if !MODES.contains(&mode) {
        return Err("unregistered scripted resource-shortage mode".into());
    }
    let recoverable = mode.starts_with("recoverable");
    let mut world = World::with_strict_capacity(4, 8, 18);
    if recoverable {
        world = world.with_recoverable_draw_shortage();
    }
    if mode != "legacy-fatal-ambient" && mode != "recoverable-invalid-operand" {
        world = world.with_pool_paid_draw();
    }
    let (mut actor, initial_pool, refill) = match mode {
        "legacy-fatal-ambient" | "recoverable-ambient" => (
            Process::new(
                1,
                vec![
                    Op::Const {
                        dst: 1,
                        value: 8,
                        next: 1,
                    },
                    Op::DrawCompute { amount: 1, next: 2 },
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
                2,
            ),
            4,
            Some(12),
        ),
        "legacy-pool-fee-early" | "recoverable-pool-fee-reserve" => (
            Process::new(
                1,
                vec![
                    Op::DrawCompute { amount: 1, next: 1 },
                    Op::Const {
                        dst: 0,
                        value: 7,
                        next: 2,
                    },
                    Op::Write {
                        address: 2,
                        value: 0,
                        next: 3,
                    },
                    Op::Halt,
                ],
                0,
            ),
            8,
            Some(if mode == "legacy-pool-fee-early" {
                15
            } else {
                16
            }),
        ),
        "recoverable-no-pool" => (
            Process::new(1, vec![Op::DrawCompute { amount: 1, next: 0 }], 0),
            0,
            None,
        ),
        "recoverable-invalid-operand" => (
            Process::new(1, vec![Op::DrawCompute { amount: 1, next: 0 }], 1),
            16,
            None,
        ),
        _ => unreachable!(),
    };
    actor.registers[2] = 9;
    actor.registers[1] = if mode == "recoverable-invalid-operand" {
        -1
    } else {
        8
    };
    world.load(actor)?;
    world.intervene(Intervention::SetComputePool {
        credits: initial_pool,
    })?;
    let actions = refill.map_or_else(Vec::new, |credits| {
        vec![EpochAction {
            epoch: 1,
            action: Intervention::SetComputePool { credits },
        }]
    });
    Ok((world, actions))
}

fn main() -> Result<(), Box<dyn Error>> {
    let path = env::args()
        .nth(1)
        .unwrap_or_else(|| "cl1-study76-recoverable-calibration.sqlite".into());
    let revision = env::var("CL1_SOURCE_REVISION").unwrap_or_else(|_| "local-unverified".into());
    let mut lab = Laboratory::open(&path)?;
    for mode in MODES {
        let (world, actions) = original(mode)?;
        let initial_total = world
            .total_compute_inventory()
            .ok_or("invalid initial credits")?;
        // SetComputePool is absolute, but all registered epoch1 actions
        // add exactly eight EXTERNAL units to the then-current common pool.
        let injected: u64 = if actions.is_empty() { 0 } else { 8 };
        let records = lab.run_epochs(
            &world,
            &[EpochCondition {
                label: mode.to_string(),
                actions,
            }],
            EPOCHS,
            QUOTA,
            &revision,
        )?;
        let result = records.first().ok_or("missing scripted world")?;
        if !lab.verify_epoch_replay(result.id)? {
            return Err(format!("Study76 original replay mismatch: {mode}").into());
        }
        let native: Vec<_> = result
            .world
            .events()
            .iter()
            .filter(|event| event.origin == Origin::Internal(1))
            .collect();
        let attempts = native.len();
        let outputs = native
            .iter()
            .filter(|event| matches!(event.kind, EventKind::CellWritten { key: 9, value: 7 }))
            .count();
        let deferred = native
            .iter()
            .filter(|event| matches!(event.kind, EventKind::DrawDeferred { .. }))
            .count();
        let trapped = native
            .iter()
            .filter(|event| matches!(event.kind, EventKind::Trapped(_)))
            .count();
        let fees = native
            .iter()
            .filter(|event| event.pool_paid_attempt)
            .count();
        let expected = match mode {
            "legacy-fatal-ambient" => (2, 0, 0, 1, 0),
            "recoverable-ambient" => (6, 1, 1, 0, 1),
            "legacy-pool-fee-early" => (1, 0, 0, 1, 1),
            "recoverable-pool-fee-reserve" => (4, 1, 0, 0, 1),
            "recoverable-no-pool" => (0, 0, 0, 0, 0),
            "recoverable-invalid-operand" => (1, 0, 0, 1, 0),
            _ => unreachable!(),
        };
        if (attempts, outputs, deferred, trapped, fees) != expected
            || result.attempts_by_epoch.iter().sum::<usize>() != attempts
            || result.world.total_compute_inventory()
                != Some(initial_total + injected - attempts as u64)
            || native.iter().any(|event| event.process != 1)
        {
            return Err(
                format!("Study76 native provenance, compute or control mismatch {mode}").into(),
            );
        }
        println!("{mode}: {attempts} charged native attempts; {outputs} actual writes; {deferred} deferred; {trapped} fatal; {fees} pool fees");
    }
    println!("Study76 finished six HUMAN-SCRIPTED source-pinned fixtures, 144 hashed epochs, ZERO independent genomes");
    Ok(())
}
