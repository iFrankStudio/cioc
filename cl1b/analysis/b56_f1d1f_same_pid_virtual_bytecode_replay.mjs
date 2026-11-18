#!/usr/bin/env node
// B56-F1d1f: independent per-instruction CIL VM for paid same-PID
// TWO-opcode data-program revision. Exact source and native evidence required.
// No evidence of new opcode synthesis, generative novelty, or cyberlife.
import fs from 'node:fs';
import crypto from 'node:crypto';
import assert from 'node:assert/strict';
import {isDeepStrictEqual} from 'node:util';

const source=fs.readFileSync(new URL('../../cl0/crates/world/tests/cl1b_b56_f1d1f_same_pid_virtual_bytecode.rs',import.meta.url),'utf8');
const manifest=fs.readFileSync(new URL('../studies/B56-F1d1f-paid-same-PID-data-bytecode-feedback-switch-manifest-v1.md',import.meta.url));
const hash=b=>crypto.createHash('sha256').update(b).digest('hex');
const maker=source.split('fn founder()->Vec<Op>{')[1]?.split('#[derive(Copy,Clone)]')[0];
assert.ok(maker&&maker.includes('vec!['),'founder exact source extraction unavailable');
const matches=[...maker.matchAll(/Op::([A-Za-z]+)(?:\{([^}]+)\})?,\s*\/\/\s*(\d+)/g)];
assert.equal(matches.length,34,'frozen F1d1f source node count');
function op(kind,attrs={}){return kind==='Halt'?'Halt':{[kind]:attrs}}
const code=matches.map((m,i)=>{
 assert.equal(Number(m[3]),i,'Rust source comments / PCs not contiguous');
 const attrs=Object.fromEntries([...((m[2]||'').matchAll(/(\w+):(-?\d+)/g))]
  .map(x=>[x[1],Number(x[2])]));
 return op(m[1],attrs);
});
assert.deepEqual(code[9],op('Branch',{test:3,nonzero:10,zero:13}));
assert.deepEqual(code[21],op('Branch',{test:3,nonzero:22,zero:25}));
assert.deepEqual(code[27],op('Write',{address:2,value:3,next:22}));
assert.equal(code.length,34);
assert.ok(code.every(node=>!node.GraphLaunch&&!node.GraphEncode),
 'new source unexpectedly contains a process-launch or code encoder');
