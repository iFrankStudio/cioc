//! B56-F1d1g: a finite, researcher-authored virtual ISA, with PID1-paid
//! two-slot data-program revision. No claim of native-opcode invention or CL life.
use cl0_world::{EventKind,Intervention,Op,Origin,Process,World};
use serde_json::{json,Value};
use std::fs::{self,File};
use std::io::{BufWriter,Write};

const CREDITS:u64=1000;
const INIT:[i64;2]=[5,7];
const HELD:[i64;2]=[11,-7];
const KEYS:[i64;13]=[7,11,20,21,31,50,51,52,53,54,55,56,60];
const FETCH_OPCODE_PC:usize=26;
const FEEDBACK_READ_PC:usize=54;
const SERVICE_WRITE_PC:usize=49;
fn founder()->Vec<Op>{
    vec![
        Op::Const{dst:0,value:0,next:1}, // pc 0
        Op::Const{dst:2,value:50,next:2}, // pc 1
        Op::Const{dst:3,value:0,next:3}, // pc 2
        Op::Write{address:2,value:3,next:4}, // pc 3
        Op::Const{dst:2,value:51,next:5}, // pc 4
        Op::Const{dst:3,value:0,next:6}, // pc 5
        Op::Write{address:2,value:3,next:7}, // pc 6
        Op::Const{dst:2,value:52,next:8}, // pc 7
        Op::Const{dst:3,value:3,next:9}, // pc 8
        Op::Write{address:2,value:3,next:10}, // pc 9
        Op::Const{dst:2,value:53,next:11}, // pc 10
        Op::Const{dst:3,value:3,next:12}, // pc 11
        Op::Write{address:2,value:3,next:13}, // pc 12
        Op::Const{dst:2,value:54,next:14}, // pc 13
        Op::Const{dst:3,value:3,next:15}, // pc 14
        Op::Write{address:2,value:3,next:16}, // pc 15
        Op::Const{dst:2,value:55,next:17}, // pc 16
        Op::Const{dst:3,value:2,next:18}, // pc 17
        Op::Write{address:2,value:3,next:19}, // pc 18
        Op::Const{dst:2,value:31,next:20}, // pc 19
        Op::Const{dst:3,value:0,next:21}, // pc 20
        Op::Write{address:2,value:3,next:22}, // pc 21
        Op::Const{dst:2,value:50,next:23}, // pc 22
        Op::Read{dst:3,address:2,next:24}, // pc 23
        Op::Const{dst:2,value:51,next:25}, // pc 24
        Op::Add{dst:2,lhs:2,rhs:3,next:26}, // pc 25
        Op::Read{dst:3,address:2,next:27}, // pc 26
        Op::Branch{test:3,nonzero:28,zero:33}, // pc 27
        Op::Const{dst:2,value:1,next:29}, // pc 28
        Op::Sub{dst:3,lhs:3,rhs:2,next:30}, // pc 29
        Op::Branch{test:3,nonzero:31,zero:35}, // pc 30
        Op::Sub{dst:3,lhs:3,rhs:2,next:32}, // pc 31
        Op::Branch{test:3,nonzero:48,zero:37}, // pc 32
        Op::Const{dst:2,value:7,next:34}, // pc 33
        Op::Read{dst:1,address:2,next:40}, // pc 34
        Op::Const{dst:2,value:0,next:36}, // pc 35
        Op::Sub{dst:1,lhs:2,rhs:1,next:40}, // pc 36
        Op::Const{dst:2,value:55,next:38}, // pc 37
        Op::Read{dst:3,address:2,next:39}, // pc 38
        Op::Add{dst:1,lhs:1,rhs:3,next:40}, // pc 39
        Op::Const{dst:2,value:56,next:41}, // pc 40
        Op::Write{address:2,value:1,next:42}, // pc 41
        Op::Const{dst:2,value:50,next:43}, // pc 42
        Op::Read{dst:3,address:2,next:44}, // pc 43
        Op::Const{dst:2,value:1,next:45}, // pc 44
        Op::Add{dst:3,lhs:3,rhs:2,next:46}, // pc 45
        Op::Const{dst:2,value:50,next:47}, // pc 46
        Op::Write{address:2,value:3,next:22}, // pc 47
        Op::Const{dst:2,value:11,next:49}, // pc 48
        Op::Write{address:2,value:1,next:50}, // pc 49
        Op::Const{dst:2,value:31,next:51}, // pc 50
        Op::Read{dst:3,address:2,next:52}, // pc 51
        Op::Branch{test:3,nonzero:76,zero:53}, // pc 52
        Op::Const{dst:2,value:20,next:54}, // pc 53
        Op::Read{dst:3,address:2,next:55}, // pc 54
        Op::Branch{test:3,nonzero:71,zero:56}, // pc 55
        Op::Const{dst:2,value:21,next:57}, // pc 56
        Op::Read{dst:3,address:2,next:58}, // pc 57
        Op::Branch{test:3,nonzero:59,zero:65}, // pc 58
        Op::Const{dst:2,value:52,next:60}, // pc 59
        Op::Const{dst:3,value:1,next:61}, // pc 60
        Op::Write{address:2,value:3,next:62}, // pc 61
        Op::Const{dst:2,value:53,next:63}, // pc 62
        Op::Const{dst:3,value:2,next:64}, // pc 63
        Op::Write{address:2,value:3,next:71}, // pc 64
        Op::Const{dst:2,value:60,next:66}, // pc 65
        Op::Const{dst:3,value:1,next:67}, // pc 66
        Op::Write{address:2,value:3,next:68}, // pc 67
        Op::Const{dst:2,value:60,next:69}, // pc 68
        Op::Const{dst:3,value:2,next:70}, // pc 69
        Op::Write{address:2,value:3,next:71}, // pc 70
        Op::Const{dst:2,value:31,next:72}, // pc 71
        Op::Const{dst:3,value:1,next:73}, // pc 72
        Op::Write{address:2,value:3,next:74}, // pc 73
        Op::Const{dst:2,value:50,next:75}, // pc 74
        Op::Write{address:2,value:0,next:22}, // pc 75
        Op::Halt, // pc 76
    ]
}
#[derive(Copy,Clone)]
enum Task{Identity,Composed}
impl Task{
    fn name(self)->&'static str{match self{Self::Identity=>"IDENTITY",Self::Composed=>"COMPOSED"}}
    fn value(self,x:i64)->i64{match self{Self::Identity=>x,Self::Composed=>-x+2}}
}
#[derive(Copy,Clone)]
enum Feedback{Truth,BlindSuccess,Inverted}
impl Feedback{
    fn name(self)->&'static str{match self{
        Self::Truth=>"TRUTH",Self::BlindSuccess=>"BLIND_SUCCESS",Self::Inverted=>"INVERTED"
    }}
    fn bit(self,truth:i64)->i64{match self{
        Self::Truth=>truth,Self::BlindSuccess=>1,Self::Inverted=>1-truth
    }}
}
fn emit(out:&mut BufWriter<File>,row:Value){
    serde_json::to_writer(&mut *out,&row).unwrap();
    out.write_all(b"\n").unwrap();
}
fn snapshot(w:&World)->Value{
    let p=w.process(1).unwrap();
    json!([p.pc,p.registers,p.credits,p.stopped,p.code.nodes])
}
fn cells(w:&World)->Value{json!(KEYS.map(|key|w.cell(key)))}
fn ext(out:&mut BufWriter<File>,w:&mut World,id:usize,key:i64,value:i64,reason:&str){
    let before=w.occupied_units();
    w.intervene(Intervention::SetCell{key,value}).unwrap();
    let e=w.events().last().unwrap();
    assert!(matches!(e.origin,Origin::External));
    assert!(w.used_cells()<=16&&w.occupied_units()<=300);
    emit(out,json!(["ext",id,e.tick,key,value,reason,before,
        snapshot(w),w.occupied_units(),cells(w)]));
}
fn run_case(
    out:&mut BufWriter<File>,id:usize,seed:usize,task:Task,feedback:Feedback,
    edit_enabled:bool,
)->Value{
    let code=founder();
    assert_eq!(code.len(),77);
    let mut w=World::with_strict_capacity(16,180,300);
    w.load(Process::new(1,code.clone(),CREDITS)).unwrap();
    emit(out,json!(["case",id,seed,task.name(),feedback.name(),edit_enabled,
        INIT[seed],HELD[seed],CREDITS,16,180,300,code,w.occupied_units()]));
    ext(out,&mut w,id,21,i64::from(edit_enabled),"authorized_edit_control");
    ext(out,&mut w,id,7,INIT[seed],"first_environment_input");
    let mut steps=0_usize;
    let mut outputs=Vec::new();
    let mut program_edit_events=0_usize;
    let mut shadow_events=0_usize;
    let mut opcode_fetches=Vec::new();
    let mut feedback_reads=0_usize;
    let mut accumulator_writes=Vec::new();
    let mut first_truth=0_i64;
    let mut delivered=0_i64;
    let mut peak=w.occupied_units();
    while w.step(){
        steps+=1;
        assert!(steps<=500,"interpreter did not stop under finite paid budget");
        let e=w.events().last().unwrap().clone();
        assert!(matches!(e.origin,Origin::Internal(1)));
        assert!(w.process(2).is_none(),"new PID not permitted in single-PID lineage test");
        assert_eq!(w.total_compute_inventory(),Some(CREDITS-steps as u64));
        assert_eq!(w.process(1).unwrap().code.nodes,code,
            "same native PID code must never be rewritten");
        assert!(w.used_cells()<=16&&w.occupied_units()<=300);
        peak=peak.max(w.occupied_units());
        if e.pc==FETCH_OPCODE_PC{
            let virtual_pc=w.cell(50).unwrap();
            let key=51+virtual_pc;
            assert!(virtual_pc>=0&&virtual_pc<=3);
            let expected=w.cell(key).unwrap();
            assert!(matches!(e.kind,EventKind::CellRead{key:k,value:v}
                if k==key&&v==expected));
            opcode_fetches.push(expected);
        }
        if e.pc==FEEDBACK_READ_PC{
            assert!(matches!(e.kind,EventKind::CellRead{key:20,..}));
            feedback_reads+=1;
        }
        if let EventKind::CellWritten{key,value}=&e.kind{
            if (*key==52&&*value==1)||(*key==53&&*value==2){
                assert!(edit_enabled,"disabled program editing cannot write real slots");
                program_edit_events+=1;
            }
            if *key==60{assert!(!edit_enabled);shadow_events+=1;}
            if *key==56{accumulator_writes.push(*value);}
            if *key==11{
                assert_eq!(e.pc,SERVICE_WRITE_PC);
                outputs.push(*value);
                assert!(outputs.len()<=2);
            }
        }
        emit(out,json!(["step",id,steps,e.tick,1,e.pc,e.kind,
            snapshot(&w),w.occupied_units(),cells(&w)]));
        if outputs.len()==1 && w.cell(20).is_none(){
            first_truth=i64::from(outputs[0]==task.value(INIT[seed]));
            delivered=feedback.bit(first_truth);
            ext(out,&mut w,id,20,delivered,"feedback_after_first_paid_return");
            ext(out,&mut w,id,7,HELD[seed],"heldout_input_after_real_service");
        }
    }
    assert_eq!(outputs.len(),2,"two paid native RETURN Write11 events required");
    assert_eq!(feedback_reads,1);
    assert_eq!(opcode_fetches.len(),
        if edit_enabled&&delivered==0 {6}else{4});
    assert_eq!(&opcode_fetches[0..2],&[0,3]);
    let composed=edit_enabled&&delivered==0;
    if composed{
        assert_eq!(&opcode_fetches[2..],&[0,1,2,3]);
        assert_eq!(&accumulator_writes[1..],&[HELD[seed],-HELD[seed],-HELD[seed]+2]);
        assert_eq!(w.cell(52),Some(1));
        assert_eq!(w.cell(53),Some(2));
        assert_eq!(program_edit_events,2);
        assert_eq!(shadow_events,0);
    }else{
        assert_eq!(&opcode_fetches[2..],&[0,3]);
        assert_eq!(&accumulator_writes[1..],&[HELD[seed]]);
        assert_eq!(w.cell(52),Some(3));
        assert_eq!(w.cell(53),Some(3));
        assert_eq!(program_edit_events,0);
        assert_eq!(shadow_events,if !edit_enabled&&delivered==0{2}else{0});
    }
    assert_eq!(w.cell(54),Some(3));
    assert_eq!(w.cell(55),Some(2));
    assert_eq!(w.cell(31),Some(1));
    assert!(w.process(2).is_none()&&w.process(1).unwrap().stopped);
    assert_eq!(accumulator_writes[0],INIT[seed]);
    let second_correct=outputs[1]==task.value(HELD[seed]);
    if edit_enabled{
        match feedback{
            Feedback::Truth=>assert!(second_correct),
            Feedback::Inverted=>assert!(!second_correct),
            Feedback::BlindSuccess=>assert_eq!(second_correct,matches!(task,Task::Identity)),
        }
    }else{
        assert_eq!(second_correct,matches!(task,Task::Identity));
    }
    let result=json!({
        "id":id,"seed":seed,"task":task.name(),"feedback_regime":feedback.name(),
        "edit_enabled":edit_enabled,"first_output":outputs[0],"second_output":outputs[1],
        "first_truth":first_truth,"delivered_feedback":delivered,
        "heldout_correct":second_correct,"actual_virtual_opcode_fetches":opcode_fetches,
        "actual_accumulator_writes":accumulator_writes,
        "paid_feedback_reads":feedback_reads,"real_program_slot_edits":program_edit_events,
        "shadow_write_events":shadow_events,
        "final_program_slots":[w.cell(51),w.cell(52),w.cell(53),w.cell(54)],
        "charged_steps":steps,"max_storage":peak,"remaining_credits":w.remaining_credits(),
        "final_process":snapshot(&w),"same_PID1":true,"native_source_unchanged":true,
        "self_generated_opcode_semantics":false,"b3_met":false,"b4_met":false,"cyberlife":false
    });
    emit(out,json!(["end",id,result]));
    result
}
#[test]
fn b56_f1d1g_same_pid_paid_multi_instruction_data_program_composition(){
    let folder=std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../cl1b-ci-evidence");
    fs::create_dir_all(&folder).unwrap();
    let mut out=BufWriter::new(File::create(folder.join(
        "b56_f1d1g_same_pid_multi_bytecode_native_steps.jsonl")).unwrap());
    let manifest=std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(
        "../../../cl1b/studies/B56-F1d1g-multi-op-bytecode-exact-layout-and-edit-suppression-v1.md");
    let bytes=fs::read(&manifest).unwrap();
    let digest=std::process::Command::new("sha256sum").arg(&manifest).output().unwrap();
    assert!(digest.status.success());
    let hash=String::from_utf8(digest.stdout).unwrap()
        .split_whitespace().next().unwrap().to_owned();
    emit(&mut out,json!(["meta","B56-F1d1g",1,
        std::env::var("GITHUB_SHA").unwrap_or_else(|_|"local".to_owned()),
        hash,bytes.len(),24,CREDITS,16,180,300,
        FETCH_OPCODE_PC,FEEDBACK_READ_PC,SERVICE_WRITE_PC]));
    let mut rows=Vec::new();
    for seed in 0..2{
        for task in [Task::Identity,Task::Composed]{
            for feedback in [Feedback::Truth,Feedback::BlindSuccess,Feedback::Inverted]{
                for edit_enabled in [true,false]{
                    let id=rows.len();
                    rows.push(run_case(&mut out,id,seed,task,feedback,edit_enabled));
                }
            }
        }
    }
    out.flush().unwrap();
    assert_eq!(rows.len(),24);
    let accuracy=|regime:&str,edit:bool|rows.iter()
      .filter(|r|r["feedback_regime"]==regime&&r["edit_enabled"]==edit&&r["heldout_correct"]==true)
      .count();
    assert_eq!(accuracy("TRUTH",true),4);
    assert_eq!(accuracy("BLIND_SUCCESS",true),2);
    assert_eq!(accuracy("INVERTED",true),0);
    for regime in ["TRUTH","BLIND_SUCCESS","INVERTED"]{
        assert_eq!(accuracy(regime,false),2);
    }
    eprintln!("B56_F1D1G_SAME_PID_COMPOSABLE_BYTECODE {}",json!({
        "real_native_worlds":24,
        "truth_enabled_correct":4,"blind_enabled_correct":2,"inverted_enabled_correct":0,
        "all_suppressed_feedback_regimes_correct_per_four":2,
        "same_native_PID":1,"native_source_changed":false,
        "multi_data_program_slot_edit":true,
        "new_primitive_invention":false,"N56_A_B_C":false,
        "B3":false,"B4":false,"cyberlife":false
    }));
}
