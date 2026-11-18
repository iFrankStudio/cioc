//! CL-1B B56-F1d1k-G3P: native PID1-paid first-candidate duplicate detection,
//! source-paid repair of middle meta opcode, and real recovery of known A.
//! FROZEN 180-node founder; bounded authored grammar, NOT full G3 or cyberlife.
use cl0_world::{EventKind,Intervention,Op,Origin,Process,World};
use serde_json::{json,Value};
use std::fs::{self,File};
use std::io::{BufWriter,Write};

const CREDITS:u64=4000;
const KEYS:[i64;16]=[7,11,20,21,31,32,33,50,52,53,56,61,62,63,64,65];
const META_OPCODE_FETCH:usize=13;
const META_CHECKPOINT:usize=30;
const META_MODE_WRITE:usize=43;
const TASK_FETCH1:usize=86;
const TASK_FETCH2:usize=91;
const TASK_CHECKPOINT:usize=94;
const TASK_WRITE52:usize=80;
const TASK_WRITE53:usize=82;
const SERVICE_WRITE:usize=96;
const FEEDBACK_READ:usize=103;
const SCORE_WRITE:usize=109;
const PAID_BASE_WRITE:usize=126;
const PAID_TAIL_WRITE:usize=128;
const PAID_RESET_OFFSET:usize=131;
const PAID_DISABLE_SHADOW:usize=48;
const PAID_DUPLICATE_REPAIR:usize=67;
const TEMP_OFFSET_WRITE:usize=141;
const ATTEMPT_COUNT_WRITE:usize=146;
const FALLBACK_OFFSET_WRITE:usize=156;
const FALLBACK_ATTEMPT_RESET:usize=161;
const HOLD_WRITE:usize=164;
const FALLBACK_HOLD_WRITE:usize=159;
const INIT_BASE_WRITE:usize=2;
const INIT_TAIL_WRITE:usize=4;
const HALT_PC:usize=179;

