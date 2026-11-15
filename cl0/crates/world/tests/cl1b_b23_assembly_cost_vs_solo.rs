//! CL-1B B23: cost/fitness counterexample to preauthored native relay assembly.
//! Same initial one process, 29 credits, and neutral world-pool packets.
//! Comparing investigator-authored different-sized code is an efficiency
//! baseline, NOT evidence of actor-discovered alternatives or cyberlife.
#[path = "cl1b_b22_native_relay_assembly.rs"]
mod canonical_b22;

use cl0_world::{EventKind, Intervention, Op, Origin, Process, World};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Arm {
    AssembledTransfer,
    AssembledNoTransfer,
    Solo(i64),
}

fn solo(request: i64) -> Vec<Op> {
    vec![
        Op::Const { dst: 0, value: request, next: 1 },
        Op::DrawCompute { amount: 0, next: 2 },
        Op::Const { dst: 2, value: 9, next: 3 },
        Op::Const { dst: 3, value: 1, next: 4 },
        Op::Write { address: 2, value: 3, next: 1 },
    ]
}

fn blocked(w: &mut World) -> usize {
    let mut count = 0_usize;
    loop {
        assert!(count < 100, "no world may execute indefinitely on a bounded packet");
        let before = w.total_compute_inventory().unwrap();
        let events = w.events().len();
        if !w.step() {
            assert_eq!(w.total_compute_inventory(), Some(before));
            assert_eq!(w.events().len(), events);
            break;
        }
        count += 1;
        assert_eq!(w.total_compute_inventory(), Some(before - 1));
        assert_eq!(w.events().len(), events + 1);
        assert!(matches!(w.events().last().unwrap().origin,
            Origin::Internal(1) | Origin::Internal(2)));
    }
    count
}

fn service_count(w: &World) -> usize {
    w.events().iter().filter(|e| matches!(
        (&e.origin, &e.kind),
        (Origin::Internal(1), EventKind::CellWritten { key: 9, value: 1 })
    )).count()
}

#[derive(Debug)]
struct Observation {
    arm: Arm,
    schedule: [u64; 2],
    founder_nodes: usize,
    bootstrap_attempts: usize,
    after_packet_attempts: [usize; 2],
    service_writes: usize,
    created_children: usize,
    remaining_pool: u64,
    ending_credits: u64,
    storage_units: usize,
}

fn observe(arm: Arm, schedule: [u64; 2]) -> Observation {
    let founder = match arm {
        Arm::AssembledTransfer => canonical_b22::founder(true),
        Arm::AssembledNoTransfer => canonical_b22::founder(false),
        Arm::Solo(request) => solo(request),
    };
    let founder_nodes = founder.len();
    let mut w = World::with_strict_capacity(16, 64, 128)
        .with_pool_paid_draw()
        .with_recoverable_draw_shortage();
    w.load(Process::new(1, founder, 29)).unwrap();
    assert!(w.process(2).is_none());
    assert_eq!(w.total_compute_inventory(), Some(29));

    let bootstrap_attempts = blocked(&mut w);
    assert_eq!(bootstrap_attempts, 29);
    assert_eq!(service_count(&w), 0, "no credited service before energy influx");
    assert_eq!(w.total_compute_inventory(), Some(0));

    let mut after_packet_attempts = [0_usize; 2];
    for (i, packet) in schedule.into_iter().enumerate() {
        let previous_pool = w.available_compute_pool();
        let before = w.total_compute_inventory().unwrap();
        w.intervene(Intervention::SetComputePool {
            credits: previous_pool + packet,
        }).unwrap();
        assert_eq!(w.total_compute_inventory(), Some(before + packet));
        assert_eq!(w.events().last().unwrap().origin, Origin::External);
        after_packet_attempts[i] = blocked(&mut w);
    }

    let created_children = w.events().iter().filter(|e| matches!(
        (&e.origin, &e.kind),
        (Origin::Internal(1), EventKind::GraphLaunched {
            parent_process: 1, child_process: 2, transferred_credits: 2,
            moved_nodes: 5, ..
        })
    )).count();
    assert_eq!(created_children, usize::from(!matches!(arm, Arm::Solo(_))));
    assert_eq!(w.process(2).is_some(), created_children == 1);
    assert_eq!(w.events().iter().filter(|e| e.origin == Origin::External).count(), 3);
    assert_eq!(w.total_compute_inventory(), Some(
        29 + schedule[0] + schedule[1]
            - bootstrap_attempts as u64
            - after_packet_attempts.iter().map(|v| *v as u64).sum::<u64>()
    ));
    assert!(w.events().iter().all(|e| !matches!(
        e.kind, EventKind::Trapped(_) | EventKind::Halted
    )));
    assert!(w.occupied_units() <= 128);
    for id in 1..=2 {
        if let Some(p) = w.process(id) {
            assert!(!p.stopped);
        }
    }
    Observation {
        arm,
        schedule,
        founder_nodes,
        bootstrap_attempts,
        after_packet_attempts,
        service_writes: service_count(&w),
        created_children,
        remaining_pool: w.available_compute_pool(),
        ending_credits: w.remaining_credits(),
        storage_units: w.occupied_units(),
    }
}

#[test]
fn b23_solo_resource_policy_is_a_nonadaptive_cost_efficiency_falsifier() {
    for schedule in [[3_u64, 5_u64], [4_u64, 4_u64]] {
        let mut trials = Vec::new();
        trials.push(observe(Arm::AssembledTransfer, schedule));
        trials.push(observe(Arm::AssembledNoTransfer, schedule));
        for requested in 1..=7 {
            trials.push(observe(Arm::Solo(requested), schedule));
        }
        assert_eq!(trials.len(), 9);
        assert!(trials.iter().all(|x| x.bootstrap_attempts == 29));
        assert!(trials.iter().all(|x| x.schedule == schedule));
        let built = &trials[0];
        let no_transfer = &trials[1];
        let solo_three = trials.iter().find(|t| t.arm == Arm::Solo(3)).unwrap();

        assert_eq!((built.founder_nodes, built.created_children), (35, 1));
        assert_eq!((no_transfer.founder_nodes, no_transfer.created_children), (35, 1));
        assert_eq!(no_transfer.service_writes, 0);
        assert!(trials.iter().filter(|t| matches!(t.arm, Arm::Solo(_))).all(|t|
            t.founder_nodes == 5 && t.created_children == 0));
        assert_eq!(built.service_writes, 1,
            "preauthored relay should restore exactly one service under both finite schedules");
        assert_eq!(solo_three.service_writes, 2,
            "a researcher-authored cheaper policy produces two outputs on the same energy");
        assert!(solo_three.service_writes > built.service_writes,
            "native graph assembly is not automatically a better adaptive resource policy");
        for trial in &trials {
            assert!(trial.storage_units <= 128);
            eprintln!(
                "B23_NATIVE schedule={:?} arm={:?} nodes={} after_packets={:?} writes={} pool={} credits={} storage={}",
                trial.schedule,trial.arm,trial.founder_nodes,trial.after_packet_attempts,
                trial.service_writes,trial.remaining_pool,trial.ending_credits,trial.storage_units
            );
        }
    }
}
