//! CL-1B B56-F1d1k-G1 — strict original PID1 double-interpreter
//! composed from TWO actor-owned fetched meta bytecode instructions.
 //! EXACT 178-opcode founder was committed before this integration source.
 //! This is a finite researcher-authored ISA, NOT de-novo algorithm invention.
use cl0_world::{EventKind,Intervention,Op,Origin,Process,World};
use serde_json::{json,Value};
use std::fs::{self,File};
use std::io::{BufWriter,Write};

const CREDITS:u64=4000;
const KEYS:[i64;14]=[7,11,20,21,31,32,33,50,52,53,56,61,62,63];
const SOURCE_NODES:usize=178;
const META_FETCH_PC:usize=36;
const META_CHECKPOINT_PC:usize=52;
const META_MODE_PC:usize=65;
const TASK_FETCH1_PC:usize=90;
const TASK_FETCH2_PC:usize=97;
const TASK_ACC1_PC:usize=95;
const TASK_ACC2_PC:usize=100;
const TASK_WRITE52_PC:usize=79;
const TASK_WRITE53_PC:usize=81;
const SERVICE_PC:usize=102;
const FEEDBACK_PC:usize=109;
const SCORE_PC:usize=115;
const BASE_WRITE_PC:usize=137;
const OFFSET_RESET_PC:usize=139;
const SHADOW_BASE_PC:usize=143;
const SHADOW_OFFSET_PC:usize=145;
const FAILED_OFFSET_PC:usize=150;
const FALLBACK_OFFSET_PC:usize=159;
const HELDOUT_STAGE_PC:usize=162;
const INITIAL_B_PC:usize=20;
const INITIAL_A_PC:usize=26;

