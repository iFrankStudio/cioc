//! CL-1B B51b: one continuous CIL process pays to assemble AND execute
//! two graph-distinct arithmetic candidates from disconnected generic
//! instruction templates. This is investigator-authored construction, NOT
//! selected learning, cross-task transfer or B3/B4 acceptance.
use cl0_world::{EventKind,Intervention,Op,Origin,Process,World};
use serde_json::{json,Value};
use std::fs::{self,File};
use std::io::{BufWriter,Write};

fn founder()->Vec<Op>{
    // Nodes 0/1 are UNWIRED arithmetic templates, node2 a reusable
    // output primitive. The 26-node initial graph contains no executed
    // Read->arithmetic->Write solution. Only paid CloneNode/PatchNext
    // can connect candidate 26, then candidate 27, to live I/O.
    vec![
        Op::Add{dst:0,lhs:0,rhs:1,next:0},             // 0 disconnected
        Op::Sub{dst:0,lhs:1,rhs:0,next:1},             // 1 disconnected
        Op::Write{address:2,value:0,next:13},         // 2 generic output
        Op::Const{dst:3,value:0,next:4},              // 3 Add template
        Op::CloneNode{dst:3,source:3,next:5},          // 4 creates node26
        Op::Const{dst:2,value:12,next:6},             // 5 bridge
        Op::PatchNext{node:3,target:2,next:7},        // 6 clone26->12
        Op::Const{dst:2,value:11,next:8},             // 7 input Read11
        Op::PatchNext{node:2,target:3,next:9},        // 8 Read11->26
        Op::Const{dst:1,value:2,next:10},             // 9 authored offset
        Op::Const{dst:2,value:7,next:11},             // 10 input key
        Op::Read{dst:0,address:2,next:0},             // 11 patched to26
        Op::Const{dst:2,value:11,next:2},             // 12 output key
        Op::Const{dst:3,value:1,next:14},             // 13 Sub template
        Op::CloneNode{dst:3,source:3,next:15},         // 14 creates node27
        Op::Const{dst:2,value:12,next:16},            // 15 bridge
        Op::PatchNext{node:3,target:2,next:17},       // 16 clone27->12
        Op::Const{dst:0,value:2,next:18},             // 17 Write node2
        Op::Const{dst:1,value:25,next:19},            // 18 Halt node25
        Op::PatchNext{node:0,target:1,next:20},       // 19 output->Halt
        Op::Const{dst:2,value:24,next:21},            // 20 input Read24
        Op::PatchNext{node:2,target:3,next:22},       // 21 Read24->27
        Op::Const{dst:1,value:2,next:23},             // 22 offset
        Op::Const{dst:2,value:7,next:24},             // 23 input key
        Op::Read{dst:0,address:2,next:0},             // 24 patched to27
        Op::Halt,                                     // 25
    ]
}
fn edge(op:&Op)->Option<usize>{
    match op{
        Op::Add{next,..}|Op::Sub{next,..}|Op::Write{next,..}
        |Op::Read{next,..}|Op::Const{next,..}|Op::CloneNode{next,..}
        |Op::PatchNext{next,..}=>Some(*next),
        _=>None,
    }
}
fn event_name(e:&EventKind)->Value{
    match e {
        EventKind::Executed=>json!("executed"),
        EventKind::CellRead{..}=>json!("read"),
        EventKind::CellWritten{..}=>json!("write"),
        EventKind::ExecutableEdgePatched{..}=>json!("patch"),
        EventKind::Halted=>json!("halt"),
        EventKind::Trapped(_)=>json!("trap"),
        EventKind::NativeCodeEditSuppressed{..}=>json!("suppressed"),
        _=>panic!("B51b unknown actual native event: {e:?}")
    }
}
fn write(out:&mut BufWriter<File>,v:Value){
    serde_json::to_writer(&mut *out,&v).unwrap();
    out.write_all(b"\n").unwrap();
}

