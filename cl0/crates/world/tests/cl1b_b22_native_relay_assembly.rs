//! B22 — one externally loaded CIL founder builds and launches a credit-relay
//! graph using charged native graph primitives, then services its own blocked
//! output after its internally constructed helper transfers compute.
//! Full helper template / construction script were authored by investigators.
//! Native success is NOT endogenous invention or cyberlife.
use cl0_world::{EventKind, Intervention, Op, Origin, Process, World};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Arm {
    BuildTransfer,
    BuildNoTransfer,
    BuildNoSecondPacket,
}

fn founder(transfer: bool) -> Vec<Op> {
    vec![
        // Graph creation and 5 source-embedded helper-template copies.
        Op::GraphNew { dst: 0, next: 1 },                         // 0
        Op::Const { dst: 1, value: 30, next: 2 },                  // 1
        Op::GraphCopyNodeLocal { graph: 0, source: 1, dst: 3, next: 3 }, // 2
        Op::Const { dst: 1, value: 31, next: 4 },                  // 3
        Op::GraphCopyNodeLocal { graph: 0, source: 1, dst: 3, next: 5 }, // 4
        Op::Const { dst: 1, value: 32, next: 6 },                  // 5
        Op::GraphCopyNodeLocal { graph: 0, source: 1, dst: 3, next: 7 }, // 6
        Op::Const { dst: 1, value: 33, next: 8 },                  // 7
        Op::GraphCopyNodeLocal { graph: 0, source: 1, dst: 3, next: 9 }, // 8
        Op::Const { dst: 1, value: 34, next: 10 },                 // 9
        Op::GraphCopyNodeLocal { graph: 0, source: 1, dst: 3, next: 11 }, // 10
        // GraphCopyNodeLocal is normalized against growing graph sizes.
        // Correct the first four edges through actual native CIL operations.
        Op::Const { dst: 1, value: 0, next: 12 },                  // 11
        Op::Const { dst: 2, value: 1, next: 13 },                  // 12
        Op::GraphSetNext { graph: 0, node: 1, target: 2, next: 14 }, // 13
        Op::Const { dst: 1, value: 1, next: 15 },                  // 14
        Op::Const { dst: 2, value: 2, next: 16 },                  // 15
        Op::GraphSetNext { graph: 0, node: 1, target: 2, next: 17 }, // 16
        Op::Const { dst: 1, value: 2, next: 18 },                  // 17
        Op::Const { dst: 2, value: 3, next: 19 },                  // 18
        Op::GraphSetNext { graph: 0, node: 1, target: 2, next: 20 }, // 19
        Op::Const { dst: 1, value: 3, next: 21 },                  // 20
        Op::Const { dst: 2, value: 4, next: 22 },                  // 21
        Op::GraphSetNext { graph: 0, node: 1, target: 2, next: 23 }, // 22
        Op::Const { dst: 1, value: 2, next: 24 },                  // 23
        Op::GraphLaunch { graph: 0, credits: 1, dst: 3, next: 25 }, // 24
        // Parent/worker: self-servicing request2 (intentionally too small
        // to finish its service without a later helper transfer).
        Op::Const { dst: 0, value: 2, next: 26 },                  // 25
        Op::DrawCompute { amount: 0, next: 27 },                   // 26
        Op::Const { dst: 2, value: 9, next: 28 },                  // 27
        Op::Const { dst: 3, value: 1, next: 29 },                  // 28
        Op::Write { address: 2, value: 3, next: 26 },              // 29
        // Inert embedded investigator template; only new graph's *native*
        // launch can execute it, with edges relocated to child indexes0..4.
        Op::Const { dst: 0, value: 4, next: 31 },                  // 30
        Op::DrawCompute { amount: 0, next: 32 },                   // 31
        Op::Const { dst: 2, value: 1, next: 33 },                  // 32
        Op::Const { dst: 3, value: 1, next: 34 },                  // 33
        if transfer {
            Op::TransferCompute { recipient: 2, amount: 3, next: 31 } // 34
        } else {
            Op::Const { dst: 1, value: 0, next: 31 }              // 34 ablation
        },
    ]
}

fn attempt_until_blocked(w: &mut World) -> usize {
    let mut count = 0_usize;
    loop {
        assert!(count < 100, "bounded total energy cannot fund unlimited attempts");
        let energy = w.total_compute_inventory().unwrap();
        let events = w.events().len();
        if !w.step() {
            assert_eq!(w.total_compute_inventory(), Some(energy));
            assert_eq!(w.events().len(), events);
            break;
        }
        count += 1;
        assert_eq!(w.total_compute_inventory(), Some(energy - 1));
        assert_eq!(w.events().len(), events + 1);
        assert!(matches!(w.events().last().unwrap().origin, Origin::Internal(1) | Origin::Internal(2)));
    }
    count
}

fn packet(w: &mut World, quantity: u64) {
    let before = w.total_compute_inventory().unwrap();
    let old_pool = w.available_compute_pool();
    w.intervene(Intervention::SetComputePool { credits: old_pool + quantity })
        .unwrap();
    assert_eq!(w.total_compute_inventory(), Some(before + quantity));
    assert_eq!(w.events().last().unwrap().origin, Origin::External);
}

