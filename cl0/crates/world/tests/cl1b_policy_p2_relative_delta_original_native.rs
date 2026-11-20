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
const H81:&str="../../../cl1b/studies/CL1B-POLICY-P1-H81-frozen-162-original-world-complete-transfer-audit-v1.json";
const KEYS:[i64;7]=[11,20,21,30,32,33,34];
const PRIVATE_CREDITS:u64=800;

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
 let goals=[goals_json[0].as_i64().unwrap(),goals_json[1].as_i64().unwrap(),
            goals_json[2].as_i64().unwrap(),goals_json[3].as_i64().unwrap()];
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
   "cl1b_policy_p2_original_{name}.jsonl"))).unwrap());
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
   let episode=world.cell(32).unwrap_or(0) as usize;assert!(episode<4);
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
 let expected=&arm["expected"];
 assert_eq!(paid,expected["paid_steps"].as_u64().unwrap() as usize);
 assert_eq!(world.process(1).unwrap().pc,expected["halt_pc"].as_u64().unwrap() as usize);
 assert_eq!(PRIVATE_CREDITS-paid as u64,expected["private_credits_left"].as_u64().unwrap());
 assert_eq!(native_stride(&world),expected["final_executable_stride"].as_i64().unwrap());
 assert_eq!(json!(world.cell(21)),expected["actor_claim"]);
 assert_eq!(json!(world.cell(32)),expected["actor_completed_stages"]);
 assert_eq!(json!(services),expected["original_paid_task_services"]);
 assert_eq!(json!(reads),expected["original_paid_feedback_reads"]);
 assert_eq!(json!(edits),expected["original_paid_policy_source_edits"]);
 assert_eq!(json!(successes),expected["original_paid_success_writes"]);
 assert_eq!(json!(failures),expected["original_paid_failure_writes"]);
 assert_eq!(json!(resets),expected["original_paid_failure_resets"]);
 assert_eq!(json!(claims),expected["original_paid_terminal_claims"]);
 assert_eq!(json!(stored_prior),expected["original_paid_previous_goal_writes"]);
 assert_eq!(json!(read_prior),expected["original_paid_previous_goal_reads"]);
 assert_eq!(world.used_cells(),expected["occupied_data_cells"].as_u64().unwrap() as usize);
 assert_eq!(peak,expected["peak_occupied"].as_u64().unwrap() as usize);
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
fn policy_p2_full_81_goal_cube_original_native_relative_policy_calibration(){
 let root=std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
 let f:Value=serde_json::from_slice(&fs::read(root.join(FROZEN)).unwrap()).unwrap();
 let source=f["original_native_CIL_source"].as_array().unwrap();
 let cases=f["all162_pre_registered_original_PID_worlds"].as_array().unwrap();
 let controls=f["adversarial_controls"].as_array().unwrap();
 assert_eq!(source.len(),80);assert_eq!(cases.len(),162);assert_eq!(controls.len(),6);
 assert_eq!(source[45],json!({"kind":"PatchConst","node":1,"value":0,"next":46}));
 assert_eq!(source[46],json!({"kind":"Const","dst":2,"value":1,"next":47}));
 assert_eq!(source[66],json!({"kind":"Write","address":2,"value":3,"next":67}));
 assert_eq!(source[69],json!({"kind":"Read","dst":0,"address":2,"next":70}));
 let earlier:Value=serde_json::from_slice(&fs::read(root.join(H81)).unwrap()).unwrap();
 let earlier_cases=earlier["all162_pre_registered_native_cases"].as_array().unwrap();
 assert_eq!(earlier_cases.len(),162);
 let dir=root.join("../../cl1b-ci-evidence");
 fs::create_dir_all(&dir).unwrap();
 let mut all=Vec::<Value>::new();
 let mut helps=0_usize;let mut harms=0_usize;let mut ties=0_usize;
 let mut total_saved=0_i64;let mut probe_saved=0_i64;
 let mut total_steps=0_u64;let mut relative_vs_P1=0_i64;
 let mut cyclic=0_usize;
 for idx in 0..81{
  let enabled=&cases[2*idx];
  let disabled=&cases[2*idx+1];
  assert_eq!(enabled["goals"],disabled["goals"]);
  assert_eq!(enabled["suppressed"],false);
  assert_eq!(disabled["suppressed"],true);
  let on=execute(source,enabled,&dir);
  let off=execute(source,disabled,&dir);
  for actual in [&on,&off]{
   assert_eq!(actual["actor_claim"],1);
   assert_eq!(actual["actor_completed_stages"],4);
   assert_eq!(actual["original_paid_task_services"].as_array().unwrap()
      .iter().filter(|s|s["truth"]==1).count(),4);
  }
  let on_steps=on["original_paid_steps"].as_u64().unwrap();
  let off_steps=off["original_paid_steps"].as_u64().unwrap();
  let diff=off_steps as i64-on_steps as i64;
  match diff.cmp(&0){
   std::cmp::Ordering::Greater=>helps+=1,
   std::cmp::Ordering::Less=>harms+=1,
   std::cmp::Ordering::Equal=>ties+=1
  }
  total_steps+=on_steps+off_steps;
  total_saved+=diff;
  let attempts_on=on["original_paid_task_services"].as_array().unwrap().len();
  let attempts_off=off["original_paid_task_services"].as_array().unwrap().len();
  probe_saved+=attempts_off as i64-attempts_on as i64;
  let p1_on=&earlier_cases[2*idx]["expected"];
  relative_vs_P1+=p1_on["paid_steps"].as_i64().unwrap()-on_steps as i64;
  let g=enabled["goals"].as_array().unwrap();
  let nums=g.iter().map(|v|v.as_i64().unwrap()).collect::<Vec<_>>();
  if nums[2]==(2*nums[1]-nums[0]+3)%3 &&
      nums[3]==(3*nums[1]-2*nums[0]+6)%3{cyclic+=1;}
  all.push(json!({"goals":enabled["goals"],"source_patch_on":on,
    "source_patch_off":off,"native_paid_instruction_saving":diff,
    "native_paid_task_probe_saving":attempts_off as i64-attempts_on as i64}));
 }
 assert_eq!(cyclic,9);
 assert_eq!((helps,harms,ties),(27,27,27));
 assert_eq!(total_saved,-54);
 assert_eq!(probe_saved,0);
 assert_eq!(total_steps,39636);
 assert_eq!(relative_vs_P1,-1917);
 let mut causal=Vec::<Value>::new();
 for control in controls{causal.push(execute(source,control,&dir));}
 assert_eq!((causal[0]["original_paid_steps"].as_u64().unwrap(),
             causal[1]["original_paid_steps"].as_u64().unwrap()),(174,221));
 assert_eq!((causal[2]["original_paid_steps"].as_u64().unwrap(),
             causal[3]["original_paid_steps"].as_u64().unwrap()),(197,242));
 assert_eq!(causal[4]["actor_claim"],1);
 assert!(causal[4]["original_paid_task_services"].as_array().unwrap()
   .iter().filter(|v|v["truth"]==1).count()<4);
 assert_eq!(causal[5]["actor_claim"],0);
 fs::write(dir.join("cl1b_policy_p2_native_relative_policy_complete_cube.json"),
  serde_json::to_vec_pretty(&json!({
    "study":"CL1B-POLICY-P2",
    "level":"NATIVE_ORIGINAL_PID_80_NODE_RELATIVE_DELTA_PAID_STRIDE_COMPLETE_81_PAIR_CUBE",
    "source_sha":std::env::var("GITHUB_SHA").unwrap_or_else(|_|"local".into()),
    "all81_paired_native_worlds":all,
    "six_adversarial_native_controls":causal,
    "counts":{"goal_sequences":81,"paired_original_native_worlds":162,
      "causal_native_controls":6,"positive_patch_transfer":helps,
      "negative_patch_transfer":harms,"neutral_patch_transfer":ties,
      "net_paid_instruction_savings":total_saved,
      "net_task_probe_savings":probe_saved,
      "all_pair_worlds_truthfully_completed_four_tasks":162,
      "total_pair_native_paid_steps":total_steps,
      "absolute_paid_difference_vs_P1_earlier_source_model":relative_vs_P1},
    "limitations":"Researcher-authored difference normalization and policy update, finite task alphabet; complete independence of future goals implies no guaranteed positive transfer or self-invented algorithm."
  })).unwrap()).unwrap();
}