const file='cl1b-ci-evidence/b56_f1d1f_same_pid_virtual_opcode_steps.jsonl';
if(!fs.existsSync(file))throw Error('F1d1f original same-SHA native PID1 instruction trace missing');
const lines=fs.readFileSync(file,'utf8').trimEnd().split(/\r?\n/);
let idx=0,paid=0,interventions=0,actualProgramUpdates=0,actualFeedbackReads=0,serviceWrites=0;
function match(expected,why){
 if(idx>=lines.length)throw Error('F1d1f native event truncation at '+why);
 const found=JSON.parse(lines[idx++]);
 if(!isDeepStrictEqual(found,expected))throw Error('F1d1f first divergence line '+idx+' at '+why+
   '\nEXPECTED '+JSON.stringify(expected)+'\nNATIVE   '+JSON.stringify(found));
}
const meta=JSON.parse(lines[idx++]);
assert.deepEqual(meta.slice(0,3),['meta','B56-F1d1f',1]);
if(process.env.GITHUB_SHA)assert.equal(meta[3],process.env.GITHUB_SHA);
assert.equal(meta[4],hash(manifest));
assert.equal(meta[5],manifest.length);
assert.deepEqual(meta.slice(6),[12,250,16,180,300]);
const keys=[7,11,20,30,31];
const cases=[];
function snapshot(w){return [w.pc,[...w.regs],w.credits,w.stopped,code]}
function cells(w){return keys.map(key=>w.data.has(key)?w.data.get(key):null)}
function storage(w){return code.length+1+w.data.size}
for(const seed of [0,1]){
 for(const task of ['IDENTITY','NEGATE']){
  for(const signal of ['TRUTH','BLIND_SUCCESS','INVERTED']){
   const id=cases.length,initial=[5,7][seed],held=[11,-7][seed];
   const w={pc:0,regs:[0,0,0,0],credits:250,stopped:false,tick:0,
    data:new Map(),peak:35,outputs:[],feedbackReads:0,modeUpdates:0,
    firstTruth:null,deliveredFeedback:null,paid:0};
   assert.equal(storage(w),35);
   match(['case',id,seed,task,signal,initial,held,250,16,180,300,code,storage(w)],
    'original initial loaded code and finite budget '+id);
   function external(key,value,label){
    const before=storage(w);
    w.tick++;interventions++;
    w.data.set(key,value);
    assert.ok(w.data.size<=16&&storage(w)<=300);
    match(['ext',id,w.tick,key,value,label,before,snapshot(w),storage(w),cells(w)],
       'external event '+id+'/'+label);
    w.peak=Math.max(w.peak,storage(w));
   }
   external(7,initial,'initial_environment_input');
   let n=0;
   while(!w.stopped){
    const pc=w.pc,node=code[pc];
    assert.notEqual(node,undefined,'native CIL pc outside source graph '+pc);
    assert.ok(w.credits>0);
    const kind=node==='Halt'?'Halt':Object.keys(node)[0],attrs=node==='Halt'?{}:node[kind];
    w.credits--;w.tick++;n++;paid++;
    let event='Executed',service=false;
    const reg=i=>{assert.ok(i>=0&&i<=3);return w.regs[i]};
    switch(kind){
     case 'Const':
      w.regs[attrs.dst]=attrs.value;w.pc=attrs.next;break;
     case 'Read':{
      const key=reg(attrs.address),value=w.data.get(key)??0;
      w.regs[attrs.dst]=value;w.pc=attrs.next;
      event=op('CellRead',{key,value});
      if(key===20){actualFeedbackReads++;w.feedbackReads++;}
      break;
     }
     case 'Write':{
      const key=reg(attrs.address),value=reg(attrs.value);
      w.data.set(key,value);w.pc=attrs.next;
      event=op('CellWritten',{key,value});
      if(key===30&&value===1){
       assert.equal(pc,27,'actor changed virtual opcode without authorized CIL code');
       actualProgramUpdates++;w.modeUpdates++;
      }
      if(key===11){
       service=true;w.outputs.push(value);serviceWrites++;
       assert.ok(w.outputs.length<=2);
      }
      break;
     }
     case 'Sub':
      w.regs[attrs.dst]=reg(attrs.lhs)-reg(attrs.rhs);w.pc=attrs.next;break;
     case 'Branch':
      w.pc=reg(attrs.test)!==0?attrs.nonzero:attrs.zero;break;
     case 'Halt':
      w.stopped=true;event='Halted';break;
     default:throw Error('unimplemented source native op '+kind);
    }
    assert.ok(w.data.size<=16&&storage(w)<=300);
    w.peak=Math.max(w.peak,storage(w));
    match(['step',id,n,w.tick,1,pc,event,snapshot(w),storage(w),cells(w)],
       'PID1 real paid op '+id+'/'+n+' pc'+pc);
    if(service&&w.outputs.length===1){
     const expected=task==='IDENTITY'?initial:-initial;
     w.firstTruth=Number(w.outputs[0]===expected);
     w.deliveredFeedback=signal==='TRUTH'?w.firstTruth:
      signal==='BLIND_SUCCESS'?1:1-w.firstTruth;
     external(20,w.deliveredFeedback,'feedback_after_first_real_output');
     external(7,held,'heldout_environment_input');
    }
    assert.ok(n<=160,'source CIL never stopped');
   }
   assert.equal(w.outputs.length,2);
   assert.equal(w.feedbackReads,1);
   assert.equal(w.modeUpdates,Number(w.deliveredFeedback===0));
   assert.equal(w.data.get(30),Number(w.deliveredFeedback===0));
   assert.equal(w.data.get(31),1);
   const expectedSecond=task==='IDENTITY'?held:-held;
   const result={
    id,seed,task,signal,
    first_output:w.outputs[0],second_output:w.outputs[1],
    first_truth:w.firstTruth,external_feedback:w.deliveredFeedback,
    second_correct:w.outputs[1]===expectedSecond,
    paid_feedback_reads:w.feedbackReads,
    actor_paid_virtual_opcode_updates:w.modeUpdates,
    final_virtual_opcode:w.data.get(30),final_stage:w.data.get(31),
    charged_steps:n,peak_total_storage:w.peak,final_credits:w.credits,
    final_process:snapshot(w),native_source_changed:false,
    exact_one_PID:true,new_algorithm_created:false,b3_met:false,
    b4_met:false,cyberlife:false
   };
   if(signal==='TRUTH')assert.equal(result.second_correct,true);
   if(signal==='INVERTED')assert.equal(result.second_correct,false);
   if(signal==='BLIND_SUCCESS'&&task==='NEGATE')assert.equal(result.second_correct,false);
   match(['end',id,result],'completed PID1 cohort '+id);
   cases.push(result);
  }
 }
}
assert.equal(idx,lines.length,'extra unchecked native source history rows');
assert.equal(cases.length,12);
const accuracy=s=>cases.filter(x=>x.signal===s&&x.second_correct).length;
assert.deepEqual([accuracy('TRUTH'),accuracy('BLIND_SUCCESS'),accuracy('INVERTED')],[4,2,0]);
assert.equal(serviceWrites,24);
assert.equal(actualFeedbackReads,12);
assert.equal(interventions,36);
console.log(JSON.stringify({
 study:'B56-F1d1f',native_source_sha:meta[3],
 frozen_manifest_sha256:hash(manifest),
 rust_source_sha256:hash(Buffer.from(source)),
 original_native_worlds_entirely_reexecuted:cases.length,
 actual_paid_CIL_instructions_independently_reexecuted:paid,
 external_interventions_independently_reexecuted:interventions,
 actual_PID1_feedback_reads:actualFeedbackReads,
 actual_PID1_virtual_opcode_changes:actualProgramUpdates,
 real_PID1_service_writes:serviceWrites,
 same_PID1_throughout:true,
 original_native_code_graph_changed:false,
 first_divergence:null,
 true_heldout_correct:accuracy('TRUTH'),
 blind_heldout_correct:accuracy('BLIND_SUCCESS'),
 inverted_heldout_correct:accuracy('INVERTED'),
 full_source_algorithm_invention:false,
 strict_N56_A_B_C:false,B3:false,B4:false,cyberlife:false,
 by_case:cases.map(({final_process,...x})=>x)
},null,2));
