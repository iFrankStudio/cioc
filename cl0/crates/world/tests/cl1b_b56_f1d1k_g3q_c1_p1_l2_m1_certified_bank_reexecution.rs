//! G3Q-C1-P1-L2-M1 native original one-PID paid safe-bank interpreter reuse.
//! Sample a paid, causally linked prefix; the process remains alive, NOT halted.
use cl0_world::{EventKind,Op,Origin,Process,World};
use serde_json::{json,Value};
use std::fs::{self,File};
use std::io::{BufWriter,Write};
const FROZEN:&str="../../../cl1b/studies/B56-F1d1k-G3Q-C1-P1-L2-M1-frozen-180-node-paid-certified-bank-reexecution-v1.json";
const ANCESTOR:&str="../../../cl1b/studies/B56-F1d1k-G3Q-C0-frozen-exact-164-node-original-CIL-founder-v1.json";
const CREDITS:u64=32_000;
fn decode(v:&Value)->Op{
 let name=v["kind"].as_str().unwrap();
 let encoded=if name=="Halt"{json!("Halt")}else{
  let mut f=serde_json::Map::new();
  for (key,val) in v.as_object().unwrap(){if key!="kind"{f.insert(key.clone(),val.clone());}}
  let mut out=serde_json::Map::new();out.insert(name.to_string(),Value::Object(f));Value::Object(out)
 };
 serde_json::from_value(encoded).expect("invalid frozen original CIL opcode")
}
#[test]
fn g3q_c1_p1_l2_m1_original_pid_paid_coverage_bank_redeployment_three_opcode_execution(){
 let root=std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
 let frozen:Value=serde_json::from_slice(&fs::read(root.join(FROZEN)).unwrap()).unwrap();
 let ancestor:Value=serde_json::from_slice(&fs::read(root.join(ANCESTOR)).unwrap()).unwrap();
 let source=frozen["exact_native_founder_code"].as_array().unwrap();
 let old=ancestor["exact_native_founder_code"].as_array().unwrap();
 assert_eq!(source.len(),180);
 assert_eq!(old.len(),164);
 assert_eq!(&source[..163],&old[..163],
     "all paid original C0 certification instructions must be exactly preserved");
 assert_eq!(frozen["strict_world"]["cells"],16);
 assert_eq!(frozen["strict_world"]["source_nodes"],180);
 assert_eq!(frozen["strict_world"]["occupied_units"],300);
 assert_eq!(frozen["matched_preload_controls"].as_array().unwrap().len(),2);
 let folder=root.join("../../cl1b-ci-evidence");
 fs::create_dir_all(&folder).unwrap();
 let mut summary=Vec::<Value>::new();
 for arm in frozen["matched_preload_controls"].as_array().unwrap() {
  let name=arm["name"].as_str().unwrap();
  let mut records=source.clone();
  if let Some(m)=arm["source_mutation"].as_object() {
   assert_eq!(m["pc"],171);
   assert_eq!(m["field"],"value");
   assert_eq!(records[171]["value"],m["before"]);
   records[171]["value"]=m["after"].clone();
  }else{assert_eq!(name,"original");}
  let founder:Vec<Op>=records.iter().map(decode).collect();
  let mut world=World::with_strict_capacity(16,180,300);
  world.load(Process::new(1,founder.clone(),CREDITS)).unwrap();
  assert_eq!(world.used_cells(),0);
  assert_eq!(world.occupied_units(),181);
  let mut steps=0_usize;
  let mut peak=world.occupied_units();
  let mut certificates=0_usize;
  let mut safe_retention=0_usize;
  let mut bank_reads:Vec<(i64,i64)>=Vec::new();
  let mut active_writes:Vec<(i64,i64)>=Vec::new();
  let mut meta_fetches:Vec<(i64,i64)>=Vec::new();
  let mut checkpoints:Vec<i64>=Vec::new();
  let mut output:Option<i64>=None;
  let mut trace=BufWriter::new(File::create(folder.join(format!(
   "b56_f1d1k_g3q_c1_p1_l2_m1_paid_prefix_{name}.jsonl"))).unwrap());
  let exact_keys:[i64;16]=[7,11,20,21,32,33,50,51,52,53,56,61,62,63,64,65];
  loop {
   assert!(world.step(),"original PID stopped before post-deployment meta result");
   steps+=1;
   assert!(steps<=CREDITS as usize);
   let event=world.events().last().unwrap();
   assert!(matches!(event.origin,Origin::Internal(1)));
   assert_eq!(world.process(1).unwrap().code.nodes,founder,
       "native original source changed without permission");
   assert_eq!(world.total_compute_inventory(),Some(CREDITS-steps as u64));
   assert!(world.process(2).is_none());
   assert!(world.used_cells()<=16);
   assert!(world.occupied_units()<=300);
   peak=peak.max(world.occupied_units());
   // Evidence is written for EVERY paid original CIL instruction in the
   // sampled prefix, not just the last bank or interpreter events.
   let original=world.process(1).unwrap();
   let native_step=json!([steps,event.pc,original.pc,original.registers,
     original.credits,exact_keys.map(|k|world.cell(k)),world.occupied_units()]);
   serde_json::to_writer(&mut trace,&native_step).unwrap();
   trace.write_all(b"\n").unwrap();
   if event.pc==106{certificates+=1;}
   if [113,117,121].contains(&event.pc){safe_retention+=1;}
   if [164,168,172].contains(&event.pc){
    match &event.kind{
     EventKind::CellRead{key,value}=>bank_reads.push((*key,*value)),
     other=>panic!("must pay to read original actor safe bank, saw {other:?}")
    }
   }
   if [166,170,174].contains(&event.pc){
    match &event.kind{
     EventKind::CellWritten{key,value}=>active_writes.push((*key,*value)),
     other=>panic!("must pay to deploy active meta bytes, saw {other:?}")
    }
   }
   if event.pc==29 && active_writes.len()==3 {
    match &event.kind{
     EventKind::CellRead{key,value}=>meta_fetches.push((*key,*value)),
     other=>panic!("must actually pay to interpret deployed byte, saw {other:?}")
    }
   }
   if event.pc==46 && active_writes.len()==3 {
    match &event.kind{
     EventKind::CellWritten{key:56,value}=>checkpoints.push(*value),
     other=>panic!("must pay for intermediate accumulator, saw {other:?}")
    }
   }
   if event.pc==59 && active_writes.len()==3 {
    match &event.kind{
     EventKind::CellWritten{key:33,value}=>output=Some(*value),
     other=>panic!("must pay for reexecuted meta-mode output, saw {other:?}")
    }
    break;
   }
  }
  trace.flush().unwrap();
  let e=&arm["expected"];
  assert_eq!(steps,e["steps"].as_u64().unwrap() as usize);
  assert_eq!(certificates,e["cert"].as_u64().unwrap() as usize);
  assert_eq!(safe_retention,e["ret"].as_u64().unwrap() as usize);
  assert_eq!(json!(bank_reads),e["bank"]);
  assert_eq!(json!(active_writes),e["active"]);
  assert_eq!(json!(meta_fetches),e["fetch"]);
  assert_eq!(json!(checkpoints),e["checkpoint"]);
  assert_eq!(json!(output),e["mode"]);
  assert_eq!(world.used_cells(),16);
  assert!(peak<=197);
  assert_eq!(world.occupied_units(),197);
  assert_eq!([world.cell(51),world.cell(52),world.cell(53)],[Some(1),Some(3),Some(2)]);
  assert!(!world.process(1).unwrap().stopped,"this protocol deliberately samples a live prefix");
  let external=world.events().iter()
      .filter(|event|matches!(event.origin,Origin::External)).collect::<Vec<_>>();
  assert_eq!(external.len(),1,"only initial PID1 load may be external");
  assert!(matches!(external[0].kind,EventKind::Loaded));
  assert_eq!(world.events().iter().filter(|event|
       matches!(event.origin,Origin::External)&&
       matches!(event.kind,EventKind::CellWritten{..})).count(),0);
  summary.push(json!({
   "arm":name,"source_nodes":180,"pid":1,
   "paid_steps_until_first_postdeployment_meta_output":steps,
   "credits_remaining":CREDITS-steps as u64,
   "peak_occupied":peak,"bank_reads":bank_reads,"active_writes":active_writes,
   "actual_paid_meta_opcode_fetches":meta_fetches,
   "actual_paid_accumulator_writes":checkpoints,"mode_output":output,
   "certificate_writes_before_deployment":certificates,
   "safe_bank_writes_before_deployment":safe_retention,
   "postdeployment_pid_still_alive":true
  }));
 }
 fs::write(folder.join("b56_f1d1k_g3q_c1_p1_l2_m1_paid_certified_bank_reexecution.json"),
  serde_json::to_vec_pretty(&json!({
   "study":"B56-F1d1k-G3Q-C1-P1-L2-M1",
   "level":"NATIVE_SINGLE_PID_PAID_FULL_META_REEXECUTION_PREFIX_NOT_TASK_SERVICE",
   "source_sha":std::env::var("GITHUB_SHA").unwrap_or_else(|_|"local".to_string()),
   "arms":summary,
   "limits":"Sampled live prefix only; no hidden task service, feedback, terminal Halt, autonomous native source mutation or new ISA."
  })).unwrap()).unwrap();
}
