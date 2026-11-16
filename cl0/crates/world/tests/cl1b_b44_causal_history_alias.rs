//! CL-1B B44: two different, observer-verifiable task-output histories
//! converge to EXACTLY the same complete serialized native World state
//! except its private event history. The individual cannot read that history.
//! All branch edits and data forgeries are explicit external lab actions,
//! NOT autonomous actor actions or a new VM provenance/fitness opcode.
use cl0_world::{Event, EventKind, Intervention, Op, Origin, Process, World};
#[path = "cl1b_b42_self_assembled_probe.rs"]
mod b42_source;

fn step(w: &mut World) -> Event {
    let before = w.total_compute_inventory().unwrap();
    assert!(w.step(), "B44 continuing actor has finite paid CIL steps");
    assert_eq!(w.total_compute_inventory(), Some(before - 1));
    let e = w.events().last().unwrap().clone();
    assert_eq!(e.origin, Origin::Internal(1));
    assert!(!matches!(e.kind, EventKind::Trapped(_) | EventKind::Halted));
    e
}

fn external(w: &mut World, action: Intervention) {
    let inventory = w.total_compute_inventory();
    w.intervene(action).expect("B44 declared legal lab intervention");
    assert_eq!(w.total_compute_inventory(), inventory,
        "external lab history construction must not top-up compute");
    assert_eq!(w.events().last().unwrap().origin, Origin::External);
}

fn patch(w: &mut World, node: usize, next: usize) {
    external(w, Intervention::PatchEdge { process: 1, node, next });
}

// This is a stronger comparison than selected registers or shared task
// cells. It retains every SERIALIZED substrate field, including tick,
// compute, scheduler and code; removes ONLY the host-visible event history.
// B42 CIL has no instruction for reading World::events().
fn guest_world_state(w: &World) -> serde_json::Value {
    let mut value = serde_json::to_value(w).expect("serializable deterministic world");
    assert!(value.as_object_mut().unwrap().remove("events").is_some(),
        "B44 scientific separation requires a genuine private event log");
    value
}

#[derive(Debug)]
struct PairOutcome {
    train: i64, holdout: i64, challenge: i64,
    true_output: i64, forged_output: i64,
    correct_native_writer: usize, wrong_native_writer: usize,
    guest_state_equal: bool, event_histories_differ: bool,
    guest_continuation_equal: bool, observed_reads_equal: bool,
    first_read: i64, first_decision: usize,
    true_training_correct: usize, true_training_wrong: usize,
    false_training_correct: usize, false_training_wrong: usize,
    true_holdout_correct: usize, true_holdout_wrong: usize,
    false_holdout_correct: usize, false_holdout_wrong: usize,
    charged_each: usize, final_route: usize, code_nodes: usize,
    truth_label_differs: bool,
}

