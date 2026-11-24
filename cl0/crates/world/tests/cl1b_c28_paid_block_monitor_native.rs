//! C28: exact frozen paid block metrology; external equal service cutoff.
use cl0_world::{EventKind,Intervention,Op,Origin,Process,World};
use serde_json::{json,Value};
use std::{fs,io::{BufWriter,Write},path::Path};
const CELLS:[i64;6]=[11,41,44,45,46,47];
fn decode(v:&Value)->Op{
 let kind=v["kind"].as_str().unwrap();
 if kind=="Halt"{return Op::Halt;}
 let mut fields=v.as_object().unwrap().clone();fields.remove("kind");
 let mut outer=serde_json::Map::new();outer.insert(kind.into(),Value::Object(fields));
 serde_json::from_value(Value::Object(outer)).unwrap()
}
fn edge(w:&World,p:usize)->usize{
 match &w.process(1).unwrap().code.nodes[p]{Op::Const{next,..}=>*next,_=>panic!("dispatch not Const")}
}
#[test]
fn c28_paid_block_monitor_frozen_native(){
 let root=Path::new(env!("CARGO_MANIFEST_DIR"));
 let m:Value=serde_json::from_slice(&fs::read(root.join("../../../cl1b/studies/CL1B-C28-PAID-BLOCK-MONITOR-frozen-60-worlds-v1.json")).unwrap()).unwrap();
 let dir=root.join("../../cl1b-c28-evidence");fs::create_dir_all(&dir).unwrap();
 let credit=m["capacity"]["credits"].as_u64().unwrap();
 let mut rows=Vec::new();
 for case in m["cases"].as_array().unwrap(){
  for (index,arm) in m["arms"].as_array().unwrap().iter().enumerate(){
   let id=format!("{}_{}",case["id"].as_str().unwrap(),arm["id"].as_str().unwrap());
   let src=&m["sources"][arm["source"].as_str().unwrap()];
   let initial=src["nodes"].as_array().unwrap().iter().map(decode).collect::<Vec<_>>();
   let old=src["old_pc"].as_array().unwrap().iter().map(|v|v.as_u64().unwrap() as usize).collect::<Vec<_>>();
   let dispatch=src["dispatch"].as_u64().unwrap() as usize;
   let off=arm["off"].as_bool().unwrap();
   let goal=case["n"].as_u64().unwrap() as usize;
   let mut expected=initial.clone();
   let mut w=World::with_strict_capacity(16,500,520);
   if off{w=w.with_native_patchnext_disabled();}
   w.load(Process::new(1,initial,credit)).unwrap();
   w.intervene(Intervention::SetCell{key:50,value:case["mode"].as_i64().unwrap()}).unwrap();
   let mut out=BufWriter::new(fs::File::create(dir.join(format!("{id}.jsonl"))).unwrap());
   let mut paid=0usize;let mut stage=0usize;let mut prod=0usize;let mut trials=0usize;
   let mut peak=w.occupied_units();let mut probes=Vec::new();let mut edits=Vec::new();
   while stage<goal && w.step(){
    paid+=1;
    let e=w.events().last().unwrap();let p=w.process(1).unwrap();
    assert_eq!(e.origin,Origin::Internal(1));assert!(!e.pool_paid_attempt);
    assert_eq!(p.credits,credit-paid as u64);assert_eq!(e.credits_after,p.credits);
    assert_eq!(w.total_compute_inventory(),Some(p.credits));
    let pc=e.pc;let op=old[pc];if op==0{probes.push(stage);}
    let mut stage_write=false;
    let kind=match &e.kind{
     EventKind::Executed=>"executed",
     EventKind::CellRead{..}=>"read",
     EventKind::CellWritten{key,value}=>{
      if *key==11{assert_eq!(*value,1);if op==12||op==42{trials+=1;}else{prod+=1;}}
      if *key==45{stage+=1;assert_eq!(*value,stage as i64);assert_eq!(prod,stage);stage_write=true;}
      "write"
     },
     EventKind::NativeCodeEditSuppressed{opcode_tag:6}=>"suppressed",
     other=>panic!("{id} unexpected event {other:?}")
    };
    if op==95||op==169{
     assert_eq!(p.registers[1],dispatch as i64);
     let target=p.registers[0] as usize;
     assert!(src["targets"].as_array().unwrap().iter().any(|v|v.as_u64()==Some(target as u64)));
     let before=match &expected[dispatch]{Op::Const{next,..}=>*next,_=>panic!()};
     if !off{if let Op::Const{next,..}=&mut expected[dispatch]{*next=target;}assert_eq!(kind,"executed");}else{assert_eq!(kind,"suppressed");}
     edits.push(json!([stage,old[before],old[target],old[edge(&w,dispatch)],off]));
    }
    assert_eq!(p.code.nodes,expected);assert!(w.occupied_units()<=520);peak=peak.max(w.occupied_units());
    let row=json!({"step":paid,"pc":pc,"next_pc":p.pc,"registers":p.registers,"credits":p.credits,"cells":CELLS.map(|k|w.cell(k)),"mode":w.cell(50),"edge":edge(&w,dispatch),"occupied":w.occupied_units(),"kind":kind});
    writeln!(out,"{}",row).unwrap();
    if stage_write && stage<goal{
     for ch in case["changes"].as_array().unwrap(){
      if ch[0].as_u64()==Some(stage as u64){
       let value=ch[1].as_i64().unwrap();
       w.intervene(Intervention::SetCell{key:50,value}).unwrap();
       writeln!(out,"{}",json!({"change":[paid,stage,value]})).unwrap();
      }
     }
    }
   }
   assert_eq!(stage,goal,"{id} did not complete");assert_eq!(prod,goal);
   assert!(!w.process(1).unwrap().stopped);
   if let Some(x)=m["calibration_expected"][case["id"].as_str().unwrap()][index].as_u64(){assert_eq!(paid as u64,x,"{id} calibration");}
   out.flush().unwrap();
   rows.push(json!({"id":id,"case":case["id"],"arm":arm["id"],"paid":paid,"production":prod,"trials":trials,"probes":probes,"edits":edits,"nodes":expected.len(),"peak_occupied":peak,"cutoff":true}));
  }
 }
 assert_eq!(rows.len(),60);
 fs::write(dir.join("summary.json"),serde_json::to_vec_pretty(&json!({"study":m["study"],"tested_sha":std::env::var("GITHUB_SHA").unwrap_or_else(|_|"local".into()),"rows":rows})).unwrap()).unwrap();
}
