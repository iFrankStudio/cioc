//! CL1B-REFILL-RULE-G2: 184 source-frozen original Rust VM worlds.
//! Same PID1 actually pays PatchConst PC28 to mutate its future PC173 decision
//! parameter; future PC175 decisions are native, not host-picked.
use cl0_world::{EventKind,Intervention,Op,Origin,Process,World};
use serde_json::{json,Value};
use std::fs::{self,File};
use std::io::{BufWriter,Write};

const MANIFEST:&str="../../../cl1b/studies/CL1B-REFILL-RULE-G2-frozen-180-node-self-modifying-184-world-v1.json";
const R1:&str="../../../cl1b/studies/CL1B-REFILL-CAUSAL-R1-frozen-source-identical-pc174-ablation-472-worlds.json";
const CREDIT:u64=6000;
const CELLS:[i64;12]=[11,20,21,30,32,33,34,35,36,37,38,39];

fn decoded(v:&Value)->Op{
 let tag=v["kind"].as_str().unwrap();
 let raw=if tag=="Halt"{json!("Halt")}else{
   let mut map=v.as_object().unwrap().clone();
   map.remove("kind");
   let mut obj=serde_json::Map::new();
   obj.insert(tag.to_string(),Value::Object(map));
   Value::Object(obj)
 };
 serde_json::from_value(raw).expect("G2 frozen original native CIL source")
}
fn source<'a>(m:&'a Value,name:&str)->&'a Vec<Value>{
 m[match name{
  "G2_changed_allow"=>"source_g2_changed_allow",
  "G2_changed_deny"=>"source_g2_changed_deny",
  "G2_always"=>"source_g2_always",
  "G2_never"=>"source_g2_never",
  _=>panic!("unknown G2 source: {name}"),
 }].as_array().unwrap()
}
fn theta(program:&[Op])->i64 {
 match &program[173]{
  Op::Const{dst:2,value,next:174}=>*value,
  x=>panic!("PC173 not mutable Const: {x:?}")
 }
}
fn edges(w:&World)->[i64;4]{
 let p=w.process(1).unwrap();
 let edge=|pc:usize|match &p.code.nodes[pc]{
   Op::Write{next,..}=>*next as i64,
   x=>panic!("PC{pc} expected Write, got {x:?}"),
 };
 let stride=match &p.code.nodes[46]{
  Op::Const{dst:2,value,next:47}=>*value,
  x=>panic!("PC46 wrong stride source: {x:?}")
 };
 [edge(12),edge(34),stride,edge(133)]
}
fn run(m:&Value,arm:&Value,dir:&std::path::Path)->Value{
 let name=arm["name"].as_str().unwrap();
 let source_name=arm["source"].as_str().unwrap();
 let mode=arm["feedback"].as_str().unwrap();
 assert!(matches!(mode,"truth"|"inverted"|"forced0"));
 let goals=arm["goals"].as_array().unwrap().iter()
    .map(|g|g.as_i64().unwrap()).collect::<Vec<_>>();
 assert_eq!(goals.len(),24);
 assert!(goals.iter().all(|g|(0..=2).contains(g)));
 let initial=source(m,source_name).iter().map(decoded).collect::<Vec<_>>();
 assert_eq!(initial.len(),180);
 let mut expected=initial.clone();
 let mut w=World::with_strict_capacity(16,180,300);
 w.load(Process::new(1,initial,CREDIT)).unwrap();
 assert_eq!(w.occupied_units(),181);
 let file=File::create(dir.join(format!("cl1b_refill_rule_g2_original_{name}.jsonl"))).unwrap();
 let mut trace=BufWriter::new(file);
 let mut paid=0_u64;
 let mut peak=181_usize;
 let mut service=Vec::<Value>::new();
 let mut decisions=Vec::<Value>::new();
 let mut refills=Vec::<Value>::new();
 let mut parameter_patches=Vec::<Value>::new();
 let mut paid_code_edits=Vec::<Value>::new();
 let mut later_mutated_reads=Vec::<Value>::new();
 let mut last_changed:Option<i64>=None;
 let mut failed_third_probe_halt=false;
 while w.step(){
  paid+=1;
  assert!(paid<CREDIT,"private compute exhausted before original CIL Halt");
  let event=w.events().last().unwrap().clone();
  assert!(matches!(event.origin,Origin::Internal(1)));
  let pc=event.pc;
  let pid=w.process(1).unwrap();
  assert_eq!(pid.credits,CREDIT-paid);
  assert_eq!(w.total_compute_inventory(),Some(CREDIT-paid));
  assert!(w.process(2).is_none());
  assert!(w.used_cells()<=16&&w.occupied_units()<=300);
  assert_eq!(pid.code.nodes.len(),180);
  if [45,88,102,122,28].contains(&pc){
   assert!(matches!(&event.kind,EventKind::Executed));
   let target=pid.registers[1] as usize;
   let value=if pc==28{pid.registers[3]}else{pid.registers[0]};
   assert_eq!(target,if pc==28{173}else{46});
   let previous=match &expected[target]{
     Op::Const{value,..}=>*value,
     x=>panic!("invalid paid PatchConst destination: {x:?}"),
   };
   if let Op::Const{value:field,..}=&mut expected[target]{*field=value;}
   if pc==28{
    assert!((0..=2).contains(&value));
    parameter_patches.push(json!({"step":paid,"pc":28,"target":173,
      "before":previous,"after":value,"changed":previous!=value}));
    if previous!=value{last_changed=Some(value);}
   }
   paid_code_edits.push(json!({"step":paid,"pc":pc,
     "target":target,"before":previous,"after":value,"kind":"PatchConst"}));
  }
  if [82,131,136,139,155,26].contains(&pc){
   assert!(matches!(&event.kind,EventKind::Executed));
   let target=pid.registers[1] as usize;
   let value=pid.registers[0] as usize;
   assert!([12,34,133].contains(&target));
   assert!(value<180);
   let before=match &expected[target]{
    Op::Write{next,..}=>*next,
    x=>panic!("invalid paid PatchNext target: {x:?}"),
   };
   if let Op::Write{next,..}=&mut expected[target]{*next=value;}
   if pc==26{assert_eq!((target,value),(133,62));}
   paid_code_edits.push(json!({"step":paid,"pc":pc,"target":target,
     "before":before,"after":value,"kind":"PatchNext"}));
  }
  assert_eq!(pid.code.nodes,expected,"unpaid or unaccounted native source mutation");
  if pc==173{
   assert!(matches!(&event.kind,EventKind::Executed));
   assert_eq!(pid.registers[2],theta(&expected));
   if let Some(v)=last_changed{
    assert_eq!(v,pid.registers[2]);
    later_mutated_reads.push(json!({"step":paid,"theta":v}));
    last_changed=None;
   }
  }
  if pc==175{
   assert!(matches!(&event.kind,EventKind::Executed));
   let difference=pid.registers[0];
   let next=pid.pc;
   let expected_jump=match source_name{
    "G2_changed_allow"=>if difference!=0{176}else{179},
    "G2_changed_deny"=>if difference!=0{179}else{176},
    "G2_always"=>176,
    "G2_never"=>179,
    _=>unreachable!()
   };
   assert_eq!(next,expected_jump);
   decisions.push(json!({"step":paid,"difference":difference,"next":next,
     "theta":theta(&expected),"last_guess":pid.registers[3]}));
  }
  if pc==178{
   assert!(matches!(&event.kind,EventKind::CellWritten{key:38,value:0}));
   assert_eq!(w.cell(38),Some(0));
   refills.push(json!({"step":paid,"pc":178}));
  }
  if pc==15 && pid.pc==61{
   assert_eq!(pid.registers[0],0);
   failed_third_probe_halt=true;
  }
  peak=peak.max(w.occupied_units());
  let label=if pc==28{"paid_parameter_patch"}else{"normal"};
  serde_json::to_writer(&mut trace,&json!(["cil",paid,pc,pid.pc,pid.registers,
   pid.credits,CELLS.map(|k|w.cell(k)),w.occupied_units(),edges(&w),
   theta(&expected),label])).unwrap();
  trace.write_all(b"\n").unwrap();
  if pc==3{
   let guess=match &event.kind{
    EventKind::CellWritten{key:11,value}=>*value,
    x=>panic!("task service must originate from paid Write11: {x:?}"),
   };
   let stage=w.cell(32).unwrap_or(0) as usize;
   assert!(stage<24);
   let truth=i64::from(guess==goals[stage]);
   let feedback=match mode{
    "truth"=>truth,"inverted"=>1-truth,"forced0"=>0,_=>unreachable!()
   };
   service.push(json!({"step":paid,"stage":stage,"guess":guess,
    "truth":truth,"feedback":feedback}));
   w.intervene(Intervention::SetCell{key:20,value:feedback}).unwrap();
   assert!(matches!(w.events().last().unwrap().origin,Origin::External));
   peak=peak.max(w.occupied_units());
   serde_json::to_writer(&mut trace,&json!(["feedback",paid,stage,goals[stage],
     guess,truth,feedback,CELLS.map(|k|w.cell(k)),w.occupied_units(),
     edges(&w),theta(&expected)])).unwrap();
   trace.write_all(b"\n").unwrap();
  }
 }
 trace.flush().unwrap();
 let pid=w.process(1).unwrap();
 assert!(pid.stopped,"native G2 must terminate normally");
 assert!(paid<CREDIT);
 let truth=service.iter().filter(|e|e["truth"]==1).count();
 let completed=w.cell(32).unwrap_or(0);
 let claim=w.cell(21).unwrap_or(0);
 if mode=="truth"{
   assert_eq!(completed,24,"incomplete actual native task stage");
   assert_eq!(truth,24,"wrong native hidden task count");
   assert_eq!(claim,1,"truthful native task completion without valid claim");
 }
 if mode=="forced0"{
  assert!(failed_third_probe_halt,"PC15 false-feedback fail-closed route untested");
  assert_eq!(completed,0);
  assert_eq!(claim,0);
 }
 if mode=="inverted"{
  assert_eq!(claim,1,"adversarial test should exercise false success claim");
  assert!(truth<service.len(),"inverted feedback exposed no incorrect probes");
 }
 json!({"name":name,"source":source_name,"regime":arm["regime"],
  "feedback":mode,"goals":goals,"steps":paid,"services":service,
  "decisions":decisions,"refills":refills,"parameter_patches":parameter_patches,
  "paid_code_edits":paid_code_edits,"later_mutated_reads":later_mutated_reads,
  "completed_stages":completed,"truthful_matches":truth,"claim":claim,
  "final_theta":theta(&expected),"final_edges":edges(&w),
  "peak_occupied":peak,"failed_third_probe_halt":failed_third_probe_halt,
  "host_source_edits":0})
}
#[test]
fn g2_frozen_184_worlds_actor_paid_mutable_rule(){
 let root=std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
 let m:Value=serde_json::from_slice(&fs::read(root.join(MANIFEST)).unwrap()).unwrap();
 let r1:Value=serde_json::from_slice(&fs::read(root.join(R1)).unwrap()).unwrap();
 assert_eq!(m["study"],"CL1B-REFILL-RULE-G2");
 assert_eq!(m["status"],"PRE_NATIVE_FROZEN");
 assert_eq!(m["source_f6_frozen_parent"],r1["original_loaded_CIL_source_f6"]);
 assert_eq!(m["counts"]["original_native_worlds"],184);
 let allowed:[usize;14]=[15,25,26,27,28,171,172,173,174,175,176,177,178,179];
 for name in ["G2_changed_allow","G2_changed_deny","G2_always","G2_never"]{
  let g=source(&m,name);
  let original=m["source_f6_frozen_parent"].as_array().unwrap();
  assert_eq!(g.len(),180);
  for (pc,(after,before)) in g.iter().zip(original).enumerate(){
    if !allowed.contains(&pc){assert_eq!(after,before,"non-preregistered source edit PC{pc}");}
  }
  assert_eq!(g[15]["zero"],61);
  assert_eq!(g[28],json!({"kind":"PatchConst","node":1,"value":3,"next":125}));
  assert_eq!(g[173],json!({"kind":"Const","dst":2,"value":0,"next":174}));
 }
 for a in ["G2_changed_allow","G2_changed_deny","G2_always","G2_never"]{
  for b in ["G2_changed_allow","G2_changed_deny","G2_always","G2_never"]{
   for (pc,(x,y)) in source(&m,a).iter().zip(source(&m,b)).enumerate(){
    if pc!=175{assert_eq!(x,y,"G2 arms may only differ at PC175");}
   }
  }
 }
 let all=m["arms"].as_array().unwrap();
 let controls=m["controls"].as_array().unwrap();
 assert_eq!(all.len(),180);
 assert_eq!(controls.len(),4);
 let dir=root.join("../../cl1b-ci-evidence");
 fs::create_dir_all(&dir).unwrap();
 let mut truth_results=Vec::new();
 let mut control_results=Vec::new();
 let mut paid_total=0_u64;
 for arm in all{
  let result=run(&m,arm,&dir);
  paid_total+=result["steps"].as_u64().unwrap();
  truth_results.push(result);
 }
 for arm in controls{
  let result=run(&m,arm,&dir);
  paid_total+=result["steps"].as_u64().unwrap();
  control_results.push(result);
 }
 let mut mutations=0_u64;
 let mut changed_then_used=0_u64;
 let mut gates=0_u64;
 for result in truth_results.iter(){
  if result["source"]=="G2_changed_allow"{
   mutations+=result["parameter_patches"].as_array().unwrap().iter()
       .filter(|v|v["changed"]==true).count() as u64;
   changed_then_used+=result["later_mutated_reads"].as_array().unwrap().len() as u64;
   gates+=result["decisions"].as_array().unwrap().len() as u64;
  }
 }
 assert!(mutations>0 && changed_then_used>0 && gates>0,
  "no paid-source-mutation → future-decision-use chain");
 let report=json!({"study":"CL1B-REFILL-RULE-G2","status":"ORIGINAL_NATIVE_FROZEN",
  "source_manifest":"cl1b/studies/CL1B-REFILL-RULE-G2-frozen-180-node-self-modifying-184-world-v1.json",
  "original_worlds":truth_results.len()+control_results.len(),
  "total_private_paid_cil":paid_total,
  "truthful_records":truth_results,"adversarial_records":control_results,
  "conditional_effective_source_mutations":mutations,
  "conditional_later_mutated_parameter_reads":changed_then_used,
  "conditional_paid_decision_gates":gates});
 fs::write(dir.join("cl1b_refill_rule_g2_native_184_worlds.json"),
  serde_json::to_vec_pretty(&report).unwrap()).unwrap();
 println!("G2 original native 184 worlds, real charged CIL={paid_total}, \
  actor parameter mutations={mutations}, later reads={changed_then_used}");
}
