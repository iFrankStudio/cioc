//! Study119 HUMAN-authored native descriptor source -> whole-code self-copying child.
//! Parent has NO typed Add/Sub, but writes opcode-tag numeric descriptors to
//! construct a child whose own full native executable program reproduces itself.
use cl0_world::{EventKind, Intervention, Op, Origin, Process, World};

const DESCRIPTOR_BASE: i64 = 100;
const PARENT_CREDITS: u64 = 1200;
const SHARED_POOL: u64 = 1000;
const FIRST_CHILD_CREDITS: u64 = 400;
const GRANDCHILD_CREDITS: u64 = 200;
const CHILD_CODE_NODES: usize = 18;

fn push_next(code: &mut Vec<Op>, make: impl FnOnce(usize) -> Op) {
    let next = code.len() + 1;
    code.push(make(next));
}
fn write_const(code: &mut Vec<Op>, reg: usize, value: i64) {
    push_next(code, |next| Op::Const { dst: reg, value, next });
}
fn descriptor_word(code: &mut Vec<Op>, offset: usize, value: i64, read_tag20: bool) {
    if read_tag20 {
        assert_eq!(offset, 0);
        write_const(code, 2, 20);
        push_next(code, |next| Op::Read { dst: 3, address: 2, next });
        write_const(code, 2, DESCRIPTOR_BASE);
    } else {
        write_const(code, 2, DESCRIPTOR_BASE + offset as i64);
        write_const(code, 3, value);
    }
    push_next(code, |next| Op::Write { address: 2, value: 3, next });
}

/// Existing VM encoding, read at runtime from the parent's own CIL-written cells.
fn designed_descriptor(i: usize) -> [i64; 5] {
    match i {
        0 => [0,2,7,1,0],
        1 => [0,3,10,2,0],
        2 => [3,0,2,3,0],
        3 => [0,2,1,4,0],
        4 => [0,0,0,2,5], // The opcode tag is genuinely native-read from cell20.
        5 => [4,3,0,6,0],
        6 => [0,2,1,7,0],
        7 => [0,1,0,8,0],
        8 => [0,3,18,9,0],
        9 => [11,0,10,0,0],          // GraphNew dstR0 next10
        10 => [12,0,1,1,11],        // Full GraphCopyNode
        11 => [1,1,1,2,12],        // Add R1=R1+R2 next12
        12 => [2,3,3,2,13],        // Sub R3=R3-R2 next13
        13 => [5,3,10,14,0],       // Branch R3 != 0 ->10 else14
        14 => [0,2,200,15,0],
        15 => [23,2,16,0,0],       // DrawCompute amountR2 next16
        16 => [17,0,2,3,17],       // GraphLaunch graphR0 creditsR2 dstR3 next17
        _ => unreachable!(),
    }
}

fn parent() -> Vec<Op> {
    let mut code = Vec::new();
    push_next(&mut code, |next| Op::GraphNew { dst:0,next });
    let halt_pointer_pc = code.len();
    write_const(&mut code, 1, 0);
    for _ in 0..CHILD_CODE_NODES {
        push_next(&mut code, |next| Op::GraphCopyNode {
            graph:0,source:1,dst:3,next,
        });
    }
    for i in 0..CHILD_CODE_NODES-1 {
        let desc = designed_descriptor(i);
        for (offset, word) in desc.into_iter().enumerate() {
            descriptor_word(&mut code, offset, word, i == 4 && offset == 0);
        }
        write_const(&mut code, 1, i as i64);
        write_const(&mut code, 2, DESCRIPTOR_BASE);
        push_next(&mut code, |next| Op::GraphReplace {
            graph:0,node:1,address:2,next,
        });
    }
    write_const(&mut code, 2, FIRST_CHILD_CREDITS as i64);
    push_next(&mut code, |next| Op::GraphLaunch {
        graph:0,credits:2,dst:3,next,
    });
    code.push(Op::Halt);
    let halt_index = (code.len()-1) as i64;
    code[halt_pointer_pc]=Op::Const {
        dst:1,value:halt_index,next:halt_pointer_pc+1,
    };
    assert!(code.len() < 1024);
    assert!(code.iter().all(|op| !matches!(op, Op::Add {..}|Op::Sub {..})));
    code
}

