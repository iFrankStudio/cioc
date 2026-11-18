#!/usr/bin/env node
// G3Q-C0 independent original PID1 fully reexecuted native CIL event replay.
// This checks the PRE-NATIVE FROZEN 164-node graph against the actual literal
// Rust CIL founder and every original PAID native instruction of one 64-rule
// continuous world. Neither a final-score oracle nor a virtual JS-only proof.
import fs from 'node:fs';
import crypto from 'node:crypto';
import assert from 'node:assert/strict';
import {isDeepStrictEqual} from 'node:util';
const load=p=>fs.readFileSync(new URL(p,import.meta.url));
const RUST_BYTES=load('../../cl0/crates/world/tests/cl1b_b56_f1d1k_g3q_c0_paid_coverage_certificate.rs');
const originalRust=RUST_BYTES.toString('utf8');
const FROZEN_BYTES=load('../studies/B56-F1d1k-G3Q-C0-frozen-exact-164-node-original-CIL-founder-v1.json');
const PROTOCOL_BYTES=load('../studies/B56-F1d1k-G3Q-C0-original-PID1-paid-64-rule-coverage-preregistration.md');
const frozen=JSON.parse(FROZEN_BYTES);
const SHA=b=>crypto.createHash('sha256').update(b).digest('hex');
const body=originalRust.split('fn founder()->Vec<Op>{')[1]?.split('fn source_fingerprint')[0];
assert.ok(body?.includes('vec!['),'original CIL source missing from Rust literal');
const matches=[...body.matchAll(/Op::([A-Za-z]+)(?:\{([^}]+)\})?,\s*\/\/ pc (\d+)/g)];
assert.equal(matches.length,164,'original native CIL source is not 164 instructions');
const nativeOp=(name,fields={})=>name==='Halt'?'Halt':{[name]:fields};
const code=matches.map((m,pc)=>{
 assert.equal(Number(m[3]),pc,'Rust native source PC comment mismatch');
 const args=Object.fromEntries([...(m[2]??'').matchAll(/(\w+):(-?\d+)/g)]
  .map(x=>[x[1],Number(x[2])]));
 return nativeOp(m[1],args);
});
assert.deepEqual(code,frozen.exact_native_founder_code.map(({kind,...args})=>
 nativeOp(kind,args)),'actual original Rust opcode differs from pre-native frozen graph');
const L=frozen.labels;
assert.deepEqual([
 L.paidDynamicMetaRead,L.paidMetaAccumulatorWrite,L.paidModeWrite,
 L.paidSeenA,L.paidSeenC,L.paidSeenB,L.paidMetaMiddleTrialMutation,
 L.paidCoverageVerdictService,L.paidRetainBase,L.paidRetainMiddle,
 L.paidRetainTail,L.paidMiddleRestore,L.paidDescriptorBaseAdvance,
 L.paidDescriptorMiddleAdvance,L.paidDescriptorTailAdvance,
 L.paidDescriptorIDAdvance,L.halt
],[29,46,59,63,69,72,90,106,113,117,121,125,135,146,156,162,163]);
const KEYS=[7,11,20,21,32,33,50,51,52,53,56,61,62,63,64,65];
assert.deepEqual(frozen.allowed_cells,KEYS);
assert.deepEqual(frozen.strict_world,{
 allowed_cells:16,max_source_nodes:180,max_storage_units:300,
 initial_private_PID1_credits:30000,PID_count:1,initial_data_cells:0,
 external_interventions:0
});
assert.ok(code.every(x=>x==='Halt'||(!x.GraphLaunch&&!x.GraphNew&&!x.GraphEncode)));
const rawPath='cl1b-ci-evidence/b56_f1d1k_g3q_c0_native_paid_64_meta_rule_coverage.jsonl';
if(!fs.existsSync(rawPath))throw Error('Missing G3Q-C0 original paid native JSONL. No free/abstract substitute.');
const lines=fs.readFileSync(rawPath,'utf8').trimEnd().split(/\r?\n/);
let cursor=0;
function check(expected,what){
 if(cursor>=lines.length)throw Error('Native trace ended prematurely at '+what);
 const native=JSON.parse(lines[cursor++]);
 if(!isDeepStrictEqual(expected,native))
  throw Error('FIRST ORIGINAL NATIVE CIL DIVERGENCE at JSONL line '+cursor+
   ': '+what+'\nEXPECTED '+JSON.stringify(expected)+
   '\nNATIVE '+JSON.stringify(native));
}
const meta=JSON.parse(lines[cursor++]);
assert.deepEqual(meta.slice(0,3),['meta','B56-F1d1k-G3Q-C0',1]);
if(process.env.GITHUB_SHA)assert.equal(meta[3],process.env.GITHUB_SHA);
assert.equal(meta[4],SHA(PROTOCOL_BYTES));
assert.equal(meta[5],PROTOCOL_BYTES.length);
assert.equal(meta[6],SHA(FROZEN_BYTES));
assert.equal(meta[7],FROZEN_BYTES.length);
assert.deepEqual(meta.slice(8),[
 164,180,16,300,30000,0,L.paidDynamicMetaRead,
 L.paidMetaAccumulatorWrite,L.paidModeWrite,L.paidSeenA,L.paidSeenC,
 L.paidSeenB,L.paidMetaMiddleTrialMutation,L.paidCoverageVerdictService,
 L.paidRetainBase,L.paidRetainMiddle,L.paidRetainTail,L.paidMiddleRestore,
 L.paidDescriptorBaseAdvance,L.paidDescriptorMiddleAdvance,
 L.paidDescriptorTailAdvance,L.paidDescriptorIDAdvance,L.halt
]);
let fnv=14695981039346656037n;
for(const b of Buffer.from(JSON.stringify(code),'utf8'))
 fnv=BigInt.asUintN(64,(fnv^BigInt(b))*1099511628211n);
