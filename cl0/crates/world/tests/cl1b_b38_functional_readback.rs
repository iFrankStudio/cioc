//! CL-1B B38: real task-output readback can condition a native in-place
//! CIL edge repair. The exact task, comparator, bad edge and repair are
//! investigator-authored; no autonomous goal or novel candidate discovery.
use cl0_world::{EventKind,Intervention,Op,Origin,Process,World};

#[derive(Clone,Copy,Debug,PartialEq,Eq)]
enum Arm{Readback,AssumeOk,ForceFix,PatchDisabled,StaticCorrect}

fn founder(arm:Arm)->Vec<Op>{
 if arm==Arm::StaticCorrect{
  return vec![
   Op::Const{dst:2,value:7,next:1},
   Op::Read{dst:0,address:2,next:2},
   Op::Const{dst:2,value:11,next:3},
   Op::Write{address:2,value:0,next:0},
  ];
 }
 vec![
  Op::Const{dst:3,value:0,next:1},                      // PC0 programmed wrong output
  Op::Const{dst:2,value:7,next:2},                      // PC1 read task input
  Op::Read{dst:0,address:2,next:3},                     // PC2
  Op::Const{dst:2,value:11,next:4},                     // PC3 editable next→11
  Op::Write{address:2,value:3,next:5},                  // PC4 wrong output if input !=0
  Op::Read{dst:1,address:2,next:6},                     // PC5 read actual output
  match arm{
   Arm::AssumeOk=>Op::Sub{dst:1,lhs:0,rhs:0,next:7},     // evidence ignored same cost
   Arm::ForceFix=>Op::Const{dst:1,value:1,next:7},      // forced repair same cost
   _=>Op::Sub{dst:1,lhs:0,rhs:1,next:7},                // real functional mismatch
  },
  Op::Branch{test:1,nonzero:8,zero:1},                  // PC7 conditional decision
  Op::Const{dst:1,value:3,next:9},                      // PC8 own code node3
  Op::Const{dst:2,value:11,next:10},                    // PC9 correct Write PC
  Op::PatchNext{node:1,target:2,next:1},                // PC10 native edit PC3.next→11
  Op::Write{address:2,value:0,next:5},                  // PC11 correct output
 ]
}

fn current_service_edge(w:&World,arm:Arm)->usize{
 let index=if arm==Arm::StaticCorrect{2}else{3};
 match &w.process(1).unwrap().code.nodes[index]{
  Op::Const{next,..}=>*next,
  _=>panic!("B38 service routing instruction lost")
 }
}
fn events_since(w:&World,previous:usize,input:i64)->(usize,usize){
 let mut correct=0;
 let mut incorrect=0;
 for e in &w.events()[previous..]{
  if let (Origin::Internal(1),EventKind::CellWritten{key:11,value})=(&e.origin,&e.kind){
   if *value==input{correct+=1;}else{incorrect+=1;}
  }
 }
 (correct,incorrect)
}

