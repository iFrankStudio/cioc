//! Study131: engineered positive-control topology, NOT natural evolution.
//! A 38-op native CIL self-copier uses ordinary shared cell12 to alternate
//! no-edit versus PatchNext BEFORE building its next whole38-op child graph.
//! Genuine same-founder original and mutant sibling births, not host code.
use cl0_world::{EventKind,Intervention,Op,Origin,Process,World};
use std::collections::{BTreeMap,BTreeSet};

const N:usize=38;
const TOTAL:u64=10_000;

fn program()->Vec<Op>{
    let p=vec![
        Op::Const{dst:2,value:7,next:1},     // 0
        Op::Const{dst:3,value:9,next:2},     // 1
        Op::Read{dst:0,address:2,next:3},    // 2 original source
        Op::Write{address:3,value:0,next:4},// 3 function
        Op::Const{dst:2,value:1,next:24},    // 4 redundant before delay
        Op::Const{dst:2,value:11,next:6},    // 5 mutation target cell
        Op::Read{dst:0,address:2,next:7},    // 6 R0 target24
        Op::Const{dst:1,value:23,next:28},   // 7 native site23, phase switch
        Op::PatchNext{node:1,target:0,next:9},// 8 native effective code edit
        Op::Const{dst:1,value:24,next:10},   // 9
        Op::Const{dst:1,value:24,next:11},   // 10 ordinary target held fixed
        Op::Write{address:2,value:1,next:12},// 11 ordinary key11
        Op::Const{dst:2,value:1,next:13},    // 12
        Op::Const{dst:1,value:0,next:14},    // 13
        Op::Const{dst:3,value:38,next:15},   // 14 all38 nodes
        Op::GraphNew{dst:0,next:16},         // 15
        Op::GraphCopyNode{graph:0,source:1,dst:1,next:17},//16
        Op::Add{dst:1,lhs:1,rhs:2,next:18},  // 17
        Op::Sub{dst:3,lhs:3,rhs:2,next:19},  // 18
        Op::Branch{test:3,nonzero:16,zero:20},//19
        Op::Const{dst:2,value:200,next:21},  // 20
        Op::DrawCompute{amount:2,next:22},   // 21
        Op::GraphLaunch{graph:0,credits:2,dst:3,next:23},//22
        Op::Const{dst:1,value:0,next:4},     // 23 mutation changes next4 ->24
        Op::Const{dst:1,value:4,next:25},    // 24 original delay
        Op::Const{dst:2,value:1,next:26},    // 25
        Op::Sub{dst:1,lhs:1,rhs:2,next:27},  // 26
        Op::Branch{test:1,nonzero:26,zero:5},// 27
        Op::Const{dst:2,value:12,next:29},   // 28 shared phase key12
        Op::Read{dst:3,address:2,next:30},   // 29 phase bit
        Op::Branch{test:3,nonzero:34,zero:31},//30
        Op::Const{dst:3,value:1,next:32},    // 31 first actor phase0->1 skip edit
        Op::Write{address:2,value:3,next:33},// 32 native write counter
        Op::Const{dst:2,value:11,next:37},   // 33 restore target cell key11
        Op::Const{dst:3,value:0,next:35},    // 34 phase1->0 perform edit
        Op::Write{address:2,value:3,next:36},// 35 native write counter
        Op::Const{dst:2,value:11,next:37},   // 36 restore key11
        Op::Branch{test:3,nonzero:9,zero:8}, // 37 skip vs edit
    ];
    assert_eq!(p.len(),N);
    p
}

