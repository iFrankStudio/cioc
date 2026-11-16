//! CL-1B B42: a continuous actor writes a paid endogenous test challenge,
//! clones its own ordinary Write primitive into a NEW executable service node,
//! and selects/retains that node only after actual task-output feedback.
//! The challenge rule, objective, and code-reuse recipe are investigator-authored.
use cl0_world::{EventKind, Intervention, Op, Origin, Process, World};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Arm { ProbePlus, ProbeMinus, NoProbe, IgnoreFeedback, PatchDisabled, StaticCorrect }

fn founder(arm: Arm) -> Vec<Op> {
    if arm == Arm::StaticCorrect {
        return vec![
            Op::Const { dst: 2, value: 7, next: 1 },
            Op::Read { dst: 0, address: 2, next: 2 },
            Op::Const { dst: 2, value: 11, next: 3 },
            Op::Write { address: 2, value: 0, next: 0 },
        ];
    }
    // Source literal is parsed independently by B42's JS CIL model.
    let mut code = vec![
        Op::Const { dst: 2, value: 7, next: 1 },         // 0 environment input
        Op::Read { dst: 0, address: 2, next: 2 },        // 1 sample original input
        Op::Const { dst: 2, value: 30, next: 3 },        // 2 actor checkpoint location
        Op::Write { address: 2, value: 0, next: 4 },     // 3 paid original-input checkpoint
        Op::Const { dst: 1, value: 1, next: 5 },         // 4 authored challenge step
        Op::Add { dst: 0, lhs: 0, rhs: 1, next: 6 },     // 5 generate probe from input
        Op::Const { dst: 2, value: 7, next: 7 },         // 6 diagnostic input address
        Op::Write { address: 2, value: 0, next: 8 },     // 7 write probe; later clone this Write
        Op::Const { dst: 3, value: 0, next: 9 },         // 8 defective source register
        Op::Const { dst: 2, value: 11, next: 10 },       // 9 editable service route
        Op::Write { address: 2, value: 3, next: 11 },    // 10 constant-zero faulty service
        Op::Read { dst: 1, address: 2, next: 12 },       // 11 actual output feedback
        Op::Sub { dst: 1, lhs: 0, rhs: 1, next: 13 },    // 12 true discrepancy
        Op::Branch { test: 1, nonzero: 14, zero: 26 },   // 13 accept or try construction
        Op::Const { dst: 2, value: 22, next: 15 },       // 14 fallback attempt flag
        Op::Read { dst: 1, address: 2, next: 16 },       // 15 read actor-paid trial marker
        Op::Branch { test: 1, nonzero: 37, zero: 17 },   // 16 second failure -> halt
        Op::Const { dst: 2, value: 7, next: 18 },        // 17 source primitive node7
        Op::CloneNode { dst: 1, source: 2, next: 19 },   // 18 new executable node at38
        Op::Const { dst: 2, value: 11, next: 20 },       // 19 feedback reader address
        Op::PatchNext { node: 1, target: 2, next: 21 },  // 20 clone.next 8->11
        Op::Const { dst: 2, value: 9, next: 22 },        // 21 service route node9
        Op::PatchNext { node: 2, target: 1, next: 23 },  // 22 route to new clone
        Op::Const { dst: 1, value: 1, next: 24 },        // 23 paid retry marker
        Op::Const { dst: 2, value: 22, next: 25 },       // 24 retry marker address
        Op::Write { address: 2, value: 1, next: 9 },     // 25 true actor retry, then test clone
        Op::Const { dst: 2, value: 9, next: 27 },        // 26 recover selected route
        Op::ReadNext { dst: 1, node: 2, next: 28 },      // 27 actual service Write index
        Op::Const { dst: 2, value: 34, next: 29 },       // 28 steady service loop
        Op::PatchNext { node: 1, target: 2, next: 30 },  // 29 retain selected Write next->34
        Op::Const { dst: 2, value: 30, next: 31 },       // 30 original checkpoint
        Op::Read { dst: 0, address: 2, next: 32 },       // 31 restore original value
        Op::Const { dst: 2, value: 7, next: 33 },        // 32 environment input address
        Op::Write { address: 2, value: 0, next: 34 },    // 33 restore input before task scoring
        Op::Const { dst: 2, value: 7, next: 35 },        // 34 steady input address
        Op::Read { dst: 0, address: 2, next: 36 },       // 35 steady actual input sample
        Op::Const { dst: 2, value: 11, next: 9 },        // 36 service output address
        Op::Halt,                                       // 37 two tried candidates failed
    ];
    match arm {
        Arm::ProbeMinus => code[4] = Op::Const { dst: 1, value: -1, next: 5 },
        Arm::NoProbe => code[5] = Op::Add { dst: 0, lhs: 0, rhs: 3, next: 6 },
        Arm::IgnoreFeedback => code[12] = Op::Sub { dst: 1, lhs: 0, rhs: 0, next: 13 },
        Arm::ProbePlus | Arm::PatchDisabled | Arm::StaticCorrect => {}
    }
    code
}

