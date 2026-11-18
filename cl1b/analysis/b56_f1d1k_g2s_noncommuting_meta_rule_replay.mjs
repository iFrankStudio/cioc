#!/usr/bin/env node
// CL-1B B56-F1d1k-G2S independent ORIGINAL native CIL source parser,
// 180-world instruction-for-instruction strict PID1 replay and malformed4.
// Bounded researcher-authored grammar; no life/novel primitive claim.
import fs from 'node:fs';
import crypto from 'node:crypto';
import assert from 'node:assert/strict';
import {isDeepStrictEqual} from 'node:util';
const load=p=>fs.readFileSync(new URL(p,import.meta.url));
const rustBuf=load('../../cl0/crates/world/tests/cl1b_b56_f1d1k_g2s_noncommuting_meta_rule.rs');
const rust=rustBuf.toString();
const frozenBuf=load('../studies/B56-F1d1k-G2S-frozen-exact-177-node-source-v1.json');
const protocolBuf=load('../studies/B56-F1d1k-G2S-native-180-world-and-noncommuting-RESET-preregistration.md');
const frozen=JSON.parse(frozenBuf);
const SHA=b=>crypto.createHash('sha256').update(b).digest('hex');
const snippet=rust.split('fn founder()->Vec<Op>{')[1]?.split('#[derive(Copy,Clone,Debug,Eq,PartialEq)]')[0];
assert.ok(snippet?.includes('vec!['),'source-native founder literal not found');
const m=[...snippet.matchAll(/Op::([A-Za-z]+)(?:\{([^}]+)\})?,\s*\/\/ pc (\d+)/g)];
assert.equal(m.length,177,'original CIL source 177-node freeze mismatch');
const wrap=(name,data={})=>name==='Halt'?'Halt':{[name]:data};
const code=m.map((row,i)=>{
 assert.equal(Number(row[3]),i,'actual native Rust source-PC comment divergence');
 const args=Object.fromEntries([...(row[2]??'').matchAll(/(\w+):(-?\d+)/g)]
  .map(x=>[x[1],Number(x[2])]));
 return wrap(row[1],args);
});
assert.deepEqual(code,frozen.code.map(({kind,...args})=>wrap(kind,args)),
 'native CIL graph differs from precommitted frozen source');
const L=frozen.labels;
assert.deepEqual([
 L.metaOpcodeFetch,L.serviceWrite,L.taskFirstOpcodeFetch,L.taskSecondOpcodeFetch,
 L.paidBaseWrite,L.paidTailWrite,L.paidResetOffset,
 L.paidMalformedRule,L.fallbackStageWrite,L.heldoutStageWrite,L.halt
],[21,83,73,78,118,120,123,12,158,161,176]);
const KEYS=[7,11,20,21,31,32,33,50,52,53,56,61,62,63,64,65];
assert.deepEqual(frozen.allowed_cell_keys,KEYS);
assert.equal(frozen.total_original_worlds,180);
assert.equal(frozen.task_outcome_worlds,120);
assert.equal(frozen.malformed_only_worlds,60);
assert.deepEqual(frozen.feedback_regimes,
 ['TRUTH','BLIND_SUCCESS','INVERTED','ABSENT','PHASE0_C_DECOY']);
assert.deepEqual(frozen.resources,{
 starting_credits:4000,max_data_cells:16,max_CIL_nodes:180,
 max_occupied_units:300,one_PID_only:true
});
assert.ok(code.every(x=>x==='Halt'||(!x.GraphLaunch&&!x.GraphNew&&!x.GraphEncode)));
let h=14695981039346656037n;
for(const byte of Buffer.from(JSON.stringify(code),'utf8'))
 h=BigInt.asUintN(64,(h^BigInt(byte))*1099511628211n);
