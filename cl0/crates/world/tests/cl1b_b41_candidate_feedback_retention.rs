//! CL-1B B41: one native CIL individual tests two externally supplied
//! executable-edge candidates and retains an edge only after paid, real
//! task-output feedback. Candidate generation and objective are NOT endogenous.
use cl0_world::{EventKind, Intervention, Op, Origin, Process, World};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Arm { Feedback, AssumePass, ForceFallback, StaticCorrect }

fn founder(arm: Arm) -> Vec<Op> {
    if arm == Arm::StaticCorrect {
        return vec![
            Op::Const { dst: 2, value: 7, next: 1 },
            Op::Read { dst: 0, address: 2, next: 2 },
            Op::Const { dst: 2, value: 11, next: 3 },
            Op::Write { address: 2, value: 0, next: 0 },
        ];
    }
    // B41 source-parser requires this array to stay literal and in PC order.
    let mut code = vec![
        Op::Const { dst: 3, value: 0, next: 1 },             // 0: bad candidate's value
        Op::Const { dst: 2, value: 20, next: 2 },            // 1: candidate A address
        Op::Read { dst: 1, address: 2, next: 3 },            // 2: untrusted candidate A
        Op::Const { dst: 2, value: 8, next: 4 },             // 3: service router PC
        Op::PatchNext { node: 2, target: 1, next: 5 },       // 4: install A (paid)
        Op::Const { dst: 2, value: 7, next: 6 },             // 5: task input cell
        Op::Read { dst: 0, address: 2, next: 7 },            // 6: current task input
        Op::Const { dst: 2, value: 11, next: 8 },            // 7: task output cell
        Op::Const { dst: 2, value: 11, next: 14 },           // 8: mutable service router
        Op::Read { dst: 1, address: 2, next: 10 },           // 9: actual task output
        Op::Sub { dst: 1, lhs: 0, rhs: 1, next: 11 },        // 10: task discrepancy
        Op::Branch { test: 1, nonzero: 20, zero: 12 },       // 11: evidence decision
        Op::Const { dst: 2, value: 8, next: 13 },            // 12: on verified success
        Op::ReadNext { dst: 1, node: 2, next: 24 },          // 13: recover selected edge
        Op::Write { address: 2, value: 3, next: 9 },         // 14: candidate constant 0
        Op::Write { address: 2, value: 0, next: 9 },         // 15: candidate live copy
        Op::Const { dst: 2, value: 21, next: 17 },           // 16: alternate candidate
        Op::Read { dst: 1, address: 2, next: 18 },           // 17: read alternate address
        Op::Const { dst: 2, value: 8, next: 19 },            // 18: router PC
        Op::PatchNext { node: 2, target: 1, next: 26 },      // 19: install B (paid)
        Op::Const { dst: 2, value: 22, next: 21 },           // 20: fallback flag cell
        Op::Read { dst: 1, address: 2, next: 22 },           // 21: check if B tested
        Op::Branch { test: 1, nonzero: 23, zero: 16 },       // 22: stop if both failed
        Op::Halt,                                           // 23: no validated source
        Op::Const { dst: 2, value: 5, next: 25 },            // 24: next for deployed Write
        Op::PatchNext { node: 1, target: 2, next: 5 },       // 25: retain selected Write
        Op::Const { dst: 1, value: 1, next: 27 },            // 26: mark second attempt
        Op::Const { dst: 2, value: 22, next: 28 },           // 27: marker address
        Op::Write { address: 2, value: 1, next: 5 },         // 28: actor-paid fallback flag
    ];
    match arm {
        Arm::AssumePass => code[10] = Op::Sub { dst: 1, lhs: 0, rhs: 0, next: 11 },
        Arm::ForceFallback => code[10] = Op::Const { dst: 1, value: 1, next: 11 },
        Arm::Feedback | Arm::StaticCorrect => {}
    }
    code
}

fn edge(w: &World, arm: Arm) -> usize {
    let node = if arm == Arm::StaticCorrect { 2 } else { 8 };
    match &w.process(1).unwrap().code.nodes[node] {
        Op::Const { next, .. } => *next,
        _ => panic!("B41 expected mutable routing Const"),
    }
}

#[derive(Debug)]
struct Outcome {
    arm: Arm, first: i64, second: i64, train: i64, holdout: i64,
    train_correct: usize, train_wrong: usize,
    holdout_correct: usize, holdout_wrong: usize,
    charged: usize, stopped: bool, final_edge: usize, final_write_next: (usize, usize),
    true_switches: usize, true_retentions: usize, fallback_writes: usize,
}

