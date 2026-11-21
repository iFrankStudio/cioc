//! CL1B-REFILL-CAUSAL-R1: 472 frozen original-native single-PID1
//! 24-stage CIL worlds with genuine actor-paid online relative-stride relearning.
use cl0_world::{EventKind,Intervention,Op,Origin,Process,World};
use serde_json::{json,Value};
use std::collections::BTreeMap;
use std::fs::{self,File};
use std::io::{BufWriter,Write};

const STUDY:&str="../../../cl1b/studies/CL1B-REFILL-CAUSAL-R1-frozen-source-identical-pc174-ablation-472-worlds.json";
const CELLS:[i64;12]=[11,20,21,30,32,33,34,35,36,37,38,39];
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
 assert!(source=="F6"||source=="F5");
 assert_eq!(src.len(),if source=="F6"{180}else{153});
 let goals=arm["goals"].as_array().unwrap().iter()
  .map(|x|x.as_i64().unwrap()).collect::<Vec<_>>();
 assert_eq!(goals.len(),24);
 assert!(goals.iter().all(|x|(0_i64..=2).contains(x)));
 let mode=arm["feedback"].as_str().unwrap();
 assert!(matches!(mode,"truth"|"inverted"|"forced0"));
 let suppression=arm["suppression"].as_str().unwrap();
 assert!(matches!(suppression,"none"|"refill174"|"placebo_unreachable180"));
 assert!(source=="F6"||suppression=="none");
 let disabled=false;
 let initial=src.iter().map(decode).collect::<Vec<_>>();
 let mut expected=initial.clone();
 let mut w=World::with_strict_capacity(16,180,300);
 if suppression=="refill174"{w=w.with_native_write_suppression(1,174,38,0);}
 if suppression=="placebo_unreachable180"{w=w.with_native_write_suppression(1,180,38,0);}
 w.load(Process::new(1,initial,CREDIT)).unwrap();
 assert_eq!(w.occupied_units(),src.len()+1);
 let mut trace=BufWriter::new(File::create(dir.join(format!(
  "cl1b_refill_causal_r1_original_{name}.jsonl"))).unwrap());
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
 let mut suppressed_refill=Vec::<Value>::new();
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
    assert!(source=="F6"||source=="F5");
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
    ("F6"|"F5",82)=>(12,137),
    ("F6"|"F5",131)=>(34,132),
    ("F6"|"F5",136)=>(34,132),
    ("F6"|"F5",139)=>(34,106),
    ("F6",155)=>(133,156),
    ("F6",177)=>(133,62),
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
   assert!(source=="F6"||source=="F5");
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
   assert!(source=="F6"||source=="F5");
   match &ev.kind{
    EventKind::CellRead{key:37,value}=>{
     assert_eq!(writes37.last().unwrap()["value"],json!(value));
     reads37.push(json!({"step":paid,"value":value}));
    },
    x=>panic!("missing actor-paid Read37: {x:?}")
   }
  }
  if pc==143{
   assert!(source=="F6"||source=="F5");
   match &ev.kind{
    EventKind::CellRead{key:38,value}=>{
     assert!((0..=1).contains(value));
     reads38.push(json!({"step":paid,"value":value}));
    },
    x=>panic!("missing paid budget Read38 {x:?}")
   }
  }
  if pc==152{
   assert!(source=="F6"||source=="F5");
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
   assert!(source=="F6"||source=="F5");
   assert!(matches!(&ev.kind,EventKind::Executed));
   let meter=w.cell(38).unwrap_or(0);
   let jump=w.process(1).unwrap().pc;
   assert_eq!(jump,if meter==1{140}else{138});
   budget_gates.push(json!({"step":paid,"meter":meter,"next":jump}));
  }
  if pc==174{
   assert_eq!(source,"F6");
   assert_eq!(w.cell(39),Some(0));
   match &ev.kind{
    EventKind::CellWritten{key:38,value}=>{
     assert_eq!(*value,0);
     assert_ne!(suppression,"refill174");
     writes38.push(json!({"step":paid,"value":value}));
    },
    EventKind::NativeCellWriteSuppressed{key:38,value:0}=>{
     assert_eq!(suppression,"refill174");
     assert_eq!(w.cell(38),Some(1),"no host-side quota write");
     suppressed_refill.push(json!({"step":paid,"pc":174,"key":38,"value":0}));
    },
    x=>panic!("unexpected paid refill provenance {x:?}")
   }
  }
  if pc==163{
   assert_eq!(source,"F6");
   match &ev.kind{
    EventKind::CellRead{key:39,value}=>{
     assert!((0..=3).contains(value));
     reads39.push(json!({"step":paid,"value":value}));
    },
    x=>panic!("missing paid clean-streak Read39: {x:?}")
   }
  }
  if [161,167,172].contains(&pc){
   assert_eq!(source,"F6");
   match &ev.kind{
    EventKind::CellWritten{key:39,value}=>{
     assert!(if pc==167{(1..=4).contains(value)}else{*value==0});
     writes39.push(json!({"step":paid,"pc":pc,"value":value}));
    },
    x=>panic!("missing paid clean-streak Write39: {x:?}")
   }
  }
  if pc==158||pc==170{
   assert_eq!(source,"F6");
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
  let pid=w.process(1).unwrap();
  assert_eq!(pid.code.nodes,expected,"unexpected postload source mutation");
  assert_eq!(pid.credits,CREDIT-paid as u64);
  assert_eq!(w.total_compute_inventory(),Some(CREDIT-paid as u64));
  assert!(w.process(2).is_none());
  assert!(w.used_cells()<=16&&w.occupied_units()<=300);
  peak=peak.max(w.occupied_units());
  let provenance=if matches!(&ev.kind,EventKind::NativeCellWriteSuppressed{key:38,value:0}){
   "suppressed_refill"
  }else{"normal"};
  serde_json::to_writer(&mut trace,&json!(["cil",paid,pc,pid.pc,pid.registers,
   pid.credits,CELLS.map(|key|w.cell(key)),w.occupied_units(),get_edges(&w),provenance])).unwrap();
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
 if source=="F6"{
  let ups=writes38.iter().filter(|x|x["value"].as_i64()==Some(1)).count();
  let downs=writes38.iter().filter(|x|x["value"].as_i64()==Some(0)).count();
  assert_eq!(edits122,ups);
  assert!(ups>=downs&&ups<=downs+1);
  assert_eq!(w.cell(38).unwrap_or(0),(ups-downs) as i64);
  assert_eq!(edges.iter().filter(|x|x["pc"]==155).count(),ups);
  assert_eq!(edges.iter().filter(|x|x["pc"]==177).count(),
   downs+suppressed_refill.len());
  assert!(if suppression=="refill174"{ups<=1&&downs==0}else{suppressed_refill.is_empty()});
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
  "feedback":mode,"suppression":suppression,"steps":paid,"services":services,
  "read20_count":observations,"truthful_matches":truthful,
  "completed_stages":w.cell(32),"claim":w.cell(21),
  "code_edits":edits,"edge_edits":edges,
  "write37":writes37,"read37":reads37,
  "write38":writes38,"read38":reads38,"budget_gates":budget_gates,
  "write39":writes39,"read39":reads39,"clean_streak_gates":clean_streak_gates,
  "suppressed_refill":suppressed_refill,
  "online_edit_budget_meter":w.cell(38).unwrap_or(0),
  "final_edges":get_edges(&w),"peak_occupied":peak,
  "original_pid_halted":true,"host_source_edits":0})
}
#[test]
fn frozen_472_original_worlds_selective_paid_refill_causality(){
 let root=std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
 let m:Value=serde_json::from_slice(&fs::read(root.join(STUDY)).unwrap()).unwrap();
 assert_eq!(m["study"],"CL1B-REFILL-CAUSAL-R1");
 assert_eq!(m["status"],"PRE_NATIVE_FROZEN_PC174_SELECTIVE_WRITE_SUPPRESSION_CAUSAL_472_WORLDS");
 let f6=m["original_loaded_CIL_source_f6"].as_array().unwrap();
 let f5=m["baseline_loaded_CIL_source_f5"].as_array().unwrap();
 assert_eq!(f6.len(),180);assert_eq!(f5.len(),153);
 assert_eq!(f6[174],json!({"kind":"Write","address":2,"value":0,"next":175}));
 assert_eq!(f6[173],json!({"kind":"Const","dst":2,"value":38,"next":174}));
 assert_eq!(f6[152],json!({"kind":"Write","address":2,"value":0,"next":153}));
 assert_eq!(f6[133]["next"],62);assert_eq!(f6[177]["kind"],"PatchNext");
 let fresh=m["fresh_cases"].as_array().unwrap();
 let old=m["exploratory_prior_cases"].as_array().unwrap();
 let new_arms=m["arms_new"].as_array().unwrap();
 let old_arms=m["arms_old"].as_array().unwrap();
 let controls=m["controls"].as_array().unwrap();
 assert_eq!(fresh.len(),45);assert_eq!(old.len(),144);
 assert_eq!(new_arms.len(),180);assert_eq!(old_arms.len(),288);
 assert_eq!(controls.len(),4);
 let dir=root.join("../../cl1b-ci-evidence");
 fs::create_dir_all(&dir).unwrap();
 let mut fresh_records=Vec::<Value>::new();
 let mut old_records=Vec::<Value>::new();
 let mut total=0_u64;
 let mut groups=BTreeMap::<String,(u64,i64,i64,i64,u64,u64)>::new();
 let mut refill_exposures=0_usize;
 let mut fresh_exposures=0_usize;
 for (i,g) in fresh.iter().enumerate(){
  let a=&new_arms[4*i];let b=&new_arms[4*i+1];
  let placebo=&new_arms[4*i+2];let f5_arm=&new_arms[4*i+3];
  assert_eq!(a["source"],"F6");assert_eq!(b["source"],"F6");
  assert_eq!(placebo["source"],"F6");assert_eq!(f5_arm["source"],"F5");
  assert_eq!(a["goals"],g["goals"]);assert_eq!(b["goals"],g["goals"]);
  assert_eq!(placebo["goals"],g["goals"]);assert_eq!(f5_arm["goals"],g["goals"]);
  let on=run(f6,a,&dir);let ablated=run(f6,b,&dir);
  let placebo_result=run(f6,placebo,&dir);let baseline=run(f5,f5_arm,&dir);
  for x in [&on,&ablated,&placebo_result,&baseline]{
   assert_eq!(x["claim"],1);assert_eq!(x["completed_stages"],24);
   assert_eq!(x["truthful_matches"],24);
   total+=x["steps"].as_u64().unwrap();
  }
  // An inactive gate must not alter any actor-visible or observer-provenanced outcome.
  let mut placebo_normalized=placebo_result.clone();
  placebo_normalized["name"]=on["name"].clone();
  placebo_normalized["suppression"]=on["suppression"].clone();
  assert_eq!(placebo_normalized,on,"unmatched PC180 placebo changed native outcome");
  assert!(placebo_result["suppressed_refill"].as_array().unwrap().is_empty());
  let exposed=ablated["suppressed_refill"].as_array().unwrap().len();
  assert_eq!(exposed,on["write38"].as_array().unwrap().iter()
     .filter(|x|x["value"]==0).count());
  refill_exposures+=exposed;fresh_exposures+=exposed;
  let cost=ablated["steps"].as_i64().unwrap()-on["steps"].as_i64().unwrap();
  let probes=ablated["services"].as_array().unwrap().len() as i64-
    on["services"].as_array().unwrap().len() as i64;
  let baseline_cost=baseline["steps"].as_i64().unwrap()-on["steps"].as_i64().unwrap();
  let key=g["regime"].as_str().unwrap().to_string();
  let e=groups.entry(key.clone()).or_insert((0,0,0,0,0,0));
  e.0+=1;e.1+=cost;e.2+=probes;e.3+=baseline_cost;e.4+=exposed as u64;
  e.5+=(cost>0) as u64;
  fresh_records.push(json!({"regime":key,"classification":g["classification"],"goals":g["goals"],
   "f6_on":on,"f6_selective_refill_disabled":ablated,
   "f6_unreachable_placebo":placebo_result,"f5_distinct_source":baseline,
   "same_source_refill_causal_paid_saving":cost,
   "same_source_refill_causal_probe_saving":probes,
   "distinct_source_f5_minus_f6_paid_saving":baseline_cost,
   "actual_suppressed_pc174_exposures":exposed}));
 }
 for (i,g) in old.iter().enumerate(){
  let a=&old_arms[2*i];let b=&old_arms[2*i+1];
  assert_eq!(a["goals"],g["goals"]);assert_eq!(b["goals"],g["goals"]);
  assert_eq!(a["source"],"F6");assert_eq!(b["source"],"F6");
  let on=run(f6,a,&dir);let ablated=run(f6,b,&dir);
  for x in [&on,&ablated]{
   assert_eq!(x["claim"],1);assert_eq!(x["completed_stages"],24);
   assert_eq!(x["truthful_matches"],24);
   total+=x["steps"].as_u64().unwrap();
  }
  let exposed=ablated["suppressed_refill"].as_array().unwrap().len();
  assert_eq!(exposed,on["write38"].as_array().unwrap().iter()
    .filter(|x|x["value"]==0).count());
  refill_exposures+=exposed;
  old_records.push(json!({"regime":g["regime"],"classification":g["classification"],
   "goals":g["goals"],"f6_on":on,"f6_selective_refill_disabled":ablated,
   "same_source_refill_causal_paid_saving":
    ablated["steps"].as_i64().unwrap()-on["steps"].as_i64().unwrap(),
   "same_source_refill_causal_probe_saving":
    ablated["services"].as_array().unwrap().len() as i64-
    on["services"].as_array().unwrap().len() as i64,
   "actual_suppressed_pc174_exposures":exposed}));
 }
 let mut adverse=Vec::<Value>::new();
 for arm in controls{
  let out=run(f6,arm,&dir);
  total+=out["steps"].as_u64().unwrap();
  adverse.push(out);
 }
 assert!(adverse[0]["truthful_matches"]<24);
 assert_eq!(adverse[2]["claim"],0);
 let group_report=groups.into_iter().map(|(k,(n,c,p,source,exposure,win))|
  (k,json!({"cases":n,"same_source_refill_causal_paid_saving":c,
   "same_source_refill_causal_probe_saving":p,
   "distinct_source_f5_minus_f6_paid_saving":source,
   "actual_suppressed_pc174_exposures":exposure,
   "original_refill_win_cases":win}))).collect::<serde_json::Map<String,Value>>();
 fs::write(dir.join("cl1b_refill_causal_r1_native_472_worlds.json"),
  serde_json::to_vec_pretty(&json!({
   "study":"CL1B-REFILL-CAUSAL-R1",
   "level":"ORIGINAL_NATIVE_472_WORLD_SOURCE_IDENTICAL_PC174_PAID_WRITE_SUPPRESSION",
   "source_sha":std::env::var("GITHUB_SHA").unwrap_or_else(|_|"local".into()),
   "fresh_records":fresh_records,"exploratory_records":old_records,"controls":adverse,
   "counts":{"worlds":472,"truthful_worlds":468,"adversarial":4,
    "fresh_frozen_cases":45,"previously_exposed_cases":144,
    "actual_suppressed_pc174_exposures":refill_exposures,
    "fresh_suppressed_exposures":fresh_exposures,
    "total_original_paid_instructions":total,"fresh_regimes":group_report},
   "caveat":"Original same-CIL-source PC174 Write38=0 effect-only causal suppression: original private instructions still charged, other CIL edits untouched. Placebo PC180 is unreachable. F5 is a different-source reference, not an isolated causal arm. Researcher-authored threshold, not invented evolution."
  })).unwrap()).unwrap();
}
