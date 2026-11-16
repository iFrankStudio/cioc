//! CL-1B B26: one CIL founder copies its OWN worker source into a
//! five-op child graph, computes a variant request via ordinary native Add,
//! edits that graph with native GraphSetConst, and launches the competitor.
//! Investigator authors the clone recipe and delta. NO endogenous discovery.
use cl0_world::{EventKind, Intervention, Op, Origin, Process, World};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Arm { Parent4Child3, Parent3Child4, Parent4Child4 }

impl Arm {
    fn parameters(self) -> (i64,i64,i64) {
        match self {
            Arm::Parent4Child3 => (4,-1,3),
            Arm::Parent3Child4 => (3,1,4),
            Arm::Parent4Child4 => (4,0,4),
        }
    }
}

fn founder(base: i64, delta: i64) -> Vec<Op> {
    vec![
        // Native graph construction: the source code at PC30..34 is
        // the SAME worker algorithm which will run in the parent, not
        // an inert separately authored helper-only program.
        Op::GraphNew { dst: 0, next: 1 }, // 0
        Op::Const { dst: 1, value: 30, next: 2 }, // 1
        Op::GraphCopyNodeLocal { graph: 0, source: 1, dst: 3, next: 3 }, // 2
        Op::Const { dst: 1, value: 31, next: 4 }, // 3
        Op::GraphCopyNodeLocal { graph: 0, source: 1, dst: 3, next: 5 }, // 4
        Op::Const { dst: 1, value: 32, next: 6 }, // 5
        Op::GraphCopyNodeLocal { graph: 0, source: 1, dst: 3, next: 7 }, // 6
        Op::Const { dst: 1, value: 33, next: 8 }, // 7
        Op::GraphCopyNodeLocal { graph: 0, source: 1, dst: 3, next: 9 }, // 8
        Op::Const { dst: 1, value: 34, next: 10 }, // 9
        Op::GraphCopyNodeLocal { graph: 0, source: 1, dst: 3, next: 11 }, // 10
        // Four explicit CIL graph edits of copied forward edges.
        Op::Const { dst: 1, value: 0, next: 12 }, // 11
        Op::Const { dst: 2, value: 1, next: 13 }, // 12
        Op::GraphSetNext { graph: 0, node: 1, target: 2, next: 14 }, // 13
        Op::Const { dst: 1, value: 1, next: 15 }, // 14
        Op::Const { dst: 2, value: 2, next: 16 }, // 15
        Op::GraphSetNext { graph: 0, node: 1, target: 2, next: 17 }, // 16
        Op::Const { dst: 1, value: 2, next: 18 }, // 17
        Op::Const { dst: 2, value: 3, next: 19 }, // 18
        Op::GraphSetNext { graph: 0, node: 1, target: 2, next: 20 }, // 19
        Op::Const { dst: 1, value: 3, next: 21 }, // 20
        Op::Const { dst: 2, value: 4, next: 22 }, // 21
        Op::GraphSetNext { graph: 0, node: 1, target: 2, next: 23 }, // 22
        // CIL itself derives the child's new policy from the parent's
        // worker request (base) and authored, ordinary signed delta.
        Op::Const { dst: 1, value: 0, next: 24 }, // 23 graph node0
        Op::Const { dst: 2, value: base, next: 25 }, // 24
        Op::Const { dst: 3, value: delta, next: 26 }, // 25
        Op::Add { dst: 2, lhs: 2, rhs: 3, next: 27 }, // 26
        Op::GraphSetConst { graph: 0, node: 1, value: 2, next: 28 }, // 27
        Op::Const { dst: 1, value: 2, next: 29 }, // 28 child startup credits
        Op::GraphLaunch { graph: 0, credits: 1, dst: 3, next: 30 }, // 29
        // The original worker/parent program, starting at PC30.
        Op::Const { dst: 0, value: base, next: 31 }, // 30
        Op::DrawCompute { amount: 0, next: 32 }, // 31
        Op::Const { dst: 2, value: 9, next: 33 }, // 32
        Op::Const { dst: 3, value: 1, next: 34 }, // 33
        Op::Write { address: 2, value: 3, next: 31 }, // 34
    ]
}

