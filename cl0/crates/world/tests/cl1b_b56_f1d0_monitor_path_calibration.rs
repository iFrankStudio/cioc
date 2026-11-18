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


// B56-F1d1a predeclared correlated-receipt price gate. This separate isolated
// microcalibration does NOT append a new organism or demonstrate policy learning.
// Pre-registration: cl1b/studies/B56-F1d1a-native-correlated-receipt-price-microcalibration-manifest-v1.md
const F1D1_KEYS:[i64;8]=[11,20,25,26,27,28,29,40];

fn f1d1_source_contract(){
    let b53=include_str!("cl1b_b53_native_response_correlation.rs");
    for literal in [
        "Op::Const{dst:2,value:26,next:107}",
        "Op::Read{dst:1,address:2,next:108}",
        "Op::Sub{dst:1,lhs:1,rhs:3,next:111}",
        "Op::Branch{test:1,nonzero:121,zero:112}",
        "Op::Sub{dst:1,lhs:1,rhs:3,next:117}",
        "Op::Branch{test:1,nonzero:121,zero:118}",
        "Op::Read{dst:3,address:2,next:120}",
        "Op::Const{dst:2,value:29,next:122}",
    ]{assert!(b53.contains(literal),"B53 paid receipt-check source drift: {literal}");}
    assert_original_f1c_source();
}

fn f1d1_source(every:bool)->Vec<Op>{
    // Fixed 168 source nodes: <=180. PC70 relocates the neutral common
    // continuation because original PC110 is a genuine B53 Sub instruction.
    let mut c=vec![Op::Halt;168];
    // Exact B53 seven-instruction paid request serial generator, relocated
    // from PC99..105 into idle PC70..76 to avoid original F1c error marker.
    c[70]=Op::Const{dst:2,value:26,next:71};
    c[71]=Op::Read{dst:3,address:2,next:72};
    c[72]=Op::Const{dst:1,value:1,next:73};
    c[73]=Op::Add{dst:3,lhs:3,rhs:1,next:74};
    c[74]=Op::Const{dst:2,value:26,next:75};
    c[75]=Op::Write{address:2,value:3,next:76};
    c[76]=Op::Const{dst:2,value:11,next:2};
    c[2]=Op::Write{address:2,value:0,next:if every{106}else{136}};
    // F1c's ordinary paid failure marker (not an oracle for true task).
    c[99]=Op::Const{dst:2,value:25,next:100};
    c[100]=Op::Const{dst:3,value:1,next:101};
    c[101]=Op::Write{address:2,value:3,next:70};
    // F1c FOURTH periodic counter: relocate key28 to key40 because B53's
    // *simultaneously live* commit serial occupies key28.
    c[136]=Op::Const{dst:2,value:40,next:137};
    c[137]=Op::Read{dst:1,address:2,next:138};
    c[138]=Op::Const{dst:3,value:1,next:139};
    c[139]=Op::Add{dst:1,lhs:1,rhs:3,next:140};
    c[140]=Op::Write{address:2,value:1,next:141};
    c[141]=Op::Const{dst:3,value:-4,next:142};
    c[142]=Op::Add{dst:3,lhs:1,rhs:3,next:144};
    c[144]=Op::Branch{test:3,nonzero:70,zero:145};
    c[145]=Op::Const{dst:2,value:40,next:146};
    c[146]=Op::Const{dst:1,value:0,next:147};
    c[147]=Op::Write{address:2,value:1,next:148};
    c[148]=Op::Const{dst:2,value:20,next:106};
    // B53 paid receipt checker, including both ordinary comparisons,
    // no hidden privilege; only terminal destinations are retargeted
    // to common PC70 and F1c's ordinary marker path99..101.
    c[106]=Op::Const{dst:2,value:26,next:107};
    c[107]=Op::Read{dst:1,address:2,next:108};
    c[108]=Op::Const{dst:2,value:27,next:109};
    c[109]=Op::Read{dst:3,address:2,next:110};
    c[110]=Op::Sub{dst:1,lhs:1,rhs:3,next:111};
    c[111]=Op::Branch{test:1,nonzero:121,zero:112};
    c[112]=Op::Const{dst:2,value:28,next:113};
    c[113]=Op::Read{dst:3,address:2,next:114};
    c[114]=Op::Const{dst:2,value:26,next:115};
    c[115]=Op::Read{dst:1,address:2,next:116};
    c[116]=Op::Sub{dst:1,lhs:1,rhs:3,next:117};
    c[117]=Op::Branch{test:1,nonzero:121,zero:118};
    c[118]=Op::Const{dst:2,value:20,next:119};
    c[119]=Op::Read{dst:3,address:2,next:120};
    c[120]=Op::Branch{test:3,nonzero:70,zero:99};
    c[121]=Op::Const{dst:2,value:29,next:122};
    c[122]=Op::Const{dst:3,value:1,next:123};
    c[123]=Op::Write{address:2,value:3,next:70};
    c
}

