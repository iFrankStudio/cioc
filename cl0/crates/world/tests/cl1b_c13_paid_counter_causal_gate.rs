//! C13 original native paid counter -> paid read -> paid branch -> optional real source restoration.
//! Authored threshold-one gate. A causal mechanism test, NOT endogenous ROI learning or B3/B4.
use cl0_world::{EventKind, Intervention, Op, Origin, Process, World};
use serde_json::{json, Value};
use std::fs::{self, File};
use std::io::{BufWriter, Write};

const C10: &str = "../../../cl1b/studies/CL1B-G2-ACTOR-PAID-STATE-TRIGGERED-ROLLBACK-C10-frozen-274-worlds-v1.json";
const C13: &str = "../../../cl1b/studies/CL1B-C13-PAID-COUNTER-CAUSAL-ROLLBACK-GATE-frozen-184-worlds-v1.json";
const CREDIT: u64 = 6000;
const CELLS: [i64; 16] = [11,20,21,30,32,33,34,35,36,37,38,39,40,41,42,43];

fn decode(v: &Value) -> Op {
    let tag = v["kind"].as_str().unwrap();
    let raw = if tag == "Halt" {
        json!("Halt")
    } else {
        let mut map = v.as_object().unwrap().clone();
        map.remove("kind");
        let mut obj = serde_json::Map::new();
        obj.insert(tag.to_string(), Value::Object(map));
        Value::Object(obj)
    };
    serde_json::from_value(raw).expect("C10 original op")
}

fn source(m: &Value, arm: &str) -> Vec<Op> {
    let gated = arm.starts_with("gated_");
    let metered = gated || arm == "c12_ungated_meter_reference";
    assert!(metered || arm == "c9_fixed_abstain_reference");
    let key = if arm == "c9_fixed_abstain_reference" {
        "frozen_source_g2_C9_checkpoint_abstain"
    } else { "frozen_source_g2_rollback" };
    let mut nodes = m[key].as_array().unwrap().iter().map(decode).collect::<Vec<_>>();
    assert_eq!(nodes.len(), 180);
    if metered {
        assert_eq!(nodes[95], Op::Branch { test:0, nonzero:99, zero:86 });
        nodes[95] = Op::Branch { test:0, nonzero:180, zero:86 };
        let n185 = if gated { 186 } else { 99 };
        let extension = [
            json!({"kind":"Const","dst":2,"value":43,"next":181}),
            json!({"kind":"Read","dst":0,"address":2,"next":182}),
            json!({"kind":"Const","dst":2,"value":1,"next":183}),
            json!({"kind":"Add","dst":0,"lhs":0,"rhs":2,"next":184}),
            json!({"kind":"Const","dst":2,"value":43,"next":185}),
            json!({"kind":"Write","address":2,"value":0,"next":n185})
        ];
        nodes.extend(extension.iter().map(decode));
    }
    if gated {
        let policy = [
            json!({"kind":"Const","dst":2,"value":43,"next":187}),
            json!({"kind":"Read","dst":0,"address":2,"next":188}),
            json!({"kind":"Const","dst":2,"value":1,"next":189}),
            json!({"kind":"Sub","dst":0,"lhs":0,"rhs":2,"next":190}),
            json!({"kind":"Branch","test":0,"nonzero":103,"zero":99})
        ];
        nodes.extend(policy.iter().map(decode));
    }
    assert_eq!(nodes.len(), if gated {191} else if metered {186} else {180});
    nodes
}

fn signature(nodes: &[Op]) -> [i64; 5] {
    let edge = |i: usize| match &nodes[i] {
        Op::Write { next, .. } => *next as i64,
        x => panic!("node {i} no Write: {x:?}")
    };
    let literal = |i: usize| match &nodes[i] {
        Op::Const { value, .. } => *value,
        x => panic!("node {i} no Const: {x:?}")
    };
    [edge(12),edge(34),literal(46),edge(133),literal(173)]
}

