//! G3Q-C1-L3-F2 native observational alias of honest and deceptive feedback.
 //! The hidden objective belongs only to the researcher task harness.
 //! Identical original PID1 source/feedback/events can encode opposite truth.
use cl0_world::{EventKind,Intervention,Op,Origin,Process,World};
use serde_json::{json,Value};
use std::fs::{self,File};
use std::io::{BufWriter,Write};
const SOURCE:&str="../../../cl1b/studies/B56-F1d1k-G3Q-C1-P1-L3-F0-frozen-122-node-feedback-relearning-founder-v1.json";
const PREREG:&str="../../../cl1b/studies/B56-F1d1k-G3Q-C1-P1-L3-F2-frozen-feedback-truth-observational-alias-preregistration.json";
const KEYS:[i64;13]=[11,20,21,32,33,50,51,52,53,56,62,63,64];
const CREDITS:u64=8000;
fn decode(v:&Value)->Op{
 let tag=v["kind"].as_str().unwrap();
 let encoded=if tag=="Halt"{json!("Halt")}else{
  let mut f=serde_json::Map::new();
  for (key,val) in v.as_object().unwrap(){if key!="kind"{f.insert(key.clone(),val.clone());}}
  let mut wrapper=serde_json::Map::new();wrapper.insert(tag.to_string(),Value::Object(f));Value::Object(wrapper)
 };
 serde_json::from_value(encoded).expect("invalid original precommitted CIL source")
}
#[test]
fn g3q_c1_l3_f2_original_native_full_world_alias_despite_opposite_task_truth(){
 let root=std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
 let p:Value=serde_json::from_slice(&fs::read(root.join(PREREG)).unwrap()).unwrap();
 let f:Value=serde_json::from_slice(&fs::read(root.join(SOURCE)).unwrap()).unwrap();
 let founder:Vec<Op>=f["exact_native_founder_code"].as_array().unwrap()
  .iter().map(decode).collect();
 assert_eq!(founder.len(),122);
 let arms=p["pairs"].as_array().unwrap();
 assert_eq!(arms.len(),2);
 let folder=root.join("../../cl1b-ci-evidence");
 fs::create_dir_all(&folder).unwrap();
 let mut world_out:Vec<World>=Vec::new();
 let mut actor_out:Vec<Vec<Value>>=Vec::new();
 let mut evidence=Vec::<Value>::new();
 for arm in arms{
  let label=arm["name"].as_str().unwrap();
  let goals=arm["hidden_goals"].as_array().unwrap().iter()
   .map(|v|v.as_i64().unwrap()).collect::<Vec<_>>();
  let feedback_regime=arm["feedback_regime"].as_str().unwrap();
  let expected=&arm["native_expected"];
  let mut world=World::with_strict_capacity(16,180,300);
  world.load(Process::new(1,founder.clone(),CREDITS)).unwrap();
  let mut original_visible=Vec::<Value>::new();
  let mut stream=BufWriter::new(File::create(folder.join(format!(
   "b56_f1d1k_g3q_c1_p1_l3_f2_native_paid_{label}.jsonl"))).unwrap());
  let mut steps=0_usize;
  let mut true_count=0_usize;
  let mut outputs=Vec::<i64>::new();
  let mut delivered=Vec::<i64>::new();
  let mut paid_feedback_reads=Vec::<i64>::new();
  while world.step(){
   steps+=1;
   assert!(steps<=CREDITS as usize);
   let event=world.events().last().unwrap().clone();
   assert!(matches!(event.origin,Origin::Internal(1)));
   let actor=world.process(1).unwrap();
   assert_eq!(actor.code.nodes,founder,"original native CIL founder changed");
   assert_eq!(world.total_compute_inventory(),Some(CREDITS-steps as u64));
   assert!(world.process(2).is_none());
   assert!(world.used_cells()<=16&&world.occupied_units()<=300);
   let record=json!(["cil",steps,event.pc,actor.pc,actor.registers,actor.credits,
      KEYS.map(|k|world.cell(k)),world.occupied_units()]);
   original_visible.push(record.clone());
   serde_json::to_writer(&mut stream,&record).unwrap();stream.write_all(b"\n").unwrap();
   if event.pc==79 {
    match &event.kind{
     EventKind::CellRead{key:20,value}=>paid_feedback_reads.push(*value),
     other=>panic!("missing original paid feedback read: {other:?}")
    }
   }
   if event.pc==71 {
    let mode=match &event.kind{
     EventKind::CellWritten{key:11,value}=>*value,
     other=>panic!("missing actual source-paid task service: {other:?}")
    };
    let episode=world.cell(32).unwrap() as usize;
    assert!(episode<4);
    let trial=world.cell(63).unwrap();
    let truth=i64::from(mode==goals[episode]);
    true_count+=truth as usize;
    let feedback=match feedback_regime{
      "truth"=>truth,"inverted"=>1-truth,
      other=>panic!("unregistered feedback regime: {other}")
    };
    assert_eq!(feedback,1,"frozen observational alias must deliver identical positive feedback");
    outputs.push(mode);delivered.push(feedback);
    world.intervene(Intervention::SetCell{key:20,value:feedback}).unwrap();
    let ext=world.events().last().unwrap();
    assert!(matches!(ext.origin,Origin::External));
    assert!(matches!(&ext.kind,EventKind::Intervened(_)));
    let projection=json!(["actor_received_feedback",steps,episode,trial,mode,feedback,
        KEYS.map(|k|world.cell(k)),world.occupied_units()]);
    original_visible.push(projection);
    let full_harness=json!(["feedback",steps,episode,goals[episode],trial,
        mode,truth,feedback,KEYS.map(|k|world.cell(k)),world.occupied_units()]);
    serde_json::to_writer(&mut stream,&full_harness).unwrap();
    stream.write_all(b"\n").unwrap();
   }
  }
  stream.flush().unwrap();
  assert_eq!(steps,expected["paid_steps"].as_u64().unwrap() as usize);
  assert_eq!(outputs,vec![1,1,1,1]);
  assert_eq!(delivered,vec![1,1,1,1]);
  assert_eq!(paid_feedback_reads,delivered);
  assert_eq!(true_count,expected["actual_goal_matches"].as_u64().unwrap() as usize);
  assert_eq!(world.cell(21),Some(1));
  assert_eq!(world.cell(32),Some(4));
  assert_eq!(world.cell(63),Some(0));
  assert_eq!(world.occupied_units(),136);
  assert!(world.process(1).unwrap().stopped);
  let external=world.events().iter().filter(|e|matches!(e.origin,Origin::External)).collect::<Vec<_>>();
  assert_eq!(external.len(),5);
  assert!(matches!(&external[0].kind,EventKind::Loaded));
  assert!(external.iter().skip(1).all(|e|matches!(&e.kind,EventKind::Intervened(_))));
  evidence.push(json!({"arm":label,"hidden_goals":goals,"feedback_regime":feedback_regime,
    "true_task_matches":true_count,"original_paid_instructions":steps,
    "task_outputs":outputs,"visible_feedback":delivered,
    "original_paid_feedback_reads":paid_feedback_reads,
    "actor_declared_success":world.cell(21),
    "world_occupied_units":world.occupied_units(),"original_process_halted":true
  }));
  world_out.push(world);
  actor_out.push(original_visible);
 }
 // Same native events, same complete world, same original PID1 history.
 // The hidden-goal truth is outside World and therefore not inferable.
 assert_eq!(actor_out[0],actor_out[1],
    "all source-visible paid steps and received feedback must be identical");
 assert_eq!(world_out[0],world_out[1],
    "even entire original Rust World+event history must be identical");
 assert_eq!(serde_json::to_vec(&world_out[0]).unwrap(),
            serde_json::to_vec(&world_out[1]).unwrap());
 assert_ne!(evidence[0]["true_task_matches"],evidence[1]["true_task_matches"]);
 assert_eq!(evidence[0]["true_task_matches"],4);
 assert_eq!(evidence[1]["true_task_matches"],0);
 fs::write(folder.join("b56_f1d1k_g3q_c1_p1_l3_f2_native_feedback_truth_alias.json"),
  serde_json::to_vec_pretty(&json!({
    "study":"B56-F1d1k-G3Q-C1-P1-L3-F2",
    "level":"NATIVE_FULL_WORLD_AND_EVENT_HISTORY_OBSERVATIONAL_ALIAS",
    "source_sha":std::env::var("GITHUB_SHA").unwrap_or_else(|_|"local".into()),
    "original_world_structurally_equal":true,
    "complete_serialized_world_bytes_equal":true,
    "original_paid_actor_visible_trace_equal":true,
    "paid_steps_each":495,"arms":evidence,
    "limits":"Hidden goal exists only in external harness. This is a semantic identifiability counterexample, not a sensor for truth."
  })).unwrap()).unwrap();
}
