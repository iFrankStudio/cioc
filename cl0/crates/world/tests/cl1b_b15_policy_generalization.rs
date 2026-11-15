//! CL-1B B15: observer-labelled, exhaustive native Rust differential
//! between two investigator-provided source variants differing by ONE literal.
//! This does NOT demonstrate self-originated policy discovery or living evolution.
//! Frozen protocol: cl1b/studies/B15-exhaustive-policy-generalization.md.
#[path = "cl1b_b14_search_policy_mutation.rs"]
mod canonical_b14;

use cl0_world::{EventKind, Intervention, Op, Origin, Process, World};

#[derive(Default, Debug, Clone)]
struct Outcomes {
    pairs: usize,
    wins: usize,
    losses: usize,
    ties: usize,
    service_delta: i64,
    adaptive_traps: usize,
    locked_traps: usize,
    adaptive_stopped: usize,
    locked_stopped: usize,
    unequal_attempts: usize,
}

#[derive(Debug)]
struct Trial {
    service: usize,
    attempts: usize,
    trapped: bool,
    stopped: bool,
    policy_exec_count: usize,
    occupied: usize,
}

fn trial(input: i64, original_edge: usize, adaptive: bool) -> Trial {
    let mut code = canonical_b14::founder();
    assert_eq!(code.len(), 35, "do not change B14 founder");
    if !adaptive {
        match &mut code[21] {
            Op::Const { value, .. } => {
                assert_eq!(*value, 1);
                *value = 2;
            }
            _ => panic!("no-switch baseline differs beyond its one Const value"),
        }
    }
    let mut w = World::with_strict_capacity(16, 64, 128)
        .with_native_executable_edit_trace();
    w.load(Process::new(1, code, 240)).unwrap();
    w.intervene(Intervention::SetCell { key: 7, value: input })
        .unwrap();
    if original_edge != 7 {
        w.intervene(Intervention::PatchEdge {
            process: 1,
            node: 6,
            next: original_edge,
        }).unwrap();
    }
    let attempts = w.run(120);
    assert!(attempts <= 120);
    let p = w.process(1).unwrap();
    assert_eq!(p.credits, 240 - attempts as u64);
    assert_eq!(w.total_compute_inventory(), Some(240 - attempts as u64));
    assert!(w.process(2).is_none());
    assert!(w.occupied_units() <= 128);
    assert!(w.events().iter().all(|event| {
        event.origin == Origin::Internal(1) || event.origin == Origin::External
    }));
    assert_eq!(
        w.events().iter().filter(|e| e.origin == Origin::External).count(),
        if original_edge == 7 { 2 } else { 3 },
        "no after-start researcher intervention"
    );
    let service = w.events().iter().filter(|e| matches!(
        (&e.origin, &e.kind),
        (Origin::Internal(1), EventKind::CellWritten { key: 11, value })
            if *value == input
    )).count();
    let trapped = w.events().iter().any(|e| matches!(
        (&e.origin, &e.kind),
        (Origin::Internal(1), EventKind::Trapped(_))
    ));
    let policy_exec_count = w.events().iter().filter(|e| {
        e.origin == Origin::Internal(1)
            && e.pc == 22
            && !matches!(e.kind, EventKind::Trapped(_))
    }).count();
    Trial {
        service,
        attempts,
        trapped,
        stopped: p.stopped,
        policy_exec_count,
        occupied: w.occupied_units(),
    }
}

fn add(out: &mut Outcomes, adaptive: &Trial, locked: &Trial) {
    out.pairs += 1;
    let delta = adaptive.service as i64 - locked.service as i64;
    out.service_delta += delta;
    if delta > 0 {
        out.wins += 1;
    } else if delta < 0 {
        out.losses += 1;
    } else {
        out.ties += 1;
    }
    out.adaptive_traps += usize::from(adaptive.trapped);
    out.locked_traps += usize::from(locked.trapped);
    out.adaptive_stopped += usize::from(adaptive.stopped);
    out.locked_stopped += usize::from(locked.stopped);
    out.unequal_attempts += usize::from(adaptive.attempts != locked.attempts);
}

#[test]
fn b15_all_350_native_worlds_falsify_universal_policy_switch_benefit() {
    let mut healthy = Outcomes::default();
    let mut development = Outcomes::default();
    let mut holdout = Outcomes::default();

    for input in [-13, 0, 7, 23, 101] {
        for next in 0..35_usize {
            let adaptive = trial(input, next, true);
            let locked = trial(input, next, false);

            let bucket = if next == 7 {
                &mut healthy
            } else if next == 10 || next == 11 {
                &mut development
            } else {
                &mut holdout
            };
            add(bucket, &adaptive, &locked);

            if next == 7 {
                assert_eq!(adaptive.service, locked.service);
                assert_eq!(adaptive.policy_exec_count, 0);
                assert_eq!(locked.policy_exec_count, 0);
            }
            if next == 10 {
                assert_eq!((adaptive.service, locked.service), (5, 1));
            }
            if next == 11 {
                assert_eq!((adaptive.service, locked.service), (3, 4));
            }
            if next == 13 {
                assert_eq!((adaptive.service, locked.service), (1, 3));
            }
            if next == 15 {
                assert_eq!((adaptive.service, locked.service), (0, 1));
            }
            assert!(adaptive.occupied <= 128);
            assert!(locked.occupied <= 128);
        }
    }

    assert_eq!((healthy.pairs, healthy.wins, healthy.losses, healthy.ties),
               (5, 0, 0, 5), "{healthy:?}");
    assert_eq!((development.pairs, development.wins, development.losses, development.ties),
               (10, 5, 5, 0), "{development:?}");
    assert_eq!(development.service_delta, 15);
    assert_eq!((holdout.pairs, holdout.wins, holdout.losses, holdout.ties),
               (160, 0, 10, 150), "{holdout:?}");
    assert_eq!(holdout.service_delta, -15);
    assert_eq!((holdout.adaptive_traps, holdout.locked_traps), (8, 8));
    assert_eq!((holdout.adaptive_stopped, holdout.locked_stopped), (61, 45));
    assert_eq!(holdout.unequal_attempts, 16);
}
