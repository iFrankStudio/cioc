//! CL-1B G3Q-C1-P1-L3-I2: prospectively frozen 180-node single original
//! PID1 repairs its executable tail using an IN-WORLD bank word, then pays
//! to re-certify. One-attempt retry guard prevents unlimited failed search.
//! Under unchanged CL0 16/180/300 caps, no original PID restart or host repair.
//!
//! Limits: researcher-authored genome and repair heuristic, no general mutation
//! search; semantic truth is supplied only to the external hidden task harness.
use cl0_world::{EventKind, Intervention, Op, Origin, Process, World};
use serde_json::{json, Value};
use std::fs::{self, File};
use std::io::{BufWriter, Write};

const FREEZE: &str = "../../../cl1b/studies/B56-F1d1k-G3Q-C1-P1-L3-I2-frozen-180-node-data-conditioned-repair-and-failfast-v1.json";
const CREDIT: u64 = 8000;
const KEYS: [i64; 16] = [7,8,9,11,20,21,32,33,50,51,52,53,56,62,63,64];

fn decode(v: &Value) -> Op {
    let kind = v["kind"].as_str().unwrap();
    let tagged = if kind == "Halt" { json!("Halt") } else {
        let mut fields = serde_json::Map::new();
        for (key,val) in v.as_object().unwrap() {
            if key != "kind" { fields.insert(key.clone(), val.clone()); }
        }
        let mut wrapped = serde_json::Map::new();
        wrapped.insert(kind.to_string(), Value::Object(fields));
        Value::Object(wrapped)
    };
    serde_json::from_value(tagged).expect("invalid frozen original CIL instruction")
}

fn executable_tail(world: &World) -> i64 {
    match &world.process(1).unwrap().code.nodes[6] {
        Op::Const { dst: 3, value, next: 7 } => *value,
        other => panic!("I2 source tail is no longer original Const6: {other:?}"),
    }
}

