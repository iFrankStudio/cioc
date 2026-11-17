//! B56-F0: newly executable source-assembled CIL monitoring path from generic
//! instruction clones, under pre-registered finite 4,000/16/180/300 bounds.
//! Native actor really pays for CloneNode/PatchNext. F0 N56-A only, not
//! self-invented metapolicy, B3, B4 or life. Source authored by experimenter.
use cl0_world::{EventKind, Intervention, Op, Origin, Process, World};
use serde_json::{json,Value};
use std::fs::{self,File};
use std::io::{BufWriter,Write};
#[path="cl1b_b51c_paid_search_retention.rs"]
mod b51c;

const BUDGET:usize=4000;
const WINDOW:usize=24;
const SEEDS:[i64;4]=[101,211,307,401];
const TRAIN:[i64;2]=[5,-3];
const HELD:[i64;3]=[11,-7,0];
const KEYS:[i64;13]=[7,11,20,21,22,24,25,27,28,30,31,40,41];

fn build_assemble_founder()->Vec<Op>{
    let mut code=b51c::founder();
    assert_eq!(code.len(),72);
    // Researcher-authored bridge only, paid actual final candidate selection
    // will still PatchNext node2 target30 before visiting node72.
    code[50]=Op::PatchNext{node:2,target:3,next:72};
    code.resize(102,Op::Halt);
    code[72]=Op::Const{dst:2,value:27,next:73};
    code[73]=Op::Read{dst:3,address:2,next:74};
    code[74]=Op::Branch{test:3,nonzero:75,zero:30};
    code[75]=Op::Const{dst:2,value:35,next:76};
    code[76]=Op::CloneNode{dst:3,source:2,next:77};
    code[77]=Op::Const{dst:2,value:40,next:78};
    code[78]=Op::Write{address:2,value:3,next:79};
    code[79]=Op::Const{dst:2,value:36,next:80};
    code[80]=Op::CloneNode{dst:3,source:2,next:81};
    code[81]=Op::Const{dst:2,value:41,next:82};
    code[82]=Op::Write{address:2,value:3,next:83};
    code[83]=Op::Const{dst:2,value:40,next:84};
    code[84]=Op::Read{dst:1,address:2,next:85};
    code[85]=Op::Const{dst:2,value:41,next:86};
    code[86]=Op::Read{dst:3,address:2,next:87};
    code[87]=Op::PatchNext{node:1,target:3,next:88};
    code[88]=Op::Const{dst:2,value:41,next:89};
    code[89]=Op::Read{dst:1,address:2,next:90};
    code[90]=Op::Const{dst:3,value:98,next:91};
    code[91]=Op::PatchNext{node:1,target:3,next:92};
    code[92]=Op::Const{dst:2,value:40,next:93};
    code[93]=Op::Read{dst:3,address:2,next:94};
    code[94]=Op::Const{dst:1,value:2,next:95};
    code[95]=Op::PatchNext{node:1,target:3,next:30};
    code[98]=Op::Branch{test:3,nonzero:30,zero:99};
    code[99]=Op::Const{dst:2,value:25,next:100};
    code[100]=Op::Const{dst:3,value:1,next:101};
    code[101]=Op::Write{address:2,value:3,next:30};
    assert_eq!(code.len(),102);
    code
}
fn build_static_founder(periodic:bool)->Vec<Op>{
    // Privileged, distinctly authored control, never used for F0 assembly.
    let mut code=b51c::founder();
    assert_eq!(code.len(),72);
    code.resize(126,Op::Halt);
    if periodic{
        code[49]=Op::Const{dst:3,value:110,next:50};
    }else{
        code[49]=Op::Const{dst:3,value:120,next:50};
    }
    code[110]=Op::Const{dst:2,value:28,next:111};
    code[111]=Op::Read{dst:1,address:2,next:112};
    code[112]=Op::Const{dst:3,value:1,next:113};
    code[113]=Op::Add{dst:1,lhs:1,rhs:3,next:114};
    code[114]=Op::Write{address:2,value:1,next:115};
    code[115]=Op::Const{dst:3,value:-4,next:116};
    code[116]=Op::Add{dst:3,lhs:1,rhs:3,next:117};
    code[117]=Op::Branch{test:3,nonzero:30,zero:118};
    code[118]=Op::Const{dst:1,value:0,next:119};
    code[119]=Op::Write{address:2,value:1,next:120};
    code[120]=Op::Const{dst:2,value:20,next:121};
    code[121]=Op::Read{dst:3,address:2,next:122};
    code[122]=Op::Branch{test:3,nonzero:30,zero:123};
    code[123]=Op::Const{dst:2,value:25,next:124};
    code[124]=Op::Const{dst:3,value:1,next:125};
    code[125]=Op::Write{address:2,value:3,next:30};
    code
}
#[derive(Clone,Copy,Debug)]
enum Arm{Actor,Blind,Disabled,NoAssembly,StaticEvery,StaticFourth}
impl Arm{
    fn name(self)->&'static str{
        match self{
            Self::Actor=>"A_ACTOR_ASSEMBLE",Self::Blind=>"B_BLIND_ASSEMBLE",
            Self::Disabled=>"C_EDITS_DISABLED",Self::NoAssembly=>"D_NO_ASSEMBLY",
            Self::StaticEvery=>"E_STATIC_EVERY",Self::StaticFourth=>"F_STATIC_FOURTH",
        }
    }
    fn initial_source(self)->Vec<Op>{
        match self{
            Self::StaticEvery=>build_static_founder(false),
            Self::StaticFourth=>build_static_founder(true),
            _=>build_assemble_founder(),
        }
    }
}
fn cells(w:&World)->Value{
    json!(KEYS.map(|key|w.cell(key)))
}
fn kind(e:&EventKind)->Value{
    match e{
        EventKind::Executed=>json!(["exec"]),
        EventKind::CellRead{key,value}=>json!(["read",key,value]),
        EventKind::CellWritten{key,value}=>json!(["write",key,value]),
        EventKind::ExecutableEdgePatched{node,before,after}=>
            json!(["patch",node,before,after]),
        EventKind::NativeCodeEditSuppressed{opcode_tag}=>json!(["suppressed",opcode_tag]),
        EventKind::Halted=>json!(["halt"]),
        EventKind::Trapped(msg)=>json!(["trap",msg]),
        _=>panic!("B56 F0 unexpected paid CIL event {e:?}"),
    }
}
fn emit(out:&mut BufWriter<File>,v:Value){
    serde_json::to_writer(&mut *out,&v).unwrap();
    out.write_all(b"\n").unwrap();
}
fn inject(out:&mut BufWriter<File>,w:&mut World,id:usize,k:i64,v:i64,reason:&str){
    let original=w.total_compute_inventory();
    w.intervene(Intervention::SetCell{key:k,value:v}).unwrap();
    assert_eq!(w.total_compute_inventory(),original);
    let e=w.events().last().unwrap();
    assert_eq!(e.origin,Origin::External);
    emit(out,json!(["ext",id,e.tick,k,v,reason,w.remaining_credits(),
        w.occupied_units(),cells(w)]));
}
fn native_step(out:&mut BufWriter<File>,w:&mut World,id:usize,paid:&mut usize)->EventKind{
    let before=w.total_compute_inventory().unwrap();
    let p=w.process(1).unwrap();
    let old_graph=if matches!(p.code.nodes[p.pc],Op::CloneNode{..}|Op::PatchNext{..}|Op::PatchConst{..}){
        Some(p.code.nodes.clone())
    }else{None};
    assert!(w.step(),"B56 native step did not charge");
    *paid+=1;
    assert_eq!(w.total_compute_inventory(),Some(before-1));
    let e=w.events().last().unwrap().clone();
    assert_eq!(e.origin,Origin::Internal(1));
    let p=w.process(1).unwrap();
    assert_eq!(p.credits,e.credits_after);
    assert!(p.code.nodes.len()<=180&&w.occupied_units()<=300);
    let mut mutation=Vec::<Value>::new();
    if let Some(before)=old_graph{
        for (n,after) in p.code.nodes.iter().enumerate(){
            if before.get(n)!=Some(after){
                mutation.push(json!([n,before.get(n),after]));
            }
        }
    }
    emit(out,json!(["step",id,*paid,e.tick,e.pc,p.pc,kind(&e.kind),
        p.registers,p.credits,p.stopped,p.code.nodes.len(),
        w.occupied_units(),cells(w),mutation]));
    e.kind
}
fn task(seed_idx:usize,stage:usize)->(i64,i64){
    let old=if seed_idx%2==0{(1,-2)}else{(-1,-3)};
    let shifted=if seed_idx%2==0{(-1,3)}else{(1,2)};
    if stage==1{shifted}else{old}
}
fn run_case(out:&mut BufWriter<File>,id:usize,seed:i64,arm:Arm)->Value{
    let idx=SEEDS.iter().position(|x|*x==seed).unwrap();
    let code=arm.initial_source();
    let initial=code.len();
    let mut p=Process::new(1,code.clone(),BUDGET as u64);
    p.pc=3;
    let mut w=World::with_strict_capacity(16,180,300)
        .with_native_executable_edit_trace();
    if matches!(arm,Arm::Disabled){w=w.with_native_code_edits_disabled();}
    w.load(p).unwrap();
    emit(out,json!(["case",id,seed,arm.name(),[task(idx,0).0,task(idx,0).1],
        [task(idx,1).0,task(idx,1).1],initial,BUDGET,
        matches!(arm,Arm::Disabled),code,w.occupied_units()]));
    inject(out,&mut w,id,27,if matches!(arm,Arm::NoAssembly){0}else{1},
        "initial_arm_selector");
    inject(out,&mut w,id,7,TRAIN[0],"first_training_probe");
    let mut paid=0_usize;
    let mut input=TRAIN[0];
    let mut training=true;
    let mut train_count=0_usize;
    let mut service_count=0_usize;
    let mut right=[0_usize;3];
    let mut wrong=[0_usize;3];
    let mut right_all=0_usize;
    let mut wrong_all=0_usize;
    let mut first72_paid=None::<usize>;
    let mut native_source_edits=0_usize;
    let mut native_clones=0_usize;
    let mut monitor_reads=0_usize;
    let mut fail_marker=0_usize;
    let mut created_monitor=0_usize;
    let mut connected_writer=0_usize;
    let mut assembly_clone_first=None::<usize>;
    let mut assembly_clone_second=None::<usize>;
    let mut initial_accept_step=None::<usize>;
    let mut chosen=None::<(i64,i64)>;
    let mut trained_good=0_usize;
    let mut trained_wrong=0_usize;
    let mut peak=w.occupied_units();
    while paid<BUDGET&&!w.process(1).unwrap().stopped{
        let n_before=w.process(1).unwrap().code.nodes.len();
        let e=native_step(out,&mut w,id,&mut paid);
        let n_after=w.process(1).unwrap().code.nodes.len();
        peak=peak.max(w.occupied_units());
        if n_after>n_before{
            native_clones+=n_after-n_before;
            if n_after==105{assembly_clone_first=Some(104);}
            if n_after==106{assembly_clone_second=Some(105);}
        }
        if matches!(e,EventKind::ExecutableEdgePatched{..}){native_source_edits+=1;}
        match e{
            EventKind::CellWritten{key:11,value}=>{
                let stage=(service_count/WINDOW).min(2);
                let (sign,k)=task(idx,stage);
                let good=value==sign*input+k;
                if training{
                    train_count+=1;
                    if good{trained_good+=1;}else{trained_wrong+=1;}
                    let bit=if matches!(arm,Arm::Blind){1}else{i64::from(good)};
                    inject(out,&mut w,id,20,bit,"candidate_training_feedback");
                    input=TRAIN[train_count%TRAIN.len()];
                    inject(out,&mut w,id,7,input,"next_training_probe");
                }else{
                    if service_count<3*WINDOW{
                        if good{right[stage]+=1;}else{wrong[stage]+=1;}
                    }
                    if good{right_all+=1;}else{wrong_all+=1;}
                    service_count+=1;
                    if service_count==3*WINDOW{first72_paid=Some(paid);}
                    let bit=if matches!(arm,Arm::Blind){1}else{i64::from(good)};
                    inject(out,&mut w,id,20,bit,"service_feedback");
                    input=HELD[((service_count%WINDOW)+idx%HELD.len())%HELD.len()];
                    inject(out,&mut w,id,7,input,"next_service_input");
                }
            },
            EventKind::ExecutableEdgePatched{node:2,before:42,after}
                if (after==30&&matches!(arm,Arm::Actor|Arm::Blind|Arm::Disabled|Arm::NoAssembly))
                    ||(after==120&&matches!(arm,Arm::StaticEvery))
                    ||(after==110&&matches!(arm,Arm::StaticFourth))=>{
                assert!(training&&train_count>=2&&train_count%2==0);
                training=false;
                initial_accept_step=Some(paid);
                chosen=Some((w.cell(22).unwrap(),w.cell(21).unwrap()));
                input=HELD[idx%HELD.len()];
                inject(out,&mut w,id,7,input,"retained_service_start");
            },
            EventKind::ExecutableEdgePatched{node:2,before:30,after:104}=>{
                connected_writer+=1;
                let p=w.process(1).unwrap();
                assert!(matches!(p.code.nodes[104],Op::Const{value:20,next:105,..}));
                assert!(matches!(p.code.nodes[105],Op::Read{next:98,..}));
                created_monitor+=1;
            },
            EventKind::CellRead{key:20,..} if !training=>{monitor_reads+=1;},
            EventKind::CellWritten{key:25,value:1} if !training=>{fail_marker+=1;},
            _=>{}
        }
    }
    let missing=3*WINDOW-service_count.min(3*WINDOW);
    let success=right.iter().sum::<usize>();
    let errors=wrong.iter().sum::<usize>();
    assert_eq!(success+errors+missing,72);
    let p=w.process(1).unwrap();
    let row=json!({"case":id,"seed":seed,"arm":arm.name(),"paid":paid,
        "stopped":p.stopped,"initial_nodes":initial,"final_nodes":p.code.nodes.len(),
        "initial_credits":BUDGET,"peak_storage":peak,
        "training_outputs":train_count,"train_correct":trained_good,
        "train_wrong":trained_wrong,"retained_at":initial_accept_step,
        "chosen":chosen,"candidate_clones_and_monitor_clones":native_clones,
        "native_executable_edge_edits":native_source_edits,
        "assembly_clone_first":assembly_clone_first,
        "assembly_clone_second":assembly_clone_second,
        "created_monitor":created_monitor,"writer_connected":connected_writer,
        "paid_service_feedback_reads":monitor_reads,
        "failure_detected_marker":fail_marker,
        "stage_correct":right,"stage_wrong":wrong,"missing":missing,
        "service_writes":service_count,"total_correct":right_all,
        "total_wrong":wrong_all,"paid_to_first72":first72_paid,
        "historical_u":(success as f64-2.0*errors as f64-missing as f64)/(paid.max(1)as f64),
        "final_pc":p.pc,"final_regs":p.registers,
        "final_graph":p.code.nodes,"final_cells":cells(&w)});
    emit(out,json!(["end",id,row]));
    row
}
#[test]
fn b56_f0_native_paid_assembly_and_static_ablations(){
    let folder=std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../cl1b-ci-evidence");
    fs::create_dir_all(&folder).unwrap();
    let output=folder.join("b56_f0_native_steps.jsonl");
    let mut out=BufWriter::new(File::create(&output).unwrap());
    let manifest=std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../../cl1b/studies/B56-F0-source-generation-native-graph-manifest-v1.md");
    let bytes=fs::read(&manifest).unwrap();
    let digest=std::process::Command::new("sha256sum").arg(&manifest).output().unwrap();
    assert!(digest.status.success());
    let source_digest=String::from_utf8(digest.stdout).unwrap()
        .split_whitespace().next().unwrap().to_owned();
    assert_eq!(source_digest.len(),64);
    emit(&mut out,json!(["meta","B56-F0",1,
        std::env::var("GITHUB_SHA").unwrap_or_else(|_|"local".to_owned()),
        source_digest,bytes.len(),BUDGET,16,180,300]));
    let arms=[Arm::Actor,Arm::Blind,Arm::Disabled,Arm::NoAssembly,
        Arm::StaticEvery,Arm::StaticFourth];
    let mut rows=Vec::<Value>::new();
    for seed in SEEDS{
        for arm in arms{
            let id=rows.len();
            rows.push(run_case(&mut out,id,seed,arm));
        }
    }
    out.flush().unwrap();
    assert_eq!(rows.len(),24);
    let a=rows.iter().filter(|r|r["arm"]=="A_ACTOR_ASSEMBLE")
        .collect::<Vec<_>>();
    let c=rows.iter().filter(|r|r["arm"]=="C_EDITS_DISABLED")
        .collect::<Vec<_>>();
    let d=rows.iter().filter(|r|r["arm"]=="D_NO_ASSEMBLY")
        .collect::<Vec<_>>();
    assert!(a.iter().all(|r|r["created_monitor"]==1&&r["writer_connected"]==1
        &&r["paid_service_feedback_reads"].as_u64().unwrap()>0
        &&r["assembly_clone_first"]==104&&r["assembly_clone_second"]==105));
    assert!(c.iter().chain(d.iter()).all(|r|r["created_monitor"]==0));
    eprintln!("B56_F0_RESULTS {}",serde_json::to_string(&rows).unwrap());
}
