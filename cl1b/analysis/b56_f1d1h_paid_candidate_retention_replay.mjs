#!/usr/bin/env node
// B56-F1d1h: independently reexecute every ACTUAL Rust paid CIL instruction,
// external input/feedback event, generated candidate, score, retained winner,
// and held-out program. All native source code comes from the Rust source and
// must match frozen pre-implementation 166-node source. Fail first divergence.
import fs from 'node:fs';
import crypto from 'node:crypto';
import assert from 'node:assert/strict';
import {isDeepStrictEqual} from 'node:util';

const load=(p)=>fs.readFileSync(new URL(p,import.meta.url));
const native=load('../../cl0/crates/world/tests/cl1b_b56_f1d1h_paid_candidate_retention.rs').toString();
const frozen=load('../studies/B56-F1d1h-frozen-exact-CIL-source-v1.json');
const prereg=load('../studies/B56-F1d1h-frozen-exact-source-and-36-world-controls-v1.md');
const declared=JSON.parse(frozen.toString());
const digest=b=>crypto.createHash('sha256').update(b).digest('hex');
const core=native.split('fn founder()->Vec<Op>{')[1]?.split('#[derive(Copy,Clone)]')[0];
assert.ok(core?.includes('vec!['),'source extraction failed: native source drift');
const rows=[...core.matchAll(/Op::([A-Za-z]+)(?:\{([^}]+)\})?,\s*\/\/ pc (\d+)/g)];
assert.equal(rows.length,166,'frozen native CIL node count drift');
const op=(name,fields={})=>name==='Halt'?'Halt':{[name]:fields};
const code=rows.map((m,index)=>{
 assert.equal(Number(m[3]),index,'Rust literal PC labels drifted '+index);
 const attrs=Object.fromEntries([...(m[2]??'').matchAll(/(\w+):(-?\d+)/g)]
     .map(v=>[v[1],Number(v[2])]));
 return op(m[1],attrs);
});
const declaredCode=declared.code.map(({kind,...props})=>op(kind,props));
assert.deepEqual(code,declaredCode,'native CIL graph diverged from preregistration');
assert.equal(declared.code.length,166);
assert.deepEqual(declared.keys,[7,11,20,21,31,32,50,51,52,53,54,55,56,61,62,63]);
assert.deepEqual(declared.candidate_opcodes,[[0,3,3,3],[0,1,3,3],[0,1,2,3]]);
assert.deepEqual(declared.cases.tasks,['IDENTITY','NEGATE','COMPOSED']);
assert.deepEqual(declared.cases.feedback,['TRUTH','BLIND_SUCCESS','INVERTED','ABSENT']);
assert.deepEqual(declared.cases.gates,[1,0,2]);
assert.deepEqual(declared.cases.probe_inputs,[-7,5]);
assert.equal(declared.cases.heldout,11);
assert.deepEqual([declared.cases.initial_credits,declared.cases.max_cells,
 declared.cases.max_nodes,declared.cases.max_total_storage],[1000,16,180,300]);
assert.ok(code.every(v=>v==='Halt'||(!v.GraphLaunch&&!v.GraphEncode&&!v.GraphNew)),
 'source unexpectedly spawns child or encodes generic native graph');

let sourceFnv=14695981039346656037n;
for(const b of Buffer.from(JSON.stringify(code),'utf8'))
 sourceFnv=BigInt.asUintN(64,(sourceFnv^BigInt(b))*1099511628211n);
const sourceHash=sourceFnv.toString(16).padStart(16,'0');
const labels=declared.labels;
assert.deepEqual([labels.fetchOpcode,labels.writeService,labels.readFeedback,
 labels.scoreWrite,labels.writeGeneratedOpcode],[31,54,61,70,104]);

