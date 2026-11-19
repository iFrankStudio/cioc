//! G3Q-C1-P1-L3-I4 — native original PID1 selects *which source byte* to
//! repair from its own paid structural-certificate evidence; exactly one edit.
//! Frozen 180-node CIL source, unchanged CL0 16/180/300 world physics.
//! Complete 64-genotype sweep + three precommitted adversarial controls.
//! Replayed independently per native charged instruction in focused CI.
//! Researcher authored the bounded strategy; NOT invented open-ended life.
use cl0_world::{EventKind,Intervention,Op,Origin,Process,World};
use serde_json::{json,Value};
use std::fs::{self,File};
use std::io::{BufWriter,Write};

const FREEZE:&str="../../../cl1b/studies/B56-F1d1k-G3Q-C1-P1-L3-I4-frozen-180-node-observation-selected-repair-v1.json";
const KEYS:[i64;14]=[7,8,9,11,20,21,32,33,50,52,56,62,63,64];
const CREDIT:u64=8000;

fn decode(v:&Value)->Op{
    let k=v["kind"].as_str().unwrap();
    let tagged=if k=="Halt"{json!("Halt")}else{
        let mut fields=serde_json::Map::new();
        for (field,value) in v.as_object().unwrap(){
            if field!="kind"{fields.insert(field.clone(),value.clone());}
        }
        let mut out=serde_json::Map::new();
        out.insert(k.to_string(),Value::Object(fields));
        Value::Object(out)
    };
    serde_json::from_value(tagged).expect("invalid prospectively frozen CIL")
}

fn executable_gene(w:&World,node:usize)->i64{
    match &w.process(1).unwrap().code.nodes[node]{
        Op::Const{dst:3,value,next} if (node==0&&*next==1)
        ||(node==3&&*next==4)||(node==6&&*next==7)=>*value,
        other=>panic!("CIL gene source differs from preregistered opcode: {other:?}"),
    }
}
fn mode_oracle(gene:[i64;3])->Vec<i64>{
    (0..3_i64).map(|trial|{
        let mut acc=0_i64;
        for (idx,raw) in gene.into_iter().enumerate(){
            let op=if idx==1{(raw+trial)%4}else{raw};
            acc=if op==3{0}else{(acc+op)%3};
        }
        acc
    }).collect()
}