fn run(arm: Arm, first: i64, second: i64, train: i64, holdout: i64) -> Outcome {
    let mut w = World::with_strict_capacity(16, 64, 128).with_native_executable_edit_trace();
    w.load(Process::new(1, founder(arm), 96)).unwrap();
    for (key, value) in [(20, first), (21, second), (7, train)] {
        w.intervene(Intervention::SetCell { key, value }).unwrap();
    }
    let mut paid = 0;
    let mut correct = [0_usize; 2];
    let mut wrong = [0_usize; 2];
    for (phase, input) in [train, holdout].into_iter().enumerate() {
        if phase == 1 {
            w.intervene(Intervention::SetCell { key: 7, value: input }).unwrap();
        }
        let start = w.events().len();
        let before = w.total_compute_inventory().unwrap();
        let steps = w.run(48);
        assert_eq!(w.total_compute_inventory(), Some(before - steps as u64));
        paid += steps;
        for event in &w.events()[start..] {
            if let (Origin::Internal(1), EventKind::CellWritten { key: 11, value }) =
                (&event.origin, &event.kind)
            {
                if *value == input { correct[phase] += 1; }
                else { wrong[phase] += 1; }
            }
        }
    }
    assert_eq!(w.total_compute_inventory(), Some(96 - paid as u64));
    assert_eq!(w.available_compute_pool(), 0);
    assert_eq!(w.cell(7), Some(holdout));
    assert!(w.process(2).is_none());
    let edits = w.events().iter().filter_map(|e| match (&e.origin, &e.kind) {
        (Origin::Internal(1), EventKind::ExecutableEdgePatched { node, before, after }) =>
            Some((*node, *before, *after)),
        _ => None,
    }).collect::<Vec<_>>();
    let true_switches = edits.iter().filter(|(node, _, _)| *node == 8).count();
    let true_retentions = edits.iter().filter(|(node, before, after)|
        (*node == 14 || *node == 15) && *before == 9 && *after == 5).count();
    let fallback_writes = w.events().iter().filter(|e| matches!(
        (&e.origin, &e.kind),
        (Origin::Internal(1), EventKind::CellWritten { key: 22, value: 1 })
    )).count();
    let write_next = |node| match &w.process(1).unwrap().code.nodes[node] {
        Op::Write { next, .. } => *next,
        _ => panic!("B41 required candidate executable Write is absent"),
    };
    let outcome = Outcome {
        arm, first, second, train, holdout,
        train_correct: correct[0], train_wrong: wrong[0],
        holdout_correct: correct[1], holdout_wrong: wrong[1],
        charged: paid, stopped: w.process(1).unwrap().stopped,
        final_edge: edge(&w, arm),
        final_write_next: if arm == Arm::StaticCorrect { (0, 0) } else { (write_next(14), write_next(15)) },
        true_switches, true_retentions, fallback_writes,
    };
    eprintln!("B41_NATIVE {:?}", outcome);
    outcome
}

#[test]
fn b41_paid_native_evidence_selects_external_candidates_but_zero_training_overfits() {
    let mut n = 0;
    for (first, second) in [(14_i64, 15_i64), (15, 14)] {
        for train in [0_i64, 17, 23] {
            for holdout in [-9_i64, 17] {
                let feedback = run(Arm::Feedback, first, second, train, holdout);
                let assumed = run(Arm::AssumePass, first, second, train, holdout);
                let forced = run(Arm::ForceFallback, first, second, train, holdout);
                let compact = run(Arm::StaticCorrect, first, second, train, holdout);
                n += 4;
                for outcome in [&feedback, &assumed, &forced, &compact] {
                    assert_eq!(outcome.first, first);
                    assert_eq!(outcome.second, second);
                    assert_eq!(outcome.train, train);
                    assert_eq!(outcome.holdout, holdout);
                    assert!(outcome.charged <= 96);
                }
                assert_eq!(compact.charged, 96);
                assert_eq!(compact.train_wrong + compact.holdout_wrong, 0);
                assert_eq!(compact.train_correct + compact.holdout_correct, 24);
                assert_eq!(compact.true_switches + compact.true_retentions + compact.fallback_writes, 0);
                assert_eq!(assumed.fallback_writes, 0);
                assert_eq!(feedback.fallback_writes, usize::from(first == 14 && train != 0));
                assert_eq!(feedback.final_edge,
                    if train == 0 { first as usize } else { 15 },
                    "B41 true task evidence should reject wrong0 only on informative training input");
                assert_eq!(feedback.true_retentions, 1,
                    "one native paid edit must retain the actor-tested candidate");
                assert!(!feedback.stopped);
                if train == 0 && first == 14 && holdout != 0 {
                    assert_eq!(feedback.holdout_correct, 0,
                        "a zero-only observation is not a general task-function witness");
                    assert!(feedback.holdout_wrong > 0);
                    assert_eq!(feedback.final_write_next, (5, 9));
                }
                if train != 0 {
                    assert!(feedback.holdout_correct > 0);
                    assert_eq!(feedback.final_write_next, (9, 5));
                }
                assert!(compact.train_correct + compact.holdout_correct >=
                    feedback.train_correct + feedback.holdout_correct,
                    "always compare adaptive overhead against the cheap immutable source");
                assert!(forced.fallback_writes >= 1);
            }
        }
    }
    assert_eq!(n, 48);
}