const founderHash=h.toString(16).padStart(16,'0');
const rawPath='cl1b-ci-evidence/b56_f1d1k_g2s_native_noncommuting_and_bad4.jsonl';
if(!fs.existsSync(rawPath))throw Error('Missing ORIGINAL native PID1 G2S JSONL; no replay allowed');
const lines=fs.readFileSync(rawPath,'utf8').trimEnd().split(/\r?\n/);
let cursor=0;
function compare(expected,what){
 if(cursor>=lines.length)throw Error('original G2S evidence ends early at '+what);
 const got=JSON.parse(lines[cursor++]);
 if(!isDeepStrictEqual(expected,got))throw Error(
  'G2S FIRST NATIVE DIVERGENCE at JSONL line '+cursor+' '+what+
  '\nEXPECTED '+JSON.stringify(expected)+'\nORIGINAL '+JSON.stringify(got));
}
const meta=JSON.parse(lines[cursor++]);
assert.deepEqual(meta.slice(0,3),['meta','B56-F1d1k-G2S',1]);
if(process.env.GITHUB_SHA)assert.equal(meta[3],process.env.GITHUB_SHA);
assert.equal(meta[4],SHA(protocolBuf));
assert.equal(meta[5],protocolBuf.length);
assert.equal(meta[6],SHA(frozenBuf));
assert.equal(meta[7],frozenBuf.length);
assert.deepEqual(meta.slice(8),[
 180,4000,16,180,300,177,L.metaOpcodeFetch,
 L.metaCheckpoint,L.metaModeWrite,L.taskFirstOpcodeFetch,L.taskSecondOpcodeFetch,
 L.taskAcc,L.taskWrite52,L.taskWrite53,L.serviceWrite,L.feedbackRead,
 L.scoreWrite,L.paidBaseWrite,L.paidTailWrite,L.paidResetOffset,
 L.paidShadow1,L.paidShadow2,L.paidShadow3,L.paidOffsetWrite,
 L.attemptCountWrite,L.paidFallbackOffset,L.heldoutStageWrite,
 L.fallbackStageWrite,L.paidMalformedRule,L.initBase,L.initTail,L.halt]);
const snapshot=s=>[s.pc,[...s.reg],s.credits,s.stopped,177,founderHash];
const cell=(s,k)=>s.data.has(k)?s.data.get(k):null;
const cells=s=>KEYS.map(k=>cell(s,k));
const occupied=s=>178+s.data.size;
const oracle=(task,x)=>task==='NEGATE'?-x:
 task==='COMPOSED'?-x+2:task==='IDENTITY'?x:x+4;
const results=[],global={original_paid:0,host_interventions:0,
 meta_fetch:0,meta_checkpoints:0,meta_compositions:0,
 task_fetch:0,task_checkpoint:0,task_bytecode_edits:0,
 persistent_rule_paid_writes:0,effective_base_changes:0,
 effective_tail_changes:0,temporary_rule_edits:0,
 paid_shadow_writes:0,paid_candidate_attempts:0,
 actual_feedback_reads:0,actual_score_writes:0,
 source_created_invalid4_writes:0,
 original_real_outputs:0,actual_fallbacks:0};
