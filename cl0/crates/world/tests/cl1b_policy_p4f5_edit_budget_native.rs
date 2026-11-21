//! CL1B-POLICY-P4F5: 434 prospectively frozen original-native single-PID1
//! 24-stage CIL worlds with genuine actor-paid online relative-stride relearning.
use cl0_world::{EventKind,Intervention,Op,Origin,Process,World};
use serde_json::{json,Value};
use std::collections::BTreeMap;
use std::fs::{self,File};
use std::io::{BufWriter,Write};

const STUDY:&str="../../../cl1b/studies/CL1B-POLICY-P4F5-frozen-actor-paid-one-edit-budget-434-world-v1.json";
const CELLS:[i64;11]=[11,20,21,30,32,33,34,35,36,37,38];
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
fn get_edges(w:&World)->[i64;3]{
 let p=w.process(1).unwrap();
 let edge=|n:usize|match &p.code.nodes[n]{
  Op::Write{address:2,value:1,next}=>*next as i64,
  x=>panic!("not native original Write edge {n}: {x:?}")
 };
 let stride=match &p.code.nodes[46]{
  Op::Const{dst:2,value,next:47}=>*value,
  x=>panic!("not native original stride PC46: {x:?}")
 };
 [edge(12),edge(34),stride]
}
fn run(src:&[Value],arm:&Value,dir:&std::path::Path)->Value{
 let name=arm["name"].as_str().unwrap();
 let source=arm["source"].as_str().unwrap();
 assert!(source=="F5"||source=="F4");
 assert_eq!(src.len(),if source=="F5"{153}else{142});
 let goals=arm["goals"].as_array().unwrap().iter()
  .map(|x|x.as_i64().unwrap()).collect::<Vec<_>>();
 assert_eq!(goals.len(),24);
 assert!(goals.iter().all(|x|(0_i64..=2).contains(x)));
 let mode=arm["feedback"].as_str().unwrap();
 assert!(matches!(mode,"truth"|"inverted"|"forced0"));
 let disabled=arm["patchnext_suppressed"].as_bool().unwrap();
 let initial=src.iter().map(decode).collect::<Vec<_>>();
 let mut expected=initial.clone();
 let mut w=World::with_strict_capacity(16,180,300);
 if disabled{w=w.with_native_patchnext_disabled();}
 w.load(Process::new(1,initial,CREDIT)).unwrap();
 assert_eq!(w.occupied_units(),src.len()+1);
 let mut trace=BufWriter::new(File::create(dir.join(format!(
  "cl1b_policy_p4f5_original_{name}.jsonl"))).unwrap());
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
    assert!(source=="F5"||source=="F4");
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
  if [82,131,136,139].contains(&pc){
   let p=w.process(1).unwrap();
   let (target,desired)=match (source,pc){
    ("F5",82)=>(12,137),("F4",82)=>(12,83),
    ("F5",131)=>(34,132),("F4",131)=>(34,106),
    ("F5",136)=>(34,132),("F5",139)=>(34,106),
    _=>unreachable!()
   };
   assert_eq!(p.registers[1],target as i64);
   assert_eq!(p.registers[0],desired as i64);
   let before=match &expected[target]{
    Op::Write{address:2,value:1,next}=>*next,
    _=>panic!("source edit target not paid executable Write"),
   };
   if disabled{
    assert!(matches!(&ev.kind,EventKind::NativeCodeEditSuppressed{opcode_tag:6}));
   }else{
    assert!(matches!(&ev.kind,EventKind::Executed));
    expected[target]=Op::Write{address:2,value:1,next:desired};
   }
   edges.push(json!({"step":paid,"pc":pc,"target":target,
    "before":before,"after":if disabled{before}else{desired},
    "suppressed":disabled}));
  }
  if pc==128||pc==124||pc==133{
   assert!(source=="F5"||source=="F4");
   assert!(pc!=133||source=="F5");
   match &ev.kind{
    EventKind::CellWritten{key:37,value}=>{
     assert_eq!(w.cell(30),Some(*value));
     writes37.push(json!({"step":paid,"pc":pc,"value":value}));
    },
    x=>panic!("missing actor-paid Write37: {x:?}")
   }
  }
  if pc==110{
   assert!(source=="F5"||source=="F4");
   match &ev.kind{
    EventKind::CellRead{key:37,value}=>{
     assert_eq!(writes37.last().unwrap()["value"],json!(value));
     reads37.push(json!({"step":paid,"value":value}));
    },
    x=>panic!("missing actor-paid Read37: {x:?}")
   }
  }
  if pc==143{
   assert_eq!(source,"F5");
   match &ev.kind{
    EventKind::CellRead{key:38,value}=>{
     assert!((0..=1).contains(value));
     reads38.push(json!({"step":paid,"value":value}));
    },
    x=>panic!("missing paid budget Read38 {x:?}")
   }
  }
  if pc==152{
   assert_eq!(source,"F5");
   match &ev.kind{
    EventKind::CellWritten{key:38,value}=>{
     assert_eq!(*value,1);
     assert_eq!(edits.iter().filter(|x|x["pc"]==122).count(),1);
     writes38.push(json!({"step":paid,"value":value}));
    },
    x=>panic!("missing paid meter Write38 {x:?}")
   }
  }
  if pc==146{
   assert_eq!(source,"F5");
   assert!(matches!(&ev.kind,EventKind::Executed));
   let meter=w.cell(38).unwrap_or(0);
   let jump=w.process(1).unwrap().pc;
   assert_eq!(jump,if meter==1{140}else{138});
   budget_gates.push(json!({"step":paid,"meter":meter,"next":jump}));
  }
  if pc==5{
   assert!(matches!(&ev.kind,EventKind::CellRead{key:20,..}));
   observations+=1;
  }
  let pid=w.process(1).unwrap();
  assert_eq!(pid.code.nodes,expected,"unexpected postload source mutation");
  assert_eq!(pid.credits,CREDIT-paid as u64);
  assert_eq!(w.total_compute_inventory(),Some(CREDIT-paid as u64));
  assert!(w.process(2).is_none());
  assert!(w.used_cells()<=16&&w.occupied_units()<=300);
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
 if source=="F5"{
  let edits122=edits.iter().filter(|x|x["pc"]==122).count();
  assert!(edits122<=1);
  assert_eq!(writes38.len(),edits122);
  assert_eq!(w.cell(38).unwrap_or(0),edits122 as i64);
 }else{
  assert!(writes38.is_empty()&&reads38.is_empty()&&budget_gates.is_empty());
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
  "online_edit_budget_meter":w.cell(38).unwrap_or(0),
  "final_edges":get_edges(&w),"peak_occupied":peak,
  "original_pid_halted":true,"host_source_edits":0})
}
#[test]
fn p4f5_original_native_434_worlds_paid_edit_budget(){
 let root=std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
 let m:Value=serde_json::from_slice(&fs::read(root.join(STUDY)).unwrap()).unwrap();
 assert_eq!(m["study"],"CL1B-POLICY-P4F5");
 assert_eq!(m["status"],"PRE_NATIVE_FROZEN_ACTOR_PAID_ONLINE_SOURCE_EDIT_BUDGET_ONE_AND_FOUR_NEW_REGIMES");
 let f5=m["original_native_CIL_source_f5"].as_array().unwrap();
 let f4=m["baseline_source_f4"].as_array().unwrap();
 assert_eq!(f5.len(),153);assert_eq!(f4.len(),142);
 assert_eq!(f5[82],json!({"kind":"PatchNext","node":1,"target":0,"next":127}));
 assert_eq!(f5[137]["next"],142);
 assert_eq!(f5[122]["next"],147);
 assert_eq!(f5[143],json!({"kind":"Read","dst":0,"address":2,"next":144}));
 assert_eq!(f5[146],json!({"kind":"Branch","test":0,"nonzero":138,"zero":140}));
 assert_eq!(f5[152],json!({"kind":"Write","address":2,"value":0,"next":134}));
 assert_eq!(f5[81]["value"],137);
 assert_eq!(f5[130]["value"],132);
 assert_eq!(f4[122]["next"],134);
 assert_eq!(f5[136],json!({"kind":"PatchNext","node":1,"target":0,"next":123}));
 assert_eq!(f5[139],json!({"kind":"PatchNext","node":1,"target":0,"next":140}));
 assert_eq!(f5[131],json!({"kind":"PatchNext","node":1,"target":0,"next":46}));
 let goals=m["goal_cases"].as_array().unwrap();
 let arms=m["arms"].as_array().unwrap();
 let controls=m["controls"].as_array().unwrap();
 assert_eq!(goals.len(),144);assert_eq!(arms.len(),432);assert_eq!(controls.len(),2);
 let dir=root.join("../../cl1b-ci-evidence");
 fs::create_dir_all(&dir).unwrap();
 let mut records=Vec::<Value>::new();
 let mut total=0_u64;
 let mut grouped=BTreeMap::<String,(i64,i64,i64,i64,u64,u64)>::new();
 for (index,g) in goals.iter().enumerate(){
  let (a,b,c)=(&arms[3*index],&arms[3*index+1],&arms[3*index+2]);
  assert_eq!(a["source"],"F5");assert_eq!(b["source"],"F5");
  assert_eq!(c["source"],"F4");
  assert_eq!(a["goals"],g["goals"]);assert_eq!(b["goals"],g["goals"]);
  assert_eq!(c["goals"],g["goals"]);
  let on=run(f5,a,&dir);let off=run(f5,b,&dir);let baseline=run(f4,c,&dir);
  for outcome in [&on,&off,&baseline]{
   assert_eq!(outcome["claim"],1);
   assert_eq!(outcome["completed_stages"],24);
   assert_eq!(outcome["truthful_matches"],24);
   total+=outcome["steps"].as_u64().unwrap();
  }
  assert_eq!(on["final_edges"][1],132);
  assert_eq!(off["final_edges"][1],62);
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
   "goals":g["goals"],"f5_on":on,"f5_off":off,"f4_on":baseline,
   "same_source_bundle_paid_saving":paired,
   "same_source_bundle_probe_saving":paired_probes,
   "cross_source_f5_vs_f4_paid_saving":cross,
   "cross_source_f5_vs_f4_probe_saving":cross_probes}));
 }
 let mut adverse=Vec::<Value>::new();
 for arm in controls{
  let result=run(f5,arm,&dir);
  total+=result["steps"].as_u64().unwrap();
  adverse.push(result);
 }
 assert!(adverse[0]["truthful_matches"].as_u64().unwrap()<24);
 assert_eq!(adverse[1]["claim"],0);
 let summaries=grouped.into_iter().map(|(k,(p,pp,c,cp,n,e))|
  (k,json!({"cases":n,"causal_bundle_paid_saving":p,
   "causal_bundle_probes_saved":pp,"cross_source_paid_saving":c,
   "cross_source_probes_saved":cp,"online_actor_edits":e})))
  .collect::<serde_json::Map<String,Value>>();
 fs::write(dir.join("cl1b_policy_p4f5_native_434_worlds.json"),
  serde_json::to_vec_pretty(&json!({"study":"CL1B-POLICY-P4F5",
   "level":"ORIGINAL_NATIVE_434_WORLD_ACTOR_PAID_ONE_ONLINE_EDIT_BUDGET",
   "source_sha":std::env::var("GITHUB_SHA").unwrap_or_else(|_|"local".into()),
   "records":records,"controls":adverse,
   "counts":{"worlds":434,"truthful_worlds":432,"adversarial":2,"frozen_goal_sequences":144,"fresh_heldout_goal_sequences":36,
    "total_original_paid_instructions":total,"regimes":summaries},
   "caveat":"Researcher-authored one-online-edit hard budget; same-source PatchNext-on/off bundles multiple effects; F5 vs F4 is a different-source performance contrast. Paid fallback PatchConst PC45/88/102 is not part of online PC122 budget."})).unwrap()).unwrap();
}
