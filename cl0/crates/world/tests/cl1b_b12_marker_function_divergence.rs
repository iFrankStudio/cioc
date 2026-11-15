//! CL-1B B12 negative test: a native marker is not proof of real service output.
//! Researcher-authored CIL and researcher-chosen single-edge lesions, not life.
use cl0_world::{EventKind, Intervention, Op, Origin, Process, World};

#[derive(Clone, Copy, Debug)]
enum Arm {
    Intact,
    MarkerExit,
    OutputExit,
    MarkerSkip,
    MarkerSkipPatchDisabled,
}

fn founder() -> Vec<Op> {
    vec![
        Op::Const { dst: 0, value: 6, next: 1 },
        Op::Const { dst: 2, value: 13, next: 2 },
        Op::Delete { address: 2, next: 3 },
        Op::Const { dst: 2, value: 7, next: 4 },
        Op::Read { dst: 1, address: 2, next: 5 },
        Op::Const { dst: 3, value: 1, next: 6 },
        Op::Const { dst: 2, value: 13, next: 7 },
        Op::Write { address: 2, value: 3, next: 8 },
        Op::Const { dst: 3, value: 11, next: 9 },
        Op::Write { address: 3, value: 1, next: 10 },
        Op::Const { dst: 2, value: 13, next: 11 },
        Op::Read { dst: 2, address: 2, next: 12 },
        Op::Branch { test: 2, nonzero: 17, zero: 13 },
        Op::ReadNext { dst: 1, node: 0, next: 14 },
        Op::Const { dst: 3, value: 3, next: 15 },
        Op::Sub { dst: 1, lhs: 1, rhs: 3, next: 16 },
        Op::PatchNext { node: 0, target: 1, next: 17 },
        Op::Const { dst: 2, value: 13, next: 18 },
        Op::Const { dst: 3, value: 11, next: 19 },
        Op::Const { dst: 0, value: 6, next: 0 },
    ]
}

fn edge6(p: &Process) -> usize {
    match &p.code.nodes[6] {
        Op::Const { next, .. } => *next,
        _ => panic!("expected Const target"),
    }
}


