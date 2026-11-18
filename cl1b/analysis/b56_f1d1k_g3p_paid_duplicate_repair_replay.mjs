#!/usr/bin/env node
// B56-F1d1k-G3P: independent source-parsed COMPLETE native PID1 event replay.
// Do not trust Rust final-state summaries: rebuild ALL 120 original process
// histories and external interventions from the prospectively frozen CIL graph.
import fs from 'node:fs';
import assert from 'node:assert/strict';
import crypto from 'node:crypto';
import {isDeepStrictEqual} from 'node:util';

const load=p=>fs.readFileSync(new URL(p,import.meta.url));
const rustBytes=load('../../cl0/crates/world/tests/cl1b_b56_f1d1k_g3p_paid_duplicate_repair.rs');
const rust=rustBytes.toString('utf8');
const graphBytes=load('../studies/B56-F1d1k-G3P-frozen-exact-180-node-source-v1.json');
const protocolBytes=load('../studies/B56-F1d1k-G3P-120-world-first-mode-duplicate-repair-preregistration.md');
const graph=JSON.parse(graphBytes);
const L=graph.labels;
const sha256=b=>crypto.createHash('sha256').update(b).digest('hex');
const text=rust.split('fn founder()->Vec<Op>{')[1]?.split('#[derive(Copy,Clone,Debug,Eq,PartialEq)]')[0];
assert.ok(text?.includes('vec!['));
const matched=[...text.matchAll(/Op::([A-Za-z]+)(?:\{([^}]+)\})?,\s*\/\/ pc (\d+)/g)];
assert.equal(matched.length,180,'wrong original CIL node count');
const wrap=(name,fields={})=>name==='Halt'?'Halt':{[name]:fields};
const code=matched.map((m,i)=>{
 assert.equal(+m[3],i,'original Rust native pc comment mismatch');
 const fields=Object.fromEntries([...(m[2]??'').matchAll(/(\w+):(-?\d+)/g)]
  .map(y=>[y[1],+y[2]]));
 return wrap(m[1],fields);
});
assert.deepEqual(code,graph.code.map(({kind,...args})=>wrap(kind,args)),
 'original executable founder differs from before-native frozen exact graph');
assert.deepEqual([
 L.metaOpcodeFetch,L.metaCheckpoint,L.metaModeWrite,
 L.taskFirstOpcodeFetch,L.taskSecondOpcodeFetch,L.taskAcc,
 L.taskWrite52,L.taskWrite53,L.serviceWrite,L.feedbackRead,
 L.scoreWrite,L.paidBaseWrite,L.paidTailWrite,L.paidResetOffset,
 L.paidDisableShadow,L.paidDuplicateRepair,L.paidOffsetWrite,
 L.attemptCountWrite,L.paidFallbackOffset,L.paidFallbackAttemptReset,
 L.heldoutStageWrite,L.fallbackStageWrite,L.initBase,L.initTail,L.halt
],[13,30,43,86,91,94,80,82,96,103,109,126,128,131,
  48,67,141,146,156,161,164,159,2,4,179]);
assert.deepEqual(graph.keys,[7,11,20,21,31,32,33,50,52,53,56,61,62,63,64,65]);
assert.deepEqual(graph.resources,{credits:4000,limit_cells:16,limit_nodes:180,limit_occupied:300,PID:1});
assert.equal(graph.total_worlds,120);
assert.deepEqual(graph.feedback_regimes,['TRUTH','BLIND_SUCCESS','INVERTED','ABSENT','PHASE0_C_DECOY']);
assert.deepEqual(graph.intervention_gate.map(x=>x.value),[1,0]);
assert.ok(code.every(x=>x==='Halt'||(!x.GraphNew&&!x.GraphLaunch&&!x.GraphEncode)));
let hash=14695981039346656037n;
for(const b of Buffer.from(JSON.stringify(code),'utf8'))
 hash=BigInt.asUintN(64,(hash^BigInt(b))*1099511628211n);
const originalHash=hash.toString(16).padStart(16,'0');

