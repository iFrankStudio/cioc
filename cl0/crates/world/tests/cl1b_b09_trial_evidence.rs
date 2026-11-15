//! CL-1B B09: CIL trial-readback after loss of an exact code-edge history.
//! Fully human-authored local decision rule. Not endogenous cyberlife.
//! Pre-registration: cl1b/studies/B09-evidence-after-history-loss.md
use cl0_world::{EventKind, Intervention, Op, Origin, Process, World};

#[derive(Clone, Copy, Debug)]
enum Arm {
    Intact,
    Bypass,
    BypassPatchDisabled,
    BypassStaleMatch,
    BypassStaleWrong,
}

fn founder() -> Vec<Op> {
    vec![
        Op::Const { dst: 0, value: 7, next: 1 },
        Op::Const { dst: 2, value: 7, next: 2 },
        Op::Const { dst: 3, value: 9, next: 3 },
        Op::Read { dst: 1, address: 2, next: 4 },
        Op::Const { dst: 0, value: 7, next: 5 },
        Op::Const { dst: 2, value: 7, next: 6 },
        Op::Const { dst: 3, value: 9, next: 7 },
        Op::Read { dst: 1, address: 2, next: 8 },
        Op::Write { address: 3, value: 1, next: 9 },
        Op::Read { dst: 2, address: 3, next: 10 },
        Op::Sub { dst: 2, lhs: 2, rhs: 1, next: 11 },
        Op::Branch { test: 2, nonzero: 12, zero: 16 },
        Op::ReadNext { dst: 1, node: 0, next: 13 },
        Op::Const { dst: 3, value: 1, next: 14 },
        Op::Sub { dst: 1, lhs: 1, rhs: 3, next: 15 },
        Op::PatchNext { node: 0, target: 1, next: 16 },
        Op::Const { dst: 2, value: 7, next: 17 },
        Op::Const { dst: 3, value: 9, next: 18 },
        Op::Const { dst: 0, value: 7, next: 7 },
    ]
}

fn live_edge(actor: &Process) -> usize {
    match &actor.code.nodes[7] {
        Op::Read { next, .. } => *next,
        _ => panic!("monitored instruction changed type"),
    }
}

#[test]
fn cl1b_b09_native_trial_readback_can_recover_but_zero_and_stale_outputs_hide_failure() {
    for input in [-13, 0, 7, 23, 101] {
        for arm in [
            Arm::Intact,
            Arm::Bypass,
            Arm::BypassPatchDisabled,
            Arm::BypassStaleMatch,
            Arm::BypassStaleWrong,
        ] {
            let mut world =
                World::with_strict_capacity(16, 32, 64).with_native_executable_edit_trace();
            if matches!(arm, Arm::BypassPatchDisabled) {
                world = world.with_native_patchnext_disabled();
            }
            world.load(Process::new(1, founder(), 200)).unwrap();
            world
                .intervene(Intervention::SetCell { key: 7, value: input })
                .unwrap();
            if matches!(arm, Arm::BypassStaleMatch | Arm::BypassStaleWrong) {
                world
                    .intervene(Intervention::SetCell {
                        key: 9,
                        value: if matches!(arm, Arm::BypassStaleMatch) {
                            input
                        } else {
                            input + 1
                        },
                    })
                    .unwrap();
            }
            if !matches!(arm, Arm::Intact) {
                world
                    .intervene(Intervention::PatchEdge {
                        process: 1,
                        node: 7,
                        next: 9,
                    })
                    .unwrap();
            }
            let external_count = if matches!(arm, Arm::Intact) {
                2
            } else if matches!(arm, Arm::BypassStaleMatch | Arm::BypassStaleWrong) {
                4
            } else {
                3
            };
            assert_eq!(
                world
                    .events()
                    .iter()
                    .filter(|e| e.origin == Origin::External)
                    .count(),
                external_count
            );

            assert_eq!(world.run(72), 72, "every arm must get equal CIL attempts");
            let actor = world.process(1).unwrap();
            assert_eq!(actor.id, 1);
            assert!(!actor.stopped);
            assert_eq!(actor.credits, 128);
            assert_eq!(world.total_compute_inventory(), Some(128));
            assert!(world.process(2).is_none());
            assert!(
                world
                    .events()
                    .iter()
                    .filter(|e| e.origin != Origin::External)
                    .all(|e| e.origin == Origin::Internal(1))
            );
            assert_eq!(
                world
                    .events()
                    .iter()
                    .filter(|e| matches!(
                        e.kind,
                        EventKind::GraphLaunched { .. } | EventKind::Trapped(_)
                    ))
                    .count(),
                0
            );

            let effective_recovery = match arm {
                Arm::Intact => false,
                Arm::Bypass | Arm::BypassPatchDisabled => input != 0,
                Arm::BypassStaleMatch => false,
                Arm::BypassStaleWrong => true,
            } && !matches!(arm, Arm::BypassPatchDisabled);
            let expected_edge = if matches!(arm, Arm::Intact) || effective_recovery {
                8
            } else {
                9
            };
            assert_eq!(live_edge(actor), expected_edge, "{arm:?}, input={input}");

            let effective_edits: Vec<(usize, usize, usize)> = world
                .events()
                .iter()
                .filter_map(|e| match (&e.origin, &e.kind) {
                    (
                        Origin::Internal(1),
                        EventKind::ExecutableEdgePatched { node, before, after },
                    ) => Some((*node, *before, *after)),
                    _ => None,
                })
                .collect();
            assert_eq!(
                effective_edits,
                if effective_recovery { vec![(7, 9, 8)] } else { vec![] },
                "{arm:?}, input={input}"
            );

            let suppressed = world
                .events()
                .iter()
                .filter(|e| {
                    e.origin == Origin::Internal(1)
                        && matches!(
                            e.kind,
                            EventKind::NativeCodeEditSuppressed { opcode_tag: 6 }
                        )
                })
                .count();
            assert_eq!(
                suppressed > 0,
                matches!(arm, Arm::BypassPatchDisabled) && input != 0
            );

            let writes: Vec<(usize, i64)> = world
                .events()
                .iter()
                .filter_map(|e| match (&e.origin, &e.kind) {
                    (Origin::Internal(1), EventKind::CellWritten { key: 9, value }) => {
                        Some((e.pc, *value))
                    }
                    _ => None,
                })
                .collect();
            assert!(
                writes.iter().all(|(pc, value)| *pc == 8 && *value == input),
                "only PC8 may produce valid native output"
            );
            if expected_edge == 8 {
                assert!(!writes.is_empty(), "{arm:?}: must regain a real service write");
            } else {
                assert!(writes.is_empty(), "{arm:?}: no real service despite readback");
            }

            let first_readback = world.events().iter().find_map(|e| match (&e.origin, &e.kind) {
                (Origin::Internal(1), EventKind::CellRead { key: 9, value }) if e.pc == 9 => {
                    Some(*value)
                }
                _ => None,
            });
            assert_eq!(
                first_readback,
                Some(match arm {
                    Arm::Intact => input,
                    Arm::BypassStaleMatch => input,
                    Arm::BypassStaleWrong => input + 1,
                    _ => 0,
                }),
                "{arm:?}, input={input}"
            );
        }
    }
}
