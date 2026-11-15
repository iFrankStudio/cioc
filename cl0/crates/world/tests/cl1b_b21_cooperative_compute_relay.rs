//! CL-1B B21: CIL-to-CIL resource relay revives a blocked worker without
//! host SetCredits, dedicated RESCUE opcode or special organism privilege.
//! The founder roles and recipient PID are researcher-designed controls.
use cl0_world::{EventKind, Intervention, Op, Origin, Process, World};

#[derive(Clone, Copy, Debug)]
enum Arm {
    Relay,
    NoTransfer,
    NoRelay,
}

fn worker() -> Vec<Op> {
    vec![
        Op::Const { dst: 0, value: 2, next: 1 },
        Op::DrawCompute { amount: 0, next: 2 },
        Op::Const { dst: 2, value: 9, next: 3 },
        Op::Const { dst: 3, value: 1, next: 4 },
        Op::Write { address: 2, value: 3, next: 1 },
    ]
}

fn relay(transfer: bool) -> Vec<Op> {
    vec![
        Op::Const { dst: 0, value: 4, next: 1 },
        Op::DrawCompute { amount: 0, next: 2 },
        Op::Const { dst: 2, value: 1, next: 3 },
        Op::Const { dst: 3, value: 1, next: 4 },
        if transfer {
            Op::TransferCompute { recipient: 2, amount: 3, next: 1 }
        } else {
            Op::Const { dst: 1, value: 0, next: 1 }
        },
    ]
}

fn charged_work_until_blocked(w: &mut World) -> usize {
    let mut n = 0_usize;
    loop {
        assert!(n < 50, "finite two-packet supply cannot fund unbounded native steps");
        let before = w.total_compute_inventory().expect("bounded compute inventory");
        let event_count = w.events().len();
        if !w.step() {
            assert_eq!(w.total_compute_inventory(), Some(before));
            assert_eq!(w.events().len(), event_count);
            return n;
        }
        n += 1;
        assert_eq!(w.total_compute_inventory(), Some(before - 1));
        assert_eq!(w.events().len(), event_count + 1);
        assert!(
            matches!(w.events().last().unwrap().origin, Origin::Internal(1) | Origin::Internal(2)),
            "every paid attempt is CIL-originated"
        );
    }
}

fn supply_neutral_pool(w: &mut World, amount: u64) {
    let before = w.total_compute_inventory().unwrap();
    let previous = w.available_compute_pool();
    w.intervene(Intervention::SetComputePool { credits: previous + amount })
        .unwrap();
    assert_eq!(w.total_compute_inventory(), Some(before + amount));
    assert_eq!(w.available_compute_pool(), previous + amount);
    let event = w.events().last().unwrap();
    assert_eq!(event.origin, Origin::External);
    assert!(matches!(event.kind, EventKind::Intervened(_)));
}

