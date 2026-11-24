//! C30: native generated executable nodes, exact output and source evolution.
use cl0_world::{EventKind,Intervention,Op,Origin,Process,World};
use serde_json::{json,Value};
use std::{fs,io::{BufWriter,Write},path::Path};
const KEYS:[i64;10]=[11,40,42,43,44,45,46,47,50,51];
fn decode(v:&Value)->Op{
 let mut f=v.as_object().unwrap().clone();let k=f.remove("kind").unwrap().as_str().unwrap().to_owned();
 let mut o=serde_json::Map::new();o.insert(k,Value::Object(f));serde_json::from_value(Value::Object(o)).unwrap()
}
fn set_pair(w:&mut World,x:i64,y:i64){
 assert!((1..=16).contains(&x)&&(1..=16).contains(&y));
 for (key,value) in [(50,x),(51,y)]{w.intervene(Intervention::SetCell{key,value}).unwrap();}
}
fn val(g:&[Op],idx:usize)->i64{match &g[idx]{Op::Const{value,..}=>*value,_=>panic!("not const")}}
fn edge(g:&[Op],idx:usize)->usize{match &g[idx]{Op::Const{next,..}|Op::Add{next,..}|Op::Sub{next,..}|Op::Read{next,..}=>*next,_=>panic!("not supported target")}}
fn binary_steps(x:usize)->usize{(usize::BITS-1-x.leading_zeros()+x.count_ones()) as usize}
#[test]
fn c30_r10_construction_provenance_native(){
 let root=Path::new(env!("CARGO_MANIFEST_DIR"));
 let m:Value=serde_json::from_slice(&fs::read(root.join("../../../cl1b/studies/CL1B-C30-R10-CONSTRUCTION-PROVENANCE-frozen-77-worlds-v1.json")).unwrap()).unwrap();
 let dir=root.join("../../cl1b-c30-r10-evidence");fs::create_dir_all(&dir).unwrap();
 let mut rows=Vec::new();let credit=20000u64;
 let mut jobs=Vec::new();
 for c in m["cases"].as_array().unwrap(){for a in m["arms"].as_array().unwrap(){jobs.push((c.clone(),a.clone()));}}
 for c in m["faults"].as_array().unwrap(){jobs.push((c.clone(),json!({"id":"fault","source":c["source"],"off":false})));}
 for (case,arm) in jobs{
  let cid=case["id"].as_str().unwrap();let aid=arm["id"].as_str().unwrap();let id=format!("{cid}_{aid}");
  let source=arm["source"].as_str().unwrap();let src=&m["sources"][source];let off=arm["off"].as_bool().unwrap();
  let l=|k:&str|src["l"][k].as_u64().unwrap() as usize;
  let guard=src["l"]["flag"].as_u64().map(|n|n as usize);let dispatch=src["l"]["dispatch"].as_u64().map(|n|n as usize);
  let initial=src["g"].as_array().unwrap().iter().map(decode).collect::<Vec<_>>();let initial_len=initial.len();
  let mut expected=initial.clone();let mut w=World::with_strict_capacity(16,200,220);
  if off{w=w.with_native_patchconst_suppression(1,l("activation"),l("flag"));}
  w.load(Process::new(1,initial,credit)).unwrap();set_pair(&mut w,case["xy"][0].as_i64().unwrap(),case["xy"][1].as_i64().unwrap());
  let goal=case["n"].as_u64().unwrap() as usize;
  let mut out=BufWriter::new(fs::File::create(dir.join(format!("{id}.jsonl"))).unwrap());
  let(mut paid,mut prod,mut trial_steps,mut production_steps)=(0usize,0usize,0usize,0usize);
  let mut service_clones=0usize;let mut peak=w.occupied_units();let mut clones=Vec::new();let mut patches=Vec::new();let mut trials=Vec::new();
  let(mut reads,mut fields,mut copies,mut checks,mut graph_peak)=(0usize,0usize,0usize,0usize,0usize);let mut wrong=Value::Null;let mut graph:Option<Vec<Op>>=None;
  while prod<goal {
   let before_regs=w.process(1).unwrap().registers;assert!(w.step(),"{id} premature stop");
   paid+=1;let e=w.events().last().unwrap();let p=w.process(1).unwrap();let pc=e.pc;
   assert_eq!(e.origin,Origin::Internal(1));assert!(!e.pool_paid_attempt);assert_eq!(p.credits,credit-paid as u64);assert_eq!(e.credits_after,p.credits);
   assert_eq!(w.total_compute_inventory(),Some(p.credits));
   let mut produced=false; if ["check_first","check_even","check_odd"].iter().any(|k|src["l"][*k].as_u64()==Some(pc as u64)){checks+=1;}
   let kind=match &e.kind{
    EventKind::GraphCreated{graph,owner}=>{assert_eq!((*graph,*owner),(1,1));"graph_new"},
    EventKind::GraphNodeCopied{graph,owner,source_process,..}=>{assert_eq!((*graph,*owner,*source_process),(1,1,1));"graph_copy"},
    EventKind::GraphFieldRead{graph,owner,..}=>{assert_eq!((*graph,*owner),(1,1));"graph_field"},
    EventKind::Executed=>"executed",EventKind::CellRead{..}=>"read",
    EventKind::CellWritten{key,value}=>{
     if *key==11{let x=w.cell(50).unwrap();let y=w.cell(51).unwrap();if source=="post_check"{assert_ne!(*value,x*y);wrong=json!({"x":x,"y":y,"actual":*value,"expected":x*y});}else{assert_eq!(*value,x*y);}let active=dispatch.map(|i|edge(&expected,i)==l("base")).unwrap_or(false);assert_eq!(service_clones,if active{if source=="r3"{x as usize}else{binary_steps(x as usize)}}else{0});service_clones=0;prod+=1;produced=true;}
     if *key==46{trials.push(json!({"output":w.cell(44).unwrap(),"cost":*value,"x":w.cell(40).unwrap(),"y":w.cell(51).unwrap()}));}
     "write"
    },
    EventKind::NativeCodeEditSuppressed{..}=>"suppressed",
    other=>panic!("{id} unexpected event {other:?}")
   };
   if pc>=initial_len{if edge(&expected,l("out"))==l("prod_out"){production_steps+=1;service_clones+=1;}else{trial_steps+=1;}}
   let mut mutation=Value::Null;
   match expected[pc].clone(){
    Op::GraphNew{dst,..}=>{assert!(graph.is_none());assert_eq!(p.registers[dst],1);assert_eq!(kind,"graph_new");graph=Some(vec![]);mutation=json!(["graph_new",1]);},
    Op::GraphCopyNode{graph:reg,source:sr,dst,..}=>{assert_eq!(before_regs[reg],1);let from=before_regs[sr] as usize;let obj=graph.as_mut().unwrap();let idx=obj.len();obj.push(expected[from].clone());assert_eq!(p.registers[dst],idx as i64);assert_eq!(kind,"graph_copy");assert!(matches!(&e.kind,EventKind::GraphNodeCopied{source_node,new_node,..} if *source_node==from&&*new_node==idx));copies+=1;mutation=json!(["graph_copy",from,idx]);},
    Op::GraphReadField{graph:reg,node,field,..}=>{assert_eq!(before_regs[reg],1);let idx=before_regs[node] as usize;let f=before_regs[field] as usize;let desc=match &graph.as_ref().unwrap()[idx]{Op::Add{dst,lhs,rhs,next}=>[1,*dst as i64,*lhs as i64,*rhs as i64,*next as i64],_=>panic!("unexpected template")};let word=desc[f];assert_eq!(p.registers[field],word);assert!(matches!(&e.kind,EventKind::GraphFieldRead{node,field,word:w,..} if *node==idx&&*field==f&&*w==word));fields+=1;mutation=json!(["graph_field",idx,f,word]);},
    Op::GraphDrop{graph:reg,..}=>{assert_eq!(before_regs[reg],1);assert!(graph.take().is_some());assert_eq!(kind,"executed");mutation=json!(["graph_drop",1]);},
    Op::ReadNext{dst,node,..}=>{let at=before_regs[node] as usize;let to=edge(&expected,at);assert_eq!(p.registers[dst],to as i64);reads+=1;mutation=json!(["read_next",at,to]);},
    Op::CloneNode{dst,source:reg,..}=>{
     let from=before_regs[reg] as usize;let idx=expected.len();if source=="none"{assert_eq!(from,if (idx-initial_len)%2==0{l("parent_add")}else{l("double_src")});}if source=="r3"{assert_eq!(from,l("parent_add"));}assert_eq!(p.registers[dst],idx as i64);
     expected.push(expected[from].clone());clones.push(json!([paid,from,idx]));mutation=json!(["clone",from,idx]);
    },
    Op::PatchNext{node,target,..}=>{
     let at=p.registers[node] as usize;let to=p.registers[target] as usize;
     assert!(at>=initial_len||at==l("special_entry")||at==l("out")||at==l("dispatch"));let before=edge(&expected,at);if at==l("special_entry"){let key=val(&expected,l("input_x"));assert_eq!(to,initial_len+w.cell(key).unwrap() as usize-1);}if at==l("dispatch"){assert!(to==l("base")||to==l("parent"));}
     match &mut expected[at]{Op::Const{next,..}|Op::Add{next,..}|Op::Sub{next,..}|Op::Read{next,..}=>*next=to,_=>panic!()}
     assert_eq!(kind,"executed");patches.push(json!([paid,pc,at,before,to,false]));mutation=json!(["next",at,before,to,false]);
    },
    Op::PatchConst{node,value,..}=>{
     let at=p.registers[node] as usize;let to=p.registers[value];assert!(Some(at)==guard||at==l("base")||at==l("input_x"));let before=val(&expected,at);
     let suppress=off&&pc==l("activation");
     if !suppress{if let Op::Const{value,..}=&mut expected[at]{*value=to;}assert_eq!(kind,"executed");}else{assert_eq!(kind,"suppressed");}
     patches.push(json!([paid,pc,at,before,to,suppress]));mutation=json!(["const",at,before,to,suppress]);
    },_=>{}
   }
   if let Some(obj)=&graph{let real=w.graph_object(1).unwrap();assert_eq!(real.owner,1);assert_eq!(&real.nodes,obj);assert_eq!(w.graph_count(),1);graph_peak=graph_peak.max(1+obj.len());}else{assert_eq!(w.graph_count(),0);}
   assert_eq!(p.code.nodes,expected);assert!(w.occupied_units()<=220);peak=peak.max(w.occupied_units());
   writeln!(out,"{}",json!({"step":paid,"pc":pc,"next_pc":p.pc,"registers":p.registers,"credits":p.credits,"cells":KEYS.map(|k|w.cell(k)),"nodes":expected.len(),"guard":guard.map(|i|val(&expected,i)),"dispatch":dispatch.map(|i|edge(&expected,i)),"out_edge":edge(&expected,l("out")),"occupied":w.occupied_units(),"kind":kind,"mutation":mutation,"graph_units":graph.as_ref().map(|o|1+o.len()).unwrap_or(0)})).unwrap();
   if produced&&prod<goal{if let Some(changes)=case["changes"].as_array(){for ch in changes{if ch[0].as_u64()==Some(prod as u64){
    let x=ch[1].as_i64().unwrap();let y=ch[2].as_i64().unwrap();set_pair(&mut w,x,y);
    writeln!(out,"{}",json!({"change":[paid,prod,x,y]})).unwrap();
   }}}}
  }
  assert_eq!(prod,goal);assert!(!w.process(1).unwrap().stopped);
  let active=guard.map(|i|val(&expected,i));
  assert!(graph.is_none());
  if source=="none"||source=="r3"||source=="post_check"{
   assert_eq!(clones.len(),16);assert_eq!(trials.len(),2);assert_eq!(trial_steps,if source=="r3"{16}else{5});
   assert_eq!(trials[0]["output"],trials[1]["output"]);assert_eq!(trials[0]["cost"],60);assert_eq!(trials[1]["cost"],if source=="r3"{31}else{20});
   assert_eq!(active,Some(if off{0}else{1}));assert_eq!(production_steps>0,!off);
   assert_eq!(edge(&expected,l("dispatch")),if off{l("parent")}else{l("base")});
   if source!="r3"{assert_eq!((reads,fields,copies,checks,graph_peak),(16,8,2,16,3));}
   if source=="post_check"{assert_eq!(wrong,json!({"x":3,"y":7,"actual":28,"expected":21}));}else{assert!(wrong.is_null());}
  }else if source=="plain"{assert!(trials.is_empty()&&clones.is_empty());}
  else{assert_eq!(active,Some(0));assert!(trials.is_empty());assert_eq!(production_steps,0);assert!(wrong.is_null());}

  if let Some(x)=m["calibration_expected"][cid][aid].as_u64(){assert_eq!(paid as u64,x,"{id} calibration");}
  out.flush().unwrap();
  rows.push(json!({"id":id,"case":cid,"cohort":case["cohort"],"arm":aid,"paid":paid,"production":prod,"initial":initial_len,"nodes":expected.len(),"peak_occupied":peak,"clones":clones,"patches":patches,"trials":trials,"active":active,"clone_trial_steps":trial_steps,"clone_production_steps":production_steps,"verified_reads":reads,"field_reads":fields,"graph_copies":copies,"selection_checks":checks,"graph_peak":graph_peak,"wrong":wrong}));
 }
 assert_eq!(rows.len(),77);
 fs::write(dir.join("summary.json"),serde_json::to_vec_pretty(&json!({"study":m["study"],"tested_sha":std::env::var("GITHUB_SHA").unwrap_or_else(|_|"local".into()),"rows":rows})).unwrap()).unwrap();
}