fn child_genome(tag: i64) -> Vec<Op> {
    let math = match tag {
        1 => Op::Add { dst:0,lhs:0,rhs:2,next:5 },
        2 => Op::Sub { dst:0,lhs:0,rhs:2,next:5 },
        _ => unreachable!("tag99 must trap before constructing a runnable child"),
    };
    vec![
        Op::Const{dst:2,value:7,next:1},
        Op::Const{dst:3,value:10,next:2},
        Op::Read{dst:0,address:2,next:3},
        Op::Const{dst:2,value:1,next:4},
        math,
        Op::Write{address:3,value:0,next:6},
        Op::Const{dst:2,value:1,next:7},
        Op::Const{dst:1,value:0,next:8},
        Op::Const{dst:3,value:18,next:9},
        Op::GraphNew{dst:0,next:10},
        Op::GraphCopyNode{graph:0,source:1,dst:1,next:11},
        Op::Add{dst:1,lhs:1,rhs:2,next:12},
        Op::Sub{dst:3,lhs:3,rhs:2,next:13},
        Op::Branch{test:3,nonzero:10,zero:14},
        Op::Const{dst:2,value:GRANDCHILD_CREDITS as i64,next:15},
        Op::DrawCompute{amount:2,next:16},
        Op::GraphLaunch{graph:0,credits:2,dst:3,next:17},
        Op::Halt,
    ]
}

fn native_wrote_value(world: &World, pid: u64, key: i64, value: i64) -> usize {
    let writes: Vec<_>=world.events().iter().filter(|e| {
        e.origin==Origin::Internal(pid) &&
            matches!(e.kind, EventKind::CellWritten{key:k,value:v} if k==key && v==value)
    }).collect();
    assert!(!writes.is_empty(), "actor {pid} never wrote the sourced target {key}={value}");
    writes.len()
}

