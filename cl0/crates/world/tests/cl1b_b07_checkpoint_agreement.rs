//! CL-1B B07: two CIL-created checkpoint references, agree-before-patch policy.
//! Explicitly human-authored and preregistered in:
//! cl1b/studies/B07-two-checkpoint-agreement-tradeoff.md
//! A causal engineering control, NOT spontaneously emerged trust or digital life.
use cl0_world::{EventKind, Intervention, Op, Origin, Process, World};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Arm {
    Clean,
    CodeDamage,
    Poison12,
    Poison13,
    CorrelatedPoison,
    DamageAndPoison12,
    DamagePatchDisabled,
}

fn founder() -> Vec<Op> {
    vec![
        Op::Const { dst: 0, value: 8, next: 1 },
        Op::ReadNext { dst: 1, node: 0, next: 2 },
        Op::Const { dst: 2, value: 12, next: 3 },
        Op::Write { address: 2, value: 1, next: 4 },
        Op::Const { dst: 2, value: 13, next: 5 },
        Op::Write { address: 2, value: 1, next: 6 },
        Op::Const { dst: 2, value: 7, next: 7 },
        Op::Const { dst: 3, value: 9, next: 8 },
        Op::Read { dst: 1, address: 2, next: 9 },
        Op::Write { address: 3, value: 1, next: 10 },
        Op::ReadNext { dst: 1, node: 0, next: 11 },
        Op::Const { dst: 2, value: 12, next: 12 },
        Op::Read { dst: 2, address: 2, next: 13 },
        Op::Const { dst: 3, value: 13, next: 14 },
        Op::Read { dst: 3, address: 3, next: 15 },
        Op::Sub { dst: 2, lhs: 2, rhs: 3, next: 16 },
        Op::Branch { test: 2, nonzero: 22, zero: 17 },
        Op::Sub { dst: 2, lhs: 1, rhs: 3, next: 18 },
        Op::Branch { test: 2, nonzero: 19, zero: 22 },
        Op::PatchNext { node: 0, target: 3, next: 22 },
        Op::Halt,
        Op::Halt,
        Op::Const { dst: 2, value: 7, next: 23 },
        Op::Const { dst: 3, value: 9, next: 8 },
    ]
}

fn target_next(actor: &Process) -> usize {
    match &actor.code.nodes[8] {
        Op::Read { next, .. } => *next,
        _ => panic!("guarded node must remain Read"),
    }
}

