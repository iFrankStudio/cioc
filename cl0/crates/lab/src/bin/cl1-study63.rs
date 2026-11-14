//! CL-1 Study63: SCRIPTED feedback/resource mechanism feasibility, NOT life.
//! Exactly six predeclared calibration modes; ZERO stochastic ecology worlds.
use cl0_lab::{EpochAction, EpochCondition, Laboratory};
use cl0_world::{EventKind, Intervention, Op, Origin, Process, World};
use std::{collections::BTreeSet, env, error::Error};

const EPOCHS: usize = 8;
const QUOTA: usize = 12;

const MODES: [&str; 6] = [
    "sensor-low-native-pool",
    "sensor-high-native-pool",
    "sensor-switch-native-pool",
    "sensor-high-native-pool-read-ablated",
    "sensor-high-empty-pool",
    "sensor-high-zero-process-credit",
];

fn scripted_world(mode: &str) -> Result<(World, Vec<EpochAction>), Box<dyn Error>> {
    if !MODES.contains(&mode) {
        return Err("unregistered feasibility calibration".into());
    }
    let mut world = World::with_strict_capacity(16, 12, 40);
    let ablated = mode == "sensor-high-native-pool-read-ablated";
    let input = if mode.starts_with("sensor-low") || mode.starts_with("sensor-switch") {
        3
    } else {
        8
    };
    let credits = if mode == "sensor-high-zero-process-credit" {
        0
    } else {
        3
    };
    let pool = if mode == "sensor-high-empty-pool" {
        0
    } else {
        200
    };
    let read = if ablated {
        Op::Const {
            dst: 2,
            value: 0,
            next: 4,
        }
    } else {
        Op::Read {
            dst: 2,
            address: 0,
            next: 4,
        }
    };
    world.load(Process::new(
        1,
        vec![
            Op::Const {
                dst: 0,
                value: 7,
                next: 1,
            },
            Op::Const {
                dst: 1,
                value: 8,
                next: 2,
            },
            Op::DrawCompute { amount: 1, next: 3 },
            read,
            Op::Const {
                dst: 0,
                value: 9,
                next: 5,
            },
            Op::Add {
                dst: 3,
                lhs: 2,
                rhs: 1,
                next: 6,
            },
            Op::Write {
                address: 0,
                value: 3,
                next: 7,
            },
            Op::Const {
                dst: 0,
                value: 7,
                next: 2,
            },
        ],
        credits,
    ))?;
    // Researcher-supplied initial DATA and ENERGY are intentionally External.
    world.intervene(Intervention::SetCell {
        key: 7,
        value: input,
    })?;
    world.intervene(Intervention::SetComputePool { credits: pool })?;
    let actions = if mode == "sensor-switch-native-pool" {
        vec![EpochAction {
            epoch: 4,
            action: Intervention::SetCell { key: 7, value: 8 },
        }]
    } else {
        Vec::new()
    };
    Ok((world, actions))
}

fn main() -> Result<(), Box<dyn Error>> {
    let path = env::args()
        .nth(1)
        .unwrap_or_else(|| "cl1-study63-feedback-calibration.sqlite".into());
    let revision = env::var("CL1_SOURCE_REVISION").unwrap_or_else(|_| "unverified-local".into());
    let mut lab = Laboratory::open(&path)?;
    for mode in MODES {
        let (world, actions) = scripted_world(mode)?;
        let original_inventory = world
            .total_compute_inventory()
            .ok_or("invalid initial compute inventory")?;
        let runs = lab.run_epochs(
            &world,
            &[EpochCondition {
                label: mode.to_string(),
                actions,
            }],
            EPOCHS,
            QUOTA,
            &revision,
        )?;
        let run = runs.first().ok_or("missing fixed mechanism fixture")?;
        if !lab.verify_epoch_replay(run.id)? {
            return Err(format!("native feedback replay mismatch: {mode}").into());
        }
        let mut writes = BTreeSet::new();
        let mut draws = 0;
        for event in run.world.events() {
            if event.origin != Origin::Internal(1) || event.process != 1 {
                continue;
            }
            if let EventKind::CellWritten { key: 9, value } = event.kind {
                writes.insert(value);
            }
            if matches!(event.kind, EventKind::Executed) && event.pc == 2 {
                draws += 1;
            }
        }
        let expected: BTreeSet<i64> = match mode {
            "sensor-low-native-pool" => [11].into(),
            "sensor-high-native-pool" => [16].into(),
            "sensor-switch-native-pool" => [11, 16].into(),
            "sensor-high-native-pool-read-ablated" => [8].into(),
            _ => BTreeSet::new(),
        };
        if writes != expected
            || (mode.contains("native-pool") && draws == 0)
            || (mode == "sensor-high-empty-pool" && draws != 0)
            || run.world.total_compute_inventory()
                != Some(original_inventory - run.attempts_by_epoch.iter().sum::<usize>() as u64)
        {
            return Err(format!("preregistered feedback gate failed in {mode}").into());
        }
        println!("{mode}: native outputs {writes:?}, native compute pool draws {draws}");
    }
    println!("Study63: exactly SIX scripted calibration fixtures; ZERO spontaneous ecology worlds; {path}");
    Ok(())
}
