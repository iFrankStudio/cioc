//! C7: 274 original-native CL0 paid executed-Const self-readback worlds, source-identical PC28-effect causality.
//! Every native CIL executes with normal fee. Only researcher off arm prevents
//! the PC28 PatchConst mutation of PC173 (host never provides actor an opcode).
use cl0_world::{EventKind,Intervention,Op,Origin,Process,World};
use serde_json::{json,Value};
use std::fs::{self,File};
use std::io::{BufWriter,Write};

const MANIFEST:&str="../../../cl1b/studies/CL1B-G2-ACTOR-EXECUTED-CODE-READBACK-C7-frozen-274-worlds-v1.json";
const G2:&str="../../../cl1b/studies/CL1B-G2-ACTOR-SELFSTATE-MIRROR-C6-frozen-274-worlds-v1.json";
const CREDIT:u64=6000;
const CELLS:[i64;14]=[11,20,21,30,32,33,34,35,36,37,38,39,40,41];

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
  "G2_readback_attest"|"G2_readback_pc28_suppressed"|"G2_readback_placebo180"=>"frozen_source_g2_readback_attest",
  "G2_readback_always"=>"frozen_source_g2_readback_always",
  "G2_readback_abstain"=>"frozen_source_g2_readback_abstain",
  "G2_intent_mirror_C6"=>"frozen_source_g2_intent_mirror_C6",
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
 if source_name=="G2_readback_pc28_suppressed"{w=w.with_native_patchconst_suppression(1,28,173);}
 if source_name=="G2_readback_placebo180"{w=w.with_native_patchconst_suppression(1,180,173);}
 w.load(Process::new(1,initial,CREDIT)).unwrap();
 assert_eq!(w.occupied_units(),181);
 let file=File::create(dir.join(format!("cl1b_g2_executed_readback_c7_original_{name}.jsonl"))).unwrap();
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
   let suppressed=pc==28&&source_name=="G2_readback_pc28_suppressed";
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
  if pc==52 && source_name!="G2_intent_mirror_C6"{
    assert!(matches!(&event.kind,EventKind::CellWritten{key:41,value}
      if *value==pid.registers[2]));
    let last173=last_executed_const_step.expect("paid C7 Write41 before actual PC173 execution");
    assert!(last173<paid);
    assert_eq!(pid.registers[2],theta(&expected),
       "actor C7 CIL value differs from physical executed Const PC173");
    assert_eq!(w.cell(41),Some(theta(&expected)));
    last_attested_step=Some(paid);
    actor_readback_writes+=1;
    if source_name=="G2_readback_pc28_suppressed"{
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
    assert!(route==27||route==125,"C7 native paid gate invalid target");
    let legacy=source_name=="G2_intent_mirror_C6";
    let key=if legacy{40}else{41};
    let witnessed=w.cell(key).unwrap_or(0);
    let physical=theta(&expected);
    if !legacy{
      attested_gate_checks+=1;
      let prev173=last_executed_const_step.expect("no actual PC173 execution before gate");
      let prev52=last_attested_step.expect("no actor paid source-value readback before gate");
      assert!(prev173<prev52&&prev52<paid,"readback is stale or missing at paid gate");
      assert_eq!(witnessed,physical,
          "actor paid readback cell41 was false at original native gate");
    }
    assert_eq!(pid.registers[0],pid.registers[3]-witnessed,
      "actor's own native CIL Read and Sub did not use witness");
    if witnessed!=physical{attested_gate_mismatches+=1;}
    let expected_route=match source_name{
      "G2_readback_attest"|"G2_readback_pc28_suppressed"|"G2_readback_placebo180"|
      "G2_intent_mirror_C6"=>if pid.registers[0]!=0{27}else{125},
      "G2_readback_always"=>27,
      "G2_readback_abstain"=>125,
      _=>unreachable!()
    };
    assert_eq!(route,expected_route,"original CIL PC67 route differs from charged program");
    if route==27{gate_allow+=1;}else{gate_abstain+=1;}
    paid_gate_records.push(json!({"step":paid,"r3":pid.registers[3],
      "r0":pid.registers[0],"route":route,"witness":witnessed,
      "actual_theta":physical,"evidence_key":key,
      "last_pc173":last_executed_const_step,"last_write":last_attested_step,
      "decision":if route==27{"EDIT"}else{"ABSTAIN"}}));
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
  let label=if pc==28&&source_name=="G2_readback_pc28_suppressed"{ "suppressed_pc28" }
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
   "final_attested_theta":w.cell(41).unwrap_or(0)})
}
#[test]
fn c6_frozen_274_original_native_actor_paid_selfstate_mirror(){
 let root=std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
 let m:Value=serde_json::from_slice(&fs::read(root.join(MANIFEST)).unwrap()).unwrap();
 let parent:Value=serde_json::from_slice(&fs::read(root.join(G2)).unwrap()).unwrap();
 assert_eq!(m["study"],"CL1B-G2-ACTOR-EXECUTED-CODE-READBACK-C7");
 assert_eq!(m["status"],"PRE_NATIVE_FROZEN");
 assert_eq!(m["counts"]["native_worlds"],274);
 let mut expected_mirror=parent["frozen_source_g2_actor_gate"].clone();
 assert_eq!(expected_mirror.as_array().unwrap().len(),180);
 assert_eq!(expected_mirror[24],json!({"kind":"Branch","test":3,"nonzero":0,"zero":0}));
 assert_eq!(expected_mirror[52],json!({"kind":"Branch","test":3,"nonzero":22,"zero":22}));
 assert_eq!(expected_mirror[57],json!({"kind":"Branch","test":3,"nonzero":22,"zero":22}));
 assert_eq!(expected_mirror[79],json!({"kind":"Branch","test":0,"nonzero":44,"zero":44}));
 assert_eq!(expected_mirror[120],json!({"kind":"Branch","test":0,"nonzero":121,"zero":121}));
 expected_mirror[23]["next"]=json!(0);
 expected_mirror[51]["next"]=json!(22);
 expected_mirror[56]["next"]=json!(22);
 expected_mirror[77]["nonzero"]=json!(44);
 expected_mirror[118]["nonzero"]=json!(121);
 expected_mirror[24]=json!({"kind":"Const","dst":2,"value":40,"next":52});
 expected_mirror[52]=json!({"kind":"Write","address":2,"value":3,"next":125});
 expected_mirror[57]=json!({"kind":"Sub","dst":0,"lhs":3,"rhs":0,"next":67});
 expected_mirror[79]=json!({"kind":"Const","dst":2,"value":40,"next":120});
 expected_mirror[120]=json!({"kind":"Read","dst":0,"address":2,"next":57});
 expected_mirror[26]["next"]=json!(79);
 expected_mirror[28]["next"]=json!(24);
 expected_mirror[67]=json!({"kind":"Branch","test":0,"nonzero":27,"zero":125});
 assert_eq!(m["frozen_source_g2_mirror"],expected_mirror,
    "C6 mirror program differs from the frozen thirteen-node surgery");
 let mut expected_always=expected_mirror.clone();expected_always[67]["zero"]=json!(27);
 assert_eq!(m["frozen_source_g2_mirror_always"],expected_always);
 let mut expected_abstain=expected_mirror.clone();expected_abstain[67]["nonzero"]=json!(125);
 assert_eq!(m["frozen_source_g2_mirror_abstain"],expected_abstain);
 assert_eq!(m["frozen_source_g2_legacy_label_gate"],parent["frozen_source_g2_actor_gate"]);
 for source_name in ["G2_mirror","G2_mirror_always","G2_mirror_abstain",
    "G2_legacy_label_gate"]{assert_eq!(source(&m,source_name).len(),180);}
 let arms=m["arms"].as_array().unwrap();
 let controls=m["controls"].as_array().unwrap();
 let cases=m["cases"].as_array().unwrap();
 assert_eq!((arms.len(),controls.len(),cases.len()),(270,4,45));
 let dir=root.join("../../cl1b-ci-evidence");
 fs::create_dir_all(&dir).unwrap();
 let mut truthful=Vec::<Value>::new();
 let mut adversarial=Vec::<Value>::new();
 let mut total_paid=0_u64;
 let mut failed_truth=Vec::<String>::new();
 let mut costs=[0_u64;6];
 let mut mirror_allowed=0_u64;let mut mirror_skipped=0_u64;
 let mut mirror_writes=0_u64;let mut mirror_noops=0_u64;
 let mut poison_mismatch=0_u64;let mut poison_effects=0_u64;
 let mut first_prefixes=0_u64;let mut never_exposed=0_u64;
 for (i,c) in cases.iter().enumerate(){
  let group=&arms[6*i..6*i+6];
  assert_eq!(group.iter().map(|a|a["source"].as_str().unwrap()).collect::<Vec<_>>(),
    vec!["G2_mirror","G2_mirror_always","G2_mirror_abstain",
         "G2_mirror_pc28_suppressed","G2_mirror_placebo180","G2_legacy_label_gate"]);
  for arm in group{assert_eq!(arm["goals"],c["goals"]);}
  let gate=run(&m,&group[0],&dir);
  let always=run(&m,&group[1],&dir);
  let abstain=run(&m,&group[2],&dir);
  let poison=run(&m,&group[3],&dir);
  let placebo=run(&m,&group[4],&dir);
  let legacy=run(&m,&group[5],&dir);
  for (j,x) in [&gate,&always,&abstain,&poison,&placebo,&legacy].iter().enumerate(){
   if x["completed_stages"]!=48 || x["truthful_matches"]!=48 ||
      x["claim"]!=1 || x["budget_exhausted"]==true {
      failed_truth.push(x["name"].as_str().unwrap().to_owned());
   }
   costs[j]+=x["steps"].as_u64().unwrap();
   total_paid+=x["steps"].as_u64().unwrap();
  }
  mirror_allowed+=gate["paid_gate_allow"].as_u64().unwrap();
  mirror_skipped+=gate["paid_gate_abstain"].as_u64().unwrap();
  mirror_writes+=gate["paid_mirror_writes"].as_u64().unwrap();
  mirror_noops+=gate["mirror_noop_skips"].as_u64().unwrap();
  poison_mismatch+=poison["mirror_gate_mismatches"].as_u64().unwrap();
  let suppressed=poison["suppressed_pc28"].as_array().unwrap();
  poison_effects+=suppressed.len() as u64;
  assert_eq!(gate["paid_gate_allow"],gate["paid_mirror_writes"]);
  assert_eq!(always["paid_gate_allow"],always["paid_mirror_writes"]);
  assert_eq!(abstain["paid_gate_allow"],0);
  assert_eq!(abstain["paid_mirror_writes"],0);
  assert_eq!(always["paid_gate_abstain"],0);
  for x in [&gate,&always,&abstain,&placebo]{
    assert_eq!(x["mirror_gate_mismatches"],0,
       "live mirror must track actual PC173 parameter with all CIL fees charged");
  }
  assert_eq!(gate["steps"],placebo["steps"]);
  assert_eq!(gate["services"],placebo["services"]);
  assert_eq!(gate["paid_gate_records"],placebo["paid_gate_records"]);
  assert_eq!(gate["parameter_patches"],placebo["parameter_patches"]);
  assert_eq!(gate["final_edges"],placebo["final_edges"]);
  let raw=|r:&Value| -> Vec<Value> {
   let name=r["name"].as_str().unwrap();
   fs::read_to_string(dir.join(format!("cl1b_g2_executed_readback_c7_original_{name}.jsonl"))).unwrap()
     .lines().map(|line|serde_json::from_str(line).unwrap()).collect()
  };
  let tgate=raw(&gate);let tplacebo=raw(&placebo);let tpoison=raw(&poison);
  assert_eq!(tgate,tplacebo,"unreachable native PC180 must not affect exact event history");
  if suppressed.is_empty(){
    never_exposed+=1;
    assert_eq!(tgate,tpoison,"unexposed poisoned comparison must be trace-identical");
  } else{
    let first_step=suppressed[0]["step"].as_u64().unwrap();
    let first_index=tpoison.iter().position(|x|
       x[0]=="cil"&&x[1]==first_step&&x[2]==28).unwrap();
    assert_eq!(tgate[..first_index],tpoison[..first_index],
       "identical original code must share all paid CIL/feedback before first suppressed patch");
    assert_eq!(tpoison[first_index][10],"suppressed_pc28");
    first_prefixes+=1;
  }
  truthful.extend([gate,always,abstain,poison,placebo,legacy]);
 }
 for arm in controls{
   let x=run(&m,arm,&dir);
   total_paid+=x["steps"].as_u64().unwrap();
   adversarial.push(x);
 }
 assert!(mirror_allowed>0&&mirror_skipped>0,"no real mirror edit/skip branch coverage");
 assert_eq!(mirror_allowed,mirror_writes);
 assert_eq!(mirror_skipped,mirror_noops,"normal mirror should only skip matching parameters");
 assert!(poison_effects>0&&first_prefixes>0,
   "no exposure of paid researcher suppression; poisoned mirror control invalid");
 let output=json!({
  "study":"CL1B-G2-ACTOR-EXECUTED-CODE-READBACK-C7",
  "status":"ORIGINAL_NATIVE_ACTOR_PAID_STATE_MIRROR",
  "frozen_manifest":"cl1b/studies/CL1B-G2-ACTOR-EXECUTED-CODE-READBACK-C7-frozen-274-worlds-v1.json",
  "total_original_native_worlds":274,
  "incomplete_truthful_world_names":failed_truth,
  "total_private_paid_cil":total_paid,
  "actor_mirror_gate_allow":mirror_allowed,
  "actor_mirror_gate_abstain":mirror_skipped,
  "actual_mirror_parameter_writes":mirror_writes,
  "skipped_paid_same_parameter_edits":mirror_noops,
  "poisoned_shadow_mismatch_gate_events":poison_mismatch,
  "poisoned_suppressed_pc28_attempts":poison_effects,
  "first_native_suppression_prefix_witnesses":first_prefixes,
  "unexposed_poison_pairs":never_exposed,
  "costs":{
    "G2_mirror":costs[0],"G2_mirror_always":costs[1],
    "G2_mirror_abstain":costs[2],"G2_mirror_pc28_suppressed":costs[3],
    "G2_mirror_placebo180":costs[4],"G2_legacy_label_gate":costs[5]
  },
  "mirror_net_saving_vs_always":costs[1] as i64-costs[0] as i64,
  "mirror_net_saving_vs_abstain":costs[2] as i64-costs[0] as i64,
  "main_efficiency_accepted":costs[0]<costs[1]&&costs[0]<costs[2],
  "truthful_records":truthful,"adversarial_records":adversarial
 });
 fs::write(dir.join("cl1b_g2_executed_readback_c7_native_274_worlds.json"),
   serde_json::to_vec_pretty(&output).unwrap()).unwrap();
 println!("C6 ORIGINAL native 274 worlds, paid={}, mirror allow/skip={}/{}, mirror writes={}, noop skips={}, shadow poison mismatches={}, costs mirror/always/abstain={}/{}/{}, beats both={}",
   total_paid,mirror_allowed,mirror_skipped,mirror_writes,mirror_noops,
   poison_mismatch,costs[0],costs[1],costs[2],
   costs[0]<costs[1]&&costs[0]<costs[2]);
 assert!(failed_truth.is_empty(),
   "C6 original-native true-stage feasibility FAILED; preserve full trace evidence");
}
