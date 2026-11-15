//! Study122: HUMAN-authored fixed child compiler; CHILD itself writes numeric
//! genotype descriptors and GraphReplace changes the grandchild's executable
//! instruction. Full 0..31 tag census plus invalid99, two source checks.
use cl0_world::{EventKind, Intervention, Op, Origin, Process, World};

const DESCRIPTOR_BASE: i64 = 100;
const PARENT_CREDITS: u64 = 1200;
const PUBLIC_POOL: u64 = 1000;
const INITIAL_CHILD_CREDITS: u64 = 400;
const NATIVE_GRANDCHILD_CREDITS: u64 = 200;
const GENOME_NODES: usize = 37;

fn append(code: &mut Vec<Op>, make: impl FnOnce(usize)->Op) {
    let next=code.len()+1;
    code.push(make(next));
}
fn parent_const(code: &mut Vec<Op>, dst:usize, value:i64) {
    append(code, |next|Op::Const{dst,value,next});
}
fn descriptor_for(op: &Op)->[i64;5] {
    let v=|x:usize|i64::try_from(x).expect("small static CIL operand");
    match op {
        Op::Const{dst,value,next}=>[0,v(*dst),*value,v(*next),0],
        Op::Add{dst,lhs,rhs,next}=>[1,v(*dst),v(*lhs),v(*rhs),v(*next)],
        Op::Sub{dst,lhs,rhs,next}=>[2,v(*dst),v(*lhs),v(*rhs),v(*next)],
        Op::Read{dst,address,next}=>[3,v(*dst),v(*address),v(*next),0],
        Op::Write{address,value,next}=>[4,v(*address),v(*value),v(*next),0],
        Op::Branch{test,nonzero,zero}=>[5,v(*test),v(*nonzero),v(*zero),0],
        Op::GraphNew{dst,next}=>[11,v(*dst),v(*next),0,0],
        Op::GraphCopyNode{graph,source,dst,next}=>[
            12,v(*graph),v(*source),v(*dst),v(*next)],
        Op::GraphReplace{graph,node,address,next}=>[
            27,v(*graph),v(*node),v(*address),v(*next)],
        Op::DrawCompute{amount,next}=>[23,v(*amount),v(*next),0,0],
        Op::GraphLaunch{graph,credits,dst,next}=>[
            17,v(*graph),v(*credits),v(*dst),v(*next)],
        _=>panic!("not one of frozen Study122 child instructions"),
    }
}

fn fixed_child_genome()->Vec<Op> {
    let code=vec![
        Op::Const{dst:2,value:7,next:1},
        Op::Const{dst:3,value:10,next:2},
        Op::Read{dst:0,address:2,next:3},
        Op::Const{dst:2,value:1,next:4},
        Op::Add{dst:0,lhs:0,rhs:2,next:5},
        Op::Write{address:3,value:0,next:6},
        Op::Const{dst:2,value:1,next:7},
        Op::Const{dst:1,value:0,next:8},
        Op::Const{dst:3,value:37,next:9},
        Op::GraphNew{dst:0,next:10},
        Op::GraphCopyNode{graph:0,source:1,dst:1,next:11},
        Op::Add{dst:1,lhs:1,rhs:2,next:12},
        Op::Sub{dst:3,lhs:3,rhs:2,next:13},
        Op::Branch{test:3,nonzero:10,zero:14},
        Op::Const{dst:2,value:20,next:15},
        Op::Read{dst:3,address:2,next:16},
        Op::Const{dst:2,value:100,next:17},
        Op::Write{address:2,value:3,next:18},
        Op::Const{dst:2,value:101,next:19},
        Op::Const{dst:3,value:0,next:20},
        Op::Write{address:2,value:3,next:21},
        Op::Const{dst:2,value:102,next:22},
        Op::Const{dst:3,value:0,next:23},
        Op::Write{address:2,value:3,next:24},
        Op::Const{dst:2,value:103,next:25},
        Op::Const{dst:3,value:2,next:26},
        Op::Write{address:2,value:3,next:27},
        Op::Const{dst:2,value:104,next:28},
        Op::Const{dst:3,value:5,next:29},
        Op::Write{address:2,value:3,next:30},
        Op::Const{dst:1,value:4,next:31},
        Op::Const{dst:2,value:100,next:32},
        Op::GraphReplace{graph:0,node:1,address:2,next:33},
        Op::Const{dst:2,value:200,next:34},
        Op::DrawCompute{amount:2,next:35},
        Op::GraphLaunch{graph:0,credits:2,dst:3,next:36},
        Op::Halt,
    ];
    assert_eq!(code.len(),GENOME_NODES);
    code
}
fn parent_builder()->Vec<Op> {
    let child=fixed_child_genome();
    let mut code=Vec::new();
    append(&mut code,|next|Op::GraphNew{dst:0,next});
    let halt_ptr=code.len();
    parent_const(&mut code,1,0);
    for _ in 0..GENOME_NODES {
        append(&mut code,|next|Op::GraphCopyNode{
            graph:0,source:1,dst:3,next,
        });
    }
    for (i,op) in child.iter().enumerate().take(GENOME_NODES-1) {
        let descriptor=descriptor_for(op);
        for (offset,word) in descriptor.into_iter().enumerate() {
            parent_const(&mut code,2,DESCRIPTOR_BASE+offset as i64);
            parent_const(&mut code,3,word);
            append(&mut code,|next|Op::Write{address:2,value:3,next});
        }
        parent_const(&mut code,1,i as i64);
        parent_const(&mut code,2,DESCRIPTOR_BASE);
        append(&mut code,|next|Op::GraphReplace{graph:0,node:1,address:2,next});
    }
    parent_const(&mut code,2,INITIAL_CHILD_CREDITS as i64);
    append(&mut code,|next|Op::GraphLaunch{graph:0,credits:2,dst:3,next});
    code.push(Op::Halt);
    let terminal=(code.len()-1) as i64;
    code[halt_ptr]=Op::Const{dst:1,value:terminal,next:halt_ptr+1};
    assert!(code.len()<1024);
    assert!(code.iter().all(|op|!matches!(op,Op::Add{..}|Op::Sub{..})));
    code
}

