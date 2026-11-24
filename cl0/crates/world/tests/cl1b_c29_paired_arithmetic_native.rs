//! C29 exact frozen arithmetic trials; all output, credit, and code effects observed.
use cl0_world::{EventKind,Intervention,Op,Origin,Process,World};
use serde_json::{json,Value};
use std::{fs,io::{BufWriter,Write},path::Path};
const CELLS:[i64;11]=[11,40,41,42,43,44,45,46,50,51,60];
fn decode(v:&Value)->Op{
 let kind=v["kind"].as_str().unwrap();
 let mut fields=v.as_object().unwrap().clone();fields.remove("kind");
 let mut outer=serde_json::Map::new();outer.insert(kind.into(),Value::Object(fields));
 serde_json::from_value(Value::Object(outer)).unwrap()
}
fn edge(w:&World,p:usize)->usize{
 match &w.process(1).unwrap().code.nodes[p]{Op::Const{next,..}=>*next,_=>panic!("dispatch not Const")}
}
fn set_pair(w:&mut World,x:i64,y:i64){
 assert!((1..=16).contains(&x)&&(1..=16).contains(&y));
 w.intervene(Intervention::SetCell{key:50,value:x}).unwrap();
 w.intervene(Intervention::SetCell{key:51,value:y}).unwrap();
}
#[test]
fn c29_frozen_paid_paired_arithmetic_native(){
 let root=Path::new(env!("CARGO_MANIFEST_DIR"));
 let m:Value=serde_json::from_slice(&fs::read(root.join("../../../cl1b/studies/CL1B-C29-PAIRED-ARITHMETIC-frozen-80-worlds-v1.json")).unwrap()).unwrap();
 let dir=root.join("../../cl1b-c29-evidence");fs::create_dir_all(&dir).unwrap();
 let credit=m["capacity"]["credits"].as_u64().unwrap();
 let mut rows=Vec::new();
 for case in m["cases"].as_array().unwrap(){
  for arm in m["arms"].as_array().unwrap(){
   let cid=case["id"].as_str().unwrap();let aid=arm["id"].as_str().unwrap();
   let id=format!("{cid}_{aid}");let source=arm["source"].as_str().unwrap();
   let src=&m["sources"][source];
   let initial=src["g"].as_array().unwrap().iter().map(decode).collect::<Vec<_>>();
   let l=|k:&str|src["labels"][k].as_u64().unwrap() as usize;
   let dispatch=l("dispatch");let off=arm["off"].as_bool().unwrap();
   let goal=case["n"].as_u64().unwrap() as usize;
   let mut expected=initial.clone();let mut w=World::with_strict_capacity(16,200,220);
   if off{w=w.with_native_patchnext_disabled();}
   w.load(Process::new(1,initial,credit)).unwrap();
   set_pair(&mut w,case["xy"][0].as_i64().unwrap(),case["xy"][1].as_i64().unwrap());
   let mut out=BufWriter::new(fs::File::create(dir.join(format!("{id}.jsonl"))).unwrap());
   let(mut paid,mut stage,mut prod,mut trials)=(0usize,0usize,0usize,0usize);
   let mut peak=w.occupied_units();let mut probes=Vec::new();let mut edits=Vec::new();let mut measurements=Vec::new();
   let mut mid=false;let mut trial_inputs=[None::<(i64,i64)>;2];
   while stage<goal && w.step(){
    paid+=1;
    let e=w.events().last().unwrap();let p=w.process(1).unwrap();let pc=e.pc;
    assert_eq!(e.origin,Origin::Internal(1));assert!(!e.pool_paid_attempt);
    assert_eq!(p.credits,credit-paid as u64);assert_eq!(e.credits_after,p.credits);
    assert_eq!(w.total_compute_inventory(),Some(p.credits));
    if pc==l("eval"){probes.push(stage);}
    // Loop entry registers contain the actual operands; only observer checks them.
    if pc==l("loop_a")&&trial_inputs[0].is_none(){trial_inputs[0]=Some((p.registers[1],p.registers[2]));}
    if pc==l("loop_b")&&trial_inputs[1].is_none(){trial_inputs[1]=Some((p.registers[2],p.registers[1]));}
    if pc==l("compare"){
     let a=trial_inputs[0].take().unwrap();let b=trial_inputs[1].take().unwrap();
     let ca=w.cell(45).unwrap();let cb=w.cell(46).unwrap();
     assert_eq!(ca,11+3*a.0);assert_eq!(cb,11+3*b.1);
     assert_eq!(w.cell(43),Some(a.0*a.1));assert_eq!(w.cell(44),Some(b.0*b.1));
     if source!="live"{assert_eq!(a,b);assert_eq!(w.cell(40),Some(a.0));assert_eq!(w.cell(41),Some(a.1));}
     measurements.push(json!({"stage":stage,"a_input":a,"b_input":b,"a":ca,"b":cb,"outputs":[w.cell(43),w.cell(44)],"live":[w.cell(50),w.cell(51)]}));
    }
    let mut stage_write=false;let mut trial_a_write=false;
    let kind=match &e.kind{
     EventKind::Executed=>"executed",
     EventKind::CellRead{..}=>"read",
     EventKind::CellWritten{key,value}=>{
      if *key==11{assert_eq!(*value,w.cell(50).unwrap()*w.cell(51).unwrap());prod+=1;}
      if *key==43||*key==44{trials+=1;trial_a_write=*key==43;}
      if *key==60{stage+=1;assert_eq!(*value,stage as i64);assert_eq!(prod,stage);stage_write=true;}
      "write"
     },
     EventKind::NativeCodeEditSuppressed{opcode_tag:6}=>"suppressed",
     other=>panic!("{id} unexpected event {other:?}")
    };
    if matches!(&expected[pc],Op::PatchNext{..}){
     assert_eq!(p.registers[1],dispatch as i64);let target=p.registers[0] as usize;
     assert!(target==l("prod_a")||target==l("prod_b"));
     let before=match &expected[dispatch]{Op::Const{next,..}=>*next,_=>panic!()};
     assert_ne!(before,target);
     if !off{if let Op::Const{next,..}=&mut expected[dispatch]{*next=target;}assert_eq!(kind,"executed");}else{assert_eq!(kind,"suppressed");}
     edits.push(json!([stage,before,target,edge(&w,dispatch),off]));
    }
    assert_eq!(p.code.nodes,expected);assert!(w.occupied_units()<=220);peak=peak.max(w.occupied_units());
    writeln!(out,"{}",json!({"step":paid,"pc":pc,"next_pc":p.pc,"registers":p.registers,"credits":p.credits,"cells":CELLS.map(|k|w.cell(k)),"edge":edge(&w,dispatch),"occupied":w.occupied_units(),"kind":kind})).unwrap();
    if trial_a_write&&!mid&&case["midTrial"].is_array(){
     mid=true;let x=case["midTrial"][0].as_i64().unwrap();let y=case["midTrial"][1].as_i64().unwrap();
     set_pair(&mut w,x,y);writeln!(out,"{}",json!({"change":[paid,stage,x,y,"mid_trial"]})).unwrap();
    }
    if stage_write&&stage<goal{
     if let Some(changes)=case["changes"].as_array(){for ch in changes{
      if ch[0].as_u64()==Some(stage as u64){
       let x=ch[1].as_i64().unwrap();let y=ch[2].as_i64().unwrap();
       set_pair(&mut w,x,y);writeln!(out,"{}",json!({"change":[paid,stage,x,y,"stage"]})).unwrap();
      }
     }}
    }
   }
   assert_eq!(stage,goal);assert_eq!(prod,goal);assert!(!w.process(1).unwrap().stopped);
   assert_eq!(trials,2*measurements.len());
   if let Some(x)=m["calibration_expected"][cid][aid].as_u64(){assert_eq!(paid as u64,x,"{id} calibration");}
   out.flush().unwrap();
   rows.push(json!({"id":id,"case":cid,"arm":aid,"paid":paid,"production":prod,"trials":trials,"probes":probes,"edits":edits,"measurements":measurements,"nodes":expected.len(),"peak_occupied":peak,"cutoff":true}));
  }
 }
 assert_eq!(rows.len(),80);
 fs::write(dir.join("summary.json"),serde_json::to_vec_pretty(&json!({"study":m["study"],"tested_sha":std::env::var("GITHUB_SHA").unwrap_or_else(|_|"local".into()),"rows":rows})).unwrap()).unwrap();
}