const HASH=fnv.toString(16).padStart(16,'0');
const s={pc:0,reg:[0,0,0,0],credits:30000,
 m:new Map(),step:0,tick:0,halted:false,peak:165};
const cell=k=>s.m.has(k)?s.m.get(k):null;
const snapshot=()=>[s.pc,[...s.reg],s.credits,s.halted,164,HASH];
const occupied=()=>165+s.m.size;
const cells=()=>KEYS.map(k=>cell(k));
const targetMode=([a,b,c])=>[a,b,c].reduce((v,op)=>op===3?0:(v+op)%3,0);
const independentCoverage=([a,b0,c])=>{
 let b=b0,modes=[];
 for(let i=0;i<3;i++){modes.push(targetMode([a,b,c]));if(i<2)b=(b+1)%4;}
 return {modes,safe:new Set(modes).size===3};
};
const summary={
 steps:0,metaReads:0,metaWrites:0,completedMeta:0,seen:0,
 retention:0,trialMiddle:0,middleRestores:0,idAdvances:0,
 baseAdvances:0,middleAdvances:0,tailAdvances:0,
 safe:0,unsafe:0,haltCount:0
};
let lastMode=0,lastSafe=[0,0,0];
const retained=[],verdicts=[];
check(['case',0,code,HASH,30000,165,snapshot(),cells()],
 'founder load and zero-initialized process1');
