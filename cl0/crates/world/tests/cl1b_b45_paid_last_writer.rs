//! CL-1B B45: generic, paid, finite last-writer provenance inside the
//! deterministic world. No fitness, output-correctness or repair oracle.
//! All validation/branching policy remains investigator-authored CIL.
//! B42's exact founder source is reused and its own-native CloneNode acts
//! with its old semantics. One continuing PID1 in each task-scored world.
use cl0_world::{EventKind, Intervention, Op, Origin, Process, World};
#[path = "cl1b_b42_self_assembled_probe.rs"]
mod b42_source;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Arm {
    GuardTrackedSpoof,
    GuardTrackedGenuine,
    GuardTrackedEcho,
    GuardUntrackedSpoof,
    OriginalTrackedSpoof,
    OriginalUntrackedSpoof,
    StaticTracked,
}

fn founder(arm: Arm) -> Vec<Op> {
    if arm == Arm::StaticTracked {
        return b42_source::founder(b42_source::Arm::StaticCorrect);
    }
    let mut code = b42_source::founder(b42_source::Arm::ProbePlus);
    if matches!(arm, Arm::GuardTrackedSpoof | Arm::GuardTrackedGenuine
        | Arm::GuardTrackedEcho | Arm::GuardUntrackedSpoof)
    {
        // Four normal charged CIL instructions, with NO dedicated verifier:
        // if value matches the challenge, check that the last writer of
        // task-output cell11 has the actor's actual PID1. ReadLastWriter
        // returns an ordinary numerical register, never a pass/fail verdict.
        // Native clone grows at index42, not the previous index38.
        code[13] = Op::Branch { test: 1, nonzero: 14, zero: 38 };
        code.push(Op::ReadLastWriter { dst: 3, address: 2, next: 39 });
        code.push(Op::Const { dst: 2, value: 1, next: 40 });
        code.push(Op::Sub { dst: 3, lhs: 3, rhs: 2, next: 41 });
        code.push(Op::Branch { test: 3, nonzero: 14, zero: 26 });
    }
    code
}

fn tracked(arm: Arm) -> bool {
    !matches!(arm, Arm::GuardUntrackedSpoof | Arm::OriginalUntrackedSpoof)
}
fn guarded(arm: Arm) -> bool {
    matches!(arm, Arm::GuardTrackedSpoof | Arm::GuardTrackedGenuine
        | Arm::GuardTrackedEcho | Arm::GuardUntrackedSpoof)
}
fn attacks(arm: Arm) -> bool {
    matches!(arm, Arm::GuardTrackedSpoof | Arm::GuardTrackedEcho
        | Arm::GuardUntrackedSpoof | Arm::OriginalTrackedSpoof
        | Arm::OriginalUntrackedSpoof)
}

#[derive(Debug)]
struct Observation {
    arm: Arm, train: i64, holdout: i64, challenge: Option<i64>,
    first_output: Option<i64>, first_read: Option<i64>, first_writer: Option<i64>,
    first_verdict: Option<usize>, forged: usize, sensor_events: usize,
    train_true: usize, train_wrong: usize, holdout_true: usize,
    holdout_wrong: usize, paid: usize, storage: usize, restored: bool,
    nodes: usize, clones: usize, final_router: usize, stopped: bool,
    false_attestation: bool,
}

