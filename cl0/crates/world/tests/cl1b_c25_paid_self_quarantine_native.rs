//! CL1B-C25: native PID1-paid self-quarantine of repeated code edits F9/F10 210 nodes in 186 original native one-PID1 worlds
//! 24-stage CIL worlds with genuine actor-paid online relative-stride relearning.
use cl0_world::{EventKind,Intervention,Op,Origin,Process,World};
use serde_json::{json,Value};
use std::collections::BTreeMap;
use std::fs::{self,File};
use std::io::{BufWriter,Write};

const PARENT:&str="../../../cl1b/studies/CL1B-POLICY-P4F6-frozen-actor-paid-clean-success-refill-542-world-v1.json";
const STUDY:&str="../../../cl1b/studies/CL1B-C25-PAID-SELF-QUARANTINE-frozen-276-worlds-v1.json";
const CELLS:[i64;13]=[11,20,21,30,32,33,34,35,36,37,38,39,40];
const CREDIT:u64=6000;
fn decode(v:&Value)->Op{
 let tag=v["kind"].as_str().unwrap();
 let obj=if tag=="Halt"{json!("Halt")}else{
  let mut props=v.as_object().unwrap().clone();
  props.remove("kind");
  let mut outer=serde_json::Map::new();
  outer.insert(tag.into(),Value::Object(props));
  Value::Object(outer)
 };
 serde_json::from_value(obj).expect("bad frozen CIL")
}
fn get_edges(w:&World)->[i64;9]{
 let p=w.process(1).unwrap();
 let edge=|n:usize|match &p.code.nodes[n]{
  Op::Write{address:2,value:1,next}=>*next as i64,
  x=>panic!("not native original Write edge {n}: {x:?}")
 };
 let stride=match &p.code.nodes[46]{
  Op::Const{dst:2,value,next:47}=>*value,
  x=>panic!("not native original stride PC46: {x:?}")
 };
 let monitor=match &p.code.nodes[133]{
  Op::Write{address:2,value:3,next}=>*next as i64,
  x=>panic!("original PC133 cheap-success Write missing: {x:?}")
 };
 let qedge=|n:usize|match p.code.nodes.get(n){
  Some(Op::Write{next,..})|Some(Op::Const{next,..})=>*next as i64,
  None=>-1, other=>panic!("bad quarantine target {n} {other:?}")
 };
 [edge(12),edge(34),stride,monitor,qedge(128),qedge(134),qedge(138),qedge(153),qedge(175)]
}
fn run(src:&[Value],arm:&Value,dir:&std::path::Path)->Value{
 let name=arm["name"].as_str().unwrap();
 let source=arm["source"].as_str().unwrap();
 assert!(source=="F9"||source=="F10"||source=="F8BASE"||source=="F5");
 assert_eq!(src.len(),if source=="F9"||source=="F10"{210}else if source=="F8BASE"{195}else{153});
 let goals=arm["goals"].as_array().unwrap().iter()
  .map(|x|x.as_i64().unwrap()).collect::<Vec<_>>();
 assert_eq!(goals.len(),24);
 assert!(goals.iter().all(|x|(0_i64..=2).contains(x)));
 let mode=arm["feedback"].as_str().unwrap();
 assert!(matches!(mode,"truth"|"inverted"|"forced0"));
 let disabled=arm["patchnext_suppressed"].as_bool().unwrap();
 let initial=src.iter().map(decode).collect::<Vec<_>>();
 let mut expected=initial.clone();
 let mut w=World::with_strict_capacity(17,210,340);
 if disabled{w=w.with_native_patchnext_disabled();}
 w.load(Process::new(1,initial,CREDIT)).unwrap();
 assert_eq!(w.occupied_units(),src.len()+1);
 let mut trace=BufWriter::new(File::create(dir.join(format!(
  "cl1b_c25_original_{name}.jsonl"))).unwrap());
 let mut paid=0_usize;
 let mut peak=w.occupied_units();
 let mut services=Vec::<Value>::new();
 let mut edits=Vec::<Value>::new();
 let mut edges=Vec::<Value>::new();
 let mut writes37=Vec::<Value>::new();
 let mut reads37=Vec::<Value>::new();
 let mut observations=0_usize;
 let mut writes38=Vec::<Value>::new();
 let mut reads38=Vec::<Value>::new();
 let mut budget_gates=Vec::<Value>::new();
 let mut writes39=Vec::<Value>::new();
 let mut reads39=Vec::<Value>::new();
 let mut clean_streak_gates=Vec::<Value>::new();
 let mut paid_attempt_reads=Vec::<Value>::new();
 let mut paid_attempt_decisions=Vec::<Value>::new();
 let mut paid_counter_reads=Vec::<Value>::new();
 let mut paid_counter_writes=Vec::<Value>::new();
 let mut paid_quarantine_self_edits=Vec::<Value>::new();
 while w.step(){
  paid+=1;
  assert!(paid<=CREDIT as usize);
  let ev=w.events().last().unwrap().clone();
  assert!(matches!(ev.origin,Origin::Internal(1)));
  let pc=ev.pc;
  if [45,88,102,122].contains(&pc){
   let p=w.process(1).unwrap();
   assert!(matches!(&ev.kind,EventKind::Executed));
   assert_eq!(p.registers[1],46);
   assert!((0..=2).contains(&p.registers[0]));
   if pc==122{
    assert!(source=="F9"||source=="F10"||source=="F8BASE"||source=="F5");
    assert_eq!(get_edges(&w)[1],106);
    assert!(!reads37.is_empty());
   }
   let before=match &expected[46]{
    Op::Const{dst:2,value,next:47}=>*value,
    _=>panic!("unexpected stride source")
   };
   expected[46]=Op::Const{dst:2,value:p.registers[0],next:47};
   edits.push(json!({"step":paid,"pc":pc,"before":before,
    "after":p.registers[0]}));
  }
  if [82,131,136,139,155,177].contains(&pc){
   let p=w.process(1).unwrap();
   let (target,desired)=match (source,pc){
    ("F9"|"F10"|"F8BASE"|"F5",82)=>(12,137),
    ("F9"|"F10"|"F8BASE"|"F5",131)=>(34,132),
    ("F9"|"F10"|"F8BASE"|"F5",136)=>(34,132),
    ("F9"|"F10"|"F8BASE"|"F5",139)=>(34,106),
    ("F9"|"F10"|"F8BASE",155)=>(133,156),
    ("F9"|"F10"|"F8BASE",177)=>(133,62),
    _=>unreachable!()
   };
   assert_eq!(p.registers[1],target as i64);
   assert_eq!(p.registers[0],desired as i64);
   let before=match &expected[target]{
    Op::Write{address:2,value,next} if *value==(if target==133{3}else{1})=>*next,
    _=>panic!("source edit target not paid executable Write"),
   };
   if disabled{
    assert!(matches!(&ev.kind,EventKind::NativeCodeEditSuppressed{opcode_tag:6}));
   }else{
    assert!(matches!(&ev.kind,EventKind::Executed));
    expected[target]=Op::Write{address:2,value:if target==133{3}else{1},next:desired};
   }
   edges.push(json!({"step":paid,"pc":pc,"target":target,
    "before":before,"after":if disabled{before}else{desired},
    "suppressed":disabled}));
  }
  if pc==128||pc==124||pc==133{
   assert!(source=="F9"||source=="F10"||source=="F8BASE"||source=="F5");
   // PC133 cheap Write37 is inherited unchanged from F5: both sources execute it.
   match &ev.kind{
    EventKind::CellWritten{key:37,value}=>{
     assert_eq!(w.cell(30),Some(*value));
     writes37.push(json!({"step":paid,"pc":pc,"value":value}));
    },
    x=>panic!("missing actor-paid Write37: {x:?}")
   }
  }
  if pc==110{
   assert!(source=="F9"||source=="F10"||source=="F8BASE"||source=="F5");
   match &ev.kind{
    EventKind::CellRead{key:37,value}=>{
     assert_eq!(writes37.last().unwrap()["value"],json!(value));
     reads37.push(json!({"step":paid,"value":value}));
    },
    x=>panic!("missing actor-paid Read37: {x:?}")
   }
  }
  if pc==143{
   assert!(source=="F9"||source=="F10"||source=="F8BASE"||source=="F5");
   match &ev.kind{
    EventKind::CellRead{key:38,value}=>{
     assert!((0..=1).contains(value));
     reads38.push(json!({"step":paid,"value":value}));
    },
    x=>panic!("missing paid budget Read38 {x:?}")
   }
  }
  if pc==152{
   assert!(source=="F9"||source=="F10"||source=="F8BASE"||source=="F5");
   match &ev.kind{
    EventKind::CellWritten{key:38,value}=>{
     assert_eq!(*value,1);
     assert_eq!(edits.iter().filter(|x|x["pc"]==122).count(),
      writes38.iter().filter(|x|x["value"].as_i64()==Some(1)).count()+1);
     writes38.push(json!({"step":paid,"value":value}));
    },
    x=>panic!("missing paid meter Write38 {x:?}")
   }
  }
  if pc==146{
   assert!(source=="F9"||source=="F10"||source=="F8BASE"||source=="F5");
   assert!(matches!(&ev.kind,EventKind::Executed));
   let meter=w.cell(38).unwrap_or(0);
   let jump=w.process(1).unwrap().pc;
   assert_eq!(jump,if meter==1{140}else{138});
   budget_gates.push(json!({"step":paid,"meter":meter,"next":jump}));
  }
  if pc==174{
   assert!(source=="F9"||source=="F10"||source=="F8BASE");
   match &ev.kind{
    EventKind::CellWritten{key:38,value}=>{
     assert_eq!(*value,0);
     assert_eq!(w.cell(39),Some(0));
     writes38.push(json!({"step":paid,"value":value}));
    },
    x=>panic!("missing original paid budget replenish Write38=0: {x:?}")
   }
  }
  if pc==163{
   assert!(source=="F9"||source=="F10"||source=="F8BASE");
   match &ev.kind{
    EventKind::CellRead{key:39,value}=>{
     assert!((0..=3).contains(value));
     reads39.push(json!({"step":paid,"value":value}));
    },
    x=>panic!("missing paid clean-streak Read39: {x:?}")
   }
  }
  if [161,167,172].contains(&pc){
   assert!(source=="F9"||source=="F10"||source=="F8BASE");
   match &ev.kind{
    EventKind::CellWritten{key:39,value}=>{
     assert!(if pc==167{(1..=4).contains(value)}else{*value==0});
     writes39.push(json!({"step":paid,"pc":pc,"value":value}));
    },
    x=>panic!("missing paid clean-streak Write39: {x:?}")
   }
  }
  if pc==158||pc==170{
   assert!(source=="F9"||source=="F10"||source=="F8BASE");
   assert!(matches!(&ev.kind,EventKind::Executed));
   let native_next=w.process(1).unwrap().pc;
   let observed=if pc==158{w.cell(33).unwrap_or(0)}else{w.cell(39).unwrap_or(0)};
   let expected_next=if pc==158{if observed!=0{159}else{162}}
     else{if observed==4{171}else{62}};
   assert_eq!(native_next,expected_next);
   clean_streak_gates.push(json!({"step":paid,"pc":pc,"value":observed,"next":native_next}));
  }
  if pc==5{
   assert!(matches!(&ev.kind,EventKind::CellRead{key:20,..}));
   observations+=1;
  }
  if pc==185{
   assert_eq!(source,"F6");
   match &ev.kind{
    EventKind::CellRead{key:40,value}=>{
      assert_eq!(*value+1,services.len() as i64);
      assert_eq!(w.cell(40).unwrap_or(0),*value);
      paid_counter_reads.push(json!({"step":paid,"value":value}));
    },
    other=>panic!("C24 original paid preincrement Read40 missing: {other:?}")
   }
  }
  if pc==189{
   assert_eq!(source,"F6");
   match &ev.kind{
    EventKind::CellWritten{key:40,value}=>{
      assert_eq!(*value,services.len() as i64);
      assert_eq!(w.cell(40),Some(*value));
      paid_counter_writes.push(json!({"step":paid,"value":value}));
    },
    other=>panic!("C24 original paid Write40 service counter missing: {other:?}")
   }
  }
  if pc==181{
   assert_eq!(source,"F6");
   match &ev.kind{
    EventKind::CellRead{key:40,value}=>{
      assert_eq!(*value,services.len() as i64);
      paid_attempt_reads.push(json!({"step":paid,"attempts":value}));
    },
    other=>panic!("C24 charged attempted task services Read40 missing: {other:?}")
   }
  }
  if pc==190||pc==194{
   assert_eq!(source,"F6");
   assert!(matches!(&ev.kind,EventKind::Executed));
   let p=w.process(1).unwrap();
   let attempts=w.cell(40).unwrap_or(0);
   if pc==190{
     assert_eq!(p.registers[0],attempts-2);
     assert_eq!(p.pc,if attempts==2{if source=="F10"{195}else{191}}else{192});
   }else{
     assert_eq!(p.registers[0],attempts-3);
     assert_eq!(p.pc,if source=="F10"{195}else if attempts==3{191}else if source=="F9"{195}else{127});
   }
   paid_attempt_decisions.push(json!({"step":paid,"pc":pc,"attempts":attempts,"next":p.pc}));
  }
  if [197,200,203,206,209].contains(&pc){
   assert!(source=="F9"||source=="F10");
   let (target,desired)=match pc{197=>(128,46),200=>(134,123),203=>(138,140),206=>(153,134),209=>(175,178),_=>unreachable!()};
   let actor=w.process(1).unwrap();
   assert_eq!(actor.registers[1],target as i64);
   assert_eq!(actor.registers[0],desired as i64);
   let before=match &expected[target]{
     Op::Write{next,..}|Op::Const{next,..}=>*next,
     other=>panic!("native self quarantine target invalid {other:?}")
   };
   if disabled{
     assert!(matches!(&ev.kind,EventKind::NativeCodeEditSuppressed{opcode_tag:6}));
   }else{
     assert!(matches!(&ev.kind,EventKind::Executed));
     match &mut expected[target]{
       Op::Write{next,..}|Op::Const{next,..}=>*next=desired,
       other=>panic!("native self quarantine edit invalid {other:?}")
     }
   }
   paid_quarantine_self_edits.push(json!({"step":paid,"pc":pc,"target":target,
      "before":before,"after":if disabled{before}else{desired},"suppressed":disabled}));
  }
  let pid=w.process(1).unwrap();
  assert_eq!(pid.code.nodes,expected,"unexpected postload source mutation");
  assert_eq!(pid.credits,CREDIT-paid as u64);
  assert_eq!(w.total_compute_inventory(),Some(CREDIT-paid as u64));
  assert!(w.process(2).is_none());
  assert!(w.used_cells()<=17&&w.occupied_units()<=340);
  peak=peak.max(w.occupied_units());
  serde_json::to_writer(&mut trace,&json!(["cil",paid,pc,pid.pc,pid.registers,
   pid.credits,CELLS.map(|key|w.cell(key)),w.occupied_units(),get_edges(&w)])).unwrap();
  trace.write_all(b"\n").unwrap();
  if pc==3{
   let guess=match &ev.kind{
    EventKind::CellWritten{key:11,value}=>*value,
    x=>panic!("original task service not paid Write11: {x:?}")
   };
   assert_eq!(guess,pid.registers[3]);
   let stage=w.cell(32).unwrap_or(0) as usize;
   assert!(stage<24);
   let truth=i64::from(guess==goals[stage]);
   let feedback=match mode{
    "truth"=>truth,"inverted"=>1-truth,"forced0"=>0,_=>unreachable!()
   };
   services.push(json!({"step":paid,"stage":stage,"guess":guess,
    "truth":truth,"feedback":feedback}));
   w.intervene(Intervention::SetCell{key:20,value:feedback}).unwrap();
   assert!(matches!(w.events().last().unwrap().origin,Origin::External));
   peak=peak.max(w.occupied_units());
   serde_json::to_writer(&mut trace,&json!(["feedback",paid,stage,goals[stage],
    guess,truth,feedback,CELLS.map(|key|w.cell(key)),w.occupied_units(),
    get_edges(&w)])).unwrap();
   trace.write_all(b"\n").unwrap();
  }
 }
 trace.flush().unwrap();
 assert!(w.process(1).unwrap().stopped);
 assert_eq!(observations,services.len());
 let edits122=edits.iter().filter(|x|x["pc"]==122).count();
 if source=="F9"||source=="F10"||source=="F8BASE"{
  let ups=writes38.iter().filter(|x|x["value"].as_i64()==Some(1)).count();
  let downs=writes38.iter().filter(|x|x["value"].as_i64()==Some(0)).count();
  assert_eq!(edits122,ups);
  assert!(ups>=downs&&ups<=downs+1);
  assert_eq!(w.cell(38).unwrap_or(0),(ups-downs) as i64);
  if source=="F8BASE"{
   assert_eq!(edges.iter().filter(|x|x["pc"]==155).count(),ups);
   assert_eq!(edges.iter().filter(|x|x["pc"]==177).count(),downs);
  }
 }else{
  assert!(edits122<=1);
  assert_eq!(writes38.len(),edits122);
  assert_eq!(w.cell(38).unwrap_or(0),edits122 as i64);
  assert!(writes39.is_empty()&&reads39.is_empty()&&clean_streak_gates.is_empty());
 }
 let truthful=services.iter().filter(|x|x["truth"]==1).count();
 let external=w.events().iter().filter(|e|matches!(e.origin,Origin::External)).count();
 assert_eq!(external,services.len()+1);
 json!({"name":name,"source":source,"regime":arm["regime"],"goals":goals,
  "feedback":mode,"suppressed":disabled,"steps":paid,"services":services,
  "read20_count":observations,"truthful_matches":truthful,
  "completed_stages":w.cell(32),"claim":w.cell(21),
  "code_edits":edits,"edge_edits":edges,
  "write37":writes37,"read37":reads37,
  "write38":writes38,"read38":reads38,"budget_gates":budget_gates,
  "write39":writes39,"read39":reads39,"clean_streak_gates":clean_streak_gates,
   "paid_attempt_reads":paid_attempt_reads,"paid_attempt_decisions":paid_attempt_decisions,
   "paid_counter_reads":paid_counter_reads,"paid_counter_writes":paid_counter_writes,
   "paid_quarantine_self_edits":paid_quarantine_self_edits,
  "online_edit_budget_meter":w.cell(38).unwrap_or(0),
  "final_edges":get_edges(&w),"peak_occupied":peak,
  "original_pid_halted":true,"host_source_edits":0})
}
#[test]
fn c25_276_original_native_actor_paid_self_quarantine_worlds(){
 let root=std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
 let m:Value=serde_json::from_slice(&fs::read(root.join(STUDY)).unwrap()).unwrap();
 let parent:Value=serde_json::from_slice(&fs::read(root.join(PARENT)).unwrap()).unwrap();
 let old:Value=serde_json::from_slice(&fs::read(root.join("../../../cl1b/studies/CL1B-C24-PAID-SERVICE-COUNT-EDIT-GATE-frozen-186-worlds-v1.json")).unwrap()).unwrap();
 assert_eq!(m["study"],"CL1B-C25-F9F10-ACTOR-PAID-PATCHNEXT-SELF-QUARANTINE");
 assert_eq!(m["status"],"PRE_NATIVE_FROZEN_SIX_ARM_276_NATIVE_WORLDS_NEW45_GOALS");
 assert_eq!(parent["study"],"CL1B-POLICY-P4F6");
 assert_eq!(old["study"],"CL1B-C24-F8-PAID-CUMULATIVE-SERVICE-PROXY-EDIT-GATE");
 let f9=m["actor_paid_source_F9_selective_210_nodes"].as_array().unwrap();
 let f10=m["actor_paid_source_F10_full_210_nodes"].as_array().unwrap();
 let f8=old["exact_frozen_F8_source_195_nodes"].as_array().unwrap();
 let f5=parent["baseline_source_f5"].as_array().unwrap();
 assert_eq!(f9.len(),210);
 assert_eq!(f10.len(),210);
 assert_eq!(f8.len(),195);
 assert_eq!(f5.len(),153);
 assert_eq!(f9[194]["nonzero"],195);
 assert_eq!(f9[194]["zero"],191);
 assert_eq!(f9[190]["zero"],191);
 assert_eq!(f10[190]["zero"],195);
 assert_eq!(f10[194]["zero"],195);
 assert_eq!(f10[194]["nonzero"],195);
 for i in 0..195{
   if i!=194{assert_eq!(f9[i],f8[i],"F9 unregistered inherited original code change at {i}");}
   if i!=190&&i!=194{assert_eq!(f10[i],f8[i],"F10 unregistered inherited original code change at {i}");}
 }
 for i in 195..210{assert_eq!(f9[i],f10[i],"F9 and F10 quarantine CIL implementation differs unexpectedly");}
 for (idx,(node,target)) in [(128,46),(134,123),(138,140),(153,134),(175,178)].iter().enumerate(){
   let pc=195+idx*3;
   assert_eq!(f9[pc],json!({"kind":"Const","dst":1,"value":node,"next":pc+1}));
   assert_eq!(f9[pc+1],json!({"kind":"Const","dst":0,"value":target,"next":pc+2}));
   assert_eq!(f9[pc+2],json!({"kind":"PatchNext","node":1,"target":0,"next":if idx==4{127}else{pc+3}}));
 }
 let goals=m["goal_cases"].as_array().unwrap();
 let arms=m["arms"].as_array().unwrap();
 let controls=m["controls"].as_array().unwrap();
 assert_eq!(goals.len(),45);
 assert_eq!(arms.len(),270);
 assert_eq!(controls.len(),6);
 let prior_files=[
 "../../../cl1b/studies/CL1B-POLICY-P4F6-frozen-actor-paid-clean-success-refill-542-world-v1.json",
 "../../../cl1b/studies/CL1B-C22-PAID-PATCHNEXT-SELF-EDIT-RETURN-frozen-141-worlds-v1.json",
 "../../../cl1b/studies/CL1B-C23-PAID-STAGE-GATE-frozen-186-worlds-v1.json",
 "../../../cl1b/studies/CL1B-C24-PAID-SERVICE-COUNT-EDIT-GATE-frozen-186-worlds-v1.json",
 "../../../cl1b/studies/CL1B-C24B-F8-PAID-SERVICE-COUNT-HELDOUT-REPLICATION-frozen-186-worlds-v1.json",
 ];
 let mut prior=std::collections::BTreeSet::new();
 for file in prior_files{
   let document:Value=serde_json::from_slice(&fs::read(root.join(file)).unwrap()).unwrap();
   for case in document["goal_cases"].as_array().unwrap(){
     prior.insert(case["goals"].to_string());
   }
 }
 let mut fresh=std::collections::BTreeSet::new();
 for case in goals{
   assert_eq!(case["goals"].as_array().unwrap().len(),24);
   assert!(!prior.contains(&case["goals"].to_string()),"C25 repeats a historical full 24-stage task");
   assert!(fresh.insert(case["goals"].to_string()),"C25 duplicated own frozen new goal");
 }
 assert_eq!(fresh.len(),45);
 let dir=root.join("../../cl1b-ci-evidence");
 fs::create_dir_all(&dir).unwrap();
 let mut records=Vec::<Value>::new();
 let mut totals=BTreeMap::<String,u64>::new();
 let labels=["f9_on","f9_off","f10_on","f10_off","f8_original","f5_original"];
 let cohort_sources=["F9","F9","F10","F10","F8BASE","F5"];
 let frozen_sources=[f9,f9,f10,f10,f8,f5];
 let mut per_regime=BTreeMap::<String,(i64,i64,i64,i64,i64,u64,u64)>::new();
 for (i,g) in goals.iter().enumerate(){
   let chunk=&arms[6*i..6*i+6];
   for j in 0..6{
     assert_eq!(chunk[j]["source"],cohort_sources[j]);
     assert_eq!(chunk[j]["goals"],g["goals"]);
     assert_eq!(chunk[j]["feedback"],"truth");
     assert_eq!(chunk[j]["patchnext_suppressed"],j==1||j==3);
   }
   let mut outcomes=Vec::<Value>::new();
   for j in 0..6{
     let output=run(frozen_sources[j],&chunk[j],&dir);
     assert!(output["truthful_matches"].as_u64().unwrap_or(0)<=24);
     assert!(output["completed_stages"].as_u64().unwrap_or(0)<=24);
     *totals.entry(labels[j].into()).or_insert(0)+=output["steps"].as_u64().unwrap();
     outcomes.push(output);
   }
   let cost=|j:usize|outcomes[j]["steps"].as_i64().unwrap();
   let qu=|j:usize|outcomes[j]["paid_quarantine_self_edits"].as_array().unwrap().len();
   let regime=g["regime"].as_str().unwrap().to_string();
   let group=per_regime.entry(regime.clone()).or_insert((0,0,0,0,0,0,0));
   group.0+=cost(1)-cost(0);
   group.1+=cost(3)-cost(2);
   group.2+=cost(4)-cost(0);
   group.3+=cost(4)-cost(2);
   group.4+=cost(5)-cost(2);
   group.5+=qu(0) as u64;
   group.6+=qu(2) as u64;
   records.push(json!({
      "regime":regime,"goals":g["goals"],"classification":g["classification"],
      "f9_on":outcomes[0],"f9_off":outcomes[1],
      "f10_on":outcomes[2],"f10_off":outcomes[3],
      "f8_original":outcomes[4],"f5_original":outcomes[5],
      "F9_same_source_physical_edit_net_paid_saving":cost(1)-cost(0),
      "F10_same_source_physical_edit_net_paid_saving":cost(3)-cost(2),
      "F9_vs_old_F8_full_program_paid_saving":cost(4)-cost(0),
      "F10_vs_old_F8_full_program_paid_saving":cost(4)-cost(2),
      "F10_vs_old_F5_full_program_paid_saving":cost(5)-cost(2),
      "F9_original_real_paid_quarantine_edits":qu(0),
      "F10_original_real_paid_quarantine_edits":qu(2)
   }));
 }
 let mut negative=Vec::<Value>::new();
 for arm in controls{
   let source=match arm["source"].as_str().unwrap(){
     "F9"=>f9,"F10"=>f10,"F8BASE"=>f8,"F5"=>f5,other=>panic!("C25 unknown frozen negative source {other}")
   };
   let outcome=run(source,arm,&dir);
   *totals.entry("negative_controls".into()).or_insert(0)+=outcome["steps"].as_u64().unwrap();
   negative.push(outcome);
 }
 assert_eq!(negative.len(),6);
 let total=totals.values().copied().sum::<u64>();
 let groups=per_regime.iter().map(|(regime,v)|json!({
  "regime":regime,"cases":9,
  "f9_same_source_paid_saving":v.0,"f10_same_source_paid_saving":v.1,
  "f9_vs_original_F8_full_program_paid_saving":v.2,
  "f10_vs_original_F8_full_program_paid_saving":v.3,
  "f10_vs_original_F5_full_program_paid_saving":v.4,
  "f9_true_paid_self_quarantine_edits":v.5,
  "f10_true_paid_self_quarantine_edits":v.6
 })).collect::<Vec<_>>();
 fs::write(dir.join("cl1b_c25_original_native_276_worlds.json"),
   serde_json::to_vec_pretty(&json!({
    "study":"CL1B-C25-F9F10-ACTOR-PAID-PATCHNEXT-SELF-QUARANTINE",
    "level":"ORIGINAL_NATIVE_C25_276_WORLD_ACTOR_PAID_SELF_QUARANTINE",
    "source_sha":std::env::var("GITHUB_SHA").unwrap_or_else(|_|"local".into()),
    "records":records,"controls":negative,
    "counts":{"worlds":276,"truthful_worlds":270,"adversarial":6,
      "frozen_goal_sequences":45,"true_stages_per_goal":24,
      "total_original_paid_instructions":total,"arm_paid_totals":totals,
      "regimes":groups},
    "limits":"Researcher-authored finite F9/F10 self-quarantine edited by real charged PID1 PatchNext. Same-source all-PatchNext effect suppression is bundled researcher negative control, not single-PC effect, not endogenous B3/B4."
   })).unwrap()).unwrap();
 println!("ORIGINAL_NATIVE_C25_276_WORLD_PID_PAID_QUARANTINE_TOTALS {:?}",totals);
}