#[derive(Debug,Default)]
struct Result131{
    same_founder_functional_siblings: bool,
    real_mutant_births: usize,
    real_ancestral_births: usize,
    paid: usize,
}
fn trial(source:i64,phase:i64,disabled:bool)->Result131{
    let original=program();
    let mut w=World::with_strict_capacity(256,64,2048)
        .with_pool_paid_draw().with_recoverable_draw_shortage()
        .with_native_executable_edit_trace();
    if disabled {w=w.with_native_patchnext_disabled();}
    w.load(Process::new(1,original.clone(),1000)).unwrap();
    w.load(Process::new(2,original.clone(),1000)).unwrap();
    for (key,value) in [(7,source),(11,24),(12,phase)] {
        w.intervene(Intervention::SetCell{key,value}).unwrap();
    }
    w.intervene(Intervention::SetComputePool{credits:8000}).unwrap();
    let paid=w.run(5000);
    assert!(paid>0&&paid<=5000);
    assert_eq!(w.total_compute_inventory(),Some(TOTAL-paid as u64));
    assert_eq!(w.events().iter().filter(|e|e.origin==Origin::External).count(),6);
    let mut alleles=BTreeMap::<u64,i64>::from([(1,4),(2,4)]);
    let mut graph_sources=BTreeMap::<u64,Vec<usize>>::new();
    let mut graph_allele=BTreeMap::<u64,i64>::new();
    let mut founders=BTreeMap::<u64,u64>::from([(1,1),(2,2)]);
    let mut ancestor_siblings=BTreeMap::<(u64,i64),Vec<u64>>::new();
    let mut src_read=BTreeMap::<u64,u64>::new();
    let mut canonical=BTreeSet::<u64>::new();
    let mut native_edits=0;
    let mut native_counter_writes=0;
    let mut births=0;
    let mut mutation_births=0;
    let mut ancestor_births=0;
    for e in w.events(){
        if e.origin==Origin::External {continue;}
        let actor=e.process;
        assert_eq!(e.origin,Origin::Internal(actor));
        assert!(alleles.contains_key(&actor));
        if e.pc==2{
            if let EventKind::CellRead{key:7,value}=e.kind{
                if value==source {src_read.insert(actor,e.tick);}
            }
        }
        if e.pc==3{
            if let EventKind::CellWritten{key:9,value}=e.kind{
                if value==source&&src_read.get(&actor).is_some_and(|tick|*tick<e.tick) {
                    canonical.insert(actor);
                }
            }
        }
        if let EventKind::CellWritten{key:12,value}=e.kind{
            assert!(e.pc==32||e.pc==35);
            assert!(value==0||value==1);
            native_counter_writes+=1;
        }
        if let EventKind::ExecutableEdgePatched{node,before,after,..}=e.kind{
            assert!(!disabled,"charged-only suppression must disable true code edit");
            assert_eq!(e.pc,8);
            assert_eq!((node,before,after),(23,4,24));
            assert_eq!(alleles[&actor],4);
            alleles.insert(actor,24);
            native_edits+=1;
        }
        if let EventKind::GraphCreated{graph,owner,..}=e.kind {
            assert_eq!(owner,actor);
            assert!(graph_sources.insert(graph,Vec::new()).is_none());
        }
        if let EventKind::GraphNodeCopied{
            graph,owner,source_process,source_node,new_node,..
        }=e.kind{
            assert_eq!((owner,source_process),(actor,actor));
            assert_eq!(e.pc,16);
            let nodes=graph_sources.get_mut(&graph).unwrap();
            assert_eq!(new_node,nodes.len());
            nodes.push(source_node);
            if source_node==23{
                assert!(graph_allele.insert(graph,alleles[&actor]).is_none());
            }
        }
        if let EventKind::GraphLaunched{
            source_graph,parent_process,child_process,moved_nodes,..
        }=e.kind{
            assert_eq!(e.pc,22);
            assert_eq!(actor,parent_process);
            assert_eq!(moved_nodes,N);
            assert_eq!(graph_sources[&source_graph],(0..N).collect::<Vec<_>>());
            let inherited=graph_allele[&source_graph];
            assert!(inherited==4||inherited==24);
            assert!(alleles.insert(child_process,inherited).is_none());
            let ancestor=founders[&actor];
            assert!(founders.insert(child_process,ancestor).is_none());
            if actor==ancestor{
                ancestor_siblings.entry((actor,inherited)).or_default().push(child_process);
            }
            births+=1;
            if inherited==24{mutation_births+=1;}else{ancestor_births+=1;}
        }
    }
    assert!(native_counter_writes>0);
    if disabled{
        assert_eq!(native_edits,0);
        assert_eq!(mutation_births,0);
    }else{
        assert!(native_edits>0);
        assert!(mutation_births>0);
        assert!(ancestor_births>0);
    }
    let both=([1_u64,2_u64]).iter().any(|founder|{
        let originals=ancestor_siblings.get(&(*founder,4)).cloned().unwrap_or_default();
        let mutants=ancestor_siblings.get(&(*founder,24)).cloned().unwrap_or_default();
        originals.iter().any(|pid|canonical.contains(pid))
            &&mutants.iter().any(|pid|canonical.contains(pid))
    });
    println!("Study131 ORIGINAL source={source} shared_counter_phase={phase} patch_disabled={disabled} paid_native_attempts={paid} true_native_births={births} born_ancestor={ancestor_births} born_mutant={mutation_births} native_effective_edits={native_edits} original_cell12_actor_writes={native_counter_writes} SAME_PARENT_FUNCTIONAL_MUTATED_AND_ORIGINAL_CHILDREN={both} [HUMAN_ENGINEERED_MUTATION_CONTROL_NOT_ADAPTATION]");
    Result131{same_founder_functional_siblings:both,
        real_mutant_births:mutation_births,real_ancestral_births:ancestor_births,paid}
}

#[test]
fn native_shared_phase_creates_unchanged_and_mutated_full_code_sibling_genotypes(){
    let mut all=BTreeMap::new();
    for source in [3_i64,8_i64] {
        for phase in [0_i64,1_i64] {
            for disabled in [false,true]{
                let outcome=trial(source,phase,disabled);
                assert!(outcome.paid>0);
                all.insert((source,phase,disabled),outcome);
            }
        }
    }
    assert_eq!(all.len(),8);
    let mut qualified_phases=Vec::new();
    for phase in [0_i64,1_i64]{
        if [3_i64,8_i64].iter().all(|source|{
            let outcome=&all[&(*source,phase,false)];
            outcome.same_founder_functional_siblings
                &&outcome.real_mutant_births>0
                &&outcome.real_ancestral_births>0
                &&all[&(*source,phase,true)].real_mutant_births==0
        }){
            qualified_phases.push(phase);
        }
    }
    println!("Study131 ORIGINAL COMPLETE: worlds=8 native_mutator_counter_phases_with_actual_same_parent_functional_ancestor_mutant_siblings_under_both_sources={qualified_phases:?} [CALIBRATION_ONLY_NOT_CAUSAL_SELECTION]");
    assert!(!qualified_phases.is_empty(),"precommitted same-parent functional code-mutant plus unchanged sibling exposure gate failed");
}
