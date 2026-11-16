//! B35 -- same continuing CIL actor computes lower resource candidates
//! from real paid TryDraw failure, patches its own executable request literal,
//! and retries. Full resource conservation; no learning/fitness oracle.
use cl0_world::{EventKind,Intervention,Op,Origin,Process,World};

#[derive(Debug,Clone,Copy,PartialEq,Eq)]
enum Arm { Adaptive, ForcedFail, StaticThree }

fn founder(arm:Arm)->Vec<Op> {
 if arm==Arm::StaticThree {
  return vec![
   Op::Const{dst:0,value:3,next:1},
   Op::DrawCompute{amount:0,next:2},
   Op::Const{dst:2,value:9,next:3},
   Op::Const{dst:3,value:1,next:4},
   Op::Write{address:2,value:3,next:1},
  ];
 }
 vec![
  Op::Const{dst:0,value:5,next:1},                        // 0 executable candidate
  Op::Const{dst:1,value:0,next:2},                        // 1 source target PC0
  Op::Const{dst:3,value:1,next:3},                        // 2 decrement one
  if arm==Arm::Adaptive {
   Op::TryDrawCompute{amount:0,dst:2,next:4}
  } else {
   Op::Const{dst:2,value:0,next:4}                       // 3 equal cost blind ablation
  },
  Op::Branch{test:2,nonzero:8,zero:5},                   // 4
  Op::Sub{dst:0,lhs:0,rhs:3,next:6},                     // 5 compute candidate Q-1
  Op::PatchConst{node:1,value:0,next:7},                  // 6 native source edit
  Op::Const{dst:2,value:0,next:3},                        // 7 repeat real trial
  Op::Const{dst:1,value:9,next:9},                        // 8
  Op::Const{dst:3,value:1,next:10},                       // 9
  Op::Write{address:1,value:3,next:11},                   // 10 genuine service
  Op::DrawCompute{amount:0,next:0},                       // 11 pool-paid gateway
 ]
}

#[derive(Debug)]
struct ResultTrace {
 arm:Arm,schedule:Vec<u64>,native_trials:Vec<(u64,u64,bool)>,
 total_writes:usize,per_packet_writes:Vec<usize>,per_packet_paid:Vec<usize>,
 edits:usize,source_request:i64,charged_steps:usize,
 final_pc:usize,local:u64,pool:u64,stopped:bool,traps:usize,
}

