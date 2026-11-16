//! CL-1B B28: native CIL copies its complete reproducer, computes a
//! signed genome variation, edits BOTH the child's resource-Draw literal and
//! the child's next-generation mutation base, and transfers child credits.
//! Hand-authored mutation rules & flux, no endogenous optimization or life.
use cl0_world::{EventKind, Intervention, Op, Origin, Process, World};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Arm { PlusOne, Identical, MinusOne, NoInflux, CopyDisabled }
impl Arm {
    fn delta(self) -> i64 {
        match self { Arm::PlusOne | Arm::NoInflux | Arm::CopyDisabled => 1,
            Arm::Identical => 0, Arm::MinusOne => -1 }
    }
}

fn founder(initial: i64,delta: i64) -> Vec<Op> {
    vec![
        Op::Const { dst: 3, value: initial, next: 1 }, //0 request genotype
        Op::DrawCompute { amount: 3, next: 2 }, //1
        Op::GraphNew { dst: 0, next: 3 }, //2
        Op::Const { dst: 1, value: 0, next: 4 }, //3
        Op::GraphCopyNode { graph: 0, source: 1, dst: 2, next: 5 }, //4
        Op::Const { dst: 3, value: 1, next: 6 }, //5
        Op::Add { dst: 1, lhs: 1, rhs: 3, next: 7 }, //6
        Op::Const { dst: 3, value: 19, next: 8 }, //7
        Op::Sub { dst: 3, lhs: 1, rhs: 3, next: 9 }, //8
        Op::Branch { test: 3, nonzero: 4, zero: 10 }, //9
        Op::Const { dst: 1, value: 0, next: 11 }, //10
        Op::Const { dst: 2, value: initial, next: 12 }, //11 mutation base IN GENOME
        Op::Const { dst: 3, value: delta, next: 13 }, //12 signed mutation size
        Op::Add { dst: 2, lhs: 2, rhs: 3, next: 14 }, //13 native computed candidate
        Op::GraphSetConst { graph: 0, node: 1, value: 2, next: 15 }, //14 copy PC0
        Op::Const { dst: 1, value: 11, next: 16 }, //15
        Op::GraphSetConst { graph: 0, node: 1, value: 2, next: 17 }, //16 copy PC11
        Op::Const { dst: 1, value: 2, next: 18 }, //17 child startup credits
        Op::GraphLaunch { graph: 0, credits: 1, dst: 2, next: 0 }, //18
    ]
}

#[derive(Debug)]
struct ResultTrace {
    arm: Arm,
    supply: Vec<u64>,
    births: Vec<(u64,u64)>,
    genome_requests: Vec<i64>,
    charged_attempts: usize,
    final_pool: u64,
    stopped: Vec<u64>,
    trapped_count: usize,
}

fn drain(w: &mut World) -> usize {
    let mut n=0_usize;
    loop {
        assert!(n<180,"finite pool packet cannot allow indefinite native instructions");
        let energy=w.total_compute_inventory().unwrap();
        let events=w.events().len();
        if !w.step() {
            assert_eq!(w.total_compute_inventory(),Some(energy));
            assert_eq!(w.events().len(),events);
            return n;
        }
        n+=1;
        assert_eq!(w.total_compute_inventory(),Some(energy-1),
            "native graph edits/copy must conserve total compute resources");
        assert_eq!(w.events().len(),events+1);
        assert!(matches!(w.events().last().unwrap().origin,
            Origin::Internal(1) | Origin::Internal(2) |
            Origin::Internal(3) | Origin::Internal(4)
        ));
    }
}

