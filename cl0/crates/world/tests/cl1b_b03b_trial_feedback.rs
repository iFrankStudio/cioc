//! CL-1B B03b: one-CIL-actor trial-feedback conditional self-modification.
//! All policy/score logic is authored by researchers. This is NOT evolved selection.
use cl0_world::{EventKind, Intervention, Op, Origin, Process, World};

fn founder() -> Vec<Op> {
    vec![
        Op::Const { dst: 2, value: 7, next: 1 },
        Op::Const { dst: 3, value: 9, next: 2 },
        Op::Read { dst: 0, address: 2, next: 3 },
        Op::Const { dst: 2, value: 11, next: 4 },
        Op::Read { dst: 1, address: 2, next: 5 },
        Op::Add { dst: 1, lhs: 0, rhs: 1, next: 6 },
        Op::Write { address: 3, value: 1, next: 7 },
        Op::Sub { dst: 1, lhs: 1, rhs: 0, next: 8 },
        Op::Branch { test: 1, nonzero: 10, zero: 9 },
        Op::Const { dst: 1, value: 21, next: 11 },
        Op::Const { dst: 1, value: 20, next: 11 },
        Op::Const { dst: 2, value: 19, next: 12 },
        Op::PatchNext { node: 2, target: 1, next: 13 },
        Op::Const { dst: 2, value: 7, next: 14 },
        Op::Read { dst: 0, address: 2, next: 15 },
        Op::Const { dst: 3, value: 9, next: 16 },
        Op::Const { dst: 2, value: 11, next: 17 },
        Op::Read { dst: 1, address: 2, next: 18 },
        Op::Add { dst: 1, lhs: 0, rhs: 1, next: 19 },
        Op::Const { dst: 2, value: 7, next: 20 },
        Op::Const { dst: 2, value: 7, next: 22 },
        Op::Write { address: 3, value: 1, next: 25 },
        Op::Write { address: 3, value: 0, next: 25 },
        Op::Halt,
        Op::Halt,
        Op::Halt,
    ]
}

#[test]
fn cl1b_b03b_same_actor_native_trial_feedback_selects_next_code_path() {
    for input in [-13, 0, 7, 23, 101] {
        for delta in [0, 1] {
            for disable_patch in [false, true] {
                let source = founder();
                assert_eq!(source.len(), 26);
                let mut world = World::with_strict_capacity(16, 32, 64)
                    .with_native_executable_edit_trace();
                if disable_patch {
                    world = world.with_native_patchnext_disabled();
                }
                world.load(Process::new(1, source, 40)).unwrap();
                for (key, value) in [(7, input), (11, delta)] {
                    world.intervene(Intervention::SetCell { key, value }).unwrap();
                }

                let attempts = world.run(40);
                assert!(!world.clone().step(), "must reach natural Halt");
                let actor = world.process(1).unwrap();
                assert!(actor.stopped);
                assert_eq!(actor.id, 1);

                let choose_b = delta == 0 && !disable_patch;
                let expected_attempts = if choose_b { 21 } else { 22 };
                assert_eq!(attempts, expected_attempts);
                assert_eq!(actor.credits, 40 - attempts as u64);
                assert_eq!(world.total_compute_inventory(), Some(40 - attempts as u64));

                let writes: Vec<(usize, i64)> = world.events().iter().filter_map(|event| {
                    match (&event.origin, &event.kind) {
                        (Origin::Internal(1), EventKind::CellWritten { key: 9, value }) =>
                            Some((event.pc, *value)),
                        _ => None,
                    }
                }).collect();
                assert_eq!(
                    writes,
                    vec![
                        (6, input + delta),
                        (if choose_b { 21 } else { 22 }, input),
                    ],
                    "trial/final service sequence must match"
                );

                let patches: Vec<(usize, usize, usize)> = world.events().iter().filter_map(|event| {
                    match (&event.origin, &event.kind) {
                        (Origin::Internal(1), EventKind::ExecutableEdgePatched { node, before, after }) =>
                            Some((*node, *before, *after)),
                        _ => None,
                    }
                }).collect();
                let suppressions = world.events().iter().filter(|event| {
                    event.origin == Origin::Internal(1) &&
                        matches!(event.kind, EventKind::NativeCodeEditSuppressed { opcode_tag: 6 })
                }).count();
                assert_eq!(patches, if choose_b { vec![(19, 20, 21)] } else { vec![] });
                assert_eq!(suppressions, usize::from(disable_patch));

                match &actor.code.nodes[19] {
                    Op::Const { next, .. } => assert_eq!(*next, if choose_b { 21 } else { 20 }),
                    _ => panic!("program target changed instruction type"),
                }
                assert_eq!(world.events().iter().filter(|event| event.origin == Origin::External).count(), 3);
                assert_eq!(
                    world.events().iter().filter(|event| matches!(
                        event.kind, EventKind::Trapped(_) | EventKind::GraphLaunched { .. }
                    )).count(),
                    0
                );
                assert!(world.events().iter().all(|event| {
                    event.origin == Origin::External || event.origin == Origin::Internal(1)
                }));
            }
        }
    }
}