fn actor_output(world:&World,pid:u64,source:i64)->Option<i64> {
    let read=world.events().iter().find(|e|e.origin==Origin::Internal(pid)&&e.pc==2
        &&matches!(e.kind,EventKind::CellRead{key:7,value} if value==source))?;
    let write=world.events().iter().find(|e|e.origin==Origin::Internal(pid)
        &&e.pc==5&&e.tick>read.tick
        &&matches!(e.kind,EventKind::CellWritten{key:10,..}))?;
    match write.kind {EventKind::CellWritten{value,..}=>Some(value),_=>None}
}

#[test]
fn full_32_tag_grandchild_mutation_by_assembled_child_in_native_cil() {
    let original_builder=parent_builder();
    let original_child=fixed_child_genome();
    let mut seen=std::collections::BTreeMap::<(i64,i64),(bool,Option<i64>,bool)>::new();
    let mut attempted_worlds=0_usize;
    for tag in (0_i64..32).chain(std::iter::once(99)) {
        for source in [3_i64,8_i64] {
            let mut w=World::with_strict_capacity(16,1024,10000)
                .with_pool_paid_draw().with_recoverable_draw_shortage();
            w.load(Process::new(1,original_builder.clone(),PARENT_CREDITS)).unwrap();
            w.intervene(Intervention::SetCell{key:7,value:source}).unwrap();
            w.intervene(Intervention::SetCell{key:20,value:tag}).unwrap();
            w.intervene(Intervention::SetComputePool{credits:PUBLIC_POOL}).unwrap();
            let attempts=w.run(2500);
            assert!(attempts>0&&attempts<=2500);
            assert_eq!(w.total_compute_inventory(),Some(PARENT_CREDITS+PUBLIC_POOL-attempts as u64));
            assert_eq!(w.events().iter().filter(|e|e.origin==Origin::External).count(),4);
            assert_eq!(w.process(1).unwrap().code.nodes,original_builder,
                "host or actor may not secretly splice the original parent executable");
            let native_parent_copies=w.events().iter().filter(|e|e.origin==Origin::Internal(1)
                &&matches!(e.kind,EventKind::GraphNodeCopied{source_process:1,..})).count();
            let native_parent_replace=w.events().iter().filter(|e|e.origin==Origin::Internal(1)
                &&matches!(e.kind,EventKind::GraphReplaced{owner:1,..})).count();
            assert_eq!((native_parent_copies,native_parent_replace),(37,36));
            let parent_launch=w.events().iter().find(|e|e.origin==Origin::Internal(1)
                &&matches!(e.kind,EventKind::GraphLaunched{
                    parent_process:1,child_process:2,
                    transferred_credits:INITIAL_CHILD_CREDITS,moved_nodes:GENOME_NODES,..
                })).expect("parent must truly natively assemble and launch child2");
            let child=&w.process(2).unwrap().code.nodes;
            assert_eq!(*child,original_child,"child2 source program must be byte identical for all33 tags");
            assert_eq!(actor_output(&w,2,source),Some(source+1),
                "child2 must demonstrate source+1 function before its own child mutation");

            let child_copies:Vec<_>=w.events().iter().filter(|e|e.origin==Origin::Internal(2)
                &&matches!(e.kind,EventKind::GraphNodeCopied{source_process:2,..})).collect();
            assert_eq!(child_copies.len(),GENOME_NODES,"native child2 must copy entire own code");
            for (index,event) in child_copies.iter().enumerate() {
                assert!(matches!(event.kind,EventKind::GraphNodeCopied{new_node,..} if new_node==index));
            }
            let last_copy_tick=child_copies.last().unwrap().tick;
            let mut native_descriptor_writes=std::collections::BTreeMap::new();
            for e in w.events() {
                if e.origin==Origin::Internal(2) {
                    if let EventKind::CellWritten{key,value}=e.kind {
                        if (DESCRIPTOR_BASE..DESCRIPTOR_BASE+5).contains(&key) {
                            assert!(e.tick>last_copy_tick,"child must first copy own complete code");
                            native_descriptor_writes.insert(key,value);
                        }
                    }
                }
            }
            assert_eq!(native_descriptor_writes.len(),5);
            assert_eq!(native_descriptor_writes[&100],tag);
            assert_eq!(native_descriptor_writes[&101],0);
            assert_eq!(native_descriptor_writes[&102],0);
            assert_eq!(native_descriptor_writes[&103],2);
            assert_eq!(native_descriptor_writes[&104],5);
            assert!(w.events().iter().any(|e|e.origin==Origin::Internal(2)
                &&e.pc==15&&matches!(e.kind,EventKind::CellRead{key:20,value} if value==tag)));
            let native_replace=w.events().iter().find(|e|e.origin==Origin::Internal(2)
                &&matches!(e.kind,EventKind::GraphReplaced{
                    owner:2,node:4,descriptor_base:DESCRIPTOR_BASE,..
                }));
            let grandchild_launch=w.events().iter().find(|e|e.origin==Origin::Internal(2)
                &&matches!(e.kind,EventKind::GraphLaunched {
                    parent_process:2,child_process:3,
                    transferred_credits:NATIVE_GRANDCHILD_CREDITS,moved_nodes:GENOME_NODES,..
                }));
            let supported=Op::decode_instruction([tag,0,0,2,5]).is_ok();
            if supported {
                let mutation=native_replace.expect("all supported opcodes should be native-GraphReplaced");
                assert_eq!(mutation.pc,32);
                assert!(mutation.tick>last_copy_tick);
                assert!(matches!(mutation.kind,EventKind::GraphReplaced{
                    descriptor:[t,0,0,2,5],..
                } if t==tag));
                let launch=grandchild_launch.expect("valid typed descriptor must launch copied grandchild");
                assert!(launch.tick>mutation.tick);
                assert!(w.events().iter().any(|e|e.origin==Origin::Internal(2)
                    &&e.pc==34&&e.tick>mutation.tick&&e.tick<launch.tick
                    &&matches!(e.kind,EventKind::Executed)),
                    "real public compute must be drawn before CIL grandchild");
                assert!(w.process(3).is_some());
                // For supported types that can self-modify a resident program
                // after starting, the birth graph provenance is the stronger
                // evidence than the later resident length/content.
                if [1_i64,2_i64].contains(&tag) {
                    let mut expected=original_child.clone();
                    expected[4]=Op::decode_instruction([tag,0,0,2,5]).unwrap();
                    assert_eq!(w.process(3).unwrap().code.nodes,expected);
                }
            } else {
                assert!(native_replace.is_none(),"invalid descriptor must never become code");
                assert!(grandchild_launch.is_none(),"invalid descriptor must not produce grandchild");
                assert!(w.events().iter().any(|e|e.origin==Origin::Internal(2)
                    &&e.pc==32&&matches!(e.kind,EventKind::Trapped(_))));
            }
            if tag==1||tag==2 {
                let expect=if tag==1 {source+1} else {source-1};
                assert_eq!(actor_output(&w,3,source),Some(expect));
            }
            if tag==99 {
                assert!(grandchild_launch.is_none());
            }
            let output=if grandchild_launch.is_some() {actor_output(&w,3,source)} else {None};
            assert!(seen.insert((tag,source),(grandchild_launch.is_some(),output,native_replace.is_some())).is_none());
            attempted_worlds+=1;
            println!("Study122 ORIGINAL source={source} opcode_tag={tag} original_parent_to_child={} child2_unchanged_source_output={} child_native_mutation={} grandchild_launched={} grandchild_native_source_output={output:?} charged_native_attempts={attempts} [HUMAN_WRITTEN_MUTATOR_TAG_UNSELECTED]",parent_launch.pc,source+1,native_replace.is_some(),grandchild_launch.is_some());
        }
    }
    assert_eq!(attempted_worlds,66);
    let mut code_mutation_tags=0;
    let mut responsive_tags=0;
    let mut grandchild_birth_tags=0;
    for tag in 0_i64..32 {
        let a=seen[&(tag,3)];
        let b=seen[&(tag,8)];
        if a.2&&b.2 {code_mutation_tags+=1;}
        if a.0&&b.0 {grandchild_birth_tags+=1;}
        if let (Some(low),Some(high))=(a.1,b.1) {
            if low!=high {responsive_tags+=1;}
        }
        println!("Study122 ORIGINAL descriptor_tag={tag} child_native_mutation_both={} real_grandchildren_both={} source3_out={:?} source8_out={:?}",a.2&&b.2,a.0&&b.0,a.1,b.1);
    }
    assert!(!seen[&(99,3)].0&&!seen[&(99,8)].0);
    println!("Study122 ORIGINAL COMPLETE: tag_denominator=32 source_controls=2 fixed_native_worlds=66 real_child_generated_code_mutation_tags={code_mutation_tags} native_grandchild_tags={grandchild_birth_tags} source_responsive_grandchild_tags={responsive_tags} [SCRIPTED_BUILDER_NOT_NATURAL_CODE_ORIGIN]");
}