#[test]
fn b22_single_native_founder_builds_and_launches_a_working_energy_relay() {
    for arm in [Arm::BuildTransfer, Arm::BuildNoTransfer, Arm::BuildNoSecondPacket] {
        let transfer = !matches!(arm, Arm::BuildNoTransfer);
        let mut w = World::with_strict_capacity(16, 64, 128)
            .with_pool_paid_draw()
            .with_recoverable_draw_shortage();
        assert_eq!(founder(transfer).len(), 35);
        w.load(Process::new(1, founder(transfer), 29)).unwrap();
        assert!(w.process(2).is_none(), "NO external helper process is loaded");
        assert_eq!(w.events().iter().filter(|e| e.origin == Origin::External).count(), 1);

        let initial = attempt_until_blocked(&mut w);
        assert_eq!(initial, 29, "{arm:?}: assembly and both child startup instructions pay from initial endowment");
        assert_eq!(w.total_compute_inventory(), Some(0));
        assert_eq!(w.available_compute_pool(), 0);
        assert_eq!(w.graph_count(), 0, "owned graph was moved into child process, not cloned at no cost");
        assert_eq!((w.process(1).unwrap().pc, w.process(1).unwrap().credits), (26, 0));
        assert_eq!((w.process(2).unwrap().pc, w.process(2).unwrap().credits), (1, 0));
        assert_eq!(w.process(1).unwrap().registers[3], 2,
            "at startup cutoff the parent has only executed PC25 Const and a deferred PC26 Draw; r3 still holds the native GraphLaunch child ID");

        let created = w.events().iter().filter(|e| matches!(
            (&e.origin, &e.kind),
            (Origin::Internal(1), EventKind::GraphCreated { owner: 1, .. })
        )).count();
        let copied = w.events().iter().filter(|e| matches!(
            (&e.origin, &e.kind),
            (Origin::Internal(1), EventKind::GraphNodeCopiedLocal { owner: 1, .. })
        )).count();
        let rewired = w.events().iter().filter(|e| matches!(
            (&e.origin, &e.kind),
            (Origin::Internal(1), EventKind::GraphNodeEdited { owner: 1, edit, .. })
                if edit == "next"
        )).count();
        let launched = w.events().iter().filter(|e| matches!(
            (&e.origin, &e.kind),
            (Origin::Internal(1), EventKind::GraphLaunched {
                parent_process: 1, child_process: 2,
                transferred_credits: 2, moved_nodes: 5, ..
            })
        )).count();
        assert_eq!((created, copied, rewired, launched), (1, 5, 4, 1));
        assert_eq!(w.process(2).unwrap().code.nodes.len(), 5);
        assert!(matches!(w.process(2).unwrap().code.nodes[4],
            Op::TransferCompute { next: 1, .. } if transfer) ||
            matches!(w.process(2).unwrap().code.nodes[4],
            Op::Const { next: 1, .. } if !transfer));

        packet(&mut w, 3);
        assert_eq!(attempt_until_blocked(&mut w), 3);
        assert_eq!((w.process(1).unwrap().pc, w.process(1).unwrap().credits), (29, 0));
        assert_eq!((w.process(2).unwrap().pc, w.process(2).unwrap().credits), (1, 0));
        assert_eq!(w.available_compute_pool(), 0);
        assert!(!w.events().iter().any(|e| matches!(
            (&e.origin, &e.kind), (Origin::Internal(1), EventKind::CellWritten { key: 9, value: 1 })
                if e.pc == 29
        )));

        if !matches!(arm, Arm::BuildNoSecondPacket) {
            packet(&mut w, 5);
            let mut remaining = 0_usize;
            let mut observed_transfer = 0_usize;
            loop {
                assert!(remaining < 20);
                let total = w.total_compute_inventory().unwrap();
                let worker_before = w.process(1).unwrap().credits;
                let helper_pc = w.process(2).unwrap().pc;
                let event_count = w.events().len();
                if !w.step() {
                    assert_eq!(w.total_compute_inventory(), Some(total));
                    assert_eq!(w.events().len(), event_count);
                    break;
                }
                remaining += 1;
                assert_eq!(w.total_compute_inventory(), Some(total - 1));
                let last = w.events().last().unwrap();
                if helper_pc == 4 && last.origin == Origin::Internal(2) {
                    if transfer {
                        observed_transfer += 1;
                        assert_eq!(w.process(1).unwrap().credits, worker_before + 1);
                    } else {
                        assert_eq!(w.process(1).unwrap().credits, worker_before);
                    }
                }
            }
            assert_eq!(remaining, 5);
            assert_eq!(observed_transfer, usize::from(transfer));
        }
        let correct_service = w.events().iter().filter(|e| matches!(
            (&e.origin, &e.kind),
            (Origin::Internal(1), EventKind::CellWritten { key: 9, value: 1 }) if e.pc == 29
        )).count();
        assert_eq!(correct_service, usize::from(matches!(arm, Arm::BuildTransfer)));
        assert_eq!(w.process(1).unwrap().pc, if correct_service > 0 { 26 } else { 29 });
        assert_eq!(w.process(1).unwrap().credits, 0);
        assert_eq!(w.process(2).unwrap().pc, 1);
        assert_eq!(w.process(2).unwrap().credits, 0);
        assert_eq!(w.available_compute_pool(), 0);
        assert_eq!(w.total_compute_inventory(), Some(0));
        assert_eq!(w.events().iter().filter(|e| e.origin == Origin::External).count(),
            if matches!(arm, Arm::BuildNoSecondPacket) { 2 } else { 3 });
        assert!(!w.process(1).unwrap().stopped);
        assert!(!w.process(2).unwrap().stopped);
        assert!(w.occupied_units() <= 128);
        assert!(w.events().iter().all(|e| !matches!(
            e.kind, EventKind::Trapped(_)
        )));
    }
}