const raw='cl1b-ci-evidence/b56_f1d1k_g3p_native_paid_duplicate_repair.jsonl';
if(!fs.existsSync(raw))throw Error('No original native paid events for G3P');
const lines=fs.readFileSync(raw,'utf8').trimEnd().split(/\r?\n/);
let line=0;
const compare=(expected,label)=>{
 if(line>=lines.length)throw Error('G3P ORIGINAL native events truncated: '+label);
 const actual=JSON.parse(lines[line++]);
 if(!isDeepStrictEqual(expected,actual))throw Error(
  'G3P FIRST NATIVE DIVERGENCE at JSONL line '+line+': '+label+
  '\nEXPECTED '+JSON.stringify(expected)+'\nACTUAL '+JSON.stringify(actual));
};
const meta=JSON.parse(lines[line++]);
assert.deepEqual(meta.slice(0,3),['meta','B56-F1d1k-G3P',1]);
if(process.env.GITHUB_SHA)assert.equal(meta[3],process.env.GITHUB_SHA);
assert.equal(meta[4],sha256(protocolBytes));
assert.equal(meta[5],protocolBytes.length);
assert.equal(meta[6],sha256(graphBytes));
assert.equal(meta[7],graphBytes.length);
assert.deepEqual(meta.slice(8),[
 120,4000,16,180,300,180,L.metaOpcodeFetch,L.metaCheckpoint,L.metaModeWrite,
 L.taskFirstOpcodeFetch,L.taskSecondOpcodeFetch,L.taskAcc,L.taskWrite52,L.taskWrite53,
 L.serviceWrite,L.feedbackRead,L.scoreWrite,L.paidBaseWrite,L.paidTailWrite,
 L.paidResetOffset,L.paidDisableShadow,L.paidDuplicateRepair,L.paidOffsetWrite,
 L.attemptCountWrite,L.paidFallbackOffset,L.paidFallbackAttemptReset,
 L.heldoutStageWrite,L.fallbackStageWrite,L.initBase,L.initTail,L.halt
]);
const KEYS=graph.keys;
const cell=(s,key)=>s.m.has(key)?s.m.get(key):null;
const cells=s=>KEYS.map(k=>cell(s,k));
const snap=s=>[s.pc,[...s.reg],s.credits,s.stopped,180,originalHash];
const occupied=s=>181+s.m.size;
const oracle=(name,x)=>name==='NEGATE'?-x:name==='COMPOSED'?-x+2:
 name==='IDENTITY'?x:name==='UNREACHABLE'?x+4:NaN;
