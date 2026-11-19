//! G3Q-C1-P1-L3-I5: original 173-node CIL chooses one native paid source
//! edit based on its own observed structural witness count AND first-gene
//! repair candidate; if the candidate is RESET3, chooses zero instead.
//! Tests 64 frozen three-word genotypes + four causal/adversarial controls.
//! Researcher-authored finite strategy; no self-invented rule search grammar.
use cl0_world::{EventKind,Intervention,Op,Origin,Process,World};
use serde_json::{json,Value};
use std::fs::{self,File};
use std::io::{BufWriter,Write};
const FREEZE:&str="../../../cl1b/studies/B56-F1d1k-G3Q-C1-P1-L3-I5-frozen-173-node-optimal-one-byte-repair-v1.json";
const KEYS:[i64;14]=[7,8,9,11,20,21,32,33,50,52,56,62,63,64];
const CREDIT:u64=8000;

fn decode(v:&Value)->Op{
 let k=v["kind"].as_str().unwrap();
 let tagged=if k=="Halt"{json!("Halt")}else{
  let mut data=serde_json::Map::new();
  for (key,value) in v.as_object().unwrap(){
   if key!="kind"{data.insert(key.clone(),value.clone());}
  }
  let mut wrapped=serde_json::Map::new();
  wrapped.insert(k.to_string(),Value::Object(data));
  Value::Object(wrapped)
 };
 serde_json::from_value(tagged).expect("invalid frozen original CIL")
}
fn gene(world:&World,source_node:usize)->i64{
 match &world.process(1).unwrap().code.nodes[source_node]{
  Op::Const{dst:3,value,next} if (source_node==0&&*next==1)||
        (source_node==3&&*next==4)||(source_node==6&&*next==7)=>*value,
  other=>panic!("original founder executable gene unexpectedly altered: {other:?}")
 }
}
fn structural_modes(words:[i64;3])->Vec<i64>{
 (0..3_i64).map(|trial|{
  let mut accumulator=0_i64;
  for (index,word) in words.into_iter().enumerate(){
   let opcode=if index==1{(word+trial)%4}else{word};
   accumulator=if opcode==3{0}else{(accumulator+opcode)%3};
  }
  accumulator
 }).collect()
}
fn pc(map:&Value,name:&str)->usize{
 map[name].as_u64().unwrap_or_else(||panic!("missing registered original PC {name}")) as usize
}
fn execute_original(
 source:&[Value],sites:&Value,name:&str,seed:[i64;3],
 feedback:&str,suppress:bool,expected:&Value,folder:&std::path::Path
)->Value{
 let mut patched_source=source.to_vec();
 for (idx,pos) in [0,3,6].into_iter().enumerate(){patched_source[pos]["value"]=json!(seed[idx]);}
 let founder=patched_source.iter().map(decode).collect::<Vec<_>>();
 let mut native=World::with_strict_capacity(16,180,300);
 if suppress{native=native.with_native_code_edits_disabled();}
 native.load(Process::new(1,founder.clone(),CREDIT)).unwrap();
 assert_eq!(native.occupied_units(),174);
 let mut trace=BufWriter::new(File::create(folder.join(format!(
  "b56_f1d1k_g3q_c1_p1_l3_i5_native_{name}.jsonl"))).unwrap());
 let mut steps=0_usize;let mut peak=native.occupied_units();
 let mut witness=Vec::<Value>::new();
 let mut resets=Vec::<Value>::new();
 let mut certifications=Vec::<usize>::new();
 let mut services=Vec::<Value>::new();
 let mut patches=Vec::<Value>::new();
 let mut evidence_decisions=Vec::<Value>::new();
 let mut feedback_reads=Vec::<i64>::new();
 let mut feedback_sent=Vec::<i64>::new();
 let mut meta_reads=0_usize;let mut retry_writes=0_usize;
 let mut edited_node:Option<usize>=None;
 while native.step(){
  steps+=1;
  assert!(steps<=CREDIT as usize);
  let ev=native.events().last().unwrap().clone();
  assert!(matches!(ev.origin,Origin::Internal(1)));
  if ev.pc==pc(sites,"witness_count_target_chooser"){
   let process=native.process(1).unwrap();
   assert!(process.registers[1]==0||process.registers[1]==1);
   let chosen=if process.registers[1]==0{6}else{3};
   evidence_decisions.push(json!({"step":steps,"decision":"witness_count",
    "register1":process.registers[1],"node_from_witness":chosen}));
  }
  if ev.pc==pc(sites,"normalized_FIRST3_branch"){
   let process=native.process(1).unwrap();
   evidence_decisions.push(json!({"step":steps,
    "decision":"first_gene_reset_normalization",
    "first_gene":seed[0],"register0":process.registers[0],
    "use_zero_fallback":process.registers[0]==0}));
  }
  if ev.pc==pc(sites,"tail_patch_PatchConst")||
     ev.pc==pc(sites,"middle_or_normalized_tail_PatchConst"){
   assert!(patches.is_empty(),"original PID1 may patch at most one word");
   let node=native.process(1).unwrap().registers[1] as usize;
   assert!(node==3||node==6,"no unauthorized CIL target index");
   if ev.pc==pc(sites,"tail_patch_PatchConst"){assert_eq!(node,6);}
   let after=gene(&native,node);
   let before=seed[if node==3{1}else{2}];
   if suppress {
    assert!(matches!(&ev.kind,EventKind::NativeCodeEditSuppressed{opcode_tag:9}));
    assert_eq!(after,before);
   } else {
    assert!(matches!(&ev.kind,EventKind::Executed));
    edited_node=Some(node);
   }
   patches.push(json!({"step":steps,"pc":ev.pc,"node":node,
    "before":before,"after":after,"edit_suppressed":suppress}));
  }
  let p=native.process(1).unwrap();
  let mut expected_source=founder.clone();
  if let Some(node)=edited_node{
   expected_source[node]=Op::Const{dst:3,value:gene(&native,node),
     next:if node==3{4}else{7}};
  }
  assert_eq!(p.code.nodes,expected_source,
    "only original actor paid selected PatchConst may mutate source");
  assert!(native.process(2).is_none());
  assert_eq!(native.total_compute_inventory(),Some(CREDIT-steps as u64));
  assert!(native.used_cells()<=16&&native.occupied_units()<=300);
  peak=peak.max(native.occupied_units());
  if ev.pc==pc(sites,"paid_meta_Read"){
   assert!(matches!(&ev.kind,EventKind::CellRead{..}));
   meta_reads+=1;
  }
  if ev.pc==pc(sites,"paid_witness_Write"){
   match &ev.kind{
    EventKind::CellWritten{key,value}=>{
     assert!((7..=9).contains(key)&&*value==1);
     witness.push(json!({"step":steps,"key":key,"value":value}));
    },
    other=>panic!("missing real original paid structural flag: {other:?}")
   }
  }
  if [pc(sites,"paid_reset7"),pc(sites,"paid_reset8"),pc(sites,"paid_reset9")]
     .contains(&ev.pc){
   match &ev.kind{
    EventKind::CellWritten{key,value}=>{
     assert!((7..=9).contains(key)&&*value==0);
     resets.push(json!({"step":steps,"key":key,"value":value}));
    },
    other=>panic!("witness isolation requires PID-paid zero: {other:?}")
   }
  }
  if ev.pc==pc(sites,"paid_certificate_pass"){
   assert!(matches!(&ev.kind,EventKind::CellWritten{key:21,value:1}));
   assert_eq!([native.cell(7),native.cell(8),native.cell(9)],
    [Some(1),Some(1),Some(1)]);
   certifications.push(steps);
  }
  if ev.pc==pc(sites,"paid_retry_guard_Write11"){
   assert!(matches!(&ev.kind,EventKind::CellWritten{key:11,value:1}));
   retry_writes+=1;
  }
  if ev.pc==pc(sites,"task_feedback_Read20"){
   match &ev.kind{
    EventKind::CellRead{key:20,value}=>feedback_reads.push(*value),
    other=>panic!("feedback not paid Read20: {other:?}")
   }
  }
  let snapshot=json!(["cil",steps,ev.pc,p.pc,p.registers,p.credits,
    KEYS.map(|key|native.cell(key)),native.occupied_units(),
    gene(&native,3),gene(&native,6)]);
  serde_json::to_writer(&mut trace,&snapshot).unwrap();
  trace.write_all(b"\n").unwrap();
  if ev.pc==pc(sites,"task_Write11"){
   assert_eq!(certifications.len(),1,
     "task service cannot bypass original paid structural certificate");
   let output=match &ev.kind{
    EventKind::CellWritten{key:11,value}=>*value,
    other=>panic!("real hidden-goal service must be PID1 Write11: {other:?}")
   };
   let episode=native.cell(32).unwrap_or(0) as usize;
   assert!(episode<4);
   let truth=i64::from(output==[0_i64,0,2,2][episode]);
   let delivered=match feedback{
    "truth"=>truth,"inverted"=>1-truth,
    other=>panic!("unregistered feedback regime {other}")
   };
   let middle=native.cell(63).unwrap();
   services.push(json!({"step":steps,"episode":episode,
    "middle":middle,"output":output,"truth":truth,"feedback":delivered}));
   feedback_sent.push(delivered);
   native.intervene(Intervention::SetCell{key:20,value:delivered}).unwrap();
   let outside=native.events().last().unwrap();
   assert!(matches!(outside.origin,Origin::External));
   assert!(matches!(&outside.kind,EventKind::Intervened(_)));
   let env=json!(["feedback",steps,episode,[0_i64,0,2,2][episode],
    middle,output,truth,delivered,KEYS.map(|key|native.cell(key)),
    native.occupied_units(),gene(&native,3),gene(&native,6)]);
   serde_json::to_writer(&mut trace,&env).unwrap();
   trace.write_all(b"\n").unwrap();
  }
 }
 trace.flush().unwrap();
 assert!(native.process(1).unwrap().stopped);
 let final_genes=[gene(&native,0),gene(&native,3),gene(&native,6)];
 let modes=structural_modes(final_genes);
 let safe=modes.iter().copied()
   .collect::<std::collections::BTreeSet<_>>().len()==3;
 assert_eq!(safe,certifications.len()==1,
   "false-original structural certificate after changed source");
 assert_eq!(steps,expected["paid_steps"].as_u64().unwrap() as usize);
 assert_eq!(native.process(1).unwrap().pc,expected["halt_pc"].as_u64().unwrap() as usize);
 assert_eq!(json!(final_genes),expected["source_after"]);
 assert_eq!(json!(modes),expected["semantic_three_modes"]);
 assert_eq!(safe,expected["semantic_structural_coverage"].as_bool().unwrap());
 assert_eq!(json!(witness),expected["original_paid_witnesses"]);
 assert_eq!(json!(resets),expected["original_paid_full_reset"]);
 assert_eq!(json!(certifications),expected["original_paid_certification"]);
 assert_eq!(json!(patches),expected["original_paid_source_patches"]);
 assert_eq!(json!(evidence_decisions),expected["original_evidence_decisions"]);
 assert_eq!(json!(services),expected["original_paid_task_services"]);
 assert_eq!(meta_reads,expected["original_paid_meta_reads"].as_u64().unwrap() as usize);
 assert_eq!(json!(feedback_reads),expected["original_paid_feedback_reads"]);
 assert_eq!(feedback_reads,feedback_sent);
 assert_eq!(retry_writes,expected["original_paid_retry_writes"].as_u64().unwrap() as usize);
 assert_eq!(json!(native.cell(21)),expected["actor_claimed_success"]);
 assert_eq!(json!(native.cell(32)),expected["actor_completed_episodes"]);
 assert_eq!(native.used_cells(),expected["occupied_data_cells"].as_u64().unwrap() as usize);
 assert_eq!(peak,expected["peak_occupied"].as_u64().unwrap() as usize);
 assert!(steps<CREDIT as usize);
 let ext=native.events().iter().filter(|e|matches!(e.origin,Origin::External))
     .collect::<Vec<_>>();
 assert_eq!(ext.len(),services.len()+1);
 assert!(matches!(&ext[0].kind,EventKind::Loaded));
 assert!(ext.iter().skip(1).all(|e|matches!(&e.kind,EventKind::Intervened(_))));
 json!({"arm":name,"initial_rule":seed,"paid_original_instructions":steps,
   "original_paid_source_edits":patches,"native_certified":safe,
   "independent_semantic_modes":modes,"original_paid_task_services":services,
   "repair_decisions":evidence_decisions,"witness_resets":resets,
   "peak_occupied":peak,"native_original_PID1_halted":true,
   "actor_claimed_success":native.cell(21),
   "true_task_matches":services.iter().filter(|s|s["truth"]==1).count(),
   "host_source_edits_after_load":0})
}
#[test]
fn g3q_c1_i5_optimal_single_byte_repair_exhaustive64_original_native(){
 let root=std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
 let f:Value=serde_json::from_slice(&fs::read(root.join(FREEZE)).unwrap()).unwrap();
 let source=f["exact_source_CIL"].as_array().unwrap();
 let sites=&f["critical_source_pcs"];
 let original=f["pre_registered_64_original_genotypes"].as_array().unwrap();
 let controls=f["pre_registered_controls"].as_array().unwrap();
 assert_eq!(source.len(),173);
 assert_eq!(original.len(),64);
 assert_eq!(controls.len(),4);
 assert_eq!(f["strict_physics"]["max_data_cells"],16);
 assert_eq!(f["strict_physics"]["max_CIL_source_nodes"],180);
 assert_eq!(f["strict_physics"]["max_total_occupied"],300);
 let folder=root.join("../../cl1b-ci-evidence");
 fs::create_dir_all(&folder).unwrap();
 let mut cases=Vec::<Value>::new();
 let mut repaired_success=0_usize;
 let mut chosen_middle=0_usize;
 let mut chosen_tail=0_usize;
 for (ordinal,arm) in original.iter().enumerate(){
  let seed=arm["seed"].as_array().unwrap();
  let a=seed[0].as_i64().unwrap();
  let b=seed[1].as_i64().unwrap();
  let c=seed[2].as_i64().unwrap();
  assert_eq!(ordinal,((a*4+b)*4+c) as usize);
  let name=format!("a{a}_b{b}_c{c}");
  let obs=execute_original(source,sites,&name,[a,b,c],
     "truth",false,&arm["expected"],&folder);
  let originally_safe=structural_modes([a,b,c]).iter()
     .copied().collect::<std::collections::BTreeSet<_>>().len()==3;
  assert_eq!(originally_safe,arm["initial_safe"].as_bool().unwrap());
  if !originally_safe&&obs["native_certified"]==true{repaired_success+=1;}
  if let Some(patch)=obs["original_paid_source_edits"].as_array().unwrap().first(){
   match patch["node"].as_u64().unwrap(){
    3=>chosen_middle+=1,6=>chosen_tail+=1,
    other=>panic!("unknown original source edit node {other}")
   }
  }
  cases.push(obs);
 }
 assert_eq!(repaired_success,32,"must reach all one-word repairable rules");
 assert_eq!((chosen_middle,chosen_tail),(24,16));
 let mut comparisons=Vec::<Value>::new();
 for c in controls{
  let name=c["name"].as_str().unwrap();
  let disabled=c["disabled"].as_bool().unwrap();
  let obs=execute_original(source,sites,name,[
   c["a"].as_i64().unwrap(),c["b"].as_i64().unwrap(),
   c["c"].as_i64().unwrap()],c["feedback"].as_str().unwrap(),
   disabled,&c["expected"],&folder);
  if disabled{
   assert_eq!(obs["native_certified"],false);
   assert_eq!(obs["original_paid_task_services"].as_array().unwrap().len(),0);
  }else{
   assert_eq!(obs["native_certified"],true);
   assert_eq!(obs["actor_claimed_success"],1);
  }
  comparisons.push(obs);
 }
 let counts=&f["pre_registered_exhaustive_counts"];
 assert_eq!(counts["one_byte_repairable"],32);
 assert_eq!(counts["one_byte_impossible"],8);
 assert_eq!(counts["repaired_safe"],32);
 assert_eq!(counts["false_pass"],0);
 assert_eq!(counts["false_reject"],0);
 fs::write(folder.join("b56_f1d1k_g3q_c1_p1_l3_i5_native_optimal_repair.json"),
  serde_json::to_vec_pretty(&json!({
   "study":"B56-F1d1k-G3Q-C1-P1-L3-I5",
   "level":"NATIVE_173_NODE_GENOME_NORMALIZED_ONE_EDIT_EXHAUSTIVE64_OPTIMAL_REPAIR",
   "source_sha":std::env::var("GITHUB_SHA").unwrap_or_else(|_|"local".to_string()),
   "original64_worlds":cases,
   "controls":comparisons,
   "counts":{"original_genotypes":64,"initially_safe":24,
    "initially_unsafe":40,"one_edit_feasible":32,
    "one_edit_impossible":8,"original_pid_paid_edits":40,
    "source_middle_edits":chosen_middle,"source_tail_edits":chosen_tail,
    "actual_successful_repairs":repaired_success,
    "false_certificates":0,"false_rejections":0},
   "limits":"Bounded researcher-authored repair decision and candidate fallback; original PID1 did not invent algorithm, cross-run ancestry or task truth."
  })).unwrap()).unwrap();
}
