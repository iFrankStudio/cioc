//! B32 — one continuing native CIL actor pays once to patch its own
//! real service-loop edge and regain a pool-payable Draw after each service.
//! Entire pruning plan and candidate request were authored by investigators.
use cl0_world::{EventKind,Intervention,Op,Origin,Process,World};

#[derive(Clone,Copy,Debug,PartialEq,Eq)]
enum Arm { Prune, PatchSuppressed, StaticThree }

fn pruning_founder()->Vec<Op>{
 vec![
    Op::Const{dst:0,value:9,next:1},              //0
    Op::DrawCompute{amount:0,next:2},             //1
    Op::Const{dst:2,value:9,next:3},              //2
    Op::Const{dst:3,value:1,next:4},              //3
    Op::Write{address:2,value:3,next:5},          //4
    Op::Const{dst:0,value:3,next:6},              //5
    Op::Const{dst:1,value:4,next:7},              //6
    Op::Const{dst:2,value:1,next:8},              //7
    Op::PatchNext{node:1,target:2,next:9},        //8: real PC4.next 5→1
    Op::Const{dst:2,value:9,next:1},              //9
 ]
}
fn static_founder()->Vec<Op>{
 vec![
    Op::Const{dst:0,value:3,next:1},
    Op::DrawCompute{amount:0,next:2},
    Op::Const{dst:2,value:9,next:3},
    Op::Const{dst:3,value:1,next:4},
    Op::Write{address:2,value:3,next:1},
 ]
}
fn service(w:&World,pc:usize)->usize{
 w.events().iter().filter(|e|matches!(
    (&e.origin,&e.kind),
    (Origin::Internal(1),EventKind::CellWritten{key:9,value:1}) if e.pc==pc
 )).count()
}
fn drain(w:&mut World)->usize{
 let mut n=0;
 loop{
    assert!(n<100,"finite environment pool cannot fund infinite native CIL instructions");
    let total=w.total_compute_inventory().unwrap();
    let ev=w.events().len();
    if !w.step(){
        assert_eq!(w.total_compute_inventory(),Some(total));
        assert_eq!(w.events().len(),ev);
        return n;
    }
    n+=1;
    assert_eq!(w.total_compute_inventory(),Some(total-1));
    assert_eq!(w.events().len(),ev+1);
    assert_eq!(w.events().last().unwrap().origin,Origin::Internal(1));
    assert!(!matches!(w.events().last().unwrap().kind,EventKind::Trapped(_) | EventKind::Halted));
 }
}
#[derive(Debug)]
struct ResultTrace {
 arm:Arm,packets:Vec<u64>,service_writes:usize,
 charged:usize,pool:u64,local:u64,pc:usize,
 service_edge:usize,edge_edits:usize,suppressed:usize,steps_by_packet:Vec<usize>,
}
fn trial(arm:Arm,packets:&[u64])->ResultTrace{
 let program=if arm==Arm::StaticThree {static_founder()} else {pruning_founder()};
 let mut w=World::with_strict_capacity(16,64,128)
  .with_pool_paid_draw().with_recoverable_draw_shortage()
  .with_native_executable_edit_trace();
 if arm==Arm::PatchSuppressed{w=w.with_native_patchnext_disabled();}
 w.load(Process::new(1,program,2)).unwrap();
 assert!(w.process(2).is_none());
 let initial=drain(&mut w);
 assert_eq!(initial,2);
 assert_eq!(w.process(1).unwrap().pc,1);
 assert_eq!(w.process(1).unwrap().credits,0);
 let mut charged=initial;
 let mut steps_by_packet=Vec::new();
 for &pkt in packets{
    let before=w.total_compute_inventory().unwrap();
    let old_pool=w.available_compute_pool();
    w.intervene(Intervention::SetComputePool{credits:old_pool+pkt}).unwrap();
    assert_eq!(w.total_compute_inventory(),Some(before+pkt));
    assert_eq!(w.events().last().unwrap().origin,Origin::External);
    let steps=drain(&mut w);steps_by_packet.push(steps);charged+=steps;
 }
 let edge=match &w.process(1).unwrap().code.nodes[if arm==Arm::StaticThree {4}else{4}] {
    Op::Write{next,..}=>*next,
    _=>panic!("B32 service instruction must remain Write"),
 };
 let changed=w.events().iter().filter(|e|matches!(
    (&e.origin,&e.kind),
    (Origin::Internal(1),EventKind::ExecutableEdgePatched{node:4,before:5,after:1})
 )).count();
 let suppressed=w.events().iter().filter(|e|matches!(
    (&e.origin,&e.kind),
    (Origin::Internal(1),EventKind::NativeCodeEditSuppressed{opcode_tag:6})
 )).count();
 assert_eq!(w.events().iter().filter(|e|e.origin==Origin::External).count(),1+packets.len());
 assert_eq!(w.total_compute_inventory(),Some(2+packets.iter().sum::<u64>()-charged as u64));
 assert!(w.process(2).is_none());
 assert!(!w.process(1).unwrap().stopped);
 assert!(w.occupied_units()<=128);
 let out=ResultTrace {
    arm,packets:packets.to_vec(),service_writes:service(&w,4),charged,
    pool:w.available_compute_pool(),local:w.process(1).unwrap().credits,
    pc:w.process(1).unwrap().pc,service_edge:edge,edge_edits:changed,
    suppressed,steps_by_packet,
 };
 eprintln!("B32_NATIVE {:?}",out);
 out
}
#[test]
fn b32_paid_once_native_edge_prune_prevents_non_draw_energy_deadlock_but_costs_more_than_static(){
 let schedules=[
    &[10_u64,4,4,4][..],
    &[10_u64,3,3,3][..],
    &[10_u64,5,5,5][..],
    &[9_u64,4,4,4][..],
 ];
 let mut worlds=0;
 for supply in schedules{
    let pruned=trial(Arm::Prune,supply);
    let suppressed=trial(Arm::PatchSuppressed,supply);
    let static_three=trial(Arm::StaticThree,supply);
    worlds+=3;
    for x in [&pruned,&suppressed,&static_three]{
        assert_eq!(x.packets,supply);
        assert_eq!(x.local,0);
        assert_eq!(x.charged+x.pool as usize,
            2+supply.iter().map(|v|*v as usize).sum::<usize>());
    }
    assert_eq!(static_three.service_edge,1);
    assert_eq!((static_three.edge_edits,static_three.suppressed),(0,0));
    assert_eq!(suppressed.service_edge,5);
    assert_eq!(suppressed.edge_edits,0);
    assert!(suppressed.suppressed>0 || supply[0]==9,
        "source pruning must really be disabled at the same paid PC8");
    if pruned.edge_edits>0{
        assert_eq!(pruned.service_edge,1);
        assert_eq!(pruned.edge_edits,1);
        assert_eq!(pruned.suppressed,0);
        assert_eq!(pruned.pc,1,
            "after paid one-time structural pruning individual must remain pool-Draw-reachable");
    }
    if supply[0]==10{
        assert!(pruned.service_writes>suppressed.service_writes,
            "native edge pruning must improve continuity over charged suppression");
        assert!(static_three.service_writes>=pruned.service_writes,
            "short static founder is an essential full-cost efficiency baseline");
    }
 }
 assert_eq!(worlds,12);
 let zero=trial(Arm::Prune,&[]);
 assert_eq!(zero.service_writes,0);
 assert_eq!(zero.edge_edits,0);
 assert_eq!(zero.pc,1);
}
