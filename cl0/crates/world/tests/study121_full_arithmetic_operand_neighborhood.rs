//! Study121 HUMAN-authored native descriptor whole tag neighborhood: all tags, not selected successes.
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
fn descriptor_word(code: &mut Vec<Op>, offset: usize, value: i64, read_runtime_operand: bool) {
    if read_runtime_operand {
        assert!(offset <= 3);
        // All four variable words come from ordinary source memory, not host
        // graph/code replacement. They must be written by the CIL parent.
        write_const(code, 2, 20 + offset as i64);
        push_next(code, |next| Op::Read { dst: 3, address: 2, next });
        write_const(code, 2, DESCRIPTOR_BASE + offset as i64);
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
            descriptor_word(&mut code, offset, word, i == 4 && offset <= 3);
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

#[derive(Clone, Copy, Debug)]
struct Outcome121 {
    child_output: Option<i64>,
    grandchild_output: Option<i64>,
    full_native_grandchild: bool,
}

#[test]
fn precommitted_every_add_sub_register_operand_is_natively_built_and_inherited() {
    let parent_code=parent();
    assert!(parent_code.iter().all(|op| !matches!(op, Op::Add {..} | Op::Sub {..})));
    let mut rows=std::collections::BTreeMap::<(i64,usize,usize,usize,i64),Outcome121>::new();
    let mut worlds=0_usize;
    for tag in [1_i64,2_i64] {
        for dst in 0_usize..4 {
            for lhs in 0_usize..4 {
                for rhs in 0_usize..4 {
                    for source in [3_i64,8_i64] {
                        let mut w=World::with_strict_capacity(16,1024,10000)
                            .with_pool_paid_draw().with_recoverable_draw_shortage();
                        w.load(Process::new(1,parent_code.clone(),PARENT_CREDITS)).unwrap();
                        for (key,value) in [(7,source),(20,tag),(21,dst as i64),
                                            (22,lhs as i64),(23,rhs as i64)] {
                            w.intervene(Intervention::SetCell { key,value }).unwrap();
                        }
                        w.intervene(Intervention::SetComputePool { credits:SHARED_POOL }).unwrap();
                        let attempts=w.run(2500);
                        assert!(attempts>0 && attempts<=2500);
                        assert_eq!(w.total_compute_inventory(),Some(PARENT_CREDITS+SHARED_POOL-attempts as u64));
                        assert_eq!(w.events().iter().filter(|e|e.origin==Origin::External).count(),7);
                        assert_eq!(w.process(1).unwrap().code.nodes,parent_code,
                            "parent cannot self-modify its executable assembler to count as new child invention");
                        assert!(!w.events().iter().any(|e| e.origin==Origin::Internal(1) &&
                            matches!(e.kind,EventKind::CellWritten {key:10,..})));
                        let parent_copies:Vec<_>=w.events().iter().filter(|e|e.origin==Origin::Internal(1)
                            && matches!(e.kind,EventKind::GraphNodeCopied { source_process:1,.. })).collect();
                        assert_eq!(parent_copies.len(),18,"parent native graph must actually contain all18 Halt children");
                        let replacements:Vec<_>=w.events().iter().filter(|e|e.origin==Origin::Internal(1)
                            && matches!(e.kind,EventKind::GraphReplaced {owner:1,..})).collect();
                        assert_eq!(replacements.len(),17,"all original typed child descriptor replacements must complete");
                        for (i,event) in replacements.iter().enumerate() {
                            let desc=if i==4 {
                                [tag,dst as i64,lhs as i64,rhs as i64,5]
                            } else {
                                designed_descriptor(i)
                            };
                            assert!(matches!(&event.kind,EventKind::GraphReplaced {
                                node,descriptor,descriptor_base:DESCRIPTOR_BASE,..
                            } if *node==i && *descriptor==desc),
                                "actual typed child-code site must equal actor-written original descriptor");
                        }
                        let parent_birth=w.events().iter().find(|e|e.origin==Origin::Internal(1)
                            && matches!(e.kind,EventKind::GraphLaunched {
                                parent_process:1,child_process:2,
                                transferred_credits:FIRST_CHILD_CREDITS,moved_nodes:CHILD_CODE_NODES,..
                            })).expect("all typed Add/Sub operands are valid native descriptor encodings");
                        assert!(parent_birth.tick>replacements.last().unwrap().tick);
                        let child=&w.process(2).unwrap().code.nodes;
                        assert_eq!(child.len(),18);
                        let expect_math=if tag==1 {
                            Op::Add {dst,lhs,rhs,next:5}
                        } else {
                            Op::Sub {dst,lhs,rhs,next:5}
                        };
                        assert_eq!(child[4],expect_math);
                        for (offset,actual_word) in [tag,dst as i64,lhs as i64,rhs as i64].iter().enumerate() {
                            let key=20+offset as i64;
                            let found=w.events().iter().any(|e|e.origin==Origin::Internal(1)
                                && matches!(e.kind,EventKind::CellRead {key:k,value:v}
                                    if k==key && v==*actual_word));
                            assert!(found,"parent failed genuine numeric operand source read");
                        }
                        let child_output=actor_sensor_and_output(&w,2,source);
                        let grandchild=w.events().iter().find(|e|e.origin==Origin::Internal(2)
                            && matches!(e.kind,EventKind::GraphLaunched {
                                parent_process:2,child_process:3,..
                            }));
                        let mut full_native_grandchild=false;
                        let mut grandchild_output=None;
                        if let Some(launch)=grandchild {
                            if let EventKind::GraphLaunched {transferred_credits,moved_nodes,..}=launch.kind {
                                full_native_grandchild=transferred_credits==GRANDCHILD_CREDITS
                                    && moved_nodes==CHILD_CODE_NODES
                                    && w.process(3).map(|p| &p.code.nodes)==Some(child);
                            }
                            let child_copies=w.events().iter().filter(|e|e.origin==Origin::Internal(2)
                                && matches!(e.kind,EventKind::GraphNodeCopied { source_process:2,.. })).count();
                            assert_eq!(child_copies,CHILD_CODE_NODES);
                            assert!(w.events().iter().any(|e|e.origin==Origin::Internal(2)
                                && e.pc==15 && matches!(e.kind,EventKind::Executed)),
                                "real native Draw before grandchild birth required");
                            grandchild_output=actor_sensor_and_output(&w,3,source);
                        }
                        let result=Outcome121 { child_output, grandchild_output, full_native_grandchild };
                        assert!(rows.insert((tag,dst,lhs,rhs,source),result).is_none());
                        worlds+=1;
                        println!("Study121 ORIGINAL tag={tag} dst={dst} lhs={lhs} rhs={rhs} source={source} child2_output={child_output:?} true_grandchild={} full_genome={} grandchild3_output={grandchild_output:?} charged_attempts={attempts} [ALL_OPERAND_COMBINATIONS_PRECOMMITTED_NO_CHERRYPICK]",grandchild.is_some(),full_native_grandchild);
                    }
                }
            }
        }
    }
    assert_eq!(worlds,256);
    let mut source_responsive=0;
    let mut fully_reproducing=0;
    let mut heritable_function=0;
    let mut derived_nonidentity=0;
    for tag in [1_i64,2_i64] {
        for dst in 0_usize..4 {
            for lhs in 0_usize..4 {
                for rhs in 0_usize..4 {
                    let low=rows[&(tag,dst,lhs,rhs,3)];
                    let high=rows[&(tag,dst,lhs,rhs,8)];
                    if low.full_native_grandchild && high.full_native_grandchild {
                        fully_reproducing+=1;
                    }
                    let responsive=low.child_output.is_some()
                        && high.child_output.is_some()
                        && low.child_output != high.child_output;
                    if responsive {
                        source_responsive+=1;
                    }
                    let inherited=responsive && low.full_native_grandchild
                        && high.full_native_grandchild
                        && low.grandchild_output==low.child_output
                        && high.grandchild_output==high.child_output;
                    if inherited {
                        heritable_function+=1;
                        if (low.child_output,high.child_output)!=(Some(3),Some(8)) {
                            derived_nonidentity+=1;
                        }
                    }
                    println!("Study121 ORIGINAL genotype tag={tag} dst={dst} lhs={lhs} rhs={rhs} source3_out={:?} source8_out={:?} source_responsive={} exact_reproducing_both={} grandchild_function_inherited={} nonidentity_derived={}",low.child_output,high.child_output,responsive,low.full_native_grandchild&&high.full_native_grandchild,inherited,inherited&&(low.child_output,high.child_output)!=(Some(3),Some(8)));
                }
            }
        }
    }
    assert_eq!(rows.len(),256);
    assert_eq!(rows[&(1,0,0,2,3)].child_output,Some(4));
    assert_eq!(rows[&(1,0,0,2,8)].child_output,Some(9));
    assert_eq!(rows[&(2,0,0,2,3)].child_output,Some(2));
    assert_eq!(rows[&(2,0,0,2,8)].child_output,Some(7));
    for tag in [1_i64,2_i64] {
        for source in [3_i64,8_i64] {
            assert!(rows[&(tag,0,0,2,source)].full_native_grandchild);
        }
    }
    println!("Study121 ORIGINAL COMPLETE: fixed_operand_genotypes=128 original_source_worlds=256 child_assembled=128 source_responsive_genotypes={source_responsive} full_native_grandchild_genotypes={fully_reproducing} source_function_inherited_genotypes={heritable_function} derived_nonidentity_function_genotypes={derived_nonidentity} [HUMAN_ASSEMBLER_NOT_ORIGIN]");
}
