//! Study86 ZERO-GENOME human-authored calibration: re-encounterable lesion.
//! This is only a VM positive/negative assay, NOT spontaneous stochastic repair.
use cl0_world::{EventKind, Intervention, Op, Origin, Process, World};

fn fixture(source: i64) -> World {
    let mut nodes = vec![
        Op::Const { dst: 1, value: 8, next: 1 },
        Op::Const { dst: 2, value: 7, next: 2 },
        Op::Const { dst: 3, value: 9, next: 3 },
    ];
    for pc in 3..7 {
        nodes.push(Op::Const { dst: 0, value: 0, next: pc + 1 });
    }
    nodes.extend([
        Op::Read { dst: 0, address: 2, next: 8 },
        Op::DrawCompute { amount: 1, next: 9 },
        Op::Write { address: 3, value: 0, next: 10 },
        Op::Branch { test: 0, nonzero: 7, zero: 7 },
    ]);
    let mut world = World::with_strict_capacity(2, 16, 32)
        .with_pool_paid_draw()
        .with_recoverable_draw_shortage()
        .with_native_executable_edit_trace();
    world.load(Process::new(1, nodes, 80)).unwrap();
    world.intervene(Intervention::SetCell { key: 7, value: source }).unwrap();
    world.intervene(Intervention::SetComputePool { credits: 400 }).unwrap();
    world
}

fn native_outputs(world: &World) -> Vec<i64> {
    world.events().iter().filter_map(|event| {
        if event.origin == Origin::Internal(1) {
            if let EventKind::CellWritten { key: 9, value } = &event.kind {
                return Some(*value);
            }
        }
        None
    }).collect()
}

#[test]
fn self_loop_lesion_stays_reachable_and_host_restoration_recovers_original_source_function() {
    for source in [3, 8] {
        let mut world = fixture(source);
        let mut control = world.clone();
        assert_eq!(world.run(12), 12);
        assert_eq!(control.run(12), 12);
        assert_eq!(world.process(1).unwrap().pc, 8);
        assert_eq!(native_outputs(&world), vec![source]);
        let original_edge = match &world.process(1).unwrap().code.nodes[8] {
            Op::DrawCompute { next, .. } => *next,
            _ => panic!("source fixture has no native Draw node8"),
        };
        assert_eq!(original_edge, 9);
        world.intervene(Intervention::PatchEdge { process: 1, node: 8, next: 8 }).unwrap();
        control.intervene(Intervention::PatchEdge { process: 1, node: 8, next: 8 }).unwrap();
        assert_eq!(world.run(4), 4);
        assert_eq!(control.run(4), 4);
        assert_eq!(world.process(1).unwrap().pc, 8);
        assert_eq!(native_outputs(&world), vec![source]);

        world.intervene(Intervention::PatchEdge { process: 1, node: 8, next: original_edge }).unwrap();
        assert_eq!(world.run(2), 2);
        assert_eq!(native_outputs(&world), vec![source, source]);
        assert!(world.events().iter().any(|event| {
            event.origin == Origin::External
                && matches!(&event.kind, EventKind::Intervened(s) if s.contains("PatchEdge"))
        }));
        assert_eq!(world.run(4), 4);
        assert_eq!(native_outputs(&world), vec![source, source, source]);

        assert_eq!(control.run(6), 6);
        assert_eq!(native_outputs(&control), vec![source]);
        assert_eq!(control.process(1).unwrap().pc, 8);
        assert_eq!(world.total_compute_inventory(), Some(480 - 22));
        assert_eq!(control.total_compute_inventory(), Some(480 - 22));
        assert!(!world.process(1).unwrap().stopped);
        assert!(!control.process(1).unwrap().stopped);
    }
}