#[test]
fn cl1b_b07_double_checkpoint_consensus_cannot_resolve_correlated_faults() {
    for input in [-13, 0, 7, 23, 101] {
        let mut prefork =
            World::with_strict_capacity(16, 32, 64).with_native_executable_edit_trace();
        prefork
            .load(Process::new(1, founder(), 250))
            .unwrap();
        prefork
            .intervene(Intervention::SetCell { key: 7, value: input })
            .unwrap();
        assert_eq!(prefork.run(8), 8);
        assert_eq!(prefork.process(1).unwrap().pc, 8);
        assert_eq!(prefork.process(1).unwrap().credits, 242);
        assert_eq!(prefork.total_compute_inventory(), Some(242));
        assert_eq!(target_next(prefork.process(1).unwrap()), 9);
        let checkpoints: Vec<(usize, i64, i64)> = prefork
            .events()
            .iter()
            .filter_map(|e| match (&e.origin, &e.kind) {
                (Origin::Internal(1), EventKind::CellWritten { key, value })
                    if *key == 12 || *key == 13 =>
                {
                    Some((e.pc, *key, *value))
                }
                _ => None,
            })
            .collect();
        assert_eq!(checkpoints, vec![(3, 12, 9), (5, 13, 9)]);

        for arm in [
            Arm::Clean,
            Arm::CodeDamage,
            Arm::Poison12,
            Arm::Poison13,
            Arm::CorrelatedPoison,
            Arm::DamageAndPoison12,
            Arm::DamagePatchDisabled,
        ] {
            let mut w = if arm == Arm::DamagePatchDisabled {
                prefork.clone().with_native_patchnext_disabled()
            } else {
                prefork.clone()
            };
            if matches!(
                arm,
                Arm::CodeDamage | Arm::DamageAndPoison12 | Arm::DamagePatchDisabled
            ) {
                w.intervene(Intervention::PatchEdge {
                    process: 1,
                    node: 8,
                    next: 10,
                })
                .unwrap();
            }
            if matches!(
                arm,
                Arm::Poison12 | Arm::CorrelatedPoison | Arm::DamageAndPoison12
            ) {
                w.intervene(Intervention::SetCell { key: 12, value: 10 })
                    .unwrap();
            }
            if matches!(arm, Arm::Poison13 | Arm::CorrelatedPoison) {
                w.intervene(Intervention::SetCell { key: 13, value: 10 })
                    .unwrap();
            }

            let start = w.events().len();
            assert_eq!(w.run(100), 100, "matched 100 instruction opportunities");
            let new_events = &w.events()[start..];
            let actor = w.process(1).unwrap();
            assert!(!actor.stopped, "all conditions remain runnable");
            assert_eq!(actor.id, 1);
            assert!(w.process(2).is_none(), "no second individual");
            assert_eq!(actor.credits, 142);
            assert_eq!(w.total_compute_inventory(), Some(142));
            assert!(new_events
                .iter()
                .all(|event| event.origin == Origin::Internal(1)));
            assert_eq!(
                new_events
                    .iter()
                    .filter(|event| matches!(
                        event.kind,
                        EventKind::Trapped(_) | EventKind::GraphLaunched { .. }
                    ))
                    .count(),
                0
            );

            let writes: Vec<(usize, i64)> = new_events
                .iter()
                .filter_map(|e| match (&e.origin, &e.kind) {
                    (Origin::Internal(1), EventKind::CellWritten { key: 9, value }) => {
                        Some((e.pc, *value))
                    }
                    _ => None,
                })
                .collect();
            let mutations: Vec<(usize, usize, usize)> = new_events
                .iter()
                .filter_map(|e| match (&e.origin, &e.kind) {
                    (
                        Origin::Internal(1),
                        EventKind::ExecutableEdgePatched {
                            node, before, after,
                        },
                    ) => Some((*node, *before, *after)),
                    _ => None,
                })
                .collect();
            let suppressed = new_events
                .iter()
                .filter(|e| {
                    e.origin == Origin::Internal(1)
                        && matches!(
                            e.kind,
                            EventKind::NativeCodeEditSuppressed { opcode_tag: 6 }
                        )
                })
                .count();

            // Exact outputs/positions are static path-audit compatibility endpoints.
            // Qualitative tradeoff was preregistered before independent simulation.
            let (service_count, final_edge, effective_edits, suppress_count, next_pc) =
                match arm {
                    Arm::Clean => (8, 9, vec![], 0, 17),
                    Arm::CodeDamage => (7, 9, vec![(8, 10, 9)], 0, 17),
                    Arm::Poison12 => (9, 9, vec![], 0, 9),
                    Arm::Poison13 => (9, 9, vec![], 0, 9),
                    Arm::CorrelatedPoison => (1, 10, vec![(8, 9, 10)], 0, 11),
                    Arm::DamageAndPoison12 => (0, 10, vec![], 0, 8),
                    Arm::DamagePatchDisabled => (0, 10, vec![], 7, 18),
                };
            assert_eq!(writes, vec![(9, input); service_count], "{arm:?}");
            assert_eq!(mutations, effective_edits, "{arm:?}");
            assert_eq!(suppressed, suppress_count, "{arm:?}");
            assert_eq!(target_next(actor), final_edge, "{arm:?}");
            assert_eq!(actor.pc, next_pc, "{arm:?}");
            assert_eq!(
                w.events()
                    .iter()
                    .filter(|e| e.origin == Origin::External)
                    .count(),
                2 + match arm {
                    Arm::Clean => 0,
                    Arm::CorrelatedPoison | Arm::DamageAndPoison12 => 2,
                    _ => 1,
                },
                "{arm:?}: provenance of researcher interventions"
            );
            assert!(
                w.events().iter().all(|e| {
                    !matches!(
                        e.kind,
                        EventKind::CellWritten { key: 12 | 13, .. }
                    ) || e.origin == Origin::Internal(1)
                }),
                "all checkpoint creation must have occurred inside CIL"
            );
        }
    }
}
