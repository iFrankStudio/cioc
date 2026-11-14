//! Study81 observer-only native PatchNext provenance, charged edit/no-edit controls.
use cl0_world::{EventKind, Op, Origin, Process, World};

fn initial(target: i64, disabled: bool) -> World {
    let nodes = vec![
        Op::Const { dst: 1, value: 2, next: 1 },
        Op::Const { dst: 0, value: target, next: 2 },
        Op::PatchNext { node: 1, target: 0, next: 3 },
        Op::Halt,
    ];
    let mut w = World::with_strict_capacity(0, 8, 12)
        .with_native_executable_edit_trace();
    if disabled {
        w = w.with_native_code_edits_disabled();
    }
    w.load(Process::new(1, nodes, 5)).unwrap();
    w
}

#[test]
fn native_patch_observation_is_actual_edge_change_and_preserves_matched_cost() {
    let mut enabled = initial(0, false);
    let mut disabled = initial(0, true);
    assert_eq!(enabled.process(1), disabled.process(1));
    assert_eq!(enabled.run(3), 3);
    assert_eq!(disabled.run(3), 3);
    assert_eq!(enabled.process(1).unwrap().pc, 3);
    assert_eq!(disabled.process(1).unwrap().pc, 3);
    assert_eq!(enabled.process(1).unwrap().credits, 2);
    assert_eq!(disabled.process(1).unwrap().credits, 2);
    assert_eq!(enabled.total_compute_inventory(), disabled.total_compute_inventory());
    assert!(matches!(
        enabled.process(1).unwrap().code.nodes[2],
        Op::PatchNext { next: 0, .. }
    ));
    assert!(matches!(
        disabled.process(1).unwrap().code.nodes[2],
        Op::PatchNext { next: 3, .. }
    ));
    assert!(enabled.events().iter().any(|e| {
        e.origin == Origin::Internal(1)
            && e.pc == 2
            && matches!(
                e.kind,
                EventKind::ExecutableEdgePatched { node: 2, before: 3, after: 0 }
            )
    }));
    assert!(disabled.events().iter().any(|e| {
        e.origin == Origin::Internal(1)
            && e.pc == 2
            && matches!(e.kind, EventKind::NativeCodeEditSuppressed { opcode_tag: 6 })
    }));
}

#[test]
fn trace_does_not_fake_effective_edit_when_edge_value_is_unchanged() {
    let mut world = initial(3, false);
    assert_eq!(world.run(3), 3);
    assert!(matches!(
        world.process(1).unwrap().code.nodes[2],
        Op::PatchNext { next: 3, .. }
    ));
    assert!(!world.events().iter().any(|e| {
        matches!(e.kind, EventKind::ExecutableEdgePatched { .. })
    }));
}
