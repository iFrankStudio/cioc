//! CL-1B B08: two distinct histories converge to the same CIL-accessible state.
//! Preregistered: cl1b/studies/B08-indistinguishable-histories.md
//! This is a deterministic information-loss boundary, not a living agent.
use cl0_world::{EventKind, Intervention, Op, Origin, Process, World};

fn founder(original_edge: usize) -> Vec<Op> {
    vec![
        Op::Const { dst: 0, value: 8, next: 1 },
        Op::ReadNext { dst: 1, node: 0, next: 2 },
        Op::Const { dst: 2, value: 12, next: 3 },
        Op::Write { address: 2, value: 1, next: 4 },
        Op::Const { dst: 2, value: 13, next: 5 },
        Op::Write { address: 2, value: 1, next: 6 },
        Op::Const { dst: 2, value: 7, next: 7 },
        Op::Const { dst: 3, value: 9, next: 8 },
        Op::Read { dst: 1, address: 2, next: original_edge },
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

fn original_checkpoint_events(world: &World) -> Vec<(usize, i64, i64)> {
    world
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
        .collect()
}

fn construct_convergent_world(
    input: i64,
    original_edge: usize,
    erase_history: bool,
) -> World {
    let mut w = World::with_strict_capacity(16, 32, 64)
        .with_native_executable_edit_trace();
    w.load(Process::new(1, founder(original_edge), 250))
        .unwrap();
    w.intervene(Intervention::SetCell { key: 7, value: input })
        .unwrap();
    assert_eq!(w.run(8), 8);
    assert_eq!(w.process(1).unwrap().pc, 8);
    assert_eq!(
        original_checkpoint_events(&w),
        vec![(3, 12, original_edge as i64), (5, 13, original_edge as i64)]
    );

    // Same NUMBER and TYPE of host interventions in each arm.
    // In the retained-reference control, two SetCell operations are no-ops.
    w.intervene(Intervention::PatchEdge {
        process: 1,
        node: 8,
        next: 10,
    })
    .unwrap();
    let saved = if erase_history { 10 } else { original_edge as i64 };
    for key in [12, 13] {
        w.intervene(Intervention::SetCell { key, value: saved })
            .unwrap();
    }

    // Native PC8 Read overwrites r1 with the common service input; previously
    // different transient registers can no longer disclose the history.
    assert!(w.step());
    assert_eq!(w.process(1).unwrap().pc, 10);
    assert_eq!(w.process(1).unwrap().credits, 241);
    assert_eq!(w.total_compute_inventory(), Some(241));
    w
}

#[test]
fn cl1b_b08_erased_different_histories_have_indistinguishable_future_cil_actions() {
    for input in [-13, 0, 7, 23, 101] {
        let mut lost = construct_convergent_world(input, 9, true);
        let mut born_different = construct_convergent_world(input, 10, true);
        let mut retained = construct_convergent_world(input, 9, false);

        // History differs in the observer-only CIL-produced checkpoint events.
        assert_ne!(original_checkpoint_events(&lost), original_checkpoint_events(&born_different));
        assert_ne!(lost.events(), born_different.events());

        // Yet the entire currently executable process and shared-state
        // interface have converged. Their inaccessible observer history has not.
        assert_eq!(lost.process(1), born_different.process(1));
        assert_eq!(lost.process(1), retained.process(1)); // same running code; only checkpoint cells differ
        for key in [7, 9, 12, 13] {
            assert_eq!(lost.cell(key), born_different.cell(key));
        }
        assert_eq!(lost.cell(12), Some(10));
        assert_eq!(lost.cell(13), Some(10));
        assert_eq!(retained.cell(12), Some(9));
        assert_eq!(retained.cell(13), Some(9));
        assert_eq!(lost.remaining_credits(), born_different.remaining_credits());
        assert_eq!(lost.tick(), born_different.tick());

        let lost_at = lost.events().len();
        let born_at = born_different.events().len();
        let retained_at = retained.events().len();
        assert_eq!(lost.run(80), 80);
        assert_eq!(born_different.run(80), 80);
        assert_eq!(retained.run(80), 80);

        // Same present native state + deterministic physics -> same future.
        assert_eq!(
            &lost.events()[lost_at..],
            &born_different.events()[born_at..],
            "native CIL cannot access erased original-code provenance"
        );
        assert_eq!(lost.process(1), born_different.process(1));
        for key in [7, 9, 12, 13] {
            assert_eq!(lost.cell(key), born_different.cell(key));
        }

        let lost_outputs = lost.events()[lost_at..]
            .iter()
            .filter(|e| {
                e.origin == Origin::Internal(1)
                    && matches!(e.kind, EventKind::CellWritten { key: 9, .. })
            })
            .count();
        let born_outputs = born_different.events()[born_at..]
            .iter()
            .filter(|e| {
                e.origin == Origin::Internal(1)
                    && matches!(e.kind, EventKind::CellWritten { key: 9, .. })
            })
            .count();
        let kept_outputs = retained.events()[retained_at..]
            .iter()
            .filter(|e| {
                e.origin == Origin::Internal(1)
                    && matches!(e.kind, EventKind::CellWritten { key: 9, value } if value == input)
            })
            .count();
        assert_eq!(lost_outputs, 0);
        assert_eq!(born_outputs, 0);
        assert!(kept_outputs >= 1);

        let lost_edits = lost.events()[lost_at..]
            .iter()
            .filter(|e| matches!(e.kind, EventKind::ExecutableEdgePatched { .. }))
            .count();
        let born_edits = born_different.events()[born_at..]
            .iter()
            .filter(|e| matches!(e.kind, EventKind::ExecutableEdgePatched { .. }))
            .count();
        assert_eq!(lost_edits, 0);
        assert_eq!(born_edits, 0);
        assert!(retained.events()[retained_at..].iter().any(|e| {
            e.origin == Origin::Internal(1)
                && matches!(
                    e.kind,
                    EventKind::ExecutableEdgePatched {
                        node: 8,
                        before: 10,
                        after: 9
                    }
                )
        }));

        for world in [&lost, &born_different, &retained] {
            assert_eq!(world.process(1).unwrap().credits, 161);
            assert_eq!(world.total_compute_inventory(), Some(161));
            assert!(!world.process(1).unwrap().stopped);
            assert!(world.process(2).is_none());
            assert!(world.events().iter().all(|e| {
                !matches!(
                    e.kind,
                    EventKind::GraphLaunched { .. } | EventKind::Trapped(_)
                )
            }));
            assert_eq!(
                world.events().iter().filter(|e| e.origin == Origin::External).count(),
                5
            );
        }
    }
}
