//! CL-1B B50: is real paid functional feedback needed for B42's authored
//! candidate, or does unconditional paid construction perform as well?
//! Prospective registration: cl1b/studies/B50-paid-feedback-vs-forced-construction.md
//! This test emits native aggregate evidence, NOT a claim of autonomous learning.
use cl0_world::{EventKind, Intervention, Op, Origin, Process, World};
use serde_json::{json, Value};
use std::fs::{self, File};
use std::io::{BufWriter, Write};

#[path="cl1b_b42_self_assembled_probe.rs"]
mod b42;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Arm { True, BlindNever, ForcedClone, Static }
const ARMS: [Arm;4] = [Arm::True,Arm::BlindNever,Arm::ForcedClone,Arm::Static];
// Frozen in the B50 prospective protocol, before native CI.
const SEQUENCES: [[i64;3];6] = [
    [0,2,-1],[-1,-1,3],[5,0,5],[-7,4,0],[1,1,-2],[19,-3,-1],
];

fn founder(arm:Arm,shift:bool)->(Process,usize) {
    let mut nodes=if arm==Arm::Static {
        b42::founder(b42::Arm::StaticCorrect)
    } else { b42::founder(b42::Arm::ProbePlus) };
    assert_eq!(nodes.len(),if arm==Arm::Static{4}else{38});
    match arm {
        Arm::BlindNever => {
            nodes[12]=Op::Sub{dst:1,lhs:0,rhs:0,next:13};
        },
        Arm::ForcedClone => {
            nodes[12]=Op::Const{dst:1,value:1,next:13};
            // Second paid trial never rejects merely because the comparator
            // was forced: it takes the existing retention path instead.
            nodes[16]=Op::Branch{test:1,nonzero:26,zero:17};
        },
        Arm::True|Arm::Static => {},
    }
    let preamble=nodes.len();
    if shift { nodes.push(Op::Const{dst:3,value:0,next:0}); }
    let initial_len=nodes.len();
    let mut process=Process::new(1,nodes,144);
    if shift {process.pc=preamble;}
    (process,initial_len)
}

fn emit(out:&mut BufWriter<File>, row:Value) {
    serde_json::to_writer(&mut *out,&row).unwrap();
    out.write_all(b"\n").unwrap();
}

fn run(case_id:usize,sequence_id:usize,shift:bool,arm:Arm,
       inputs:[i64;3],out:&mut BufWriter<File>)->Value {
    let (process,initial_len)=founder(arm,shift);
    let mut w=World::with_strict_capacity(16,80,192)
        .with_native_executable_edit_trace();
    w.load(process).unwrap();
    let mut correct=[0_usize;3];
    let mut wrong=[0_usize;3];
    let mut charged=0_usize;
    let mut edits=0_usize;
    let mut peak_storage=w.occupied_units();
    for (window,input) in inputs.into_iter().enumerate() {
        w.intervene(Intervention::SetCell{key:7,value:input}).unwrap();
        assert_eq!(w.events().last().unwrap().origin,Origin::External);
        peak_storage=peak_storage.max(w.occupied_units());
        for _ in 0..48 {
            if w.process(1).unwrap().stopped {break;}
            let prior_inventory=w.total_compute_inventory().unwrap();
            assert!(w.step(),"B50 actor must execute the accounted step");
            assert_eq!(w.total_compute_inventory(),Some(prior_inventory-1));
            charged+=1;
            let event=w.events().last().unwrap();
            assert_eq!(event.origin,Origin::Internal(1));
            assert_eq!(event.process,1);
            assert!(!event.pool_paid_attempt);
            assert_eq!(event.credits_after,w.process(1).unwrap().credits);
            if matches!(&event.kind,EventKind::ExecutableEdgePatched{..}) {
                edits+=1;
            }
            if let EventKind::CellWritten{key:11,value}=&event.kind {
                if Some(*value)==w.cell(7) {correct[window]+=1;}
                else {wrong[window]+=1;}
            }
            peak_storage=peak_storage.max(w.occupied_units());
            assert!(peak_storage<=192);
            assert!(w.process(1).unwrap().code.nodes.len()<=80);
        }
    }
    let p=w.process(1).unwrap();
    let stopped=p.stopped;
    assert_eq!(w.total_compute_inventory(),Some(144-charged as u64));
    assert!(w.process(2).is_none());
    let route9=if arm==Arm::Static {None} else {
        Some(match &p.code.nodes[9] {
            Op::Const{next,..}=>*next,_=>panic!("B50 missing route9"),
        })
    };
    json!({"type":"case","case":case_id,"sequence":sequence_id,
        "shift":shift,"arm":format!("{arm:?}"),"inputs":inputs,
        "initial_len":initial_len,"charged":charged,
        "correct":correct,"wrong":wrong,"edits":edits,
        "stopped":stopped,"peak_storage":peak_storage,
        "clones":p.code.nodes.len()-initial_len,"route9":route9,
        "final_code_len":p.code.nodes.len(),"final_pc":p.pc,
        "final_registers":p.registers,"final_cells":[
            w.cell(7),w.cell(11),w.cell(22),w.cell(30)]})
}

#[test]
fn b50_paid_feedback_versus_nonhalting_blind_forced_construction() {
    // Cargo runs in its crate manifest CWD, whereas the independent JS
    // observer runs from cl0/. Use the exact same job evidence directory.
    let artifact_dir=std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../cl1b-ci-evidence");
    fs::create_dir_all(&artifact_dir).unwrap();
    let path=artifact_dir.join("b50_native_cases.jsonl");
    let mut out=BufWriter::new(File::create(&path).unwrap());
    emit(&mut out,json!({"type":"meta","study":"B50","version":1,
        "source_sha":std::env::var("GITHUB_SHA")
            .unwrap_or_else(|_|"local".into())}));
    let mut rows=Vec::new();
    for (sequence_id,&inputs) in SEQUENCES.iter().enumerate() {
        for shift in [false,true] {
            for arm in ARMS {
                let case_id=rows.len();
                let row=run(case_id,sequence_id,shift,arm,inputs,&mut out);
                emit(&mut out,row.clone());
                rows.push(row);
            }
        }
    }
    out.flush().unwrap();
    assert_eq!(rows.len(),48);
    // Exposure is a prerequisite for comparing nonhalting controls. If
    // this fails, the experimental gate fails; no denominator repair.
    for row in &rows {
        assert_eq!(row["charged"],144,
            "B50 equal realized exposure failure (do not exclude world)");
        assert_eq!(row["stopped"],false);
    }
    for shift in [false,true] {
        for arm in ["True","BlindNever","ForcedClone","Static"] {
            let cohort=rows.iter().filter(|r|
                r["shift"]==shift && r["arm"]==arm).collect::<Vec<_>>();
            assert_eq!(cohort.len(),6);
            let mut correct=[0_u64;3];
            let mut wrong=[0_u64;3];
            let mut edits=0_u64;
            for row in cohort {
                for window in 0..3 {
                    correct[window]+=row["correct"][window].as_u64().unwrap();
                    wrong[window]+=row["wrong"][window].as_u64().unwrap();
                }
                edits+=row["edits"].as_u64().unwrap();
            }
            eprintln!("B50_NATIVE shift={shift} arm={arm} correct={correct:?} wrong={wrong:?} edits={edits} charged=864");
        }
    }
    eprintln!("B50_NATIVE 48 worlds; 6912 paid CIL instructions; native aggregate replay by independent JS is a separate CI gate: {}",path.display());
}