const total={
 paid_CIL_instructions:0,external_interventions:0,
 meta_opcode_fetches:0,meta_checkpoint_writes:0,meta_program_executions:0,
 task_opcode_fetches:0,task_checkpoint_writes:0,task_program_edits:0,
 persistent_rule_writes:0,effective_base_edits:0,effective_tail_edits:0,
 temporary_rule_edits:0,paid_duplicate_repairs:0,paid_bypass_shadows:0,
 paid_fallback_guard_resets:0,failed_candidate_attempt_writes:0,
 feedback_reads:0,score_writes:0,real_service_outputs:0,paid_fallbacks:0
};
const cases=[];
let globalPeak=0;
for(const seed of graph.seeds){
 for(let sid=0;sid<graph.tasks.length;sid++){
  const schedule=graph.tasks[sid];
  for(const signal of graph.feedback_regimes){
   for(const arm of graph.intervention_gate){
    const gate=arm.value,id=cases.length;
    const s={pc:0,reg:[0,0,0,0],m:new Map(),tick:0,credits:4000,
     stopped:false,paid:0,externalCount:0,peak:181,
     metaFetch:[],metaCheckpoints:0,metaPrograms:[],
     taskFetches:0,taskCheckpoints:0,taskWrites:[],
     services:[],metaWrites:[],repairs:[],
     bypassShadows:0,fallbackGuardResets:0,attempts:0,
     feedbackReads:0,scoreWrites:0,initialRuleWrites:0,
     fallbacks:0,heldoutPaid:[]};
    compare(['case',id,seed.id,sid,signal,gate,schedule,seed.probes,
     seed.heldout,code,originalHash,occupied(s),4000,16,180,300],
     'original founder+case PID1 '+id);
    const external=(key,value,reason)=>{
     assert.ok([21,7,20].includes(key),'host wrote unauthorized process meta data');
     s.tick++;s.externalCount++;total.external_interventions++;
     s.m.set(key,value);
     assert.ok(s.m.size<=16&&occupied(s)<=300);
     s.peak=Math.max(s.peak,occupied(s));
     compare(['ext',id,s.tick,key,value,reason,snap(s),occupied(s),cells(s)],
      'host intervention '+id+'/'+reason);
    };
    external(21,gate,'initial_rule_control');
    external(7,seed.probes[0],'first_probe_input');
    while(!s.stopped){
     const pc=s.pc,def=code[pc];
     assert.notEqual(def,undefined,'source jumped past prospectively frozen CIL graph');
     assert.ok(s.credits>0,'free private credit or exhaustion');
     const type=def==='Halt'?'Halt':Object.keys(def)[0],
      o=def==='Halt'?{}:def[type];
     const r=i=>{assert.ok(i>=0&&i<4);return s.reg[i]};
     s.paid++;s.tick++;s.credits--;total.paid_CIL_instructions++;
     let event='Executed',afterProbe=null,endedHeldout=null;
     switch(type){
      case 'Const':s.reg[o.dst]=o.value;s.pc=o.next;break;
      case 'Add':s.reg[o.dst]=r(o.lhs)+r(o.rhs);s.pc=o.next;break;
      case 'Sub':s.reg[o.dst]=r(o.lhs)-r(o.rhs);s.pc=o.next;break;
      case 'Branch':s.pc=r(o.test)!==0?o.nonzero:o.zero;break;
      case 'Read':{
       const key=r(o.address),value=s.m.get(key)??0;
       s.reg[o.dst]=value;s.pc=o.next;
       event=wrap('CellRead',{key,value});
       if(pc===L.metaOpcodeFetch){
        const vpc=cell(s,50);
        assert.ok(vpc>=0&&vpc<=2);
        assert.equal(key,62+vpc);
        assert.equal(value,s.m.get(key)??0);
        s.metaFetch.push([cell(s,31),vpc,key,value,s.tick]);
        total.meta_opcode_fetches++;
       }
       if(pc===L.taskFirstOpcodeFetch||pc===L.taskSecondOpcodeFetch){
        assert.equal(key,pc===L.taskFirstOpcodeFetch?52:53);
        s.taskFetches++;total.task_opcode_fetches++;
       }
       if(pc===L.feedbackRead){
        assert.equal(key,20);
        assert.equal(value,s.m.get(20)??0);
        s.feedbackReads++;total.feedback_reads++;
       }
       break;
      }
      case 'Write':{
       const key=r(o.address),value=r(o.value);
       const previous=cell(s,key);
       s.m.set(key,value);s.pc=o.next;
       event=wrap('CellWritten',{key,value});
       if(pc===L.initBase||pc===L.initTail){
        assert.equal(key,pc===L.initBase?62:64);
        assert.equal(value,pc===L.initBase?1:0);
        s.initialRuleWrites++;
       }
       if(pc===L.metaCheckpoint){
        assert.equal(key,56);
        s.metaCheckpoints++;total.meta_checkpoint_writes++;
       }
       if(pc===L.metaModeWrite){
        assert.equal(key,33);assert.ok(value>=0&&value<=2);
        s.metaPrograms.push([cell(s,31),cell(s,62),cell(s,63),
         cell(s,64),value,s.tick]);
        total.meta_program_executions++;
       }
       if(pc===L.taskAcc){
        assert.equal(key,56);
        s.taskCheckpoints++;total.task_checkpoint_writes++;
       }
       if(pc===L.taskWrite52||pc===L.taskWrite53){
        assert.equal(key,pc===L.taskWrite52?52:53);
        s.taskWrites.push([cell(s,31),cell(s,33),key,value,s.tick]);
        total.task_program_edits++;
       }
       if([L.paidBaseWrite,L.paidTailWrite,L.paidResetOffset,L.paidOffsetWrite,
           L.paidFallbackOffset,L.paidDuplicateRepair].includes(pc)){
        const expectedKey=pc===L.paidBaseWrite?62:pc===L.paidTailWrite?64:63;
        assert.equal(key,expectedKey);
        if([L.paidBaseWrite,L.paidTailWrite,L.paidResetOffset].includes(pc)){
         total.persistent_rule_writes++;
         if(pc===L.paidBaseWrite&&previous!==value)total.effective_base_edits++;
         if(pc===L.paidTailWrite&&previous!==value)total.effective_tail_edits++;
        }else total.temporary_rule_edits++;
        if(pc===L.paidFallbackOffset){
         s.fallbacks++;total.paid_fallbacks++;
        }
        if(pc===L.paidDuplicateRepair){
         assert.equal(gate,1);
         s.repairs.push([cell(s,31),cell(s,65),cell(s,33),cell(s,62),
          value,s.tick]);
         total.paid_duplicate_repairs++;
        }
        s.metaWrites.push([cell(s,31),cell(s,33),key,value,pc,s.tick]);
       }
       if(pc===L.paidDisableShadow){
        assert.equal(gate,0);assert.equal(key,56);assert.equal(value,0);
        s.bypassShadows++;total.paid_bypass_shadows++;
       }
       if(pc===L.paidFallbackAttemptReset){
        assert.equal(key,65);assert.equal(value,0);
        s.fallbackGuardResets++;total.paid_fallback_guard_resets++;
       }
       if(pc===L.attemptCountWrite){
        assert.equal(key,65);s.attempts++;
        total.failed_candidate_attempt_writes++;
       }
       if(pc===L.scoreWrite){
        assert.equal(key,61);
        s.scoreWrites++;total.score_writes++;
       }
       if(key===11){
        assert.equal(pc,L.serviceWrite);
        const phase=cell(s,31)??0,stage=cell(s,32)??0,mode=cell(s,33);
        assert.ok(phase===0||phase===1);
        assert.ok(stage>=0&&stage<=2&&mode>=0&&mode<=2);
        s.services.push([phase,stage,mode,value,cell(s,62),cell(s,63),
         cell(s,64),cell(s,52),cell(s,53)]);
        total.real_service_outputs++;
        if(stage<2)afterProbe=[phase,stage,mode,value];
        else{endedHeldout=phase;s.heldoutPaid.push(s.paid);}
       }
       break;
      }
      case 'Halt':
       assert.equal(pc,L.halt);
       s.stopped=true;event='Halted';break;
      default:throw Error('source-native CIL primitive unsupported '+type);
     }
     assert.equal(s.credits,4000-s.paid);
     assert.ok(s.paid<=4000&&s.m.size<=16&&occupied(s)<=300);
     s.peak=Math.max(s.peak,occupied(s));
     globalPeak=Math.max(globalPeak,s.peak);
     compare(['step',id,s.paid,s.tick,1,pc,event,
      snap(s),occupied(s),cells(s)],
      'PID1 paid original CIL '+id+'/'+s.paid+' pc'+pc);
     if(afterProbe){
      const [phase,stage,mode,output]=afterProbe;
      const truth=Number(output===oracle(schedule[phase],seed.probes[stage]));
      const bit=signal==='TRUTH'?truth:
       signal==='INVERTED'?1-truth:
       signal==='BLIND_SUCCESS'?1:
       signal==='ABSENT'?null:
       phase===0&&mode===1?1:truth;
      if(bit!==null)external(20,bit,'paid_probe_feedback');
      external(7,stage===0?seed.probes[1]:seed.probes[0],
       'next_probe_input');
     }
     if(pc===L.heldoutStageWrite||pc===L.fallbackStageWrite){
      assert.equal(type,'Write');
      assert.equal(cell(s,32),2);
      external(7,seed.heldout,pc===L.heldoutStageWrite?
       'valid_candidate_heldout':'fallback_heldout');
     }
     if(endedHeldout===0)
      external(7,seed.probes[0],'next_hidden_task_probe0');
    }
    assert.equal(s.initialRuleWrites,2);
    assert.ok(s.peak<=197);
    const held=s.services.filter(x=>x[1]===2);
    const phases=held.map(x=>{
     const phase=x[0],output=x[3];
     return {phase,task:schedule[phase],mode:x[2],output,
      expected:oracle(schedule[phase],seed.heldout),
      correct:output===oracle(schedule[phase],seed.heldout),
      meta_bytecode:[x[4],x[5],x[6]],
      task_bytecode:[x[7],x[8]]};
    });
    const correct=phases.filter(x=>x.correct).length;
    assert.equal(cell(s,31),2);
    assert.equal(phases.length,2);
    assert.equal(s.heldoutPaid.length,2);
    assert.ok(s.metaFetch.length>=6&&s.metaFetch.length%3===0);
    assert.equal(s.metaCheckpoints,s.metaFetch.length);
    assert.equal(s.metaPrograms.length*3,s.metaFetch.length);
    assert.equal(s.taskFetches,s.services.length*2);
    assert.equal(s.taskCheckpoints,s.services.length);
    if(gate===0){
     assert.equal(s.repairs.length,0);
     assert.equal(s.bypassShadows,s.metaPrograms.length);
    }else{
     assert.equal(gate,1);
     assert.equal(s.bypassShadows,0);
    }
    if(signal==='TRUTH'){
     assert.equal(correct,(gate===1?
      [2,2,2,1,2,2]:[2,2,2,1,1,2])[sid]);
     if(sid===4){
      assert.equal(phases[1].correct,gate===1);
      if(gate===1){
       assert.equal(phases[1].mode,0);
       assert.ok(s.repairs.length>0);
      }else{
       assert.equal(phases[1].mode,2);
       assert.ok(s.fallbacks>0);
      }
     }
     if(sid===3){
      assert.equal(phases[1].correct,false);
      assert.ok(s.fallbacks>0);
     }
    }
    const phaseCosts=[
     s.heldoutPaid[0],s.heldoutPaid[1]-s.heldoutPaid[0]
    ];
    const result={
     id,seed:seed.id,schedule:sid,regime:signal,gate,
     real_original_PID1_service_outputs:s.services,
     true_heldout_by_phase:phases,true_heldout_correct:correct,
     paid_meta_instruction_fetches:s.metaFetch,
     paid_meta_intermediate_checkpoints:s.metaCheckpoints,
     charged_three_instruction_meta_compositions:s.metaPrograms,
     charged_task_op_fetches:s.taskFetches,
     paid_task_accumulator_checkpoints:s.taskCheckpoints,
     real_task_program_slot_writes:s.taskWrites,
     actor_paid_meta_data_writes:s.metaWrites,
     paid_guard_bypass_shadow_writes:s.bypassShadows,
     paid_original_duplicate_repair_writes:s.repairs,
     paid_fallback_guard_reset_writes:s.fallbackGuardResets,
     source_paid_candidate_attempt_writes:s.attempts,
     source_paid_feedback_reads:s.feedbackReads,
     source_paid_score_writes:s.scoreWrites,
     source_paid_fallback_entries:s.fallbacks,
     phase_costs_to_real_heldout:phaseCosts,
     paid_CIL_instructions:s.paid,
     external_interventions:s.externalCount,
     measured_peak_storage:s.peak,
     final_original_PID1:snap(s),
     final_present_or_missing_cells:cells(s),
     remaining_private_credits:s.credits,
     unaltered_original_PID1:true,native_source_unchanged:true,
     preauthored_meta_grammar:true,universal_bad_opcode_handling:false,
     guard_checks_first_candidate_duplicate_only:true,
     full_G3:false,full_Hk:false,B3:false,B4:false,cyberlife:false
    };
    compare(['end',id,result],'native original PID1 end row '+id);
    cases.push(result);
   }
  }
 }
}
assert.equal(cases.length,120);
assert.equal(line,lines.length,'unverified trailing original native event records');
const pick=(seed,sid,signal,gate)=>cases.find(x=>x.seed===seed&&x.schedule===sid
 &&x.regime===signal&&x.gate===gate);
