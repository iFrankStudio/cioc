//! CL-1B B00: narrow human-authored SELF-code-edit substrate feasibility.
//! Not an autonomous adaptive/rollback experiment and not a life claim.
use cl0_world::{EventKind, Op, Origin, Process, World};

fn founder() -> Vec<Op> {
    vec![
        Op::Const {
            dst: 0,
            value: 3,
            next: 1,
        },
        Op::Const {
            dst: 1,
            value: 4,
            next: 2,
        },
        Op::PatchNext {
            node: 0,
            target: 1,
            next: 3,
        },
        Op::Const {
            dst: 2,
            value: 99,
            next: 5,
        },
        Op::Const {
            dst: 2,
            value: 42,
            next: 5,
        },
        Op::Halt,
    ]
}

#[test]
fn cl1b_b00_single_process_native_self_edit_vs_charged_suppression() {
    let initial = founder();
    let mut observed = Vec::new();
    for suppress_patch in [false, true] {
        let mut world =
            World::with_strict_capacity(8, 8, 16).with_native_executable_edit_trace();
        if suppress_patch {
            world = world.with_native_patchnext_disabled();
        }
        world.load(Process::new(1, initial.clone(), 12)).unwrap();
        let attempts = world.run(12);
        assert!(!world.clone().step(), "must naturally terminate");
        let actor = world.process(1).unwrap();
        assert!(actor.stopped);
        let expected_next = if suppress_patch { 5 } else { 4 };
        let expected_r2 = if suppress_patch { 99 } else { 42 };
        assert_eq!(
            actor.code.nodes[3],
            (Op::Const {
                dst: 2,
                value: 99,
                next: expected_next
            })
        );
        assert_eq!(actor.registers[2], expected_r2);
        assert_eq!(attempts, if suppress_patch { 5 } else { 6 });
        assert_eq!(actor.credits, 12 - attempts as u64);
        assert_eq!(
            world.total_compute_inventory(),
            Some(12 - attempts as u64)
        );
        assert_eq!(
            world
                .events()
                .iter()
                .filter(|e| matches!(e.kind, EventKind::GraphLaunched { .. }))
                .count(),
            0
        );
        let edits = world
            .events()
            .iter()
            .filter(|e| {
                e.origin == Origin::Internal(1)
                    && matches!(
                        e.kind,
                        EventKind::ExecutableEdgePatched {
                            node: 3,
                            before: 5,
                            after: 4
                        }
                    )
            })
            .count();
        let suppressed = world
            .events()
            .iter()
            .filter(|e| {
                e.origin == Origin::Internal(1)
                    && matches!(e.kind, EventKind::NativeCodeEditSuppressed { .. })
            })
            .count();
        assert_eq!(edits, usize::from(!suppress_patch));
        assert_eq!(suppressed, usize::from(suppress_patch));
        assert!(
            world
                .events()
                .iter()
                .all(|e| e.origin == Origin::Internal(1) || e.origin == Origin::External)
        );
        observed.push((attempts, expected_r2));
    }
    assert_eq!(observed, [(6, 42), (5, 99)]);
}
