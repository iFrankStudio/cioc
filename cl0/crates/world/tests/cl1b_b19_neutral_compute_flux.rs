//! CL-1B B19: neutral external world-pool flux and paid native CIL acquisition.
//! The host supplies the ENVIRONMENT, never a designated actor's credits.
//! This demonstrates feasibility of access, NOT emergent metabolism or life.
use cl0_world::{EventKind, Intervention, Op, Origin, Process, World};

#[derive(Clone, Copy, Debug)]
enum Arm {
    RenewableAccess,
    NoDrawOpcode,
    DrawNotPoolPaid,
    SubthresholdThenCrossing,
    NoInflux,
}

fn founder(draw: bool) -> Vec<Op> {
    vec![
        Op::Const { dst: 0, value: 4, next: 1 },
        if draw {
            Op::DrawCompute { amount: 0, next: 2 }
        } else {
            Op::Const { dst: 1, value: 0, next: 2 }
        },
        Op::Const { dst: 2, value: 9, next: 3 },
        Op::Const { dst: 3, value: 1, next: 4 },
        Op::Write { address: 2, value: 3, next: 1 },
    ]
}

// Independently audit every charged transition, rather than only comparing
// beginning/end aggregate counters.
fn native_attempts_until_blocked(w: &mut World) -> usize {
    let mut n = 0_usize;
    loop {
        assert!(n < 100, "bounded packet must not supply unlimited attempts");
        let before = w.total_compute_inventory().expect("bounded native inventory");
        let events_before = w.events().len();
        if !w.step() {
            assert_eq!(before, w.total_compute_inventory().unwrap());
            assert_eq!(events_before, w.events().len());
            break;
        }
        n += 1;
        let after = w.total_compute_inventory().expect("bounded native inventory");
        assert_eq!(before - after, 1, "every native attempt costs one unit");
        assert_eq!(w.events().len(), events_before + 1);
        let event = w.events().last().unwrap();
        assert_eq!(event.origin, Origin::Internal(1));
        assert_eq!(event.process, 1);
    }
    n
}

fn external_neutral_pool_packet(w: &mut World, packet: u64) {
    let before = w.total_compute_inventory().unwrap();
    let old_pool = w.available_compute_pool();
    let events_before = w.events().len();
    w.intervene(Intervention::SetComputePool {
        credits: old_pool.checked_add(packet).expect("no overflow"),
    })
    .unwrap();
    assert_eq!(w.total_compute_inventory(), Some(before + packet));
    assert_eq!(w.available_compute_pool(), old_pool + packet);
    assert_eq!(w.events().len(), events_before + 1);
    let event = w.events().last().unwrap();
    assert_eq!(event.origin, Origin::External);
    assert!(matches!(event.kind, EventKind::Intervened(_)));
}

#[test]
fn b19_identity_neutral_pulses_support_cil_draw_but_never_mint_native_compute() {
    for arm in [
        Arm::RenewableAccess,
        Arm::NoDrawOpcode,
        Arm::DrawNotPoolPaid,
        Arm::SubthresholdThenCrossing,
        Arm::NoInflux,
    ] {
        let can_draw = !matches!(arm, Arm::NoDrawOpcode);
        let mut world = World::with_strict_capacity(16, 64, 128)
            .with_recoverable_draw_shortage();
        if !matches!(arm, Arm::DrawNotPoolPaid) {
            world = world.with_pool_paid_draw();
        }
        world.load(Process::new(1, founder(can_draw), 2)).unwrap();
        assert_eq!(world.total_compute_inventory(), Some(2));
        assert_eq!(world.available_compute_pool(), 0);
        assert_eq!(founder(can_draw).len(), 5);

        let startup = native_attempts_until_blocked(&mut world);
        assert_eq!(startup, 2, "{arm:?}");
        assert_eq!(world.process(1).unwrap().credits, 0);
        assert_eq!(world.total_compute_inventory(), Some(0));
        assert!(!world.process(1).unwrap().stopped);

        let packets: &[u64] = match arm {
            Arm::RenewableAccess | Arm::NoDrawOpcode | Arm::DrawNotPoolPaid => &[5, 5, 5, 5, 5, 5],
            Arm::SubthresholdThenCrossing => &[4, 1],
            Arm::NoInflux => &[],
        };
        let mut post_packet_attempts = 0_usize;
        for (index, &packet) in packets.iter().enumerate() {
            external_neutral_pool_packet(&mut world, packet);
            let attempts = native_attempts_until_blocked(&mut world);
            post_packet_attempts += attempts;
            match arm {
                Arm::RenewableAccess => {
                    assert_eq!(attempts, 5, "each 5-credit environmental input funds five charged actions");
                    assert_eq!(world.process(1).unwrap().pc, 1);
                    assert_eq!(world.process(1).unwrap().credits, 0);
                    assert_eq!(world.available_compute_pool(), 0);
                }
                Arm::SubthresholdThenCrossing => {
                    assert_eq!(attempts, if index == 0 { 0 } else { 5 });
                    assert_eq!(world.available_compute_pool(), if index == 0 { 4 } else { 0 });
                }
                Arm::NoDrawOpcode | Arm::DrawNotPoolPaid => {
                    assert_eq!(attempts, 0, "{arm:?}: pool availability alone cannot pay arbitrary opcodes");
                }
                Arm::NoInflux => unreachable!(),
            }
        }

        let writes = world.events().iter().filter(|event| {
            event.origin == Origin::Internal(1)
                && event.pc == 4
                && matches!(event.kind, EventKind::CellWritten { key: 9, value: 1 })
        }).count();
        let pool_paid_draws = world.events().iter().filter(|event| {
            event.origin == Origin::Internal(1)
                && event.pc == 1
                && event.pool_paid_attempt
        }).count();
        let deferred_draws = world.events().iter().filter(|event| {
            event.origin == Origin::Internal(1)
                && event.pc == 1
                && matches!(event.kind, EventKind::DrawDeferred { .. })
        }).count();

        let (expected_writes, expected_draws, expected_attempts, expected_pool) = match arm {
            Arm::RenewableAccess => (6, 6, 30, 0),
            Arm::NoDrawOpcode => (0, 0, 0, 30),
            Arm::DrawNotPoolPaid => (0, 0, 0, 30),
            Arm::SubthresholdThenCrossing => (1, 1, 5, 0),
            Arm::NoInflux => (0, 0, 0, 0),
        };
        assert_eq!(writes, expected_writes, "{arm:?}");
        assert_eq!(pool_paid_draws, expected_draws, "{arm:?}");
        assert_eq!(post_packet_attempts, expected_attempts, "{arm:?}");
        assert_eq!(world.available_compute_pool(), expected_pool, "{arm:?}");
        assert_eq!(world.process(1).unwrap().credits, 0);
        assert!(!world.process(1).unwrap().stopped);
        assert!(world.process(2).is_none());
        assert!(world.occupied_units() <= 128);
        assert!(deferred_draws >= usize::from(can_draw));
        assert_eq!(
            world.events().iter().filter(|e| e.origin == Origin::External).count(),
            1 + packets.len(),
            "one externally loaded founder, then only identity-neutral world-pool pulses"
        );
        // The actor can execute more than its initial local2 credits only
        // when external pool inputs were made accessible through native Draw.
        if matches!(arm, Arm::RenewableAccess) {
            assert!(post_packet_attempts > 2);
        }
    }
}
