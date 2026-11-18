// B56-F1d1a: independent source-extracted entire native microcalibration
// CIL reexecution. Direct preassembled sensors, NOT B3/B4 or self-invention.
import fs from 'node:fs';
import crypto from 'node:crypto';
import assert from 'node:assert/strict';
import {isDeepStrictEqual} from 'node:util';
import {CIL} from './b51c_native_paid_search_replay.mjs';

const rust=fs.readFileSync(new URL('../../cl0/crates/world/tests/cl1b_b56_f1d0_monitor_path_calibration.rs',import.meta.url),'utf8');
const b53=fs.readFileSync(new URL('../../cl0/crates/world/tests/cl1b_b53_native_response_correlation.rs',import.meta.url),'utf8');
const f1c=fs.readFileSync(new URL('../../cl0/crates/world/tests/cl1b_b56_f1c_pilot_ordered_trial.rs',import.meta.url),'utf8');
const manifest=fs.readFileSync(new URL('../studies/B56-F1d1a-native-correlated-receipt-price-microcalibration-manifest-v1.md',import.meta.url));
const manifestSha=crypto.createHash('sha256').update(manifest).digest('hex');
const log='cl1b-ci-evidence/b56_f1d1a_native_correlated_receipt_costs.jsonl';
if(!fs.existsSync(log))throw Error('F1d1a missing same-source native CIL history: cannot certify');
const lines=fs.readFileSync(log,'utf8').trimEnd().split(/\r?\n/);
let index=0,steps=0,externals=0,feedbackReads=0,receiptRejections=0;
function check(expected,context){
  if(index>=lines.length)throw Error('B56 F1d1a truncated before '+context);
  const observed=JSON.parse(lines[index++]);
  if(!isDeepStrictEqual(observed,expected)){
    throw Error('B56 F1d1a FIRST CIL DIVERGENCE '+context+' row '+index+
      '\nindependently reexecuted '+JSON.stringify(expected)+
      '\nnative original '+JSON.stringify(observed));
  }
}
const meta=JSON.parse(lines[index++]);
assert.deepEqual(meta.slice(0,3),['f1d1_meta','B56-F1d1a',1]);
assert.equal(meta[4],manifestSha);
assert.equal(meta[5],manifest.length);
assert.deepEqual(meta.slice(6),[128,500,16,180,300]);
if(process.env.GITHUB_SHA)assert.equal(meta[3],process.env.GITHUB_SHA,'exact original source SHA');
if(!rust.includes('fn f1d1_source(every:bool)->Vec<Op>')||
   !rust.includes('with_strict_capacity(16,180,300)')||
   !rust.includes('proc.pc=70;')||
   !rust.includes('let mut c=vec![Op::Halt;168];'))
 throw Error('F1d1a native implementation resource/protocol drift');
for(const s of [
 'Op::Const{dst:2,value:26,next:107}',
 'Op::Read{dst:1,address:2,next:108}',
 'Op::Branch{test:1,nonzero:121,zero:112}',
 'Op::Branch{test:1,nonzero:121,zero:118}',
 'Op::Read{dst:3,address:2,next:120}',
]){
 if(!b53.includes(s)||!rust.includes('c['+({ 
 'Op::Const{dst:2,value:26,next:107}':106,
 'Op::Read{dst:1,address:2,next:108}':107,
 'Op::Branch{test:1,nonzero:121,zero:112}':111,
 'Op::Branch{test:1,nonzero:121,zero:118}':117,
 'Op::Read{dst:3,address:2,next:120}':119,
 }[s])+']='+s+';'))
  throw Error('Original B53 paid nonce comparison circuit changed '+s);
}
for(const s of [
 'code[136]=Op::Const{dst:2,value:28,next:137}',
 'code[144]=Op::Branch{test:3,nonzero:110,zero:145}',
 'code[147]=Op::Write{address:2,value:1,next:148}',
])if(!f1c.includes(s))throw Error('F1c source-provenance invariant changed: '+s);
const block=rust.split('fn f1d1_source(every:bool)->Vec<Op>{')[1]?.split('\nfn f1d1_cells(')[0];
if(!block)throw Error('F1d1a source graph block missing');
const sourceOps=new Map();
for(const match of block.matchAll(/c\[(\d+)\]=Op::([A-Z][A-Za-z]+)\{([^{}\n]*)\};/g)){
 const attrs=Object.fromEntries([...match[3].matchAll(/(\w+)\s*:\s*(-?\d+)/g)]
  .map(x=>[x[1],Number(x[2])]));
 const op={kind:match[2],...attrs};
 if(sourceOps.has(Number(match[1])))throw Error('duplicate CIL op '+match[1]);
 sourceOps.set(Number(match[1]),op);
}
if(!block.includes('c[2]=Op::Write{address:2,value:0,next:if every{106}else{136}};')||
   !block.includes('c[148]=Op::Const{dst:2,value:20,next:106};')||
   !block.includes('c[136]=Op::Const{dst:2,value:40,next:137};')||
   !block.includes('c[120]=Op::Branch{test:3,nonzero:70,zero:99};')||
   !block.includes('c[123]=Op::Write{address:2,value:3,next:70};')||
   sourceOps.size<35)
 throw Error('F1d1a paid source extraction/aliasing/terminal drift '+sourceOps.size);
