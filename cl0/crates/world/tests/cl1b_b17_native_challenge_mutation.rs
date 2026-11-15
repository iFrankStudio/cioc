//! CL-1B B17: full Rust-VM differential of the pre-registered 4,495
//! one-edge mutant worlds, with actual PC16 branch decisions rather than
//! trusting a readback count or a separate success marker.
//! Observer-selected mutations are NOT autonomous self-evolution.
//! Native CI must actually run before any Rust result can be accepted.
#[path = "cl1b_b16_functional_challenge.rs"]
mod canonical_b16;

use cl0_world::{EventKind, Intervention, Op, Origin, Process, World};

fn next_edge(op: &Op) -> Option<usize> {
    match op {
        Op::Const { next, .. }
        | Op::Read { next, .. }
        | Op::Write { next, .. }
        | Op::Delete { next, .. }
        | Op::Sub { next, .. }
        | Op::ReadNext { next, .. }
        | Op::PatchNext { next, .. } => Some(*next),
        _ => None,
    }
}

#[derive(Debug, Default)]
struct Trial {
    attempts: usize,
    trap: bool,
    halted: bool,
    checker_reached: usize,
    false_positive_decisions: usize,
    false_negative_decisions: usize,
    branch_without_complete_read: usize,
    bypassed_checker_reads: usize,
    actual_challenge_writes: usize,
    ordinary_outputs: usize,
}

fn execute(input: i64, mutation: Option<(usize, usize)>, founder: &[Op]) -> Trial {
    let mut w = World::with_strict_capacity(16, 64, 128)
        .with_native_executable_edit_trace();
    w.load(Process::new(1, founder.to_vec(), 240)).unwrap();
    w.intervene(Intervention::SetCell { key: 7, value: input })
        .unwrap();
    if let Some((node, next)) = mutation {
        w.intervene(Intervention::PatchEdge { process: 1, node, next })
            .unwrap();
    }

    let mut out = Trial::default();
    let mut challenge_in_epoch = 0_usize;
    // PC13 read of output11 is normally followed by PC14, PC15, PC16.
    // The decision counts do NOT depend on this weaker provenance signal.
    let mut pending_stage = 0_usize;
    let mut had_bypassed_read = false;
    let mut had_branch_without_read = false;

    for _ in 0..120 {
        let pc_before = w.process(1).unwrap().pc;
        let mut consumed_read = false;
        if pending_stage != 0 {
            if pc_before == pending_stage {
                if pc_before == 16 {
                    consumed_read = true;
                }
                pending_stage = if pc_before == 16 {
                    0
                } else {
                    pending_stage + 1
                };
            } else {
                had_bypassed_read = true;
                pending_stage = 0;
            }
        }
        if pc_before == 0 {
            challenge_in_epoch = 0;
        }

        if !w.step() {
            // Halt/trap may have occurred at the previous charged attempt.
            break;
        }
        out.attempts += 1;
        let event = w.events().last().expect("charged step must emit native event");
        assert_eq!(event.origin, Origin::Internal(1));
        assert_eq!(event.pc, pc_before);
        match &event.kind {
            EventKind::CellWritten { key: 11, value: 17 } if pc_before == 11 => {
                out.actual_challenge_writes += 1;
                challenge_in_epoch += 1;
            }
            EventKind::CellWritten { key: 11, value } if pc_before == 31 && *value == input => {
                out.ordinary_outputs += 1;
            }
            EventKind::CellRead { .. } if pc_before == 13 => {
                pending_stage = 14;
            }
            EventKind::Trapped(_) => {
                out.trap = true;
            }
            EventKind::Halted => {
                out.halted = true;
            }
            _ => {}
        }

        if pc_before == 16 && matches!(event.kind, EventKind::Executed) {
            out.checker_reached += 1;
            if !consumed_read {
                had_branch_without_read = true;
            }
            match w.process(1).unwrap().pc {
                24 => {
                    if challenge_in_epoch == 0 {
                        out.false_positive_decisions += 1;
                    }
                }
                17 => {
                    if challenge_in_epoch > 0 {
                        out.false_negative_decisions += 1;
                    }
                }
                other => panic!("unchanged Branch PC16 has unexpected target {other}"),
            }
        }
        if out.trap || out.halted {
            break;
        }
    }

    out.branch_without_complete_read = usize::from(had_branch_without_read);
    out.bypassed_checker_reads = usize::from(had_bypassed_read);
    assert_eq!(w.process(1).unwrap().credits, 240 - out.attempts as u64);
    assert_eq!(w.total_compute_inventory(), Some(240 - out.attempts as u64));
    assert!(w.process(2).is_none());
    assert!(w.occupied_units() <= 128);
    assert_eq!(
        w.events().iter().filter(|e| e.origin == Origin::External).count(),
        if mutation.is_some() { 3 } else { 2 }
    );
    out
}

