//! CL1B C27: original native paid candidate remeasurement and physical source rollback across environment changes
//! Each of eight original native worlds uses EXACT identical frozen 160-node CIL.
use cl0_world::{EventKind, Intervention, Op, Origin, Process, World};
use serde_json::{json,Value};
use std::fs::{self,File};
use std::io::{BufWriter,Write};
use std::path::Path;

const STUDY: &str = "../../../cl1b/studies/CL1B-C27-PAID-CREDIT-REMEASURE-ROLLBACK-frozen-8-worlds-v1.json";
const CREDIT: u64 = 3000;
const CELLS: [i64;7] = [11,41,42,43,44,45,50];

fn decode(v:&Value)->Op{
 let kind=v["kind"].as_str().unwrap();
 let obj=if kind=="Halt"{json!("Halt")}else{
  let mut map=v.as_object().unwrap().clone();
  map.remove("kind");
  let mut outer=serde_json::Map::new();
  outer.insert(kind.into(),Value::Object(map));
  Value::Object(outer)
 };
 serde_json::from_value(obj).expect("invalid C26 pre-native frozen CIL source")
}

fn source_edge(w:&World)->usize{
 match &w.process(1).unwrap().code.nodes[100]{
  Op::Const{dst:2,value:50,next}=>*next,
  other=>panic!("C26 source edge did not remain executable Const {other:?}")
 }
}

