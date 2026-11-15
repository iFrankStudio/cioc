//! CL-1B B11: bounded, researcher-scripted CIL serial search over native code edits.
//! Protocol: cl1b/studies/B11-serial-native-candidate-search.md
//! The policy, objective and candidate neighborhood are NOT emergent.
use cl0_world::{EventKind, Intervention, Op, Origin, Process, World};

#[derive(Clone, Copy, Debug)]
enum Arm {
    Sham,
    Lesion8,
    Lesion9,
    Lesion10,
    PatchDisabled,
    StalePreseed,
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
        Op::Const { dst: 3, value: 1, next: 15 },
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
fn b11_one_native_actor_proposes_tests_and_retains_multiple_serial_code_candidates() {
    for input in [-13, 0, 7, 23, 101] {
        for arm in [
            Arm::Sham,
            Arm::Lesion8,
            Arm::Lesion9,
            Arm::Lesion10,
            Arm::PatchDisabled,
            Arm::StalePreseed,
        ] {
            let mut w = World::with_strict_capacity(16, 32, 64)
                .with_native_executable_edit_trace();
            if matches!(arm, Arm::PatchDisabled) {
                w = w.with_native_patchnext_disabled();
            }
            w.load(Process::new(1, founder(), 250)).unwrap();
            w.intervene(Intervention::SetCell { key: 7, value: input })
                .unwrap();

            if matches!(arm, Arm::StalePreseed) {
                w.intervene(Intervention::SetCell { key: 11, value: input })
                    .unwrap();
                w.intervene(Intervention::SetCell { key: 13, value: 1 })
                    .unwrap();
            }
            let lesion = match arm {
                Arm::Sham => None,
                Arm::Lesion8 => Some(8),
                Arm::Lesion9 => Some(9),
                Arm::Lesion10 | Arm::PatchDisabled | Arm::StalePreseed => Some(10),
            };
            if let Some(next) = lesion {
                w.intervene(Intervention::PatchEdge {
                    process: 1,
                    node: 6,
                    next,
                }).unwrap();
            }

            assert_eq!(w.run(100), 100);
            let p = w.process(1).unwrap();
            assert_eq!(p.id, 1);
            assert!(!p.stopped);
            assert!(w.process(2).is_none());
            assert_eq!(p.credits, 150);
            assert_eq!(w.total_compute_inventory(), Some(150));
            assert_eq!(
                w.events().iter().filter(|e| matches!(
                    e.kind, EventKind::Trapped(_) | EventKind::GraphLaunched { .. }
                )).count(), 0
            );
            assert!(w.events().iter().all(|e| {
                e.origin == Origin::Internal(1) || e.origin == Origin::External
            }));

            let edits: Vec<(usize, usize, usize)> = w.events().iter().filter_map(|e| {
                match (&e.origin, &e.kind) {
                    (Origin::Internal(1),
                        EventKind::ExecutableEdgePatched { node, before, after }) =>
                            Some((*node, *before, *after)),
                    _ => None,
                }
            }).collect();
            let expected_edits = match arm {
                Arm::Sham | Arm::PatchDisabled => vec![],
                Arm::Lesion8 => vec![(6, 8, 7)],
                Arm::Lesion9 => vec![(6, 9, 8), (6, 8, 7)],
                Arm::Lesion10 | Arm::StalePreseed =>
                    vec![(6, 10, 9), (6, 9, 8), (6, 8, 7)],
            };
            assert_eq!(edits, expected_edits, "{arm:?} input={input}");
            assert_eq!(
                edge6(p),
                if matches!(arm, Arm::PatchDisabled) { 10 } else { 7 }
            );

            let output: Vec<(usize, i64)> = w.events().iter().filter_map(|e| {
                match (&e.origin, &e.kind) {
                    (Origin::Internal(1),
                        EventKind::CellWritten { key: 11, value }) => Some((e.pc, *value)),
                    _ => None,
                }
            }).collect();
            let markers: Vec<(usize, i64)> = w.events().iter().filter_map(|e| {
                match (&e.origin, &e.kind) {
                    (Origin::Internal(1),
                        EventKind::CellWritten { key: 13, value }) => Some((e.pc, *value)),
                    _ => None,
                }
            }).collect();
            let side_effects: Vec<(usize, i64)> = w.events().iter().filter_map(|e| {
                match (&e.origin, &e.kind) {
                    (Origin::Internal(1),
                        EventKind::CellWritten { key: 1, value }) => Some((e.pc, *value)),
                    _ => None,
                }
            }).collect();

            let (services, marker_count, side_effect_count, last_pc) = match arm {
                Arm::Sham => (6, 6, 0, 4),
                Arm::Lesion8 => (6, 5, 0, 1),
                Arm::Lesion9 => (5, 4, 1, 19),
                Arm::Lesion10 | Arm::StalePreseed => (4, 3, 1, 18),
                Arm::PatchDisabled => (0, 0, 0, 18),
            };
            assert_eq!(output, vec![(9, input); services], "{arm:?}");
            assert_eq!(markers, vec![(7, 1); marker_count], "{arm:?}");
            assert_eq!(side_effects, vec![(9, input); side_effect_count], "{arm:?}");
            assert_eq!(p.pc, last_pc);
            assert_eq!(
                w.cell(1),
                if side_effect_count > 0 { Some(input) } else { None },
                "a bad candidate's extra storage charge must remain visible"
            );
            assert!(w.occupied_units() <= 64);

            let checks: Vec<i64> = w.events().iter().filter_map(|e| {
                match (&e.origin, &e.kind) {
                    (Origin::Internal(1),
                        EventKind::CellRead { key: 13, value }) if e.pc == 11 =>
                            Some(*value),
                    _ => None,
                }
            }).collect();
            let expected_failed_trials = match arm {
                Arm::Sham => 0,
                Arm::Lesion8 => 1,
                Arm::Lesion9 => 2,
                Arm::Lesion10 | Arm::StalePreseed => 3,
                Arm::PatchDisabled => checks.len(),
            };
            assert!(checks.len() >= expected_failed_trials);
            assert!(
                checks.iter().take(expected_failed_trials).all(|value| *value == 0),
                "every candidate proposal must follow a fresh failed trial"
            );
            if !matches!(arm, Arm::PatchDisabled) {
                assert!(checks.iter().skip(expected_failed_trials).all(|value| *value == 1));
            }
            let suppressions = w.events().iter().filter(|e| {
                e.origin == Origin::Internal(1)
                    && matches!(e.kind, EventKind::NativeCodeEditSuppressed { opcode_tag: 6 })
            }).count();
            assert_eq!(suppressions, if matches!(arm, Arm::PatchDisabled) { 6 } else { 0 });
            assert_eq!(
                w.events().iter().filter(|e| e.origin == Origin::External).count(),
                match arm {
                    Arm::Sham => 2,
                    Arm::StalePreseed => 5,
                    _ => 3,
                }
            );
        }
    }
}
