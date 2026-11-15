//! CL1 Study140: pre-outcome HUMAN-engineered offspring-graph-only native code
//! mutation. Original founders NEVER edit own executable. Genuine typed
//! GraphReplace of copied child 49-node graph, with active vs dormant null.
use cl0_world::{EventKind,Intervention,Op,Origin,Process,World};
use std::collections::{BTreeMap,BTreeSet};
const N:usize=49;
const TOTAL:u64=10_000;
const K:usize=320;

#[derive(Clone,Copy,Debug,PartialEq,Eq,PartialOrd,Ord)]
enum Locus{Pc4,Pc9}
impl Locus {
 fn site(self)->usize{if self==Self::Pc4{4}else{9}}
 fn original_next(self)->usize{if self==Self::Pc4{5}else{10}}
 fn original_const_value(self)->i64{if self==Self::Pc4{12}else{13}}
 fn target(self,ordinal:i64)->i64{ordinal+if self==Self::Pc4{0}else{5}}
}
#[derive(Clone,Copy,Debug,PartialEq,Eq,PartialOrd,Ord)]
enum Arm{Active,Neutral,Sham}
impl Arm{
 fn site(self,locus:Locus)->usize{
   if self==Self::Neutral{48}else{locus.site()}
 }
 fn original_next(self,locus:Locus)->usize{
   if self==Self::Neutral{48}else{locus.original_next()}
 }
 fn descriptor_dst(self)->usize{if self==Self::Neutral{3}else{2}}
 fn descriptor_value(self,locus:Locus)->i64{
   if self==Self::Neutral{0}else{locus.original_const_value()}
 }
}
fn program(arm:Arm,locus:Locus)->Vec<Op>{
 let code=vec![
  Op::Const{dst:2,value:7,next:1}, //0 source key
  Op::Const{dst:3,value:9,next:2}, //1 output key
  Op::Read{dst:0,address:2,next:3}, //2 source
  Op::Write{address:3,value:0,next:4}, //3 native function
  Op::Const{dst:2,value:12,next:5}, //4 ACTIVE typed code genotype site
  Op::Read{dst:3,address:2,next:6}, //5 old shared phase
  Op::Const{dst:1,value:1,next:7}, //6
  Op::Sub{dst:3,lhs:1,rhs:3,next:8}, //7 new phase=1-old
  Op::Write{address:2,value:3,next:9}, //8 native shared phase flip
  Op::Const{dst:2,value:13,next:10}, //9 temporary flag key
  Op::Write{address:2,value:3,next:11}, //10
  Op::Const{dst:2,value:1,next:12}, //11 counter increment
  Op::Const{dst:1,value:0,next:13}, //12
  Op::Const{dst:3,value:N as i64,next:14}, //13
  Op::GraphNew{dst:0,next:15}, //14 own descendant graph
  Op::GraphCopyNode{graph:0,source:1,dst:1,next:16}, //15
  Op::Add{dst:1,lhs:1,rhs:2,next:17}, //16
  Op::Sub{dst:3,lhs:3,rhs:2,next:18}, //17
  Op::Branch{test:3,nonzero:15,zero:19}, //18
  Op::Const{dst:2,value:13,next:20}, //19 flag
  Op::Read{dst:3,address:2,next:21}, //20
  Op::Branch{test:3,nonzero:22,zero:25}, //21 real edit vs original
  Op::Const{dst:2,value:11,next:23}, //22 original numeric target key
  if arm==Arm::Sham {Op::Const{dst:3,value:arm.original_next(locus) as i64,next:26}} else {Op::Read{dst:3,address:2,next:26}}, //23 sham constant same native VM charge, active/null read true target
  Op::Halt, //24 unreachable placeholder
  Op::Const{dst:3,value:arm.original_next(locus) as i64,next:26}, //25 no change
  Op::Const{dst:2,value:100,next:27}, //26 descriptor tag key
  Op::Const{dst:1,value:0,next:28}, //27 Op::Const tag0
  Op::Write{address:2,value:1,next:29}, //28
  Op::Const{dst:2,value:101,next:30}, //29 dst key
  Op::Const{dst:1,value:arm.descriptor_dst() as i64,next:31}, //30
  Op::Write{address:2,value:1,next:32}, //31
  Op::Const{dst:2,value:102,next:33}, //32 Const literal key
  Op::Const{dst:1,value:arm.descriptor_value(locus),next:34}, //33
  Op::Write{address:2,value:1,next:35}, //34
  Op::Const{dst:2,value:103,next:36}, //35 Const.next key
  Op::Write{address:2,value:3,next:37}, //36 dynamic mutation edge
  Op::Const{dst:2,value:104,next:38}, //37
  Op::Const{dst:1,value:0,next:39}, //38
  Op::Write{address:2,value:1,next:40}, //39
  Op::Const{dst:1,value:arm.site(locus) as i64,next:41}, //40
  Op::Const{dst:2,value:100,next:42}, //41
  Op::GraphReplace{graph:0,node:1,address:2,next:43}, //42 native child-only mutation
  Op::Const{dst:2,value:400,next:44}, //43
  Op::DrawCompute{amount:2,next:45}, //44 native pool charge
  Op::GraphLaunch{graph:0,credits:2,dst:3,next:46}, //45
  Op::Const{dst:2,value:7,next:47}, //46 loop father, unchanged
  Op::Const{dst:3,value:9,next:2}, //47
  Op::Const{dst:3,value:0,next:48}, //48 DORMANT real executable
 ];
 assert_eq!(code.len(),N);
 assert!(!code.iter().any(|x|matches!(x,Op::PatchNext{..})));
 code
}
fn splitmix64(state:&mut u64)->u64{
 *state=state.wrapping_add(0x9e3779b97f4a7c15);
 let mut z=*state;
 z=(z^(z>>30)).wrapping_mul(0xbf58476d1ce4e5b9);
 z=(z^(z>>27)).wrapping_mul(0x94d049bb133111eb);
 z^(z>>31)
}
#[derive(Clone,Debug)]
struct Birth{
 parent:u64,depth:usize,born_code:Vec<Op>,full:bool,
 parent_attempt_at_birth:usize,
}
#[derive(Default,Debug)]
struct Summary{
 native_births:usize, full_born:usize, mutant_full_born:usize,
 canonical_ancestral:usize,canonical_mutated:usize,
 k320_eligible_fathers:usize,
 k320_a_parents:usize,k320_h_parents:usize,
 k320_a_children:usize,k320_h_children:usize,
 neutral_dormant_pc_attempts:usize,
 native_graph_mutation_events:usize,
}
fn trial(locus:Locus,target:i64,arm:Arm,source:i64,phase:i64)->Summary{
 let original=program(arm,locus);
 let mut w=World::with_strict_capacity(256,64,2048)
     .with_pool_paid_draw().with_recoverable_draw_shortage()
     .with_native_executable_edit_trace();
 w.load(Process::new(1,original.clone(),1000)).unwrap();
 w.load(Process::new(2,original.clone(),1000)).unwrap();
 for (key,value) in [(7,source),(11,target),(12,phase)] {
  w.intervene(Intervention::SetCell{key,value}).unwrap();
 }
 w.intervene(Intervention::SetComputePool{credits:8000}).unwrap();
 let attempts=w.run(10000);
 let mut check_terminal=w.clone();
 assert!(!check_terminal.step(),"all native Worlds must reach true terminal");
 assert_eq!(w.total_compute_inventory(),Some(TOTAL-attempts as u64));
 assert_eq!(w.events().iter().filter(|e|e.origin==Origin::External).count(),6);
 assert_eq!(w.process(1).unwrap().code.nodes,original);
 assert_eq!(w.process(2).unwrap().code.nodes,original);

 let mut actors=BTreeMap::<u64,Vec<Op>>::from([(1,original.clone()),(2,original.clone())]);
 let mut births=BTreeMap::<u64,Birth>::new();
 let mut graph=BTreeMap::<u64,Vec<Op>>::new();
 let mut graph_sources=BTreeMap::<u64,Vec<usize>>::new();
 let mut owner=BTreeMap::<u64,u64>::new();
 let mut reads=BTreeMap::<u64,u64>::new();
 let mut canonical=BTreeSet::<u64>::new();
 let mut age=BTreeMap::<u64,usize>::new();
 let mut child_attempt=BTreeMap::<u64,usize>::new();
 let mut native_graph_mutation_events=0_usize;
 let mut neutral_dormant_pc_attempts=0_usize;
 for e in w.events() {
  if e.origin==Origin::External {continue;}
  let actor=e.process;
  assert_eq!(e.origin,Origin::Internal(actor));
  assert!(actors.contains_key(&actor));
  *age.entry(actor).or_default()+=1;
  assert!(e.pc<actors[&actor].len());
  if arm==Arm::Neutral&&e.pc==48 {neutral_dormant_pc_attempts+=1;}
  match &e.kind {
   EventKind::CellRead{key:7,value} if e.pc==2&&*value==source=>{
    reads.insert(actor,e.tick);
   }
   EventKind::CellWritten{key:9,value} if e.pc==3&&*value==source=>{
    if reads.get(&actor).is_some_and(|t|*t<e.tick){canonical.insert(actor);}
   }
   EventKind::ExecutableEdgePatched{..}=>panic!("graph-only native offspring mutation CANNOT edit actor source"),
   EventKind::GraphCreated{graph:g,owner:o}=>{
    assert_eq!(*o,actor);
    assert!(owner.insert(*g,actor).is_none());
    assert!(graph.insert(*g,Vec::new()).is_none());
    assert!(graph_sources.insert(*g,Vec::new()).is_none());
   }
   EventKind::GraphNodeCopied{graph:g,owner:o,source_process,source_node,new_node}=>{
    assert_eq!((*o,*source_process),(actor,actor));
    assert_eq!(e.pc,15);
    assert_eq!(owner[g],actor);
    assert_eq!(*new_node,graph[g].len());
    assert!(*source_node<actors[&actor].len());
    graph.get_mut(g).unwrap().push(actors[&actor][*source_node].clone());
    graph_sources.get_mut(g).unwrap().push(*source_node);
   }
   EventKind::GraphReplaced{graph:g,owner:o,node,descriptor_base,descriptor}=>{
    assert_eq!(e.pc,42);
    assert_eq!(*o,actor);
    assert_eq!(owner[g],actor);
    assert_eq!(*node,arm.site(locus));
    assert_eq!(*descriptor_base,100);
    let expect=if arm==Arm::Neutral {
        [0,3,0,descriptor[3],0]
      } else {[0,2,locus.original_const_value(),descriptor[3],0]};
    assert_eq!(*descriptor,expect);
    let new_op=Op::decode_instruction(*descriptor).unwrap();
    assert!(*node<graph[g].len());
    graph.get_mut(g).unwrap()[*node]=new_op;
    native_graph_mutation_events+=1;
   }
   EventKind::GraphLaunched{source_graph,parent_process,child_process,moved_nodes,..}=>{
    assert_eq!(*parent_process,actor);
    assert_eq!(e.pc,45);
    assert_eq!(owner[source_graph],actor);
    assert_eq!(*moved_nodes,graph[source_graph].len());
    let b=graph[source_graph].clone();
    assert!(!b.is_empty()&&b.len()<=64);
    let full=b.len()==N&&graph_sources[source_graph]==(0..N).collect::<Vec<_>>();
    assert!(actors.insert(*child_process,b.clone()).is_none());
    assert!(child_attempt.insert(*child_process,age[&actor]).is_none());
    let depth=births.get(&actor).map_or(1,|p|p.depth+1);
    assert!(births.insert(*child_process,Birth{
      parent:actor,depth,born_code:b,full,parent_attempt_at_birth:age[&actor],
    }).is_none());
   }
   _=>{}
  }
 }
 for (pid,code) in &actors{
  assert_eq!(w.process(*pid).unwrap().code.nodes,*code,
    "all actual child executable code must be reconstructed from native graph events");
 }
 assert_eq!(actors.len(),births.len()+2);
 assert_eq!(age.values().copied().sum::<usize>(),attempts);
 let mut r=Summary{native_births:births.len(),
   native_graph_mutation_events,neutral_dormant_pc_attempts,..Default::default()};
 r.full_born=births.values().filter(|b|b.full).count();
 r.mutant_full_born=births.values().filter(|b|b.full&&b.born_code!=original).count();
 r.canonical_ancestral=births.iter().filter(|(pid,b)|b.full&&b.born_code==original&&canonical.contains(pid)).count();
 r.canonical_mutated=births.iter().filter(|(pid,b)|b.full&&b.born_code!=original&&canonical.contains(pid)).count();
 if arm==Arm::Sham {assert_eq!(r.mutant_full_born,0);}
 for father in [1_u64,2_u64]{
  let a=births.iter().filter_map(|(pid,b)|(
    b.parent==father&&b.depth==1&&b.full&&b.born_code==original&&canonical.contains(pid)
   ).then_some(*pid)).collect::<Vec<_>>();
  let h=births.iter().filter_map(|(pid,b)|(
    b.parent==father&&b.depth==1&&b.full&&b.born_code!=original&&canonical.contains(pid)
   ).then_some(*pid)).collect::<Vec<_>>();
  let a_ex=a.iter().filter(|p|age.get(p).copied().unwrap_or(0)>=K).copied().collect::<Vec<_>>();
  let h_ex=h.iter().filter(|p|age.get(p).copied().unwrap_or(0)>=K).copied().collect::<Vec<_>>();
  if a_ex.is_empty()||h_ex.is_empty(){continue;}
  r.k320_eligible_fathers+=1;
  r.k320_a_parents+=a_ex.len();
  r.k320_h_parents+=h_ex.len();
  let child_count=|pids:&[u64]|births.iter().filter(|(cid,b)|
      pids.contains(&b.parent)&&b.full&&canonical.contains(cid)
      &&b.parent_attempt_at_birth<=K
    ).count();
  r.k320_a_children+=child_count(&a_ex);
  r.k320_h_children+=child_count(&h_ex);
 }
 println!("Study140 ORIGINAL locus={locus:?} target={target} active_or_null={arm:?} source={source} global_phase={phase} original_target={target} paid_attempts={attempts} native_graph_replace={} born={} full={} changed_full={} source_function_original={} source_function_changed={} k320_eligible_true_fathers={} k320_unedited_parents={} k320_mutated_parents={} k320_unedited_children={} k320_mutated_children={} dormant_pc48_attempts={} [NO_PARENT_EXECUTABLE_EDIT]",r.native_graph_mutation_events,r.native_births,r.full_born,r.mutant_full_born,r.canonical_ancestral,r.canonical_mutated,r.k320_eligible_fathers,r.k320_a_parents,r.k320_h_parents,r.k320_a_children,r.k320_h_children,r.neutral_dormant_pc_attempts);
 r
}