fn run(m: &Value, name: &str, arm: &str, mode: &str, goals: &[i64],
    dir: &std::path::Path) -> Value {
    assert!(matches!(mode,"truth"|"inverted"|"forced0"));
    assert_eq!(goals.len(),48);
    assert!(goals.iter().all(|&v|(0..=2).contains(&v)));
    let gated=arm.starts_with("gated_");
    let metered=gated||arm=="c12_ungated_meter_reference";
    let suppress=arm=="gated_pc102_effect_suppressed";
    let clamp=match arm {
        "gated_host_clamp_one"=>Some(1),
        "gated_host_force_two"=>Some(2),
        _=>None
    };
    let code=source(m,arm);
    let mut expected=code.clone();
    let mut world=World::with_strict_capacity(16,191,300);
    if suppress { world=world.with_native_patchconst_suppression(1,102,173); }
    let init_occupancy=code.len()+1;
    world.load(Process::new(1,code,CREDIT)).unwrap();
    assert_eq!(world.occupied_units(),init_occupancy);
    let mut trace=BufWriter::new(File::create(dir.join(format!(
        "cl1b_c13_original_{name}.jsonl"))).unwrap());
    let mut paid=0_u64;
    let mut peak=init_occupancy;
    let mut service=Vec::<Value>::new();
    let mut edits=Vec::<Value>::new();
    let mut decisions=Vec::<Value>::new();
    let mut writes=0_u64;
    let mut candidate_events=0_u64;
    let mut pc102_paid=0_u64;
    let mut physically_changed=0_u64;
    let mut allowed=0_u64;
    let mut skipped=0_u64;
    let mut external_tampers=0_u64;
    let mut last_paid_185=None::<u64>;
    let mut last_95_nonzero=None::<u64>;
    while world.step() {
        paid+=1;
        assert!(paid<=CREDIT,"private budget exceeded");
        let e=world.events().last().unwrap().clone();
        assert_eq!(e.origin,Origin::Internal(1));
        let pc=e.pc;
        let p=world.process(1).unwrap();
        assert_eq!(p.credits,CREDIT-paid);
        assert_eq!(world.total_compute_inventory(),Some(CREDIT-paid));
        assert!(world.process(2).is_none());
        assert!(world.used_cells()<=16&&world.occupied_units()<=300);
        assert_eq!(p.code.nodes.len(),expected.len());

        if [45,88,102,122,28].contains(&pc) {
            let target=p.registers[1] as usize;
            let value=if pc==28 {p.registers[3]} else {p.registers[0]};
            let before=match &expected[target] {
                Op::Const{value,..}=>*value,
                x=>panic!("expected patch target Const: {x:?}")
            };
            let suppressed=pc==102&&suppress;
            if suppressed {
                assert!(matches!(e.kind,EventKind::NativeCodeEditSuppressed{opcode_tag:9}));
            } else {
                assert!(matches!(e.kind,EventKind::Executed));
                if let Op::Const{value:old,..}=&mut expected[target] {*old=value;}
            }
            if pc==102 {
                assert_eq!(target,173);
                assert_eq!(value,world.cell(42).unwrap_or(0),"PC102 value must be actor-stored parent");
                pc102_paid+=1;
                if !suppressed&&before!=value {physically_changed+=1;}
                if gated {
                    assert_eq!(last_paid_185.map(|x|x<paid),Some(true));
                }
            }
            edits.push(json!([pc,target,before,value,suppressed]));
        }
        if [26,82,131,136,139,155].contains(&pc) {
            let target=p.registers[1] as usize;
            let dest=p.registers[0] as usize;
            if let Op::Write{next,..}=&mut expected[target] {
                edits.push(json!([pc,target,*next,dest,false]));
                *next=dest;
            } else {panic!("expected PatchNext target Write");}
        }
        assert_eq!(p.code.nodes,expected,"unpaid source mutation in {name} at {paid}");

        if pc==95&&metered&&p.pc==180 {
            last_95_nonzero=Some(paid);
        }
        if pc==185 {
            assert!(metered);
            assert!(matches!(&e.kind,EventKind::CellWritten{key:43,..}));
            candidate_events+=1;
            writes+=1;
            last_paid_185=Some(paid);
            assert!(last_95_nonzero.unwrap()<paid);
            if clamp.is_none() {
                assert_eq!(world.cell(43),Some(candidate_events as i64),
                    "native uncorrupted actor meter must equal candidate count");
            }
        }
        if pc==190 {
            assert!(gated);
            assert!(matches!(e.kind,EventKind::Executed));
            assert!(last_paid_185.unwrap()<paid);
            let next=if p.registers[0]==0{99}else{103};
            assert_eq!(p.pc,next,"PC190 paid branch did not consume paid Read43");
            if next==99{allowed+=1;}else{skipped+=1;}
            decisions.push(json!({
                "step":paid,"count_read_from_paid_cell43":p.registers[0]+1,
                "route":if next==99{"ALLOW_PC102"}else{"SKIP_PC102"},
                "pc":190,"next_pc":next,"candidate_index":candidate_events
            }));
        }
        peak=peak.max(world.occupied_units());
        serde_json::to_writer(&mut trace,&json!([
            "cil",paid,pc,p.pc,p.registers,p.credits,
            CELLS.map(|k|world.cell(k)),world.occupied_units(),
            signature(&p.code.nodes),
            if pc==102&&suppress {"suppressed_pc102"}
            else if pc==185 {"paid_meter_write"}
            else if pc==190 {"paid_decision"} else {"normal"}
        ])).unwrap();
        trace.write_all(b"\n").unwrap();
        if pc==185 {
            if let Some(v)=clamp {
                let before=world.cell(43).unwrap();
                world.intervene(Intervention::SetCell{key:43,value:v}).unwrap();
                assert!(matches!(world.events().last().unwrap().origin,Origin::External));
                external_tampers+=1;
                peak=peak.max(world.occupied_units());
                serde_json::to_writer(&mut trace,&json!([
                    "tamper",paid,43,before,v,CELLS.map(|k|world.cell(k)),
                    world.occupied_units(),signature(&world.process(1).unwrap().code.nodes)
                ])).unwrap();
                trace.write_all(b"\n").unwrap();
            }
        }
        if pc==3 {
            let guess=match &e.kind {
                EventKind::CellWritten{key:11,value}=>*value,
                x=>panic!("service must arise from native paid Write11: {x:?}")
            };
            let stage=world.cell(32).unwrap_or(0) as usize;
            assert!(stage<48,"service after last hidden stage");
            let truth=i64::from(guess==goals[stage]);
            let feedback=match mode {
                "truth"=>truth,"inverted"=>1-truth,"forced0"=>0,_=>unreachable!()
            };
            service.push(json!([stage,guess,truth,feedback]));
            world.intervene(Intervention::SetCell{key:20,value:feedback}).unwrap();
            assert!(matches!(world.events().last().unwrap().origin,Origin::External));
            peak=peak.max(world.occupied_units());
            serde_json::to_writer(&mut trace,&json!([
                "feedback",paid,stage,goals[stage],guess,truth,feedback,
                CELLS.map(|k|world.cell(k)),world.occupied_units(),
                signature(&world.process(1).unwrap().code.nodes)
            ])).unwrap();
            trace.write_all(b"\n").unwrap();
        }
    }
    trace.flush().unwrap();
    assert_eq!(writes,candidate_events);
    if gated {
        assert_eq!(candidate_events,allowed+skipped);
        assert_eq!(pc102_paid,allowed);
    }
    if metered&&!gated {assert_eq!(candidate_events,pc102_paid);}
    if let Some(v)=clamp {
        assert_eq!(external_tampers,candidate_events);
        assert_eq!(world.cell(43),if candidate_events>0 {Some(v)} else {None});
        if v==1{assert_eq!(skipped,0);} else {assert_eq!(allowed,0);}
    } else {
        assert_eq!(external_tampers,0);
        assert_eq!(world.cell(43),if metered&&candidate_events>0 {
            Some(candidate_events as i64)
        } else {None});
    }
    if gated&&clamp.is_none() {
        assert!(allowed<=1);
        assert_eq!(allowed,u64::from(candidate_events>0));
    }
    let true_count=service.iter().filter(|x|x[2]==1).count();
    let completed=world.cell(32).unwrap_or(0);
    let claim=world.cell(21).unwrap_or(0);
    let budget_exhausted=paid==CREDIT;
    // Actual task losses remain in denominator; never turn negative science into CI failure.
    if mode=="forced0" { assert_eq!(completed,0); }
    if mode=="inverted" { assert!(true_count<service.len()); }
    json!({
        "name":name,"arm":arm,"feedback":mode,"goals":goals,
        "steps":paid,"services":service,"paid_code_edits":edits,
        "paid_decisions":decisions,"candidate_events":candidate_events,
        "actor_paid_meter_writes":writes,"actor_paid_pc102":pc102_paid,
        "physical_pc102_value_changes":physically_changed,
        "allow_pc102":allowed,"skip_pc102":skipped,
        "externally_tampered_cell43":external_tampers,
        "meter_value":world.cell(43),"completed_stages":completed,
        "true_matches":true_count,"claim":claim,"peak_occupied":peak,
        "final_signature":signature(&world.process(1).unwrap().code.nodes),
        "budget_exhausted":budget_exhausted
    })
}

