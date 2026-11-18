#!/usr/bin/env node
// CL-1B B56-F1d1j. Independently replay ALL ORIGINAL native paid CIL events,
// 60-world environment interventions, persistent source-written mutation-rule
// descriptor, future rule-conditioned program edits and corrupt-feedback harm.
// Researcher-authored grammar is NOT spontaneous new semantic invention.
import fs from 'node:fs';
import crypto from 'node:crypto';
import assert from 'node:assert/strict';
import {isDeepStrictEqual} from 'node:util';
const load=p=>fs.readFileSync(new URL(p,import.meta.url));
const native=load('../../cl0/crates/world/tests/cl1b_b56_f1d1j_paid_mutation_rule_memory.rs').toString();
const manifest=load('../studies/B56-F1d1j-exact-177-node-rule-memory-and-60-world-matrix-manifest-v1.md');
const frozenBytes=load('../studies/B56-F1d1j-frozen-exact-177-node-source-v1.json');
const frozen=JSON.parse(frozenBytes);
const sha256=bytes=>crypto.createHash('sha256').update(bytes).digest('hex');
const originalNative=fs.readFileSync('cl1b-ci-evidence/b56_f1d1j_native_paid_mutation_rule_memory.jsonl','utf8').trimEnd().split(/\r?\n/);
const sourceSegment=native.split('fn founder()->Vec<Op>{')[1]?.split('#[derive(Copy,Clone,Debug,Eq,PartialEq)]')[0];
assert.ok(sourceSegment?.includes('vec!['),'native Rust CIL graph literal missing');
const rows=[...sourceSegment.matchAll(/Op::([A-Za-z]+)(?:\{([^}]+)\})?,\s*\/\/ pc (\d+)/g)];
assert.equal(rows.length,177,'unexpected original paid CIL source length');
const op=(name,fields={})=>name==='Halt'?'Halt':{[name]:fields};
const code=rows.map((m,i)=>{
 assert.equal(Number(m[3]),i,'source native PC numbering mismatch');
 const fields=Object.fromEntries([...(m[2]??'').matchAll(/(\w+):(-?\d+)/g)]
  .map(x=>[x[1],Number(x[2])]));
 return op(m[1],fields);
});
assert.deepEqual(code,frozen.code.map(({kind,...fields})=>op(kind,fields)),
 'native source changed relative to BEFORE-IMPLEMENTATION frozen 177 nodes');
assert.deepEqual(frozen.allowed_cell_keys,[7,11,20,21,31,32,33,50,51,52,53,54,56,61,62,63]);
assert.deepEqual(frozen.gate_values,[1,0,2]);
assert.deepEqual(frozen.regimes,['TRUTH','BLIND_SUCCESS','INVERTED','ABSENT','PHASE0_C_DECOY']);
assert.deepEqual(frozen.schedules,[
 ['NEGATE','NEGATE'],['COMPOSED','COMPOSED'],
 ['NEGATE','COMPOSED'],['NEGATE','UNREACHABLE']
]);
assert.deepEqual([frozen.starting_credits,frozen.limit_cells,
 frozen.limit_code_nodes,frozen.limit_total_units],[4000,16,180,300]);
assert.ok(code.every(x=>x==='Halt'||(!x.GraphLaunch&&!x.GraphEncode&&!x.GraphNew)),
 'new graph/process primitive included in frozen source');
const L=frozen.labels;
assert.deepEqual([
 L.initFrozenR1Write,L.fetchOpcode,L.serviceWrite,L.feedbackRead,L.scoreWrite,
 L.paidRuleChange,L.paidRuleShadow,L.paidProgramWrite52,L.paidProgramWrite53,
 L.ruleReadForMutation,L.ruleReadForFallback,L.heldoutStageWrite
],[32,37,59,66,72,96,98,138,140,111,125,152]);
let f=14695981039346656037n;
for(const byte of Buffer.from(JSON.stringify(code),'utf8'))
 f=BigInt.asUintN(64,(f^BigInt(byte))*1099511628211n);
