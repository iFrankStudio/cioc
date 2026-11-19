//! G3Q-C1-P0 narrow native bridge. Frozen exact founder committed before test.
//! NOT a functional hidden task service or completed C1.
use cl0_world::{EventKind,Op,Origin,Process,World};
use serde_json::{json,Value};
use std::fs;
const CREDITS:u64=32_000;
const FROZEN:&str="../../../cl1b/studies/B56-F1d1k-G3Q-C1-P0-frozen-180-node-paid-bank-deployment-founder-v1.json";
const ANCESTOR:&str="../../../cl1b/studies/B56-F1d1k-G3Q-C0-frozen-exact-164-node-original-CIL-founder-v1.json";
fn decode(record:&Value)->Op{
 let kind=record["kind"].as_str().unwrap();
 let encoded=if kind=="Halt"{json!("Halt")}else{
  let mut args=serde_json::Map::new();
  for (k,v) in record.as_object().unwrap(){if k!="kind"{args.insert(k.to_string(),v.clone());}}
  let mut wrapper=serde_json::Map::new();
  wrapper.insert(kind.to_string(),Value::Object(args));
  Value::Object(wrapper)
 };
 serde_json::from_value(encoded).expect("invalid frozen CIL opcode")
}
#[test]
fn g3q_c1_p0_original_pid_paid_bank_to_active_three_word_deployment(){
 let root=std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
 let frozen:Value=serde_json::from_slice(&fs::read(root.join(FROZEN)).unwrap()).unwrap();
 let ancestor:Value=serde_json::from_slice(&fs::read(root.join(ANCESTOR)).unwrap()).unwrap();
 let records=frozen["exact_native_founder_code"].as_array().unwrap();
 let orig=ancestor["exact_native_founder_code"].as_array().unwrap();
 assert_eq!(records.len(),180);
 assert_eq!(orig.len(),164);
 assert_eq!(&records[..163],&orig[..163],"original source must remain byte-structurally identical until old Halt");
 assert_eq!(frozen["allowed_cells"],ancestor["allowed_cells"]);
 let founder:Vec<Op>=records.iter().map(decode).collect();
 assert_eq!(founder.len(),180);
 let mut world=World::with_strict_capacity(16,180,300);
 world.load(Process::new(1,founder.clone(),CREDITS)).unwrap();
 assert_eq!(world.used_cells(),0);
 assert_eq!(world.occupied_units(),181);
 let mut steps=0_usize;
 let mut peak=world.occupied_units();
 let mut bank_reads=Vec::new();
 let mut active_writes=Vec::new();
 let mut diagnostics=Vec::new();
 let mut certificates=0_usize;
 let mut retention=0_usize;
 let mut events_in_bridge=0_usize;
 while world.step(){
  steps+=1;
  assert!(steps<=CREDITS as usize);
  let e=world.events().last().unwrap();
  assert!(matches!(e.origin,Origin::Internal(1)));
  assert_eq!(world.process(1).unwrap().code.nodes,founder,"original source edit forbidden");
  assert_eq!(world.total_compute_inventory(),Some(CREDITS-steps as u64));
  assert!(world.process(2).is_none());
  assert!(world.used_cells()<=16);
  assert!(world.occupied_units()<=300);
  peak=peak.max(world.occupied_units());
  if e.pc==106{certificates+=1;}
  if [113,117,121].contains(&e.pc){retention+=1;}
  if e.pc>=163{events_in_bridge+=1;}
  if [164,168,172].contains(&e.pc){
   match &e.kind{EventKind::CellRead{key,value}=>bank_reads.push((*key,*value)),
    other=>panic!("expected original actor paid bank Read: {other:?}")};
  }
  if [166,170,174].contains(&e.pc){
   match &e.kind{EventKind::CellWritten{key,value,..}=>active_writes.push((*key,*value)),
    other=>panic!("expected original actor paid active Write: {other:?}")};
  }
  if [176,178].contains(&e.pc){diagnostics.push(e.pc);}
 }
 assert_eq!(certificates,64);
 assert_eq!(retention,72);
 assert_eq!(bank_reads,vec![(51,1),(52,3),(53,2)]);
 assert_eq!(active_writes,vec![(62,1),(63,3),(64,2)]);
 assert_eq!(diagnostics,vec![176,178]);
 assert_eq!(world.cell(51),Some(1));
 assert_eq!(world.cell(52),Some(3));
 assert_eq!(world.cell(53),Some(2));
 assert_eq!(world.cell(62),Some(1));
 assert_eq!(world.cell(63),Some(3));
 assert_eq!(world.cell(64),Some(2));
 assert_eq!(world.cell(11),Some(2));
 assert_eq!(steps,24_066);
 assert_eq!(events_in_bridge,17);
 assert!(peak<=197);
 assert!(world.process(1).unwrap().stopped);
 let folder=root.join("../../cl1b-ci-evidence");
 fs::create_dir_all(&folder).unwrap();
 fs::write(folder.join("b56_f1d1k_g3q_c1_p0_paid_bank_deployment_summary.json"),
  serde_json::to_vec_pretty(&json!({
    "study":"B56-F1d1k-G3Q-C1-P0",
    "level":"NATIVE_NARROW_DEPLOYMENT_BRIDGE_NOT_TASK_SERVICE",
    "github_sha":std::env::var("GITHUB_SHA").unwrap_or_else(|_|"local".into()),
    "source_nodes":founder.len(),"original_pid":1,"credits_start":CREDITS,
    "charged_steps":steps,"credits_remain":CREDITS-steps as u64,
    "peak_occupied":peak,"certificate_writes":certificates,
    "paid_retention_writes":retention,"bank_reads":bank_reads,
    "active_writes":active_writes,"diagnostic_output":world.cell(11),
    "restarts":0,"postload_external_cell_interventions":0,
    "limitations":"No hidden task, true service, task feedback, task shift, or independent replay. No full C1 achievement."
  })).unwrap()).unwrap();
}