const comparisons=[];let enabledCorrect=0,bypassCorrect=0;
for(let seed=0;seed<2;seed++){
 for(let sid=0;sid<6;sid++){
  const enabled=pick(seed,sid,'TRUTH',1),bypass=pick(seed,sid,'TRUTH',0);
  assert.equal(enabled.true_heldout_correct,[2,2,2,1,2,2][sid]);
  assert.equal(bypass.true_heldout_correct,[2,2,2,1,1,2][sid]);
  enabledCorrect+=enabled.true_heldout_correct;
  bypassCorrect+=bypass.true_heldout_correct;
 }
 const s4=pick(seed,4,'TRUTH',1),off=pick(seed,4,'TRUTH',0),
  s5=pick(seed,5,'TRUTH',1),s3=pick(seed,3,'TRUTH',1);
 assert.deepEqual(s4.charged_three_instruction_meta_compositions
  .filter(p=>p[0]===1).map(p=>p[4]),[2,1,2,0]);
 assert.deepEqual(off.charged_three_instruction_meta_compositions
  .filter(p=>p[0]===1).map(p=>p[4]),[2,1,2,2]);
 assert.deepEqual(s5.charged_three_instruction_meta_compositions
  .filter(p=>p[0]===1).map(p=>p[4]),[0,0,1]);
 const repair=s4.paid_original_duplicate_repair_writes.find(p=>p[0]===1);
 assert.equal(repair?.[4],2);
 assert.equal(s4.true_heldout_by_phase[1].mode,0);
 assert.equal(off.true_heldout_by_phase[1].mode,2);
 assert.equal(s3.true_heldout_by_phase[1].correct,false);
 comparisons.push({seed,
  S4_guarded_paid_repair:repair,
  S4_guarded_phase1_cost:s4.phase_costs_to_real_heldout[1],
  S4_bypass_phase1_cost:off.phase_costs_to_real_heldout[1],
  S4_guarded_correct:s4.true_heldout_correct,
  S4_bypass_correct:off.true_heldout_correct,
  S5_guarded_phase1_cost:s5.phase_costs_to_real_heldout[1],
  S5_bypass_phase1_cost:pick(seed,5,'TRUTH',0).phase_costs_to_real_heldout[1]});
}
assert.equal(enabledCorrect,22);
assert.equal(bypassCorrect,20);
const descriptors=new Set();
for(const c of cases)for(const p of c.charged_three_instruction_meta_compositions)
 descriptors.add([p[1],p[2],p[3]].join(','));
