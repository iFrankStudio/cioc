//! Study133: engineered positive-control topology, NOT natural evolution.
//! A 38-op native CIL self-copier uses ordinary shared cell12 to alternate
//! no-edit versus PatchNext BEFORE building its next whole38-op child graph.
//! Genuine same-founder original and mutant sibling births, not host code.
use cl0_world::{EventKind,Intervention,Op,Origin,Process,World};
use std::collections::{BTreeMap,BTreeSet};

const N:usize=38;
const TOTAL:u64=10_000;

fn program(site:i64)->Vec<Op>{
    let p=vec![
        Op::Const{dst:2,value:7,next:1},     // 0
        Op::Const{dst:3,value:9,next:2},     // 1
        Op::Read{dst:0,address:2,next:3},    // 2 original source
        Op::Write{address:3,value:0,next:4},// 3 function
        Op::Const{dst:2,value:1,next:24},    // 4 redundant before delay
        Op::Const{dst:2,value:11,next:6},    // 5 mutation target cell
        Op::Read{dst:0,address:2,next:7},    // 6 R0 target24
        Op::Const{dst:1,value:site,next:28},   // 7 native site23, phase switch
        Op::PatchNext{node:1,target:0,next:9},// 8 native effective code edit
        Op::Const{dst:1,value:37,next:10},   // 9
        Op::Sub{dst:1,lhs:1,rhs:0,next:11},   // 10 ordinary target held fixed
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

fn splitmix64(state:&mut u64)->u64{
    *state=state.wrapping_add(0x9e3779b97f4a7c15);
    let mut z=*state;
    z=(z^(z>>30)).wrapping_mul(0xbf58476d1ce4e5b9);
    z=(z^(z>>27)).wrapping_mul(0x94d049bb133111eb);
    z^(z>>31)
}
#[derive(Clone,Debug)]
struct Born132{
    parent:u64,
    root:u64,
    depth:usize,
    code:Vec<Op>,
    full:bool,
}
#[derive(Default)]
struct Results132{
    original_seed:u64,
    same_parent_functional_siblings:bool,
    eligible_matched_functional_rate_comparisons:usize,
    mutant_higher_reproduction_rate:usize,
    mutant_lower_reproduction_rate:usize,
    mutant_equal_reproduction_rate:usize,
    exposed_240_eligible:usize,
    exposed_240_mutant_higher:usize,
    exposed_240_mutant_lower:usize,
    exposed_240_tie:usize,
    exposed_240_excluded_ancestor:usize,
    exposed_240_excluded_mutant:usize,
    exposed_400_eligible:usize,
    exposed_400_mutant_higher:usize,
    exposed_400_mutant_lower:usize,
    exposed_400_tie:usize,
    exposed_400_excluded_ancestor:usize,
    exposed_400_excluded_mutant:usize,
    functional_expressed_mutant_children:usize,
    all_native_births:usize,
    full_mutant_births:usize,
    full_original_births:usize,
    edits:usize,
}
fn native_patch(op:&Op,before:usize,after:usize)->Op{
    let mut image=serde_json::to_value(op).expect("actual op must serialize");
    let root=image.as_object_mut().expect("one tagged native op");
    assert_eq!(root.len(),1);
    let fields=root.values_mut().next().unwrap().as_object_mut()
        .expect("only writable native next-edge op emitted an effective patch");
    assert_eq!(fields.get("next").and_then(|v|v.as_u64()),Some(before as u64));
    fields.insert("next".to_string(),serde_json::json!(after));
    serde_json::from_value(image).expect("native patched instruction must decode")
}
fn trial(seed:u64,source:i64,phase:i64,disabled:bool)->Results132{
    let mut state=seed;
    let site=(splitmix64(&mut state)%38) as i64;
    let target=(splitmix64(&mut state)%38) as i64;
    let original=program(site);
    let mut w=World::with_strict_capacity(256,64,2048)
        .with_pool_paid_draw().with_recoverable_draw_shortage()
        .with_native_executable_edit_trace();
    if disabled {w=w.with_native_patchnext_disabled();}
    w.load(Process::new(1,original.clone(),1000)).unwrap();
    w.load(Process::new(2,original.clone(),1000)).unwrap();
    for (key,value) in [(7,source),(11,target),(12,phase)] {
        w.intervene(Intervention::SetCell{key,value}).unwrap();
    }
    w.intervene(Intervention::SetComputePool{credits:8000}).unwrap();
    let attempts=w.run(10000);
    assert!(attempts>0&&attempts<=10000);
    let mut natural_terminal=w.clone();
    assert!(!natural_terminal.step(),
        "study133 requires actual no-more-compute native scheduler terminal, not a fixed cutoff");
    assert_eq!(w.total_compute_inventory(),Some(TOTAL-attempts as u64));
    assert_eq!(w.events().iter().filter(|e|e.origin==Origin::External).count(),6);
    let mut code=BTreeMap::<u64,Vec<Op>>::from([(1,original.clone()),(2,original.clone())]);
    let mut root=BTreeMap::<u64,u64>::from([(1,1),(2,2)]);
    let mut born=BTreeMap::<u64,Born132>::new();
    let mut sources=BTreeMap::<u64,Vec<usize>>::new();
    let mut graphs=BTreeMap::<u64,Vec<Op>>::new();
    let mut owner=BTreeMap::<u64,u64>::new();
    let mut last_read=BTreeMap::<u64,u64>::new();
    let mut canonical=BTreeSet::<u64>::new();
    let mut expressed=BTreeSet::<u64>::new();
    let mut charged_native_actor_attempts=BTreeMap::<u64,usize>::new();
    let mut birth_parent_attempt_index=BTreeMap::<u64,usize>::new();
    let mut edits=0_usize;
    let mut native_counter_writes=0_usize;
    for event in w.events() {
        if event.origin==Origin::External {continue;}
        let actor=event.process;
        assert_eq!(event.origin,Origin::Internal(actor));
        assert!(code.contains_key(&actor),"all native actors must have a true parent");
        *charged_native_actor_attempts.entry(actor).or_default()+=1;
        let pc=event.pc;
        assert!(pc<code[&actor].len());
        if let Some(birth)=born.get(&actor) {
            if birth.full&&birth.code[pc]!=original[pc]
                &&code[&actor][pc]==birth.code[pc]
                &&!matches!(event.kind,EventKind::Trapped(_)){
                expressed.insert(actor);
            }
        }
        match &event.kind {
            EventKind::CellRead{key:7,value} if pc==2&&*value==source=>{
                last_read.insert(actor,event.tick);
            }
            EventKind::CellWritten{key:9,value} if pc==3&&*value==source=>{
                if last_read.get(&actor).is_some_and(|t|*t<event.tick){
                    canonical.insert(actor);
                }
            }
            EventKind::CellWritten{key:12,value}=>{
                assert!(pc==32||pc==35);
                assert!(*value==0||*value==1);
                native_counter_writes+=1;
            }
            EventKind::ExecutableEdgePatched{node,before,after}=>{
                assert!(!disabled&&pc==8);
                // The initial actor's site is seed-determined, but a later
                // genuinely inherited CIL control-flow mutation can skip the
                // Const at PC7; its own mutation register R1 may then point
                // elsewhere. This is a REAL secondary native edit, not a host
                // intervention or auditor error. Preserve it unfiltered.
                assert!(*node<code[&actor].len());
                assert!(*after<38);
                assert_ne!(before,after);
                let current=code.get_mut(&actor).unwrap();
                current[*node]=native_patch(&current[*node],*before,*after);
                edits+=1;
            }
            EventKind::GraphCreated{graph,owner:o}=>{
                assert_eq!(*o,actor);
                assert!(owner.insert(*graph,actor).is_none());
                assert!(graphs.insert(*graph,vec![]).is_none());
                assert!(sources.insert(*graph,vec![]).is_none());
            }
            EventKind::GraphNodeCopied{graph,owner:o,source_process,source_node,new_node}=>{
                assert_eq!(pc,16);
                assert_eq!((*o,*source_process),(actor,actor));
                assert_eq!(owner[graph],actor);
                let g=graphs.get_mut(graph).unwrap();
                assert_eq!(*new_node,g.len());
                assert!(*source_node<code[&actor].len());
                g.push(code[&actor][*source_node].clone());
                sources.get_mut(graph).unwrap().push(*source_node);
            }
            EventKind::GraphLaunched{source_graph,parent_process,child_process,moved_nodes,..}=>{
                assert_eq!(pc,22);
                assert_eq!(*parent_process,actor);
                assert_eq!(owner[source_graph],actor);
                let generated=graphs[source_graph].clone();
                assert_eq!(generated.len(),*moved_nodes);
                assert!(birth_parent_attempt_index.insert(
                    *child_process,charged_native_actor_attempts[&actor]).is_none());
                assert!(!generated.is_empty()&&generated.len()<=64);
                assert!(code.insert(*child_process,generated.clone()).is_none());
                let ancestor=root[&actor];
                assert!(root.insert(*child_process,ancestor).is_none());
                let generation=born.get(&actor).map_or(1,|p|p.depth+1);
                let full=generated.len()==N&&sources[source_graph]==(0..N).collect::<Vec<_>>();
                assert!(born.insert(*child_process,Born132{
                    parent:actor,root:ancestor,depth:generation,code:generated,full
                }).is_none());
            }
            _=>{}
        }
    }
    for (pid,c) in &code{
        assert_eq!(w.process(*pid).unwrap().code.nodes,*c,
            "all native executable patches and newborn code must source-replay exactly");
    }
    assert_eq!(code.len(),w.events().iter().filter(|e|
        matches!(e.kind,EventKind::GraphLaunched{..})).count()+2);
    assert!(native_counter_writes>0);
    if disabled{
        assert_eq!(edits,0);
        assert!(born.values().filter(|b|b.full).all(|b|b.code==original),
            "narrow PatchNext-disabled must never create a new faithful full genotype");
    }
    let mut result=Results132{original_seed:seed,edits,all_native_births:born.len(),..Default::default()};
    result.full_mutant_births=born.values().filter(|b|b.full&&b.code!=original).count();
    result.full_original_births=born.values().filter(|b|b.full&&b.code==original).count();
    result.functional_expressed_mutant_children=born.iter().filter(|(pid,b)|
        b.full&&b.code!=original&&canonical.contains(pid)&&expressed.contains(pid)
    ).count();
    for founder in [1_u64,2_u64] {
        let all_ancestor:Vec<u64>=born.iter().filter_map(|(pid,b)|
            (b.parent==founder&&b.depth==1&&b.full&&b.code==original
             &&canonical.contains(pid)).then_some(*pid)
        ).collect();
        let all_mutant:Vec<u64>=born.iter().filter_map(|(pid,b)|
            (b.parent==founder&&b.depth==1&&b.full&&b.code!=original
             &&canonical.contains(pid)).then_some(*pid)
        ).collect();
        if all_ancestor.is_empty()||all_mutant.is_empty(){continue;}
        result.same_parent_functional_siblings=true;
        result.eligible_matched_functional_rate_comparisons+=1;
        let children_of=|group:&[u64]|->usize{
            born.iter().filter(|(child,b)|
                group.contains(&b.parent)&&b.full&&canonical.contains(child)
            ).count()
        };
        let original_kids=children_of(&all_ancestor);
        let mutated_kids=children_of(&all_mutant);
        let lhs=mutated_kids*all_ancestor.len();
        let rhs=original_kids*all_mutant.len();
        if lhs>rhs{result.mutant_higher_reproduction_rate+=1;}
        else if lhs<rhs{result.mutant_lower_reproduction_rate+=1;}
        else{result.mutant_equal_reproduction_rate+=1;}
        println!("Study133 ORIGINAL same_parent_terminal_unrestricted_rate seed={seed} source={source} phase={phase} disabled={disabled} founder={founder} full_source_function_ancestral_parents={} full_source_function_mutant_parents={} full_source_function_ancestral_immediate_children={original_kids} full_source_function_mutant_immediate_children={mutated_kids} mutant_higher={} [BIRTH_ORDER_CONFOUNDED]",all_ancestor.len(),all_mutant.len(),lhs>rhs);
        for limit in [240_usize,400_usize] {
            let exposed_a=all_ancestor.iter().copied().filter(|pid|
                charged_native_actor_attempts.get(pid).copied().unwrap_or(0)>=limit
            ).collect::<Vec<_>>();
            let exposed_h=all_mutant.iter().copied().filter(|pid|
                charged_native_actor_attempts.get(pid).copied().unwrap_or(0)>=limit
            ).collect::<Vec<_>>();
            let missing_a=all_ancestor.len()-exposed_a.len();
            let missing_h=all_mutant.len()-exposed_h.len();
            if limit==240{
                result.exposed_240_excluded_ancestor+=missing_a;
                result.exposed_240_excluded_mutant+=missing_h;
            } else {
                result.exposed_400_excluded_ancestor+=missing_a;
                result.exposed_400_excluded_mutant+=missing_h;
            }
            if exposed_a.is_empty()||exposed_h.is_empty(){
                println!("Study133 ORIGINAL actor_native_exposure rate INELIGIBLE seed={seed} source={source} phase={phase} disabled={disabled} founder={founder} K={limit} ancestral_exposure_complete={} mutant_exposure_complete={} ancestral_incomplete={missing_a} mutant_incomplete={missing_h}",exposed_a.len(),exposed_h.len());
                continue;
            }
            let within_k=|group:&[u64]|->usize{
                born.iter().filter(|(child,b)|
                    group.contains(&b.parent)&&b.full&&canonical.contains(child)
                    &&birth_parent_attempt_index[child]<=limit
                ).count()
            };
            let a_kids=within_k(&exposed_a);
            let h_kids=within_k(&exposed_h);
            let numerator_h=h_kids*exposed_a.len();
            let numerator_a=a_kids*exposed_h.len();
            let higher=numerator_h>numerator_a;
            let lower=numerator_h<numerator_a;
            if limit==240 {
                result.exposed_240_eligible+=1;
                if higher{result.exposed_240_mutant_higher+=1;}
                else if lower{result.exposed_240_mutant_lower+=1;}
                else{result.exposed_240_tie+=1;}
            }else{
                result.exposed_400_eligible+=1;
                if higher{result.exposed_400_mutant_higher+=1;}
                else if lower{result.exposed_400_mutant_lower+=1;}
                else{result.exposed_400_tie+=1;}
            }
            println!("Study133 ORIGINAL actor_native_exposure_rate seed={seed} source={source} phase={phase} disabled={disabled} founder={founder} K={limit} original_exposure_complete_parents={} mutant_exposure_complete_parents={} ancestor_functional_immediate_children_within_K={a_kids} mutant_functional_immediate_children_within_K={h_kids} ancestor_incomplete={missing_a} mutant_incomplete={missing_h} mutant_rate_higher={higher} mutant_rate_lower={lower} [EXPOSURE_MATCHED_NOT_CAUSAL]",exposed_a.len(),exposed_h.len());
        }
    }
    println!("Study133 ORIGINAL seed={seed} site={site} initial_target={target} source={source} phase={phase} disabled={disabled} true_native_terminal_paid_attempts={attempts} born_native_actors={} full_ancestral_births={} full_mutant_births={} expressed_source_functional_mutant_births={} original_native_effective_edits={} same_parent_source_functional_both_alleles={} terminal_unmatched_rate_comparisons={} terminal_unmatched_mutant_higher={} terminal_unmatched_mutant_lower={} terminal_unmatched_tie={} eligible_age240={} age240_mutant_higher={} age240_mutant_lower={} age240_tie={} eligible_age400={} age400_mutant_higher={} age400_mutant_lower={} age400_tie={} [TRUE_NATIVE_TERMINAL_AGE_EXPOSURE_ONLY]",result.all_native_births,result.full_original_births,result.full_mutant_births,result.functional_expressed_mutant_children,result.edits,result.same_parent_functional_siblings,result.eligible_matched_functional_rate_comparisons,result.mutant_higher_reproduction_rate,result.mutant_lower_reproduction_rate,result.mutant_equal_reproduction_rate,result.exposed_240_eligible,result.exposed_240_mutant_higher,result.exposed_240_mutant_lower,result.exposed_240_tie,result.exposed_400_eligible,result.exposed_400_mutant_higher,result.exposed_400_mutant_lower,result.exposed_400_tie);
    result
}

#[test]
fn prospective_all16_random_native_mutation_site_target_sibling_controls(){
    let mut results=BTreeMap::new();
    let mut genotypes=BTreeSet::new();
    for seed in 7_700_000_u64..7_700_016{
        let mut state=seed;
        let site=splitmix64(&mut state)%38;
        let target=splitmix64(&mut state)%38;
        genotypes.insert((site,target));
        for source in [3_i64,8_i64]{
            for phase in [0_i64,1_i64]{
                for disabled in [false,true]{
                    let r=trial(seed,source,phase,disabled);
                    assert_eq!(r.original_seed,seed);
                    assert!(results.insert((seed,source,phase,disabled),r).is_none());
                }
            }
        }
    }
    assert_eq!(results.len(),128);
    let dual_source_phase0_exposure=(7_700_000_u64..7_700_016).filter(|seed|
        [3_i64,8_i64].iter().all(|source|
            results[&(*seed,*source,0,false)].same_parent_functional_siblings
        )
    ).collect::<Vec<_>>();
    let eligible=results.values().map(|r|r.eligible_matched_functional_rate_comparisons).sum::<usize>();
    let positives=results.values().map(|r|r.mutant_higher_reproduction_rate).sum::<usize>();
    let negatives=results.values().map(|r|r.mutant_lower_reproduction_rate).sum::<usize>();
    let ties=results.values().map(|r|r.mutant_equal_reproduction_rate).sum::<usize>();
    assert_eq!(positives+negatives+ties,eligible);
    let age240_eligible=results.values().map(|r|r.exposed_240_eligible).sum::<usize>();
    let age240_high=results.values().map(|r|r.exposed_240_mutant_higher).sum::<usize>();
    let age240_low=results.values().map(|r|r.exposed_240_mutant_lower).sum::<usize>();
    let age240_tie=results.values().map(|r|r.exposed_240_tie).sum::<usize>();
    let age400_eligible=results.values().map(|r|r.exposed_400_eligible).sum::<usize>();
    let age400_high=results.values().map(|r|r.exposed_400_mutant_higher).sum::<usize>();
    let age400_low=results.values().map(|r|r.exposed_400_mutant_lower).sum::<usize>();
    let age400_tie=results.values().map(|r|r.exposed_400_tie).sum::<usize>();
    assert_eq!(age240_high+age240_low+age240_tie,age240_eligible);
    assert_eq!(age400_high+age400_low+age400_tie,age400_eligible);
    println!("Study133 ORIGINAL COMPLETE: fresh_original_seed_labels=16 distinct_native_site_target_pairs={} true_resource_terminal_worlds=128 original_phase0_functional_same_parent_sibling_seeds_both_sources={dual_source_phase0_exposure:?} all_world_UNMATCHED_rate_comparisons={eligible} crude_mutant_higher={positives} crude_mutant_lower={negatives} crude_ties={ties} AGE240_eligible={age240_eligible} AGE240_mutant_higher={age240_high} AGE240_mutant_lower={age240_low} AGE240_ties={age240_tie} AGE400_eligible={age400_eligible} AGE400_mutant_higher={age400_high} AGE400_mutant_lower={age400_low} AGE400_ties={age400_tie} [OBSERVATIONAL_NATIVE_EXPOSURE_NOT_CAUSAL_SELECTION]",genotypes.len());
    assert!(!dual_source_phase0_exposure.is_empty(),
        "prospectively frozen all-site-target same-original-parent functional sibling exposure gate FAILED");
}
