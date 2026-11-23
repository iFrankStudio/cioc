//! C16 original native PID1 single-use evidence ticket with paid memory and causal controls.
//! Source-authored success-memory policy, NOT learned ROI / B3 / B4.
use cl0_world::{EventKind, Intervention, Op, Origin, Process, World};
use serde_json::{json, Value};
use std::fs::{self, File};
use std::io::{BufWriter, Write};

const C10: &str="../../../cl1b/studies/CL1B-G2-ACTOR-PAID-STATE-TRIGGERED-ROLLBACK-C10-frozen-274-worlds-v1.json";
const C14_FROZEN: &str="../../../cl1b/studies/CL1B-C14-NATIVE-FEEDBACK-ATTESTED-SUCCESS-SOURCE-RESTORATION-frozen-274-worlds-v1.json";
const C16_FROZEN: &str="../../../cl1b/studies/CL1B-C16-SINGLE-USE-PAID-SUCCESS-CERTIFICATE-frozen-274-worlds-v1.json";
const CELLS: [i64; 17]=[11,20,21,30,32,33,34,35,36,37,38,39,40,41,42,44,45];
const CREDIT: u64=6000;

fn decode(v:&Value)->Op {
    let tag=v["kind"].as_str().unwrap();
    let raw=if tag=="Halt" {json!("Halt")} else {
        let mut m=v.as_object().unwrap().clone();
        m.remove("kind");
        let mut out=serde_json::Map::new();
        out.insert(tag.to_string(),Value::Object(m));
        Value::Object(out)
    };
    serde_json::from_value(raw).expect("source node")
}
fn source(m:&Value,arm:&str)->Vec<Op> {
    let ticket=arm=="truth_one_use_reentry"||arm.starts_with("same_source_");
    let reentry=ticket||arm=="different_source_c15_unlimited_reentry";
    let c14=reentry||arm=="different_source_c14c_no_reentry";
    let old_c9=arm=="different_source_c9_fixed_abstain";
    assert!(c14||old_c9);
    let key=if old_c9{"frozen_source_g2_C9_checkpoint_abstain"}
        else{"frozen_source_g2_rollback"};
    let mut code=m[key].as_array().unwrap().iter().map(decode).collect::<Vec<_>>();
    assert_eq!(code.len(),180);
    if c14 {
        assert_eq!(code[6],Op::Branch{test:0,nonzero:29,zero:7});
        assert_eq!(code[99],Op::Const{dst:2,value:42,next:100});
        code[6]=Op::Branch{test:0,nonzero:180,zero:7};
        code[99]=Op::Const{dst:2,value:44,next:100};
        code.extend([
            json!({"kind":"Const","dst":2,"value":41,"next":181}),
            json!({"kind":"ReadLastWriter","dst":0,"address":2,"next":182}),
            json!({"kind":"Const","dst":2,"value":1,"next":183}),
            json!({"kind":"Sub","dst":0,"lhs":0,"rhs":2,"next":184}),
            json!({"kind":"Branch","test":0,"nonzero":29,"zero":185}),
            json!({"kind":"Const","dst":2,"value":41,"next":186}),
            json!({"kind":"Read","dst":0,"address":2,"next":187}),
            json!({"kind":"Const","dst":2,"value":44,"next":188}),
            json!({"kind":"Write","address":2,"value":0,"next":29})
        ].iter().map(decode));
    }
    if reentry {
        assert_eq!(code[95],Op::Branch{test:0,nonzero:99,zero:86});
        assert_eq!(code[102],Op::PatchConst{node:1,value:0,next:103});
        code[95]=Op::Branch{test:0,nonzero:189,zero:86};
        code[102]=Op::PatchConst{node:1,value:0,next:173};
        code.extend([
            json!({"kind":"Const","dst":2,"value":44,"next":190}),
            json!({"kind":"ReadLastWriter","dst":0,"address":2,"next":191}),
            json!({"kind":"Const","dst":2,"value":1,"next":192}),
            json!({"kind":"Sub","dst":0,"lhs":0,"rhs":2,"next":193}),
            json!({"kind":"Branch","test":0,"nonzero":103,"zero":99})
        ].iter().map(decode));
    }
    if ticket {
        assert_eq!(code[188],Op::Write{address:2,value:0,next:29});
        assert_eq!(code[95],Op::Branch{test:0,nonzero:189,zero:86});
        assert_eq!(code[102],Op::PatchConst{node:1,value:0,next:173});
        code[188]=Op::Write{address:2,value:0,next:194};
        code[95]=Op::Branch{test:0,nonzero:197,zero:86};
        code[102]=Op::PatchConst{node:1,value:0,next:200};
        code.extend([
            json!({"kind":"Const","dst":2,"value":45,"next":195}),
            json!({"kind":"Const","dst":0,"value":0,"next":196}),
            json!({"kind":"Write","address":2,"value":0,"next":29}),
            json!({"kind":"Const","dst":2,"value":45,"next":198}),
            json!({"kind":"Read","dst":0,"address":2,"next":199}),
            json!({"kind":"Branch","test":0,"nonzero":103,"zero":189}),
            json!({"kind":"Const","dst":2,"value":45,"next":201}),
            json!({"kind":"Const","dst":0,"value":1,"next":202}),
            json!({"kind":"Write","address":2,"value":0,"next":173}),
        ].iter().map(decode));
    }
    assert_eq!(code.len(),if ticket {203} else if reentry {194} else if c14 {189} else {180});
    code
}
fn signature(code:&[Op])->[i64;5] {
    let ed=|i:usize|match &code[i] {
        Op::Write{next,..}=>*next as i64,
        x=>panic!("expected Write[{i}]: {x:?}")
    };
    let lit=|i:usize|match &code[i] {
        Op::Const{value,..}=>*value,
        x=>panic!("expected Const[{i}]: {x:?}")
    };
    [ed(12),ed(34),lit(46),ed(133),lit(173)]
}
fn trial(m:&Value,name:&str,arm:&str,mode:&str,goals:&[i64],
    dir:&std::path::Path)->Value {
    assert!(matches!(mode,"truth"|"inverted"|"forced0"));
    assert_eq!(goals.len(),48);
    let ticket=arm=="truth_one_use_reentry"||arm.starts_with("same_source_");
    let reentry=ticket||arm=="different_source_c15_unlimited_reentry";
    let c14=reentry||arm=="different_source_c14c_no_reentry";
    let suppress=arm=="same_source_pc102_effect_suppressed";
    let suppress_ticket=arm=="same_source_pc202_ticket_consumption_suppressed";
    let clamp:Option<i64>=None;
    let code=source(m,arm);
    let mut expected=code.clone();
    let mut world=World::with_strict_capacity(16,203,300).with_cell_write_provenance();
    assert!(world.cell_write_provenance_enabled());
    if suppress {world=world.with_native_patchconst_suppression(1,102,173);}
    if suppress_ticket {world=world.with_native_write_suppression(1,202,45,1);}
    world.load(Process::new(1,code,CREDIT)).unwrap();
    let mut trace=BufWriter::new(File::create(dir.join(
        format!("cl1b_c16_original_{name}.jsonl"))).unwrap());
    let mut paid=0_u64;
    let mut peak=world.occupied_units();
    let mut services=Vec::<Value>::new();
    let mut edits=Vec::<Value>::new();
    let mut memories=Vec::<Value>::new();
    let mut restores=Vec::<Value>::new();
    let mut subsequent_readbacks=Vec::<Value>::new();
    let mut success_records=0_u64;
    let mut truly_correct_records=0_u64;
    let mut external_tampers=0_u64;
    let mut attempted_pc102_173=0_u64;
    let mut effective_restores=0_u64;
    let mut last_pc173=None::<u64>;
    let mut last_pc52=None::<u64>;
    let mut last_service=None::<(u64,i64,i64)>;
    let mut pending_restored=None::<(u64,i64)>;
    let mut writer_checks=Vec::<Value>::new();
    let mut skipped_unverified_positive=0_u64;
    let mut stale_success_code_samples=0_u64;
    let mut writer_auth_checks=0_u64;
    let mut writer_auth_allowed=0_u64;
    let mut writer_auth_blocked=0_u64;
    let mut pending_reentry=false;
    let mut pending_pc52=false;
    let mut reentry_executions=0_u64;
    let mut post_reentry_pc52_writes=0_u64;
    let mut certificates_minted=0_u64;
    let mut ticket_read_attempts=0_u64;
    let mut tickets_spent=0_u64;
    let mut ticket_gate_rejections=0_u64;
    let mut suppressed_ticket_writes=0_u64;
    while world.step() {
        paid+=1;
        assert!(paid<=CREDIT,"native private budget too small");
        let e=world.events().last().unwrap().clone();
        assert_eq!(e.origin,Origin::Internal(1));
        let at=e.pc;
        if pending_reentry {
            assert_eq!(at,173,"C15 paid restored source must enter physical PC173");
            reentry_executions+=1;
            pending_reentry=false;
            pending_pc52=true;
        }
        let p=world.process(1).unwrap();
        assert_eq!(p.credits,CREDIT-paid);
        assert_eq!(world.total_compute_inventory(),Some(CREDIT-paid));
        assert!(world.process(2).is_none());
        assert!(world.used_cells()<=16);
        assert!(world.occupied_units()<=300);
        assert_eq!(p.code.nodes.len(),if ticket{203}else if reentry{194}else if c14{189}else{180});
        if [45,88,102,122,28].contains(&at) {
            let target=p.registers[1] as usize;
            let val=if at==28{p.registers[3]}else{p.registers[0]};
            let before=match &expected[target]{
                Op::Const{value,..}=>*value,
                x=>panic!("patch must target Const: {x:?}")
            };
            let suppressed=at==102&&suppress;
            if suppressed {
                assert!(matches!(e.kind,EventKind::NativeCodeEditSuppressed{opcode_tag:9}));
            } else {
                assert!(matches!(e.kind,EventKind::Executed));
                if let Op::Const{value,..}=&mut expected[target] {*value=val;}
            }
            if at==102&&target==173 {
                attempted_pc102_173+=1;
                if ticket {
                    assert_eq!(p.pc,200,"C16 PC102 must pay to mark spent evidence before physical code reentry");
                } else if reentry {
                    assert_eq!(p.pc,173,"C15 reference PC102 must directly reenter physical PC173");
                    pending_reentry=true;
                }
                assert_eq!(val,world.cell(if c14{44}else{42}).unwrap_or(0),
                    "paid restore source MUST match visible original actor memory");
                let effective=!suppressed&&val!=before;
                if effective {
                    effective_restores+=1;
                    pending_restored=Some((paid,val));
                }
                restores.push(json!([paid,before,val,suppressed,effective]));
            }
            edits.push(json!([at,target,before,val,suppressed]));
        }
        if [26,82,131,136,139,155].contains(&at) {
            let target=p.registers[1] as usize;
            let dest=p.registers[0] as usize;
            if let Op::Write{next,..}=&mut expected[target] {
                edits.push(json!([at,target,*next,dest,false]));
                *next=dest;
            } else {panic!("PatchNext must target Write");}
        }
        assert_eq!(p.code.nodes,expected,"unexpected unpaid native source mutation in {name} at {paid}");
        if at==173 {
            assert_eq!(p.registers[2],signature(&p.code.nodes)[4]);
            last_pc173=Some(paid);
            if let Some((rp,rv))=pending_restored.take() {
                assert!(rp<paid&&p.registers[2]==rv);
                subsequent_readbacks.push(json!([rp,paid,rv]));
            }
        }
        if at==52 {
            if pending_pc52 {
                post_reentry_pc52_writes+=1;
                pending_pc52=false;
            }
            assert!(last_pc173.unwrap()<paid);
            assert!(matches!(&e.kind,EventKind::CellWritten{key:41,value}
                if *value==p.registers[2]));
            last_pc52=Some(paid);
        }
        if at==181&&c14 {
            assert!([-1,0,1].contains(&p.registers[0]));
            assert!(matches!(&e.kind,EventKind::CellWriterSensed{key:41,writer} if *writer==p.registers[0]));
            writer_checks.push(json!([paid,p.registers[0]]));
        }
        if at==184&&c14 {
            assert!(matches!(e.kind,EventKind::Executed));
            if p.pc==29 {
                skipped_unverified_positive+=1;
            } else {
                assert_eq!(p.pc,185);
                assert!(last_pc173.is_some()&&last_pc52.is_some());
            }
        }
        if at==196&&ticket {
            assert_eq!(world.cell(45),Some(0));
            assert_eq!(world.cell_last_writer(45),Some(1));
            assert!(matches!(&e.kind,EventKind::CellWritten{key:45,value:0}));
            certificates_minted+=1;
        }
        if at==198&&ticket {
            assert!(matches!(&e.kind,EventKind::CellRead{key:45,value} if *value==p.registers[0]));
            ticket_read_attempts+=1;
        }
        if at==199&&ticket {
            assert!(matches!(e.kind,EventKind::Executed));
            if p.pc==103 {
                assert_eq!(world.cell(45),Some(1));
                ticket_gate_rejections+=1;
            } else {
                assert_eq!(p.pc,189);
                assert!(world.cell(45).unwrap_or(0)==0);
            }
        }
        if at==202&&ticket {
            assert_eq!(p.pc,173);
            if suppress_ticket {
                assert!(matches!(&e.kind,EventKind::NativeCellWriteSuppressed{key:45,value:1}));
                assert_eq!(world.cell(45),Some(0));
                suppressed_ticket_writes+=1;
            } else {
                assert!(matches!(&e.kind,EventKind::CellWritten{key:45,value:1}));
                assert_eq!(world.cell(45),Some(1));
                assert_eq!(world.cell_last_writer(45),Some(1));
            }
            tickets_spent+=1;
            pending_reentry=true;
        }
        if at==190&&reentry {
            writer_auth_checks+=1;
            assert!(matches!(&e.kind,EventKind::CellWriterSensed{key:44,writer}
                if *writer==p.registers[0]));
        }
        if at==193&&reentry {
            assert!(matches!(e.kind,EventKind::Executed));
            if p.pc==99 {
                writer_auth_allowed+=1;
                assert_eq!(world.cell_last_writer(44),Some(1));
            } else {
                assert_eq!(p.pc,103);
                writer_auth_blocked+=1;
            }
        }
        if at==188 {
            assert!(c14);
            let (step,truth,feedback)=last_service.expect("last service must exist");
            assert!(step<paid&&feedback==1,"paid sample cannot predate feedback");
            assert!(last_pc173.unwrap()<last_pc52.unwrap()&&last_pc52.unwrap()<step);
            let current=world.cell(41).unwrap();
            assert!(matches!(&e.kind,EventKind::CellWritten{key:44,value}
                if *value==current));
            assert_eq!(p.registers[0],current);
            success_records+=1;
            if truth==1 {truly_correct_records+=1;}
            if current!=signature(&p.code.nodes)[4] {stale_success_code_samples+=1;}
            memories.push(json!([paid,step,truth,feedback,current,signature(&p.code.nodes)[4]]));
        }
        peak=peak.max(world.occupied_units());
        serde_json::to_writer(&mut trace,&json!([
            "cil",paid,at,p.pc,p.registers,p.credits,
            CELLS.map(|k|world.cell(k)),world.occupied_units(),
            signature(&p.code.nodes),
            if at==102&&suppress{"suppressed_pc102"}
            else if at==202&&suppress_ticket{"suppressed_pc202"}
            else if at==188{"paid_success_source_write"}else{"normal"}
        ])).unwrap();
        trace.write_all(b"\n").unwrap();
        if at==188 {
            if let Some(value)=clamp {
                let old=world.cell(44).unwrap();
                world.intervene(Intervention::SetCell{key:44,value}).unwrap();
                assert_eq!(world.events().last().unwrap().origin,Origin::External);
                external_tampers+=1;
                peak=peak.max(world.occupied_units());
                serde_json::to_writer(&mut trace,&json!([
                    "tamper",paid,44,old,value,CELLS.map(|k|world.cell(k)),
                    world.occupied_units(),signature(&world.process(1).unwrap().code.nodes)
                ])).unwrap();
                trace.write_all(b"\n").unwrap();
            }
        }
        if at==3 {
            let guess=match &e.kind {
                EventKind::CellWritten{key:11,value}=>*value,
                x=>panic!("service must be original paid Write11 {x:?}")
            };
            let stage=world.cell(32).unwrap_or(0) as usize;
            assert!(stage<48);
            let truth=i64::from(guess==goals[stage]);
            let feedback=match mode {
                "truth"=>truth,"inverted"=>1-truth,"forced0"=>0,_=>unreachable!()
            };
            services.push(json!([stage,guess,truth,feedback]));
            last_service=Some((paid,truth,feedback));
            world.intervene(Intervention::SetCell{key:20,value:feedback}).unwrap();
            assert_eq!(world.events().last().unwrap().origin,Origin::External);
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
    assert_eq!(success_records,memories.len() as u64);
    assert_eq!(external_tampers,if clamp.is_some(){success_records}else{0});
    if !c14 {assert_eq!(success_records,0);}
    if mode=="truth"&&c14 {assert_eq!(success_records,truly_correct_records);}
    let true_count=services.iter().filter(|x|x[2]==1).count();
    let completed=world.cell(32).unwrap_or(0);
    let claim=world.cell(21).unwrap_or(0);
    let exhausted=paid==CREDIT;
    if mode=="forced0" {assert_eq!(completed,0);}
    if mode=="inverted" {assert!(true_count<services.len());}
    json!({
        "name":name,"arm":arm,"feedback":mode,"goals":goals,
        "steps":paid,"services":services,"source_edits":edits,
        "success_source_memories":memories,"physical_parent_restorations":restores,
        "subsequent_physical_pc173_readbacks":subsequent_readbacks,
        "claimed_success_records":success_records,
        "paid_writer_checks":writer_checks,
        "skipped_unverified_positive":skipped_unverified_positive,
        "stale_success_code_samples":stale_success_code_samples,
        "paid_source_writer_auth_checks":writer_auth_checks,
        "paid_source_writer_auth_allowed":writer_auth_allowed,
        "paid_source_writer_auth_blocked":writer_auth_blocked,
        "paid_pc102_reentry_pc173_actual_executions":reentry_executions,
        "paid_pc52_after_reentry_actual_writes":post_reentry_pc52_writes,
        "pending_reentry_when_credits_exhausted":pending_reentry,
        "paid_success_certificates_minted":certificates_minted,
        "paid_ticket_read_attempts":ticket_read_attempts,
        "paid_ticket_spend_writes":tickets_spent,
        "paid_ticket_consumed_gate_rejections":ticket_gate_rejections,
        "paid_ticket_spend_write_effect_suppressed":suppressed_ticket_writes,
        "final_ticket_cell45":world.cell(45),
        "truly_correct_source_records":truly_correct_records,
        "external_cell44_tampers":external_tampers,
        "paid_pc102_target173_attempts":attempted_pc102_173,
        "effective_pc173_restorations":effective_restores,
        "final_cell44":world.cell(44),
        "completed_stages":completed,"true_matches":true_count,"claim":claim,
        "peak_occupied":peak,"final_signature":signature(&world.process(1).unwrap().code.nodes),
        "budget_exhausted":exhausted
    })
}
#[test]
fn c16_original_274_native_one_use_success_certificate() {
    let root=std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let parent:Value=serde_json::from_slice(&fs::read(root.join(C10)).unwrap()).unwrap();
    let frozen:Value=serde_json::from_slice(&fs::read(root.join(C14_FROZEN)).unwrap()).unwrap();
    let policy:Value=serde_json::from_slice(&fs::read(root.join(C16_FROZEN)).unwrap()).unwrap();
    assert_eq!(frozen["study"],"CL1B-C14-NATIVE-FEEDBACK-ATTESTED-SUCCESS-SOURCE-RESTORATION");
    assert_eq!(policy["study"],"CL1B-C16-SINGLE-USE-PAID-SUCCESS-CERTIFICATE");
    assert_eq!(policy["status"],"PRE_NATIVE_FROZEN");
    let exact_source=source(&parent,"truth_one_use_reentry");
    assert_eq!(exact_source.len(),203);
    assert_eq!(frozen["status"],"PRE_NATIVE_FROZEN");
    let cases=policy["cases"].as_array().unwrap();
    let arms=policy["arms"].as_array().unwrap();
    let controls=policy["false_feedback_controls"].as_array().unwrap();
    assert_eq!((cases.len(),arms.len(),controls.len()),(45,6,4));
    assert_eq!(policy["counts"]["all_native_worlds"],274);
    let mut old=std::collections::BTreeSet::new();
    for case in parent["cases"].as_array().unwrap(){old.insert(case["goals"].to_string());}
    for case in frozen["cases"].as_array().unwrap(){old.insert(case["goals"].to_string());}
    let mut fresh=std::collections::BTreeSet::new();
    let out_dir=root.join("../../cl1b-ci-evidence");
    fs::create_dir_all(&out_dir).unwrap();
    let mut records=Vec::new();
    let mut adversarial=Vec::new();
    let mut charged=0_u64;
    let mut success_records=0_u64;
    let mut genuine_restores=0_u64;
    let mut independent_gain_cases=0_u64;
    let mut sample_caused_changed_cases=0_u64;
    let mut arm_fees=std::collections::BTreeMap::<String,u64>::new();
    let mut arm_completed=std::collections::BTreeMap::<String,u64>::new();
    let mut arm_physical=std::collections::BTreeMap::<String,u64>::new();
    let mut incomplete=Vec::new();
    for case in cases {
        assert!(!old.contains(&case["goals"].to_string()));
        assert!(fresh.insert(case["goals"].to_string()));
        let base=case["name"].as_str().unwrap();
        let goals=case["goals"].as_array().unwrap().iter().map(|x|x.as_i64().unwrap()).collect::<Vec<_>>();
        let mut row=Vec::new();
        for a in arms {
            let arm=a.as_str().unwrap();
            let v=trial(&parent,&format!("{base}_{arm}"),arm,"truth",&goals,&out_dir);
            charged+=v["steps"].as_u64().unwrap();
            *arm_fees.entry(arm.to_string()).or_default()+=v["steps"].as_u64().unwrap();
            *arm_physical.entry(arm.to_string()).or_default()+=v["effective_pc173_restorations"].as_u64().unwrap();
            let ok=v["completed_stages"]==48&&v["true_matches"]==48&&v["claim"]==1
                &&v["budget_exhausted"]==false;
            if ok {*arm_completed.entry(arm.to_string()).or_default()+=1;}
            else {incomplete.push(v["name"].as_str().unwrap().to_string());}
            row.push(v);
        }
        for j in 1..4 {
            assert_eq!(source(&parent,arms[0].as_str().unwrap()),
                source(&parent,arms[j].as_str().unwrap()));
        }
        if row[0]["steps"]!=row[1]["steps"] ||
           row[0]["services"]!=row[1]["services"] {
            independent_gain_cases+=1;
        }
        if row[0]["final_signature"]!=row[2]["final_signature"] ||
           row[0]["final_signature"]!=row[3]["final_signature"] ||
           row[0]["services"]!=row[2]["services"] ||
           row[0]["services"]!=row[3]["services"] {
            sample_caused_changed_cases+=1;
        }
        success_records+=row[0]["claimed_success_records"].as_u64().unwrap();
        genuine_restores+=row[0]["effective_pc173_restorations"].as_u64().unwrap();
        records.push(json!({"name":base,"family":case["family"],"arms":row}));
    }
    for c in controls {
        let arm=c["arm"].as_str().unwrap();
        let mode=c["feedback"].as_str().unwrap();
        let goal_case=cases.iter().find(|x|x["name"]==c["case"]).unwrap();
        let goals=goal_case["goals"].as_array().unwrap().iter().map(|x|x.as_i64().unwrap()).collect::<Vec<_>>();
        let v=trial(&parent,c["name"].as_str().unwrap(),arm,mode,&goals,&out_dir);
        charged+=v["steps"].as_u64().unwrap();
        adversarial.push(v);
    }
    let result=json!({
        "study":"CL1B-C16-SINGLE-USE-PAID-SUCCESS-CERTIFICATE",
        "status":"ORIGINAL_NATIVE_PAID_SINGLE_USE_EVIDENCE_NOT_ENDOGENOUS_LEARNING",
        "native_worlds":274,"true_worlds":270,"false_feedback_worlds":4,
        "frozen_new_cases":45,"total_paid_cil":charged,
        "main_actor_paid_success_records":success_records,
        "main_actual_physical_pc173_restores":genuine_restores,
        "same_source_live_vs_suppressed_paid_or_feedback_difference_cases":independent_gain_cases,
        "same_source_live_vs_tampered_state_or_feedback_difference_cases":sample_caused_changed_cases,
        "arm_fees":arm_fees,"arm_completed_worlds":arm_completed,
        "arm_physical_restore_changes":arm_physical,
        "incomplete_true_world_names":incomplete,
        "records":records,"adversarial_records":adversarial
    });
    fs::write(out_dir.join("cl1b_c16_original_native_274_worlds.json"),
        serde_json::to_vec_pretty(&result).unwrap()).unwrap();
    // A zero-evidence result is a legitimate, preregistered negative, not a VM validation failure.
    // C16 must not synthesize PID1 writer provenance or delete such worlds.
    // A zero physical restoration result is likewise NOT_DEMONSTRATED, not a test infrastructure failure.
    println!("C16 ORIGINAL NATIVE worlds=274 paid={} actor_success_records={} physical_source_restores={} same_source_effect_cases={} tampered_effect_cases={} total_arm_fees={:?} true_full_completion={:?} unfinished={} ",
        charged,success_records,genuine_restores,independent_gain_cases,sample_caused_changed_cases,
        result["arm_fees"],result["arm_completed_worlds"],incomplete.len());
}