function graph(every){
 const code=Array.from({length:168},()=>({kind:'Halt'}));
 for(const [i,op] of sourceOps)code[i]={...op};
 code[2]={kind:'Write',address:2,value:0,next:every?106:136};
 return code;
}
const opJson=op=>op.kind==='Halt'?'Halt':{
 [op.kind]:Object.fromEntries(Object.entries(op).filter(([k])=>k!=='kind'))
};
const ev=e=>{
 switch(e.type){
  case 'executed':return ['exec'];
  case 'read':return ['read',e.key,e.value];
  case 'write':return ['write',e.key,e.value];
  case 'patch':return ['patch',e.node,e.before,e.after];
  case 'halt':return ['halt'];
  case 'trap':return ['trap',e.message];
  case 'suppressed':return ['suppressed',e.tag];
  default:throw Error('Unrecognized native CIL event '+JSON.stringify(e));
 }
};
const keys=[11,20,25,26,27,28,29,40];
const cells=m=>keys.map(k=>m.cell(k));
function paid(m,id){
 const r=m.step(id);
 steps++;
 check(['f1d1_step',id,m.paid,m.tick,r.pc,m.pc,ev(r.event),[...m.regs],
   m.credits,m.stopped,m.code.length,m.storage(),cells(m)],
   'native CIL step '+id+'/'+m.paid+'/'+r.pc);
 feedbackReads+=Number(r.event.type==='read'&&r.event.key===20);
 receiptRejections+=Number(r.event.type==='write'&&r.event.key===29&&r.event.value===1);
 return r.event;
}
function packetCell(m,id,n,key,value){
 m.external(id,key,value,'packet_after_real_paid_write');
 externals++;
 check(['f1d1_ext',id,n,m.tick,key,value,m.credits,m.storage(),cells(m)],
   'external receipt cell '+id+'/'+n+'/'+key);
}
const rows=[];
for(let mode=0;mode<4;mode++)for(let mask=0;mask<16;mask++)for(const every of [true,false]){
 const id=rows.length;
 const code=graph(every);
 const m=new CIL(1,-2,'TRUE');
 m.code=code.map(o=>({...o}));
 m.pc=70;m.credits=500;m.maxDataCells=16;m.maxCodeNodes=180;m.maxStorage=300;
 check(['f1d1_case',id,every?'EVERY':'FOURTH',mode,mask,
   500,16,180,300,code.map(opJson),m.storage()], 'native source case '+id);
 let reads=0,rejects=0,forgedFalse=0,nonceWrites=0,serviceWrites=0;
 const costs=[];
 for(let n=0;n<4;n++){
  assert.equal(m.pc,70,'continuous PID1 control returned to same source entry');
  const serviceStart=m.paid;
  for(let j=0;j<8;j++){
   const e=paid(m,id);
   if(j===5){
    assert.deepEqual([e.type,e.key,e.value],['write',26,n+1]);
    nonceWrites++;
   }
   if(j===7){
    assert.deepEqual([e.type,e.key,e.value],['write',11,0]);
    serviceWrites++;
   }
  }
  assert.equal(m.paid-serviceStart,8,'real paid nonce+service output');
  const nonce=n+1,truth=(mask>>n)&1;
  const bit=mode===0?truth:1;
  const echo=mode===1?nonce-1:nonce;
  const commit=mode===1||mode===2?nonce-1:nonce;
  for(const [k,v] of [[20,bit],[27,echo],[28,commit]]){
   packetCell(m,id,n,k,v);
  }
  const before=m.paid,pcs=[];
  let sampleReads=0,sampleRejects=0;
  while(m.pc!==70){
   const pc=m.pc;
   assert.ok(pcs.length<35,'bounded original CIL program');
   const e=paid(m,id);
   if(e.type==='read'&&e.key===20){sampleReads++;reads++;}
   if(e.type==='write'&&e.key===29&&e.value===1){sampleRejects++;rejects++;}
   pcs.push(pc);
  }
  const sampled=every||n===3;
  const prefix=every?0:n===3?12:8;
  const checkCost=!sampled?0:mode===0?15+(truth===0?3:0):mode===1?9:15;
  const price=prefix+checkCost;
  assert.equal(m.paid-before,price,'charged native monitor-price loop');
  assert.equal(sampleReads,Number(sampled&&(mode===0||mode===3)));
  assert.equal(sampleRejects,Number(sampled&&(mode===1||mode===2)));
  if(sampled&&mode===3&&truth===0)forgedFalse++;
  costs.push(price);
  check(['f1d1_segment',id,n,truth,bit,nonce,echo,commit,
   price,sampleReads,sampleRejects,pcs,m.paid,m.credits,cells(m)],
   'native path and counter '+id+'/'+n);
 }
 const monitor=costs.reduce((a,b)=>a+b,0);
 const result={
  id,policy:every?'EVERY':'FOURTH',mode,truth_mask:mask,
  monitor_cost:monitor,shared_nonce_and_writer_cost:32,
  total_paid:m.paid,actual_paid_read20:reads,
  paid_reject_markers29:rejects,
  forged_current_false_success_accepted:forgedFalse,
  segment_costs:costs,nonce_writes:nonceWrites,
  service_writes:serviceWrites,end_pc:m.pc,
  unused_credits:m.credits,code_nodes:m.code.length,
  occupied_storage:m.storage(),cells:cells(m)
 };
 check(['f1d1_end',id,result],'native complete same-PID world '+id);
 rows.push(result);
}
assert.equal(index,lines.length,'no unchecked original native event rows');
for(let mode=0;mode<4;mode++)for(let mask=0;mask<16;mask++){
 const i=(mode*16+mask)*2;
 const every=rows[i],fourth=rows[i+1];
 assert.equal(every.actual_paid_read20,mode===0||mode===3?4:0);
 assert.equal(fourth.actual_paid_read20,mode===0||mode===3?1:0);
 assert.equal(every.paid_reject_markers29,mode===1||mode===2?4:0);
 assert.equal(fourth.paid_reject_markers29,mode===1||mode===2?1:0);
 if(mode===1)assert.ok(every.monitor_cost<fourth.monitor_cost);
 else assert.ok(every.monitor_cost>fourth.monitor_cost);
}
console.log(JSON.stringify({
 study:'B56-F1d1a',kind:'preassembled_native_vm_resource_and_semantic_gate',
 source_sha:meta[3],preregistered_manifest_sha256:manifestSha,
 original_native_worlds_entirely_reexecuted:rows.length,
 native_paid_CIL_steps_independently_reexecuted:steps,
 external_receipt_interventions_independently_reexecuted:externals,
 original_native_feedback_bit_reads_reexecuted:feedbackReads,
 native_rejection_marker_writes_reexecuted:receiptRejections,
 four_service_output_segments:rows.length*4,
 source_extracted_from_current_Rust_and_B53_F1c:true,
 exact_first_divergence:null,
 conditional_cost_tradeoff:rows.filter(v=>v.mode===0||v.mode===3).length,
 old_replay_price_reverses:16,
 signal_semantic_forgery_not_authenticatable:true,
 useful_actor_policy_selection:false,
 strict_n56a_b_c:false,b3_met:false,b4_met:false,cyberlife:false
},null,2));