const codeHash=f.toString(16).padStart(16,'0');
let line=0;
const check=(expected,why)=>{
 if(line>=originalNative.length)throw Error('B56-F1d1j native original history truncated '+why);
 const actual=JSON.parse(originalNative[line++]);
 if(!isDeepStrictEqual(expected,actual))
  throw Error('B56-F1d1j FIRST NATIVE DIVERGENCE at line '+line+' '+why+
   '\nEXPECTED '+JSON.stringify(expected)+'\nACTUAL   '+JSON.stringify(actual));
};
const meta=JSON.parse(originalNative[line++]);
assert.deepEqual(meta.slice(0,3),['meta','B56-F1d1j',1]);
if(process.env.GITHUB_SHA)assert.equal(meta[3],process.env.GITHUB_SHA);
assert.equal(meta[4],sha256(manifest));
assert.equal(meta[5],manifest.length);
assert.equal(meta[6],sha256(frozenBytes));
assert.equal(meta[7],frozenBytes.length);
assert.deepEqual(meta.slice(8),[
 60,4000,16,180,300,177,L.fetchOpcode,L.serviceWrite,L.feedbackRead,L.scoreWrite,
 L.paidRuleChange,L.paidRuleShadow,L.paidProgramWrite52,L.paidProgramWrite53,
 L.ruleReadForMutation,L.ruleReadForFallback,L.heldoutStageWrite,L.initFrozenR1Write
]);
const keys=frozen.allowed_cell_keys;
const snap=s=>[s.pc,[...s.reg],s.credits,s.stopped,177,codeHash];
const cell=(s,k)=>s.memory.has(k)?s.memory.get(k):null;
const cells=s=>keys.map(k=>cell(s,k));
const occupied=s=>178+s.memory.size;
const program=s=>[51,52,53,54].map(k=>cell(s,k));
const truth=(task,x)=>task==='NEGATE'?-x:task==='COMPOSED'?-x+2:task==='UNREACHABLE'?x+4:x;
const results=[];
const total={paid:0,external:0,feedback_reads:0,score_writes:0,
 attempt_writes:0,rule_writes:0,rule_shadow:0,original_code_edits:0,
 rule_proposal_reads:0,fallback_reads:0,original_opcode_fetches:0,
 original_accumulator_writes:0,original_service_writes:0,
 initial_rule1_writes:0};