fn lineage(w:&World)->Vec<(u64,u64)> {
    w.events().iter().filter_map(|e|match (&e.origin,&e.kind) {
        (Origin::Internal(owner),EventKind::GraphLaunched{
            parent_process,child_process,transferred_credits:2,moved_nodes:19,..
        }) if owner==parent_process=>Some((*parent_process,*child_process)),
        _=>None,
    }).collect()
}
fn request(code:&[Op],node:usize)->i64 {
    match &code[node] { Op::Const{value,..}=>*value,
        _=>panic!("expected inherited executable request literal")
    }
}
fn execute(arm:Arm, supply:&[u64]) -> ResultTrace {
    let delta=arm.delta();
    let source=founder(127,delta);
    assert_eq!(source.len(),19);
    let mut w=World::with_strict_capacity(16,64,128)
        .with_pool_paid_draw()
        .with_recoverable_draw_shortage();
    if arm==Arm::CopyDisabled {w=w.with_native_code_edits_disabled();}
    w.load(Process::new(1,source,2)).unwrap();
    assert!(w.process(2).is_none());
    assert_eq!(drain(&mut w),2);
    assert_eq!(w.process(1).unwrap().pc,1);
    assert_eq!(w.total_compute_inventory(),Some(0));
    let mut attempted=2_usize;

    for (i,&packet) in supply.iter().enumerate() {
        let energy=w.total_compute_inventory().unwrap();
        let old_pool=w.available_compute_pool();
        let events=w.events().len();
        w.intervene(Intervention::SetComputePool{credits:old_pool+packet}).unwrap();
        assert_eq!(w.total_compute_inventory(),Some(energy+packet));
        assert_eq!(w.events().len(),events+1);
        assert_eq!(w.events().last().unwrap().origin,Origin::External);
        let n=drain(&mut w);
        attempted+=n;
        if arm==Arm::PlusOne && supply==[128,129,130] {
            // Explicit post-hoc B28b correction to the *failed*
            // preregistered 128/129/130 charged-generation prediction.
            // Request128 left one local credit after a completed birth;
            // its parent can pay PC0, return to Draw, and reproduce AGAIN.
            assert_eq!(n,[128,129,129][i],
                "actual native resource use includes a residual credit and a branching ancestor");
            assert_eq!(lineage(&w).len(),i+1);
        }
    }

    let births=lineage(&w);
    let mut inherited_request=std::collections::BTreeMap::<u64,i64>::new();
    inherited_request.insert(1,127);
    for &(parent,child) in &births {
        let previous=*inherited_request.get(&parent).expect("every child has existing source parent");
        assert!(inherited_request.insert(child,previous+delta).is_none(),
            "child is born exactly once");
    }
    let mut requests=Vec::new();
    let mut stopped=Vec::new();
    for id in 1..=((births.len()+1) as u64) {
        let p=w.process(id).expect("all launched children remain in world");
        assert_eq!(p.code.nodes.len(),19);
        let expected_genome=founder(*inherited_request.get(&id)
            .expect("native parent-child graph must cover every observed process"),delta);
        assert_eq!(p.code.nodes,expected_genome,
            "the ENTIRE descendant genome must inherit BOTH computed request and mutation base");
        assert_eq!(request(&p.code.nodes,0),request(&p.code.nodes,11));
        requests.push(request(&p.code.nodes,0));
        if p.stopped {stopped.push(id);}
    }
    assert_eq!(w.process((births.len()+2) as u64).is_none(),true);
    assert_eq!(w.events().iter().filter(|e|e.origin==Origin::External).count(),1+supply.len());
    assert_eq!(w.total_compute_inventory(),Some(
        2+supply.iter().sum::<u64>()-attempted as u64
    ));
    let copied=w.events().iter().filter(|e|matches!(
        e.kind,EventKind::GraphNodeCopied{..}
    )).count();
    let edits=w.events().iter().filter(|e|matches!(
        e.kind,EventKind::GraphNodeEdited{edit:ref field,..} if field=="const"
    )).count();
    let traps=w.events().iter().filter(|e|matches!(
        e.kind,EventKind::Trapped(_)
    )).count();
    assert_eq!(copied%19,0,"all GraphCopyNode instructions must follow full 19-op copies");
    assert_eq!(edits%2,0,"each successful mutated graph records both genome literal edits");
    if arm==Arm::CopyDisabled {
        assert_eq!((births.len(),copied,edits),(0,0,0));
        assert_eq!(traps,1);
    }else if arm==Arm::NoInflux {
        assert_eq!((births.len(),copied,edits,traps),(0,0,0,0));
    }else{
        assert_eq!(copied,19*(births.len()+traps),
            "even an unlaunchable child must pay to construct its full graph");
        assert_eq!(edits,2*(births.len()+traps),
            "both mutations must precede any attempted native launch");
    }
    let mut successful_births_by_parent=std::collections::BTreeMap::<u64,usize>::new();
    for &(parent,child) in &births {
        *successful_births_by_parent.entry(parent).or_insert(0)+=1;
        let child_p=w.process(child).unwrap();
        let expected=*inherited_request.get(&child).unwrap();
        assert_eq!(request(&child_p.code.nodes,0),expected);
        assert_eq!(request(&child_p.code.nodes,11),expected);
    }
    for (parent,birth_count) in successful_births_by_parent {
        let own_copies=w.events().iter().filter_map(|e|match (&e.origin,&e.kind) {
            (Origin::Internal(owner),EventKind::GraphNodeCopied{
                source_process,source_node,..
            }) if *owner==parent && *source_process==parent =>Some(*source_node),
            _=>None,
        }).collect::<Vec<_>>();
        assert_eq!(own_copies,(0..birth_count).flat_map(|_|0..19)
            .collect::<Vec<usize>>(),
            "each successive birth must copy all 19 nodes even if parent reproduces twice");
        let literal_changes=w.events().iter().filter_map(|e|match (&e.origin,&e.kind) {
            (Origin::Internal(owner),EventKind::GraphNodeEdited{
                node,edit,..
            }) if *owner==parent && edit=="const" =>Some(*node),
            _=>None,
        }).collect::<Vec<_>>();
        assert_eq!(literal_changes,(0..birth_count).flat_map(|_|[0,11])
            .collect::<Vec<usize>>(),
            "each birth must modify both inherited genotype literals in its own cloned graph");
    }
    assert!(w.occupied_units()<=128);
    let result=ResultTrace{
        arm,supply:supply.to_vec(),births,genome_requests:requests,
        charged_attempts:attempted,final_pool:w.available_compute_pool(),
        stopped,trapped_count:traps
    };
    eprintln!("B28_NATIVE {:?}",result);
    result
}

