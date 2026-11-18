#!/usr/bin/env node
// CL-1B B56-F1d1i independently reexecute ORIGINAL native paid CIL,
// original external I/O/feedback, program-slot mutation, 3-phase hidden
// regime shifts, and source-branch prior-best fallback in one continuous PID.
// Do not infer spontaneous instruction invention or cyberlife.
import fs from 'node:fs';
import crypto from 'node:crypto';
import assert from 'node:assert/strict';
import {isDeepStrictEqual} from 'node:util';

const load=p=>fs.readFileSync(new URL(p,import.meta.url));
const rust=load('../../cl0/crates/world/tests/cl1b_b56_f1d1i_conditioned_mutation_rollback.rs').toString('utf8');
const specBuffer=load('../studies/B56-F1d1i-frozen-exact-source-v1.json');
const manifestBuffer=load('../studies/B56-F1d1i-frozen-157-node-phase-shift-and-fallback-manifest-v1.md');
const spec=JSON.parse(specBuffer);
const sha256=b=>crypto.createHash('sha256').update(b).digest('hex');
const span=rust.split('fn founder()->Vec<Op>{')[1]?.split('#[derive(Copy,Clone,Debug,PartialEq,Eq)]')[0];
assert.ok(span?.includes('vec!['),'native source literal extraction missing');
const sourceRows=[...span.matchAll(/Op::([A-Za-z]+)(?:\{([^}]+)\})?,\s*\/\/ pc (\d+)/g)];
assert.equal(sourceRows.length,157,'original F1d1i CIL node count must remain 157');
const op=(kind,attrs={})=>kind==='Halt'?'Halt':{[kind]:attrs};
const code=sourceRows.map((m,i)=>{
 assert.equal(Number(m[3]),i,'source PC comments drift '+i);
 const args=Object.fromEntries([...(m[2]||'').matchAll(/(\w+):(-?\d+)/g)]
   .map(x=>[x[1],Number(x[2])]));
 return op(m[1],args);
});
assert.deepEqual(code,spec.code.map(({kind,...attrs})=>op(kind,attrs)),
 'actual Rust source is not source-frozen preregistration');
const labels=spec.labels;
assert.deepEqual([labels.fetchOpcode,labels.serviceWrite,labels.feedbackRead,
 labels.scoreWrite,labels.paidRealSlot52,labels.paidRealSlot53,
 labels.shadowWrite1,labels.shadowWrite2,labels.rollback,
 labels.heldoutStageWrite],[30,52,59,65,123,125,127,129,93,141]);
assert.deepEqual(spec.allowed_keys,
 [7,11,20,21,31,32,33,50,51,52,53,54,56,61,62,63]);
assert.deepEqual(spec.schedules,
 [['IDENTITY','COMPOSED','IDENTITY'],['COMPOSED','NEGATE','UNREACHABLE']]);
assert.deepEqual(spec.regimes,['TRUTH','BLIND_SUCCESS','INVERTED','ABSENT']);
assert.deepEqual(spec.gates,[1,0]);
assert.deepEqual([spec.starting_credits,spec.max_data_cells,
 spec.max_native_nodes,spec.max_total_storage],[4000,16,180,300]);
assert.ok(code.every(o=>o==='Halt'||(!o.GraphLaunch&&!o.GraphEncode&&!o.GraphNew)),
 'source unexpectedly creates native process/graph');
let fnv=14695981039346656037n;
for(const byte of Buffer.from(JSON.stringify(code),'utf8')){
 fnv=BigInt.asUintN(64,(fnv^BigInt(byte))*1099511628211n);
}
const codeHash=fnv.toString(16).padStart(16,'0');

const evidence='cl1b-ci-evidence/b56_f1d1i_native_mutation_regime_shift.jsonl';
if(!fs.existsSync(evidence))throw Error('missing ORIGINAL exact-source native F1d1i JSONL');
const lines=fs.readFileSync(evidence,'utf8').trimEnd().split(/\r?\n/);
let cursor=0;
function eq(expected,what){
 if(cursor>=lines.length)throw Error('F1d1i original native evidence truncated at '+what);
 const actual=JSON.parse(lines[cursor++]);
 if(!isDeepStrictEqual(expected,actual))throw Error(
  'F1d1i FIRST NATIVE DIVERGENCE at line '+cursor+': '+what+
  '\nEXPECTED '+JSON.stringify(expected)+'\nACTUAL   '+JSON.stringify(actual));
}
const meta=JSON.parse(lines[cursor++]);
assert.deepEqual(meta.slice(0,3),['meta','B56-F1d1i',1]);
if(process.env.GITHUB_SHA)assert.equal(meta[3],process.env.GITHUB_SHA);
assert.equal(meta[4],sha256(manifestBuffer));
assert.equal(meta[5],manifestBuffer.length);
assert.equal(meta[6],sha256(specBuffer));
assert.equal(meta[7],specBuffer.length);
assert.deepEqual(meta.slice(8),
 [16,4000,16,180,300,157,30,52,59,65,123,125,127,129,93,141]);
