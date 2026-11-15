//! CL-1B B13 native/observer differential for a frozen, exhaustive mutation set.
//! This is a POST-HOC Rust conformance check of an observer-side robustness scan,
//! not native evolution, native candidate generation, or a prospectively successful
//! CL-1B scientific gate. B13 preregistration: cl1b/studies/B13-exhaustive-marker-mutation-screen.md
#[path = "cl1b_b10_freshness.rs"]
mod canonical_b10;

use cl0_world::{EventKind, Intervention, Op, Origin, Process, World};

fn current_next(op: &Op) -> Option<usize> {
    match op {
        Op::Const { next, .. }
        | Op::Read { next, .. }
        | Op::Write { next, .. }
        | Op::Delete { next, .. }
        | Op::ReadNext { next, .. }
        | Op::Sub { next, .. }
        | Op::PatchNext { next, .. } => Some(*next),
        _ => None,
    }
}

#[derive(Default)]
struct Counts {
    worlds: usize,
    false_positive_worlds: usize,
    false_negative_worlds: usize,
    trapped_worlds: usize,
    false_positive_decisions: usize,
    false_negative_decisions: usize,
    marker_without_service: usize,
    service_without_marker: usize,
    actual_decision_fp_worlds: usize,
    actual_decision_fn_worlds: usize,
    actual_decision_fp_events: usize,
    actual_decision_fn_events: usize,
    skipped_marker_read_worlds: usize,
}

