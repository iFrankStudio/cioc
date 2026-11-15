//! CL-1B B02: researcher-authored native self-repair substrate calibration.
//! This is NOT spontaneous recovery. Protocol: cl1b/studies/B02-native-repair-calibration.md.
use cl0_world::{Event, EventKind, Intervention, Op, Origin, Process, World};

#[derive(Clone, Copy, Debug)]
enum Arm {
    Sham,
    Damage,
    Suppressed,
}

fn founder() -> Vec<Op> {
    vec![
        Op::Const {
            dst: 0,
            value: 3,
            next: 1,
        },
        Op::Const {
            dst: 2,
            value: 7,
            next: 2,
        },
        Op::Const {
            dst: 3,
            value: 9,
            next: 3,
        },
        Op::Read {
            dst: 1,
            address: 2,
            next: 4,
        },
        Op::Write {
            address: 3,
            value: 1,
            next: 5,
        },
        Op::ReadNext {
            dst: 1,
            node: 0,
            next: 6,
        },
        Op::Const {
            dst: 3,
            value: 4,
            next: 7,
        },
        Op::Sub {
            dst: 1,
            lhs: 1,
            rhs: 3,
            next: 8,
        },
        Op::Branch {
            test: 1,
            nonzero: 9,
            zero: 10,
        },
        Op::PatchNext {
            node: 0,
            target: 3,
            next: 10,
        },
        Op::Const {
            dst: 3,
            value: 9,
            next: 11,
        },
        Op::Const {
            dst: 2,
            value: 7,
            next: 3,
        },
    ]
}

fn service_outputs(events: &[Event]) -> Vec<(usize, i64)> {
    events
        .iter()
        .filter_map(|e| match (&e.origin, &e.kind) {
            (Origin::Internal(1), EventKind::CellWritten { key: 9, value }) => {
                Some((e.pc, *value))
            }
            _ => None,
        })
        .collect()
}

#[test]
fn cl1b_b02_single_native_actor_recovers_a_corrupted_edge_with_paid_cil() {
    for input in [-13, 0, 7, 23, 101] {
        let mut base =
            World::with_strict_capacity(16, 32, 64).with_native_executable_edit_trace();
        base.load(Process::new(1, founder(), 100)).unwrap();
        base.intervene(Intervention::SetCell {
            key: 7,
            value: input,
        })
        .unwrap();
        for _ in 0..11 {
            assert!(base.step(), "initial 11 attempts must be schedulable");
        }
        assert_eq!(base.process(1).unwrap().pc, 3);
        assert_eq!(service_outputs(base.events()), vec![(4, input)]);
        assert_eq!(base.process(1).unwrap().credits, 89);
        assert_eq!(base.total_compute_inventory(), Some(89));

        for arm in [Arm::Sham, Arm::Damage, Arm::Suppressed] {
            let mut world = match arm {
                Arm::Suppressed => base.clone().with_native_patchnext_disabled(),
                _ => base.clone(),
            };
            let changed_next = if matches!(arm, Arm::Sham) { 4 } else { 5 };
            world
                .intervene(Intervention::PatchEdge {
                    process: 1,
                    node: 3,
                    next: changed_next,
                })
                .unwrap();

            let before = world.events().len();
            let attempts = world.run(40);
            assert_eq!(attempts, 40);
            let after = &world.events()[before..];
            let actor = world.process(1).unwrap();
            assert_eq!(actor.pc, 3, "every arm must have equal local opportunities");
            assert!(!actor.stopped);
            assert_eq!(actor.credits, 49);
            assert_eq!(world.total_compute_inventory(), Some(49));
            assert_eq!(world.remaining_credits(), 49);
            assert_eq!(
                world
                    .events()
                    .iter()
                    .filter(|e| e.origin == Origin::External)
                    .count(),
                3
            );
            assert!(after.iter().all(|e| e.origin == Origin::Internal(1)));
            assert_eq!(
                after
                    .iter()
                    .filter(|e| {
                        matches!(
                            e.kind,
                            EventKind::Trapped(_) | EventKind::GraphLaunched { .. }
                        )
                    })
                    .count(),
                0
            );

            let outputs = service_outputs(after);
            let repairs: Vec<(usize, usize, usize)> = after
                .iter()
                .filter_map(|e| match &e.kind {
                    EventKind::ExecutableEdgePatched {
                        node,
                        before,
                        after,
                    } => Some((*node, *before, *after)),
                    _ => None,
                })
                .collect();
            let suppressed = after
                .iter()
                .filter(|e| {
                    matches!(
                        e.kind,
                        EventKind::NativeCodeEditSuppressed { opcode_tag: 6 }
                    )
                })
                .count();

            let (expected_outputs, expected_repairs, expected_suppressed, final_edge) =
                match arm {
                    Arm::Sham => (5, vec![], 0, 4),
                    Arm::Damage => (4, vec![(3, 5, 4)], 0, 4),
                    Arm::Suppressed => (0, vec![], 5, 5),
                };
            assert_eq!(
                outputs,
                vec![(4, input); expected_outputs],
                "wrong sequence of native service outputs for arm {arm:?}"
            );
            assert_eq!(repairs, expected_repairs, "wrong repair trace for {arm:?}");
            assert_eq!(suppressed, expected_suppressed);
            match &actor.code.nodes[3] {
                Op::Read { next, .. } => assert_eq!(*next, final_edge),
                _ => panic!("protected code node was unexpectedly replaced"),
            }
        }
    }
}