fn original_world(
    frozen:&[Value],name:&str,initial:[i64;3],feedback_regime:&str,
    suppressed:bool,expected:&Value,dir:&std::path::Path
)->Value{
    let mut source=frozen.to_vec();
    for (i,pc) in [0,3,6].into_iter().enumerate(){
        source[pc]["value"]=json!(initial[i]);
    }
    let founder=source.iter().map(decode).collect::<Vec<Op>>();
    let mut native=World::with_strict_capacity(16,180,300);
    if suppressed{native=native.with_native_code_edits_disabled();}
    native.load(Process::new(1,founder.clone(),CREDIT)).unwrap();
    assert_eq!(native.occupied_units(),181);
    let mut trace=BufWriter::new(File::create(dir.join(format!(
      "b56_f1d1k_g3q_c1_p1_l3_i4_original_paid_{name}.jsonl"))).unwrap());
    let mut steps=0_usize;
    let mut peak=native.occupied_units();
    let mut certificate_marks=Vec::<Value>::new();
    let mut witness_resets=Vec::<Value>::new();
    let mut passes=Vec::<usize>::new();
    let mut services=Vec::<Value>::new();
    let mut edits=Vec::<Value>::new();
    let mut feedback_reads=Vec::<i64>::new();
    let mut feedback_writes=Vec::<i64>::new();
    let mut paid_meta_fetches=0_usize;
    let mut guard_writes=0_usize;
    let mut selected_node:Option<Value>=None;
    let mut patched_node:Option<usize>=None;
    while native.step(){
        steps+=1;assert!(steps<=CREDIT as usize);
        let event=native.events().last().unwrap().clone();
        assert!(matches!(event.origin,Origin::Internal(1)),
          "native charged instruction must belong to original PID1");
        if event.pc==173{
            assert!(selected_node.is_none());
            let p=native.process(1).unwrap();
            let cond=p.registers[1];
            assert!(cond==0||cond==1,"paid witness residual selection must be binary");
            let target=if cond!=0{3}else{6};
            selected_node=Some(json!({"step":steps,"register1":cond,"chosen_node":target}));
        }
        if [177,179].contains(&event.pc){
            assert!(edits.is_empty(),"at most one original executable source patch");
            let node=if event.pc==177{6}else{3};
            assert_eq!(selected_node.as_ref().unwrap()["chosen_node"],json!(node));
            let prior=executable_gene(&native,node);
            let observed_after=prior;
            if suppressed{
                assert!(matches!(&event.kind,EventKind::NativeCodeEditSuppressed{opcode_tag:9}));
            }else{
                assert!(matches!(&event.kind,EventKind::Executed));
                patched_node=Some(node);
            }
            edits.push(json!({"step":steps,"pc":event.pc,
                "node":node,"before":initial[if node==3{1}else{2}],
                "after":observed_after,"edit_suppressed":suppressed}));
        }
        let p=native.process(1).unwrap();
        let mut expected_source=founder.clone();
        if let Some(node)=patched_node{
            expected_source[node]=Op::Const{
                dst:3,value:if node==3{0}else{initial[0]},
                next:if node==3{4}else{7}
            };
        }
        assert_eq!(p.code.nodes,expected_source,
            "only original actor-paid observation-selected PatchConst may edit source");
        assert!(native.process(2).is_none());
        assert_eq!(native.total_compute_inventory(),Some(CREDIT-steps as u64));
        assert!(native.used_cells()<=16&&native.occupied_units()<=300);
        peak=peak.max(native.occupied_units());
        if event.pc==25{
            assert!(matches!(&event.kind,EventKind::CellRead{..}));
            paid_meta_fetches+=1;
        }
        if event.pc==118{
            match &event.kind{
                EventKind::CellWritten{key,value}=>{
                    assert!((7..=9).contains(key)&&*value==1);
                    certificate_marks.push(json!({"step":steps,"key":key,"value":value}));
                }
                other=>panic!("missing actor paid coverage witness: {other:?}")
            }
        }
        if [166,168,170].contains(&event.pc){
            match &event.kind{
                EventKind::CellWritten{key,value}=>{
                    assert!((7..=9).contains(key)&&*value==0);
                    witness_resets.push(json!({"step":steps,"key":key,"value":value}));
                }
                other=>panic!("missing native paid witness reset: {other:?}")
            }
        }
        if event.pc==158{
            assert!(matches!(&event.kind,EventKind::CellWritten{key:21,value:1}));
            assert_eq!([native.cell(7),native.cell(8),native.cell(9)],
                       [Some(1),Some(1),Some(1)]);
            passes.push(steps);
        }
        if event.pc==163{
            assert!(matches!(&event.kind,EventKind::CellWritten{key:11,value:1}));
            guard_writes+=1;
        }
        if event.pc==67{
            match &event.kind{
                EventKind::CellRead{key:20,value}=>feedback_reads.push(*value),
                other=>panic!("original PID1 has not paid to Read20 feedback: {other:?}")
            }
        }
        let payload=json!(["cil",steps,event.pc,p.pc,p.registers,p.credits,
            KEYS.map(|key|native.cell(key)),native.occupied_units(),
            executable_gene(&native,3),executable_gene(&native,6)]);
        serde_json::to_writer(&mut trace,&payload).unwrap();
        trace.write_all(b"\n").unwrap();
        if event.pc==59{
            assert_eq!(passes.len(),1,
                "native task service cannot run prior to complete source-paid certificate");
            let output=match &event.kind{
                EventKind::CellWritten{key:11,value}=>*value,
                other=>panic!("task must be source-paid Write11: {other:?}")
            };
            let episode=native.cell(32).unwrap_or(0) as usize;
            assert!(episode<4);
            let goal=[0_i64,0,2,2][episode];
            let actual=i64::from(output==goal);
            let feedback=match feedback_regime{
                "truth"=>actual,"inverted"=>1-actual,
                other=>panic!("unknown pre-registered feedback regime {other}")
            };
            let active=native.cell(63).unwrap();
            services.push(json!({"step":steps,"episode":episode,"middle":active,
                "output":output,"truth":actual,"feedback":feedback}));
            feedback_writes.push(feedback);
            native.intervene(Intervention::SetCell{key:20,value:feedback}).unwrap();
            let outside=native.events().last().unwrap();
            assert!(matches!(outside.origin,Origin::External));
            assert!(matches!(&outside.kind,EventKind::Intervened(_)));
            let row=json!(["feedback",steps,episode,goal,active,output,actual,
                feedback,KEYS.map(|key|native.cell(key)),native.occupied_units(),
                executable_gene(&native,3),executable_gene(&native,6)]);
            serde_json::to_writer(&mut trace,&row).unwrap();
            trace.write_all(b"\n").unwrap();
        }
    }
    trace.flush().unwrap();
    assert!(native.process(1).unwrap().stopped,
        "bounded fail-fast and all original PID1 tasks must naturally Halt");
    let final_genotype=[executable_gene(&native,0),
       executable_gene(&native,3),executable_gene(&native,6)];
    let outcomes=mode_oracle(final_genotype);
    let safe=outcomes.iter().copied()
        .collect::<std::collections::BTreeSet<_>>().len()==3;
    assert_eq!(safe,passes.len()==1,
        "structural certificate must match independent semantic 3-mode oracle");
    let exact=&expected;
    assert_eq!(steps,exact["paid_steps"].as_u64().unwrap() as usize);
    assert_eq!(native.process(1).unwrap().pc,exact["halt_pc"].as_u64().unwrap() as usize);
    assert_eq!(json!(final_genotype),exact["source_rule_after"]);
    assert_eq!(json!(outcomes),exact["actual_mode_sequence"]);
    assert_eq!(safe,exact["actual_three_mode_coverage"].as_bool().unwrap());
    assert_eq!(json!(certificate_marks),exact["paid_witness_marks"]);
    assert_eq!(json!(witness_resets),exact["paid_witness_resets"]);
    assert_eq!(json!(passes),exact["certificate_pass_steps"]);
    assert_eq!(json!(services),exact["original_paid_services"]);
    assert_eq!(json!(edits),exact["original_source_edits"]);
    assert_eq!(json!(selected_node),exact["chosen_repair_provenance"]);
    assert_eq!(paid_meta_fetches,exact["original_paid_meta_fetches"].as_u64().unwrap() as usize);
    assert_eq!(json!(feedback_reads),exact["original_paid_feedback_reads"]);
    assert_eq!(feedback_reads,feedback_writes);
    assert_eq!(guard_writes,exact["original_paid_retry_guard_writes"].as_u64().unwrap() as usize);
    assert_eq!(native.used_cells(),exact["occupied_data_cells"].as_u64().unwrap() as usize);
    assert_eq!(peak,exact["peak_occupied_units"].as_u64().unwrap() as usize);
    assert_eq!(json!(native.cell(21)),exact["actor_declared_success"]);
    assert_eq!(json!(native.cell(32)),exact["actor_final_episode"]);
    assert!(steps<CREDIT as usize);
    let outside=native.events().iter().filter(|ev|matches!(ev.origin,Origin::External))
        .collect::<Vec<_>>();
    assert_eq!(outside.len(),services.len()+1);
    assert!(matches!(&outside[0].kind,EventKind::Loaded));
    assert!(outside.iter().skip(1).all(|ev|matches!(&ev.kind,EventKind::Intervened(_))));
    json!({
        "arm":name,"initial_rule":initial,"original_paid_steps":steps,
        "original_task_services":services,"original_paid_source_edits":edits,
        "native_original_PID1_Halted":true,"native_certified":safe,
        "semantic_rule_modes":outcomes,"choice_provenance":selected_node,
        "original_paid_witness_resets":witness_resets,
        "peak_occupied":peak,"postload_host_source_edits":0,
        "true_task_matches":services.iter().filter(|v|v["truth"]==1).count(),
        "actor_declared_success":native.cell(21)
    })
}

