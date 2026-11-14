//! Study103: fully precommitted HUMAN-authored native functional mutation inheritance.
//! This is a CIL substrate experiment, not spontaneous evolution or life.
use cl0_world::{Event, EventKind, Intervention, Op, Origin, Process, World};
use std::collections::{BTreeMap, BTreeSet};

const NODES: usize = 28;
const GRANT: u64 = 400;
const FAST: usize = 5;
const SLOW: usize = 24;

fn code(native_mutation: bool) -> Vec<Op> {
    let mut nodes = vec![
        Op::Const { dst: 2, value: 7, next: 1 },
        Op::Const { dst: 3, value: 9, next: 2 },
        Op::Read { dst: 0, address: 2, next: 3 },
        Op::Write { address: 3, value: 0, next: 4 },
        Op::Const { dst: 2, value: 1, next: 5 }, // inherited functional route
        Op::Const { dst: 2, value: 11, next: 6 },
        Op::Read { dst: 0, address: 2, next: 7 },
        Op::Const { dst: 1, value: 4, next: 8 },
        Op::PatchNext { node: 1, target: 0, next: 9 },
        Op::Const { dst: 1, value: 29, next: 10 },
        Op::Sub { dst: 1, lhs: 1, rhs: 0, next: 11 },
        Op::Write { address: 2, value: 1, next: 12 },
        Op::Const { dst: 2, value: 1, next: 13 },
        Op::Const { dst: 1, value: 0, next: 14 },
        Op::Const { dst: 3, value: 28, next: 15 },
        Op::GraphNew { dst: 0, next: 16 },
        Op::GraphCopyNode { graph: 0, source: 1, dst: 1, next: 17 },
        Op::Add { dst: 1, lhs: 1, rhs: 2, next: 18 },
        Op::Sub { dst: 3, lhs: 3, rhs: 2, next: 19 },
        Op::Branch { test: 3, nonzero: 16, zero: 20 },
        Op::Const { dst: 2, value: 400, next: 21 },
        Op::DrawCompute { amount: 2, next: 22 },
        Op::GraphLaunch { graph: 0, credits: 2, dst: 3, next: 23 },
        Op::Const { dst: 1, value: 0, next: 4 }, // parent remains able to branch
        Op::Const { dst: 1, value: 20, next: 25 },
        Op::Const { dst: 2, value: 1, next: 26 },
        Op::Sub { dst: 1, lhs: 1, rhs: 2, next: 27 },
        Op::Branch { test: 1, nonzero: 26, zero: 5 },
    ];
    if !native_mutation {
        // HUMAN-coded source-level comparator, charged same as PatchNext.
        // This is NOT the genetic-mutation ablation physics.
        nodes[8] = Op::Const { dst: 1, value: 4, next: 9 };
    }
    assert_eq!(nodes.len(), NODES);
    nodes
}

fn world(source: i64, native_mutation: bool, pool: u64) -> World {
    let mut world = World::with_strict_capacity(4, 64, 1024)
        .with_pool_paid_draw()
        .with_recoverable_draw_shortage()
        .with_native_executable_edit_trace();
    for id in [1, 2] {
        world.load(Process::new(id, code(native_mutation), 500)).unwrap();
    }
    world.intervene(Intervention::SetCell { key: 7, value: source }).unwrap();
    world.intervene(Intervention::SetCell { key: 11, value: FAST as i64 }).unwrap();
    world.intervene(Intervention::SetComputePool { credits: pool }).unwrap();
    world
}

fn inherited_edge(w: &World, actor: u64) -> usize {
    match &w.process(actor).expect("known real CIL process").code.nodes[4] {
        Op::Const { next, .. } => *next,
        other => panic!("functional code node4 replaced unexpectedly: {other:?}"),
    }
}

#[derive(Clone, Copy, Debug)]
struct Birth {
    parent: u64,
    child: u64,
    birth_edge: usize,
}

