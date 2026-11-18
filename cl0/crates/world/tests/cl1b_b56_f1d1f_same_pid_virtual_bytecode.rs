//! B56-F1d1f: original PID1 pays for a TWO-OPCODE data-program switch.
//! New researcher-authored founder; N56/B3/B4/cyberlife NEVER inferred.
use cl0_world::{EventKind,Intervention,Op,Origin,Process,World};
use serde_json::{json,Value};
use std::fs::{self,File};
use std::io::{BufWriter,Write};
const CREDITS:u64=250;
const INITIAL:[i64;2]=[5,7];
const HELD:[i64;2]=[11,-7];
const KEYS:[i64;5]=[7,11,20,30,31];

fn founder()->Vec<Op>{
    vec![
        Op::Const{dst:0,value:0,next:1},        // 0 r0=0
        Op::Const{dst:2,value:30,next:2},       // 1 mode cell key
        Op::Write{address:2,value:0,next:3},   // 2 initialize virtual opcode0
        Op::Const{dst:2,value:31,next:4},       // 3 stage cell key
        Op::Write{address:2,value:0,next:5},   // 4 initialize stage0
        Op::Const{dst:2,value:7,next:6},        // 5 input cell key
        Op::Read{dst:1,address:2,next:7},       // 6 r1=input
        Op::Const{dst:2,value:30,next:8},       // 7 mode address
        Op::Read{dst:3,address:2,next:9},       // 8 virtual opcode from data
        Op::Branch{test:3,nonzero:10,zero:13},  // 9 dispatch mode1 NEG vs mode0 ID
        Op::Sub{dst:1,lhs:0,rhs:1,next:11},     // 10 NEG (0-input)
        Op::Const{dst:2,value:11,next:12},      // 11 output address
        Op::Write{address:2,value:1,next:16},  // 12 NEG service
        Op::Const{dst:2,value:11,next:14},      // 13 ID output address
        Op::Write{address:2,value:1,next:16},  // 14 ID service
        Op::Halt,                               // 15 unreachable
        Op::Const{dst:2,value:31,next:17},      // 16 stage address
        Op::Read{dst:3,address:2,next:18},      // 17 stage flag
        Op::Branch{test:3,nonzero:33,zero:19},  // 18 stop after second service
        Op::Const{dst:2,value:20,next:20},      // 19 feedback address
        Op::Read{dst:3,address:2,next:21},      // 20 paid feedback Read
        Op::Branch{test:3,nonzero:22,zero:25},  // 21 1 keep,0 update
        Op::Const{dst:2,value:31,next:23},      // 22 stage address
        Op::Const{dst:3,value:1,next:24},       // 23 stage value1
        Op::Write{address:2,value:3,next:29},  // 24 mark second trial
        Op::Const{dst:2,value:30,next:26},      // 25 virtual program key
        Op::Const{dst:3,value:1,next:27},       // 26 opcode1 NEG
        Op::Write{address:2,value:3,next:22},  // 27 PAID update to virtual opcode1
        Op::Halt,                               // 28 unreachable
        Op::Const{dst:2,value:7,next:30},       // 29 input key for next trial
        Op::Read{dst:1,address:2,next:7},       // 30 new input
        Op::Halt,                               // 31 unreachable
        Op::Halt,                               // 32 unreachable
        Op::Halt,                               // 33 terminal
    ]
}
#[derive(Copy,Clone)]
enum Task{Identity,Negate}
impl Task{
    fn name(self)->&'static str{match self{Self::Identity=>"IDENTITY",Self::Negate=>"NEGATE"}}
    fn wanted(self,x:i64)->i64{match self{Self::Identity=>x,Self::Negate=>-x}}
}
#[derive(Copy,Clone)]
enum Signal{Truth,BlindSuccess,Inverted}
impl Signal{
    fn name(self)->&'static str{match self{
        Self::Truth=>"TRUTH",Self::BlindSuccess=>"BLIND_SUCCESS",Self::Inverted=>"INVERTED"}}
    fn feedback(self,truth:i64)->i64{match self{
        Self::Truth=>truth,Self::BlindSuccess=>1,Self::Inverted=>1-truth
    }}
}
fn emit(out:&mut BufWriter<File>,r:Value){
    serde_json::to_writer(&mut *out,&r).unwrap();
    out.write_all(b"\n").unwrap();
}
fn snapshot(w:&World)->Value{
    let p=w.process(1).unwrap();
    json!([p.pc,p.registers,p.credits,p.stopped,p.code.nodes])
}
fn cells(w:&World)->Value{json!(KEYS.map(|key|w.cell(key)))}
fn ext(out:&mut BufWriter<File>,w:&mut World,id:usize,key:i64,value:i64,label:&str){
    let before=w.occupied_units();
    w.intervene(Intervention::SetCell{key,value}).unwrap();
    let e=w.events().last().unwrap();
    assert!(matches!(e.origin,Origin::External));
    assert!(w.occupied_units()<=300&&w.used_cells()<=16);
    emit(out,json!(["ext",id,e.tick,key,value,label,before,
        snapshot(w),w.occupied_units(),cells(w)]));
}
fn case(out:&mut BufWriter<File>,id:usize,seed:usize,task:Task,signal:Signal)->Value{
    let code=founder();
    assert_eq!(code.len(),34);
    let mut w=World::with_strict_capacity(16,180,300);
    w.load(Process::new(1,code.clone(),CREDITS)).unwrap();
    assert!(w.process(2).is_none());
    emit(out,json!(["case",id,seed,task.name(),signal.name(),INITIAL[seed],
        HELD[seed],CREDITS,16,180,300,code,w.occupied_units()]));
    ext(out,&mut w,id,7,INITIAL[seed],"initial_environment_input");
    let mut steps=0_usize;
    let mut outputs:Vec<i64>=Vec::new();
    let mut charged_feedback_reads=0;
    let mut mode_updates=0;
    let mut first_truth=0;
    let mut delivered_feedback=0;
    let mut peak=w.occupied_units();
    while w.step(){
        steps+=1;
        assert!(steps<=160,"one-PID data interpreter failed to stop");
        let e=w.events().last().unwrap().clone();
        assert!(matches!(e.origin,Origin::Internal(1)));
        assert!(w.process(2).is_none(),"virtual program may not spawn any PID");
        assert_eq!(w.total_compute_inventory(),Some(CREDITS-steps as u64));
        assert_eq!(w.process(1).unwrap().code.nodes,code,"native opcode source must be immutable");
        assert!(w.used_cells()<=16);
        peak=peak.max(w.occupied_units());
        assert!(peak<=300);
        if let EventKind::CellRead{key:20,..}=e.kind{charged_feedback_reads+=1;}
        if let EventKind::CellWritten{key:30,value:1}=e.kind{
            assert_eq!(e.pc,27,"only original PID1 paid virtual opcode mutation");
            mode_updates+=1;
        }
        let mut first_output=false;
        if let EventKind::CellWritten{key:11,value}=e.kind{
            outputs.push(value);
            assert!(outputs.len()<=2);
            first_output=outputs.len()==1;
        }
        emit(out,json!(["step",id,steps,e.tick,1,e.pc,e.kind,
            snapshot(&w),w.occupied_units(),cells(&w)]));
        if first_output{
            first_truth=i64::from(outputs[0]==task.wanted(INITIAL[seed]));
            delivered_feedback=signal.feedback(first_truth);
            ext(out,&mut w,id,20,delivered_feedback,"feedback_after_first_real_output");
            ext(out,&mut w,id,7,HELD[seed],"heldout_environment_input");
        }
    }
    assert_eq!(outputs.len(),2,"two actually executed CIL service Writes expected");
    assert_eq!(charged_feedback_reads,1,"one real CIL feedback Read");
    assert_eq!(mode_updates,usize::from(delivered_feedback==0));
    assert_eq!(w.cell(30),Some(i64::from(delivered_feedback==0)));
    assert_eq!(w.cell(31),Some(1));
    assert!(w.process(1).unwrap().stopped);
    assert!(w.process(2).is_none());
    let second_correct=outputs[1]==task.wanted(HELD[seed]);
    let expected_second=if delivered_feedback==0{-HELD[seed]}else{HELD[seed]};
    assert_eq!(outputs[1],expected_second);
    if matches!(signal,Signal::Truth){assert!(second_correct)}
    if matches!(signal,Signal::Inverted){assert!(!second_correct)}
    if matches!(signal,Signal::BlindSuccess)&&matches!(task,Task::Negate){assert!(!second_correct)}
    let result=json!({
        "id":id,"seed":seed,"task":task.name(),"signal":signal.name(),
        "first_output":outputs[0],"second_output":outputs[1],
        "first_truth":first_truth,"external_feedback":delivered_feedback,
        "second_correct":second_correct,"paid_feedback_reads":charged_feedback_reads,
        "actor_paid_virtual_opcode_updates":mode_updates,
        "final_virtual_opcode":w.cell(30),"final_stage":w.cell(31),
        "charged_steps":steps,"peak_total_storage":peak,"final_credits":w.remaining_credits(),
        "final_process":snapshot(&w),"native_source_changed":false,
        "exact_one_PID":true,"new_algorithm_created":false,"b3_met":false,
        "b4_met":false,"cyberlife":false
    });
    emit(out,json!(["end",id,result]));
    result
}
#[test]
fn b56_f1d1f_single_pid_paid_virtual_program_feedback_revision(){
    let folder=std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../cl1b-ci-evidence");
    fs::create_dir_all(&folder).unwrap();
    let mut out=BufWriter::new(File::create(
        folder.join("b56_f1d1f_same_pid_virtual_opcode_steps.jsonl")).unwrap());
    let manifest=std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(
        "../../../cl1b/studies/B56-F1d1f-paid-same-PID-data-bytecode-feedback-switch-manifest-v1.md");
    let b=fs::read(&manifest).unwrap();
    let h=std::process::Command::new("sha256sum").arg(&manifest).output().unwrap();
    assert!(h.status.success());
    let hash=String::from_utf8(h.stdout).unwrap()
        .split_whitespace().next().unwrap().to_owned();
    emit(&mut out,json!(["meta","B56-F1d1f",1,
        std::env::var("GITHUB_SHA").unwrap_or_else(|_|"local".to_owned()),
        hash,b.len(),12,CREDITS,16,180,300]));
    let mut rows=Vec::new();
    for seed in 0..INITIAL.len(){
        for task in [Task::Identity,Task::Negate]{
            for signal in [Signal::Truth,Signal::BlindSuccess,Signal::Inverted]{
                let id=rows.len();rows.push(case(&mut out,id,seed,task,signal));
            }
        }
    }
    out.flush().unwrap();
    assert_eq!(rows.len(),12);
    let true_rows:Vec<_>=rows.iter().filter(|r|r["signal"]=="TRUTH").collect();
    let blind_rows:Vec<_>=rows.iter().filter(|r|r["signal"]=="BLIND_SUCCESS").collect();
    let invert_rows:Vec<_>=rows.iter().filter(|r|r["signal"]=="INVERTED").collect();
    assert_eq!(true_rows.iter().filter(|r|r["second_correct"]==true).count(),4);
    assert_eq!(blind_rows.iter().filter(|r|r["second_correct"]==true).count(),2);
    assert_eq!(invert_rows.iter().filter(|r|r["second_correct"]==true).count(),0);
    eprintln!("B56_F1D1F_PAID_SAME_PID_DATA_INTERPRETER {}",json!({
       "isolated_worlds":12,"true_heldout_correct":4,
       "blind_heldout_correct":2,"inverted_heldout_correct":0,
       "native_source_modifications":0,"PID2_existed":false,
       "generic_algorithm_invention":false,"N56_A_B_C":false,
       "B3":false,"B4":false,"cyberlife":false
    }));
}