fn execute(case_id:usize,input:i64,out:&mut BufWriter<File>){
    let code=founder();
    assert_eq!(code.len(),26);
    let mut process=Process::new(1,code,64);
    process.pc=3;
    let mut w=World::with_strict_capacity(16,80,192)
        .with_native_executable_edit_trace();
    w.load(process).unwrap();
    w.intervene(Intervention::SetCell{key:7,value:input}).unwrap();
    assert_eq!(w.events().last().unwrap().origin,Origin::External);
    write(out,json!({"type":"case","case":case_id,"input":input,
        "initial_pc":3,"initial_code_len":26,"initial_credits":64,
        "after_external_storage":w.occupied_units()}));
    let mut writes=Vec::new();
    let mut patches=Vec::new();
    let mut paid=0;
    let mut peak=w.occupied_units();
    while !w.process(1).unwrap().stopped && paid<64 {
        let before=w.total_compute_inventory().unwrap();
        let old_len=w.process(1).unwrap().code.nodes.len();
        assert!(w.step());
        paid+=1;
        assert_eq!(w.total_compute_inventory(),Some(before-1));
        let e=w.events().last().unwrap();
        assert_eq!(e.origin,Origin::Internal(1));
        assert_eq!(e.process,1);
        assert!(!e.pool_paid_attempt);
        let p=w.process(1).unwrap();
        assert_eq!(e.credits_after,p.credits);
        if let EventKind::CellWritten{key:11,value}=&e.kind {writes.push(*value);}
        if let EventKind::ExecutableEdgePatched{node,before,after}=&e.kind {
            patches.push((*node,*before,*after));
        }
        let cur_len=p.code.nodes.len();
        peak=peak.max(w.occupied_units());
        assert!(peak<=192 && cur_len<=80);
        write(out,json!({"type":"step","case":case_id,"step":paid,
            "tick":e.tick,"pc":e.pc,"pc_after":p.pc,
            "kind":event_name(&e.kind),"credits":p.credits,
            "regs":p.registers,"stopped":p.stopped,
            "code_len":cur_len,"code_growth":cur_len-old_len,
            "edge2":edge(&p.code.nodes[2]),
            "edge11":edge(&p.code.nodes[11]),
            "edge24":edge(&p.code.nodes[24]),
            "candidate26":p.code.nodes.get(26).map(|n|json!({
                "opcode":if matches!(n,Op::Add{..}){"Add"}else{"other"},
                "next":edge(n)})),
            "candidate27":p.code.nodes.get(27).map(|n|json!({
                "opcode":if matches!(n,Op::Sub{..}){"Sub"}else{"other"},
                "next":edge(n)})),
            "cells":[w.cell(7),w.cell(11)],
            "storage":w.occupied_units()}));
    }
    let p=w.process(1).unwrap();
    assert_eq!(paid,28,"one paid native CIL construction sequence");
    assert!(p.stopped);
    assert_eq!(writes,vec![input+2,2-input]);
    assert_eq!(patches,vec![
        (26,0,12),(11,0,26),(27,1,12),(2,13,25),(24,0,27)
    ]);
    assert_eq!(p.code.nodes.len(),28);
    assert!(matches!(p.code.nodes[26],Op::Add{next:12,..}));
    assert!(matches!(p.code.nodes[27],Op::Sub{next:12,..}));
    assert!(matches!(p.code.nodes[0],Op::Add{next:0,..}));
    assert!(matches!(p.code.nodes[1],Op::Sub{next:1,..}));
    assert_eq!(w.cell(11),Some(2-input));
    assert_eq!(w.occupied_units(),31);
    assert_eq!(w.total_compute_inventory(),Some(36));
    write(out,json!({"type":"end","case":case_id,"paid":paid,
        "writes":writes,"patches":patches,"peak_storage":peak,
        "stopped":p.stopped,"final_code_len":p.code.nodes.len(),
        "credits":p.credits}));
}

/// Native controls and trace are ONE study, not numerous trivial test targets.
#[test]
fn b51b_native_composition_paid_two_executable_candidates(){
    let artifact_dir=std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../cl1b-ci-evidence");
    fs::create_dir_all(&artifact_dir).unwrap();
    let path=artifact_dir.join("b51b_native_steps.jsonl");
    let mut out=BufWriter::new(File::create(&path).unwrap());
    write(&mut out,json!({"type":"meta","study":"B51b","version":1,
        "source_sha":std::env::var("GITHUB_SHA").unwrap_or_else(|_|"local".into())}));
    for (id,input) in [5_i64,-3,0].into_iter().enumerate(){
        execute(id,input,&mut out);
    }
    out.flush().unwrap();
    // Negative control: patch suppression prevents either cloned arithmetic
    // node from becoming reachable from the normal input-read route. It can
    // still spend compute cloning, but cannot generate true output.
    let mut process=Process::new(1,founder(),64);
    process.pc=3;
    let mut disabled=World::with_strict_capacity(16,80,192)
        .with_native_patchnext_disabled().with_native_executable_edit_trace();
    disabled.load(process).unwrap();
    disabled.intervene(Intervention::SetCell{key:7,value:5}).unwrap();
    let n=disabled.run(64);
    assert_eq!(n,64);
    assert_eq!(disabled.total_compute_inventory(),Some(0));
    assert!(!disabled.events().iter().any(|e|matches!(
        (&e.origin,&e.kind),
        (Origin::Internal(1),EventKind::CellWritten{key:11,..})
    )));
    assert!(!disabled.events().iter().any(|e|matches!(
        (&e.origin,&e.kind),
        (Origin::Internal(1),EventKind::ExecutableEdgePatched{..})
    )));
    eprintln!("B51B_NATIVE three worlds * 28 paid steps, two new executable graphs per world; mutation-disabled control 64 charges / zero output; separate JS per-step replay pending; trace={}",path.display());
}
