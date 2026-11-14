//! Study118: HUMAN-authored CIL builder creates a new typed executable child
//! opcode from CIL-written descriptors. Not an emergent algorithm or life.
use cl0_world::{EventKind, Intervention, Op, Origin, Process, World};

const DESCRIPTOR_BASE: i64 = 100;
const CHILD_GRANT: u64 = 60;
const PARENT_INITIAL: u64 = 500;

fn append(code: &mut Vec<Op>, make: impl FnOnce(usize) -> Op) {
    let next = code.len() + 1;
    code.push(make(next));
}

fn constant(code: &mut Vec<Op>, dst: usize, value: i64) {
    append(code, |next| Op::Const { dst, value, next });
}

fn descriptor_word(code: &mut Vec<Op>, index: usize, word: i64, native_read_tag: bool) {
    if native_read_tag {
        // Typed new child math instruction is selected by a REAL native
        // Read on environmental numeric information, not by host code edits.
        assert_eq!(index, 0);
        constant(code, 2, 20);
        append(code, |next| Op::Read {
            dst: 3, address: 2, next,
        });
        constant(code, 2, DESCRIPTOR_BASE);
    } else {
        constant(code, 2, DESCRIPTOR_BASE + index as i64);
        constant(code, 3, word);
    }
    append(code, |next| Op::Write {
        address: 2, value: 3, next,
    });
}

fn desired_descriptor(node: usize) -> [i64; 5] {
    match node {
        0 => [0, 2, 7, 1, 0],         // Const R2=source key7 ->1
        1 => [3, 0, 2, 2, 0],         // Read source7 into R0 ->2
        2 => [0, 3, 10, 3, 0],        // Const R3=output key10 ->3
        3 => [0, 2, 1, 4, 0],         // Const R2=one ->4
        4 => [0, 0, 0, 2, 5],         // First tag filled by native Read20
        5 => [4, 3, 0, 6, 0],         // Write output10 from R0 ->6
        _ => unreachable!("only node0..5 is replaced"),
    }
}

fn parent_builder() -> Vec<Op> {
    let mut code = Vec::<Op>::new();
    append(&mut code, |next| Op::GraphNew { dst: 0, next });
    let source_index_const = code.len();
    constant(&mut code, 1, 0); // Patched below to our own final Halt index.
    for _ in 0..7 {
        append(&mut code, |next| Op::GraphCopyNode {
            graph: 0, source: 1, dst: 3, next,
        });
    }

    for node in 0..6 {
        let descriptor = desired_descriptor(node);
        for (offset, word) in descriptor.into_iter().enumerate() {
            descriptor_word(&mut code, offset, word, node == 4 && offset == 0);
        }
        constant(&mut code, 1, node as i64);
        constant(&mut code, 2, DESCRIPTOR_BASE);
        append(&mut code, |next| Op::GraphReplace {
            graph: 0, node: 1, address: 2, next,
        });
    }
    constant(&mut code, 2, CHILD_GRANT as i64);
    append(&mut code, |next| Op::GraphLaunch {
        graph: 0, credits: 2, dst: 3, next,
    });
    code.push(Op::Halt);
    let own_halt_index = (code.len() - 1) as i64;
    code[source_index_const] = Op::Const {
        dst: 1, value: own_halt_index, next: source_index_const + 1,
    };
    assert!(code.len() < 256);
    assert!(code.iter().all(|op| !matches!(op, Op::Add { .. } | Op::Sub { .. })),
        "the parent builder must NOT contain an executable Add or Sub");
    code
}

fn generated_child(math_opcode_tag: i64) -> Vec<Op> {
    let computation = match math_opcode_tag {
        1 => Op::Add { dst: 0, lhs: 0, rhs: 2, next: 5 },
        2 => Op::Sub { dst: 0, lhs: 0, rhs: 2, next: 5 },
        _ => panic!("no functional child may be synthesized from invalid data tag"),
    };
    vec![
        Op::Const { dst: 2, value: 7, next: 1 },
        Op::Read { dst: 0, address: 2, next: 2 },
        Op::Const { dst: 3, value: 10, next: 3 },
        Op::Const { dst: 2, value: 1, next: 4 },
        computation,
        Op::Write { address: 3, value: 0, next: 6 },
        Op::Halt,
    ]
}

