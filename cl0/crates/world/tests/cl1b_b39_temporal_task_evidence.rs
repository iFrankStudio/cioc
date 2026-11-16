//! B39: exact B38 CIL founder under a task-input change between native
//! Read(key7) and Write(key11)/readback. A truthful old observation can
//! yield a false current-time success/failure, without fabricated events.
use cl0_world::{EventKind,Intervention,Op,Origin,Process,World};
#[path="cl1b_b38_functional_readback.rs"]
mod canonical_b38;
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
enum Arrival{BeforeRead,AfterRead}
#[derive(Clone,Debug)]
struct ResultTrace{
 old:i64,new:i64,arrival:Arrival,readback:bool,
 sampled:i64,first_written:i64,checkpoint_pc:usize,
 first_output_current_correct:bool,first_verdict_correct:Option<bool>,
 charged:usize,final_edge:usize,real_patches:usize,
 outputs_correct_after_update:usize,outputs_wrong_after_update:usize,
}
fn charged(w:&mut World)->bool{
 let n=w.events().len();
 let before=w.total_compute_inventory().unwrap();
 let ok=w.step();
 assert_eq!(w.total_compute_inventory(),Some(before-u64::from(ok)));
 assert_eq!(w.events().len(),n+usize::from(ok));
 if ok{
  assert_eq!(w.events().last().unwrap().origin,Origin::Internal(1));
  assert!(!matches!(w.events().last().unwrap().kind,EventKind::Trapped(_)|EventKind::Halted));
 }
 ok
}
fn step_to_input_read(w:&mut World)->i64{
 for _ in 0..8{
  assert!(charged(w));
  let e=w.events().last().unwrap();
  if let EventKind::CellRead{key:7,value}=e.kind{return value;}
 }
 panic!("exact B38 source must read the real task input within eight charged instructions")
}
fn step_to_checkpoint(w:&mut World,readback:bool)->(usize,i64){
 for _ in 0..12{
  assert!(charged(w));
  let e=w.events().last().unwrap();
  if readback&&e.pc==7{
   let actual=w.cell(11).expect("native B38 first Write must have occurred");
   return (w.process(1).unwrap().pc,actual);
  }
  if !readback&&matches!(e.kind,EventKind::CellWritten{key:11,..}){
   return (w.process(1).unwrap().pc,w.cell(11).unwrap());
  }
 }
 panic!("B39 native task must complete first write and comparison checkpoint")
}
fn edge(w:&World,readback:bool)->usize{
 let index=if readback{3}else{2};
 match &w.process(1).unwrap().code.nodes[index]{
  Op::Const{next,..}=>*next,_=>panic!("B39 must reuse canonical B38 source")
 }
}
fn trial(old:i64,new:i64,arrival:Arrival,readback:bool)->ResultTrace{
 let arm=if readback{canonical_b38::Arm::Readback}else{canonical_b38::Arm::StaticCorrect};
 let mut w=World::with_strict_capacity(16,64,128)
  .with_native_executable_edit_trace();
 w.load(Process::new(1,canonical_b38::founder(arm),96)).unwrap();
 w.intervene(Intervention::SetCell{key:7,value:old}).unwrap();
 assert_eq!(w.events().last().unwrap().origin,Origin::External);
 if arrival==Arrival::BeforeRead{
  w.intervene(Intervention::SetCell{key:7,value:new}).unwrap();
 }
 let sampled=step_to_input_read(&mut w);
 assert_eq!(sampled,if arrival==Arrival::BeforeRead{new}else{old});
 if arrival==Arrival::AfterRead {
  w.intervene(Intervention::SetCell{key:7,value:new}).unwrap();
 }
 let update_event_end=w.events().len();
 assert_eq!(w.cell(7),Some(new));
 let (checkpoint_pc,first_written)=step_to_checkpoint(&mut w,readback);
 let first_output_current_correct=first_written==new;
 let first_verdict_correct=if readback{
  // PC7 branches nonzero to8, zero to1: the chosen path is a
  // self-calculated (potentially stale) result, not a host score.
  assert!(checkpoint_pc==1||checkpoint_pc==8);
  Some((checkpoint_pc==1)==first_output_current_correct)
 }else{None};
 assert_eq!(edge(&w,readback),if readback{4}else{3},
  "no native self-repair may occur before first discrepancy decision");
 let attempts_so_far=w.events().iter().filter(|e|matches!(e.origin,Origin::Internal(1))).count();
 let rest=96-attempts_so_far;
 assert_eq!(w.run(rest),rest);
 assert_eq!(w.total_compute_inventory(),Some(0));
 assert!(!w.process(1).unwrap().stopped);
 assert_eq!(w.events().iter().filter(|e|matches!(e.origin,Origin::External)).count(),3);
 let real_patches=w.events().iter().filter(|e|matches!(
  (&e.origin,&e.kind),
  (Origin::Internal(1),EventKind::ExecutableEdgePatched{node:3,before:4,after:11})
 )).count();
 let correct=w.events()[update_event_end..].iter().filter(|e|matches!(
  (&e.origin,&e.kind),
  (Origin::Internal(1),EventKind::CellWritten{key:11,value}) if *value==new
 )).count();
 let wrong=w.events()[update_event_end..].iter().filter(|e|matches!(
  (&e.origin,&e.kind),
  (Origin::Internal(1),EventKind::CellWritten{key:11,value}) if *value!=new
 )).count();
 let output=ResultTrace{old,new,arrival,readback,sampled,first_written,
  checkpoint_pc,first_output_current_correct,first_verdict_correct,
  charged:96,final_edge:edge(&w,readback),real_patches,
  outputs_correct_after_update:correct,outputs_wrong_after_update:wrong};
 eprintln!("B39_NATIVE {:?}",output);
 output
}