#[derive(Default)]
struct Totals {
    worlds: usize,
    fp_worlds: usize,
    fn_worlds: usize,
    reached_worlds: usize,
    traps: usize,
    halted_or_trapped: usize,
    branch_without_read: usize,
    bypassed_read: usize,
    fp_events: usize,
    fn_events: usize,
}

fn collect(total: &mut Totals, trial: &Trial) {
    total.worlds += 1;
    total.fp_worlds += usize::from(trial.false_positive_decisions > 0);
    total.fn_worlds += usize::from(trial.false_negative_decisions > 0);
    total.reached_worlds += usize::from(trial.checker_reached > 0);
    total.traps += usize::from(trial.trap);
    total.halted_or_trapped += usize::from(trial.trap || trial.halted);
    total.branch_without_read += usize::from(trial.branch_without_complete_read > 0);
    total.bypassed_read += usize::from(trial.bypassed_checker_reads > 0);
    total.fp_events += trial.false_positive_decisions;
    total.fn_events += trial.false_negative_decisions;
}

#[test]
fn b17_all_4495_native_mutants_test_actual_challenge_payload_and_branch_integrity() {
    let founder = canonical_b16::founder();
    assert_eq!(founder.len(), 32);
    let edges: Vec<(usize, usize)> = founder
        .iter()
        .enumerate()
        .filter_map(|(node, op)| next_edge(op).map(|next| (node, next)))
        .collect();
    assert_eq!(edges.len(), 29);
    let mut total = Totals::default();
    let expected_traps = [41, 32, 24, 16, 41];

    for (i, input) in [-13, 0, 7, 23, 101].into_iter().enumerate() {
        let baseline = execute(input, None, &founder);
        assert_eq!(baseline.attempts, 120);
        assert!(!baseline.trap && !baseline.halted);
        assert!(baseline.actual_challenge_writes > 0);
        assert!(baseline.checker_reached > 0);
        assert_eq!((baseline.false_positive_decisions, baseline.false_negative_decisions), (0, 0));

        let mut local = Totals::default();
        for &(node, original) in &edges {
            for next in 0..32 {
                if next == original {
                    continue;
                }
                let trial = execute(input, Some((node, next)), &founder);
                collect(&mut local, &trial);
                if node == 10 && next == 13 {
                    assert_eq!(trial.actual_challenge_writes, 0);
                    assert!(trial.false_positive_decisions > 0);
                    assert!(trial.ordinary_outputs > 0);
                }
            }
        }
        assert_eq!(local.worlds, 899);
        assert_eq!(local.fp_worlds, 12);
        assert_eq!(local.fn_worlds, 78);
        assert_eq!(local.reached_worlds, 525);
        assert_eq!(local.traps, expected_traps[i]);
        assert_eq!(local.branch_without_read, 67);
        assert_eq!(local.bypassed_read, 93);
        assert_eq!(local.fp_events, 46);
        assert_eq!(local.fn_events, 335);
        total.worlds += local.worlds;
        total.fp_worlds += local.fp_worlds;
        total.fn_worlds += local.fn_worlds;
        total.reached_worlds += local.reached_worlds;
        total.traps += local.traps;
        total.halted_or_trapped += local.halted_or_trapped;
        total.branch_without_read += local.branch_without_read;
        total.bypassed_read += local.bypassed_read;
        total.fp_events += local.fp_events;
        total.fn_events += local.fn_events;
    }
    assert_eq!(total.worlds, 4_495);
    assert_eq!(total.fp_worlds, 60);
    assert_eq!(total.fn_worlds, 390);
    assert_eq!(total.reached_worlds, 2_625);
    assert_eq!(total.traps, 154);
    assert_eq!(total.halted_or_trapped, 394);
    assert_eq!(total.branch_without_read, 335);
    assert_eq!(total.bypassed_read, 465);
    assert_eq!(total.fp_events, 230);
    assert_eq!(total.fn_events, 1_675);
}