#[derive(Debug)]
struct Trial {
    arm: Arm,
    supply: [u64;6],
    service_parent: usize,
    service_child: usize,
    service_by_packet: [(usize,usize);6],
    native_attempts: usize,
    residual_pool: u64,
    final_positions: [(usize,u64);2],
}

fn step_until_blocked(w: &mut World) -> usize {
    let mut n=0;
    loop {
        assert!(n<100,"finite charged energy budget forbids unbounded work");
        let before=w.total_compute_inventory().unwrap();
        let events=w.events().len();
        if !w.step() {
            assert_eq!(w.total_compute_inventory(),Some(before));
            assert_eq!(w.events().len(),events);
            break;
        }
        n+=1;
        assert_eq!(w.total_compute_inventory(),Some(before-1));
        assert_eq!(w.events().len(),events+1);
        assert!(matches!(w.events().last().unwrap().origin,
            Origin::Internal(1) | Origin::Internal(2)));
        assert!(!matches!(w.events().last().unwrap().kind,
            EventKind::Trapped(_) | EventKind::Halted));
    }
    n
}

fn count_service(w:&World,pid:u64,pc:usize)->usize {
    w.events().iter().filter(|e|matches!(
        (&e.origin,&e.kind),
        (Origin::Internal(owner),EventKind::CellWritten{key:9,value:1})
            if *owner==pid && e.pc==pc
    )).count()
}

