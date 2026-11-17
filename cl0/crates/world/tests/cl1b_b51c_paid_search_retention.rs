//! B51c source-pinned native trial: one continuous paid CIL actor composes
//! two arithmetic operators and searches signed offsets from feedback bits.
//! The search policy and tasks are researcher-authored. NOT B3/B4/cyberlife.
use cl0_world::{EventKind,Intervention,Op,Origin,Process,World};
use serde_json::{json,Value};
use std::fs::{self,File};
use std::io::{BufWriter,Write};

const BUDGET:usize=1300;
const CAP:usize=192;
const TRAIN:[i64;2]=[5,-3];
const HELD:[i64;3]=[11,-7,0];

fn founder()->Vec<Op>{
    // 0/1 disconnected generic arithmetic templates, 2 generic output;
    // initial executable input node33 loops to itself, not to a candidate.
    // All connecting clones and patches are paid native actor instructions.
    // k ranges -3..3, operator Add then Sub: NO task-mode-specific source.
    vec![
        Op::Add{dst:0,lhs:0,rhs:1,next:0}, // 0
        Op::Sub{dst:0,lhs:1,rhs:0,next:1}, // 1
        Op::Write{address:2,value:0,next:35}, // 2
        Op::Const{dst:1,value:-3,next:4}, // 3
        Op::Const{dst:2,value:21,next:5}, // 4
        Op::Write{address:2,value:1,next:6}, // 5
        Op::Const{dst:1,value:0,next:7}, // 6
        Op::Const{dst:2,value:22,next:8}, // 7
        Op::Write{address:2,value:1,next:9}, // 8
        Op::Const{dst:2,value:0,next:10}, // 9
        Op::CloneNode{dst:3,source:2,next:11}, // 10
        Op::Const{dst:2,value:30,next:12}, // 11
        Op::Write{address:2,value:3,next:13}, // 12
        Op::Const{dst:2,value:34,next:14}, // 13
        Op::PatchNext{node:3,target:2,next:15}, // 14
        Op::Const{dst:2,value:1,next:16}, // 15
        Op::CloneNode{dst:3,source:2,next:17}, // 16
        Op::Const{dst:2,value:31,next:18}, // 17
        Op::Write{address:2,value:3,next:19}, // 18
        Op::Const{dst:2,value:34,next:20}, // 19
        Op::PatchNext{node:3,target:2,next:21}, // 20
        Op::Const{dst:2,value:22,next:22}, // 21
        Op::Read{dst:3,address:2,next:23}, // 22
        Op::Branch{test:3,nonzero:26,zero:24}, // 23
        Op::Const{dst:2,value:30,next:25}, // 24
        Op::Read{dst:3,address:2,next:28}, // 25
        Op::Const{dst:2,value:31,next:27}, // 26
        Op::Read{dst:3,address:2,next:28}, // 27
        Op::Const{dst:2,value:33,next:29}, // 28
        Op::PatchNext{node:2,target:3,next:30}, // 29
        Op::Const{dst:2,value:21,next:31}, // 30
        Op::Read{dst:1,address:2,next:32}, // 31
        Op::Const{dst:2,value:7,next:33}, // 32
        Op::Read{dst:0,address:2,next:33}, // 33
        Op::Const{dst:2,value:11,next:2}, // 34
        Op::Const{dst:2,value:20,next:36}, // 35
        Op::Read{dst:3,address:2,next:37}, // 36
        Op::Const{dst:2,value:24,next:38}, // 37
        Op::Write{address:2,value:3,next:39}, // 38
        Op::Const{dst:2,value:2,next:40}, // 39
        Op::Const{dst:3,value:42,next:41}, // 40
        Op::PatchNext{node:2,target:3,next:30}, // 41
        Op::Const{dst:2,value:20,next:43}, // 42
        Op::Read{dst:3,address:2,next:44}, // 43
        Op::Branch{test:3,nonzero:45,zero:51}, // 44
        Op::Const{dst:2,value:24,next:46}, // 45
        Op::Read{dst:3,address:2,next:47}, // 46
        Op::Branch{test:3,nonzero:48,zero:51}, // 47
        Op::Const{dst:2,value:2,next:49}, // 48
        Op::Const{dst:3,value:30,next:50}, // 49
        Op::PatchNext{node:2,target:3,next:30}, // 50
        Op::Const{dst:2,value:2,next:52}, // 51
        Op::Const{dst:3,value:35,next:53}, // 52
        Op::PatchNext{node:2,target:3,next:54}, // 53
        Op::Const{dst:2,value:21,next:55}, // 54
        Op::Read{dst:1,address:2,next:56}, // 55
        Op::Const{dst:3,value:1,next:57}, // 56
        Op::Add{dst:1,lhs:1,rhs:3,next:58}, // 57
        Op::Write{address:2,value:1,next:59}, // 58
        Op::Const{dst:3,value:4,next:60}, // 59
        Op::Sub{dst:3,lhs:1,rhs:3,next:61}, // 60
        Op::Branch{test:3,nonzero:21,zero:62}, // 61
        Op::Const{dst:2,value:22,next:63}, // 62
        Op::Read{dst:3,address:2,next:64}, // 63
        Op::Branch{test:3,nonzero:71,zero:65}, // 64
        Op::Const{dst:1,value:-3,next:66}, // 65
        Op::Const{dst:2,value:21,next:67}, // 66
        Op::Write{address:2,value:1,next:68}, // 67
        Op::Const{dst:1,value:1,next:69}, // 68
        Op::Const{dst:2,value:22,next:70}, // 69
        Op::Write{address:2,value:1,next:21}, // 70
        Op::Halt, // 71
    ]
}

