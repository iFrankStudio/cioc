//! CL-1B B14: one CIL process mutates its own candidate generator literal.
//! Human-authored feedback and policy: this is a mechanics calibration,
//! NOT autonomous evolutionary origin, an agent-defined goal or a B3 result.
use cl0_world::{EventKind, Intervention, Op, Origin, Process, World};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Arm {
    Sham7,
    Lesion10,
    Lesion10PolicyLocked,
    Lesion10PatchNextDisabled,
    Lesion11,
    Lesion11PolicyLocked,
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
        Op::Branch { test: 2, nonzero: 32, zero: 13 },
        Op::Const { dst: 2, value: 12, next: 14 },
        Op::Read { dst: 2, address: 2, next: 15 },
        Op::Branch { test: 2, nonzero: 20, zero: 16 },
        Op::Const { dst: 3, value: 1, next: 17 },
        Op::Const { dst: 2, value: 12, next: 18 },
        Op::Write { address: 2, value: 3, next: 27 },
        Op::Halt,
        Op::Const { dst: 2, value: 29, next: 21 },
        Op::Const { dst: 3, value: 1, next: 22 },
        Op::PatchConst { node: 2, value: 3, next: 23 },
        Op::Const { dst: 2, value: 12, next: 24 },
        Op::Const { dst: 3, value: 2, next: 25 },
        Op::Write { address: 2, value: 3, next: 27 },
        Op::Halt,
        Op::Const { dst: 0, value: 6, next: 28 },
        Op::ReadNext { dst: 1, node: 0, next: 29 },
        Op::Const { dst: 3, value: 2, next: 30 },
        Op::Sub { dst: 1, lhs: 1, rhs: 3, next: 31 },
        Op::PatchNext { node: 0, target: 1, next: 32 },
        Op::Const { dst: 2, value: 13, next: 33 },
        Op::Const { dst: 3, value: 11, next: 34 },
        Op::Const { dst: 0, value: 6, next: 0 },
    ]
}

fn policy_step(p: &Process) -> i64 {
    match &p.code.nodes[29] {
        Op::Const { value, .. } => *value,
        _ => panic!("candidate step must be in editable Const at PC29"),
    }
}

fn source_edge(p: &Process) -> usize {
    match &p.code.nodes[6] {
        Op::Const { next, .. } => *next,
        _ => panic!("source edit must preserve native opcode"),
    }
}

