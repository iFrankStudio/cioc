//! Study101: registered 10 HUMAN-scripted founder standing-variation controls.
//! No stochastic evolution: first F/S genotypes are researcher authored.
use cl0_world::{EventKind, Intervention, Op, Origin, Process, World};
use std::collections::{BTreeMap, BTreeSet};

const NODES: usize = 20;
const LIMIT: usize = 2200;
const GRANT: u64 = 160;

fn source_code(fast: bool) -> Vec<Op> {
    let nodes = vec![
        Op::Const { dst: 2, value: 7, next: 1 },
        Op::Const { dst: 3, value: 9, next: 2 },
        Op::Read { dst: 0, address: 2, next: 3 },
        Op::Write { address: 3, value: 0, next: 4 },
        Op::Const { dst: 2, value: 1, next: if fast { 5 } else { 16 } },
        Op::Const { dst: 1, value: 0, next: 6 },
        Op::Const { dst: 3, value: 20, next: 7 },
        Op::GraphNew { dst: 0, next: 8 },
        Op::GraphCopyNode { graph: 0, source: 1, dst: 1, next: 9 },
        Op::Add { dst: 1, lhs: 1, rhs: 2, next: 10 },
        Op::Sub { dst: 3, lhs: 3, rhs: 2, next: 11 },
        Op::Branch { test: 3, nonzero: 8, zero: 12 },
        Op::Const { dst: 2, value: 160, next: 13 },
        Op::DrawCompute { amount: 2, next: 14 },
        Op::GraphLaunch { graph: 0, credits: 2, dst: 3, next: 15 },
        Op::Halt,
        Op::Const { dst: 1, value: 20, next: 17 },
        Op::Const { dst: 2, value: 1, next: 18 },
        Op::Sub { dst: 1, lhs: 1, rhs: 2, next: 19 },
        Op::Branch { test: 1, nonzero: 18, zero: 5 },
    ];
    assert_eq!(nodes.len(), NODES);
    nodes
}

fn native_world(source: i64, first_fast: bool, second_fast: bool, pool: u64) -> World {
    assert!(source == 3 || source == 8);
    let mut w = World::with_strict_capacity(4, 64, 1024)
        .with_pool_paid_draw()
        .with_recoverable_draw_shortage();
    w.load(Process::new(1, source_code(first_fast), GRANT)).unwrap();
    w.load(Process::new(2, source_code(second_fast), GRANT)).unwrap();
    w.intervene(Intervention::SetCell { key: 7, value: source }).unwrap();
    w.intervene(Intervention::SetComputePool { credits: pool }).unwrap();
    w
}

#[derive(Debug)]
struct Summary {
    attempted: usize,
    descendants_first: usize,
    descendants_second: usize,
    draws: usize,
    deferred: usize,
    leftover_pool: u64,
    native_writers: usize,
}

