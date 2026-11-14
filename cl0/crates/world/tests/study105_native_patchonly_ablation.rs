//! CL-1 Study105: generic PatchNext-only experimental ablation mechanics.
//! HUMAN-authored source, no independent stochastic ecology or evolution.
use cl0_world::{EventKind, Intervention, Op, Origin, Process, World};
use std::collections::{BTreeMap, BTreeSet};

fn self_copying_mutator() -> Vec<Op> {
    let nodes = vec![
        Op::Const { dst: 2, value: 7, next: 1 },
        Op::Const { dst: 3, value: 9, next: 2 },
        Op::Read { dst: 0, address: 2, next: 3 },
        Op::Write { address: 3, value: 0, next: 4 },
        Op::Const { dst: 1, value: 19, next: 5 },
        Op::Const { dst: 2, value: 1, next: 6 },
        Op::PatchNext { node: 1, target: 2, next: 7 },
        Op::Const { dst: 2, value: 1, next: 8 },
        Op::Const { dst: 1, value: 0, next: 9 },
        Op::Const { dst: 3, value: 20, next: 10 },
        Op::GraphNew { dst: 0, next: 11 },
        Op::GraphCopyNode { graph: 0, source: 1, dst: 1, next: 12 },
        Op::Add { dst: 1, lhs: 1, rhs: 2, next: 13 },
        Op::Sub { dst: 3, lhs: 3, rhs: 2, next: 14 },
        Op::Branch { test: 3, nonzero: 11, zero: 15 },
        Op::Const { dst: 2, value: 200, next: 16 },
        Op::DrawCompute { amount: 2, next: 17 },
        Op::GraphLaunch { graph: 0, credits: 2, dst: 3, next: 18 },
        Op::Halt,
        Op::Const { dst: 0, value: 0, next: 19 }, // harmless heritable change 19->1
    ];
    assert_eq!(nodes.len(), 20);
    nodes
}

fn world(source: i64, patch_disabled: bool) -> World {
    let mut w = World::with_strict_capacity(4, 64, 512)
        .with_pool_paid_draw()
        .with_recoverable_draw_shortage()
        .with_native_executable_edit_trace();
    if patch_disabled {
        w = w.with_native_patchnext_disabled();
    }
    w.load(Process::new(1, self_copying_mutator(), 220)).unwrap();
    w.intervene(Intervention::SetCell { key: 7, value: source }).unwrap();
    w.intervene(Intervention::SetComputePool { credits: 1000 }).unwrap();
    w
}

fn child_links(w: &World) -> Vec<(u64, u64)> {
    w.events().iter().filter_map(|e| {
        if let EventKind::GraphLaunched {
            parent_process,
            child_process,
            transferred_credits,
            moved_nodes,
            ..
        } = e.kind {
            assert_eq!(e.origin, Origin::Internal(parent_process));
            assert_eq!(e.pc, 17);
            assert_eq!((transferred_credits, moved_nodes), (200, 20));
            Some((parent_process, child_process))
        } else {
            None
        }
    }).collect()
}