#[test]
fn cl1b_b14_actor_changes_its_own_future_candidate_policy_but_can_overfit() {
    for input in [-13, 0, 7, 23, 101] {
        for arm in [
            Arm::Sham7,
            Arm::Lesion10,
            Arm::Lesion10PolicyLocked,
            Arm::Lesion10PatchNextDisabled,
            Arm::Lesion11,
            Arm::Lesion11PolicyLocked,
        ] {
            let mut world =
                World::with_strict_capacity(16, 64, 128).with_native_executable_edit_trace();
            if arm == Arm::Lesion10PatchNextDisabled {
                world = world.with_native_patchnext_disabled();
            }
            world.load(Process::new(1, founder(), 240)).unwrap();
            world
                .intervene(Intervention::SetCell { key: 7, value: input })
                .unwrap();
            let lesion = match arm {
                Arm::Sham7 => None,
                Arm::Lesion10 | Arm::Lesion10PolicyLocked | Arm::Lesion10PatchNextDisabled => {
                    Some(10)
                }
                Arm::Lesion11 | Arm::Lesion11PolicyLocked => Some(11),
            };
            if let Some(next) = lesion {
                world
                    .intervene(Intervention::PatchEdge {
                        process: 1,
                        node: 6,
                        next,
                    })
                    .unwrap();
            }
            if matches!(arm, Arm::Lesion10PolicyLocked | Arm::Lesion11PolicyLocked) {
                // Researcher intervention disables only PC22's code-policy edit.
                // Unlike the patch-next-disabled world law this also changes
                // program flow and is explicitly a non-cost-matched control.
                world
                    .intervene(Intervention::PatchEdge {
                        process: 1,
                        node: 21,
                        next: 23,
                    })
                    .unwrap();
            }

            assert_eq!(founder().len(), 35);
            assert_eq!(policy_step(world.process(1).unwrap()), 2);
            let mut policy_mutations: Vec<(i64, i64)> = Vec::new();
            for _ in 0..120 {
                let pc = world.process(1).unwrap().pc;
                let before = policy_step(world.process(1).unwrap());
                assert!(world.step(), "{arm:?}: actor must remain schedulable");
                if pc == 22 {
                    let after = policy_step(world.process(1).unwrap());
                    policy_mutations.push((before, after));
                }
            }
            let actor = world.process(1).unwrap();
            assert_eq!(actor.id, 1);
            assert!(!actor.stopped);
            assert!(world.process(2).is_none());
            assert_eq!(actor.credits, 120);
            assert_eq!(world.total_compute_inventory(), Some(120));
            assert!(world.occupied_units() <= 128);
            assert_eq!(
                world.events().iter().filter(|e| matches!(
                    e.kind, EventKind::Trapped(_) | EventKind::GraphLaunched { .. }
                )).count(), 0
            );
            assert!(world.events().iter().all(|e| {
                e.origin == Origin::External || e.origin == Origin::Internal(1)
            }));
            let observed_edits: Vec<(usize, usize, usize)> = world
                .events()
                .iter()
                .filter_map(|e| match (&e.origin, &e.kind) {
                    (
                        Origin::Internal(1),
                        EventKind::ExecutableEdgePatched { node, before, after },
                    ) => Some((*node, *before, *after)),
                    _ => None,
                })
                .collect();
            let marker_count = world.events().iter().filter(|e| matches!(
                (&e.origin, &e.kind),
                (Origin::Internal(1), EventKind::CellWritten { key: 13, value: 1 })
            )).count();
            let service_count = world.events().iter().filter(|e| matches!(
                (&e.origin, &e.kind),
                (Origin::Internal(1), EventKind::CellWritten { key: 11, value })
                    if *value == input
            )).count();
            let wrong_key_count = world.events().iter().filter(|e| matches!(
                (&e.origin, &e.kind),
                (Origin::Internal(1), EventKind::CellWritten { key: 1, value })
                    if *value == input
            )).count();
            let suppressions = world.events().iter().filter(|e| {
                e.origin == Origin::Internal(1)
                    && matches!(e.kind, EventKind::NativeCodeEditSuppressed { opcode_tag: 6 })
            }).count();
            let (expected_edits, expected_policy, expected_policy_attempts,
                expected_services, expected_markers, expected_wrong, expected_edge,
                expected_pc, expected_suppressed) = match arm {
                Arm::Sham7 => (vec![], 2, vec![], 7, 8, 0, 7, 8, 0),
                Arm::Lesion10 => (
                    vec![(6, 10, 8), (6, 8, 7)], 1, vec![(2, 1)],
                    5, 4, 0, 7, 3, 0
                ),
                Arm::Lesion10PolicyLocked => (
                    vec![(6, 10, 8), (6, 8, 6)], 2, vec![],
                    1, 0, 0, 6, 6, 0
                ),
                Arm::Lesion10PatchNextDisabled => (
                    vec![], 1, vec![(2, 1), (1, 1), (1, 1)],
                    0, 0, 0, 10, 22, 4
                ),
                Arm::Lesion11 => (
                    vec![(6, 11, 9), (6, 9, 8), (6, 8, 7)],
                    1, vec![(2, 1), (1, 1)],
                    3, 3, 1, 7, 8, 0
                ),
                Arm::Lesion11PolicyLocked => (
                    vec![(6, 11, 9), (6, 9, 7)], 2, vec![],
                    4, 4, 1, 7, 6, 0
                ),
            };
            assert_eq!(observed_edits, expected_edits, "{arm:?} / {input}");
            assert_eq!(policy_step(actor), expected_policy, "{arm:?} / {input}");
            assert_eq!(policy_mutations, expected_policy_attempts, "{arm:?} / {input}");
            assert_eq!(service_count, expected_services, "{arm:?} / {input}");
            assert_eq!(marker_count, expected_markers, "{arm:?} / {input}");
            assert_eq!(wrong_key_count, expected_wrong, "{arm:?} / {input}");
            assert_eq!(source_edge(actor), expected_edge, "{arm:?} / {input}");
            assert_eq!(actor.pc, expected_pc, "{arm:?} / {input}");
            assert_eq!(suppressions, expected_suppressed, "{arm:?} / {input}");
            assert_eq!(
                world.events().iter().filter(|e| e.origin == Origin::External).count(),
                match arm {
                    Arm::Sham7 => 2,
                    Arm::Lesion10 | Arm::Lesion10PatchNextDisabled | Arm::Lesion11 => 3,
                    Arm::Lesion10PolicyLocked | Arm::Lesion11PolicyLocked => 4,
                }
            );
        }
    }
}