fn founder()->Vec<Op>{
    vec![
        Op::Const{dst:2,value:31,next:1}, // pc 0
        Op::Write{address:2,value:0,next:2}, // pc 1
        Op::Const{dst:2,value:32,next:3}, // pc 2
        Op::Write{address:2,value:0,next:4}, // pc 3
        Op::Const{dst:2,value:33,next:5}, // pc 4
        Op::Write{address:2,value:0,next:6}, // pc 5
        Op::Const{dst:2,value:50,next:7}, // pc 6
        Op::Write{address:2,value:0,next:8}, // pc 7
        Op::Const{dst:2,value:61,next:9}, // pc 8
        Op::Write{address:2,value:0,next:10}, // pc 9
        Op::Const{dst:2,value:63,next:11}, // pc 10
        Op::Write{address:2,value:0,next:12}, // pc 11
        Op::Const{dst:2,value:62,next:13}, // pc 12
        Op::Const{dst:3,value:1,next:14}, // pc 13
        Op::Write{address:2,value:3,next:15}, // pc 14
        Op::Const{dst:2,value:21,next:16}, // pc 15
        Op::Read{dst:3,address:2,next:17}, // pc 16
        Op::Const{dst:2,value:2,next:18}, // pc 17
        Op::Sub{dst:3,lhs:3,rhs:2,next:19}, // pc 18
        Op::Branch{test:3,nonzero:23,zero:20}, // pc 19
        Op::Const{dst:2,value:62,next:21}, // pc 20
        Op::Const{dst:3,value:2,next:22}, // pc 21
        Op::Write{address:2,value:3,next:28}, // pc 22
        Op::Const{dst:2,value:1,next:24}, // pc 23
        Op::Sub{dst:3,lhs:3,rhs:2,next:25}, // pc 24
        Op::Branch{test:3,nonzero:28,zero:26}, // pc 25
        Op::Const{dst:2,value:62,next:27}, // pc 26
        Op::Write{address:2,value:0,next:28}, // pc 27
        Op::Const{dst:2,value:50,next:29}, // pc 28
        Op::Write{address:2,value:0,next:30}, // pc 29
        Op::Const{dst:2,value:56,next:31}, // pc 30
        Op::Write{address:2,value:0,next:32}, // pc 31
        Op::Const{dst:2,value:50,next:33}, // pc 32
        Op::Read{dst:3,address:2,next:34}, // pc 33
        Op::Const{dst:2,value:62,next:35}, // pc 34
        Op::Add{dst:2,lhs:2,rhs:3,next:36}, // pc 35
        Op::Read{dst:3,address:2,next:37}, // pc 36
        Op::Const{dst:2,value:3,next:38}, // pc 37
        Op::Sub{dst:1,lhs:3,rhs:2,next:39}, // pc 38
        Op::Branch{test:1,nonzero:40,zero:177}, // pc 39
        Op::Const{dst:2,value:56,next:41}, // pc 40
        Op::Read{dst:1,address:2,next:42}, // pc 41
        Op::Add{dst:1,lhs:1,rhs:3,next:43}, // pc 42
        Op::Const{dst:2,value:3,next:44}, // pc 43
        Op::Sub{dst:3,lhs:1,rhs:2,next:45}, // pc 44
        Op::Branch{test:3,nonzero:47,zero:46}, // pc 45
        Op::Const{dst:1,value:0,next:51}, // pc 46
        Op::Const{dst:2,value:4,next:48}, // pc 47
        Op::Sub{dst:3,lhs:1,rhs:2,next:49}, // pc 48
        Op::Branch{test:3,nonzero:51,zero:50}, // pc 49
        Op::Const{dst:1,value:1,next:51}, // pc 50
        Op::Const{dst:2,value:56,next:52}, // pc 51
        Op::Write{address:2,value:1,next:53}, // pc 52
        Op::Const{dst:2,value:50,next:54}, // pc 53
        Op::Read{dst:3,address:2,next:55}, // pc 54
        Op::Const{dst:2,value:1,next:56}, // pc 55
        Op::Add{dst:3,lhs:3,rhs:2,next:57}, // pc 56
        Op::Const{dst:2,value:50,next:58}, // pc 57
        Op::Write{address:2,value:3,next:59}, // pc 58
        Op::Const{dst:2,value:2,next:60}, // pc 59
        Op::Sub{dst:3,lhs:3,rhs:2,next:61}, // pc 60
        Op::Branch{test:3,nonzero:32,zero:62}, // pc 61
        Op::Const{dst:2,value:56,next:63}, // pc 62
        Op::Read{dst:3,address:2,next:64}, // pc 63
        Op::Const{dst:2,value:33,next:65}, // pc 64
        Op::Write{address:2,value:3,next:66}, // pc 65
        Op::Const{dst:2,value:33,next:67}, // pc 66
        Op::Read{dst:3,address:2,next:68}, // pc 67
        Op::Branch{test:3,nonzero:69,zero:72}, // pc 68
        Op::Const{dst:2,value:1,next:70}, // pc 69
        Op::Sub{dst:3,lhs:3,rhs:2,next:71}, // pc 70
        Op::Branch{test:3,nonzero:76,zero:74}, // pc 71
        Op::Const{dst:1,value:1,next:73}, // pc 72
        Op::Const{dst:3,value:0,next:78}, // pc 73
        Op::Const{dst:1,value:0,next:75}, // pc 74
        Op::Const{dst:3,value:2,next:78}, // pc 75
        Op::Const{dst:1,value:0,next:77}, // pc 76
        Op::Const{dst:3,value:0,next:78}, // pc 77
        Op::Const{dst:2,value:52,next:79}, // pc 78
        Op::Write{address:2,value:1,next:80}, // pc 79
        Op::Const{dst:2,value:53,next:81}, // pc 80
        Op::Write{address:2,value:3,next:82}, // pc 81
        Op::Const{dst:2,value:32,next:83}, // pc 82
        Op::Read{dst:3,address:2,next:84}, // pc 83
        Op::Const{dst:2,value:2,next:85}, // pc 84
        Op::Sub{dst:3,lhs:3,rhs:2,next:86}, // pc 85
        Op::Branch{test:3,nonzero:87,zero:160}, // pc 86
        Op::Const{dst:2,value:7,next:88}, // pc 87
        Op::Read{dst:1,address:2,next:89}, // pc 88
        Op::Const{dst:2,value:52,next:90}, // pc 89
        Op::Read{dst:3,address:2,next:91}, // pc 90
        Op::Branch{test:3,nonzero:94,zero:92}, // pc 91
        Op::Const{dst:2,value:0,next:93}, // pc 92
        Op::Sub{dst:1,lhs:2,rhs:1,next:94}, // pc 93
        Op::Const{dst:2,value:56,next:95}, // pc 94
        Op::Write{address:2,value:1,next:96}, // pc 95
        Op::Const{dst:2,value:53,next:97}, // pc 96
        Op::Read{dst:3,address:2,next:98}, // pc 97
        Op::Add{dst:1,lhs:1,rhs:3,next:99}, // pc 98
        Op::Const{dst:2,value:56,next:100}, // pc 99
        Op::Write{address:2,value:1,next:101}, // pc 100
        Op::Const{dst:2,value:11,next:102}, // pc 101
        Op::Write{address:2,value:1,next:103}, // pc 102
        Op::Const{dst:2,value:32,next:104}, // pc 103
        Op::Read{dst:3,address:2,next:105}, // pc 104
        Op::Const{dst:2,value:2,next:106}, // pc 105
        Op::Sub{dst:3,lhs:3,rhs:2,next:107}, // pc 106
        Op::Branch{test:3,nonzero:108,zero:163}, // pc 107
        Op::Const{dst:2,value:20,next:109}, // pc 108
        Op::Read{dst:1,address:2,next:110}, // pc 109
        Op::Branch{test:1,nonzero:111,zero:116}, // pc 110
        Op::Const{dst:2,value:61,next:112}, // pc 111
        Op::Read{dst:3,address:2,next:113}, // pc 112
        Op::Const{dst:1,value:1,next:114}, // pc 113
        Op::Add{dst:3,lhs:3,rhs:1,next:115}, // pc 114
        Op::Write{address:2,value:3,next:116}, // pc 115
        Op::Const{dst:2,value:32,next:117}, // pc 116
        Op::Read{dst:3,address:2,next:118}, // pc 117
        Op::Const{dst:1,value:1,next:119}, // pc 118
        Op::Add{dst:3,lhs:3,rhs:1,next:120}, // pc 119
        Op::Write{address:2,value:3,next:121}, // pc 120
        Op::Const{dst:2,value:2,next:122}, // pc 121
        Op::Sub{dst:3,lhs:3,rhs:2,next:123}, // pc 122
        Op::Branch{test:3,nonzero:87,zero:124}, // pc 123
        Op::Const{dst:2,value:61,next:125}, // pc 124
        Op::Read{dst:3,address:2,next:126}, // pc 125
        Op::Const{dst:2,value:2,next:127}, // pc 126
        Op::Sub{dst:3,lhs:3,rhs:2,next:128}, // pc 127
        Op::Branch{test:3,nonzero:146,zero:129}, // pc 128
        Op::Const{dst:2,value:21,next:130}, // pc 129
        Op::Read{dst:3,address:2,next:131}, // pc 130
        Op::Const{dst:2,value:1,next:132}, // pc 131
        Op::Sub{dst:3,lhs:3,rhs:2,next:133}, // pc 132
        Op::Branch{test:3,nonzero:140,zero:134}, // pc 133
        Op::Const{dst:2,value:33,next:135}, // pc 134
        Op::Read{dst:3,address:2,next:136}, // pc 135
        Op::Const{dst:2,value:62,next:137}, // pc 136
        Op::Write{address:2,value:3,next:138}, // pc 137
        Op::Const{dst:2,value:63,next:139}, // pc 138
        Op::Write{address:2,value:0,next:160}, // pc 139
        Op::Const{dst:2,value:33,next:141}, // pc 140
        Op::Read{dst:3,address:2,next:142}, // pc 141
        Op::Const{dst:2,value:56,next:143}, // pc 142
        Op::Write{address:2,value:3,next:144}, // pc 143
        Op::Const{dst:2,value:56,next:145}, // pc 144
        Op::Write{address:2,value:0,next:160}, // pc 145
        Op::Const{dst:2,value:63,next:147}, // pc 146
        Op::Read{dst:3,address:2,next:148}, // pc 147
        Op::Const{dst:1,value:1,next:149}, // pc 148
        Op::Add{dst:3,lhs:3,rhs:1,next:150}, // pc 149
        Op::Write{address:2,value:3,next:151}, // pc 150
        Op::Const{dst:2,value:3,next:152}, // pc 151
        Op::Sub{dst:3,lhs:3,rhs:2,next:153}, // pc 152
        Op::Branch{test:3,nonzero:154,zero:158}, // pc 153
        Op::Const{dst:2,value:32,next:155}, // pc 154
        Op::Write{address:2,value:0,next:156}, // pc 155
        Op::Const{dst:2,value:61,next:157}, // pc 156
        Op::Write{address:2,value:0,next:28}, // pc 157
        Op::Const{dst:2,value:63,next:159}, // pc 158
        Op::Write{address:2,value:0,next:28}, // pc 159
        Op::Const{dst:2,value:32,next:161}, // pc 160
        Op::Const{dst:3,value:2,next:162}, // pc 161
        Op::Write{address:2,value:3,next:87}, // pc 162
        Op::Const{dst:2,value:31,next:164}, // pc 163
        Op::Read{dst:3,address:2,next:165}, // pc 164
        Op::Const{dst:1,value:1,next:166}, // pc 165
        Op::Add{dst:3,lhs:3,rhs:1,next:167}, // pc 166
        Op::Write{address:2,value:3,next:168}, // pc 167
        Op::Const{dst:2,value:2,next:169}, // pc 168
        Op::Sub{dst:3,lhs:3,rhs:2,next:170}, // pc 169
        Op::Branch{test:3,nonzero:171,zero:177}, // pc 170
        Op::Const{dst:2,value:32,next:172}, // pc 171
        Op::Write{address:2,value:0,next:173}, // pc 172
        Op::Const{dst:2,value:61,next:174}, // pc 173
        Op::Write{address:2,value:0,next:175}, // pc 174
        Op::Const{dst:2,value:63,next:176}, // pc 175
        Op::Write{address:2,value:0,next:28}, // pc 176
        Op::Halt, // pc 177
    ]
}
#[derive(Copy,Clone,Debug,PartialEq,Eq)]
enum Task{Identity,Negate,Composed,Unreachable}
impl Task{
    fn label(self)->&'static str{match self{
        Self::Identity=>"IDENTITY",Self::Negate=>"NEGATE",
        Self::Composed=>"COMPOSED",Self::Unreachable=>"UNREACHABLE"}}
    fn oracle(self,x:i64)->i64{match self{
        Self::Identity=>x,Self::Negate=>-x,
        Self::Composed=>-x+2,Self::Unreachable=>x+4}}
}
const TASK_SCHEDULES:[[Task;2];4]=[
    [Task::Negate,Task::Negate],
    [Task::Composed,Task::Negate],
    [Task::Negate,Task::Composed],
    [Task::Identity,Task::Unreachable],
];
#[derive(Copy,Clone)]
struct Seed{probe:[i64;2],heldout:i64}
const SEEDS:[Seed;2]=[
    Seed{probe:[-7,5],heldout:11},
    Seed{probe:[5,-7],heldout:-11},
];
#[derive(Copy,Clone,Debug)]
enum Regime{Truth,BlindSuccess,Inverted,Absent,Phase0CDecoy}
impl Regime{
    fn label(self)->&'static str{match self{
        Self::Truth=>"TRUTH",Self::BlindSuccess=>"BLIND_SUCCESS",
        Self::Inverted=>"INVERTED",Self::Absent=>"ABSENT",
        Self::Phase0CDecoy=>"PHASE0_C_DECOY"}}
    fn bit(self,correct:i64,phase:i64,mode:i64)->Option<i64>{match self{
        Self::Truth=>Some(correct),Self::BlindSuccess=>Some(1),
        Self::Inverted=>Some(1-correct),Self::Absent=>None,
        Self::Phase0CDecoy=>Some(if phase==0&&mode==1{1}else{correct}),
    }}
}
fn emit(out:&mut BufWriter<File>,row:Value){
    serde_json::to_writer(&mut *out,&row).unwrap();
    out.write_all(b"\n").unwrap();
}
fn fnv_code(code:&[Op])->String{
    let bytes=serde_json::to_vec(code).unwrap();
    let hash=bytes.iter().fold(14695981039346656037_u64,|h,b|{
        (h^u64::from(*b)).wrapping_mul(1099511628211)
    });
    format!("{hash:016x}")
}
fn snap(w:&World,hash:&str)->Value{
    let p=w.process(1).unwrap();
    json!([p.pc,p.registers,p.credits,p.stopped,p.code.nodes.len(),hash])
}
fn data(w:&World)->Value{json!(KEYS.map(|k|w.cell(k)))}
fn task_code(w:&World)->Value{json!([w.cell(52),w.cell(53)])}
fn external(out:&mut BufWriter<File>,w:&mut World,id:usize,key:i64,value:i64,
    reason:&str,hash:&str){
    assert!(matches!(key,7|20|21),"external source may write input/feedback/gate ONLY");
    w.intervene(Intervention::SetCell{key,value}).unwrap();
    let e=w.events().last().unwrap();
    assert!(matches!(e.origin,Origin::External));
    assert!(w.used_cells()<=16&&w.occupied_units()<=300);
    emit(out,json!(["ext",id,e.tick,key,value,reason,snap(w,hash),
        w.occupied_units(),data(w)]));
}