const evidence='cl1b-ci-evidence/b56_f1d1h_native_paid_candidate_retention.jsonl';
if(!fs.existsSync(evidence))throw Error('original SAME-SHA paid CIL native history absent');
const lines=fs.readFileSync(evidence,'utf8').trimEnd().split(/\r?\n/);
let cursor=0;
function compare(expected,why){
 if(cursor>=lines.length)throw Error('truncated original event history '+why);
 const got=JSON.parse(lines[cursor++]);
 if(!isDeepStrictEqual(expected,got))throw Error(
  'B56-F1d1h FIRST NATIVE DIVERGENCE at line '+cursor+' '+why+
  '\nEXPECTED '+JSON.stringify(expected)+'\nACTUAL   '+JSON.stringify(got));
}
const meta=JSON.parse(lines[cursor++]);
assert.deepEqual(meta.slice(0,3),['meta','B56-F1d1h',1]);
assert.equal(meta[4],digest(prereg));
assert.equal(meta[5],prereg.length);
assert.equal(meta[6],digest(frozen));
assert.equal(meta[7],frozen.length);
assert.deepEqual(meta.slice(8),
 [36,1000,16,180,300,166,31,54,61,70,104]);
if(process.env.GITHUB_SHA)assert.equal(meta[3],process.env.GITHUB_SHA);