const KEYS=spec.allowed_keys;
const snap=s=>[s.pc,[...s.regs],s.credits,s.stopped,157,codeHash];
const cell=(s,k)=>s.data.has(k)?s.data.get(k):null;
const cells=s=>KEYS.map(k=>cell(s,k));
const prog=s=>[51,52,53,54].map(k=>cell(s,k));
const occ=s=>158+s.data.size;
const desired=(task,x)=>{
 switch(task){
  case 'IDENTITY':return x;
  case 'NEGATE':return -x;
  case 'COMPOSED':return -x+2;
  case 'UNREACHABLE':return x+4;
  default:throw Error('unexpected hidden task '+task);
 }
};
let nativePaid=0,allExternal=0,allFeedbackReads=0,allScores=0,allRealEdits=0;
let allShadowEdits=0,allRollback=0,allFetches=0,allAccWrites=0,allServices=0;
let maxStorage=0,unexpectedProcesses=0;
const outcomes=[];
for(let scheduleId=0;scheduleId<2;scheduleId++){
 for(const signal of spec.regimes){
  for(const gate of spec.gates){
   const id=outcomes.length,schedule=spec.schedules[scheduleId];
   const s={
    pc:0,regs:[0,0,0,0],credits:4000,stopped:false,tick:0,
    data:new Map(),paid:0,peak:158,services:[],feedbackReads:0,
    scoreWrites:0,realEdits:0,shadowEdits:0,rollbackCount:0,
    rollbackReadValues:[],fetchCount:0,accWrites:0
   };
   eq(['case',id,scheduleId,signal,gate,schedule,code,codeHash,
      occ(s),4000,16,180,300],'frozen source world '+id);
   function ext(key,value,reason){
    assert.ok([7,20,21].includes(key),'unauthorized external cell edit');
    allExternal++;
    s.tick++;
    s.data.set(key,value);
    assert.ok(s.data.size<=16&&occ(s)<=300);
    s.peak=Math.max(s.peak,occ(s));
    eq(['ext',id,s.tick,key,value,reason,snap(s),occ(s),cells(s)],
       'external environment '+id+' '+reason);
   }
   ext(21,gate,'single_external_edit_gate');
   ext(7,-7,'first_phase_probe_0');
   while(!s.stopped){
    const pc=s.pc,raw=code[pc];
    assert.notEqual(raw,undefined,'invalid CIL program pc '+pc);
    assert.ok(s.credits>0,'spent all credits before native halt');
    const kind=raw==='Halt'?'Halt':Object.keys(raw)[0];
    const o=raw==='Halt'?{}:raw[kind];
    const reg=i=>{assert.ok(i>=0&&i<=3);return s.regs[i]};
    s.tick++;s.paid++;nativePaid++;s.credits--;
    let event='Executed';
    let justProbe=null,finishedEpoch=null;
    switch(kind){
     case 'Const':
      s.regs[o.dst]=o.value;s.pc=o.next;break;
     case 'Add':
      s.regs[o.dst]=reg(o.lhs)+reg(o.rhs);s.pc=o.next;break;
     case 'Sub':
      s.regs[o.dst]=reg(o.lhs)-reg(o.rhs);s.pc=o.next;break;
     case 'Read':{
      const key=reg(o.address),value=s.data.get(key)??0;
      s.regs[o.dst]=value;s.pc=o.next;
      event=op('CellRead',{key,value});
      if(pc===labels.fetchOpcode){
       const vpc=cell(s,50);
       assert.ok(Number.isInteger(vpc)&&vpc>=0&&vpc<=3);
       assert.equal(key,51+vpc,'wrong dynamic virtual instruction address');
       assert.equal(value,cell(s,key));
       s.fetchCount++;allFetches++;
      }
      if(pc===labels.feedbackRead){
       assert.equal(key,20);
       s.feedbackReads++;allFeedbackReads++;
       assert.equal(value,s.data.get(20)??0);
      }
      if(pc===labels.rollback+1){
       assert.equal(key,62,'prior-best fallback must pay Read62');
       s.rollbackReadValues.push(value);
      }
      break;
     }
     case 'Write':{
      const key=reg(o.address),value=reg(o.value);
      s.data.set(key,value);s.pc=o.next;
      event=op('CellWritten',{key,value});
      if(pc===labels.scoreWrite){
       assert.equal(key,61);
       s.scoreWrites++;allScores++;
      }
      if(pc===labels.paidRealSlot52||pc===labels.paidRealSlot53){
       assert.equal(gate,1,'free/disabled program slot edit');
       assert.equal(key,pc===labels.paidRealSlot52?52:53);
       s.realEdits++;allRealEdits++;
      }
      if(pc===labels.shadowWrite1||pc===labels.shadowWrite2){
       assert.equal(gate,0,'shadow program edit without ablation');
       assert.equal(key,56);
       s.shadowEdits++;allShadowEdits++;
      }
      if(key===56&&pc!==labels.shadowWrite1&&pc!==labels.shadowWrite2){
       s.accWrites++;allAccWrites++;
      }
      if(key===11){
       assert.equal(pc,labels.serviceWrite);
       const epoch=cell(s,31),stage=cell(s,32),mode=cell(s,33);
       assert.ok(epoch>=0&&epoch<=2&&stage>=0&&stage<=2);
       assert.ok(mode>=0&&mode<=2);
       s.services.push([epoch,stage,mode,value,prog(s)]);
       allServices++;
       if(stage<2)justProbe=[epoch,stage,value];
       else finishedEpoch=epoch;
      }
      break;
     }
     case 'Branch':
      s.pc=reg(o.test)!==0?o.nonzero:o.zero;break;
     case 'Halt':
      s.stopped=true;event='Halted';break;
     default:throw Error('source CIL primitive changed '+kind);
    }
    if(pc===labels.rollback){
     s.rollbackCount++;allRollback++;
    }
    assert.equal(s.credits,4000-s.paid,'unaccounted compute transfer');
    assert.ok(s.paid<=4000);
    assert.ok(s.data.size<=16&&occ(s)<=300);
    s.peak=Math.max(s.peak,occ(s));
    maxStorage=Math.max(maxStorage,s.peak);
    eq(['step',id,s.paid,s.tick,1,pc,event,snap(s),occ(s),cells(s)],
       'charged PID1 original CIL '+id+'/'+s.paid+' PC '+pc);
    if(justProbe){
     const [epoch,stage,output]=justProbe;
     const correct=Number(output===desired(schedule[epoch],[-7,5][stage]));
     const bit=signal==='TRUTH'?correct:
      signal==='BLIND_SUCCESS'?1:
      signal==='INVERTED'?1-correct:null;
     if(bit!==null)ext(20,bit,'post_probe_correctness_bit');
     ext(7,stage===0?5:-7,'next_probe_default_input');
    }
    if(pc===labels.heldoutStageWrite){
     assert.equal(kind,'Write');
     assert.equal(cell(s,32),2);
     ext(7,11,'actor_selected_heldout_input');
    }
    if(finishedEpoch!==null&&finishedEpoch<2){
     ext(7,-7,'next_phase_hidden_task_probe0');
    }
   }
   assert.equal(cell(s,31),3);
   assert.ok(s.rollbackReadValues.every(mode=>mode>=0&&mode<=2));
   const byPhase=s.services.filter(x=>x[1]===2).map(([epoch,stage,mode,output,actualBytecode])=>({
    epoch,task:schedule[epoch],mode,output,
    expected:desired(schedule[epoch],11),
    correct:output===desired(schedule[epoch],11),
    actual_bytecode:actualBytecode
   }));
   assert.equal(byPhase.length,3);
   const correctCount=byPhase.filter(p=>p.correct).length;
   if(gate===1&&signal==='TRUTH'){
    assert.equal(correctCount,scheduleId===0?3:2);
    assert.deepEqual(byPhase.map(p=>p.mode),
      scheduleId===0?[0,1,0]:[1,2,2]);
    if(scheduleId===1){
     assert.ok(s.rollbackCount>0&&s.rollbackReadValues.includes(2));
     assert.equal(byPhase[2].correct,false);
     assert.deepEqual(byPhase[2].actual_bytecode,[0,1,3,3]);
    }
   }
   if(gate===0){
    assert.equal(s.realEdits,0);
    assert.ok(s.services.every(x=>isDeepStrictEqual(x[4],[0,3,3,3])));
   }else{assert.equal(s.shadowEdits,0)}
   const result={
    id,schedule_id:scheduleId,signal,gate,
    all_real_service_events:s.services,
    heldout_by_phase:byPhase,
    heldout_correct_count:correctCount,
    charged_feedback_reads:s.feedbackReads,
    actor_paid_score_writes:s.scoreWrites,
    actor_paid_real_opcode_slot_writes:s.realEdits,
    actor_paid_suppressed_shadow_writes:s.shadowEdits,
    prior_best_rollback_entry_count:s.rollbackCount,
    prior_best_rollback_read_values:s.rollbackReadValues,
    virtual_opcode_fetches:s.fetchCount,
    charged_accumulator_checkpoint_writes:s.accWrites,
    max_storage:s.peak,paid_instructions:s.paid,
    remaining_credits:s.credits,
    final_process:snap(s),final_cells:cells(s),
    same_PID1:true,native_source_unchanged:true,
    author_primitive_ISA:true,b3_met:false,b4_met:false,cyberlife:false
   };
   eq(['end',id,result],'original final paid phase states '+id);
   outcomes.push(result);
  }
 }
}
assert.equal(cursor,lines.length,'native source had unchecked trailing rows');
assert.equal(outcomes.length,16);
assert.equal(unexpectedProcesses,0);
const phase=(sid,signal,gate)=>outcomes.find(x=>x.schedule_id===sid&&x.signal===signal&&x.gate===gate);
for(const sid of [0,1])assert.equal(phase(sid,'TRUTH',1).heldout_correct_count,sid===0?3:2);
const adaptiveTruth=[phase(0,'TRUTH',1),phase(1,'TRUTH',1)];
assert.deepEqual(adaptiveTruth[0].heldout_by_phase.map(x=>x.mode),[0,1,0]);
assert.deepEqual(adaptiveTruth[1].heldout_by_phase.map(x=>x.mode),[1,2,2]);
// Check same source and first native input, counterfactual that only the
// environment-supplied post-service truth bits differ before proposed edits.
const s0truth=phase(0,'TRUTH',1),s0invert=phase(0,'INVERTED',1);
assert.deepEqual(s0truth.all_real_service_events[0],
 s0invert.all_real_service_events[0]);