#[derive(Debug)]
struct Outcome {
    arm: Arm, train: i64, holdout: i64, probe: Option<i64>,
    diagnostic_correct: usize, diagnostic_wrong: usize,
    train_correct: usize, train_wrong: usize, holdout_correct: usize, holdout_wrong: usize,
    charged: usize, stopped: bool, restored: bool, code_nodes: usize,
    final_edge: usize, cloned_writer_next: Option<usize>,
    clone_events: usize, route_edits: usize, retention_edits: usize, suppressed: usize,
    original_input_preserved: bool,
}

fn run(arm: Arm, train: i64, holdout: i64) -> Outcome {
    let mut w = World::with_strict_capacity(16, 80, 192).with_native_executable_edit_trace();
    if arm == Arm::PatchDisabled { w = w.with_native_patchnext_disabled(); }
    w.load(Process::new(1, founder(arm), 128)).unwrap();
    w.intervene(Intervention::SetCell { key: 7, value: train }).unwrap();
    let mut paid=0;
    let mut diagnostic=[0_usize;2];
    let mut work_train=[0_usize;2];
    let mut work_holdout=[0_usize;2];
    let mut restored=false;
    let mut probe=None;
    for (phase, input) in [train, holdout].into_iter().enumerate() {
        if phase==1 { w.intervene(Intervention::SetCell { key: 7, value: input }).unwrap(); }
        let old_inventory=w.total_compute_inventory().unwrap();
        let old_len=w.events().len();
        let n=w.run(64);
        paid+=n;
        assert_eq!(w.total_compute_inventory(),Some(old_inventory-n as u64));
        for e in &w.events()[old_len..] {
            if e.origin != Origin::Internal(1) { continue; }
            match &e.kind {
                EventKind::CellWritten{key:7,value} if e.pc == 7 => probe=Some(*value),
                EventKind::CellWritten{key:7,value} if e.pc == 33 => {
                    assert_eq!(*value,train,"B42 must restore environment input after diagnostic");
                    restored=true;
                }
                EventKind::CellWritten{key:11,value} => {
                    let bucket=if phase==0 && !restored && arm!=Arm::StaticCorrect { &mut diagnostic }
                        else if phase==0 { &mut work_train }
                        else { &mut work_holdout };
                    let expected=if phase==0 && !restored && arm!=Arm::StaticCorrect { probe.expect("probe occurred") }
                        else { input };
                    bucket[usize::from(*value!=expected)]+=1;
                }
                _=>{}
            }
        }
    }
    assert_eq!(w.total_compute_inventory(),Some(128-paid as u64));
    assert!(w.process(2).is_none());
    let edges=w.events().iter().filter_map(|e| match (&e.origin,&e.kind){
        (Origin::Internal(1),EventKind::ExecutableEdgePatched{node,before,after})=>Some((*node,*before,*after)),
        _=>None,
    }).collect::<Vec<_>>();
    let source=&w.process(1).unwrap().code.nodes;
    let (final_edge,cloned_writer_next)=if arm==Arm::StaticCorrect {(3,None)} else {
        let e=match &source[9] { Op::Const{next,..}=>*next,_=>panic!("B42 router lost") };
        let clone=if source.len()>38 {match &source[38] {Op::Write{next,..}=>Some(*next),_=>panic!("B42 cloned wrong opcode")}}else{None};
        (e,clone)
    };
    let out=Outcome {
        arm,train,holdout,probe,diagnostic_correct:diagnostic[0],diagnostic_wrong:diagnostic[1],
        train_correct:work_train[0],train_wrong:work_train[1],
        holdout_correct:work_holdout[0],holdout_wrong:work_holdout[1],
        charged:paid, stopped:w.process(1).unwrap().stopped,restored,
        code_nodes:source.len(),final_edge,cloned_writer_next,
        clone_events:usize::from(source.len()>38),
        route_edits:edges.iter().filter(|(n,_,_)|*n==9).count(),
        retention_edits:edges.iter().filter(|(n,b,a)|(*n==10||*n==38)&&*b==11&&*a==34).count(),
        suppressed:w.events().iter().filter(|e|matches!((&e.origin,&e.kind),
            (Origin::Internal(1),EventKind::NativeCodeEditSuppressed{opcode_tag:6}))).count(),
        original_input_preserved:w.cell(7)==Some(holdout),
    };
    eprintln!("B42_NATIVE {:?}",out);
    out
}