fn native_kind(kind:&EventKind)->Value{
    match kind{
        EventKind::Executed=>json!({"type":"executed"}),
        EventKind::CellRead{key,value}=>json!({"type":"read","key":key,"value":value}),
        EventKind::CellWritten{key,value}=>json!({"type":"write","key":key,"value":value}),
        EventKind::ExecutableEdgePatched{node,before,after}=>
            json!({"type":"patch","node":node,"before":before,"after":after}),
        EventKind::NativeCodeEditSuppressed{opcode_tag}=>
            json!({"type":"suppressed","tag":opcode_tag}),
        EventKind::Halted=>json!({"type":"halt"}),
        EventKind::Trapped(m)=>json!({"type":"trap","message":m}),
        _=>panic!("B51c unsupported guest event {kind:?}"),
    }
}
fn edge(op:&Op)->Option<usize>{
    match op{
        Op::Const{next,..}|Op::Add{next,..}|Op::Sub{next,..}
        |Op::Read{next,..}|Op::Write{next,..}|Op::PatchNext{next,..}
        |Op::CloneNode{next,..}=>Some(*next),
        _=>None,
    }
}
fn emit(out:&mut BufWriter<File>,value:Value){
    serde_json::to_writer(&mut *out,&value).unwrap();
    out.write_all(b"\n").unwrap();
}
fn cells(world:&World)->Value{
    json!([world.cell(7),world.cell(11),world.cell(20),world.cell(21),
        world.cell(22),world.cell(24),world.cell(30),world.cell(31)])
}
fn set_cell(out:&mut BufWriter<File>,world:&mut World,case:usize,
            key:i64,value:i64,reason:&str) {
    let before=world.total_compute_inventory().unwrap();
    world.intervene(Intervention::SetCell{key,value}).unwrap();
    let event=world.events().last().unwrap();
    assert_eq!(event.origin,Origin::External);
    assert_eq!(world.total_compute_inventory(),Some(before));
    emit(out,json!({"type":"external","case":case,"reason":reason,
        "key":key,"value":value,"tick":event.tick,
        "credits":world.remaining_credits(),"storage":world.occupied_units(),
        "cells":cells(world)}));
}
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
enum Arm{ True, BlindSuccess }
impl Arm{fn name(self)->&'static str {
    match self{Arm::True=>"TRUE",Arm::BlindSuccess=>"BLIND_SUCCESS"}
}}
fn run_actor(out:&mut BufWriter<File>,case:usize,s:i64,k:i64,arm:Arm)->Value{
    let program=founder();
    assert_eq!(program.len(),72);
    assert!(matches!(program[0],Op::Add{next:0,..}));
    assert!(matches!(program[1],Op::Sub{next:1,..}));
    assert!(matches!(program[33],Op::Read{next:33,..}));
    let mut world=World::with_strict_capacity(16,120,CAP)
        .with_native_executable_edit_trace();
    let mut process=Process::new(1,program,BUDGET as u64);
    process.pc=3; // seed controller; arithmetic templates0/1 are disconnected
    world.load(process).unwrap();
    emit(out,json!({"type":"case","case":case,"s":s,"k":k,
        "arm":arm.name(),"initial_len":72,"initial_credits":BUDGET,
        "initial_storage":world.occupied_units()}));
    let mut input=TRAIN[0];
    set_cell(out,&mut world,case,7,input,"initial_training");
    let mut training_outputs=0_usize;
    let mut feedback_count=0_usize;
    let mut accepted=false;
    let mut accept_step=None::<usize>;
    let mut chosen=None::<[i64;2]>;
    let mut heldout_outputs=0_usize;
    let mut correct=0_usize;
    let mut wrong=0_usize;
    let mut training_correct=0_usize;
    let mut training_wrong=0_usize;
    let mut patches=0_usize;
    let mut clones=0_usize;
    let mut paid=0_usize;
    let mut peak=world.occupied_units();
    while paid<BUDGET && !world.process(1).unwrap().stopped {
        let prior=world.total_compute_inventory().unwrap();
        let old_len=world.process(1).unwrap().code.nodes.len();
        assert!(world.step());
        paid+=1;
        assert_eq!(world.total_compute_inventory(),Some(prior-1));
        let event=world.events().last().unwrap().clone();
        assert_eq!(event.origin,Origin::Internal(1));
        assert_eq!(event.process,1);
        assert!(!event.pool_paid_attempt);
        let p=world.process(1).unwrap();
        assert_eq!(event.credits_after,p.credits);
        let grown=p.code.nodes.len()-old_len;
        clones+=grown;
        patches+=usize::from(matches!(event.kind,
            EventKind::ExecutableEdgePatched{..}));
        peak=peak.max(world.occupied_units());
        assert!(peak<=CAP && p.code.nodes.len()<=120);
        emit(out,json!({"type":"step","case":case,"step":paid,
            "tick":event.tick,"pc":event.pc,"pc_after":p.pc,
            "event":native_kind(&event.kind),"regs":p.registers,
            "credits":p.credits,"stopped":p.stopped,
            "code_len":p.code.nodes.len(),"growth":grown,
            "edge2":edge(&p.code.nodes[2]),
            "edge33":edge(&p.code.nodes[33]),
            "edge72":p.code.nodes.get(72).and_then(edge),
            "edge73":p.code.nodes.get(73).and_then(edge),
            "storage":world.occupied_units(),"cells":cells(&world)}));
        match event.kind {
            EventKind::CellWritten{key:11,value}=>{
                let expected=s*input+k;
                let truly_correct=value==expected;
                if accepted {
                    heldout_outputs+=1;
                    if truly_correct{correct+=1;}else{wrong+=1;}
                    input=HELD[heldout_outputs%HELD.len()];
                    set_cell(out,&mut world,case,7,input,"heldout_next");
                } else {
                    training_outputs+=1;
                    if truly_correct{training_correct+=1;}
                    else{training_wrong+=1;}
                    let feedback=if arm==Arm::True {
                        i64::from(truly_correct)
                    } else {1};
                    set_cell(out,&mut world,case,20,feedback,"post_output_bit");
                    feedback_count+=1;
                    input=TRAIN[training_outputs%TRAIN.len()];
                    set_cell(out,&mut world,case,7,input,"training_next");
                }
            },
            EventKind::ExecutableEdgePatched{node:2,before:42,after:30}=>{
                assert!(!accepted && training_outputs>=2);
                assert_eq!(training_outputs%2,0);
                accepted=true;
                accept_step=Some(paid);
                chosen=Some([world.cell(22).unwrap(),world.cell(21).unwrap()]);
                input=HELD[0];
                set_cell(out,&mut world,case,7,input,"retained_holdout_start");
            },
            _=>{},
        }
    }
    let p=world.process(1).unwrap();
    let result=json!({"type":"end","case":case,"s":s,"k":k,"arm":arm.name(),
        "paid":paid,"stopped":p.stopped,"credits":p.credits,
        "accepted":accepted,"accept_step":accept_step,"chosen":chosen,
        "training_outputs":training_outputs,"feedback_count":feedback_count,
        "training_correct":training_correct,"training_wrong":training_wrong,
        "holdout_outputs":heldout_outputs,"holdout_correct":correct,
        "holdout_wrong":wrong,"clones":clones,"patches":patches,
        "peak_storage":peak,"final_pc":p.pc,"final_regs":p.registers,
        "final_len":p.code.nodes.len(),"final_cells":cells(&world),
        "edge2":edge(&p.code.nodes[2]),"edge33":edge(&p.code.nodes[33])});
    emit(out,result.clone());
    result
}
fn static_oracle(s:i64,k:i64)->Value{
    let op=if s==1{Op::Add{dst:0,lhs:0,rhs:1,next:4}}
           else{Op::Sub{dst:0,lhs:1,rhs:0,next:4}};
    let program=vec![
        Op::Const{dst:1,value:k,next:1},
        Op::Const{dst:2,value:7,next:2},
        Op::Read{dst:0,address:2,next:3},
        op,
        Op::Const{dst:2,value:11,next:5},
        Op::Write{address:2,value:0,next:0},
    ];
    let mut w=World::with_strict_capacity(16,120,CAP);
    w.load(Process::new(1,program,BUDGET as u64)).unwrap();
    let mut input=HELD[0];
    w.intervene(Intervention::SetCell{key:7,value:input}).unwrap();
    let mut correct=0_usize;let mut wrong=0_usize;
    for _ in 0..BUDGET{
        assert!(w.step());
        let event=w.events().last().unwrap().clone();
        assert_eq!(event.origin,Origin::Internal(1));
        if let EventKind::CellWritten{key:11,value}=event.kind {
            if value==s*input+k{correct+=1;}else{wrong+=1;}
            input=HELD[(correct+wrong)%HELD.len()];
            w.intervene(Intervention::SetCell{key:7,value:input}).unwrap();
        }
    }
    json!({"s":s,"k":k,"paid":BUDGET,"correct":correct,"wrong":wrong,
        "stopped":w.process(1).unwrap().stopped})
}
fn disabled_control()->Value{
    let mut w=World::with_strict_capacity(16,120,CAP)
        .with_native_patchnext_disabled();
    let mut process=Process::new(1,founder(),BUDGET as u64);
    process.pc=3;
    w.load(process).unwrap();
    w.intervene(Intervention::SetCell{key:7,value:5}).unwrap();
    let n=w.run(BUDGET);
    let writes=w.events().iter().filter(|e|
        matches!((&e.origin,&e.kind),
            (Origin::Internal(1),EventKind::CellWritten{key:11,..}))
    ).count();
    json!({"paid":n,"writes":writes,"stopped":w.process(1).unwrap().stopped,
        "remaining_credits":w.remaining_credits()})
}
#[test]
fn b51c_paid_search_real_feedback_source_retention_and_native_controls(){
    let dir=std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../cl1b-ci-evidence");
    fs::create_dir_all(&dir).unwrap();
    let path=dir.join("b51c_native_steps.jsonl");
    let mut out=BufWriter::new(File::create(&path).unwrap());
    emit(&mut out,json!({"type":"meta","study":"B51c","version":1,
        "source_sha":std::env::var("GITHUB_SHA")
            .unwrap_or_else(|_|"local".into())}));
    let mut end_rows=Vec::new();
    let mut static_rows=Vec::new();
    for s in [-1_i64,1]{
        for k in [-3_i64,-2,2,3]{
            for arm in [Arm::True,Arm::BlindSuccess]{
                let id=end_rows.len();
                end_rows.push(run_actor(&mut out,id,s,k,arm));
            }
            static_rows.push(static_oracle(s,k));
        }
    }
    out.flush().unwrap();
    assert_eq!(end_rows.len(),16);
    assert_eq!(static_rows.len(),8);
    let disabled=disabled_control();
    eprintln!("B51C_NATIVE_END {}",serde_json::to_string(&end_rows).unwrap());
    eprintln!("B51C_STATIC {}",serde_json::to_string(&static_rows).unwrap());
    eprintln!("B51C_DISABLED {disabled}");
    // Implementation invariants only. Scientific hypotheses are evaluated
    // independently, WITHOUT rewriting or dropping negative observations.
    for row in &end_rows {
        assert!(row["paid"].as_u64().unwrap()<=BUDGET as u64);
        assert!(row["peak_storage"].as_u64().unwrap()<=CAP as u64);
        assert!(row["clones"].as_u64().unwrap()<=2);
        assert_eq!(row["training_outputs"].as_u64().unwrap()%2,0);
    }
    eprintln!("B51C_NATIVE trace={} -- independent per-event CIL interpreter required",path.display());
}