#[test]
fn complete_two_locus_nine_target_native_child_only_allele_census(){
 let mut original=BTreeMap::new();
 for locus in [Locus::Pc4,Locus::Pc9] {
  for ordinal in 5_i64..=13 {
   let target=locus.target(ordinal);
   for arm in [Arm::Active,Arm::Neutral,Arm::Sham] {
    for source in [3_i64,8_i64] {
     for phase in [0_i64,1_i64] {
      let r=trial(locus,target,arm,source,phase);
      assert!(original.insert((locus,target,arm,source,phase),r).is_none());
     }
    }
   }
  }
 }
 assert_eq!(original.len(),216);
 let mut all4_eligible=Vec::<(Locus,i64)>::new();
 let mut pc9_changed_site_sign_flip=Vec::<i64>::new();
 let mut eligible_by_locus=BTreeMap::<Locus,usize>::new();
 let mut sign_counts=BTreeMap::<(Locus,i64),[usize;3]>::new();
 for locus in [Locus::Pc4,Locus::Pc9] {
  for ordinal in 5_i64..=13 {
   let target=locus.target(ordinal);
   let mut signs=Vec::<(i64,i64,i8)>::new();
   for source in [3_i64,8_i64] {
    for phase in [0_i64,1_i64] {
     let a=&original[&(locus,target,Arm::Active,source,phase)];
     let n=&original[&(locus,target,Arm::Neutral,source,phase)];
     let sham=&original[&(locus,target,Arm::Sham,source,phase)];
     assert_eq!(sham.mutant_full_born,0);
     let eligible=a.k320_eligible_fathers>0&&n.k320_eligible_fathers>0
       &&n.neutral_dormant_pc_attempts==0;
     if !eligible {
      println!("Study140 ORIGINAL GENOTYPE locus={locus:?} target={target} source={source} phase={phase} age320_comparable=false active_fathers={} neutral_fathers={} [RETAINED_ALL_EXACT18]",a.k320_eligible_fathers,n.k320_eligible_fathers);
      continue;
     }
     *eligible_by_locus.entry(locus).or_default()+=1;
     let an=(a.k320_h_children*a.k320_a_parents) as i128
          -(a.k320_a_children*a.k320_h_parents) as i128;
     let ad=(a.k320_h_parents*a.k320_a_parents) as i128;
     let nn=(n.k320_h_children*n.k320_a_parents) as i128
          -(n.k320_a_children*n.k320_h_parents) as i128;
     let nd=(n.k320_h_parents*n.k320_a_parents) as i128;
     assert!(ad>0&&nd>0);
     let effect=an*nd-nn*ad;
     let sign=if effect<0{-1_i8}else if effect>0{1_i8}else{0_i8};
     let sc=sign_counts.entry((locus,target)).or_insert([0,0,0]);
     sc[(sign+1) as usize]+=1;
     signs.push((source,phase,sign));
     println!("Study140 ORIGINAL GENOTYPE locus={locus:?} target={target} source={source} phase={phase} age320_comparable=true active_changed_minus_unchanged={an}/{ad} neutral_changed_minus_unchanged={nn}/{nd} active_minus_neutral_sign={sign} [REAL_NATIVE_OWNER_GRAPH_MUTATION]");
    }
   }
   if signs.len()==4 {
    all4_eligible.push((locus,target));
    if locus==Locus::Pc9&&target!=locus.original_next() as i64{
     let phase0=signs.iter().filter(|x|x.1==0).map(|x|x.2).collect::<Vec<_>>();
     let phase1=signs.iter().filter(|x|x.1==1).map(|x|x.2).collect::<Vec<_>>();
     if phase0.len()==2&&phase1.len()==2
      &&phase0[0]==phase0[1]&&phase1[0]==phase1[1]
      &&phase0[0]!=0&&phase1[0]!=0&&phase0[0]!=phase1[0]{
       pc9_changed_site_sign_flip.push(target);
     }
    }
   }
  }
 }
 println!("Study140 ORIGINAL COMPLETE: truly_distinct_original_site_target_genotypes=18 fully_original_worlds=216 original_native_charge_conserved=true all_four_source_phase_comparable_genotypes={all4_eligible:?} source_phase_eligible_cells_by_locus={eligible_by_locus:?} exact_signs_by_allele={sign_counts:?} site_PC9_fully_comparable_changed_alleles_with_phase_sign_flip={pc9_changed_site_sign_flip:?} [HUMAN_NATIVE_GRAPH_MUTATOR_NOT_ADAPTIVE_SELECTION]");
 assert!(!pc9_changed_site_sign_flip.is_empty(),
  "Study140 preregistered transfer of phase sign instability to a SECOND active CIL locus FAILED; preserve all18 unique allele worlds");
}
