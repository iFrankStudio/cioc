//! Study120 HUMAN-authored native descriptor whole tag neighborhood: all tags, not selected successes.
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

/// Count true actor-native source→output (not a host-computed stand-in).
fn actor_sensor_and_output(w: &World, pid: u64, source: i64) -> Option<i64> {
    let read = w.events().iter().find(|e| {
        e.origin == Origin::Internal(pid) && e.pc == 2
            && matches!(e.kind, EventKind::CellRead { key: 7, value } if value == source)
    })?;
    let write = w.events().iter().find(|e| {
        e.origin == Origin::Internal(pid) && e.pc == 5 && e.tick > read.tick
            && matches!(e.kind, EventKind::CellWritten { key: 10, .. })
    })?;
    if let EventKind::CellWritten { value, .. } = write.kind {
        Some(value)
    } else {
        None
    }
}

#[test]
fn complete_precommitted_32_tag_native_assembler_child_grandchild_census() {
    let original_parent=parent();
    assert!(original_parent.iter().all(|op| !matches!(op,Op::Add {..}|Op::Sub {..})));
    let mut both_sources = std::collections::BTreeMap::<i64, [usize; 7]>::new();
    let mut observed = std::collections::BTreeMap::<(i64,i64), (bool,Option<i64>,bool,Option<i64>)>::new();
    let mut seen = 0_usize;
    for tag in (0_i64..32).chain(std::iter::once(99)) {
        for source in [3_i64,8_i64] {
            let mut w=World::with_strict_capacity(16,1024,10000)
                .with_pool_paid_draw().with_recoverable_draw_shortage();
            w.load(Process::new(1,original_parent.clone(),PARENT_CREDITS)).unwrap();
            w.intervene(Intervention::SetCell { key:7,value:source }).unwrap();
            w.intervene(Intervention::SetCell { key:20,value:tag }).unwrap();
            w.intervene(Intervention::SetComputePool { credits:SHARED_POOL }).unwrap();
            let actual=w.run(2500);
            assert!(actual>0 && actual<=2500);
            assert_eq!(w.total_compute_inventory(),Some(PARENT_CREDITS+SHARED_POOL-actual as u64));
            assert_eq!(w.events().iter().filter(|e|e.origin==Origin::External).count(),4);
            assert_eq!(w.process(1).unwrap().code.nodes, original_parent);
            assert!(!w.events().iter().any(|e|e.origin==Origin::Internal(1)
                && matches!(e.kind,EventKind::CellWritten { key:10,.. })),
                "original human assembler must never directly produce descendant output");
            let parent_copies: Vec<_>=w.events().iter().filter(|e| {
                e.origin==Origin::Internal(1)
                    && matches!(e.kind,EventKind::GraphNodeCopied { source_process:1,.. })
            }).collect();
            assert_eq!(parent_copies.len(),CHILD_CODE_NODES,"parent must make full raw graph");
            let replacements:Vec<_>=w.events().iter().filter(|e| {
                e.origin==Origin::Internal(1)
                    && matches!(e.kind,EventKind::GraphReplaced { owner:1,.. })
            }).collect();
            for (i,e) in replacements.iter().enumerate() {
                let expect=if i==4 {[tag,0,0,2,5]} else {designed_descriptor(i)};
                assert!(matches!(&e.kind,EventKind::GraphReplaced {
                    node,descriptor,descriptor_base:DESCRIPTOR_BASE,..
                } if *node==i && *descriptor==expect));
            }
            let desc_count=w.events().iter().filter(|e| {
                e.origin==Origin::Internal(1)
                    && matches!(e.kind,EventKind::CellWritten { key,.. }
                        if (DESCRIPTOR_BASE..DESCRIPTOR_BASE+5).contains(&key))
            }).count();
            let real_tag_read=w.events().iter().any(|e| {
                e.origin==Origin::Internal(1)
                    && matches!(e.kind,EventKind::CellRead { key:20,value } if value==tag)
            });
            assert!(real_tag_read,"the tag must be obtained by native descriptor read");
            assert!(desc_count>=25 && desc_count<=85);
            assert!(replacements.len()>=4 && replacements.len()<=17);
            let parent_birth:Vec<_>=w.events().iter().filter(|e| {
                e.origin==Origin::Internal(1)
                    && matches!(e.kind,EventKind::GraphLaunched {
                        parent_process:1,child_process:2,..
                    })
            }).collect();
            assert!(parent_birth.len()<=1);
            let child=parent_birth.len()==1;
            if child {
                assert_eq!(replacements.len(),17,"native GraphReplace must construct all child nodes");
                assert_eq!(desc_count,85,"original actor must write all descriptors");
                assert!(matches!(parent_birth[0].kind,EventKind::GraphLaunched {
                    transferred_credits:FIRST_CHILD_CREDITS,moved_nodes:CHILD_CODE_NODES,..
                }));
                // Some valid but non-self-copying descriptors execute genuine
                // native code-append operations after birth (e.g. tag8).
                // Birth is proven by moved_nodes=18, not final resident length.
                assert!(w.process(2).unwrap().code.nodes.len()>=CHILD_CODE_NODES,
                    "resident child may grow code, never erase its assembled prefix");
                let kind=&w.process(2).unwrap().code.nodes[4];
                assert!(match tag {
                    1 => matches!(kind,Op::Add {dst:0,lhs:0,rhs:2,next:5}),
                    2 => matches!(kind,Op::Sub {dst:0,lhs:0,rhs:2,next:5}),
                    _ => true,
                });
            } else {
                assert!(w.process(2).is_none(),"host or native code fabricated a child");
            }
            let child_output=if child {actor_sensor_and_output(&w,2,source)} else {None};
            let native_grandchild=w.events().iter().find(|e| {
                e.origin==Origin::Internal(2)
                    && matches!(e.kind,EventKind::GraphLaunched {
                        parent_process:2,child_process:3,..
                    })
            });
            let grandchild=native_grandchild.is_some();
            let mut grandchild_exact=false;
            let mut grandchild_output=None;
            if let Some(event)=native_grandchild {
                if let EventKind::GraphLaunched {
                    transferred_credits, moved_nodes, ..
                }=event.kind {
                    grandchild_exact=transferred_credits==GRANDCHILD_CREDITS
                        && moved_nodes==CHILD_CODE_NODES
                        && w.process(3).is_some()
                        && w.process(3).unwrap().code.nodes==w.process(2).unwrap().code.nodes;
                }
                grandchild_output=actor_sensor_and_output(&w,3,source);
                assert!(w.events().iter().any(|e|e.origin==Origin::Internal(2)
                    && e.pc==15 && matches!(e.kind,EventKind::Executed)),
                    "grandchild must be preceded by true native DrawCompute");
            }
            if tag==99 {
                assert!(!child && !grandchild && child_output.is_none());
                assert_eq!(replacements.len(),4);
                assert_eq!(desc_count,25);
            }
            if tag==1 || tag==2 {
                let expect=if tag==1 {source+1} else {source-1};
                assert!(child && grandchild && grandchild_exact);
                assert_eq!(child_output,Some(expect));
                assert_eq!(grandchild_output,Some(expect));
                assert_eq!(w.process(2).unwrap().code.nodes,child_genome(tag));
                assert_eq!(w.process(3).unwrap().code.nodes,child_genome(tag));
            }
            let entry=both_sources.entry(tag).or_insert([0;7]);
            entry[0]+=usize::from(child);
            entry[1]+=usize::from(child_output.is_some());
            entry[2]+=usize::from(grandchild);
            entry[3]+=usize::from(grandchild_exact);
            entry[4]+=usize::from(grandchild_output.is_some());
            entry[5]+=usize::from(w.events().iter().any(|e|e.origin==Origin::Internal(1)
                && matches!(e.kind,EventKind::Trapped(_))));
            entry[6]+=replacements.len();
            assert!(observed.insert((tag,source),
                (child,child_output,grandchild,grandchild_output)).is_none());
            seen+=1;
            println!("Study120 ORIGINAL source={source} descriptor_tag={tag} parent_replacements={} native_child={} child_source_output={child_output:?} native_grandchild={} exact_child_genome={} grandchild_source_output={grandchild_output:?} original_charged_attempts={actual} [PRECOMMITTED_ALL_TAGS_HUMAN_ASSEMBLER]",replacements.len(),child,grandchild,grandchild_exact);
        }
    }
    assert_eq!(seen,66);
    for tag in 0_i64..32 {
        assert!(both_sources.contains_key(&tag),"no tag in census can be dropped");
    }
    for tag in [1_i64,2_i64] {
        let low=observed[&(tag,3)].1.unwrap();
        let high=observed[&(tag,8)].1.unwrap();
        assert_eq!(high-low,5,"known math must truly depend on source");
    }
    println!("Study120 ORIGINAL COMPLETE: descriptor_tags=32 plus_invalid_sentinel=1 source_controls=2 original_worlds={seen} source_value_dependent=true");
    for (tag,counts) in both_sources {
        println!("Study120 ORIGINAL per_tag={tag} source_pair_child={} source_pair_child_writers={} source_pair_grandchild={} source_pair_exact_grandchild={} source_pair_grandchild_writers={} source_pair_parent_traps={} source_pair_native_replaced_nodes={} [NOT_NATURAL_ORIGIN]",counts[0],counts[1],counts[2],counts[3],counts[4],counts[5],counts[6]);
    }
}