fn run(arm: Arm, train: i64, holdout: i64) -> Observation {
    let mut w = World::with_strict_capacity(16, 80, 192)
        .with_native_executable_edit_trace();
    if tracked(arm) { w = w.with_cell_write_provenance(); }
    let code = founder(arm);
    let base_len = code.len();
    w.load(Process::new(1, code, 128)).unwrap();
    w.intervene(Intervention::SetCell { key: 7, value: train }).unwrap();
    let mut challenge = None;
    let mut first_output = None;
    let mut first_read = None;
    let mut first_writer = None;
    let mut first_verdict = None;
    let mut forged = 0_usize;
    let mut sensor_events = 0_usize;
    let mut score = [[0_usize;2];2];
    let mut paid = 0_usize;
    let mut restored = false;
    for (phase,input) in [train,holdout].into_iter().enumerate() {
        if phase == 1 {
            w.intervene(Intervention::SetCell { key: 7, value: input }).unwrap();
        }
        for _ in 0..64 {
            if w.process(1).unwrap().stopped { break; }
            let old = w.total_compute_inventory().unwrap();
            assert!(w.step());
            paid += 1;
            assert_eq!(w.total_compute_inventory(), Some(old - 1));
            let e = w.events().last().unwrap().clone();
            assert_eq!(e.origin, Origin::Internal(1));
            assert!(!matches!(&e.kind, EventKind::Trapped(_)));
            if phase==0 && arm!=Arm::StaticTracked {
                match &e.kind {
                    EventKind::CellWritten { key:7,value } if e.pc==7 =>
                        challenge=Some(*value),
                    EventKind::CellWritten { key:11,value } if e.pc==10
                        && first_output.is_none() => {
                        first_output=Some(*value);
                        assert_eq!(*value,0);
                        assert_eq!(w.process(1).unwrap().pc, 11);
                        if attacks(arm) {
                            let injected = if arm==Arm::GuardTrackedEcho {
                                0
                            } else { train+1 };
                            w.intervene(Intervention::SetCell { key:11,value:injected }).unwrap();
                            assert_eq!(w.events().last().unwrap().origin, Origin::External);
                            forged += 1;
                            if tracked(arm) {
                                assert_eq!(w.cell_last_writer(11), Some(0));
                            }
                        }
                    }
                    EventKind::CellRead { key:11,value } if e.pc==11
                        && first_read.is_none() => first_read=Some(*value),
                    EventKind::CellWriterSensed {key:11,writer} => {
                        sensor_events += 1;
                        if first_writer.is_none() { first_writer=Some(*writer); }
                    }
                    EventKind::CellWritten {key:7,value} if e.pc==33 => {
                        assert_eq!(*value,train);
                        restored=true;
                    }
                    _=>{}
                }
                if e.pc==13 && first_verdict.is_none() {
                    first_verdict=Some(w.process(1).unwrap().pc);
                }
            }
            if let EventKind::CellWritten {key:11,value} = e.kind {
                // Diagnostic Write(s) while actor owns the challenge input
                // are not credited as external-task service.
                if phase==1 || restored || arm==Arm::StaticTracked {
                    score[phase][usize::from(value!=input)] += 1;
                }
            }
        }
    }
    assert_eq!(w.total_compute_inventory(), Some(128-paid as u64));
    assert!(w.process(2).is_none());
    let source=&w.process(1).unwrap().code.nodes;
    let nodes=source.len();
    let router=if arm==Arm::StaticTracked {3} else {
        match &source[9] {Op::Const {next,..}=>*next,_=>panic!("B45 lost B42 router")}
    };
    let r=Observation {
        arm,train,holdout,challenge,first_output,first_read,first_writer,first_verdict,
        forged,sensor_events,train_true:score[0][0],train_wrong:score[0][1],
        holdout_true:score[1][0],holdout_wrong:score[1][1],
        paid,storage:w.occupied_units(),restored,nodes,
        clones:usize::from(nodes>base_len), final_router:router,
        stopped:w.process(1).unwrap().stopped,
        false_attestation: first_read==challenge && first_output!=challenge
            && first_verdict==Some(26),
    };
    eprintln!("B45_NATIVE {:?}",r);
    r
}

