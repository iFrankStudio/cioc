// B56-F1d0 independent native VM micro-price replay:
// 2 monitor routes × all16 four-bit feedback combinations, not paid source creation.
import fs from 'node:fs';
import crypto from 'node:crypto';
import assert from 'node:assert/strict';
import {isDeepStrictEqual} from 'node:util';
import {CIL, founder as b51c} from './b51c_native_paid_search_replay.mjs';

const rust=fs.readFileSync(new URL('../../cl0/crates/world/tests/cl1b_b56_f1d0_monitor_path_calibration.rs',import.meta.url),'utf8');
const f1c=fs.readFileSync(new URL('../../cl0/crates/world/tests/cl1b_b56_f1c_pilot_ordered_trial.rs',import.meta.url),'utf8');
const manifest=fs.readFileSync(new URL('../studies/B56-F1d0-native-monitor-path-price-calibration-manifest-v1.md',import.meta.url));
const manifestSha=crypto.createHash('sha256').update(manifest).digest('hex');
const evidence='cl1b-ci-evidence/b56_f1d0_native_monitor_path_costs.jsonl';
if(!fs.existsSync(evidence))throw Error('B56 F1d0 missing real native source history');
const lines=fs.readFileSync(evidence,'utf8').trimEnd().split(/\r?\n/);
let index=0,steps=0,externals=0,readEvents=0;
function check(expected,where){
  if(index>=lines.length)throw Error('B56 F1d0 truncated '+where);
  const got=JSON.parse(lines[index++]);
  if(!isDeepStrictEqual(expected,got))throw Error('B56 F1d0 FIRST DIVERGENCE '+where+
    ' line '+index+'\nexpected '+JSON.stringify(expected)+'\nnative '+JSON.stringify(got));
}
const meta=JSON.parse(lines[index++]);
if(meta[0]!=='meta'||meta[1]!=='B56-F1d0'||meta[2]!==1||
 meta[4]!==manifestSha||meta[5]!==manifest.length||meta[6]!==32||
 meta[7]!==500||meta[8]!==16||meta[9]!==180||meta[10]!==300||
 (process.env.GITHUB_SHA&&process.env.GITHUB_SHA!==meta[3]))
 throw Error('B56 F1d0 source SHA/manifest/finite-native bounds mismatch');

const src=f1c.split('fn build_two()->Vec<Op>{')[1]?.split('fn build_static(fourth:bool)')[0];
if(!src||!rust.includes('let s=include_str!("cl1b_b56_f1c_pilot_ordered_trial.rs")')||
   !rust.includes('proc.pc=110')||
   !rust.includes('c[110]=Op::Const{dst:2,value:11,next:2}')||
   !rust.includes('c[2]=Op::Write{address:2,value:0,next:if every{164}else{136}}'))
 throw Error('B56 F1d0 source extraction is not original Rust F1c');
const actor=new Map();
for(const m of src.matchAll(/code\[(\d+)\]=Op::([A-Za-z]+)\{([^}]+)\};/g)){
 const attrs=Object.fromEntries([...m[3].matchAll(/(\w+)\s*:\s*(-?\d+)/g)]
  .map(x=>[x[1],Number(x[2])]));
 actor.set(Number(m[1]),{kind:m[2],...attrs});
}
const checkNodes=[98,99,100,101,136,137,138,139,140,141,142,144,145,146,147,148];
for(const i of checkNodes)if(!actor.has(i))throw Error('F1c source node missing '+i);
if(actor.get(148).next!==30||
   b51c[35]?.kind!=='Const'||b51c[35].value!==20||
   b51c[36]?.kind!=='Read'||b51c[36].next!==37)
 throw Error('original B51c generic feedback clone-source provenance changed');
// Cross-check the microbenchmark's actual literal Op source against independently
// source-parsed F1c CIL; the neutral return harness is explicitly separate.
for(const i of checkNodes.filter(x=>x!==148)){
 const x=actor.get(i);
 const nativeText=rust.includes('c['+i+']=Op::'+x.kind+'{'+
  Object.entries(x).filter(([k])=>k!=='kind').map(([k,v])=>k+':'+v).join(',')+'};');
 if(!nativeText)throw Error('F1d0 Rust benchmark node diverges from original F1c at '+i);
}
if(!rust.includes('c[148]=Op::Const{dst:2,value:20,next:166};'))
 throw Error('F1d0 CIL must source-realistically include actor-paid 148.next 30→166');