fn pair(train: i64, holdout: i64) -> PairOutcome {
    let mut ancestor = World::with_strict_capacity(16, 80, 192)
        .with_native_executable_edit_trace();
    ancestor.load(Process::new(
        1, b42_source::founder(b42_source::Arm::ProbePlus), 128
    )).unwrap();
    external(&mut ancestor, Intervention::SetCell { key: 7, value: train });
    for _ in 0..9 { step(&mut ancestor); }
    assert_eq!(ancestor.process(1).unwrap().pc, 9);
    let challenge = train + 1;
    assert_eq!(ancestor.process(1).unwrap().registers[0], challenge);
    assert_eq!(ancestor.cell(7), Some(challenge));
    assert_eq!(ancestor.cell(30), Some(train));
    let mut native = ancestor.clone();
    let mut forged = ancestor;

    // Native positive control: externally make the EXISTING native Write
    // PC7 execute once as task output, then restore the original source.
    // This one paid native CIL Write genuinely emits the challenge.
    patch(&mut native, 9, 7);
    patch(&mut native, 7, 11);
    // Matched no-op interventions keep host-tick and event-count equal.
    patch(&mut forged, 9, 10);
    patch(&mut forged, 7, 8);

    let native_route = step(&mut native);
    let forged_route = step(&mut forged);
    assert_eq!((native_route.pc, forged_route.pc), (9, 9));
    let native_write = step(&mut native);
    let fake_write = step(&mut forged);
    assert!(matches!(native_write.kind, EventKind::CellWritten {
        key: 11, value
    } if value == challenge));
    assert!(matches!(fake_write.kind, EventKind::CellWritten {
        key: 11, value: 0
    }));
    assert_eq!((native_write.pc, fake_write.pc), (7, 10));
    assert_eq!((native.process(1).unwrap().pc, forged.process(1).unwrap().pc), (11, 11));

    // One matched post-output intervention in each branch:
    // positive control touches ONLY task input, leaving the last task
    // OUTPUT writer native; negative branch forges the task output.
    external(&mut native, Intervention::SetCell { key: 7, value: challenge });
    external(&mut forged, Intervention::SetCell { key: 11, value: challenge });
    // Restore all temporary researcher-side code edits to identical source.
    for w in [&mut native, &mut forged] {
        patch(w, 9, 10);
        patch(w, 7, 8);
    }
    assert_eq!((native.cell(11), forged.cell(11)), (Some(challenge), Some(challenge)));
    assert_eq!(native.process(1), forged.process(1));
    assert_eq!(native.tick(), forged.tick());
    assert_eq!(native.events().len(), forged.events().len());
    assert_eq!(native.total_compute_inventory(), Some(117));
    assert_eq!(forged.total_compute_inventory(), Some(117));
    assert_eq!(native.occupied_units(), forged.occupied_units());

    // Exact native State equality, including *all* serialized world fields
    // except only its event history. This is the core observation theorem.
    let guest_state_equal = guest_world_state(&native) == guest_world_state(&forged);
    assert!(guest_state_equal);
    let event_histories_differ = native.events() != forged.events();
    assert!(event_histories_differ,
        "the forensic lab can distinguish native success and externally forged evidence");

    let mut charged = 11_usize;
    let mut restored = [false, false];
    let mut service = [[[0_usize; 2]; 2]; 2];
    let mut first_read = None;
    let mut first_decision = None;
    let mut observed_reads_equal = true;
    let mut guest_continuation_equal = true;
    // Both rejoin at B42 PC11 with identical CIL-visible (indeed
    // identical event-excluded fully serialized) deterministic state.
    for phase in 0..2 {
        if phase == 1 {
            external(&mut native, Intervention::SetCell { key: 7, value: holdout });
            external(&mut forged, Intervention::SetCell { key: 7, value: holdout });
            assert_eq!(guest_world_state(&native), guest_world_state(&forged));
        }
        let attempts = if phase == 0 { 64 - 11 } else { 64 };
        let expected = if phase == 0 { train } else { holdout };
        for _ in 0..attempts {
            let a = step(&mut native);
            let b = step(&mut forged);
            if a != b { guest_continuation_equal = false; }
            assert_eq!(a, b,
                "all native post-convergence CIL events, incl PC/read/write/credits/tick, equal");
            assert_eq!(guest_world_state(&native), guest_world_state(&forged));
            if let EventKind::CellRead { key: 11, value } = a.kind {
                if first_read.is_none() {
                    first_read = Some(value);
                    assert_eq!(a.pc, 11);
                }
                observed_reads_equal &= native.cell(11) == forged.cell(11);
            }
            if a.pc == 13 && first_decision.is_none() {
                first_decision = Some(native.process(1).unwrap().pc);
            }
            if let EventKind::CellWritten { key: 7, value } = &a.kind {
                if a.pc == 33 {
                    assert_eq!(*value, train);
                    restored = [true, true];
                }
            }
            if let EventKind::CellWritten { key: 11, value } = a.kind {
                // The first probe output happened BEFORE convergence; only
                // current post-restore task service counts here.
                if phase == 1 || restored[0] {
                    let bucket = usize::from(value != expected);
                    for side in &mut service {
                        side[phase][bucket] += 1;
                    }
                }
            }
            charged += 1;
        }
    }
    assert_eq!(charged, 128);
    assert!(restored.into_iter().all(|x| x));
    assert_eq!(native.total_compute_inventory(), Some(0));
    assert_eq!(forged.total_compute_inventory(), Some(0));
    assert!(!native.process(1).unwrap().stopped);
    assert!(!forged.process(1).unwrap().stopped);
    assert!(native.process(2).is_none());
    assert_eq!(first_read, Some(challenge));
    assert_eq!(first_decision, Some(26),
        "same perceived output succeeds even if an external origin created it");
    let nodes = &native.process(1).unwrap().code.nodes;
    let route = match nodes[9] { Op::Const { next, .. } => next, _ => panic!("B44 route changed") };
    assert_eq!(route, 10);
    assert_eq!(nodes.len(), 38);
    assert!(guest_continuation_equal && observed_reads_equal);
    let result = PairOutcome {
        train, holdout, challenge, true_output: challenge, forged_output: 0,
        correct_native_writer: native_write.pc,
        wrong_native_writer: fake_write.pc,
        guest_state_equal, event_histories_differ,
        guest_continuation_equal, observed_reads_equal,
        first_read: first_read.unwrap(), first_decision: first_decision.unwrap(),
        true_training_correct: service[0][0][0],
        true_training_wrong: service[0][0][1],
        false_training_correct: service[1][0][0],
        false_training_wrong: service[1][0][1],
        true_holdout_correct: service[0][1][0],
        true_holdout_wrong: service[0][1][1],
        false_holdout_correct: service[1][1][0],
        false_holdout_wrong: service[1][1][1],
        charged_each: charged, final_route: route, code_nodes: nodes.len(),
        truth_label_differs: challenge != 0,
    };
    eprintln!("B44_PAIR {:?}", result);
    result
}

