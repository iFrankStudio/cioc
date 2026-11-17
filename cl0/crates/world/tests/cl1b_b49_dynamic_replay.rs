//! B49: independent dynamic replay oracle *outside* the Rust VM.
//! This native target emits an exact, source-pinned, one-row-per-paid-CIL-step
//! trace. A separately authored JS CIL interpreter must reproduce every row
//! in CI, not merely search source strings or compare final aggregates.
//! No new opcode, actor-authored policy, mutation heuristic, or fitness oracle.
use cl0_world::{EventKind, Intervention, Op, Origin, Process, World};
use serde_json::{json,Value};
use std::fs::{self,File};
use std::io::{BufWriter,Write};

#[path="cl1b_b42_self_assembled_probe.rs"]
mod b42;

#[derive(Clone,Copy,Debug,PartialEq,Eq)]
enum Arm { True, BlindSelf, BlindZero, Static }
const ARMS:[Arm;4]=[Arm::True,Arm::BlindSelf,Arm::BlindZero,Arm::Static];

// B48's 8 previously unseen sequences, plus four NEW frozen B49 inputs
// not used by B47 or B48. Order matters for the independent replay.
const B48_UNSEEN:[[i64;3];8]=[
    [23,0,-1],[-9,17,23],[1,-9,0],[0,23,17],
    [17,-9,-9],[-1,1,23],[23,23,0],[-9,0,1],
];
const B49_NEW:[[i64;3];4]=[
    [2,0,-7],[-4,31,0],[0,7,-4],[31,2,7],
];

fn founder(arm:Arm,shift:bool)->(Process,usize) {
    let mut nodes=if arm==Arm::Static {
        b42::founder(b42::Arm::StaticCorrect)
    } else { b42::founder(b42::Arm::ProbePlus) };
    assert_eq!(nodes.len(),if arm==Arm::Static{4}else{38});
    match arm {
        Arm::BlindSelf => nodes[12]=Op::Sub{dst:1,lhs:0,rhs:0,next:13},
        Arm::BlindZero => nodes[12]=Op::Sub{dst:1,lhs:0,rhs:3,next:13},
        Arm::True|Arm::Static => {},
    }
    let preamble=nodes.len();
    if shift { nodes.push(Op::Const{dst:3,value:0,next:0}); }
    let original=nodes.len();
    let mut p=Process::new(1,nodes,144);
    if shift {p.pc=preamble;}
    (p,original)
}
fn patch_edge(p:&Process,node:usize)->Option<usize>{
    match p.code.nodes.get(node)?{
        Op::Const{next,..}|Op::Add{next,..}|Op::Sub{next,..}
        |Op::Read{next,..}|Op::Write{next,..}
        |Op::CloneNode{next,..}|Op::PatchNext{next,..}
        |Op::ReadNext{next,..}=>Some(*next),
        _=>None,
    }
}
fn native_kind(kind:&EventKind)->Value {
    match kind {
        EventKind::Executed=>json!({"type":"executed"}),
        EventKind::CellRead{key,value}=>json!({"type":"read","key":key,"value":value}),
        EventKind::CellWritten{key,value}=>json!({"type":"write","key":key,"value":value}),
        EventKind::ExecutableEdgePatched{node,before,after}=>
            json!({"type":"patch","node":node,"before":before,"after":after}),
        EventKind::Halted=>json!({"type":"halted"}),
        EventKind::Trapped(message)=>json!({"type":"trapped","message":message}),
        _=>panic!("B49 intentionally narrow CIL replay cannot classify {kind:?}"),
    }
}
fn emit(writer:&mut BufWriter<File>,v:Value){
    serde_json::to_writer(&mut *writer,&v).unwrap();
    writer.write_all(b"\n").unwrap();
}

#[derive(Debug)]
struct NativeCase {
    block:usize,seed:usize,shift:bool,arm:Arm,
    inputs:[i64;3],paid:usize,correct:[usize;3],wrong:[usize;3],
    edits:usize,halted:bool,peak_storage:usize,clone_count:usize,
}

