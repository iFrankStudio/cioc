//! CL-1B B05: scripted reciprocal-monitor protection and misleading throughput.
//! Preregistered in cl1b/studies/B05-reciprocal-monitors-throughput-paradox.md.
//! Engineer-authored source and lesions: not an emergent-life or fitness claim.
use cl0_world::{EventKind, Intervention, Op, Origin, Process, World};

#[derive(Clone, Copy, Debug)]
enum Arm {
    Sham,
    AOnly,
    BOnly,
    Double,
}

fn founder() -> Vec<Op> {
    vec![
        Op::Const { dst: 0, value: 9, next: 1 },
        Op::ReadNext { dst: 1, node: 0, next: 2 },
        Op::Const { dst: 2, value: 12, next: 3 },
        Op::Write { address: 2, value: 1, next: 4 },
        Op::Const { dst: 0, value: 17, next: 5 },
        Op::ReadNext { dst: 1, node: 0, next: 6 },
        Op::Const { dst: 2, value: 13, next: 7 },
        Op::Write { address: 2, value: 1, next: 8 },
        Op::Const { dst: 0, value: 17, next: 9 },
        Op::ReadNext { dst: 1, node: 0, next: 10 },
        Op::Const { dst: 2, value: 13, next: 11 },
        Op::Read { dst: 3, address: 2, next: 12 },
        Op::Sub { dst: 1, lhs: 1, rhs: 3, next: 13 },
        Op::Branch { test: 1, nonzero: 14, zero: 15 },
        Op::PatchNext { node: 0, target: 3, next: 15 },
        Op::Const { dst: 2, value: 7, next: 16 },
        Op::Const { dst: 0, value: 9, next: 17 },
        Op::ReadNext { dst: 1, node: 0, next: 18 },
        Op::Const { dst: 2, value: 12, next: 19 },
        Op::Read { dst: 3, address: 2, next: 20 },
        Op::Sub { dst: 1, lhs: 1, rhs: 3, next: 21 },
        Op::Branch { test: 1, nonzero: 22, zero: 23 },
        Op::PatchNext { node: 0, target: 3, next: 23 },
        Op::Const { dst: 2, value: 7, next: 24 },
        Op::Const { dst: 3, value: 9, next: 25 },
        Op::Read { dst: 1, address: 2, next: 26 },
        Op::Write { address: 3, value: 1, next: 27 },
        Op::Const { dst: 0, value: 0, next: 8 },
    ]
}

fn code_edge(actor: &Process, node: usize) -> usize {
    match &actor.code.nodes[node] {
        Op::ReadNext { next, .. } => *next,
        _ => panic!("protected monitor must remain typed ReadNext"),
    }
}