fn one_world(out:&mut BufWriter<File>,id:usize,seed_id:usize,sid:usize,
    regime:Regime,gate:i64)->Value{
    let code=founder();
    assert_eq!(code.len(),SOURCE_NODES);
    let hash=fnv_code(&code);
    let tasks=TASK_SCHEDULES[sid];
    let seed=SEEDS[seed_id];
    let mut w=World::with_strict_capacity(16,180,300);
    w.load(Process::new(1,code.clone(),CREDITS)).unwrap();
    emit(out,json!(["case",id,seed_id,sid,regime.label(),gate,
        tasks.map(|t|t.label()),seed.probe,seed.heldout,
        code,hash,w.occupied_units(),CREDITS,16,180,300]));
    external(out,&mut w,id,21,gate,"initial_immutable_rule_control",&hash);
    external(out,&mut w,id,7,seed.probe[0],"first_task_probe",&hash);
    let mut paid=0_usize;
    let mut ext_events=2_usize;
    let mut peak=w.occupied_units();
    let mut real_services:Vec<Value>=Vec::new();
    let mut meta_fetches:Vec<Value>=Vec::new();
    let mut meta_programs:Vec<Value>=Vec::new();
    let mut meta_checkpoints:Vec<Value>=Vec::new();
    let mut meta_base_edits:Vec<Value>=Vec::new();
    let mut meta_offset_edits:Vec<Value>=Vec::new();
    let mut meta_shadows:Vec<Value>=Vec::new();
    let mut task_program_writes:Vec<Value>=Vec::new();
    let mut task_fetches=0_usize;
    let mut task_checkpoints=0_usize;
    let mut feedback_reads=0_usize;
    let mut score_writes=0_usize;
    let mut fallbacks=0_usize;
    let mut paid_at_heldout:Vec<usize>=Vec::new();
    while w.step(){
        paid+=1;
        assert!(paid<=CREDITS as usize,"finite PID1 compute exhausted");
        let e=w.events().last().unwrap().clone();
        assert!(matches!(e.origin,Origin::Internal(1)),"foreign instruction origin");
        assert!(w.process(2).is_none(),"unexpected PID2");
        assert_eq!(w.process(1).unwrap().code.nodes,code,"native executable source mutated");
        assert_eq!(w.total_compute_inventory(),Some(CREDITS-paid as u64));
        assert!(w.used_cells()<=16&&w.occupied_units()<=300);
        peak=peak.max(w.occupied_units());
        if e.pc==META_FETCH_PC {
            let meta_pc=w.cell(50).unwrap();
            assert!((0..=1).contains(&meta_pc));
            let key=62+meta_pc;
            if let EventKind::CellRead{key:k,value}=&e.kind{
                assert_eq!(*k,key);
                assert!((0..=2).contains(value),"invalid source-owned meta opcode");
                meta_fetches.push(json!([w.cell(31),meta_pc,key,value,e.tick]));
            }else{panic!("meta fetch PC must pay actual Read");}
        }
        if e.pc==META_CHECKPOINT_PC{
            if let EventKind::CellWritten{key:56,value}=&e.kind{
                meta_checkpoints.push(json!([w.cell(31),w.cell(50),value,e.tick]));
            }else{panic!("meta intermediate must pay Write56");}
        }
        if e.pc==META_MODE_PC{
            if let EventKind::CellWritten{key:33,value}=&e.kind{
                assert!((0..=2).contains(value));
                meta_programs.push(json!([w.cell(31),w.cell(62),w.cell(63),value,e.tick]));
            }else{panic!("interpreter mode not paid");}
        }
        if e.pc==TASK_FETCH1_PC||e.pc==TASK_FETCH2_PC{
            let want=if e.pc==TASK_FETCH1_PC{52}else{53};
            assert!(matches!(&e.kind,EventKind::CellRead{key,..} if *key==want));
            task_fetches+=1;
        }
        if e.pc==TASK_ACC1_PC||e.pc==TASK_ACC2_PC{
            assert!(matches!(&e.kind,EventKind::CellWritten{key:56,..}));
            task_checkpoints+=1;
        }
        if e.pc==BASE_WRITE_PC||e.pc==OFFSET_RESET_PC{
            assert_eq!(gate,1);
            if let EventKind::CellWritten{key,value}=&e.kind{
                assert_eq!(*key,if e.pc==BASE_WRITE_PC{62}else{63});
                meta_base_edits.push(json!([w.cell(31),w.cell(33),key,value,e.tick]));
            }else{panic!("actor actual meta code edit missing");}
        }
        if e.pc==SHADOW_BASE_PC||e.pc==SHADOW_OFFSET_PC{
            assert_ne!(gate,1);
            if let EventKind::CellWritten{key:56,value}=&e.kind{
                meta_shadows.push(json!([w.cell(31),w.cell(33),value,e.tick]));
            }else{panic!("disallowed actual meta edit under ablation");}
        }
        if e.pc==FAILED_OFFSET_PC||e.pc==FALLBACK_OFFSET_PC {
            if let EventKind::CellWritten{key:63,value}=&e.kind{
                meta_offset_edits.push(json!([w.cell(31),value,e.pc,e.tick]));
                if e.pc==FALLBACK_OFFSET_PC{fallbacks+=1;}
            }else{panic!("rule-program offset rewrite not native");}
        }
        if e.pc==TASK_WRITE52_PC||e.pc==TASK_WRITE53_PC {
            if let EventKind::CellWritten{key,value}=&e.kind{
                assert_eq!(*key,if e.pc==TASK_WRITE52_PC{52}else{53});
                task_program_writes.push(json!([w.cell(31),w.cell(33),key,value,e.tick]));
            }else{panic!("task program slot not native written");}
        }
        if e.pc==FEEDBACK_PC{
            assert!(matches!(&e.kind,EventKind::CellRead{key:20,value}
                if *value==w.cell(20).unwrap_or(0)));
            feedback_reads+=1;
        }
        if e.pc==SCORE_PC{
            assert!(matches!(&e.kind,EventKind::CellWritten{key:61,..}));
            score_writes+=1;
        }
        let mut probe:Option<(usize,usize,i64,i64)>=None;
        let mut ended:Option<usize>=None;
        if let EventKind::CellWritten{key:11,value}=&e.kind{
            assert_eq!(e.pc,SERVICE_PC);
            let phase=w.cell(31).unwrap();
            let stage=w.cell(32).unwrap();
            let mode=w.cell(33).unwrap();
            assert!((0..=1).contains(&phase)&&(0..=2).contains(&stage));
            assert!((0..=2).contains(&mode));
            real_services.push(json!([phase,stage,mode,value,task_code(&w),
                w.cell(62),w.cell(63)]));
            if stage<2 {probe=Some((phase as usize,stage as usize,mode,*value));}
            else{ended=Some(phase as usize);paid_at_heldout.push(paid);}
        }
        emit(out,json!(["step",id,paid,e.tick,1,e.pc,e.kind,
            snap(&w,&hash),w.occupied_units(),data(&w)]));
        if let Some((phase,stage,mode,actual))=probe {
            let expected=tasks[phase].oracle(seed.probe[stage]);
            let truth=i64::from(actual==expected);
            if let Some(bit)=regime.bit(truth,phase as i64,mode){
                external(out,&mut w,id,20,bit,"after_paid_probe_feedback",&hash);
                ext_events+=1;
            }
            external(out,&mut w,id,7,
                if stage==0{seed.probe[1]}else{seed.probe[0]},
                "next_probe_input",&hash);
            ext_events+=1;
        }
        if e.pc==HELDOUT_STAGE_PC{
            assert!(matches!(&e.kind,EventKind::CellWritten{key:32,value:2}));
            external(out,&mut w,id,7,seed.heldout,"actor_requested_heldout",&hash);
            ext_events+=1;
        }
        if let Some(phase)=ended{
            if phase==0 {
                external(out,&mut w,id,7,seed.probe[0],"next_hidden_task_phase",&hash);
                ext_events+=1;
            }
        }
    }
    assert!(w.process(1).unwrap().stopped);
    assert!(w.process(2).is_none());
    assert_eq!(w.cell(31),Some(2));
    assert_eq!(paid_at_heldout.len(),2);
    assert!(meta_fetches.len()>=4&&meta_fetches.len()%2==0);
    assert_eq!(meta_checkpoints.len(),meta_fetches.len());
    assert_eq!(meta_programs.len()*2,meta_fetches.len());
    assert_eq!(task_fetches,real_services.len()*2);
    assert_eq!(task_checkpoints,real_services.len()*2);
    assert!(meta_programs.iter().all(|p|(0..=2).contains(&p[3].as_i64().unwrap())));
    if gate==1 {
        assert!(meta_shadows.is_empty());
    }else{
        assert!(meta_base_edits.is_empty());
        assert_eq!(w.cell(62),Some(if gate==2{2}else if gate==3{0}else{1}));
    }
    let heldout:Vec<Value>=real_services.iter().filter(|s|s[1]==2).cloned().collect();
    assert_eq!(heldout.len(),2);
    let phases:Vec<Value>=heldout.iter().map(|s|{
        let phase=s[0].as_u64().unwrap() as usize;
        let output=s[3].as_i64().unwrap();
        json!({"phase":phase,"task":tasks[phase].label(),"selected_mode":s[2],
            "output":output,"expected":tasks[phase].oracle(seed.heldout),
            "correct":output==tasks[phase].oracle(seed.heldout),
            "task_bytecode":s[4],"meta_base":s[5],"meta_offset":s[6]})
    }).collect();
    let good=phases.iter().filter(|x|x["correct"]==true).count();
    if matches!(regime,Regime::Truth){
        assert_eq!(good,if sid==3{1}else{2},"preregistered heldout truth result");
        if sid==3{
            assert_eq!(phases[1]["correct"],false);
            assert_eq!(phases[1]["expected"],seed.heldout+4);
            assert!(fallbacks>0);
        }
    }
    let costs=[paid_at_heldout[0],paid_at_heldout[1]-paid_at_heldout[0]];
    let result=json!({
        "id":id,"seed":seed_id,"schedule":sid,"regime":regime.label(),"gate":gate,
        "original_PID1_real_service_events":real_services,
        "heldout_by_phase":phases,"heldout_correct":good,
        "native_paid_meta_opcode_fetches":meta_fetches,
        "native_paid_meta_intermediate_checkpoints":meta_checkpoints,
        "native_composed_meta_program_results":meta_programs,
        "native_actor_meta_descriptor_writes":meta_base_edits,
        "native_actor_failed_probe_meta_offset_writes":meta_offset_edits,
        "native_paid_meta_write_suppression_shadows":meta_shadows,
        "native_task_program_writes":task_program_writes,
        "native_task_opcode_reads":task_fetches,
        "native_task_intermediate_checkpoints":task_checkpoints,
        "real_feedback_reads":feedback_reads,"real_candidate_score_writes":score_writes,
        "source_paid_policy_fallbacks":fallbacks,
        "phase_costs_at_real_heldout":[costs[0],costs[1]],
        "actual_paid_CIL_instructions":paid,
        "actual_external_interventions":ext_events,
        "maximum_occupied_storage":peak,
        "final_meta_bytecode":[w.cell(62),w.cell(63)],
        "final_process":snap(&w,&hash),"final_cells":data(&w),
        "credits_left":w.remaining_credits(),
        "native_source_unchanged":true,"original_PID1_only":true,
        "authored_meta_opcode_ISA":true,"generic_unbounded_rule_synthesis":false,
        "b3":false,"b4":false,"cyberlife":false
    });
    emit(out,json!(["end",id,result]));
    result
}
#[test]
fn b56_f1d1k_g1_two_level_fetched_actor_paid_meta_rule_composition(){
    let folder=std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../cl1b-ci-evidence");
    fs::create_dir_all(&folder).unwrap();
    let mut out=BufWriter::new(File::create(folder.join(
        "b56_f1d1k_g1_native_two_level_meta_rule.jsonl")).unwrap());
    let manifest=std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(
        "../../../cl1b/studies/B56-F1d1k-G1-exact-two-level-meta-bytecode-and-160-world-controls-preregistration.md");
    let graph=std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(
        "../../../cl1b/studies/B56-F1d1k-G1-frozen-exact-178-node-source-v1.json");
    let hash_file=|p:&std::path::Path|{
        let q=std::process::Command::new("sha256sum").arg(p).output().unwrap();
        assert!(q.status.success());
        String::from_utf8(q.stdout).unwrap().split_whitespace()
            .next().unwrap().to_string()
    };
    let bytes=fs::read(&graph).unwrap();
    let spec:Value=serde_json::from_slice(&bytes).unwrap();
    let code=founder();
    assert_eq!(code.len(),SOURCE_NODES);
    for (i,original) in code.iter().enumerate(){
        let x=&spec["code"][i];
        let kind=x["kind"].as_str().unwrap();
        let expected=if kind=="Halt"{json!("Halt")}else{
            let mut fields=serde_json::Map::new();
            for (k,v) in x.as_object().unwrap(){
                if k!="kind"{fields.insert(k.to_string(),v.clone());}
            }
            let mut outer=serde_json::Map::new();
            outer.insert(kind.to_string(),Value::Object(fields));
            Value::Object(outer)
        };
        assert_eq!(serde_json::to_value(original).unwrap(),expected,
            "F1d1k native CIL PC{i} differs from PRECOMMITTED graph");
    }
    assert_eq!(spec["resources"]["initial_credits"],CREDITS);
    assert_eq!(spec["resources"]["max_cells"],16);
    assert_eq!(spec["resources"]["max_native_nodes"],180);
    assert_eq!(spec["resources"]["max_total_storage"],300);
    assert_eq!(spec["case_count"],160);
    emit(&mut out,json!(["meta","B56-F1d1k-G1",1,
        std::env::var("GITHUB_SHA").unwrap_or_else(|_|"local".to_string()),
        hash_file(&manifest),fs::metadata(&manifest).unwrap().len(),
        hash_file(&graph),bytes.len(),160,CREDITS,16,180,300,SOURCE_NODES,
        META_FETCH_PC,META_CHECKPOINT_PC,META_MODE_PC,
        TASK_FETCH1_PC,TASK_FETCH2_PC,TASK_ACC1_PC,TASK_ACC2_PC,
        TASK_WRITE52_PC,TASK_WRITE53_PC,SERVICE_PC,FEEDBACK_PC,SCORE_PC,
        BASE_WRITE_PC,OFFSET_RESET_PC,SHADOW_BASE_PC,SHADOW_OFFSET_PC,
        FAILED_OFFSET_PC,FALLBACK_OFFSET_PC,HELDOUT_STAGE_PC]));
    let mut results=Vec::new();
    for seed_id in 0..2{
        for schedule_id in 0..4{
            for regime in [
                Regime::Truth,Regime::BlindSuccess,Regime::Inverted,
                Regime::Absent,Regime::Phase0CDecoy
            ]{
                for gate in [1_i64,0,2,3]{
                    let id=results.len();
                    results.push(one_world(&mut out,id,seed_id,schedule_id,regime,gate));
                }
            }
        }
    }
    out.flush().unwrap();
    assert_eq!(results.len(),160);
    let choose=|seed:usize,sid:usize,regime:&str,gate:i64|->&Value{
        results.iter().find(|x|x["seed"]==seed
            &&x["schedule"]==sid&&x["regime"]==regime&&x["gate"]==gate).unwrap()
    };
    for seed in 0..2{
        let repeated=choose(seed,0,"TRUTH",1);
        let frozen=choose(seed,0,"TRUTH",0);
        let privileged=choose(seed,0,"TRUTH",2);
        assert!(repeated["phase_costs_at_real_heldout"][1].as_u64().unwrap()
            <frozen["phase_costs_at_real_heldout"][1].as_u64().unwrap());
        assert!(privileged["actual_paid_CIL_instructions"].as_u64().unwrap()
            <repeated["actual_paid_CIL_instructions"].as_u64().unwrap());
        let decoy=choose(seed,0,"PHASE0_C_DECOY",1);
        let first_truth_next=&repeated["native_composed_meta_program_results"];
        let first_decoy_next=&decoy["native_composed_meta_program_results"];
        let first=(0..first_truth_next.as_array().unwrap().len())
            .find(|i|first_truth_next[*i][0]==1).unwrap();
        let second=(0..first_decoy_next.as_array().unwrap().len())
            .find(|i|first_decoy_next[*i][0]==1).unwrap();
        assert_eq!(first_truth_next[first][1],2);
        assert_eq!(first_decoy_next[second][1],1);
        assert_eq!(first_truth_next[first][3],2);
        assert_eq!(first_decoy_next[second][3],1);
        assert_eq!(decoy["heldout_by_phase"][0]["correct"],false);
        assert_eq!(repeated["heldout_by_phase"][0]["correct"],true);
        let shift=choose(seed,2,"TRUTH",1);
        let no_learning=choose(seed,2,"TRUTH",0);
        assert!(shift["phase_costs_at_real_heldout"][1].as_u64().unwrap()
            >no_learning["phase_costs_at_real_heldout"][1].as_u64().unwrap());
        let identity=choose(seed,3,"TRUTH",1);
        let phase0:Vec<i64>=identity["native_composed_meta_program_results"]
            .as_array().unwrap().iter()
            .filter(|x|x[0]==0)
            .map(|x|x[3].as_i64().unwrap()).collect();
        assert_eq!(phase0,vec![1,2,0],"single PID1 must execute 3 effective meta programs");
        assert_eq!(identity["heldout_by_phase"][1]["correct"],false);
    }
    eprintln!("B56_F1D1K_G1_TWO_LEVEL_NATIVE {}",json!({
        "strict_native_PID1_worlds":160,"original_CIL_nodes":178,
        "allowed_data_keys":14,"source_unchanged":true,
        "two_meta_opcodes_dynamically_fetched":true,
        "three_composed_rule_programs_in_one_PID":true,
        "truth_repeated_task_phase1_benefit":true,
        "shift_negative_transfer":true,
        "targeted_false_feedback_changes_future_rule_executions":true,
        "unsolved_task_remains_wrong":true,
        "new_meta_semantics_self_invented":false,
        "full_Hk_met":false,"B3":false,"B4":false,"cyberlife":false
    }));
}