#[test]
fn c13_184_original_native_paid_counter_real_read_branch_causal_rollback() {
    let root=std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let c10:Value=serde_json::from_slice(&fs::read(root.join(C10)).unwrap()).unwrap();
    let m:Value=serde_json::from_slice(&fs::read(root.join(C13)).unwrap()).unwrap();
    assert_eq!(c10["study"],"CL1B-G2-ACTOR-PAID-STATE-TRIGGERED-ROLLBACK-C10");
    assert_eq!(m["study"],"CL1B-C13-PAID-COUNTER-CAUSAL-ROLLBACK-GATE");
    assert_eq!(m["status"],"PRE_NATIVE_FROZEN");
    let cases=m["cases"].as_array().unwrap();
    assert_eq!(cases.len(),30);
    let arms=m["arms"].as_array().unwrap();
    assert_eq!(arms.len(),6);
    let controls=m["controls"].as_array().unwrap();
    assert_eq!(controls.len(),4);
    assert_eq!(m["counts"]["worlds"],184);
    let mut old_goals=std::collections::BTreeSet::new();
    for x in c10["cases"].as_array().unwrap() {
        old_goals.insert(x["goals"].to_string());
    }
    let mut new_goals=std::collections::BTreeSet::new();
    let dir=root.join("../../cl1b-ci-evidence");
    fs::create_dir_all(&dir).unwrap();
    let mut records=Vec::new();
    let mut total_steps=0_u64;
    let mut incomplete=Vec::new();
    let mut different_paid_decisions=0_u64;
    let mut live_skips=0_u64;
    let mut all_tampers=0_u64;
    let mut arm_steps=std::collections::BTreeMap::<String,u64>::new();
    let mut arm_completions=std::collections::BTreeMap::<String,u64>::new();
    let mut arm_pc102=std::collections::BTreeMap::<String,u64>::new();
    let mut arm_candidate=std::collections::BTreeMap::<String,u64>::new();
    for case in cases {
        let goals=case["goals"].as_array().unwrap().iter()
            .map(|v|v.as_i64().unwrap()).collect::<Vec<_>>();
        assert_eq!(goals.len(),48);
        assert!(!old_goals.contains(&case["goals"].to_string()));
        assert!(new_goals.insert(case["goals"].to_string()),"duplicated new task schedule");
        let base=case["name"].as_str().unwrap();
        let mut row=Vec::new();
        for arm in arms {
            let kind=arm.as_str().unwrap();
            let v=run(&c10,&format!("{base}_{kind}"),kind,"truth",&goals,&dir);
            total_steps+=v["steps"].as_u64().unwrap();
            *arm_steps.entry(kind.to_string()).or_default()+=v["steps"].as_u64().unwrap();
            *arm_candidate.entry(kind.to_string()).or_default()+=
                v["candidate_events"].as_u64().unwrap();
            *arm_pc102.entry(kind.to_string()).or_default()+=
                v["actor_paid_pc102"].as_u64().unwrap();
            if v["completed_stages"]==48 && v["true_matches"]==48 &&
                v["claim"]==1 && v["budget_exhausted"]==false {
                *arm_completions.entry(kind.to_string()).or_default()+=1;
            } else {
                incomplete.push(v["name"].as_str().unwrap().to_string());
            }
            all_tampers+=v["externally_tampered_cell43"].as_u64().unwrap();
            row.push(v);
        }
        // All C13 gated arms begin from exactly the same original 191-node CIL.
        // At the first C13 paid PC190, all other provenance matches except
        // the transparently logged host tamper that changes the count in controls.
        assert_eq!(source(&c10,arms[0].as_str().unwrap()),
            source(&c10,arms[1].as_str().unwrap()));
        assert_eq!(source(&c10,arms[0].as_str().unwrap()),
            source(&c10,arms[2].as_str().unwrap()));
        assert_eq!(source(&c10,arms[0].as_str().unwrap()),
            source(&c10,arms[3].as_str().unwrap()));
        live_skips+=row[0]["skip_pc102"].as_u64().unwrap();
        if row[0]["actor_paid_pc102"]!=row[2]["actor_paid_pc102"] ||
            row[0]["actor_paid_pc102"]!=row[3]["actor_paid_pc102"] {
            different_paid_decisions+=1;
        }
        records.push(json!({"name":base,"family":case["family"],"arms":row}));
    }
    let mut adversarial=Vec::new();
    for control in controls {
        let original=cases.iter().find(|c|c["name"]==control["case"]).unwrap();
        let goals=original["goals"].as_array().unwrap().iter()
            .map(|v|v.as_i64().unwrap()).collect::<Vec<_>>();
        let arm=control["arm"].as_str().unwrap();
        let mode=control["feedback"].as_str().unwrap();
        let name=control["name"].as_str().unwrap();
        let v=run(&c10,name,arm,mode,&goals,&dir);
        total_steps+=v["steps"].as_u64().unwrap();
        all_tampers+=v["externally_tampered_cell43"].as_u64().unwrap();
        adversarial.push(v);
    }
    let out=json!({
        "study":"CL1B-C13-PAID-COUNTER-CAUSAL-ROLLBACK-GATE",
        "status":"ORIGINAL_NATIVE_PAID_COUNTER_DECISION_NOT_LEARNED_ROI",
        "tested_new_cases":30,"native_worlds":184,
        "truthful_worlds":180,"adversarial_worlds":4,
        "total_private_paid_cil":total_steps,
        "gated_live_skipped_actual_pc102":live_skips,
        "cases_with_live_vs_tampered_paid_pc102_difference":different_paid_decisions,
        "external_tamper_actions":all_tampers,
        "arm_total_paid_cil":arm_steps,
        "arm_completed_truthful_worlds":arm_completions,
        "arm_actual_paid_pc102":arm_pc102,
        "arm_actor_written_candidate_events":arm_candidate,
        "incomplete_truthful_world_names":incomplete,
        "records":records,"adversarial_records":adversarial
    });
    fs::write(dir.join("cl1b_c13_native_184_worlds.json"),
        serde_json::to_vec_pretty(&out).unwrap()).unwrap();
    assert!(live_skips>0 && different_paid_decisions>0,
        "native actor-owned memory has no causal effect on physically executed paid PC102");
    println!("C13 ORIGINAL NATIVE: 184 worlds; new cases=30 paid={} true_gate_skips={} causally_changed_cases={} true_arm_fee={:?} full_success={:?} incomplete_true_worlds={} third_party_tampers={}",
        total_steps,live_skips,different_paid_decisions,
        out["arm_total_paid_cil"],out["arm_completed_truthful_worlds"],
        incomplete.len(),all_tampers);
}
