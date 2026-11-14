//! CL-1 Study102 human-scripted native CIL branching: zero ecological genotypes.
//! Researcher-prewired copy/reproduction loop is not an evolutionary discovery.
use cl0_world::{EventKind, Intervention, Op, Origin, Process, World};
use std::collections::{BTreeMap, BTreeSet};

const GRANT: u64 = 240;
const NODES: usize = 16;

fn source_code(copy_disabled: bool) -> Vec<Op> {
    let mut nodes = vec![
        Op::Const { dst: 2, value: 7, next: 1 },
        Op::Const { dst: 3, value: 9, next: 2 },
        Op::Read { dst: 0, address: 2, next: 3 },
        Op::Write { address: 3, value: 0, next: 4 },
        Op::Const { dst: 2, value: 1, next: 5 },
        Op::Const { dst: 1, value: 0, next: 6 },
        Op::Const { dst: 3, value: 16, next: 7 },
        Op::GraphNew { dst: 0, next: 8 },
        Op::GraphCopyNode { graph: 0, source: 1, dst: 1, next: 9 },
        Op::Add { dst: 1, lhs: 1, rhs: 2, next: 10 },
        Op::Sub { dst: 3, lhs: 3, rhs: 2, next: 11 },
        Op::Branch { test: 3, nonzero: 8, zero: 12 },
        Op::Const { dst: 2, value: 240, next: 13 },
        Op::DrawCompute { amount: 2, next: 14 },
        Op::GraphLaunch { graph: 0, credits: 2, dst: 3, next: 15 },
        Op::Const { dst: 1, value: 0, next: 4 },
    ];
    if copy_disabled {
        nodes[8] = Op::Const { dst: 1, value: 0, next: 9 };
    }
    assert_eq!(nodes.len(), NODES);
    nodes
}

fn world(source: i64, pool: u64, copy_disabled: bool) -> World {
    let mut w = World::with_strict_capacity(4, 64, 1024)
        .with_pool_paid_draw()
        .with_recoverable_draw_shortage();
    w.load(Process::new(1, source_code(copy_disabled), 400))
        .unwrap();
    w.intervene(Intervention::SetCell { key: 7, value: source })
        .unwrap();
    w.intervene(Intervention::SetComputePool { credits: pool })
        .unwrap();
    w
}

fn genealogy(w: &World) -> Vec<(u64, u64)> {
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
                assert_eq!(transferred_credits, GRANT);
                assert_eq!(moved_nodes, NODES);
                Some((parent_process, child_process))
            } else {
                None
            }
        })
        .collect()
}

#[test]
fn native_cil_can_branch_into_siblings_and_native_grandchildren() {
    for source in [3, 8] {
        let mut w = world(source, 2400, false);
        let initial = w.total_compute_inventory().unwrap();
        let charged = w.run(1500);
        assert!(charged > 200);
        assert_eq!(w.total_compute_inventory(), Some(initial - charged as u64));
        let edges = genealogy(&w);
        let first_parent_children: Vec<_> = edges
            .iter()
            .filter(|(parent, _)| *parent == 1)
            .map(|(_, child)| *child)
            .collect();
        assert!(first_parent_children.len() >= 2, "same living parent must launch native sibling offspring");
        assert_ne!(first_parent_children[0], first_parent_children[1]);
        let grandchild = edges.iter().find(|(parent, _)| *parent != 1)
            .copied().expect("a native daughter itself must reproduce");
        assert!(edges.iter().any(|(parent, _)| *parent == grandchild.0));
        let established: BTreeSet<_> = [1_u64, first_parent_children[0], first_parent_children[1], grandchild.1]
            .into_iter().collect();

        let mut read = BTreeMap::new();
        let mut writers = BTreeSet::new();
        let mut draws = BTreeSet::new();
        let mut copied = BTreeMap::<u64, BTreeSet<usize>>::new();
        for e in w.events() {
            if e.origin != Origin::Internal(e.process) {
                continue;
            }
            match &e.kind {
                EventKind::CellRead { key: 7, value } => {
                    assert_eq!(e.pc, 2);
                    assert_eq!(*value, source);
                    read.insert(e.process, *value);
                }
                EventKind::CellWritten { key: 9, value } => {
                    assert_eq!(e.pc, 3);
                    assert_eq!(*value, source);
                    assert_eq!(read.get(&e.process), Some(&source));
                    writers.insert(e.process);
                }
                EventKind::Executed if e.pc == 13 => { draws.insert(e.process); }
                EventKind::GraphNodeCopied { source_process, source_node, .. } => {
                    assert_eq!(*source_process, e.process);
                    copied.entry(e.process).or_default().insert(*source_node);
                }
                _ => {}
            }
        }
        for actor in established {
            assert!(writers.contains(&actor), "each established descendant must run own source function");
            assert_eq!(w.process(actor).unwrap().code.nodes, source_code(false));
        }
        let producers: BTreeSet<_> = edges.iter().map(|(parent, _)| *parent).collect();
        for parent in producers {
            assert!(draws.contains(&parent), "each reproducing parent must draw real shared compute");
            assert_eq!(copied.get(&parent).map(BTreeSet::len), Some(NODES));
        }
        assert_eq!(w.events().iter().filter(|e| e.origin == Origin::External).count(), 3);
        assert_eq!(w.cell(9), Some(source));

        let mut cold = world(source, 0, false);
        let cold_initial = cold.total_compute_inventory().unwrap();
        let cold_attempts = cold.run(500);
        assert_eq!(cold.total_compute_inventory(), Some(cold_initial - cold_attempts as u64));
        assert!(genealogy(&cold).is_empty());
        assert!(cold.events().iter().any(|e| {
            e.origin == Origin::Internal(1)
                && matches!(e.kind, EventKind::DrawDeferred { requested: GRANT, .. })
        }));

        let mut no_copy = world(source, 2400, true);
        let no_copy_initial = no_copy.total_compute_inventory().unwrap();
        let no_copy_attempts = no_copy.run(500);
        assert_eq!(no_copy.total_compute_inventory(), Some(no_copy_initial - no_copy_attempts as u64));
        assert!(genealogy(&no_copy).is_empty());
        assert!(no_copy.events().iter().any(|e| {
            e.origin == Origin::Internal(1)
                && matches!(e.kind, EventKind::Trapped(_))
        }), "original empty graph launch must not generate a child");
    }
}
