//! B55a-PREFLIGHT: actual paid Rust CIL metapolicy edits, NOT the full frozen
//! B55a 7-arm x 3-ecology x 8-seed experiment. The nonce-authenticated E3
//! and G controls are deliberately NOT represented by this narrower pilot.
//! The 3 monitor routes are researcher-authored, never self-invented; B3/B4 fail.
//! Source identical across pilot arms; fixed arm uses explicitly external cell27.
use cl0_world::{EventKind,Intervention,Op,Origin,Process,World};
use serde_json::{json,Value};
use std::fs::{self,File};
use std::io::{BufWriter,Write};
#[path="cl1b_b52_native_paid_revalidation.rs"] mod b52;
const SEEDS:[i64;4]=[101,211,307,401];
const WINDOW:usize=24;
const BUDGET:usize=4000;
const HELD:[i64;3]=[11,-7,0];
const TRAIN:[i64;2]=[5,-3];
fn founder()->Vec<Op>{
    let mut code=b52::founder();
    assert_eq!(code.len(),92);
    // Precreated dead padding is a costly researcher-authored structure;
    // only native paid PatchConst / PatchNext makes the trial/policy live.
    code.resize(178,Op::Halt);
    code[92]=Op::Const{dst:2,value:76,next:93};
    code[93]=Op::Const{dst:3,value:120,next:94};
    code[94]=Op::PatchConst{node:2,value:3,next:72};
    code[120]=Op::Const{dst:2,value:23,next:121};
    code[121]=Op::Read{dst:1,address:2,next:122};
    code[122]=Op::Const{dst:3,value:1,next:123};
    code[123]=Op::Add{dst:1,lhs:1,rhs:3,next:124};
    code[124]=Op::Write{address:2,value:1,next:125};
    code[125]=Op::Const{dst:3,value:-4,next:126};
    code[126]=Op::Add{dst:3,lhs:1,rhs:3,next:127};
    code[127]=Op::Branch{test:3,nonzero:128,zero:145};
    code[128]=Op::Const{dst:3,value:-1,next:129};
    code[129]=Op::Add{dst:3,lhs:1,rhs:3,next:130};
    code[130]=Op::Branch{test:3,nonzero:131,zero:137};
    code[131]=Op::Const{dst:3,value:-2,next:132};
    code[132]=Op::Add{dst:3,lhs:1,rhs:3,next:133};
    code[133]=Op::Branch{test:3,nonzero:30,zero:137};
    code[137]=Op::Const{dst:2,value:20,next:138};
    code[138]=Op::Read{dst:3,address:2,next:139};
    code[139]=Op::Branch{test:3,nonzero:30,zero:140};
    code[140]=Op::Const{dst:2,value:26,next:141};
    code[141]=Op::Read{dst:1,address:2,next:142};
    code[142]=Op::Const{dst:3,value:1,next:143};
    code[143]=Op::Add{dst:1,lhs:1,rhs:3,next:144};
    code[144]=Op::Write{address:2,value:1,next:30};
    code[145]=Op::Const{dst:2,value:27,next:146};
    code[146]=Op::Read{dst:3,address:2,next:147};
    code[147]=Op::Branch{test:3,nonzero:174,zero:148};
    code[148]=Op::Const{dst:2,value:26,next:149};
    code[149]=Op::Read{dst:1,address:2,next:150};
    code[150]=Op::Branch{test:1,nonzero:151,zero:155};
    code[151]=Op::Const{dst:3,value:-2,next:152};
    code[152]=Op::Add{dst:3,lhs:1,rhs:3,next:153};
    code[153]=Op::Branch{test:3,nonzero:157,zero:159};
    code[155]=Op::Const{dst:3,value:30,next:174};
    code[157]=Op::Const{dst:3,value:160,next:174};
    code[159]=Op::Const{dst:3,value:77,next:174};
    code[160]=Op::Const{dst:2,value:28,next:161};
    code[161]=Op::Read{dst:1,address:2,next:162};
    code[162]=Op::Const{dst:3,value:1,next:163};
    code[163]=Op::Add{dst:1,lhs:1,rhs:3,next:164};
    code[164]=Op::Write{address:2,value:1,next:165};
    code[165]=Op::Const{dst:3,value:-4,next:166};
    code[166]=Op::Add{dst:3,lhs:1,rhs:3,next:167};
    code[167]=Op::Branch{test:3,nonzero:30,zero:168};
    code[168]=Op::Const{dst:1,value:0,next:169};
    code[169]=Op::Write{address:2,value:1,next:77};
    code[174]=Op::Const{dst:2,value:76,next:175};
    code[175]=Op::PatchConst{node:2,value:3,next:176};
    code[176]=Op::Const{dst:2,value:2,next:177};
    code[177]=Op::PatchNext{node:2,target:3,next:30};
    code
}
fn edge(op:&Op)->Option<usize>{
    match op{
        Op::Const{next,..}|Op::Add{next,..}|Op::Sub{next,..}
        |Op::Read{next,..}|Op::Write{next,..}
        |Op::PatchNext{next,..}|Op::PatchConst{next,..}
        |Op::CloneNode{next,..}=>Some(*next),
        _=>None
    }
}
fn code76(w:&World)->i64{
    match &w.process(1).unwrap().code.nodes[76]{
        Op::Const{value,..}=>*value,
        x=>panic!("B55 trial source node76 drift {x:?}"),
    }
}
fn cells(w:&World)->Value{
    json!([w.cell(7),w.cell(11),w.cell(20),w.cell(21),w.cell(22),
        w.cell(23),w.cell(24),w.cell(25),w.cell(26),w.cell(27),w.cell(28),
        w.cell(30),w.cell(31)])
}
fn event_kind(e:&EventKind)->Value{
    match e{
        EventKind::Executed=>json!(["exec"]),
        EventKind::CellRead{key,value}=>json!(["read",key,value]),
        EventKind::CellWritten{key,value}=>json!(["write",key,value]),
        EventKind::ExecutableEdgePatched{node,before,after}=>json!(["patch",node,before,after]),
        EventKind::NativeCodeEditSuppressed{opcode_tag}=>json!(["suppressed",opcode_tag]),
        EventKind::Trapped(msg)=>json!(["trap",msg]),
        EventKind::Halted=>json!(["halt"]),
        _=>panic!("Unexpected B55a pilot native event: {e:?}"),
    }
}
fn emit(out:&mut BufWriter<File>,v:Value){
    serde_json::to_writer(&mut *out,&v).unwrap();
    out.write_all(b"\n").unwrap();
}
fn external(out:&mut BufWriter<File>,w:&mut World,id:usize,key:i64,val:i64,why:&str){
    let c=w.total_compute_inventory();
    w.intervene(Intervention::SetCell{key,value:val}).unwrap();
    assert_eq!(w.total_compute_inventory(),c);
    let e=w.events().last().unwrap();
    assert_eq!(e.origin,Origin::External);
    emit(out,json!(["ext",id,e.tick,key,val,why,w.remaining_credits(),
        w.occupied_units(),cells(w)]));
}
fn one_step(out:&mut BufWriter<File>,w:&mut World,id:usize,paid:&mut usize)->EventKind{
    let c=w.total_compute_inventory().unwrap();
    assert!(w.step());
    *paid+=1;
    assert_eq!(w.total_compute_inventory(),Some(c-1));
    let e=w.events().last().unwrap().clone();
    assert_eq!(e.origin,Origin::Internal(1));
    let p=w.process(1).unwrap();
    assert_eq!(p.credits,e.credits_after);
    assert!(p.code.nodes.len()<=180&&w.occupied_units()<=300);
    emit(out,json!(["step",id,*paid,e.tick,e.pc,p.pc,
        event_kind(&e.kind),p.registers,p.credits,p.stopped,
        p.code.nodes.len(),edge(&p.code.nodes[2]),code76(w),
        w.occupied_units(),cells(w)]));
    e.kind
}
fn task(base:(i64,i64),next:(i64,i64),ecology:&str,stage:usize)->(i64,i64){
    if ecology=="E2"&&stage==1{next}else{base}
}
fn pilot(out:&mut BufWriter<File>,id:usize,seed:i64,arm:&str,ecology:&str)->Value{
    let idx=SEEDS.iter().position(|x|*x==seed).unwrap();
    let (base,shift,tag)=if idx%2==0{
        ((1_i64,-2_i64),(-1_i64,3_i64),"A")
    }else{((-1_i64,-3_i64),(1_i64,2_i64),"B")};
    let fixed=match arm{
        "A"|"C"=>0_i64,
        "B"=>77,
        "D"=>30,
        "E"=>160,
        _=>panic!("pilot arm not registered {arm}"),
    };
    let mut p=Process::new(1,founder(),BUDGET as u64);
    p.pc=92;
    let mut w=World::with_strict_capacity(16,180,300)
        .with_native_executable_edit_trace();
    w.load(p).unwrap();
    emit(out,json!(["case",id,seed,arm,ecology,tag,base,shift,
        BUDGET,178,w.occupied_units()]));
    let mut paid=0_usize;
    external(out,&mut w,id,27,fixed,"fixed_arm_value_0_means_actor_choice");
    let mut input=TRAIN[0];
    external(out,&mut w,id,7,input,"start_training");
    let mut stage="INITIAL_TRAIN";
    let mut training=0_usize;
    let mut trial=0_usize;
    let mut service=0_usize;
    let mut reopened=0_usize;
    let mut retentions=0_usize;
    let mut trial_observations=0_usize;
    let mut monitor_reads=0_usize;
    let mut false_trials=0_usize;
    let mut edits=0_usize;
    let mut chosen=0_i64;
    let mut correct=[0_usize;3];
    let mut wrong=[0_usize;3];
    let mut first_recovery_paid=Vec::<usize>::new();
    let mut shift_start_step=0_usize;
    let mut pending_false=false;
    let mut first_shift_correct=None::<usize>;
    while paid<BUDGET&&!w.process(1).unwrap().stopped{
        let e=one_step(out,&mut w,id,&mut paid);
        if matches!(e,EventKind::ExecutableEdgePatched{..}){edits+=1;}
        match e{
            EventKind::CellWritten{key:11,value}=>{
                let phase=(service/WINDOW).min(2);
                let (s,k)=task(base,shift,ecology,phase);
                let good=value==s*input+k;
                match stage{
                    "INITIAL_TRAIN"|"REOPEN_TRAIN"=>{
                        training+=1;
                        external(out,&mut w,id,20,i64::from(good),"task_training_truth");
                        input=TRAIN[training%2];
                        external(out,&mut w,id,7,input,"next_training_probe");
                    },
                    "TRIAL"=>{
                        trial+=1;
                        // All initial policy trials have identical frozen
                        // previous-task context; no hidden future mode marker.
                        let bit=if arm=="C"{1}else{i64::from(good)};
                        if !good{false_trials+=1;}
                        external(out,&mut w,id,20,bit,"trial_actor_visible_feedback");
                        input=TRAIN[trial%2];
                        external(out,&mut w,id,7,input,"next_trial_probe");
                    },
                    "SERVICE"=>{
                        if service<3*WINDOW{
                            if good{correct[phase]+=1;}else{wrong[phase]+=1;}
                            service+=1;
                            if ecology=="E2"&&service==WINDOW{
                                shift_start_step=paid;
                            }
                            if ecology=="E2"&&phase==1&&good&&first_shift_correct.is_none(){
                                first_shift_correct=Some(paid);
                            }
                        }
                        pending_false=!good;
                        external(out,&mut w,id,20,i64::from(good),"service_feedback_truth");
                        input=HELD[((service%WINDOW)+idx%3)%HELD.len()];
                        external(out,&mut w,id,7,input,"next_heldout_input");
                    },
                    _=>panic!("unexpected stage {stage}"),
                }
            },
            EventKind::ExecutableEdgePatched{node:2,before:42,after:120}=>{
                assert_eq!(stage,"INITIAL_TRAIN");
                retentions+=1;stage="TRIAL";
                input=TRAIN[0];
                external(out,&mut w,id,7,input,"post_retention_trial_start");
            },
            EventKind::ExecutableEdgePatched{node:2,before:120,after}=>{
                assert_eq!(stage,"TRIAL");
                assert_eq!(trial,4,"4 real trial writes before actor policy patch");
                assert!([30,77,160].contains(&after));
                chosen=after as i64;
                retentions+=1;
                stage="SERVICE";
                input=HELD[idx%3];
                external(out,&mut w,id,7,input,"actor_source_choice_enters_service");
            },
            EventKind::CellWritten{key:25,value:1}=>{
                assert_eq!(stage,"SERVICE");
                reopened+=1;training=0;
                stage="REOPEN_TRAIN";
                input=TRAIN[0];
                external(out,&mut w,id,7,input,"native_watchdog_reopens_search");
            },
            EventKind::ExecutableEdgePatched{node:2,before:42,after}=>{
                assert_eq!(stage,"REOPEN_TRAIN");
                assert_eq!(after as i64,chosen,
                    "same paid source retains actor-selected monitor");
                retentions+=1;
                stage="SERVICE";
                input=HELD[((service%WINDOW)+idx%3)%HELD.len()];
                external(out,&mut w,id,7,input,"repaired_source_returns_service");
                first_recovery_paid.push(paid);
            },
            EventKind::CellRead{key:20,..} if stage=="TRIAL"=>{
                trial_observations+=1;
            },
            EventKind::CellRead{key:20,..} if stage=="SERVICE"=>{
                monitor_reads+=1;
            },
            _=>{},
        }
    }
    let missing=3*WINDOW-service.min(3*WINDOW);
    let total_c=correct.iter().sum::<usize>();
    let total_w=wrong.iter().sum::<usize>();
    assert_eq!(total_c+total_w+missing,72);
    let u=(total_c as f64-2.0*total_w as f64-missing as f64)/(paid.max(1) as f64);
    let proc=w.process(1).unwrap();
    let outrow=json!({"id":id,"seed":seed,"ecology":ecology,"arm":arm,
        "task":tag,"paid":paid,"stopped":proc.stopped,
        "trial_outputs":trial,"trial_observations":trial_observations,
        "trial_wrong":false_trials,"monitor_choice":chosen,
        "correct_by_stage":correct,"wrong_by_stage":wrong,
        "correct":total_c,"wrong":total_w,"missing":missing,"utility":u,
        "service_outputs":service,"feedback_reads":monitor_reads,
        "retentions":retentions,"reopens":reopened,"actor_patch_events":edits,
        "program_len":proc.code.nodes.len(),"storage":w.occupied_units(),
        "shift_start_paid":shift_start_step,
        "first_shift_correct_paid":first_shift_correct,
        "recovery_patches_paid":first_recovery_paid,
        "initial_cap":[16,180,300],"initial_budget":BUDGET,
        "final_cells":cells(&w)});
    emit(out,json!(["end",id,outrow]));
    outrow
}
#[test]
fn b55a_preflight_paid_monitor_policy_choice(){
    let output=std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../cl1b-ci-evidence");
    fs::create_dir_all(&output).unwrap();
    let file=output.join("b55a_preflight_native_steps.jsonl");
    let mut out=BufWriter::new(File::create(&file).unwrap());
    let manifest=std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../../cl1b/studies/B55a-executable-resource-manifest-v1.md");
    let bytes=fs::read(&manifest).unwrap();
    // SHA-256 is computed by the local runner's standard operating system
    // tooling in the lab harness; never a VM or actor source instruction.
    let sha=std::process::Command::new("sha256sum")
        .arg(&manifest).output().unwrap();
    assert!(sha.status.success());
    let line=String::from_utf8(sha.stdout).unwrap();
    let manifest_sha=line.split_whitespace().next().unwrap().to_owned();
    assert_eq!(manifest_sha.len(),64);
    emit(&mut out,json!(["meta","B55a-PREFLIGHT",1,
        std::env::var("GITHUB_SHA").unwrap_or_else(|_|"local".to_owned()),
        manifest_sha,bytes.len(),178,4000]));
    let mut results=Vec::new();
    for ecology in ["E1","E2"]{
        for seed in SEEDS{
            for arm in ["A","B","C","D","E"]{
                let id=results.len();
                results.push(pilot(&mut out,id,seed,arm,ecology));
            }
        }
    }
    out.flush().unwrap();
    assert_eq!(results.len(),40);
    for r in &results{
        assert!(r["program_len"].as_u64().unwrap()<=180);
        assert!(r["storage"].as_u64().unwrap()<=300);
        assert!(r["paid"].as_u64().unwrap()<=4000);
        assert_eq!(r["trial_outputs"],json!(4));
        assert!([30,77,160].contains(&(r["monitor_choice"].as_i64().unwrap())));
    }
    eprintln!("B55A_PREFLIGHT_NATIVE {}",serde_json::to_string(&results).unwrap());
    eprintln!("B55A_PREFLIGHT_TRACE {}",file.display());
}
