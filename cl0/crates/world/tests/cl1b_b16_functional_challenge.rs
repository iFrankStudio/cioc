//! CL-1B B16: one actor probes actual payload production with its own
//! nonzero challenge, then native-edits a damaged code edge.
//! Human-authored task/diagnosis; checker-path corruption remains a falsifier.
//! See cl1b/studies/B16-functional-challenge-and-forgery.md.
use cl0_world::{EventKind, Intervention, Op, Origin, Process, World};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Arm {
    Intact,
    Skip,
    Stale,
    Disabled,
    CheckerBypass,
    TimedForgery,
}

fn founder() -> Vec<Op> {
    vec![
        Op::Const { dst: 0, value: 15, next: 1 },
        Op::Const { dst: 2, value: 7, next: 2 },
        Op::Read { dst: 1, address: 2, next: 3 },
        Op::Write { address: 0, value: 1, next: 4 },
        Op::Const { dst: 1, value: 17, next: 5 },
        Op::Write { address: 2, value: 1, next: 6 },
        Op::Const { dst: 2, value: 11, next: 7 },
        Op::Delete { address: 2, next: 8 },
        Op::Const { dst: 2, value: 7, next: 9 },
        Op::Read { dst: 1, address: 2, next: 10 },
        Op::Const { dst: 3, value: 11, next: 11 },
        Op::Write { address: 3, value: 1, next: 12 },
        Op::Const { dst: 2, value: 11, next: 13 },
        Op::Read { dst: 2, address: 2, next: 14 },
        Op::Const { dst: 3, value: 17, next: 15 },
        Op::Sub { dst: 2, lhs: 2, rhs: 3, next: 16 },
        Op::Branch { test: 2, nonzero: 17, zero: 24 },
        Op::Const { dst: 0, value: 10, next: 18 },
        Op::ReadNext { dst: 1, node: 0, next: 19 },
        Op::Const { dst: 3, value: 1, next: 20 },
        Op::Sub { dst: 1, lhs: 1, rhs: 3, next: 21 },
        Op::PatchNext { node: 0, target: 1, next: 24 },
        Op::Halt,
        Op::Halt,
        Op::Const { dst: 2, value: 15, next: 25 },
        Op::Read { dst: 1, address: 2, next: 26 },
        Op::Const { dst: 2, value: 7, next: 27 },
        Op::Write { address: 2, value: 1, next: 28 },
        Op::Const { dst: 2, value: 7, next: 29 },
        Op::Read { dst: 1, address: 2, next: 30 },
        Op::Const { dst: 3, value: 11, next: 31 },
        Op::Write { address: 3, value: 1, next: 0 },
    ]
}

fn probe_edge(p: &Process) -> usize {
    match &p.code.nodes[10] {
        Op::Const { next, .. } => *next,
        _ => panic!("probe service entry must remain Const"),
    }
}