let peak=0;
for(let sid=0;sid<frozen.schedules.length;sid++){
 for(const regime of frozen.regimes){
  for(const gate of frozen.gate_values){
   const id=results.length,tasks=frozen.schedules[sid];
   const s={
    pc:0,reg:[0,0,0,0],credits:4000,stopped:false,memory:new Map(),
    tick:0,steps:0,peak:178,interventions:0,
    services:[],ruleWrites:[],shadowRules:[],programWrites:[],
    ruleProposalReads:[],fallbackReads:[],initialRuleWrites:0,
    feedbackReads:0,scoreWrites:0,attemptWrites:0,opcodeFetches:0,
    accumulatorWrites:0,heldoutCharged:[]
   };
   check(['case',id,sid,regime,gate,tasks,code,codeHash,occupied(s),
      4000,16,180,300],'unchanged original PID1 source world '+id);
   function external(key,value,reason){
    assert.ok([7,20,21].includes(key),'unauthorized host mutation');
    s.tick++;s.interventions++;total.external++;
    s.memory.set(key,value);
    assert.ok(s.memory.size<=16&&occupied(s)<=300);
    s.peak=Math.max(s.peak,occupied(s));
    check(['ext',id,s.tick,key,value,reason,snap(s),occupied(s),cells(s)],
      'strict external environment '+id+'/'+reason);
   }
   external(21,gate,'single_readonly_experiment_gate');
   external(7,-7,'phase0_probe0_input');
   while(!s.stopped){
    const pc=s.pc,raw=code[pc];
    assert.notEqual(raw,undefined,'invalid original CIL PC '+pc);
    assert.ok(s.credits>0,'PID1 ran out of charged budget');
    const kind=raw==='Halt'?'Halt':Object.keys(raw)[0];
    const o=raw==='Halt'?{}:raw[kind];
    const reg=i=>{assert.ok(i>=0&&i<=3);return s.reg[i]};
    s.steps++;total.paid++;s.tick++;s.credits--;
    let event='Executed',newProbe=null,finishedHeldout=null;
    switch(kind){
     case 'Const':s.reg[o.dst]=o.value;s.pc=o.next;break;
     case 'Add':s.reg[o.dst]=reg(o.lhs)+reg(o.rhs);s.pc=o.next;break;
     case 'Sub':s.reg[o.dst]=reg(o.lhs)-reg(o.rhs);s.pc=o.next;break;
     case 'Read':{
      const key=reg(o.address),value=s.memory.get(key)??0;
      s.reg[o.dst]=value;s.pc=o.next;
      event=op('CellRead',{key,value});
      if(pc===L.fetchOpcode){
       const vpc=cell(s,50);assert.ok(vpc>=0&&vpc<=3);
       assert.equal(key,51+vpc);
       assert.equal(value,cell(s,key));
       s.opcodeFetches++;total.original_opcode_fetches++;
      }
      if(pc===L.feedbackRead){
       assert.equal(key,20);
       assert.equal(value,s.memory.get(20)??0);
       s.feedbackReads++;total.feedback_reads++;
      }
      if(pc===L.ruleReadForMutation){
       assert.equal(key,63);
       s.ruleProposalReads.push([cell(s,31),cell(s,33),value,s.tick]);
       total.rule_proposal_reads++;
      }
      if(pc===L.ruleReadForFallback){
       assert.equal(key,63);
       s.fallbackReads.push([cell(s,31),cell(s,33),value,s.tick]);
       total.fallback_reads++;
      }
      break;
     }
     case 'Write':{
      const key=reg(o.address),value=reg(o.value);
      s.memory.set(key,value);s.pc=o.next;
      event=op('CellWritten',{key,value});
      if(pc===L.initFrozenR1Write){
       assert.equal(gate,2);assert.equal(key,63);assert.equal(value,1);
       s.initialRuleWrites++;total.initial_rule1_writes++;
      }
      if(pc===L.paidRuleChange){
       assert.equal(gate,1);assert.equal(key,63);
       s.ruleWrites.push([cell(s,31),cell(s,33),value,s.tick]);
       total.rule_writes++;
      }
      if(pc===L.paidRuleShadow){
       assert.ok(gate===0||gate===2);assert.equal(key,56);
       s.shadowRules.push([cell(s,31),cell(s,33),value,s.tick]);
       total.rule_shadow++;
      }
      if(pc===L.paidProgramWrite52||pc===L.paidProgramWrite53){
       assert.equal(key,pc===L.paidProgramWrite52?52:53);
       s.programWrites.push([cell(s,31),cell(s,33),key,value,s.tick]);
       total.original_code_edits++;
      }
      if(pc===L.scoreWrite){assert.equal(key,61);s.scoreWrites++;total.score_writes++;}
      if(pc===L.attemptWrite){assert.equal(key,62);s.attemptWrites++;total.attempt_writes++;}
      if(pc===L.accWrite){assert.equal(key,56);s.accumulatorWrites++;total.original_accumulator_writes++;}
      if(key===11){
       assert.equal(pc,L.serviceWrite);
       total.original_service_writes++;
       const phase=cell(s,31),stage=cell(s,32),mode=cell(s,33);
       assert.ok(phase>=0&&phase<2&&stage>=0&&stage<=2&&mode>=0&&mode<=2);
       s.services.push([phase,stage,mode,value,program(s),cell(s,63)]);
       if(stage<2)newProbe=[phase,stage,mode,value];
       else{finishedHeldout=phase;s.heldoutCharged.push(s.steps);}
      }
      break;
     }
     case 'Branch':s.pc=reg(o.test)!==0?o.nonzero:o.zero;break;
     case 'Halt':s.stopped=true;event='Halted';break;
     default:throw Error('unaccounted native primitive '+kind);
    }
    assert.equal(s.credits,4000-s.steps);
    assert.ok(s.steps<=4000&&s.memory.size<=16&&occupied(s)<=300);
    s.peak=Math.max(s.peak,occupied(s));peak=Math.max(peak,s.peak);
    check(['step',id,s.steps,s.tick,1,pc,event,snap(s),occupied(s),cells(s)],
     'original PID1 charged instruction '+id+'/'+s.steps+' PC'+pc);
    if(newProbe){
     const [phase,stage,mode,value]=newProbe;
     const correct=Number(value===truth(tasks[phase],[-7,5][stage]));
     const bit=regime==='TRUTH'?correct:
      regime==='BLIND_SUCCESS'?1:
      regime==='INVERTED'?1-correct:
      regime==='ABSENT'?null:
      phase===0&&mode===1?1:correct;
     if(bit!==null)external(20,bit,'post_real_probe_feedback');
     external(7,stage===0?5:-7,'next_probe_default_input');
    }
    if(pc===L.heldoutStageWrite){
     assert.equal(kind,'Write');assert.equal(cell(s,32),2);
     external(7,11,'actor_requested_heldout_input');
    }
    if(finishedHeldout===0)external(7,-7,'phase1_new_task_input');
   }
   assert.equal(cell(s,31),2);
   assert.equal(s.heldoutCharged.length,2);
   assert.equal(s.initialRuleWrites,gate===2?1:0);
   if(gate===1)assert.equal(s.shadowRules.length,0);
   else{
    assert.equal(s.ruleWrites.length,0);
    assert.equal(cell(s,63),gate===2?1:0);
   }
   const heldouts=s.services.filter(v=>v[1]===2);
   assert.equal(heldouts.length,2,'paid heldout required each phase');
   const phases=heldouts.map(([phase,stage,mode,value,bytecode,rule])=>({
    phase,task:tasks[phase],retained_mode:mode,
    output:value,oracle_value:truth(tasks[phase],11),
    correct:value===truth(tasks[phase],11),
    bytecode,learned_rule:rule
   }));
   const correct=phases.filter(x=>x.correct).length;
   if(regime==='TRUTH'){
    assert.equal(correct,sid===3?1:2);
    if(sid===0&&gate===1){
     assert.equal(cell(s,63),1);
     assert.equal(s.ruleProposalReads.find(x=>x[0]===1)[2],1);
    }
    if(sid===3){
     assert.equal(phases[1].correct,false);
     assert.equal(phases[1].oracle_value,15);
     assert.ok(s.fallbackReads.length>0);
     if(gate===1){
      assert.equal(phases[1].retained_mode,2);
      assert.equal(phases[1].output,-11);
     }
    }
   }
   const costs=[s.heldoutCharged[0],s.heldoutCharged[1]-s.heldoutCharged[0]];
   const result={
    id,schedule_id:sid,signal:regime,gate,
    two_phase_actual_service_events:s.services,
    heldout_phase_results:phases,
    heldout_correct_count:correct,
    rule_write_events:s.ruleWrites,
    rule_shadow_events:s.shadowRules,
    original_PID1_paid_program_slot_write_events:s.programWrites,
    original_paid_rule_proposal_reads:s.ruleProposalReads,
    original_paid_fallback_rule_reads:s.fallbackReads,
    paid_initial_rule1_writes:s.initialRuleWrites,
    paid_feedback_reads:s.feedbackReads,
    paid_score_writes:s.scoreWrites,
    paid_failed_candidate_writes:s.attemptWrites,
    dynamic_opcode_fetches:s.opcodeFetches,
    paid_accumulator_writes:s.accumulatorWrites,
    phase_costs_at_heldout_services:costs,
    charged_steps:s.steps,
    external_interventions:s.interventions,
    max_total_storage:s.peak,
    final_rule:cell(s,63),
    final_process:snap(s),final_cells:cells(s),
    remaining_credits:s.credits,
    same_PID1:true,original_code_graph_unchanged:true,
    invented_new_mutation_grammar:false,
    b3_met:false,b4_met:false,cyberlife:false
   };
   check(['end',id,result],'source-origin actor-meta rule and heldout case'+id);
   results.push(result);
  }
 }
}
assert.equal(line,originalNative.length,'unchecked raw original native CIL tail');
assert.equal(results.length,60);
function choose(sid,regime,gate){return results.find(x=>x.schedule_id===sid&&x.signal===regime&&x.gate===gate)}
const s0=choose(0,'TRUTH',1),s0fixed=choose(0,'TRUTH',0),
 s0oracle=choose(0,'TRUTH',2),s0decoy=choose(0,'PHASE0_C_DECOY',1);
