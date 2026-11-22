//! C10: 274 original-native CL0 state-triggered paid physical code parent restoration, source-identical PC28-effect causality.
//! Every native CIL executes with normal fee. Only researcher off arm prevents
//! the PC28 PatchConst mutation of PC173 (host never provides actor an opcode).
use cl0_world::{EventKind,Intervention,Op,Origin,Process,World};
use serde_json::{json,Value};
use std::fs::{self,File};
use std::io::{BufWriter,Write};

const MANIFEST:&str="../../../cl1b/studies/CL1B-G2-ACTOR-PAID-STATE-TRIGGERED-ROLLBACK-C10-frozen-274-worlds-v1.json";
const G2:&str="../../../cl1b/studies/CL1B-G2-ACTOR-PAID-TRUE-PARENT-CHECKPOINT-C9-frozen-274-worlds-v1.json";
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
  "G2_rollback"|"G2_rollback_pc102_suppressed"|"G2_rollback_pc28_suppressed"|
  "G2_rollback_placebo180"=>"frozen_source_g2_rollback",
  "G2_C9_checkpoint_no_rollback"=>"frozen_source_g2_C9_checkpoint_no_rollback",
  "G2_C9_checkpoint_abstain"=>"frozen_source_g2_C9_checkpoint_abstain",
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
 if source_name=="G2_rollback_pc28_suppressed"{w=w.with_native_patchconst_suppression(1,28,173);}
 if source_name=="G2_rollback_pc102_suppressed"{w=w.with_native_patchconst_suppression(1,102,173);}
 if source_name=="G2_rollback_placebo180"{w=w.with_native_patchconst_suppression(1,180,173);}
 w.load(Process::new(1,initial,CREDIT)).unwrap();
 assert_eq!(w.occupied_units(),181);
 let file=File::create(dir.join(format!("cl1b_g2_state_rollback_c10_original_{name}.jsonl"))).unwrap();
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
 let mut history_restore_records=Vec::<Value>::new();
 let mut actual_parent_restores=0_u64;
 let mut restoration_readback_records=Vec::<Value>::new();
 let mut last_actual_parent_restore:Option<(u64,i64)>=None;
 let mut last_positive_cell36_trigger:Option<u64>=None;
 let mut suppressed_pc102_count=0_u64;
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
   let actor_rollback=pc==102&&source_name.starts_with("G2_rollback");
   let suppressed=(pc==28&&source_name=="G2_rollback_pc28_suppressed")||
      (actor_rollback&&source_name=="G2_rollback_pc102_suppressed");
   if suppressed{assert!(matches!(&event.kind,EventKind::NativeCodeEditSuppressed{opcode_tag:9}));}
   else{assert!(matches!(&event.kind,EventKind::Executed));}
   let target=pid.registers[1] as usize;
   let value=if pc==28{pid.registers[3]}else{pid.registers[0]};
   assert_eq!(target,if pc==28||actor_rollback{173}else{46});
   let previous=match &expected[target]{
     Op::Const{value,..}=>*value,
     x=>panic!("invalid paid PatchConst destination: {x:?}"),
   };
   if !suppressed{
    if let Op::Const{value:field,..}=&mut expected[target]{*field=value;}
   }
   if pc==28{
    assert!((0..=2).contains(&value));
    { // all C10 arms that execute PC28 must have a genuine paid C9 parent checkpoint
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
    }else if previous!=value{
     last_changed=Some(value);
     last_actual_parent_restore=None;
    }
   }
   if actor_rollback{
    let trig=last_positive_cell36_trigger.expect("paid C10 PC102 has no recent positive original PC95");
    assert_eq!(paid-trig,4,"C10 native paid PC95→99→100→101→102 path discontinuous");
    assert!(w.cell(36).unwrap_or(0)!=0);
    assert_eq!(value,w.cell(42).unwrap_or(0),"C10 parent restore must use actor-owned memory42");
    if checkpoint_writes>0{
     assert_eq!(value,most_recent_checkpoint.unwrap().1,
       "C10 restore did not use latest actual physical pre-patch parent");
    }
    if suppressed{suppressed_pc102_count+=1;}
    let physical_restore=checkpoint_writes>0&&!suppressed&&previous!=value;
    if physical_restore{
      actual_parent_restores+=1;
      last_changed=Some(value);
      last_actual_parent_restore=Some((paid,value));
    }
    history_restore_records.push(json!({"step":paid,"pc":102,"paid_history_trigger_step":trig,
      "checkpoint_exists":checkpoint_writes>0,"parent_from_actor_cell42":value,
      "before":previous,"after":if suppressed{previous}else{value},
      "suppressed":suppressed,"actual_physical_parent_restoration":physical_restore}));
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
      "G2_rollback"|"G2_rollback_pc102_suppressed"|"G2_rollback_pc28_suppressed"|
      "G2_rollback_placebo180"|"G2_C9_checkpoint_no_rollback"=>
         if pid.registers[0]!=0{1}else{125},
      "G2_C9_checkpoint_abstain"=>125,
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
   if pc==95 && source_name.starts_with("G2_rollback") && pid.pc==99{
     assert!(matches!(&event.kind,EventKind::Executed));
     assert!(w.cell(36).unwrap_or(0)!=0);
     last_positive_cell36_trigger=Some(paid);
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
   if let Some((restep,retheta))=last_actual_parent_restore{
     assert!(paid>restep&&pid.registers[2]==retheta,
        "C10 paid source readback after real restore is wrong");
     restoration_readback_records.push(json!({"step":paid,
         "rollback_step":restep,"actual_readback_theta":retheta}));
     last_actual_parent_restore=None;
   }
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
  let label=if pc==28&&source_name=="G2_rollback_pc28_suppressed"{ "suppressed_pc28" }
    else if pc==102&&source_name=="G2_rollback_pc102_suppressed"{"suppressed_pc102"}
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
   "final_parent_checkpoint":w.cell(42).unwrap_or(0),
   "rollback_attempt_records":history_restore_records,
   "genuine_paid_physical_parent_restores":actual_parent_restores,
   "later_actual_rollback_readbacks":restoration_readback_records,
   "suppressed_pc102_count":suppressed_pc102_count})
}


