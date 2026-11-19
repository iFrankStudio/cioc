//! G3Q-C1-P1-L1 original PID1: rule-dependent arithmetic, NOT hidden task learning.
use cl0_world::{EventKind,Op,Origin,Process,World};
use serde_json::{json,Value};
use std::fs;
const CREDITS:u64=32_000;
const FROZEN:&str="../../../cl1b/studies/B56-F1d1k-G3Q-C1-P1-L1-frozen-179-node-original-rule-dependent-arithmetic-v1.json";
fn decode(record:&Value)->Op{
 let kind=record["kind"].as_str().unwrap();
 let encoded=if kind=="Halt"{json!("Halt")}else{
  let mut args=serde_json::Map::new();
  for (k,v) in record.as_object().unwrap(){if k!="kind"{args.insert(k.to_string(),v.clone());}}
  let mut wrapper=serde_json::Map::new();wrapper.insert(kind.to_string(),Value::Object(args));
  Value::Object(wrapper)
 };
 serde_json::from_value(encoded).expect("invalid frozen CIL")
}
#[test]
fn g3q_c1_p1_l1_paid_rule_drives_original_pid_arithmetic(){
 let root=std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
 let frozen:Value=serde_json::from_slice(&fs::read(root.join(FROZEN)).unwrap()).unwrap();
 let source=frozen["exact_native_founder_code"].as_array().unwrap();
 assert_eq!(source.len(),179);
 assert_eq!(frozen["source_pcs"]["arithmetic_service"],177);
 assert_eq!(frozen["world"]["max_code_nodes"],180);
 let initial=source.clone();
 let mut observations=Vec::<Value>::new();
 // The alternate founder is a *researcher pre-load mutation*, not PID1 self-edit.
 // It changes only a paid Read address 53 -> 51 at PC169.
 for (label,read_bank_tail_from) in [("frozen_original",53),("preload_address_mutant",51)] {
  let mut records=initial.clone();
  assert_eq!(records[169],json!({"kind":"Const","dst":2,"value":53,"next":170}));
  records[169]["value"]=json!(read_bank_tail_from);
  let founder:Vec<Op>=records.iter().map(decode).collect();
  let mut world=World::with_strict_capacity(16,180,300);
  world.load(Process::new(1,founder.clone(),CREDITS)).unwrap();
  assert_eq!(world.used_cells(),0);
  let mut steps=0_usize;
  let mut peak=world.occupied_units();
  let mut cert=0_usize;let mut retention=0_usize;
  let mut reads=Vec::new();let mut writes=Vec::new();
  let mut operand=None;let mut output=None;let mut arithmetic_pc=0_usize;
  while world.step(){
   steps+=1;
   assert!(steps<=CREDITS as usize);
   let event=world.events().last().unwrap();
   assert!(matches!(event.origin,Origin::Internal(1)));
   assert_eq!(world.process(1).unwrap().code.nodes,founder);
   assert_eq!(world.total_compute_inventory(),Some(CREDITS-steps as u64));
   assert!(world.process(2).is_none());
   assert!(world.used_cells()<=16);
   assert!(world.occupied_units()<=300);
   peak=peak.max(world.occupied_units());
   if event.pc==106 {cert+=1;}
   if [111,115,119].contains(&event.pc){retention+=1;}
   if [162,166,170].contains(&event.pc){
    match &event.kind {
     EventKind::CellRead{key,value}=>reads.push((*key,*value)),
     other=>panic!("expected paid bank read, got {other:?}")
    }
   }
   if [164,168,172].contains(&event.pc){
    match &event.kind {
     EventKind::CellWritten{key,value}=>writes.push((*key,*value)),
     other=>panic!("expected paid active write, got {other:?}")
    }
   }
   if event.pc==174 {
    match &event.kind {EventKind::CellRead{key:65,value}=>operand=Some(*value),
     other=>panic!("expected paid internal input read, got {other:?}")}
   }
   if event.pc==175 {arithmetic_pc+=1;}
   if event.pc==177 {
    match &event.kind {EventKind::CellWritten{key:11,value}=>output=Some(*value),
     other=>panic!("expected real arithmetic output, got {other:?}")}
   }
  }
  assert_eq!(steps,23_939);
  assert_eq!(cert,64);
  assert_eq!(retention,72);
  assert_eq!(arithmetic_pc,1);
  assert_eq!(reads.len(),3);assert_eq!(writes.len(),3);
  assert_eq!(operand,Some(63));
  assert_eq!(world.cell(65),Some(63));
  assert_eq!([world.cell(51),world.cell(52),world.cell(53)],[Some(1),Some(3),Some(2)]);
  assert!(world.process(1).unwrap().stopped);
  assert!(peak<=196);
  if label=="frozen_original"{
   assert_eq!(reads,vec![(51,1),(52,3),(53,2)]);
   assert_eq!(writes,vec![(62,1),(63,3),(64,2)]);
   assert_eq!(output,Some(65));
  }else{
   assert_eq!(reads,vec![(51,1),(52,3),(51,1)]);
   assert_eq!(writes,vec![(62,1),(63,3),(64,1)]);
   assert_eq!(output,Some(64));
  }
  observations.push(json!({"label":label,"steps":steps,"credits_remaining":CREDITS-steps as u64,
    "peak":peak,"bank_reads":reads,"active_writes":writes,
    "endogenous_input":operand,"arithmetic_output":output,"source_nodes":founder.len()}));
 }
 let root=std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
 let folder=root.join("../../cl1b-ci-evidence");
 fs::create_dir_all(&folder).unwrap();
 fs::write(folder.join("b56_f1d1k_g3q_c1_p1_l1_native_rule_arithmetic.json"),
  serde_json::to_vec_pretty(&json!({
   "study":"B56-F1d1k-G3Q-C1-P1-L1","level":"NATIVE_BOUNDED_RULE_DEPENDENT_ARITHMETIC_NO_HIDDEN_TASK",
   "source_sha":std::env::var("GITHUB_SHA").unwrap_or_else(|_|"local".into()),
   "observations":observations,
   "limits":"No task shift, heldout goal, feedback-driven adaptation, or novel primitive."
  })).unwrap()).unwrap();
}
