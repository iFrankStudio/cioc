//! CL-1B B03a: CIL self-introspection + internally computed variant.
//! Deliberately researcher-authored, and does not demonstrate adaptation.
use cl0_world::{EventKind, Intervention, Op, Origin, Process, World};

fn founder() -> Vec<Op> {
    vec![
        Op::Const {
            dst: 0,
            value: 9,
            next: 1,
        },
        Op::ReadNext {
            dst: 1,
            node: 0,
            next: 2,
        },
        Op::Const {
            dst: 2,
            value: 11,
            next: 3,
        },
        Op::Read {
            dst: 2,
            address: 2,
            next: 4,
        },
        Op::Add {
            dst: 1,
            lhs: 1,
            rhs: 2,
            next: 5,
        },
        Op::PatchNext {
            node: 0,
            target: 1,
            next: 6,
        },
        Op::Const {
            dst: 2,
            value: 7,
            next: 7,
        },
        Op::Const {
            dst: 3,
            value: 9,
            next: 8,
        },
        Op::Read {
            dst: 1,
            address: 2,
            next: 9,
        },
        Op::Write {
            address: 3,
            value: 1,
            next: 10,
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
        Op::Halt,
    ]
}

#[test]
fn cl1b_b03a_self_computed_candidate_can_skip_instruction_without_function_loss() {
    for value in [-13, 0, 7, 23, 101] {
        for delta in [0, 1] {
            let mut world =
                World::with_strict_capacity(16, 16, 64).with_native_executable_edit_trace();
            world.load(Process::new(1, founder(), 32)).unwrap();
            for (key, input) in [(7, value), (11, delta)] {
                world
                    .intervene(Intervention::SetCell { key, value: input })
                    .unwrap();
            }

            let attempts = world.run(32);
            assert!(!world.clone().step(), "world must reach natural terminal");
            let actor = world.process(1).unwrap();
            assert!(actor.stopped);
            assert_eq!(actor.id, 1);
            assert_eq!(actor.credits, 32 - attempts as u64);
            assert_eq!(
                world.total_compute_inventory(),
                Some(32 - attempts as u64)
            );
            assert_eq!(attempts, if delta == 0 { 13 } else { 12 });

            match &actor.code.nodes[9] {
                Op::Write { next, .. } => assert_eq!(*next, 10 + delta as usize),
                _ => panic!("candidate-target instruction type changed"),
            }

            let service: Vec<(usize, i64)> = world
                .events()
                .iter()
                .filter_map(|event| match (&event.origin, &event.kind) {
                    (Origin::Internal(1), EventKind::CellWritten { key: 9, value }) => {
                        Some((event.pc, *value))
                    }
                    _ => None,
                })
                .collect();
            assert_eq!(service, vec![(9, value), (11, value)]);

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
            if delta == 0 {
                assert!(patches.is_empty());
            } else {
                assert_eq!(patches, vec![(9, 10, 11)]);
            }
            assert_eq!(
                world
                    .events()
                    .iter()
                    .filter(|event| event.origin == Origin::External)
                    .count(),
                3
            );
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
                0
            );
            assert!(world.events().iter().all(|event| {
                event.origin == Origin::External || event.origin == Origin::Internal(1)
            }));
        }
    }
}
