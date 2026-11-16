//! B37: multi-actor fairness stress test of optional generic direct-pool
//! instruction execution. Author-written CIL WORKER vs useless BURNER;
//! no evolutionary inference or privileged fitness reward in the scheduler.
use cl0_world::{EventKind,Intervention,Op,Origin,Process,World};

#[derive(Clone,Copy,Debug,PartialEq,Eq)]
enum Composition{SoloWorker,BurnerFirst,WorkerFirst,DualWorker}
fn worker()->Vec<Op>{
 vec![
  Op::Const{dst:2,value:9,next:1},
  Op::Const{dst:3,value:1,next:2},
  Op::Write{address:2,value:3,next:0},
 ]
}
fn burner()->Vec<Op>{
 vec![
  Op::Const{dst:0,value:7,next:1},
  Op::Branch{test:0,nonzero:0,zero:0},
 ]
}
fn classify(c:Composition)->(Vec<(u64,Vec<Op>)>,Vec<u64>){
 match c{
  Composition::SoloWorker=>(vec![(1,worker())],vec![1]),
  Composition::BurnerFirst=>(vec![(1,burner()),(2,worker())],vec![2]),
  Composition::WorkerFirst=>(vec![(1,worker()),(2,burner())],vec![1]),
  Composition::DualWorker=>(vec![(1,worker()),(2,worker())],vec![1,2]),
 }
}
#[derive(Debug)]
struct Outcome{
 group:Composition,supply:Vec<u64>,direct:bool,steps:usize,
 productive:usize,worker_steps:usize,burner_steps:usize,
 pool_paid_attempts:usize,pool:u64
}
fn run(group:Composition,supply:&[u64],direct:bool)->Outcome{
 let (initial,worker_ids)=classify(group);
 let mut w=World::with_strict_capacity(16,64,128)
  .with_pool_paid_draw().with_recoverable_draw_shortage();
 if direct{w=w.with_pool_paid_all_instructions();}
 for (pid,code) in initial {
  w.load(Process::new(pid,code,0)).unwrap();
 }
 let mut steps=0;
 for &packet in supply {
  let before=w.total_compute_inventory().unwrap();
  w.intervene(Intervention::SetComputePool{
   credits:w.available_compute_pool()+packet
  }).unwrap();
  assert_eq!(w.total_compute_inventory(),Some(before+packet));
  assert_eq!(w.events().last().unwrap().origin,Origin::External);
  let mut paid=0;
  loop {
   let budget=w.total_compute_inventory().unwrap();
   let events=w.events().len();
   if !w.step(){
    assert_eq!(w.total_compute_inventory(),Some(budget));
    assert_eq!(w.events().len(),events);
    break;
   }
   paid+=1;steps+=1;
   assert!(paid<=18,"the entire common pool was only 18 units");
   assert_eq!(w.total_compute_inventory(),Some(budget-1));
   assert_eq!(w.events().len(),events+1);
   assert!(matches!(w.events().last().unwrap().origin,Origin::Internal(1)|Origin::Internal(2)));
   assert!(!matches!(w.events().last().unwrap().kind,
    EventKind::Trapped(_)|EventKind::Halted));
  }
 }
 let worker_steps=w.events().iter().filter(|e|matches!(e.origin,Origin::Internal(pid) if worker_ids.contains(&pid))).count();
 let burner_steps=w.events().iter().filter(|e|matches!(e.origin,Origin::Internal(pid) if !worker_ids.contains(&pid))).count();
 let productive=w.events().iter().filter(|e|matches!(
  (&e.origin,&e.kind),
  (Origin::Internal(pid),EventKind::CellWritten{key:9,value:1})
    if worker_ids.contains(pid)&&e.pc==2
 )).count();
 let pool_paid_attempts=w.events().iter().filter(|e|
  matches!(e.origin,Origin::Internal(_))&&e.pool_paid_attempt
 ).count();
 assert_eq!(steps,worker_steps+burner_steps);
 assert_eq!(w.total_compute_inventory(),Some(supply.iter().sum::<u64>()-steps as u64));
 assert!(w.process(3).is_none());
 assert!(w.occupied_units()<=128);
 assert_eq!(w.events().iter().filter(|e|e.origin==Origin::External).count(),
  (if group==Composition::SoloWorker{1}else{2})+supply.len());
 for worker_id in &worker_ids {
  assert!(!w.process(*worker_id).unwrap().stopped);
 }
 let outcome=Outcome{
  group,supply:supply.to_vec(),direct,steps,productive,worker_steps,burner_steps,
  pool_paid_attempts,pool:w.available_compute_pool()
 };
 eprintln!("B37_NATIVE {:?}",outcome);
 outcome
}

#[test]
fn b37_global_pool_executes_unproductive_competitors_at_same_real_compute_cost(){
 let schedules=[
  &[3_u64,6,9][..],
  &[6_u64,6,6][..],
  &[1_u64,2,3,4,5,3][..],
 ];
 let mut number=0;
 for supply in schedules{
  for group in [
   Composition::SoloWorker,Composition::BurnerFirst,
   Composition::WorkerFirst,Composition::DualWorker
  ]{
   let outcome=run(group,supply,true);
   number+=1;
   assert_eq!(outcome.steps,18);
   assert_eq!(outcome.pool_paid_attempts,18);
   assert_eq!(outcome.pool,0);
   assert_eq!(outcome.productive*3,outcome.worker_steps,
    "real worker service demands exactly three paid CIL instructions");
   assert_eq!(outcome.burner_steps,outcome.steps-outcome.worker_steps);
   if group==Composition::BurnerFirst||group==Composition::WorkerFirst{
    assert_eq!(outcome.productive,3,
      "a useless equal-scheduled source consumes half the pool, independent of PID");
    assert_eq!((outcome.worker_steps,outcome.burner_steps),(9,9));
   }else{
    assert_eq!(outcome.productive,6);
    assert_eq!(outcome.burner_steps,0);
   }
  }
 }
 assert_eq!(number,12);
 for group in [Composition::SoloWorker,Composition::BurnerFirst]{
  let blocked=run(group,&[18],false);
  assert_eq!(blocked.steps,0);
  assert_eq!(blocked.productive,0);
  assert_eq!(blocked.pool,18,
   "legacy Draw-only zero-credit non-Draw actors are ineligible despite pool");
  number+=1;
 }
 assert_eq!(number,14);
}