fn execute_and_record_births(w: &mut World, limit: usize) -> (usize, Vec<Birth>) {
    let mut attempted = 0;
    let mut births = Vec::new();
    while attempted < limit && w.step() {
        attempted += 1;
        let event = w.events().last().expect("charged native attempt produced event");
        if let EventKind::GraphLaunched {
            parent_process,
            child_process,
            transferred_credits,
            moved_nodes,
            ..
        } = event.kind
        {
            assert_eq!(event.origin, Origin::Internal(parent_process));
            assert_eq!(event.pc, 22);
            assert_eq!(transferred_credits, GRANT);
            assert_eq!(moved_nodes, NODES);
            let edge = inherited_edge(w, child_process);
            assert_eq!(edge, inherited_edge(w, parent_process));
            births.push(Birth { parent: parent_process, child: child_process, birth_edge: edge });
        }
    }
    (attempted, births)
}

fn verify_source_and_provenance(w: &World, source: i64, births: &[Birth]) -> (usize, usize) {
    assert_eq!(w.events().iter().filter(|e| e.origin == Origin::External).count(), 5);
    let mut read_by_actor = BTreeMap::<u64, i64>::new();
    let mut writers = BTreeSet::new();
    let mut draws = BTreeMap::<u64, usize>::new();
    let mut launched = BTreeMap::<u64, usize>::new();
    let mut copied = BTreeMap::<u64, BTreeSet<usize>>::new();
    let mut mutated = 0;
    let mut suppressed = 0;
    for e in w.events() {
        if e.origin != Origin::Internal(e.process) {
            continue;
        }
        match &e.kind {
            EventKind::CellRead { key: 7, value } => {
                assert_eq!(e.pc, 2);
                assert_eq!(*value, source);
                read_by_actor.insert(e.process, *value);
            }
            EventKind::CellWritten { key: 9, value } => {
                assert_eq!(e.pc, 3);
                assert_eq!(*value, source);
                assert_eq!(read_by_actor.get(&e.process), Some(&source));
                writers.insert(e.process);
            }
            EventKind::CellRead { key: 11, value } => {
                assert_eq!(e.pc, 6);
                assert!([FAST as i64, SLOW as i64].contains(value));
            }
            EventKind::CellWritten { key: 11, value } => {
                assert_eq!(e.pc, 11);
                assert!([FAST as i64, SLOW as i64].contains(value));
            }
            EventKind::ExecutableEdgePatched { node, before, after } => {
                assert_eq!(e.pc, 8);
                assert_eq!(*node, 4);
                assert!([FAST, SLOW].contains(before));
                assert!([FAST, SLOW].contains(after));
                assert_ne!(before, after);
                mutated += 1;
            }
            EventKind::NativeCodeEditSuppressed { .. } => suppressed += 1,
            EventKind::Executed if e.pc == 21 => {
                *draws.entry(e.process).or_default() += 1;
            }
            EventKind::GraphNodeCopied { source_process, source_node, .. } => {
                assert_eq!(*source_process, e.process);
                copied.entry(e.process).or_default().insert(*source_node);
            }
            EventKind::GraphLaunched { parent_process, child_process, .. } => {
                assert_eq!(e.process, *parent_process);
                let previous = *launched.get(parent_process).unwrap_or(&0);
                let successful_draws = *draws.get(parent_process).unwrap_or(&0);
                assert!(successful_draws > previous, "each launch requires actual native Draw");
                assert_eq!(copied.get(parent_process).map(BTreeSet::len), Some(NODES));
                *launched.entry(*parent_process).or_default() += 1;
                assert!(births.iter().any(|b| b.parent == *parent_process && b.child == *child_process));
            }
            _ => {}
        }
    }
    assert_eq!(suppressed, 0);
    assert!(writers.contains(&1) && writers.contains(&2));
    assert_eq!(w.cell(9), Some(source));
    (mutated, writers.len())
}

fn first_pc(events: &[Event], actor: u64, pc: usize) -> Option<u64> {
    events.iter().find(|e| e.origin == Origin::Internal(actor) && e.pc == pc).map(|e| e.tick)
}

