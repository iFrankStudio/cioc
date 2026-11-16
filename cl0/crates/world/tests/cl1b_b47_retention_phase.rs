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


// B48 is housed in the already-registered B47 integration target to avoid
// changing the CI target manifest before a native compilation check.
// Its separate prospective protocol is cl1b/studies/B48-*.md.
// This is an experimental assay, NOT a B3/B4 acceptance assertion.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
enum B48Arm { True, BlindSelf, BlindZero, EditDisabled, Compact }
#[derive(Debug)]
struct B48Row {
    arm:B48Arm, phase_shift:bool, seed:usize, raw_correct:[usize;3],
    raw_wrong:[usize;3], qualified_correct:[usize;3],
    qualified_wrong:[usize;3], paid:usize, edits:usize,
    suppressed:usize, stopped:bool, max_storage:usize,
}
fn b48_run(arm:B48Arm,phase_shift:bool,seed:usize,inputs:[i64;3])->B48Row {
    let mut code=if arm==B48Arm::Compact {
        b42::founder(b42::Arm::StaticCorrect)
    } else {
        b42::founder(b42::Arm::ProbePlus)
    };
    if arm==B48Arm::BlindSelf {
        code[12]=Op::Sub{dst:1,lhs:0,rhs:0,next:13};
    } else if arm==B48Arm::BlindZero {
        code[12]=Op::Sub{dst:1,lhs:0,rhs:3,next:13};
    }
    // The identical founder graph is retained; an appended ordinary paid
    // instruction changes the initial phase without renumbering any node.
    let entry=code.len();
    if phase_shift {code.push(Op::Const{dst:3,value:0,next:0});}
    let mut p=Process::new(1,code,144);
    if phase_shift {p.pc=entry;}
    let mut w=World::with_strict_capacity(16,80,192).with_native_executable_edit_trace();
    if arm==B48Arm::EditDisabled {w=w.with_native_patchnext_disabled();}
    w.load(p).unwrap();
    let mut row=B48Row{arm,phase_shift,seed,raw_correct:[0;3],
        raw_wrong:[0;3],qualified_correct:[0;3],qualified_wrong:[0;3],
        paid:0,edits:0,suppressed:0,stopped:false,max_storage:w.occupied_units()};
    let mut restored=false;
    for (window,input) in inputs.into_iter().enumerate() {
        w.intervene(Intervention::SetCell{key:7,value:input}).unwrap();
        for _ in 0..48 {
            if w.process(1).unwrap().stopped {break;}
            let before=w.total_compute_inventory().unwrap();
            assert!(w.step());
            row.paid+=1;
            assert_eq!(w.total_compute_inventory(),Some(before-1));
            row.max_storage=row.max_storage.max(w.occupied_units());
            let e=w.events().last().unwrap();
            assert_eq!(e.origin,Origin::Internal(1));
            assert!(!matches!(e.kind,EventKind::Trapped(_)));
            match &e.kind {
                EventKind::ExecutableEdgePatched{..}=>row.edits+=1,
                EventKind::NativeCodeEditSuppressed{..}=>row.suppressed+=1,
                EventKind::CellWritten{key:7,..} if e.pc==33=>restored=true,
                EventKind::CellWritten{key:11,value}=>{
                    // Score against the actual current world input, not merely
                    // the externally injected value: probe writes alter cell7.
                    let actual=w.cell(7).expect("input exists at output event");
                    if *value==actual {row.raw_correct[window]+=1;}
                    else {row.raw_wrong[window]+=1;}
                    if arm==B48Arm::Compact || window>0 || restored {
                        if *value==actual {row.qualified_correct[window]+=1;}
                        else {row.qualified_wrong[window]+=1;}
                    }
                },
                _=>{}
            }
        }
    }
    row.stopped=w.process(1).unwrap().stopped;
    assert!(row.paid<=144 && row.max_storage<=192);
    eprintln!("B48_NATIVE {:?}",row);
    row
}
#[test]
fn b48_feedback_semantics_phase_cross_over_prospective_screen() {
    const OLD:[[i64;3];8]=[
        [0,17,-9],[-1,23,0],[17,17,-9],[1,0,23],
        [23,-9,1],[-9,1,17],[0,0,23],[17,-1,0]
    ];
    const NEW:[[i64;3];8]=[
        [23,0,-1],[-9,17,23],[1,-9,0],[0,23,17],
        [17,-9,-9],[-1,1,23],[23,23,0],[-9,0,1]
    ];
    const ARMS48:[B48Arm;5]=[
        B48Arm::True,B48Arm::BlindSelf,B48Arm::BlindZero,
        B48Arm::EditDisabled,B48Arm::Compact
    ];
    // [block old/new][phase 0/1][arm][holdout window 1/2]
    // each cell = (raw correct, raw wrong, qualified correct, qualified wrong).
    let mut totals=[[[[[0usize;4];2];5];2];2];
    let mut paired=[[[[0usize;3];2];2];2]; // win/loss/tie vs two shams
    for (block,sequences) in [OLD,NEW].into_iter().enumerate() {
        for (seed,inputs) in sequences.into_iter().enumerate() {
            for shift in [false,true] {
                let ph=usize::from(shift);
                let rows:Vec<_>=ARMS48.iter().map(|&a|b48_run(a,shift,seed,inputs)).collect();
                for (a,r) in rows.iter().enumerate() {
                    assert_eq!(r.arm,ARMS48[a]);
                    assert_eq!(r.seed,seed);
                    for h in 0..2 {
                        let w=h+1;
                        for (j,value) in [
                            r.raw_correct[w],r.raw_wrong[w],
                            r.qualified_correct[w],r.qualified_wrong[w]
                        ].into_iter().enumerate() {totals[block][ph][a][h][j]+=value;}
                    }
                }
                for sham in 0..2 {
                    for h in 0..2 {
                        let w=h+1;
                        let net=|r:&B48Row|r.raw_correct[w] as i64-r.raw_wrong[w] as i64;
                        let a=net(&rows[0]);let b=net(&rows[sham+1]);
                        paired[block][ph][sham][if a>b{0}else if a<b{1}else{2}]+=1;
                    }
                }
            }
        }
    }
    eprintln!("B48_AGGREGATE {:?}",totals);
    eprintln!("B48_PAIRED {:?}",paired);
    // The test verifies event accounting and prints a falsifiable outcome;
    // no expected-positive conclusion is hardcoded into the assertion.
    for block in 0..2 {for phase in 0..2 {
        for arm in 0..5 {for h in 0..2 {
            assert!(totals[block][phase][arm][h][2]<=totals[block][phase][arm][h][0]);
            assert!(totals[block][phase][arm][h][3]<=totals[block][phase][arm][h][1]);
        }}
    }}
}
