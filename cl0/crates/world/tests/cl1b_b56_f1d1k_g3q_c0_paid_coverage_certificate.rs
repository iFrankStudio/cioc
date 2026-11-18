//! B56-F1d1k-G3Q-C0 native original single PID1 CIL rule coverage.
 //! The exact 164 CIL instructions and 64-case predictions were frozen
 //! before this Rust test. No host meta-byte or verdict injection.
 use cl0_world::{EventKind,Op,Origin,Process,World};
 use serde_json::{json,Value};
 use std::fs::{self,File};
 use std::io::{BufWriter,Write};
 const CREDITS:u64=30_000;
 const KEYS:[i64;16]=[7,11,20,21,32,33,50,51,52,53,56,61,62,63,64,65];
 const META_FETCH:usize=29;
 const META_CHECKPOINT:usize=46;
 const META_RESULT:usize=59;
 const FLAG_A:usize=63;
 const FLAG_C:usize=69;
 const FLAG_B:usize=72;
 const PAID_TRIAL_MIDDLE:usize=91;
 const PAID_FLAG_SUMMARY_SERVICE:usize=106;
 const RETAIN_BASE:usize=113;
 const RETAIN_MIDDLE:usize=117;
 const RETAIN_TAIL:usize=121;
 const PAID_MIDDLE_RESTORE:usize=125;
 const PAID_DESCRIPTOR_BASE:usize=135;
 const PAID_DESCRIPTOR_MIDDLE:usize=146;
 const PAID_DESCRIPTOR_TAIL:usize=156;
 const PAID_DESCRIPTOR_ID:usize=162;
 const HALT_PC:usize=163;
 fn founder()->Vec<Op>{
  vec![
        Op::Const{dst:0,value:0,next:1}, // pc 0
        Op::Const{dst:2,value:62,next:2}, // pc 1
        Op::Write{address:2,value:0,next:3}, // pc 2
        Op::Const{dst:2,value:63,next:4}, // pc 3
        Op::Write{address:2,value:0,next:5}, // pc 4
        Op::Const{dst:2,value:64,next:6}, // pc 5
        Op::Write{address:2,value:0,next:7}, // pc 6
        Op::Const{dst:2,value:65,next:8}, // pc 7
        Op::Write{address:2,value:0,next:9}, // pc 8
        Op::Const{dst:2,value:63,next:10}, // pc 9
        Op::Read{dst:3,address:2,next:11}, // pc 10
        Op::Const{dst:2,value:61,next:12}, // pc 11
        Op::Write{address:2,value:3,next:13}, // pc 12
        Op::Const{dst:2,value:7,next:14}, // pc 13
        Op::Write{address:2,value:0,next:15}, // pc 14
        Op::Const{dst:2,value:20,next:16}, // pc 15
        Op::Write{address:2,value:0,next:17}, // pc 16
        Op::Const{dst:2,value:21,next:18}, // pc 17
        Op::Write{address:2,value:0,next:19}, // pc 18
        Op::Const{dst:2,value:32,next:20}, // pc 19
        Op::Write{address:2,value:0,next:21}, // pc 20
        Op::Const{dst:2,value:50,next:22}, // pc 21
        Op::Write{address:2,value:0,next:23}, // pc 22
        Op::Const{dst:2,value:56,next:24}, // pc 23
        Op::Write{address:2,value:0,next:25}, // pc 24
        Op::Const{dst:2,value:50,next:26}, // pc 25
        Op::Read{dst:3,address:2,next:27}, // pc 26
        Op::Const{dst:2,value:62,next:28}, // pc 27
        Op::Add{dst:2,lhs:2,rhs:3,next:29}, // pc 28
        Op::Read{dst:3,address:2,next:30}, // pc 29
        Op::Const{dst:2,value:3,next:31}, // pc 30
        Op::Sub{dst:1,lhs:3,rhs:2,next:32}, // pc 31
        Op::Branch{test:1,nonzero:34,zero:33}, // pc 32
        Op::Const{dst:1,value:0,next:45}, // pc 33
        Op::Const{dst:2,value:56,next:35}, // pc 34
        Op::Read{dst:1,address:2,next:36}, // pc 35
        Op::Add{dst:1,lhs:1,rhs:3,next:37}, // pc 36
        Op::Const{dst:2,value:3,next:38}, // pc 37
        Op::Sub{dst:3,lhs:1,rhs:2,next:39}, // pc 38
        Op::Branch{test:3,nonzero:41,zero:40}, // pc 39
        Op::Const{dst:1,value:0,next:45}, // pc 40
        Op::Const{dst:2,value:4,next:42}, // pc 41
        Op::Sub{dst:3,lhs:1,rhs:2,next:43}, // pc 42
        Op::Branch{test:3,nonzero:45,zero:44}, // pc 43
        Op::Const{dst:1,value:1,next:45}, // pc 44
        Op::Const{dst:2,value:56,next:46}, // pc 45
        Op::Write{address:2,value:1,next:47}, // pc 46
        Op::Const{dst:2,value:50,next:48}, // pc 47
        Op::Read{dst:3,address:2,next:49}, // pc 48
        Op::Const{dst:2,value:1,next:50}, // pc 49
        Op::Add{dst:3,lhs:3,rhs:2,next:51}, // pc 50
        Op::Const{dst:2,value:50,next:52}, // pc 51
        Op::Write{address:2,value:3,next:53}, // pc 52
        Op::Const{dst:2,value:3,next:54}, // pc 53
        Op::Sub{dst:3,lhs:3,rhs:2,next:55}, // pc 54
        Op::Branch{test:3,nonzero:25,zero:56}, // pc 55
        Op::Const{dst:2,value:56,next:57}, // pc 56
        Op::Read{dst:3,address:2,next:58}, // pc 57
        Op::Const{dst:2,value:33,next:59}, // pc 58
        Op::Write{address:2,value:3,next:60}, // pc 59
        Op::Branch{test:3,nonzero:64,zero:61}, // pc 60
        Op::Const{dst:2,value:7,next:62}, // pc 61
        Op::Const{dst:3,value:1,next:63}, // pc 62
        Op::Write{address:2,value:3,next:73}, // pc 63
        Op::Const{dst:2,value:1,next:65}, // pc 64
        Op::Sub{dst:3,lhs:3,rhs:2,next:66}, // pc 65
        Op::Branch{test:3,nonzero:70,zero:67}, // pc 66
        Op::Const{dst:2,value:20,next:68}, // pc 67
        Op::Const{dst:3,value:1,next:69}, // pc 68
        Op::Write{address:2,value:3,next:73}, // pc 69
        Op::Const{dst:2,value:21,next:71}, // pc 70
        Op::Const{dst:3,value:1,next:72}, // pc 71
        Op::Write{address:2,value:3,next:73}, // pc 72
        Op::Const{dst:2,value:32,next:74}, // pc 73
        Op::Read{dst:3,address:2,next:75}, // pc 74
        Op::Const{dst:2,value:1,next:76}, // pc 75
        Op::Add{dst:3,lhs:3,rhs:2,next:77}, // pc 76
        Op::Const{dst:2,value:32,next:78}, // pc 77
        Op::Write{address:2,value:3,next:79}, // pc 78
        Op::Const{dst:2,value:3,next:80}, // pc 79
        Op::Sub{dst:3,lhs:3,rhs:2,next:81}, // pc 80
        Op::Branch{test:3,nonzero:82,zero:92}, // pc 81
        Op::Const{dst:2,value:63,next:83}, // pc 82
        Op::Read{dst:3,address:2,next:84}, // pc 83
        Op::Const{dst:2,value:1,next:85}, // pc 84
        Op::Add{dst:3,lhs:3,rhs:2,next:86}, // pc 85
        Op::Const{dst:2,value:4,next:87}, // pc 86
        Op::Sub{dst:1,lhs:3,rhs:2,next:88}, // pc 87
        Op::Branch{test:1,nonzero:90,zero:89}, // pc 88
        Op::Const{dst:3,value:0,next:90}, // pc 89
        Op::Const{dst:2,value:63,next:91}, // pc 90
        Op::Write{address:2,value:3,next:21}, // pc 91
        Op::Const{dst:2,value:7,next:93}, // pc 92
        Op::Read{dst:1,address:2,next:94}, // pc 93
        Op::Const{dst:2,value:20,next:95}, // pc 94
        Op::Read{dst:3,address:2,next:96}, // pc 95
        Op::Add{dst:1,lhs:1,rhs:3,next:97}, // pc 96
        Op::Const{dst:2,value:21,next:98}, // pc 97
        Op::Read{dst:3,address:2,next:99}, // pc 98
        Op::Add{dst:1,lhs:1,rhs:3,next:100}, // pc 99
        Op::Const{dst:2,value:3,next:101}, // pc 100
        Op::Sub{dst:3,lhs:1,rhs:2,next:102}, // pc 101
        Op::Branch{test:3,nonzero:103,zero:104}, // pc 102
        Op::Const{dst:3,value:0,next:105}, // pc 103
        Op::Const{dst:3,value:1,next:105}, // pc 104
        Op::Const{dst:2,value:11,next:106}, // pc 105
        Op::Write{address:2,value:3,next:107}, // pc 106
        Op::Const{dst:2,value:11,next:108}, // pc 107
        Op::Read{dst:3,address:2,next:109}, // pc 108
        Op::Branch{test:3,nonzero:110,zero:122}, // pc 109
        Op::Const{dst:2,value:62,next:111}, // pc 110
        Op::Read{dst:3,address:2,next:112}, // pc 111
        Op::Const{dst:2,value:51,next:113}, // pc 112
        Op::Write{address:2,value:3,next:114}, // pc 113
        Op::Const{dst:2,value:61,next:115}, // pc 114
        Op::Read{dst:3,address:2,next:116}, // pc 115
        Op::Const{dst:2,value:52,next:117}, // pc 116
        Op::Write{address:2,value:3,next:118}, // pc 117
        Op::Const{dst:2,value:64,next:119}, // pc 118
        Op::Read{dst:3,address:2,next:120}, // pc 119
        Op::Const{dst:2,value:53,next:121}, // pc 120
        Op::Write{address:2,value:3,next:122}, // pc 121
        Op::Const{dst:2,value:61,next:123}, // pc 122
        Op::Read{dst:3,address:2,next:124}, // pc 123
        Op::Const{dst:2,value:63,next:125}, // pc 124
        Op::Write{address:2,value:3,next:126}, // pc 125
        Op::Const{dst:2,value:62,next:127}, // pc 126
        Op::Read{dst:3,address:2,next:128}, // pc 127
        Op::Const{dst:2,value:1,next:129}, // pc 128
        Op::Add{dst:3,lhs:3,rhs:2,next:130}, // pc 129
        Op::Const{dst:2,value:4,next:131}, // pc 130
        Op::Sub{dst:1,lhs:3,rhs:2,next:132}, // pc 131
        Op::Branch{test:1,nonzero:134,zero:133}, // pc 132
        Op::Const{dst:3,value:0,next:134}, // pc 133
        Op::Const{dst:2,value:62,next:135}, // pc 134
        Op::Write{address:2,value:3,next:136}, // pc 135
        Op::Branch{test:1,nonzero:157,zero:137}, // pc 136
        Op::Const{dst:2,value:63,next:138}, // pc 137
        Op::Read{dst:3,address:2,next:139}, // pc 138
        Op::Const{dst:2,value:1,next:140}, // pc 139
        Op::Add{dst:3,lhs:3,rhs:2,next:141}, // pc 140
        Op::Const{dst:2,value:4,next:142}, // pc 141
        Op::Sub{dst:1,lhs:3,rhs:2,next:143}, // pc 142
        Op::Branch{test:1,nonzero:145,zero:144}, // pc 143
        Op::Const{dst:3,value:0,next:145}, // pc 144
        Op::Const{dst:2,value:63,next:146}, // pc 145
        Op::Write{address:2,value:3,next:147}, // pc 146
        Op::Branch{test:1,nonzero:157,zero:148}, // pc 147
        Op::Const{dst:2,value:64,next:149}, // pc 148
        Op::Read{dst:3,address:2,next:150}, // pc 149
        Op::Const{dst:2,value:1,next:151}, // pc 150
        Op::Add{dst:3,lhs:3,rhs:2,next:152}, // pc 151
        Op::Const{dst:2,value:4,next:153}, // pc 152
        Op::Sub{dst:1,lhs:3,rhs:2,next:154}, // pc 153
        Op::Branch{test:1,nonzero:155,zero:163}, // pc 154
        Op::Const{dst:2,value:64,next:156}, // pc 155
        Op::Write{address:2,value:3,next:157}, // pc 156
        Op::Const{dst:2,value:65,next:158}, // pc 157
        Op::Read{dst:3,address:2,next:159}, // pc 158
        Op::Const{dst:2,value:1,next:160}, // pc 159
        Op::Add{dst:3,lhs:3,rhs:2,next:161}, // pc 160
        Op::Const{dst:2,value:65,next:162}, // pc 161
        Op::Write{address:2,value:3,next:9}, // pc 162
        Op::Halt, // pc 163
  ]
 }
 fn source_fingerprint(code:&[Op])->String{
  let bytes=serde_json::to_vec(code).unwrap();
  let hash=bytes.iter().fold(14695981039346656037_u64,|h,b|
    (h^u64::from(*b)).wrapping_mul(1099511628211));
  format!("{hash:016x}")
 }
 fn sha256(path:&std::path::Path)->String{
  let out=std::process::Command::new("sha256sum").arg(path).output().unwrap();
  assert!(out.status.success());
  String::from_utf8(out.stdout).unwrap().split_whitespace().next().unwrap().into()
 }
 fn emit(w:&mut BufWriter<File>,v:Value){
  serde_json::to_writer(&mut *w,&v).unwrap();
  w.write_all(b"\n").unwrap();
 }
 fn state(world:&World,fingerprint:&str)->Value{
  let p=world.process(1).unwrap();
  json!([p.pc,p.registers,p.credits,p.stopped,p.code.nodes.len(),fingerprint])
 }
 fn cells(world:&World)->Value{json!(KEYS.map(|k|world.cell(k)))}
 fn original_meta_value(word:i64,acc:i64)->i64{
  if word==3{0}else{(acc+word)%3}
 }
 fn independent_researcher_coverage(descriptor:[i64;3])->([i64;3],bool){
  let mut middle=descriptor[1];
  let mut seq=[0_i64;3];
  for (i,entry) in seq.iter_mut().enumerate(){
   let mut acc=0;
   for op in [descriptor[0],middle,descriptor[2]]{
    acc=original_meta_value(op,acc);
   }
   *entry=acc;
   if i<2{middle=(middle+1)%4;}
  }
  let complete=seq[0]!=seq[1]&&seq[0]!=seq[2]&&seq[1]!=seq[2];
  (seq,complete)
 }
 #[test]
 fn b56_f1d1k_g3q_c0_native_paid_64_rule_coverage_and_safe_bank(){
  let folder=std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
    .join("../../cl1b-ci-evidence");
  fs::create_dir_all(&folder).unwrap();
  let mut event_out=BufWriter::new(File::create(folder.join(
    "b56_f1d1k_g3q_c0_native_paid_64_meta_rule_coverage.jsonl")).unwrap());
  let manifest=std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(
    "../../../cl1b/studies/B56-F1d1k-G3Q-C0-original-PID1-paid-64-rule-coverage-preregistration.md");
  let frozen_path=std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(
    "../../../cl1b/studies/B56-F1d1k-G3Q-C0-frozen-exact-164-node-original-CIL-founder-v1.json");
  let frozen_bytes=fs::read(&frozen_path).unwrap();
  let frozen:Value=serde_json::from_slice(&frozen_bytes).unwrap();
  let founder=founder();
  assert_eq!(founder.len(),164);
  assert_eq!(frozen["exact_native_founder_code"].as_array().unwrap().len(),164);
  for (pc,op) in founder.iter().enumerate(){
   let record=&frozen["exact_native_founder_code"][pc];
   let kind=record["kind"].as_str().unwrap();
   let expected=if kind=="Halt"{json!("Halt")}else{
    let mut args=serde_json::Map::new();
    for (k,v) in record.as_object().unwrap(){
     if k!="kind"{args.insert(k.to_string(),v.clone());}
    }
    let mut outer=serde_json::Map::new();
    outer.insert(kind.to_string(),Value::Object(args));
    Value::Object(outer)
   };
   assert_eq!(serde_json::to_value(op).unwrap(),expected,
     "original native CIL founder differs from precommitted opcode pc {pc}");
  }
  assert_eq!(frozen["allowed_cells"],json!(KEYS));
  assert_eq!(frozen["strict_world"]["allowed_cells"],16);
  assert_eq!(frozen["strict_world"]["max_source_nodes"],180);
  assert_eq!(frozen["strict_world"]["max_storage_units"],300);
  assert_eq!(frozen["strict_world"]["initial_private_PID1_credits"],CREDITS);
  let fingerprint=source_fingerprint(&founder);
  let mut world=World::with_strict_capacity(16,180,300);
  world.load(Process::new(1,founder.clone(),CREDITS)).unwrap();
  assert_eq!(world.used_cells(),0);
  assert_eq!(world.occupied_units(),165);
  assert_eq!(world.process(2).is_none(),true);
  let tested_sha=std::env::var("GITHUB_SHA").unwrap_or_else(|_|"local".to_string());
  emit(&mut event_out,json!(["meta","B56-F1d1k-G3Q-C0",1,tested_sha,
   sha256(&manifest),fs::metadata(&manifest).unwrap().len(),
   sha256(&frozen_path),frozen_bytes.len(),
   164,180,16,300,CREDITS,0,
   META_FETCH,META_CHECKPOINT,META_RESULT,FLAG_A,FLAG_C,FLAG_B,
   PAID_TRIAL_MIDDLE,PAID_FLAG_SUMMARY_SERVICE,
   RETAIN_BASE,RETAIN_MIDDLE,RETAIN_TAIL,PAID_MIDDLE_RESTORE,
   PAID_DESCRIPTOR_BASE,PAID_DESCRIPTOR_MIDDLE,PAID_DESCRIPTOR_TAIL,
   PAID_DESCRIPTOR_ID,HALT_PC]));
  emit(&mut event_out,json!(["case",0,founder,fingerprint,CREDITS,
   world.occupied_units(),state(&world,&fingerprint),cells(&world)]));
  let mut steps=0_usize;
  let mut peak=world.occupied_units();
  let mut verdicts=Vec::<Value>::new();
  let mut total_meta_reads=0_usize;
  let mut total_meta_checkpoints=0_usize;
  let mut total_meta_results=0_usize;
  let mut total_seen=0_usize;
  let mut total_retention=0_usize;
  let mut total_trial_edits=0_usize;
  let mut total_middle_restores=0_usize;
  let mut id_edits=0_usize;
  let mut reported_safe=0_usize;
  let mut reported_unsafe=0_usize;
  let mut retained_events=Vec::<Value>::new();
  let mut last_retained:[i64;3]=[0,0,0];
  let mut halt_count=0_usize;
  let mut descriptor_base_edits=0_usize;
  let mut descriptor_middle_edits=0_usize;
  let mut descriptor_tail_edits=0_usize;
  let mut last_mode:i64=0;
  while world.step(){
   steps+=1;
   assert!(steps<=CREDITS as usize);
   let event=world.events().last().unwrap().clone();
   assert!(matches!(event.origin,Origin::Internal(1)));
   assert!(world.process(2).is_none());
   assert_eq!(world.process(1).unwrap().code.nodes,founder);
   assert_eq!(world.total_compute_inventory(),Some(CREDITS-steps as u64));
   assert!(world.used_cells()<=16);
   assert!(world.occupied_units()<=300);
   peak=peak.max(world.occupied_units());
   if event.pc==META_FETCH{
    let meta_pc=world.cell(50).unwrap();
    assert!((0..=2).contains(&meta_pc));
    assert!(matches!(&event.kind,EventKind::CellRead{key,value}
     if *key==62+meta_pc&&*value==world.cell(*key).unwrap()));
    total_meta_reads+=1;
   }
   if event.pc==META_CHECKPOINT{
    assert!(matches!(&event.kind,EventKind::CellWritten{key:56,..}));
    total_meta_checkpoints+=1;
   }
   if event.pc==META_RESULT{
    if let EventKind::CellWritten{key:33,value}=&event.kind{
     last_mode=*value;
     assert!((0..=2).contains(value));
     total_meta_results+=1;
    }else{panic!("actor omitted charged meta result Write33");}
   }
   if [FLAG_A,FLAG_C,FLAG_B].contains(&event.pc){
    let key=if event.pc==FLAG_A{7}else if event.pc==FLAG_C{20}else{21};
    assert!(matches!(&event.kind,EventKind::CellWritten{key:k,value:1}
      if *k==key));
    assert_eq!(last_mode,if key==7{0}else if key==20{1}else{2});
    total_seen+=1;
   }
   if event.pc==PAID_TRIAL_MIDDLE{
    assert!(matches!(&event.kind,EventKind::CellWritten{key:63,..}));
    total_trial_edits+=1;
   }
   if event.pc==PAID_MIDDLE_RESTORE{
    assert!(matches!(&event.kind,EventKind::CellWritten{key:63,value}
     if Some(*value)==world.cell(61)));
    total_middle_restores+=1;
   }
   if event.pc==PAID_DESCRIPTOR_BASE{descriptor_base_edits+=1;}
   if event.pc==PAID_DESCRIPTOR_MIDDLE{descriptor_middle_edits+=1;}
   if event.pc==PAID_DESCRIPTOR_TAIL{descriptor_tail_edits+=1;}
   if event.pc==PAID_DESCRIPTOR_ID{
    assert!(matches!(&event.kind,EventKind::CellWritten{key:65,..}));
    id_edits+=1;
   }
   if [RETAIN_BASE,RETAIN_MIDDLE,RETAIN_TAIL].contains(&event.pc){
    let (expect,keyslot)=if event.pc==RETAIN_BASE{(62,51)}
      else if event.pc==RETAIN_MIDDLE{(61,52)}else{(64,53)};
    assert!(matches!(&event.kind,EventKind::CellWritten{key,value}
      if *key==keyslot&&Some(*value)==world.cell(expect)));
    assert_eq!(world.cell(11),Some(1));
    total_retention+=1;
    retained_events.push(json!([world.cell(65),keyslot,world.cell(keyslot),event.tick]));
   }
   if event.pc==PAID_FLAG_SUMMARY_SERVICE{
    let idx=world.cell(65).unwrap();
    let current=[world.cell(62).unwrap(),world.cell(61).unwrap(),
      world.cell(64).unwrap()];
    assert_eq!(idx,verdicts.len() as i64);
    assert_eq!(current[0],idx%4);
    assert_eq!(current[1],idx/4%4);
    assert_eq!(current[2],idx/16%4);
    let (seq,expected)=independent_researcher_coverage(current);
    if let EventKind::CellWritten{key:11,value}=&event.kind{
     assert_eq!(*value,i64::from(expected));
     assert!(matches!(world.cell(7),Some(0|1)));
     assert!(matches!(world.cell(20),Some(0|1)));
     assert!(matches!(world.cell(21),Some(0|1)));
     let flags=[world.cell(7).unwrap(),world.cell(20).unwrap(),
       world.cell(21).unwrap()];
     let witness=(seq.contains(&0),seq.contains(&1),seq.contains(&2));
     assert_eq!(flags,[i64::from(witness.0),i64::from(witness.1),
       i64::from(witness.2)]);
     if expected{reported_safe+=1;last_retained=current;}
     else{reported_unsafe+=1;}
     verdicts.push(json!({"id":idx,"descriptor":current,"modes":seq,
       "native_PID1_paid_witness_flags":flags,
       "source_paid_verdict":value,
       "source_paid_step":steps,"source_paid_tick":event.tick}));
    }else{panic!("coverage certificate not emitted by paid native PID1 Write11");}
   }
   if event.pc==HALT_PC{
    assert!(matches!(&event.kind,EventKind::Halted));
    halt_count+=1;
   }
   emit(&mut event_out,json!(["step",0,steps,event.tick,1,event.pc,event.kind,
    state(&world,&fingerprint),world.occupied_units(),cells(&world)]));
  }
  assert!(world.process(1).unwrap().stopped);
  assert_eq!(world.process(2).is_none(),true);
  // The native kernel records the one initial host-triggered Process::load
  // as Origin::External / EventKind::Loaded. That is NOT a post-load
  // intervention and must not be misreported as host-written metadata.
  let external=world.events().iter()
    .filter(|e|matches!(e.origin,Origin::External)).collect::<Vec<_>>();
  assert_eq!(external.len(),1,"only original PID1 process load may be external");
  assert!(matches!(external[0].kind,EventKind::Loaded));
  assert_eq!(world.events().iter().filter(|e|
    matches!(e.origin,Origin::External) &&
    matches!(e.kind,EventKind::CellWritten{..})).count(),0,
    "zero external cell writes after initial load");
  assert_eq!(steps,24050);
  assert_eq!(verdicts.len(),64);
  assert_eq!(total_meta_reads,576);
  assert_eq!(total_meta_checkpoints,576);
  assert_eq!(total_meta_results,192);
  assert_eq!(total_seen,192);
  assert_eq!(total_retention,72);
  assert_eq!(total_trial_edits,128);
  assert_eq!(total_middle_restores,64);
  assert_eq!(id_edits,63);
  assert_eq!(reported_safe,24);
  assert_eq!(reported_unsafe,40);
  assert_eq!(peak,181);
  assert_eq!(halt_count,1);
  assert_eq!(world.cell(65),Some(63));
  assert_eq!(last_retained,[1,3,2]);
  assert_eq!([world.cell(51),world.cell(52),world.cell(53)],
   [Some(1),Some(3),Some(2)]);
  for (idx,expected) in [(42_usize,1_i64),(48_usize,0_i64)]{
   assert_eq!(verdicts[idx]["source_paid_verdict"],expected);
  }
  assert_eq!(retained_events.len(),72);
  let report=json!({
   "study":"B56-F1d1k-G3Q-C0",
   "source_tested_sha":std::env::var("GITHUB_SHA").unwrap_or_else(|_|"local".to_string()),
   "precommitted_source_nodes":164,
   "original_PID1_continuously_alive_for_64_rules":true,
   "original_CIL_source_unchanged":true,
   "original_PID1_paid_instructions":steps,
   "host_interventions":0,
   "native_dynamic_meta_opcode_reads":total_meta_reads,
   "paid_meta_accumulator_checkpoints":total_meta_checkpoints,
   "paid_completed_three_word_meta_programs":total_meta_results,
   "paid_seen_flag_writes":total_seen,
   "paid_source_coverage_verdict_services":verdicts.len(),
   "paid_accepted_rule_retention_writes":total_retention,
   "paid_meta_middle_trial_mutations":total_trial_edits,
   "paid_meta_middle_restores":total_middle_restores,
   "paid_descriptor_base_advances":descriptor_base_edits,
   "paid_descriptor_middle_advances":descriptor_middle_edits,
   "paid_descriptor_tail_advances":descriptor_tail_edits,
   "paid_descriptor_ID_advances":id_edits,
   "safe_descriptors":reported_safe,
   "unsafe_descriptors":reported_unsafe,
   "source_generated_safe_descriptor_2_2_2_accepted":true,
   "source_generated_RESEThai_descriptor_0_0_3_rejected":true,
   "PID1_halts_after_full_64_rule_enumeration":true,
   "last_retained_safe_descriptor":last_retained,
   "peak_total_storage_units":peak,
   "final_source_PID1":state(&world,&fingerprint),
   "final_all_16_present_or_missing_cells":cells(&world),
   "private_credits_remaining":world.remaining_credits(),
   "all_64_original_PID1_verdicts":verdicts,
   "original_PID1_safe_bank_retention_events":retained_events,
   "full_G3Q":false,
   "task_program_services":false,
   "feedback_training":false,
   "cycle_safe_meta_repair":false,
   "new_primitive_ISA_invention":false,
   "Hk":false,"B3":false,"B4":false,"cyberlife":false
  });
  emit(&mut event_out,json!(["end",0,report]));
  event_out.flush().unwrap();
  eprintln!("B56_F1D1K_G3Q_C0_NATIVE_PID1_PAID_64_RULE_COVERAGE {}",json!({
   "original_single_PID1":true,"source_nodes":164,
   "all_rule_descriptors_checked_by_actual_PID1":64,
   "paid_native_instructions":steps,"safe":reported_safe,
   "rejected":reported_unsafe,"dynamic_meta_reads":total_meta_reads,
   "paid_safe_bank_writes":total_retention,"peak_storage":peak,
   "no_host_rule_injection":true,"no_source_mutation":true,
   "source_generated_2_2_2_safe":true,"source_generated_0_0_3_rejected":true,
   "actual_task_program_executions":false,"full_G3Q":false,
   "Hk":false,"B3":false,"B4":false,"cyberlife":false
  }));
 }