#[test]
fn functional_native_mutation_is_inherited_and_actually_changes_descendant_execution_cost() {
    for source in [3, 8] {
        let mut active = world(source, true, 6000);
        let initial = active.total_compute_inventory().unwrap();
        assert_eq!(initial, 7000);
        let (attempts, births) = execute_and_record_births(&mut active, 3000);
        assert_eq!(active.total_compute_inventory(), Some(initial - attempts as u64));
        let (mutations, writers) = verify_source_and_provenance(&active, source, &births);
        assert!(mutations > 0, "real CIL must mutate a functional executable edge");
        assert!(births.iter().any(|b| b.parent == 1) && births.iter().any(|b| b.parent == 2));
        let founder1_children = births.iter().filter(|b| b.parent == 1).count();
        let founder2_children = births.iter().filter(|b| b.parent == 2).count();
        assert!(founder1_children >= 2 || founder2_children >= 2, "one founder must make native siblings");
        assert!(births.iter().any(|b| b.parent >= 3), "a genuine descendant must reproduce");

        let fast_births = births.iter().filter(|b| b.birth_edge == FAST).count();
        let slow_births = births.iter().filter(|b| b.birth_edge == SLOW).count();
        assert!(fast_births > 0 && slow_births > 0, "native mutated functional fast and slow alleles must both be born");
        let mut expressed_fast = 0;
        let mut expressed_slow = 0;
        for birth in &births {
            let first_branch = first_pc(active.events(), birth.child, 4);
            let first_join = first_pc(active.events(), birth.child, 5);
            if let (Some(branch), Some(join)) = (first_branch, first_join) {
                assert!(join > branch);
                let slow_pc = first_pc(active.events(), birth.child, SLOW);
                if birth.birth_edge == SLOW {
                    assert!(slow_pc.is_some_and(|tick| tick > branch && tick < join),
                        "slow CIL allele did not execute actual charged delay path");
                    expressed_slow += 1;
                } else {
                    assert!(slow_pc.is_none_or(|tick| tick > join),
                        "fast CIL allele unexpectedly entered the slow delay");
                    expressed_fast += 1;
                }
            }
        }
        assert!(expressed_fast > 0 && expressed_slow > 0,
            "both inherited functional variants must be executed in descendants");
        println!("Study103 native code variation, source={source}: attempts={attempts} births={} fast_births={fast_births} slow_births={slow_births} expressed_fast={expressed_fast} expressed_slow={expressed_slow} native_mutations={mutations} native_function_writers={writers} [HUMAN-SCRIPTED MUTAGEN; NO EVOLUTION CLAIM]", births.len());

        let mut disabled = world(source, false, 6000);
        let disabled_initial = disabled.total_compute_inventory().unwrap();
        let (disabled_attempts, disabled_births) = execute_and_record_births(&mut disabled, 3000);
        assert_eq!(disabled.total_compute_inventory(), Some(disabled_initial - disabled_attempts as u64));
        let (disabled_mutations, _) = verify_source_and_provenance(&disabled, source, &disabled_births);
        assert_eq!(disabled_mutations, 0);
        assert!(!disabled_births.is_empty());
        assert!(disabled_births.iter().all(|b| b.birth_edge == FAST));
        assert!(disabled.process(1).unwrap().code.nodes.iter().zip(code(false).iter()).all(|(a,b)| a==b));

        let mut cold = world(source, true, 0);
        let cold_initial = cold.total_compute_inventory().unwrap();
        let (cold_attempts, cold_births) = execute_and_record_births(&mut cold, 1500);
        assert_eq!(cold.total_compute_inventory(), Some(cold_initial - cold_attempts as u64));
        assert!(cold_births.is_empty());
        assert!(cold.events().iter().any(|e| {
            e.origin == Origin::Internal(e.process)
                && matches!(e.kind, EventKind::DrawDeferred { requested: GRANT, .. })
        }));
    }
}
