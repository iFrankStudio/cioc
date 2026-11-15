//! CL-1B B04: a single CIL actor saves an executable edge and later restores it.
//! Protocol: cl1b/studies/B04-actor-owned-checkpoint-rollback.md.
//! Engineering-only: observer specified the guard and lesion, no spontaneous repair.
use cl0_world::{EventKind, Intervention, Op, Origin, Process, World};

#[derive(Clone, Copy, Debug)]
enum Arm {
    Sham,
    Damage,
    PatchDisabled,
    CheckpointCorrupted,
}

fn founder() -> Vec<Op> {
    vec![
        Op::Const { dst: 0, value: 6, next: 1 },
        Op::Const { dst: 2, value: 12, next: 2 },
        Op::ReadNext { dst: 1, node: 0, next: 3 },
        Op::Write { address: 2, value: 1, next: 4 },
        Op::Const { dst: 2, value: 7, next: 5 },
        Op::Const { dst: 3, value: 9, next: 6 },
        Op::Read { dst: 1, address: 2, next: 7 },
        Op::Write { address: 3, value: 1, next: 8 },
        Op::ReadNext { dst: 1, node: 0, next: 9 },
        Op::Const { dst: 2, value: 12, next: 10 },
        Op::Read { dst: 3, address: 2, next: 11 },
        Op::Sub { dst: 1, lhs: 1, rhs: 3, next: 12 },
        Op::Branch { test: 1, nonzero: 13, zero: 14 },
        Op::PatchNext { node: 0, target: 3, next: 14 },
        Op::Const { dst: 2, value: 7, next: 15 },
        Op::Const { dst: 3, value: 9, next: 6 },
    ]
}

#[test]
fn cl1b_b04_paid_native_checkpoint_restores_code_and_failed_controls_remain_visible() {
    for input in [-13, 0, 7, 23, 101] {
        let mut baseline =
            World::with_strict_capacity(16, 32, 64).with_native_executable_edit_trace();
        baseline.load(Process::new(1, founder(), 120)).unwrap();
        baseline
            .intervene(Intervention::SetCell {
                key: 7,
                value: input,
            })
            .unwrap();

        assert_eq!(baseline.run(15), 15);
        assert_eq!(baseline.process(1).unwrap().pc, 6);
        assert_eq!(baseline.process(1).unwrap().credits, 105);
        assert_eq!(baseline.total_compute_inventory(), Some(105));
        assert!(
            baseline
                .events()
                .iter()
                .any(|event| {
                    event.origin == Origin::Internal(1)
                        && event.pc == 3
                        && matches!(
                            event.kind,
                            EventKind::CellWritten { key: 12, value: 7 }
                        )
                }),
            "original executable edge must be snapshotted by paid native CIL, not host"
        );
        assert_eq!(
            baseline
                .events()
                .iter()
                .filter(|event| {
                    event.origin == Origin::Internal(1)
                        && matches!(
                            event.kind,
                            EventKind::CellWritten { key: 9, value } if value == input
                        )
                })
                .count(),
            1
        );

        for arm in [
            Arm::Sham,
            Arm::Damage,
            Arm::PatchDisabled,
            Arm::CheckpointCorrupted,
        ] {
            let mut world = baseline.clone();
            if matches!(arm, Arm::PatchDisabled) {
                world = world.with_native_patchnext_disabled();
            }
            if matches!(arm, Arm::CheckpointCorrupted) {
                world
                    .intervene(Intervention::SetCell { key: 12, value: 8 })
                    .unwrap();
            }
            world
                .intervene(Intervention::PatchEdge {
                    process: 1,
                    node: 6,
                    next: if matches!(arm, Arm::Sham) { 7 } else { 8 },
                })
                .unwrap();

            let before = world.events().len();
            assert_eq!(world.run(72), 72);
            let after = &world.events()[before..];
            let actor = world.process(1).unwrap();
            assert_eq!(actor.id, 1);
            assert!(!actor.stopped);
            assert_eq!(actor.pc, 6);
            assert_eq!(actor.credits, 33);
            assert_eq!(world.total_compute_inventory(), Some(33));
            assert_eq!(world.remaining_credits(), 33);
            assert!(after.iter().all(|event| event.origin == Origin::Internal(1)));
            assert_eq!(
                world
                    .events()
                    .iter()
                    .filter(|event| event.origin == Origin::External)
                    .count(),
                if matches!(arm, Arm::CheckpointCorrupted) { 4 } else { 3 }
            );
            assert_eq!(
                after
                    .iter()
                    .filter(|event| {
                        matches!(
                            event.kind,
                            EventKind::GraphLaunched { .. } | EventKind::Trapped(_)
                        )
                    })
                    .count(),
                0
            );

            let output: Vec<(usize, i64)> = after
                .iter()
                .filter_map(|event| match (&event.origin, &event.kind) {
                    (Origin::Internal(1), EventKind::CellWritten { key: 9, value }) => {
                        Some((event.pc, *value))
                    }
                    _ => None,
                })
                .collect();
            let repairs: Vec<(usize, usize, usize)> = after
                .iter()
                .filter_map(|event| match (&event.origin, &event.kind) {
                    (
                        Origin::Internal(1),
                        EventKind::ExecutableEdgePatched {
                            node,
                            before,
                            after,
                        },
                    ) => Some((*node, *before, *after)),
                    _ => None,
                })
                .collect();
            let suppressed = after
                .iter()
                .filter(|event| {
                    event.origin == Origin::Internal(1)
                        && matches!(
                            event.kind,
                            EventKind::NativeCodeEditSuppressed { opcode_tag: 6 }
                        )
                })
                .count();

            let (expected_output, expected_repairs, expected_suppress, final_edge) =
                match arm {
                    Arm::Sham => (8, vec![], 0, 7),
                    Arm::Damage => (7, vec![(6, 8, 7)], 0, 7),
                    Arm::PatchDisabled => (0, vec![], 8, 8),
                    Arm::CheckpointCorrupted => (0, vec![], 0, 8),
                };
            assert_eq!(output, vec![(7, input); expected_output], "{arm:?}");
            assert_eq!(repairs, expected_repairs, "{arm:?}");
            assert_eq!(suppressed, expected_suppress, "{arm:?}");
            match &actor.code.nodes[6] {
                Op::Read { next, .. } => assert_eq!(*next, final_edge),
                _ => panic!("external lesion must edit edge, not instruction type"),
            }
        }
    }
}
