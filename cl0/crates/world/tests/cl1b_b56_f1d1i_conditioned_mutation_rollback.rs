//! CL-1B B56-F1d1i — researcher-authored SAME PID native paid
//! 3-phase feedback-conditioned mutation and source-level prior-best rollback.
//! Exact native instruction graph was committed BEFORE this file.
use cl0_world::{EventKind,Intervention,Op,Origin,Process,World};
use serde_json::{json,Value};
use std::fs::{self,File};
use std::io::{BufWriter,Write};

const CREDITS:u64=4000;
const KEYS:[i64;16]=[7,11,20,21,31,32,33,50,51,52,53,54,56,61,62,63];
const PROBES:[i64;2]=[-7,5];
const HELDOUT:i64=11;
const FETCH_PC:usize=30;
const SERVICE_PC:usize=52;
const FEEDBACK_PC:usize=59;
const SCORE_WRITE_PC:usize=65;
const REAL_EDIT_52_PC:usize=123;
const REAL_EDIT_53_PC:usize=125;
const SHADOW_1_PC:usize=127;
const SHADOW_2_PC:usize=129;
const ROLLBACK_PC:usize=93;
const HELDOUT_STAGE_PC:usize=141;
fn founder()->Vec<Op>{
    vec![
        Op::Const{dst:0,value:0,next:1}, // pc 0
        Op::Const{dst:2,value:31,next:2}, // pc 1
        Op::Write{address:2,value:0,next:3}, // pc 2
        Op::Const{dst:2,value:32,next:4}, // pc 3
        Op::Write{address:2,value:0,next:5}, // pc 4
        Op::Const{dst:2,value:33,next:6}, // pc 5
        Op::Write{address:2,value:0,next:7}, // pc 6
        Op::Const{dst:2,value:50,next:8}, // pc 7
        Op::Write{address:2,value:0,next:9}, // pc 8
        Op::Const{dst:2,value:51,next:10}, // pc 9
        Op::Write{address:2,value:0,next:11}, // pc 10
        Op::Const{dst:2,value:61,next:12}, // pc 11
        Op::Write{address:2,value:0,next:13}, // pc 12
        Op::Const{dst:2,value:62,next:14}, // pc 13
        Op::Write{address:2,value:0,next:15}, // pc 14
        Op::Const{dst:2,value:63,next:16}, // pc 15
        Op::Write{address:2,value:0,next:17}, // pc 16
        Op::Const{dst:2,value:52,next:18}, // pc 17
        Op::Const{dst:3,value:3,next:19}, // pc 18
        Op::Write{address:2,value:3,next:20}, // pc 19
        Op::Const{dst:2,value:53,next:21}, // pc 20
        Op::Const{dst:3,value:3,next:22}, // pc 21
        Op::Write{address:2,value:3,next:23}, // pc 22
        Op::Const{dst:2,value:54,next:24}, // pc 23
        Op::Const{dst:3,value:3,next:25}, // pc 24
        Op::Write{address:2,value:3,next:26}, // pc 25
        Op::Const{dst:2,value:50,next:27}, // pc 26
        Op::Read{dst:3,address:2,next:28}, // pc 27
        Op::Const{dst:2,value:51,next:29}, // pc 28
        Op::Add{dst:2,lhs:2,rhs:3,next:30}, // pc 29
        Op::Read{dst:3,address:2,next:31}, // pc 30
        Op::Branch{test:3,nonzero:32,zero:37}, // pc 31
        Op::Const{dst:2,value:1,next:33}, // pc 32
        Op::Sub{dst:3,lhs:3,rhs:2,next:34}, // pc 33
        Op::Branch{test:3,nonzero:35,zero:39}, // pc 34
        Op::Sub{dst:3,lhs:3,rhs:2,next:36}, // pc 35
        Op::Branch{test:3,nonzero:51,zero:41}, // pc 36
        Op::Const{dst:2,value:7,next:38}, // pc 37
        Op::Read{dst:1,address:2,next:43}, // pc 38
        Op::Const{dst:2,value:0,next:40}, // pc 39
        Op::Sub{dst:1,lhs:2,rhs:1,next:43}, // pc 40
        Op::Const{dst:3,value:2,next:42}, // pc 41
        Op::Add{dst:1,lhs:1,rhs:3,next:43}, // pc 42
        Op::Const{dst:2,value:56,next:44}, // pc 43
        Op::Write{address:2,value:1,next:45}, // pc 44
        Op::Const{dst:2,value:50,next:46}, // pc 45
        Op::Read{dst:3,address:2,next:47}, // pc 46
        Op::Const{dst:2,value:1,next:48}, // pc 47
        Op::Add{dst:3,lhs:3,rhs:2,next:49}, // pc 48
        Op::Const{dst:2,value:50,next:50}, // pc 49
        Op::Write{address:2,value:3,next:26}, // pc 50
        Op::Const{dst:2,value:11,next:52}, // pc 51
        Op::Write{address:2,value:1,next:53}, // pc 52
        Op::Const{dst:2,value:32,next:54}, // pc 53
        Op::Read{dst:3,address:2,next:55}, // pc 54
        Op::Const{dst:2,value:2,next:56}, // pc 55
        Op::Sub{dst:3,lhs:3,rhs:2,next:57}, // pc 56
        Op::Branch{test:3,nonzero:58,zero:142}, // pc 57
        Op::Const{dst:2,value:20,next:59}, // pc 58
        Op::Read{dst:1,address:2,next:60}, // pc 59
        Op::Branch{test:1,nonzero:61,zero:66}, // pc 60
        Op::Const{dst:2,value:61,next:62}, // pc 61
        Op::Read{dst:3,address:2,next:63}, // pc 62
        Op::Const{dst:1,value:1,next:64}, // pc 63
        Op::Add{dst:3,lhs:3,rhs:1,next:65}, // pc 64
        Op::Write{address:2,value:3,next:66}, // pc 65
        Op::Const{dst:2,value:32,next:67}, // pc 66
        Op::Read{dst:3,address:2,next:68}, // pc 67
        Op::Const{dst:1,value:1,next:69}, // pc 68
        Op::Add{dst:3,lhs:3,rhs:1,next:70}, // pc 69
        Op::Write{address:2,value:3,next:71}, // pc 70
        Op::Const{dst:2,value:2,next:72}, // pc 71
        Op::Sub{dst:3,lhs:3,rhs:2,next:73}, // pc 72
        Op::Branch{test:3,nonzero:74,zero:76}, // pc 73
        Op::Const{dst:2,value:50,next:75}, // pc 74
        Op::Write{address:2,value:0,next:26}, // pc 75
        Op::Const{dst:2,value:61,next:77}, // pc 76
        Op::Read{dst:3,address:2,next:78}, // pc 77
        Op::Const{dst:2,value:2,next:79}, // pc 78
        Op::Sub{dst:3,lhs:3,rhs:2,next:80}, // pc 79
        Op::Branch{test:3,nonzero:85,zero:81}, // pc 80
        Op::Const{dst:2,value:33,next:82}, // pc 81
        Op::Read{dst:3,address:2,next:83}, // pc 82
        Op::Const{dst:2,value:62,next:84}, // pc 83
        Op::Write{address:2,value:3,next:139}, // pc 84
        Op::Const{dst:2,value:63,next:86}, // pc 85
        Op::Read{dst:3,address:2,next:87}, // pc 86
        Op::Const{dst:1,value:1,next:88}, // pc 87
        Op::Add{dst:3,lhs:3,rhs:1,next:89}, // pc 88
        Op::Write{address:2,value:3,next:90}, // pc 89
        Op::Const{dst:2,value:3,next:91}, // pc 90
        Op::Sub{dst:3,lhs:3,rhs:2,next:92}, // pc 91
        Op::Branch{test:3,nonzero:97,zero:93}, // pc 92
        Op::Const{dst:2,value:62,next:94}, // pc 93
        Op::Read{dst:3,address:2,next:95}, // pc 94
        Op::Const{dst:2,value:33,next:96}, // pc 95
        Op::Write{address:2,value:3,next:107}, // pc 96
        Op::Const{dst:2,value:33,next:98}, // pc 97
        Op::Read{dst:3,address:2,next:99}, // pc 98
        Op::Const{dst:1,value:1,next:100}, // pc 99
        Op::Add{dst:3,lhs:3,rhs:1,next:101}, // pc 100
        Op::Const{dst:2,value:3,next:102}, // pc 101
        Op::Sub{dst:1,lhs:3,rhs:2,next:103}, // pc 102
        Op::Branch{test:1,nonzero:105,zero:104}, // pc 103
        Op::Const{dst:3,value:0,next:105}, // pc 104
        Op::Const{dst:2,value:33,next:106}, // pc 105
        Op::Write{address:2,value:3,next:107}, // pc 106
        Op::Const{dst:2,value:33,next:108}, // pc 107
        Op::Read{dst:3,address:2,next:109}, // pc 108
        Op::Branch{test:3,nonzero:110,zero:113}, // pc 109
        Op::Const{dst:2,value:1,next:111}, // pc 110
        Op::Sub{dst:3,lhs:3,rhs:2,next:112}, // pc 111
        Op::Branch{test:3,nonzero:117,zero:115}, // pc 112
        Op::Const{dst:1,value:3,next:114}, // pc 113
        Op::Const{dst:3,value:3,next:119}, // pc 114
        Op::Const{dst:1,value:1,next:116}, // pc 115
        Op::Const{dst:3,value:2,next:119}, // pc 116
        Op::Const{dst:1,value:1,next:118}, // pc 117
        Op::Const{dst:3,value:3,next:119}, // pc 118
        Op::Const{dst:2,value:21,next:120}, // pc 119
        Op::Read{dst:2,address:2,next:121}, // pc 120
        Op::Branch{test:2,nonzero:122,zero:126}, // pc 121
        Op::Const{dst:2,value:52,next:123}, // pc 122
        Op::Write{address:2,value:1,next:124}, // pc 123
        Op::Const{dst:2,value:53,next:125}, // pc 124
        Op::Write{address:2,value:3,next:130}, // pc 125
        Op::Const{dst:2,value:56,next:127}, // pc 126
        Op::Write{address:2,value:1,next:128}, // pc 127
        Op::Const{dst:2,value:56,next:129}, // pc 128
        Op::Write{address:2,value:3,next:130}, // pc 129
        Op::Const{dst:2,value:63,next:131}, // pc 130
        Op::Read{dst:3,address:2,next:132}, // pc 131
        Op::Const{dst:2,value:3,next:133}, // pc 132
        Op::Sub{dst:3,lhs:3,rhs:2,next:134}, // pc 133
        Op::Branch{test:3,nonzero:135,zero:139}, // pc 134
        Op::Const{dst:2,value:32,next:136}, // pc 135
        Op::Write{address:2,value:0,next:137}, // pc 136
        Op::Const{dst:2,value:61,next:138}, // pc 137
        Op::Write{address:2,value:0,next:74}, // pc 138
        Op::Const{dst:2,value:32,next:140}, // pc 139
        Op::Const{dst:3,value:2,next:141}, // pc 140
        Op::Write{address:2,value:3,next:74}, // pc 141
        Op::Const{dst:2,value:31,next:143}, // pc 142
        Op::Read{dst:3,address:2,next:144}, // pc 143
        Op::Const{dst:1,value:1,next:145}, // pc 144
        Op::Add{dst:3,lhs:3,rhs:1,next:146}, // pc 145
        Op::Write{address:2,value:3,next:147}, // pc 146
        Op::Const{dst:2,value:3,next:148}, // pc 147
        Op::Sub{dst:3,lhs:3,rhs:2,next:149}, // pc 148
        Op::Branch{test:3,nonzero:150,zero:156}, // pc 149
        Op::Const{dst:2,value:32,next:151}, // pc 150
        Op::Write{address:2,value:0,next:152}, // pc 151
        Op::Const{dst:2,value:61,next:153}, // pc 152
        Op::Write{address:2,value:0,next:154}, // pc 153
        Op::Const{dst:2,value:63,next:155}, // pc 154
        Op::Write{address:2,value:0,next:74}, // pc 155
        Op::Halt, // pc 156
    ]
}
#[derive(Copy,Clone,Debug,PartialEq,Eq)]
enum Task{Identity,Negate,Composed,Unreachable}
impl Task{
    fn name(self)->&'static str{match self{
        Self::Identity=>"IDENTITY",Self::Negate=>"NEGATE",
        Self::Composed=>"COMPOSED",Self::Unreachable=>"UNREACHABLE"}}
    fn wanted(self,x:i64)->i64{match self{
        Self::Identity=>x,Self::Negate=>-x,
        Self::Composed=>-x+2,Self::Unreachable=>x+4}}
}
const SCHEDULES:[[Task;3];2]=[
    [Task::Identity,Task::Composed,Task::Identity],
    [Task::Composed,Task::Negate,Task::Unreachable]
];
#[derive(Copy,Clone)]
enum Signal{Truth,BlindSuccess,Inverted,Absent}
impl Signal{
    fn name(self)->&'static str{match self{
        Self::Truth=>"TRUTH",Self::BlindSuccess=>"BLIND_SUCCESS",
        Self::Inverted=>"INVERTED",Self::Absent=>"ABSENT"}}
    fn bit(self,correct:i64)->Option<i64>{match self{
        Self::Truth=>Some(correct),Self::BlindSuccess=>Some(1),
        Self::Inverted=>Some(1-correct),Self::Absent=>None}}
}
fn emit(out:&mut BufWriter<File>,row:Value){
    serde_json::to_writer(&mut *out,&row).unwrap();
    out.write_all(b"\n").unwrap();
}
fn fnv_code(code:&[Op])->String{
    let bytes=serde_json::to_vec(code).unwrap();
    let h=bytes.iter().fold(14695981039346656037_u64,|acc,byte|{
        (acc^u64::from(*byte)).wrapping_mul(1099511628211)
    });
    format!("{h:016x}")
}
fn process(w:&World,hash:&str)->Value{
    let p=w.process(1).unwrap();
    json!([p.pc,p.registers,p.credits,p.stopped,p.code.nodes.len(),hash])
}
fn cells(w:&World)->Value{json!(KEYS.map(|k|w.cell(k)))}
fn program(w:&World)->Value{json!([51,52,53,54].map(|k|w.cell(k)))}
fn ext(out:&mut BufWriter<File>,w:&mut World,id:usize,key:i64,value:i64,
    reason:&str,hash:&str){
    assert!(matches!(key,7|20|21),"host intervention outside input/feedback/gate");
    w.intervene(Intervention::SetCell{key,value}).unwrap();
    let e=w.events().last().unwrap();
    assert!(matches!(e.origin,Origin::External));
    assert!(w.used_cells()<=16&&w.occupied_units()<=300);
    emit(out,json!(["ext",id,e.tick,key,value,reason,
        process(w,hash),w.occupied_units(),cells(w)]));
}
fn case(out:&mut BufWriter<File>,id:usize,schedule_id:usize,
    signal:Signal,gate:i64)->Value{
    let founder=founder();
    assert_eq!(founder.len(),157);
    let hash=fnv_code(&founder);
    let schedule=SCHEDULES[schedule_id];
    let mut w=World::with_strict_capacity(16,180,300);
    w.load(Process::new(1,founder.clone(),CREDITS)).unwrap();
    emit(out,json!(["case",id,schedule_id,signal.name(),gate,
        schedule.map(|t|t.name()),founder,hash,w.occupied_units(),
        CREDITS,16,180,300]));
    ext(out,&mut w,id,21,gate,"single_external_edit_gate",&hash);
    ext(out,&mut w,id,7,PROBES[0],"first_phase_probe_0",&hash);
    let mut paid=0_usize;
    let mut services:Vec<Value>=Vec::new();
    let mut probe_feedback_reads=0_usize;
    let mut score_writes=0_usize;
    let mut real_edits=0_usize;
    let mut shadow_edits=0_usize;
    let mut rollback_attempts=0_usize;
    let mut rollback_sources=Vec::new();
    let mut fetches=0_usize;
    let mut accumulator_writes=0_usize;
    let mut peak=w.occupied_units();
    while w.step(){
        paid+=1;
        assert!(paid<=CREDITS as usize,
            "actor failed finite native compute budget: paid={paid}");
        let e=w.events().last().unwrap().clone();
        assert!(matches!(e.origin,Origin::Internal(1)));
        assert!(w.process(2).is_none(),"strict 1-PID program spawned child");
        assert_eq!(w.process(1).unwrap().code.nodes,founder,
            "native CIL source graph mutated");
        assert_eq!(w.total_compute_inventory(),Some(CREDITS-paid as u64));
        assert!(w.used_cells()<=16&&w.occupied_units()<=300);
        peak=peak.max(w.occupied_units());
        if e.pc==FETCH_PC{
            let vpc=w.cell(50).unwrap();
            assert!((0..=3).contains(&vpc));
            let key=51+vpc;
            assert!(matches!(&e.kind,EventKind::CellRead{key:k,value}
                if *k==key&&Some(*value)==w.cell(key)));
            fetches+=1;
        }
        if e.pc==FEEDBACK_PC{
            assert!(matches!(&e.kind,EventKind::CellRead{key:20,value}
                if *value==w.cell(20).unwrap_or(0)));
            probe_feedback_reads+=1;
        }
        if e.pc==SCORE_WRITE_PC{
            assert!(matches!(&e.kind,EventKind::CellWritten{key:61,..}));
            score_writes+=1;
        }
        if e.pc==REAL_EDIT_52_PC||e.pc==REAL_EDIT_53_PC{
            assert_eq!(gate,1);
            assert!(matches!(&e.kind,EventKind::CellWritten{key:52|53,..}));
            real_edits+=1;
        }
        if e.pc==SHADOW_1_PC||e.pc==SHADOW_2_PC{
            assert_eq!(gate,0);
            assert!(matches!(&e.kind,EventKind::CellWritten{key:56,..}));
            shadow_edits+=1;
        }
        if e.pc==ROLLBACK_PC{rollback_attempts+=1;}
        if e.pc==ROLLBACK_PC+1{
            if let EventKind::CellRead{key:62,value}=e.kind{
                rollback_sources.push(value);
            }else{panic!("actor rollback failed to pay Read62");}
        }
        if matches!(&e.kind,EventKind::CellWritten{key:56,..})
            && e.pc!=SHADOW_1_PC&&e.pc!=SHADOW_2_PC{
            accumulator_writes+=1;
        }
        let mut probe_to_react:Option<(usize,usize,i64)>=None;
        let mut finished_epoch:Option<usize>=None;
        if let EventKind::CellWritten{key:11,value}=e.kind{
            assert_eq!(e.pc,SERVICE_PC);
            let epoch=w.cell(31).unwrap() as usize;
            let stage=w.cell(32).unwrap() as usize;
            let mode=w.cell(33).unwrap();
            assert!(epoch<3&&stage<=2&&(0..=2).contains(&mode));
            services.push(json!([epoch,stage,mode,value,program(&w)]));
            if stage<2 {
                probe_to_react=Some((epoch,stage,value));
            }else{
                finished_epoch=Some(epoch);
            }
        }
        emit(out,json!(["step",id,paid,e.tick,1,e.pc,e.kind,
            process(&w,&hash),w.occupied_units(),cells(&w)]));
        // Input/feedback are the ONLY external mutable task signals.
        if let Some((epoch,stage,output))=probe_to_react{
            let expected=schedule[epoch].wanted(PROBES[stage]);
            let truth=i64::from(output==expected);
            if let Some(bit)=signal.bit(truth){
                ext(out,&mut w,id,20,bit,"post_probe_correctness_bit",&hash);
            }
            ext(out,&mut w,id,7,if stage==0{PROBES[1]}else{PROBES[0]},
                "next_probe_default_input",&hash);
        }
        if e.pc==HELDOUT_STAGE_PC {
            assert!(matches!(&e.kind,EventKind::CellWritten{key:32,value:2}));
            ext(out,&mut w,id,7,HELDOUT,"actor_selected_heldout_input",&hash);
        }
        if let Some(epoch)=finished_epoch{
            if epoch<2{
                ext(out,&mut w,id,7,PROBES[0],"next_phase_hidden_task_probe0",&hash);
            }
        }
    }
    assert!(w.process(1).unwrap().stopped);
    assert!(w.process(2).is_none());
    assert_eq!(w.cell(31),Some(3),"must actually complete all 3 epochs");
    assert!(rollback_sources.iter().all(|m|(0..=2).contains(m)));
    let heldout:Vec<_>=services.iter().filter(|s|s[1]==2).collect();
    assert_eq!(heldout.len(),3,"need real heldout service in all task phases");
    let per_phase:Vec<_>=heldout.iter().map(|r|{
        let epoch=r[0].as_u64().unwrap() as usize;
        let got=r[3].as_i64().unwrap();
        json!({
            "epoch":epoch,"task":schedule[epoch].name(),
            "mode":r[2],"output":got,
            "expected":schedule[epoch].wanted(HELDOUT),
            "correct":got==schedule[epoch].wanted(HELDOUT),
            "actual_bytecode":r[4],
        })
    }).collect();
    let correct=per_phase.iter().filter(|x|x["correct"]==true).count();
    if gate==1&&matches!(signal,Signal::Truth){
        assert_eq!(correct,if schedule_id==0{3}else{2});
        let sequence:Vec<i64>=per_phase.iter()
           .map(|r|r["mode"].as_i64().unwrap()).collect();
        assert_eq!(sequence,if schedule_id==0{vec![0,1,0]}else{vec![1,2,2]});
        if schedule_id==1{
            assert!(rollback_attempts>=1);
            assert!(rollback_sources.contains(&2));
            assert_eq!(per_phase[2]["correct"],false);
            assert_eq!(per_phase[2]["actual_bytecode"],json!([0,1,3,3]));
        }
    }
    if gate==0{
        assert!(services.iter().all(|s|s[4]==json!([0,3,3,3])));
        assert_eq!(real_edits,0);
    }else{
        assert_eq!(shadow_edits,0);
    }
    let result=json!({
        "id":id,"schedule_id":schedule_id,"signal":signal.name(),"gate":gate,
        "all_real_service_events":services,
        "heldout_by_phase":per_phase,
        "heldout_correct_count":correct,
        "charged_feedback_reads":probe_feedback_reads,
        "actor_paid_score_writes":score_writes,
        "actor_paid_real_opcode_slot_writes":real_edits,
        "actor_paid_suppressed_shadow_writes":shadow_edits,
        "prior_best_rollback_entry_count":rollback_attempts,
        "prior_best_rollback_read_values":rollback_sources,
        "virtual_opcode_fetches":fetches,
        "charged_accumulator_checkpoint_writes":accumulator_writes,
        "max_storage":peak,"paid_instructions":paid,
        "remaining_credits":w.remaining_credits(),
        "final_process":process(&w,&hash),"final_cells":cells(&w),
        "same_PID1":true,"native_source_unchanged":true,
        "author_primitive_ISA":true,"b3_met":false,"b4_met":false,"cyberlife":false
    });
    emit(out,json!(["end",id,result]));
    result
}
#[test]
fn b56_f1d1i_single_pid_feedback_conditioned_mutation_and_source_rollback(){
    let folder=std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
       .join("../../cl1b-ci-evidence");
    fs::create_dir_all(&folder).unwrap();
    let mut out=BufWriter::new(File::create(
        folder.join("b56_f1d1i_native_mutation_regime_shift.jsonl")).unwrap());
    let manifest=std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(
       "../../../cl1b/studies/B56-F1d1i-frozen-157-node-phase-shift-and-fallback-manifest-v1.md");
    let spec=std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(
       "../../../cl1b/studies/B56-F1d1i-frozen-exact-source-v1.json");
    let checksum=|p:&std::path::Path|{
       let out=std::process::Command::new("sha256sum").arg(p).output().unwrap();
       assert!(out.status.success());
       String::from_utf8(out.stdout).unwrap()
           .split_whitespace().next().unwrap().to_owned()
    };
    let bytes=fs::read(&spec).unwrap();
    let parsed:Value=serde_json::from_slice(&bytes).unwrap();
    let original=founder();
    assert_eq!(parsed["code"].as_array().unwrap().len(),original.len());
    assert_eq!(original.len(),157);
    for (i,real) in original.iter().enumerate(){
       let node=&parsed["code"][i];
       let kind=node["kind"].as_str().unwrap();
       let frozen=if kind=="Halt"{json!("Halt")}else{
          let mut body=serde_json::Map::new();
          for (k,v) in node.as_object().unwrap(){
             if k!="kind"{body.insert(k.clone(),v.clone());}
          }
          let mut op=serde_json::Map::new();
          op.insert(kind.to_owned(),Value::Object(body));
          Value::Object(op)
       };
       assert_eq!(serde_json::to_value(real).unwrap(),frozen,
          "native op differs from frozen F1d1i node {i}");
    }
    emit(&mut out,json!(["meta","B56-F1d1i",1,
       std::env::var("GITHUB_SHA").unwrap_or_else(|_|"local".to_owned()),
       checksum(&manifest),fs::metadata(&manifest).unwrap().len(),
       checksum(&spec),bytes.len(),16,CREDITS,16,180,300,
       original.len(),FETCH_PC,SERVICE_PC,FEEDBACK_PC,SCORE_WRITE_PC,
       REAL_EDIT_52_PC,REAL_EDIT_53_PC,SHADOW_1_PC,SHADOW_2_PC,
       ROLLBACK_PC,HELDOUT_STAGE_PC]));
    let mut rows=Vec::new();
    for sid in 0..2{
       for signal in [Signal::Truth,Signal::BlindSuccess,Signal::Inverted,Signal::Absent]{
          for gate in [1_i64,0]{
             let id=rows.len();
             rows.push(case(&mut out,id,sid,signal,gate));
          }
       }
    }
    out.flush().unwrap();
    assert_eq!(rows.len(),16);
    for sid in 0..2{
       let true_enabled=&rows.iter()
           .find(|r|r["schedule_id"]==sid&&r["signal"]=="TRUTH"&&r["gate"]==1)
           .unwrap();
       assert_eq!(true_enabled["heldout_correct_count"],
           if sid==0{3}else{2});
    }
    eprintln!("B56_F1D1I_NATIVE_CONDITIONED_MUTATION_AND_ROLLBACK {}",json!({
       "isolated_native_worlds":16,
       "native_source_CIL_nodes":157,
       "data_address_union":16,
       "same_PID1":true,
       "true_schedule1_heldout_correct":3,
       "true_schedule2_heldout_correct":2,
       "unlearnable_task_fallback_not_success":true,
       "new_opcode_semantics_invented":false,
       "B3":false,"B4":false,"cyberlife":false
    }));
}