while(!s.halted){
 assert.ok(s.credits>0,'private CIL credit exhaustion');
 const pc=s.pc,object=code[pc];
 assert.notEqual(object,undefined,'invalid original source native CIL PC');
 const kind=object==='Halt'?'Halt':Object.keys(object)[0],
  op=object==='Halt'?{}:object[kind],r=i=>s.reg[i];
 s.step++;s.tick++;s.credits--;
 let event='Executed';
 switch(kind){
  case 'Const':s.reg[op.dst]=op.value;s.pc=op.next;break;
  case 'Read':{
   const k=r(op.address),v=s.m.get(k)??0;
   s.reg[op.dst]=v;s.pc=op.next;
   event=nativeOp('CellRead',{key:k,value:v});
   if(pc===L.paidDynamicMetaRead){
    assert.ok([62,63,64].includes(k));
    assert.equal(k,62+(cell(50)??0));
    summary.metaReads++;
   }
   break;
  }
  case 'Write':{
   const k=r(op.address),v=r(op.value);
   assert.ok(KEYS.includes(k),'founder paid write to undisclosed key'+k);
   s.m.set(k,v);s.pc=op.next;
   event=nativeOp('CellWritten',{key:k,value:v});
   if(pc===L.paidMetaAccumulatorWrite){
    assert.equal(k,56);summary.metaWrites++;
   }
   if(pc===L.paidModeWrite){
    assert.equal(k,33);assert.ok(v>=0&&v<=2);
    lastMode=v;summary.completedMeta++;
   }
   if([L.paidSeenA,L.paidSeenC,L.paidSeenB].includes(pc)){
    const target=pc===L.paidSeenA?7:pc===L.paidSeenC?20:21;
    assert.equal(k,target);assert.equal(v,1);
    assert.equal(lastMode,target===7?0:target===20?1:2);
    summary.seen++;
   }
   if(pc===L.paidMetaMiddleTrialMutation){
    assert.equal(k,63);summary.trialMiddle++;
   }
   if(pc===L.paidMiddleRestore){
    assert.equal(k,63);assert.equal(v,cell(61));
    summary.middleRestores++;
   }
   if(pc===L.paidDescriptorBaseAdvance){assert.equal(k,62);summary.baseAdvances++;}
   if(pc===L.paidDescriptorMiddleAdvance){assert.equal(k,63);summary.middleAdvances++;}
   if(pc===L.paidDescriptorTailAdvance){assert.equal(k,64);summary.tailAdvances++;}
   if(pc===L.paidDescriptorIDAdvance){assert.equal(k,65);summary.idAdvances++;}
   if([L.paidRetainBase,L.paidRetainMiddle,L.paidRetainTail].includes(pc)){
    const expectedSlot=pc===L.paidRetainBase?51:
     pc===L.paidRetainMiddle?52:53;
    const source=pc===L.paidRetainBase?62:
     pc===L.paidRetainMiddle?61:64;
    assert.equal(k,expectedSlot);
    assert.equal(v,cell(source));
    assert.equal(cell(11),1);
    summary.retention++;
    retained.push([cell(65),k,v,s.tick]);
   }
   if(pc===L.paidCoverageVerdictService){
    assert.equal(k,11);
    const id=cell(65),desc=[cell(62),cell(61),cell(64)];
    assert.equal(id,verdicts.length);
    assert.deepEqual(desc,[id%4,Math.floor(id/4)%4,Math.floor(id/16)%4]);
    const {modes,safe}=independentCoverage(desc);
    assert.equal(v,Number(safe),'original CIL source-paid certificate wrong');
    const flags=[cell(7),cell(20),cell(21)];
    assert.deepEqual(flags,[Number(modes.includes(0)),
     Number(modes.includes(1)),Number(modes.includes(2))]);
    if(safe){summary.safe++;lastSafe=desc;}else summary.unsafe++;
    verdicts.push({
     id,descriptor:desc,modes,
     native_PID1_paid_witness_flags:flags,
     source_paid_verdict:v,source_paid_step:s.step,
     source_paid_tick:s.tick
    });
   }
   break;
  }
  case 'Add':s.reg[op.dst]=r(op.lhs)+r(op.rhs);s.pc=op.next;break;
  case 'Sub':s.reg[op.dst]=r(op.lhs)-r(op.rhs);s.pc=op.next;break;
  case 'Branch':s.pc=r(op.test)!==0?op.nonzero:op.zero;break;
  case 'Halt':
   assert.equal(pc,L.halt);
   s.halted=true;event='Halted';summary.haltCount++;break;
  default:throw Error('unsupported original CIL primitive '+kind);
 }
 summary.steps++;
 assert.equal(s.credits,30000-s.step);
 assert.ok(s.m.size<=16&&occupied()<=300);
 s.peak=Math.max(s.peak,occupied());
 check(['step',0,s.step,s.tick,1,pc,event,snapshot(),occupied(),cells()],
  'original PID1 step'+s.step+'/pc'+pc);
}
assert.equal(cursor,lines.length-1,'exactly one final native report remains');
assert.equal(verdicts.length,64);
assert.equal(summary.steps,24050);
assert.equal(summary.metaReads,576);
assert.equal(summary.metaWrites,576);
assert.equal(summary.completedMeta,192);
assert.equal(summary.seen,192);
assert.equal(summary.retention,72);
assert.equal(summary.trialMiddle,128);
assert.equal(summary.middleRestores,64);
assert.equal(summary.idAdvances,63);
assert.equal(summary.safe,24);
assert.equal(summary.unsafe,40);
assert.equal(summary.haltCount,1);
assert.equal(s.peak,181);
assert.equal(cell(65),63);
assert.deepEqual(lastSafe,[1,3,2]);
assert.deepEqual([cell(51),cell(52),cell(53)],[1,3,2]);
assert.equal(verdicts[42].source_paid_verdict,1);
assert.deepEqual(verdicts[42].descriptor,[2,2,2]);
assert.deepEqual(verdicts[42].modes,[0,2,1]);
assert.equal(verdicts[48].source_paid_verdict,0);
assert.deepEqual(verdicts[48].descriptor,[0,0,3]);
assert.deepEqual(verdicts[48].modes,[0,0,0]);
const nativeReport={
 study:'B56-F1d1k-G3Q-C0',source_tested_sha:meta[3],
 precommitted_source_nodes:164,
 original_PID1_continuously_alive_for_64_rules:true,
 original_CIL_source_unchanged:true,
 original_PID1_paid_instructions:summary.steps,
 host_interventions:0,
 native_dynamic_meta_opcode_reads:summary.metaReads,
 paid_meta_accumulator_checkpoints:summary.metaWrites,
 paid_completed_three_word_meta_programs:summary.completedMeta,
 paid_seen_flag_writes:summary.seen,
 paid_source_coverage_verdict_services:verdicts.length,
 paid_accepted_rule_retention_writes:summary.retention,
 paid_meta_middle_trial_mutations:summary.trialMiddle,
 paid_meta_middle_restores:summary.middleRestores,
 paid_descriptor_base_advances:summary.baseAdvances,
 paid_descriptor_middle_advances:summary.middleAdvances,
 paid_descriptor_tail_advances:summary.tailAdvances,
 paid_descriptor_ID_advances:summary.idAdvances,
 safe_descriptors:summary.safe,unsafe_descriptors:summary.unsafe,
 source_generated_safe_descriptor_2_2_2_accepted:true,
 source_generated_RESEThai_descriptor_0_0_3_rejected:true,
 PID1_halts_after_full_64_rule_enumeration:true,
 last_retained_safe_descriptor:lastSafe,
 peak_total_storage_units:s.peak,
 final_source_PID1:snapshot(),
 final_all_16_present_or_missing_cells:cells(),
 private_credits_remaining:s.credits,
 all_64_original_PID1_verdicts:verdicts,
 original_PID1_safe_bank_retention_events:retained,
 full_G3Q:false,task_program_services:false,
 feedback_training:false,cycle_safe_meta_repair:false,
 new_primitive_ISA_invention:false,
 Hk:false,B3:false,B4:false,cyberlife:false
};
check(['end',0,nativeReport],'entire original PID1 64-rule native summary');
assert.equal(cursor,lines.length,'no unchecked trailing native trace');
console.log(JSON.stringify({
 study:'B56-F1d1k-G3Q-C0',
 exact_original_source_sha:meta[3],
 frozen_original_CIL_graph_sha256:SHA(FROZEN_BYTES),
 precommitted_protocol_sha256:SHA(PROTOCOL_BYTES),
 native_Rust_literal_source_sha256:SHA(RUST_BYTES),
 original_PID1_count:1,
 separate_original_PID1_restarts:0,
 source_native_original_opcode_count:164,
 original_PID1_charged_steps_fully_reexecuted:summary.steps,
 native_complete_rules_checked:verdicts.length,
 true_coverage_accepted_original_rule_count:summary.safe,
 true_coverage_rejected_original_rule_count:summary.unsafe,
 verified_paid_dynamic_meta_opcode_fetches:summary.metaReads,
 verified_paid_meta_accumulator_writes:summary.metaWrites,
 verified_complete_three_instruction_interpretations:summary.completedMeta,
 verified_original_PID1_seen_flag_writes:summary.seen,
 original_paid_verdict_write11_events:verdicts.length,
 original_safe_only_retention_writes:summary.retention,
 original_paid_middle_opcode_mutations:summary.trialMiddle,
 original_paid_original_rule_restores:summary.middleRestores,
 original_paid_original_rule_index_advances:summary.idAdvances,
 original_paid_base_carries:summary.baseAdvances,
 original_paid_middle_carries:summary.middleAdvances,
 original_paid_tail_carries:summary.tailAdvances,
 original_native_private_credits_remaining:s.credits,
 actual_peak_total_occupied_storage:s.peak,
 all_host_interventions:0,
 original_PID1_unmodified:true,original_native_CIL_source_unmodified:true,
 has_original_meta_2_2_2_acceptance:true,
 has_original_meta_0_0_3_rejection:true,
 unsafe_rules_never_promoted_to_safe_bank:true,
 original_rule_bank_retains_last_safe_1_3_2:true,
 original_PID1_source_halts_after_64:true,
 first_divergence:null,
 task_service_execution_not_in_scope:true,
 full_G3Q:false,Hk:false,B3:false,B4:false,cyberlife:false,
 checked_original_64_rule_verdicts:verdicts,
 source_paid_retention_events:retained
},null,2));
