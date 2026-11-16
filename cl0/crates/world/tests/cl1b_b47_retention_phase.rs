//! B47 prospective phase/feedback retention screen. This is an engineered
//! falsifier, NOT endogenous adaptation. Scores actual native Write events.
use cl0_world::{EventKind, Intervention, Op, Origin, Process, World};
#[path = "cl1b_b42_self_assembled_probe.rs"]
mod b42;
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
enum Arm { Feedback, Blind, Disabled, Phase, Compact, Oracle }
const ARMS:[Arm;6]=[Arm::Feedback,Arm::Blind,Arm::Disabled,Arm::Phase,Arm::Compact,Arm::Oracle];
fn founder(arm:Arm)->Vec<Op>{
    match arm {
        Arm::Compact|Arm::Oracle=>b42::founder(b42::Arm::StaticCorrect),
        Arm::Blind=>b42::founder(b42::Arm::IgnoreFeedback),
        Arm::Disabled=>b42::founder(b42::Arm::PatchDisabled),
        Arm::Phase=>{
            // Immutable correct copier with a paid phase shift. Unlike the
            // feedback actor, this arm cannot mutate executable edges.
            let mut c=b42::founder(b42::Arm::StaticCorrect);
            c[3]=Op::Write{address:2,value:0,next:4};
            c.push(Op::Const{dst:3,value:0,next:0});
            c
        },
        Arm::Feedback=>b42::founder(b42::Arm::ProbePlus),
    }
}
#[derive(Debug)]
struct ResultRow { arm:Arm, seed:usize, correct:[usize;3], wrong:[usize;3],
    paid:usize, edits:usize, suppressed:usize, stopped:bool, nodes:usize, storage:usize }
fn trial(arm:Arm,seed:usize,inputs:[i64;3])->ResultRow {
    let mut w=World::with_strict_capacity(16,80,192).with_native_executable_edit_trace();
    if arm==Arm::Disabled { w=w.with_native_patchnext_disabled(); }
    let original=founder(arm).len();
    w.load(Process::new(1,founder(arm),144)).unwrap();
    let mut correct=[0;3]; let mut wrong=[0;3]; let mut paid=0;
    let mut edits=0; let mut suppressed=0; let mut storage=w.occupied_units();
    let mut restored=false;
    for (phase,input) in inputs.into_iter().enumerate(){
        w.intervene(Intervention::SetCell{key:7,value:input}).unwrap();
        for _ in 0..48 {
            if w.process(1).unwrap().stopped {break;}
            let before=w.total_compute_inventory().unwrap();
            assert!(w.step());
            paid+=1;
            assert_eq!(w.total_compute_inventory(),Some(before-1));
            storage=storage.max(w.occupied_units());
            let e=w.events().last().unwrap();
            assert_eq!(e.origin,Origin::Internal(1));
            assert!(!matches!(e.kind,EventKind::Trapped(_)));
            match &e.kind {
                EventKind::ExecutableEdgePatched{..}=>edits+=1,
                EventKind::NativeCodeEditSuppressed{..}=>suppressed+=1,
                EventKind::CellWritten{key:7,..} if e.pc==33=>restored=true,
                EventKind::CellWritten{key:11,value}=>{
                    // Pre-restoration writes are diagnostic, not service.
                    if matches!(arm,Arm::Compact|Arm::Oracle)||phase>0||restored {
                        if *value==input {correct[phase]+=1;} else {wrong[phase]+=1;}
                    }
                },
                _=>{}
            }
        }
    }
    let row=ResultRow{arm,seed,correct,wrong,paid,edits,suppressed,
        stopped:w.process(1).unwrap().stopped,
        nodes:w.process(1).unwrap().code.nodes.len(),storage};
    assert!(row.paid<=144 && row.storage<=192);
    assert!(row.nodes>=original);
    eprintln!("B47_NATIVE {:?}",row);
    row
}
#[test]
fn b47_retention_phase_and_unseen_regimes(){
    // Seeds and regimes frozen in source, including zero and repeated inputs.
    const SEQUENCES:[[i64;3];8]=[
        [0,17,-9],[-1,23,0],[17,17,-9],[1,0,23],
        [23,-9,1],[-9,1,17],[0,0,23],[17,-1,0]
    ];
    let mut totals=[[0usize;8];6];
    for (seed,seq) in SEQUENCES.into_iter().enumerate(){
        let rows:Vec<_>=ARMS.iter().map(|&a|trial(a,seed,seq)).collect();
        for (i,r) in rows.iter().enumerate(){
            assert_eq!(r.arm,ARMS[i]); assert_eq!(r.seed,seed);
            totals[i][0]+=r.correct[1]; totals[i][1]+=r.wrong[1];
            totals[i][2]+=r.correct[2]; totals[i][3]+=r.wrong[2];
            totals[i][4]+=r.paid; totals[i][5]+=r.edits;
            totals[i][6]+=r.suppressed; totals[i][7]+=usize::from(r.stopped);
        }
        assert_eq!(rows[4].wrong,[0;3]);
        assert_eq!(rows[5].wrong,[0;3]);
        assert_eq!(rows[4].correct,rows[5].correct);
        assert_eq!(rows[0].paid,144);
        assert_eq!(rows[3].edits,0);
    }
    eprintln!("B47_AGGREGATE {:?}",totals);
    // H47 requires feedback to beat every baseline in BOTH holdout windows.
    // Record a negative finding without disguising it as a failed test.
    let beats=(1..=2).all(|phase|{
        let col=if phase==1{0}else{2};
        totals[0][col]>totals[1][col] &&
        totals[0][col]>totals[2][col] &&
        totals[0][col]>totals[3][col] &&
        totals[0][col]>totals[4][col]
    });
    eprintln!("B47_H47_ALL_BASELINES_BOTH_HOLDOUTS={beats}");
    assert!(!beats,"unexpected B47 positive requires independent event replay before acceptance");
}
