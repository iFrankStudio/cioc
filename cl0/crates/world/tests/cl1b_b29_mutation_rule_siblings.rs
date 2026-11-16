//! B29: full native 23-node, self-copying CIL lineage with paid
//! in-parent mutation-rule inversion. The policy and environmental schedules
//! are HUMAN AUTHORED; no discovered algorithm, no life claim.
use cl0_world::{EventKind, Intervention, Op, Origin, Process, World};

fn founder(initial: i64,delta: i64, flip: bool) -> Vec<Op> {
    vec![
        Op::Const { dst: 3, value: initial, next: 1 }, //0 request genotype
        Op::DrawCompute { amount: 3, next: 2 }, //1
        Op::GraphNew { dst: 0, next: 3 }, //2
        Op::Const { dst: 1, value: 0, next: 4 }, //3
        Op::GraphCopyNode { graph: 0, source: 1, dst: 2, next: 5 }, //4
        Op::Const { dst: 3, value: 1, next: 6 }, //5
        Op::Add { dst: 1, lhs: 1, rhs: 3, next: 7 }, //6
        Op::Const { dst: 3, value: 23, next: 8 }, //7
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
        Op::GraphLaunch { graph: 0, credits: 1, dst: 2, next: 19 }, //18
        Op::Const { dst: 1, value: 0, next: 20 }, //19
        Op::Sub { dst: 3, lhs: 1, rhs: 3, next: 21 }, //20: invert signed delta
        Op::Const { dst: 1, value: 12, next: 22 }, //21
        if flip {
            Op::PatchConst { node: 1, value: 3, next: 0 } //22: native self-mutation
        } else {
            Op::Const { dst: 1, value: 12, next: 0 } //22: same-cost no-mutation control
        },
    ]
}


#[derive(Clone,Copy,Debug,PartialEq,Eq)]
enum Arm { Flip, NoFlip, NoFlux, CopyDisabled }

fn request(p:&Process)->i64 {
    match &p.code.nodes[0] { Op::Const{value,..}=>*value,
        _=>panic!("first genotype node must be a Const") }
}
fn delta(p:&Process)->i64 {
    match &p.code.nodes[12] { Op::Const{value,..}=>*value,
        _=>panic!("mutation-step genotype node must be a Const") }
}
fn births(w:&World)->Vec<(u64,u64)> {
    w.events().iter().filter_map(|e|match (&e.origin,&e.kind) {
        (Origin::Internal(owner),EventKind::GraphLaunched {
            parent_process,child_process,transferred_credits:2,moved_nodes:23,..
        }) if owner==parent_process=>Some((*parent_process,*child_process)),
        _=>None,
    }).collect()
}

#[derive(Debug)]
struct Snapshot {
    arm:Arm,
    births:Vec<(u64,u64)>,
    source_edits:Vec<(u64,i64,i64)>,
    genotypes:Vec<(u64,i64,i64)>,
    total_attempts:usize,
    pool:u64,
    traps:usize,
    copies:usize,
    graph_literal_edits:usize,
    external_actions:usize,
    stopped:Vec<u64>,
    energy:u64,
    packet_steps:Vec<usize>,
}