#[test]
fn b45_paid_last_writer_provenance_has_universal_source_and_bounded_storage() {
    let src = Op::ReadLastWriter {dst:1,address:0,next:2};
    assert_eq!(Op::decode_instruction([33,1,0,2,0]).unwrap(),src);

    // Exact two units per data cell, one for data and one for origin,
    // on the opt-in world, with no uncharged extra capacity allocation.
    let mut w = World::with_strict_capacity(8,8,4).with_cell_write_provenance();
    w.load(Process::new(1,vec![Op::Delete{address:0,next:0}],3)).unwrap();
    assert_eq!(w.occupied_units(),2);
    w.intervene(Intervention::SetCell{key:0,value:42}).unwrap();
    assert_eq!(w.occupied_units(),4);
    assert_eq!(w.cell_last_writer(0),Some(0));
    assert!(w.intervene(Intervention::SetCell{key:1,value:100}).is_err());
    assert_eq!(w.cell_last_writer(1),None);
    assert_eq!(w.occupied_units(),4);
    assert_eq!(w.run(1),1); // real charged native Delete, frees both slots
    assert_eq!(w.occupied_units(),2);
    assert_eq!(w.cell_last_writer(0),None);
    w.intervene(Intervention::SetCell{key:1,value:100}).unwrap();
    assert_eq!(w.occupied_units(),4);
    let bytes=serde_json::to_string(&w).unwrap();
    let loaded:World=serde_json::from_str(&bytes).unwrap();
    assert_eq!(w,loaded);
    assert_eq!(loaded.cell_last_writer(1),Some(0));

    // Native actor-write allocation must reject one free slot, not
    // write data while silently allocating free provenance metadata.
    let mut tight=World::with_strict_capacity(8,8,4).with_cell_write_provenance();
    tight.load(Process::new(1,vec![
        Op::Const{dst:0,value:7,next:1},
        Op::Write{address:0,value:0,next:1},
    ],3)).unwrap(); // code2 + strict header1 => only one free unit
    assert_eq!(tight.run(2),2);
    assert_eq!(tight.cell(7),None);
    assert_eq!(tight.cell_last_writer(7),None);
    assert_eq!(tight.occupied_units(),3);
    assert!(tight.process(1).unwrap().stopped);
    assert_eq!(tight.remaining_credits(),1);

    // The origin tag identifies ANOTHER ordinary native PID, not
    // "goodness". ReadLastWriter returns a charged register number.
    let mut peers=World::with_strict_capacity(8,16,32).with_cell_write_provenance();
    peers.load(Process::new(2,vec![
        Op::Const{dst:0,value:7,next:1},
        Op::Const{dst:1,value:99,next:2},
        Op::Write{address:0,value:1,next:3},
        Op::Halt,
    ],8)).unwrap();
    peers.intervene(Intervention::SetCell{key:7,value:17}).unwrap();
    assert_eq!(peers.cell_last_writer(7),Some(0));
    assert_eq!(peers.run(3),3);
    assert_eq!(peers.cell_last_writer(7),Some(2));
    peers.load(Process::new(1,vec![
        Op::Const{dst:0,value:7,next:1},
        src,
        Op::Halt,
    ],6)).unwrap();
    // Scheduler may first continue PID2; complete enough to read PID1.
    assert!(peers.run(4)>=2);
    assert_eq!(peers.process(1).unwrap().registers[1],2);
    assert!(peers.events().iter().any(|e|matches!(
        (&e.origin,&e.kind),
        (Origin::Internal(1),EventKind::CellWriterSensed{key:7,writer:2})
    )));

    // Explicit legacy backward compatibility, default world has no new
    // serializable provenance fields, and untracked source is UNKNOWN -1.
    let old=World::with_strict_capacity(8,8,32);
    let json=serde_json::to_value(&old).unwrap();
    assert!(json.get("cell_write_provenance_enabled").is_none());
    assert!(json.get("cell_last_writer").is_none());
    let old_copy:World=serde_json::from_value(json).unwrap();
    assert!(!old_copy.cell_write_provenance_enabled());
    assert_eq!(old_copy.occupied_units(),0);
}