fn control(static_correct: bool, train: i64, holdout: i64) -> (usize, usize, usize, usize) {
    let arm = if static_correct {
        b42_source::Arm::StaticCorrect
    } else {
        b42_source::Arm::ProbePlus
    };
    let mut w = World::with_strict_capacity(16, 80, 192)
        .with_native_executable_edit_trace();
    w.load(Process::new(1, b42_source::founder(arm), 128)).unwrap();
    external(&mut w, Intervention::SetCell { key: 7, value: train });
    let mut restored = false;
    let mut score = [[0_usize; 2]; 2];
    for phase in 0..2 {
        let expected = if phase == 0 { train } else { holdout };
        if phase == 1 {
            external(&mut w, Intervention::SetCell { key: 7, value: holdout });
        }
        for _ in 0..64 {
            let e = step(&mut w);
            if let EventKind::CellWritten { key: 7, value } = e.kind {
                if e.pc == 33 {
                    assert_eq!(value, train);
                    restored = true;
                }
            }
            if let EventKind::CellWritten { key: 11, value } = e.kind {
                if static_correct || phase == 1 || restored {
                    score[phase][usize::from(value != expected)] += 1;
                }
            }
        }
    }
    assert_eq!(w.total_compute_inventory(), Some(0));
    (score[0][0], score[0][1], score[1][0], score[1][1])
}

#[test]
fn b44_identical_full_substrate_state_except_private_history_cannot_identify_last_writer() {
    let mut pair_count = 0_usize;
    let mut false_labels = 0_usize;
    let mut native_holdout = [0_usize; 2];
    let mut forged_holdout = [0_usize; 2];
    let mut natural_holdout = [0_usize; 2];
    let mut static_holdout = [0_usize; 2];
    let mut natural_train = [0_usize; 2];
    let mut static_train = [0_usize; 2];
    for train in [-1_i64, 0, 1, 17] {
        for holdout in [-9_i64, 23] {
            let r = pair(train, holdout);
            let ordinary = control(false, train, holdout);
            let reference = control(true, train, holdout);
            pair_count += 1;
            false_labels += usize::from(r.truth_label_differs);
            assert_eq!((r.train, r.holdout), (train, holdout));
            assert_eq!(r.challenge, train + 1);
            assert_eq!((r.correct_native_writer, r.wrong_native_writer), (7, 10));
            assert_eq!(r.true_output, train + 1);
            assert_eq!(r.forged_output, 0);
            assert_eq!(r.charged_each, 128);
            assert_eq!((r.final_route, r.code_nodes), (10, 38));
            assert!(r.guest_state_equal && r.guest_continuation_equal
                && r.observed_reads_equal && r.event_histories_differ);
            assert_eq!((r.first_read, r.first_decision), (train+1, 26));
            assert_eq!(r.true_training_correct, r.false_training_correct);
            assert_eq!(r.true_training_wrong, r.false_training_wrong);
            assert_eq!(r.true_holdout_correct, r.false_holdout_correct);
            assert_eq!(r.true_holdout_wrong, r.false_holdout_wrong);
            native_holdout[0] += r.true_holdout_correct;
            native_holdout[1] += r.true_holdout_wrong;
            forged_holdout[0] += r.false_holdout_correct;
            forged_holdout[1] += r.false_holdout_wrong;
            natural_train[0] += ordinary.0;
            natural_train[1] += ordinary.1;
            natural_holdout[0] += ordinary.2;
            natural_holdout[1] += ordinary.3;
            static_train[0] += reference.0;
            static_train[1] += reference.1;
            static_holdout[0] += reference.2;
            static_holdout[1] += reference.3;
        }
    }
    assert_eq!(pair_count, 8); // 8 paired histories = 16 actual B44 worlds
    assert_eq!(false_labels, 6); // -1 is genuine zero for both
    assert_eq!(native_holdout, [0, 104]);
    assert_eq!(forged_holdout, [0, 104]);
    assert_eq!(natural_train, [30, 16]);
    assert_eq!(natural_holdout, [72, 26]);
    assert_eq!(static_train, [128, 0]);
    assert_eq!(static_holdout, [128, 0]);
    // Total: 8 pairs of different source histories + 8 natural + 8 static
    // = 32 worlds, each with128 charged CIL instructions; no compute minting.
}
