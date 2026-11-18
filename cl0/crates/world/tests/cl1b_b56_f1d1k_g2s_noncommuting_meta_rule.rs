//! CL-1B B56-F1d1k-G2S: native PID1-paid noncommuting RESET at meta level,
//! exact PRECOMMITTED 177-node founder, actor-generated bad4 and negative
//! reachable-program exclusion controls. NOT full Hk or cyberlife.
use cl0_world::{EventKind,Intervention,Op,Origin,Process,World};
use serde_json::{json,Value};
use std::fs::{self,File};
use std::io::{BufWriter,Write};

const CREDITS:u64=4000;
const KEYS:[i64;16]=[7,11,20,21,31,32,33,50,52,53,56,61,62,63,64,65];
const META_OPCODE_FETCH:usize=21;
const META_CHECKPOINT:usize=41;
const META_MODE_WRITE:usize=54;
const TASK_FETCH1:usize=73;
const TASK_FETCH2:usize=78;
const TASK_CHECKPOINT:usize=81;
const TASK_WRITE52:usize=67;
const TASK_WRITE53:usize=69;
const SERVICE_WRITE:usize=83;
const FEEDBACK_READ:usize=90;
const SCORE_WRITE:usize=96;
const PAID_BASE_WRITE:usize=118;
const PAID_TAIL_WRITE:usize=120;
const PAID_RESET_OFFSET:usize=123;
const SHADOW1:usize=127;
const SHADOW2:usize=128;
const SHADOW3:usize=130;
const TEMP_OFFSET_WRITE:usize=140;
const ATTEMPT_COUNT_WRITE:usize=145;
const FALLBACK_OFFSET_WRITE:usize=155;
const HOLD_WRITE:usize=161;
const FALLBACK_HOLD_WRITE:usize=158;
const BAD4_WRITE:usize=12;
const INIT_BASE_WRITE:usize=2;
const INIT_TAIL_WRITE:usize=4;
const HALT_PC:usize=176;

