//! CL-1B B20: source-defined lack of CIL-visible recovery path after
//! blocked / insufficient DrawCompute, with neutral supply and fixed request sizes.
//! Human-authored program and policies: NOT autonomous learning or cyberlife.
use cl0_world::{EventKind, Intervention, Op, Origin, Process, World};

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
struct Window {
    attempts: usize,
    credits: u64,
    pool: u64,
    pc: usize,
    writes: usize,
    draws: usize,
}

fn run_until_blocked(w: &mut World) -> usize {
    let mut n = 0;
    loop {
        assert!(n < 100, "bounded packet cannot fund unbounded instruction attempts");
        let before_e = w.total_compute_inventory().expect("bounded inventory");
        let before_events = w.events().len();
        if !w.step() {
            assert_eq!(w.total_compute_inventory(), Some(before_e));
            assert_eq!(w.events().len(), before_events);
            break;
        }
        n += 1;
        assert_eq!(w.total_compute_inventory(), Some(before_e - 1));
        assert_eq!(w.events().len(), before_events + 1);
        assert_eq!(w.events().last().unwrap().origin, Origin::Internal(1));
    }
    n
}

fn assess(request: i64, packet: u64) -> (Vec<Window>, World) {
    let mut w = World::with_strict_capacity(16, 64, 128)
        .with_pool_paid_draw()
        .with_recoverable_draw_shortage();
    w.load(Process::new(1, founder(request), 2)).unwrap();
    assert_eq!(run_until_blocked(&mut w), 2);
    assert_eq!(w.process(1).unwrap().pc, 1);
    assert_eq!(w.process(1).unwrap().credits, 0);
    assert!(!w.process(1).unwrap().stopped);
    let packets = if packet == 0 { 0 } else { 4 };
    let mut windows = Vec::new();
    for index in 0..packets {
        let before_total = w.total_compute_inventory().unwrap();
        let before_events = w.events().len();
        let old_pool = w.available_compute_pool();
        w.intervene(Intervention::SetComputePool {
            credits: old_pool + packet,
        }).unwrap();
        assert_eq!(w.total_compute_inventory(), Some(before_total + packet));
        assert_eq!(w.events().len(), before_events + 1);
        assert_eq!(w.events().last().unwrap().origin, Origin::External);
        let events_at_start = w.events().len();
        let attempts = run_until_blocked(&mut w);
        if request == 4 && packet == 4 && index == 0 {
            assert_eq!(attempts, 0,
                "not enough pool to pay both Draw instruction fee and 4-credit transfer");
            assert_eq!(events_at_start, w.events().len(),
                "blocked Draw produces NO CIL event or readable failure status");
            assert_eq!(w.available_compute_pool(), 4);
            assert_eq!(w.process(1).unwrap().pc, 1);
        }
        let count = |key: i64, pc: usize| {
            w.events().iter().filter(|e| {
                e.origin == Origin::Internal(1) && e.pc == pc
                    && matches!(&e.kind,
                        EventKind::CellWritten { key: written_key, value: 1 }
                            if *written_key == key)
            }).count()
        };
        let draws = w.events().iter().filter(|e| {
            e.origin == Origin::Internal(1) && e.pc == 1
                && e.pool_paid_attempt
                && !matches!(e.kind, EventKind::Trapped(_) | EventKind::DrawDeferred { .. })
        }).count();
        windows.push(Window {
            attempts,
            credits: w.process(1).unwrap().credits,
            pool: w.available_compute_pool(),
            pc: w.process(1).unwrap().pc,
            writes: count(9, 4),
            draws,
        });
    }
    (windows, w)
}

#[test]
fn b20_request_threshold_and_unobservable_shortage_are_distinct_from_adaptation() {
    let cases: &[(i64, u64, &[usize], usize, usize, u64, usize)] = &[
        // request, pulse, per-window native attempts, services, Draws, ending pool, ending PC
        (2, 4, &[3, 0, 0, 0], 0, 1, 13, 4),
        (3, 4, &[4, 4, 4, 4], 4, 4, 0, 1),
        (4, 4, &[0, 5, 5, 5], 3, 3, 1, 1),
        (2, 5, &[3, 0, 0, 0], 0, 1, 17, 4),
        (3, 5, &[4, 4, 4, 8], 5, 5, 0, 1),
        (4, 5, &[5, 5, 5, 5], 4, 4, 0, 1),
        (4, 0, &[], 0, 0, 0, 1),
    ];
    for &(request, packet, expected_attempts, services, draws, pool, pc) in cases {
        let (windows, w) = assess(request, packet);
        let per_window: Vec<usize> = windows.iter().map(|window| window.attempts).collect();
        assert_eq!(per_window, expected_attempts, "request={request} packet={packet}");
        let native_services = w.events().iter().filter(|e| {
            e.origin == Origin::Internal(1) && e.pc == 4
                && matches!(e.kind, EventKind::CellWritten { key: 9, value: 1 })
        }).count();
        let native_draws = w.events().iter().filter(|e| {
            e.origin == Origin::Internal(1) && e.pc == 1
                && e.pool_paid_attempt
                && !matches!(e.kind, EventKind::Trapped(_) | EventKind::DrawDeferred { .. })
        }).count();
        assert_eq!(native_services, services);
        assert_eq!(native_draws, draws);
        assert_eq!(w.process(1).unwrap().pc, pc);
        assert_eq!(w.available_compute_pool(), pool);
        assert_eq!(w.process(1).unwrap().credits, 0);
        assert!(!w.process(1).unwrap().stopped);
        assert!(w.process(2).is_none());
        assert!(w.occupied_units() <= 128);
        assert_eq!(
            w.events().iter().filter(|e| e.origin == Origin::External).count(),
            1 + usize::from(packet > 0) * 4
        );
        assert!(w.events().iter().all(|e| {
            e.origin == Origin::External || e.origin == Origin::Internal(1)
        }));
        for window in &windows {
            assert_eq!(window.credits, 0);
            assert_eq!(window.writes <= services, true);
            assert_eq!(window.draws <= draws, true);
            assert!(window.pool <= packet * 4);
            assert_eq!(
                window.pc,
                if request == 2 { 4 } else { 1 },
                "native policy has no feedback branch after being stuck"
            );
        }
        let total_inflow = packet * 4;
        let attempts = 2_u64 + windows.iter().map(|w| w.attempts as u64).sum::<u64>();
        assert_eq!(
            w.total_compute_inventory(),
            Some(2 + total_inflow - attempts),
            "externally supplied world energy must be exactly accounted"
        );
    }
}
