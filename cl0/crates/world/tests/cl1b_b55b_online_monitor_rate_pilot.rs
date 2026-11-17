//! B55b-PILOT: paid online monitor-rate revision within original 4k/180 constraints.
//! Researcher-authored periodic policy menu, not autonomous metapolicy creation.
//! Narrow source-identical 5-arm, 2-ecology, 4-seed feasibility control only.
use cl0_world::{EventKind,Intervention,Op,Origin,Process,World};
use serde_json::{json,Value};
use std::fs::{self,File};
use std::io::{BufWriter,Write};
#[path="cl1b_b55a_paid_monitor_choice_preflight.rs"] mod b55a;
const SEEDS:[i64;4]=[101,211,307,401];
const WINDOW:usize=24;
const BUDGET:usize=4000;
const HELD:[i64;3]=[11,-7,0];
const TRAIN:[i64;2]=[5,-3];
fn founder()->Vec<Op>{
    // Researcher-created, identical across arm sources; no host source repair.
    // Reuse exactly 178 source nodes and both native-paid clone slots178-179.
    let mut code=b55a::founder();
    assert_eq!(code.len(),178);
    // B55a's zero-success initial policy was NEVER; the pre-registered
    // pilot minimum-exploration floor now makes it FOURTH.
    code[155]=Op::Const{dst:3,value:160,next:174};
    // Researcher-authoring disclosure: redirect the old paid Read20
    // branch to paid rate-control code. Actual future source edits are
    // CIL-origin PatchConst and PatchNext, never a Rust host decision.
    code[79]=Op::Branch{test:3,nonzero:95,zero:103};
    code[95]=Op::Const{dst:2,value:27,next:96};
    code[96]=Op::Read{dst:1,address:2,next:97};
    code[97]=Op::Branch{test:1,nonzero:30,zero:98};
    code[98]=Op::Const{dst:2,value:29,next:99};
    code[99]=Op::Read{dst:1,address:2,next:100};
    code[100]=Op::Const{dst:3,value:1,next:101};
    code[101]=Op::Add{dst:1,lhs:1,rhs:3,next:102};
    code[102]=Op::Write{address:2,value:1,next:109};
    code[103]=Op::Const{dst:2,value:27,next:104};
    code[104]=Op::Read{dst:1,address:2,next:105};
    code[105]=Op::Branch{test:1,nonzero:80,zero:106};
    code[106]=Op::Const{dst:2,value:29,next:107};
    code[107]=Op::Const{dst:1,value:0,next:108};
    code[108]=Op::Write{address:2,value:1,next:134};
    code[109]=Op::Const{dst:3,value:-3,next:110};
    code[110]=Op::Add{dst:3,lhs:1,rhs:3,next:111};
    code[111]=Op::Branch{test:3,nonzero:30,zero:112};
    code[112]=Op::Const{dst:1,value:0,next:113};
    code[113]=Op::Write{address:2,value:1,next:114};
    code[114]=Op::Const{dst:2,value:76,next:115};
    code[115]=Op::Const{dst:3,value:160,next:116};
    code[116]=Op::PatchConst{node:2,value:3,next:117};
    code[117]=Op::Const{dst:2,value:2,next:118};
    code[118]=Op::Const{dst:3,value:160,next:119};
    code[119]=Op::PatchNext{node:2,target:3,next:30};
    code[134]=Op::Const{dst:2,value:76,next:135};
    code[135]=Op::Const{dst:3,value:77,next:136};
    code[136]=Op::PatchConst{node:2,value:3,next:154};
    code[154]=Op::Const{dst:2,value:2,next:156};
    code[156]=Op::Const{dst:3,value:77,next:158};
    code[158]=Op::PatchNext{node:2,target:3,next:80};
    assert_eq!(code.len(),178);
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
    let mut all_service_correct=0_usize;
    let mut all_service_wrong=0_usize;
    let mut first72_paid=None::<usize>;
    let mut rate_changes=Vec::<Value>::new();
    let mut paid_const_rewrites=0_usize;
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
        let pc_before=w.process(1).unwrap().pc;
        let e=one_step(out,&mut w,id,&mut paid);
        if pc_before==116||pc_before==136{
            paid_const_rewrites+=1;
            assert!([77,160].contains(&code76(&w)));
        }
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
                        if good{all_service_correct+=1;}else{all_service_wrong+=1;}
                        if service<3*WINDOW{
                            if good{correct[phase]+=1;}else{wrong[phase]+=1;}
                            if ecology=="E2"&&phase==1&&good&&first_shift_correct.is_none(){
                                first_shift_correct=Some(paid);
                            }
                        }
                        service+=1;
                        if ecology=="E2"&&service==WINDOW{shift_start_step=paid;}
                        if service==3*WINDOW{first72_paid=Some(paid);}
                        pending_false=!good;
                        external(out,&mut w,id,20,if arm=="C"{1}else{i64::from(good)},
                            if arm=="C"{"service_blind_success"}else{"service_feedback_truth"});
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
            EventKind::ExecutableEdgePatched{node:2,before,after}
                if stage=="SERVICE" && ((before==160&&after==77)||
                                        (before==77&&after==160)) =>{
                // Any B55b policy-rate source change MUST be the direct
                // consequence of paid native VM CIL instructions, not lab.
                // The source edit is checked by the actual value of node76.
                assert_eq!(code76(&w),after as i64);
                chosen=after as i64;
                rate_changes.push(json!({"before":before,"after":after,
                    "paid":paid,"service_writes_before":service,
                    "pc":pc_before}));
            },
            EventKind::CellWritten{key:25,value:1}=>{
                assert_eq!(stage,"SERVICE");
                reopened+=1;training=0;
                stage="REOPEN_TRAIN";
                input=TRAIN[0];
                external(out,&mut w,id,7,input,"native_watchdog_reopens_search");
            },
            EventKind::ExecutableEdgePatched{node:2,before:42,after}
                if stage=="REOPEN_TRAIN" && after as i64==chosen =>{
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
        "service_outputs":service.min(72),"all_service_outputs":service,
        "all_correct":all_service_correct,"all_wrong":all_service_wrong,
        "first72_paid":first72_paid,
        "feedback_reads":monitor_reads,"rate_changes":rate_changes,
        "paid_rate_const_rewrites":paid_const_rewrites,
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
fn b55b_pilot_paid_online_monitor_rate_revision(){
    let output=std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../cl1b-ci-evidence");
    fs::create_dir_all(&output).unwrap();
    let file=output.join("b55b_pilot_native_steps.jsonl");
    let mut out=BufWriter::new(File::create(&file).unwrap());
    let manifest=std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../../cl1b/studies/B55b-pilot-executable-manifest-v1.md");
    let bytes=fs::read(&manifest).unwrap();
    // SHA-256 is computed by the local runner's standard operating system
    // tooling in the lab harness; never a VM or actor source instruction.
    let sha=std::process::Command::new("sha256sum")
        .arg(&manifest).output().unwrap();
    assert!(sha.status.success());
    let line=String::from_utf8(sha.stdout).unwrap();
    let manifest_sha=line.split_whitespace().next().unwrap().to_owned();
    assert_eq!(manifest_sha.len(),64);
    emit(&mut out,json!(["meta","B55b-PILOT",1,
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
        assert_eq!(r["paid"],json!(4000));
        assert!([30,77,160].contains(&(r["monitor_choice"].as_i64().unwrap())));
    }
    eprintln!("B55B_PILOT_NATIVE {}",serde_json::to_string(&results).unwrap());
    eprintln!("B55B_PILOT_TRACE {}",file.display());
}
