//! CL-1 Study100: HUMAN-authored native executable mutation inheritance fixture.
//! These worlds are not independently sampled ecology and do NOT prove adaptation.
use cl0_world::{EventKind, Intervention, Op, Origin, Process, World};
use std::collections::{BTreeMap, BTreeSet};

const NODES: usize = 24;
const CHILD_GRANT: u64 = 160;

fn seed(with_mutation: bool) -> Vec<Op> {
    vec![
        Op::Const { dst: 2, value: 7, next: 1 },
        Op::Const { dst: 3, value: 9, next: 2 },
        Op::Read { dst: 0, address: 2, next: 3 },
        Op::Write { address: 3, value: 0, next: 4 },
        Op::Const { dst: 2, value: 11, next: 5 },
        Op::Read { dst: 0, address: 2, next: 6 },
        Op::Const { dst: 1, value: 1, next: 7 },
        Op::Add { dst: 0, lhs: 0, rhs: 1, next: 8 },
        Op::Write { address: 2, value: 0, next: 9 },
        Op::Const { dst: 1, value: 23, next: 10 },
        if with_mutation {
            Op::PatchNext { node: 1, target: 0, next: 11 }
        } else {
            Op::Const { dst: 1, value: 23, next: 11 }
        },
        Op::Const { dst: 2, value: 1, next: 12 },
        Op::Const { dst: 1, value: 0, next: 13 },
        Op::Const { dst: 3, value: 24, next: 14 },
        Op::GraphNew { dst: 0, next: 15 },
        Op::GraphCopyNode { graph: 0, source: 1, dst: 1, next: 16 },
        Op::Add { dst: 1, lhs: 1, rhs: 2, next: 17 },
        Op::Sub { dst: 3, lhs: 3, rhs: 2, next: 18 },
        Op::Branch { test: 3, nonzero: 15, zero: 19 },
        Op::Const { dst: 2, value: 160, next: 20 },
        Op::DrawCompute { amount: 2, next: 21 },
        Op::GraphLaunch { graph: 0, credits: 2, dst: 3, next: 22 },
        Op::Halt,
        Op::Const { dst: 0, value: 0, next: 23 }, // dormant, neutral edit target
    ]
}

fn fixture(source: i64, mutation: bool, pool: u64) -> World {
    let mut world = World::with_strict_capacity(4, 64, 1024)
        .with_pool_paid_draw()
        .with_recoverable_draw_shortage();
    let program = seed(mutation);
    assert_eq!(program.len(), NODES);
    world.load(Process::new(1, program, 192)).unwrap();
    world.intervene(Intervention::SetCell { key: 7, value: source }).unwrap();
    world.intervene(Intervention::SetCell { key: 11, value: 0 }).unwrap();
    world.intervene(Intervention::SetComputePool { credits: pool }).unwrap();
    world
}

fn links(w: &World) -> Vec<(u64, u64)> {
    w.events()
        .iter()
        .filter_map(|e| {
            if let EventKind::GraphLaunched {
                parent_process,
                child_process,
                transferred_credits,
                moved_nodes,
                ..
            } = e.kind
            {
                assert_eq!(e.origin, Origin::Internal(parent_process));
                assert_eq!(transferred_credits, CHILD_GRANT);
                assert_eq!(moved_nodes, NODES);
                Some((parent_process, child_process))
            } else {
                None
            }
        })
        .collect()
}

fn mutations(w: &World) -> Vec<(u64, usize, usize)> {
    w.events()
        .iter()
        .filter_map(|e| {
            if let EventKind::ExecutableEdgePatched { node, before, after } = e.kind {
                assert_eq!(e.origin, Origin::Internal(e.process));
                assert_eq!(e.pc, 10, "only authored native PatchNext can modify code");
                assert_eq!(node, 23, "only dormant edge may be modified");
                Some((e.process, before, after))
            } else {
                None
            }
        })
        .collect()
}