fn audit(w: &World, source: i64, first_fast: bool, second_fast: bool, pool: u64, attempted: usize) -> Summary {
    assert!(attempted <= LIMIT);
    assert_eq!(w.total_compute_inventory(), Some(GRANT * 2 + pool - attempted as u64));
    assert_eq!(w.events().iter().filter(|e| e.origin == Origin::External).count(), 4);
    assert_eq!(w.cell(9), Some(source));

    let mut founder_fast = BTreeMap::<u64, bool>::from([(1, first_fast), (2, second_fast)]);
    let mut origin = BTreeMap::<u64, u64>::from([(1, 1), (2, 2)]);
    let mut copied = BTreeMap::<u64, BTreeSet<usize>>::new();
    let mut native_reads = BTreeSet::<u64>::new();
    let mut native_writers = BTreeSet::<u64>::new();
    let mut descendants = BTreeMap::<u64, usize>::new();
    let mut draws = 0;
    let mut deferred = 0;

    for e in w.events() {
        if e.origin != Origin::Internal(e.process) {
            continue;
        }
        match &e.kind {
            EventKind::GraphLaunched {
                parent_process,
                child_process,
                transferred_credits,
                moved_nodes,
                ..
            } => {
                assert_eq!(*parent_process, e.process);
                assert_eq!(e.pc, 14);
                assert_eq!(*transferred_credits, GRANT);
                assert_eq!(*moved_nodes, NODES);
                let f = *founder_fast.get(parent_process).expect("unknown native parent");
                let founder = *origin.get(parent_process).expect("parent's original founder");
                assert!(founder_fast.insert(*child_process, f).is_none());
                assert!(origin.insert(*child_process, founder).is_none());
                *descendants.entry(founder).or_default() += 1;
                assert_eq!(
                    w.process(*child_process).unwrap().code.nodes,
                    source_code(f),
                    "native child must inherit its founding variant exactly"
                );
            }
            EventKind::GraphNodeCopied { source_process, source_node, .. } => {
                assert_eq!(*source_process, e.process);
                copied.entry(e.process).or_default().insert(*source_node);
            }
            EventKind::CellRead { key: 7, value } => {
                assert_eq!(e.pc, 2);
                assert_eq!(*value, source);
                native_reads.insert(e.process);
            }
            EventKind::CellWritten { key: 9, value } => {
                assert_eq!(e.pc, 3);
                assert_eq!(*value, source);
                assert!(native_reads.contains(&e.process), "child wrote source before native read");
                native_writers.insert(e.process);
            }
            EventKind::Executed if e.pc == 13 => draws += 1,
            EventKind::DrawDeferred { requested: GRANT, .. } if e.pc == 13 => deferred += 1,
            _ => {}
        }
    }
    let child_total: usize = descendants.values().sum();
    assert_eq!(draws, child_total, "real Draw must fund each native child");
    for (parent, copied_nodes) in copied {
        if origin.contains_key(&parent) && copied_nodes.len() == NODES {
            // All copy-complete parents must have the fully authored native code.
            assert_eq!(w.process(parent).unwrap().code.nodes, source_code(founder_fast[&parent]));
        }
    }
    for (&pid, &f) in &founder_fast {
        assert_eq!(w.process(pid).unwrap().code.nodes, source_code(f));
    }
    if pool == 0 {
        assert_eq!(child_total, 0);
        assert_eq!(draws, 0);
        assert!(deferred > 0);
    } else {
        assert!(child_total >= 2, "both founders must have a chance to launch in open pool");
        assert!(*descendants.get(&1).unwrap_or(&0) >= 1);
        assert!(*descendants.get(&2).unwrap_or(&0) >= 1);
    }
    Summary {
        attempted,
        descendants_first: *descendants.get(&1).unwrap_or(&0),
        descendants_second: *descendants.get(&2).unwrap_or(&0),
        draws,
        deferred,
        leftover_pool: w.available_compute_pool(),
        native_writers: native_writers.len(),
    }
}

#[test]
fn finite_public_resource_competition_preserves_native_genomes_and_lineage() {
    let conditions = [
        ("mixed-F1-S2", true, false, 1600),
        ("mixed-S1-F2", false, true, 1600),
        ("same-F1-F2", true, true, 1600),
        ("same-S1-S2", false, false, 1600),
        ("mixed-F1-S2-closed-pool", true, false, 0),
    ];
    // Original variants differ at ONE CIL source edge, not in opcode repertoire.
    let fast = source_code(true);
    let slow = source_code(false);
    assert_eq!(fast.len(), slow.len());
    assert_eq!(
        fast.iter().zip(&slow).filter(|(a, b)| a != b).count(),
        1
    );
    for source in [3, 8] {
        for (label, first_fast, second_fast, pool) in conditions {
            let mut w = native_world(source, first_fast, second_fast, pool);
            let attempts = w.run(LIMIT);
            let outcome = audit(&w, source, first_fast, second_fast, pool, attempts);
            println!(
                "Study101 source={source}/{label}: attempts={} founder1_children={} founder2_children={} native_draws={} deferred={} native_writers={} pool_left={} [HUMAN-SCRIPTED, no evolutionary claim]",
                outcome.attempted,
                outcome.descendants_first,
                outcome.descendants_second,
                outcome.draws,
                outcome.deferred,
                outcome.native_writers,
                outcome.leftover_pool,
            );
        }
    }
}