for(const i of [164,165,166,167]){
 if(!rust.includes('c['+i+']'))throw Error('F1d0 clone destination not represented '+i);
}
function opJson(o){
 return o.kind==='Halt'?'Halt':{
  [o.kind]:Object.fromEntries(Object.entries(o).filter(([k])=>k!=='kind'))
 };
}
function graph(every){
 const c=Array.from({length:168},()=>({kind:'Halt'}));
 c[2]={kind:'Write',address:2,value:0,next:every?164:136};
 c[110]={kind:'Const',dst:2,value:11,next:2};
 for(const n of checkNodes)c[n]={...actor.get(n)};
 c[148].next=166; // actual F1c actor-paid successful PatchNext source outcome
 c[164]={...b51c[35],next:165};
 c[165]={...b51c[36],next:98};
 c[166]={...b51c[35],next:167};
 c[167]={...b51c[36],next:98};
 return c;
}
function ev(e){
 if(e.type==='executed')return ['exec'];
 if(e.type==='read')return ['read',e.key,e.value];
 if(e.type==='write')return ['write',e.key,e.value];
 if(e.type==='patch')return ['patch',e.node,e.before,e.after];
 if(e.type==='suppressed')return ['suppressed',e.tag];
 if(e.type==='halt')return ['halt'];
 if(e.type==='trap')return ['trap',e.message];
 throw Error('Unsupported native microbenchmark event '+JSON.stringify(e));
}
const keys=[11,20,25,28];
const cells=m=>keys.map(k=>m.cell(k));
function step(m,id){
 const r=m.step(id);steps++;
 check(['step',id,m.paid,m.tick,r.pc,m.pc,ev(r.event),[...m.regs],
    m.credits,m.stopped,m.code.length,m.storage(),cells(m)],
  'native paid instruction '+id+'/'+m.paid+'/'+r.pc);
 readEvents+=Number(r.event.type==='read'&&r.event.key===20);
 return r.event;
}
function external(m,id,bit){
 m.external(id,20,bit,'feedback_after_real_service_write');externals++;
 check(['ext',id,m.tick,20,bit,'feedback_after_real_service_write',
   m.credits,m.storage(),cells(m)],'native external bit '+id+'/'+m.tick);
}
const rows=[];
for(let mask=0;mask<16;mask++){
 for(const every of [true,false]){
  const id=rows.length,code=graph(every);
  const m=new CIL(1,-2,'TRUE');
  m.code=code.map(o=>({...o}));
  m.pc=110;m.credits=500;m.maxDataCells=16;
  m.maxCodeNodes=180;m.maxStorage=300;
  check(['case',id,every?'EVERY':'FOURTH',mask,500,code.map(opJson),
    m.storage()],'original source case '+id);
  let reads=0,marked=0;
  const costs=[],paths=[];
  for(let n=0;n<4;n++){
   assert.equal(m.pc,110);
   assert.equal(step(m,id).type,'executed');
   const service=step(m,id);
   assert.equal(service.type,'write');
   assert.equal(service.key,11);
   assert.equal(service.value,0);
   const bit=(mask>>n)&1;
   external(m,id,bit);
   const start=m.paid,pcs=[];
   while(m.pc!==110){
    const pc=m.pc;assert.ok(pcs.length<40);
    const event=step(m,id);
    if(event.type==='read'&&event.key===20)reads++;
    if(event.type==='write'&&event.key===25&&event.value===1)marked++;
    pcs.push(pc);
   }
   const cost=m.paid-start;
   const want=every?(bit===0?6:3):(n===3?(bit===0?18:15):8);
   assert.equal(cost,want,'source-priced monitor actual output path');
   costs.push(cost);paths.push(pcs);
   check(['segment',id,n,bit,cost,costs.reduce((a,b)=>a+b,0),
     reads,marked,m.credits,m.storage(),cells(m)],
     'real paid monitor path '+id+'/'+n);
  }
  const monitor=costs.reduce((a,b)=>a+b,0);
  const result={
   id,policy:every?'EVERY':'FOURTH',mask,
   bits:Array.from({length:4},(_,i)=>(mask>>i)&1),
   paid_including_neutral_harness:m.paid,harness_steps:8,
   monitor_cost:monitor,segment_costs:costs,monitor_pcs:paths,
   feedback_reads:reads,failure_markers:marked,
   unused_paid_credits:m.credits,final_pc:m.pc,final_regs:[...m.regs],
   final_nodes:m.code.length,final_graph:m.code.map(opJson),
   final_cells:cells(m),final_storage:m.storage()
  };
  check(['end',id,result],'native complete full VM world '+id);
  rows.push(result);
 }
}
assert.equal(index,lines.length,'no unchecked CIL events');
const pairs=[];
for(let mask=0;mask<16;mask++){
 const a=rows[mask*2],b=rows[mask*2+1];
 assert.equal(a.monitor_cost,12+3*(4-a.bits.reduce((u,v)=>u+v,0)));
 assert.equal(b.monitor_cost,39+3*(1-b.bits[3]));
 assert.equal(a.feedback_reads,4);assert.equal(b.feedback_reads,1);
 assert.ok(a.monitor_cost<b.monitor_cost);
 pairs.push({mask,feedback:a.bits,every:a.monitor_cost,fourth:b.monitor_cost,
   every_feedback_reads:4,fourth_feedback_reads:1,
   extra_periodic_compute:b.monitor_cost-a.monitor_cost});
}
console.log(JSON.stringify({
 study:'B56-F1d0',source_sha:meta[3],frozen_manifest_sha256:manifestSha,
 source_extracted_from_original_F1c:true,
 direct_native_VM_worlds_replayed:rows.length,
 original_paid_native_instructions_replayed:steps,
 external_feedback_interventions_replayed:externals,
 true_native_feedback_Read20_events_replayed:readEvents,
 monitor_service_segments:128,
 first_divergence:null,
 strict_bounded_world:true,
 actor_paid_monitor_assembly_tested:false,
 all_16_paired_bit_patterns_every_cheaper:true,
 every_range:[Math.min(...rows.filter(r=>r.policy==='EVERY').map(r=>r.monitor_cost)),
              Math.max(...rows.filter(r=>r.policy==='EVERY').map(r=>r.monitor_cost))],
 fourth_range:[Math.min(...rows.filter(r=>r.policy==='FOURTH').map(r=>r.monitor_cost)),
              Math.max(...rows.filter(r=>r.policy==='FOURTH').map(r=>r.monitor_cost))],
 positive_provenance_extra_check_cost_to_make_fourth_cheaper:10,
 b56_f1d_full_trial_completed:false,n56b_met:false,n56c_met:false,
 b3_met:false,b4_met:false,cyberlife_established:false,
 pairs
},null,2));
