//! G3Q-C1-P1-L3-I1: one original PID1 certifies its own researcher-authored
//! three-word rule, detects frozen unsafe RESET-tail counterexample, pays
//! PatchConst of its ORIGINAL executable source PC6 from3 to2, resets prior
//! witness and restarts from PC0, then pays to re-certify and serve real
//! hidden tasks with authentic feedback. Original physics unchanged.
//! The exact 178 CIL nodes were frozen BEFORE this Rust experiment.
use cl0_world::{EventKind,Intervention,Op,Origin,Process,World};
use serde_json::{json,Value};
use std::fs::{self,File};
use std::io::{BufWriter,Write};
const SOURCE:&str="../../../cl1b/studies/B56-F1d1k-G3Q-C1-P1-L3-I1-frozen-178-node-certificate-driven-native-self-repair-v1.json";
const KEYS:[i64;16]=[7,8,9,11,20,21,32,33,50,51,52,53,56,62,63,64];
const CREDITS:u64=8000;
fn decode(v:&Value)->Op {
 let name=v["kind"].as_str().unwrap();
 let tagged=if name=="Halt"{json!("Halt")}else{
  let mut fields=serde_json::Map::new();
  for (k,val) in v.as_object().unwrap(){if k!="kind"{fields.insert(k.clone(),val.clone());}}
  let mut tagged=serde_json::Map::new();
  tagged.insert(name.to_string(),Value::Object(fields));Value::Object(tagged)
 };
 serde_json::from_value(tagged).expect("invalid frozen CIL op")
}
fn executable_six(world:&World)->i64{
 match &world.process(1).unwrap().code.nodes[6]{
  Op::Const{dst:3,value,next:7}=>*value,
  other=>panic!("unexpected candidate literal PC6: {other:?}")
 }
}
#[test]
fn g3q_c1_i1_original_single_pid_paid_patchconst_recertification_and_hidden_task(){
 let root=std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
 let frozen:Value=serde_json::from_slice(&fs::read(root.join(SOURCE)).unwrap()).unwrap();
 let source=frozen["exact_native_founder_code"].as_array().unwrap();
 let arms=frozen["prespecified_original_arms"].as_array().unwrap();
 assert_eq!(source.len(),178);assert_eq!(arms.len(),3);
 assert_eq!(source[176],json!({"kind":"PatchConst","node":1,"value":3,"next":0}));
 let dir=root.join("../../cl1b-ci-evidence");fs::create_dir_all(&dir).unwrap();
 let mut reports=Vec::<Value>::new();
 for arm in arms {
  let name=arm["name"].as_str().unwrap();
  let mut records=source.clone();
  if arm["preload_mutation"]==true {
   assert_eq!(records[6]["value"],2);
   records[6]["value"]=json!(3);
  }
  let founder:Vec<Op>=records.iter().map(decode).collect();
  let mut after_repair=founder.clone();
  after_repair[6]=Op::Const{dst:3,value:2,next:7};
  let mut world=World::with_strict_capacity(16,180,300);
  world.load(Process::new(1,founder.clone(),CREDITS)).unwrap();
  assert_eq!(world.occupied_units(),179);
  let mut paid=0_usize;
  let mut peak=world.occupied_units();
  let mut edit_records=Vec::<Value>::new();
  let mut witnesses=Vec::<Value>::new();
  let mut services=Vec::<Value>::new();
  let mut successes=0_usize;
  let mut reset_witness_events=0_usize;
  let mut current_source_mutated=false;
  let mut real_meta_fetches=0_usize;
  let mut feedback_read=Vec::<i64>::new();
  let mut emitted_feedback=Vec::<i64>::new();
  let mut trace=BufWriter::new(File::create(dir.join(format!(
     "b56_f1d1k_g3q_c1_p1_l3_i1_original_paid_{name}.jsonl"))).unwrap());
  let goals=arm["goals"].as_array().unwrap().iter().map(|x|x.as_i64().unwrap()).collect::<Vec<_>>();
  let regime=arm["feedback"].as_str().unwrap();
  while world.step(){
   paid+=1;assert!(paid<=CREDITS as usize);
   let ev=world.events().last().unwrap().clone();
   assert!(matches!(ev.origin,Origin::Internal(1)));
   if ev.pc==176{
    assert_eq!(name!="already_safe",true);
    assert!(!current_source_mutated);
    assert_eq!(executable_six(&world),2);
    assert!(matches!(&ev.kind,EventKind::Executed));
    current_source_mutated=true;
    edit_records.push(json!({"step":paid,"pc":176,"node":6,
                             "before":3,"after":2,"suppressed":false}));
   }
   let p=world.process(1).unwrap();
   assert_eq!(&p.code.nodes,if current_source_mutated{&after_repair}else{&founder},
     "all source mutation must be the single paid PatchConst of node6");
   assert_eq!(world.total_compute_inventory(),Some(CREDITS-paid as u64));
   assert!(world.process(2).is_none());
   assert!(world.used_cells()<=16&&world.occupied_units()<=300);
   peak=peak.max(world.occupied_units());
   if ev.pc==130{
    match &ev.kind{
     EventKind::CellWritten{key,value}=>{
      assert!((7..=9).contains(key));assert_eq!(*value,1);
      witnesses.push(json!({"step":paid,"key":key,"value":value}));
     },
     other=>panic!("original actor did not pay to record witness: {other:?}")
    }
   }
   if ev.pc==173 {
    assert!(matches!(&ev.kind,EventKind::CellWritten{key:7,value:0}));
    reset_witness_events+=1;
   }
   if ev.pc==170{
    assert!(matches!(&ev.kind,EventKind::CellWritten{key:21,value:1}));
    assert_eq!([world.cell(7),world.cell(8),world.cell(9)],[Some(1),Some(1),Some(1)]);
    successes+=1;
   }
   if ev.pc==37{
    assert!(matches!(&ev.kind,EventKind::CellRead{..}));
    real_meta_fetches+=1;
   }
   if ev.pc==79{
    match &ev.kind{
     EventKind::CellRead{key:20,value}=>feedback_read.push(*value),
     other=>panic!("original source failed to pay for Read20: {other:?}")
    }
   }
   let paid_state=json!(["cil",paid,ev.pc,p.pc,p.registers,p.credits,
       KEYS.map(|k|world.cell(k)),world.occupied_units(),executable_six(&world)]);
   serde_json::to_writer(&mut trace,&paid_state).unwrap();trace.write_all(b"\n").unwrap();
   if ev.pc==71{
    assert_eq!(successes,1,"real tasks can only start after actor recertification");
    let out=match &ev.kind{
     EventKind::CellWritten{key:11,value}=>*value,
     other=>panic!("expected native actor paid real task Write11: {other:?}")
    };
    let episode=world.cell(32).unwrap() as usize;
    assert!(episode<4);
    let truth=i64::from(out==goals[episode]);
    let feedback=match regime{"truth"=>truth,"inverted"=>1-truth,
      other=>panic!("unsupported feedback mode {other}")};
    services.push(json!({"step":paid,"epoch":episode,"middle":world.cell(63),
       "output":out,"truth":truth,"feedback":feedback}));
    emitted_feedback.push(feedback);
    world.intervene(Intervention::SetCell{key:20,value:feedback}).unwrap();
    let ext=world.events().last().unwrap();
    assert!(matches!(ext.origin,Origin::External));
    assert!(matches!(&ext.kind,EventKind::Intervened(_)));
    let record=json!(["feedback",paid,episode,goals[episode],world.cell(63),
       out,truth,feedback,KEYS.map(|k|world.cell(k)),world.occupied_units(),
       executable_six(&world)]);
    serde_json::to_writer(&mut trace,&record).unwrap();trace.write_all(b"\n").unwrap();
   }
  }
  trace.flush().unwrap();
  let expect=&arm["expected"];
  assert_eq!(paid,expect["steps"].as_u64().unwrap() as usize);
  assert_eq!(world.process(1).unwrap().pc,expect["halt"].as_u64().unwrap() as usize);
  assert!(world.process(1).unwrap().stopped);
  assert_eq!(json!(edit_records),expect["patches"]);
  assert_eq!(json!(witnesses.iter().map(|v|json!([
     v["step"],v["key"],v["value"]])).collect::<Vec<_>>()),expect["cert_witnesses"]);
  assert_eq!(json!(services),expect["services"]);
  assert_eq!(executable_six(&world),expect["sourcePC6value"].as_i64().unwrap());
  assert_eq!(successes,expect["certPass"].as_array().unwrap().len());
  assert_eq!(reset_witness_events,edit_records.len());
  assert_eq!(real_meta_fetches,expect["meta"].as_u64().unwrap() as usize);
  assert_eq!(feedback_read,emitted_feedback);
  assert_eq!(world.used_cells(),expect["maxCells"].as_u64().unwrap() as usize);
  assert_eq!(peak,founder.len()+1+world.used_cells());
  assert_eq!(world.total_compute_inventory(),Some(CREDITS-paid as u64));
  assert_eq!(world.cell(21),Some(1));
  let externals=world.events().iter().filter(|e|matches!(e.origin,Origin::External)).collect::<Vec<_>>();
  assert_eq!(externals.len(),1+services.len());
  assert!(matches!(&externals[0].kind,EventKind::Loaded));
  assert!(externals.iter().skip(1).all(|ev|matches!(&ev.kind,EventKind::Intervened(_))));
  reports.push(json!({"arm":name,"original_paid_steps":paid,"credits_remaining":CREDITS-paid as u64,
    "original_pid1_halted":true,"source_edits":edit_records,
    "structural_certificate_success_count":successes,"paid_witnesses":witnesses,
    "paid_task_services":services,"paid_metacode_fetches":real_meta_fetches,
    "paid_feedback_reads":feedback_read,"final_executable_pc6":executable_six(&world),
    "peak_occupied":peak,"postload_external_source_or_rule_writes":0}));
 }
 fs::write(dir.join("b56_f1d1k_g3q_c1_p1_l3_i1_native_source_self_repair.json"),
   serde_json::to_vec_pretty(&json!({
     "study":"B56-F1d1k-G3Q-C1-P1-L3-I1",
     "level":"NATIVE_ONE_PID_PROGRAMMED_CERTIFICATE_DRIVEN_SOURCE_REPAIR_AND_TASK_RELEARNING",
     "source_sha":std::env::var("GITHUB_SHA").unwrap_or_else(|_|"local".to_string()),
     "arms":reports,
     "limits":"Narrow preprogrammed literal repair only; not self-discovered generic rule replacement or general cyberlife."
   })).unwrap()).unwrap();
}
#[test]
fn g3q_c1_i1_patchconst_disabled_world_ablation_no_certification_progress(){
 let root=std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
 let f:Value=serde_json::from_slice(&fs::read(root.join(SOURCE)).unwrap()).unwrap();
 let control=&f["pre_registered_patch_suppressed_negative_world"];
 let mut source=f["exact_native_founder_code"].as_array().unwrap().clone();
 assert_eq!(source[6]["value"],2);source[6]["value"]=json!(3);
 let code:Vec<Op>=source.iter().map(decode).collect();
 let mut world=World::with_strict_capacity(16,180,300)
   .with_native_code_edits_disabled();
 world.load(Process::new(1,code.clone(),CREDITS)).unwrap();
 let folder=root.join("../../cl1b-ci-evidence");fs::create_dir_all(&folder).unwrap();
 let mut trace=BufWriter::new(File::create(folder.join(
   "b56_f1d1k_g3q_c1_p1_l3_i1_original_paid_unsafe_seed_edit_suppressed.jsonl")).unwrap());
 let mut paid=0_usize;let mut edit_attempts=0_usize;
 let mut failed_certificates=0_usize;let mut services=0_usize;
 while world.step(){
  paid+=1;assert!(paid<=CREDITS as usize);
  let ev=world.events().last().unwrap().clone();
  assert!(matches!(ev.origin,Origin::Internal(1)));
  assert_eq!(world.process(1).unwrap().code.nodes,code);
  assert_eq!(world.total_compute_inventory(),Some(CREDITS-paid as u64));
  assert_eq!(executable_six(&world),3);
  if ev.pc==176{
   assert!(matches!(&ev.kind,EventKind::NativeCodeEditSuppressed{opcode_tag:9}));
   edit_attempts+=1;
  }
  if ev.pc==173{failed_certificates+=1;}
  if ev.pc==71{services+=1;}
  let p=world.process(1).unwrap();
  serde_json::to_writer(&mut trace,&json!(["cil",paid,ev.pc,p.pc,
   p.registers,p.credits,KEYS.map(|k|world.cell(k)),world.occupied_units(),
   executable_six(&world)])).unwrap();trace.write_all(b"\n").unwrap();
 }
 trace.flush().unwrap();
 let expected=&control["expected"];
 assert_eq!(paid,8000);
 assert_eq!(edit_attempts,expected["source_edits"].as_u64().unwrap() as usize);
 assert_eq!(failed_certificates,expected["repair_failures"].as_u64().unwrap() as usize);
 assert_eq!(services,0);
 assert_eq!(world.used_cells(),expected["used_cells"].as_u64().unwrap() as usize);
 assert_eq!(world.occupied_units(),expected["occupied"].as_u64().unwrap() as usize);
 assert_eq!(world.process(1).unwrap().pc,expected["pc"].as_u64().unwrap() as usize);
 assert!(!world.process(1).unwrap().stopped,
   "no original Halt: native process only runs out of compute");
 assert_eq!(world.total_compute_inventory(),Some(0));
}
