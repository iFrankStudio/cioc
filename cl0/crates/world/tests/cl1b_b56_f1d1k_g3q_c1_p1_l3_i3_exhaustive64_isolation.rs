//! G3Q-C1-P1-L3-I3: original-native, prospectively frozen exhaustive
//! 64-rule certification soundness after actor-paid self-edit and COMPLETE
//! per-cycle witness isolation (7/8/9). No new CL0 primitives or relaxed
//! resource physics. A single integration test checks 64 separate original
//! uninterrupted PID1 lifetimes plus inverted feedback and disabled-edit arms.
//! This is a finite authored grammar and one bank-derived repair, NOT life.
use cl0_world::{EventKind, Intervention, Op, Origin, Process, World};
use serde_json::{json,Value};
use std::fs::{self,File};
use std::io::{BufWriter,Write};

const FROZEN:&str="../../../cl1b/studies/B56-F1d1k-G3Q-C1-P1-L3-I3-frozen-176-node-exhaustive-certificate-isolation-v1.json";
const KEYS:[i64;14]=[7,8,9,11,20,21,32,33,50,52,56,62,63,64];
const CREDIT:u64=8000;

fn decode(v:&Value)->Op{
    let tag=v["kind"].as_str().unwrap();
    let encoded=if tag=="Halt"{json!("Halt")}else{
        let mut fields=serde_json::Map::new();
        for (k,val) in v.as_object().unwrap(){
            if k!="kind"{fields.insert(k.clone(),val.clone());}
        }
        let mut wrapper=serde_json::Map::new();
        wrapper.insert(tag.to_string(),Value::Object(fields));
        Value::Object(wrapper)
    };
    serde_json::from_value(encoded).expect("invalid frozen original CIL opcode")
}
fn native_tail(w:&World)->i64{
    match &w.process(1).unwrap().code.nodes[6]{
        Op::Const{dst:3,value,next:7}=>*value,
        other=>panic!("source no longer has expected paid-edit target6: {other:?}")
    }
}
fn semantic_modes(a:i64,b:i64,c:i64)->Vec<i64>{
    (0..3).map(|trial|{
        let mut accum=0_i64;
        for (pos,raw) in [a,b,c].into_iter().enumerate(){
            let word=if pos==1{(raw+trial)%4}else{raw};
            accum=if word==3{0}else{(accum+word)%3};
        }
        accum
    }).collect()
}
fn run_original(
    template:&[Value],name:&str,a:i64,b:i64,c:i64,regime:&str,
    disable_edit:bool,expected:&Value,dir:&std::path::Path
)->Value{
    let mut source=template.to_vec();
    for (pc,value) in [(0,a),(3,b),(6,c)]{source[pc]["value"]=json!(value);}
    let founder:Vec<Op>=source.iter().map(decode).collect();
    let mut repaired=founder.clone();
    repaired[6]=Op::Const{dst:3,value:a,next:7};
    let mut world=World::with_strict_capacity(16,180,300);
    if disable_edit{world=world.with_native_code_edits_disabled();}
    world.load(Process::new(1,founder.clone(),CREDIT)).unwrap();
    assert_eq!(world.occupied_units(),177);
    let mut trace=BufWriter::new(File::create(dir.join(format!(
        "b56_f1d1k_g3q_c1_p1_l3_i3_native_{name}.jsonl"))).unwrap());
    let mut steps=0_usize;let mut peak=world.occupied_units();
    let mut witnesses=Vec::<Value>::new();
    let mut resets=Vec::<Value>::new();
    let mut services=Vec::<Value>::new();
    let mut edits=Vec::<Value>::new();
    let mut cert_passes=Vec::<usize>::new();
    let mut meta_fetches=0_usize;
    let mut paid_feedback_reads=Vec::<i64>::new();
    let mut written_feedback=Vec::<i64>::new();
    let mut edit_performed=false;
    let goals=[0_i64,0,2,2];
    while world.step(){
        steps+=1;assert!(steps<=CREDIT as usize);
        let event=world.events().last().unwrap().clone();
        assert!(matches!(event.origin,Origin::Internal(1)));
        if event.pc==174{
            assert!(edits.is_empty(),"more than one original source repair attempt");
            let literal=native_tail(&world);
            if disable_edit{
                assert!(matches!(&event.kind,EventKind::NativeCodeEditSuppressed{opcode_tag:9}));
                assert_eq!(literal,c);
            }else{
                assert!(matches!(&event.kind,EventKind::Executed));
                assert_eq!(literal,a);
                edit_performed=true;
            }
            edits.push(json!({"step":steps,"pc":174,"node":6,
                "before":c,"after":literal,"edit_suppressed":disable_edit}));
        }
        let actor=world.process(1).unwrap();
        assert_eq!(&actor.code.nodes,
            if edit_performed{&repaired}else{&founder},
            "source edits must be original PID1-paid PatchConst only");
        assert!(world.process(2).is_none(),"no reproduction or restarts in I3");
        assert_eq!(world.total_compute_inventory(),Some(CREDIT-steps as u64));
        assert!(world.used_cells()<=16 && world.occupied_units()<=300);
        peak=peak.max(world.occupied_units());
        if event.pc==25{
            assert!(matches!(&event.kind,EventKind::CellRead{..}));
            meta_fetches+=1;
        }
        if event.pc==118{
            match &event.kind{
                EventKind::CellWritten{key,value}=>{
                    assert!((7..=9).contains(key)&&*value==1);
                    witnesses.push(json!({"step":steps,"key":key,"value":value}));
                }
                other=>panic!("certificate must be original source-paid witness: {other:?}")
            }
        }
        if [166,168,170].contains(&event.pc){
            match &event.kind{
                EventKind::CellWritten{key,value}=>{
                    assert!((7..=9).contains(key)&&*value==0);
                    resets.push(json!({"step":steps,"key":key,"value":value}));
                }
                other=>panic!("missing original PID1-paid witness isolation: {other:?}")
            }
        }
        if event.pc==158{
            assert!(matches!(&event.kind,EventKind::CellWritten{key:21,value:1}));
            assert_eq!([world.cell(7),world.cell(8),world.cell(9)],
                [Some(1),Some(1),Some(1)]);
            cert_passes.push(steps);
        }
        if event.pc==67{
            match &event.kind{
                EventKind::CellRead{key:20,value}=>paid_feedback_reads.push(*value),
                other=>panic!("no actor-paid Read20 feedback sensor: {other:?}")
            }
        }
        let snapshot=json!(["cil",steps,event.pc,actor.pc,actor.registers,
            actor.credits,KEYS.map(|key|world.cell(key)),
            world.occupied_units(),native_tail(&world)]);
        serde_json::to_writer(&mut trace,&snapshot).unwrap();
        trace.write_all(b"\n").unwrap();
        if event.pc==59{
            assert_eq!(cert_passes.len(),1,
                "incomplete structural rule must never enter real task service");
            let out=match &event.kind{
                EventKind::CellWritten{key:11,value}=>*value,
                other=>panic!("native service must be original paid Write11: {other:?}")
            };
            let episode=world.cell(32).unwrap_or(0) as usize;
            assert!(episode<4);
            let truth=i64::from(out==goals[episode]);
            let delivered=match regime{
                "truth"=>truth,"inverted"=>1-truth,
                _=>panic!("unregistered feedback regime"),
            };
            let active_middle=world.cell(63).unwrap();
            services.push(json!({"step":steps,"episode":episode,"middle":active_middle,
                "output":out,"truth":truth,"feedback":delivered}));
            written_feedback.push(delivered);
            // Exactly one host action AFTER original paid task service.
            world.intervene(Intervention::SetCell{key:20,value:delivered}).unwrap();
            let external=world.events().last().unwrap();
            assert!(matches!(external.origin,Origin::External));
            assert!(matches!(&external.kind,EventKind::Intervened(_)));
            let env=json!(["feedback",steps,episode,goals[episode],
                active_middle,out,truth,delivered,
                KEYS.map(|key|world.cell(key)),world.occupied_units(),
                native_tail(&world)]);
            serde_json::to_writer(&mut trace,&env).unwrap();
            trace.write_all(b"\n").unwrap();
        }
    }
    trace.flush().unwrap();
    assert!(world.process(1).unwrap().stopped,
        "all64 cases must fail-fast/success-Halt under the 8000 credit cap");
    assert_eq!(steps,expected["steps"].as_u64().unwrap() as usize);
    assert_eq!(world.process(1).unwrap().pc,
        expected["halt_pc"].as_u64().unwrap() as usize);
    assert_eq!(json!(witnesses),expected["witnesses"]);
    assert_eq!(json!(resets),expected["reset_events"]);
    assert_eq!(json!(edits),expected["source_edits"]);
    assert_eq!(json!(cert_passes),expected["certificate_passes"]);
    assert_eq!(json!(services),expected["task_services"]);
    assert_eq!(meta_fetches,expected["paid_meta_fetches"].as_u64().unwrap() as usize);
    assert_eq!(json!(paid_feedback_reads),expected["paid_feedback_reads"]);
    assert_eq!(paid_feedback_reads,written_feedback);
    assert_eq!(peak,expected["peak_occupied"].as_u64().unwrap() as usize);
    assert_eq!(world.used_cells(),expected["used_cells"].as_u64().unwrap() as usize);
    assert_eq!(json!(world.cell(21)),expected["actor_success"]);
    assert_eq!(json!(world.cell(32)),expected["final_episode"]);
    assert_eq!(native_tail(&world),expected["final_source_rule"][2].as_i64().unwrap());
    let modes=semantic_modes(a,b,native_tail(&world));
    assert_eq!(json!(modes),expected["actual_three_mode_outputs"]);
    let truly_safe=modes.iter().copied().collect::<std::collections::BTreeSet<_>>().len()==3;
    assert_eq!(truly_safe,expected["structurally_safe"].as_bool().unwrap());
    assert_eq!(truly_safe,cert_passes.len()==1,
        "UNSOUND CERTIFICATE: actor's in-world witness verification did not match complete rule coverage");
    let external=world.events().iter().filter(|ev|
        matches!(ev.origin,Origin::External)).collect::<Vec<_>>();
    assert_eq!(external.len(),services.len()+1);
    assert!(matches!(&external[0].kind,EventKind::Loaded));
    assert!(external.iter().skip(1).all(|ev|matches!(&ev.kind,EventKind::Intervened(_))));
    Value::Object(serde_json::Map::from_iter([
        ("arm".to_string(),json!(name)),
        ("seed".to_string(),json!([a,b,c])),
        ("paid_original_instructions".to_string(),json!(steps)),
        ("certified".to_string(),json!(cert_passes.len()==1)),
        ("independent_semantic_structural_safe".to_string(),json!(truly_safe)),
        ("source_patches".to_string(),json!(edits)),
        ("paid_witness_reset_events".to_string(),json!(resets)),
        ("task_service_events".to_string(),json!(services)),
        ("true_goal_matches".to_string(),json!(services.iter()
            .filter(|v|v["truth"]==1).count())),
        ("actor_declared_success".to_string(),json!(world.cell(21))),
        ("peak_occupied".to_string(),json!(peak)),
        ("original_PID1_halted".to_string(),json!(true)),
        ("host_rule_source_edits".to_string(),json!(0)),
    ]))
}
#[test]
fn g3q_c1_i3_all64_native_source_paid_certificate_isolation_is_sound(){
    let root=std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let f:Value=serde_json::from_slice(&fs::read(root.join(FROZEN)).unwrap()).unwrap();
    let source=f["exact_original_native_CIL_source"].as_array().unwrap();
    assert_eq!(source.len(),176);
    assert_eq!(f["physics"]["data_capacity"],16);
    assert_eq!(f["physics"]["source_nodes_max"],180);
    assert_eq!(f["physics"]["total_occupied_cap"],300);
    assert_eq!(source[174],json!({"kind":"PatchConst","node":1,"value":3,"next":0}));
    let cases=f["all64_original_pid1_cases"].as_array().unwrap();
    let ctrls=f["adversarial_controls"].as_array().unwrap();
    assert_eq!(cases.len(),64);
    assert_eq!(ctrls.len(),2);
    let dir=root.join("../../cl1b-ci-evidence");
    fs::create_dir_all(&dir).unwrap();
    let mut native_cases=Vec::<Value>::new();
    let mut actual_safe=0_usize;
    let mut actual_patches=0_usize;
    for (ordinal,case) in cases.iter().enumerate(){
        let seed=case["seed"].as_array().unwrap();
        let a=seed[0].as_i64().unwrap();
        let b=seed[1].as_i64().unwrap();
        let c=seed[2].as_i64().unwrap();
        assert_eq!(ordinal,((a*4+b)*4+c) as usize,
            "no researcher-selected candidate omissions or duplicate genotypes");
        let name=format!("a{a}_b{b}_c{c}");
        let result=run_original(source,&name,a,b,c,"truth",false,case,&dir);
        actual_safe+=usize::from(result["certified"]==true);
        actual_patches+=usize::from(!result["source_patches"].as_array().unwrap().is_empty());
        native_cases.push(result);
    }
    assert_eq!(actual_safe,30);
    assert_eq!(actual_patches,40);
    let mut controls=Vec::<Value>::new();
    for (i,c) in ctrls.iter().enumerate(){
        let name=c["name"].as_str().unwrap();
        let disabled=c["disabled"].as_bool().unwrap();
        let result=run_original(source,name,c["a"].as_i64().unwrap(),
            c["b"].as_i64().unwrap(),c["c"].as_i64().unwrap(),
            c["feedback"].as_str().unwrap(),disabled,&c["expected"],&dir);
        if i==0{
            assert_eq!(result["certified"],true);
            assert_eq!(result["actor_declared_success"],1);
            assert_eq!(result["true_goal_matches"],0);
        }else{
            assert_eq!(result["certified"],false);
            assert_eq!(result["task_service_events"].as_array().unwrap().len(),0);
            assert_eq!(result["source_patches"][0]["edit_suppressed"],true);
        }
        controls.push(result);
    }
    fs::write(dir.join("b56_f1d1k_g3q_c1_p1_l3_i3_native_exhaustive64_summary.json"),
        serde_json::to_vec_pretty(&json!({
            "study":"B56-F1d1k-G3Q-C1-P1-L3-I3",
            "level":"NATIVE_EXHAUSTIVE_ALL64_ORIGINAL_PID_PAID_WITNESS_ISOLATION",
            "source_sha":std::env::var("GITHUB_SHA").unwrap_or_else(|_|"local".into()),
            "all64_native_original_worlds":native_cases,
            "adversarial_controls":controls,
            "counts":{"genotypes":64,"initial_safe":24,"initially_unsafe":40,
                "source_patches":actual_patches,"verified_structurally_safe_after_repair":actual_safe,
                "false_certificates":0,"false_rejections":0},
            "limits":"Finite authored grammar and one bank-derived rule patch, not autonomously evolved algorithm. Different original PID1 instance per arm."
        })).unwrap()).unwrap();
}