fn run(case_id:usize,block:usize,seed:usize,shift:bool,arm:Arm,
       inputs:[i64;3],out:&mut BufWriter<File>)->NativeCase{
    let (p,initial_code_len)=founder(arm,shift);
    let mut w=World::with_strict_capacity(16,80,192)
        .with_native_executable_edit_trace();
    w.load(p).unwrap();
    let mut correct=[0usize;3];
    let mut wrong=[0usize;3];
    let mut paid=0usize;
    let mut edits=0usize;
    let mut peak=w.occupied_units();
    emit(out,json!({"type":"case","case":case_id,"block":block,"seed":seed,
        "shift":shift,"arm":format!("{arm:?}"),"inputs":inputs,
        "initial_len":initial_code_len,
        "initial_pc":w.process(1).unwrap().pc,
        "initial_tick":w.tick(),"initial_credits":144,
        "initial_storage":w.occupied_units()}));
    for (window,input) in inputs.into_iter().enumerate(){
        w.intervene(Intervention::SetCell{key:7,value:input}).unwrap();
        assert_eq!(w.events().last().unwrap().origin,Origin::External);
        emit(out,json!({"type":"input","case":case_id,"window":window,
            "input":input,"tick":w.tick(),"cell7":w.cell(7),
            "credits":w.remaining_credits(),
            "storage":w.occupied_units()}));
        for attempt in 0..48 {
            if w.process(1).unwrap().stopped {break;}
            let inventory=w.total_compute_inventory().unwrap();
            assert!(w.step());
            paid+=1;
            assert_eq!(w.total_compute_inventory(),Some(inventory-1));
            let event=w.events().last().unwrap();
            assert_eq!(event.origin,Origin::Internal(1));
            assert_eq!(event.process,1);
            assert_eq!(event.pool_paid_attempt,false);
            let p=w.process(1).unwrap();
            assert_eq!(event.credits_after,p.credits);
            if matches!(&event.kind,EventKind::ExecutableEdgePatched{..}){edits+=1;}
            let kind=native_kind(&event.kind);
            let storage=w.occupied_units();
            peak=peak.max(storage);
            assert!(storage<=192);
            assert!(p.code.nodes.len()<=80);
            if let EventKind::CellWritten{key:11,value}=&event.kind {
                // B49 raw scoring: actual input as of the actual native
                // Write event. Never ignore diagnostic writes or stopped worlds.
                let expected=w.cell(7).expect("input7 must exist");
                if *value==expected {correct[window]+=1;}
                else {wrong[window]+=1;}
            }
            let created=p.code.nodes.len()-initial_code_len;
            emit(out,json!({"type":"step","case":case_id,"window":window,
                "attempt":attempt,"tick":event.tick,"pc":event.pc,
                "pc_after":p.pc,"event":kind,"regs":p.registers,
                "credits":p.credits,"stopped":p.stopped,
                "cells":[w.cell(7),w.cell(11),w.cell(22),w.cell(30)],
                "code_len":p.code.nodes.len(),
                "route9":patch_edge(p,9),
                "edge7":patch_edge(p,7),
                "edge10":patch_edge(p,10),
                "edge11":patch_edge(p,11),
                "clone_count":created,
                "new_edge":if created>0{patch_edge(p,p.code.nodes.len()-1)}else{None},
                "storage":storage}));
        }
    }
    let p=w.process(1).unwrap();
    assert_eq!(w.total_compute_inventory(),Some(144-paid as u64));
    assert!(w.process(2).is_none());
    let row=NativeCase{block,seed,shift,arm,inputs,paid,correct,wrong,
        edits,halted:p.stopped,peak_storage:peak,
        clone_count:p.code.nodes.len()-initial_code_len};
    emit(out,json!({"type":"end","case":case_id,"paid":row.paid,
        "correct":row.correct,"wrong":row.wrong,"edits":row.edits,
        "stopped":row.halted,"peak_storage":row.peak_storage,
        "clone_count":row.clone_count}));
    row
}