fn founder()->Vec<Op>{
    vec![
        Op::Const{dst:2,value:62,next:1}, // pc 0
        Op::Const{dst:3,value:1,next:2}, // pc 1
        Op::Write{address:2,value:3,next:3}, // pc 2
        Op::Const{dst:2,value:64,next:4}, // pc 3
        Op::Write{address:2,value:0,next:5}, // pc 4
        Op::Const{dst:2,value:50,next:6}, // pc 5
        Op::Write{address:2,value:0,next:7}, // pc 6
        Op::Const{dst:2,value:56,next:8}, // pc 7
        Op::Write{address:2,value:0,next:9}, // pc 8
        Op::Const{dst:2,value:50,next:10}, // pc 9
        Op::Read{dst:3,address:2,next:11}, // pc 10
        Op::Const{dst:2,value:62,next:12}, // pc 11
        Op::Add{dst:2,lhs:2,rhs:3,next:13}, // pc 12
        Op::Read{dst:3,address:2,next:14}, // pc 13
        Op::Const{dst:2,value:3,next:15}, // pc 14
        Op::Sub{dst:1,lhs:3,rhs:2,next:16}, // pc 15
        Op::Branch{test:1,nonzero:18,zero:17}, // pc 16
        Op::Const{dst:1,value:0,next:29}, // pc 17
        Op::Const{dst:2,value:56,next:19}, // pc 18
        Op::Read{dst:1,address:2,next:20}, // pc 19
        Op::Add{dst:1,lhs:1,rhs:3,next:21}, // pc 20
        Op::Const{dst:2,value:3,next:22}, // pc 21
        Op::Sub{dst:3,lhs:1,rhs:2,next:23}, // pc 22
        Op::Branch{test:3,nonzero:25,zero:24}, // pc 23
        Op::Const{dst:1,value:0,next:29}, // pc 24
        Op::Const{dst:2,value:4,next:26}, // pc 25
        Op::Sub{dst:3,lhs:1,rhs:2,next:27}, // pc 26
        Op::Branch{test:3,nonzero:29,zero:28}, // pc 27
        Op::Const{dst:1,value:1,next:29}, // pc 28
        Op::Const{dst:2,value:56,next:30}, // pc 29
        Op::Write{address:2,value:1,next:31}, // pc 30
        Op::Const{dst:2,value:50,next:32}, // pc 31
        Op::Read{dst:3,address:2,next:33}, // pc 32
        Op::Const{dst:2,value:1,next:34}, // pc 33
        Op::Add{dst:3,lhs:3,rhs:2,next:35}, // pc 34
        Op::Const{dst:2,value:50,next:36}, // pc 35
        Op::Write{address:2,value:3,next:37}, // pc 36
        Op::Const{dst:2,value:3,next:38}, // pc 37
        Op::Sub{dst:3,lhs:3,rhs:2,next:39}, // pc 38
        Op::Branch{test:3,nonzero:9,zero:40}, // pc 39
        Op::Const{dst:2,value:56,next:41}, // pc 40
        Op::Read{dst:3,address:2,next:42}, // pc 41
        Op::Const{dst:2,value:33,next:43}, // pc 42
        Op::Write{address:2,value:3,next:44}, // pc 43
        Op::Const{dst:2,value:21,next:45}, // pc 44
        Op::Read{dst:3,address:2,next:46}, // pc 45
        Op::Branch{test:3,nonzero:49,zero:47}, // pc 46
        Op::Const{dst:2,value:56,next:48}, // pc 47
        Op::Write{address:2,value:0,next:68}, // pc 48
        Op::Const{dst:2,value:65,next:50}, // pc 49
        Op::Read{dst:3,address:2,next:51}, // pc 50
        Op::Branch{test:3,nonzero:52,zero:68}, // pc 51
        Op::Const{dst:2,value:33,next:53}, // pc 52
        Op::Read{dst:3,address:2,next:54}, // pc 53
        Op::Const{dst:2,value:62,next:55}, // pc 54
        Op::Read{dst:1,address:2,next:56}, // pc 55
        Op::Sub{dst:3,lhs:3,rhs:1,next:57}, // pc 56
        Op::Branch{test:3,nonzero:68,zero:58}, // pc 57
        Op::Const{dst:2,value:63,next:59}, // pc 58
        Op::Read{dst:3,address:2,next:60}, // pc 59
        Op::Const{dst:1,value:1,next:61}, // pc 60
        Op::Add{dst:3,lhs:3,rhs:1,next:62}, // pc 61
        Op::Const{dst:2,value:4,next:63}, // pc 62
        Op::Sub{dst:1,lhs:3,rhs:2,next:64}, // pc 63
        Op::Branch{test:1,nonzero:66,zero:65}, // pc 64
        Op::Const{dst:3,value:0,next:66}, // pc 65
        Op::Const{dst:2,value:63,next:67}, // pc 66
        Op::Write{address:2,value:3,next:5}, // pc 67
        Op::Const{dst:2,value:33,next:69}, // pc 68
        Op::Read{dst:3,address:2,next:70}, // pc 69
        Op::Branch{test:3,nonzero:73,zero:71}, // pc 70
        Op::Const{dst:1,value:1,next:72}, // pc 71
        Op::Const{dst:3,value:0,next:79}, // pc 72
        Op::Const{dst:1,value:0,next:74}, // pc 73
        Op::Const{dst:2,value:1,next:75}, // pc 74
        Op::Sub{dst:3,lhs:3,rhs:2,next:76}, // pc 75
        Op::Branch{test:3,nonzero:77,zero:78}, // pc 76
        Op::Const{dst:3,value:0,next:79}, // pc 77
        Op::Const{dst:3,value:2,next:79}, // pc 78
        Op::Const{dst:2,value:52,next:80}, // pc 79
        Op::Write{address:2,value:1,next:81}, // pc 80
        Op::Const{dst:2,value:53,next:82}, // pc 81
        Op::Write{address:2,value:3,next:83}, // pc 82
        Op::Const{dst:2,value:7,next:84}, // pc 83
        Op::Read{dst:1,address:2,next:85}, // pc 84
        Op::Const{dst:2,value:52,next:86}, // pc 85
        Op::Read{dst:3,address:2,next:87}, // pc 86
        Op::Branch{test:3,nonzero:90,zero:88}, // pc 87
        Op::Const{dst:2,value:0,next:89}, // pc 88
        Op::Sub{dst:1,lhs:2,rhs:1,next:90}, // pc 89
        Op::Const{dst:2,value:53,next:91}, // pc 90
        Op::Read{dst:3,address:2,next:92}, // pc 91
        Op::Add{dst:1,lhs:1,rhs:3,next:93}, // pc 92
        Op::Const{dst:2,value:56,next:94}, // pc 93
        Op::Write{address:2,value:1,next:95}, // pc 94
        Op::Const{dst:2,value:11,next:96}, // pc 95
        Op::Write{address:2,value:1,next:97}, // pc 96
        Op::Const{dst:2,value:32,next:98}, // pc 97
        Op::Read{dst:3,address:2,next:99}, // pc 98
        Op::Const{dst:2,value:2,next:100}, // pc 99
        Op::Sub{dst:3,lhs:3,rhs:2,next:101}, // pc 100
        Op::Branch{test:3,nonzero:102,zero:165}, // pc 101
        Op::Const{dst:2,value:20,next:103}, // pc 102
        Op::Read{dst:1,address:2,next:104}, // pc 103
        Op::Branch{test:1,nonzero:105,zero:110}, // pc 104
        Op::Const{dst:2,value:61,next:106}, // pc 105
        Op::Read{dst:3,address:2,next:107}, // pc 106
        Op::Const{dst:1,value:1,next:108}, // pc 107
        Op::Add{dst:3,lhs:3,rhs:1,next:109}, // pc 108
        Op::Write{address:2,value:3,next:110}, // pc 109
        Op::Const{dst:2,value:32,next:111}, // pc 110
        Op::Read{dst:3,address:2,next:112}, // pc 111
        Op::Const{dst:1,value:1,next:113}, // pc 112
        Op::Add{dst:3,lhs:3,rhs:1,next:114}, // pc 113
        Op::Write{address:2,value:3,next:115}, // pc 114
        Op::Const{dst:2,value:2,next:116}, // pc 115
        Op::Sub{dst:3,lhs:3,rhs:2,next:117}, // pc 116
        Op::Branch{test:3,nonzero:83,zero:118}, // pc 117
        Op::Const{dst:2,value:61,next:119}, // pc 118
        Op::Read{dst:3,address:2,next:120}, // pc 119
        Op::Const{dst:2,value:2,next:121}, // pc 120
        Op::Sub{dst:3,lhs:3,rhs:2,next:122}, // pc 121
        Op::Branch{test:3,nonzero:132,zero:123}, // pc 122
        Op::Const{dst:2,value:33,next:124}, // pc 123
        Op::Read{dst:3,address:2,next:125}, // pc 124
        Op::Const{dst:2,value:62,next:126}, // pc 125
        Op::Write{address:2,value:3,next:127}, // pc 126
        Op::Const{dst:2,value:64,next:128}, // pc 127
        Op::Write{address:2,value:3,next:129}, // pc 128
        Op::Const{dst:2,value:63,next:130}, // pc 129
        Op::Const{dst:3,value:3,next:131}, // pc 130
        Op::Write{address:2,value:3,next:162}, // pc 131
        Op::Const{dst:2,value:63,next:133}, // pc 132
        Op::Read{dst:3,address:2,next:134}, // pc 133
        Op::Const{dst:1,value:1,next:135}, // pc 134
        Op::Add{dst:3,lhs:3,rhs:1,next:136}, // pc 135
        Op::Const{dst:2,value:4,next:137}, // pc 136
        Op::Sub{dst:1,lhs:3,rhs:2,next:138}, // pc 137
        Op::Branch{test:1,nonzero:140,zero:139}, // pc 138
        Op::Const{dst:3,value:0,next:140}, // pc 139
        Op::Const{dst:2,value:63,next:141}, // pc 140
        Op::Write{address:2,value:3,next:142}, // pc 141
        Op::Const{dst:2,value:65,next:143}, // pc 142
        Op::Read{dst:3,address:2,next:144}, // pc 143
        Op::Const{dst:1,value:1,next:145}, // pc 144
        Op::Add{dst:3,lhs:3,rhs:1,next:146}, // pc 145
        Op::Write{address:2,value:3,next:147}, // pc 146
        Op::Const{dst:2,value:3,next:148}, // pc 147
        Op::Sub{dst:3,lhs:3,rhs:2,next:149}, // pc 148
        Op::Branch{test:3,nonzero:150,zero:154}, // pc 149
        Op::Const{dst:2,value:32,next:151}, // pc 150
        Op::Write{address:2,value:0,next:152}, // pc 151
        Op::Const{dst:2,value:61,next:153}, // pc 152
        Op::Write{address:2,value:0,next:5}, // pc 153
        Op::Const{dst:2,value:63,next:155}, // pc 154
        Op::Const{dst:3,value:3,next:156}, // pc 155
        Op::Write{address:2,value:3,next:157}, // pc 156
        Op::Const{dst:2,value:32,next:158}, // pc 157
        Op::Const{dst:3,value:2,next:159}, // pc 158
        Op::Write{address:2,value:3,next:160}, // pc 159
        Op::Const{dst:2,value:65,next:161}, // pc 160
        Op::Write{address:2,value:0,next:5}, // pc 161
        Op::Const{dst:2,value:32,next:163}, // pc 162
        Op::Const{dst:3,value:2,next:164}, // pc 163
        Op::Write{address:2,value:3,next:83}, // pc 164
        Op::Const{dst:2,value:31,next:166}, // pc 165
        Op::Read{dst:3,address:2,next:167}, // pc 166
        Op::Const{dst:1,value:1,next:168}, // pc 167
        Op::Add{dst:3,lhs:3,rhs:1,next:169}, // pc 168
        Op::Write{address:2,value:3,next:170}, // pc 169
        Op::Const{dst:2,value:2,next:171}, // pc 170
        Op::Sub{dst:3,lhs:3,rhs:2,next:172}, // pc 171
        Op::Branch{test:3,nonzero:173,zero:179}, // pc 172
        Op::Const{dst:2,value:32,next:174}, // pc 173
        Op::Write{address:2,value:0,next:175}, // pc 174
        Op::Const{dst:2,value:61,next:176}, // pc 175
        Op::Write{address:2,value:0,next:177}, // pc 176
        Op::Const{dst:2,value:65,next:178}, // pc 177
        Op::Write{address:2,value:0,next:5}, // pc 178
        Op::Halt, // pc 179
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
 assert_eq!(founder.len(),180);
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
 let mut guard_bypass_shadows=0_usize;
 let mut attempt_writes=0_usize;
 let mut feedback_reads=0_usize;
 let mut score_writes=0_usize;
 let mut paid_duplicate_repairs=Vec::<Value>::new();
 let mut fallback_guard_resets=0_usize;
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
      FALLBACK_OFFSET_WRITE,PAID_DUPLICATE_REPAIR].contains(&e.pc){
   if let EventKind::CellWritten{key,value}=&e.kind{
    let want=if e.pc==PAID_BASE_WRITE{62}else if e.pc==PAID_TAIL_WRITE{64}else{63};
    assert_eq!(*key,want);
    meta_writes.push(json!([w.cell(31),w.cell(33),key,value,e.pc,e.tick]));
    if e.pc==FALLBACK_OFFSET_WRITE{fallback_count+=1;}
    if e.pc==PAID_DUPLICATE_REPAIR{
     assert_eq!(gate,1);
     paid_duplicate_repairs.push(json!([w.cell(31),w.cell(65),w.cell(33),
      w.cell(62),value,e.tick]));
    }
   }else{panic!("source-paid G2S rule edit absent");}
  }
  if e.pc==PAID_DISABLE_SHADOW{
   assert_eq!(gate,0);
   assert!(matches!(&e.kind,EventKind::CellWritten{key:56,value:0}));
   guard_bypass_shadows+=1;
  }
  if e.pc==FALLBACK_ATTEMPT_RESET{
   assert!(matches!(&e.kind,EventKind::CellWritten{key:65,value:0}));
   fallback_guard_resets+=1;
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
  if e.pc==HALT_PC{
   assert!(matches!(&e.kind,EventKind::Halted));
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
 assert!(peak<=197);
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
 assert_eq!(w.cell(31),Some(2),"both hidden task phases must finish");
 assert_eq!(phase_results.len(),2);
 assert_eq!(phase_heldout_paid.len(),2);
 assert!(meta_fetch.len()>=6&&meta_fetch.len()%3==0);
 assert_eq!(meta_checkpoints,meta_fetch.len());
 assert_eq!(meta_programs.len()*3,meta_fetch.len());
 assert_eq!(task_fetch_count,service_outputs.len()*2);
 assert_eq!(task_checkpoint_count,service_outputs.len());
 if gate==0{
  assert!(paid_duplicate_repairs.is_empty());
  assert_eq!(guard_bypass_shadows,meta_programs.len());
 }else{
  assert_eq!(gate,1);
  assert_eq!(guard_bypass_shadows,0);
 }
 if matches!(signal,Signal::Truth){
  const GUARDED:[usize;6]=[2,2,2,1,2,2];
  const UNGUARDED:[usize;6]=[2,2,2,1,1,2];
  assert_eq!(correct,if gate==1{GUARDED[sid]}else{UNGUARDED[sid]});
  if sid==4{
   assert_eq!(phase_results[1]["correct"],gate==1);
   if gate==1{
    assert_eq!(phase_results[1]["mode"],0);
    assert!(!paid_duplicate_repairs.is_empty());
   }else{
    assert_eq!(phase_results[1]["mode"],2);
    assert!(fallback_count>0);
   }
  }
  if sid==3{
   assert_eq!(phase_results[1]["correct"],false);
   assert!(fallback_count>0);
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
  "paid_guard_bypass_shadow_writes":guard_bypass_shadows,
  "paid_original_duplicate_repair_writes":paid_duplicate_repairs,
  "paid_fallback_guard_reset_writes":fallback_guard_resets,
  "source_paid_candidate_attempt_writes":attempt_writes,
  "source_paid_feedback_reads":feedback_reads,
  "source_paid_score_writes":score_writes,
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
  "guard_checks_first_candidate_duplicate_only":true,
  "full_G3":false,
  "full_Hk":false,"B3":false,"B4":false,"cyberlife":false
 });
 emit(out,json!(["end",id,result]));
 result
}
#[test]
fn b56_f1d1k_g3p_paid_first_candidate_duplicate_guard(){
 let folder=std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
  .join("../../cl1b-ci-evidence");
 fs::create_dir_all(&folder).unwrap();
 let mut out=BufWriter::new(File::create(folder.join(
  "b56_f1d1k_g3p_native_paid_duplicate_repair.jsonl")).unwrap());
 let manifest=std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(
  "../../../cl1b/studies/B56-F1d1k-G3P-120-world-first-mode-duplicate-repair-preregistration.md");
 let frozen_path=std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(
  "../../../cl1b/studies/B56-F1d1k-G3P-frozen-exact-180-node-source-v1.json");
 let sum=|p:&std::path::Path|{
  let r=std::process::Command::new("sha256sum").arg(p).output().unwrap();
  assert!(r.status.success());
  String::from_utf8(r.stdout).unwrap().split_whitespace().next().unwrap().to_string()
 };
 let data=fs::read(&frozen_path).unwrap();
 let frozen:Value=serde_json::from_slice(&data).unwrap();
 let source=founder();
 assert_eq!(source.len(),180);
 for (pc,actual) in source.iter().enumerate(){
  let f=&frozen["code"][pc];
  let opcode=f["kind"].as_str().unwrap();
  let expected=if opcode=="Halt"{json!("Halt")}else{
   let mut fields=serde_json::Map::new();
   for (k,v) in f.as_object().unwrap(){
    if k!="kind"{fields.insert(k.clone(),v.clone());}
   }
   let mut outer=serde_json::Map::new();
   outer.insert(opcode.to_string(),Value::Object(fields));
   Value::Object(outer)
  };
  assert_eq!(serde_json::to_value(actual).unwrap(),expected,
   "G3P original native PID1 CIL drift at frozen pc {pc}");
 }
 assert_eq!(frozen["keys"],json!(KEYS));
 assert_eq!(frozen["resources"]["credits"],CREDITS);
 assert_eq!(frozen["resources"]["limit_cells"],16);
 assert_eq!(frozen["resources"]["limit_nodes"],180);
 assert_eq!(frozen["resources"]["limit_occupied"],300);
 assert_eq!(frozen["total_worlds"],120);
 emit(&mut out,json!(["meta","B56-F1d1k-G3P",1,
  std::env::var("GITHUB_SHA").unwrap_or_else(|_|"local".to_string()),
  sum(&manifest),fs::metadata(&manifest).unwrap().len(),
  sum(&frozen_path),data.len(),120,CREDITS,16,180,300,source.len(),
  META_OPCODE_FETCH,META_CHECKPOINT,META_MODE_WRITE,
  TASK_FETCH1,TASK_FETCH2,TASK_CHECKPOINT,TASK_WRITE52,TASK_WRITE53,
  SERVICE_WRITE,FEEDBACK_READ,SCORE_WRITE,PAID_BASE_WRITE,PAID_TAIL_WRITE,
  PAID_RESET_OFFSET,PAID_DISABLE_SHADOW,PAID_DUPLICATE_REPAIR,
  TEMP_OFFSET_WRITE,ATTEMPT_COUNT_WRITE,FALLBACK_OFFSET_WRITE,
  FALLBACK_ATTEMPT_RESET,HOLD_WRITE,FALLBACK_HOLD_WRITE,
  INIT_BASE_WRITE,INIT_TAIL_WRITE,HALT_PC]));
 let mut cases=Vec::<Value>::new();
 for seed in 0..2{
  for sid in 0..6{
   for regime in [Signal::Truth,Signal::BlindSuccess,Signal::Inverted,
    Signal::Absent,Signal::Phase0CDecoy]{
    for gate in [1_i64,0]{
     let id=cases.len();
     cases.push(world(&mut out,id,seed,sid,regime,gate));
    }
   }
  }
 }
 out.flush().unwrap();
 assert_eq!(cases.len(),120);
 let get=|seed:usize,sid:usize,sig:&str,gate:i64|->&Value{
  cases.iter().find(|r|r["seed"]==seed&&r["schedule"]==sid
   &&r["regime"]==sig&&r["gate"]==gate).unwrap()
 };
 for seed in 0..2{
  let s4=get(seed,4,"TRUTH",1);
  let bypass=get(seed,4,"TRUTH",0);
  assert_eq!(s4["true_heldout_correct"],2);
  assert_eq!(bypass["true_heldout_correct"],1);
  let modes=s4["charged_three_instruction_meta_compositions"].as_array().unwrap()
   .iter().filter(|r|r[0]==1).map(|r|r[4].as_i64().unwrap())
   .collect::<Vec<_>>();
  assert_eq!(modes,vec![2,1,2,0],
   "actor paid to REINTERPRET repaired meta-code before service");
  let bypass_modes=bypass["charged_three_instruction_meta_compositions"]
   .as_array().unwrap().iter().filter(|r|r[0]==1)
   .map(|r|r[4].as_i64().unwrap()).collect::<Vec<_>>();
  assert_eq!(bypass_modes,vec![2,1,2,2],
   "same-source guard disabled must try duplicate B");
  let repaired=s4["paid_original_duplicate_repair_writes"].as_array().unwrap()
   .iter().find(|r|r[0]==1).unwrap();
  assert_eq!(repaired[4],2,"Write63(2) makes [2,2,2] interpreted A");
  let s5=get(seed,5,"TRUTH",1);
  assert_eq!(s5["true_heldout_correct"],2);
  let s5m=s5["charged_three_instruction_meta_compositions"]
   .as_array().unwrap().iter().filter(|r|r[0]==1)
   .map(|r|r[4].as_i64().unwrap()).collect::<Vec<_>>();
  assert_eq!(s5m,vec![0,0,1]);
  let s3=get(seed,3,"TRUTH",1);
  assert_eq!(s3["true_heldout_correct"],1);
  assert_eq!(get(seed,3,"TRUTH",0)["true_heldout_correct"],1);
 }
 eprintln!("B56_F1D1K_G3P_NATIVE_PAID_DUPLICATE_GUARD {}",json!({
  "original_PID1_worlds":120,"CIL_source_nodes":180,"data_keys":16,
  "source_unchanged":true,
  "source_paid_repeat_detection":true,
  "actor_paid_Write63_repairs_meta_program":true,
  "truth_S4_reachable_A_recovered":true,
  "source_identical_bypass_S4_fails":true,
  "truth_S5_duplicate_A_probe_avoided":true,
  "S3_unreachable_task_still_unsolved":true,
  "full_64_descriptor_coverage":false,
  "native_malformed_opcode_checks":false,
  "authored_meta_opcode_ISA":true,
  "full_G3":false,"full_Hk":false,"B3":false,"B4":false,"cyberlife":false
 }));
}
