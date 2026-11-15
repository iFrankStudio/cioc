//! CL-1B B01: human-engineered same-process functional continuity calibration.
//! Predeclared in cl1b/studies/B01-multi-edit-functional-continuity.md.
//! This tests native self-modification and negative controls, NOT autonomous adaptation.
use cl0_world::{EventKind, Intervention, Op, Origin, Process, World};

#[derive(Clone, Copy, Debug)]
enum Arm {
    Functional,
    Suppressed,
    Damaging,
}

fn founder(damaging: bool) -> Vec<Op> {
    vec![
        Op::Const {
            dst: 2,
            value: 7,
            next: 1,
        },
        Op::Const {
            dst: 3,
            value: 9,
            next: 2,
        },
        Op::Read {
            dst: 1,
            address: 2,
            next: 3,
        },
        Op::Write {
            address: 3,
            value: 1,
            next: 4,
        },
        Op::Const {
            dst: 0,
            value: 9,
            next: 5,
        },
        Op::Const {
            dst: 1,
            value: if damaging { 20 } else { 10 },
            next: 6,
        },
        Op::PatchNext {
            node: 0,
            target: 1,
            next: 7,
        },
        Op::Const {
            dst: 2,
            value: 7,
            next: 8,
        },
        Op::Const {
            dst: 3,
            value: 9,
            next: 9,
        },
        Op::Read {
            dst: 1,
            address: 2,
            next: 11,
        },
        Op::Const {
            dst: 0,
            value: 0,
            next: 11,
        },
        Op::Write {
            address: 3,
            value: 1,
            next: 12,
        },
        Op::Const {
            dst: 0,
            value: 17,
            next: 13,
        },
        Op::Const {
            dst: 1,
            value: 18,
            next: 14,
        },
        Op::PatchNext {
            node: 0,
            target: 1,
            next: 15,
        },
        Op::Const {
            dst: 2,
            value: 7,
            next: 16,
        },
        Op::Const {
            dst: 3,
            value: 9,
            next: 17,
        },
        Op::Read {
            dst: 1,
            address: 2,
            next: 19,
        },
        Op::Const {
            dst: 0,
            value: 0,
            next: 19,
        },
        Op::Write {
            address: 3,
            value: 1,
            next: 20,
        },
        Op::Halt,
    ]
}

#[test]
fn cl1b_b01_three_functional_windows_under_two_self_edits() {
    let inputs = [-13, 0, 7, 23, 101];
    for input in inputs {
        for arm in [Arm::Functional, Arm::Suppressed, Arm::Damaging] {
            let source = founder(matches!(arm, Arm::Damaging));
            assert_eq!(source.len(), 21);
            let mut world =
                World::with_strict_capacity(16, 32, 64).with_native_executable_edit_trace();
            if matches!(arm, Arm::Suppressed) {
                world = world.with_native_patchnext_disabled();
            }
            world.load(Process::new(1, source, 40)).unwrap();
            world
                .intervene(Intervention::SetCell {
                    key: 7,
                    value: input,
                })
                .unwrap();

            let attempts = world.run(40);
            assert!(!world.clone().step(), "all arms must reach natural terminal");
            let actor = world.process(1).unwrap();
            assert!(actor.stopped);
            assert_eq!(actor.credits, 40 - attempts as u64);
            assert_eq!(
                world.total_compute_inventory(),
                Some(40 - attempts as u64)
            );

            let writes: Vec<(usize, i64)> = world
                .events()
                .iter()
                .filter_map(|event| match (&event.origin, &event.kind) {
                    (Origin::Internal(1), EventKind::CellWritten { key: 9, value }) => {
                        Some((event.pc, *value))
                    }
                    _ => None,
                })
                .collect();
            let patches: Vec<(usize, usize, usize)> = world
                .events()
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
            let suppressed = world
                .events()
                .iter()
                .filter(|event| {
                    event.origin == Origin::Internal(1)
                        && matches!(
                            event.kind,
                            EventKind::NativeCodeEditSuppressed { opcode_tag: 6 }
                        )
                })
                .count();

            assert_eq!(
                world
                    .events()
                    .iter()
                    .filter(|event| {
                        matches!(
                            event.kind,
                            EventKind::GraphLaunched { .. } | EventKind::Trapped(_)
                        )
                    })
                    .count(),
                0,
                "no spawn or trap in any arm"
            );
            assert_eq!(
                world
                    .events()
                    .iter()
                    .filter(|event| event.origin == Origin::External)
                    .count(),
                2,
                "load and input initialization only"
            );
            assert!(
                world
                    .events()
                    .iter()
                    .filter(|event| event.origin != Origin::External)
                    .all(|event| event.origin == Origin::Internal(1))
            );

            let (expected_writes, expected_patches, expected_suppressed, next9, next17) =
                match arm {
                    Arm::Functional => (
                        vec![(3, input), (11, input), (19, input)],
                        vec![(9, 11, 10), (17, 19, 18)],
                        0,
                        10,
                        18,
                    ),
                    Arm::Suppressed => (
                        vec![(3, input), (11, input), (19, input)],
                        vec![],
                        2,
                        11,
                        19,
                    ),
                    Arm::Damaging => (
                        vec![(3, input)],
                        vec![(9, 11, 20)],
                        0,
                        20,
                        19,
                    ),
                };
            assert_eq!(writes, expected_writes, "functional window mismatch");
            assert_eq!(patches, expected_patches, "unexpected native edits");
            assert_eq!(suppressed, expected_suppressed);
            match &actor.code.nodes[9] {
                Op::Read { next, .. } => assert_eq!(*next, next9),
                _ => panic!("first target node changed type"),
            }
            match &actor.code.nodes[17] {
                Op::Read { next, .. } => assert_eq!(*next, next17),
                _ => panic!("second target node changed type"),
            }
            assert_eq!(
                attempts,
                match arm {
                    Arm::Functional => 21,
                    Arm::Suppressed => 19,
                    Arm::Damaging => 11,
                }
            );
        }
    }
}
