//! G3Q-C1-P1-L3-I6U: original native single PID1 pays for up to two
//! executable source edits with independent full 3-flag recertification.
//! This is a FIXED second-edit causal baseline, NOT learned repair policy.
//! Preregistered exact 179 CIL nodes and all64 original genomes + controls.
//! unchanged World::with_strict_capacity(16,180,300).
use cl0_world::{EventKind,Intervention,Op,Origin,Process,World};
use serde_json::{json,Value};
use std::fs::{self,File};
use std::io::{BufWriter,Write};
const FROZEN:&str="../../../cl1b/studies/B56-F1d1k-G3Q-C1-P1-L3-I6U-frozen-180-node-bounded-task-policy-search-v1.json";
const KEYS:[i64;14]=[7,8,9,11,20,21,32,33,50,52,56,62,63,64];
const CREDIT:u64=8000;
fn decode(v:&Value)->Op{
 let name=v["kind"].as_str().unwrap();
 let tagged=if name=="Halt"{json!("Halt")}else{
  let mut fields=serde_json::Map::new();
  for (k,value) in v.as_object().unwrap(){
   if k!="kind"{fields.insert(k.clone(),value.clone());}
  }
  let mut obj=serde_json::Map::new();
  obj.insert(name.to_string(),Value::Object(fields));
  Value::Object(obj)
 };
 serde_json::from_value(tagged).expect("invalid frozen I6U original CIL")
}
fn gene(w:&World,at:usize)->i64{
 match &w.process(1).unwrap().code.nodes[at]{
  Op::Const{dst:3,value,next} if (at==0&&*next==1)||
    (at==3&&*next==4)||(at==6&&*next==7)=>*value,
  other=>panic!("original source gene changed unexpectedly: {other:?}")
 }
}
fn mode_oracle(g:[i64;3])->Vec<i64>{
 (0..3_i64).map(|trial|{
  let mut accumulator=0_i64;
  for (pos,word) in g.into_iter().enumerate(){
   let opcode=if pos==1{(word+trial)%4}else{word};
   accumulator=if opcode==3{0}else{(accumulator+opcode)%3};
  }
  accumulator
 }).collect()
}
fn run(frozen:&[Value],name:&str,seed:[i64;3],goals:[i64;4],
  feedback_regime:&str,suppress_edits:bool,predicted:&Value,
  evidence_dir:&std::path::Path)->Value{
 let mut native_source=frozen.to_vec();
 for (pc,val) in [(0,seed[0]),(3,seed[1]),(6,seed[2])]{
  native_source[pc]["value"]=json!(val);
 }
 let founder:Vec<Op>=native_source.iter().map(decode).collect();
 let mut expected_current_source=founder.clone();
 let mut native=World::with_strict_capacity(16,180,300);
 if suppress_edits{native=native.with_native_code_edits_disabled();}
 native.load(Process::new(1,founder.clone(),CREDIT)).unwrap();
 assert_eq!(native.occupied_units(),181);
 let mut trace=BufWriter::new(File::create(evidence_dir.join(format!(
  "b56_f1d1k_g3q_c1_p1_l3_i6u_original_{name}.jsonl"))).unwrap());
 let mut paid=0_usize;
 let mut peak=native.occupied_units();
 let mut witnesses=Vec::<Value>::new();
 let mut resets=Vec::<Value>::new();
 let mut certs=Vec::<usize>::new();
 let mut services=Vec::<Value>::new();
 let mut patches=Vec::<Value>::new();
 let mut decisions=Vec::<Value>::new();
 let mut retry_writes=Vec::<Value>::new();
 let mut guard_reads=Vec::<Value>::new();
 let mut paid_meta_reads=0_usize;
 let mut paid_feedback_reads=Vec::<i64>::new();
 let mut delivered_feedback=Vec::<i64>::new();
 assert!(goals.iter().all(|target|(0_i64..=2_i64).contains(target)),
    "native task goals must be within prospectively registered finite alphabet");
 while native.step(){
  paid+=1; assert!(paid<=CREDIT as usize);
  let ev=native.events().last().unwrap().clone();
  assert!(matches!(ev.origin,Origin::Internal(1)));
  let p=native.process(1).unwrap();
  if [150,162,168,174,178].contains(&ev.pc){
   let test=match &founder[ev.pc]{
    Op::Branch{test,..}=>*test as usize,
    other=>panic!("native actor decision was not original paid Branch: {other:?}")
   };
   let on=p.registers[test];
   let target=match &founder[ev.pc]{
    Op::Branch{nonzero,zero,..}=>if on!=0{*nonzero}else{*zero},
    _=>unreachable!()
   };
   decisions.push(json!({"step":paid,"pc":ev.pc,"condition":on,"target":target}));
  }
  if ev.pc==149{
   assert!(matches!(&ev.kind,EventKind::CellRead{key:11,..}));
   guard_reads.push(json!({"step":paid,"value":p.registers[0]}));
  }
  if [152,177].contains(&ev.pc){
   match &ev.kind{
    EventKind::CellWritten{key:11,value}=>{
     retry_writes.push(json!({"step":paid,"key":11,"value":value}));
    }
    other=>panic!("paid retry guard mark absent: {other:?}")
   }
  }
  if [169,171].contains(&ev.pc){
   assert!(patches.len()<2,"original actor cannot perform third source edit");
   // Full native paid witness isolation before BOTH original code edits.
   assert_eq!(resets.len(),3*(patches.len()+1));
   assert_eq!([native.cell(7),native.cell(8),native.cell(9)],
      [Some(0),Some(0),Some(0)],"stale coverage witness before executable patch");
   let target=p.registers[1] as usize;
   assert!(target==3||target==6,"only two registered source targets");
   let old_value=match &expected_current_source[target]{
    Op::Const{value,..}=>*value,
    other=>panic!("source target is not editable Const: {other:?}")
   };
   let after=gene(&native,target);
   if suppress_edits{
    assert!(matches!(&ev.kind,EventKind::NativeCodeEditSuppressed{opcode_tag:9}));
    assert_eq!(after,old_value);
   }else{
    assert!(matches!(&ev.kind,EventKind::Executed));
    let next=if target==3{4}else{7};
    expected_current_source[target]=Op::Const{dst:3,value:after,next};
   }
   patches.push(json!({"step":paid,"pc":ev.pc,"node":target,
    "before":old_value,"after":after,"edit_suppressed":suppress_edits}));
  }
  assert_eq!(p.code.nodes,expected_current_source,
   "only original charged PatchConst may change executable source");
  assert!(native.process(2).is_none(),"no new process or hidden restart");
  assert_eq!(native.total_compute_inventory(),Some(CREDIT-paid as u64));
  assert!(native.used_cells()<=16&&native.occupied_units()<=300);
  peak=peak.max(native.occupied_units());
  if ev.pc==24{
   assert!(matches!(&ev.kind,EventKind::CellRead{..}));
   paid_meta_reads+=1;
  }
  if ev.pc==107{
   match &ev.kind{
    EventKind::CellWritten{key,value}=>{
     assert!((7..=9).contains(key)&&*value==1);
     witnesses.push(json!({"step":paid,"key":key,"value":value}));
    }
    other=>panic!("native paid witness expected: {other:?}")
   }
  }
  if [155,157,159].contains(&ev.pc){
   match &ev.kind{
    EventKind::CellWritten{key,value}=>{
     assert!((7..=9).contains(key)&&*value==0);
     resets.push(json!({"step":paid,"key":key,"value":value}));
    }
    other=>panic!("native actor must pay full flag reset: {other:?}")
   }
  }
  if ev.pc==147{
   assert!(matches!(&ev.kind,EventKind::CellWritten{key:21,value:1}));
   assert_eq!([native.cell(7),native.cell(8),native.cell(9)],
      [Some(1),Some(1),Some(1)]);
   certs.push(paid);
  }
  if ev.pc==62{
   match &ev.kind{
    EventKind::CellRead{key:20,value}=>paid_feedback_reads.push(*value),
    other=>panic!("native original feedback Read20 absent: {other:?}")
   }
  }
  let row=json!(["cil",paid,ev.pc,p.pc,p.registers,p.credits,
   KEYS.map(|key|native.cell(key)),native.occupied_units(),
   gene(&native,3),gene(&native,6)]);
  serde_json::to_writer(&mut trace,&row).unwrap();
  trace.write_all(b"\n").unwrap();
  if ev.pc==54{
   assert_eq!(certs.len(),1,"original task service started prior to paid certificate");
   let output=match &ev.kind{
    EventKind::CellWritten{key:11,value}=>*value,
    other=>panic!("not a real actor paid Write11 task service: {other:?}")
   };
   let episode=native.cell(32).unwrap_or(0) as usize;
   assert!(episode<4);
   let goal=goals[episode];
   let truth=i64::from(output==goal);
   let signal=match feedback_regime{
    "truth"=>truth,"inverted"=>1-truth,
    other=>panic!("unregistered feedback regime {other}")
   };
   let middle=native.cell(63).unwrap();
   services.push(json!({"step":paid,"episode":episode,"middle":middle,
      "output":output,"truth":truth,"feedback":signal}));
   delivered_feedback.push(signal);
   native.intervene(Intervention::SetCell{key:20,value:signal}).unwrap();
   let outside=native.events().last().unwrap();
   assert!(matches!(outside.origin,Origin::External));
   assert!(matches!(&outside.kind,EventKind::Intervened(_)));
   serde_json::to_writer(&mut trace,&json!(["feedback",paid,episode,goal,
     middle,output,truth,signal,KEYS.map(|key|native.cell(key)),
     native.occupied_units(),gene(&native,3),gene(&native,6)])).unwrap();
   trace.write_all(b"\n").unwrap();
  }
 }
 trace.flush().unwrap();
 assert!(native.process(1).unwrap().stopped,
  "original native PID did not halt before prescribed paid credit budget");
 let final_rule=[gene(&native,0),gene(&native,3),gene(&native,6)];
 let modes=mode_oracle(final_rule);
 let safe=modes.iter().copied().collect::<std::collections::BTreeSet<_>>().len()==3;
 assert_eq!(safe,certs.len()==1,"false coverage certificate or false rejection");
 assert_eq!(paid,predicted["paid_steps"].as_u64().unwrap() as usize);
 assert_eq!(native.process(1).unwrap().pc,predicted["halt_pc"].as_u64().unwrap() as usize);
 assert_eq!(json!(final_rule),predicted["final_rule"]);
 assert_eq!(json!(modes),predicted["semantic_modes"]);
 assert_eq!(safe,predicted["semantic_coverage"].as_bool().unwrap());
 assert_eq!(json!(witnesses),predicted["paid_witnesses"]);
 assert_eq!(json!(resets),predicted["paid_resets"]);
 assert_eq!(json!(certs),predicted["certificate"]);
 assert_eq!(json!(patches),predicted["source_edits"]);
 assert_eq!(json!(decisions),predicted["paid_decisions"]);
 assert_eq!(json!(retry_writes),predicted["paid_retry_writes"]);
 assert_eq!(json!(guard_reads),predicted["paid_guard_reads"]);
 assert_eq!(json!(services),predicted["task_service"]);
 assert_eq!(paid_meta_reads,predicted["meta_Read_count"].as_u64().unwrap() as usize);
 assert_eq!(json!(paid_feedback_reads),predicted["paid_feedback_reads"]);
 assert_eq!(paid_feedback_reads,delivered_feedback);
 assert_eq!(native.used_cells(),predicted["used_cells"].as_u64().unwrap() as usize);
 assert_eq!(peak,predicted["peak_occupied"].as_u64().unwrap() as usize);
 assert_eq!(json!(native.cell(21)),predicted["actor_claim"]);
 assert_eq!(json!(native.cell(32)),predicted["actor_episode"]);
 let externals=native.events().iter().filter(|event|matches!(event.origin,Origin::External)).collect::<Vec<_>>();
 assert_eq!(externals.len(),services.len()+1);
 assert!(matches!(&externals[0].kind,EventKind::Loaded));
 assert!(externals.iter().skip(1).all(|event|matches!(&event.kind,EventKind::Intervened(_))));
 json!({"arm":name,"seed":seed,"paid_original_steps":paid,
    "original_PID1_naturally_halted":true,"original_PID1_source_edits":patches,
    "native_certified":safe,"semantic_modes":modes,"paid_retry_guard_events":retry_writes,
    "paid_full_witness_reset_events":resets,"real_task_services":services,
    "original_meta_reads":paid_meta_reads,"peak_occupied":peak,
    "final_source_rule":final_rule,"postload_host_source_writes":0,
    "actor_claimed_success":native.cell(21),"actor_completed_episode":native.cell(32)})
}
#[test]
fn g3q_c1_i6u_original_pid_task_reward_driven_bounded_reverse_search_3_regimes(){
 let root=std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
 let manifest:Value=serde_json::from_slice(&fs::read(root.join(FROZEN)).unwrap()).unwrap();
 let source=manifest["exact_original_native_CIL"].as_array().unwrap();
 let regimes=manifest["task_regimes"].as_array().unwrap();
 let arms=manifest["pre_registered_original_regime_arms"].as_array().unwrap();
 let controls=manifest["pre_registered_controls"].as_array().unwrap();
 assert_eq!(source.len(),180);
 assert_eq!(regimes.len(),3);
 assert_eq!(arms.len(),192);
 assert_eq!(controls.len(),3);
 assert_eq!(source[82],json!({"kind":"Const","dst":2,"value":4,"next":83}));
 assert_eq!(source[88],json!({"kind":"Sub","dst":3,"lhs":3,"rhs":2,"next":89}));
 assert_eq!(source[89],json!({"kind":"Const","dst":2,"value":-1,"next":90}));
 assert_eq!(source[92],json!({"kind":"Const","dst":3,"value":2,"next":93}));
 // Exactly four researcher-authored source-instruction changes against
 // original native-verified I6S. Every other CIL node and edge remains.
 let old:Value=serde_json::from_slice(&fs::read(root.join(
   "../../../cl1b/studies/B56-F1d1k-G3Q-C1-P1-L3-I6S-frozen-180-node-two-edits-isolated-true-task-service-v1.json"
 )).unwrap()).unwrap();
 let parent=old["original_native_CIL_source"].as_array().unwrap();
 assert_eq!(parent.len(),180);
 let modified=(0..180).filter(|&pc|parent[pc]!=source[pc]).collect::<Vec<_>>();
 assert_eq!(modified,vec![82,88,89,92],"unregistered original CIL source changes");
 let evidence=root.join("../../cl1b-ci-evidence");
 fs::create_dir_all(&evidence).unwrap();
 let mut original_worlds=Vec::<Value>::new();
 let mut initially_safe=0_usize;
 let mut unsafe_once=0_usize;
 let mut unsafe_twice=0_usize;
 let mut actual_complete=0_usize;
 let mut true_outputs=0_usize;
 let mut task_attempts=0_usize;
 for (i,arm) in arms.iter().enumerate(){
  let v=arm["seed"].as_array().unwrap();
  let seed=[v[0].as_i64().unwrap(),v[1].as_i64().unwrap(),v[2].as_i64().unwrap()];
  assert_eq!(i%64,((seed[0]*4+seed[1])*4+seed[2]) as usize);
  let regime=arm["regime"].as_str().unwrap();
  assert_eq!(arm["regime"],regimes[i/64]["id"]);
  assert_eq!(arm["goals"],regimes[i/64]["goals"]);
  let gv=arm["goals"].as_array().unwrap();
  assert_eq!(gv.len(),4);
  let goals=[gv[0].as_i64().unwrap(),gv[1].as_i64().unwrap(),
             gv[2].as_i64().unwrap(),gv[3].as_i64().unwrap()];
  let name=format!("{}_a{}_b{}_c{}",regime,seed[0],seed[1],seed[2]);
  let mut observed=run(source,&name,seed,goals,"truth",false,&arm["expected"],&evidence);
  observed["regime"]=json!(regime);
  observed["goals"]=json!(goals);
  assert_eq!(observed["native_certified"],true);
  assert_eq!(observed["actor_claimed_success"],1);
  assert_eq!(observed["actor_completed_episode"],4);
  let first_safe=mode_oracle(seed).iter().copied()
    .collect::<std::collections::BTreeSet<_>>().len()==3;
  assert_eq!(first_safe,arm["initial_structurally_safe"].as_bool().unwrap());
  let edit_count=observed["original_PID1_source_edits"].as_array().unwrap().len();
  if first_safe{
   initially_safe+=1;
   assert_eq!(edit_count,0);
  }else if edit_count==1{unsafe_once+=1;}
   else if edit_count==2{unsafe_twice+=1;}
   else{panic!("unsafe founder without exactly one or two original paid executable edits");}
  let service=observed["real_task_services"].as_array().unwrap();
  assert!(service.len()>=4);
  task_attempts+=service.len();
  true_outputs+=service.iter().filter(|s|s["truth"]==1).count();
  actual_complete+=1;
  original_worlds.push(observed);
 }
 assert_eq!((initially_safe,unsafe_once,unsafe_twice,actual_complete),
  (72,96,24,192));
 let counts=&manifest["pre_registered_counts"];
 assert_eq!(task_attempts,counts["task_service_attempts"].as_u64().unwrap() as usize);
 assert_eq!(true_outputs,counts["true_task_matches"].as_u64().unwrap() as usize);
 let mut observed_controls=Vec::<Value>::new();
 for arm in controls{
  let v=arm["seed"].as_array().unwrap();
  let seed=[v[0].as_i64().unwrap(),v[1].as_i64().unwrap(),v[2].as_i64().unwrap()];
  let gv=arm["goals"].as_array().unwrap();
  let goals=[gv[0].as_i64().unwrap(),gv[1].as_i64().unwrap(),
             gv[2].as_i64().unwrap(),gv[3].as_i64().unwrap()];
  let name=arm["name"].as_str().unwrap();
  let disabled=arm["suppressed"].as_bool().unwrap();
  let mode=arm["feedback"].as_str().unwrap();
  let mut result=run(source,name,seed,goals,mode,disabled,&arm["expected"],&evidence);
  result["regime"]=arm["regime"].clone();
  result["goals"]=json!(goals);
  if disabled{
   assert_eq!(result["native_certified"],false);
   assert!(result["real_task_services"].as_array().unwrap().is_empty());
  }else if mode=="inverted"{
   assert_eq!(result["native_certified"],true);
   assert_eq!(result["actor_claimed_success"],1);
   let correct=result["real_task_services"].as_array().unwrap().iter()
      .filter(|s|s["truth"]==1).count();
   assert!(correct<4,"adversarial inverted feedback must demonstrate semantic deception");
  }
  observed_controls.push(result);
 }
 let info=json!({
   "study":"B56-F1d1k-G3Q-C1-P1-L3-I6U",
   "level":"NATIVE_ORIGINAL_PID_180_NODE_TASK_POLICY_3_REGIMES_192_WORLDS",
   "source_sha":std::env::var("GITHUB_SHA").unwrap_or_else(|_|"local".into()),
   "regimes":regimes,
   "original192_worlds":original_worlds,
   "controls":observed_controls,
   "counts":{"regimes":3,"original_genotype_worlds":192,"initial_safe":72,
     "one_edit_rescued":96,"two_edit_rescued":24,
     "all_structurally_certified":192,"all_four_goal_task_completed":192,
     "task_service_attempts":task_attempts,"true_task_matches":true_outputs,
     "false_certificates":0,"false_rejections":0},
   "limits":"Researcher-authored after inspecting finite training outcomes: bounded descending task parameter learner with four attempts. Not a self-invented algorithm, independently blind adaptive policy, indefinite cyberlife or proof for arbitrary goal distributions."
 });
 fs::write(evidence.join("b56_f1d1k_g3q_c1_p1_l3_i6u_native_task_policy_shift.json"),
   serde_json::to_vec_pretty(&info).unwrap()).unwrap();
}
