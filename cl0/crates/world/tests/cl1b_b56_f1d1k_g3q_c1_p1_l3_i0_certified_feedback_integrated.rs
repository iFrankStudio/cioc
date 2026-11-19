//! G3Q-C1-P1-L3-I0: one original PID1 certifies its own active three-word
//! coverage, gates actual task service, and relearns via real external feedback.
//! Original grammar and proposed candidates researcher-authored, not evolved.
use cl0_world::{EventKind,Intervention,Op,Origin,Process,World};
use serde_json::{json,Value};
use std::fs::{self,File};
use std::io::{BufWriter,Write};
const SOURCE:&str="../../../cl1b/studies/B56-F1d1k-G3Q-C1-P1-L3-I0-frozen-single-PID-certified-feedback-founder-v1.json";
const ANCESTOR:&str="../../../cl1b/studies/B56-F1d1k-G3Q-C1-P1-L3-F0-frozen-122-node-feedback-relearning-founder-v1.json";
const KEYS:[i64;16]=[7,8,9,11,20,21,32,33,50,51,52,53,56,62,63,64];
const CREDIT:u64=8000;
fn decode(v:&Value)->Op {
 let tag=v["kind"].as_str().unwrap();
 let wrapped=if tag=="Halt"{json!("Halt")}else{
  let mut f=serde_json::Map::new();
  for (k,val) in v.as_object().unwrap(){if k!="kind"{f.insert(k.clone(),val.clone());}}
  let mut w=serde_json::Map::new();w.insert(tag.into(),Value::Object(f));Value::Object(w)
 };
 serde_json::from_value(wrapped).expect("frozen original CIL source invalid")
}
#[test]
fn g3q_c1_p1_l3_i0_native_one_pid_certification_gated_feedback_relearning(){
 let root=std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
 let frozen:Value=serde_json::from_slice(&fs::read(root.join(SOURCE)).unwrap()).unwrap();
 let ancestor:Value=serde_json::from_slice(&fs::read(root.join(ANCESTOR)).unwrap()).unwrap();
 let orig=ancestor["exact_native_founder_code"].as_array().unwrap();
 let nodes=frozen["exact_native_founder_code"].as_array().unwrap();
 let arms=frozen["prespecified_arms"].as_array().unwrap();
 assert_eq!(nodes.len(),172);
 assert_eq!(orig.len(),122);
 assert_eq!(arms.len(),4);
 assert_eq!(frozen["strict_world"]["data_cell_capacity"],16);
 assert_eq!(frozen["strict_world"]["source_nodes_max"],180);
 assert_eq!(frozen["strict_world"]["total_storage_capacity"],300);
 assert_eq!(&nodes[..67],&orig[..67]);
 assert_eq!(&nodes[68..122],&orig[68..122]);
 assert_eq!(nodes[67]["zero"],122);
 let dir=root.join("../../cl1b-ci-evidence");
 fs::create_dir_all(&dir).unwrap();
 let mut results=Vec::<Value>::new();
 for arm in arms{
  let name=arm["name"].as_str().unwrap();
  let expected=&arm["expected"];
  let mut records=nodes.clone();
  if let Some(m)=arm["mutation"].as_object(){
   let pc=m["pc"].as_u64().unwrap() as usize;
   assert!([3,6].contains(&pc));
   assert_eq!(records[pc]["value"],m["value_before"]);
   records[pc]["value"]=m["value_after"].clone();
  }
  let founder:Vec<Op>=records.iter().map(decode).collect();
  let goals=arm["goals"].as_array().unwrap().iter().map(|x|x.as_i64().unwrap()).collect::<Vec<_>>();
  let regime=arm["feedback"].as_str().unwrap();
  assert_eq!(goals.len(),4);
  let mut world=World::with_strict_capacity(16,180,300);
  world.load(Process::new(1,founder.clone(),CREDIT)).unwrap();
  assert_eq!(world.occupied_units(),173);
  assert_eq!(world.used_cells(),0);
  let mut steps=0_usize;let mut peak=world.occupied_units();
  let mut marks=Vec::<Value>::new();let mut services=Vec::<Value>::new();
  let mut meta_reads=0_usize;let mut accumulators=0_usize;
  let mut cert_pass_count=0_usize;let mut actor_feedback_reads=Vec::<i64>::new();
  let mut sent_feedback=Vec::<i64>::new();
  let mut trace=BufWriter::new(File::create(dir.join(format!(
     "b56_f1d1k_g3q_c1_p1_l3_i0_native_paid_{name}.jsonl"))).unwrap());
  while world.step(){
   steps+=1;
   assert!(steps<=CREDIT as usize);
   let ev=world.events().last().unwrap().clone();
   assert!(matches!(ev.origin,Origin::Internal(1)));
   let pid=world.process(1).unwrap();
   assert_eq!(pid.code.nodes,founder,"native original source cannot be host-rewritten or self-edited in I0");
   assert_eq!(world.total_compute_inventory(),Some(CREDIT-steps as u64));
   assert!(world.process(2).is_none());
   assert!(world.used_cells()<=16&&world.occupied_units()<=300);
   peak=peak.max(world.occupied_units());
   if ev.pc==37{
    assert!(matches!(&ev.kind,EventKind::CellRead{..}));
    meta_reads+=1;
   }
   if ev.pc==58{
    assert!(matches!(&ev.kind,EventKind::CellWritten{key:56,..}));
    accumulators+=1;
   }
   if ev.pc==130{
    match &ev.kind{
     EventKind::CellWritten{key,value}=>{
      assert!((7..=9).contains(key));
      assert_eq!(*value,1);
      assert_eq!(world.cell(21),Some(0),"task service must be gated before certificate");
      marks.push(json!({"step":steps,"key":key,"value":value}));
     }
     other=>panic!("structural certificate must be PID1-paid flag Write: {other:?}")
    }
   }
   if ev.pc==170 {
    assert!(matches!(&ev.kind,EventKind::CellWritten{key:21,value:1}));
    assert_eq!(marks.len(),3);
    assert_eq!([world.cell(7),world.cell(8),world.cell(9)],[Some(1),Some(1),Some(1)]);
    cert_pass_count+=1;
   }
   if ev.pc==79{
    match &ev.kind{
     EventKind::CellRead{key:20,value}=>actor_feedback_reads.push(*value),
     other=>panic!("no original actor-paid Read20 feedback: {other:?}")
    }
   }
   let paid=json!(["cil",steps,ev.pc,pid.pc,pid.registers,pid.credits,
     KEYS.map(|k|world.cell(k)),world.occupied_units()]);
   serde_json::to_writer(&mut trace,&paid).unwrap();trace.write_all(b"\n").unwrap();
   if ev.pc==71 {
    assert_eq!(cert_pass_count,1,"native task service executed before the internal certificate gate");
    let result=match &ev.kind{
     EventKind::CellWritten{key:11,value}=>*value,
     other=>panic!("service must be PID1 paid Write11: {other:?}")
    };
    let ep=world.cell(32).unwrap() as usize;
    assert!(ep<4);
    let mid=world.cell(63).unwrap();
    let truth=i64::from(result==goals[ep]);
    let feedback=match regime{"truth"=>truth,"inverted"=>1-truth,
      other=>panic!("unregistered feedback regime: {other}")};
    let record=json!({"step":steps,"episode":ep,"middle":mid,
       "output":result,"truth":truth,"feedback":feedback});
    services.push(record);
    sent_feedback.push(feedback);
    world.intervene(Intervention::SetCell{key:20,value:feedback}).unwrap();
    assert!(matches!(world.events().last().unwrap().origin,Origin::External));
    assert_eq!(world.cell(20),Some(feedback));
    let env=json!(["feedback",steps,ep,goals[ep],mid,result,truth,feedback,
      KEYS.map(|k|world.cell(k)),world.occupied_units()]);
    serde_json::to_writer(&mut trace,&env).unwrap();trace.write_all(b"\n").unwrap();
   }
  }
  trace.flush().unwrap();
  assert_eq!(steps,expected["paid_steps"].as_u64().unwrap() as usize);
  assert_eq!(CREDIT-steps as u64,expected["credits_remaining"].as_u64().unwrap());
  assert_eq!(json!(marks),expected["certificate_witness_events"]);
  assert_eq!(json!(services),expected["service_events"]);
  assert_eq!(cert_pass_count,usize::from(services.len()>0));
  assert_eq!(world.cell(21),expected["success_claim"].as_i64());
  assert_eq!(world.cell(32),expected["episodes_completed"].as_i64());
  assert_eq!(world.cell(63),expected["final_active_middle"].as_i64());
  assert_eq!(world.used_cells(),expected["cells_used"].as_u64().unwrap() as usize);
  assert_eq!(peak,founder.len()+1+world.used_cells());
  assert_eq!(meta_reads,expected["meta_fetches"].as_u64().unwrap() as usize);
  assert_eq!(accumulators,expected["accumulator_checkpoints"].as_u64().unwrap() as usize);
  assert_eq!(actor_feedback_reads,sent_feedback);
  assert_eq!(world.process(1).unwrap().pc,expected["halt_pc"].as_u64().unwrap() as usize);
  assert!(world.process(1).unwrap().stopped);
  let external=world.events().iter().filter(|e|matches!(e.origin,Origin::External)).collect::<Vec<_>>();
  assert_eq!(external.len(),services.len()+1);
  assert!(matches!(&external[0].kind,EventKind::Loaded));
  assert!(external.iter().skip(1).all(|e|matches!(&e.kind,EventKind::Intervened(_))));
  if name=="unsafe_tail3_cert_reject" {
   assert_eq!(services.len(),0);
   assert_eq!(cert_pass_count,0);
   assert_eq!(world.cell(21),Some(0));
  }
  if name=="safe_bank_deceptive_feedback"{
   assert_eq!(world.cell(21),Some(1));
   assert!(services.iter().all(|ev|ev["truth"]==0));
  }
  results.push(json!({
   "arm":name,"paid_steps":steps,"credits_remaining":CREDIT-steps as u64,
   "source_nodes":founder.len(),"initial_bank":expected["bank"],
   "certificate_marks":marks,"certificate_passes":cert_pass_count,
   "task_services":services,"paid_meta_fetches":meta_reads,
   "paid_accumulator_checkpoints":accumulators,
   "paid_feedback_reads":actor_feedback_reads,"environment_feedback_writes":sent_feedback,
   "final_halt_pc":world.process(1).unwrap().pc,
   "native_original_pid_halted":true,"peak_occupied":peak,
   "postload_external_rule_or_source_writes":0
  }));
 }
 fs::write(dir.join("b56_f1d1k_g3q_c1_p1_l3_i0_native_certified_feedback.json"),
  serde_json::to_vec_pretty(&json!({
    "study":"B56-F1d1k-G3Q-C1-P1-L3-I0",
    "level":"NATIVE_ONE_PID_STRUCTURAL_CERTIFICATION_AND_HIDDEN_TASK_FEEDBACK_RELEARNING",
    "source_sha":std::env::var("GITHUB_SHA").unwrap_or_else(|_|"local".to_string()),
    "arms":results,
    "limits":"Investigator-written candidate bank; actor pays to validate coverage but did not search 64 rules or create novel opcodes. Deceptive feedback can still fool actor."
  })).unwrap()).unwrap();
}
