//! Study80 Phase-0: HUMAN-AUTHORED instruction-level feasibility fixtures only.
//! These are not independent ecological genomes and do not establish endogenous repair.
use cl0_world::{EventKind, Intervention, Op, Origin, Process, World};

fn fixture(source: i64, native_patch: bool) -> World {
    let mut nodes = vec![
        Op::Const { dst: 1, value: 3, next: 1 }, // editable Read PC
        Op::Const { dst: 2, value: 4, next: 2 }, // source cell key AND target PC
        Op::Const { dst: 3, value: 9, next: 3 }, // output cell key
        Op::Read { dst: 0, address: 2, next: 4 },
        Op::Write { address: 3, value: 0, next: 5 },
        Op::PatchNext { node: 1, target: 2, next: 3 },
    ];
    if !native_patch {
        // Mechanism-only mutation-disabled comparator; no new VM policy is implied.
        nodes[5] = Op::Const { dst: 0, value: 0, next: 3 };
    }
    let mut w = World::with_strict_capacity(4, 16, 64);
    w.load(Process::new(1, nodes, 48)).expect("valid authored CIL fixture");
    w.intervene(Intervention::SetCell { key: 4, value: source }).unwrap();
    w
}

fn writes(w: &World) -> Vec<i64> {
    w.events().iter().filter_map(|e| {
        if e.origin == Origin::Internal(1) {
            if let EventKind::CellWritten { key: 9, value } = &e.kind {
                return Some(*value);
            }
        }
        None
    }).collect()
}

#[test]
fn native_patch_restores_a_real_functional_edge_after_external_lesion() {
    for source in [3, 8] {
        let mut w = fixture(source, true);
        assert_eq!(w.run(6), 6);
        assert_eq!(writes(&w), vec![source]); // intact native Read -> Write
        assert_eq!(w.process(1).unwrap().pc, 3);
        let before = w.total_compute_inventory().unwrap();

        // The host BREAKS the only Read -> Write edge; no replacement code is supplied.
        w.intervene(Intervention::PatchEdge { process: 1, node: 3, next: 5 }).unwrap();
        assert!(w.events().iter().any(|e| {
            e.origin == Origin::External
                && matches!(&e.kind, EventKind::Intervened(s) if s.contains("PatchEdge"))
        }));
        assert_eq!(w.run(1), 1); // native Read skips Write; functional loss
        assert_eq!(writes(&w), vec![source]);
        assert_eq!(w.process(1).unwrap().pc, 5);
        assert_eq!(w.run(1), 1); // native PatchNext repairs Read.next = 4
        assert!(matches!(
            &w.process(1).unwrap().code.nodes[3],
            Op::Read { next: 4, .. }
        ));
        assert_eq!(w.run(2), 2); // native Read -> native Write recovers
        assert_eq!(writes(&w), vec![source, source]);
        assert_eq!(w.total_compute_inventory(), Some(before - 4));
        assert!(w.events().iter().all(|e| !matches!(e.kind, EventKind::Trapped(_))));
    }
}

#[test]
fn mutation_disabled_and_explicit_host_restoration_controls() {
    for source in [3, 8] {
        let mut disabled = fixture(source, false);
        disabled.run(6); // Read and Write happened before lesion
        assert_eq!(writes(&disabled), vec![source]);
        disabled.intervene(Intervention::PatchEdge { process: 1, node: 3, next: 5 }).unwrap();
        assert_eq!(disabled.run(8), 8);
        assert_eq!(writes(&disabled), vec![source]); // loss persists with no native PatchNext
        assert!(matches!(&disabled.process(1).unwrap().code.nodes[3], Op::Read { next: 5, .. }));

        let mut host_restored = fixture(source, false);
        host_restored.run(6);
        host_restored.intervene(Intervention::PatchEdge { process: 1, node: 3, next: 5 }).unwrap();
        host_restored.run(2);
        assert_eq!(writes(&host_restored), vec![source]);
        host_restored.intervene(Intervention::PatchEdge { process: 1, node: 3, next: 4 }).unwrap();
        host_restored.run(2);
        assert_eq!(writes(&host_restored), vec![source, source]);
    }
}