fn founder()->Vec<Op>{
    vec![
        Op::Const{dst:2,value:62,next:1}, // pc 0
        Op::Const{dst:3,value:1,next:2}, // pc 1
        Op::Write{address:2,value:3,next:3}, // pc 2
        Op::Const{dst:2,value:64,next:4}, // pc 3
        Op::Write{address:2,value:0,next:5}, // pc 4
        Op::Const{dst:2,value:21,next:6}, // pc 5
        Op::Read{dst:3,address:2,next:7}, // pc 6
        Op::Const{dst:2,value:2,next:8}, // pc 7
        Op::Sub{dst:3,lhs:3,rhs:2,next:9}, // pc 8
        Op::Branch{test:3,nonzero:13,zero:10}, // pc 9
        Op::Const{dst:2,value:64,next:11}, // pc 10
        Op::Const{dst:3,value:4,next:12}, // pc 11
        Op::Write{address:2,value:3,next:13}, // pc 12
        Op::Const{dst:2,value:50,next:14}, // pc 13
        Op::Write{address:2,value:0,next:15}, // pc 14
        Op::Const{dst:2,value:56,next:16}, // pc 15
        Op::Write{address:2,value:0,next:17}, // pc 16
        Op::Const{dst:2,value:50,next:18}, // pc 17
        Op::Read{dst:3,address:2,next:19}, // pc 18
        Op::Const{dst:2,value:62,next:20}, // pc 19
        Op::Add{dst:2,lhs:2,rhs:3,next:21}, // pc 20
        Op::Read{dst:3,address:2,next:22}, // pc 21
        Op::Const{dst:2,value:3,next:23}, // pc 22
        Op::Sub{dst:1,lhs:3,rhs:2,next:24}, // pc 23
        Op::Branch{test:1,nonzero:25,zero:28}, // pc 24
        Op::Const{dst:2,value:4,next:26}, // pc 25
        Op::Sub{dst:1,lhs:3,rhs:2,next:27}, // pc 26
        Op::Branch{test:1,nonzero:29,zero:176}, // pc 27
        Op::Const{dst:1,value:0,next:40}, // pc 28
        Op::Const{dst:2,value:56,next:30}, // pc 29
        Op::Read{dst:1,address:2,next:31}, // pc 30
        Op::Add{dst:1,lhs:1,rhs:3,next:32}, // pc 31
        Op::Const{dst:2,value:3,next:33}, // pc 32
        Op::Sub{dst:3,lhs:1,rhs:2,next:34}, // pc 33
        Op::Branch{test:3,nonzero:36,zero:35}, // pc 34
        Op::Const{dst:1,value:0,next:40}, // pc 35
        Op::Const{dst:2,value:4,next:37}, // pc 36
        Op::Sub{dst:3,lhs:1,rhs:2,next:38}, // pc 37
        Op::Branch{test:3,nonzero:40,zero:39}, // pc 38
        Op::Const{dst:1,value:1,next:40}, // pc 39
        Op::Const{dst:2,value:56,next:41}, // pc 40
        Op::Write{address:2,value:1,next:42}, // pc 41
        Op::Const{dst:2,value:50,next:43}, // pc 42
        Op::Read{dst:3,address:2,next:44}, // pc 43
        Op::Const{dst:2,value:1,next:45}, // pc 44
        Op::Add{dst:3,lhs:3,rhs:2,next:46}, // pc 45
        Op::Const{dst:2,value:50,next:47}, // pc 46
        Op::Write{address:2,value:3,next:48}, // pc 47
        Op::Const{dst:2,value:3,next:49}, // pc 48
        Op::Sub{dst:3,lhs:3,rhs:2,next:50}, // pc 49
        Op::Branch{test:3,nonzero:17,zero:51}, // pc 50
        Op::Const{dst:2,value:56,next:52}, // pc 51
        Op::Read{dst:3,address:2,next:53}, // pc 52
        Op::Const{dst:2,value:33,next:54}, // pc 53
        Op::Write{address:2,value:3,next:55}, // pc 54
        Op::Const{dst:2,value:33,next:56}, // pc 55
        Op::Read{dst:3,address:2,next:57}, // pc 56
        Op::Branch{test:3,nonzero:60,zero:58}, // pc 57
        Op::Const{dst:1,value:1,next:59}, // pc 58
        Op::Const{dst:3,value:0,next:66}, // pc 59
        Op::Const{dst:1,value:0,next:61}, // pc 60
        Op::Const{dst:2,value:1,next:62}, // pc 61
        Op::Sub{dst:3,lhs:3,rhs:2,next:63}, // pc 62
        Op::Branch{test:3,nonzero:64,zero:65}, // pc 63
        Op::Const{dst:3,value:0,next:66}, // pc 64
        Op::Const{dst:3,value:2,next:66}, // pc 65
        Op::Const{dst:2,value:52,next:67}, // pc 66
        Op::Write{address:2,value:1,next:68}, // pc 67
        Op::Const{dst:2,value:53,next:69}, // pc 68
        Op::Write{address:2,value:3,next:70}, // pc 69
        Op::Const{dst:2,value:7,next:71}, // pc 70
        Op::Read{dst:1,address:2,next:72}, // pc 71
        Op::Const{dst:2,value:52,next:73}, // pc 72
        Op::Read{dst:3,address:2,next:74}, // pc 73
        Op::Branch{test:3,nonzero:77,zero:75}, // pc 74
        Op::Const{dst:2,value:0,next:76}, // pc 75
        Op::Sub{dst:1,lhs:2,rhs:1,next:77}, // pc 76
        Op::Const{dst:2,value:53,next:78}, // pc 77
        Op::Read{dst:3,address:2,next:79}, // pc 78
        Op::Add{dst:1,lhs:1,rhs:3,next:80}, // pc 79
        Op::Const{dst:2,value:56,next:81}, // pc 80
        Op::Write{address:2,value:1,next:82}, // pc 81
        Op::Const{dst:2,value:11,next:83}, // pc 82
        Op::Write{address:2,value:1,next:84}, // pc 83
        Op::Const{dst:2,value:32,next:85}, // pc 84
        Op::Read{dst:3,address:2,next:86}, // pc 85
        Op::Const{dst:2,value:2,next:87}, // pc 86
        Op::Sub{dst:3,lhs:3,rhs:2,next:88}, // pc 87
        Op::Branch{test:3,nonzero:89,zero:162}, // pc 88
        Op::Const{dst:2,value:20,next:90}, // pc 89
        Op::Read{dst:1,address:2,next:91}, // pc 90
        Op::Branch{test:1,nonzero:92,zero:97}, // pc 91
        Op::Const{dst:2,value:61,next:93}, // pc 92
        Op::Read{dst:3,address:2,next:94}, // pc 93
        Op::Const{dst:1,value:1,next:95}, // pc 94
        Op::Add{dst:3,lhs:3,rhs:1,next:96}, // pc 95
        Op::Write{address:2,value:3,next:97}, // pc 96
        Op::Const{dst:2,value:32,next:98}, // pc 97
        Op::Read{dst:3,address:2,next:99}, // pc 98
        Op::Const{dst:1,value:1,next:100}, // pc 99
        Op::Add{dst:3,lhs:3,rhs:1,next:101}, // pc 100
        Op::Write{address:2,value:3,next:102}, // pc 101
        Op::Const{dst:2,value:2,next:103}, // pc 102
        Op::Sub{dst:3,lhs:3,rhs:2,next:104}, // pc 103
        Op::Branch{test:3,nonzero:70,zero:105}, // pc 104
        Op::Const{dst:2,value:61,next:106}, // pc 105
        Op::Read{dst:3,address:2,next:107}, // pc 106
        Op::Const{dst:2,value:2,next:108}, // pc 107
        Op::Sub{dst:3,lhs:3,rhs:2,next:109}, // pc 108
        Op::Branch{test:3,nonzero:131,zero:110}, // pc 109
        Op::Const{dst:2,value:21,next:111}, // pc 110
        Op::Read{dst:3,address:2,next:112}, // pc 111
        Op::Const{dst:2,value:1,next:113}, // pc 112
        Op::Sub{dst:3,lhs:3,rhs:2,next:114}, // pc 113
        Op::Branch{test:3,nonzero:124,zero:115}, // pc 114
        Op::Const{dst:2,value:33,next:116}, // pc 115
        Op::Read{dst:3,address:2,next:117}, // pc 116
        Op::Const{dst:2,value:62,next:118}, // pc 117
        Op::Write{address:2,value:3,next:119}, // pc 118
        Op::Const{dst:2,value:64,next:120}, // pc 119
        Op::Write{address:2,value:3,next:121}, // pc 120
        Op::Const{dst:2,value:63,next:122}, // pc 121
        Op::Const{dst:3,value:3,next:123}, // pc 122
        Op::Write{address:2,value:3,next:159}, // pc 123
        Op::Const{dst:2,value:33,next:125}, // pc 124
        Op::Read{dst:3,address:2,next:126}, // pc 125
        Op::Const{dst:2,value:56,next:127}, // pc 126
        Op::Write{address:2,value:3,next:128}, // pc 127
        Op::Write{address:2,value:3,next:129}, // pc 128
        Op::Const{dst:3,value:3,next:130}, // pc 129
        Op::Write{address:2,value:3,next:159}, // pc 130
        Op::Const{dst:2,value:63,next:132}, // pc 131
        Op::Read{dst:3,address:2,next:133}, // pc 132
        Op::Const{dst:1,value:1,next:134}, // pc 133
        Op::Add{dst:3,lhs:3,rhs:1,next:135}, // pc 134
        Op::Const{dst:2,value:4,next:136}, // pc 135
        Op::Sub{dst:1,lhs:3,rhs:2,next:137}, // pc 136
        Op::Branch{test:1,nonzero:139,zero:138}, // pc 137
        Op::Const{dst:3,value:0,next:139}, // pc 138
        Op::Const{dst:2,value:63,next:140}, // pc 139
        Op::Write{address:2,value:3,next:141}, // pc 140
        Op::Const{dst:2,value:65,next:142}, // pc 141
        Op::Read{dst:3,address:2,next:143}, // pc 142
        Op::Const{dst:1,value:1,next:144}, // pc 143
        Op::Add{dst:3,lhs:3,rhs:1,next:145}, // pc 144
        Op::Write{address:2,value:3,next:146}, // pc 145
        Op::Const{dst:2,value:3,next:147}, // pc 146
        Op::Sub{dst:3,lhs:3,rhs:2,next:148}, // pc 147
        Op::Branch{test:3,nonzero:149,zero:153}, // pc 148
        Op::Const{dst:2,value:32,next:150}, // pc 149
        Op::Write{address:2,value:0,next:151}, // pc 150
        Op::Const{dst:2,value:61,next:152}, // pc 151
        Op::Write{address:2,value:0,next:13}, // pc 152
        Op::Const{dst:2,value:63,next:154}, // pc 153
        Op::Const{dst:3,value:3,next:155}, // pc 154
        Op::Write{address:2,value:3,next:156}, // pc 155
        Op::Const{dst:2,value:32,next:157}, // pc 156
        Op::Const{dst:3,value:2,next:158}, // pc 157
        Op::Write{address:2,value:3,next:13}, // pc 158
        Op::Const{dst:2,value:32,next:160}, // pc 159
        Op::Const{dst:3,value:2,next:161}, // pc 160
        Op::Write{address:2,value:3,next:70}, // pc 161
        Op::Const{dst:2,value:31,next:163}, // pc 162
        Op::Read{dst:3,address:2,next:164}, // pc 163
        Op::Const{dst:1,value:1,next:165}, // pc 164
        Op::Add{dst:3,lhs:3,rhs:1,next:166}, // pc 165
        Op::Write{address:2,value:3,next:167}, // pc 166
        Op::Const{dst:2,value:2,next:168}, // pc 167
        Op::Sub{dst:3,lhs:3,rhs:2,next:169}, // pc 168
        Op::Branch{test:3,nonzero:170,zero:176}, // pc 169
        Op::Const{dst:2,value:32,next:171}, // pc 170
        Op::Write{address:2,value:0,next:172}, // pc 171
        Op::Const{dst:2,value:61,next:173}, // pc 172
        Op::Write{address:2,value:0,next:174}, // pc 173
        Op::Const{dst:2,value:65,next:175}, // pc 174
        Op::Write{address:2,value:0,next:13}, // pc 175
        Op::Halt, // pc 176
    ]
}
#[derive(Copy,Clone,Debug,Eq,PartialEq)]
enum Task{Negate,Composed,Identity,Unreachable}
impl Task{
 fn label(self)->&'static str{match self{
  Self::Negate=>"NEGATE",Self::Composed=>"COMPOSED",
  Self::Identity=>"IDENTITY",Self::Unreachable=>"UNREACHABLE"}}
 fn target(self,x:i64)->i64{match self{
  Self::Negate=>-x,Self::Composed=>-x+2,
  Self::Identity=>x,Self::Unreachable=>x+4}}
}
const SCHEDULES:[[Task;2];6]=[
 [Task::Negate,Task::Negate],
 [Task::Composed,Task::Negate],
 [Task::Negate,Task::Composed],
 [Task::Identity,Task::Unreachable],
 [Task::Negate,Task::Identity],
 [Task::Identity,Task::Composed]
];
#[derive(Copy,Clone)]
struct Seed{probes:[i64;2],heldout:i64}
const SEEDS:[Seed;2]=[
 Seed{probes:[-7,5],heldout:11},
 Seed{probes:[5,-7],heldout:-11}
];
#[derive(Copy,Clone)]
enum Signal{Truth,BlindSuccess,Inverted,Absent,Phase0CDecoy}
impl Signal{
 fn label(self)->&'static str{match self{
  Self::Truth=>"TRUTH",Self::BlindSuccess=>"BLIND_SUCCESS",
  Self::Inverted=>"INVERTED",Self::Absent=>"ABSENT",
  Self::Phase0CDecoy=>"PHASE0_C_DECOY"}}
 fn bit(self,correct:i64,phase:i64,mode:i64)->Option<i64>{match self{
  Self::Truth=>Some(correct),Self::BlindSuccess=>Some(1),
  Self::Inverted=>Some(1-correct),Self::Absent=>None,
  Self::Phase0CDecoy=>Some(if phase==0&&mode==1{1}else{correct})
 }}
}
fn emit(out:&mut BufWriter<File>,data:Value){
 serde_json::to_writer(&mut *out,&data).unwrap();
 out.write_all(b"\n").unwrap();
}
fn fnv(code:&[Op])->String{
 let buf=serde_json::to_vec(code).unwrap();
 let hash=buf.iter().fold(14695981039346656037_u64,|h,x|{
  (h^u64::from(*x)).wrapping_mul(1099511628211)
 });
 format!("{hash:016x}")
}
fn state(w:&World,hash:&str)->Value{
 let p=w.process(1).unwrap();
 json!([p.pc,p.registers,p.credits,p.stopped,p.code.nodes.len(),hash])
}
fn mem(w:&World)->Value{json!(KEYS.map(|key|w.cell(key)))}
fn external(out:&mut BufWriter<File>,w:&mut World,id:usize,key:i64,val:i64,
 reason:&str,hash:&str){
 assert!(matches!(key,7|20|21),"environment attempted privileged data edit");
 w.intervene(Intervention::SetCell{key,value:val}).unwrap();
 let e=w.events().last().unwrap();
 assert!(matches!(e.origin,Origin::External));
 assert!(w.used_cells()<=16&&w.occupied_units()<=300);
 emit(out,json!(["ext",id,e.tick,key,val,reason,state(w,hash),
  w.occupied_units(),mem(w)]));
}
fn world(out:&mut BufWriter<File>,id:usize,seed_id:usize,sid:usize,
 signal:Signal,gate:i64)->Value{
 let founder=founder();
 assert_eq!(founder.len(),177);
 let hash=fnv(&founder);
 let seed=SEEDS[seed_id];
 let schedule=SCHEDULES[sid];
 let mut w=World::with_strict_capacity(16,180,300);
 w.load(Process::new(1,founder.clone(),CREDITS)).unwrap();
 emit(out,json!(["case",id,seed_id,sid,signal.label(),gate,
  schedule.map(|x|x.label()),seed.probes,seed.heldout,
  founder,hash,w.occupied_units(),CREDITS,16,180,300]));
 external(out,&mut w,id,21,gate,"initial_rule_control",&hash);
 external(out,&mut w,id,7,seed.probes[0],"first_probe_input",&hash);
 let mut paid=0_usize;
 let mut environment_events=2_usize;
 let mut peak=w.occupied_units();
 let mut service_outputs=Vec::<Value>::new();
 let mut meta_fetch=Vec::<Value>::new();
 let mut meta_checkpoints=0_usize;
 let mut meta_programs=Vec::<Value>::new();
 let mut task_fetch_count=0_usize;
 let mut task_checkpoint_count=0_usize;
 let mut task_writes=Vec::<Value>::new();
 let mut meta_writes=Vec::<Value>::new();
 let mut meta_shadows=0_usize;
 let mut attempt_writes=0_usize;
 let mut feedback_reads=0_usize;
 let mut score_writes=0_usize;
 let mut bad4_writes=0_usize;
 let mut source_bad_halt=false;
 let mut source_initial_rule_writes=0_usize;
 let mut fallback_count=0_usize;
 let mut phase_heldout_paid=Vec::<usize>::new();
 while w.step(){
  paid+=1;
  assert!(paid<=CREDITS as usize,"PID1 native budget exhausted");
  let e=w.events().last().unwrap().clone();
  assert!(matches!(e.origin,Origin::Internal(1)),"original source PID1 origin violated");
  assert!(w.process(2).is_none(),"invalid PID2");
  assert_eq!(w.process(1).unwrap().code.nodes,founder,
   "native executable source mutated");
  assert_eq!(w.total_compute_inventory(),Some(CREDITS-paid as u64));
  assert!(w.used_cells()<=16&&w.occupied_units()<=300);
  peak=peak.max(w.occupied_units());
  if e.pc==INIT_BASE_WRITE{
   assert!(matches!(&e.kind,EventKind::CellWritten{key:62,value:1}));
   source_initial_rule_writes+=1;
  }
  if e.pc==INIT_TAIL_WRITE{
   assert!(matches!(&e.kind,EventKind::CellWritten{key:64,value:0}));
   source_initial_rule_writes+=1;
  }
  if e.pc==BAD4_WRITE{
   assert_eq!(gate,2);
   assert!(matches!(&e.kind,EventKind::CellWritten{key:64,value:4}));
   bad4_writes+=1;
  }
  if e.pc==META_OPCODE_FETCH{
   let vpc=w.cell(50).unwrap();
   assert!((0..=2).contains(&vpc));
   let key=62+vpc;
   if let EventKind::CellRead{key:k,value}=&e.kind{
    assert_eq!(*k,key,"dynamic meta-PC read is not address62+PC");
    assert_eq!(*value,w.cell(key).unwrap_or(0));
    meta_fetch.push(json!([w.cell(31),vpc,key,value,e.tick]));
   }else{panic!("meta opcode fetch must pay native Read");}
  }
  if e.pc==META_CHECKPOINT{
   assert!(matches!(&e.kind,EventKind::CellWritten{key:56,..}));
   meta_checkpoints+=1;
  }
  if e.pc==META_MODE_WRITE{
   if let EventKind::CellWritten{key:33,value}=&e.kind{
    assert!((0..=2).contains(value));
    meta_programs.push(json!([w.cell(31),w.cell(62),w.cell(63),w.cell(64),value,e.tick]));
   }else{panic!("meta result must be PID1 paid Write33");}
  }
  if e.pc==TASK_FETCH1||e.pc==TASK_FETCH2{
   assert!(matches!(&e.kind,EventKind::CellRead{key,..}
    if *key==if e.pc==TASK_FETCH1{52}else{53}));
   task_fetch_count+=1;
  }
  if e.pc==TASK_CHECKPOINT{
   assert!(matches!(&e.kind,EventKind::CellWritten{key:56,..}));
   task_checkpoint_count+=1;
  }
  if e.pc==TASK_WRITE52||e.pc==TASK_WRITE53{
   if let EventKind::CellWritten{key,value}=&e.kind{
    assert_eq!(*key,if e.pc==TASK_WRITE52{52}else{53});
    task_writes.push(json!([w.cell(31),w.cell(33),key,value,e.tick]));
   }else{panic!("PID1 task-program edit missing");}
  }
  if [PAID_BASE_WRITE,PAID_TAIL_WRITE,PAID_RESET_OFFSET,TEMP_OFFSET_WRITE,
      FALLBACK_OFFSET_WRITE].contains(&e.pc){
   if let EventKind::CellWritten{key,value}=&e.kind{
    let want=if e.pc==PAID_BASE_WRITE{62}else if e.pc==PAID_TAIL_WRITE{64}else{63};
    assert_eq!(*key,want);
    if [PAID_BASE_WRITE,PAID_TAIL_WRITE,PAID_RESET_OFFSET].contains(&e.pc) {
     assert_eq!(gate,1);
    }
    meta_writes.push(json!([w.cell(31),w.cell(33),key,value,e.pc,e.tick]));
    if e.pc==FALLBACK_OFFSET_WRITE{fallback_count+=1;}
   }else{panic!("source-paid G2S rule edit absent");}
  }
  if [SHADOW1,SHADOW2,SHADOW3].contains(&e.pc){
   assert_eq!(gate,0);
   assert!(matches!(&e.kind,EventKind::CellWritten{key:56,..}));
   meta_shadows+=1;
  }
  if e.pc==ATTEMPT_COUNT_WRITE{
   assert!(matches!(&e.kind,EventKind::CellWritten{key:65,..}));
   attempt_writes+=1;
  }
  if e.pc==FEEDBACK_READ{
   assert!(matches!(&e.kind,EventKind::CellRead{key:20,value}
     if *value==w.cell(20).unwrap_or(0)));
   feedback_reads+=1;
  }
  if e.pc==SCORE_WRITE{
   assert!(matches!(&e.kind,EventKind::CellWritten{key:61,..}));
   score_writes+=1;
  }
  let mut probe:Option<(usize,usize,i64,i64)>=None;
  let mut heldout_phase:Option<usize>=None;
  if let EventKind::CellWritten{key:11,value}=&e.kind{
   assert_eq!(e.pc,SERVICE_WRITE);
   let phase=w.cell(31).unwrap_or(0);
   let stage=w.cell(32).unwrap_or(0);
   let mode=w.cell(33).unwrap();
   assert!((0..=1).contains(&phase)&&(0..=2).contains(&stage)
      &&(0..=2).contains(&mode));
   service_outputs.push(json!([phase,stage,mode,value,w.cell(62),
     w.cell(63),w.cell(64),w.cell(52),w.cell(53)]));
   if stage<2{
    probe=Some((phase as usize,stage as usize,mode,*value));
   }else{
    heldout_phase=Some(phase as usize);
    phase_heldout_paid.push(paid);
   }
  }
  if e.pc==HALT_PC {
   if gate==2{
    assert!(w.process(1).unwrap().stopped);
    assert!(matches!(&e.kind,EventKind::Halted));
    source_bad_halt=true;
   }
  }
  emit(out,json!(["step",id,paid,e.tick,1,e.pc,e.kind,
    state(&w,&hash),w.occupied_units(),mem(&w)]));
  if let Some((phase,stage,mode,actual))=probe{
   let expected=schedule[phase].target(seed.probes[stage]);
   let truth=i64::from(actual==expected);
   if let Some(bit)=signal.bit(truth,phase as i64,mode){
    external(out,&mut w,id,20,bit,"paid_probe_feedback",&hash);
    environment_events+=1;
   }
   external(out,&mut w,id,7,
    if stage==0{seed.probes[1]}else{seed.probes[0]},
    "next_probe_input",&hash);
   environment_events+=1;
  }
  if e.pc==HOLD_WRITE||e.pc==FALLBACK_HOLD_WRITE{
   assert!(matches!(&e.kind,EventKind::CellWritten{key:32,value:2}));
   external(out,&mut w,id,7,seed.heldout,
    if e.pc==HOLD_WRITE{"valid_candidate_heldout"}else{"fallback_heldout"},&hash);
   environment_events+=1;
  }
  if let Some(phase)=heldout_phase{
   if phase==0{
    external(out,&mut w,id,7,seed.probes[0],"next_hidden_task_probe0",&hash);
    environment_events+=1;
   }
  }
 }
 assert!(w.process(1).unwrap().stopped);
 assert!(w.process(2).is_none());
 assert_eq!(source_initial_rule_writes,2);
 assert!(peak<=194);
 let heldout=service_outputs.iter().filter(|x|x[1]==2).cloned().collect::<Vec<_>>();
 let phase_results=heldout.iter().map(|r|{
  let phase=r[0].as_u64().unwrap() as usize;
  let output=r[3].as_i64().unwrap();
  json!({"phase":phase,"task":schedule[phase].label(),
   "mode":r[2],"output":output,
   "expected":schedule[phase].target(seed.heldout),
   "correct":output==schedule[phase].target(seed.heldout),
   "meta_bytecode":[r[4],r[5],r[6]],
   "task_bytecode":[r[7],r[8]]})
 }).collect::<Vec<_>>();
 let correct=phase_results.iter().filter(|r|r["correct"]==true).count();
 if gate==2{
  assert!(source_bad_halt,"actor generated invalid4 must actually Halt");
  assert_eq!(bad4_writes,1);
  assert_eq!(service_outputs.len(),0,"invalid meta source must fail before service");
  assert_eq!(feedback_reads,0);
  assert_eq!(meta_fetch.len(),3);
  assert_eq!(meta_fetch[2][3],4);
  assert!(phase_results.is_empty());
  assert!(meta_writes.is_empty());
  assert_eq!(w.cell(64),Some(4));
 }else{
  assert_eq!(w.cell(31),Some(2),"both hidden task phases must complete");
  assert_eq!(phase_results.len(),2);
  assert_eq!(phase_heldout_paid.len(),2);
  assert!(meta_fetch.len()>=6&&meta_fetch.len()%3==0);
  assert_eq!(meta_checkpoints,meta_fetch.len());
  assert_eq!(meta_programs.len()*3,meta_fetch.len());
  assert_eq!(task_fetch_count,service_outputs.len()*2);
  assert_eq!(task_checkpoint_count,service_outputs.len());
  if gate==0{
   assert_eq!(meta_shadows%3,0);
   assert!(meta_writes.iter().all(|x|x[2]==63));
   assert_eq!(w.cell(62),Some(1));
   assert_eq!(w.cell(64),Some(0));
  }else{
   assert_eq!(meta_shadows,0);
  }
  if matches!(signal,Signal::Truth)&&gate==1{
   const EXPECTED:[usize;6]=[2,2,2,1,1,2];
   assert_eq!(correct,EXPECTED[sid]);
   if sid==4{
    assert_eq!(phase_results[1]["correct"],false);
    assert_eq!(phase_results[1]["mode"],2);
    assert_eq!(phase_results[1]["expected"],seed.heldout);
    assert!(fallback_count>0);
   }
   if sid==3{
    assert_eq!(phase_results[1]["correct"],false);
   }
  }
 }
 let phase_costs=if phase_heldout_paid.len()==2{
   json!([phase_heldout_paid[0],phase_heldout_paid[1]-phase_heldout_paid[0]])
 }else{json!([])};
 let result=json!({
  "id":id,"seed":seed_id,"schedule":sid,"regime":signal.label(),"gate":gate,
  "real_original_PID1_service_outputs":service_outputs,
  "true_heldout_by_phase":phase_results,
  "true_heldout_correct":correct,
  "paid_meta_instruction_fetches":meta_fetch,
  "paid_meta_intermediate_checkpoints":meta_checkpoints,
  "charged_three_instruction_meta_compositions":meta_programs,
  "charged_task_op_fetches":task_fetch_count,
  "paid_task_accumulator_checkpoints":task_checkpoint_count,
  "real_task_program_slot_writes":task_writes,
  "actor_paid_meta_data_writes":meta_writes,
  "paid_meta_write_ablations":meta_shadows,
  "source_paid_candidate_attempt_writes":attempt_writes,
  "source_paid_feedback_reads":feedback_reads,
  "source_paid_score_writes":score_writes,
  "source_generated_bad4_writes":bad4_writes,
  "source_generated_bad4_halt":source_bad_halt,
  "source_paid_fallback_entries":fallback_count,
  "phase_costs_to_real_heldout":phase_costs,
  "paid_CIL_instructions":paid,
  "external_interventions":environment_events,
  "measured_peak_storage":peak,
  "final_original_PID1":state(&w,&hash),
  "final_present_or_missing_cells":mem(&w),
  "remaining_private_credits":w.remaining_credits(),
  "unaltered_original_PID1":true,"native_source_unchanged":true,
  "preauthored_meta_grammar":true,
  "universal_bad_opcode_handling":false,
  "full_Hk":false,"B3":false,"B4":false,"cyberlife":false
 });
 emit(out,json!(["end",id,result]));
 result
}
#[test]
fn b56_f1d1k_g2s_noncommuting_rule_structure_and_paid_bad4(){
 let dir=std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
   .join("../../cl1b-ci-evidence");
 fs::create_dir_all(&dir).unwrap();
 let mut out=BufWriter::new(File::create(dir.join(
   "b56_f1d1k_g2s_native_noncommuting_and_bad4.jsonl")).unwrap());
 let manifest=std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(
   "../../../cl1b/studies/B56-F1d1k-G2S-native-180-world-and-noncommuting-RESET-preregistration.md");
 let json_spec=std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(
   "../../../cl1b/studies/B56-F1d1k-G2S-frozen-exact-177-node-source-v1.json");
 let sha256_file=|path:&std::path::Path|{
  let r=std::process::Command::new("sha256sum").arg(path).output().unwrap();
  assert!(r.status.success());
  String::from_utf8(r.stdout).unwrap().split_whitespace().next().unwrap().to_string()
 };
 let buf=fs::read(&json_spec).unwrap();
 let frozen:Value=serde_json::from_slice(&buf).unwrap();
 let src=founder();
 assert_eq!(src.len(),177);
 for (i,real) in src.iter().enumerate(){
  let node=&frozen["code"][i];
  let kind=node["kind"].as_str().unwrap();
  let expect=if kind=="Halt"{json!("Halt")}else{
   let mut inner=serde_json::Map::new();
   for (k,v) in node.as_object().unwrap(){
    if k!="kind"{inner.insert(k.to_string(),v.clone());}
   }
   let mut outer=serde_json::Map::new();
   outer.insert(kind.to_string(),Value::Object(inner));
   Value::Object(outer)
  };
  assert_eq!(serde_json::to_value(real).unwrap(),expect,
   "original native founder opcode differs from prospective frozen PC {i}");
 }
 assert_eq!(frozen["allowed_cell_keys"],json!(KEYS));
 assert_eq!(frozen["resources"]["starting_credits"],CREDITS);
 assert_eq!(frozen["resources"]["max_data_cells"],16);
 assert_eq!(frozen["resources"]["max_CIL_nodes"],180);
 assert_eq!(frozen["resources"]["max_occupied_units"],300);
 assert_eq!(frozen["total_original_worlds"],180);
 emit(&mut out,json!(["meta","B56-F1d1k-G2S",1,
  std::env::var("GITHUB_SHA").unwrap_or_else(|_|"local".to_owned()),
  sha256_file(&manifest),fs::metadata(&manifest).unwrap().len(),
  sha256_file(&json_spec),buf.len(),
  180,CREDITS,16,180,300,src.len(),META_OPCODE_FETCH,
  META_CHECKPOINT,META_MODE_WRITE,TASK_FETCH1,TASK_FETCH2,
  TASK_CHECKPOINT,TASK_WRITE52,TASK_WRITE53,SERVICE_WRITE,
  FEEDBACK_READ,SCORE_WRITE,PAID_BASE_WRITE,PAID_TAIL_WRITE,
  PAID_RESET_OFFSET,SHADOW1,SHADOW2,SHADOW3,TEMP_OFFSET_WRITE,
  ATTEMPT_COUNT_WRITE,FALLBACK_OFFSET_WRITE,HOLD_WRITE,
  FALLBACK_HOLD_WRITE,BAD4_WRITE,INIT_BASE_WRITE,INIT_TAIL_WRITE,HALT_PC]));
 let mut records=Vec::<Value>::new();
 for seed in 0..2{
  for sid in 0..6{
   for signal in [Signal::Truth,Signal::BlindSuccess,Signal::Inverted,
    Signal::Absent,Signal::Phase0CDecoy]{
    for gate in [1_i64,0,2]{
     let id=records.len();
     records.push(world(&mut out,id,seed,sid,signal,gate));
    }
   }
  }
 }
 out.flush().unwrap();
 assert_eq!(records.len(),180);
 let pick=|seed:usize,sid:usize,sig:&str,gate:i64|->&Value{
  records.iter().find(|v|v["seed"]==seed&&v["schedule"]==sid
   &&v["regime"]==sig&&v["gate"]==gate).unwrap()
 };
 for seed in 0..2{
  let s4=pick(seed,4,"TRUTH",1);
  let s4control=pick(seed,4,"TRUTH",0);
  assert_eq!(s4["true_heldout_correct"],1);
  assert_eq!(s4control["true_heldout_correct"],2);
  let phase1_modes=s4["charged_three_instruction_meta_compositions"]
    .as_array().unwrap().iter().filter(|e|e[0]==1)
    .map(|e|e[4].as_i64().unwrap()).collect::<Vec<_>>();
  assert_eq!(phase1_modes,vec![2,1,2,2],
   "learned three-word B,RESET,B excludes reachable A");
  let s5=pick(seed,5,"TRUTH",1);
  let s5modes=s5["charged_three_instruction_meta_compositions"]
    .as_array().unwrap().iter().filter(|e|e[0]==1)
    .map(|e|e[4].as_i64().unwrap()).collect::<Vec<_>>();
  assert_eq!(s5modes,vec![0,0,1],
   "learned A,RESET,A causes repeated A before C");
  let s2=pick(seed,2,"TRUTH",1);
  assert_eq!(s2["true_heldout_correct"],2);
  assert_eq!(pick(seed,2,"TRUTH",0)["true_heldout_correct"],1);
  for sid in 0..6{
   let invalid=pick(seed,sid,"TRUTH",2);
   assert_eq!(invalid["source_generated_bad4_halt"],true);
   assert_eq!(invalid["source_generated_bad4_writes"],1);
   assert_eq!(invalid["real_original_PID1_service_outputs"],json!([]));
  }
 }
 eprintln!("B56_F1D1K_G2S_NATIVE_NONCOMMUTING_AND_BAD4 {}",json!({
  "original_PID1_worlds":180,"native_CIL_nodes":177,"max_shared_data_cells":16,
  "source_identical_malformed4_cases":60,"normal_task_cases":120,
  "source_generated_invalid4_fails_before_any_service":true,
  "three_dynamic_meta_opcode_reads_per_program":true,
  "learned_B_RESET_B_excludes_reachable_A":true,
  "learned_A_RESET_A_repeats_A_before_C":true,
  "loss_of_search_completeness":true,
  "authored_meta_ISA":true,"full_Hk":false,
  "B3":false,"B4":false,"cyberlife":false
 }));
}
