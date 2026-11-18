//! B56-F1d1d: a NEW, researcher-authored CIL assembler founder under the
//! unchanged strict world. Paid GraphEncode can create Branch(test=1) absent
//! from its parent source. GraphLaunch creates PID2: NOT CL-1B same-PID origin.
use cl0_world::{EventKind,Op,Origin,Process,World};
use serde_json::{json,Value};
use std::fs::{self,File};
use std::io::{BufWriter,Write};

const CREDITS:u64=1000;
const CHILD_CREDITS:i64=100;
const DESCRIPTORS:[[i64;5];7]=[
    [0,1,0,1,0],   // child node0: Const r1 := ZERO/ONE, next1
    [5,1,2,3,0],   // child node1: genuinely GENERATED Branch(test r1)
    [0,3,1,4,0],   // child node2: nonzero -> r3=1
    [0,3,0,4,0],   // child node3: zero -> r3=0
    [0,2,11,5,0],  // child node4: address register2 = 11
    [4,2,3,6,0],   // child node5: Write key11 = r3
    [19,0,0,0,0],  // child node6: Halt
];

#[derive(Copy,Clone)]
enum Arm{BranchZero,BranchOne,NoEncode}
impl Arm{
    fn name(self)->&'static str{match self{
        Self::BranchZero=>"BRANCH_ZERO",
        Self::BranchOne=>"BRANCH_ONE",
        Self::NoEncode=>"NO_ENCODE",
    }}
    fn bit(self)->i64{if matches!(self,Self::BranchOne){1}else{0}}
}
fn source(arm:Arm)->(Vec<Op>,Vec<[i64;5]>){
    let mut desc=DESCRIPTORS.to_vec();
    desc[0][2]=arm.bit(); // authored condition, never an autonomous choice
    let mut c=vec![Op::GraphNew{dst:0,next:1}];
    let mut first_encode=None;
    for words in &desc{
        for (i,word) in words.iter().enumerate(){
            let pc=c.len();
            c.push(Op::Const{dst:2,value:40+i as i64,next:pc+1});
            c.push(Op::Const{dst:3,value:*word,next:pc+2});
            c.push(Op::Write{address:2,value:3,next:pc+3});
        }
        let pc=c.len();
        c.push(Op::Const{dst:1,value:40,next:pc+1});
        if first_encode.is_none(){first_encode=Some(pc+1);}
        c.push(Op::GraphEncode{graph:0,address:1,dst:2,next:pc+2});
    }
    let pc=c.len();
    c.push(Op::Const{dst:1,value:CHILD_CREDITS,next:pc+1});
    c.push(Op::GraphLaunch{graph:0,credits:1,dst:2,next:pc+2});
    c.push(Op::Halt);
    assert_eq!(c.len(),123);
    assert!(c.iter().all(|op|!matches!(op,Op::Branch{..})));
    assert_eq!(first_encode,Some(17));
    if matches!(arm,Arm::NoEncode){c[17]=Op::Halt;}
    (c,desc)
}
fn emit(f:&mut BufWriter<File>,row:Value){
    serde_json::to_writer(&mut *f,&row).unwrap();
    f.write_all(b"\n").unwrap();
}
fn memory(w:&World)->Value{json!([11,40,41,42,43,44].map(|k|w.cell(k)))}
fn process(w:&World,id:u64)->Value{
    if let Some(p)=w.process(id){json!([p.pc,p.registers,p.credits,p.stopped,p.code.nodes.len()])}
    else{Value::Null}
}
fn case(out:&mut BufWriter<File>,id:usize,arm:Arm)->Value{
    let (program,descs)=source(arm);
    let mut w=World::with_strict_capacity(16,180,300);
    w.load(Process::new(1,program.clone(),CREDITS)).unwrap();
    emit(out,json!(["case",id,arm.name(),CREDITS,16,180,300,
        program,descs,w.occupied_units(),w.total_compute_inventory()]));
    let mut n=0_usize;let mut parent_paid=0;let mut child_paid=0;
    let mut writes=0;let mut encodes=0;let mut graph_created=0;
    let mut launches=0;let mut branch_executed=0;let mut result_writes=0;
    let mut peak_storage=w.occupied_units();
    loop{
        let before=w.total_compute_inventory().unwrap();
        if !w.step(){break;}
        n+=1;
        assert!(n<=220,"new-origin preflight unexpectedly looping");
        assert_eq!(w.total_compute_inventory(),Some(before-1),
            "GraphNew/GraphEncode/GraphLaunch must be genuinely charged");
        peak_storage=peak_storage.max(w.occupied_units());
        assert!(peak_storage<=300);
        let e=w.events().last().unwrap().clone();
        let pid=match e.origin{
            Origin::Internal(x)=>x,
            _=>panic!("unexpected external origin in entirely actor-paid native study")
        };
        if pid==1{parent_paid+=1;}else if pid==2{child_paid+=1;}
        else{panic!("new-origin preflight has wrong PID {pid}");}
        if let EventKind::CellWritten{key,value}=&e.kind{
            if *key>=40&&*key<=44 {assert_eq!(pid,1);writes+=1;}
            if *key==11 {
                assert_eq!(pid,2,"ONLY generated child may service");
                assert_eq!(*value,arm.bit());
                result_writes+=1;
            }
        }
        if matches!(e.kind,EventKind::GraphCreated{graph:1,owner:1}){
            assert_eq!(pid,1);graph_created+=1;
        }
        if let EventKind::GraphEncoded{
            graph:1,owner:1,new_node,descriptor_base:40,descriptor
        }=&e.kind{
            assert_eq!(pid,1);
            assert_eq!(descriptor,&descs[*new_node]);
            encodes+=1;
        }
        if let EventKind::GraphLaunched{
            source_graph:1,parent_process:1,child_process:2,
            transferred_credits:100,moved_nodes:7
        }=&e.kind{
            assert_eq!(pid,1);launches+=1;
            let child=w.process(2).unwrap();
            assert!(matches!(child.code.nodes[1],
                Op::Branch{test:1,nonzero:2,zero:3}),
                "native encoded novel Branch must have register1");
        }
        if pid==2 && e.pc==1 {
            assert!(matches!(w.process(2).unwrap().code.nodes[1],
                Op::Branch{test:1,nonzero:2,zero:3}));
            assert_eq!(w.process(2).unwrap().pc,if arm.bit()==1{2}else{3});
            branch_executed+=1;
        }
        let graph=w.graph_object(1).map(|g|json!(g.nodes));
        emit(out,json!(["step",id,n,e.tick,pid,e.pc,e.kind,
            process(&w,1),process(&w,2),w.total_compute_inventory(),
            w.occupied_units(),memory(&w),graph]));
    }
    assert_eq!(graph_created,1);
    if matches!(arm,Arm::NoEncode){
        assert_eq!((writes,encodes,launches,branch_executed,result_writes),
            (5,0,0,0,0));
        assert!(w.process(2).is_none());
        assert_eq!(w.cell(11),None);
    }else{
        assert_eq!((writes,encodes,launches,branch_executed,result_writes),
            (35,7,1,1,1));
        assert_eq!(w.cell(11),Some(arm.bit()));
        assert!(w.graph_object(1).is_none());
        assert_eq!(w.process(2).unwrap().code.nodes.len(),7);
        assert_eq!(w.process(2).unwrap().credits,94);
    }
    let result=json!({"id":id,"arm":arm.name(),
        "initial_parent_contains_Branch":false,
        "graph_created_by_pid1":graph_created,
        "descriptor_cell_writes_paid_by_pid1":writes,
        "graph_encoded_nodes_paid_by_pid1":encodes,
        "graph_launches_by_pid1":launches,
        "novel_Branch_test1_executed_by_pid2":branch_executed,
        "actual_pid2_service_writes":result_writes,
        "service_output11":w.cell(11),
        "paid_steps":n,"parent_paid":parent_paid,
        "child_paid":child_paid,"peak_total_storage":peak_storage,
        "final_total_credits":w.total_compute_inventory(),
        "PID1":process(&w,1),"PID2":process(&w,2),
        "same_pid_continuous_individual":false,
        "b3_met":false,"b4_met":false,"cyberlife":false});
    emit(out,json!(["end",id,result]));
    result
}
#[test]
fn b56_f1d1d_actor_paid_novel_branch_from_data_to_new_pid(){
    let p=std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../cl1b-ci-evidence");
    fs::create_dir_all(&p).unwrap();
    let mut out=BufWriter::new(File::create(
        p.join("b56_f1d1d_native_graph_encoded_branch.jsonl")).unwrap());
    let manifest=std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(
        "../../../cl1b/studies/B56-F1d1d-paid-graph-encode-novel-branch-structural-feasibility-manifest-v1.md");
    let bytes=fs::read(&manifest).unwrap();
    let digest=std::process::Command::new("sha256sum").arg(&manifest).output().unwrap();
    assert!(digest.status.success());
    let hash=String::from_utf8(digest.stdout).unwrap()
        .split_whitespace().next().unwrap().to_owned();
    emit(&mut out,json!(["meta","B56-F1d1d",1,
        std::env::var("GITHUB_SHA").unwrap_or_else(|_|"local".to_owned()),
        hash,bytes.len(),3,CREDITS,16,180,300]));
    let mut results=Vec::new();
    for arm in [Arm::BranchZero,Arm::BranchOne,Arm::NoEncode]{
        let id=results.len();results.push(case(&mut out,id,arm));
    }
    out.flush().unwrap();
    assert_eq!(results.len(),3);
    eprintln!("B56_F1D1D_GRAPH_OPCODE_ORIGIN {}",json!({
        "parent_source_Branch_test1_absent":true,
        "PID1_paid_encoded_graph_nodes":14,
        "PID2_executes_generated_Branch_test1":true,
        "no_encode_prevents_child_launch":true,
        "same_pid_evolution":false,
        "b3_met":false,"b4_met":false,"cyberlife":false
    }));
}
