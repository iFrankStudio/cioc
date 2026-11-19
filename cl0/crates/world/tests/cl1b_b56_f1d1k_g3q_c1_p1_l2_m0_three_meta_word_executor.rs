//! G3Q-C1-P1-L2-M0: bounded ORIGINAL native paid three-word interpreter.
//! This 66-node founder is independently authored and NOT integrated C1 learning.
use cl0_world::{EventKind,Op,Origin,Process,World};
use serde_json::{json,Value};
use std::fs;
const FROZEN:&str="../../../cl1b/studies/B56-F1d1k-G3Q-C1-P1-L2-M0-frozen-66-node-full-three-word-executor-v1.json";
const CREDITS:u64=1000;
fn decode(record:&Value)->Op {
 let kind=record["kind"].as_str().unwrap();
 let wrapped=if kind=="Halt"{json!("Halt")}else{
  let mut fields=serde_json::Map::new();
  for (k,v) in record.as_object().unwrap(){if k!="kind"{fields.insert(k.clone(),v.clone());}}
  let mut map=serde_json::Map::new();map.insert(kind.to_string(),Value::Object(fields));Value::Object(map)
 };
 serde_json::from_value(wrapped).expect("frozen opcode must decode")
}
#[test]
fn g3q_c1_p1_l2_m0_three_meta_words_paid_native_execution() {
 let root=std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
 let f:Value=serde_json::from_slice(&fs::read(root.join(FROZEN)).unwrap()).unwrap();
 let frozen=f["exact_native_founder_code"].as_array().unwrap().clone();
 let arms=f["pre_registered_arms"].as_array().unwrap();
 assert_eq!(frozen.len(),66);
 assert_eq!(arms.len(),4);
 assert_eq!(f["strict_world"]["allowed_cells"],16);
 assert_eq!(f["strict_world"]["max_code_nodes"],180);
 assert_eq!(f["strict_world"]["max_occupied"],300);
 let mut observations=Vec::new();
 for arm in arms {
  let name=arm["name"].as_str().unwrap();
  let mut source=frozen.clone();
  if let Some(pc)=arm["mutated_pc"].as_u64() {
   let pc=pc as usize;
   assert!([0,3,6].contains(&pc));
   assert_eq!(source[pc]["kind"],"Const");
   source[pc]["value"]=arm["mutated_value"].clone();
  }else{assert_eq!(name,"original");}
  let founder:Vec<Op>=source.iter().map(decode).collect();
  let mut w=World::with_strict_capacity(16,180,300);
  w.load(Process::new(1,founder.clone(),CREDITS)).unwrap();
  assert_eq!(w.used_cells(),0);
  let mut steps=0_usize;
  let mut peak=w.occupied_units();
  let mut bank_reads=Vec::new();
  let mut active_writes=Vec::new();
  let mut meta_fetches=Vec::new();
  let mut meta_accumulators=Vec::new();
  let mut service=Vec::new();
  while w.step(){
   steps+=1;
   assert!(steps<=CREDITS as usize);
   let event=w.events().last().unwrap();
   assert!(matches!(event.origin,Origin::Internal(1)));
   assert_eq!(w.process(1).unwrap().code.nodes,founder,"native executable remains immutable");
   assert_eq!(w.total_compute_inventory(),Some(CREDITS-steps as u64));
   assert!(w.process(2).is_none());
   assert!(w.used_cells()<=16);
   assert!(w.occupied_units()<=300);
   peak=peak.max(w.occupied_units());
   match &event.kind {
    EventKind::CellRead{key,value} if [10,14,18].contains(&event.pc)=>bank_reads.push(json!([key,value])),
    EventKind::CellRead{key,value} if event.pc==30=>meta_fetches.push(json!([key,value])),
    EventKind::CellWritten{key,value} if [12,16,20].contains(&event.pc)=>active_writes.push(json!([key,value])),
    EventKind::CellWritten{key:56,value} if event.pc==51=>meta_accumulators.push(json!(value)),
    EventKind::CellWritten{key:11,value} if event.pc==64=>service.push(json!(value)),
    _=>{}
   }
  }
  assert!(w.process(1).unwrap().stopped);
  assert_eq!(steps,arm["steps"].as_u64().unwrap() as usize);
  assert_eq!(bank_reads,arm["bankReads"]);
  assert_eq!(active_writes,arm["activeWrites"]);
  assert_eq!(meta_fetches,arm["fetch"]);
  assert_eq!(meta_accumulators,arm["accWrites"]);
  assert_eq!(service,arm["out"]);
  assert_eq!(service.len(),1);
  assert_eq!(meta_fetches.len(),3);
  assert_eq!(meta_accumulators.len(),3);
  assert_eq!(w.cell(11),arm["output"].as_i64());
  assert_eq!(w.used_cells(),9);
  assert!(peak<=76);
  observations.push(json!({"name":name,"steps":steps,"credits_remaining":CREDITS-steps as u64,
    "bank_reads":bank_reads,"active_writes":active_writes,"meta_fetches":meta_fetches,
    "meta_accumulators":meta_accumulators,"service":service,"peak":peak}));
 }
 let folder=root.join("../../cl1b-ci-evidence");fs::create_dir_all(&folder).unwrap();
 fs::write(folder.join("b56_f1d1k_g3q_c1_p1_l2_m0_native_three_word_executor.json"),
  serde_json::to_vec_pretty(&json!({
   "study":"B56-F1d1k-G3Q-C1-P1-L2-M0",
   "level":"NATIVE_SINGLE_PID_THREE_WORD_BOUNDED_INTERPRETER_NOT_CERTIFIED_LEARNING",
   "source_sha":std::env::var("GITHUB_SHA").unwrap_or_else(|_|"local".into()),
   "founder_nodes":66,"world_caps":[16,180,300],"observations":observations,
   "limits":"Preloaded source-authored rule bank, no earned within-PID C0 certification, no hidden feedback, no adaptation, no self-edit."
  })).unwrap()).unwrap();
}