#[test]
fn native_self_copy_inherits_real_neutral_executable_mutation_across_generations() {
    for source in [3, 8] {
        let base = seed(true);
        let mut w = fixture(source, true, 2000);
        let initial = w.total_compute_inventory().unwrap();
        assert_eq!(initial, 2192);
        let charged = w.run(1800);
        assert!(charged > 200, "multiple generations must be fully exercised");
        assert_eq!(w.total_compute_inventory(), Some(initial - charged as u64));
        let genealogy = links(&w);
        assert!(genealogy.len() >= 3, "native CIL grandchildren must reproduce");
        for (i, &(parent, child)) in genealogy.iter().take(3).enumerate() {
            assert_eq!((parent, child), ((i + 1) as u64, (i + 2) as u64));
        }

        let patch_events = mutations(&w);
        assert!(patch_events.len() >= 4, "four generations must edit their own real code");
        for (i, &(actor, before, after)) in patch_events.iter().take(4).enumerate() {
            assert_eq!(actor, (i + 1) as u64);
            assert_eq!(before, if i == 0 { 23 } else { i });
            assert_eq!(after, i + 1);
            assert!(after < NODES, "mutated dormant edge remains valid CIL");
        }

        let mut read_by_process = BTreeMap::<u64, i64>::new();
        let mut writers = BTreeSet::new();
        let mut draws = BTreeSet::new();
        let mut copies = BTreeMap::<u64, BTreeSet<usize>>::new();
        for e in w.events() {
            if e.origin != Origin::Internal(e.process) {
                continue;
            }
            match &e.kind {
                EventKind::CellRead { key: 7, value } => {
                    assert_eq!(e.pc, 2);
                    assert_eq!(*value, source);
                    read_by_process.insert(e.process, *value);
                }
                EventKind::CellWritten { key: 9, value } => {
                    assert_eq!(e.pc, 3);
                    assert_eq!(*value, source);
                    assert_eq!(read_by_process.get(&e.process), Some(&source));
                    writers.insert(e.process);
                }
                EventKind::GraphNodeCopied { source_process, source_node, .. } => {
                    assert_eq!(*source_process, e.process);
                    copies.entry(e.process).or_default().insert(*source_node);
                }
                EventKind::Executed if e.pc == 20 => {
                    draws.insert(e.process);
                }
                _ => {}
            }
        }
        for actor in 1..=4 {
            assert!(writers.contains(&actor), "source function not inherited and executed");
            assert!(draws.contains(&actor), "native generation failed actual pool Draw");
            assert_eq!(copies.get(&actor).map(BTreeSet::len), Some(NODES));
            let child = w.process(actor).expect("child exists");
            assert_eq!(child.code.nodes.len(), NODES);
            for (index, (original, inherited)) in base.iter().zip(child.code.nodes.iter()).enumerate() {
                if index != 23 {
                    assert_eq!(original, inherited, "other genome sites must remain unchanged");
                }
            }
            let expected_edge = actor as usize;
            assert!(matches!(child.code.nodes[23], Op::Const { next, .. } if next == expected_edge));
        }
        assert_eq!(w.cell(9), Some(source));
        assert_eq!(w.cell(11), Some(patch_events.len() as i64));
        assert_eq!(w.events().iter().filter(|e| e.origin == Origin::External).count(), 4);

        let mut edit_disabled = fixture(source, false, 2000);
        let disabled_initial = edit_disabled.total_compute_inventory().unwrap();
        let disabled_attempts = edit_disabled.run(1800);
        assert_eq!(edit_disabled.total_compute_inventory(), Some(disabled_initial - disabled_attempts as u64));
        assert!(links(&edit_disabled).len() >= 3);
        assert!(mutations(&edit_disabled).is_empty());
        for actor in 1..=4 {
            assert_eq!(edit_disabled.process(actor).unwrap().code.nodes, seed(false));
        }
        assert_eq!(edit_disabled.cell(9), Some(source));

        let mut no_pool = fixture(source, true, 0);
        let cold_initial = no_pool.total_compute_inventory().unwrap();
        let cold_attempts = no_pool.run(300);
        assert_eq!(no_pool.total_compute_inventory(), Some(cold_initial - cold_attempts as u64));
        assert!(links(&no_pool).is_empty());
        assert_eq!(mutations(&no_pool), vec![(1, 23, 1)]);
        assert!(no_pool.events().iter().any(|e| {
            e.origin == Origin::Internal(1)
                && matches!(e.kind, EventKind::DrawDeferred { requested: CHILD_GRANT, .. })
        }));
    }
}
