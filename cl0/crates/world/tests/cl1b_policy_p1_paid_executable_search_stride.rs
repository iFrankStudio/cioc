//! CL1B-POLICY-P1: original PID1 pays to mutate an executable
//! candidate-TRANSITION policy (Const r2 at PC46) based on real feedback,
//! rather than P0's editable task-output byte. Frozen 62-node source.
//! Causal contrast: two cyclic tasks, source editing enabled/disabled,
//! inverse-reward and all-zero sham. No algorithm invention is claimed.
use cl0_world::{EventKind,Intervention,Op,Origin,Process,World};
use serde_json::{json,Value};
use std::fs::{self,File};
use std::io::{BufWriter,Write};
const FROZEN:&str="../../../cl1b/studies/CL1B-POLICY-P1-frozen-62-node-feedback-conditioned-executable-search-stride-v1.json";
const KEYS:[i64;6]=[11,20,21,30,32,33];
const CREDIT:u64=800;
fn decode(v:&Value)->Op{
 let tag=v["kind"].as_str().unwrap();
 let raw=if tag=="Halt"{json!("Halt")}else{
  let mut fields=serde_json::Map::new();
  for (k,val) in v.as_object().unwrap(){
   if k!="kind"{fields.insert(k.clone(),val.clone());}
  }
  let mut outer=serde_json::Map::new();
  outer.insert(tag.to_string(),Value::Object(fields));Value::Object(outer)
 };
 serde_json::from_value(raw).expect("frozen original CIL source invalid")
}
fn stride(world:&World)->i64{
 match &world.process(1).unwrap().code.nodes[46]{
  Op::Const{dst:2,value,next:47}=>*value,
  other=>panic!("source POLICY_STRIDE must be original Const r2 at PC46: {other:?}")
 }
}
fn execute(source:&[Value],case:&Value,folder:&std::path::Path)->Value{
 let name=case["name"].as_str().unwrap();
 let a=case["goals"].as_array().unwrap();
 let goals=[a[0].as_i64().unwrap(),a[1].as_i64().unwrap(),
    a[2].as_i64().unwrap(),a[3].as_i64().unwrap()];
 let feedback=case["feedback"].as_str().unwrap();
 let suppressed=case["suppressed"].as_bool().unwrap();
 assert!(goals.iter().all(|x|(0_i64..=2_i64).contains(x)));
 let immutable=source.iter().map(decode).collect::<Vec<_>>();
 let mut expected_current_source=immutable.clone();
 let mut world=World::with_strict_capacity(16,180,300);
 if suppressed{world=world.with_native_code_edits_disabled();}
 world.load(Process::new(1,immutable.clone(),CREDIT)).unwrap();
 assert_eq!(world.occupied_units(),63);
 let mut out=BufWriter::new(File::create(folder.join(format!(
  "cl1b_policy_p1_original_native_{name}.jsonl"))).unwrap());
 let mut charged=0_usize;
 let mut peak=world.occupied_units();
 let mut services=Vec::<Value>::new();
 let mut feedback_reads=Vec::<Value>::new();
 let mut patches=Vec::<Value>::new();
 let mut success_counts=Vec::<Value>::new();
 let mut failure_counts=Vec::<Value>::new();
 let mut attempt_resets=Vec::<Value>::new();
 let mut terminal_claims=Vec::<Value>::new();
 let mut delivered=Vec::<i64>::new();
 let mut last_feedback:Option<i64>=None;
 while world.step(){
  charged+=1;assert!(charged<=CREDIT as usize,"source exceeds privately funded compute");
  let event=world.events().last().unwrap().clone();
  assert!(matches!(event.origin,Origin::Internal(1)));
  let pc=event.pc;
  if pc==5{
   match &event.kind{
    EventKind::CellRead{key:20,value}=>{
     last_feedback=Some(*value);
     feedback_reads.push(json!({"step":charged,"value":value}));
    },
    other=>panic!("native feedback not an original actor-paid Read20: {other:?}")
   }
  }
  if pc==45{
   assert_eq!(last_feedback,Some(1),"executable policy-stride patch requires paid positive feedback");
   assert!(patches.is_empty(),"only one paid adaptation of executable search policy");
   let process=world.process(1).unwrap();
   assert_eq!(process.registers[1],46);
   let previous=match &expected_current_source[46]{
    Op::Const{dst:2,value,next:47}=>*value,
    other=>panic!("policy stride source invalid before patch: {other:?}")
   };
   let after=stride(&world);
   if suppressed{
    assert!(matches!(&event.kind,EventKind::NativeCodeEditSuppressed{opcode_tag:9}));
    assert_eq!(after,previous);
   }else{
    assert!(matches!(&event.kind,EventKind::Executed));
    assert_eq!(after,process.registers[3],"patched search stride must equal calibrated successful task value");
    expected_current_source[46]=Op::Const{dst:2,value:after,next:47};
   }
   patches.push(json!({"step":charged,"source_node":46,
       "before":previous,"after":after,"edit_suppressed":suppressed}));
  }
  if pc==34{
   match &event.kind{
    EventKind::CellWritten{key:32,value}=>success_counts.push(json!({"step":charged,"value":value})),
    other=>panic!("missing paid task-progress cell32 write: {other:?}")
   }
  }
  if pc==12{
   match &event.kind{
    EventKind::CellWritten{key:33,value}=>failure_counts.push(json!({"step":charged,"value":value})),
    other=>panic!("missing paid task-failure counter write: {other:?}")
   }
  }
  if pc==37{
   match &event.kind{
    EventKind::CellWritten{key:33,value:0}=>attempt_resets.push(json!({"step":charged,"value":0})),
    other=>panic!("missing paid attempt reset: {other:?}")
   }
  }
  if [27,60].contains(&pc){
   match &event.kind{
    EventKind::CellWritten{key:21,value}=>{
     assert_eq!(*value,if pc==27{0}else{1});
     terminal_claims.push(json!({"step":charged,"value":value}));
    },
    other=>panic!("missing original PID paid terminal claim write: {other:?}")
   }
  }
  let p=world.process(1).unwrap();
  assert_eq!(p.code.nodes,expected_current_source,
    "only actor-paid patch of executable POLICY_STRIDE PC46 may modify original code");
  assert!(world.process(2).is_none(),"no process replacement or child PID");
  assert_eq!(world.total_compute_inventory(),Some(CREDIT-charged as u64));
  assert!(world.used_cells()<=16&&world.occupied_units()<=300);
  peak=peak.max(world.occupied_units());
  serde_json::to_writer(&mut out,&json!(["cil",charged,pc,p.pc,
     p.registers,p.credits,KEYS.map(|key|world.cell(key)),
     world.occupied_units(),stride(&world)])).unwrap();
  out.write_all(b"\n").unwrap();
  if pc==3{
   let output=match &event.kind{
    EventKind::CellWritten{key:11,value}=>*value,
    other=>panic!("not a genuine original PID-paid Write11 service: {other:?}")
   };
   assert_eq!(output,p.registers[3],
      "task output must come from actor internal guess, NOT edited policy stride instruction");
   let episode=world.cell(32).unwrap_or(0) as usize;
   assert!(episode<4);
   let truth=i64::from(output==goals[episode]);
   let bit=match feedback{
    "truth"=>truth,"inverted"=>1-truth,
    "forced0"=>0,
    other=>panic!("unregistered reward regime {other}")
   };
   services.push(json!({"step":charged,"episode":episode,
      "output":output,"truth":truth,"feedback":bit}));
   delivered.push(bit);
   world.intervene(Intervention::SetCell{key:20,value:bit}).unwrap();
   let external=world.events().last().unwrap();
   assert!(matches!(external.origin,Origin::External));
   assert!(matches!(&external.kind,EventKind::Intervened(_)));
   peak=peak.max(world.occupied_units());
   serde_json::to_writer(&mut out,&json!(["feedback",charged,episode,
     goals[episode],output,truth,bit,KEYS.map(|key|world.cell(key)),
     world.occupied_units(),stride(&world)])).unwrap();
   out.write_all(b"\n").unwrap();
  }
 }
 out.flush().unwrap();
 assert!(world.process(1).unwrap().stopped);
 assert_eq!(delivered,feedback_reads.iter()
    .map(|x|x["value"].as_i64().unwrap()).collect::<Vec<_>>());
 let expected=&case["expected"];
 assert_eq!(charged,expected["paid_steps"].as_u64().unwrap() as usize);
 assert_eq!(world.process(1).unwrap().pc,expected["halt_pc"].as_u64().unwrap() as usize);
 assert_eq!(CREDIT-charged as u64,expected["private_credits_left"].as_u64().unwrap());
 assert_eq!(stride(&world),expected["executable_policy_stride"].as_i64().unwrap());
 assert_eq!(json!(world.cell(21)),expected["actor_claimed_success"]);
 assert_eq!(json!(world.cell(32)),expected["actor_completed_stages"]);
 assert_eq!(json!(patches),expected["original_paid_source_policy_patches"]);
 assert_eq!(json!(services),expected["original_paid_task_services"]);
 assert_eq!(json!(feedback_reads),expected["original_paid_feedback_reads"]);
 assert_eq!(json!(success_counts),expected["original_paid_success_count_writes"]);
 assert_eq!(json!(failure_counts),expected["original_paid_fail_count_writes"]);
 assert_eq!(json!(attempt_resets),expected["original_paid_fail_reset_writes"]);
 assert_eq!(json!(terminal_claims),expected["original_paid_terminal_claims"]);
 assert_eq!(world.used_cells(),expected["occupied_data_cells"].as_u64().unwrap() as usize);
 assert_eq!(peak,expected["peak_occupied"].as_u64().unwrap() as usize);
 let external=world.events().iter().filter(|v|matches!(v.origin,Origin::External)).collect::<Vec<_>>();
 assert_eq!(external.len(),services.len()+1);
 assert!(matches!(&external[0].kind,EventKind::Loaded));
 assert!(external.iter().skip(1).all(|v|matches!(&v.kind,EventKind::Intervened(_))));
 json!({"arm":name,"goals":goals,"feedback_regime":feedback,
     "source_edit_suppressed":suppressed,
     "paid_original_steps":charged,"final_original_executable_stride":stride(&world),
     "original_paid_policy_source_edits":patches,
     "original_paid_task_services":services,
     "original_paid_feedback_reads":feedback_reads,
     "actor_claimed_success":world.cell(21),
     "actor_completed_stages":world.cell(32),
     "original_PID1_halted":true,"peak_occupied":peak,
     "postload_host_source_edits":0})
}
#[test]
fn policy_p1_actor_feedback_pays_to_edit_executable_candidate_search_stride(){
 let root=std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
 let frozen:Value=serde_json::from_slice(&fs::read(root.join(FROZEN)).unwrap()).unwrap();
 let source=frozen["original_native_CIL_source"].as_array().unwrap();
 let cases=frozen["pre_registered_native_cases"].as_array().unwrap();
 assert_eq!(source.len(),62);
 assert_eq!(cases.len(),6);
 assert_eq!(source[46],json!({"kind":"Const","dst":2,"value":1,"next":47}));
 assert_eq!(source[45],json!({"kind":"PatchConst","node":1,"value":3,"next":46}));
 let dir=root.join("../../cl1b-ci-evidence");
 fs::create_dir_all(&dir).unwrap();
 let mut observed=Vec::<Value>::new();
 for case in cases{
  observed.push(execute(source,case,&dir));
 }
 assert_eq!(observed[0]["actor_claimed_success"],1);
 assert_eq!(observed[1]["actor_claimed_success"],1);
 assert_eq!(observed[1]["final_original_executable_stride"],2);
 assert_eq!(observed[2]["final_original_executable_stride"],1);
 let active=observed[1]["paid_original_steps"].as_u64().unwrap();
 let disabled=observed[2]["paid_original_steps"].as_u64().unwrap();
 assert_eq!((active,disabled),(149,196));
 assert_eq!(disabled-active,47);
 assert_eq!(observed[0]["paid_original_steps"],observed[3]["paid_original_steps"]);
 assert_eq!(observed[4]["actor_claimed_success"],1);
 assert!(observed[4]["original_paid_task_services"].as_array().unwrap()
    .iter().filter(|x|x["truth"]==1).count()<4,
    "inverted feedback demonstrates success claim does not equal truthful task success");
 assert_eq!(observed[5]["actor_claimed_success"],0);
 let total=observed.iter().map(|x|x["paid_original_steps"].as_u64().unwrap()).sum::<u64>();
 assert_eq!(total,856);
 fs::write(dir.join("cl1b_policy_p1_native_paid_executable_stride_adaptation.json"),
  serde_json::to_vec_pretty(&json!({
   "study":"CL1B-POLICY-P1",
   "level":"NATIVE_ORIGINAL_PID_FEEDBACK_CONDITIONED_EXECUTABLE_SEARCH_STRIDE",
   "source_sha":std::env::var("GITHUB_SHA").unwrap_or_else(|_|"local".into()),
   "worlds":observed,
   "counts":{"original_native_worlds":6,"true_feedback_worlds":4,
       "private_original_paid_steps":total,
       "stride2_source_enabled_saved_charged_steps":47,
       "stride2_source_enabled_serviced_probes":5,
       "stride2_edit_disabled_serviced_probes":7},
   "limits":"Researcher designed source adaptation of transition stride, not invention of an algorithm; fixed finite cyclic tasks and a one-step paid causal source-edit mechanism."
  })).unwrap()).unwrap();
}
