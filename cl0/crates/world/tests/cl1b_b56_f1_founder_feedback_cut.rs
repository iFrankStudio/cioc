//! B56-F1 causal preflight: cutting inherited B51c feedback without replacing it.
//! Native paid negative controls; NOT a generated monitor or an F1 success.
//! All arms keep the original searcher and capacity. Two routing-cut arms
//! change one founder edge; two new information controls keep all source
//! edges identical but replace the externally supplied truth bit.
//! Frozen predictions before native execution: blind-success may prematurely
//! accept a wrong candidate; inverted truth should impede true retention.
//! Equal paid steps do NOT imply equal output opportunities or scheduling.
//! No privileged solver is injected.
use cl0_world::{EventKind,Intervention,Op,Origin,Process,World};
use serde_json::Value;
use std::fs::{self,File};
use std::io::{BufWriter,Write};
use serde_json::json;
#[path="cl1b_b51c_paid_search_retention.rs"]
mod b51c;

#[derive(Clone,Copy)]
enum Arm { Intact, CutOutput, CutFeedbackRead, BlindSuccess, InvertedFeedback }
impl Arm {
    fn name(self)->&'static str {match self {
        Self::Intact=>"intact",Self::CutOutput=>"cut_output_to_feedback",
        Self::CutFeedbackRead=>"cut_feedback_read_to_search",
        Self::BlindSuccess=>"intact_routing_blind_success",
        Self::InvertedFeedback=>"intact_routing_inverted_feedback"
    }}
}
const KEYS:[i64;8]=[7,11,20,21,22,24,30,31];
fn cells(w:&World)->Value {json!(KEYS.map(|k|w.cell(k)))}
fn event_kind(e:&EventKind)->Value{
    match e{
        EventKind::Executed=>json!(["exec"]),
        EventKind::CellRead{key,value}=>json!(["read",key,value]),
        EventKind::CellWritten{key,value}=>json!(["write",key,value]),
        EventKind::ExecutableEdgePatched{node,before,after}=>json!(["patch",node,before,after]),
        EventKind::NativeCodeEditSuppressed{opcode_tag}=>json!(["suppressed",opcode_tag]),
        EventKind::Halted=>json!(["halt"]),
        EventKind::Trapped(msg)=>json!(["trap",msg]),
        _=>panic!("B56-F1a source event unsupported: {e:?}"),
    }
}
fn emit(out:&mut BufWriter<File>,row:Value){
    serde_json::to_writer(&mut *out,&row).unwrap();
    out.write_all(b"\n").unwrap();
}
fn external(out:&mut BufWriter<File>,w:&mut World,id:usize,k:i64,v:i64,why:&str){
    let inv=w.total_compute_inventory();
    w.intervene(Intervention::SetCell{key:k,value:v}).unwrap();
    assert_eq!(w.total_compute_inventory(),inv);
    let e=w.events().last().unwrap();
    assert_eq!(e.origin,Origin::External);
    emit(out,json!(["ext",id,e.tick,k,v,why,w.remaining_credits(),w.occupied_units(),cells(w)]));
}
fn trial(arm:Arm,id:usize,out:&mut BufWriter<File>)->serde_json::Value {
    let mut code=b51c::founder();
    assert_eq!(code.len(),72);
    assert!(matches!(code[2],Op::Write{address:2,value:0,next:35}));
    assert!(matches!(code[36],Op::Read{dst:3,address:2,next:37}));
    match arm {
        Arm::Intact|Arm::BlindSuccess|Arm::InvertedFeedback=>{},
        Arm::CutOutput=>code[2]=Op::Write{address:2,value:0,next:30},
        Arm::CutFeedbackRead=>code[36]=Op::Read{dst:3,address:2,next:30},
    }
    let source=code.clone();
    let mut p=Process::new(1,code,1300);
    p.pc=3;
    let mut w=World::with_strict_capacity(16,180,300)
        .with_native_executable_edit_trace();
    w.load(p).expect("all five founders must fit strict capacity");
    emit(out,json!(["case",id,arm.name(),source,1300,w.occupied_units()]));
    external(out,&mut w,id,7,5,"initial_training_input");
    let mut paid=0_usize;
    let mut writes=0_usize;
    let mut feedback_reads=0_usize;
    let mut accepted=0_usize;
    let mut correct=0_usize;
    let mut wrong=0_usize;
    let mut input=5_i64;
    let mut patches=0_usize;
    let mut final_trace=Vec::new();
    while paid<1300 && !w.process(1).unwrap().stopped {
        let before=w.total_compute_inventory().unwrap();
        assert!(w.step());
        paid+=1;
        assert_eq!(w.total_compute_inventory(),Some(before-1));
        let event=w.events().last().unwrap().clone();
        assert_eq!(event.origin,Origin::Internal(1));
        let p=w.process(1).unwrap();
        assert_eq!(p.credits,event.credits_after);
        assert!(p.code.nodes.len()<=180&&w.occupied_units()<=300);
        let next=|i:usize|match p.code.nodes.get(i){
            Some(Op::Const{next,..})|Some(Op::Add{next,..})|Some(Op::Sub{next,..})
            |Some(Op::Read{next,..})|Some(Op::Write{next,..})
            |Some(Op::PatchNext{next,..})|Some(Op::PatchConst{next,..})
            |Some(Op::CloneNode{next,..})=>Some(*next),
            _=>None
        };
        emit(out,json!(["step",id,paid,event.tick,event.pc,p.pc,
            event_kind(&event.kind),p.registers,p.credits,p.stopped,
            p.code.nodes.len(),next(2),next(33),next(72),next(73),
            w.occupied_units(),cells(&w)]));
        if final_trace.len()<16 {
            final_trace.push(json!([event.pc,format!("{:?}",event.kind)]));
        }
        match event.kind {
            EventKind::CellWritten{key:11,value}=>{
                writes+=1;
                let good=value==input-2;
                if good {correct+=1;} else {wrong+=1;}
                // Source and routing are unchanged in these two controls. Only the
                // external bit is manipulated, with the same intervention cost.
                let feedback=match arm {
                    Arm::BlindSuccess=>1,
                    Arm::InvertedFeedback=>i64::from(!good),
                    _=>i64::from(good),
                };
                external(out,&mut w,id,20,feedback,"post_output_feedback");
                input=if writes%2==0{5}else{-3};
                external(out,&mut w,id,7,input,"next_training_or_service_input");
            },
            EventKind::CellRead{key:20,..}=>feedback_reads+=1,
            EventKind::ExecutableEdgePatched{node:2,before:42,after:30}=>accepted+=1,
            EventKind::ExecutableEdgePatched{..}=>patches+=1,
            _=>{}
        }
    }
    let outcome=json!({"arm":arm.name(),"paid":paid,"writes":writes,
        "feedback_reads":feedback_reads,"accepted":accepted,
        "correct":correct,"wrong":wrong,"other_patches":patches,
        "final_pc":w.process(1).unwrap().pc,
        "final_nodes":w.process(1).unwrap().code.nodes.len(),
        "peak_storage_at_end":w.occupied_units(),
        "native_trace_sample_len":final_trace.len()});
    emit(out,json!(["end",id,outcome,w.process(1).unwrap().code.nodes,cells(&w)]));
    outcome
}
#[test]
fn b56_f1_inherited_training_feedback_cut_native_falsifier() {
    let output=std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../cl1b-ci-evidence");
    fs::create_dir_all(&output).unwrap();
    let path=output.join("b56_f1a_native_feedback_ablation_steps.jsonl");
    let mut writer=BufWriter::new(File::create(&path).unwrap());
    emit(&mut writer,json!(["meta","B56-F1a",1,
        std::env::var("GITHUB_SHA").unwrap_or_else(|_|"local".to_owned()),
        1300,16,180,300]));
    let rows=[Arm::Intact,Arm::CutOutput,Arm::CutFeedbackRead,
        Arm::BlindSuccess,Arm::InvertedFeedback]
        .iter().enumerate().map(|(i,arm)|trial(*arm,i,&mut writer))
        .collect::<Vec<_>>();
    writer.flush().unwrap();
    assert_eq!(rows.len(),5);
    eprintln!("B56_F1A_NATIVE_TRACE {}",path.display());
    // Frozen prospective prediction: both information-corruption controls keep
    // the intact feedback path, but should impair true candidate retention.
    // This is a prediction, NOT an assertion of a favorable result.
    // Assert only physical invariants, never the desired scientific outcome.
    // The recorded counterfactual results decide whether feedback is required.
    for row in &rows {
        assert!(row["paid"].as_u64().unwrap()<=1300);
        assert!(row["final_nodes"].as_u64().unwrap()<=180);
        assert!(row["peak_storage_at_end"].as_u64().unwrap()<=300);
    }
    assert_eq!(rows[1]["feedback_reads"],0,
        "cutting the output edge should make feedback unreachable");
    eprintln!("B56_F1_FOUNDER_CUT_NATIVE {}",json!(rows));
}
