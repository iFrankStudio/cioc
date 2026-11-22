//! C9: 274 original-native CL0 paid actual parent-code checkpoint worlds, source-identical PC28-effect causality.
//! Every native CIL executes with normal fee. Only researcher off arm prevents
//! the PC28 PatchConst mutation of PC173 (host never provides actor an opcode).
use cl0_world::{EventKind,Intervention,Op,Origin,Process,World};
use serde_json::{json,Value};
use std::fs::{self,File};
use std::io::{BufWriter,Write};

const MANIFEST:&str="../../../cl1b/studies/CL1B-G2-ACTOR-PAID-TRUE-PARENT-CHECKPOINT-C9-frozen-274-worlds-v1.json";
const G2:&str="../../../cl1b/studies/CL1B-G2-ACTOR-EXECUTED-CODE-READBACK-C7-frozen-274-worlds-v1.json";
const CREDIT:u64=6000;
const CELLS:[i64;15]=[11,20,21,30,32,33,34,35,36,37,38,39,40,41,42];

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
  "G2_checkpoint"|"G2_checkpoint_pc28_suppressed"|"G2_checkpoint_placebo180"=>"frozen_source_g2_checkpoint",
  "G2_checkpoint_bypass"=>"frozen_source_g2_checkpoint_bypass",
  "G2_checkpoint_always"=>"frozen_source_g2_checkpoint_always",
  "G2_checkpoint_abstain"=>"frozen_source_g2_checkpoint_abstain",
  _=>panic!("unknown G2 source: {name}"),
 }].as_array().unwrap()
}
fn theta(program:&[Op])->i64 {
 match &program[173]{
  Op::Const{dst:2,value,next} if *next==24 || *next==174=>*value,
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
 if source_name=="G2_checkpoint_pc28_suppressed"{w=w.with_native_patchconst_suppression(1,28,173);}
 if source_name=="G2_checkpoint_placebo180"{w=w.with_native_patchconst_suppression(1,180,173);}
 w.load(Process::new(1,initial,CREDIT)).unwrap();
 assert_eq!(w.occupied_units(),181);
 let file=File::create(dir.join(format!("cl1b_g2_true_parent_checkpoint_c9_original_{name}.jsonl"))).unwrap();
 let mut trace=BufWriter::new(file);
 let mut paid=0_u64;
 let mut peak=181_usize;
 let mut service=Vec::<Value>::new();
 let mut gate_allow=0_u64;
 let mut gate_abstain=0_u64;
 let mut paid_gate_records=Vec::<Value>::new();
 let mut actor_readback_writes=0_u64;
 let mut attested_gate_checks=0_u64;
 let mut attested_gate_mismatches=0_u64;
 let mut attested_after_suppressed_patch=0_u64;
 let mut last_executed_const_step:Option<u64>=None;
 let mut last_attested_step:Option<u64>=None;
 let mut last_suppressed_step:Option<u64>=None;
 let mut actor_readback_records=Vec::<Value>::new();
 let mut checkpoint_records=Vec::<Value>::new();
 let mut checkpoint_writes=0_u64;
 let mut most_recent_checkpoint:Option<(u64,i64)>=None;
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
   let suppressed=pc==28&&source_name=="G2_checkpoint_pc28_suppressed";
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
    if source_name!="G2_checkpoint_bypass"{
     let (checkpoint_step,parent_theta)=most_recent_checkpoint.expect("paid PC28 lacks preceding actor checkpoint");
     assert!(checkpoint_step<paid&&paid-checkpoint_step==2,
        "PC97 checkpoint must precede paid PC27→PC28 by exactly two");
     assert_eq!(parent_theta,previous);
     assert_eq!(w.cell(42),Some(previous));
    }
    parameter_patches.push(json!({"step":paid,"pc":28,"target":173,
      "before":previous,"attempted":value,"after":if suppressed{previous}else{value},
      "changed":!suppressed&&previous!=value,"suppressed":suppressed}));
    if suppressed{
     last_suppressed_step=Some(paid);
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
  if pc==52{
    assert!(matches!(&event.kind,EventKind::CellWritten{key:41,value}
      if *value==pid.registers[2]));
    let last173=last_executed_const_step.expect("paid C7 Write41 before actual PC173 execution");
    assert!(last173<paid);
    assert_eq!(pid.registers[2],theta(&expected),
       "actor C7 CIL value differs from physical executed Const PC173");
    assert_eq!(w.cell(41),Some(theta(&expected)));
    last_attested_step=Some(paid);
    actor_readback_writes+=1;
    if source_name=="G2_checkpoint_pc28_suppressed"{
      if let Some(first_suppression)=last_suppressed_step{
       if last173>first_suppression{attested_after_suppressed_patch+=1;}
      }
    }
    actor_readback_records.push(json!({"step":paid,"last_pc173":last173,
      "actual_theta":theta(&expected),"written_cell41":w.cell(41).unwrap()}));
   }
   if pc==67{
    assert!(matches!(&event.kind,EventKind::Executed));
    let route=pid.pc;
    assert!(route==1||route==27||route==125,"native C9 checkpoint gate forbidden route");
    let physical=theta(&expected);
    let witnessed=w.cell(41).unwrap_or(0);
    attested_gate_checks+=1;
    let last173=last_executed_const_step.expect("no paid PC173 before C9 gate");
    let last52=last_attested_step.expect("no paid Write41 before C9 gate");
    assert!(last173<last52&&last52<paid);
    assert_eq!(witnessed,physical,"paid physical source readback failed");
    assert_eq!(pid.registers[0],pid.registers[3]-witnessed);
    let route_expected=match source_name{
      "G2_checkpoint"|"G2_checkpoint_pc28_suppressed"|"G2_checkpoint_placebo180"=>
        if pid.registers[0]!=0{1}else{125},
      "G2_checkpoint_bypass"=>if pid.registers[0]!=0{27}else{125},
      "G2_checkpoint_always"=>1,
      "G2_checkpoint_abstain"=>125,
      _=>unreachable!()
    };
    assert_eq!(route,route_expected,"native C9 paid CIL gate differs from initial source");
    if route==125{gate_abstain+=1;}else{gate_allow+=1;}
    paid_gate_records.push(json!({"step":paid,"r3":pid.registers[3],
       "r0":pid.registers[0],"route":route,"witness":witnessed,
       "actual_theta":physical,"evidence_key":41,
       "last_pc173":last_executed_const_step,"last_write":last_attested_step,
       "decision":if route==125{"ABSTAIN"}else{"EDIT"}}));
   }
   if pc==97{
     assert!(matches!(&event.kind,EventKind::CellWritten{key:42,value}
       if *value==pid.registers[0]));
     assert_eq!(pid.pc,27);
     assert_eq!(w.cell(42),Some(theta(&expected)));
     assert_eq!(pid.registers[0],theta(&expected));
     assert_eq!(w.cell(41),Some(theta(&expected)));
     checkpoint_writes+=1;
     most_recent_checkpoint=Some((paid,theta(&expected)));
     checkpoint_records.push(json!({"step":paid,"previous_physical_theta":theta(&expected),
        "cell42":w.cell(42).unwrap(),"last_verified_readback":last_attested_step}));
   }
      if pc==173{
   last_executed_const_step=Some(paid);
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
   let expected_jump=if difference!=0{176}else{179};
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
  let label=if pc==28&&source_name=="G2_checkpoint_pc28_suppressed"{ "suppressed_pc28" }
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
  "host_source_edits":0,"budget_exhausted":budget_exhausted,
   "paid_gate_allow":gate_allow,"paid_gate_abstain":gate_abstain,
   "paid_gate_records":paid_gate_records,
   "actor_readback_writes":actor_readback_writes,
   "attested_gate_checks":attested_gate_checks,
   "attested_gate_mismatches":attested_gate_mismatches,
   "attested_after_suppressed_patch":attested_after_suppressed_patch,
   "actor_readback_records":actor_readback_records,
   "final_attested_theta":w.cell(41).unwrap_or(0),
   "checkpoint_records":checkpoint_records,
   "actor_paid_checkpoint_writes":checkpoint_writes,
   "final_parent_checkpoint":w.cell(42).unwrap_or(0)})
}

#[test]
fn c9_274_native_paid_true_parent_checkpoint_before_every_pc28(){
 let root=std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
 let m:Value=serde_json::from_slice(&fs::read(root.join(MANIFEST)).unwrap()).unwrap();
 let parent:Value=serde_json::from_slice(&fs::read(root.join(G2)).unwrap()).unwrap();
 assert_eq!(m["study"],"CL1B-G2-ACTOR-PAID-TRUE-PARENT-CHECKPOINT-C9");
 assert_eq!(m["status"],"PRE_NATIVE_FROZEN");
 assert_eq!(m["counts"]["native_worlds"],274);
 let mut exact=parent["frozen_source_g2_readback_attest"].clone();
 assert_eq!(exact.as_array().unwrap().len(),180);
 assert_eq!(exact[0],json!({"kind":"Const","dst":2,"value":30,"next":1}));
 assert_eq!(exact[1],json!({"kind":"Read","dst":3,"address":2,"next":2}));
 assert_eq!(exact[2],json!({"kind":"Const","dst":2,"value":11,"next":3}));
 assert_eq!(exact[88]["next"],96);
 assert_eq!(exact[96],json!({"kind":"Const","dst":2,"value":36,"next":97}));
 assert_eq!(exact[97],json!({"kind":"Const","dst":0,"value":1,"next":98}));
 assert_eq!(exact[67]["nonzero"],27);
 exact[0]=json!({"kind":"Const","dst":2,"value":11,"next":3});
 exact[1]=json!({"kind":"Const","dst":2,"value":41,"next":2});
 exact[2]=json!({"kind":"Read","dst":0,"address":2,"next":96});
 exact[88]["next"]=json!(98);
 exact[96]=json!({"kind":"Const","dst":2,"value":42,"next":97});
 exact[97]=json!({"kind":"Write","address":2,"value":0,"next":27});
 exact[67]["nonzero"]=json!(1);
 assert_eq!(m["frozen_source_g2_checkpoint"],exact,
    "C9 founder differs from the seven-node protocol frozen before native runs");
 let mut var=exact.clone();var[67]["nonzero"]=json!(27);
 assert_eq!(m["frozen_source_g2_checkpoint_bypass"],var);
 var=exact.clone();var[67]["zero"]=json!(1);
 assert_eq!(m["frozen_source_g2_checkpoint_always"],var);
 var=exact.clone();var[67]["nonzero"]=json!(125);
 assert_eq!(m["frozen_source_g2_checkpoint_abstain"],var);
 for n in ["G2_checkpoint","G2_checkpoint_bypass","G2_checkpoint_always",
           "G2_checkpoint_abstain"]{assert_eq!(source(&m,n).len(),180);}
 let arms=m["arms"].as_array().unwrap();let cases=m["cases"].as_array().unwrap();
 let controls=m["controls"].as_array().unwrap();
 assert_eq!((arms.len(),cases.len(),controls.len()),(270,45,4));
 let dir=root.join("../../cl1b-ci-evidence");fs::create_dir_all(&dir).unwrap();
 let mut truthful=Vec::<Value>::new();let mut adversarial=Vec::<Value>::new();
 let mut paid_total=0_u64;let mut costs=[0_u64;6];let mut incomplete=Vec::<String>::new();
 let mut checkpoint_writes=0_u64;let mut attempt_count=0_u64;
 let mut bypass_extra_cost=0_i64;let mut strict_prefixes=0_u64;
 let mut suppressions=0_u64;let mut no_suppression=0_u64;let mut attested_after_suppression=0_u64;
 let mut gate_edits=0_u64;let mut gate_abstains=0_u64;let mut flat_deltas=Vec::<Value>::new();
 let raw=|r:&Value|->Vec<Value>{
   let name=r["name"].as_str().unwrap();
   fs::read_to_string(dir.join(format!("cl1b_g2_true_parent_checkpoint_c9_original_{name}.jsonl"))).unwrap()
    .lines().map(|line|serde_json::from_str(line).unwrap()).collect()
 };
 let service_semantics=|r:&Value|->Vec<Value>{
   r["services"].as_array().unwrap().iter()
    .map(|x|json!([x["stage"],x["guess"],x["truth"],x["feedback"]])).collect()
 };
 for (i,cas) in cases.iter().enumerate(){
  let group=&arms[6*i..6*i+6];
  assert_eq!(group.iter().map(|a|a["source"].as_str().unwrap()).collect::<Vec<_>>(),
    vec!["G2_checkpoint","G2_checkpoint_bypass","G2_checkpoint_always",
         "G2_checkpoint_abstain","G2_checkpoint_pc28_suppressed","G2_checkpoint_placebo180"]);
  for a in group{assert_eq!(a["goals"],cas["goals"]);}
  let cp=run(&m,&group[0],&dir);let bypass=run(&m,&group[1],&dir);
  let always=run(&m,&group[2],&dir);let never=run(&m,&group[3],&dir);
  let suppressed=run(&m,&group[4],&dir);let placebo=run(&m,&group[5],&dir);
  for (j,r) in [&cp,&bypass,&always,&never,&suppressed,&placebo].iter().enumerate(){
   if r["completed_stages"]!=48||r["truthful_matches"]!=48||
      r["claim"]!=1||r["budget_exhausted"]==true{
     incomplete.push(r["name"].as_str().unwrap().to_string());
   }
   costs[j]+=r["steps"].as_u64().unwrap();
   paid_total+=r["steps"].as_u64().unwrap();
   assert_eq!(r["attested_gate_mismatches"],0);
  }
  assert_eq!(bypass["actor_paid_checkpoint_writes"],0);
  assert_eq!(never["actor_paid_checkpoint_writes"],0);
  assert_eq!(cp["actor_paid_checkpoint_writes"],cp["parameter_patches"].as_array().unwrap().len());
  assert_eq!(cp["actor_paid_checkpoint_writes"],cp["paid_gate_allow"]);
  assert_eq!(always["actor_paid_checkpoint_writes"],always["paid_gate_allow"]);
  assert_eq!(never["paid_gate_allow"],0);
  assert_eq!(always["paid_gate_abstain"],0);
  let attempts=cp["parameter_patches"].as_array().unwrap().len() as i64;
  let delta=cp["steps"].as_i64().unwrap()-bypass["steps"].as_i64().unwrap();
  assert_eq!(delta,4*attempts,
     "C9 actor-owned parent record costs must be EXACTLY four paid CIL per edit");
  assert_eq!(cp["final_theta"],bypass["final_theta"]);
  assert_eq!(cp["final_edges"],bypass["final_edges"]);
  assert_eq!(service_semantics(&cp),service_semantics(&bypass),
     "C9 checkpoint must not alter any true task guess, service or feedback");
  assert_eq!(raw(&cp),raw(&placebo),
     "C9 paid CIL full-history placebo must be identical including 42");
  assert_eq!(cp["steps"],placebo["steps"]);
  assert_eq!(cp["checkpoint_records"],placebo["checkpoint_records"]);
  let sup= suppressed["suppressed_pc28"].as_array().unwrap();
  suppressions+=sup.len() as u64;
  attested_after_suppression+=suppressed["attested_after_suppressed_patch"].as_u64().unwrap();
  if sup.is_empty(){
    no_suppression+=1;
    assert_eq!(raw(&cp),raw(&suppressed));
  }else{
    let tsup=raw(&suppressed);let tbase=raw(&cp);
    let first=sup[0]["step"].as_u64().unwrap();
    let k=tsup.iter().position(|x|x[0]=="cil"&&x[1]==first&&x[2]==28).unwrap();
    assert_eq!(tbase[..k],tsup[..k]);
    assert_eq!(tsup[k][10],"suppressed_pc28");
    strict_prefixes+=1;
  }
  checkpoint_writes+=cp["actor_paid_checkpoint_writes"].as_u64().unwrap();
  attempt_count+=attempts as u64;
  bypass_extra_cost+=delta;
  gate_edits+=cp["paid_gate_allow"].as_u64().unwrap();
  gate_abstains+=cp["paid_gate_abstain"].as_u64().unwrap();
  flat_deltas.push(json!({"case":cas["name"],
    "checkpointed_true_paid_edits":attempts,
    "all_charge_checkpoint":cp["steps"],"all_charge_bypass":bypass["steps"],
    "exact_additional_paid_CIL":delta,
    "same_guesses_feedback_and_final_executable_source":true}));
  truthful.extend([cp,bypass,always,never,suppressed,placebo]);
 }
 for a in controls{
  let r=run(&m,a,&dir);
  paid_total+=r["steps"].as_u64().unwrap();
  adversarial.push(r);
 }
 assert!(attempt_count>0&&checkpoint_writes>0);
 assert_eq!(checkpoint_writes,attempt_count);
 assert!(suppressions>0&&attested_after_suppression>0&&strict_prefixes>0);
 let output=json!({
  "study":"CL1B-G2-ACTOR-PAID-TRUE-PARENT-CHECKPOINT-C9",
  "status":"ORIGINAL_NATIVE_ACTOR_PAID_PREPATCH_PHYSICAL_PARENT_CHECKPOINT",
  "frozen_manifest":"cl1b/studies/CL1B-G2-ACTOR-PAID-TRUE-PARENT-CHECKPOINT-C9-frozen-274-worlds-v1.json",
  "native_worlds":274,"total_paid_private_cil":paid_total,
  "incomplete_truthful":incomplete,
  "actual_actor_checkpoint_writes":checkpoint_writes,
  "actual_paid_pc28_attempts_in_checkpoint_arm":attempt_count,
  "checkpoint_gate_allow":gate_edits,"checkpoint_gate_abstain":gate_abstains,
  "exact_additional_private_paid_CIL_vs_same_rewired_bypass":bypass_extra_cost,
  "checkpoint_fee_per_attempt":4,
  "source_identical_suppressed_patch_attempts":suppressions,
  "source_identical_suppressed_later_physical_readbacks":attested_after_suppression,
  "suppression_original_native_strict_prefixes":strict_prefixes,
  "unexposed_suppression_pairs":no_suppression,
  "costs":{
    "G2_checkpoint":costs[0],
    "G2_checkpoint_bypass":costs[1],
    "G2_checkpoint_always":costs[2],
    "G2_checkpoint_abstain":costs[3],
    "G2_checkpoint_pc28_suppressed":costs[4],
    "G2_checkpoint_placebo180":costs[5]
  },
  "per_case":flat_deltas,"truthful_records":truthful,"adversarial_records":adversarial
 });
 fs::write(dir.join("cl1b_g2_true_parent_checkpoint_c9_native_274_worlds.json"),
   serde_json::to_vec_pretty(&output).unwrap()).unwrap();
 println!("C9 ORIGINAL NATIVE: 274 worlds, private paid={}, actually paid physical-parent checkpoints={}, actual PC28 edits={}, measured CIL overhead={} exactly 4/edit, source suppress={} and readback after suppression={}, incomplete={}",
  paid_total,checkpoint_writes,attempt_count,bypass_extra_cost,
  suppressions,attested_after_suppression,incomplete.len());
 assert!(incomplete.is_empty(),"C9 complete true-stage gate FAIL; original source and negative proof already saved");
}