#[test]
fn b42_actor_generated_challenge_and_reused_instruction_candidate_have_limits(){
    let mut worlds=0;
    // Source-parsed observer forecasts frozen before the first native trial.
    // Columns: train true/false, heldout true/false, paid, clones, restored, halted.
    let mut totals=[[0_usize;8];6];
    for train in [-1_i64,0,1,17] {
        for holdout in [-9_i64,23] {
            let plus=run(Arm::ProbePlus,train,holdout);
            let minus=run(Arm::ProbeMinus,train,holdout);
            let no_probe=run(Arm::NoProbe,train,holdout);
            let ignore=run(Arm::IgnoreFeedback,train,holdout);
            let disabled=run(Arm::PatchDisabled,train,holdout);
            let compact=run(Arm::StaticCorrect,train,holdout);
            worlds+=6;
            for (i,r) in [&plus,&minus,&no_probe,&ignore,&disabled,&compact].into_iter().enumerate() {
                totals[i][0]+=r.train_correct;
                totals[i][1]+=r.train_wrong;
                totals[i][2]+=r.holdout_correct;
                totals[i][3]+=r.holdout_wrong;
                totals[i][4]+=r.charged;
                totals[i][5]+=r.clone_events;
                totals[i][6]+=usize::from(r.restored);
                totals[i][7]+=usize::from(r.stopped);
                assert_eq!(r.train,train);
                assert_eq!(r.holdout,holdout);
                assert!(r.charged<=128);
            }
            assert_eq!(plus.probe,Some(train+1));
            assert_eq!(minus.probe,Some(train-1));
            assert_eq!(no_probe.probe,Some(train));
            assert_eq!(ignore.probe,Some(train+1));
            assert_eq!(disabled.probe,Some(train+1));
            assert_eq!(plus.code_nodes,if train==-1{38}else{39});
            assert_eq!(minus.code_nodes,if train==1{38}else{39});
            assert_eq!(plus.final_edge,if train==-1{10}else{38});
            assert_eq!(minus.final_edge,if train==1{10}else{38});
            assert_eq!(no_probe.final_edge,if train==0{10}else{38});
            assert_eq!(ignore.final_edge,10);
            assert_eq!(disabled.final_edge,10);
            assert_eq!(plus.clone_events,usize::from(train!=-1));
            assert_eq!(plus.retention_edits,usize::from(train==-1)+usize::from(train!=-1));
            assert!(plus.restored && minus.restored && no_probe.restored && ignore.restored);
            assert!(plus.original_input_preserved);
            assert_eq!(compact.train_wrong+compact.holdout_wrong,0);
            assert_eq!(compact.code_nodes,4);
            assert_eq!(compact.charged,128);
            if train==0 {
                assert!(plus.holdout_correct>0);
                assert_eq!(no_probe.holdout_correct,0);
                assert!(no_probe.holdout_wrong>0);
            }
            if train==-1 {
                assert_eq!(plus.holdout_correct,0);
                assert!(plus.holdout_wrong>0);
            }
            if train==1 {
                assert_eq!(minus.holdout_correct,0);
                assert!(minus.holdout_wrong>0);
            }
            assert_eq!(ignore.holdout_correct,0);
            assert!(ignore.holdout_wrong>0);
            assert_eq!(disabled.route_edits+disabled.retention_edits,0);
            assert!(disabled.suppressed>0);
            assert!(compact.train_correct+compact.holdout_correct>=plus.train_correct+plus.holdout_correct);
        }
    }
    assert_eq!(worlds,48);
    assert_eq!(totals,[
        [30,16,72,26,1024,6,8,0], // probe x+1
        [30,16,72,26,1024,6,8,0], // probe x-1: moved blind spot
        [46,0,72,26,1024,6,8,0],  // no probe: input0 blind spot
        [16,48,0,104,1024,0,8,0],// evaluator ablated
        [0,4,0,0,312,8,2,8],      // code editing blocked; halts
        [128,0,128,0,1024,0,0,0], // cheap immutable copier
    ], "native B42 event totals must match frozen independent CIL forecast");
}
