//! C3: 292 original-native CL0 worlds, source-identical PC28-effect causality.
//! Every native CIL executes with normal fee. Only researcher off arm prevents
//! the PC28 PatchConst mutation of PC173 (host never provides actor an opcode).
use cl0_world::{EventKind,Intervention,Op,Origin,Process,World};
use serde_json::{json,Value};
use std::fs::{self,File};
use std::io::{BufWriter,Write};

const MANIFEST:&str="../../../cl1b/studies/CL1B-G2-PC28-DECISION-ALIAS-C3-frozen-292-worlds-v1.json";
const G2:&str="../../../cl1b/studies/CL1B-G2-PC28-LIFECYCLE-C2-frozen-184-worlds-v1.json";
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
  "G2_on"|"G2_pc28_suppressed"|"G2_placebo180"=>"frozen_source_g2_on",
  "G2_never"=>"frozen_source_g2_never",
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
 assert_eq!(goals.len(),48);
 assert!(goals.iter().all(|g|(0..=2).contains(g)));
 let initial=source(m,source_name).iter().map(decoded).collect::<Vec<_>>();
 assert_eq!(initial.len(),180);
 let mut expected=initial.clone();
 let mut w=World::with_strict_capacity(16,180,300);
 if source_name=="G2_pc28_suppressed"{w=w.with_native_patchconst_suppression(1,28,173);}
 if source_name=="G2_placebo180"{w=w.with_native_patchconst_suppression(1,180,173);}
 w.load(Process::new(1,initial,CREDIT)).unwrap();
 assert_eq!(w.occupied_units(),181);
 let file=File::create(dir.join(format!("cl1b_g2_pc28_c3_original_{name}.jsonl"))).unwrap();
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
 let mut suppressed_pc28=Vec::<Value>::new();
 let mut failed_third_probe_halt=false;
 while w.step(){
  paid+=1;
  assert!(paid<=CREDIT,"private CIL inventory exceeded 6000");
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
   let suppressed=pc==28&&source_name=="G2_pc28_suppressed";
   if suppressed{assert!(matches!(&event.kind,EventKind::NativeCodeEditSuppressed{opcode_tag:9}));}
   else{assert!(matches!(&event.kind,EventKind::Executed));}
   let target=pid.registers[1] as usize;
   let value=if pc==28{pid.registers[3]}else{pid.registers[0]};
   assert_eq!(target,if pc==28{173}else{46});
   let previous=match &expected[target]{
     Op::Const{value,..}=>*value,
     x=>panic!("invalid paid PatchConst destination: {x:?}"),
   };
   if !suppressed{
    if let Op::Const{value:field,..}=&mut expected[target]{*field=value;}
   }
   if pc==28{
    assert!((0..=2).contains(&value));
    parameter_patches.push(json!({"step":paid,"pc":28,"target":173,
      "before":previous,"attempted":value,"after":if suppressed{previous}else{value},
      "changed":!suppressed&&previous!=value,"suppressed":suppressed}));
    if suppressed{
     assert_eq!(theta(&expected),previous);
     suppressed_pc28.push(json!({"step":paid,"pc":28,"target":173,"attempted":value,
       "before":previous,"after":previous}));
    }else if previous!=value{last_changed=Some(value);}
   }
   paid_code_edits.push(json!({"step":paid,"pc":pc,
     "target":target,"before":previous,"attempted":value,
     "after":if suppressed{previous}else{value},
     "kind":"PatchConst","suppressed":suppressed}));
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
    "G2_on"|"G2_pc28_suppressed"|"G2_placebo180"=>if difference!=0{176}else{179},
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
  let label=if pc==28&&source_name=="G2_pc28_suppressed"{ "suppressed_pc28" }
    else if pc==28{"paid_parameter_patch"}else{"normal"};
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
   assert!(stage<48);
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
 let budget_exhausted=paid>=CREDIT;
 assert!(pid.stopped || budget_exhausted,"original native PID remains live after stepping stopped");
 let truth=service.iter().filter(|e|e["truth"]==1).count();
 let completed=w.cell(32).unwrap_or(0);
 let claim=w.cell(21).unwrap_or(0);
 if mode=="truth"{
   // No completion assertion here: record ALL 184 worlds, then fail the
   // frozen all-truthful-worlds completion gate after publishing summary.
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
  "paid_code_edits":paid_code_edits,"suppressed_pc28":suppressed_pc28,
  "later_mutated_reads":later_mutated_reads,
  "completed_stages":completed,"truthful_matches":truth,"claim":claim,
  "final_theta":theta(&expected),"final_edges":edges(&w),
  "peak_occupied":peak,"failed_third_probe_halt":failed_third_probe_halt,
  "host_source_edits":0,"budget_exhausted":budget_exhausted})
}
#[test]
fn c3_frozen_292_original_native_same_source_patchconst_causality(){
 let root=std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
 let m:Value=serde_json::from_slice(&fs::read(root.join(MANIFEST)).unwrap()).unwrap();
 let parent:Value=serde_json::from_slice(&fs::read(root.join(G2)).unwrap()).unwrap();
 assert_eq!(m["study"],"CL1B-G2-PC28-DECISION-ALIAS-C3");
 assert_eq!(m["status"],"PRE_NATIVE_FROZEN");
 assert_eq!(m["counts"]["native_worlds"],292);
 assert_eq!(m["frozen_source_g2_on"],parent["frozen_source_g2_on"]);
 assert_eq!(m["frozen_source_g2_never"],parent["frozen_source_g2_never"]);
 assert_eq!(m["counts"]["anchors"],9);
 assert_eq!(m["counts"]["futures_per_anchor"],8);
 assert_eq!(m["generation"]["shared_prefix_stages"],24);
 assert_eq!(source(&m,"G2_on").len(),180);
 assert_eq!(source(&m,"G2_never").len(),180);
 let arms=m["arms"].as_array().unwrap();
 let controls=m["controls"].as_array().unwrap();
 let cases=m["cases"].as_array().unwrap();
 assert_eq!(arms.len(),288);assert_eq!(controls.len(),4);assert_eq!(cases.len(),72);
 let dir=root.join("../../cl1b-ci-evidence");
 fs::create_dir_all(&dir).unwrap();
 let mut results=Vec::<Value>::new();
 let mut deceptive=Vec::<Value>::new();
 let mut total_paid=0_u64;
 let mut incomplete_truthful=Vec::<String>::new();
 let mut actual_suppressions=0_u64;
 let mut first_prefixes=0_u64;
 let mut no_exposure=0_u64;
 let mut paired_savings=0_i64;
 let mut wins=0_usize;
 let mut losses=0_usize;
 let mut ties=0_usize;
 for (i,c) in cases.iter().enumerate(){
  let pair=&arms[4*i..4*i+4];
  assert_eq!(pair.iter().map(|a|a["source"].as_str().unwrap()).collect::<Vec<_>>(),
    vec!["G2_on","G2_pc28_suppressed","G2_placebo180","G2_never"]);
  for arm in pair{assert_eq!(arm["goals"],c["goals"]);}
  let on=run(&m,&pair[0],&dir);
  let off=run(&m,&pair[1],&dir);
  let placebo=run(&m,&pair[2],&dir);
  let never=run(&m,&pair[3],&dir);
  for x in [&on,&off,&placebo,&never]{
   if x["completed_stages"]!=48 || x["truthful_matches"]!=48 || x["claim"]!=1 || x["budget_exhausted"]==true {
    incomplete_truthful.push(x["name"].as_str().unwrap().to_string());
   }
   total_paid+=x["steps"].as_u64().unwrap();
  }
  assert_eq!(on["steps"],placebo["steps"]);
  assert_eq!(on["services"],placebo["services"]);
  assert_eq!(on["parameter_patches"],placebo["parameter_patches"]);
  assert_eq!(on["decisions"],placebo["decisions"]);
  assert_eq!(on["final_edges"],placebo["final_edges"]);
  // Stronger than summary agreement: EVERY original CIL+feedback JSONL row.
  let raw=|r:&Value| -> Vec<Value> {
   let name=r["name"].as_str().unwrap();
   fs::read_to_string(dir.join(format!("cl1b_g2_pc28_c3_original_{name}.jsonl"))).unwrap()
    .lines().map(|line|serde_json::from_str(line).unwrap()).collect()
  };
  let on_trace=raw(&on);
  let placebo_trace=raw(&placebo);
  let off_trace=raw(&off);
  assert_eq!(on_trace,placebo_trace,"unreachable PC180 placebo changed original native history");
  let suppressed=off["suppressed_pc28"].as_array().unwrap();
  actual_suppressions+=suppressed.len() as u64;
  if suppressed.is_empty(){
    no_exposure+=1;
    assert_eq!(on_trace,off_trace,"without PC28 exposure the worlds must match exactly");
  }else{
    let first_step=suppressed[0]["step"].as_u64().unwrap();
    let first_index=off_trace.iter().position(|x|x[0]=="cil"&&x[1]==first_step&&x[2]==28)
     .expect("first native CIL PC28 suppression trace missing");
    assert_eq!(on_trace[..first_index],off_trace[..first_index],
      "different original native world histories before first selective suppression");
    assert_eq!(off_trace[first_index][10],"suppressed_pc28");
    first_prefixes+=1;
  }
  let saving=off["steps"].as_i64().unwrap()-on["steps"].as_i64().unwrap();
  paired_savings+=saving;
  if saving>0{wins+=1;}else if saving<0{losses+=1;}else{ties+=1;}
  results.extend([on,off,placebo,never]);
 }
 for c in controls{
  let x=run(&m,c,&dir);
  total_paid+=x["steps"].as_u64().unwrap();
  deceptive.push(x);
 }
 assert_eq!(wins+losses+ties,72);
 assert!(actual_suppressions>0&&first_prefixes>0,
   "no original-native selective PC28 intervention actually exercised");
 // Mismatch controls must trap, not silently turn a different instruction into a no-op.
 let mut mismatched=World::with_strict_capacity(16,180,300)
    .with_native_patchconst_suppression(1,1,173);
 mismatched.load(Process::new(1,vec![
  Op::Const{dst:1,value:0,next:1},
  Op::PatchConst{node:1,value:0,next:2},
  Op::Halt,
 ],4)).unwrap();
 assert!(mismatched.step());
 assert!(mismatched.step());
 assert!(matches!(&mismatched.events().last().unwrap().kind,
    EventKind::Trapped(message) if message.contains("target mismatch")));
 let mut wrong_opcode=World::with_strict_capacity(16,180,300)
    .with_native_patchconst_suppression(1,0,0);
 wrong_opcode.load(Process::new(1,vec![Op::Const{dst:0,value:0,next:1},Op::Halt],4)).unwrap();
 assert!(wrong_opcode.step());
 assert!(matches!(&wrong_opcode.events().last().unwrap().kind,
    EventKind::Trapped(message) if message.contains("opcode mismatch")));
 let output=json!({
  "study":"CL1B-G2-PC28-DECISION-ALIAS-C3",
  "status":"ORIGINAL_NATIVE_SOURCE_IDENTICAL_PC28_CAUSAL",
  "frozen_manifest":"cl1b/studies/CL1B-G2-PC28-DECISION-ALIAS-C3-frozen-292-worlds-v1.json",
  "total_original_native_worlds":292,
  "incomplete_truthful_world_names":incomplete_truthful,
  "total_private_paid_cil":total_paid,
  "actual_suppressed_pc28_attempts":actual_suppressions,
  "first_exposure_prefix_witnesses":first_prefixes,
  "no_exposure_pairs":no_exposure,
  "paired_suppressed_minus_on_cost":paired_savings,
  "wins_on":wins,"losses_on":losses,"ties":ties,
  "truthful_records":results,"adversarial_records":deceptive
 });
 fs::write(dir.join("cl1b_g2_pc28_c3_native_184_worlds.json"),
  serde_json::to_vec_pretty(&output).unwrap()).unwrap();
 println!("C3 ORIGINAL native 292 worlds, charged={}, PC28-suppressed={}, \
  on wins/losses/ties={}/{}/{}, suppressed-minus-on={}",
  total_paid,actual_suppressions,wins,losses,ties,paired_savings);
 assert!(incomplete_truthful.is_empty(),"C3 pre-registered 48-stage completion gate failed; preserve native summary");
}
