//! G3Q-C1-P1-L3-I6R: original native single PID1 pays for up to two
//! executable source edits with independent full 3-flag recertification.
//! This is a FIXED second-edit causal baseline, NOT learned repair policy.
//! Preregistered exact 179 CIL nodes and all64 original genomes + controls.
//! unchanged World::with_strict_capacity(16,180,300).
use cl0_world::{EventKind,Intervention,Op,Origin,Process,World};
use serde_json::{json,Value};
use std::fs::{self,File};
use std::io::{BufWriter,Write};
const FROZEN:&str="../../../cl1b/studies/B56-F1d1k-G3Q-C1-P1-L3-I6R-frozen-180-node-fully-isolated-two-repairs-v1.json";
const KEYS:[i64;14]=[7,8,9,11,20,21,32,33,50,52,56,62,63,64];
const CREDIT:u64=8000;
fn decode(v:&Value)->Op{
 let name=v["kind"].as_str().unwrap();
 let tagged=if name=="Halt"{json!("Halt")}else{
  let mut fields=serde_json::Map::new();
  for (k,value) in v.as_object().unwrap(){
   if k!="kind"{fields.insert(k.clone(),value.clone());}
  }
  let mut obj=serde_json::Map::new();
  obj.insert(name.to_string(),Value::Object(fields));
  Value::Object(obj)
 };
 serde_json::from_value(tagged).expect("invalid frozen I6R original CIL")
}
fn gene(w:&World,at:usize)->i64{
 match &w.process(1).unwrap().code.nodes[at]{
  Op::Const{dst:3,value,next} if (at==0&&*next==1)||
    (at==3&&*next==4)||(at==6&&*next==7)=>*value,
  other=>panic!("original source gene changed unexpectedly: {other:?}")
 }
}
fn mode_oracle(g:[i64;3])->Vec<i64>{
 (0..3_i64).map(|trial|{
  let mut accumulator=0_i64;
  for (pos,word) in g.into_iter().enumerate(){
   let opcode=if pos==1{(word+trial)%4}else{word};
   accumulator=if opcode==3{0}else{(accumulator+opcode)%3};
  }
  accumulator
 }).collect()
}
fn run(frozen:&[Value],name:&str,seed:[i64;3],
  feedback_regime:&str,suppress_edits:bool,predicted:&Value,
  evidence_dir:&std::path::Path)->Value{
 let mut native_source=frozen.to_vec();
 for (pc,val) in [(0,seed[0]),(3,seed[1]),(6,seed[2])]{
  native_source[pc]["value"]=json!(val);
 }
 let founder:Vec<Op>=native_source.iter().map(decode).collect();
 let mut expected_current_source=founder.clone();
 let mut native=World::with_strict_capacity(16,180,300);
 if suppress_edits{native=native.with_native_code_edits_disabled();}
 native.load(Process::new(1,founder.clone(),CREDIT)).unwrap();
 assert_eq!(native.occupied_units(),181);
 let mut trace=BufWriter::new(File::create(evidence_dir.join(format!(
  "b56_f1d1k_g3q_c1_p1_l3_i6r_original_{name}.jsonl"))).unwrap());
 let mut paid=0_usize;
 let mut peak=native.occupied_units();
 let mut witnesses=Vec::<Value>::new();
 let mut resets=Vec::<Value>::new();
 let mut certs=Vec::<usize>::new();
 let mut services=Vec::<Value>::new();
 let mut patches=Vec::<Value>::new();
 let mut decisions=Vec::<Value>::new();
 let mut retry_writes=Vec::<Value>::new();
 let mut guard_reads=Vec::<Value>::new();
 let mut paid_meta_reads=0_usize;
 let mut paid_feedback_reads=Vec::<i64>::new();
 let mut delivered_feedback=Vec::<i64>::new();
 let goals=[0_i64,0,2,2];
 while native.step(){
  paid+=1; assert!(paid<=CREDIT as usize);
  let ev=native.events().last().unwrap().clone();
  assert!(matches!(ev.origin,Origin::Internal(1)));
  let p=native.process(1).unwrap();
  if [150,162,168,174,178].contains(&ev.pc){
   let test=match &founder[ev.pc]{
    Op::Branch{test,..}=>*test as usize,
    other=>panic!("native actor decision was not original paid Branch: {other:?}")
   };
   let on=p.registers[test];
   let target=match &founder[ev.pc]{
    Op::Branch{nonzero,zero,..}=>if on!=0{*nonzero}else{*zero},
    _=>unreachable!()
   };
   decisions.push(json!({"step":paid,"pc":ev.pc,"test":on,"target":target}));
  }
  if ev.pc==149{
   assert!(matches!(&ev.kind,EventKind::CellRead{key:11,..}));
   guard_reads.push(json!({"step":paid,"value":p.registers[0]}));
  }
  if [152,177].contains(&ev.pc){
   match &ev.kind{
    EventKind::CellWritten{key:11,value}=>{
     retry_writes.push(json!({"step":paid,"key":11,"value":value}));
    }
    other=>panic!("paid retry guard mark absent: {other:?}")
   }
  }
  if [169,171].contains(&ev.pc){
   assert!(patches.len()<2,"original actor cannot perform third source edit");
   // Full native paid witness isolation before BOTH original code edits.
   assert_eq!(resets.len(),3*(patches.len()+1));
   assert_eq!([native.cell(7),native.cell(8),native.cell(9)],
      [Some(0),Some(0),Some(0)],"stale coverage witness before executable patch");
   let target=p.registers[1] as usize;
   assert!(target==3||target==6,"only two registered source targets");
   let old_value=match &expected_current_source[target]{
    Op::Const{value,..}=>*value,
    other=>panic!("source target is not editable Const: {other:?}")
   };
   let after=gene(&native,target);
   if suppress_edits{
    assert!(matches!(&ev.kind,EventKind::NativeCodeEditSuppressed{opcode_tag:9}));
    assert_eq!(after,old_value);
   }else{
    assert!(matches!(&ev.kind,EventKind::Executed));
    let next=if target==3{4}else{7};
    expected_current_source[target]=Op::Const{dst:3,value:after,next};
   }
   patches.push(json!({"step":paid,"pc":ev.pc,"node":target,
    "before":old_value,"after":after,"edit_suppressed":suppress_edits}));
  }
  assert_eq!(p.code.nodes,expected_current_source,
   "only original charged PatchConst may change executable source");
  assert!(native.process(2).is_none(),"no new process or hidden restart");
  assert_eq!(native.total_compute_inventory(),Some(CREDIT-paid as u64));
  assert!(native.used_cells()<=16&&native.occupied_units()<=300);
  peak=peak.max(native.occupied_units());
  if ev.pc==25{
   assert!(matches!(&ev.kind,EventKind::CellRead{..}));
   paid_meta_reads+=1;
  }
  if ev.pc==107{
   match &ev.kind{
    EventKind::CellWritten{key,value}=>{
     assert!((7..=9).contains(key)&&*value==1);
     witnesses.push(json!({"step":paid,"key":key,"value":value}));
    }
    other=>panic!("native paid witness expected: {other:?}")
   }
  }
  if [155,157,159].contains(&ev.pc){
   match &ev.kind{
    EventKind::CellWritten{key,value}=>{
     assert!((7..=9).contains(key)&&*value==0);
     resets.push(json!({"step":paid,"key":key,"value":value}));
    }
    other=>panic!("native actor must pay full flag reset: {other:?}")
   }
  }
  if ev.pc==147{
   assert!(matches!(&ev.kind,EventKind::CellWritten{key:21,value:1}));
   assert_eq!([native.cell(7),native.cell(8),native.cell(9)],
      [Some(1),Some(1),Some(1)]);
   certs.push(paid);
  }
  if ev.pc==62{
   match &ev.kind{
    EventKind::CellRead{key:20,value}=>paid_feedback_reads.push(*value),
    other=>panic!("native original feedback Read20 absent: {other:?}")
   }
  }
  let row=json!(["cil",paid,ev.pc,p.pc,p.registers,p.credits,
   KEYS.map(|key|native.cell(key)),native.occupied_units(),
   gene(&native,3),gene(&native,6)]);
  serde_json::to_writer(&mut trace,&row).unwrap();
  trace.write_all(b"\n").unwrap();
  if ev.pc==54{
   assert_eq!(certs.len(),1,"original task service started prior to paid certificate");
   let output=match &ev.kind{
    EventKind::CellWritten{key:11,value}=>*value,
    other=>panic!("not a real actor paid Write11 task service: {other:?}")
   };
   let episode=native.cell(32).unwrap_or(0) as usize;
   assert!(episode<4);
   let goal=goals[episode];
   let truth=i64::from(output==goal);
   let signal=match feedback_regime{
    "truth"=>truth,"inverted"=>1-truth,
    other=>panic!("unregistered feedback regime {other}")
   };
   let middle=native.cell(63).unwrap();
   services.push(json!({"step":paid,"episode":episode,"middle":middle,
      "output":output,"truth":truth,"feedback":signal}));
   delivered_feedback.push(signal);
   native.intervene(Intervention::SetCell{key:20,value:signal}).unwrap();
   let outside=native.events().last().unwrap();
   assert!(matches!(outside.origin,Origin::External));
   assert!(matches!(&outside.kind,EventKind::Intervened(_)));
   serde_json::to_writer(&mut trace,&json!(["feedback",paid,episode,goal,
     middle,output,truth,signal,KEYS.map(|key|native.cell(key)),
     native.occupied_units(),gene(&native,3),gene(&native,6)])).unwrap();
   trace.write_all(b"\n").unwrap();
  }
 }
 trace.flush().unwrap();
 assert!(native.process(1).unwrap().stopped,
  "original native PID did not halt before prescribed paid credit budget");
 let final_rule=[gene(&native,0),gene(&native,3),gene(&native,6)];
 let modes=mode_oracle(final_rule);
 let safe=modes.iter().copied().collect::<std::collections::BTreeSet<_>>().len()==3;
 assert_eq!(safe,certs.len()==1,"false coverage certificate or false rejection");
 assert_eq!(paid,predicted["paid_steps"].as_u64().unwrap() as usize);
 assert_eq!(native.process(1).unwrap().pc,predicted["halt_pc"].as_u64().unwrap() as usize);
 assert_eq!(json!(final_rule),predicted["final_rule"]);
 assert_eq!(json!(modes),predicted["semantic_modes"]);
 assert_eq!(safe,predicted["semantic_coverage"].as_bool().unwrap());
 assert_eq!(json!(witnesses),predicted["paid_witnesses"]);
 assert_eq!(json!(resets),predicted["paid_resets"]);
 assert_eq!(json!(certs),predicted["certificate"]);
 assert_eq!(json!(patches),predicted["source_edits"]);
 assert_eq!(json!(decisions),predicted["paid_decisions"]);
 assert_eq!(json!(retry_writes),predicted["paid_retry_writes"]);
 assert_eq!(json!(guard_reads),predicted["paid_guard_reads"]);
 assert_eq!(json!(services),predicted["task_service"]);
 assert_eq!(paid_meta_reads,predicted["meta_Read_count"].as_u64().unwrap() as usize);
 assert_eq!(json!(paid_feedback_reads),predicted["paid_feedback_reads"]);
 assert_eq!(paid_feedback_reads,delivered_feedback);
 assert_eq!(native.used_cells(),predicted["used_cells"].as_u64().unwrap() as usize);
 assert_eq!(peak,predicted["peak_occupied"].as_u64().unwrap() as usize);
 assert_eq!(json!(native.cell(21)),predicted["actor_claim"]);
 assert_eq!(json!(native.cell(32)),predicted["actor_episode"]);
 let externals=native.events().iter().filter(|event|matches!(event.origin,Origin::External)).collect::<Vec<_>>();
 assert_eq!(externals.len(),services.len()+1);
 assert!(matches!(&externals[0].kind,EventKind::Loaded));
 assert!(externals.iter().skip(1).all(|event|matches!(&event.kind,EventKind::Intervened(_))));
 json!({"arm":name,"seed":seed,"paid_original_steps":paid,
    "original_PID1_naturally_halted":true,"original_PID1_source_edits":patches,
    "native_certified":safe,"semantic_modes":modes,"paid_retry_guard_events":retry_writes,
    "paid_full_witness_reset_events":resets,"real_task_services":services,
    "original_meta_reads":paid_meta_reads,"peak_occupied":peak,
    "final_source_rule":final_rule,"postload_host_source_writes":0,
    "actor_claimed_success":native.cell(21)})
}
#[test]
fn g3q_c1_i6r_exact_original_pid_pays_two_source_patches_and_three_certification_rounds(){
 let root=std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
 let manifest:Value=serde_json::from_slice(&fs::read(root.join(FROZEN)).unwrap()).unwrap();
 let source=manifest["original_native_CIL_source"].as_array().unwrap();
 let arms=manifest["all64_original_world_cases"].as_array().unwrap();
 let controls=manifest["controls"].as_array().unwrap();
 assert_eq!(source.len(),180);
 assert_eq!(arms.len(),64);
 assert_eq!(controls.len(),3);
 assert_eq!(source[150]["nonzero"],172);
 assert_eq!(source[171],json!({"kind":"PatchConst","node":1,"value":0,"next":0}));
 assert_eq!(source[174],json!({"kind":"Branch","test":1,"nonzero":98,"zero":175}));
 let evidence=root.join("../../cl1b-ci-evidence");
 fs::create_dir_all(&evidence).unwrap();
 let mut original_worlds=Vec::<Value>::new();
 let mut original_single_edit_repairs=0_usize;
 let mut original_two_edit_repairs=0_usize;
 let mut original_unsafe=0_usize;
 for (i,arm) in arms.iter().enumerate(){
  let v=arm["seed"].as_array().unwrap();
  let seed=[v[0].as_i64().unwrap(),v[1].as_i64().unwrap(),v[2].as_i64().unwrap()];
  assert_eq!(i,((seed[0]*4+seed[1])*4+seed[2]) as usize);
  let name=format!("a{}_b{}_c{}",seed[0],seed[1],seed[2]);
  let observed=run(source,&name,seed,"truth",false,&arm["expected"],&evidence);
  let initial=mode_oracle(seed).iter().copied()
    .collect::<std::collections::BTreeSet<_>>().len()==3;
  assert_eq!(initial,arm["initial_structurally_safe"].as_bool().unwrap());
  if !initial{
   original_unsafe+=1;
   assert_eq!(observed["native_certified"],true);
   match observed["original_PID1_source_edits"].as_array().unwrap().len(){
    1=>original_single_edit_repairs+=1,
    2=>original_two_edit_repairs+=1,
    _=>panic!("incorrect repair edit count for originally unsafe founder")
   }
  }
  original_worlds.push(observed);
 }
 assert_eq!((original_unsafe,original_single_edit_repairs,original_two_edit_repairs),(40,32,8));
 let mut ablations=Vec::<Value>::new();
 for arm in controls{
  let v=arm["seed"].as_array().unwrap();
  let seed=[v[0].as_i64().unwrap(),v[1].as_i64().unwrap(),v[2].as_i64().unwrap()];
  let result=run(source,arm["name"].as_str().unwrap(),seed,
    arm["feedback"].as_str().unwrap(),arm["suppressed"].as_bool().unwrap(),
    &arm["expected"],&evidence);
  if arm["suppressed"]==true{
   assert_eq!(result["native_certified"],false);
   assert!(result["real_task_services"].as_array().unwrap().is_empty());
  }
  ablations.push(result);
 }
 fs::write(evidence.join("b56_f1d1k_g3q_c1_p1_l3_i6r_native_bounded_two_edit.json"),
  serde_json::to_vec_pretty(&json!({
   "study":"B56-F1d1k-G3Q-C1-P1-L3-I6R",
   "level":"NATIVE_ORIGINAL_PID_180_NODE_BOUNDED_TWO_EDITS_FULL_WITNESS_ISOLATION",
   "source_sha":std::env::var("GITHUB_SHA").unwrap_or_else(|_|"local".into()),
   "original64_worlds":original_worlds,
   "controls":ablations,
   "counts":{"initial_safe":24,"initial_unsafe":40,
     "one_edit_rescued":original_single_edit_repairs,
     "two_edit_rescued":original_two_edit_repairs,
     "all64_structurally_certified":64,
     "false_certifications":0,"false_rejections":0},
   "limits":"Second source edit is preauthored fixed middle3->0. This does NOT establish PID-learned or evidence-selected second-edit policy."
  })).unwrap()).unwrap();
}
