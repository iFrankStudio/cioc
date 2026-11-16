//! CL-1B B34: charged nonblocking TryDrawCompute gives CIL a real
//! success/failure register, allowing an authored resource fallback and
//! native executable literal patch. No endogenous learning or fitness oracle.
use cl0_world::{EventKind,Intervention,Op,Origin,Process,World};

#[derive(Clone,Copy,Debug,PartialEq,Eq)]
enum Arm{Try,ForcedFail,StaticThree}
fn founder(arm:Arm)->Vec<Op>{
 if arm==Arm::StaticThree{
   return vec![
    Op::Const{dst:0,value:3,next:1},
    Op::DrawCompute{amount:0,next:2},
    Op::Const{dst:2,value:9,next:3},
    Op::Const{dst:3,value:1,next:4},
    Op::Write{address:2,value:3,next:1},
   ];
 }
 vec![
  Op::Const{dst:0,value:5,next:1}, //0 authored candidate
  Op::Const{dst:1,value:0,next:2}, //1 source node0
  if arm==Arm::Try {
   Op::TryDrawCompute{amount:0,dst:2,next:3}
  }else{
   Op::Const{dst:2,value:0,next:3} // same-cost forced-fail comparison
  }, //2
  Op::Branch{test:2,nonzero:7,zero:4}, //3
  Op::Const{dst:0,value:3,next:5}, //4 authored fallback
  Op::PatchConst{node:1,value:0,next:6}, //5 real self edit of first request
  Op::DrawCompute{amount:0,next:7}, //6 generic zero-credit resource gateway
  Op::Const{dst:2,value:9,next:8}, //7
  Op::Const{dst:3,value:1,next:9}, //8
  Op::Write{address:2,value:3,next:6}, //9
 ]
}
#[derive(Debug)]
struct ResultTrace{
 arm:Arm,packet:u64,paid:usize,outputs:usize,try_events:Vec<(u64,u64,bool)>,
 source_request:i64,edits:usize,pc:usize,local:u64,pool:u64,traps:usize
}
fn drain(w:&mut World)->usize{
 let mut n=0;
 loop {
  assert!(n<70,"finite neutral-pool budget cannot support unbounded work");
  let energy=w.total_compute_inventory().unwrap();
  let events=w.events().len();
  if !w.step(){
   assert_eq!(w.total_compute_inventory(),Some(energy));
   assert_eq!(w.events().len(),events);
   return n;
  }
  n+=1;
  assert_eq!(w.total_compute_inventory(),Some(energy-1));
  assert_eq!(w.events().len(),events+1);
  assert_eq!(w.events().last().unwrap().origin,Origin::Internal(1));
  assert!(!matches!(w.events().last().unwrap().kind,EventKind::Trapped(_)|EventKind::Halted));
 }
}
fn trial(arm:Arm,packet:u64)->ResultTrace{
 let mut w=World::with_strict_capacity(16,64,128)
  .with_pool_paid_draw().with_recoverable_draw_shortage()
  .with_native_executable_edit_trace();
 let code=founder(arm);
 assert_eq!(code.len(),if arm==Arm::StaticThree{5}else{10});
 w.load(Process::new(1,code,6)).unwrap();
 if packet>0{
  w.intervene(Intervention::SetComputePool{credits:packet}).unwrap();
  assert_eq!(w.events().last().unwrap().origin,Origin::External);
 }
 let paid=drain(&mut w);
 let attempts=w.events().iter().filter_map(|e|match (&e.origin,&e.kind){
  (Origin::Internal(1),EventKind::ComputeDrawTried{
   requested,available_before,success
  }) if e.pc==2=>Some((*requested,*available_before,*success)),
  _=>None,
 }).collect::<Vec<_>>();
 let outputs=w.events().iter().filter(|e|matches!(
  (&e.origin,&e.kind),
  (Origin::Internal(1),EventKind::CellWritten{key:9,value:1})
   if e.pc==if arm==Arm::StaticThree{4}else{9}
 )).count();
 // PatchConst emits a charged native PC5 execution; the actual
 // executable source literal is audited separately below.
 let edits=w.events().iter().filter(|e|
  e.origin==Origin::Internal(1) && e.pc==5 &&
  !matches!(e.kind,EventKind::Trapped(_))
 ).count();
 let source_request=match &w.process(1).unwrap().code.nodes[0]{
  Op::Const{value,..}=>*value,_=>panic!("first resource-request literal mutated to wrong opcode")
 };
 let traps=w.events().iter().filter(|e|matches!(e.kind,EventKind::Trapped(_))).count();
 assert_eq!(w.total_compute_inventory(),Some(6+packet-paid as u64));
 assert_eq!(w.events().iter().filter(|e|e.origin==Origin::External).count(),
  1+usize::from(packet>0));
 assert!(w.process(2).is_none());
 assert!(w.occupied_units()<=128);
 let out=ResultTrace{
  arm,packet,paid,outputs,try_events:attempts,source_request,edits,
  pc:w.process(1).unwrap().pc,local:w.process(1).unwrap().credits,
  pool:w.available_compute_pool(),traps
 };
 eprintln!("B34_NATIVE {:?}",out);
 out
}
#[test]
fn b34_nonblocking_real_resource_outcome_is_visible_to_cil_and_triggers_authored_fallback(){
 // Numeric encoding is private to the kernel and tested in its mandatory
 // B34 library ABI check; integration tests verify its public decoder.

 assert_eq!(Op::decode_instruction([32,0,2,3,0]).unwrap(),
   Op::TryDrawCompute{amount:0,dst:2,next:3});
 for packet in [0_u64,4,5,6]{
  let candidate=trial(Arm::Try,packet);
  let forced=trial(Arm::ForcedFail,packet);
  let static3=trial(Arm::StaticThree,packet);
  assert_eq!(candidate.try_events,vec![(5,packet,packet>=5)]);
  assert!(forced.try_events.is_empty()&&static3.try_events.is_empty());
  assert_eq!(candidate.source_request,if packet>=5{5}else{3});
  assert_eq!(forced.source_request,3);
  assert_eq!(static3.source_request,3);
  assert_eq!(candidate.edits,usize::from(packet<5));
  assert_eq!(forced.edits,1);
  assert_eq!(static3.edits,0);
  assert_eq!(candidate.traps+forced.traps+static3.traps,0);
  assert_eq!(candidate.local+candidate.pool+candidate.paid as u64,6+packet);
  assert_eq!(forced.local+forced.pool+forced.paid as u64,6+packet);
  assert_eq!(static3.local+static3.pool+static3.paid as u64,6+packet);
  if packet==4{
   assert_eq!(candidate.outputs,1);
   assert_eq!(forced.outputs,1);
   assert!(static3.outputs>=candidate.outputs);
   assert_eq!(candidate.pc,6);
  }
  if packet==0{
   assert_eq!(candidate.outputs,0);
  }
  assert!(static3.outputs>=candidate.outputs,
    "a cheap static baseline must remain in the denominator");
 }
}
#[test]
fn b34_trydraw_is_not_pool_paid_or_a_free_rescue_at_zero_local_compute(){
 let mut w=World::with_strict_capacity(16,64,128)
  .with_pool_paid_draw().with_recoverable_draw_shortage();
 w.load(Process::new(1,vec![Op::TryDrawCompute{amount:0,dst:1,next:0}],0)).unwrap();
 w.intervene(Intervention::SetComputePool{credits:100}).unwrap();
 let before=w.events().len();
 assert!(!w.step(),"nonblocking attempts are local-credit-paid only");
 assert_eq!(w.events().len(),before);
 assert_eq!(w.available_compute_pool(),100);
 assert_eq!(w.process(1).unwrap().registers,[0;4]);
 let mut bad=World::with_strict_capacity(16,64,128);
 bad.load(Process::new(1,vec![Op::TryDrawCompute{amount:0,dst:4,next:0}],1)).unwrap();
 assert!(bad.step(),"invalid register is a real charged trap");
 assert!(matches!(bad.events().last().unwrap().kind,EventKind::Trapped(_)));
 assert_eq!(bad.process(1).unwrap().credits,0);
 assert!(Op::decode_instruction([32,-1,0,0,0]).is_err());
}
