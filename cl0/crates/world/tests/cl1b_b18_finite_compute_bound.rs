//! CL-1B B18: finite compute-inventory obstruction to indefinite native operation.
//! This is a physics-necessity result, NOT proof of organism self-maintenance.
//! Protocol: cl1b/studies/B18-finite-compute-no-perpetual-maintenance.md.
use cl0_world::{EventKind, Intervention, Op, Origin, Process, World};

fn measure_one_charged_attempt(w: &mut World) -> bool {
    let before = w.total_compute_inventory().expect("finite inventory");
    let previous_events = w.events().len();
    let executed = w.step();
    if executed {
        let after = w.total_compute_inventory().expect("finite inventory");
        assert_eq!(before - after, 1, "native attempt must consume exactly one credit");
        assert_eq!(w.events().len(), previous_events + 1);
        let event = w.events().last().expect("executed event");
        assert!(
            matches!(event.origin, Origin::Internal(_)),
            "each native attempt must have internal provenance"
        );
        assert_eq!(
            event.pool_paid_attempt as u64
                + u64::from(!event.pool_paid_attempt),
            1,
            "the one instruction charge must originate from either actor or world pool"
        );
    } else {
        assert_eq!(before, w.total_compute_inventory().unwrap());
        assert_eq!(previous_events, w.events().len());
    }
    executed
}

fn exhaust_without_external_energy(mut w: World, starting_inventory: u64) -> (World, usize) {
    assert_eq!(w.total_compute_inventory(), Some(starting_inventory));
    let mut attempts = 0_usize;
    while measure_one_charged_attempt(&mut w) {
        attempts += 1;
        assert!(
            attempts as u64 <= starting_inventory,
            "a closed native world must never produce more attempts than initial inventory"
        );
    }
    assert!(attempts as u64 <= starting_inventory);
    assert_eq!(
        w.total_compute_inventory(),
        Some(starting_inventory - attempts as u64)
    );
    (w, attempts)
}

#[test]
fn b18_finite_single_loop_exhausts_all_compute_despite_loop_shaped_code() {
    let mut w = World::with_strict_capacity(16, 64, 128);
    w.load(Process::new(
        1,
        vec![Op::Const { dst: 0, value: 0, next: 0 }],
        10,
    ))
    .unwrap();

    let (mut finished, attempts) = exhaust_without_external_energy(w, 10);
    assert_eq!(attempts, 10);
    assert_eq!(finished.total_compute_inventory(), Some(0));
    assert_eq!(finished.process(1).unwrap().pc, 0);
    assert_eq!(finished.process(1).unwrap().credits, 0);
    assert!(!finished.process(1).unwrap().stopped, "credit exhaustion != biological aging");
    assert_eq!(finished.run(100), 0);
}

#[test]
fn b18_native_draw_and_return_rearrange_but_never_create_compute() {
    let mut w = World::with_strict_capacity(16, 64, 128).with_pool_paid_draw();
    w.load(Process::new(
        1,
        vec![
            Op::Const { dst: 0, value: 1, next: 1 },
            Op::DrawCompute { amount: 0, next: 2 },
            Op::ReturnCompute { amount: 0, next: 0 },
        ],
        4,
    ))
    .unwrap();
    w.intervene(Intervention::SetComputePool { credits: 5 })
        .unwrap();
    assert_eq!(w.total_compute_inventory(), Some(9));

    let (finished, attempts) = exhaust_without_external_energy(w, 9);
    assert!(attempts > 0 && attempts <= 9);
    assert!(
        finished.process(1).unwrap().stopped
            || finished.total_compute_inventory() == Some(0),
        "a stranded pool balance or total exhaustion must explain non-continuation"
    );
    let external_initial_pool = finished.events().iter().filter(|e| e.origin == Origin::External).count();
    assert_eq!(external_initial_pool, 2, "one process load and one externally supplied pool");
}

#[test]
fn b18_transfer_between_two_native_processes_does_not_mint_compute() {
    let mut w = World::with_strict_capacity(16, 64, 128);
    w.load(Process::new(
        1,
        vec![
            Op::Const { dst: 0, value: 2, next: 1 },
            Op::Const { dst: 1, value: 1, next: 2 },
            Op::TransferCompute { recipient: 0, amount: 1, next: 2 },
        ],
        6,
    ))
    .unwrap();
    w.load(Process::new(
        2,
        vec![Op::Const { dst: 0, value: 0, next: 0 }],
        4,
    ))
    .unwrap();

    let (finished, attempts) = exhaust_without_external_energy(w, 10);
    assert!(attempts > 0 && attempts <= 10);
    assert!(finished.process(1).is_some());
    assert!(finished.process(2).is_some());
    assert_eq!(
        finished.events().iter().filter(|e| e.origin == Origin::External).count(),
        2,
        "only the two original founder loads are host operations"
    );
    assert!(
        finished.events().iter().any(|e| matches!(e.origin, Origin::Internal(2))),
        "recipient actor must have charged activity"
    );
}

#[test]
fn b18_explicit_external_refill_is_not_actor_owned_compute_creation() {
    let mut w = World::with_strict_capacity(16, 64, 128);
    w.load(Process::new(
        1,
        vec![Op::Const { dst: 0, value: 0, next: 0 }],
        5,
    ))
    .unwrap();
    let (mut finished, attempts) = exhaust_without_external_energy(w, 5);
    assert_eq!(attempts, 5);
    assert_eq!(finished.total_compute_inventory(), Some(0));
    let existing_external = finished.events().iter().filter(|e| e.origin == Origin::External).count();
    assert_eq!(existing_external, 1);

    // This breaks the closed-system premise. It is explicitly external,
    // not a new native CIL self-maintenance capability.
    finished
        .intervene(Intervention::SetCredits { process: 1, credits: 5 })
        .unwrap();
    assert_eq!(finished.total_compute_inventory(), Some(5));
    assert!(matches!(
        (&finished.events().last().unwrap().origin, &finished.events().last().unwrap().kind),
        (Origin::External, EventKind::Intervened(_))
    ));
    let (mut finished_again, new_attempts) = exhaust_without_external_energy(finished, 5);
    assert_eq!(new_attempts, 5);
    assert_eq!(finished_again.total_compute_inventory(), Some(0));
    assert_eq!(finished_again.run(10), 0);
    assert_eq!(
        finished_again.events().iter().filter(|e| e.origin == Origin::External).count(),
        2,
        "repeated attempts are possible only because host injected fresh compute"
    );
}