#[test]
fn b13_all_single_next_edge_mutants_match_independent_proxy_failure_classification() {
    let founder = canonical_b10::founder();
    assert_eq!(founder.len(), 20);
    let patchable: Vec<(usize, usize)> = founder
        .iter()
        .enumerate()
        .filter_map(|(index, op)| current_next(op).map(|next| (index, next)))
        .collect();
    assert_eq!(patchable.len(), 19);

    let expected_per_input = [
        (-13, 15_usize, 8_usize, 74_usize),
        (0, 6, 20, 66),
        (7, 15, 8, 52),
        (23, 15, 8, 70),
        (101, 15, 8, 74),
    ];
    let mut cumulative = Counts::default();

    for (input, expected_fp, expected_fn, expected_trapped) in expected_per_input {
        let mut local = Counts::default();
        let mut baseline = World::with_strict_capacity(16, 32, 64)
            .with_native_executable_edit_trace();
        baseline
            .load(Process::new(1, founder.clone(), 200))
            .unwrap();
        baseline
            .intervene(Intervention::SetCell { key: 7, value: input })
            .unwrap();
        assert_eq!(baseline.run(80), 80);
        assert!(!baseline.process(1).unwrap().stopped);
        assert_eq!(
            baseline
                .events()
                .iter()
                .filter(|event| matches!(
                    (&event.origin, &event.kind),
                    (Origin::Internal(1), EventKind::CellWritten { key: 11, value })
                        if *value == input
                ))
                .count(),
            5,
            "the frozen intact B10 founder must perform five valid services"
        );

        for &(node, original) in &patchable {
            for target in 0..founder.len() {
                if target == original {
                    continue;
                }
                let mut world = World::with_strict_capacity(16, 32, 64)
                    .with_native_executable_edit_trace();
                world
                    .load(Process::new(1, founder.clone(), 200))
                    .unwrap();
                world
                    .intervene(Intervention::SetCell { key: 7, value: input })
                    .unwrap();
                world
                    .intervene(Intervention::PatchEdge {
                        process: 1,
                        node,
                        next: target,
                    })
                    .unwrap();
                let attempts = world.run(80);
                assert!(attempts <= 80);
                assert_eq!(world.process(1).unwrap().credits, 200 - attempts as u64);
                assert_eq!(
                    world.total_compute_inventory(),
                    Some(200 - attempts as u64)
                );
                assert!(world.process(2).is_none());
                assert!(world.occupied_units() <= 64);

                let mut valid_since_reset = 0_usize;
                let mut valid_services = 0_usize;
                let mut genuine_markers = 0_usize;
                let mut false_positives = 0_usize;
                let mut false_negatives = 0_usize;
                let mut trapped = false;
                // Post-hoc B13b robustness audit: a marker READ counts as an
                // actual decision only if PC12 Branch runs immediately after it.
                // Preserve the original B13 readback counts separately.
                let mut pending_marker: Option<(i64, usize)> = None;
                let mut strict_fp = 0_usize;
                let mut strict_fn = 0_usize;
                let mut bypassed_marker_reads = 0_usize;

                for event in world.events() {
                    if let Some((observed, recent_service_count)) = pending_marker.take() {
                        if event.origin == Origin::Internal(1)
                            && event.pc == 12
                            && matches!(event.kind, EventKind::Executed)
                        {
                            if observed != 0 && recent_service_count == 0 {
                                strict_fp += 1;
                            } else if observed == 0 && recent_service_count > 0 {
                                strict_fn += 1;
                            }
                        } else {
                            bypassed_marker_reads += 1;
                        }
                    }
                    if event.origin != Origin::Internal(1) {
                        continue;
                    }
                    if event.pc == 2 {
                        // Executing PC2 is the CIL-authored Delete(key13).
                        valid_since_reset = 0;
                    }
                    match &event.kind {
                        EventKind::CellWritten { key: 11, value }
                            if *value == input =>
                        {
                            valid_services += 1;
                            valid_since_reset += 1;
                        }
                        EventKind::CellWritten { key: 13, value: 1 } => {
                            genuine_markers += 1;
                        }
                        EventKind::CellRead { key: 13, value } if event.pc == 11 => {
                            pending_marker = Some((*value, valid_since_reset));
                            if *value != 0 && valid_since_reset == 0 {
                                false_positives += 1;
                            } else if *value == 0 && valid_since_reset != 0 {
                                false_negatives += 1;
                            }
                        }
                        EventKind::Trapped(_) => {
                            trapped = true;
                        }
                        _ => {}
                    }
                }

                local.actual_decision_fp_worlds += usize::from(strict_fp > 0);
                local.actual_decision_fn_worlds += usize::from(strict_fn > 0);
                local.actual_decision_fp_events += strict_fp;
                local.actual_decision_fn_events += strict_fn;
                local.skipped_marker_read_worlds += usize::from(bypassed_marker_reads > 0);
                local.worlds += 1;
                local.false_positive_worlds += usize::from(false_positives > 0);
                local.false_negative_worlds += usize::from(false_negatives > 0);
                local.trapped_worlds += usize::from(trapped);
                local.false_positive_decisions += false_positives;
                local.false_negative_decisions += false_negatives;
                local.marker_without_service +=
                    usize::from(genuine_markers > 0 && valid_services == 0);
                local.service_without_marker +=
                    usize::from(valid_services > 0 && genuine_markers == 0);
            }
        }
        assert_eq!(local.worlds, 361, "complete finite mutation denominator");
        assert_eq!(
            (local.actual_decision_fp_worlds, local.actual_decision_fn_worlds),
            (expected_fp, expected_fn),
            "actual immediately-consumed branch decisions differ from B13 proxy at input={input}"
        );
        assert_eq!(
            local.skipped_marker_read_worlds, 19,
            "B13b expects 19 bypassed-marker-read mutants per input"
        );
        assert_eq!(
            (local.false_positive_worlds, local.false_negative_worlds, local.trapped_worlds),
            (expected_fp, expected_fn, expected_trapped),
            "native Rust differed from independent observer-side scan for input={input}"
        );
        cumulative.actual_decision_fp_worlds += local.actual_decision_fp_worlds;
        cumulative.actual_decision_fn_worlds += local.actual_decision_fn_worlds;
        cumulative.actual_decision_fp_events += local.actual_decision_fp_events;
        cumulative.actual_decision_fn_events += local.actual_decision_fn_events;
        cumulative.skipped_marker_read_worlds += local.skipped_marker_read_worlds;
        cumulative.worlds += local.worlds;
        cumulative.false_positive_worlds += local.false_positive_worlds;
        cumulative.false_negative_worlds += local.false_negative_worlds;
        cumulative.trapped_worlds += local.trapped_worlds;
        cumulative.false_positive_decisions += local.false_positive_decisions;
        cumulative.false_negative_decisions += local.false_negative_decisions;
        cumulative.marker_without_service += local.marker_without_service;
        cumulative.service_without_marker += local.service_without_marker;
    }

    assert_eq!(cumulative.worlds, 1_805);
    assert_eq!(cumulative.false_positive_worlds, 66);
    assert_eq!(cumulative.false_negative_worlds, 52);
    assert_eq!(cumulative.trapped_worlds, 336);
    assert_eq!(cumulative.false_positive_decisions, 381);
    assert_eq!(cumulative.false_negative_decisions, 82);
    assert_eq!(cumulative.marker_without_service, 218);
    assert_eq!(cumulative.service_without_marker, 52);
    assert_eq!(cumulative.actual_decision_fp_worlds, 66);
    assert_eq!(cumulative.actual_decision_fn_worlds, 52);
    assert_eq!(cumulative.actual_decision_fp_events, 381);
    assert_eq!(cumulative.actual_decision_fn_events, 82);
    assert_eq!(cumulative.skipped_marker_read_worlds, 95);
}
