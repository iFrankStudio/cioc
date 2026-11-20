//! CL1B-POLICY-P2: original PID1 pays to learn source PC46 candidate stride
//! from TWO actual task successes rather than copying the absolute answer.
//! Source-80 and 162 original paired goal worlds, six adversity controls.
//! This is a researcher-written relative-delta heuristic: finite negative
//! transfer remains a first-class preregistered possible result.
use cl0_world::{EventKind,Intervention,Op,Origin,Process,World};
use serde_json::{json,Value};
use std::fs::{self,File};
use std::io::{BufWriter,Write};

const FROZEN:&str="../../../cl1b/studies/CL1B-POLICY-P2-frozen-80-node-actor-paid-relative-delta-search-policy-v1.json";
const KEYS:[i64;7]=[11,20,21,30,32,33,34];
const PRIVATE_CREDITS:u64=1600;
const P3:&str="../../../cl1b/studies/CL1B-POLICY-P3-frozen-eight-stage-regime-transfer-v1.json";

fn op(v:&Value)->Op{
 let tag=v["kind"].as_str().unwrap();
 let raw=if tag=="Halt"{json!("Halt")}else{
  let mut fields=serde_json::Map::new();
  for (key,value) in v.as_object().unwrap(){
   if key!="kind"{fields.insert(key.clone(),value.clone());}
  }
  let mut outer=serde_json::Map::new();
  outer.insert(tag.to_string(),Value::Object(fields));Value::Object(outer)
 };
 serde_json::from_value(raw).expect("invalid prospectively frozen CIL source")
}
fn native_stride(world:&World)->i64{
 match &world.process(1).unwrap().code.nodes[46]{
  Op::Const{dst:2,value,next:47}=>*value,
  other=>panic!("runtime source PC46 not executable stride Const: {other:?}")
 }
}
fn execute(source:&[Value],arm:&Value,dir:&std::path::Path)->Value{
 let name=arm["name"].as_str().unwrap();
 let goals_json=arm["goals"].as_array().unwrap();
 let goals=goals_json.iter().map(|v|v.as_i64().unwrap()).collect::<Vec<_>>();
 assert_eq!(goals.len(),8);
 assert!(goals.iter().all(|x|(0_i64..=2).contains(x)));
 let feedback_regime=arm["feedback"].as_str().unwrap();
 let suppress=arm["suppressed"].as_bool().unwrap();
 let initial=source.iter().map(op).collect::<Vec<_>>();
 let mut current_source=initial.clone();
 let mut world=World::with_strict_capacity(16,180,300);
 if suppress{world=world.with_native_code_edits_disabled();}
 world.load(Process::new(1,initial,PRIVATE_CREDITS)).unwrap();
 assert_eq!(world.occupied_units(),81);
 let mut trace=BufWriter::new(File::create(dir.join(format!(
   "cl1b_policy_p3_original_{name}.jsonl"))).unwrap());
 let mut paid=0_usize;let mut peak=world.occupied_units();
 let mut services=Vec::<Value>::new();
 let mut reads=Vec::<Value>::new();
 let mut edits=Vec::<Value>::new();
 let mut successes=Vec::<Value>::new();
 let mut failures=Vec::<Value>::new();
 let mut resets=Vec::<Value>::new();
 let mut claims=Vec::<Value>::new();
 let mut stored_prior=Vec::<Value>::new();
 let mut read_prior=Vec::<Value>::new();
 let mut delivered=Vec::<i64>::new();
 let mut recent_paid_feedback:Option<i64>=None;
 while world.step(){
  paid+=1;assert!(paid<=PRIVATE_CREDITS as usize);
  let ev=world.events().last().unwrap().clone();
  assert!(matches!(ev.origin,Origin::Internal(1)),
     "paid source edits and task service must originate from original PID1");
  if ev.pc==5{
   match &ev.kind{
    EventKind::CellRead{key:20,value}=>{
      recent_paid_feedback=Some(*value);
      reads.push(json!({"step":paid,"value":value}));
    },
    other=>panic!("missing paid original Read20: {other:?}")
   }
  }
  if ev.pc==66{
   match &ev.kind{
    EventKind::CellWritten{key:34,value}=>{
      assert!(stored_prior.is_empty());
      stored_prior.push(json!({"step":paid,"value":value}));
    },
    other=>panic!("stage0 did not pay to store previous goal34: {other:?}")
   }
  }
  if ev.pc==69{
   match &ev.kind{
    EventKind::CellRead{key:34,value}=>{
      assert_eq!(stored_prior.len(),1);
      assert!(read_prior.is_empty());
      read_prior.push(json!({"step":paid,"value":value}));
      assert_eq!(stored_prior[0]["value"],json!(value));
    },
    other=>panic!("relative stride has not paid to read prior-goal34: {other:?}")
   }
  }
  if ev.pc==45{
   assert_eq!(recent_paid_feedback,Some(1),
     "policy meta adaptation requires actual paid positive feedback");
   assert_eq!(stored_prior.len(),1);
   assert_eq!(read_prior.len(),1);
   assert!(edits.is_empty(),"bounded original one-time meta-policy patch");
   let actor=world.process(1).unwrap();
   assert_eq!(actor.registers[1],46);
   let requested=actor.registers[0];
   assert!((0..=2).contains(&requested));
   let prior=match &current_source[46]{
    Op::Const{dst:2,value,next:47}=>*value,
    other=>panic!("lost frozen original executable policy source: {other:?}")
   };
   let actual=native_stride(&world);
   if suppress{
    assert!(matches!(&ev.kind,EventKind::NativeCodeEditSuppressed{opcode_tag:9}));
    assert_eq!(actual,prior);
   }else{
    assert!(matches!(&ev.kind,EventKind::Executed));
    assert_eq!(actual,requested);
    current_source[46]=Op::Const{dst:2,value:actual,next:47};
   }
   edits.push(json!({"step":paid,"source_node":46,
     "before":prior,"after":actual,"requested":requested,"edit_suppressed":suppress}));
  }
  if ev.pc==34{
   match &ev.kind{
    EventKind::CellWritten{key:32,value}=>{
      successes.push(json!({"step":paid,"value":value}));
    },
    other=>panic!("missing source-paid stage success: {other:?}")
   }
  }
  if ev.pc==12{
   match &ev.kind{
    EventKind::CellWritten{key:33,value}=>{
      failures.push(json!({"step":paid,"value":value}));
    },
    other=>panic!("missing source-paid failure count: {other:?}")
   }
  }
  if ev.pc==37{
   assert!(matches!(&ev.kind,EventKind::CellWritten{key:33,value:0}));
   resets.push(json!({"step":paid,"value":0}));
  }
  if [27,60].contains(&ev.pc){
   match &ev.kind{
    EventKind::CellWritten{key:21,value}=>{
      assert_eq!(*value,if ev.pc==27{0}else{1});
      claims.push(json!({"step":paid,"value":value}));
    },
    other=>panic!("missing actor paid true termination claim: {other:?}")
   }
  }
  let p=world.process(1).unwrap();
  assert_eq!(p.code.nodes,current_source,
    "unpaid or host source mutation detected beyond PC46");
  assert!(world.process(2).is_none());
  assert_eq!(world.total_compute_inventory(),Some(PRIVATE_CREDITS-paid as u64));
  assert!(world.used_cells()<=16&&world.occupied_units()<=300);
  peak=peak.max(world.occupied_units());
  serde_json::to_writer(&mut trace,&json!(["cil",paid,ev.pc,
    p.pc,p.registers,p.credits,KEYS.map(|key|world.cell(key)),
    world.occupied_units(),native_stride(&world)])).unwrap();
  trace.write_all(b"\n").unwrap();
  if ev.pc==3{
   let output=match &ev.kind{
    EventKind::CellWritten{key:11,value}=>*value,
    other=>panic!("task service was not source-paid original Write11: {other:?}")
   };
   assert_eq!(output,p.registers[3],"original task guess not from actor register");
   let episode=world.cell(32).unwrap_or(0) as usize;assert!(episode<8);
   let truth=i64::from(output==goals[episode]);
   let bit=match feedback_regime{
      "truth"=>truth,"inverted"=>1-truth,"forced0"=>0,
      other=>panic!("unexpected reward regime {other}")
   };
   services.push(json!({"step":paid,"episode":episode,
     "output":output,"truth":truth,"feedback":bit}));
   delivered.push(bit);
   world.intervene(Intervention::SetCell{key:20,value:bit}).unwrap();
   let outside=world.events().last().unwrap();
   assert!(matches!(outside.origin,Origin::External));
   assert!(matches!(&outside.kind,EventKind::Intervened(_)));
   peak=peak.max(world.occupied_units());
   serde_json::to_writer(&mut trace,&json!(["feedback",paid,episode,
      goals[episode],output,truth,bit,
      KEYS.map(|key|world.cell(key)),world.occupied_units(),
      native_stride(&world)])).unwrap();
   trace.write_all(b"\n").unwrap();
  }
 }
 trace.flush().unwrap();
 assert!(world.process(1).unwrap().stopped);
 assert_eq!(reads.len(),services.len());
 assert_eq!(delivered,reads.iter()
   .map(|x|x["value"].as_i64().unwrap()).collect::<Vec<_>>());
 let external=world.events().iter().filter(|e|matches!(e.origin,Origin::External)).collect::<Vec<_>>();
 assert_eq!(external.len(),services.len()+1);
 assert!(matches!(&external[0].kind,EventKind::Loaded));
 assert!(external.iter().skip(1).all(|x|matches!(&x.kind,EventKind::Intervened(_))));
 json!({"arm":name,"goals":goals,"feedback_regime":feedback_regime,
   "source_edit_suppressed":suppress,
   "original_paid_steps":paid,"final_executable_stride":native_stride(&world),
   "original_paid_policy_edits":edits,"original_paid_task_services":services,
   "original_paid_feedback_reads":reads,
   "original_paid_previous_goal_writes":stored_prior,
   "original_paid_previous_goal_reads":read_prior,
   "actor_claim":world.cell(21),"actor_completed_stages":world.cell(32),
   "original_PID1_halted":true,"peak_occupied":peak,
   "postload_host_source_edits":0})
}


