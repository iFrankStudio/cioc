//! CL-1B B52: researcher-authored watchdog, enacted by one paid native PID1.
//! B51c's FIRST 72 executable nodes are source-identical; the ONLY bootstrap
//! source change is a native-paid PatchNext of node49's edge to an appended
//! watchdog-target bridge. No host alters executable code or replaces PID1.
//! SUCCESS within this grammar is NOT self-invention, B3/B4 or cyberlife.
use cl0_world::{EventKind,Intervention,Op,Origin,Process,World};
use serde_json::{json,Value};
use std::fs::{self,File};
use std::io::{BufWriter,Write};

#[path="cl1b_b51c_paid_search_retention.rs"]
mod b51c;

const BUDGET:usize=3000;
const TRAIN:[i64;2]=[5,-3];
const HELD:[i64;3]=[11,-7,0];
const WINDOW:usize=24;

#[derive(Clone,Copy,Debug,PartialEq,Eq)]
enum Arm{True,StaleSuccess,ForcedZero,StationaryTrue,StaleDelayed}
impl Arm{
    fn name(self)->&'static str{
        match self{
            Self::True=>"TRUE",Self::StaleSuccess=>"STALE_SUCCESS",
            Self::ForcedZero=>"FORCED_ZERO",Self::StationaryTrue=>"STATIONARY_TRUE",
            Self::StaleDelayed=>"STALE_DELAYED",
        }
    }
}

pub(crate) fn founder()->Vec<Op>{
    // Important: retain all 72 B51c ops exactly; no switchable correct
    // task program exists at initial startup.
    let mut code=b51c::founder();
    assert_eq!(code.len(),72);
    code.extend([
        // Paid bootstrap: redirect B51c node49.next (not its const value)
        // to node76; when B51c subsequently accepts a candidate, node76
        // overwrites r3 to point at service watchdog 77 before PatchNext50.
        Op::Const{dst:2,value:49,next:73},       // 72
        Op::Const{dst:3,value:76,next:74},       // 73
        Op::PatchNext{node:2,target:3,next:75},  // 74
        Op::Const{dst:2,value:0,next:3},         // 75
        Op::Const{dst:3,value:77,next:50},       // 76
        // Every retained service output MUST traverse PAID feedback Read.
        Op::Const{dst:2,value:20,next:78},       // 77
        Op::Read{dst:3,address:2,next:79},       // 78
        Op::Branch{test:3,nonzero:30,zero:80},   // 79
        // A *paid native marker Write* requests the externally frozen train
        // probes. It does not disclose task function or inject executable code.
        Op::Const{dst:2,value:25,next:81},       // 80
        Op::Const{dst:1,value:1,next:82},        // 81
        Op::Write{address:2,value:1,next:83},    // 82
        // Native source reinitializes its OWN search-state data cells.
        Op::Const{dst:1,value:-3,next:84},       // 83
        Op::Const{dst:2,value:21,next:85},       // 84
        Op::Write{address:2,value:1,next:86},    // 85
        Op::Const{dst:1,value:0,next:87},        // 86
        Op::Const{dst:2,value:22,next:88},       // 87
        Op::Write{address:2,value:1,next:89},    // 88
        // Return the source writer2 to B51c's first-feedback-probe path,
        // then use the *same* original candidate routing program.
        Op::Const{dst:2,value:2,next:90},        // 89
        Op::Const{dst:3,value:35,next:91},       // 90
        Op::PatchNext{node:2,target:3,next:21},  // 91
    ]);
    assert_eq!(code.len(),92);
    code
}
fn edge(op:&Op)->Option<usize>{
    match op{
        Op::Const{next,..}|Op::Add{next,..}|Op::Sub{next,..}
        |Op::Read{next,..}|Op::Write{next,..}
        |Op::PatchNext{next,..}|Op::CloneNode{next,..}=>Some(*next),
        _=>None
    }
}
fn kind(e:&EventKind)->Value{
    match e{
        EventKind::Executed=>json!({"type":"executed"}),
        EventKind::CellRead{key,value}=>json!({"type":"read","key":key,"value":value}),
        EventKind::CellWritten{key,value}=>json!({"type":"write","key":key,"value":value}),
        EventKind::ExecutableEdgePatched{node,before,after}=>json!({
            "type":"patch","node":node,"before":before,"after":after}),
        EventKind::Halted=>json!({"type":"halt"}),
        EventKind::Trapped(msg)=>json!({"type":"trap","message":msg}),
        _=>panic!("Unexpected B52 native event {e:?}")
    }
}
fn cells(w:&World)->Value{
    json!([w.cell(7),w.cell(11),w.cell(20),w.cell(21),
        w.cell(22),w.cell(24),w.cell(25),w.cell(30),w.cell(31)])
}
fn emit(out:&mut BufWriter<File>,v:Value){
    serde_json::to_writer(&mut *out,&v).unwrap();
    out.write_all(b"\n").unwrap();
}
fn external(out:&mut BufWriter<File>,w:&mut World,case:usize,
            key:i64,value:i64,reason:&str){
    let before=w.total_compute_inventory().unwrap();
    w.intervene(Intervention::SetCell{key,value}).unwrap();
    let e=w.events().last().unwrap();
    assert_eq!(e.origin,Origin::External);
    assert_eq!(w.total_compute_inventory(),Some(before));
    emit(out,json!({"type":"external","case":case,"tick":e.tick,
        "key":key,"value":value,"reason":reason,
        "credits":w.remaining_credits(),
        "storage":w.occupied_units(),"cells":cells(w)}));
}
fn mode(a:(i64,i64),b:(i64,i64),arm:Arm,stage:usize)->(i64,i64){
    if stage==0 || arm==Arm::StationaryTrue || stage==2{a}else{b}
}
fn step(out:&mut BufWriter<File>,w:&mut World,case:usize,paid:&mut usize,
        peak:&mut usize)->EventKind{
    let before=w.total_compute_inventory().unwrap();
    let n=w.process(1).unwrap().code.nodes.len();
    assert!(w.step());
    *paid+=1;
    let e=w.events().last().unwrap().clone();
    assert_eq!(e.origin,Origin::Internal(1));
    assert_eq!(w.total_compute_inventory(),Some(before-1));
    assert!(!e.pool_paid_attempt);
    let p=w.process(1).unwrap();
    assert_eq!(p.credits,e.credits_after);
    *peak=(*peak).max(w.occupied_units());
    assert!(*peak<=256 && p.code.nodes.len()<=160);
    emit(out,json!({"type":"step","case":case,
        "step":*paid,"tick":e.tick,"pc":e.pc,
        "pc_after":p.pc,"event":kind(&e.kind),
        "regs":p.registers,"credits":p.credits,
        "stopped":p.stopped,"code_len":p.code.nodes.len(),
        "growth":p.code.nodes.len()-n,
        "edge2":edge(&p.code.nodes[2]),
        "edge33":edge(&p.code.nodes[33]),
        "edge49":edge(&p.code.nodes[49]),
        "edge72":p.code.nodes.get(92).and_then(edge),
        "edge73":p.code.nodes.get(93).and_then(edge),
        "cells":cells(w),"storage":w.occupied_units()}));
    e.kind
}