#[test]
fn c10_274_native_paid_actor_pc102_restores_real_prior_theta_from_cell42(){
 let root=std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
 let m:Value=serde_json::from_slice(&fs::read(root.join(MANIFEST)).unwrap()).unwrap();
 let p:Value=serde_json::from_slice(&fs::read(root.join(G2)).unwrap()).unwrap();
 assert_eq!(m["study"],"CL1B-G2-ACTOR-PAID-STATE-TRIGGERED-ROLLBACK-C10");
 assert_eq!(m["status"],"PRE_NATIVE_FROZEN");
 assert_eq!(m["counts"]["original_native_worlds"],274);
 let mut exact=p["frozen_source_g2_checkpoint"].clone();
 assert_eq!(exact.as_array().unwrap().len(),180);
 assert_eq!(exact[99],json!({"kind":"Const","dst":2,"value":35,"next":100}));
 assert_eq!(exact[101],json!({"kind":"Const","dst":1,"value":46,"next":102}));
 assert_eq!(exact[102],json!({"kind":"PatchConst","node":1,"value":0,"next":103}));
 assert_eq!(exact[95],json!({"kind":"Branch","test":0,"nonzero":99,"zero":86}));
 exact[99]["value"]=json!(42);
 exact[101]["value"]=json!(173);
 assert_eq!(m["frozen_source_g2_rollback"],exact,
    "C10 exact two-node physical source rollback program differs from freeze");
 assert_eq!(m["frozen_source_g2_C9_checkpoint_no_rollback"],p["frozen_source_g2_checkpoint"]);
 assert_eq!(m["frozen_source_g2_C9_checkpoint_abstain"],p["frozen_source_g2_checkpoint_abstain"]);
 let cases=m["cases"].as_array().unwrap();
 let arms=m["arms"].as_array().unwrap();
 let controls=m["controls"].as_array().unwrap();
 assert_eq!((cases.len(),arms.len(),controls.len()),(45,270,4));
 let dir=root.join("../../cl1b-ci-evidence");
 fs::create_dir_all(&dir).unwrap();
 let mut truthful=Vec::<Value>::new();let mut adversarial=Vec::<Value>::new();
 let mut cost=[0_u64;6];let mut total_fee=0_u64;
 let mut incomplete=Vec::<String>::new();
 let mut parent_saves=0_u64;
 let mut charged_rollback_attempts=0_u64;
 let mut actual_source_reverts=0_u64;
 let mut actor_readback_after_restore=0_u64;
 let mut host_suppressed_pc102=0_u64;
 let mut host_suppressed_pc28=0_u64;
 let mut prefixes_pc102=0_u64;
 let mut no_pc102_exposures=0_u64;
 let mut per_case=Vec::<Value>::new();
 let raw=|r:&Value|->Vec<Value>{
  let name=r["name"].as_str().unwrap();
  fs::read_to_string(dir.join(format!("cl1b_g2_state_rollback_c10_original_{name}.jsonl"))).unwrap()
   .lines().map(|line|serde_json::from_str(line).unwrap()).collect()
 };
 for (i,case) in cases.iter().enumerate(){
  let group=&arms[6*i..6*i+6];
  assert_eq!(group.iter().map(|a|a["source"].as_str().unwrap()).collect::<Vec<_>>(),
    vec!["G2_rollback","G2_rollback_pc102_suppressed",
         "G2_rollback_pc28_suppressed","G2_rollback_placebo180",
         "G2_C9_checkpoint_no_rollback","G2_C9_checkpoint_abstain"]);
  for arm in group{assert_eq!(arm["goals"],case["goals"]);}
  let live=run(&m,&group[0],&dir);
  let sup_rollback=run(&m,&group[1],&dir);
  let sup_edit=run(&m,&group[2],&dir);
  let placebo=run(&m,&group[3],&dir);
  let old=run(&m,&group[4],&dir);
  let fixed_never=run(&m,&group[5],&dir);
  for (j,v) in [&live,&sup_rollback,&sup_edit,&placebo,&old,&fixed_never].iter().enumerate(){
   total_fee+=v["steps"].as_u64().unwrap();
   cost[j]+=v["steps"].as_u64().unwrap();
   if v["completed_stages"]!=48||v["truthful_matches"]!=48||
       v["claim"]!=1||v["budget_exhausted"]==true{
      incomplete.push(v["name"].as_str().unwrap().to_owned());
   }
   assert_eq!(v["attested_gate_mismatches"],0);
   assert_eq!(v["actor_paid_checkpoint_writes"],
       v["parameter_patches"].as_array().unwrap().len());
  }
  assert_eq!(raw(&live),raw(&placebo),
    "C10 unreachable PC180 researcher control altered original native paid CIL");
  assert_eq!(live["steps"],placebo["steps"]);
  assert_eq!(live["rollback_attempt_records"],placebo["rollback_attempt_records"]);
  assert_eq!(live["genuine_paid_physical_parent_restores"],
    placebo["genuine_paid_physical_parent_restores"]);
  for v in [&live,&sup_rollback,&sup_edit,&placebo]{
   let attempts=v["rollback_attempt_records"].as_array().unwrap();
   assert_eq!(attempts.len(),v["paid_code_edits"].as_array().unwrap().iter()
        .filter(|x|x["pc"]==102).count());
   assert!(attempts.iter().all(|x|x["parent_from_actor_cell42"]==x["after"]
        || x["suppressed"]==true),"paid PID1 must use self-owned previous code value");
  }
  assert_eq!(sup_rollback["genuine_paid_physical_parent_restores"],0,
     "strict host suppression PC102 must really eliminate physical source restoration");
  let first_supp=sup_rollback["rollback_attempt_records"].as_array().unwrap()
      .iter().find(|x|x["suppressed"]==true);
  if let Some(first)=first_supp{
   let ts=raw(&sup_rollback);let tl=raw(&live);
   let first_paid=first["step"].as_u64().unwrap();
   let k=ts.iter().position(|x|x[0]=="cil"&&x[1]==first_paid&&x[2]==102).unwrap();
   assert_eq!(tl[..k],ts[..k],
       "same-initial-CIL C10 original charged history must match before first PC102 effect suppression");
   assert_eq!(ts[k][10],"suppressed_pc102");
   prefixes_pc102+=1;
  }else{
   no_pc102_exposures+=1;
   assert_eq!(raw(&live),raw(&sup_rollback),
      "no native suppressed PC102 observed, source-identical worlds must match");
  }
  parent_saves+=live["actor_paid_checkpoint_writes"].as_u64().unwrap();
  charged_rollback_attempts+=live["rollback_attempt_records"].as_array().unwrap().len() as u64;
  actual_source_reverts+=live["genuine_paid_physical_parent_restores"].as_u64().unwrap();
  actor_readback_after_restore+=live["later_actual_rollback_readbacks"].as_array().unwrap().len() as u64;
  host_suppressed_pc102+=sup_rollback["suppressed_pc102_count"].as_u64().unwrap();
  host_suppressed_pc28+=sup_edit["suppressed_pc28"].as_array().unwrap().len() as u64;
  per_case.push(json!({"case":case["name"],"regime":case["regime"],
     "paid_parent_saves":live["actor_paid_checkpoint_writes"],
     "attempted_paid_pc102_restore":live["rollback_attempt_records"].as_array().unwrap().len(),
     "actual_physical_parent_restores":live["genuine_paid_physical_parent_restores"],
     "later_paid_source_confirmation":live["later_actual_rollback_readbacks"].as_array().unwrap().len(),
     "live_cost":live["steps"],"same_source_suppress_PC102_cost":sup_rollback["steps"],
     "different_source_C9_no_rollback_cost":old["steps"],
     "different_source_C9_never_edit_cost":fixed_never["steps"]}));
  truthful.extend([live,sup_rollback,sup_edit,placebo,old,fixed_never]);
 }
 for arm in controls{
  let v=run(&m,arm,&dir);
  total_fee+=v["steps"].as_u64().unwrap();
  adversarial.push(v);
 }
 let output=json!({
  "study":"CL1B-G2-ACTOR-PAID-STATE-TRIGGERED-ROLLBACK-C10",
  "status":"ORIGINAL_NATIVE_PID1_STATE_TRIGGERED_REAL_PARENT_SOURCE_ROLLBACK",
  "frozen_manifest":"cl1b/studies/CL1B-G2-ACTOR-PAID-STATE-TRIGGERED-ROLLBACK-C10-frozen-274-worlds-v1.json",
  "native_worlds":274,"total_private_paid_cil":total_fee,
  "failed_truthful_worlds":incomplete,
  "actor_paid_physical_parent_checkpoints":parent_saves,
  "actor_paid_PC102_restore_attempts":charged_rollback_attempts,
  "actual_native_physical_parent_restorations":actual_source_reverts,
  "later_paid_true_PC173_readbacks_of_restored_parent":actor_readback_after_restore,
  "host_research_suppressed_pc102_effects":host_suppressed_pc102,
  "host_research_suppressed_pc28_effects":host_suppressed_pc28,
  "strict_same_source_first_pc102_suppression_prefixes":prefixes_pc102,
  "unexposed_rollback_suppressor_pairs":no_pc102_exposures,
  "costs":{
    "G2_rollback":cost[0],
    "G2_rollback_pc102_suppressed":cost[1],
    "G2_rollback_pc28_suppressed":cost[2],
    "G2_rollback_placebo180":cost[3],
    "G2_C9_checkpoint_no_rollback":cost[4],
    "G2_C9_checkpoint_abstain":cost[5]
  },
  "same_initial_source_rollback_effect_net_saving":
      cost[1] as i64-cost[0] as i64,
  "paid_rollback_cheaper_than_C9_no_rollback_different_source":cost[0]<cost[4],
  "paid_rollback_cheaper_than_C9_never_edit_different_source":cost[0]<cost[5],
  "per_case":per_case,
  "truthful_records":truthful,"adversarial_records":adversarial
 });
 fs::write(dir.join("cl1b_g2_state_rollback_c10_native_274_worlds.json"),
   serde_json::to_vec_pretty(&output).unwrap()).unwrap();
 println!("C10 ORIGINAL NATIVE: 274 worlds paid={} state-triggered_PC102={} true_parent_restore={} later_physically_readback={} host_no_effect_PC102={} native_costs_rollback/suppress102/C9/never={}/{}/{}/{} failed_truthful={}",
    total_fee,charged_rollback_attempts,actual_source_reverts,actor_readback_after_restore,
    host_suppressed_pc102,cost[0],cost[1],cost[4],cost[5],incomplete.len());
 assert!(incomplete.is_empty(),"C10 failed hidden true-stage completion; retain entire original source charged trace");
 assert!(actual_source_reverts>0&&actor_readback_after_restore>0,
  "C10 physical rollback actor not exposed or never truly read back; no false achievement claim");
 assert!(host_suppressed_pc102>0&&prefixes_pc102>0,
  "C10 source-identical rollback suppression not exposed");
}