#[test]
fn policy_p3_eight_stage_native_regime_transfer_and_controls(){
 let root=std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
 let f:Value=serde_json::from_slice(&fs::read(root.join(FROZEN)).unwrap()).unwrap();
 let p3:Value=serde_json::from_slice(&fs::read(root.join(P3)).unwrap()).unwrap();
 assert_eq!(p3["status"],"PRE_NATIVE_FROZEN_EIGHT_STAGE_REGIME_TRANSFER_PROTOCOL");
 let mut source=f["original_native_CIL_source"].as_array().unwrap().clone();
 assert_eq!(source.len(),80);
 assert_eq!(source[38],json!({"kind":"Const","dst":2,"value":4,"next":39}));
 source[38]["value"]=json!(8);
 assert_eq!(source[45],json!({"kind":"PatchConst","node":1,"value":0,"next":46}));
 assert_eq!(source[46],json!({"kind":"Const","dst":2,"value":1,"next":47}));
 let arms=p3["arms"].as_array().unwrap();
 let controls=p3["controls"].as_array().unwrap();
 assert_eq!(arms.len(),54);
 assert_eq!(controls.len(),2);
 let dir=root.join("../../cl1b-ci-evidence");
 fs::create_dir_all(&dir).unwrap();
 let mut pairs=Vec::<Value>::new();
 let mut sums=std::collections::BTreeMap::<String,(i64,i64,usize)>::new();
 let mut total_steps=0_u64;
 for two in arms.chunks_exact(2){
  let first=&two[0];let second=&two[1];
  assert_eq!(first["goals"],second["goals"]);
  assert_eq!(first["regime"],second["regime"]);
  assert_eq!(first["suppressed"],false);
  assert_eq!(second["suppressed"],true);
  let on=execute(&source,first,&dir);
  let off=execute(&source,second,&dir);
  for x in [&on,&off]{
   assert_eq!(x["actor_claim"],1);
   assert_eq!(x["actor_completed_stages"],8);
   assert_eq!(x["original_paid_policy_edits"].as_array().unwrap().len(),1);
   assert_eq!(x["original_paid_task_services"].as_array().unwrap().iter().filter(|e|e["truth"]==1).count(),8);
  }
  let on_cost=on["original_paid_steps"].as_i64().unwrap();
  let off_cost=off["original_paid_steps"].as_i64().unwrap();
  total_steps+=(on_cost+off_cost) as u64;
  let probe_diff=off["original_paid_task_services"].as_array().unwrap().len() as i64
    -on["original_paid_task_services"].as_array().unwrap().len() as i64;
  let regime=first["regime"].as_str().unwrap().to_string();
  let slot=sums.entry(regime.clone()).or_insert((0,0,0));
  slot.0+=off_cost-on_cost;slot.1+=probe_diff;slot.2+=1;
  pairs.push(json!({"regime":regime,"goals":first["goals"],
    "source_patch_on":on,"source_patch_off":off,
    "native_paid_instruction_saving":off_cost-on_cost,
    "native_paid_task_probe_saving":probe_diff}));
 }
 let mut controls_out=Vec::<Value>::new();
 for arm in controls{
  let result=execute(&source,arm,&dir);
  total_steps+=result["original_paid_steps"].as_u64().unwrap();
  controls_out.push(result);
 }
 assert_eq!(controls_out[0]["actor_claim"],1);
 assert!(controls_out[0]["original_paid_task_services"].as_array().unwrap()
  .iter().filter(|x|x["truth"]==1).count()<8);
 assert_eq!(controls_out[1]["actor_claim"],0);
 let regimes=sums.iter().map(|(key,(s,p,n))|
  (key.clone(),json!({"pairs":n,"net_paid_instruction_saving":s,"net_task_probe_saving":p}))
 ).collect::<serde_json::Map<String,Value>>();
 let evidence=json!({"study":"CL1B-POLICY-P3","source_sha":std::env::var("GITHUB_SHA").unwrap_or_else(|_|"local".into()),
  "level":"NATIVE_ORIGINAL_PID_80_NODE_EIGHT_STAGE_56_WORLD_CAUSAL_TRANSFER",
  "preload_horizon_node38":8,"private_credits":PRIVATE_CREDITS,
  "pairs":pairs,"controls":controls_out,
  "counts":{"original_worlds":56,"paired_worlds":54,"causal_controls":2,
    "total_original_paid_instructions":total_steps,"regimes":regimes},
  "nonclaims":"Only a researcher-authored one-time relative-stride policy; no revalidation, re-patching, rollback or novel policy invention."});
 fs::write(dir.join("cl1b_policy_p3_native_eight_stage_regime_transfer.json"),
  serde_json::to_vec_pretty(&evidence).unwrap()).unwrap();
}