#[test]
fn g3q_c1_i4_all64_original_pid_evidence_selected_two_target_repair(){
    let root=std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let f:Value=serde_json::from_slice(&fs::read(root.join(FREEZE)).unwrap()).unwrap();
    let code=f["self_modified_original_CIL_source"].as_array().unwrap();
    assert_eq!(code.len(),180);
    assert_eq!(f["world_caps"]["strict_data_cells"],16);
    assert_eq!(f["world_caps"]["source_nodes"],180);
    assert_eq!(f["world_caps"]["strict_total_occupied"],300);
    assert_eq!(code[177],json!({"kind":"PatchConst","node":1,"value":3,"next":0}));
    assert_eq!(code[179],json!({"kind":"PatchConst","node":1,"value":3,"next":0}));
    let cases=f["precommitted_all64_original_pid1_arms"].as_array().unwrap();
    let controls=f["precommitted_controls"].as_array().unwrap();
    assert_eq!(cases.len(),64);assert_eq!(controls.len(),3);
    let dir=root.join("../../cl1b-ci-evidence");
    fs::create_dir_all(&dir).unwrap();
    let mut original_cases=Vec::<Value>::new();
    let mut fixed=0_usize;
    let mut chosen_tail=0_usize;
    let mut chosen_middle=0_usize;
    for (i,arm) in cases.iter().enumerate(){
        let seed=arm["seed"].as_array().unwrap();
        let a=seed[0].as_i64().unwrap();
        let b=seed[1].as_i64().unwrap();
        let c=seed[2].as_i64().unwrap();
        assert_eq!(i,((a*4+b)*4+c) as usize,"missing or reordered genotype");
        let name=format!("a{a}_b{b}_c{c}");
        let result=original_world(code,&name,[a,b,c],"truth",false,
            &arm["expected"],&dir);
        let initially_safe=mode_oracle([a,b,c]).iter().copied()
            .collect::<std::collections::BTreeSet<_>>().len()==3;
        assert_eq!(initially_safe,arm["initial_structurally_safe"].as_bool().unwrap());
        if !initially_safe && result["native_certified"]==true{fixed+=1;}
        if let Some(op)=result["original_paid_source_edits"].as_array().unwrap().first(){
            match op["node"].as_u64().unwrap(){
                3=>chosen_middle+=1,6=>chosen_tail+=1,_=>panic!("unexpected patch target")
            }
        }
        original_cases.push(result);
    }
    assert_eq!(fixed,30,"native repaired success must beat verified I3 6/40");
    assert_eq!(chosen_tail,16);
    assert_eq!(chosen_middle,24);
    let mut original_controls=Vec::new();
    for control in controls{
        let name=control["name"].as_str().unwrap();
        let suppress=control["disabled"].as_bool().unwrap();
        let result=original_world(code,name,[
            control["first"].as_i64().unwrap(),
            control["middle"].as_i64().unwrap(),
            control["tail"].as_i64().unwrap()],
            control["feedback"].as_str().unwrap(),
            suppress,&control["expected"],&dir);
        if suppress{
            assert_eq!(result["native_certified"],false);
            assert_eq!(result["original_task_services"].as_array().unwrap().len(),0);
        }else{
            assert_eq!(result["native_certified"],true);
            assert_eq!(result["actor_declared_success"],1);
        }
        original_controls.push(result);
    }
    let summary=f["precommitted_native_summaries"].clone();
    assert_eq!(summary["initial_safe"],24);
    assert_eq!(summary["repaired_to_safe"],30);
    assert_eq!(summary["false_certificates"],0);
    assert_eq!(summary["false_rejections"],0);
    fs::write(dir.join("b56_f1d1k_g3q_c1_p1_l3_i4_native_exhaustive64_observation_repair.json"),
        serde_json::to_vec_pretty(&json!({
            "study":"B56-F1d1k-G3Q-C1-P1-L3-I4",
            "level":"NATIVE_ORIGINAL_PID_EVIDENCE_CONDITIONED_EDIT_TARGET_EXHAUSTIVE64",
            "source_sha":std::env::var("GITHUB_SHA").unwrap_or_else(|_|"local".to_string()),
            "original64_native_arms":original_cases,
            "matched_native_controls":original_controls,
            "counts":{"initial_safe":24,"initial_unsafe":40,
                "self_edits":40,"chosen_tail":16,"chosen_middle":24,
                "repaired_to_safe":fixed,"final_safe":54,
                "false_certificates":0,"false_rejections":0},
            "comparison":"I3 exact-native 6/40 rescued; I4 exact-native 30/40 expected, subject to independent exact-source CI replay",
            "limits":"Two-way preauthored structural-evidence repair policy; actor does not autonomously invent or generalize meta-rule grammar."
        })).unwrap()).unwrap();
}
