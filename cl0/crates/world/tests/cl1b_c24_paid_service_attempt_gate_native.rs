//! CL1B-C24: paid 195-CIL cumulative task-service edit gate in 186 original native one-PID1 worlds
//! 24-stage CIL worlds with genuine actor-paid online relative-stride relearning.
use cl0_world::{EventKind,Intervention,Op,Origin,Process,World};
use serde_json::{json,Value};
use std::collections::BTreeMap;
use std::fs::{self,File};
use std::io::{BufWriter,Write};

const PARENT:&str="../../../cl1b/studies/CL1B-POLICY-P4F6-frozen-actor-paid-clean-success-refill-542-world-v1.json";
const STUDY:&str="../../../cl1b/studies/CL1B-C24-PAID-SERVICE-COUNT-EDIT-GATE-frozen-186-worlds-v1.json";
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
fn get_edges(w:&World)->[i64;4]{
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
 [edge(12),edge(34),stride,monitor]
}
fn run(src:&[Value],arm:&Value,dir:&std::path::Path)->Value{
 let name=arm["name"].as_str().unwrap();
 let source=arm["source"].as_str().unwrap();
 assert!(source=="F6"||source=="F6BASE"||source=="F5");
 assert_eq!(src.len(),if source=="F6"{195}else if source=="F6BASE"{180}else{153});
 let goals=arm["goals"].as_array().unwrap().iter()
  .map(|x|x.as_i64().unwrap()).collect::<Vec<_>>();
 assert_eq!(goals.len(),24);
 assert!(goals.iter().all(|x|(0_i64..=2).contains(x)));
 let mode=arm["feedback"].as_str().unwrap();
 assert!(matches!(mode,"truth"|"inverted"|"forced0"));
 let disabled=arm["patchnext_suppressed"].as_bool().unwrap();
 let initial=src.iter().map(decode).collect::<Vec<_>>();
 let mut expected=initial.clone();
 let mut w=World::with_strict_capacity(17,195,320);
 if disabled{w=w.with_native_patchnext_disabled();}
 w.load(Process::new(1,initial,CREDIT)).unwrap();
 assert_eq!(w.occupied_units(),src.len()+1);
 let mut trace=BufWriter::new(File::create(dir.join(format!(
  "cl1b_c24_original_{name}.jsonl"))).unwrap());
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
 let mut paid_stage_reads=Vec::<Value>::new();
 let mut paid_stage_branches=Vec::<Value>::new();
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
    assert!(source=="F6"||source=="F6BASE"||source=="F5");
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
    ("F6"|"F6BASE"|"F5",82)=>(12,137),
    ("F6"|"F6BASE"|"F5",131)=>(34,132),
    ("F6"|"F6BASE"|"F5",136)=>(34,132),
    ("F6"|"F6BASE"|"F5",139)=>(34,106),
    ("F6"|"F6BASE",155)=>(133,156),
    ("F6"|"F6BASE",177)=>(133,62),
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
   assert!(source=="F6"||source=="F6BASE"||source=="F5");
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
   assert!(source=="F6"||source=="F6BASE"||source=="F5");
   match &ev.kind{
    EventKind::CellRead{key:37,value}=>{
     assert_eq!(writes37.last().unwrap()["value"],json!(value));
     reads37.push(json!({"step":paid,"value":value}));
    },
    x=>panic!("missing actor-paid Read37: {x:?}")
   }
  }
  if pc==143{
   assert!(source=="F6"||source=="F6BASE"||source=="F5");
   match &ev.kind{
    EventKind::CellRead{key:38,value}=>{
     assert!((0..=1).contains(value));
     reads38.push(json!({"step":paid,"value":value}));
    },
    x=>panic!("missing paid budget Read38 {x:?}")
   }
  }
  if pc==152{
   assert!(source=="F6"||source=="F6BASE"||source=="F5");
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
   assert!(source=="F6"||source=="F6BASE"||source=="F5");
   assert!(matches!(&ev.kind,EventKind::Executed));
   let meter=w.cell(38).unwrap_or(0);
   let jump=w.process(1).unwrap().pc;
   assert_eq!(jump,if meter==1{140}else{138});
   budget_gates.push(json!({"step":paid,"meter":meter,"next":jump}));
  }
  if pc==174{
   assert!(source=="F6"||source=="F6BASE");
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
   assert!(source=="F6"||source=="F6BASE");
   match &ev.kind{
    EventKind::CellRead{key:39,value}=>{
     assert!((0..=3).contains(value));
     reads39.push(json!({"step":paid,"value":value}));
    },
    x=>panic!("missing paid clean-streak Read39: {x:?}")
   }
  }
  if [161,167,172].contains(&pc){
   assert!(source=="F6"||source=="F6BASE");
   match &ev.kind{
    EventKind::CellWritten{key:39,value}=>{
     assert!(if pc==167{(1..=4).contains(value)}else{*value==0});
     writes39.push(json!({"step":paid,"pc":pc,"value":value}));
    },
    x=>panic!("missing paid clean-streak Write39: {x:?}")
   }
  }
  if pc==158||pc==170{
   assert!(source=="F6"||source=="F6BASE");
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
  if pc==181{
   assert_eq!(source,"F6");
   match &ev.kind{
    EventKind::CellRead{key:32,value}=>{
      assert_eq!(*value,w.cell(32).unwrap_or(0));
      paid_stage_reads.push(json!({"step":paid,"stage":value}));
    },
    other=>panic!("C23 charged Read32 missing: {other:?}")
   }
  }
  if pc==182{
   assert_eq!(source,"F6");
   assert!(matches!(&ev.kind,EventKind::Executed));
   let p=w.process(1).unwrap();
   let stage=p.registers[0];
   assert_eq!(stage,w.cell(32).unwrap_or(0));
   assert_eq!(p.pc,if stage!=0{183}else{127});
   paid_stage_branches.push(json!({"step":paid,"stage":stage,"next":p.pc}));
  }
  let pid=w.process(1).unwrap();
  assert_eq!(pid.code.nodes,expected,"unexpected postload source mutation");
  assert_eq!(pid.credits,CREDIT-paid as u64);
  assert_eq!(w.total_compute_inventory(),Some(CREDIT-paid as u64));
  assert!(w.process(2).is_none());
  assert!(w.used_cells()<=17&&w.occupied_units()<=320);
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
 if source=="F6"||source=="F6BASE"{
  let ups=writes38.iter().filter(|x|x["value"].as_i64()==Some(1)).count();
  let downs=writes38.iter().filter(|x|x["value"].as_i64()==Some(0)).count();
  assert_eq!(edits122,ups);
  assert!(ups>=downs&&ups<=downs+1);
  assert_eq!(w.cell(38).unwrap_or(0),(ups-downs) as i64);
  assert_eq!(edges.iter().filter(|x|x["pc"]==155).count(),ups);
  assert_eq!(edges.iter().filter(|x|x["pc"]==177).count(),downs);
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
   "paid_stage_reads":paid_stage_reads,"paid_stage_branches":paid_stage_branches,
  "online_edit_budget_meter":w.cell(38).unwrap_or(0),
  "final_edges":get_edges(&w),"peak_occupied":peak,
  "original_pid_halted":true,"host_source_edits":0})
}
#[test]
fn c24_186_original_native_paid_service_attempt_gate(){
 let root=std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
 let m:Value=serde_json::from_slice(&fs::read(root.join(STUDY)).unwrap()).unwrap();
 let parent:Value=serde_json::from_slice(&fs::read(root.join(PARENT)).unwrap()).unwrap();
 assert_eq!(m["study"],"CL1B-C24-F8-PAID-CUMULATIVE-SERVICE-PROXY-EDIT-GATE");
 assert_eq!(m["status"],"PRE_NATIVE_FROZEN_NEW_GOALS_FULL_186_WORLD");
 assert_eq!(parent["study"],"CL1B-POLICY-P4F6");
 assert_eq!(parent["status"],"PRE_NATIVE_FROZEN_ACTOR_PAID_FOUR_CLEAN_SUCCESS_REPLENISHABLE_ONE_ONLINE_EDIT_BUDGET");
 let f6=m["exact_frozen_F8_source_195_nodes"].as_array().unwrap();
 let f6base=parent["original_native_CIL_source_f6"].as_array().unwrap();
 let f5=parent["baseline_source_f5"].as_array().unwrap();
 assert_eq!(f6.len(),195);assert_eq!(f6base.len(),180);assert_eq!(f5.len(),153);
 assert_eq!(f6[3]["next"],184);assert_eq!(f6[81]["next"],180);
 assert_eq!(f6base[3]["next"],4);assert_eq!(f6base[81]["next"],82);
 for i in 0..180{if i!=3&&i!=81{assert_eq!(f6[i],f6base[i],"unregistered original source node alteration");}}
 assert_eq!(f6[180],json!({"kind":"Const","dst":2,"value":40,"next":181}));
 assert_eq!(f6[181],json!({"kind":"Read","dst":0,"address":2,"next":182}));
 assert_eq!(f6[182],json!({"kind":"Const","dst":2,"value":2,"next":183}));
 assert_eq!(f6[183],json!({"kind":"Sub","dst":0,"lhs":0,"rhs":2,"next":190}));
 assert_eq!(f6[184],json!({"kind":"Const","dst":2,"value":40,"next":185}));
 assert_eq!(f6[185],json!({"kind":"Read","dst":0,"address":2,"next":186}));
 assert_eq!(f6[189],json!({"kind":"Write","address":2,"value":0,"next":4}));
 assert_eq!(f6[190],json!({"kind":"Branch","test":0,"nonzero":192,"zero":191}));
 assert_eq!(f6[191],json!({"kind":"Const","dst":0,"value":137,"next":82}));
 assert_eq!(f6[194],json!({"kind":"Branch","test":0,"nonzero":127,"zero":191}));
 assert_eq!(f6[82],json!({"kind":"PatchNext","node":1,"target":0,"next":127}));
 assert_eq!(f6[137]["next"],142);
 assert_eq!(f6[122]["next"],147);
 assert_eq!(f6[143],json!({"kind":"Read","dst":0,"address":2,"next":144}));
 assert_eq!(f6[146],json!({"kind":"Branch","test":0,"nonzero":138,"zero":140}));
 assert_eq!(f6[152],json!({"kind":"Write","address":2,"value":0,"next":153}));
 assert_eq!(f5[152]["next"],134);
 assert_eq!(f6[133]["next"],62);
 assert_eq!(f6[155],json!({"kind":"PatchNext","node":1,"target":0,"next":134}));
 assert_eq!(f6[177],json!({"kind":"PatchNext","node":1,"target":0,"next":178}));
 assert_eq!(f6[170],json!({"kind":"Branch","test":0,"nonzero":62,"zero":171}));
 assert_eq!(f6[81]["value"],137);
 assert_eq!(f6[130]["value"],132);
 assert_eq!(f5[122]["next"],147);
 assert_eq!(f6[136],json!({"kind":"PatchNext","node":1,"target":0,"next":123}));
 assert_eq!(f6[139],json!({"kind":"PatchNext","node":1,"target":0,"next":140}));
 assert_eq!(f6[131],json!({"kind":"PatchNext","node":1,"target":0,"next":46}));
 let goals=m["goal_cases"].as_array().unwrap();
 let arms=m["arms"].as_array().unwrap();
 let controls=m["controls"].as_array().unwrap();
 assert_eq!(goals.len(),45);assert_eq!(arms.len(),180);assert_eq!(controls.len(),6);
 let c22:Value=serde_json::from_slice(&fs::read(root.join("../../../cl1b/studies/CL1B-C22-PAID-PATCHNEXT-SELF-EDIT-RETURN-frozen-141-worlds-v1.json")).unwrap()).unwrap();
 let mut old=std::collections::BTreeSet::new();
 for g in parent["goal_cases"].as_array().unwrap(){old.insert(g["goals"].to_string());}
 for g in c22["goal_cases"].as_array().unwrap(){old.insert(g["goals"].to_string());}
 let c23:Value=serde_json::from_slice(&fs::read(root.join("../../../cl1b/studies/CL1B-C23-PAID-STAGE-GATE-frozen-186-worlds-v1.json")).unwrap()).unwrap();
 for g in c23["goal_cases"].as_array().unwrap(){old.insert(g["goals"].to_string());}
 let mut fresh=std::collections::BTreeSet::new();
 for g in goals{
   assert_eq!(g["goals"].as_array().unwrap().len(),24);
   assert!(!old.contains(&g["goals"].to_string()),"C23 goal reused old");
   assert!(fresh.insert(g["goals"].to_string()),"C23 goal duplicated");
 }
 assert_eq!(fresh.len(),45);
 let dir=root.join("../../cl1b-ci-evidence");
 fs::create_dir_all(&dir).unwrap();
 let mut records=Vec::<Value>::new();
 let mut total=0_u64;
 let mut grouped=BTreeMap::<String,(i64,i64,i64,i64,u64,u64)>::new();
 for (index,g) in goals.iter().enumerate(){
  let (a,b,c,d)=(&arms[4*index],&arms[4*index+1],&arms[4*index+2],&arms[4*index+3]);
  assert_eq!(a["source"],"F6");assert_eq!(b["source"],"F6");
  assert_eq!(c["source"],"F6BASE");assert_eq!(d["source"],"F5");
  assert_eq!(a["goals"],g["goals"]);assert_eq!(b["goals"],g["goals"]);
  assert_eq!(c["goals"],g["goals"]);assert_eq!(d["goals"],g["goals"]);
  let on=run(f6,a,&dir);let off=run(f6,b,&dir);let original=run(f6base,c,&dir);let baseline=run(f5,d,&dir);
  for outcome in [&on,&off,&original,&baseline]{
   assert!(outcome["completed_stages"].as_u64().unwrap_or(0)<=24);
   assert!(outcome["truthful_matches"].as_u64().unwrap_or(0)<=24);
   total+=outcome["steps"].as_u64().unwrap();
  }
  // Zero exposure, bad accuracy, or negative edit ROI are valid: preserve them.
  let paired=off["steps"].as_i64().unwrap()-on["steps"].as_i64().unwrap();
  let paired_probes=off["services"].as_array().unwrap().len() as i64-
    on["services"].as_array().unwrap().len() as i64;
  let cross=baseline["steps"].as_i64().unwrap()-on["steps"].as_i64().unwrap();
  let cross_probes=baseline["services"].as_array().unwrap().len() as i64-
    on["services"].as_array().unwrap().len() as i64;
  let name=g["regime"].as_str().unwrap().to_string();
  let entry=grouped.entry(name.clone()).or_insert((0,0,0,0,0,0));
  entry.0+=paired;entry.1+=paired_probes;entry.2+=cross;entry.3+=cross_probes;
  entry.4+=1;
  entry.5+=on["code_edits"].as_array().unwrap().iter()
   .filter(|x|x["pc"]==122).count() as u64;
  records.push(json!({"regime":name,"classification":g["classification"],
   "goals":g["goals"],"f7_on":on,"f7_off":off,"f6_original":original,"f5_original":baseline,
    "F7_on_minus_F6_original_paid_CIL":on["steps"].as_i64().unwrap()-original["steps"].as_i64().unwrap(),
    "F7_on_minus_F5_original_paid_CIL":on["steps"].as_i64().unwrap()-baseline["steps"].as_i64().unwrap(),
   "same_source_bundle_paid_saving":paired,
   "same_source_bundle_probe_saving":paired_probes,
   "cross_source_f6_vs_f5_paid_saving":cross,
   "cross_source_f6_vs_f5_probe_saving":cross_probes}));
 }
 let mut adverse=Vec::<Value>::new();
 for arm in controls{
  let result=run(if arm["source"]=="F5"{f5}else if arm["source"]=="F6BASE"{f6base}else{f6},arm,&dir);
  total+=result["steps"].as_u64().unwrap();
  adverse.push(result);
 }
 // Preserve all adversarial true-truth outcomes, even if self-claim is wrong.
 let summaries=grouped.into_iter().map(|(k,(p,pp,c,cp,n,e))|
  (k,json!({"cases":n,"causal_bundle_paid_saving":p,
   "causal_bundle_probes_saved":pp,"cross_source_paid_saving":c,
   "cross_source_probes_saved":cp,"online_actor_edits":e})))
  .collect::<serde_json::Map<String,Value>>();
 fs::write(dir.join("cl1b_c24_original_native_186_worlds.json"),
  serde_json::to_vec_pretty(&json!({"study":"CL1B-C24-F8-PAID-CUMULATIVE-SERVICE-PROXY-EDIT-GATE",
   "level":"ORIGINAL_NATIVE_C24_186_WORLD_PID_PAID_SERVICE_COUNTER_GATE",
   "source_sha":std::env::var("GITHUB_SHA").unwrap_or_else(|_|"local".into()),
   "records":records,"controls":adverse,
   "counts":{"worlds":186,"truthful_worlds":180,"adversarial":6,"frozen_goal_sequences":45,"fresh_heldout_goal_sequences":45,
    "total_original_paid_instructions":total,"regimes":summaries},
   "caveat":"Researcher-authored F8 195-node fully paid service count edit gate, not earned ROI; same-source PatchNext effects bundled, original F6/F5 distinct source."})).unwrap()).unwrap();
}
