//! C33: original native recurrence, paid reuse activation and explicit boundary falsifiers.
use cl0_world::{EventKind,Intervention,Op,Origin,Process,World};
use serde_json::{json,Value};
use std::{fs,io::{BufWriter,Write},path::Path};
const KEYS:[i64;11]=[11,12,41,42,43,44,50,51,52,53,58];
fn decode(v:&Value)->Op{let mut f=v.as_object().unwrap().clone();let k=f.remove("kind").unwrap().as_str().unwrap().to_owned();let mut o=serde_json::Map::new();o.insert(k,Value::Object(f));serde_json::from_value(Value::Object(o)).unwrap()}
fn set_inputs(w:&mut World,q:&[i64]){assert_eq!(q.len(),4);for(i,&v)in q.iter().enumerate(){assert!((1..=16).contains(&v));w.intervene(Intervention::SetCell{key:50+i as i64,value:v}).unwrap();}}
fn edge(g:&[Op],i:usize)->usize{match &g[i]{Op::Const{next,..}|Op::Add{next,..}=>*next,_=>panic!("edge")}}
fn rec(a:i64,b:i64,k:usize)->i64{(0..k).fold(b,|z,_|a*z+b)}
fn depth(x:usize)->usize{(usize::BITS-1-x.leading_zeros()+x.count_ones()) as usize}
#[test]
fn c33_r1_native_reuse(){
 let root=Path::new(env!("CARGO_MANIFEST_DIR"));let m:Value=serde_json::from_slice(&fs::read(root.join("../../../cl1b/studies/CL1B-C33-R1-NATIVE-REUSE-frozen-82-worlds-v1.json")).unwrap()).unwrap();
 let dir=root.join("../../cl1b-c33-r1-evidence");fs::create_dir_all(&dir).unwrap();let mut rows=vec![];let credit=200000u64;
 let mut jobs=vec![];for c in m["cases"].as_array().unwrap(){for a in m["arms"].as_array().unwrap(){jobs.push((c.clone(),a.clone()));}}
 for c in m["faults"].as_array().unwrap(){jobs.push((c.clone(),json!({"id":"fault","source":c["source"],"off":false}))); }
 for (c,a) in jobs{
  let cid=c["id"].as_str().unwrap();let aid=a["id"].as_str().unwrap();let id=format!("{cid}_{aid}");let source=a["source"].as_str().unwrap();let off=a["off"].as_bool().unwrap();let fault=aid=="fault";let src=&m["sources"][source];let l=|key:&str|src["l"][key].as_u64().unwrap() as usize;
  let mut g=src["g"].as_array().unwrap().iter().map(decode).collect::<Vec<_>>();let initial=g.len();let mut w=World::with_strict_capacity(16,200,220);if off{w=w.with_native_patchconst_suppression(1,l("activate"),l("flag"));}
  w.load(Process::new(1,g.clone(),credit)).unwrap();let mut q=c["xy"].as_array().unwrap().iter().map(|v|v.as_i64().unwrap()).collect::<Vec<_>>();set_inputs(&mut w,&q);let k=c["k"].as_u64().unwrap() as usize;w.intervene(Intervention::SetCell{key:58,value:k as i64}).unwrap();
  let goal=c["n"].as_u64().unwrap() as usize;let mut trace=BufWriter::new(fs::File::create(dir.join(format!("{id}.jsonl"))).unwrap());
  let(mut paid,mut prod,mut kernels,mut service_kernels,mut clone_steps,mut service_clones,mut reused_count)=(0usize,0usize,0usize,0usize,0usize,0usize,0usize);let mut first=None;let mut reused=false;let mut expected_entry=0;let mut peak=w.occupied_units();let mut clones=vec![];let mut activations=vec![];let mut wrong=Value::Null;
  while prod<goal{
   let before=w.process(1).unwrap();let r=before.registers;let pc=before.pc;
   if pc==l("batch"){first=None;reused=false;}if pc==l("reuse"){reused=true;}
   if pc==l("kernel_dispatch"){let i=if first.is_none(){0}else{2};assert_eq!(r[1],q[i]);assert_eq!(r[2],rec(q[i],q[i+1],service_kernels%k));expected_entry=initial+r[1] as usize-1;service_kernels+=1;kernels+=1;}
   if pc>=initial{clone_steps+=1;service_clones+=1;}
   assert!(w.step(),"{id} stopped");paid+=1;let e=w.events().last().unwrap();let p=w.process(1).unwrap();assert_eq!(e.pc,pc);assert_eq!(e.origin,Origin::Internal(1));assert_eq!(p.credits,credit-paid as u64);assert_eq!(e.credits_after,p.credits);assert_eq!(w.total_compute_inventory(),Some(p.credits));
   let mut produced=false;let kind=match &e.kind{
    EventKind::Executed=>"executed",EventKind::CellRead{..}=>"read",EventKind::NativeCodeEditSuppressed{..}=>"suppressed",
    EventKind::CellWritten{key,value}=>{if *key==11{assert_eq!(*value,rec(q[0],q[1],k));assert!(first.is_none());assert_eq!(service_kernels,k);first=Some(*value);}
    if *key==12{let want=rec(q[2],q[3],k);assert_eq!(first,Some(rec(q[0],q[1],k)));if fault{assert_ne!(*value,want);wrong=json!({"actual":value,"expected":want});assert!(reused);}else{assert_eq!(*value,want);assert_eq!(reused,edge(&g,l("gate"))==l("check")&&q[0]==q[2]&&q[1]==q[3]);}
    assert_eq!(service_kernels,k*(if reused{1}else{2}));assert_eq!(service_clones,if source=="plain"{0}else{k*(depth(q[0] as usize)+if reused{0}else{depth(q[2] as usize)})});if reused{reused_count+=1;}service_kernels=0;service_clones=0;prod+=1;produced=true;}"write"},
    other=>panic!("{id}: {other:?}")};
   let mut mutation=Value::Null;
   match g[pc].clone(){
    Op::CloneNode{dst,source:sr,..}=>{let from=r[sr] as usize;let idx=g.len();assert_eq!(from,if (idx-initial)%2==0{l("parent_add")}else{l("double_src")});assert_eq!(p.registers[dst],idx as i64);g.push(g[from].clone());clones.push(json!([paid,from,idx]));mutation=json!(["clone",from,idx]);},
    Op::PatchNext{node,target,..}=>{let at=r[node] as usize;let to=r[target] as usize;let old=edge(&g,at);if at>=initial{let x=at-initial+1;assert_eq!(to,if x==1{l("out")}else if x%2==1{at-1}else{initial+x/2-1});}else if at==l("special_entry"){assert_eq!(to,expected_entry);}else if at==l("kernel_dispatch"){assert_eq!(to,l("base"));}else if at==l("return_route"){assert_eq!(to,if first.is_none(){l("first_result")}else{l("second_result")});}else if at==l("decision"){assert_eq!(to,l("gate"));}else if at==l("gate"){assert_eq!(to,l("check"));assert!(!off);}else{panic!("unexpected patch");}
    match &mut g[at]{Op::Const{next,..}|Op::Add{next,..}=>*next=to,_=>panic!("target")};assert_eq!(kind,"executed");mutation=json!(["next",at,old,to,false]);},
    Op::PatchConst{node,value,..}=>{let at=r[node] as usize;let to=r[value];let suppress=off&&pc==l("activate");if at==l("flag"){assert_eq!(pc,l("activate"));assert_eq!(to,1);assert_eq!(q[0],q[2]);assert_eq!(q[1],q[3]);assert!(activations.is_empty());activations.push(json!([paid,prod,suppress]));}else{assert_eq!(at,l("base"));assert_eq!(to,initial as i64-1);}let old=match &mut g[at]{Op::Const{value,..}=>{let old=*value;if !suppress{*value=to;}old},_=>panic!("const")};assert_eq!(kind,if suppress{"suppressed"}else{"executed"});mutation=json!(["const",at,old,to,suppress]);},_=>{}
   }
   assert_eq!(p.code.nodes,g);assert_eq!(w.graph_count(),0);peak=peak.max(w.occupied_units());assert!(peak<=220);
   writeln!(trace,"{}",json!({"step":paid,"pc":pc,"next_pc":p.pc,"registers":p.registers,"credits":p.credits,"cells":KEYS.map(|key|w.cell(key)),"nodes":g.len(),"gate":edge(&g,l("gate")),"occupied":w.occupied_units(),"kind":kind,"mutation":mutation})).unwrap();
   if produced&&prod<goal{for ch in c["changes"].as_array().unwrap(){if ch[0].as_u64()==Some(prod as u64){q=ch.as_array().unwrap()[1..].iter().map(|v|v.as_i64().unwrap()).collect();set_inputs(&mut w,&q);writeln!(trace,"{}",json!({"change":[paid,prod,q]})).unwrap();}}}
  }
  assert_eq!(prod,goal);assert_eq!(clones.len(),if source=="plain"{0}else{16});assert_eq!(wrong.is_null(),!fault);if let Some(cost)=m["calibration_expected"][cid][aid].as_u64(){assert_eq!(paid as u64,cost);}
  trace.flush().unwrap();rows.push(json!({"id":id,"case":cid,"cohort":c["cohort"],"arm":aid,"k":k,"paid":paid,"batches":prod,"kernels":kernels,"clone_steps":clone_steps,"reused":reused_count,"initial":initial,"nodes":g.len(),"peak":peak,"activations":activations,"clones":clones,"wrong":wrong}));
 }
 assert_eq!(rows.len(),82);fs::write(dir.join("summary.json"),serde_json::to_vec_pretty(&json!({"study":m["study"],"tested_sha":std::env::var("GITHUB_SHA").unwrap_or_else(|_|"local".into()),"rows":rows})).unwrap()).unwrap();
}
