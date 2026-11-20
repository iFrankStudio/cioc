//! CL1B-POLICY-P4F0: real original Rust CL0 CIL actor self-arms a
//! failure-triggered monitor using paid PatchNext PC82 -> Write PC12.next83.
//! At two actual unsuccessful task service attempts the same PID may pay
//! PatchConst PC88 to reconfigure executable search stride PC46.
//! All 110 worlds are independent original single-PID actors, not a lineage.
use cl0_world::{EventKind,Intervention,Op,Origin,Process,World};
use serde_json::{json,Value};
use std::collections::BTreeMap;
use std::fs::{self,File};
use std::io::{BufWriter,Write};

const MANIFEST:&str="../../../cl1b/studies/CL1B-POLICY-P4F0-frozen-event-gated-24-stage-110-world-v1.json";
const KEYS:[i64;7]=[11,20,21,30,32,33,34];
const CREDITS:u64=6000;

fn op(v:&Value)->Op{
 let tag=v["kind"].as_str().unwrap();
 let repr=if tag=="Halt"{json!("Halt")}else{
  let mut fields=serde_json::Map::new();
  for (k,val) in v.as_object().unwrap(){
   if k!="kind"{fields.insert(k.clone(),val.clone());}
  }
  let mut obj=serde_json::Map::new();
  obj.insert(tag.into(),Value::Object(fields));Value::Object(obj)
 };
 serde_json::from_value(repr).expect("frozen original CIL opcode invalid")
}
fn stride(w:&World)->i64{
 match &w.process(1).unwrap().code.nodes[46]{
  Op::Const{dst:2,value,next:47}=>*value,
  x=>panic!("source PC46 is not live original executable stride: {x:?}")
 }
}
fn fail_edge(w:&World)->usize{
 match &w.process(1).unwrap().code.nodes[12]{
  Op::Write{address:2,value:1,next}=>*next,
  x=>panic!("source PC12 is not original paid failure memory Write: {x:?}")
 }
}
fn run(code:&[Value],arm:&Value,dir:&std::path::Path)->Value{
 let name=arm["name"].as_str().unwrap();
 let goals=arm["goals"].as_array().unwrap().iter()
   .map(|v|v.as_i64().unwrap()).collect::<Vec<_>>();
 assert_eq!(goals.len(),24);
 assert!(goals.iter().all(|g|(0_i64..=2).contains(g)));
 let mode=arm["feedback"].as_str().unwrap();
 assert!(["truth","inverted","forced0"].contains(&mode));
 let disabled=arm["patchnext_suppressed"].as_bool().unwrap();
 let original=code.iter().map(op).collect::<Vec<_>>();
 assert_eq!(original.len(),91);
 let mut expected=original.clone();
 let mut world=World::with_strict_capacity(16,180,300);
 if disabled{world=world.with_native_patchnext_disabled();}
 world.load(Process::new(1,original,CREDITS)).unwrap();
 assert_eq!(world.occupied_units(),92);
 let mut trace=BufWriter::new(File::create(dir.join(format!("cl1b_policy_p4f0_original_{name}.jsonl"))).unwrap());
 let mut paid=0_usize;
 let mut peak=world.occupied_units();
 let mut services=Vec::<Value>::new();
 let mut reads=Vec::<Value>::new();
 let mut const_edits=Vec::<Value>::new();
 let mut edge_edits=Vec::<Value>::new();
 let mut memory_writes=Vec::<Value>::new();
 let mut memory_reads=Vec::<Value>::new();
 let mut failure_writes=Vec::<Value>::new();
 let mut recent_feedback:Option<i64>=None;
 while world.step(){
  paid+=1;
  assert!(paid<=CREDITS as usize,"original PID exceeded privately charged budget");
  let ev=world.events().last().unwrap().clone();
  assert!(matches!(ev.origin,Origin::Internal(1)),"non-PID event paid for original executable mutation");
  if ev.pc==3{
   assert!(matches!(&ev.kind,EventKind::CellWritten{key:11,..}));
  }
  if ev.pc==5{
   match &ev.kind{
    EventKind::CellRead{key:20,value}=>{
     recent_feedback=Some(*value);
     reads.push(json!({"step":paid,"value":value}));
    },
    x=>panic!("original actor missing paid Read20: {x:?}")
   }
  }
  if ev.pc==12{
   match &ev.kind{
    EventKind::CellWritten{key:33,value}=>{
     assert!((1..=3).contains(value));
     failure_writes.push(json!({"step":paid,"value":value}));
    },
    x=>panic!("not original charged failure-count write: {x:?}")
   }
  }
  if ev.pc==66{
   match &ev.kind{
    EventKind::CellWritten{key:34,value}=>{
     assert!(memory_writes.is_empty());
     memory_writes.push(json!({"step":paid,"value":value}));
    },
    x=>panic!("first goal is not original paid Write34: {x:?}")
   }
  }
  if ev.pc==69{
   match &ev.kind{
    EventKind::CellRead{key:34,value}=>{
     assert_eq!(memory_writes.len(),1);
     assert!(memory_reads.is_empty());
     assert_eq!(memory_writes[0]["value"],json!(value));
     memory_reads.push(json!({"step":paid,"value":value}));
    },
    x=>panic!("relative policy not using paid Read34: {x:?}")
   }
  }
  if ev.pc==45 || ev.pc==88{
   let pid=world.process(1).unwrap();
   assert_eq!(pid.registers[1],46);
   assert!(matches!(&ev.kind,EventKind::Executed));
   let desired=pid.registers[0];
   assert!((0..=2).contains(&desired));
   if ev.pc==45{
    assert_eq!(world.cell(32),Some(2));
    assert_eq!(recent_feedback,Some(1));
    assert_eq!(memory_reads.len(),1);
   }else{
    assert_eq!(world.cell(33),Some(2),"fallback may fire only after two paid failed service attempts");
    assert_eq!(desired,1,"fallback is researcher-authored constant stride1, not invented");
    assert_eq!(fail_edge(&world),83,"detector must be actor-armed by native PatchNext before fallback");
   }
   let old=match &expected[46]{
    Op::Const{dst:2,value,next:47}=>*value,
    x=>panic!("unexpected expected prior source: {x:?}")
   };
   expected[46]=Op::Const{dst:2,value:desired,next:47};
   assert_eq!(stride(&world),desired);
   const_edits.push(json!({"step":paid,"source_pc":ev.pc,"target":46,
    "before":old,"after":desired,"requested":desired}));
  }
  if ev.pc==82{
   assert_eq!(world.cell(32),Some(2));
   let pid=world.process(1).unwrap();
   assert_eq!(pid.registers[1],12);
   assert_eq!(pid.registers[0],83);
   assert!(edge_edits.is_empty(),"actor arms detector exactly once");
   let before=match &expected[12]{
    Op::Write{address:2,value:1,next}=>*next,
    x=>panic!("invalid prior failure path: {x:?}")
   };
   assert_eq!(before,13);
   if disabled{
    assert!(matches!(&ev.kind,EventKind::NativeCodeEditSuppressed{opcode_tag:6}));
   }else{
    assert!(matches!(&ev.kind,EventKind::Executed));
    expected[12]=Op::Write{address:2,value:1,next:83};
   }
   assert_eq!(fail_edge(&world),if disabled{13}else{83});
   edge_edits.push(json!({"step":paid,"source_pc":82,"target":12,"before":before,
    "after":fail_edge(&world),"requested":83,"suppressed":disabled}));
  }
  let p=world.process(1).unwrap();
  assert_eq!(p.code.nodes,expected,"unpaid, host-side or unexpected source edit");
  assert_eq!(p.credits,CREDITS-paid as u64);
  assert_eq!(world.total_compute_inventory(),Some(CREDITS-paid as u64));
  assert!(world.process(2).is_none());
  assert!(world.used_cells()<=16&&world.occupied_units()<=300);
  peak=peak.max(world.occupied_units());
  serde_json::to_writer(&mut trace,&json!(["cil",paid,ev.pc,p.pc,p.registers,p.credits,
   KEYS.map(|k|world.cell(k)),world.occupied_units(),fail_edge(&world),stride(&world)])).unwrap();
  trace.write_all(b"\n").unwrap();
  if ev.pc==3{
   let output=match &ev.kind{
    EventKind::CellWritten{key:11,value}=>*value,
    x=>panic!("task service not original paid Write11: {x:?}")
   };
   assert_eq!(output,p.registers[3]);
   let stage=world.cell(32).unwrap_or(0) as usize;
   assert!(stage<24);
   let truth=i64::from(output==goals[stage]);
   let feedback=match mode{"truth"=>truth,"inverted"=>1-truth,"forced0"=>0,_=>unreachable!()};
   services.push(json!({"step":paid,"episode":stage,"output":output,
    "truth":truth,"feedback":feedback}));
   world.intervene(Intervention::SetCell{key:20,value:feedback}).unwrap();
   let outside=world.events().last().unwrap();
   assert!(matches!(outside.origin,Origin::External));
   assert!(matches!(&outside.kind,EventKind::Intervened(_)));
   peak=peak.max(world.occupied_units());
   serde_json::to_writer(&mut trace,&json!(["feedback",paid,stage,goals[stage],
    output,truth,feedback,KEYS.map(|k|world.cell(k)),
    world.occupied_units(),fail_edge(&world),stride(&world)])).unwrap();
   trace.write_all(b"\n").unwrap();
  }
 }
 trace.flush().unwrap();
 assert!(world.process(1).unwrap().stopped);
 assert_eq!(reads.len(),services.len());
 assert_eq!(memory_writes.len(),if services.iter().any(|x|x["feedback"]==1){1}else{0});
 assert!(edge_edits.len()<=1);
 assert!(const_edits.len()>=if memory_reads.is_empty(){0}else{1});
 let actual_truth=services.iter().filter(|x|x["truth"]==1).count();
 let external=world.events().iter().filter(|e|matches!(e.origin,Origin::External)).collect::<Vec<_>>();
 assert_eq!(external.len(),services.len()+1);
 assert!(matches!(&external[0].kind,EventKind::Loaded));
 assert!(external.iter().skip(1).all(|e|matches!(&e.kind,EventKind::Intervened(_))));
 json!({"name":name,"regime":arm["regime"],"goals":goals,"feedback":mode,
  "patchnext_suppressed":disabled,
  "paid_steps":paid,"task_probes":services.len(),"actual_truthful_matches":actual_truth,
  "completed_stages":world.cell(32),"actor_claim":world.cell(21),
  "final_failure_edge":fail_edge(&world),"final_executable_stride":stride(&world),
  "paid_const_edits":const_edits,"paid_edge_edits":edge_edits,
  "paid_task_services":services,"paid_feedback_reads":reads,
  "paid_previous_goal_writes":memory_writes,"paid_previous_goal_reads":memory_reads,
  "paid_failure_count_writes":failure_writes,
  "peak_occupied":peak,"original_PID1_halted":true,"host_postload_source_edits":0})
}
#[test]
fn p4f0_original_native_110_world_24_stage_event_armed_policy_falsifier(){
 let root=std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
 let f:Value=serde_json::from_slice(&fs::read(root.join(MANIFEST)).unwrap()).unwrap();
 assert_eq!(f["study"],"CL1B-POLICY-P4F0");
 assert_eq!(f["status"],"PRE_NATIVE_FROZEN_EVENT_DRIVEN_DETECTOR_AND_HELDOUT_24_STAGE_PROTOCOL");
 let source=f["original_native_CIL_source"].as_array().unwrap();
 assert_eq!(source.len(),91);
 assert_eq!(source[12],json!({"kind":"Write","address":2,"value":1,"next":13}));
 assert_eq!(source[38],json!({"kind":"Const","dst":2,"value":24,"next":39}));
 assert_eq!(source[45],json!({"kind":"PatchConst","node":1,"value":0,"next":80}));
 assert_eq!(source[46],json!({"kind":"Const","dst":2,"value":1,"next":47}));
 assert_eq!(source[82],json!({"kind":"PatchNext","node":1,"target":0,"next":46}));
 assert_eq!(source[88],json!({"kind":"PatchConst","node":1,"value":0,"next":89}));
 let arms=f["arms"].as_array().unwrap();let controls=f["controls"].as_array().unwrap();
 assert_eq!(arms.len(),108);assert_eq!(controls.len(),2);
 let dir=root.join("../../cl1b-ci-evidence");
 fs::create_dir_all(&dir).unwrap();
 let mut pairs=Vec::<Value>::new();
 let mut counts=BTreeMap::<String,(i64,i64,u64,u64,u64,u64)>::new();
 let mut total=0_u64;
 for pair in arms.chunks_exact(2){
  let on_arm=&pair[0];let off_arm=&pair[1];
  assert_eq!(on_arm["goals"],off_arm["goals"]);
  assert_eq!(on_arm["regime"],off_arm["regime"]);
  assert_eq!(on_arm["patchnext_suppressed"],false);
  assert_eq!(off_arm["patchnext_suppressed"],true);
  let on=run(source,on_arm,&dir);let off=run(source,off_arm,&dir);
  for x in [&on,&off]{
   assert_eq!(x["actor_claim"],1);
   assert_eq!(x["completed_stages"],24);
   assert_eq!(x["actual_truthful_matches"],24);
   assert_eq!(x["paid_edge_edits"].as_array().unwrap().len(),1);
   assert!(x["paid_const_edits"].as_array().unwrap().len()>=1);
  }
  assert_eq!(on["final_failure_edge"],83);
  assert_eq!(off["final_failure_edge"],13);
  let saving=off["paid_steps"].as_i64().unwrap()-on["paid_steps"].as_i64().unwrap();
  let probe_saving=off["task_probes"].as_i64().unwrap()-on["task_probes"].as_i64().unwrap();
  total+=on["paid_steps"].as_u64().unwrap()+off["paid_steps"].as_u64().unwrap();
  let regime=on_arm["regime"].as_str().unwrap().to_string();
  let r=counts.entry(regime.clone()).or_insert((0,0,0,0,0,0));
  r.0+=saving;r.1+=probe_saving;r.2+=1;
  r.3+=u64::from(saving>0);r.4+=u64::from(saving<0);r.5+=u64::from(saving==0);
  pairs.push(json!({"regime":regime,"goals":on_arm["goals"],
    "detector_on":on,"patchnext_suppressed":off,
    "native_paid_saving":saving,"native_probe_saving":probe_saving}));
 }
 let mut adversity=Vec::<Value>::new();
 for control in controls{
  let result=run(source,control,&dir);
  total+=result["paid_steps"].as_u64().unwrap();
  adversity.push(result);
 }
 assert_eq!(adversity[0]["actor_claim"],1);
 assert!(adversity[0]["actual_truthful_matches"].as_u64().unwrap()<24);
 assert_eq!(adversity[1]["actor_claim"],0);
 let regime_json=counts.into_iter().map(|(name,(s,p,n,harmfree,harmed,tied))|
   (name,json!({"pairs":n,"net_paid_instruction_saving":s,"net_task_probe_saving":p,
    "beneficial":harmfree,"harmful":harmed,"neutral":tied})))
  .collect::<serde_json::Map<String,Value>>();
 let evidence=json!({"study":"CL1B-POLICY-P4F0",
  "level":"ORIGINAL_NATIVE_110_WORLD_EVENT_ARMED_PATCHNEXT24_EXACT_CIL",
  "source_sha":std::env::var("GITHUB_SHA").unwrap_or_else(|_|"local".into()),
  "pairs":pairs,"controls":adversity,
  "counts":{"native_worlds":110,"paired_worlds":108,"native_controls":2,
   "all_truthful_paired_worlds_complete_24":108,
   "total_paid_original_instructions":total,"regimes":regime_json},
  "limitations":"Actor-paid PatchNext arming and conditional PatchConst fallback are real; threshold2 and fallback stride1 were researcher-authored. 110 worlds are not a lineage; no open-ended cyberlife."});
 fs::write(dir.join("cl1b_policy_p4f0_native_event_armed_110_worlds.json"),
  serde_json::to_vec_pretty(&evidence).unwrap()).unwrap();
}