let globalPeak=0;
for(const seed of frozen.two_probe_seeds){
 for(let sid=0;sid<frozen.schedules.length;sid++){
  const schedule=frozen.schedules[sid];
  for(const regime of frozen.feedback_regimes){
   for(const ctl of frozen.experimental_gates){
    const gate=ctl.value,id=results.length;
    const s={pc:0,reg:[0,0,0,0],data:new Map(),tick:0,credits:4000,
     stopped:false,paid:0,externalEvents:0,peak:178,
     service:[],metaFetch:[],metaCheckpoint:0,metaPrograms:[],
     taskFetch:0,taskCheckpoint:0,taskWrites:[],metaWrites:[],
     shadows:0,attempts:0,feedbackReads:0,scoreWrites:0,
     malformedWrites:0,invalidHalt:false,initMetaWrites:0,
     fallback:0,paidAtHeldout:[]};
    compare(['case',id,seed.id,sid,regime,gate,schedule,
     seed.probes,seed.heldout,code,founderHash,occupied(s),4000,16,180,300],
     'strict unmutated native original process source world'+id);
    const external=(key,value,reason)=>{
     assert.ok([7,20,21].includes(key),'original host edited forbidden key'+key);
     s.tick++;s.externalEvents++;global.host_interventions++;
     s.data.set(key,value);
     assert.ok(s.data.size<=16&&occupied(s)<=300);
     s.peak=Math.max(s.peak,occupied(s));
     compare(['ext',id,s.tick,key,value,reason,snapshot(s),
      occupied(s),cells(s)],'original host SetCell'+id+'/'+reason);
    };
    external(21,gate,'initial_rule_control');
    external(7,seed.probes[0],'first_probe_input');
    while(!s.stopped){
     const pc=s.pc,raw=code[pc];
     assert.notEqual(raw,undefined,'native PC outside frozen original CIL');
     assert.ok(s.credits>0,'unaccounted finite compute grant');
     const kind=raw==='Halt'?'Halt':Object.keys(raw)[0],
      o=raw==='Halt'?{}:raw[kind];
     const r=i=>{assert.ok(i>=0&&i<=3);return s.reg[i]};
     const before=key=>cell(s,key);
     s.paid++;s.tick++;s.credits--;global.original_paid++;
     let event='Executed',justProbe=null,finishedPhase=null;
     switch(kind){
      case 'Const':s.reg[o.dst]=o.value;s.pc=o.next;break;
      case 'Add':s.reg[o.dst]=r(o.lhs)+r(o.rhs);s.pc=o.next;break;
      case 'Sub':s.reg[o.dst]=r(o.lhs)-r(o.rhs);s.pc=o.next;break;
      case 'Branch':s.pc=r(o.test)!==0?o.nonzero:o.zero;break;
      case 'Read':{
       const key=r(o.address),value=s.data.get(key)??0;
       s.reg[o.dst]=value;s.pc=o.next;
       event=wrap('CellRead',{key,value});
       if(pc===L.metaOpcodeFetch){
        const vpc=cell(s,50);assert.ok(vpc>=0&&vpc<=2);
        assert.equal(key,62+vpc);
        assert.equal(value,s.data.get(key)??0);
        s.metaFetch.push([cell(s,31),vpc,key,value,s.tick]);
        global.meta_fetch++;
       }
       if(pc===L.taskFirstOpcodeFetch||pc===L.taskSecondOpcodeFetch){
        assert.equal(key,pc===L.taskFirstOpcodeFetch?52:53);
        s.taskFetch++;global.task_fetch++;
       }
       if(pc===L.feedbackRead){
        assert.equal(key,20);assert.equal(value,s.data.get(20)??0);
        s.feedbackReads++;global.actual_feedback_reads++;
       }
       break;
      }
      case 'Write':{
       const key=r(o.address),value=r(o.value),prev=before(key);
       s.data.set(key,value);s.pc=o.next;
       event=wrap('CellWritten',{key,value});
       if(pc===L.initBase||pc===L.initTail){
        assert.equal(key,pc===L.initBase?62:64);
        assert.equal(value,pc===L.initBase?1:0);
        s.initMetaWrites++;
       }
       if(pc===L.paidMalformedRule){
        assert.equal(gate,2);assert.equal(key,64);assert.equal(value,4);
        s.malformedWrites++;global.source_created_invalid4_writes++;
       }
       if(pc===L.metaCheckpoint){
        assert.equal(key,56);
        s.metaCheckpoint++;global.meta_checkpoints++;
       }
       if(pc===L.metaModeWrite){
        assert.equal(key,33);assert.ok(value>=0&&value<=2);
        s.metaPrograms.push([cell(s,31),cell(s,62),cell(s,63),
         cell(s,64),value,s.tick]);
        global.meta_compositions++;
       }
       if(pc===L.taskAcc){
        assert.equal(key,56);
        s.taskCheckpoint++;global.task_checkpoint++;
       }
       if(pc===L.taskWrite52||pc===L.taskWrite53){
        assert.equal(key,pc===L.taskWrite52?52:53);
        s.taskWrites.push([cell(s,31),cell(s,33),key,value,s.tick]);
        global.task_bytecode_edits++;
       }
       if([L.paidBaseWrite,L.paidTailWrite,L.paidResetOffset,
           L.paidOffsetWrite,L.paidFallbackOffset].includes(pc)){
        const expect=pc===L.paidBaseWrite?62:pc===L.paidTailWrite?64:63;
        assert.equal(key,expect);
        if([L.paidBaseWrite,L.paidTailWrite,L.paidResetOffset].includes(pc)){
         assert.equal(gate,1);
         global.persistent_rule_paid_writes++;
         if(pc===L.paidBaseWrite&&prev!==value)global.effective_base_changes++;
         if(pc===L.paidTailWrite&&prev!==value)global.effective_tail_changes++;
        }else global.temporary_rule_edits++;
        if(pc===L.paidFallbackOffset){s.fallback++;global.actual_fallbacks++;}
        s.metaWrites.push([cell(s,31),cell(s,33),key,value,pc,s.tick]);
       }
       if([L.paidShadow1,L.paidShadow2,L.paidShadow3].includes(pc)){
        assert.equal(gate,0);assert.equal(key,56);
        s.shadows++;global.paid_shadow_writes++;
       }
       if(pc===L.attemptCountWrite){
        assert.equal(key,65);
        s.attempts++;global.paid_candidate_attempts++;
       }
       if(pc===L.scoreWrite){
        assert.equal(key,61);
        s.scoreWrites++;global.actual_score_writes++;
       }
       if(key===11){
        assert.equal(pc,L.serviceWrite);
        const phase=cell(s,31)??0,stage=cell(s,32)??0,mode=cell(s,33);
        assert.ok(phase>=0&&phase<=1&&stage>=0&&stage<=2&&mode>=0&&mode<=2);
        s.service.push([phase,stage,mode,value,cell(s,62),
         cell(s,63),cell(s,64),cell(s,52),cell(s,53)]);
        global.original_real_outputs++;
        if(stage<2)justProbe=[phase,stage,mode,value];
        else {finishedPhase=phase;s.paidAtHeldout.push(s.paid);}
       }
       break;
      }
      case 'Halt':
       s.stopped=true;event='Halted';
       if(gate===2){
        assert.equal(pc,L.halt);
        s.invalidHalt=true;
       }
       break;
      default:throw Error('native source contains unknown CIL primitive '+kind);
     }
     assert.equal(s.credits,4000-s.paid,'private CIL charge mismatch');
     assert.ok(s.paid<=4000&&s.data.size<=16&&occupied(s)<=300);
     s.peak=Math.max(s.peak,occupied(s));globalPeak=Math.max(globalPeak,s.peak);
     compare(['step',id,s.paid,s.tick,1,pc,event,snapshot(s),
      occupied(s),cells(s)],'original paid PID1 CIL step '+id+'/'+s.paid+' PC'+pc);
     if(justProbe){
      const [phase,stage,mode,actual]=justProbe;
      const correct=Number(actual===oracle(schedule[phase],seed.probes[stage]));
      const bit=regime==='TRUTH'?correct:
       regime==='INVERTED'?1-correct:regime==='BLIND_SUCCESS'?1:
       regime==='ABSENT'?null:phase===0&&mode===1?1:correct;
      if(bit!==null)external(20,bit,'paid_probe_feedback');
      external(7,stage===0?seed.probes[1]:seed.probes[0],'next_probe_input');
     }
     if(pc===L.heldoutStageWrite||pc===L.fallbackStageWrite){
      assert.equal(kind,'Write');assert.equal(cell(s,32),2);
      external(7,seed.heldout,pc===L.heldoutStageWrite?
       'valid_candidate_heldout':'fallback_heldout');
     }
     if(finishedPhase===0)
      external(7,seed.probes[0],'next_hidden_task_probe0');
    }
    assert.equal(s.initMetaWrites,2);
    assert.ok(s.peak<=194);
    const held=s.service.filter(x=>x[1]===2);
    const phases=held.map(x=>{
     const phase=x[0],output=x[3];
     return {phase,task:schedule[phase],mode:x[2],output,
      expected:oracle(schedule[phase],seed.heldout),
      correct:output===oracle(schedule[phase],seed.heldout),
      meta_bytecode:[x[4],x[5],x[6]],task_bytecode:[x[7],x[8]]};
    });
    const correct=phases.filter(x=>x.correct).length;
    if(gate===2){
     assert.equal(s.malformedWrites,1);
     assert.equal(s.invalidHalt,true);
     assert.equal(s.service.length,0);
     assert.equal(s.feedbackReads,0);
     assert.equal(s.metaFetch.length,3);
     assert.equal(s.metaFetch[2][3],4);
     assert.equal(s.metaWrites.length,0);
     assert.equal(cell(s,64),4);
     assert.equal(phases.length,0);
    }else{
     assert.equal(cell(s,31),2);
     assert.equal(phases.length,2);
     assert.equal(s.paidAtHeldout.length,2);
     assert.ok(s.metaFetch.length>=6&&s.metaFetch.length%3===0);
     assert.equal(s.metaCheckpoint,s.metaFetch.length);
     assert.equal(s.metaPrograms.length*3,s.metaFetch.length);
     assert.equal(s.taskFetch,s.service.length*2);
     assert.equal(s.taskCheckpoint,s.service.length);
     if(gate===0){
      assert.equal(s.shadows%3,0);
      assert.ok(s.metaWrites.every(x=>x[2]===63));
      assert.equal(cell(s,62),1);
      assert.equal(cell(s,64),0);
     }else assert.equal(s.shadows,0);
     if(regime==='TRUTH'&&gate===1){
      assert.equal(correct,[2,2,2,1,1,2][sid]);
      if(sid===4){
       assert.equal(phases[1].correct,false);
       assert.equal(phases[1].mode,2);
       assert.equal(phases[1].expected,seed.heldout);
       assert.ok(s.fallback>0);
      }
      if(sid===3)assert.equal(phases[1].correct,false);
     }
    }
    const phaseCosts=s.paidAtHeldout.length===2?
     [s.paidAtHeldout[0],s.paidAtHeldout[1]-s.paidAtHeldout[0]]:[];
    const result={
     id,seed:seed.id,schedule:sid,regime,gate,
     real_original_PID1_service_outputs:s.service,
     true_heldout_by_phase:phases,true_heldout_correct:correct,
     paid_meta_instruction_fetches:s.metaFetch,
     paid_meta_intermediate_checkpoints:s.metaCheckpoint,
     charged_three_instruction_meta_compositions:s.metaPrograms,
     charged_task_op_fetches:s.taskFetch,
     paid_task_accumulator_checkpoints:s.taskCheckpoint,
     real_task_program_slot_writes:s.taskWrites,
     actor_paid_meta_data_writes:s.metaWrites,
     paid_meta_write_ablations:s.shadows,
     source_paid_candidate_attempt_writes:s.attempts,
     source_paid_feedback_reads:s.feedbackReads,
     source_paid_score_writes:s.scoreWrites,
     source_generated_bad4_writes:s.malformedWrites,
     source_generated_bad4_halt:s.invalidHalt,
     source_paid_fallback_entries:s.fallback,
     phase_costs_to_real_heldout:phaseCosts,
     paid_CIL_instructions:s.paid,
     external_interventions:s.externalEvents,
     measured_peak_storage:s.peak,
     final_original_PID1:snapshot(s),
     final_present_or_missing_cells:cells(s),
     remaining_private_credits:s.credits,
     unaltered_original_PID1:true,native_source_unchanged:true,
     preauthored_meta_grammar:true,universal_bad_opcode_handling:false,
     full_Hk:false,B3:false,B4:false,cyberlife:false
    };
    compare(['end',id,result],'complete original paid native run '+id);
    results.push(result);
   }
  }
 }
}
assert.equal(results.length,180);
assert.equal(cursor,lines.length,'original native JSONL has unchecked trailing events');
const pick=(seed,sid,sig,gate)=>results.find(x=>x.seed===seed
 &&x.schedule===sid&&x.regime===sig&&x.gate===gate);