assert.equal(s0.heldout_correct_count,2);
assert.equal(s0fixed.heldout_correct_count,2);
assert.ok(s0.phase_costs_at_heldout_services[1]<
 s0fixed.phase_costs_at_heldout_services[1],
 'learned R1 must save original paid CIL compared with fixed R0');
assert.ok(s0oracle.charged_steps<s0.charged_steps,
 'a priori privileged fixed R1 control must remain cheaper');
assert.equal(s0.original_paid_rule_proposal_reads.find(x=>x[0]===1)[2],1);
assert.equal(s0decoy.original_paid_rule_proposal_reads.find(x=>x[0]===1)[2],0);
const p1Truth=s0.original_PID1_paid_program_slot_write_events.filter(x=>x[0]===1);
const p1Decoy=s0decoy.original_PID1_paid_program_slot_write_events.filter(x=>x[0]===1);
assert.deepEqual(p1Truth.slice(0,2).map(x=>[x[1],x[2],x[3]]),
 [[2,52,1],[2,53,3]]);
assert.deepEqual(p1Decoy.slice(0,2).map(x=>[x[1],x[2],x[3]]),
 [[1,52,1],[1,53,2]]);
assert.equal(s0.heldout_phase_results[1].correct,true);
assert.equal(s0decoy.heldout_phase_results[0].correct,false);
assert.equal(choose(1,'TRUTH',1).heldout_correct_count,2);
assert.ok(choose(1,'TRUTH',1).phase_costs_at_heldout_services[1]<
 choose(1,'TRUTH',2).phase_costs_at_heldout_services[1]);
