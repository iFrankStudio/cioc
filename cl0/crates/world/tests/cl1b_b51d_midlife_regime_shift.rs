//! B51d preregistered *negative* midlife-regime control, unchanged B51c CIL.
//! One PID1, same 72-node code, no actor process reset or host code edit.
//! Real post-retention feedback is external data; use requires PAID CIL Read.
use cl0_world::{EventKind,Intervention,Op,Origin,Process,World};
use serde_json::{json,Value};
use std::fs::{self,File};
use std::io::{BufWriter,Write};

#[path="cl1b_b51c_paid_search_retention.rs"]
mod b51c;

const BUDGET:usize=1300;
const CAP:usize=192;
const TRAIN:[i64;2]=[5,-3];
const HELD:[i64;3]=[11,-7,0];
const WINDOW:usize=24;
const WINDOWS:usize=3;

#[derive(Clone,Copy,Debug,PartialEq,Eq)]
enum Arm{Stationary,ShiftNoFeedback,ShiftWithFeedback,ShiftReturn}
impl Arm{fn tag(self)->&'static str{
    match self{
        Arm::Stationary=>"STATIONARY",
        Arm::ShiftNoFeedback=>"SHIFT_NO_FEEDBACK",
        Arm::ShiftWithFeedback=>"SHIFT_WITH_FEEDBACK",
        Arm::ShiftReturn=>"SHIFT_RETURN"
    }
}}
#[derive(Clone,Copy,Debug)]
struct Task{ s:i64,k:i64 }
fn modes(a:Task,b:Task,arm:Arm,stage:usize)->Task{
    if stage==0 || arm==Arm::Stationary || (stage==2&&arm==Arm::ShiftReturn) { a }
    else { b }
}
fn emit(out:&mut BufWriter<File>,v:Value){
    serde_json::to_writer(&mut *out,&v).unwrap();
    out.write_all(b"\n").unwrap();
}
fn cell_state(w:&World)->Value{
    json!([w.cell(7),w.cell(11),w.cell(20),w.cell(21),
        w.cell(22),w.cell(24),w.cell(30),w.cell(31)])
}
fn edge(n:&Op)->Option<usize>{
    match n{
        Op::Const{next,..}|Op::Add{next,..}|Op::Sub{next,..}
        |Op::Read{next,..}|Op::Write{next,..}|Op::PatchNext{next,..}
        |Op::CloneNode{next,..}=>Some(*next),
        _=>None
    }
}
fn event_kind(event:&EventKind)->Value{
    match event{
        EventKind::Executed=>json!({"type":"executed"}),
        EventKind::CellRead{key,value}=>json!({"type":"read","key":key,"value":value}),
        EventKind::CellWritten{key,value}=>json!({"type":"write","key":key,"value":value}),
        EventKind::ExecutableEdgePatched{node,before,after}=>
            json!({"type":"patch","node":node,"before":before,"after":after}),
        EventKind::Halted=>json!({"type":"halt"}),
        EventKind::Trapped(msg)=>json!({"type":"trap","message":msg}),
        _=>panic!("B51d unknown CIL event {event:?}")
    }
}
fn external(out:&mut BufWriter<File>,w:&mut World,case:usize,
            key:i64,value:i64,reason:&str){
    let before=w.total_compute_inventory().unwrap();
    w.intervene(Intervention::SetCell{key,value}).unwrap();
    let e=w.events().last().unwrap();
    assert_eq!(e.origin,Origin::External);
    assert_eq!(w.total_compute_inventory(),Some(before),
        "external evaluator MUST NOT credit compute");
    emit(out,json!({"type":"external","case":case,"reason":reason,
        "key":key,"value":value,"tick":e.tick,"credits":w.remaining_credits(),
        "storage":w.occupied_units(),"cells":cell_state(w)}));
}
fn step_native(out:&mut BufWriter<File>,w:&mut World,case:usize,
               charged:&mut usize,peak:&mut usize)->EventKind{
    let before=w.total_compute_inventory().unwrap();
    let old_len=w.process(1).unwrap().code.nodes.len();
    assert!(w.step());
    *charged+=1;
    let e=w.events().last().unwrap().clone();
    assert_eq!(e.origin,Origin::Internal(1));
    assert_eq!(e.process,1);
    assert!(!e.pool_paid_attempt);
    assert_eq!(w.total_compute_inventory(),Some(before-1));
    let p=w.process(1).unwrap();
    assert_eq!(e.credits_after,p.credits);
    let growth=p.code.nodes.len()-old_len;
    *peak=(*peak).max(w.occupied_units());
    assert!(*peak<=CAP && p.code.nodes.len()<=120);
    emit(out,json!({"type":"step","case":case,"step":*charged,
        "tick":e.tick,"pc":e.pc,"pc_after":p.pc,
        "event":event_kind(&e.kind),"regs":p.registers,
        "credits":p.credits,"stopped":p.stopped,
        "code_len":p.code.nodes.len(),"growth":growth,
        "edge2":edge(&p.code.nodes[2]),"edge33":edge(&p.code.nodes[33]),
        "edge72":p.code.nodes.get(72).and_then(edge),
        "edge73":p.code.nodes.get(73).and_then(edge),
        "storage":w.occupied_units(),"cells":cell_state(w)}));
    e.kind
}
fn run_case(out:&mut BufWriter<File>,case:usize,transition:&str,
            a:Task,b:Task,phase:usize,arm:Arm)->Value{
    let code=b51c::founder();
    assert_eq!(code.len(),72);
    assert!(matches!(code[0],Op::Add{next:0,..}));
    assert!(matches!(code[1],Op::Sub{next:1,..}));
    assert!(matches!(code[33],Op::Read{next:33,..}));
    let mut w=World::with_strict_capacity(16,120,CAP)
        .with_native_executable_edit_trace();
    let mut proc=Process::new(1,code,BUDGET as u64);
    proc.pc=3;
    w.load(proc).unwrap();
    emit(out,json!({"type":"case","case":case,"transition":transition,
        "arm":arm.tag(),"phase":phase,"from":[a.s,a.k],"to":[b.s,b.k],
        "initial_len":72,"initial_credits":BUDGET,
        "initial_storage":w.occupied_units()}));
    let mut charged=0_usize;
    let mut peak=w.occupied_units();
    if phase==1 {
        // Genuine paid CIL initializer op at pc3, delivered *before*
        // the task's initial external input. No code/branch modification;
        // this is a task-input-unobserving charged preamble, not a fake clock.
        let e=step_native(out,&mut w,case,&mut charged,&mut peak);
        assert!(matches!(e,EventKind::Executed));
        assert_eq!(w.process(1).unwrap().pc,4);
    }
    let mut input=TRAIN[0];
    external(out,&mut w,case,7,input,"initial_training");
    let mut training_outputs=0_usize;
    let mut training_correct=0_usize;
    let mut training_wrong=0_usize;
    let mut accepted=false;
    let mut retain_step=None::<usize>;
    let mut chosen=None::<[i64;2]>;
    let mut total_held=0_usize;
    let mut correct=[0_usize;WINDOWS];
    let mut wrong=[0_usize;WINDOWS];
    let mut post_feedback_writes=0_usize;
    let mut feedback_reads_after_retain=0_usize;
    let mut patches_after_retain=0_usize;
    let mut clones_after_retain=0_usize;
    while charged<BUDGET && !w.process(1).unwrap().stopped
        && total_held<WINDOWS*WINDOW {
        let prev_nodes=w.process(1).unwrap().code.nodes.len();
        let e=step_native(out,&mut w,case,&mut charged,&mut peak);
        if accepted {
            if matches!(&e,EventKind::CellRead{key:20,..}) {
                feedback_reads_after_retain+=1;
            }
            if matches!(&e,EventKind::ExecutableEdgePatched{..}){
                patches_after_retain+=1;
            }
            clones_after_retain+=w.process(1).unwrap().code.nodes.len()-prev_nodes;
        }
        match e {
            EventKind::CellWritten{key:11,value}=>{
                if !accepted{
                    let good=value==a.s*input+a.k;
                    training_outputs+=1;
                    if good {training_correct+=1;} else {training_wrong+=1;}
                    external(out,&mut w,case,20,i64::from(good),
                        "post_output_bit");
                    input=TRAIN[training_outputs%TRAIN.len()];
                    external(out,&mut w,case,7,input,"training_next");
                }else{
                    let stage=total_held/WINDOW;
                    let task=modes(a,b,arm,stage);
                    let good=value==task.s*input+task.k;
                    if good {correct[stage]+=1;} else {wrong[stage]+=1;}
                    total_held+=1;
                    if arm==Arm::ShiftWithFeedback && stage>=1 {
                        // Ordinary EXTERNAL data only; no paid actor Read is
                        // implied by a host SetCell.
                        external(out,&mut w,case,20,i64::from(good),
                            "post_shift_feedback");
                        post_feedback_writes+=1;
                    }
                    if total_held%WINDOW==0 && total_held<WINDOWS*WINDOW{
                        let new_stage=total_held/WINDOW;
                        let old=modes(a,b,arm,new_stage-1);
                        let new=modes(a,b,arm,new_stage);
                        // This lab-only phase marker produces NO VM event.
                        emit(out,json!({"type":"lab_transition","case":case,
                            "stage":new_stage,"after_outputs":total_held,
                            "from":[old.s,old.k],"to":[new.s,new.k],
                            "charged":charged,"tick":w.tick()}));
                    }
                    if total_held<WINDOWS*WINDOW{
                        let within=total_held%WINDOW;
                        input=HELD[(within+phase)%HELD.len()];
                        external(out,&mut w,case,7,input,"heldout_next");
                    }
                }
            },
            EventKind::ExecutableEdgePatched{node:2,before:42,after:30}=>{
                assert!(!accepted && training_outputs>=2);
                assert_eq!(training_outputs%2,0);
                accepted=true;
                retain_step=Some(charged);
                chosen=Some([w.cell(22).unwrap(),w.cell(21).unwrap()]);
                input=HELD[phase];
                external(out,&mut w,case,7,input,"retained_holdout_start");
            },
            _=>{}
        }
    }
    let p=w.process(1).unwrap();
    let row=json!({"type":"end","case":case,"transition":transition,
        "arm":arm.tag(),"phase":phase,"from":[a.s,a.k],"to":[b.s,b.k],
        "paid":charged,"credits":p.credits,"stopped":p.stopped,
        "retained":accepted,"retain_step":retain_step,"chosen":chosen,
        "training_outputs":training_outputs,"training_correct":training_correct,
        "training_wrong":training_wrong,"heldout_outputs":total_held,
        "window_correct":correct,"window_wrong":wrong,
        "post_shift_feedback_writes":post_feedback_writes,
        "feedback_reads_after_retention":feedback_reads_after_retain,
        "patches_after_retention":patches_after_retain,
        "clones_after_retention":clones_after_retain,
        "final_len":p.code.nodes.len(),"final_pc":p.pc,
        "final_regs":p.registers,"edge2":edge(&p.code.nodes[2]),
        "edge33":edge(&p.code.nodes[33]),"peak_storage":peak,
        "final_cells":cell_state(&w)});
    emit(out,row.clone());
    row
}
fn static_segment(task:Task,phase:usize)->Value{
    // A deliberately UNFAIR, task-informed upper bound. The laboratory
    // rebuilds a new oracle for EACH hidden regime; the actual CIL actor
    // never receives this privilege or the hidden (s,k).
    let arithmetic=if task.s==1{
        Op::Add{dst:0,lhs:0,rhs:1,next:4}
    }else{
        Op::Sub{dst:0,lhs:1,rhs:0,next:4}
    };
    let code=vec![
        Op::Const{dst:1,value:task.k,next:1},
        Op::Const{dst:2,value:7,next:2},
        Op::Read{dst:0,address:2,next:3},
        arithmetic,
        Op::Const{dst:2,value:11,next:5},
        Op::Write{address:2,value:0,next:0},
    ];
    let mut w=World::with_strict_capacity(16,120,CAP);
    w.load(Process::new(1,code,BUDGET as u64)).unwrap();
    let mut input=HELD[phase];
    w.intervene(Intervention::SetCell{key:7,value:input}).unwrap();
    let mut count=0_usize;
    let mut wrong=0_usize;
    while count<WINDOW{
        assert!(w.step());
        if let EventKind::CellWritten{key:11,value}=w.events().last().unwrap().kind{
            if value!=task.s*input+task.k{wrong+=1;}
            count+=1;
            if count<WINDOW{
                input=HELD[(count+phase)%HELD.len()];
                w.intervene(Intervention::SetCell{key:7,value:input}).unwrap();
            }
        }
    }
    assert_eq!(w.remaining_credits(),(BUDGET-6*WINDOW) as u64);
    json!({"mode":[task.s,task.k],"phase":phase,"paid":6*WINDOW,
        "correct":count-wrong,"wrong":wrong})
}
#[test]
fn b51d_midlife_hidden_task_shift_native_falsifier(){
    let folder=std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../cl1b-ci-evidence");
    fs::create_dir_all(&folder).unwrap();
    let path=folder.join("b51d_native_steps.jsonl");
    let mut out=BufWriter::new(File::create(&path).unwrap());
    emit(&mut out,json!({"type":"meta","study":"B51d","version":1,
        "source_sha":std::env::var("GITHUB_SHA")
            .unwrap_or_else(|_|"local".into()),
        "founder_source":"cl1b_b51c_paid_search_retention.rs"}));
    let transitions=[
        ("A",Task{s:1,k:-2},Task{s:-1,k:3}),
        ("B",Task{s:-1,k:-3},Task{s:1,k:2}),
    ];
    let arms=[Arm::Stationary,Arm::ShiftNoFeedback,
        Arm::ShiftWithFeedback,Arm::ShiftReturn];
    let mut cases=Vec::new();
    for (name,a,b) in transitions{
        for phase in 0..2{
            for arm in arms{
                let id=cases.len();
                cases.push(run_case(&mut out,id,name,a,b,phase,arm));
            }
            for stage in 0..WINDOWS{
                // The exact oracle is RESEEDED with privileged knowledge
                // of the selected mode, so this is NOT actor transfer.
                let mode=if stage==0 {a}else{b};
                let native=static_segment(mode,phase);
                emit(&mut out,json!({"type":"static","transition":name,
                    "phase":phase,"stage":stage,"result":native}));
            }
        }
    }
    assert_eq!(cases.len(),16);
    out.flush().unwrap();
    for a in cases.chunks(4){
        let expected=a[0]["paid"].clone();
        for row in a{
            assert_eq!(row["paid"],expected,
                "all four arms must pay equal actual native instructions");
            assert!(row["paid"].as_u64().unwrap()<=BUDGET as u64);
            assert!(row["peak_storage"].as_u64().unwrap()<=CAP as u64);
        }
    }
    eprintln!("B51D_NATIVE_END {}",serde_json::to_string(&cases).unwrap());
    eprintln!("B51D_NATIVE trace={}",path.display());
}