#[test]
fn g3q_c1_i2_original_pid_endogenous_bank_repair_one_retry_guard_and_task_feedback() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let frozen: Value = serde_json::from_slice(&fs::read(root.join(FREEZE)).unwrap()).unwrap();
    let template = frozen["complete_original_CIL_source"].as_array().unwrap();
    assert_eq!(template.len(), 180);
    assert_eq!(frozen["strict_physics"]["data_capacity"], 16);
    assert_eq!(frozen["strict_physics"]["max_original_source_nodes"], 180);
    assert_eq!(frozen["strict_physics"]["max_occupied_units"], 300);
    assert_eq!(template[177], json!({"kind":"Read","dst":3,"address":2,"next":178}));
    assert_eq!(template[178], json!({"kind":"PatchConst","node":1,"value":3,"next":0}));
    let arms = frozen["pre_registered_arms"].as_array().unwrap();
    let neg = &frozen["pre_registered_control"];
    assert_eq!(arms.len(), 8);
    let dir = root.join("../../cl1b-ci-evidence");
    fs::create_dir_all(&dir).unwrap();
    let mut reports = Vec::<Value>::new();

    for arm in arms.iter().chain(std::iter::once(neg)) {
        let name = arm["name"].as_str().unwrap();
        let disabled = name == "source_edit_suppressed_counterfactual";
        let expected = &arm["expected"];
        let mut genome = template.clone();
        for (pc,field) in [(0,"first"),(3,"middle"),(6,"tail")] {
            genome[pc]["value"] = arm[field].clone();
        }
        let initial: Vec<Op> = genome.iter().map(decode).collect();
        let mut repaired = initial.clone();
        repaired[6] = Op::Const {
            dst:3, value:arm["first"].as_i64().unwrap(), next:7
        };
        let mut world = World::with_strict_capacity(16,180,300);
        if disabled { world = world.with_native_code_edits_disabled(); }
        world.load(Process::new(1,initial.clone(),CREDIT)).unwrap();
        assert_eq!(world.occupied_units(),181);
        let mut native = BufWriter::new(File::create(dir.join(format!(
            "b56_f1d1k_g3q_c1_p1_l3_i2_native_paid_{name}.jsonl"))).unwrap());
        let goals = [0_i64,0,2,2];
        let regime = arm["feedback"].as_str().unwrap();
        let mut steps = 0_usize;
        let mut peak = world.occupied_units();
        let mut witnesses = Vec::<Value>::new();
        let mut services = Vec::<Value>::new();
        let mut edits = Vec::<Value>::new();
        let mut certificate_passes = Vec::<usize>::new();
        let mut meta_fetches = 0_usize;
        let mut feedback_reads = Vec::<i64>::new();
        let mut feedback_sent = Vec::<i64>::new();
        let mut patch_applied = false;
        while world.step() {
            steps += 1;
            assert!(steps <= CREDIT as usize);
            let ev = world.events().last().unwrap().clone();
            assert!(matches!(ev.origin, Origin::Internal(1)));
            if ev.pc == 178 {
                assert!(edits.is_empty(), "one active repair attempt per lifetime");
                let old = executable_tail(&world);
                let desired = arm["first"].as_i64().unwrap();
                if disabled {
                    assert!(matches!(&ev.kind, EventKind::NativeCodeEditSuppressed{opcode_tag:9}));
                    assert_eq!(old,3);
                } else {
                    assert!(matches!(&ev.kind, EventKind::Executed));
                    assert_eq!(old, desired);
                    patch_applied = true;
                }
                edits.push(json!({"step":steps,"pc":178,"node":6,
                  "before":3,"after":old,"edit_suppressed":disabled}));
            }
            let p = world.process(1).unwrap();
            assert_eq!(&p.code.nodes,
                if patch_applied { &repaired } else { &initial },
                "only actor-paid PatchConst may modify the loaded source");
            assert!(world.process(2).is_none());
            assert_eq!(world.total_compute_inventory(), Some(CREDIT-steps as u64));
            assert!(world.used_cells() <= 16);
            assert!(world.occupied_units() <= 300);
            peak = peak.max(world.occupied_units());
            if ev.pc == 33 {
                assert!(matches!(&ev.kind,EventKind::CellRead{..}));
                meta_fetches += 1;
            }
            if ev.pc == 126 {
                match &ev.kind {
                    EventKind::CellWritten{key,value} => {
                        assert!((7..=9).contains(key) && *value==1);
                        witnesses.push(json!({"step":steps,"key":key,"value":value}));
                    }
                    other => panic!("PID1 structural witness did not pay Write: {other:?}"),
                }
            }
            if ev.pc == 166 {
                assert!(matches!(&ev.kind,EventKind::CellWritten{key:21,value:1}));
                assert_eq!([world.cell(7),world.cell(8),world.cell(9)],
                    [Some(1),Some(1),Some(1)]);
                certificate_passes.push(steps);
            }
            if ev.pc == 75 {
                match &ev.kind {
                    EventKind::CellRead{key:20,value} => feedback_reads.push(*value),
                    other => panic!("task feedback was not read by PID1 CIL: {other:?}"),
                }
            }
            let row = json!(["cil",steps,ev.pc,p.pc,p.registers,p.credits,
                KEYS.map(|k|world.cell(k)),world.occupied_units(),executable_tail(&world)]);
            serde_json::to_writer(&mut native,&row).unwrap();
            native.write_all(b"\n").unwrap();
            if ev.pc == 67 {
                assert_eq!(certificate_passes.len(),1,
                    "actual task service without valid original actor certificate");
                let output = match &ev.kind {
                    EventKind::CellWritten{key:11,value} => *value,
                    other => panic!("task output not genuine original Write11: {other:?}"),
                };
                let episode = world.cell(32).unwrap_or(0) as usize;
                assert!(episode < goals.len());
                let truth = i64::from(output==goals[episode]);
                let bit = match regime {
                    "truth" => truth,
                    "inverted" => 1-truth,
                    _ => panic!("unregistered task-feedback regime"),
                };
                let middle = world.cell(63).unwrap();
                services.push(json!({"step":steps,"episode":episode,"middle":middle,
                    "output":output,"truth":truth,"feedback":bit}));
                feedback_sent.push(bit);
                world.intervene(Intervention::SetCell{key:20,value:bit}).unwrap();
                let ext = world.events().last().unwrap();
                assert!(matches!(ext.origin,Origin::External));
                assert!(matches!(&ext.kind,EventKind::Intervened(_)));
                let env = json!(["feedback",steps,episode,goals[episode],middle,
                    output,truth,bit,KEYS.map(|k|world.cell(k)),
                    world.occupied_units(),executable_tail(&world)]);
                serde_json::to_writer(&mut native,&env).unwrap();
                native.write_all(b"\n").unwrap();
            }
        }
        native.flush().unwrap();
        assert!(world.process(1).unwrap().stopped,
            "bounded failed repair must stop, not burn entire compute budget");
        assert_eq!(steps,expected["steps"].as_u64().unwrap() as usize);
        assert_eq!(world.process(1).unwrap().pc,expected["halt_pc"].as_u64().unwrap() as usize);
        assert_eq!(json!(witnesses), expected["certificate_witnesses"]);
        assert_eq!(json!(certificate_passes),expected["cert_pass_steps"]);
        assert_eq!(json!(services),expected["services"]);
        assert_eq!(json!(edits),expected["source_edits"]);
        assert_eq!(meta_fetches,expected["paid_meta_fetches"].as_u64().unwrap() as usize);
        assert_eq!(feedback_reads,feedback_sent);
        assert_eq!(executable_tail(&world),expected["source_tail_final"].as_i64().unwrap());
        assert_eq!(world.used_cells(),expected["used_cells"].as_u64().unwrap() as usize);
        assert_eq!(peak,expected["peak_occupied"].as_u64().unwrap() as usize);
        assert_eq!(json!(world.cell(21)),expected["task_success_claim"]);
        assert_eq!(json!(world.cell(32)),expected["episode_final"]);
        let external = world.events().iter().filter(|ev|
            matches!(ev.origin, Origin::External)).collect::<Vec<_>>();
        assert_eq!(external.len(),services.len()+1);
        assert!(matches!(&external[0].kind, EventKind::Loaded));
        assert!(external.iter().skip(1).all(|ev|matches!(&ev.kind,EventKind::Intervened(_))));
        if !expected["cert_pass_steps"].as_array().unwrap().is_empty() {
            assert!(!services.is_empty());
        } else {
            assert!(services.is_empty());
        }
        if name == "unrepairable_middle_one" || name == "unrepairable_middle_two" || disabled {
            assert!(steps<8000 && certificate_passes.is_empty());
            assert_eq!(world.process(1).unwrap().pc,179);
            assert_eq!(services.len(),0);
        }
        reports.push(json!({"arm":name,"original_paid_steps":steps,
            "private_credits_remaining":CREDIT-steps as u64,
            "original_executable_source_nodes":180,
            "certified":certificate_passes.len()==1,
            "certificate_witnesses":witnesses,
            "self_edits":edits,
            "paid_task_services":services,
            "paid_feedback_reads":feedback_reads,
            "paid_meta_fetches":meta_fetches,
            "final_executable_tail":executable_tail(&world),
            "actor_success_claim":world.cell(21),
            "final_halt_pc":world.process(1).unwrap().pc,
            "peak_occupied_units":peak,
            "single_original_pid1":true,
            "native_halted":true,
            "host_source_edits_after_load":0}));
    }
    fs::write(dir.join("b56_f1d1k_g3q_c1_p1_l3_i2_native_data_repair_and_failfast.json"),
        serde_json::to_vec_pretty(&json!({
          "study":"B56-F1d1k-G3Q-C1-P1-L3-I2",
          "level":"NATIVE_180_NODE_ORIGINAL_PID_DATA_CONDITIONED_EXECUTABLE_REPAIR_AND_BOUND_FAILFAST",
          "source_sha":std::env::var("GITHUB_SHA").unwrap_or_else(|_|"local".to_string()),
          "arms":reports,
          "limits":"Data-conditioned copying one researcher-authored genome word, not self-invented grammar or open-ended life. Feedback semantic truth remains unverified from inside the actor."
        })).unwrap()).unwrap();
}
