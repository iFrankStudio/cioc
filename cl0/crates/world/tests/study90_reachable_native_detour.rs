//! Study90 Phase-0: human-authored REACHABLE native repair-path calibration.
//! Zero stochastic ecological genotypes; a researcher designed this CIL loop.
use cl0_world::{EventKind, Intervention, Op, Origin, Process, World};

fn seeded_fixture(source: i64) -> World {
    let mut world = World::with_strict_capacity(4, 16, 64)
        .with_native_executable_edit_trace();
    world
        .load(Process::new(
            1,
            vec![
                Op::Const { dst: 1, value: 8, next: 1 },
                Op::Const { dst: 2, value: 7, next: 2 },
                Op::Const { dst: 3, value: 9, next: 3 },
                Op::Read { dst: 0, address: 2, next: 4 },
                // Generic CIL edit is reachable BEFORE the injured PC8,
                // even after an externally damaged PC8->PC0 detour.
                Op::PatchNext { node: 1, target: 3, next: 5 },
                Op::Add { dst: 0, lhs: 0, rhs: 1, next: 6 },
                Op::Sub { dst: 0, lhs: 0, rhs: 1, next: 7 },
                Op::Add { dst: 0, lhs: 0, rhs: 1, next: 8 },
                Op::Sub { dst: 0, lhs: 0, rhs: 1, next: 9 },
                Op::Write { address: 3, value: 0, next: 10 },
                Op::Branch { test: 1, nonzero: 0, zero: 0 },
            ],
            96,
        ))
        .unwrap();
    world.intervene(Intervention::SetCell { key: 7, value: source }).unwrap();
    assert_eq!(world.run(19), 19);
    assert_eq!(world.process(1).unwrap().pc, 8);
    assert_eq!(writes(&world), vec![source]);
    assert!(matches!(world.process(1).unwrap().code.nodes[8], Op::Sub { next: 9, .. }));
    world
}

fn writes(world: &World) -> Vec<i64> {
    world.events().iter().filter_map(|event| {
        if event.origin != Origin::Internal(1) || event.pc != 9 {
            return None;
        }
        match &event.kind {
            EventKind::CellWritten { key: 9, value } => Some(*value),
            _ => None,
        }
    }).collect()
}

#[test]
fn diverted_actor_can_internally_reach_a_real_code_edit_before_regaining_original_function() {
    for source in [3, 8] {
        let baseline = seeded_fixture(source);
        let mut intact = baseline.clone();
        let mut patched = baseline.clone();
        let mut disabled = baseline.clone().with_native_code_edits_disabled();
        let mut host = baseline.clone().with_native_code_edits_disabled();

        // All four arms start with exactly the same pre-treatment CIL process,
        // live native source-read lineage, first output and remaining credits.
        for world in [&patched, &disabled, &host] {
            assert_eq!(baseline.process(1).unwrap(), world.process(1).unwrap());
        }
        let pre_inventory = baseline.total_compute_inventory().unwrap();
        for world in [&mut patched, &mut disabled, &mut host] {
            world.intervene(Intervention::PatchEdge {
                process: 1, node: 8, next: 0
            }).unwrap();
            assert_eq!(world.run(1), 1);
            assert_eq!(writes(world), vec![source]);
            assert_eq!(world.process(1).unwrap().pc, 0);
        }

        assert_eq!(intact.run(17), 17);
        assert!(writes(&intact).len() > 1);
        // Explicit external host repair in its separate provenance-positive arm.
        host.intervene(Intervention::PatchEdge {
            process: 1, node: 8, next: 9
        }).unwrap();
        for world in [&mut patched, &mut disabled, &mut host] {
            assert_eq!(world.run(16), 16);
        }
        assert_eq!(writes(&patched), vec![source, source]);
        assert_eq!(writes(&disabled), vec![source]);
        assert_eq!(writes(&host), vec![source, source]);

        assert!(matches!(patched.process(1).unwrap().code.nodes[8], Op::Sub { next: 9, .. }));
        assert!(matches!(disabled.process(1).unwrap().code.nodes[8], Op::Sub { next: 0, .. }));
        assert!(matches!(host.process(1).unwrap().code.nodes[8], Op::Sub { next: 9, .. }));
        assert!(patched.events().iter().any(|event| {
            event.origin == Origin::Internal(1)
                && event.pc == 4
                && matches!(event.kind, EventKind::ExecutableEdgePatched {
                    node: 8, before: 0, after: 9
                })
        }));
        assert!(disabled.events().iter().any(|event| {
            event.origin == Origin::Internal(1)
                && event.pc == 4
                && matches!(event.kind, EventKind::NativeCodeEditSuppressed { opcode_tag: 6 })
        }));
        for world in [&intact, &patched, &disabled, &host] {
            assert_eq!(world.total_compute_inventory(), Some(pre_inventory - 17));
            assert!(world.events().iter().all(|e| !matches!(e.kind, EventKind::Trapped(_))));
        }
    }
}