fn drain(w:&mut World,changes:&mut Vec<(u64,i64,i64)>)->usize {
    let mut n=0;
    loop {
        assert!(n<220,"bounded external compute prevents an unbounded source-edit trial");
        let old_energy=w.total_compute_inventory().unwrap();
        let old_events=w.events().len();
        let before_delta=(1..=8_u64).filter_map(|id|
            w.process(id).map(|p|(id,p.pc,delta(p)))).collect::<Vec<_>>();
        if !w.step() {
            assert_eq!(w.total_compute_inventory(),Some(old_energy));
            assert_eq!(w.events().len(),old_events);
            return n;
        }
        n+=1;
        assert_eq!(w.total_compute_inventory(),Some(old_energy-1));
        assert_eq!(w.events().len(),old_events+1);
        let latest=w.events().last().unwrap();
        let id=match latest.origin {Origin::Internal(id)=>id,
            _=>panic!("external action during native paid drain")};
        assert!(!matches!(latest.kind,EventKind::Halted));
        for (old_id,pc,old_d) in before_delta {
            let newer=delta(w.process(old_id).unwrap());
            if newer!=old_d {
                assert_eq!(old_id,id,"a process may not edit a different process' own source delta");
                assert_eq!(pc,22,"native self-edit of signed delta must execute at PC22");
                assert_eq!(newer,-old_d,"true source edit must negate exactly the old delta");
                changes.push((id,old_d,newer));
            }
        }
    }
}
fn trial(arm:Arm,packets:&[u64])->Snapshot {
    let flip=arm!=Arm::NoFlip;
    let source=founder(156,1,flip);
    assert_eq!(source.len(),23);
    let mut w=World::with_strict_capacity(16,64,128)
        .with_pool_paid_draw()
        .with_recoverable_draw_shortage()
        .with_native_executable_edit_trace();
    if arm==Arm::CopyDisabled {w=w.with_native_code_edits_disabled();}
    w.load(Process::new(1,source,2)).unwrap();
    assert!(w.process(2).is_none());
    let mut edits=Vec::new();
    let initial=drain(&mut w,&mut edits);
    assert_eq!(initial,2);
    assert!(edits.is_empty());
    assert_eq!(w.process(1).unwrap().pc,1);
    assert_eq!(w.total_compute_inventory(),Some(0));
    let mut attempts=initial;
    let mut packet_steps=Vec::new();
    for &packet in packets {
        let total=w.total_compute_inventory().unwrap();
        let pool=w.available_compute_pool();
        w.intervene(Intervention::SetComputePool{credits:pool+packet}).unwrap();
        assert_eq!(w.total_compute_inventory(),Some(total+packet));
        assert_eq!(w.events().last().unwrap().origin,Origin::External);
        let count=drain(&mut w,&mut edits);
        packet_steps.push(count);
        attempts+=count;
    }
    let line=births(&w);
    let mut genotypes=Vec::new();
    let mut stopped=Vec::new();
    for id in 1..=((line.len()+1) as u64) {
        let p=w.process(id).expect("native child must exist for every traced birth");
        assert_eq!(p.code.nodes.len(),23);
        assert!(!p.code.nodes.iter().any(|o|matches!(o,Op::GraphCopyNodeLocal{..})),
            "heredity must copy parent's full native executable, not a dormant helper template");
        genotypes.push((id,request(p),delta(p)));
        if p.stopped {stopped.push(id);}
    }
    assert!(w.process((line.len()+2) as u64).is_none());
    let copies=w.events().iter().filter(|e|matches!(e.kind,EventKind::GraphNodeCopied{..})).count();
    let graph_literal_edits=w.events().iter().filter(|e|matches!(
        e.kind,EventKind::GraphNodeEdited{edit:ref kind,..} if kind=="const"
    )).count();
    let traps=w.events().iter().filter(|e|matches!(e.kind,EventKind::Trapped(_))).count();
    assert_eq!(w.events().iter().filter(|e|e.origin==Origin::External).count(),1+packets.len());
    assert_eq!(w.total_compute_inventory(),Some(2+packets.iter().sum::<u64>()-attempts as u64));
    assert!(w.occupied_units()<=128);
    assert!(w.graph_count()<=1);
    if arm!=Arm::CopyDisabled {
        assert_eq!(copies,line.len()*23,"every birth must pay to copy complete 23-op parent source");
        assert_eq!(graph_literal_edits,line.len()*2,
            "every child must receive exactly two native resource-genotype graph edits");
        assert_eq!(traps,0);
    }
    let result=Snapshot {
        arm,births:line,source_edits:edits,genotypes,
        total_attempts:attempts,pool:w.available_compute_pool(),traps,copies,
        graph_literal_edits,external_actions:1+packets.len(),stopped,
        energy:w.total_compute_inventory().unwrap(),packet_steps,
    };
    eprintln!("B29_NATIVE {:?}",result);
    result
}

#[test]
fn b29_native_same_parent_produces_distinct_full_genome_siblings_after_paid_mutation_rule_inversion() {
    let packets=&[157_u64,157,156][..];
    let flip=trial(Arm::Flip,packets);
    let no_flip=trial(Arm::NoFlip,packets);
    let no_flux=trial(Arm::NoFlux,&[]);
    let suppressed=trial(Arm::CopyDisabled,packets);
    assert_eq!(flip.births,vec![(1,2),(1,3),(3,4)]);
    assert_eq!(flip.source_edits,vec![(1,1,-1),(1,-1,1),(3,-1,1)]);
    assert_eq!(flip.genotypes,vec![(1,156,1),(2,157,1),(3,155,1),(4,154,-1)]);
    assert_eq!(flip.packet_steps,vec![157,157,156]);
    assert_eq!(flip.pool,0);
    assert_eq!(no_flip.births,vec![(1,2),(1,3)],
        "without native mutation-rule flip, both siblings carry request157");
    assert!(no_flip.source_edits.is_empty());
    assert_eq!(no_flip.genotypes,vec![(1,156,1),(2,157,1),(3,157,1)]);
    assert_eq!(no_flip.packet_steps,vec![157,157,0]);
    assert_eq!(no_flip.pool,156);
    assert!(no_flux.births.is_empty());
    assert!(no_flux.source_edits.is_empty());
    assert_eq!(no_flux.genotypes,vec![(1,156,1)]);
    assert_eq!(no_flux.total_attempts,2);
    assert!(suppressed.births.is_empty());
    assert!(suppressed.source_edits.is_empty());
    assert_eq!(suppressed.traps,1);
    assert_eq!(suppressed.copies,0);
    assert_eq!(suppressed.genotypes,vec![(1,156,1)]);
}
