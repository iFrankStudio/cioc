//! CL-1B B43: falsify B42's self-assessment using an externally forged
//! feedback-cell value between its own true service Write and paid Read.
//! B42 founder CIL is imported unchanged. No new VM semantics or life oracle.
//! Host injections are explicit adversarial interventions, NOT native actions.
use cl0_world::{EventKind, Intervention, Origin, Process, World};
#[path = "cl1b_b42_self_assembled_probe.rs"]
mod b42_source;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Arm {
    Genuine,
    ForgedBeforeRead,
    SameValueBeforeRead,
    ForgedAfterRead,
    StaticCorrect,
}

#[derive(Debug)]
struct Observation {
    arm: Arm,
    train: i64,
    holdout: i64,
    probe: Option<i64>,
    first_native_output: Option<i64>,
    first_feedback_read: Option<i64>,
    first_branch_pc: Option<usize>,
    external_spoofs: usize,
    diagnostic_writes: usize,
    train_correct: usize,
    train_wrong: usize,
    holdout_correct: usize,
    holdout_wrong: usize,
    paid: usize,
    final_router: usize,
    source_nodes: usize,
    restored: bool,
    stopped: bool,
    wrong_accept: bool,
    true_native_clones: usize,
}

fn run(arm: Arm, train: i64, holdout: i64) -> Observation {
    let mut w = World::with_strict_capacity(16, 80, 192)
        .with_native_executable_edit_trace();
    let static_source = arm == Arm::StaticCorrect;
    let b42_arm = if static_source {
        b42_source::Arm::StaticCorrect
    } else {
        b42_source::Arm::ProbePlus
    };
    let source = b42_source::founder(b42_arm);
    assert_eq!(source.len(), if static_source { 4 } else { 38 });
    w.load(Process::new(1, source, 128)).unwrap();
    w.intervene(Intervention::SetCell { key: 7, value: train }).unwrap();
    let mut first_native_output = None;
    let mut first_feedback_read = None;
    let mut first_branch_pc = None;
    let mut probe = None;
    let mut external_spoofs = 0_usize;
    let mut diagnostic_writes = 0_usize;
    let mut actual_service = [[0_usize; 2]; 2];
    let mut restored = false;
    let mut paid = 0_usize;
    for (phase, expected) in [train, holdout].into_iter().enumerate() {
        if phase == 1 {
            w.intervene(Intervention::SetCell { key: 7, value: expected }).unwrap();
        }
        for _ in 0..64 {
            if w.process(1).unwrap().stopped { break; }
            let inventory_before = w.total_compute_inventory().unwrap();
            assert!(w.step(), "B43 one live actor must consume its instruction");
            paid += 1;
            assert_eq!(w.total_compute_inventory(), Some(inventory_before - 1));
            let e = w.events().last().unwrap().clone();
            assert_eq!(e.origin, Origin::Internal(1));
            assert!(!matches!(e.kind, EventKind::Trapped(_)), "B43 requires no native trap");
            if phase == 0 && !static_source {
                match &e.kind {
                    EventKind::CellWritten { key: 7, value } if e.pc == 7 =>
                        probe = Some(*value),
                    EventKind::CellWritten { key: 11, value } if e.pc == 10
                        && first_native_output.is_none() => {
                        first_native_output = Some(*value);
                        assert_eq!(*value, 0);
                        assert_eq!(w.process(1).unwrap().pc, 11);
                        if matches!(arm, Arm::ForgedBeforeRead | Arm::SameValueBeforeRead) {
                            // Simulate hostile modification of a public output cell.
                            // External intervention occurs AFTER actual native Write0,
                            // BEFORE CIL Read11. Only the attacker chooses this value.
                            let value = if arm == Arm::ForgedBeforeRead {
                                train + 1
                            } else { 0 };
                            w.intervene(Intervention::SetCell { key: 11, value }).unwrap();
                            external_spoofs += 1;
                            assert_eq!(w.events().last().unwrap().origin, Origin::External);
                        }
                    }
                    EventKind::CellRead { key: 11, value } if e.pc == 11
                        && first_feedback_read.is_none() => {
                        first_feedback_read = Some(*value);
                        if arm == Arm::ForgedAfterRead {
                            // Same data as forged-before but TOO LATE for the
                            // actual CIL register, testing timing causality.
                            w.intervene(Intervention::SetCell { key: 11, value: train + 1 })
                                .unwrap();
                            external_spoofs += 1;
                            assert_eq!(w.events().last().unwrap().origin, Origin::External);
                        }
                    }
                    EventKind::CellWritten { key: 7, value } if e.pc == 33 => {
                        assert_eq!(*value, train, "B43 must restore original task input");
                        restored = true;
                    }
                    _ => {}
                }
                if e.pc == 13 && first_branch_pc.is_none() {
                    first_branch_pc = Some(w.process(1).unwrap().pc);
                    assert!(matches!(first_branch_pc, Some(14) | Some(26)));
                }
            }
            if let EventKind::CellWritten { key: 11, value } = e.kind {
                if phase == 0 && !restored && !static_source {
                    diagnostic_writes += 1;
                } else {
                    let idx = usize::from(value != expected);
                    actual_service[phase][idx] += 1;
                }
            }
        }
    }
    assert_eq!(w.total_compute_inventory(), Some(128 - paid as u64));
    assert_eq!(w.available_compute_pool(), 0);
    assert!(w.process(2).is_none());
    let code = &w.process(1).unwrap().code.nodes;
    let final_router = if static_source {
        3
    } else {
        match &code[9] {
            cl0_world::Op::Const { next, .. } => *next,
            _ => panic!("B43 service router must be B42's genuine node9"),
        }
    };
    let clones = usize::from(code.len() == 39);
    let wrong_accept = !static_source
        && first_branch_pc == Some(26)
        && first_native_output != probe;
    let result = Observation {
        arm, train, holdout, probe, first_native_output, first_feedback_read,
        first_branch_pc, external_spoofs, diagnostic_writes,
        train_correct: actual_service[0][0], train_wrong: actual_service[0][1],
        holdout_correct: actual_service[1][0], holdout_wrong: actual_service[1][1],
        paid, final_router, source_nodes: code.len(), restored,
        stopped: w.process(1).unwrap().stopped, wrong_accept,
        true_native_clones: clones,
    };
    eprintln!("B43_NATIVE {:?}", result);
    result
}