#[test]
fn b16_actor_issued_challenge_detects_skipped_payload_but_not_corrupted_checker() {
    for input in [-13, 0, 7, 23, 101] {
        for arm in [
            Arm::Intact,
            Arm::Skip,
            Arm::Stale,
            Arm::Disabled,
            Arm::CheckerBypass,
            Arm::TimedForgery,
        ] {
            let mut w = World::with_strict_capacity(16, 64, 128)
                .with_native_executable_edit_trace();
            if arm == Arm::Disabled {
                w = w.with_native_patchnext_disabled();
            }
            w.load(Process::new(1, founder(), 240)).unwrap();
            w.intervene(Intervention::SetCell { key: 7, value: input })
                .unwrap();
            if arm == Arm::Stale {
                w.intervene(Intervention::SetCell { key: 11, value: 17 })
                    .unwrap();
            }
            let changed = match arm {
                Arm::Intact => None,
                Arm::CheckerBypass => Some(13),
                _ => Some(12),
            };
            if let Some(next) = changed {
                w.intervene(Intervention::PatchEdge {
                    process: 1, node: 10, next,
                }).unwrap();
            }
            assert_eq!(founder().len(), 32);
            if arm == Arm::TimedForgery {
                assert_eq!(w.run(12), 12);
                assert_eq!(w.process(1).unwrap().pc, 13);
                assert_eq!(w.cell(11), None);
                assert_eq!(w.cell(7), Some(17));
                assert_eq!(probe_edge(w.process(1).unwrap()), 12);
                w.intervene(Intervention::SetCell { key: 11, value: 17 })
                    .unwrap();
                assert_eq!(w.run(108), 108);
            } else {
                assert_eq!(w.run(120), 120);
            }
            let actor = w.process(1).unwrap();
            assert_eq!(actor.id, 1);
            assert_eq!(actor.credits, 120);
            assert_eq!(w.total_compute_inventory(), Some(120));
            assert!(!actor.stopped);
            assert!(w.process(2).is_none());
            assert!(w.occupied_units() <= 128);
            assert_eq!(
                w.events().iter().filter(|e| matches!(
                    e.kind, EventKind::Trapped(_) | EventKind::GraphLaunched { .. }
                )).count(), 0
            );
            assert_eq!(
                w.events().iter().filter(|e| e.origin == Origin::External).count(),
                match arm {
                    Arm::Intact => 2,
                    Arm::Stale | Arm::TimedForgery => 4,
                    _ => 3,
                }
            );

            let probe_outputs: Vec<i64> = w.events().iter().filter_map(|e| match (&e.origin, &e.kind) {
                (Origin::Internal(1), EventKind::CellWritten { key: 11, value }) if e.pc == 11 => Some(*value),
                _ => None,
            }).collect();
            let regular_outputs: Vec<i64> = w.events().iter().filter_map(|e| match (&e.origin, &e.kind) {
                (Origin::Internal(1), EventKind::CellWritten { key: 11, value }) if e.pc == 31 => Some(*value),
                _ => None,
            }).collect();
            assert!(probe_outputs.iter().all(|v| *v == 17), "the native challenge output must be17");
            assert!(regular_outputs.iter().all(|v| *v == input), "the original user input must be restored");
            assert!(!regular_outputs.is_empty(), "ordinary service is distinct from challenge work");
            let (expected_challenge, expected_regular, expected_suppressions) = match arm {
                Arm::Intact => (5, 4, 0),
                Arm::Skip | Arm::Stale => (4, 4, 0),
                Arm::Disabled => (0, 4, 4),
                Arm::CheckerBypass => (0, 5, 0),
                Arm::TimedForgery => (3, 4, 0),
            };
            assert_eq!(probe_outputs.len(), expected_challenge, "{arm:?} input={input}");
            assert_eq!(regular_outputs.len(), expected_regular, "{arm:?} input={input}");

            let patches: Vec<(usize, usize, usize)> = w.events().iter().filter_map(|e| match (&e.origin, &e.kind) {
                (Origin::Internal(1), EventKind::ExecutableEdgePatched { node, before, after })
                    => Some((*node, *before, *after)),
                _ => None,
            }).collect();
            let suppressed = w.events().iter().filter(|e| {
                e.origin == Origin::Internal(1)
                    && matches!(e.kind, EventKind::NativeCodeEditSuppressed { opcode_tag: 6 })
            }).count();
            assert_eq!(suppressed, expected_suppressions, "{arm:?} input={input}");
            let checks: Vec<(i64, i64)> = w.events().iter().filter_map(|e| match (&e.origin, &e.kind) {
                (Origin::Internal(1), EventKind::CellRead { key, value }) if e.pc == 13 =>
                    Some((*key, *value)),
                _ => None,
            }).collect();
            assert!(!checks.is_empty());

            match arm {
                Arm::Intact => {
                    assert!(!probe_outputs.is_empty());
                    assert!(patches.is_empty());
                    assert_eq!(probe_edge(actor), 11);
                    assert_eq!(checks[0], (11, 17));
                }
                Arm::Skip | Arm::Stale => {
                    assert!(!probe_outputs.is_empty());
                    assert_eq!(patches, vec![(10, 12, 11)]);
                    assert_eq!(probe_edge(actor), 11);
                    assert_eq!(checks[0], (11, 0));
                    assert_eq!(suppressed, 0);
                }
                Arm::Disabled => {
                    assert!(probe_outputs.is_empty(), "no actual tested payload write");
                    assert!(patches.is_empty());
                    assert_eq!(probe_edge(actor), 12);
                    assert_eq!(checks[0], (11, 0));
                    assert!(suppressed > 0);
                }
                Arm::CheckerBypass => {
                    assert!(probe_outputs.is_empty(), "internal result checker is deceived");
                    assert!(patches.is_empty());
                    assert_eq!(probe_edge(actor), 13);
                    assert_eq!(checks[0], (7, 17), "wrong address read creates internal false positive");
                }
                Arm::TimedForgery => {
                    assert!(!probe_outputs.is_empty());
                    assert_eq!(patches, vec![(10, 12, 11)]);
                    assert_eq!(probe_edge(actor), 11);
                    assert_eq!(checks[0], (11, 17), "externally injected fake response");
                    assert!(checks.iter().any(|(key, value)| *key == 11 && *value == 0),
                        "a later unspoofed trial must expose the missing payload");
                }
            }

            let deleted = w.events().iter().filter(|e| {
                e.origin == Origin::Internal(1) && e.pc == 7
            }).count();
            assert!(deleted > 0);
            let original_saves = w.events().iter().filter(|e| matches!(
                (&e.origin, &e.kind),
                (Origin::Internal(1), EventKind::CellWritten { key: 15, value })
                    if e.pc == 3 && *value == input
            )).count();
            assert!(original_saves > 0, "CIL itself must preserve original input");
        }
    }
}
