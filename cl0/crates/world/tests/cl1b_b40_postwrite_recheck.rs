//! CL-1B B40: same actor re-reads actual environmental input AFTER
//! writing output, before considering a paid native PatchNext. This is a
//! researcher-authored temporal consistency repair, NOT a goal learner.
use cl0_world::{EventKind,Intervention,Op,Origin,Process,World};
#[path="cl1b_b38_functional_readback.rs"]
mod b38_source;
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
enum Arm{Readback,Recheck,StaticCorrect}
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
enum Arrival{BeforeRead,AfterRead}

fn refreshed_founder()->Vec<Op>{
 let mut code=b38_source::founder(b38_source::Arm::Readback);
 assert_eq!(code.len(),12);
 match &mut code[5]{
  Op::Read{next,..}=>*next=12,
  _=>panic!("B40 must redirect B38's actual output read, not a host oracle")
 }
 code.extend([
  Op::Const{dst:2,value:7,next:13},
  Op::Read{dst:0,address:2,next:14},
  Op::Const{dst:2,value:11,next:6},
 ]);
 assert_eq!(code.len(),15);
 code
}
fn founder(arm:Arm)->Vec<Op>{
 match arm{
  Arm::Readback=>b38_source::founder(b38_source::Arm::Readback),
  Arm::Recheck=>refreshed_founder(),
  Arm::StaticCorrect=>b38_source::founder(b38_source::Arm::StaticCorrect)
 }
}
#[derive(Debug)]
struct Observation{
 arm:Arm,old:i64,new:i64,arrival:Arrival,initial_sample:i64,
 first_output:i64,current_input:i64,check_pc:usize,judgement_correct:Option<bool>,
 reads_at_first_check:Vec<i64>,paid:usize,
 correct_writes:usize,wrong_writes:usize,source_edits:usize,final_edge:usize,
}
fn step(w:&mut World)->bool{
 let inv=w.total_compute_inventory().unwrap();
 let n=w.events().len();
 let did=w.step();
 assert_eq!(w.total_compute_inventory(),Some(inv-u64::from(did)));
 assert_eq!(w.events().len(),n+usize::from(did));
 if did{
  assert_eq!(w.events().last().unwrap().origin,Origin::Internal(1));
  assert!(!matches!(w.events().last().unwrap().kind,EventKind::Trapped(_)|EventKind::Halted));
 }
 did
}
fn edge(w:&World,arm:Arm)->usize{
 let node=if arm==Arm::StaticCorrect{2}else{3};
 match &w.process(1).unwrap().code.nodes[node]{
  Op::Const{next,..}=>*next,_=>panic!("B40 executable output control not a Const")
 }
}
fn run(arm:Arm,old:i64,new:i64,arrival:Arrival)->Observation{
 let mut w=World::with_strict_capacity(16,64,128)
  .with_native_executable_edit_trace();
 w.load(Process::new(1,founder(arm),96)).unwrap();
 w.intervene(Intervention::SetCell{key:7,value:old}).unwrap();
 if arrival==Arrival::BeforeRead{
  w.intervene(Intervention::SetCell{key:7,value:new}).unwrap();
 }
 let mut first_sample=None;
 for _ in 0..8{
  assert!(step(&mut w));
  if let EventKind::CellRead{key:7,value}=&w.events().last().unwrap().kind{
   first_sample=Some(*value);break;
  }
 }
 let initial_sample=first_sample.expect("native CIL source must sample input");
 assert_eq!(initial_sample,if arrival==Arrival::BeforeRead{new}else{old});
 if arrival==Arrival::AfterRead{
  w.intervene(Intervention::SetCell{key:7,value:new}).unwrap();
 }
 let first_after_update=w.events().len();
 let mut check_pc=None;
 let mut first_output=None;
 for _ in 0..20{
  assert!(step(&mut w));
  let e=w.events().last().unwrap();
  if (arm!=Arm::StaticCorrect&&e.pc==7)||
      (arm==Arm::StaticCorrect&&matches!(e.kind,EventKind::CellWritten{key:11,..})){
   check_pc=Some(w.process(1).unwrap().pc);
   first_output=w.cell(11);
   break;
  }
 }
 let check_pc=check_pc.expect("B40 first service output / feedback checkpoint must execute");
 let first_output=first_output.unwrap();
 assert_eq!(edge(&w,arm),if arm==Arm::StaticCorrect{3}else{4},
  "native edge cannot change before first B40 judgement");
 let real_current_input=w.cell(7).unwrap();
 assert_eq!(real_current_input,new);
 let judgement_correct=if arm==Arm::StaticCorrect{None}else{
  assert!(check_pc==1||check_pc==8);
  Some((check_pc==1)==(first_output==new))
 };
 let reads_at_first_check=w.events().iter().filter_map(|e|match (&e.origin,&e.kind){
  (Origin::Internal(1),EventKind::CellRead{key:7,value})=>Some(*value),
  _=>None,
 }).collect::<Vec<_>>();
 let charged_so_far=w.events().iter().filter(|e|matches!(e.origin,Origin::Internal(1))).count();
 assert_eq!(w.run(96-charged_so_far),96-charged_so_far);
 assert_eq!(w.total_compute_inventory(),Some(0));
 assert!(!w.process(1).unwrap().stopped);
 assert!(w.process(2).is_none());
 let mut correct=0;
 let mut wrong=0;
 for e in &w.events()[first_after_update..]{
  if let (Origin::Internal(1),EventKind::CellWritten{key:11,value})=(&e.origin,&e.kind){
   if *value==new{correct+=1;}else{wrong+=1;}
  }
 }
 let source_edits=w.events().iter().filter(|e|matches!(
  (&e.origin,&e.kind),
  (Origin::Internal(1),EventKind::ExecutableEdgePatched{node:3,before:4,after:11})
 )).count();
 let out=Observation{arm,old,new,arrival,initial_sample,first_output,
  current_input:new,check_pc,judgement_correct,reads_at_first_check,
  paid:96,correct_writes:correct,wrong_writes:wrong,
  source_edits,final_edge:edge(&w,arm)};
 eprintln!("B40_NATIVE {:?}",out);
 out
}
#[test]
fn b40_postwrite_paid_task_recheck_corrects_b39_false_evidence_at_extra_compute_cost(){
 let mut worlds=0;
 for (old,new) in [(0_i64,17_i64),(17,0)]{
  for arrival in [Arrival::BeforeRead,Arrival::AfterRead]{
   let early=arrival==Arrival::BeforeRead;
   let reference=run(Arm::Readback,old,new,arrival);
   let fresh=run(Arm::Recheck,old,new,arrival);
   let static_correct=run(Arm::StaticCorrect,old,new,arrival);
   worlds+=3;
   assert_eq!((reference.paid,fresh.paid,static_correct.paid),(96,96,96));
   assert_eq!(reference.judgement_correct,Some(early));
   assert_eq!(fresh.judgement_correct,Some(true),
    "post-output re-read must agree with latest task input for registered update timing");
   assert_eq!(fresh.reads_at_first_check,
    if early{vec![new,new]}else{vec![old,new]});
   assert_eq!(reference.reads_at_first_check,vec![if early{new}else{old}]);
   assert_eq!(fresh.current_input,new);
   assert_eq!(static_correct.source_edits,0);
   assert_eq!(static_correct.final_edge,3);
   assert!(static_correct.correct_writes>=fresh.correct_writes,
    "strong immutable copier must remain in true total-cost denominator");
   assert_eq!(fresh.final_edge,if new!=0{11}else{4});
   assert_eq!(fresh.source_edits,usize::from(new!=0));
   if !early&&old==0{
    assert_eq!(reference.check_pc,1,"B39 stale false success");
    assert_eq!(fresh.check_pc,8,"B40 current-input mismatch detected");
   }
   if !early&&old==17{
    assert_eq!(reference.check_pc,8,"B39 stale false failure");
    assert_eq!(fresh.check_pc,1,"B40 avoids gratuitous native code mutation");
    assert_eq!(fresh.source_edits,0);
   }
  }
 }
 assert_eq!(worlds,12);
}