#[test]
fn cl1b_b05_reciprocal_monitors_recover_single_lesions_not_double() {
    for input in [-13, 0, 7, 23, 101] {
        let mut base =
            World::with_strict_capacity(16, 32, 64).with_native_executable_edit_trace();
        base.load(Process::new(1, founder(), 300)).unwrap();
        base.intervene(Intervention::SetCell {
            key: 7,
            value: input,
        })
        .unwrap();
        assert_eq!(base.run(8), 8);
        assert_eq!(base.process(1).unwrap().pc, 8);
        assert_eq!(base.process(1).unwrap().credits, 292);
        assert_eq!(base.total_compute_inventory(), Some(292));
        let checkpoints: Vec<(usize, i64, i64)> = base.events().iter().filter_map(|event| {
            match (&event.origin, &event.kind) {
                (Origin::Internal(1), EventKind::CellWritten { key, value })
                    if *key == 12 || *key == 13 =>
                {
                    Some((event.pc, *key, *value))
                }
                _ => None,
            }
        }).collect();
        assert_eq!(checkpoints, vec![(3, 12, 10), (7, 13, 18)]);

        for arm in [Arm::Sham, Arm::AOnly, Arm::BOnly, Arm::Double] {
            let mut world = base.clone();
            if matches!(arm, Arm::AOnly | Arm::Double) {
                world.intervene(Intervention::PatchEdge {
                    process: 1,
                    node: 9,
                    next: 15,
                }).unwrap();
            }
            if matches!(arm, Arm::BOnly | Arm::Double) {
                world.intervene(Intervention::PatchEdge {
                    process: 1,
                    node: 17,
                    next: 23,
                }).unwrap();
            }

            let start = world.events().len();
            assert_eq!(world.run(100), 100, "all arms must get 100 opportunities");
            let after = &world.events()[start..];
            let actor = world.process(1).unwrap();
            assert!(!actor.stopped, "the same actor must remain runnable");
            assert_eq!(actor.id, 1);
            assert_eq!(actor.credits, 192);
            assert_eq!(world.total_compute_inventory(), Some(192));
            assert!(after.iter().all(|event| event.origin == Origin::Internal(1)));
            assert_eq!(
                world.events().iter().filter(|event| event.origin == Origin::External).count(),
                match arm {
                    Arm::Sham => 2,
                    Arm::AOnly | Arm::BOnly => 3,
                    Arm::Double => 4,
                }
            );
            assert_eq!(world.events().iter().filter(|event| matches!(
                event.kind, EventKind::GraphLaunched { .. } | EventKind::Trapped(_)
            )).count(), 0);

            let output: Vec<(usize, i64)> = after.iter().filter_map(|event| {
                match (&event.origin, &event.kind) {
                    (Origin::Internal(1), EventKind::CellWritten { key: 9, value }) =>
                        Some((event.pc, *value)),
                    _ => None,
                }
            }).collect();
            assert!(!output.is_empty(), "all four arms remain superficially productive");
            assert!(output.iter().all(|(pc, value)| *pc == 26 && *value == input));

            let edits: Vec<(usize, usize, usize)> = after.iter().filter_map(|event| {
                match (&event.origin, &event.kind) {
                    (Origin::Internal(1), EventKind::ExecutableEdgePatched {
                        node, before, after
                    }) => Some((*node, *before, *after)),
                    _ => None,
                }
            }).collect();
            let (expected_edits, edge_a, edge_b) = match arm {
                Arm::Sham => (vec![], 10, 18),
                Arm::AOnly => (vec![(9, 15, 10)], 10, 18),
                Arm::BOnly => (vec![(17, 23, 18)], 10, 18),
                Arm::Double => (vec![], 15, 23),
            };
            assert_eq!(edits, expected_edits, "native recovery must be causal");
            assert_eq!(code_edge(actor, 9), edge_a);
            assert_eq!(code_edge(actor, 17), edge_b);
            assert!(
                world.events().iter().all(|event| {
                    !matches!(
                        event.kind,
                        EventKind::CellWritten { key: 12 | 13, .. }
                    ) || event.origin == Origin::Internal(1)
                }),
                "checkpoint cells must only be written by native actor"
            );
        }
    }
}


