//! CL1B-POLICY-P0: orthogonal minimal native mechanism proof.
//! An original single PID1 pays for actual Write11 task services, receives
//! only external feedback20, reads that feedback with paid Read, and then
//! pays PatchConst on its OWN executable task-output Const at source PC1.
//! This is researcher-authored finite candidate cycling, not invented code.
use cl0_world::{EventKind,Intervention,Op,Origin,Process,World};
use serde_json::{json,Value};
use std::fs::{self,File};
use std::io::{BufWriter,Write};
const FROZEN:&str="../../../cl1b/studies/CL1B-POLICY-P0-frozen-43-node-task-feedback-paid-executable-patch-v1.json";
const KEYS:[i64;5]=[11,20,21,32,33];
const INITIAL_CREDITS:u64=800;
fn decode(v:&Value)->Op{
 let kind=v["kind"].as_str().unwrap();
 let tagged=if kind=="Halt"{json!("Halt")}else{
  let mut data=serde_json::Map::new();
  for (k,value) in v.as_object().unwrap(){
   if k!="kind"{data.insert(k.clone(),value.clone());}
  }
  let mut wrap=serde_json::Map::new();
  wrap.insert(kind.to_string(),Value::Object(data));
  Value::Object(wrap)
 };
 serde_json::from_value(tagged).expect("invalid original source CIL")
}
fn source_guess(world:&World)->i64{
 match &world.process(1).unwrap().code.nodes[1]{
  Op::Const{dst:3,value,next:2}=>*value,
  op=>panic!("task policy executable source Const1 unauthorized mutation: {op:?}")
 }
}
fn run(source:&[Value],name:&str,goals:[i64;4],feedback_regime:&str,
   disabled:bool,expected:&Value,dir:&std::path::Path)->Value{
 assert!(goals.iter().all(|x|(0_i64..=2_i64).contains(x)));
 let original=source.iter().map(decode).collect::<Vec<_>>();
 let mut expected_code=original.clone();
 let mut world=World::with_strict_capacity(16,180,300);
 if disabled{world=world.with_native_code_edits_disabled();}
 world.load(Process::new(1,original.clone(),INITIAL_CREDITS)).unwrap();
 assert_eq!(world.occupied_units(),44);
 let mut stream=BufWriter::new(File::create(dir.join(format!(
    "cl1b_policy_p0_native_original_{name}.jsonl"))).unwrap());
 let mut paid=0_usize;
 let mut peak=world.occupied_units();
 let mut edits=Vec::<Value>::new();
 let mut services=Vec::<Value>::new();
 let mut feedback_reads=Vec::<Value>::new();
 let mut success_writes=Vec::<Value>::new();
 let mut attempt_writes=Vec::<Value>::new();
 let mut terminal_writes=Vec::<Value>::new();
 let mut last_paid_feedback:Option<i64>=None;
 let mut feedback_delivered=Vec::<i64>::new();
 while world.step(){
  paid+=1;
  assert!(paid<=INITIAL_CREDITS as usize,"private paid instruction cap");
  let ev=world.events().last().unwrap().clone();
  assert!(matches!(ev.origin,Origin::Internal(1)));
  if ev.pc==4{
   match &ev.kind{
    EventKind::CellRead{key:20,value}=>{
     last_paid_feedback=Some(*value);
     feedback_reads.push(json!({"step":paid,"value":value}));
    }
    other=>panic!("actual original actor failed paid Read20: {other:?}")
   }
  }
  if ev.pc==22{
   assert!(edits.len()<16);
   assert_eq!(last_paid_feedback,Some(0),
      "source edit requires recent original PID-paid Read20 of task failure");
   let proc=world.process(1).unwrap();
   assert_eq!(proc.registers[1],1);
   let before=match &expected_code[1]{
    Op::Const{dst:3,value,next:2}=>*value,
    _=>panic!("existing source has lost registered executable Const at PC1")
   };
   let after=source_guess(&world);
   assert!((0..=2).contains(&after));
   if disabled{
    assert!(matches!(&ev.kind,EventKind::NativeCodeEditSuppressed{opcode_tag:9}));
    assert_eq!(after,before);
   }else{
    assert!(matches!(&ev.kind,EventKind::Executed));
    assert_ne!(after,before,"original paid source mutation must change value");
    expected_code[1]=Op::Const{dst:3,value:after,next:2};
   }
   edits.push(json!({"step":paid,"pc":22,"node":1,
     "before":before,"after":after,"edit_suppressed":disabled}));
  }
  if ev.pc==11{
   match &ev.kind{
    EventKind::CellWritten{key:33,value}=>{
     attempt_writes.push(json!({"step":paid,"value":value}));
    }
    other=>panic!("missing paid failure-counter update: {other:?}")
   }
  }
  if ev.pc==32{
   match &ev.kind{
    EventKind::CellWritten{key:32,value}=>{
     assert!((1..=4).contains(value));
     success_writes.push(json!({"step":paid,"value":value}));
    }
    other=>panic!("missing native task progress mark: {other:?}")
   }
  }
  if [25,41].contains(&ev.pc){
   match &ev.kind{
    EventKind::CellWritten{key:21,value}=>{
     assert_eq!(*value,if ev.pc==25{0}else{1});
     terminal_writes.push(json!({"step":paid,"value":value}));
    }
    other=>panic!("missing original PID task termination mark: {other:?}")
   }
  }
  let proc=world.process(1).unwrap();
  assert_eq!(proc.code.nodes,expected_code,
   "no host injected or unpaid executable source edit");
  assert!(world.process(2).is_none(),"no second process or actor replacement");
  assert_eq!(world.total_compute_inventory(),Some(INITIAL_CREDITS-paid as u64));
  assert!(world.used_cells()<=16&&world.occupied_units()<=300);
  peak=peak.max(world.occupied_units());
  let row=json!(["cil",paid,ev.pc,proc.pc,proc.registers,proc.credits,
    KEYS.map(|key|world.cell(key)),world.occupied_units(),source_guess(&world)]);
  serde_json::to_writer(&mut stream,&row).unwrap();
  stream.write_all(b"\n").unwrap();
  if ev.pc==2{
   let output=match &ev.kind{
    EventKind::CellWritten{key:11,value}=>*value,
    other=>panic!("no actor-paid genuine Write11 task service: {other:?}")
   };
   assert_eq!(output,source_guess(&world),
     "native task output not directly controlled by original actor's executable policy byte");
   let ep=world.cell(32).unwrap_or(0) as usize;
   assert!(ep<4);
   let goal=goals[ep];
   let truth=i64::from(output==goal);
   let fb=match feedback_regime{
    "truth"=>truth,
    "inverted"=>1-truth,
    "forced0"=>0,
    other=>panic!("unregistered hidden feedback regime: {other}")
   };
   services.push(json!({"step":paid,"episode":ep,"output":output,
       "truth":truth,"feedback":fb}));
   feedback_delivered.push(fb);
   world.intervene(Intervention::SetCell{key:20,value:fb}).unwrap();
   let ext=world.events().last().unwrap();
   assert!(matches!(ext.origin,Origin::External));
   assert!(matches!(&ext.kind,EventKind::Intervened(_)));
   peak=peak.max(world.occupied_units());
   serde_json::to_writer(&mut stream,&json!(["feedback",paid,ep,goal,
     output,truth,fb,KEYS.map(|key|world.cell(key)),world.occupied_units(),
     source_guess(&world)])).unwrap();
   stream.write_all(b"\n").unwrap();
  }
 }
 stream.flush().unwrap();
 assert!(world.process(1).unwrap().stopped,"original paid PID1 did not Halt");
 assert_eq!(feedback_reads.len(),services.len());
 assert_eq!(feedback_delivered,feedback_reads.iter()
   .map(|v|v["value"].as_i64().unwrap()).collect::<Vec<_>>());
 assert_eq!(paid,expected["paid_steps"].as_u64().unwrap() as usize);
 assert_eq!(world.process(1).unwrap().pc,expected["halt_pc"].as_u64().unwrap() as usize);
 assert_eq!(INITIAL_CREDITS-paid as u64,expected["credits_remaining"].as_u64().unwrap());
 assert_eq!(source_guess(&world),expected["final_executable_guess"].as_i64().unwrap());
 assert_eq!(json!(world.cell(21)),expected["actor_claimed_success"]);
 assert_eq!(json!(world.cell(32)),expected["actor_completed_episode"]);
 assert_eq!(json!(edits),expected["original_paid_source_patches"]);
 assert_eq!(json!(services),expected["original_paid_task_services"]);
 assert_eq!(json!(feedback_reads),expected["original_paid_feedback_reads"]);
 assert_eq!(json!(success_writes),expected["original_paid_success_writes"]);
 assert_eq!(json!(attempt_writes),expected["original_paid_attempt_writes"]);
 assert_eq!(json!(terminal_writes),expected["terminal_claim_writes"]);
 assert_eq!(world.used_cells(),expected["occupied_data_cells"].as_u64().unwrap() as usize);
 assert_eq!(peak,expected["peak_occupied"].as_u64().unwrap() as usize);
 let external=world.events().iter().filter(|v|matches!(v.origin,Origin::External)).collect::<Vec<_>>();
 assert_eq!(external.len(),services.len()+1);
 assert!(matches!(&external[0].kind,EventKind::Loaded));
 assert!(external.iter().skip(1).all(|e|matches!(&e.kind,EventKind::Intervened(_))));
 json!({"arm":name,"goals":goals,"paid_original_steps":paid,
   "final_original_executable_const1":source_guess(&world),
   "actor_claimed_success":world.cell(21),
   "actor_completed_episode":world.cell(32),
   "original_paid_source_patches":edits,
   "original_paid_task_services":services,
   "original_paid_feedback_reads":feedback_reads,
   "original_PID1_Halted":true,
   "peak_occupied":peak,
   "host_source_edit_events_after_load":0})
}
#[test]
fn policy_p0_original_task_feedback_mutates_paid_executable_const(){
 let root=std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
 let frozen:Value=serde_json::from_slice(&fs::read(root.join(FROZEN)).unwrap()).unwrap();
 let source=frozen["original_native_CIL"].as_array().unwrap();
 let arms=frozen["original_native_task_cases"].as_array().unwrap();
 let controls=frozen["causal_controls"].as_array().unwrap();
 assert_eq!(source.len(),43);
 assert_eq!(arms.len(),4);
 assert_eq!(controls.len(),4);
 assert_eq!(source[1],json!({"kind":"Const","dst":3,"value":0,"next":2}));
 assert_eq!(source[2],json!({"kind":"Write","address":2,"value":3,"next":3}));
 assert_eq!(source[22],json!({"kind":"PatchConst","node":1,"value":3,"next":0}));
 let directory=root.join("../../cl1b-ci-evidence");
 fs::create_dir_all(&directory).unwrap();
 let mut native_arms=Vec::<Value>::new();
 let mut native_controls=Vec::<Value>::new();
 let mut total_patches=0_usize;
 for arm in arms{
  let name=arm["name"].as_str().unwrap();
  let g=arm["goals"].as_array().unwrap();
  let goals=[g[0].as_i64().unwrap(),g[1].as_i64().unwrap(),
             g[2].as_i64().unwrap(),g[3].as_i64().unwrap()];
  let observed=run(source,name,goals,"truth",false,&arm["expected"],&directory);
  assert_eq!(observed["actor_claimed_success"],1);
  assert_eq!(observed["actor_completed_episode"],4);
  let service=observed["original_paid_task_services"].as_array().unwrap();
  assert!(service.len()>=4);
  assert_eq!(service.iter().filter(|x|x["truth"]==1).count(),4);
  total_patches+=observed["original_paid_source_patches"].as_array().unwrap().len();
  native_arms.push(observed);
 }
 assert_eq!(total_patches,20);
 for ctrl in controls{
  let name=ctrl["name"].as_str().unwrap();
  let g=ctrl["goals"].as_array().unwrap();
  let goals=[g[0].as_i64().unwrap(),g[1].as_i64().unwrap(),
             g[2].as_i64().unwrap(),g[3].as_i64().unwrap()];
  let mode=ctrl["feedback"].as_str().unwrap();
  let disabled=ctrl["suppressed"].as_bool().unwrap();
  let observed=run(source,name,goals,mode,disabled,&ctrl["expected"],&directory);
  if disabled || mode=="forced0"{
   assert_ne!(observed["actor_claimed_success"],1,
      "disabled source edits or sham all-zero rewards must not claim task success");
  }else if mode=="inverted"{
   assert_eq!(observed["actor_claimed_success"],1);
   assert!(observed["original_paid_task_services"].as_array().unwrap()
      .iter().filter(|v|v["truth"]==1).count()<4,
      "inverted feedback must reveal an actor-versus-truth discrepancy");
  }
  native_controls.push(observed);
 }
 let all_steps=native_arms.iter().chain(native_controls.iter())
   .map(|v|v["paid_original_steps"].as_u64().unwrap()).sum::<u64>();
 assert_eq!(all_steps,1094);
 fs::write(directory.join("cl1b_policy_p0_native_task_source_policy.json"),
   serde_json::to_vec_pretty(&json!({
    "study":"CL1B-POLICY-P0",
    "level":"NATIVE_ORIGINAL_PID_FEEDBACK_CONDITIONED_PAID_EXECUTABLE_CONST",
    "source_sha":std::env::var("GITHUB_SHA").unwrap_or_else(|_|"local".into()),
    "original_task_regimes":native_arms,"controls":native_controls,
    "counts":{"original_worlds":4,"causal_controls":4,
      "paid_original_cil_steps":all_steps,
      "actual_source_mutations":total_patches,
      "full_truthful_four_stage_task_completions":4},
    "limits":"Separate 43-node mechanism proof, not 180-node G3Q coverage or novel algorithm invention; actor edits source task-output byte under a fully human-authored finite candidate-cycle policy."
   })).unwrap()).unwrap();
}