fn step_checked(w:&mut World)->bool{
 let before=w.total_compute_inventory().unwrap();
 let previous_events=w.events().len();
 let stepped=w.step();
 assert_eq!(w.total_compute_inventory(),Some(before-u64::from(stepped)),
  "resource attempt, source edit, failed launch and work must all cost native compute");
 assert_eq!(w.events().len(),previous_events+usize::from(stepped));
 if stepped {
  assert_eq!(w.events().last().unwrap().origin,Origin::Internal(1));
 }
 stepped
}
fn drain(w:&mut World)->usize{
 let mut count=0;
 while step_checked(w){
  count+=1;
  assert!(count<160,"finite common-pool packet forbids unbounded native work");
 }
 count
}
fn edits(w:&World)->usize{
 w.events().iter().filter(|e|
  e.origin==Origin::Internal(1)&&e.pc==6&&
  !matches!(e.kind,EventKind::Trapped(_))
 ).count()
}
fn real_writes(w:&World,arm:Arm)->usize{
 let service_pc=if arm==Arm::StaticThree{4}else{10};
 w.events().iter().filter(|e|matches!(
  (&e.origin,&e.kind),
  (Origin::Internal(1),EventKind::CellWritten{key:9,value:1}) if e.pc==service_pc
 )).count()
}
fn run(arm:Arm,schedule:&[u64])->ResultTrace{
 let mut w=World::with_strict_capacity(16,64,128)
  .with_pool_paid_draw().with_recoverable_draw_shortage()
  .with_native_executable_edit_trace();
 let source=founder(arm);
 assert_eq!(source.len(),if arm==Arm::StaticThree{5}else{12});
 w.load(Process::new(1,source,14)).unwrap();
 assert!(w.process(2).is_none());
 let mut steps=0;
 let mut per_packet_paid=Vec::new();
 let mut per_packet_writes=Vec::new();
 for &packet in schedule{
  let old=w.total_compute_inventory().unwrap();
  let old_pool=w.available_compute_pool();
  w.intervene(Intervention::SetComputePool{credits:old_pool+packet}).unwrap();
  assert_eq!(w.total_compute_inventory(),Some(old+packet));
  assert_eq!(w.events().last().unwrap().origin,Origin::External);
  let paid=drain(&mut w);steps+=paid;
  per_packet_paid.push(paid);
  per_packet_writes.push(real_writes(&w,arm));
 }
 let p=w.process(1).unwrap();
 let actual_request=match &p.code.nodes[0] {
  Op::Const{value,..}=>*value,_=>panic!("CIL native candidate must remain executable Const")
 };
 let native_trials=w.events().iter().filter_map(|e|match (&e.origin,&e.kind){
  (Origin::Internal(1),EventKind::ComputeDrawTried{
   requested,available_before,success
  }) if e.pc==3=>Some((*requested,*available_before,*success)),_=>None
 }).collect::<Vec<_>>();
 let trap_count=w.events().iter().filter(|e|matches!(e.kind,EventKind::Trapped(_))).count();
 assert_eq!(w.total_compute_inventory(),Some(14+schedule.iter().sum::<u64>()-steps as u64));
 assert_eq!(w.events().iter().filter(|e|e.origin==Origin::External).count(),1+schedule.len());
 assert!(w.occupied_units()<=128);
 assert!(w.process(2).is_none());
 if arm!=Arm::Adaptive {
  assert!(native_trials.is_empty(),
   "equal-cost blind loop and immutable baseline must not observe real trial outcomes");
 }
 let result=ResultTrace{
  arm,schedule:schedule.to_vec(),native_trials,total_writes:real_writes(&w,arm),
  per_packet_writes,per_packet_paid,
  edits:edits(&w),source_request:actual_request,charged_steps:steps,
  final_pc:p.pc,local:p.credits,pool:w.available_compute_pool(),
  stopped:p.stopped,traps:trap_count,
 };
 eprintln!("B35_NATIVE {:?}",result);
 result
}
#[test]
fn b35_native_repeated_paid_candidate_trials_and_source_mutations_are_not_free_learning(){
 let schedules=[
  &[4_u64,5,5][..], &[4_u64,12,12][..],
  &[5_u64,5,5][..], &[6_u64,8,12][..]
 ];
 let expected_adaptive_outputs=[1_usize,3,1,1];
 let expected_forced_outputs=[0_usize,0,0,0];
 let expected_static_outputs=[3_usize,7,4,7];
 let expected_adaptive_source=[4_i64,4,5,5];
 let expected_adaptive_pc=[4_usize,4,5,5];
 let expected_adaptive_pool=[5_u64,1,4,15];
 let mut worlds=0;
 for (index,schedule) in schedules.into_iter().enumerate(){
  let adaptive=run(Arm::Adaptive,schedule);
  let forced=run(Arm::ForcedFail,schedule);
  let static3=run(Arm::StaticThree,schedule);
  worlds+=3;
  // Source-parsed JS model predictions frozen before native Rust CI.
  assert_eq!([adaptive.total_writes,forced.total_writes,static3.total_writes],
   [expected_adaptive_outputs[index],expected_forced_outputs[index],
    expected_static_outputs[index]],
   "B35 complete native services must preserve the weak adaptive/strong static comparison");
  assert_eq!(adaptive.source_request,expected_adaptive_source[index],
   "the source-encoded retained candidate is the actual genome literal");
  assert_eq!(adaptive.final_pc,expected_adaptive_pc[index],
   "failed resource attempts can strand an actor at ordinary non-Draw control flow");
  assert_eq!(adaptive.pool,expected_adaptive_pool[index],
   "unusable shared compute must not be silently awarded to the stuck actor");
  assert_eq!(adaptive.traps,0);
  for result in [&adaptive,&forced,&static3]{
   assert_eq!(result.schedule,schedule);
   assert_eq!(result.charged_steps as u64+result.local+result.pool,
      14+schedule.iter().sum::<u64>());
   assert_eq!(result.per_packet_writes.len(),schedule.len());
   assert_eq!(result.per_packet_paid.len(),schedule.len());
  }
  assert_eq!(static3.source_request,3);
  assert_eq!(static3.edits,0);
  assert!(adaptive.native_trials.iter().all(|(requested,available,success)|
    *success==(*requested<=*available)
  ),"CIL-visible evidence must reflect real shared pool acquisition");
  assert!(adaptive.edits<=adaptive.native_trials.iter().filter(|(_,_,ok)|!ok).count(),
    "only completed failed trials may trigger the signed decrement/real source edit");
  assert!(static3.total_writes>=adaptive.total_writes,
    "retain compact immutable total-cost baseline in EVERY schedule");
 }
 assert_eq!(worlds,12);
}