let correctEnabled=0;
const truthControls=[];
for(let seed=0;seed<2;seed++){
 const s0=pick(seed,0,'TRUTH',1),s0control=pick(seed,0,'TRUTH',0);
 const s2=pick(seed,2,'TRUTH',1),s2control=pick(seed,2,'TRUTH',0);
 const s4=pick(seed,4,'TRUTH',1),s4control=pick(seed,4,'TRUTH',0);
 const s5=pick(seed,5,'TRUTH',1);
 assert.equal(s4.true_heldout_correct,1);
 assert.equal(s4control.true_heldout_correct,2);
 assert.deepEqual(s4.charged_three_instruction_meta_compositions
  .filter(x=>x[0]===1).map(x=>x[4]),[2,1,2,2]);
 assert.deepEqual(s5.charged_three_instruction_meta_compositions
  .filter(x=>x[0]===1).map(x=>x[4]),[0,0,1]);
 assert.equal(s2.true_heldout_correct,2);
 assert.equal(s2control.true_heldout_correct,1);
 for(let sid=0;sid<6;sid++){
  const t=pick(seed,sid,'TRUTH',1);
  assert.equal(t.true_heldout_correct,[2,2,2,1,1,2][sid]);
  correctEnabled+=t.true_heldout_correct;
  const bad=pick(seed,sid,'TRUTH',2);
  assert.equal(bad.source_generated_bad4_halt,true);
  assert.equal(bad.source_generated_bad4_writes,1);
  assert.deepEqual(bad.real_original_PID1_service_outputs,[]);
 }
 truthControls.push({seed,
  repeated_NEGATE_learned_phase1_cost:s0.phase_costs_to_real_heldout[1],
  repeated_NEGATE_shadow_phase1_cost:s0control.phase_costs_to_real_heldout[1],
  NEGATE_to_COMPOSED_learned_correct:s2.true_heldout_correct,
  NEGATE_to_COMPOSED_shadow_correct:s2control.true_heldout_correct,
  NEGATE_to_IDENTITY_learned_correct:s4.true_heldout_correct,
  NEGATE_to_IDENTITY_shadow_correct:s4control.true_heldout_correct,
  IDENTITY_to_COMPOSED_learned_second_phase_seq:
   s5.charged_three_instruction_meta_compositions.filter(x=>x[0]===1).map(x=>x[4])});
}
assert.equal(correctEnabled,20);
const metaPairs=new Set(),normalCaseSteps=[],brokenCaseSteps=[];
for(const x of results){
 for(const program of x.charged_three_instruction_meta_compositions)
  metaPairs.add([program[1],program[2],program[3]].join(','));
 (x.gate===2?brokenCaseSteps:normalCaseSteps).push(x.paid_CIL_instructions);
}
console.log(JSON.stringify({
 study:'B56-F1d1k-G2S',
 original_run_source_sha:meta[3],
 precommitted_source_graph_sha256:SHA(frozenBuf),
 precommitted_protocol_sha256:SHA(protocolBuf),
 original_rust_source_sha256:SHA(rustBuf),
 original_CIL_nodes:177,
 original_worlds_independently_reexecuted:180,
 normal_task_worlds_independently_reexecuted:120,
 actor_generated_invalid4_worlds_independently_reexecuted:60,
 paid_native_PID1_instructions_reexecuted:global.original_paid,
 original_host_interventions_reexecuted:global.host_interventions,
 dynamic_three_word_meta_opcode_fetches:global.meta_fetch,
 real_paid_meta_checkpoints:global.meta_checkpoints,
 source_paid_complete_three_word_meta_programs:global.meta_compositions,
 real_paid_task_opcode_reads:global.task_fetch,
 actual_paid_task_accumulator_checkpoints:global.task_checkpoint,
 actual_task_program_writes:global.task_bytecode_edits,
 actor_paid_validated_meta_descriptor_writes:global.persistent_rule_paid_writes,
 actual_effective_base_opcode_changes:global.effective_base_changes,
 actual_effective_tail_opcode_changes:global.effective_tail_changes,
 actor_paid_temporary_meta_offsets_and_rollbacks:global.temporary_rule_edits,
 actor_paid_shadow_meta_writes:global.paid_shadow_writes,
 actor_paid_candidate_failure_counter_writes:global.paid_candidate_attempts,
 paid_feedback_reads:global.actual_feedback_reads,
 paid_score_writes:global.actual_score_writes,
 actual_original_PID1_real_services:global.original_real_outputs,
 actual_fallback_branch_events:global.actual_fallbacks,
 actual_actor_source_generated_bad4_writes:global.source_created_invalid4_writes,
 checked_original_invalid4_halts_without_services:60,
 observed_distinct_three_word_meta_descriptor_strings:[...metaPairs].sort(),
 number_of_distinct_three_word_meta_descriptor_strings:metaPairs.size,
 original_normal_world_paid_step_range:[
  Math.min(...normalCaseSteps),Math.max(...normalCaseSteps)],
 original_malformed_world_paid_step_range:[
  Math.min(...brokenCaseSteps),Math.max(...brokenCaseSteps)],
 maximum_observed_storage_units:globalPeak,
 first_divergence:null,
 original_PID1_unchanged_in_each_world:true,
 immutable_original_CIL_founder:true,
 original_VM_physics_unchanged:true,
 truthful_enabled_heldout_correct:20,
 truthful_enabled_heldout_total:24,
 truth_NEGATE_to_IDENTITY_learned_rule_misses_known_solution:true,
 truth_NEGATE_to_IDENTITY_shadow_reaches_solution:true,
 truth_IDENTITY_learned_three_trial_sequence_A_A_C:true,
 truth_NEGATE_learned_three_trial_sequence_B_C_B:true,
 universal_bad_opcode_validation:false,
 genuine_yoked_rule_transplant:false,
 privileged_optimal_rule_baseline:false,
 authored_RESET_semantics_not_self_invented:true,
 strong_general_Hk:false,B53_authenticated_feedback:false,
 N56_A_B_C:false,B3:false,B4:false,cyberlife:false,
 causal_counterfactuals:truthControls,
 cases:results.map(({real_original_PID1_service_outputs,paid_meta_instruction_fetches,
  real_task_program_slot_writes,final_original_PID1,
  final_present_or_missing_cells,...small})=>small)
},null,2));