const keys=declared.keys;
function snap(s){return [s.pc,[...s.regs],s.credits,s.stopped,166,sourceHash]}
function cells(s){return keys.map(key=>s.data.has(key)?s.data.get(key):null)}
function occ(s){return 167+s.data.size}
function oracle(task,x){
 if(task==='IDENTITY')return x;
 if(task==='NEGATE')return -x;
 if(task==='COMPOSED')return -x+2;
 throw Error('unrecognized task');
}
const outcomes=[];let paid=0,external=0,fetches=0,scoreWrites=0,generated=0;
let checkpointWrites=0,scoreReads=0,serviceWrites=0,allFeedbackReads=0,shadowWrites=0;
for(const task of declared.cases.tasks){
 for(const signal of declared.cases.feedback){
  for(const gate of declared.cases.gates){
   const id=outcomes.length,frozenGate=gate===2,probes=frozenGate?2:6;
   const state={
    pc:0,regs:[0,0,0,0],credits:1000,stopped:false,tick:0,
    data:new Map(),n:0,peak:167,services:[],feedbackReads:0,
    scoringWrites:0,generatedWrites:0,
    fetchHistory:[],accCheckpointHistory:[]
   };
   compare(['case',id,task,signal,gate,code,sourceHash,occ(state),
     1000,[-7,5],11,probes],'initial source '+id);
   function externalSet(key,value,reason){
    assert.ok([7,20,21].includes(key));
    state.tick++;external++;
    state.data.set(key,value);
    assert.ok(state.data.size<=16&&occ(state)<=300);
    state.peak=Math.max(state.peak,occ(state));
    compare(['ext',id,state.tick,key,value,reason,snap(state),
     occ(state),cells(state)],'source-limited external '+id+' '+reason);
   }
   externalSet(21,gate,'gate_before_first_charge');
   externalSet(7,-7,'initial_probe_input');
   while(!state.stopped){
    const pc=state.pc,node=code[pc];
    assert.notEqual(node,undefined,'CIL PC invalid '+id+'/'+pc);
    assert.ok(state.credits>0,'native credit exhaustion '+id);
    const kind=node==='Halt'?'Halt':Object.keys(node)[0],o=node==='Halt'?{}:node[kind];
    const reg=i=>{assert.ok(i>=0&&i<=3);return state.regs[i];};
    state.credits--;state.n++;state.tick++;paid++;
    let event='Executed';
    switch(kind){
     case 'Const':state.regs[o.dst]=o.value;state.pc=o.next;break;
     case 'Add':state.regs[o.dst]=reg(o.lhs)+reg(o.rhs);state.pc=o.next;break;
     case 'Sub':state.regs[o.dst]=reg(o.lhs)-reg(o.rhs);state.pc=o.next;break;
     case 'Read':{
      const key=reg(o.address),value=state.data.get(key)??0;
      state.regs[o.dst]=value;state.pc=o.next;
      event=op('CellRead',{key,value});
      if(pc===labels.fetchOpcode){
       const vpc=state.data.get(50);
       assert.ok(vpc>=0&&vpc<=3);
       assert.equal(key,51+vpc);
       state.fetchHistory.push([state.data.get(31),state.data.get(32),vpc,value]);
       fetches++;
      }
      if(pc===labels.readFeedback){assert.equal(key,20);allFeedbackReads++;state.feedbackReads++;}
      if([61,62,63].includes(key))scoreReads++;
      break;
     }
     case 'Write':{
      const key=reg(o.address),value=reg(o.value);
      state.data.set(key,value);state.pc=o.next;
      event=op('CellWritten',{key,value});
      if(pc===labels.scoreWrite){
       assert.ok([61,62,63].includes(key));state.scoringWrites++;scoreWrites++;
      }
      if(pc===labels.writeGeneratedOpcode){
       assert.equal(gate,1);assert.ok(key===52||key===53);
       state.generatedWrites++;generated++;
      }
      if(key===55&&pc===106){shadowWrites++;}
      if(key===56){state.accCheckpointHistory.push(value);checkpointWrites++;}
      if(key===11){assert.equal(pc,labels.writeService);serviceWrites++;
       state.services.push([state.data.get(31),state.data.get(32),value]);}
      break;
     }
     case 'Branch':
      state.pc=reg(o.test)!==0?o.nonzero:o.zero;break;
     case 'Halt':state.stopped=true;event='Halted';break;
     default:throw Error('unknown CIL op '+kind);
    }
    assert.ok(state.n<=1000);
    assert.ok(state.data.size<=16&&occ(state)<=300);
    state.peak=Math.max(state.peak,occ(state));
    compare(['step',id,state.n,state.tick,1,pc,event,snap(state),
      occ(state),cells(state)],'actual native paid '+id+'/'+state.n+' pc'+pc);
    if(kind==='Write'&&pc===labels.writeService){
     const [candidate,stage,value]=state.services.at(-1);
     if(stage!==2){
      assert.ok(candidate>=0&&candidate<=2);
      assert.ok(stage>=0&&stage<=1);
      const correctness=Number(value===oracle(task,[-7,5][stage]));
      const bit=signal==='TRUTH'?correctness:
       signal==='BLIND_SUCCESS'?1:
       signal==='INVERTED'?1-correctness:null;
      if(bit!==null)externalSet(20,bit,'probe_correctness_feedback');
      const next=stage===0?5:frozenGate||candidate===2?11:-7;
      externalSet(7,next,'next_probe_or_heldout_input');
     }
    }
   }
   assert.equal(state.services.length,probes+1);
   assert.equal(state.feedbackReads,probes);
   const scores=[61,62,63].map(k=>state.data.get(k));
   assert.ok(scores.every(v=>v>=0&&v<=2));
   const best=[1,2].reduce((a,i)=>scores[i]>scores[a]?i:a,0);
   assert.equal(state.data.get(31),best,'native program failed to retain winning score');
   assert.equal(state.data.get(32),2);
   const finalCode=[51,52,53,54].map(k=>state.data.get(k));
   assert.deepEqual(finalCode,gate===1?declared.candidate_opcodes[best]:declared.candidate_opcodes[0]);
   const last=state.services.at(-1);
   assert.equal(last[1],2);
   const correct=last[2]===oracle(task,11);
   if(gate===1&&signal==='TRUTH')assert.equal(correct,true);
   if(gate===1&&signal==='INVERTED')assert.equal(correct,false);
   if(gate!==1)assert.equal(correct,task==='IDENTITY');
   const result={
    id,task,signal,gate,initial_program:[0,3,3,3],
    probe_services:state.services.length-1,
    actual_paid_services:state.services,
    actual_virtual_fetches:state.fetchHistory,
    accumulator_checkpoints:state.accCheckpointHistory,
    score_cells:scores,paid_score_updates:state.scoringWrites,
    paid_generated_program_slot_edits:state.generatedWrites,
    retained_winner:best,reinstated_program:finalCode,
    heldout_correct:correct,charged_instructions:state.n,
    peak_total_storage:state.peak,
    remaining_credits:state.credits,final_process:snap(state),
    single_PID1:true,source_code_unchanged:true,
    native_opcode_generation:false,
    b3_met:false,b4_met:false,cyberlife:false
   };
   compare(['end',id,result],'complete PID1 candidate retention case'+id);
   outcomes.push(result);
  }
 }
}
assert.equal(cursor,lines.length,'unchecked original native history tail');
assert.equal(outcomes.length,36);
const select=(gate,signal)=>outcomes.filter(x=>x.gate===gate&&x.signal===signal);
const accuracy=(gate,signal)=>select(gate,signal).filter(x=>x.heldout_correct).length;
assert.deepEqual(['TRUTH','BLIND_SUCCESS','INVERTED','ABSENT'].map(x=>accuracy(1,x)),[3,1,0,1]);
for(const gate of [0,2])
 assert.deepEqual(['TRUTH','BLIND_SUCCESS','INVERTED','ABSENT'].map(x=>accuracy(gate,x)),[1,1,1,1]);
