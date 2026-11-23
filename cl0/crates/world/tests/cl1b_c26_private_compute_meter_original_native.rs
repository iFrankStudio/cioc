//! CL1B C26: original native charged PID-own resource metrology, source edit selection
//! Each of four original native worlds uses EXACT identical frozen 160-node CIL.
use cl0_world::{EventKind, Intervention, Op, Origin, Process, World};
use serde_json::{json,Value};
use std::fs::{self,File};
use std::io::{BufWriter,Write};
use std::path::Path;

const STUDY: &str = "../../../cl1b/studies/CL1B-C26-PAID-OWN-CREDIT-METER-CANDIDATE-CHOICE-frozen-4-worlds-v1.json";
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
 let mode=arm["mode"].as_i64().unwrap();
 let disabled=arm["patchnext_suppressed"].as_bool().unwrap();
 let name=arm["id"].as_str().unwrap();
 assert!(mode==0||mode==1);
 let initial=src.iter().map(decode).collect::<Vec<_>>();
 assert_eq!(initial.len(),160);
 assert_eq!(initial[0],Op::SenseOwnCredits{dst:0,next:1});
 assert_eq!(initial[30],Op::SenseOwnCredits{dst:0,next:31});
 assert_eq!(initial[60],Op::SenseOwnCredits{dst:0,next:61});
 assert_eq!(initial[95],Op::PatchNext{node:1,target:0,next:100});
 assert_eq!(initial[100],Op::Const{dst:2,value:50,next:101});
 let mut expected=initial.clone();
 let mut w=World::with_strict_capacity(16,160,185);
 if disabled{w=w.with_native_patchnext_disabled();}
 w.load(Process::new(1,initial,CREDIT)).unwrap();
 assert_eq!(w.occupied_units(),161);
 w.intervene(Intervention::SetCell{key:50,value:mode}).unwrap();
 assert_eq!(w.occupied_units(),162);
 let mut trace=BufWriter::new(File::create(folder.join(format!("cl1b_c26_original_{name}.jsonl"))).unwrap());
 let mut paid=0_usize;
 let mut samples=Vec::<Value>::new();
 let mut edits=Vec::<Value>::new();
 let mut services=0_usize;
 let mut trial_services=0_usize;
 let mut peak=w.occupied_units();
 while w.step(){
  paid+=1;
  assert!(paid<=CREDIT as usize);
  let ev=w.events().last().unwrap();
  assert_eq!(ev.origin,Origin::Internal(1));
  let pc=ev.pc;
  let p=w.process(1).unwrap();
  assert_eq!(p.credits,CREDIT-paid as u64);
  assert_eq!(ev.credits_after,p.credits);
  assert_eq!(w.total_compute_inventory(),Some(p.credits));
  assert!(!ev.pool_paid_attempt);
  if [0,30,60].contains(&pc){
   assert!(matches!(ev.kind,EventKind::Executed));
   assert_eq!(p.registers[0],p.credits as i64,"C26 private sensor must charge itself BEFORE observing");
   samples.push(json!({"step":paid,"pc":pc,"post_debit_private_credits":p.credits}));
  }
  if pc==95 {
   assert_eq!(p.registers[1],100);
   assert_eq!(p.registers[0],131);
   assert_eq!(source_edge(&w),if disabled{101}else{131});
   if disabled{
    assert!(matches!(ev.kind,EventKind::NativeCodeEditSuppressed{opcode_tag:6}));
   }else{
    assert!(matches!(ev.kind,EventKind::Executed));
    expected[100]=Op::Const{dst:2,value:50,next:131};
   }
   edits.push(json!({"step":paid,"pc":pc,"node":100,"new_next":if disabled{101}else{131},"suppressed":disabled}));
  }
  if pc==107||pc==137{
   assert!(matches!(ev.kind,EventKind::CellWritten{key:11,value:1}));
   services+=1;
  }
  if pc==12||pc==42 {
   assert!(matches!(ev.kind,EventKind::CellWritten{key:11,value:1}));
   trial_services+=1;
  }
  assert_eq!(p.code.nodes,expected,"unregistered program edit outside C26 ordinary PatchNext");
  assert!(w.used_cells()<=16&&w.occupied_units()<=185);
  peak=peak.max(w.occupied_units());
  let kind=match &ev.kind{
   EventKind::Executed=>"executed",
   EventKind::CellRead{..}=>"read",
   EventKind::CellWritten{..}=>"write",
   EventKind::NativeCodeEditSuppressed{opcode_tag:6}=>"suppressed",
   EventKind::Halted=>"halt",
   other=>panic!("unexpected native C26 CIL event {other:?}")
  };
  serde_json::to_writer(&mut trace,&json!({
   "charged_step":paid,"pc":pc,"next_pc":p.pc,"registers":p.registers,
   "own_credits_after":p.credits,
   "cells":CELLS.map(|k|w.cell(k)),
   "source_pc100_next":source_edge(&w),
   "occupied_units":w.occupied_units(),
   "original_native_event":kind
  })).unwrap();
  trace.write_all(b"\n").unwrap();
 }
 trace.flush().unwrap();
 let p=w.process(1).unwrap();
 assert!(p.stopped);
 assert_eq!(w.cell(45),Some(24));
 assert_eq!(w.cell(11),Some(1));
 assert_eq!(services,24);
 assert_eq!(trial_services,2);
 assert_eq!(samples.len(),3);
 let sensed=samples.iter().map(|v|v["post_debit_private_credits"].as_u64().unwrap()).collect::<Vec<_>>();
 assert_eq!(w.cell(41),Some(sensed[0] as i64));
 assert_eq!(w.cell(42),Some(sensed[1] as i64));
 assert_eq!(w.cell(43),Some(sensed[2] as i64));
 let cost_a=sensed[0]-sensed[1];
 let cost_b=sensed[1]-sensed[2];
 assert!(cost_a>0&&cost_b>0);
 assert_eq!(w.cell(44),Some(i64::from(cost_b<cost_a)));
 assert_eq!(w.cell(44),Some(mode));
 assert_eq!(edits.len(),if mode==1{1}else{0});
 assert_eq!(source_edge(&w),if mode==1&&!disabled{131}else{101});
 assert_eq!(p.credits,CREDIT-paid as u64);
 json!({"id":name,"mode":mode,"patchnext_suppressed":disabled,
   "total_paid_cil":paid,"private_credits_remain":p.credits,
   "sensor_records":samples,"trial_cost_a_paid":cost_a,"trial_cost_b_paid":cost_b,
   "actor_chosen_candidate":w.cell(44).unwrap(),
   "actor_paid_real_edit_records":edits,"final_executable_next_pc100":source_edge(&w),
   "total_real_production_services":services,"total_real_trial_services":trial_services,
   "completed_production_services":w.cell(45).unwrap(),"final_service_output":w.cell(11).unwrap(),
   "peak_storage":peak,"trapped":false,"original_pid_halted":true})
}
#[test]
fn c26_original_native_four_worlds_private_credit_meter_chooses_paid_source(){
 let root=Path::new(env!("CARGO_MANIFEST_DIR"));
 let m:Value=serde_json::from_slice(&fs::read(root.join(STUDY)).unwrap()).unwrap();
 assert_eq!(m["study"],"CL1B-C26-PAID-OWN-CREDIT-METROLOGY-CANDIDATE-CHOICE");
 assert_eq!(m["status"],"PRE_NATIVE_FROZEN_FOUR_WORLDS_GENERIC_ABI_TAG35");
 let src=m["original_program_160_node_CIL"].as_array().unwrap();
 let arms=m["arms"].as_array().unwrap();
 assert_eq!(src.len(),160);
 assert_eq!(arms.len(),4);
 let dir=root.join("../../cl1b-ci-evidence");
 fs::create_dir_all(&dir).unwrap();
 let mut results=Vec::<Value>::new();
 for (index,arm) in arms.iter().enumerate(){
   assert_eq!(arm["mode"],(index/2) as i64);
   assert_eq!(arm["patchnext_suppressed"],index%2==1);
   results.push(single_world(src,arm,&dir));
 }
 let off_m0=results[1]["total_paid_cil"].as_i64().unwrap();
 let on_m0=results[0]["total_paid_cil"].as_i64().unwrap();
 let off_m1=results[3]["total_paid_cil"].as_i64().unwrap();
 let on_m1=results[2]["total_paid_cil"].as_i64().unwrap();
 let summary=json!({
  "study":"CL1B-C26-PAID-OWN-CREDIT-METROLOGY-CANDIDATE-CHOICE",
  "level":"ORIGINAL_NATIVE_C26_PAID_OWN_COMPUTE_PRICE_COMPARISON_FOUR_WORLDS",
  "source_sha":std::env::var("GITHUB_SHA").unwrap_or_else(|_|"local".into()),
  "results":results,
  "counts":{"original_native_worlds":4,"actual_paid_cil_total":results.iter().map(|r|r["total_paid_cil"].as_u64().unwrap()).sum::<u64>(),
      "true_production_services":96,"true_trial_services":8},
  "same_initial_source_mode0_patchnext_effect_net_paid_saving":off_m0-on_m0,
  "same_initial_source_mode1_patchnext_effect_net_paid_saving":off_m1-on_m1,
  "scientific_status":"NEUTRAL_PAID_RESOURCE_OBSERVABILITY_AND_FINITE_AUTHOR_DEFINED_CANDIDATES_ONLY_B3_B4_NOT_MET"});
 fs::write(dir.join("cl1b_c26_original_native_four_worlds.json"),serde_json::to_vec_pretty(&summary).unwrap()).unwrap();
 println!("C26 ORIGINAL PAID SENSOR ARM COSTS mode0 on/off {on_m0}/{off_m0}, mode1 on/off {on_m1}/{off_m1}");
}
