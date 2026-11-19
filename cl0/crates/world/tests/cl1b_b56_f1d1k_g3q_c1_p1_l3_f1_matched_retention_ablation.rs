//! G3Q-C1-L3-F1: equal 122-node original PID1 feedback-memory ON/OFF.
 //! Researcher pre-load ablation, NOT an actor self-edit; truthful hidden goals.
use cl0_world::{EventKind,Intervention,Op,Origin,Process,World};
use serde_json::{json,Value};
use std::fs::{self,File};
use std::io::{BufWriter,Write};
const F0:&str="../../../cl1b/studies/B56-F1d1k-G3Q-C1-P1-L3-F0-frozen-122-node-feedback-relearning-founder-v1.json";
const F1:&str="../../../cl1b/studies/B56-F1d1k-G3Q-C1-P1-L3-F1-frozen-122-node-matched-retention-ablation-v1.json";
const KEYS:[i64;13]=[11,20,21,32,33,50,51,52,53,56,62,63,64];
const CREDITS:u64=8000;
fn decode(v:&Value)->Op{
 let tag=v["kind"].as_str().unwrap();
 let tagged=if tag=="Halt"{json!("Halt")}else{
  let mut data=serde_json::Map::new();
  for (key,value) in v.as_object().unwrap(){if key!="kind"{data.insert(key.clone(),value.clone());}}
  let mut out=serde_json::Map::new();out.insert(tag.to_string(),Value::Object(data));
  Value::Object(out)
 };
 serde_json::from_value(tagged).expect("invalid preregistered founder")
}
#[test]
fn g3q_c1_l3_f1_same_physics_equal_length_memory_on_off_causal_comparison(){
 let root=std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
 let a:Value=serde_json::from_slice(&fs::read(root.join(F0)).unwrap()).unwrap();
 let b:Value=serde_json::from_slice(&fs::read(root.join(F1)).unwrap()).unwrap();
 let on=a["exact_native_founder_code"].as_array().unwrap();
 let off=b["exact_preload_control_source"].as_array().unwrap();
 assert_eq!(on.len(),122);assert_eq!(off.len(),122);
 assert_eq!(on[93],b["only_modified_opcodes"][0]["original"]);
 assert_eq!(on[94],b["only_modified_opcodes"][1]["original"]);
 assert_eq!(off[93],b["only_modified_opcodes"][0]["control"]);
 assert_eq!(off[94],b["only_modified_opcodes"][1]["control"]);
 for pc in 0..122{if pc!=93&&pc!=94{assert_eq!(on[pc],off[pc], "unexpected source mismatch PC {pc}");}}
 let cases=b["goal_tasks_and_source_model_predictions"].as_array().unwrap();
 assert_eq!(cases.len(),4);
 let folder=root.join("../../cl1b-ci-evidence");
 fs::create_dir_all(&folder).unwrap();
 let mut observations=Vec::<Value>::new();
 for (policy,source) in [("on",on),("off",off)]{
  let founder:Vec<Op>=source.iter().map(decode).collect();
  for case in cases{
   let name=case["name"].as_str().unwrap();
   let goals=case["goals"].as_array().unwrap().iter()
      .map(|x|x.as_i64().unwrap()).collect::<Vec<_>>();
   assert_eq!(goals.len(),4);
   let expected=&case[policy];
   let mut world=World::with_strict_capacity(16,180,300);
   world.load(Process::new(1,founder.clone(),CREDITS)).unwrap();
   let mut trace=BufWriter::new(File::create(folder.join(format!(
       "b56_f1d1k_g3q_c1_p1_l3_f1_native_{policy}_{name}.jsonl"))).unwrap());
   let mut steps=0_usize;
   let mut probes=[0_usize;4];
   let mut services=Vec::<Value>::new();
   let mut true_successes=0_usize;
   let mut original_reads=Vec::<i64>::new();
   let mut feedbacks=Vec::<i64>::new();
   let mut meta_fetches=0_usize;
   let mut accumulator=0_usize;
   while world.step(){
    steps+=1;assert!(steps<=CREDITS as usize);
    let event=world.events().last().unwrap().clone();
    assert!(matches!(event.origin,Origin::Internal(1)));
    let p=world.process(1).unwrap();
    assert_eq!(p.code.nodes,founder,"original native source must not be modified");
    assert_eq!(world.total_compute_inventory(),Some(CREDITS-steps as u64));
    assert!(world.process(2).is_none());
    assert!(world.used_cells()<=16);
    assert!(world.occupied_units()<=300);
    if event.pc==37{assert!(matches!(&event.kind,EventKind::CellRead{..}));meta_fetches+=1;}
    if event.pc==58{assert!(matches!(&event.kind,EventKind::CellWritten{key:56,..}));accumulator+=1;}
    if event.pc==79{
     match &event.kind{
      EventKind::CellRead{key:20,value}=>original_reads.push(*value),
      other=>panic!("original PID did not pay for feedback: {other:?}")
     }
    }
    serde_json::to_writer(&mut trace,&json!(["cil",steps,event.pc,p.pc,
       p.registers,p.credits,KEYS.map(|k|world.cell(k)),world.occupied_units()])).unwrap();
    trace.write_all(b"\n").unwrap();
    if event.pc==71{
     let result=match &event.kind{
      EventKind::CellWritten{key:11,value}=>*value,
      other=>panic!("task service not originally paid Write11: {other:?}")
     };
     let episode=world.cell(32).unwrap() as usize;
     assert!(episode<4);
     let trial=world.cell(63).unwrap();
     assert!((0..=2).contains(&trial));
     let truth=i64::from(result==goals[episode]);
     true_successes+=truth as usize;
     probes[episode]+=1;assert!(probes[episode]<=3);
     services.push(json!([episode,goals[episode],trial,result,truth,truth,steps]));
     feedbacks.push(truth);
     world.intervene(Intervention::SetCell{key:20,value:truth}).unwrap();
     let environmental=world.events().last().unwrap();
     assert!(matches!(environmental.origin,Origin::External));
     assert!(matches!(&environmental.kind,EventKind::Intervened(_)));
     serde_json::to_writer(&mut trace,&json!(["feedback",steps,episode,
        goals[episode],trial,result,truth,truth,
        KEYS.map(|k|world.cell(k)),world.occupied_units()])).unwrap();
     trace.write_all(b"\n").unwrap();
    }
   }
   trace.flush().unwrap();
   assert!(world.process(1).unwrap().stopped);
   assert_eq!(steps,expected["steps"].as_u64().unwrap() as usize);
   assert_eq!(probes.iter().copied().filter(|n|*n>0).collect::<Vec<_>>(),
    expected["probes"].as_array().unwrap().iter()
     .map(|x|x.as_u64().unwrap() as usize).collect::<Vec<_>>());
   assert_eq!(true_successes,expected["true_successes"].as_u64().unwrap() as usize);
   assert_eq!(services.len(),probes.iter().sum::<usize>());
   assert_eq!(original_reads,feedbacks);
   assert_eq!(meta_fetches,3*services.len());
   assert_eq!(accumulator,3*services.len());
   assert_eq!(world.used_cells(),13);
   assert_eq!(world.occupied_units(),136);
   assert_eq!(world.cell(21),Some(i64::from(true_successes==4)));
   let external=world.events().iter().filter(|e|
     matches!(e.origin,Origin::External)).collect::<Vec<_>>();
   assert_eq!(external.len(),services.len()+1);
   assert!(matches!(&external[0].kind,EventKind::Loaded));
   observations.push(json!({
    "arm":format!("{policy}_{name}"),"policy":policy,"case":name,
    "paid_steps":steps,"credits_remaining":CREDITS-steps as u64,
    "service_events":services,"episode_probes":probes,
    "true_task_matches":true_successes,
    "original_paid_feedback_reads":original_reads,
    "meta_fetches":meta_fetches,"accumulator_checkpoints":accumulator,
    "actor_claimed_success":world.cell(21),"occupied_units":world.occupied_units(),
    "native_original_pid1_halted":true,"source_nodes":founder.len()
   }));
  }
 }
 fs::write(folder.join("b56_f1d1k_g3q_c1_p1_l3_f1_native_retention_ablation.json"),
   serde_json::to_vec_pretty(&json!({
    "study":"B56-F1d1k-G3Q-C1-P1-L3-F1",
    "level":"NATIVE_EQUAL_LENGTH_PRELOAD_RETENTION_MEMORY_ABLATION",
    "source_sha":std::env::var("GITHUB_SHA").unwrap_or_else(|_|"local".to_string()),
    "records":observations,
    "limits":"Two-node researcher source ablation; not same-source dynamic genotype change, C0 earning, or unconditional advantage."
   })).unwrap()).unwrap();
}