fn case(out:&mut BufWriter<File>,id:usize,tag:&str,
        a:(i64,i64),b:(i64,i64),arm:Arm,phase:usize)->Value{
    let code=founder();
    let mut p=Process::new(1,code,BUDGET as u64);
    p.pc=72;
    let mut w=World::with_strict_capacity(16,160,256)
        .with_native_executable_edit_trace();
    w.load(p).unwrap();
    emit(out,json!({"type":"case","case":id,"transition":tag,
        "from":[a.0,a.1],"to":[b.0,b.1],"arm":arm.name(),
        "phase":phase,"initial_len":92,"initial_credits":BUDGET,
        "initial_storage":w.occupied_units()}));
    let mut paid=0_usize;
    let mut peak=w.occupied_units();
    if phase==1{
        // REAL paid first initializer, no task input observed yet.
        let first=step(out,&mut w,id,&mut paid,&mut peak);
        assert!(matches!(first,EventKind::Executed));
        assert_eq!(w.process(1).unwrap().pc,73);
    }
    let mut input=TRAIN[0];
    external(out,&mut w,id,7,input,"initial_training");
    let mut trial=true;
    let mut trial_outputs=0_usize;
    let mut retained=0_usize;
    let mut reopened=0_usize;
    let mut choices:Vec<[i64;2]>=Vec::new();
    let mut service_total=0_usize;
    let mut correct=[0_usize;3];
    let mut wrong=[0_usize;3];
    let mut service_feedback_reads=0_usize;
    let mut post_retention_patches=0_usize;
    let mut prev_true_feedback=1_i64; // success bit from accepted training
    let mut stale_false_successes=0_usize;
    let mut native_bootstrap_patches=0_usize;
    let mut all_feedback_reads=0_usize;
    let mut training_outputs_total=0_usize;
    let mut training_correct=0_usize;
    let mut training_wrong=0_usize;
    while paid<BUDGET && !w.process(1).unwrap().stopped{
        let event=step(out,&mut w,id,&mut paid,&mut peak);
        if let EventKind::CellRead{key:20,..}=event{all_feedback_reads+=1;}
        if let EventKind::ExecutableEdgePatched{node:49,before:50,after:76}=event{
            native_bootstrap_patches+=1;
        }
        if retained>0 && matches!(event,EventKind::ExecutableEdgePatched{..}){
            post_retention_patches+=1;
        }
        match event{
            EventKind::CellWritten{key:11,value}=>{
                let stage=(service_total/WINDOW).min(2);
                let task=mode(a,b,arm,stage);
                let good=value==task.0*input+task.1;
                if trial {
                    trial_outputs+=1;
                    training_outputs_total+=1;
                    if good{training_correct+=1;}else{training_wrong+=1;}
                    external(out,&mut w,id,20,i64::from(good),"trial_feedback");
                    input=TRAIN[trial_outputs%TRAIN.len()];
                    external(out,&mut w,id,7,input,"training_next");
                }else{
                    if service_total<3*WINDOW{
                        if good{correct[stage]+=1;}else{wrong[stage]+=1;}
                        service_total+=1;
                    }
                    let f=match arm{
                        Arm::True|Arm::StationaryTrue=>i64::from(good),
                        Arm::ForcedZero=>0,
                        Arm::StaleSuccess=>1,
                        Arm::StaleDelayed=>prev_true_feedback,
                    };
                    if arm!=Arm::StaleSuccess{
                        external(out,&mut w,id,20,f,"service_feedback");
                    }
                    if f==1 && !good {stale_false_successes+=1;}
                    prev_true_feedback=i64::from(good);
                    // The lab never sets search/sign/offset or source;
                    // it only supplies the next predetermined input.
                    input=HELD[((service_total%WINDOW)+phase)%HELD.len()];
                    external(out,&mut w,id,7,input,"service_next");
                }
            },
            EventKind::ExecutableEdgePatched{node:2,before:42,after:77}=>{
                assert!(trial && trial_outputs>=2 && trial_outputs%2==0);
                trial=false;
                retained+=1;
                choices.push([w.cell(22).unwrap(),w.cell(21).unwrap()]);
                input=HELD[phase];
                external(out,&mut w,id,7,input,"retained_start");
                prev_true_feedback=1;
            },
            EventKind::CellWritten{key:25,value:1}=>{
                assert!(!trial && retained>0,
                    "actor may request fresh trials only after genuine retention");
                trial=true;
                reopened+=1;
                trial_outputs=0;
                input=TRAIN[0];
                external(out,&mut w,id,7,input,"native_reopen_request");
            },
            EventKind::CellRead{key:20,..} if !trial => {
                service_feedback_reads+=1;
            },
            _=>{}
        }
    }
    assert_eq!(paid,BUDGET,"both alive and blind arms spend full paid budget");
    assert_eq!(native_bootstrap_patches,1);
    let p=w.process(1).unwrap();
    let result=json!({"type":"end","case":id,"transition":tag,
        "from":[a.0,a.1],"to":[b.0,b.1],"arm":arm.name(),"phase":phase,
        "paid":paid,"stopped":p.stopped,"credits":p.credits,
        "retentions":retained,"reopens":reopened,"choices":choices,
        "service_writes":service_total,"window_correct":correct,
        "window_wrong":wrong,
        "service_feedback_reads":service_feedback_reads,
        "all_feedback_reads":all_feedback_reads,
        "post_retention_patches":post_retention_patches,
        "stale_false_successes":stale_false_successes,
        "training_outputs":training_outputs_total,
        "training_correct":training_correct,"training_wrong":training_wrong,
        "bootstrapped_edges":native_bootstrap_patches,
        "peak_storage":peak,"final_pc":p.pc,"final_len":p.code.nodes.len(),
        "final_regs":p.registers,"final_cells":cells(&w),
        "final_edge2":edge(&p.code.nodes[2]),
        "final_edge49":edge(&p.code.nodes[49])});
    emit(out,result.clone());
    result
}
#[test]
fn b52_native_paid_revalidation_and_reopenable_search(){
    let folder=std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../cl1b-ci-evidence");
    fs::create_dir_all(&folder).unwrap();
    let path=folder.join("b52_native_steps.jsonl");
    let mut out=BufWriter::new(File::create(&path).unwrap());
    emit(&mut out,json!({"type":"meta","study":"B52","version":1,
        "source_sha":std::env::var("GITHUB_SHA")
            .unwrap_or_else(|_|"local".into()),
        "base_founder":"cl1b_b51c_paid_search_retention.rs"}));
    let arms=[Arm::True,Arm::StaleSuccess,Arm::ForcedZero,
        Arm::StationaryTrue,Arm::StaleDelayed];
    let transitions=[
        ("A",(1_i64,-2_i64),(-1_i64,3_i64)),
        ("B",(-1_i64,-3_i64),(1_i64,2_i64)),
    ];
    let mut rows=Vec::new();
    for (tag,a,b) in transitions{
        for phase in [0,1]{
            for arm in arms{
                let id=rows.len();
                rows.push(case(&mut out,id,tag,a,b,arm,phase));
            }
        }
    }
    out.flush().unwrap();
    assert_eq!(rows.len(),20);
    for row in &rows{
        assert_eq!(row["paid"],json!(3000));
        assert!(row["peak_storage"].as_u64().unwrap()<=256);
        assert_eq!(row["bootstrapped_edges"],json!(1));
    }
    eprintln!("B52_NATIVE_OUTCOMES {}",serde_json::to_string(&rows).unwrap());
    eprintln!("B52_NATIVE trace={}",path.display());
}