#[test]
fn b43_forged_output_before_native_read_causes_false_acceptance_and_failed_retention() {
    let mut worlds = 0_usize;
    let mut wrong_accepts = [0_usize; 5];
    for train in [-1_i64, 0, 1, 17] {
        for holdout in [-9_i64, 23] {
            let natural = run(Arm::Genuine, train, holdout);
            let forged = run(Arm::ForgedBeforeRead, train, holdout);
            let echo = run(Arm::SameValueBeforeRead, train, holdout);
            let late = run(Arm::ForgedAfterRead, train, holdout);
            let compact = run(Arm::StaticCorrect, train, holdout);
            worlds += 5;
            for (i, r) in [&natural, &forged, &echo, &late, &compact].into_iter().enumerate() {
                assert_eq!((r.train, r.holdout), (train, holdout));
                assert_eq!(r.paid, 128);
                assert!(!r.stopped);
                wrong_accepts[i] += usize::from(r.wrong_accept);
            }
            for r in [&natural, &forged, &echo, &late] {
                assert_eq!(r.probe, Some(train + 1));
                assert_eq!(r.first_native_output, Some(0));
                assert_eq!(r.first_branch_pc, Some(if r.first_feedback_read == r.probe { 26 } else { 14 }));
                assert!(r.restored);
                assert!(r.diagnostic_writes >= 1);
            }
            assert_eq!(natural.external_spoofs, 0);
            assert_eq!(forged.external_spoofs, 1);
            assert_eq!(echo.external_spoofs, 1);
            assert_eq!(late.external_spoofs, 1);
            assert_eq!(forged.first_feedback_read, Some(train + 1));
            for r in [&natural, &echo, &late] {
                assert_eq!(r.first_feedback_read, Some(0));
                assert_eq!(r.first_branch_pc, Some(if train == -1 { 26 } else { 14 }));
                assert_eq!(r.final_router, if train == -1 { 10 } else { 38 });
                assert_eq!(r.true_native_clones, usize::from(train != -1));
                assert_eq!(r.holdout_correct, natural.holdout_correct);
                assert_eq!(r.holdout_wrong, natural.holdout_wrong);
            }
            assert_eq!(forged.first_branch_pc, Some(26));
            assert_eq!(forged.final_router, 10);
            assert_eq!(forged.true_native_clones, 0);
            assert_eq!(forged.source_nodes, 38);
            assert_eq!(forged.holdout_correct, 0);
            assert!(forged.holdout_wrong > 0);
            if train != -1 {
                assert!(natural.holdout_correct > 0);
                assert_eq!(forged.wrong_accept, true,
                    "external data spoof creates a false confidence verdict in CIL");
            }
            assert_eq!(compact.holdout_correct, 16);
            assert_eq!(compact.holdout_wrong, 0);
            assert_eq!(compact.final_router, 3);
            assert_eq!(compact.external_spoofs, 0);
            assert!(!compact.wrong_accept);
            assert!(compact.train_correct + compact.holdout_correct >=
                natural.train_correct + natural.holdout_correct);
        }
    }
    assert_eq!(worlds, 40);
    // These are independent source-logic implications, not post-selected trials:
    // natural + matched echo + delayed same-value injections retain B42's
    // one genuine blind spot (train -1; two holdout worlds); early forgery
    // turns the remaining six worlds into additional false-acceptance cases.
    assert_eq!(wrong_accepts, [0, 6, 0, 0, 0]);
}
