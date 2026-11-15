//! CL-1B B10: authored CIL freshness marker calibration and timed spoof.
 //! The policy is designed by investigators, not emergent cyberlife.
 //! Protocol: cl1b/studies/B10-native-freshness-and-spoof.md
use cl0_world::{EventKind, Intervention, Op, Origin, Process, World};

#[derive(Clone, Copy, Debug)]
enum Arm {
    Intact,
    Damaged,
    StaleBoth,
    Disabled,
    TimedSpoof,
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
fn b10_native_freshness_defeats_stale_values_but_timed_spoof_fools_one_trial() {
    for input in [-13, 0, 7, 23, 101] {
        for arm in [
            Arm::Intact,
            Arm::Damaged,
            Arm::StaleBoth,
            Arm::Disabled,
            Arm::TimedSpoof,
        ] {
            let mut w = World::with_strict_capacity(16, 32, 64)
                .with_native_executable_edit_trace();
            if matches!(arm, Arm::Disabled) {
                w = w.with_native_patchnext_disabled();
            }
            w.load(Process::new(1, founder(), 200)).unwrap();
            w.intervene(Intervention::SetCell { key: 7, value: input })
                .unwrap();
            if matches!(arm, Arm::StaleBoth) {
                w.intervene(Intervention::SetCell { key: 11, value: input })
                    .unwrap();
                w.intervene(Intervention::SetCell { key: 13, value: 1 })
                    .unwrap();
            }
            if !matches!(arm, Arm::Intact) {
                w.intervene(Intervention::PatchEdge {
                    process: 1, node: 6, next: 10,
                }).unwrap();
            }

            if matches!(arm, Arm::TimedSpoof) {
                assert_eq!(w.run(8), 8, "PC0..6,10 must be attempted before PC11");
                assert_eq!(w.process(1).unwrap().pc, 11);
                assert_eq!(w.cell(13), None, "native PC2 must clear marker");
                assert_eq!(w.cell(11), None, "no native service write occurred");
                assert_eq!(edge6(w.process(1).unwrap()), 10);
                w.intervene(Intervention::SetCell { key: 13, value: 1 }).unwrap();
                assert_eq!(w.run(72), 72);
            } else {
                assert_eq!(w.run(80), 80);
            }

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

            let edits: Vec<(usize, usize, usize)> = w.events().iter().filter_map(|e| {
                if e.origin != Origin::Internal(1) { return None; }
                match &e.kind {
                    EventKind::ExecutableEdgePatched { node, before, after } =>
                        Some((*node, *before, *after)),
                    _ => None,
                }
            }).collect();
            assert_eq!(
                edits,
                if matches!(arm, Arm::Damaged | Arm::StaleBoth | Arm::TimedSpoof) {
                    vec![(6, 10, 7)]
                } else { vec![] },
                "unexpected self-edit in {arm:?} input={input}"
            );
            assert_eq!(
                edge6(p),
                if matches!(arm, Arm::Disabled) { 10 } else { 7 }
            );
            let readbacks: Vec<(usize, i64)> = w.events().iter().filter_map(|e| {
                match (&e.origin, &e.kind) {
                    (Origin::Internal(1), EventKind::CellRead { key: 13, value }) =>
                        Some((e.pc, *value)),
                    _ => None,
                }
            }).collect();
            assert!(!readbacks.is_empty());
            assert_eq!(readbacks[0], (11, if matches!(arm, Arm::Intact | Arm::TimedSpoof) { 1 } else { 0 }));
            // A forged marker creates a false 'healthy' decision before any
            // native source of service evidence appears.
            if matches!(arm, Arm::TimedSpoof) {
                let first_read_tick = w.events().iter().find(|e| {
                    e.origin == Origin::Internal(1) && e.pc == 11
                        && matches!(e.kind, EventKind::CellRead { key: 13, value: 1 })
                }).unwrap().tick;
                let first_edit_tick = w.events().iter().find(|e| {
                    e.origin == Origin::Internal(1)
                        && matches!(e.kind, EventKind::ExecutableEdgePatched { .. })
                }).unwrap().tick;
                assert!(first_edit_tick > first_read_tick);
            }

            let service_writes = w.events().iter().filter(|e| {
                e.origin == Origin::Internal(1) && e.pc == 9
                    && matches!(e.kind, EventKind::CellWritten { key: 11, value } if value == input)
            }).count();
            assert_eq!(
                service_writes,
                match arm {
                    Arm::Intact => 5,
                    Arm::Damaged | Arm::StaleBoth => 4,
                    Arm::Disabled => 0,
                    Arm::TimedSpoof => 3,
                },
                "full instruction-window service count; zero input is not missing evidence"
            );
            assert_eq!(
                w.events().iter().filter(|e| {
                    e.origin == Origin::Internal(1) && e.pc == 7
                        && matches!(e.kind, EventKind::CellWritten { key: 13, value: 1 })
                }).count(),
                service_writes,
                "each genuine source-produced marker must accompany a service output"
            );
            assert!(w.events().iter().any(|e| {
                e.origin == Origin::Internal(1) && e.pc == 2
            }), "the actor must execute its own marker deletion");
            assert_eq!(
                w.events().iter().filter(|e| e.origin == Origin::External).count(),
                match arm {
                    Arm::Intact => 2,
                    Arm::Damaged | Arm::Disabled => 3,
                    Arm::StaleBoth => 5,
                    Arm::TimedSpoof => 4,
                }
            );
        }
    }
}