for(const task of ['IDENTITY','NEGATE','COMPOSED']){
 const x=outcomes.find(r=>r.gate===1&&r.task===task&&r.signal==='TRUTH');
 assert.equal(x.retained_winner,['IDENTITY','NEGATE','COMPOSED'].indexOf(task));
 assert.equal(x.score_cells[x.retained_winner],2);
 assert.equal(x.probe_services,6);
}
const counts=outcomes.map(x=>x.charged_instructions);
const maxStorage=Math.max(...outcomes.map(x=>x.peak_total_storage));
console.log(JSON.stringify({
 study:'B56-F1d1h',source_sha:meta[3],
 frozen_manifest_sha256:digest(prereg),
 frozen_exact_CIL_source_sha256:digest(frozen),
 rust_native_source_sha256:digest(Buffer.from(native)),
 original_native_cases_reexecuted:outcomes.length,
 source_native_CIL_nodes:code.length,
 exact_actor_code_source_FNV:sourceHash,
 real_paid_CIL_instructions_reexecuted:paid,
 external_environment_interventions_reexecuted:external,
 charged_feedback_reads:allFeedbackReads,
 actor_paid_score_writes:scoreWrites,
 paid_score_reads:scoreReads,
 actor_paid_generated_bytecode_edits:generated,
 actor_paid_shadow_edits:shadowWrites,
 actual_virtual_opcode_fetches:fetches,
 real_paid_accumulator_checkpoints:checkpointWrites,
 real_native_service_outputs:serviceWrites,
 charged_instruction_range:[Math.min(...counts),Math.max(...counts)],
 max_total_storage:maxStorage,
 first_divergence:null,
 same_original_PID1:true,
 native_code_never_changes:true,
 candidate_programs_built_incrementally:true,
 no_host_selected_winner:true,
 true_winner_and_correct_per_three:3,
 blinded_correct_per_three:accuracy(1,'BLIND_SUCCESS'),
 inverted_correct_per_three:accuracy(1,'INVERTED'),
 absent_feedback_correct_per_three:accuracy(1,'ABSENT'),
 suppressed_generation_is_not_three_distinct_candidates:true,
 frozen_baseline_only_A:true,
 primitive_ISA_and_selection_rule_are_human_authored:true,
 source_original_F1c_retention:false,
 strict_N56_A_B_C:false,B3:false,B4:false,cyberlife:false,
 cases:outcomes.map(({actual_virtual_fetches,accumulator_checkpoints,final_process,...rest})=>rest)
},null,2));