#[test]
fn b45_authored_guard_can_use_paid_provenance_but_not_prove_general_correctness() {
    let arms=[
        Arm::GuardTrackedSpoof,
        Arm::GuardTrackedGenuine,
        Arm::GuardTrackedEcho,
        Arm::GuardUntrackedSpoof,
        Arm::OriginalTrackedSpoof,
        Arm::OriginalUntrackedSpoof,
        Arm::StaticTracked,
    ];
    let mut n=0;
    let mut totals=[[0_usize;9];7];
    for train in [-1_i64,0,1,17] {
        for holdout in [-9_i64,23] {
            let rs:Vec<_>=arms.iter().map(|&arm|run(arm,train,holdout)).collect();
            n+=rs.len();
            for (i,r) in rs.iter().enumerate() {
                assert_eq!(r.arm,arms[i]);
                assert_eq!((r.train,r.holdout),(train,holdout));
                assert!(r.paid<=128);
                assert!(r.storage<=192);
                totals[i][0]+=r.train_true;
                totals[i][1]+=r.train_wrong;
                totals[i][2]+=r.holdout_true;
                totals[i][3]+=r.holdout_wrong;
                totals[i][4]+=r.paid;
                totals[i][5]+=r.clones;
                totals[i][6]+=usize::from(r.stopped);
                totals[i][7]+=r.sensor_events;
                totals[i][8]+=usize::from(r.false_attestation);
                if r.arm!=Arm::StaticTracked {
                    assert_eq!(r.challenge,Some(train+1));
                    assert_eq!(r.first_output,Some(0));
                    assert!(r.restored || r.stopped);
                }
            }
            let [guard,fair,echo,untracked,original,legacy,static_ref]=
                [&rs[0],&rs[1],&rs[2],&rs[3],&rs[4],&rs[5],&rs[6]];
            assert_eq!(guard.forged,1);
            assert_eq!(echo.forged,1);
            assert_eq!(untracked.forged,1);
            assert_eq!(original.forged,1);
            assert_eq!(legacy.forged,1);
            assert_eq!(fair.forged,0);
            assert_eq!(static_ref.forged,0);
            assert_eq!(guard.first_read,Some(train+1));
            assert_eq!(original.first_read,Some(train+1));
            assert_eq!(legacy.first_read,Some(train+1));
            assert_eq!(guard.first_writer,Some(0));
            assert_eq!(untracked.first_writer,Some(-1));
            assert_eq!(fair.first_writer,Some(1));
            assert_eq!(original.first_writer,None);
            assert_eq!(legacy.first_writer,None);
            assert!(guard.sensor_events>=1);
            assert!(untracked.sensor_events>=1);
            assert_eq!(original.sensor_events,0);
            assert_eq!(legacy.sensor_events,0);
            assert_eq!(original.final_router,10);
            assert_eq!(legacy.final_router,10);
            assert_eq!(original.holdout_true,0);
            assert_eq!(legacy.holdout_true,0);
            assert_eq!(static_ref.holdout_true,16);
            assert_eq!(static_ref.holdout_wrong,0);
        }
    }
    assert_eq!(n,56);
    eprintln!("B45_AGGREGATE {:?}",totals);
    // Frozen BEFORE the first native B45 CI, computed by a standalone
    // source-parsed CIL interpreter, not by observing Rust results.
    // Columns: train true/wrong, holdout true/wrong, charged attempts,
    // clones, halts, paid origin reads, falsified success attestations.
    assert_eq!(totals, [
        [24,0,96,8,1024,8,0,16,0],  // charged authenticated guard, spoof
        [24,14,78,26,1024,6,0,8,0], // provenance guard, honest feedback
        [30,0,102,2,1024,8,0,10,0], // same-value external write: overreacts
        [0,0,0,0,344,8,8,16,0],     // no provenance: fail-closed halt
        [16,48,0,104,1024,0,0,0,6],// provenance available but ignored
        [16,48,0,104,1024,0,0,0,6],// old untracked world, same code
        [128,0,128,0,1024,0,0,0,0], // compact correct independent source
    ], "B45 native event totals must match independent frozen forecast");
}
