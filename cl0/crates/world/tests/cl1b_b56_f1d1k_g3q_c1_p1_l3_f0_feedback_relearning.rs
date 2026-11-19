//! B56-F1d1k-G3Q-C1-P1-L3-F0: original one-PID paid task feedback/relearning.
//! Source-frozen finite grammar, real postservice environment feedback,
//! no host source/active-rule injection, not C0-earned rule certification.
use cl0_world::{EventKind,Intervention,Op,Origin,Process,World};
use serde_json::{json,Value};
use std::fs::{self,File};
use std::io::{BufWriter,Write};
const FROZEN:&str="../../../cl1b/studies/B56-F1d1k-G3Q-C1-P1-L3-F0-frozen-122-node-feedback-relearning-founder-v1.json";
const CREDITS:u64=8_000;
const KEYS:[i64;13]=[11,20,21,32,33,50,51,52,53,56,62,63,64];
fn decode(v:&Value)->Op{
 let tag=v["kind"].as_str().unwrap();
 let encoded=if tag=="Halt"{json!("Halt")}else{
  let mut f=serde_json::Map::new();
  for (key,val) in v.as_object().unwrap(){if key!="kind"{f.insert(key.clone(),val.clone());}}
  let mut x=serde_json::Map::new();x.insert(tag.to_string(),Value::Object(f));Value::Object(x)
 };
 serde_json::from_value(encoded).expect("bad frozen CIL opcode")
}
#[test]
fn g3q_c1_l3_f0_original_pid_feedback_search_and_midlife_shift(){
 let root=std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
 let manifest:Value=serde_json::from_slice(&fs::read(root.join(FROZEN)).unwrap()).unwrap();
 let frozen=manifest["exact_native_founder_code"].as_array().unwrap();
 assert_eq!(frozen.len(),122);
 let arms=manifest["prespecified_arms"].as_array().unwrap();
 assert_eq!(arms.len(),6);
 let code:Vec<Op>=frozen.iter().map(decode).collect();
 let dir=root.join("../../cl1b-ci-evidence");fs::create_dir_all(&dir).unwrap();
 let mut all=Vec::new();
 for arm in arms {
  let name=arm["name"].as_str().unwrap();
  let regime=arm["feedback"].as_str().unwrap();
  let targets=arm["goals"].as_array().unwrap().iter()
      .map(|v|v.as_i64().unwrap()).collect::<Vec<_>>();
  assert_eq!(targets.len(),4);
  let mut world=World::with_strict_capacity(16,180,300);
  world.load(Process::new(1,code.clone(),CREDITS)).unwrap();
  assert_eq!(world.occupied_units(),123);
  let mut trace=BufWriter::new(File::create(dir.join(format!(
      "b56_f1d1k_g3q_c1_p1_l3_f0_paid_trace_{name}.jsonl"))).unwrap());
  let mut steps=0_usize;let mut peak=world.occupied_units();
  let mut services=Vec::<Value>::new();let mut probes=[0_usize;4];
  let mut source_reads=Vec::<i64>::new();let mut delivered=Vec::<i64>::new();
  let mut true_matches=0_usize;let mut fetches=0_usize;let mut checkpoints=0_usize;
  while world.step(){
   steps+=1;assert!(steps<=CREDITS as usize);
   let event=world.events().last().unwrap().clone();
   assert!(matches!(event.origin,Origin::Internal(1)));
   let pid=world.process(1).unwrap();
   assert_eq!(pid.code.nodes,code,"no actor/host source edit");
   assert_eq!(world.total_compute_inventory(),Some(CREDITS-steps as u64));
   assert!(world.process(2).is_none());
   assert!(world.used_cells()<=16&&world.occupied_units()<=300);
   peak=peak.max(world.occupied_units());
   if event.pc==37{
    assert!(matches!(&event.kind,EventKind::CellRead{..}));fetches+=1;
   }
   if event.pc==58{
    assert!(matches!(&event.kind,EventKind::CellWritten{key:56,..}));checkpoints+=1;
   }
   if event.pc==79{
    match &event.kind{
     EventKind::CellRead{key:20,value}=>source_reads.push(*value),
     other=>panic!("missing paid feedback Read20: {other:?}")
    }
   }
   let paid=json!(["cil",steps,event.pc,pid.pc,pid.registers,pid.credits,
                   KEYS.map(|key|world.cell(key)),world.occupied_units()]);
   serde_json::to_writer(&mut trace,&paid).unwrap();trace.write_all(b"\n").unwrap();
   if event.pc==71{
    let mode=match &event.kind{
     EventKind::CellWritten{key:11,value}=>*value,
     other=>panic!("not an original PID1 task service: {other:?}")
    };
    assert!((0..=2).contains(&mode));
    let episode=world.cell(32).unwrap() as usize;
    assert!(episode<4);
    let candidate=world.cell(63).unwrap();
    assert!((0..=2).contains(&candidate));
    let truth=i64::from(mode==targets[episode]);
    true_matches+=truth as usize;
    let feedback=match regime{"truth"=>truth,"blind"=>0,"inverted"=>1-truth,
      other=>panic!("unknown feedback arm: {other}")};
    probes[episode]+=1;assert!(probes[episode]<=3);
    services.push(json!([episode,targets[episode],candidate,mode,truth,feedback,steps]));
    delivered.push(feedback);
    // The hidden target remains exclusively in the external task harness.
    // A generic, explicitly logged SetCell20 is the ONLY postload intervention.
    world.intervene(Intervention::SetCell{key:20,value:feedback}).unwrap();
    let external=world.events().last().unwrap();
    assert!(matches!(external.origin,Origin::External));
    assert!(matches!(&external.kind,EventKind::Intervened(_)));
    assert_eq!(world.cell(20),Some(feedback));
    let env=json!(["feedback",steps,episode,targets[episode],candidate,
       mode,truth,feedback,KEYS.map(|key|world.cell(key)),world.occupied_units()]);
    serde_json::to_writer(&mut trace,&env).unwrap();trace.write_all(b"\n").unwrap();
   }
  }
  trace.flush().unwrap();
  assert_eq!(steps,arm["paid_steps"].as_u64().unwrap() as usize);
  assert_eq!(services.len(),arm["services"].as_u64().unwrap() as usize);
  assert_eq!(world.cell(21),arm["success"].as_i64());
  assert_eq!(world.cell(32),arm["final_episode"].as_i64());
  assert_eq!(world.cell(63),arm["final_trial_byte"].as_i64());
  assert_eq!(true_matches,arm["truth_matches"].as_u64().unwrap() as usize);
  assert_eq!(probes.iter().copied().filter(|x|*x>0).collect::<Vec<_>>(),
    arm["episode_probes"].as_array().unwrap().iter()
       .map(|x|x.as_u64().unwrap() as usize).collect::<Vec<_>>());
  assert_eq!(source_reads,delivered,"feedback must be sensor-read by original PID");
  assert_eq!(fetches,services.len()*3);
  assert_eq!(checkpoints,services.len()*3);
  assert_eq!(peak,136);
  assert_eq!(world.used_cells(),13);
  assert!(world.process(1).unwrap().stopped);
  let externals=world.events().iter().filter(|e|
      matches!(e.origin,Origin::External)).collect::<Vec<_>>();
  assert_eq!(externals.len(),services.len()+1);
  assert!(matches!(&externals[0].kind,EventKind::Loaded));
  assert!(externals.iter().skip(1).all(|e|
      matches!(&e.kind,EventKind::Intervened(_))));
  if name=="inverted_feedback"{
   assert_eq!(world.cell(21),Some(1));
   assert_eq!(true_matches,0,"belief in success must NOT be reported as fitness");
  }
  all.push(json!({"arm":name,"goals":targets,"feedback_regime":regime,
    "paid_steps":steps,"credits_remain":CREDITS-steps as u64,
    "services":services,"episode_probes":probes,
    "original_paid_feedback_reads":source_reads,
    "true_service_matches":true_matches,
    "actor_success_claim":world.cell(21),
    "final_episode":world.cell(32),"final_active_middle":world.cell(63),
    "meta_fetches":fetches,"accumulator_checkpoints":checkpoints,
    "peak_occupied":peak,"single_original_pid1":true,"self_halted":true,
    "postload_environment_feedback_writes":externals.len()-1}));
 }
 fs::write(dir.join("b56_f1d1k_g3q_c1_p1_l3_f0_native_feedback_relearning.json"),
  serde_json::to_vec_pretty(&json!({
   "study":"B56-F1d1k-G3Q-C1-P1-L3-F0",
   "level":"NATIVE_ONE_PID_FEEDBACK_CONDITIONED_FINITE_META_SEARCH_NOT_EARNED_C0",
   "source_sha":std::env::var("GITHUB_SHA").unwrap_or_else(|_|"local".into()),
   "arms":all,
   "limits":"Researcher-authored safe starting bank; feedback task oracle; not integrated C0, self-discovered algorithm or open-ended life."
  })).unwrap()).unwrap();
}
