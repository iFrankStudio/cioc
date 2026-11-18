#![recursion_limit="256"]
//! B56-F1d0 native VM microcalibration of the original F1c CIL monitor paths.
//! Paths are deliberately PREASSEMBLED; no paid composition / learning claim.
use cl0_world::{EventKind, Intervention, Op, Origin, Process, World};
use serde_json::{json,Value};
use std::fs::{self,File};
use std::io::{BufWriter,Write};

const CREDITS:u64=500;
const KEYS:[i64;4]=[11,20,25,28];

fn assert_original_f1c_source(){
    let s=include_str!("cl1b_b56_f1c_pilot_ordered_trial.rs");
    let requisite=[
        "code[98]=Op::Branch{test:3,nonzero:110,zero:99};",
        "code[99]=Op::Const{dst:2,value:25,next:100};",
        "code[100]=Op::Const{dst:3,value:1,next:101};",
        "code[101]=Op::Write{address:2,value:3,next:110};",
        "code[136]=Op::Const{dst:2,value:28,next:137};",
        "code[137]=Op::Read{dst:1,address:2,next:138};",
        "code[138]=Op::Const{dst:3,value:1,next:139};",
        "code[139]=Op::Add{dst:1,lhs:1,rhs:3,next:140};",
        "code[140]=Op::Write{address:2,value:1,next:141};",
        "code[141]=Op::Const{dst:3,value:-4,next:142};",
        "code[142]=Op::Add{dst:3,lhs:1,rhs:3,next:144};",
        "code[144]=Op::Branch{test:3,nonzero:110,zero:145};",
        "code[145]=Op::Const{dst:2,value:28,next:146};",
        "code[146]=Op::Const{dst:1,value:0,next:147};",
        "code[147]=Op::Write{address:2,value:1,next:148};",
        "code[148]=Op::Const{dst:2,value:20,next:30};",
        "code[75]=Op::Const{dst:2,value:35,next:76};",
        "code[79]=Op::Const{dst:2,value:36,next:80};",
        "code[87]=Op::PatchNext{node:1,target:3,next:88};",
        "code[91]=Op::PatchNext{node:1,target:3,next:92};",
    ];
    for line in requisite {assert!(s.contains(line),"native CIL source drift: {line}");}
    let b=include_str!("cl1b_b51c_paid_search_retention.rs");
    assert!(b.contains("Op::Const{dst:2,value:20,next:36}"));
    assert!(b.contains("Op::Read{dst:3,address:2,next:37}"));
}
fn monitor_source(every:bool)->Vec<Op>{
    let mut c=vec![Op::Halt;168];
    c[2]=Op::Write{address:2,value:0,next:if every{164}else{136}};
    // Researcher-defined neutral loop, identical in both arms and excluded from monitor cost.
    c[110]=Op::Const{dst:2,value:11,next:2};
    // Actual source-extracted F1c service monitor, including original paid-clone destinations.
    c[98]=Op::Branch{test:3,nonzero:110,zero:99};
    c[99]=Op::Const{dst:2,value:25,next:100};
    c[100]=Op::Const{dst:3,value:1,next:101};
    c[101]=Op::Write{address:2,value:3,next:110};
    c[136]=Op::Const{dst:2,value:28,next:137};
    c[137]=Op::Read{dst:1,address:2,next:138};
    c[138]=Op::Const{dst:3,value:1,next:139};
    c[139]=Op::Add{dst:1,lhs:1,rhs:3,next:140};
    c[140]=Op::Write{address:2,value:1,next:141};
    c[141]=Op::Const{dst:3,value:-4,next:142};
    c[142]=Op::Add{dst:3,lhs:1,rhs:3,next:144};
    c[144]=Op::Branch{test:3,nonzero:110,zero:145};
    c[145]=Op::Const{dst:2,value:28,next:146};
    c[146]=Op::Const{dst:1,value:0,next:147};
    c[147]=Op::Write{address:2,value:1,next:148};
    // The source's real paid patch changed 148.next 30→166 in F1c.
    c[148]=Op::Const{dst:2,value:20,next:166};
    c[164]=Op::Const{dst:2,value:20,next:165};
    c[165]=Op::Read{dst:3,address:2,next:98};
    c[166]=Op::Const{dst:2,value:20,next:167};
    c[167]=Op::Read{dst:3,address:2,next:98};
    c
}
fn emit(w:&mut BufWriter<File>,row:Value){
    serde_json::to_writer(&mut *w,&row).unwrap();
    w.write_all(b"\n").unwrap();
}
fn cells(w:&World)->Value{json!(KEYS.map(|k|w.cell(k)))}
fn event(e:&EventKind)->Value{
    match e{
        EventKind::Executed=>json!(["exec"]),
        EventKind::CellRead{key,value}=>json!(["read",key,value]),
        EventKind::CellWritten{key,value}=>json!(["write",key,value]),
        EventKind::ExecutableEdgePatched{node,before,after}=>json!(["patch",node,before,after]),
        EventKind::NativeCodeEditSuppressed{opcode_tag}=>json!(["suppressed",opcode_tag]),
        EventKind::Trapped(msg)=>json!(["trap",msg]),
        EventKind::Halted=>json!(["halt"]),
        _=>panic!("F1d0 unsupported native CIL event: {e:?}")
    }
}
fn next(out:&mut BufWriter<File>,w:&mut World,id:usize,paid:&mut usize)->EventKind{
    let inv=w.total_compute_inventory().unwrap();
    assert!(w.step(),"F1d0 native CIL did not make progress");
    *paid+=1;
    assert_eq!(w.total_compute_inventory(),Some(inv-1),"native CIL credit debit");
    let e=w.events().last().unwrap().clone();
    assert_eq!(e.origin,Origin::Internal(1));
    let p=w.process(1).unwrap();
    assert!(!p.stopped);
    assert_eq!(p.credits,e.credits_after);
    assert!(p.code.nodes.len()<=180&&w.occupied_units()<=300);
    emit(out,json!(["step",id,*paid,e.tick,e.pc,p.pc,event(&e.kind),
        p.registers,p.credits,p.stopped,p.code.nodes.len(),w.occupied_units(),cells(w)]));
    e.kind
}
fn external(out:&mut BufWriter<File>,w:&mut World,id:usize,key:i64,value:i64){
    let prev=w.total_compute_inventory();
    w.intervene(Intervention::SetCell{key,value}).unwrap();
    assert_eq!(w.total_compute_inventory(),prev);
    let e=w.events().last().unwrap();
    assert_eq!(e.origin,Origin::External);
    emit(out,json!(["ext",id,e.tick,key,value,"feedback_after_real_service_write",
        w.remaining_credits(),w.occupied_units(),cells(w)]));
}
fn trial(out:&mut BufWriter<File>,id:usize,every:bool,mask:usize)->Value{
    let source=monitor_source(every);
    let mut proc=Process::new(1,source.clone(),CREDITS);
    proc.pc=110;
    let mut w=World::with_strict_capacity(16,180,300);
    w.load(proc).expect("strict unchanged resource limits");
    emit(out,json!(["case",id,if every{"EVERY"}else{"FOURTH"},mask,CREDITS,
        source,w.occupied_units()]));
    let mut paid=0_usize;
    let mut reads=0_usize;
    let mut failures_marked=0_usize;
    let mut costs=Vec::new();
    let mut segment_pcs=Vec::new();
    for n in 0..4{
        assert_eq!(w.process(1).unwrap().pc,110,"neutral output-loop boundary");
        assert!(matches!(next(out,&mut w,id,&mut paid),EventKind::Executed));
        assert!(matches!(next(out,&mut w,id,&mut paid),
            EventKind::CellWritten{key:11,value:0}));
        let bit:i64=if (mask>>n)&1==1{1}else{0};
        external(out,&mut w,id,20,bit);
        let start=paid;
        let mut pcs=Vec::new();
        while w.process(1).unwrap().pc!=110 {
            let pc=w.process(1).unwrap().pc;
            assert!(pcs.len()<40,"bounded native monitor CIL");
            let e=next(out,&mut w,id,&mut paid);
            if matches!(e,EventKind::CellRead{key:20,..}){reads+=1;}
            if matches!(e,EventKind::CellWritten{key:25,value:1}){failures_marked+=1;}
            pcs.push(pc);
        }
        let charge=paid-start;
        let want=if every{if bit==0{6}else{3}}
            else if n==3{if bit==0{18}else{15}}else{8};
        assert_eq!(charge,want,"source-extracted monitored path debit");
        costs.push(charge);
        segment_pcs.push(pcs);
        emit(out,json!(["segment",id,n,bit,charge,costs.iter().sum::<usize>(),
            reads,failures_marked,w.remaining_credits(),w.occupied_units(),
            cells(&w)]));
    }
    let monitor_cost=costs.iter().sum::<usize>();
    let expected=if every{12+3*(0..4).filter(|i|(mask>>i)&1==0).count()}
                 else{39+if (mask>>3)&1==0{3}else{0}};
    assert_eq!(monitor_cost,expected);
    assert_eq!(reads,if every{4}else{1});
    assert_eq!(paid,8+monitor_cost);
    let p=w.process(1).unwrap();
    let result=json!({"id":id,"policy":if every{"EVERY"}else{"FOURTH"},
        "mask":mask,"bits":(0..4).map(|i|((mask>>i)&1)as i64).collect::<Vec<_>>(),
        "paid_including_neutral_harness":paid,"harness_steps":8,
        "monitor_cost":monitor_cost,"segment_costs":costs,
        "monitor_pcs":segment_pcs,"feedback_reads":reads,
        "failure_markers":failures_marked,
        "unused_paid_credits":p.credits,
        "final_pc":p.pc,"final_regs":p.registers,
        "final_nodes":p.code.nodes.len(),
        "final_graph":p.code.nodes,"final_cells":cells(&w),
        "final_storage":w.occupied_units()});
    emit(out,json!(["end",id,result]));
    result
}
#[test]
fn b56_f1d0_exhaustive_native_paid_source_monitor_costs(){
    assert_original_f1c_source();
    let folder=std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../cl1b-ci-evidence");
    fs::create_dir_all(&folder).unwrap();
    let path=folder.join("b56_f1d0_native_monitor_path_costs.jsonl");
    let mut out=BufWriter::new(File::create(&path).unwrap());
    let manifest=std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../../cl1b/studies/B56-F1d0-native-monitor-path-price-calibration-manifest-v1.md");
    let bytes=fs::read(&manifest).unwrap();
    let digest=std::process::Command::new("sha256sum").arg(&manifest).output().unwrap();
    assert!(digest.status.success());
    let hash=String::from_utf8(digest.stdout).unwrap()
        .split_whitespace().next().unwrap().to_owned();
    emit(&mut out,json!(["meta","B56-F1d0",1,
        std::env::var("GITHUB_SHA").unwrap_or_else(|_|"local".to_owned()),
        hash,bytes.len(),32,500,16,180,300]));
    let mut cases=Vec::new();
    for mask in 0..16{
        for every in [true,false]{
            let id=cases.len();
            cases.push(trial(&mut out,id,every,mask));
        }
    }
    out.flush().unwrap();
    assert_eq!(cases.len(),32);
    for mask in 0..16{
        let a=cases[mask*2]["monitor_cost"].as_u64().unwrap();
        let b=cases[mask*2+1]["monitor_cost"].as_u64().unwrap();
        assert!(a<b);
    }
    eprintln!("B56_F1D0_NATIVE_MONITOR_PRICE {}",json!({
        "worlds":cases.len(),"paid_monitor_instructions_every":cases.iter()
          .step_by(2).map(|v|v["monitor_cost"].as_u64().unwrap()).sum::<u64>(),
        "paid_monitor_instructions_fourth":cases.iter()
          .skip(1).step_by(2).map(|v|v["monitor_cost"].as_u64().unwrap()).sum::<u64>(),
        "all16_feedback_masks_every_cheaper":true,
        "source_assembly_experiment":false,
        "b3_met":false,"b4_met":false}));
}