#[test]
fn original_human_builder_synthesizes_self_reproducing_native_math_child_and_grandchild() {
    let original_parent = parent();
    for source in [3_i64,8_i64] {
        for tag in [1_i64,2_i64,99_i64] {
            let mut world=World::with_strict_capacity(16,1024,10000)
                .with_pool_paid_draw()
                .with_recoverable_draw_shortage();
            world.load(Process::new(1,original_parent.clone(),PARENT_CREDITS)).unwrap();
            world.intervene(Intervention::SetCell{key:7,value:source}).unwrap();
            world.intervene(Intervention::SetCell{key:20,value:tag}).unwrap();
            world.intervene(Intervention::SetComputePool{credits:SHARED_POOL}).unwrap();
            for address in DESCRIPTOR_BASE..DESCRIPTOR_BASE+5 {
                assert_eq!(world.cell(address),None,"all descriptors must be actor-created");
            }
            assert_eq!(world.cell(10),None,"human must not insert output");
            let attempts=world.run(2500);
            println!("Study119 six-control ORIGINAL source={source} input_tag={tag} attempted={attempts} resident_processes={} pool={} native_last_event={:?}",world.events().iter().filter(|e|matches!(e.kind,EventKind::GraphLaunched{..})).count()+1,world.available_compute_pool(),world.events().last().map(|e| (&e.pc,&e.kind)));
            assert!(attempts>0 && attempts<=2500);
            assert_eq!(world.total_compute_inventory(),Some(PARENT_CREDITS+SHARED_POOL-attempts as u64));
            assert_eq!(world.events().iter().filter(|e| e.origin==Origin::External).count(),4);
            assert_eq!(world.process(1).unwrap().code.nodes,original_parent,
                "parent original program must not mutate from external CIL success");

            let copies:Vec<_>=world.events().iter().filter(|e| {
                e.origin==Origin::Internal(1) &&
                matches!(e.kind,EventKind::GraphNodeCopied{source_process:1,..})
            }).collect();
            let edits:Vec<_>=world.events().iter().filter(|e| {
                e.origin==Origin::Internal(1) &&
                matches!(e.kind,EventKind::GraphReplaced{owner:1,descriptor_base:DESCRIPTOR_BASE,..})
            }).collect();
            let parent_births:Vec<_>=world.events().iter().filter(|e| {
                e.origin==Origin::Internal(1) &&
                matches!(e.kind,EventKind::GraphLaunched{parent_process:1,..})
            }).collect();
            assert_eq!(copies.len(),CHILD_CODE_NODES,"parent must copy all 18 native Halt nodes");
            for (i,e) in copies.iter().enumerate() {
                assert!(matches!(e.kind,EventKind::GraphNodeCopied{source_process:1,new_node,..} if new_node==i));
            }
            let fields=world.events().iter().filter(|e|{
                e.origin==Origin::Internal(1) &&
                matches!(e.kind,EventKind::CellWritten{key,..}
                    if (DESCRIPTOR_BASE..DESCRIPTOR_BASE+5).contains(&key))
            }).count();
            if tag==99 {
                assert_eq!(fields,25);
                assert_eq!(edits.len(),4);
                assert!(parent_births.is_empty());
                assert!(world.process(2).is_none());
                assert_eq!(world.cell(10),None);
                assert!(world.events().iter().any(|e|e.origin==Origin::Internal(1)
                    && matches!(e.kind,EventKind::Trapped(_))));
                println!("Study119 original HUMAN invalid descriptor trap source={source}, newchild=0, grandchildren=0");
                continue;
            }
            assert_eq!(fields,(CHILD_CODE_NODES-1)*5);
            assert_eq!(edits.len(),CHILD_CODE_NODES-1);
            assert_eq!(parent_births.len(),1);
            for (index,e) in edits.iter().enumerate() {
                let expected_descriptor=if index==4 {
                    [tag,0,0,2,5]
                } else {
                    designed_descriptor(index)
                };
                assert!(matches!(&e.kind,EventKind::GraphReplaced{node,descriptor,..}
                    if *node==index && *descriptor==expected_descriptor));
            }
            let built=child_genome(tag);
            assert_eq!(world.process(2).unwrap().code.nodes,built,
                "fully native-assembled first child code is a true 18-node self-copier");
            assert!(matches!(parent_births[0].kind,EventKind::GraphLaunched{
                parent_process:1,child_process:2,transferred_credits:FIRST_CHILD_CREDITS,
                moved_nodes:CHILD_CODE_NODES,..}));
            let result=if tag==1 {source+1} else {source-1};
            native_wrote_value(&world,2,10,result);
            assert!(!world.events().iter().any(|e|
                e.origin==Origin::Internal(1) &&
                matches!(e.kind,EventKind::CellWritten{key:10,..})),
                "original parent must never execute assembled child's output function");
            let grandchild_birth=world.events().iter().find(|e| {
                e.origin==Origin::Internal(2) &&
                matches!(e.kind,EventKind::GraphLaunched {
                    parent_process:2,child_process:3,
                    transferred_credits:GRANDCHILD_CREDITS,
                    moved_nodes:CHILD_CODE_NODES,..})
            }).expect("the assembled child itself must natively launch a complete grandchild");
            assert_eq!(grandchild_birth.pc,16);
            assert_eq!(world.process(3).unwrap().code.nodes,built,
                "grandchild must inherit exact synthesized executable 18-node code");
            native_wrote_value(&world,3,10,result);
            let real_child_copies=world.events().iter().filter(|e| {
                e.origin==Origin::Internal(2)
                    && matches!(e.kind,EventKind::GraphNodeCopied{
                        source_process:2, ..
                    })
            }).count();
            assert_eq!(real_child_copies,CHILD_CODE_NODES);
            assert!(world.events().iter().any(|e| {
                e.origin==Origin::Internal(2) && e.pc==15 &&
                    matches!(e.kind,EventKind::Executed)
            }),"child must acquire real shared compute before financing grandchild");
            for pid in [2_u64,3_u64] {
                let read=world.events().iter().find(|e|e.origin==Origin::Internal(pid)
                    && matches!(e.kind,EventKind::CellRead{key:7,value} if value==source))
                    .expect("newly assembled child or grandchild did not actually read external source");
                let write=world.events().iter().find(|e|e.origin==Origin::Internal(pid)
                    && matches!(e.kind,EventKind::CellWritten{key:10,value} if value==result))
                    .expect("newly assembled descendant did not really execute its own typed arithmetic");
                assert!(read.tick<write.tick);
            }
            println!("Study119 ORIGINAL HUMAN builder verified source={source} tag={tag} born_child2=1 born_grandchild3=1 true_native_source_output={result} full_inherited_18_node_add_sub=1, no spontaneous CIL invention");
        }
    }
}
