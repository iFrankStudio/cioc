//! Study96 Phase0: HUMAN-AUTHORED parent executable graph construction.
//! Mechanistic VM feasibility only. ZERO independent stochastic ecological worlds.
use cl0_world::{Event, EventKind, Intervention, Op, Origin, Process, World};

fn fixture(source: i64, mode: &str) -> World {
    assert!([3, 8].contains(&source));
    assert!(["local", "absolute-stale", "zero-credit"].contains(&mode));

    let copy = |next: usize| {
        if mode == "absolute-stale" {
            Op::GraphCopyNode { graph: 0, source: 1, dst: 2, next }
        } else {
            Op::GraphCopyNodeLocal { graph: 0, source: 1, dst: 2, next }
        }
    };
    let amount = if mode == "zero-credit" { 0 } else { 24 };

    // Parent program itself has NO executable shared Write operation in the
    // parent construction path. All source-sensitive output must be inherited
    // CIL code actually executed by the native child.
    let mut code = vec![
        Op::GraphNew { dst: 0, next: 1 },       // PC0
        Op::Const { dst: 1, value: 30, next: 2 },
        copy(3),                               // PC2, child node0
        Op::Const { dst: 1, value: 31, next: 4 },
        copy(5),                               // PC4, child node1
        Op::Const { dst: 1, value: 32, next: 6 },
        copy(7),                               // PC6, child node2
        Op::Const { dst: 1, value: 33, next: 8 },
        copy(9),                               // PC8, child node3
        Op::Const { dst: 1, value: 34, next: 10 },
        copy(11),                              // PC10, child node4
        Op::Const { dst: 1, value: 0, next: 12 },
        Op::Const { dst: 3, value: 1, next: 13 },
        Op::GraphSetNext { graph: 0, node: 1, target: 3, next: 14 },
        Op::Const { dst: 1, value: 1, next: 15 },
        Op::Const { dst: 3, value: 2, next: 16 },
        Op::GraphSetNext { graph: 0, node: 1, target: 3, next: 17 },
        Op::Const { dst: 1, value: 2, next: 18 },
        Op::Const { dst: 3, value: 3, next: 19 },
        Op::GraphSetNext { graph: 0, node: 1, target: 3, next: 20 },
        Op::Const { dst: 1, value: 3, next: 21 },
        Op::Const { dst: 3, value: 4, next: 22 },
        Op::GraphSetNext { graph: 0, node: 1, target: 3, next: 23 },
        Op::Const { dst: 1, value: amount, next: 24 },
        Op::GraphLaunch { graph: 0, credits: 1, dst: 2, next: 25 },
        Op::Halt,
    ];
    assert_eq!(code.len(), 26);
    while code.len() < 30 {
        code.push(Op::Halt);
    }
    code.extend([
        Op::Const { dst: 2, value: 7, next: 31 },      // template source node30
        Op::Const { dst: 3, value: 9, next: 32 },      // template source node31
        Op::Read { dst: 0, address: 2, next: 33 },     // template source node32
        Op::Write { address: 3, value: 0, next: 34 },  // template source node33
        Op::Branch { test: 2, nonzero: 32, zero: 32 }, // template source node34
    ]);
    assert_eq!(code.len(), 35);

    let mut world = World::with_strict_capacity(4, 48, 128);
    world.load(Process::new(1, code, 96)).expect("valid native original parent");
    world.intervene(Intervention::SetCell { key: 7, value: source })
        .expect("source port can be populated");
    world
}

fn child_writes(events: &[Event], value: i64) -> Vec<u64> {
    events.iter().filter_map(|event| {
        if event.origin == Origin::Internal(2)
            && event.process == 2
            && event.pc == 3
            && matches!(event.kind, EventKind::CellWritten { key: 9, value: v } if v == value)
        {
            Some(event.tick)
        } else {
            None
        }
    }).collect()
}

#[test]
fn fully_native_child_executes_inherited_source_reader_and_repeated_writer() {
    for source in [3, 8] {
        for mode in ["local", "absolute-stale", "zero-credit"] {
            let mut world = fixture(source, mode);
            let initial = world.total_compute_inventory().unwrap();
            assert_eq!(initial, 96);
            let attempts = world.run(100);
            assert_eq!(world.total_compute_inventory(), Some(initial - attempts as u64));
            assert!(world.events().iter().all(|event| {
                !(event.origin == Origin::Internal(1)
                    && matches!(event.kind, EventKind::CellWritten { .. }))
            }), "parent never performs native shared output");

            let parent_copies = world.events().iter().filter(|event| {
                event.origin == Origin::Internal(1)
                    && match mode {
                        "absolute-stale" => matches!(event.kind, EventKind::GraphNodeCopied { .. }),
                        _ => matches!(event.kind, EventKind::GraphNodeCopiedLocal { .. }),
                    }
            }).count();
            assert_eq!(parent_copies, 5);

            let child_launches = world.events().iter().filter(|event| {
                event.origin == Origin::Internal(1)
                    && matches!(event.kind, EventKind::GraphLaunched {
                        moved_nodes: 5, ..
                    })
            }).count();
            assert_eq!(child_launches, 1, "parent must launch real graph as child");
            let child = world.process(2).expect("native child launched");
            assert_eq!(child.code.nodes.len(), 5);
            assert!(matches!(child.code.nodes[0], Op::Const { dst: 2, value: 7, next: 1 }));
            assert!(matches!(child.code.nodes[1], Op::Const { dst: 3, value: 9, next: 2 }));
            assert!(matches!(child.code.nodes[2], Op::Read { dst: 0, address: 2, next: 3 }));
            assert!(matches!(child.code.nodes[3], Op::Write { address: 3, value: 0, next: 4 }));
            let original_branch_target = if mode == "absolute-stale" { 32 } else { 2 };
            assert!(matches!(child.code.nodes[4], Op::Branch {
                test: 2, nonzero, zero
            } if nonzero == original_branch_target && zero == original_branch_target));

            let native_child: Vec<_> = world.events().iter().filter(|event| {
                event.origin == Origin::Internal(2) && event.process == 2
            }).collect();
            let writes = child_writes(world.events(), source);
            let reads = native_child.iter().filter(|event| {
                matches!(event.kind, EventKind::CellRead { key: 7, value } if value == source)
                    && event.pc == 2
            }).count();

            match mode {
                "local" => {
                    assert_eq!(child.credits, 0);
                    assert_eq!(native_child.len(), 24);
                    assert!(reads >= 6 && writes.len() >= 6);
                    let middle = native_child.len() / 2;
                    assert!(native_child[..middle].iter().any(|e| matches!(
                        e.kind, EventKind::CellWritten { key: 9, value } if value == source
                    )));
                    assert!(native_child[middle..].iter().any(|e| matches!(
                        e.kind, EventKind::CellWritten { key: 9, value } if value == source
                    )));
                    assert!(native_child.iter().all(|event| !matches!(event.kind, EventKind::Trapped(_))));
                    assert_eq!(world.cell(9), Some(source));
                }
                "absolute-stale" => {
                    assert!(writes.len() <= 1,
                        "absolute copied Branch cannot sustain functional child outputs");
                    assert!(native_child.iter().any(|event| {
                        matches!(&event.kind, EventKind::Trapped(error) if error.contains("invalid"))
                    }));
                }
                "zero-credit" => {
                    assert_eq!(child.credits, 0);
                    assert!(native_child.is_empty());
                    assert!(writes.is_empty());
                }
                _ => unreachable!(),
            }
        }
    }
}