fn trial(arm:Arm,supply:[u64;6])->Trial {
    let (base,delta,child_request)=arm.parameters();
    let mut w=World::with_strict_capacity(16,64,128)
        .with_pool_paid_draw()
        .with_recoverable_draw_shortage();
    assert_eq!(founder(base,delta).len(),35);
    w.load(Process::new(1,founder(base,delta),34)).unwrap();
    assert!(w.process(2).is_none());
    assert_eq!(w.total_compute_inventory(),Some(34));
    assert_eq!(w.events().iter().filter(|e|e.origin==Origin::External).count(),1);

    let bootstrap=step_until_blocked(&mut w);
    assert_eq!(bootstrap,34);
    assert_eq!(w.total_compute_inventory(),Some(0));
    assert_eq!(w.graph_count(),0,"launched graph moves into child, not minting storage");
    assert_eq!(w.process(1).unwrap().pc,31);
    assert_eq!(w.process(2).unwrap().pc,1);
    assert_eq!(w.process(1).unwrap().credits,0);
    assert_eq!(w.process(2).unwrap().credits,0);
    assert_eq!(w.process(1).unwrap().code.nodes.len(),35);
    assert_eq!(w.process(2).unwrap().code.nodes.len(),5);
    assert!(matches!(&w.process(1).unwrap().code.nodes[30],
        Op::Const{value,..} if *value==base));
    assert!(matches!(&w.process(2).unwrap().code.nodes[0],
        Op::Const{value,next:1,..} if *value==child_request));
    assert_eq!(
        &w.process(2).unwrap().code.nodes[1..],
        &[
            Op::DrawCompute{amount:0,next:2},
            Op::Const{dst:2,value:9,next:3},
            Op::Const{dst:3,value:1,next:4},
            Op::Write{address:2,value:3,next:1},
        ]
    );

    let graph_created=w.events().iter().filter(|e|matches!(
        (&e.origin,&e.kind),
        (Origin::Internal(1),EventKind::GraphCreated{owner:1,..})
    )).count();
    let copied=w.events().iter().filter(|e|matches!(
        (&e.origin,&e.kind),
        (Origin::Internal(1),EventKind::GraphNodeCopiedLocal{
            owner:1,source_process:1,source_node:30..=34,..
        })
    )).count();
    let modified_edges=w.events().iter().filter(|e|matches!(
        (&e.origin,&e.kind),
        (Origin::Internal(1),EventKind::GraphNodeEdited{owner:1,edit,..})
            if edit=="next"
    )).count();
    let modified_policy=w.events().iter().filter(|e|matches!(
        (&e.origin,&e.kind),
        (Origin::Internal(1),EventKind::GraphNodeEdited{owner:1,node:0,edit,..})
            if edit=="const"
    )).count();
    let launched=w.events().iter().filter(|e|matches!(
        (&e.origin,&e.kind),
        (Origin::Internal(1),EventKind::GraphLaunched{
            parent_process:1,child_process:2,transferred_credits:2,moved_nodes:5,..
        })
    )).count();
    assert_eq!((graph_created,copied,modified_edges,modified_policy,launched),(1,5,4,1,1));
    assert_eq!(count_service(&w,1,34),0);
    assert_eq!(count_service(&w,2,4),0);

    let mut native_attempts=bootstrap;
    let mut service_by_packet=[(0,0);6];
    let mut prev_p=0;
    let mut prev_c=0;
    for (i,amount) in supply.into_iter().enumerate() {
        let before=w.total_compute_inventory().unwrap();
        let pool=w.available_compute_pool();
        w.intervene(Intervention::SetComputePool{credits:pool+amount}).unwrap();
        assert_eq!(w.total_compute_inventory(),Some(before+amount));
        assert_eq!(w.events().last().unwrap().origin,Origin::External);
        native_attempts+=step_until_blocked(&mut w);
        let p=count_service(&w,1,34);
        let c=count_service(&w,2,4);
        service_by_packet[i]=(p-prev_p,c-prev_c);
        prev_p=p;prev_c=c;
    }
    assert_eq!(w.events().iter().filter(|e|e.origin==Origin::External).count(),7);
    assert_eq!(w.total_compute_inventory(),Some(
        34+supply.iter().sum::<u64>()-native_attempts as u64
    ));
    assert!(w.occupied_units()<=128);
    assert!(!w.process(1).unwrap().stopped);
    assert!(!w.process(2).unwrap().stopped);
    let result=Trial{
        arm,supply,service_parent:prev_p,service_child:prev_c,service_by_packet,
        native_attempts,residual_pool:w.available_compute_pool(),
        final_positions:[
            (w.process(1).unwrap().pc,w.process(1).unwrap().credits),
            (w.process(2).unwrap().pc,w.process(2).unwrap().credits),
        ],
    };
    eprintln!("B26_NATIVE {:?}",result);
    result
}

#[test]
fn b26_native_computed_mutation_is_inherited_by_first_cil_born_child_and_filtered_by_neutral_pool() {
    let schedules=[
        [4,4,4,4,4,4],
        [5,5,5,5,5,5],
        [4,5,4,5,4,5],
        [5,4,5,4,5,4],
    ];
    let mut tested=0;
    for (i,schedule) in schedules.into_iter().enumerate() {
        let pc=trial(Arm::Parent4Child3,schedule);
        let cp=trial(Arm::Parent3Child4,schedule);
        let same=trial(Arm::Parent4Child4,schedule);
        tested+=3;
        for v in [&pc,&cp,&same] {
            assert_eq!(v.supply,schedule);
            assert_eq!(v.service_by_packet.iter().map(|x|x.0).sum::<usize>(),v.service_parent);
            assert_eq!(v.service_by_packet.iter().map(|x|x.1).sum::<usize>(),v.service_child);
            assert!(v.final_positions.iter().all(|&(_,credit)|credit==0));
        }
        if i==0 {
            assert_eq!((pc.service_parent,pc.service_child),(0,6),
                "mutation yields child request3 and wins scarce 4-credit packets");
            assert_eq!((cp.service_parent,cp.service_child),(6,0),
                "ID-swapped parent request3 also wins scarce packets");
            // Equal request4 siblings can accumulate unspent packets, so
            // cannot assume that they fail forever on 4-credit pulses.
            assert!(same.service_parent+same.service_child>0);
        }
    }
    assert_eq!(tested,12);
}