const transfer=choose(2,'TRUTH',1),transferFixed=choose(2,'TRUTH',0);
assert.equal(transfer.heldout_correct_count,2);
assert.ok(transfer.phase_costs_at_heldout_services[1]>
 transferFixed.phase_costs_at_heldout_services[1],'negative transfer must not disappear');
const impossible=choose(3,'TRUTH',1);
assert.equal(impossible.heldout_correct_count,1);
assert.equal(impossible.heldout_phase_results[1].output,-11);
assert.equal(impossible.heldout_phase_results[1].oracle_value,15);
assert.ok(impossible.original_paid_fallback_rule_reads.some(x=>x[0]===1&&x[2]===1));
const paidRange=results.map(x=>x.charged_steps);
const sumCorrect=results.filter(x=>x.gate===1&&x.signal==='TRUTH')
 .reduce((n,x)=>n+x.heldout_correct_count,0);
console.log(JSON.stringify({
 study:'B56-F1d1j',
 native_source_SHA:meta[3],
 frozen_source_manifest_sha256:sha256(manifest),
 frozen_exact_opcodes_sha256:sha256(frozenBytes),
 native_Rust_file_sha256:sha256(Buffer.from(native)),
 source_reconstructed_original_CIL_nodes:177,
 original_worlds_independently_reexecuted:results.length,
 actual_paid_native_CIL_instructions_replayed:total.paid,
 independent_external_interventions:total.external,
 paid_feedback_reads:total.feedback_reads,
 paid_source_score_writes:total.score_writes,
 paid_failed_candidate_cost_surrogate_updates:total.attempt_writes,
 actor_paid_true_mutation_rule_updates:total.rule_writes,
 actor_paid_suppressed_rule_shadow_writes:total.rule_shadow,
 actual_program_bytecode_slot_edits:total.original_code_edits,
 paid_rule_reads_for_future_mutations:total.rule_proposal_reads,
 source_paid_fallback_rule_reads:total.fallback_reads,
 dynamic_virtual_opcode_fetches:total.original_opcode_fetches,
 paid_accumulator_checkpoint_writes:total.original_accumulator_writes,
 real_PID1_service_outputs:total.original_service_writes,
 source_actor_paid_initial_R1_control_writes:total.initial_rule1_writes,
 original_charged_step_range:[Math.min(...paidRange),Math.max(...paidRange)],
 peak_total_storage:peak,
 first_divergence:null,
 original_PID1_unchanged:true,
 original_native_executable_code_unchanged:true,
 genuine_persistent_data_encoded_rule_changes:true,
 corrupted_training_feedback_changes_later_program_slot_edit:true,
 original_two_probe_candidate_cost_surrogate_not_process_credit_sensor:true,
 truth_repeated_negate_learned_rule_phase1_cost:s0.phase_costs_at_heldout_services[1],
 truth_repeated_negate_frozen_R0_phase1_cost:s0fixed.phase_costs_at_heldout_services[1],
 truth_repeated_negate_oracle_frozen_R1_total_cost:s0oracle.charged_steps,
 truth_composed_phase1_mutable_cost:choose(1,'TRUTH',1).phase_costs_at_heldout_services[1],
 truth_composed_phase1_frozen_R1_cost:choose(1,'TRUTH',2).phase_costs_at_heldout_services[1],
 truth_shift_negative_transfer_mutable_phase1_cost:transfer.phase_costs_at_heldout_services[1],
 truth_shift_negative_transfer_frozen_R0_phase1_cost:transferFixed.phase_costs_at_heldout_services[1],
 truth_successful_heldout_per_8:sumCorrect,
 truth_impossible_task_fallback_wrong:true,
 learned_rule_semantics_and_primitive_ops_researcher_authored:true,
 authentic_B53_truth_provenance:false,
 original_F1c_founder_continuity:false,
 strict_N56_A_B_C:false,B3:false,B4:false,cyberlife:false,
 by_case:results.map(({
  two_phase_actual_service_events,original_PID1_paid_program_slot_write_events,
  final_process,final_cells,rule_write_events,rule_shadow_events,
  original_paid_rule_proposal_reads,original_paid_fallback_rule_reads,...rest
 })=>({...rest,rule_write_events,rule_shadow_events,
     original_paid_rule_proposal_reads,original_paid_fallback_rule_reads}))
},null,2));