#[test]
fn byte_identical_genome_native_edit_ablation_preserves_charge_and_execution_edge() {
    for source in [3, 8] {
        let mut editable = fixture(source, true);
        // The flag changes generic world law, never CIL genotype or process endowment.
        let mut suppressed = fixture(source, true).with_native_code_edits_disabled();
        assert_eq!(editable.process(1).unwrap(), suppressed.process(1).unwrap());
        assert!(!editable.native_code_edits_disabled());
        assert!(suppressed.native_code_edits_disabled());

        assert_eq!(editable.run(6), 6);
        assert_eq!(suppressed.run(6), 6);
        assert_eq!(writes(&editable), writes(&suppressed));
        assert_eq!(editable.process(1).unwrap().code, suppressed.process(1).unwrap().code);
        assert_eq!(editable.total_compute_inventory(), suppressed.total_compute_inventory());

        for world in [&mut editable, &mut suppressed] {
            world
                .intervene(Intervention::PatchEdge {
                    process: 1,
                    node: 3,
                    next: 5,
                })
                .unwrap();
            assert_eq!(world.run(2), 2); // first no-output Read, then native edit attempt
        }
        assert_eq!(editable.process(1).unwrap().pc, 3);
        assert_eq!(suppressed.process(1).unwrap().pc, 3);
        assert!(matches!(
            editable.process(1).unwrap().code.nodes[3],
            Op::Read { next: 4, .. }
        ));
        assert!(matches!(
            suppressed.process(1).unwrap().code.nodes[3],
            Op::Read { next: 5, .. }
        ));
        assert!(suppressed.events().iter().any(|e| {
            e.origin == Origin::Internal(1)
                && matches!(
                    e.kind,
                    EventKind::NativeCodeEditSuppressed { opcode_tag: 6 }
                )
        }));
        assert!(!editable.events().iter().any(|e| {
            matches!(e.kind, EventKind::NativeCodeEditSuppressed { .. })
        }));
        assert_eq!(editable.total_compute_inventory(), suppressed.total_compute_inventory());

        assert_eq!(editable.run(2), 2);
        assert_eq!(suppressed.run(2), 2);
        assert_eq!(writes(&editable), vec![source, source]);
        assert_eq!(writes(&suppressed), vec![source]);
        assert_eq!(editable.total_compute_inventory(), suppressed.total_compute_inventory());
        assert!(editable.events().iter().all(|e| !matches!(e.kind, EventKind::Trapped(_))));
        assert!(suppressed.events().iter().all(|e| !matches!(e.kind, EventKind::Trapped(_))));
    }
}

#[test]
fn native_edit_ablation_does_not_turn_invalid_edit_into_success() {
    for disabled in [false, true] {
        let nodes = vec![
            Op::Const {
                dst: 1,
                value: 99,
                next: 1,
            },
            Op::Const {
                dst: 2,
                value: 0,
                next: 2,
            },
            Op::PatchNext {
                node: 1,
                target: 2,
                next: 0,
            },
        ];
        let mut world = World::with_strict_capacity(4, 8, 16);
        if disabled {
            world = world.with_native_code_edits_disabled();
        }
        world.load(Process::new(1, nodes.clone(), 8)).unwrap();
        assert_eq!(world.run(3), 3);
        assert_eq!(world.process(1).unwrap().code.nodes, nodes);
        assert!(world.process(1).unwrap().stopped);
        assert_eq!(world.process(1).unwrap().credits, 5);
        assert!(world.events().iter().any(|e| {
            e.origin == Origin::Internal(1)
                && matches!(&e.kind, EventKind::Trapped(reason) if reason == "invalid node")
        }));
        assert!(!world.events().iter().any(|e| {
            matches!(e.kind, EventKind::NativeCodeEditSuppressed { .. })
        }));
    }
}
