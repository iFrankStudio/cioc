//! B56-F1 causal preflight: cutting inherited B51c feedback without replacing it.
//! Native paid negative controls; NOT a generated monitor or an F1 success.
//! Each arm keeps the same original searcher and capacity. Differences are
//! restricted to a single founder edge. No privileged solver is injected.
use cl0_world::{EventKind,Intervention,Op,Process,World};
use serde_json::json;
#[path="cl1b_b51c_paid_search_retention.rs"]
mod b51c;

#[derive(Clone,Copy)]
enum Arm { Intact, CutOutput, CutFeedbackRead }
impl Arm {
    fn name(self)->&'static str {match self {
        Self::Intact=>"intact",Self::CutOutput=>"cut_output_to_feedback",
        Self::CutFeedbackRead=>"cut_feedback_read_to_search"
    }}
}
fn trial(arm:Arm)->serde_json::Value {
    let mut code=b51c::founder();
    assert_eq!(code.len(),72);
    assert!(matches!(code[2],Op::Write{address:2,value:0,next:35}));
    assert!(matches!(code[36],Op::Read{dst:3,address:2,next:37}));
    match arm {
        Arm::Intact=>{},
        Arm::CutOutput=>code[2]=Op::Write{address:2,value:0,next:30},
        Arm::CutFeedbackRead=>code[36]=Op::Read{dst:3,address:2,next:30},
    }
    let mut p=Process::new(1,code,1300);
    p.pc=3;
    let mut w=World::with_strict_capacity(16,180,300)
        .with_native_executable_edit_trace();
    w.load(p).expect("all three founders must fit strict capacity");
    w.intervene(Intervention::SetCell{key:7,value:5}).unwrap();
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
        if final_trace.len()<16 {
            final_trace.push(json!([event.pc,format!("{:?}",event.kind)]));
        }
        match event.kind {
            EventKind::CellWritten{key:11,value}=>{
                writes+=1;
                let good=value==input-2;
                if good {correct+=1;} else {wrong+=1;}
                w.intervene(Intervention::SetCell{key:20,value:i64::from(good)}).unwrap();
                input=if writes%2==0{5}else{-3};
                w.intervene(Intervention::SetCell{key:7,value:input}).unwrap();
            },
            EventKind::CellRead{key:20,..}=>feedback_reads+=1,
            EventKind::ExecutableEdgePatched{node:2,before:42,after:30}=>accepted+=1,
            EventKind::ExecutableEdgePatched{..}=>patches+=1,
            _=>{}
        }
    }
    json!({"arm":arm.name(),"paid":paid,"writes":writes,
        "feedback_reads":feedback_reads,"accepted":accepted,
        "correct":correct,"wrong":wrong,"other_patches":patches,
        "final_pc":w.process(1).unwrap().pc,
        "final_nodes":w.process(1).unwrap().code.nodes.len(),
        "peak_storage_at_end":w.occupied_units(),
        "initial_trace_sample":final_trace})
}
#[test]
fn b56_f1_inherited_training_feedback_cut_native_falsifier() {
    let rows=[Arm::Intact,Arm::CutOutput,Arm::CutFeedbackRead]
        .map(trial);
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