fn single_world(src:&[Value],arm:&Value,folder:&Path)->Value{
 let start=arm["initial_mode"].as_i64().unwrap();
 let change=arm["mode_after_stage12"].as_i64();
 let disabled=arm["patchnext_suppressed"].as_bool().unwrap();
 let id=arm["id"].as_str().unwrap();
 assert!(start==0||start==1);
 assert!(change.is_none()||change==Some(0)||change==Some(1));
 let initial=src.iter().map(decode).collect::<Vec<_>>();
 assert_eq!(initial.len(),170);
 assert_eq!(initial[0],Op::SenseOwnCredits{dst:0,next:1});
 assert_eq!(initial[30],Op::SenseOwnCredits{dst:0,next:31});
 assert_eq!(initial[60],Op::SenseOwnCredits{dst:0,next:61});
 assert_eq!(initial[95],Op::PatchNext{node:1,target:0,next:100});
 assert_eq!(initial[164],Op::ReadNext{dst:0,node:1,next:165});
 assert_eq!(initial[169],Op::PatchNext{node:1,target:0,next:100});
 assert_eq!(initial[100],Op::Const{dst:2,value:50,next:101});
 let mut expected=initial.clone();
 let mut world=World::with_strict_capacity(16,170,196);
 if disabled{world=world.with_native_patchnext_disabled();}
 world.load(Process::new(1,initial,CREDIT)).unwrap();
 assert_eq!(world.occupied_units(),171);
 world.intervene(Intervention::SetCell{key:50,value:start}).unwrap();
 assert_eq!(world.occupied_units(),172);
 let mut out=BufWriter::new(File::create(folder.join(format!("cl1b_c27_original_{id}.jsonl"))).unwrap());
 let mut paid=0_usize;
 let mut sensors=Vec::<Value>::new();
 let mut decisions=Vec::<Value>::new();
 let mut edits=Vec::<Value>::new();
 let mut environment_events=Vec::<Value>::new();
 let mut midpoint_seen=false;
 let mut production=0_usize;
 let mut trials=0_usize;
 let mut peak=world.occupied_units();
 while world.step(){
  paid+=1;
  assert!(paid<=CREDIT as usize);
  let e=world.events().last().unwrap();
  assert_eq!(e.origin,Origin::Internal(1));
  let pc=e.pc;
  let actor=world.process(1).unwrap();
  assert_eq!(actor.credits,CREDIT-paid as u64);
  assert_eq!(e.credits_after,actor.credits);
  assert!(!e.pool_paid_attempt);
  assert_eq!(world.total_compute_inventory(),Some(actor.credits));
  if [0,30,60].contains(&pc){
   assert!(matches!(e.kind,EventKind::Executed));
   assert_eq!(actor.registers[0],actor.credits as i64);
   sensors.push(json!({"step":paid,"pc":pc,"own_private_credits_after":actor.credits,
    "mode":world.cell(50),"stage":world.cell(45).unwrap_or(0)}));
  }
  if [82,92].contains(&pc){
   assert!(matches!(e.kind,EventKind::CellWritten{key:44,..}));
   let chosen=world.cell(44).unwrap();
   assert_eq!(chosen,if pc==82{0}else{1});
   assert_eq!(chosen,world.cell(50).unwrap());
   decisions.push(json!({"step":paid,"pc":pc,"chosen":chosen,
    "mode":world.cell(50),"stage":world.cell(45).unwrap_or(0)}));
  }
  if [95,169].contains(&pc){
   let (node,target)=if pc==95{(100,131)}else{(100,101)};
   assert_eq!(actor.registers[1],node);
   assert_eq!(actor.registers[0],target);
   let prior=match &expected[node as usize]{
    Op::Const{dst:2,value:50,next}=>*next,
    other=>panic!("C27 cannot patch non-Const dispatch {other:?}")
   };
   if disabled{
    assert!(matches!(e.kind,EventKind::NativeCodeEditSuppressed{opcode_tag:6}));
   }else{
    assert!(matches!(e.kind,EventKind::Executed));
    expected[node as usize]=Op::Const{dst:2,value:50,next:target as usize};
   }
   edits.push(json!({"step":paid,"pc":pc,"node":node,"attempted_target":target,
    "prior":prior,"actual_after":source_edge(&world),"suppressed":disabled,
    "mode":world.cell(50),"stage":world.cell(45).unwrap_or(0)}));
  }
  if pc==164{
   assert!(matches!(e.kind,EventKind::Executed));
   assert_eq!(actor.registers[0],source_edge(&world) as i64);
  }
  if [107,137].contains(&pc){
   assert!(matches!(e.kind,EventKind::CellWritten{key:11,value:1}));
   production+=1;
  }
  if [12,42].contains(&pc){
   assert!(matches!(e.kind,EventKind::CellWritten{key:11,value:1}));
   trials+=1;
  }
  assert_eq!(actor.code.nodes,expected,"unregistered C27 source edit");
  assert!(world.used_cells()<=16&&world.occupied_units()<=196);
  peak=peak.max(world.occupied_units());
  let kind=match &e.kind{
   EventKind::Executed=>"executed",
   EventKind::CellRead{..}=>"read",
   EventKind::CellWritten{..}=>"write",
   EventKind::NativeCodeEditSuppressed{opcode_tag:6}=>"suppressed",
   EventKind::Halted=>"halt",
   x=>panic!("C27 unexpected native CIL event {x:?}")
  };
  serde_json::to_writer(&mut out,&json!({
   "charged_step":paid,"pc":pc,"next_pc":actor.pc,"registers":actor.registers,
   "own_credits_after":actor.credits,
   "cells":CELLS.map(|k|world.cell(k)),"source_pc100_next":source_edge(&world),
   "occupied_units":world.occupied_units(),"original_native_event":kind
  })).unwrap();
  out.write_all(b"\n").unwrap();
  if pc==155 && world.cell(45)==Some(12) && !midpoint_seen{
   midpoint_seen=true;
   if let Some(new_mode)=change{
    let old=world.cell(50).unwrap();
    assert_ne!(old,new_mode);
    world.intervene(Intervention::SetCell{key:50,value:new_mode}).unwrap();
    let event=json!({"after_charged_step":paid,"stage":12,"key":50,
      "old_mode":old,"new_mode":new_mode,"occupied_units":world.occupied_units(),
      "source_pc100_next":source_edge(&world)});
    environment_events.push(event.clone());
    serde_json::to_writer(&mut out,&json!({"environment_change":event})).unwrap();
    out.write_all(b"\n").unwrap();
   }
  }
 }
 out.flush().unwrap();
 let actor=world.process(1).unwrap();
 assert!(actor.stopped);
 assert!(midpoint_seen);
 assert_eq!(world.cell(45),Some(24));
 assert_eq!(world.cell(11),Some(1));
 assert_eq!(production,24);
 assert_eq!(trials,4);
 assert_eq!(sensors.len(),6);
 assert_eq!(decisions.len(),2);
 assert_eq!(environment_events.len(),if change.is_some(){1}else{0});
 let costs=sensors.chunks_exact(3).map(|chunk|{
  let a=chunk[0]["own_private_credits_after"].as_u64().unwrap()
      -chunk[1]["own_private_credits_after"].as_u64().unwrap();
  let b=chunk[1]["own_private_credits_after"].as_u64().unwrap()
      -chunk[2]["own_private_credits_after"].as_u64().unwrap();
  json!({"A":a,"B":b,"actor_chose_B":b<a})
 }).collect::<Vec<_>>();
 assert_eq!(costs.len(),2);
 let mode2=change.unwrap_or(start);
 assert_eq!(decisions[0]["chosen"],start);
 assert_eq!(decisions[1]["chosen"],mode2);
 assert_eq!(costs[0]["actor_chose_B"],start==1);
 assert_eq!(costs[1]["actor_chose_B"],mode2==1);
 let final_edge=if disabled||mode2==0{101}else{131};
 assert_eq!(source_edge(&world),final_edge);
 let restore=edits.iter().filter(|e|e["pc"]==169).count();
 assert_eq!(restore,if start==1&&mode2==0&&!disabled{1}else{0});
 assert_eq!(actor.credits,CREDIT-paid as u64);
 json!({"id":id,"initial_mode":start,"mode_after_stage12":change,
  "patchnext_suppressed":disabled,
  "total_paid_cil":paid,"private_credits_remain":actor.credits,
  "sensor_records":sensors,"cost_comparisons_by_paid_phase":costs,
  "actor_paid_decisions":decisions,
  "actor_paid_patchnext_edits":edits,
  "actor_paid_live_rollback_pc169_count":restore,
  "environment_events":environment_events,
  "final_executable_next_pc100":source_edge(&world),
  "real_production_services":production,"real_trial_services":trials,
  "completed_production_services":world.cell(45).unwrap(),
  "final_output":world.cell(11).unwrap(),"peak_occupied":peak,
  "original_pid_halted":true,"trapped":false})
}
#[test]
fn c27_original_paid_candidate_remeasure_and_native_source_rollback_eight_worlds(){
 let root=Path::new(env!("CARGO_MANIFEST_DIR"));
 let m:Value=serde_json::from_slice(&fs::read(root.join(STUDY)).unwrap()).unwrap();
 let old:Value=serde_json::from_slice(&fs::read(root.join("../../../cl1b/studies/CL1B-C26-PAID-OWN-CREDIT-METER-CANDIDATE-CHOICE-frozen-4-worlds-v1.json")).unwrap()).unwrap();
 assert_eq!(m["study"],"CL1B-C27-INTRA-LIFECYCLE-PAID-RESOURCE-REMEASUREMENT-SOURCE-ROLLBACK");
 assert_eq!(m["status"],"PRE_NATIVE_FROZEN_8_ORIGINAL_WORLDS_IN_LIFECYCLE_ADAPT_AND_REVERT");
 assert_eq!(old["study"],"CL1B-C26-PAID-OWN-CREDIT-METROLOGY-CANDIDATE-CHOICE");
 let source=m["original_full_frozen_170_node_CIL_source"].as_array().unwrap();
 let parent=old["original_program_160_node_CIL"].as_array().unwrap();
 assert_eq!(source.len(),170);
 for i in 0..160{
  if i!=82&&i!=158{assert_eq!(source[i],parent[i],"C27 unexpected inherited C26 CIL source change at {i}");}
 }
 assert_eq!(source[82],json!({"kind":"Write","address":2,"value":0,"next":163}));
 assert_eq!(source[158],json!({"kind":"Branch","test":0,"nonzero":160,"zero":159}));
 assert_eq!(source[164],json!({"kind":"ReadNext","dst":0,"node":1,"next":165}));
 assert_eq!(source[169],json!({"kind":"PatchNext","node":1,"target":0,"next":100}));
 let arms=m["arms"].as_array().unwrap();
 assert_eq!(arms.len(),8);
 let dir=root.join("../../cl1b-ci-evidence");
 fs::create_dir_all(&dir).unwrap();
 let mut results=Vec::<Value>::new();
 for (i,arm) in arms.iter().enumerate(){
  assert_eq!(arm["patchnext_suppressed"],i%2==1);
  results.push(single_world(source,arm,&dir));
 }
 let mut savings=Vec::<Value>::new();
 for i in 0..4{
  let on=&results[i*2];
  let off=&results[i*2+1];
  assert_eq!(on["initial_mode"],off["initial_mode"]);
  assert_eq!(on["mode_after_stage12"],off["mode_after_stage12"]);
  let x=off["total_paid_cil"].as_i64().unwrap()-on["total_paid_cil"].as_i64().unwrap();
  savings.push(json!({"cohort":on["id"],"same_initial_source_paid_self_edit_effect_saving":x,
    "on_paid":on["total_paid_cil"],"off_paid":off["total_paid_cil"]}));
 }
 let sum=results.iter().map(|r|r["total_paid_cil"].as_u64().unwrap()).sum::<u64>();
 let summary=json!({"study":"CL1B-C27-INTRA-LIFECYCLE-PAID-RESOURCE-REMEASUREMENT-SOURCE-ROLLBACK",
  "level":"ORIGINAL_NATIVE_C27_EIGHT_WORLD_PAID_RESOURCE_REMEASURE_AND_SOURCE_ROLLBACK",
  "source_sha":std::env::var("GITHUB_SHA").unwrap_or_else(|_|"local".into()),
  "results":results,"same_initial_source_paid_effect_by_regime":savings,
  "counts":{"original_native_worlds":8,"total_paid_CIL":sum,
    "production_services":192,"trial_services":32,"paid_private_sensor_executions":48},
  "scientific_scope":"Author-defined two candidates and two environment regimes, actual paid private-resource remeasurement, native self-edit and rollback; not invented search, no B3 or B4."
 });
 fs::write(dir.join("cl1b_c27_original_native_eight_worlds.json"),
  serde_json::to_vec_pretty(&summary).unwrap()).unwrap();
 println!("C27 ORIGINAL 8 PAID NATIVE WORLDS {:?}",savings);
}
