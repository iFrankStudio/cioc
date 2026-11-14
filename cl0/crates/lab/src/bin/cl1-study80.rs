//! CL-1 Study80 Phase-0: human-scripted, zero-genome functional edge repair calibration.
//! Not a prospective ecology, not a living organism, not spontaneous repair.
use cl0_lab::{EpochAction, EpochCondition, Laboratory};
use cl0_world::{EventKind, Intervention, Op, Origin, Process, World};
use std::{env, error::Error};

const EPOCHS: usize = 12;
const QUOTA: usize = 2;
const SOURCE_VALUES: [i64; 2] = [3, 8];
const ARMS: [&str; 5] = ["intact", "sham", "lesion-edit", "lesion-disabled", "lesion-host-repair"];

fn source_world(source: i64, disabled: bool) -> Result<World, Box<dyn Error>> {
    let mut world = World::with_strict_capacity(4, 16, 64);
    if disabled {
        world = world.with_native_code_edits_disabled();
    }
    world.load(Process::new(
        1,
        vec![
            Op::Const { dst: 1, value: 3, next: 1 },
            Op::Const { dst: 2, value: 4, next: 2 },
            Op::Const { dst: 3, value: 9, next: 3 },
            Op::Read { dst: 0, address: 2, next: 4 },
            Op::Write { address: 3, value: 0, next: 5 },
            Op::PatchNext { node: 1, target: 2, next: 3 },
        ],
        48,
    ))?;
    world.intervene(Intervention::SetCell { key: 4, value: source })?;
    Ok(world)
}

fn schedule(arm: &str) -> Vec<EpochAction> {
    let mut actions = Vec::new();
    if arm != "intact" {
        actions.push(EpochAction {
            epoch: 3,
            action: Intervention::PatchEdge {
                process: 1,
                node: 3,
                next: if arm == "sham" { 4 } else { 5 },
            },
        });
    }
    if arm == "lesion-host-repair" {
        actions.push(EpochAction {
            epoch: 4,
            action: Intervention::PatchEdge { process: 1, node: 3, next: 4 },
        });
    }
    actions
}

fn native_writes(world: &World) -> Vec<i64> {
    world.events().iter().filter_map(|e| match (&e.origin, &e.kind) {
        (Origin::Internal(1), EventKind::CellWritten { key: 9, value }) => Some(*value),
        _ => None,
    }).collect()
}

fn main() -> Result<(), Box<dyn Error>> {
    let path = env::args().nth(1).unwrap_or_else(|| "cl1-study80-phase0.sqlite".into());
    let revision = env::var("CL1_SOURCE_REVISION").unwrap_or_else(|_| "unverified-local-checkout".into());
    let mut lab = Laboratory::open(&path)?;
    let mut total = 0;
    for source in SOURCE_VALUES {
        for arm in ARMS {
            let disabled = arm == "lesion-disabled" || arm == "lesion-host-repair";
            let world = source_world(source, disabled)?;
            let runs = lab.run_epochs(
                &world,
                &[EpochCondition {
                    label: format!("study80-phase0/source={source}/{arm}"),
                    actions: schedule(arm),
                }],
                EPOCHS,
                QUOTA,
                &revision,
            )?;
            let run = runs.first().ok_or("missing Study80 fixture")?;
            if !lab.verify_epoch_replay(run.id)? {
                return Err(format!("failed full SQLite epoch replay: {source}/{arm}").into());
            }
            if run.attempts_by_epoch != vec![QUOTA; EPOCHS]
                || run.world.total_compute_inventory() != Some(48 - (EPOCHS * QUOTA) as u64)
            {
                return Err(format!("native attempt or inventory mismatch: {source}/{arm}").into());
            }
            let writes = native_writes(&run.world);
            if writes.iter().any(|value| *value != source) {
                return Err(format!("bad source-value ancestry: {source}/{arm}").into());
            }
            let expected = match arm {
                "lesion-disabled" => writes.len() == 1,
                "lesion-edit" | "lesion-host-repair" | "sham" | "intact" => writes.len() >= 3,
                _ => unreachable!(),
            };
            if !expected {
                return Err(format!("functional calibration failed {source}/{arm}: {writes:?}").into());
            }
            if run.world.events().iter().any(|e| matches!(e.kind, EventKind::Trapped(_))) {
                return Err(format!("native trap in scripted calibration: {source}/{arm}").into());
            }
            if arm == "lesion-disabled"
                && !run.world.events().iter().any(|e| {
                    e.origin == Origin::Internal(1)
                        && matches!(e.kind, EventKind::NativeCodeEditSuppressed { opcode_tag: 6 })
                })
            {
                return Err("missing charged native edit suppression provenance".into());
            }
            let next = match &run.world.process(1).ok_or("missing native actor")?.code.nodes[3] {
                Op::Read { next, .. } => *next,
                _ => return Err("source-bearing node was replaced".into()),
            };
            if next != if arm == "lesion-disabled" { 5 } else { 4 } {
                return Err(format!("wrong preserved native functional edge: {source}/{arm}").into());
            }
            total += 1;
        }
    }
    if total != 10 {
        return Err("missing scripted control".into());
    }
    println!("Study80 Phase-0: {total} human-authored control runs, {} exact SHA256 epoch replays; ZERO ecological genomes; no life claim; {path}", total * EPOCHS);
    Ok(())
}