const steps=cases.map(x=>x.paid_CIL_instructions);
console.log(JSON.stringify({
 study:'B56-F1d1k-G3P',tested_source_sha:meta[3],
 precommitted_exact_source_sha256:sha256(graphBytes),
 precommitted_control_protocol_sha256:sha256(protocolBytes),
 rust_original_PID1_file_sha256:sha256(rustBytes),
 original_CIL_source_nodes:180,original_PID1_worlds_reexecuted:120,
 actual_paid_original_CIL_steps_reexecuted:total.paid_CIL_instructions,
 source_authorized_host_interventions:total.external_interventions,
 dynamic_meta_instruction_fetches:total.meta_opcode_fetches,
 meta_intermediate_paid_checkpoints:total.meta_checkpoint_writes,
 complete_three_op_meta_program_executions:total.meta_program_executions,
 original_paid_task_opcode_reads:total.task_opcode_fetches,
 task_intermediate_paid_writes:total.task_checkpoint_writes,
 actual_paid_task_code_slot_edits:total.task_program_edits,
 paid_feedback_validated_meta_descriptor_writes:total.persistent_rule_writes,
 actor_paid_effective_base_changes:total.effective_base_edits,
 actor_paid_effective_tail_changes:total.effective_tail_edits,
 paid_trial_and_repair_meta_writes:total.temporary_rule_edits,
 original_PID1_paid_duplicate_repair_writes:total.paid_duplicate_repairs,
 paid_guard_bypass_shadow_writes:total.paid_bypass_shadows,
 paid_fallback_guard_attempt_resets:total.paid_fallback_guard_resets,
 paid_candidate_failure_counter_edits:total.failed_candidate_attempt_writes,
 paid_feedback_reads:total.feedback_reads,
 paid_scoring_writes:total.score_writes,
 paid_original_PID1_service_outputs:total.real_service_outputs,
 source_paid_fallback_entries:total.paid_fallbacks,
 observed_meta_descriptor_data_states:[...descriptors].sort(),
 per_world_paid_instruction_range:[Math.min(...steps),Math.max(...steps)],
 measured_peak_total_storage_units:globalPeak,
 first_divergence:null,
 unchanged_original_PID1_per_world:true,
 original_frozen_CIL_code_unmodified:true,
 source_identical_G3P_S4_known_A_recovered:true,
 source_identical_guard_bypass_S4_misses_A:true,
 actual_paid_repair_write63_precedes_real_A_service:true,
 S5_duplicate_A_detected_before_probe:true,
 unreachable_x_plus_4_remains_incorrect:true,
 truthful_enabled_correct_heldouts:enabledCorrect,
 truthful_guard_bypass_correct_heldouts:bypassCorrect,
 truthful_heldout_total_per_arm:24,
 complete_64_meta_program_coverage_proven:false,
 meta_opcode_type_safety_tested:false,
 trusted_independent_feedback_provenance:false,
 proper_yoked_oracle_control:false,
 general_Hk:false,B3:false,B4:false,cyberlife:false,
 seed_matched_truth_controls:comparisons,
 cases:cases.map(({real_original_PID1_service_outputs,
   paid_meta_instruction_fetches,real_task_program_slot_writes,
   final_original_PID1,final_present_or_missing_cells,...rest})=>rest)
},null,2));