#[test]
fn b39_live_input_change_produces_false_readback_success_and_false_repair_signal(){
 let mut worlds=0;
 for (old,new) in [(0_i64,17_i64),(17,0)]{
  for arrival in [Arrival::BeforeRead,Arrival::AfterRead]{
   let sensed=trial(old,new,arrival,true);
   let static_correct=trial(old,new,arrival,false);
   worlds+=2;
   assert_eq!(sensed.charged,96);
   assert_eq!(static_correct.charged,96);
   assert_eq!(static_correct.real_patches,0);
   assert_eq!(static_correct.final_edge,3);
   assert_eq!(sensed.final_edge,
    if old==17&&arrival==Arrival::BeforeRead{4}else{11});
   assert_eq!(sensed.real_patches,if sensed.final_edge==11{1}else{0});
   if arrival==Arrival::AfterRead{
    assert_eq!(sensed.first_written,0);
    assert_eq!(sensed.sampled,old);
    assert_eq!(static_correct.sampled,old);
    assert_eq!(static_correct.first_written,old);
    assert!(!static_correct.first_output_current_correct,
     "even a correct immutable program can briefly write a stale input value");
    assert_eq!(sensed.first_verdict_correct,Some(false),
     "native output readback can be truthful for stale expected input but false for current task");
    if old==0{
     assert_eq!(sensed.checkpoint_pc,1,
      "wrong current output was falsely accepted using stale zero input");
     assert!(!sensed.first_output_current_correct);
    }else{
     assert_eq!(sensed.checkpoint_pc,8,
      "currently correct output triggered an unnecessary native repair branch");
     assert!(sensed.first_output_current_correct);
    }
   }else{
    assert_eq!(sensed.sampled,new);
    assert_eq!(static_correct.sampled,new);
    assert_eq!(static_correct.first_written,new);
    assert!(static_correct.first_output_current_correct);
    assert_eq!(sensed.first_verdict_correct,Some(true));
   }
 }
 }
 assert_eq!(worlds,8);
}