// B06: preserve the registered two-repairs prediction as falsified by independent
// control-flow simulation; verify the *observed* one-repair order masking in the
// actual VM instead. This is post hoc compatibility, NOT preregistered success.
// Reference: cl1b/studies/B06-poisoned-checkpoint-self-damage.md.
#[test]
fn cl1b_b06_poisoned_checkpoints_cause_self_damage_and_order_masks_double_fault() {
    for input in [-13, 0, 7, 23, 101] {
        let mut base =
            World::with_strict_capacity(16, 32, 64).with_native_executable_edit_trace();
        base.load(Process::new(1, founder(), 300)).unwrap();
        base.intervene(Intervention::SetCell { key: 7, value: input }).unwrap();
        assert_eq!(base.run(8), 8);
        assert_eq!(base.process(1).unwrap().pc, 8);
        assert_eq!(code_edge(base.process(1).unwrap(), 9), 10);
        assert_eq!(code_edge(base.process(1).unwrap(), 17), 18);
        let original_checkpoints: Vec<(usize, i64, i64)> = base.events().iter().filter_map(|event| {
            match (&event.origin, &event.kind) {
                (Origin::Internal(1), EventKind::CellWritten { key, value })
                    if *key == 12 || *key == 13 =>
                {
                    Some((event.pc, *key, *value))
                }
                _ => None,
            }
        }).collect();
        assert_eq!(original_checkpoints, vec![(3, 12, 10), (7, 13, 18)]);

        for condition in 0..5_usize {
            let mut world = if condition == 4 {
                base.clone().with_native_patchnext_disabled()
            } else {
                base.clone()
            };
            if condition == 1 || condition >= 3 {
                world.intervene(Intervention::SetCell { key: 13, value: 23 }).unwrap();
            }
            if condition == 2 || condition >= 3 {
                world.intervene(Intervention::SetCell { key: 12, value: 15 }).unwrap();
            }
            let before = world.events().len();
            assert_eq!(world.run(100), 100);
            let after = &world.events()[before..];
            let actor = world.process(1).unwrap();
            assert_eq!(actor.id, 1);
            assert!(!actor.stopped);
            assert_eq!(actor.credits, 192);
            assert_eq!(world.total_compute_inventory(), Some(192));
            assert_eq!(
                world.events().iter().filter(|event| event.origin == Origin::External).count(),
                match condition {
                    0 => 2,
                    1 | 2 => 3,
                    3 | 4 => 4,
                    _ => unreachable!(),
                }
            );
            assert!(after.iter().all(|event| event.origin == Origin::Internal(1)));
            assert_eq!(
                after.iter().filter(|event| matches!(
                    event.kind, EventKind::Trapped(_) | EventKind::GraphLaunched { .. }
                )).count(),
                0
            );
            // No external executable-code lesion is allowed in B06.
            assert!(!world.events().iter().any(|event| {
                event.origin == Origin::External
                    && matches!(
                        &event.kind,
                        EventKind::Intervened(description) if description.contains("PatchEdge")
                    )
            }));

            let outputs: Vec<(usize, i64)> = after.iter().filter_map(|event| {
                match (&event.origin, &event.kind) {
                    (Origin::Internal(1), EventKind::CellWritten { key: 9, value }) =>
                        Some((event.pc, *value)),
                    _ => None,
                }
            }).collect();
            let repairs: Vec<(usize, usize, usize)> = after.iter().filter_map(|event| {
                match (&event.origin, &event.kind) {
                    (Origin::Internal(1), EventKind::ExecutableEdgePatched {
                        node, before, after
                    }) => Some((*node, *before, *after)),
                    _ => None,
                }
            }).collect();
            let suppressed = after.iter().filter(|event| {
                event.origin == Origin::Internal(1)
                    && matches!(event.kind, EventKind::NativeCodeEditSuppressed { opcode_tag: 6 })
            }).count();
            let (count, edges, patch, halted_pc, suppressed_count) = match condition {
                0 => (5, (10, 18), vec![], 19, 0),
                1 => (7, (10, 23), vec![(17, 18, 23)], 9, 0),
                2 => (6, (15, 18), vec![(9, 10, 15)], 25, 0),
                // The two-patch preregistered expectation was refuted:
                // monitor A's first wrong edit disables B before it can act.
                3 => (7, (10, 23), vec![(17, 18, 23)], 9, 0),
                4 => (5, (10, 18), vec![], 8, 10),
                _ => unreachable!(),
            };
            assert_eq!(outputs, vec![(26, input); count], "condition={condition}");
            assert_eq!(repairs, patch, "condition={condition}");
            assert_eq!(suppressed, suppressed_count, "condition={condition}");
            assert_eq!(code_edge(actor, 9), edges.0, "condition={condition}");
            assert_eq!(code_edge(actor, 17), edges.1, "condition={condition}");
            assert_eq!(actor.pc, halted_pc, "condition={condition}");
        }
    }
}