#[test]
fn b12_same_actor_can_emit_fresh_success_marker_while_real_service_is_absent() {
    for input in [-13, 0, 7, 23, 101] {
        for arm in [
            Arm::Intact,
            Arm::MarkerExit,
            Arm::OutputExit,
            Arm::MarkerSkip,
            Arm::MarkerSkipPatchDisabled,
        ] {
            let mut w = World::with_strict_capacity(16, 32, 64)
                .with_native_executable_edit_trace();
            if matches!(arm, Arm::MarkerSkipPatchDisabled) {
                w = w.with_native_patchnext_disabled();
            }
            w.load(Process::new(1, founder(), 200)).unwrap();
            w.intervene(Intervention::SetCell { key: 7, value: input }).unwrap();
            if let Some((node, next)) = match arm {
                Arm::Intact => None,
                Arm::MarkerExit => Some((7, 10)),
                Arm::OutputExit => Some((8, 10)),
                Arm::MarkerSkip | Arm::MarkerSkipPatchDisabled => Some((6, 8)),
            } {
                w.intervene(Intervention::PatchEdge { process: 1, node, next }).unwrap();
            }

            assert_eq!(w.run(80), 80, "fixed opportunity; no hidden retries");
            let p = w.process(1).unwrap();
            assert_eq!(p.id, 1);
            assert!(!p.stopped);
            assert!(w.process(2).is_none());
            assert_eq!(p.credits, 120);
            assert_eq!(w.total_compute_inventory(), Some(120));
            assert_eq!(
                w.events().iter().filter(|e| matches!(
                    e.kind, EventKind::Trapped(_) | EventKind::GraphLaunched { .. }
                )).count(), 0
            );

            let markers: Vec<(usize, i64)> = w.events().iter().filter_map(|e| {
                match (&e.origin, &e.kind) {
                    (Origin::Internal(1),
                        EventKind::CellWritten { key: 13, value }) =>
                            Some((e.pc, *value)),
                    _ => None
                }
            }).collect();
            let services: Vec<(usize, i64)> = w.events().iter().filter_map(|e| {
                match (&e.origin, &e.kind) {
                    (Origin::Internal(1),
                        EventKind::CellWritten { key: 11, value }) =>
                            Some((e.pc, *value)),
                    _ => None
                }
            }).collect();
            let marker_reads: Vec<(usize, i64)> = w.events().iter().filter_map(|e| {
                match (&e.origin, &e.kind) {
                    (Origin::Internal(1),
                        EventKind::CellRead { key: 13, value }) =>
                            Some((e.pc, *value)),
                    _ => None
                }
            }).collect();
            let edits: Vec<(usize, usize, usize)> = w.events().iter().filter_map(|e| {
                match (&e.origin, &e.kind) {
                    (Origin::Internal(1),
                        EventKind::ExecutableEdgePatched { node, before, after }) =>
                            Some((*node, *before, *after)),
                    _ => None
                }
            }).collect();
            let suppressed = w.events().iter().filter(|e| {
                e.origin == Origin::Internal(1)
                    && matches!(e.kind, EventKind::NativeCodeEditSuppressed { opcode_tag: 6 })
            }).count();
            assert!(w.events().iter().all(|e| {
                e.origin == Origin::External || e.origin == Origin::Internal(1)
            }));
            assert_eq!(
                w.events().iter().filter(|e| e.origin == Origin::External).count(),
                if matches!(arm, Arm::Intact) { 2 } else { 3 }
            );
            assert!(markers.iter().all(|(pc, value)| *pc == 7 && *value == 1));
            assert!(services.iter().all(|(pc, value)| *pc == 9 && *value == input));
            assert!(marker_reads.iter().all(|(pc, value)| *pc == 11 && (*value == 0 || *value == 1)));
            let (expected_markers, expected_services, expected_reads, expected_suppression) =
                match arm {
                    Arm::Intact => (5, 5, 5, 0),
                    Arm::MarkerExit => (6, 0, 6, 0),
                    Arm::OutputExit => (5, 0, 5, 0),
                    Arm::MarkerSkip => (0, 1, 1, 0),
                    Arm::MarkerSkipPatchDisabled => (0, 4, 4, 4),
                };
            assert_eq!(markers.len(), expected_markers, "{arm:?}");
            assert_eq!(services.len(), expected_services, "{arm:?}");
            assert_eq!(marker_reads.len(), expected_reads, "{arm:?}");
            assert_eq!(suppressed, expected_suppression, "{arm:?}");

            match arm {
                Arm::Intact => {
                    assert!(!markers.is_empty());
                    assert_eq!(markers.len(), services.len());
                    assert!(edits.is_empty());
                    assert_eq!(edge6(p), 7);
                }
                Arm::MarkerExit | Arm::OutputExit => {
                    assert!(!markers.is_empty(), "native marker must be generated afresh");
                    assert!(services.is_empty(), "marker falsely certifies absent service");
                    assert!(marker_reads.iter().any(|(_, value)| *value == 1));
                    assert!(edits.is_empty(), "false-positive blocks native repair");
                    assert_eq!(edge6(p), 7);
                }
                Arm::MarkerSkip => {
                    assert!(!services.is_empty(), "service actually worked before attempted repair");
                    assert!(markers.is_empty(), "real service without fresh marker");
                    assert!(marker_reads.iter().any(|(_, value)| *value == 0));
                    assert_eq!(edits.first(), Some(&(6, 8, 5)));
                    assert_eq!(edge6(p), 5, "repair may create a self-loop");
                }
                Arm::MarkerSkipPatchDisabled => {
                    assert!(!services.is_empty(), "blocked edit preserves usable service");
                    assert!(markers.is_empty());
                    assert!(marker_reads.iter().any(|(_, value)| *value == 0));
                    assert!(edits.is_empty());
                    assert!(suppressed > 0);
                    assert_eq!(edge6(p), 8);
                }
            }
        }
    }
}
