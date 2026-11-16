//! B36: Alternative generic environmental compute law, not organism repair.
//! Reuses B35's EXACT source and tests draw-only vs any-op one-credit pool
//! payment, preserving true native resource conservation and STATIC3 control.
use cl0_world::{EventKind,Intervention,Op,Origin,Process,World};
#[path="cl1b_b35_multitrial_retention.rs"]
mod canonical_b35;
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
enum Law{DrawOnly,AllNative}
#[derive(Debug)]
struct ResultTrace {
 law:Law,adaptive:bool,schedule:Vec<u64>,service_writes:usize,per_packet_writes:Vec<usize>,
 paid:usize,pool_paid_non_draw:usize,source_request:i64,local:u64,pool:u64,
 pc:usize,stopped:bool,traps:usize,patches:usize,
}
fn actual_work(w:&World,adaptive:bool)->usize{
 let service=if adaptive{10}else{4};
 w.events().iter().filter(|e|matches!(
  (&e.origin,&e.kind),
  (Origin::Internal(1),EventKind::CellWritten{key:9,value:1}) if e.pc==service
 )).count()
}
fn drain(w:&mut World)->usize{
 let mut n=0;
 loop{
  assert!(n<200,"finite neutral shared compute must bound native execution");
  let inv=w.total_compute_inventory().unwrap();
  let prev=w.events().len();
  if !w.step(){
   assert_eq!(w.total_compute_inventory(),Some(inv));
   assert_eq!(w.events().len(),prev);
   return n;
  }
  n+=1;
  assert_eq!(w.total_compute_inventory(),Some(inv-1),
   "direct shared-pool instruction fees must preserve real world compute accounting");
  assert_eq!(w.events().len(),prev+1);
  assert_eq!(w.events().last().unwrap().origin,Origin::Internal(1));
 }
}
fn case(law:Law,adaptive:bool,schedule:&[u64])->ResultTrace{
 let code=canonical_b35::founder(if adaptive {
  canonical_b35::Arm::Adaptive
 }else{
  canonical_b35::Arm::StaticThree
 });
 let initial_code_len=code.len();
 let mut w=World::with_strict_capacity(16,64,128)
  .with_pool_paid_draw().with_recoverable_draw_shortage();
 if law==Law::AllNative {
  w=w.with_pool_paid_all_instructions();
 }
 assert_eq!(w.pool_paid_all_instructions_enabled(),law==Law::AllNative);
 w.load(Process::new(1,code,14)).unwrap();
 let mut paid=0;
 let mut per_packet_writes=Vec::new();
 for &packet in schedule{
  let old=w.total_compute_inventory().unwrap();
  let current=w.available_compute_pool();
  w.intervene(Intervention::SetComputePool{credits:current+packet}).unwrap();
  assert_eq!(w.total_compute_inventory(),Some(old+packet));
  assert_eq!(w.events().last().unwrap().origin,Origin::External);
  paid+=drain(&mut w);
  per_packet_writes.push(actual_work(&w,adaptive));
 }
 let p=w.process(1).unwrap();
 assert_eq!(p.code.nodes.len(),initial_code_len);
 assert!(w.process(2).is_none());
 let source_request=match &p.code.nodes[0] {Op::Const{value,..}=>*value,_=>panic!("not CIL candidate literal")};
 let non_draw=w.events().iter().filter(|e|
  e.origin==Origin::Internal(1)&&e.pool_paid_attempt&&
  e.pc!=(if adaptive{11}else{1})
 ).count();
 let traps=w.events().iter().filter(|e|matches!(e.kind,EventKind::Trapped(_))).count();
 let patches=w.events().iter().filter(|e|e.origin==Origin::Internal(1)&&e.pc==6&&!matches!(e.kind,EventKind::Trapped(_))).count();
 assert_eq!(w.events().iter().filter(|e|e.origin==Origin::External).count(),1+schedule.len());
 assert_eq!(w.total_compute_inventory(),Some(14+schedule.iter().sum::<u64>()-paid as u64));
 assert!(w.occupied_units()<=128);
 if law==Law::DrawOnly {assert_eq!(non_draw,0);}
 let result=ResultTrace{
  law,adaptive,schedule:schedule.to_vec(),service_writes:actual_work(&w,adaptive),
  per_packet_writes,paid,pool_paid_non_draw:non_draw,source_request,
  local:p.credits,pool:w.available_compute_pool(),pc:p.pc,stopped:p.stopped,
  traps,patches
 };
 eprintln!("B36_NATIVE {:?}",result);
 result
}
#[test]
fn b36_any_instruction_can_use_neutral_pool_credit_without_free_compute_or_automatic_fitness(){
 let schedules=[
  &[4_u64,5,5][..],&[4_u64,12,12][..],
  &[5_u64,5,5][..],&[6_u64,8,12][..],
 ];
 let baseline_adaptive=[1_usize,3,1,1];
 let baseline_static=[3_usize,7,4,7];
 // Independent source-parsed CIL predictions frozen BEFORE any B36 Rust CI.
 // More executable instructions become affordable, but adaptive work
 // remains worse than an ultra-compact immutable native actor.
 let all_native_adaptive=[1_usize,3,1,2];
 let all_native_static=[3_usize,7,4,7];
 let all_native_pool=[0_u64,0,0,0];
 let pool_paid_non_draw_adapt=[5_usize,1,4,6];
 let mut worlds=0;
 let mut direct_charge=0;
 for (i,schedule) in schedules.into_iter().enumerate(){
  let draw_adapt=case(Law::DrawOnly,true,schedule);
  let all_adapt=case(Law::AllNative,true,schedule);
  let draw_static=case(Law::DrawOnly,false,schedule);
  let all_static=case(Law::AllNative,false,schedule);
  worlds+=4;
  assert_eq!(draw_adapt.service_writes,baseline_adaptive[i],
   "new optional world law must not alter native B35 draw-only baseline");
  assert_eq!(draw_static.service_writes,baseline_static[i]);
  assert_eq!(all_adapt.service_writes,all_native_adaptive[i],
   "B36 paid-any-op source model prediction includes real work and overhead");
  assert_eq!(all_static.service_writes,all_native_static[i],
   "compact static native service must not be affected by all-pool semantics");
  assert_eq!(all_adapt.pool,all_native_pool[i]);
  assert_eq!(all_adapt.pool_paid_non_draw,pool_paid_non_draw_adapt[i],
   "must audit each real pool-paid non-Draw attempt, not just new work output");
  assert_eq!(all_adapt.traps,0);
  assert_eq!(draw_adapt.pool_paid_non_draw,0);
  assert_eq!(draw_static.pool_paid_non_draw,0);
  direct_charge+=all_adapt.pool_paid_non_draw+all_static.pool_paid_non_draw;
  for x in [&draw_adapt,&all_adapt,&draw_static,&all_static] {
   assert_eq!(x.paid as u64+x.local+x.pool,
    14+schedule.iter().sum::<u64>());
  }
  assert!(all_adapt.service_writes<=all_static.service_writes,
   "new environment physics must be compared to the cheapest immutable actor");
 }
 assert_eq!(worlds,16);
 assert!(direct_charge>0,"the alternative environment must really fund a non-Draw CIL instruction");
}
#[test]
fn b36_generic_pool_pay_rule_cannot_create_credit_or_revive_stopped_actor(){
 let mut zero=World::new(4,8).with_pool_paid_all_instructions();
 zero.load(Process::new(1,vec![Op::Const{dst:0,value:9,next:0}],0)).unwrap();
 assert!(!zero.step(),"no local or shared compute energy means no instruction");
 zero.intervene(Intervention::SetComputePool{credits:2}).unwrap();
 assert!(zero.step(),"ordinary Const is now a charged shared-pool native opcode");
 assert_eq!(zero.available_compute_pool(),1);
 assert_eq!(zero.process(1).unwrap().registers[0],9);
 assert_eq!(zero.process(1).unwrap().credits,0);
 assert!(zero.events().last().unwrap().pool_paid_attempt);
 let mut trapped=World::new(4,8).with_pool_paid_all_instructions();
 trapped.load(Process::new(1,vec![Op::Branch{test:9,nonzero:0,zero:0}],0)).unwrap();
 trapped.intervene(Intervention::SetComputePool{credits:10}).unwrap();
 assert!(trapped.step());
 assert!(matches!(trapped.events().last().unwrap().kind,EventKind::Trapped(_)));
 assert!(trapped.process(1).unwrap().stopped);
 let before=trapped.events().len();
 assert!(!trapped.step(),"native trap state remains stopped, even with neutral energy");
 assert_eq!(trapped.events().len(),before);
}