#[test]
fn b28_two_heritable_source_literals_accumulate_across_native_cil_generations() {
    let increasing=[128,129,130];
    let flat=[128,128,128];
    let decreasing=[130,129,128];
    let mut cases=Vec::new();
    for arm in [Arm::PlusOne,Arm::Identical,Arm::MinusOne] {
        for schedule in [&increasing[..],&flat[..],&decreasing[..]] {
            cases.push(execute(arm,schedule));
        }
    }
    cases.push(execute(Arm::NoInflux,&[]));
    cases.push(execute(Arm::CopyDisabled,&increasing));
    assert_eq!(cases.len(),11);
    let plus_increasing=&cases[0];
    assert_eq!(plus_increasing.arm,Arm::PlusOne);
    assert_eq!(plus_increasing.supply,increasing);
    assert_eq!(plus_increasing.births,vec![(1,2),(2,3),(2,4)],
        "one extra compute credit allows the same parent to create a second sibling");
    assert_eq!(plus_increasing.genome_requests,vec![127,128,129,129],
        "the fourth process is a sibling from parent2, not a fourth-depth mutation");
    assert_eq!(plus_increasing.trapped_count,0);
    assert_eq!(plus_increasing.final_pool,1,
        "plus-one resource mutation leaves one neutral pool credit unused");
    assert_eq!(plus_increasing.charged_attempts,2+128+129+129);
    let plus_flat=&cases[1];
    assert_eq!(plus_flat.genome_requests,vec![127,128,129],
        "with equal flat packets, request128 needs at least129 pool credits to reproduce");
    let exact=&cases[4]; // identical and flat
    assert_eq!(exact.arm,Arm::Identical);
    assert_eq!(exact.births,vec![(1,2),(2,3),(3,4)]);
    assert_eq!(exact.genome_requests,vec![127,127,127,127]);
    let minus_increasing=&cases[6];
    assert_eq!(minus_increasing.arm,Arm::MinusOne);
    assert_eq!(minus_increasing.births,vec![(1,2)]);
    assert_eq!(minus_increasing.genome_requests,vec![127,126]);
    assert!(minus_increasing.stopped.contains(&2),
        "child request126 cannot pay125 instructions and transfer2 to launch grandchild");
    assert_eq!(minus_increasing.trapped_count,1);
    assert_eq!(cases[9].births.len(),0);
    assert_eq!(cases[10].births.len(),0);
}
