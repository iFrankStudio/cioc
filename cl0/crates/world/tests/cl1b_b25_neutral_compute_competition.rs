//! CL-1B B25: equal-energy, equal-size two-policy competition for a
//! neutral world compute pool. Candidates are investigator-authored and fixed.
//! Native scheduler ID bias must be separated from ecological filtering.
use cl0_world::{EventKind, Intervention, Op, Origin, Process, World};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Assignment { Order34, Order43, Solo3, Solo4 }

fn founder(request: i64) -> Vec<Op> {
    vec![
        Op::Const { dst: 0, value: request, next: 1 },
        Op::DrawCompute { amount: 0, next: 2 },
        Op::Const { dst: 2, value: 9, next: 3 },
        Op::Const { dst: 3, value: 1, next: 4 },
        Op::Write { address: 2, value: 3, next: 1 },
    ]
}

#[derive(Debug)]
struct Trial {
    assignment: Assignment,
    schedule: [u64; 6],
    request3_writes: usize,
    request4_writes: usize,
    total_attempts: usize,
    residual_pool: u64,
    charged_after_packets: [usize; 6],
    writers_after_packets: [(usize,usize); 6],
    final_positions: Vec<(i64,usize,u64)>,
}

fn exhausted(w: &mut World) -> usize {
    let mut count=0;
    loop {
        assert!(count<100, "bounded packet work must halt when none eligible");
        let energy = w.total_compute_inventory().unwrap();
        let events = w.events().len();
        if !w.step() {
            assert_eq!(w.total_compute_inventory(),Some(energy));
            assert_eq!(w.events().len(),events);
            return count;
        }
        count += 1;
        assert_eq!(w.total_compute_inventory(),Some(energy-1));
        assert_eq!(w.events().len(),events+1);
        assert!(matches!(w.events().last().unwrap().origin,
            Origin::Internal(1) | Origin::Internal(2)));
        assert!(!matches!(w.events().last().unwrap().kind,EventKind::Trapped(_) | EventKind::Halted));
    }
}

fn measure(assignment: Assignment,schedule:[u64;6]) -> Trial {
    let policy_map: Vec<(u64,i64)> = match assignment {
        Assignment::Order34 => vec![(1,3),(2,4)],
        Assignment::Order43 => vec![(1,4),(2,3)],
        Assignment::Solo3 => vec![(1,3)],
        Assignment::Solo4 => vec![(1,4)],
    };
    let mut w=World::with_strict_capacity(16,64,128)
        .with_pool_paid_draw()
        .with_recoverable_draw_shortage();
    for &(pid,request) in &policy_map {
        w.load(Process::new(pid,founder(request),2)).unwrap();
    }
    let endowment=2*(policy_map.len() as u64);
    assert_eq!(w.total_compute_inventory(),Some(endowment));
    let mut total_attempts=exhausted(&mut w);
    assert_eq!(total_attempts,2*policy_map.len());
    assert!(policy_map.iter().all(|&(pid,_)| {
        let p=w.process(pid).unwrap();
        p.pc==1 && p.credits==0 && !p.stopped && p.code.nodes.len()==5
    }));
    assert!(!w.events().iter().any(|e| matches!(
        (&e.origin,&e.kind),
        (Origin::Internal(_),EventKind::CellWritten {key:9,value:1})
    )));

    let services=|w:&World,pid:u64| w.events().iter().filter(|e| matches!(
        (&e.origin,&e.kind),
        (Origin::Internal(id),EventKind::CellWritten {key:9,value:1})
            if *id==pid && e.pc==4
    )).count();
    let by_request=|w:&World,req:i64| policy_map.iter().filter(|(_,r)|*r==req)
        .map(|(pid,_)|services(w,*pid)).sum::<usize>();
    let mut charged_after_packets=[0_usize;6];
    let mut writers_after_packets=[(0_usize,0_usize);6];
    let mut last3=0;
    let mut last4=0;
    for (index,packet) in schedule.into_iter().enumerate() {
        let energy_before=w.total_compute_inventory().unwrap();
        let pool_before=w.available_compute_pool();
        w.intervene(Intervention::SetComputePool{credits:pool_before+packet}).unwrap();
        assert_eq!(w.total_compute_inventory(),Some(energy_before+packet));
        assert_eq!(w.events().last().unwrap().origin,Origin::External);
        let delta=exhausted(&mut w);
        charged_after_packets[index]=delta;
        total_attempts+=delta;
        let now3=by_request(&w,3);
        let now4=by_request(&w,4);
        writers_after_packets[index]=(now3-last3,now4-last4);
        last3=now3;last4=now4;
    }
    let final_positions=policy_map.iter().map(|&(pid,request)|{
        let p=w.process(pid).unwrap();
        assert!(!p.stopped);
        (request,p.pc,p.credits)
    }).collect::<Vec<_>>();
    assert_eq!(w.events().iter().filter(|e|e.origin==Origin::External).count(),
        policy_map.len()+schedule.len());
    assert_eq!(w.total_compute_inventory(),Some(
        endowment+schedule.iter().sum::<u64>()-total_attempts as u64
    ));
    assert!(w.occupied_units()<=128);
    assert_eq!(w.graph_count(),0);
    let output=Trial{
        assignment,schedule,request3_writes:last3,request4_writes:last4,
        total_attempts,residual_pool:w.available_compute_pool(),
        charged_after_packets,writers_after_packets,final_positions,
    };
    eprintln!("B25_NATIVE {:?}",output);
    output
}

#[test]
fn b25_neutral_pool_ecological_filter_must_survive_id_swap_without_claiming_evolution() {
    let schedules=[
        [4,4,4,4,4,4],
        [5,5,5,5,5,5],
        [4,5,4,5,4,5],
        [5,4,5,4,5,4],
    ];
    let mut verified=0;
    for (i,schedule) in schedules.into_iter().enumerate() {
        let a=measure(Assignment::Order34,schedule);
        let b=measure(Assignment::Order43,schedule);
        let solo3=measure(Assignment::Solo3,schedule);
        let solo4=measure(Assignment::Solo4,schedule);
        verified+=4;
        for trial in [&a,&b,&solo3,&solo4] {
            assert_eq!(trial.schedule,schedule);
            assert_eq!(trial.total_attempts+trial.residual_pool as usize+
                trial.final_positions.iter().map(|(_,_,c)|*c as usize).sum::<usize>(),
                2*trial.final_positions.len()+schedule.iter().map(|v|*v as usize).sum::<usize>(),
                "world resource conservation must include every native actor");
            assert_eq!(trial.writers_after_packets.iter().map(|pair|pair.0).sum::<usize>(),
                trial.request3_writes);
            assert_eq!(trial.writers_after_packets.iter().map(|pair|pair.1).sum::<usize>(),
                trial.request4_writes);
        }
        assert_eq!(solo3.request4_writes,0);
        assert_eq!(solo4.request3_writes,0);
        if i==0 {
            assert_eq!((a.request3_writes,a.request4_writes),(6,0));
            assert_eq!((b.request3_writes,b.request4_writes),(6,0));
            assert_eq!((solo3.request3_writes,solo4.request4_writes),(6,0),
                "a lone request4 actor cannot pay its 5-credit minimum from 4-credit pulses");
        }
    }
    assert_eq!(verified,16);
}