#[derive(Debug)]
struct Trial{
 arm:Arm,history:(i64,i64),correct:Vec<usize>,incorrect:Vec<usize>,
 writes:Vec<usize>,charged:usize,edits:usize,suppressed:usize,
 edge_after_first:usize,final_edge:usize,stopped:bool,traps:usize,
 final_input:i64,final_output:Option<i64>,
}
fn run(arm:Arm,history:(i64,i64))->Trial{
 let mut w=World::with_strict_capacity(16,64,128)
  .with_native_executable_edit_trace();
 if arm==Arm::PatchDisabled{w=w.with_native_patchnext_disabled();}
 let source=founder(arm);
 assert_eq!(source.len(),if arm==Arm::StaticCorrect{4}else{12});
 w.load(Process::new(1,source,96)).unwrap();
 let mut correct=Vec::new();
 let mut incorrect=Vec::new();
 let mut writes=Vec::new();
 let mut after_first=0;
 let mut paid=0;
 for (phase,&input) in [history.0,history.1].iter().enumerate(){
  w.intervene(Intervention::SetCell{key:7,value:input}).unwrap();
  assert_eq!(w.events().last().unwrap().origin,Origin::External);
  let old_len=w.events().len();
  let old_compute=w.total_compute_inventory().unwrap();
  let steps=w.run(48);
  assert_eq!(steps,48,"B38 fixed native opportunity 48 paid instructions per phase");
  assert_eq!(w.total_compute_inventory(),Some(old_compute-48));
  paid+=steps;
  let (c,b)=events_since(&w,old_len,input);
  correct.push(c);
  incorrect.push(b);
  writes.push(c+b);
  if phase==0{after_first=current_service_edge(&w,arm);}
  assert_eq!(w.cell(7),Some(input));
  assert!(!w.process(1).unwrap().stopped);
 }
 assert_eq!(w.total_compute_inventory(),Some(0));
 assert_eq!(w.process(1).unwrap().credits,0);
 assert_eq!(w.available_compute_pool(),0);
 assert_eq!(w.events().iter().filter(|e|matches!(e.origin,Origin::External)).count(),3);
 assert!(w.process(2).is_none());
 assert!(w.occupied_units()<=128);
 let edits=w.events().iter().filter(|e|matches!(
  (&e.origin,&e.kind),
  (Origin::Internal(1),EventKind::ExecutableEdgePatched{
    node:3,before:4,after:11
  })
 )).count();
 let suppressed=w.events().iter().filter(|e|matches!(
  (&e.origin,&e.kind),
  (Origin::Internal(1),EventKind::NativeCodeEditSuppressed{opcode_tag:6})
 )).count();
 let traps=w.events().iter().filter(|e|matches!(e.kind,EventKind::Trapped(_))).count();
 let t=Trial{arm,history,correct,incorrect,writes,charged:paid,edits,suppressed,
  edge_after_first:after_first,final_edge:current_service_edge(&w,arm),
  stopped:w.process(1).unwrap().stopped,traps,
  final_input:w.cell(7).unwrap(),final_output:w.cell(11)};
 eprintln!("B38_NATIVE {:?}",t);
 t
}
#[test]
fn b38_native_output_readback_can_trigger_a_real_inplace_source_repair(){
 let histories=[(0_i64,17_i64),(17,0),(23,-9),(-9,23)];
 let mut count=0;
 for history in histories{
  let mut cases=Vec::new();
  for arm in [Arm::Readback,Arm::AssumeOk,Arm::ForceFix,
   Arm::PatchDisabled,Arm::StaticCorrect]{
    let result=run(arm,history);
    assert_eq!(result.charged,96);
    assert_eq!(result.traps,0);
    assert!(!result.stopped);
    assert_eq!(result.correct.len(),2);
    assert_eq!(result.incorrect.len(),2);
    assert_eq!(result.writes.len(),2);
    assert_eq!(result.final_input,history.1);
    cases.push(result);
    count+=1;
  }
  let [readback,assumed,forced,disabled,static_correct]=
   [&cases[0],&cases[1],&cases[2],&cases[3],&cases[4]];
  assert_eq!(readback.edits,1,
    "the actual native output mismatch must cause exactly one paid edge patch");
  assert_eq!(readback.final_edge,11);
  assert_eq!(assumed.edits,0);
  assert_eq!(assumed.final_edge,4);
  assert_eq!(forced.edits,1);
  assert_eq!(forced.final_edge,11);
  assert_eq!(disabled.edits,0);
  assert_eq!(disabled.final_edge,4);
  assert!(disabled.suppressed>0);
  assert_eq!(static_correct.edits,0);
  assert_eq!(static_correct.final_edge,3);
  assert!(static_correct.correct.iter().sum::<usize>()>=
   readback.correct.iter().sum::<usize>(),
    "true function evaluation must retain much cheaper immutable baseline");
  if history.0==0{
   assert_eq!(readback.edge_after_first,4,
     "no mismatch exists before input changes: must NOT preemptively repair");
   assert_eq!(readback.incorrect[0],0);
   assert_eq!(readback.correct[0],readback.writes[0]);
  }else{
   assert_eq!(readback.edge_after_first,11,
     "real nonzero first input requires same-actor paid native source repair");
  }
 }
 assert_eq!(count,20);
}