#[test]
fn b21_native_cil_companion_relay_restores_blocked_worker_under_neutral_compute_pool() {
    for arm in [Arm::Relay, Arm::NoTransfer, Arm::NoRelay] {
        let mut w = World::with_strict_capacity(16, 64, 128)
            .with_pool_paid_draw()
            .with_recoverable_draw_shortage();
        w.load(Process::new(1, worker(), 2)).unwrap();
        if !matches!(arm, Arm::NoRelay) {
            w.load(Process::new(2, relay(matches!(arm, Arm::Relay)), 2))
                .unwrap();
        }
        let initial_inventory = if matches!(arm, Arm::NoRelay) { 2 } else { 4 };
        assert_eq!(w.total_compute_inventory(), Some(initial_inventory));

        let initial_attempts = charged_work_until_blocked(&mut w);
        assert_eq!(initial_attempts, if matches!(arm, Arm::NoRelay) { 2 } else { 4 });
        assert_eq!(w.process(1).unwrap().pc, 1);
        assert_eq!(w.process(1).unwrap().credits, 0);
        if let Some(p) = w.process(2) {
            assert_eq!(p.pc, 1);
            assert_eq!(p.credits, 0);
        }

        supply_neutral_pool(&mut w, 3);
        let after_three = charged_work_until_blocked(&mut w);
        assert_eq!(after_three, 3);
        assert_eq!(w.process(1).unwrap().pc, 4);
        assert_eq!(w.process(1).unwrap().credits, 0);
        assert_eq!(w.available_compute_pool(), 0);
        assert_eq!(
            w.events().iter().filter(|e| matches!(
                (&e.origin, &e.kind),
                (Origin::Internal(1), EventKind::CellWritten { key: 9, value: 1 })
            )).count(),
            0,
            "worker genuinely blocked before it can execute its output instruction"
        );
        if let Some(p) = w.process(2) {
            assert_eq!(p.pc, 1);
            assert_eq!(p.credits, 0);
        }

        supply_neutral_pool(&mut w, 5);
        // Measure actual worker credit movement at the exact helper PC4
        // instruction; EventKind::Executed alone cannot distinguish Transfer
        // from the negative-control Const with the same PC.
        let mut after_five = 0_usize;
        let mut witnessed_credit_relays = 0_usize;
        loop {
            assert!(after_five < 20);
            let before_e = w.total_compute_inventory().unwrap();
            let worker_before = w.process(1).unwrap().credits;
            let helper_before_pc = w.process(2).map(|p| p.pc);
            let previous_events = w.events().len();
            if !w.step() {
                assert_eq!(w.events().len(), previous_events);
                assert_eq!(w.total_compute_inventory(), Some(before_e));
                break;
            }
            after_five += 1;
            assert_eq!(w.events().len(), previous_events + 1);
            assert_eq!(w.total_compute_inventory(), Some(before_e - 1));
            if w.events().last().unwrap().origin == Origin::Internal(2)
                && helper_before_pc == Some(4)
            {
                if matches!(arm, Arm::Relay) {
                    assert_eq!(w.process(1).unwrap().credits, worker_before + 1,
                        "native helper transfer must credit worker directly");
                    witnessed_credit_relays += 1;
                } else {
                    assert_eq!(w.process(1).unwrap().credits, worker_before,
                        "matched Const at relay PC4 must not credit worker");
                }
            }
        }
        assert_eq!(witnessed_credit_relays, usize::from(matches!(arm, Arm::Relay)));
        assert_eq!(
            after_five,
            if matches!(arm, Arm::NoRelay) { 0 } else { 5 },
            "{arm:?}"
        );
        let native_service_count = w.events().iter().filter(|e| matches!(
            (&e.origin, &e.kind),
            (Origin::Internal(1), EventKind::CellWritten { key: 9, value: 1 })
                if e.pc == 4
        )).count();
        let helper_pc4_count = w.events().iter().filter(|e| matches!(
            (&e.origin, &e.kind),
            (Origin::Internal(2), EventKind::Executed) if e.pc == 4
        )).count();
        let expected_services = usize::from(matches!(arm, Arm::Relay));
        assert_eq!(native_service_count, expected_services, "{arm:?}");
        assert_eq!(helper_pc4_count, usize::from(!matches!(arm, Arm::NoRelay)),
                   "both helper policies execute PC4; only TransferCompute moves worker credits");
        assert_eq!(w.process(1).unwrap().pc, if expected_services == 1 { 1 } else { 4 });
        assert_eq!(w.process(1).unwrap().credits, 0);
        assert_eq!(w.available_compute_pool(),
                   if matches!(arm, Arm::NoRelay) { 5 } else { 0 });
        assert_eq!(w.total_compute_inventory(),
                   Some(if matches!(arm, Arm::NoRelay) { 5 } else { 0 }));
        assert_eq!(
            w.events().iter().filter(|e| e.origin == Origin::External).count(),
            if matches!(arm, Arm::NoRelay) { 3 } else { 4 },
            "only original founder loads and two neutral world supply packets are external"
        );
        assert!(w.process(1).is_some());
        if let Some(helper) = w.process(2) {
            assert!(!helper.stopped);
            assert_eq!(helper.pc, 1);
            assert_eq!(helper.credits, 0);
        }
        assert!(!w.process(1).unwrap().stopped);
        assert!(w.occupied_units() <= 128);
        assert!(w.events().iter().all(|e| {
            !matches!(e.kind, EventKind::Trapped(_) | EventKind::GraphLaunched { .. })
        }));
        assert_eq!(w.run(10), 0,
                   "no indefinite runtime without further external supply");
    }
}
