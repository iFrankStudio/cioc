//! G3Q-C1-P1-L2-M2 single ORIGINAL PID1 paid self-patched terminal rule cycle.
//! Frozen source was committed before this Rust test. Not hidden task learning.
use cl0_world::{EventKind,Op,Origin,Process,World};
use serde_json::{json,Value};
use std::fs::{self,File};
use std::io::{BufWriter,Write};
const FROZEN:&str="../../../cl1b/studies/B56-F1d1k-G3Q-C1-P1-L2-M2-frozen-179-node-self-terminal-certified-meta-cycle-v1.json";
const ANCESTOR:&str="../../../cl1b/studies/B56-F1d1k-G3Q-C0-frozen-exact-164-node-original-CIL-founder-v1.json";
const CREDITS:u64=32_000;
const KEYS:[i64;16]=[7,11,20,21,32,33,50,51,52,53,56,61,62,63,64,65];
fn decode(v:&Value)->Op {
 let name=v["kind"].as_str().unwrap();
 let tagged=if name=="Halt"{json!("Halt")}else{
  let mut f=serde_json::Map::new();
  for (k,val) in v.as_object().unwrap(){if k!="kind"{f.insert(k.clone(),val.clone());}}
  let mut o=serde_json::Map::new();o.insert(name.to_string(),Value::Object(f));Value::Object(o)
 };
 serde_json::from_value(tagged).expect("invalid frozen source opcode")
}
fn patched(founder:&[Op])->Vec<Op>{
 let mut expected=founder.to_vec();
 expected[59]=match &founder[59] {
  Op::Write{address,value,next:60}=>Op::Write{address:*address,value:*value,next:178},
  other=>panic!("original dynamic meta source changed: {other:?}")
 };
 expected
}
fn native_edge(world:&World)->usize {
 match &world.process(1).unwrap().code.nodes[59] {
  Op::Write{next,..}=>*next,
  other=>panic!("expected patchable meta Write59: {other:?}")
 }
}
#[test]
fn g3q_c1_p1_l2_m2_original_pid_paid_certified_meta_and_self_halt(){
 let root=std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
 let frozen:Value=serde_json::from_slice(&fs::read(root.join(FROZEN)).unwrap()).unwrap();
 let ancestor:Value=serde_json::from_slice(&fs::read(root.join(ANCESTOR)).unwrap()).unwrap();
 let records=frozen["exact_native_founder_code"].as_array().unwrap();
 let original=ancestor["exact_native_founder_code"].as_array().unwrap();
 assert_eq!(records.len(),179);
 assert_eq!(original.len(),164);
 assert_eq!(&records[..163],&original[..163]);
 assert_eq!(frozen["expected_frozen_arms"].as_array().unwrap().len(),2);
 assert_eq!(records[177],json!({"kind":"PatchNext","node":2,"target":3,"next":21}));
 assert_eq!(records[178],json!({"kind":"Halt"}));
 let folder=root.join("../../cl1b-ci-evidence");
 fs::create_dir_all(&folder).unwrap();
 let mut observations=Vec::new();
 for arm in frozen["expected_frozen_arms"].as_array().unwrap(){
  let name=arm["name"].as_str().unwrap();
  let mut source=records.clone();
  if let Some(change)=arm["preload_source_mutation"].as_object(){
   assert_eq!(change["pc"],171);
   assert_eq!(change["field"],"value");
   assert_eq!(source[171]["value"],change["from"]);
   source[171]["value"]=change["to"].clone();
  }else{assert_eq!(name,"original");}
  let founder:Vec<Op>=source.iter().map(decode).collect();
  let edited=patched(&founder);
  let mut world=World::with_strict_capacity(16,180,300)
    .with_native_executable_edit_trace();
  world.load(Process::new(1,founder.clone(),CREDITS)).unwrap();
  assert_eq!(world.occupied_units(),180);
  assert_eq!(world.used_cells(),0);
  let mut trace=BufWriter::new(File::create(folder.join(format!(
   "b56_f1d1k_g3q_c1_p1_l2_m2_original_paid_steps_{name}.jsonl"))).unwrap());
  let mut steps=0_usize;let mut peak=world.occupied_units();
  let mut cert=0_usize;let mut retention=0_usize;
  let mut bank=Vec::<(i64,i64)>::new();let mut active=Vec::<(i64,i64)>::new();
  let mut fetch=Vec::<(i64,i64)>::new();let mut acc=Vec::<i64>::new();
  let mut modes=Vec::<i64>::new();let mut patches=0_usize;
  let mut self_source_edit_step=0_usize;
  let mut halted=0_usize;
  while world.step(){
   steps+=1;
   assert!(steps<=CREDITS as usize);
   let event=world.events().last().unwrap();
   assert!(matches!(event.origin,Origin::Internal(1)));
   let p=world.process(1).unwrap();
   // The PatchNext state mutation has already happened when its charged
   // native event is observed; count it before validating the new graph.
   let source_changed = patches > 0 || event.pc == 177;
   assert_eq!(&p.code.nodes,if source_changed {&edited} else {&founder},
     "any executable change besides the one source-paid meta edge is forbidden");
   assert_eq!(world.total_compute_inventory(),Some(CREDITS-steps as u64));
   assert!(world.process(2).is_none());
   assert!(world.used_cells()<=16);
   assert!(world.occupied_units()<=300);
   peak=peak.max(world.occupied_units());
   if event.pc==106{cert+=1;}
   if [113,117,121].contains(&event.pc){retention+=1;}
   if [164,168,172].contains(&event.pc){
    match &event.kind {
     EventKind::CellRead{key,value}=>bank.push((*key,*value)),
     other=>panic!("missing charged retained bank Read: {other:?}")
    }
   }
   if [166,170,174].contains(&event.pc){
    match &event.kind {
     EventKind::CellWritten{key,value}=>active.push((*key,*value)),
     other=>panic!("missing charged active meta Write: {other:?}")
    }
   }
   if event.pc==177{
    assert_eq!(patches,0,"self code edit may happen only once");
    assert!(matches!(&event.kind,
     EventKind::ExecutableEdgePatched{node:59,before:60,after:178}));
    assert_eq!(native_edge(&world),178);
    patches+=1;self_source_edit_step=steps;
   }
   if event.pc==29&&patches==1{
    match &event.kind{
     EventKind::CellRead{key,value}=>fetch.push((*key,*value)),
     other=>panic!("deployed metacode not charged: {other:?}")
    }
   }
   if event.pc==46&&patches==1{
    match &event.kind{
     EventKind::CellWritten{key:56,value}=>acc.push(*value),
     other=>panic!("not a paid accumulator step: {other:?}")
    }
   }
   if event.pc==59&&patches==1{
    match &event.kind{
     EventKind::CellWritten{key:33,value}=>modes.push(*value),
     other=>panic!("no paid interpreted mode output: {other:?}")
    }
    assert_eq!(p.pc,178,"actor's own newly patched edge must now target Halt");
   }
   if event.pc==178{
    assert!(matches!(&event.kind,EventKind::Halted));
    halted+=1;
   }
   // Native source PIN + full snapshot of every original charged instruction.
   // The new final field records original actor's executable PC59 edge;
   // unlike immutable-source M1, the source state really changes here.
   let snapshot=json!([steps,event.pc,p.pc,p.registers,p.credits,
     KEYS.map(|key|world.cell(key)),world.occupied_units(),native_edge(&world)]);
   serde_json::to_writer(&mut trace,&snapshot).unwrap();
   trace.write_all(b"\n").unwrap();
  }
  trace.flush().unwrap();
  let expected=&arm["expected"];
  assert_eq!(steps,expected["paid_steps"].as_u64().unwrap() as usize);
  assert_eq!(cert,64);
  assert_eq!(retention,72);
  assert_eq!(patches,1);
  assert_eq!(self_source_edit_step,24064);
  assert_eq!(halted,1);
  assert_eq!(json!(bank),expected["retained_bank_reads"]);
  assert_eq!(json!(active),expected["active_writes"]);
  assert_eq!(json!(fetch),expected["dynamic_meta_reads"]);
  assert_eq!(json!(acc),expected["accumulator_checkpoints"]);
  assert_eq!(json!(modes),json!([expected["meta_mode_output"]]));
  assert_eq!(native_edge(&world),178);
  assert!(world.process(1).unwrap().stopped);
  assert_eq!(world.process(1).unwrap().pc,178);
  assert_eq!([world.cell(51),world.cell(52),world.cell(53)],
   [Some(1),Some(3),Some(2)]);
  assert_eq!(world.used_cells(),16);
  assert_eq!(peak,196);
  assert_eq!(world.occupied_units(),196);
  assert_eq!(CREDITS-steps as u64,7851);
  let external=world.events().iter()
    .filter(|e|matches!(e.origin,Origin::External)).collect::<Vec<_>>();
  assert_eq!(external.len(),1,"external actions must be only first PID load");
  assert!(matches!(external[0].kind,EventKind::Loaded));
  observations.push(json!({
   "arm":name,"native_paid_steps":steps,"original_single_pid":1,
   "credits_remaining":CREDITS-steps as u64,"peak_storage":peak,
   "original_certificates":cert,"paid_bank_retention":retention,
   "bank_reads":bank,"active_writes":active,"meta_fetches":fetch,
   "accumulator_checkpoints":acc,"computed_modes":modes,
   "source_patch_count":patches,"source_patch_step":self_source_edit_step,
   "source_patch_opcode_pc":177,"edited_original_node":59,
   "edited_edge_from":60,"edited_edge_to":178,"halt_pc":178,
   "original_pid_self_halted":true,"postload_external_writes":0
  }));
 }
 fs::write(folder.join("b56_f1d1k_g3q_c1_p1_l2_m2_native_self_terminal_meta_cycle.json"),
  serde_json::to_vec_pretty(&json!({
   "study":"B56-F1d1k-G3Q-C1-P1-L2-M2",
   "level":"NATIVE_SINGLE_PID_PROGRAMMED_SELF_PATCHED_TERMINAL_META_CYCLE",
   "source_sha":std::env::var("GITHUB_SHA").unwrap_or_else(|_|"local".into()),
   "arms":observations,
   "limits":"Researcher-authored programmed PatchNext, no hidden task or feedback-driven learned self-edit."
  })).unwrap()).unwrap();
}
#[test]
fn g3q_c1_p1_l2_m2_patchnext_disabled_causal_ablation(){
 let root=std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
 let f:Value=serde_json::from_slice(&fs::read(root.join(FROZEN)).unwrap()).unwrap();
 let founder:Vec<Op>=f["exact_native_founder_code"].as_array().unwrap().iter().map(decode).collect();
 let mut world=World::with_strict_capacity(16,180,300)
  .with_native_patchnext_disabled().with_native_executable_edit_trace();
 world.load(Process::new(1,founder.clone(),CREDITS)).unwrap();
 let mut saw_patch_instruction=0_usize;let mut saw_postservice=false;
 let mut steps=0_usize;
 while world.step(){
  steps+=1;
  assert!(steps<=CREDITS as usize);
  let e=world.events().last().unwrap();
  assert!(matches!(e.origin,Origin::Internal(1)));
  assert_eq!(world.process(1).unwrap().code.nodes,founder,
     "disabled PatchNext must not mutate original native graph");
  if e.pc==177{
   saw_patch_instruction+=1;
   assert_eq!(native_edge(&world),60);
  }
  if saw_patch_instruction>0&&e.pc==59{
   assert!(matches!(&e.kind,EventKind::CellWritten{key:33,value:2}));
   assert_eq!(world.process(1).unwrap().pc,60);
   assert!(!world.process(1).unwrap().stopped);
   saw_postservice=true;
   break;
  }
 }
 assert_eq!(saw_patch_instruction,1);
 assert!(saw_postservice);
 assert_eq!(steps,24_148);
 assert_eq!(native_edge(&world),60);
}