#[test]
fn same_genome_narrow_patch_suppression_preserves_native_whole_code_reproduction() {
    for source in [3, 8] {
        let mut enabled = world(source, false);
        let mut disabled = world(source, true);
        assert_eq!(enabled.process(1).unwrap(), disabled.process(1).unwrap());
        assert!(!enabled.native_patchnext_disabled());
        assert!(disabled.native_patchnext_disabled());
        let start_enabled = enabled.total_compute_inventory().unwrap();
        let start_disabled = disabled.total_compute_inventory().unwrap();
        assert_eq!(start_enabled, start_disabled);
        let attempted_enabled = enabled.run(800);
        let attempted_disabled = disabled.run(800);
        assert_eq!(attempted_enabled, attempted_disabled,
            "neutral mutation suppression must not itself change native instruction charges");
        for (w, original_inventory, attempted) in [
            (&enabled, start_enabled, attempted_enabled),
            (&disabled, start_disabled, attempted_disabled),
        ] {
            assert_eq!(w.total_compute_inventory(), Some(original_inventory - attempted as u64));
            assert_eq!(w.events().iter().filter(|e| e.origin == Origin::External).count(), 3);
            assert_eq!(w.cell(9), Some(source));
            let links = child_links(w);
            assert!(links.len() >= 2, "must keep true native children and grandchildren");
            assert!(links.iter().any(|(parent, _)| *parent >= 2),
                "native code copying must stay active under the narrowed control");

            let mut native_reads = BTreeSet::new();
            let mut native_writes = BTreeSet::new();
            let mut copied = BTreeMap::<u64, BTreeSet<usize>>::new();
            let mut native_draws = BTreeSet::new();
            for e in w.events() {
                if e.origin != Origin::Internal(e.process) {
                    continue;
                }
                match &e.kind {
                    EventKind::CellRead { key: 7, value } => {
                        assert_eq!(e.pc, 2);
                        assert_eq!(*value, source);
                        native_reads.insert(e.process);
                    }
                    EventKind::CellWritten { key: 9, value } => {
                        assert_eq!(e.pc, 3);
                        assert_eq!(*value, source);
                        assert!(native_reads.contains(&e.process));
                        native_writes.insert(e.process);
                    }
                    EventKind::GraphNodeCopied { source_process, source_node, .. } => {
                        assert_eq!(*source_process, e.process);
                        copied.entry(e.process).or_default().insert(*source_node);
                    }
                    EventKind::Executed if e.pc == 16 => {
                        native_draws.insert(e.process);
                    }
                    _ => {}
                }
            }
            for (parent, _) in links {
                assert!(native_writes.contains(&parent));
                assert!(native_draws.contains(&parent));
                assert_eq!(copied.get(&parent).map(BTreeSet::len), Some(20));
            }
        }

        let enabled_links = child_links(&enabled);
        let disabled_links = child_links(&disabled);
        assert_eq!(enabled_links, disabled_links, "only mutation effect may differ");
        assert!(enabled.events().iter().any(|e| {
            e.origin == Origin::Internal(1)
                && e.pc == 6
                && matches!(e.kind,
                    EventKind::ExecutableEdgePatched { node: 19, before: 19, after: 1 })
        }));
        assert!(!disabled.events().iter().any(|e| {
            matches!(e.kind, EventKind::ExecutableEdgePatched { .. })
        }));
        assert!(disabled.events().iter().any(|e| {
            e.origin == Origin::Internal(1)
                && e.pc == 6
                && matches!(e.kind, EventKind::NativeCodeEditSuppressed { opcode_tag: 6 })
        }));
        assert!(!enabled.events().iter().any(|e| {
            matches!(e.kind, EventKind::NativeCodeEditSuppressed { .. })
        }));
        for actor in 1..=3 {
            assert!(matches!(enabled.process(actor).unwrap().code.nodes[19],
                Op::Const { next: 1, .. }));
            assert!(matches!(disabled.process(actor).unwrap().code.nodes[19],
                Op::Const { next: 19, .. }));
        }
    }
}

#[test]
fn invalid_native_patchnext_traps_with_unchanged_cost_and_reason() {
    for disabled in [false, true] {
        let source = vec![
            Op::Const { dst: 1, value: 99, next: 1 },
            Op::Const { dst: 2, value: 0, next: 2 },
            Op::PatchNext { node: 1, target: 2, next: 0 },
        ];
        let mut w = World::with_strict_capacity(4, 8, 16);
        if disabled {
            w = w.with_native_patchnext_disabled();
        }
        w.load(Process::new(1, source.clone(), 8)).unwrap();
        assert_eq!(w.run(3), 3);
        assert_eq!(w.process(1).unwrap().credits, 5);
        assert_eq!(w.process(1).unwrap().code.nodes, source);
        assert!(w.process(1).unwrap().stopped);
        assert!(w.events().iter().any(|e| {
            e.origin == Origin::Internal(1)
                && matches!(&e.kind, EventKind::Trapped(reason) if reason == "invalid edge")
        }));
        assert!(!w.events().iter().any(|e| {
            matches!(e.kind, EventKind::NativeCodeEditSuppressed { .. })
        }));
    }
}