assert.ok(s0truth.actor_paid_real_opcode_slot_writes!==s0invert.actor_paid_real_opcode_slot_writes);
const realCounts=outcomes.map(x=>x.paid_instructions);
console.log(JSON.stringify({
 study:'B56-F1d1i',
 tested_native_SHA:meta[3],
 preregistered_exact_manifest_sha256:sha256(manifestBuffer),
 preregistered_157_node_graph_sha256:sha256(specBuffer),
 rust_source_sha256:sha256(Buffer.from(rust)),
 source_node_count:157,
 original_strict_native_worlds_independently_replayed:16,
 actual_original_paid_CIL_instructions_reexecuted:nativePaid,
 actual_external_environment_interventions_replayed:allExternal,
 actual_paid_feedback_reads:allFeedbackReads,
 actual_paid_candidate_score_writes:allScores,
 real_actor_paid_virtual_bytecode_edits:allRealEdits,
 paid_generation_suppression_shadow_edits:allShadowEdits,
 prior_best_fallback_enter_events:allRollback,
 original_dynamic_virtual_instruction_fetches:allFetches,
 real_native_accumulator_writes:allAccWrites,
 actual_paid_service_outputs:allServices,
 charged_instruction_range:[Math.min(...realCounts),Math.max(...realCounts)],
 peak_storage:maxStorage,
 first_divergence:null,
 unchanged_same_original_PID1:true,
 original_native_CIL_source_unchanged:true,
 same_source_truth_vs_inverted_next_mutation_diverges:true,
 truth_s1_identity_composed_identity_heldout_correct:adaptiveTruth[0].heldout_correct_count,
 truth_s2_composed_negate_unreachable_heldout_correct:adaptiveTruth[1].heldout_correct_count,
 no_available_program_solves_unreachable_task:true,
 fallback_was_source_paid_not_environment_written:true,
 absence_vs_negative_feedback_indistinguishable:true,
 interpreter_and_mutation_cycle_authored_by_researcher:true,
 source_F1c_continuity:false,
 strict_N56_A_B_C:false,B3:false,B4:false,cyberlife:false,
 summary_by_case:outcomes.map(({
  all_real_service_events,final_process,final_cells,heldout_by_phase,...rest
 })=>({...rest,heldout_by_phase}))
},null,2));