#[test]
fn internal_numeric_descriptors_are_converted_to_new_executable_child_ops() {
    let parent = parent_builder();
    for source in [3_i64, 8_i64] {
        for tag in [1_i64, 2_i64, 99_i64] {
            let mut world = World::with_strict_capacity(16, 256, 1024);
            world.load(Process::new(1, parent.clone(), PARENT_INITIAL)).unwrap();
            world.intervene(Intervention::SetCell { key: 7, value: source }).unwrap();
            world.intervene(Intervention::SetCell { key: 20, value: tag }).unwrap();
            assert_eq!(world.cell(10), None, "host cannot initialize native output");
            for key in DESCRIPTOR_BASE..DESCRIPTOR_BASE + 5 {
                assert_eq!(world.cell(key), None, "all five instruction descriptor cells must start absent");
            }
            let attempts = world.run(250);
            println!("Study118 six-world original mechanism trace: source={source}, tag={tag}, charged_native_attempts={attempts}, native_last_event={:?}", world.events().last().map(|e| (&e.pc, &e.kind)));
            assert!(attempts > 0 && attempts <= 250,
                "only the prespecified maximum 250 attempted native operations is a scientific bound");
            assert_eq!(world.total_compute_inventory(),
                Some(PARENT_INITIAL - attempts as u64),
                "parent and child must consume real credits without minting compute");
            assert_eq!(world.events().iter()
                .filter(|e| e.origin == Origin::External).count(), 3,
                "exactly one parent load and two original external data inputs");

            let created: Vec<_> = world.events().iter().filter(|e|
                matches!(e.kind, EventKind::GraphCreated { owner: 1, .. })
                && e.origin == Origin::Internal(1)).collect();
            let copied: Vec<_> = world.events().iter().filter(|e|
                matches!(e.kind, EventKind::GraphNodeCopied {
                    owner: 1, source_process: 1, ..
                }) && e.origin == Origin::Internal(1)).collect();
            let replaced: Vec<_> = world.events().iter().filter(|e|
                matches!(e.kind, EventKind::GraphReplaced {
                    owner: 1, descriptor_base: DESCRIPTOR_BASE, ..
                }) && e.origin == Origin::Internal(1)).collect();
            let launched: Vec<_> = world.events().iter().filter(|e|
                matches!(e.kind, EventKind::GraphLaunched {
                    parent_process: 1, ..
                }) && e.origin == Origin::Internal(1)).collect();
            assert_eq!(created.len(), 1, "graph must be allocated from CIL");
            assert_eq!(copied.len(), 7, "all seven original Halt nodes must be copied by CIL");
            for (i, event) in copied.iter().enumerate() {
                assert!(matches!(event.kind, EventKind::GraphNodeCopied {
                    source_process: 1, new_node, ..
                } if new_node == i));
            }
            // All individual descriptor fields must have originated from
            // actual CIL Write, never from an external SetCell intervention.
            let authored_fields = world.events().iter().filter(|e| {
                e.origin == Origin::Internal(1)
                    && matches!(e.kind, EventKind::CellWritten { key, .. }
                        if (DESCRIPTOR_BASE..DESCRIPTOR_BASE + 5).contains(&key))
            }).count();
            assert_eq!(authored_fields, 30, "6 node descriptors * 5 internal cell writes");
            if tag == 99 {
                assert_eq!(replaced.len(), 4,
                    "unknown tag must trap precisely at the fifth native GraphReplace");
                assert!(launched.is_empty());
                assert!(world.process(2).is_none());
                assert_eq!(world.cell(10), None);
                assert!(world.events().iter().any(|e|
                    e.origin == Origin::Internal(1)
                    && matches!(e.kind, EventKind::Trapped(_))));
                println!("Study118 original human calibration: source={source}, tag=99, typed-invalid trap, native attempts={attempts}, no child");
                continue;
            }
            assert_eq!(replaced.len(), 6, "all six child opcode replacements must really execute");
            assert_eq!(launched.len(), 1, "only the CIL actor may launch its own child");
            for (i, event) in replaced.iter().enumerate() {
                match &event.kind {
                    EventKind::GraphReplaced { node, descriptor, owner, .. } => {
                        assert_eq!(*owner, 1);
                        assert_eq!(*node, i);
                        let expected = if i == 4 {
                            [tag, 0, 0, 2, 5]
                        } else {
                            desired_descriptor(i)
                        };
                        assert_eq!(*descriptor, expected,
                            "source-created descriptor contents must match child executable node");
                    }
                    _ => unreachable!(),
                }
            }
            assert!(matches!(launched[0].kind,
                EventKind::GraphLaunched {
                    child_process: 2, transferred_credits: CHILD_GRANT, moved_nodes: 7, ..
                }));
            assert_eq!(world.process(1).unwrap().code.nodes, parent,
                "parent program source must not be patched by host or CIL");
            assert_eq!(world.process(2).unwrap().code.nodes, generated_child(tag),
                "child typed executable Add/Sub must be native-generated from data");
            let expected_result = if tag == 1 { source + 1 } else { source - 1 };
            assert_eq!(world.cell(10), Some(expected_result));
            let read = world.events().iter().find(|e| e.process == 2
                && e.origin == Origin::Internal(2)
                && e.pc == 1
                && matches!(e.kind, EventKind::CellRead { key: 7, value } if value == source))
                .expect("only newborn may execute correct source-dependent read");
            let write = world.events().iter().find(|e| e.process == 2
                && e.origin == Origin::Internal(2)
                && e.pc == 5
                && matches!(e.kind, EventKind::CellWritten { key: 10, value } if value == expected_result))
                .expect("child must execute newly composed source-dependent math and output");
            assert!(read.tick < write.tick);
            assert!(!world.events().iter().any(|e| e.process == 1
                && matches!(e.kind, EventKind::CellWritten { key: 10, .. })),
                "parent must not directly output child's new function");
            assert!(world.events().iter().any(|e| e.process == 2 && e.pc == 4
                && e.origin == Origin::Internal(2)
                && matches!(e.kind, EventKind::Executed)),
                "child itself must execute its newly synthesized Add/Sub instruction");
            println!("Study118 original HUMAN-CODED builder: source={source}, tag={tag}, child_output={expected_result}, native_attempts={attempts}, full-descriptor-synthesized child=1; not spontaneous CIL genesis");
        }
    }
}