fn f1d1_cells(w:&World)->Value { json!(F1D1_KEYS.map(|k|w.cell(k))) }
fn f1d1_step(out:&mut BufWriter<File>,w:&mut World,id:usize,paid:&mut usize)->EventKind{
    let before=w.total_compute_inventory().unwrap();
    assert!(w.step(),"F1d1a native source did not make progress");
    *paid+=1;
    let e=w.events().last().unwrap().clone();
    let p=w.process(1).unwrap();
    assert_eq!(e.origin,Origin::Internal(1));
    assert_eq!(w.total_compute_inventory(),Some(before-1));
    assert_eq!(p.credits,e.credits_after);
    assert!(!p.stopped);
    assert!(p.code.nodes.len()<=180&&w.occupied_units()<=300);
    emit(out,json!(["f1d1_step",id,*paid,e.tick,e.pc,p.pc,event(&e.kind),
        p.registers,p.credits,p.stopped,p.code.nodes.len(),w.occupied_units(),
        f1d1_cells(w)]));
    e.kind
}
fn f1d1_external(out:&mut BufWriter<File>,w:&mut World,
                 id:usize,n:usize,key:i64,value:i64){
    let before=w.total_compute_inventory();
    w.intervene(Intervention::SetCell{key,value}).unwrap();
    assert_eq!(w.total_compute_inventory(),before);
    let e=w.events().last().unwrap();
    assert_eq!(e.origin,Origin::External);
    assert!(w.occupied_units()<=300);
    emit(out,json!(["f1d1_ext",id,n,e.tick,key,value,w.remaining_credits(),
        w.occupied_units(),f1d1_cells(w)]));
}
fn f1d1_trial(out:&mut BufWriter<File>,id:usize,
               every:bool,mode:usize,mask:usize)->Value{
    let source=f1d1_source(every);
    let mut proc=Process::new(1,source.clone(),500);
    proc.pc=70;
    let mut w=World::with_strict_capacity(16,180,300);
    w.load(proc).expect("F1d1a unchanged strict resources");
    emit(out,json!(["f1d1_case",id,if every{"EVERY"}else{"FOURTH"},
        mode,mask,500,16,180,300,source,w.occupied_units()]));
    let mut paid=0_usize;
    let mut costs=Vec::new();
    let mut sampled=0_usize;
    let mut rejections=0_usize;
    let mut false_current_accepted=0_usize;
    let mut nonce_writes=0_usize;
    let mut service_writes=0_usize;
    for n in 0..4{
        assert_eq!(w.process(1).unwrap().pc,70);
        let service_start=paid;
        let mut nonce_count=0;
        for j in 0..8{
            let kind=f1d1_step(out,&mut w,id,&mut paid);
            if j==5{
                assert!(matches!(kind,EventKind::CellWritten{key:26,..}));
                nonce_count+=1;nonce_writes+=1;
                assert_eq!(w.cell(26),Some((n+1)as i64));
            }
            if j==7{
                assert!(matches!(kind,EventKind::CellWritten{key:11,value:0}));
                service_writes+=1;
            }
        }
        assert_eq!(nonce_count,1);
        assert_eq!(paid-service_start,8);
        let nonce=(n+1)as i64;
        let truth=if (mask>>n)&1==1{1_i64}else{0_i64};
        let (bit,echo,commit)=match mode{
            0=>(truth,nonce,nonce),     // FRESH
            1=>(1,nonce-1,nonce-1),    // REPLAY_OLD
            2=>(1,nonce,nonce-1),      // TORN_COMMIT
            3=>(1,nonce,nonce),        // FORGED_CURRENT
            _=>unreachable!()
        };
        f1d1_external(out,&mut w,id,n,20,bit);
        f1d1_external(out,&mut w,id,n,27,echo);
        f1d1_external(out,&mut w,id,n,28,commit);
        let start=paid;
        let mut pcs=Vec::new();
        let mut this_reads=0_usize;
        let mut this_rejected=0_usize;
        while w.process(1).unwrap().pc!=70{
            let pc=w.process(1).unwrap().pc;
            assert!(pcs.len()<35,"F1d1a bounded native paid checker path");
            let e=f1d1_step(out,&mut w,id,&mut paid);
            if matches!(e,EventKind::CellRead{key:20,..}){this_reads+=1;sampled+=1;}
            if matches!(e,EventKind::CellWritten{key:29,value:1}){this_rejected+=1;rejections+=1;}
            pcs.push(pc);
        }
        let sample=every||n==3;
        let prefix=if every{0}else if n==3{12}else{8};
        let checker=if !sample{0}else{
            match mode{0=>15+if truth==0{3}else{0},1=>9,2=>15,3=>15,_=>unreachable!()}
        };
        let expected=prefix+checker;
        assert_eq!(paid-start,expected,"B53/F1c source paid path charge");
        assert_eq!(this_reads,usize::from(sample&&(mode==0||mode==3)));
        assert_eq!(this_rejected,usize::from(sample&&(mode==1||mode==2)));
        if sample&&mode==3&&truth==0{
            false_current_accepted+=1;
        }
        costs.push(expected);
        emit(out,json!(["f1d1_segment",id,n,truth,bit,nonce,echo,commit,
            expected,this_reads,this_rejected,pcs,paid,w.remaining_credits(),
            f1d1_cells(&w)]));
    }
    let monitor_cost=costs.iter().sum::<usize>();
    let predicted=match (every,mode){
        (true,0)=>60+3*(0..4).filter(|n|(mask>>n)&1==0).count(),
        (false,0)=>51+if (mask>>3)&1==0{3}else{0},
        (true,1)=>36,(false,1)=>45,
        (true,2)=>60,(false,2)=>51,
        (true,3)=>60,(false,3)=>51,
        _=>unreachable!()
    };
    assert_eq!(monitor_cost,predicted);
    assert_eq!(paid,32+monitor_cost);
    assert_eq!(nonce_writes,4);
    assert_eq!(service_writes,4);
    let p=w.process(1).unwrap();
    let result=json!({"id":id,"policy":if every{"EVERY"}else{"FOURTH"},
        "mode":mode,"truth_mask":mask,"monitor_cost":monitor_cost,
        "shared_nonce_and_writer_cost":32,"total_paid":paid,
        "actual_paid_read20":sampled,"paid_reject_markers29":rejections,
        "forged_current_false_success_accepted":false_current_accepted,
        "segment_costs":costs,"nonce_writes":nonce_writes,
        "service_writes":service_writes,"end_pc":p.pc,
        "unused_credits":p.credits,"code_nodes":p.code.nodes.len(),
        "occupied_storage":w.occupied_units(),"cells":f1d1_cells(&w)});
    emit(out,json!(["f1d1_end",id,result]));
    result
}
#[test]
fn b56_f1d1a_correlated_receipts_native_price_gate(){
    f1d1_source_contract();
    let folder=std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../cl1b-ci-evidence");
    fs::create_dir_all(&folder).unwrap();
    let path=folder.join("b56_f1d1a_native_correlated_receipt_costs.jsonl");
    let mut out=BufWriter::new(File::create(&path).unwrap());
    let manifest=std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(
        "../../../cl1b/studies/B56-F1d1a-native-correlated-receipt-price-microcalibration-manifest-v1.md");
    let bytes=fs::read(&manifest).unwrap();
    let digest=std::process::Command::new("sha256sum").arg(&manifest).output().unwrap();
    assert!(digest.status.success());
    let hash=String::from_utf8(digest.stdout).unwrap()
        .split_whitespace().next().unwrap().to_owned();
    emit(&mut out,json!(["f1d1_meta","B56-F1d1a",1,
        std::env::var("GITHUB_SHA").unwrap_or_else(|_|"local".to_owned()),
        hash,bytes.len(),128,500,16,180,300]));
    let mut cases=Vec::new();
    for mode in 0..4{
        for mask in 0..16{
            for every in [true,false]{
                let id=cases.len();
                cases.push(f1d1_trial(&mut out,id,every,mode,mask));
            }
        }
    }
    out.flush().unwrap();
    assert_eq!(cases.len(),128);
    for mode in 0..4{
        for mask in 0..16{
            let k=(mode*16+mask)*2;
            let every=cases[k]["monitor_cost"].as_u64().unwrap();
            let fourth=cases[k+1]["monitor_cost"].as_u64().unwrap();
            if mode==1{assert!(every<fourth,"old replay reverses price order");}
            else{assert!(every>fourth,"fresh/torn/forge has price frontier");}
        }
    }
    eprintln!("B56_F1D1A_NATIVE_PRICE_GATE {}",json!({
        "worlds":cases.len(),"four_genuine_service_writes_per_world":true,
        "correlation_checks_native_paid":true,
        "strict_resources_respected":true,
        "source_assembly_experiment":false,
        "learner_selected_policy":false,"b3_met":false,"b4_met":false}));
}
