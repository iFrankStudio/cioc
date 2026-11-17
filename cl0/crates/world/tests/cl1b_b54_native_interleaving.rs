//! CL-1B B54: actual native CIL response verification time-of-check/time-of-use.
//! The B53 124-node source is reused without editing a single CIL node;
//! only the experimenter schedules ordinary external cell writes BETWEEN
//! PAID native instructions. Not a truth oracle, B3/B4 or cyberlife.
use cl0_world::{EventKind,Intervention,Op,Origin,Process,World};
use serde_json::{json,Value};
use std::fs::{self,File};
use std::io::{BufWriter,Write};

#[path="cl1b_b53_native_response_correlation.rs"]
mod b53;

const BUDGET:usize=4000;
const WINDOW:usize=24;
const TRAIN:[i64;2]=[5,-3];
const HELD:[i64;3]=[11,-7,0];

#[derive(Clone,Copy,Debug,PartialEq,Eq)]
enum Arm{FreshAtomic,LateBitFlip,LateCommitFlip,StableForgedCurrent,OldNonceReplay}
impl Arm{
    fn tag(self)->&'static str{
        match self{
            Self::FreshAtomic=>"FRESH_ATOMIC",
            Self::LateBitFlip=>"LATE_BIT_FLIP",
            Self::LateCommitFlip=>"LATE_COMMIT_FLIP",
            Self::StableForgedCurrent=>"STABLE_FORGED_CURRENT",
            Self::OldNonceReplay=>"OLD_NONCE_REPLAY",
        }
    }
}
fn founder()->Vec<Op>{b53::founder()}
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
        EventKind::ExecutableEdgePatched{node,before,after}=>
            json!({"type":"patch","node":node,"before":before,"after":after}),
        EventKind::Halted=>json!({"type":"halt"}),
        EventKind::Trapped(msg)=>json!({"type":"trap","message":msg}),
        _=>panic!("B54 unexpected paid CIL event {e:?}"),
    }
}
fn state(w:&World)->Value{
    json!([w.cell(7),w.cell(11),w.cell(20),w.cell(21),w.cell(22),
        w.cell(24),w.cell(25),w.cell(26),w.cell(27),w.cell(28),
        w.cell(29),w.cell(30),w.cell(31)])
}
fn emit(out:&mut BufWriter<File>,v:Value){
    serde_json::to_writer(&mut *out,&v).unwrap();
    out.write_all(b"\n").unwrap();
}
fn external(out:&mut BufWriter<File>,w:&mut World,id:usize,
            key:i64,value:i64,reason:&str){
    let credits_before=w.total_compute_inventory().unwrap();
    w.intervene(Intervention::SetCell{key,value}).unwrap();
    let e=w.events().last().unwrap();
    assert_eq!(e.origin,Origin::External);
    assert_eq!(w.total_compute_inventory(),Some(credits_before));
    emit(out,json!({"type":"external","case":id,"reason":reason,
        "key":key,"value":value,"tick":e.tick,
        "credits":w.remaining_credits(),"storage":w.occupied_units(),
        "cells":state(w)}));
}
fn step(out:&mut BufWriter<File>,w:&mut World,id:usize,
        paid:&mut usize,peak:&mut usize)->EventKind{
    let credits_before=w.total_compute_inventory().unwrap();
    let old_len=w.process(1).unwrap().code.nodes.len();
    assert!(w.step());
    *paid+=1;
    let e=w.events().last().unwrap().clone();
    assert_eq!(e.origin,Origin::Internal(1));
    assert!(!e.pool_paid_attempt);
    assert_eq!(w.total_compute_inventory(),Some(credits_before-1));
    let p=w.process(1).unwrap();
    assert_eq!(p.credits,e.credits_after);
    *peak=(*peak).max(w.occupied_units());
    assert!(*peak<=300&&p.code.nodes.len()<=180);
    emit(out,json!({"type":"step","case":id,"step":*paid,
        "tick":e.tick,"pc":e.pc,"pc_after":p.pc,
        "event":kind(&e.kind),"regs":p.registers,"credits":p.credits,
        "stopped":p.stopped,"code_len":p.code.nodes.len(),
        "growth":p.code.nodes.len()-old_len,
        "edge2":edge(&p.code.nodes[2]),"edge34":edge(&p.code.nodes[34]),
        "edge49":edge(&p.code.nodes[49]),"edge76":edge(&p.code.nodes[76]),
        "edge124":p.code.nodes.get(124).and_then(edge),
        "edge125":p.code.nodes.get(125).and_then(edge),
        "storage":w.occupied_units(),"cells":state(w)}));
    e.kind
}
fn current_task(a:(i64,i64),b:(i64,i64),stage:usize)->(i64,i64){
    if stage==1 {b}else{a}
}
fn packet(out:&mut BufWriter<File>,w:&mut World,id:usize,
    bit:i64,echo:i64,commit:i64,reason:&str){
    external(out,w,id,20,bit,&format!("{reason}_bit"));
    external(out,w,id,27,echo,&format!("{reason}_echo"));
    external(out,w,id,28,commit,&format!("{reason}_commit"));
}
fn run_case(out:&mut BufWriter<File>,id:usize,transition:&str,
    a:(i64,i64),b:(i64,i64),arm:Arm,phase:usize)->Value{
    let code=founder();
    let mut proc=Process::new(1,code,BUDGET as u64);
    proc.pc=92;
    let mut w=World::with_strict_capacity(16,180,300)
        .with_native_executable_edit_trace();
    w.load(proc).unwrap();
    emit(out,json!({"type":"case","case":id,"transition":transition,
        "from":[a.0,a.1],"to":[b.0,b.1],"arm":arm.tag(),
        "phase":phase,"initial_len":124,"initial_credits":BUDGET,
        "initial_storage":w.occupied_units()}));
    let mut paid=0_usize;
    let mut peak=w.occupied_units();
    if phase==1{
        let first=step(out,&mut w,id,&mut paid,&mut peak);
        assert!(matches!(first,EventKind::Executed));
        assert_eq!(w.process(1).unwrap().pc,93);
    }
    let mut input=TRAIN[0];
    external(out,&mut w,id,7,input,"initial_training");
    let mut trial=true;
    let mut trial_outputs=0_usize;
    let mut retained=0_usize;
    let mut reopened=0_usize;
    let mut choices=Vec::<[i64;2]>::new();
    let mut service_count=0_usize;
    let mut correct=[0_usize;3];
    let mut wrong=[0_usize;3];
    let mut rejected=0_usize;
    let mut post_retention_feedback_reads=0_usize;
    let mut response_echo_reads=0_usize;
    let mut response_commit_reads=0_usize;
    let mut false_accepted_success=0_usize;
    let mut serial_writes=0_usize;
    let mut last_serial=0_i64;
    let mut packet_before=(0_i64,0_i64);
    let mut replay_success=(0_i64,1_i64);
    let mut previous_submit_truth=true;
    let mut post_retention_patches=0_usize;
    let mut bootstrap_patches=0_usize;
    let mut training_outputs_total=0_usize;
    let mut training_wrong=0_usize;
    let mut service_feedback_packets=0_usize;
    let mut late_bit_flips=0_usize;
    let mut late_commit_flips=0_usize;
    let mut last_late_bit_serial=0_i64;
    while paid<BUDGET&&!w.process(1).unwrap().stopped{
        // CRITICAL: after both serial checks, the actual next paid native
        // instruction is PC119 Read(cell20). An EXTERNAL late overwrite
        // cannot grant credits or change code, but can flip its conclusion.
        if !trial&&arm==Arm::LateBitFlip&&!previous_submit_truth
            &&w.process(1).unwrap().pc==119
            &&last_late_bit_serial!=last_serial{
            external(out,&mut w,id,20,1,"late_bit_flip_after_paid_commit_check");
            late_bit_flips+=1;
            last_late_bit_serial=last_serial;
        }
        let e=step(out,&mut w,id,&mut paid,&mut peak);
        if retained>0&&matches!(e,EventKind::ExecutableEdgePatched{..}){
            post_retention_patches+=1;
        }
        if let EventKind::ExecutableEdgePatched{node,before,after}=&e{
            if (*node==76&&*before==50&&*after==97)
                ||(*node==34&&*before==2&&*after==99)
                ||(*node==49&&*before==50&&*after==76) {
                bootstrap_patches+=1;
            }
        }
        match e{
            EventKind::CellWritten{key:26,value}=>{
                assert_eq!(value,last_serial+1,"PID1 must PAY for monotonic nonce");
                last_serial=value;
                serial_writes+=1;
            },
            EventKind::CellWritten{key:11,value}=>{
                assert!(last_serial>0&&w.cell(26)==Some(last_serial));
                let stage=(service_count/WINDOW).min(2);
                let t=current_task(a,b,stage);
                let good=value==t.0*input+t.1;
                let nonce=last_serial;
                if trial{
                    trial_outputs+=1;
                    training_outputs_total+=1;
                    if !good{training_wrong+=1;}
                    packet(out,&mut w,id,i64::from(good),nonce,nonce,
                        "training_truth");
                    packet_before=(nonce,i64::from(good));
                    if good{replay_success=(nonce,1);}
                    input=TRAIN[trial_outputs%TRAIN.len()];
                    external(out,&mut w,id,7,input,"training_next");
                }else{
                    if service_count<3*WINDOW{
                        if good{correct[stage]+=1;}else{wrong[stage]+=1;}
                        service_count+=1;
                        if service_count%WINDOW==0&&service_count<3*WINDOW{
                            emit(out,json!({"type":"lab_transition","case":id,
                                "stage":service_count/WINDOW,
                                "after_service_outputs":service_count,
                                "charged":paid,"tick":w.tick()}));
                        }
                    }
                    let (echo,bit,commit)=match arm{
                        Arm::FreshAtomic|Arm::LateBitFlip|
                        Arm::LateCommitFlip=>(nonce,i64::from(good),nonce),
                        Arm::StableForgedCurrent=>(nonce,1,nonce),
                        Arm::OldNonceReplay=>
                            (replay_success.0,replay_success.1,replay_success.0),
                    };
                    packet(out,&mut w,id,bit,echo,commit,"service_packet");
                    service_feedback_packets+=1;
                    packet_before=(nonce,i64::from(good));
                    previous_submit_truth=good;
                    input=HELD[((service_count%WINDOW)+phase)%HELD.len()];
                    external(out,&mut w,id,7,input,"service_next");
                }
            },
            EventKind::ExecutableEdgePatched{node:2,before:42,after:106}=>{
                assert!(trial&&trial_outputs>=2&&trial_outputs%2==0);
                trial=false;retained+=1;
                choices.push([w.cell(22).unwrap(),w.cell(21).unwrap()]);
                input=HELD[phase];
                external(out,&mut w,id,7,input,"retained_start");
            },
            EventKind::CellWritten{key:25,value:1}=>{
                assert!(!trial&&retained>0);
                trial=true;reopened+=1;trial_outputs=0;input=TRAIN[0];
                external(out,&mut w,id,7,input,"native_reopen_request");
            },
            EventKind::CellWritten{key:29,value:1}=>{
                assert!(!trial&&retained>0);
                rejected+=1;
            },
            EventKind::CellRead{key:27,..} if !trial=>{
                response_echo_reads+=1;
            },
            EventKind::CellRead{key:28,..} if !trial=>{
                response_commit_reads+=1;
                if arm==Arm::LateCommitFlip{
                    // Commit really was correct at the actor's PAID Read.
                    // Changing it afterward cannot update the already-read
                    // register; demonstrates non-atomic external packet state.
                    external(out,&mut w,id,28,last_serial-1,
                        "late_commit_flip_after_paid_commit_read");
                    late_commit_flips+=1;
                }
            },
            EventKind::CellRead{key:20,value} if !trial=>{
                post_retention_feedback_reads+=1;
                if value==1&&!previous_submit_truth{
                    false_accepted_success+=1;
                }
            },
            _=>{}
        }
    }
    assert_eq!(paid,BUDGET,"all source-identical arms must expend their 4000 paid CIL credits");
    assert_eq!(bootstrap_patches,3,"source must pay for every bootstrap source edit");
    let p=w.process(1).unwrap();
    let summary=json!({"type":"end","case":id,"transition":transition,
        "from":[a.0,a.1],"to":[b.0,b.1],"arm":arm.tag(),
        "phase":phase,"paid":paid,"credits":p.credits,"stopped":p.stopped,
        "retentions":retained,"reopens":reopened,"choices":choices,
        "service_writes":service_count,"window_correct":correct,
        "window_wrong":wrong,"nonce_writes":serial_writes,
        "final_nonce":last_serial,"response_echo_reads":response_echo_reads,
        "response_commit_reads":response_commit_reads,
        "service_feedback_reads":post_retention_feedback_reads,
        "rejections":rejected,
        "false_accepted_success":false_accepted_success,
        "late_bit_flips":late_bit_flips,
        "late_commit_flips":late_commit_flips,
        "post_retention_patches":post_retention_patches,
        "bootstrap_patches":bootstrap_patches,
        "training_outputs":training_outputs_total,
        "training_wrong":training_wrong,
        "service_feedback_packets":service_feedback_packets,
        "final_pc":p.pc,"final_len":p.code.nodes.len(),
        "peak_storage":peak,"final_regs":p.registers,
        "final_cells":state(&w),
        "final_edge2":edge(&p.code.nodes[2]),
        "final_edge34":edge(&p.code.nodes[34]),
        "final_edge76":edge(&p.code.nodes[76])});
    emit(out,summary.clone());
    summary
}
#[test]
fn b54_native_paid_interleaving_falsifiers(){
    let folder=std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../cl1b-ci-evidence");
    fs::create_dir_all(&folder).unwrap();
    let path=folder.join("b54_native_steps.jsonl");
    let mut out=BufWriter::new(File::create(&path).unwrap());
    emit(&mut out,json!({"type":"meta","study":"B54","version":1,
        "source_sha":std::env::var("GITHUB_SHA")
            .unwrap_or_else(|_|"local".into()),
        "founder_source":"cl1b_b53_native_response_correlation.rs",
        "serial_protocol":"native_inc26_echo27_commit28_bit20"}));
    let arms=[Arm::FreshAtomic,Arm::LateBitFlip,Arm::LateCommitFlip,
        Arm::StableForgedCurrent,Arm::OldNonceReplay];
    let transitions=[
        ("A",(1_i64,-2_i64),(-1_i64,3_i64)),
        ("B",(-1_i64,-3_i64),(1_i64,2_i64)),
    ];
    let mut cases=Vec::new();
    for (name,a,b) in transitions{
        for phase in [0,1]{
            for arm in arms{
                let id=cases.len();
                cases.push(run_case(&mut out,id,name,a,b,arm,phase));
            }
        }
    }
    out.flush().unwrap();
    assert_eq!(cases.len(),20);
    for row in &cases{
        assert_eq!(row["paid"],json!(4000));
        assert!(row["peak_storage"].as_u64().unwrap()<=300);
        assert_eq!(row["bootstrap_patches"],json!(3));
    }
    eprintln!("B54_NATIVE_OUTCOMES {}",serde_json::to_string(&cases).unwrap());
    eprintln!("B54_NATIVE trace={}",path.display());
}