#[test]
fn b49_independent_dynamic_replay_of_native_code_mutation_and_raw_task_outputs(){
    // The independent observer in .github/workflows/cl1b-focused.yml
    // reads this immutable native trace from its *own* CIL execution model.
    let artifact_dir="cl1b-ci-evidence";
    fs::create_dir_all(artifact_dir).unwrap();
    let path=format!("{artifact_dir}/b49_native_steps.jsonl");
    let mut file=BufWriter::new(File::create(&path).unwrap());
    emit(&mut file,json!({"type":"meta","study":"B49","version":1,
        "source_sha":std::env::var("GITHUB_SHA").unwrap_or_else(|_|"local".into())}));

    let mut case_id=0usize;
    let mut rows=Vec::new();
    for (block,sequences) in [
        (0_usize,B48_UNSEEN.as_slice()),
        (1_usize,B49_NEW.as_slice()),
    ]{
        for (seed,&seq) in sequences.iter().enumerate(){
            for shift in [false,true] {
                for arm in ARMS {
                    rows.push(run(case_id,block,seed,shift,arm,seq,&mut file));
                    case_id+=1;
                }
            }
        }
    }
    file.flush().unwrap();
    assert_eq!(rows.len(),96);
    let mut sum=[[[[0usize;2];2];4];2]; // phase, arm, holdout, correct/wrong
    let mut exposure=[[[0usize;3];4];2]; // block, arm, paid/halts/edits
    for r in &rows {
        assert!(r.paid<=144 && r.peak_storage<=192);
        let a=match r.arm{
            Arm::True=>0,Arm::BlindSelf=>1,Arm::BlindZero=>2,Arm::Static=>3,
        };
        let p=usize::from(r.shift);
        if r.block==0 {
            sum[p][a][0][0]+=r.correct[1];sum[p][a][0][1]+=r.wrong[1];
            sum[p][a][1][0]+=r.correct[2];sum[p][a][1][1]+=r.wrong[2];
        }
        exposure[r.block][a][0]+=r.paid;
        exposure[r.block][a][1]+=usize::from(r.halted);
        exposure[r.block][a][2]+=r.edits;
    }
    // Existing B48 unseen-block native aggregate is a REPLICATION CHECK,
    // not an independent forecast; this target's real independent check
    // is the separate event-by-event source-parsed JS interpreter in CI.
    assert_eq!(sum,[
        [
            [[64,15,0,0],[64,16,0,0]],
            [[18,54,0,0],[20,60,0,0]],
            [[0,9,0,0],[0,10,0,0]],
            [[96,0,0,0],[96,0,0,0]],
        ],
        [
            [[64,15,0,0],[63,10,0,0]],
            [[18,54,0,0],[20,60,0,0]],
            [[0,9,0,0],[0,10,0,0]],
            [[89,7,0,0],[89,7,0,0]],
        ],
    ]);
    eprintln!("B49_B48_UNSEEN_REPRO {:?}",sum);
    eprintln!("B49_EXPOSURE {:?}",exposure);
    for block in 0..2 {
        assert_eq!(exposure[block][0][0],if block==0{8*2*144}else{4*2*144});
        assert_eq!(exposure[block][1][0],if block==0{8*2*144}else{4*2*144});
        assert_eq!(exposure[block][3][0],if block==0{8*2*144}else{4*2*144});
        assert_eq!(exposure[block][0][1],0);
        assert_eq!(exposure[block][1][1],0);
        assert_eq!(exposure[block][3][1],0);
        // The blind-zero branch may stop; preserve its actual lost exposure.
        assert!(exposure[block][2][0]<=if block==0{8*2*144}else{4*2*144});
    }
    eprintln!("B49_NATIVE cases=96 trace={} NOTE independent replay pending",path);
}
