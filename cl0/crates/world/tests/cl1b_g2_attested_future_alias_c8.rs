//! C8: same-attested-CIL invisible future shift full-paid outcome alias, source-identical PC28-effect causality.
//! Every native CIL executes with normal fee. Only researcher off arm prevents
//! the PC28 PatchConst mutation of PC173 (host never provides actor an opcode).
use cl0_world::{EventKind,Intervention,Op,Origin,Process,World};
use serde_json::{json,Value};
use std::fs::{self,File};
use std::io::{BufWriter,Write};

const MANIFEST:&str="../../../cl1b/studies/CL1B-G2-ATTESTED-VALUE-FUTURE-ALIAS-C8-frozen-274-worlds-v1.json";
const G2:&str="../../../cl1b/studies/CL1B-G2-ACTOR-EXECUTED-CODE-READBACK-C7-frozen-274-worlds-v1.json";
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
 let file=File::create(dir.join(format!("cl1b_g2_attested_future_alias_c8_original_{name}.jsonl"))).unwrap();
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
fn c8_native_274_attested_source_same_history_future_value_alias(){
 let root=std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
 let m:Value=serde_json::from_slice(&fs::read(root.join(MANIFEST)).unwrap()).unwrap();
 let parent:Value=serde_json::from_slice(&fs::read(root.join(G2)).unwrap()).unwrap();
 assert_eq!(m["study"],"CL1B-G2-ATTESTED-VALUE-FUTURE-ALIAS-C8");
 assert_eq!(m["status"],"PRE_NATIVE_FROZEN");
 assert_eq!(m["counts"]["native_worlds"],274);
 for field in ["frozen_source_g2_readback_attest","frozen_source_g2_readback_always",
               "frozen_source_g2_readback_abstain","frozen_source_g2_intent_mirror_C6"]{
  assert_eq!(m[field],parent[field],
     "C8 cannot mutate or replace C7 180-node proven original native actor source");
 }
 assert_eq!(m["counts"]["anchors"],15);
 assert_eq!(m["counts"]["alternate_futures_per_anchor"],3);
 for name in ["G2_readback_attest","G2_readback_always",
              "G2_readback_abstain","G2_intent_mirror_C6"]{
   assert_eq!(source(&m,name).len(),180);
 }
 let arms=m["arms"].as_array().unwrap();
 let controls=m["controls"].as_array().unwrap();
 let cases=m["cases"].as_array().unwrap();
 assert_eq!((cases.len(),arms.len(),controls.len()),(45,270,4));
 let dir=root.join("../../cl1b-ci-evidence");
 fs::create_dir_all(&dir).unwrap();
 let mut truthful=Vec::<Value>::new();let mut adversarial=Vec::<Value>::new();
 let mut total_paid=0_u64;
 let mut incomplete=Vec::<String>::new();
 let mut costs=[0_u64;6];
 let mut readback_writes=0_u64;let mut attested_checks=0_u64;
 let mut attested_mismatches=0_u64;let mut suppressed_attested=0_u64;
 let mut suppressions=0_u64;let mut first_prefixes=0_u64;
 let mut no_suppression=0_u64;
 let mut allow=0_u64;let mut abstain=0_u64;
 for (i,c) in cases.iter().enumerate(){
  let group=&arms[6*i..6*i+6];
  assert_eq!(group.iter().map(|a|a["source"].as_str().unwrap()).collect::<Vec<_>>(),
    vec!["G2_readback_attest","G2_readback_always","G2_readback_abstain",
         "G2_readback_pc28_suppressed","G2_readback_placebo180","G2_intent_mirror_C6"]);
  for arm in group{assert_eq!(arm["goals"],c["goals"]);}
  let gate=run(&m,&group[0],&dir);
  let always=run(&m,&group[1],&dir);
  let never=run(&m,&group[2],&dir);
  let suppressed=run(&m,&group[3],&dir);
  let placebo=run(&m,&group[4],&dir);
  let old=run(&m,&group[5],&dir);
  for (j,x) in [&gate,&always,&never,&suppressed,&placebo,&old].iter().enumerate(){
   if x["completed_stages"]!=48||x["truthful_matches"]!=48||
      x["claim"]!=1||x["budget_exhausted"]==true{
     incomplete.push(x["name"].as_str().unwrap().to_owned());
   }
   total_paid+=x["steps"].as_u64().unwrap();
   costs[j]+=x["steps"].as_u64().unwrap();
  }
  allow+=gate["paid_gate_allow"].as_u64().unwrap();
  abstain+=gate["paid_gate_abstain"].as_u64().unwrap();
  readback_writes+=gate["actor_readback_writes"].as_u64().unwrap();
  attested_checks+=gate["attested_gate_checks"].as_u64().unwrap();
  for x in [&gate,&always,&never,&suppressed,&placebo]{
   attested_mismatches+=x["attested_gate_mismatches"].as_u64().unwrap();
   assert_eq!(x["attested_gate_mismatches"],0,
     "readback must represent true executed physical theta, including suppressed-edit arm");
   assert_eq!(x["attested_gate_checks"],
     x["paid_gate_allow"].as_u64().unwrap()+x["paid_gate_abstain"].as_u64().unwrap());
   assert_eq!(x["actor_readback_writes"],x["attested_gate_checks"],
     "C7 native paid PC173 and PC52 readback must precede each PC67 gate exactly once");
  }
  suppressed_attested+=suppressed["attested_after_suppressed_patch"].as_u64().unwrap();
  suppressions+=suppressed["suppressed_pc28"].as_array().unwrap().len() as u64;
  assert_eq!(always["paid_gate_abstain"],0);
  assert_eq!(never["paid_gate_allow"],0);
  assert_eq!(gate["steps"],placebo["steps"]);
  assert_eq!(gate["services"],placebo["services"]);
  assert_eq!(gate["actor_readback_records"],placebo["actor_readback_records"]);
  assert_eq!(gate["paid_gate_records"],placebo["paid_gate_records"]);
  assert_eq!(gate["parameter_patches"],placebo["parameter_patches"]);
  let raw=|x:&Value| -> Vec<Value>{
    let name=x["name"].as_str().unwrap();
    fs::read_to_string(dir.join(format!("cl1b_g2_attested_future_alias_c8_original_{name}.jsonl"))).unwrap()
       .lines().map(|row|serde_json::from_str(row).unwrap()).collect()
  };
  let tgate=raw(&gate);let tplacebo=raw(&placebo);let tsuppressed=raw(&suppressed);
  assert_eq!(tgate,tplacebo,"inaccessible PC180 placebo changed native paid original CIL events");
  let sf=suppressed["suppressed_pc28"].as_array().unwrap();
  if sf.is_empty(){
    no_suppression+=1;
    assert_eq!(tgate,tsuppressed,"identical-source unexposed native worlds must match");
  }else{
    let paid=sf[0]["step"].as_u64().unwrap();
    let j=tsuppressed.iter().position(|x|
       x[0]=="cil"&&x[1]==paid&&x[2]==28).expect("suppressed paid PC28 row not found");
    assert_eq!(tgate[..j],tsuppressed[..j],
      "source-identical paid native history must match until first effect suppression");
    assert_eq!(tsuppressed[j][10],"suppressed_pc28");
    first_prefixes+=1;
  }
  truthful.extend([gate,always,never,suppressed,placebo,old]);
 }
 for c in controls{
   let x=run(&m,c,&dir);
   total_paid+=x["steps"].as_u64().unwrap();
   adversarial.push(x);
 }
 assert!(allow>0&&abstain>0,"genuine native paid actor branches not both exercised");
 assert!(readback_writes>0&&attested_checks>0);
 assert_eq!(attested_mismatches,0);
 assert!(suppressions>0&&first_prefixes>0&&suppressed_attested>0,
   "no original-native same-source suppressed PatchConst with later true self-code readback");
 // New C8 causal-identifiability question: the actor cannot see future goals.
 // This is an exact 180-pair same-arm full ORIGINAL native trace-prefix test,
 // not just a comparison of summarized feedback or program source hashes.
 let mut prefix_exact_equal_pairs=0_u64;
 let mut source_attested_before_fork=0_u64;
 let mut live_patch_before_fork=0_u64;
 let mut positive=0_u64;let mut negative=0_u64;let mut zero=0_u64;
 let mut anchors_flipping_signed_value=0_u64;
 let mut exposed_flipping_signed_value=0_u64;
 let mut future_alias_records=Vec::<Value>::new();
 let native_trace=|summary:&Value|->Vec<Value>{
   let name=summary["name"].as_str().unwrap();
   fs::read_to_string(dir.join(format!("cl1b_g2_attested_future_alias_c8_original_{name}.jsonl"))).unwrap()
     .lines().map(|line|serde_json::from_str(line).unwrap()).collect()
 };
 for (anchor_index,anchor) in m["anchors"].as_array().unwrap().iter().enumerate(){
  let prefix=anchor["prefix"].as_array().unwrap();
  assert_eq!(prefix.len(),24);
  for shift in 0..3{
   let actual=&cases[3*anchor_index+shift]["goals"];
   assert_eq!(&actual.as_array().unwrap()[..24],&prefix[..],
     "new hidden future changed the earlier real service contract");
  }
  assert_ne!(cases[3*anchor_index]["goals"],cases[3*anchor_index+1]["goals"]);
  assert_ne!(cases[3*anchor_index]["goals"],cases[3*anchor_index+2]["goals"]);
  assert_ne!(cases[3*anchor_index+1]["goals"],cases[3*anchor_index+2]["goals"]);
  for arm_index in 0..6{
   let baseline=&truthful[18*anchor_index+arm_index];
   let canonical=native_trace(baseline);
   let boundary_index=canonical.iter().position(|r|r[0]=="feedback"&&r[2]==24)
      .expect("native first stage24 feedback row missing");
   let first24=&canonical[..boundary_index];
   assert!(first24.iter().any(|r|r[0]=="cil"&&r[2]==173),
     "future fork must occur after real paid physical Const execution");
   for shift in 1..3{
    let matched=native_trace(&truthful[18*anchor_index+6*shift+arm_index]);
    let changed=matched.iter().position(|r|r[0]=="feedback"&&r[2]==24)
       .expect("future variant missing first stage24 feedback");
    assert_eq!(first24,&matched[..changed],
       "real original-native CIL+feedback history before hidden future fork is distinguishable");
    prefix_exact_equal_pairs+=1;
   }
  }
  let baseline=native_trace(&truthful[18*anchor_index]);
  let fork_paid=baseline.iter().find(|r|r[0]=="feedback"&&r[2]==24).unwrap()[1].as_u64().unwrap();
  let first_live=&truthful[18*anchor_index];
  let source_witnessed=first_live["actor_readback_records"].as_array().unwrap().iter()
      .any(|r|r["step"].as_u64().unwrap()<fork_paid);
  let patch_exposed=first_live["parameter_patches"].as_array().unwrap().iter()
      .any(|r|r["step"].as_u64().unwrap()<fork_paid);
  if source_witnessed{source_attested_before_fork+=1;}
  if patch_exposed{live_patch_before_fork+=1;}
  let mut deltas=Vec::<i64>::new();
  let mut paid_costs=Vec::<Value>::new();
  for shift in 0..3{
   let index=18*anchor_index+6*shift;
   let live=truthful[index]["steps"].as_i64().unwrap();
   let off=truthful[index+3]["steps"].as_i64().unwrap();
   let d=off-live;
   deltas.push(d);
   paid_costs.push(json!({"future":cases[3*anchor_index+shift]["tail"],
     "live":live,"suppressed":off,"same_source_suppressed_minus_live":d,
     "privileged_hindsight_choice":if d>0{"allow_all"}else if d<0{"suppress_all"}else{"either"}}));
   if d>0{positive+=1;}else if d<0{negative+=1;}else{zero+=1;}
  }
  let sign_flip=deltas.iter().any(|x|*x>0)&&deltas.iter().any(|x|*x<0);
  if sign_flip{anchors_flipping_signed_value+=1;}
  if sign_flip&&patch_exposed&&source_witnessed{exposed_flipping_signed_value+=1;}
  future_alias_records.push(json!({
   "anchor":anchor["anchor"],"regime":anchor["regime"],
   "paid_code_readback_before_fork":source_witnessed,
   "paid_pc28_patch_before_fork":patch_exposed,
   "branch_before_future_first_feedback_paid_instruction":fork_paid,
   "native_fork_stage":24,
   "exact_complete_original_native_prefix_equal_across_all_six_arms":true,
   "signed_same_source_pc28_effect_full_lifecycle":deltas,
   "strict_positive_and_negative_future_sign_flip":sign_flip,
   "future_paid_outcomes":paid_costs
  }));
 }
 assert_eq!(prefix_exact_equal_pairs,15*6*2,
    "C8 90 original-native same-prefix alias comparisons incomplete");
 assert_eq!(positive+negative+zero,45);
 let output=json!({
  "study":"CL1B-G2-ATTESTED-VALUE-FUTURE-ALIAS-C8",
  "status":"ORIGINAL_NATIVE_ACTOR_PAID_EXECUTED_CONST_SELF_READBACK",
  "total_original_native_worlds":274,
  "frozen_manifest":"cl1b/studies/CL1B-G2-ATTESTED-VALUE-FUTURE-ALIAS-C8-frozen-274-worlds-v1.json",
  "total_private_paid_cil":total_paid,
  "incomplete_truthful_world_names":incomplete,
  "actor_paid_readback_writes":readback_writes,
  "actor_attested_paid_gate_checks":attested_checks,
  "actor_attested_gate_mismatches_all_matching_arms":attested_mismatches,
  "actor_normal_gate_edits":allow,"actor_normal_gate_abstains":abstain,
  "actual_paid_pc28_effect_suppressions":suppressions,
  "true_physical_attestations_after_effect_suppression":suppressed_attested,
  "first_paid_suppression_prefix_witnesses":first_prefixes,
  "no_suppression_pairs":no_suppression,
  "prefix_alias_pairs_exact":prefix_exact_equal_pairs,
  "total_anchor_histories":15,
  "anchors_paid_source_readback_before_fork":source_attested_before_fork,
  "anchors_paid_pc28_before_fork":live_patch_before_fork,
  "future_benefit_positive":positive,
  "future_benefit_negative":negative,
  "future_benefit_tied":zero,
  "anchors_with_both_effect_signs":anchors_flipping_signed_value,
  "exposed_attested_anchors_with_both_effect_signs":exposed_flipping_signed_value,
  "future_alias_records":future_alias_records,
  "costs":{
    "G2_readback_attest":costs[0],
    "G2_readback_always":costs[1],
    "G2_readback_abstain":costs[2],
    "G2_readback_pc28_suppressed":costs[3],
    "G2_readback_placebo180":costs[4],
    "G2_intent_mirror_C6":costs[5]
  },
  "readback_net_saving_vs_always":costs[1] as i64-costs[0] as i64,
  "readback_net_saving_vs_abstain":costs[2] as i64-costs[0] as i64,
  "causal_suppressed_minus_readback_cost":costs[3] as i64-costs[0] as i64,
  "main_efficiency_accepted":costs[0]<costs[1]&&costs[0]<costs[2],
  "truthful_records":truthful,"adversarial_records":adversarial
 });
 fs::write(dir.join("cl1b_g2_attested_future_alias_c8_native_274_worlds.json"),
   serde_json::to_vec_pretty(&output).unwrap()).unwrap();
 println!("C8 ORIGINAL future-alias 274 worlds; paid={}; live execution readbacks={}; native gates={}/{}; suppressed readback-after-edit={}; all-armed attested mismatches={}; cost gate/always/never={}/{}/{}; accepted={}",
   total_paid,readback_writes,allow,abstain,suppressed_attested,
   attested_mismatches,costs[0],costs[1],costs[2],
   costs[0]<costs[1]&&costs[0]<costs[2]);
 assert!(incomplete.is_empty(),"C8 truthful completion feasibility failure; preserve original native evidence");
}
