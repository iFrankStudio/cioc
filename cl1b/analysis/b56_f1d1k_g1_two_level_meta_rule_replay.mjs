#!/usr/bin/env node
// B56-F1d1k-G1: source-parse frozen 178 native CIL nodes, independently
// reexecute every original PID1 paid step & environment intervention across
// the EXACT precommitted 160-world matrix. Separate interpreter from Rust.
// Do not infer arbitrary rule invention, authenticated truth or cyberlife.
import fs from 'node:fs';
import assert from 'node:assert/strict';
import crypto from 'node:crypto';
import {isDeepStrictEqual} from 'node:util';
const load=p=>fs.readFileSync(new URL(p,import.meta.url));
const native=load('../../cl0/crates/world/tests/cl1b_b56_f1d1k_g1_two_level_meta_rule.rs').toString('utf8');
const frozenBuffer=load('../studies/B56-F1d1k-G1-frozen-exact-178-node-source-v1.json');
const protocolBuffer=load('../studies/B56-F1d1k-G1-exact-two-level-meta-bytecode-and-160-world-controls-preregistration.md');
const spec=JSON.parse(frozenBuffer.toString('utf8'));
const sha256=bytes=>crypto.createHash('sha256').update(bytes).digest('hex');
const slice=native.split('fn founder()->Vec<Op>{')[1]?.split('#[derive(Copy,Clone,Debug,PartialEq,Eq)]')[0];
assert.ok(slice?.includes('vec!['),'Rust CIL source literal cannot be parsed');
const matches=[...slice.matchAll(/Op::([A-Za-z]+)(?:\{([^}]+)\})?,\s*\/\/ pc (\d+)/g)];
assert.equal(matches.length,178,'wrong number of original paid CIL source nodes');
const op=(name,fields={})=>name==='Halt'?'Halt':{[name]:fields};
const code=matches.map((m,i)=>{
 assert.equal(Number(m[3]),i,'Rust founder PC comments no longer frozen');
 return op(m[1],Object.fromEntries(
  [...(m[2]??'').matchAll(/(\w+):(-?\d+)/g)].map(x=>[x[1],Number(x[2])])
 ));
});
assert.deepEqual(code,spec.code.map(({kind,...args})=>op(kind,args)),
 'actual Rust original CIL differs from PRE-IMPLEMENTATION frozen graph');
const L=spec.labels;
assert.equal(L.metaOpcodeFetch,36);
assert.equal(L.metaCheckpoint,52);
assert.equal(L.taskFirstOpcodeFetch,90);
assert.equal(L.taskSecondOpcodeFetch,97);
assert.equal(L.serviceWrite,102);
assert.equal(L.feedbackRead,109);
assert.equal(L.metaBaseWrite,137);
assert.equal(L.failedOffsetWrite,150);
assert.equal(L.heldoutStageWrite,162);
assert.equal(spec.resources.initial_credits,4000);
assert.equal(spec.resources.max_cells,16);
assert.equal(spec.resources.max_native_nodes,180);
assert.equal(spec.resources.max_total_storage,300);
assert.equal(spec.case_count,160);
const KEYS=[7,11,20,21,31,32,33,50,52,53,56,61,62,63];
assert.deepEqual(Object.values(spec.keys).sort((a,b)=>a-b),KEYS);
assert.ok(code.every(x=>x==='Halt'||(!x.GraphLaunch&&!x.GraphNew&&!x.GraphEncode)));
const SHA=process.env.GITHUB_SHA||null;
let h=14695981039346656037n;
for(const byte of Buffer.from(JSON.stringify(code),'utf8'))
 h=BigInt.asUintN(64,(h^BigInt(byte))*1099511628211n);
const codehash=h.toString(16).padStart(16,'0');
const raw='cl1b-ci-evidence/b56_f1d1k_g1_native_two_level_meta_rule.jsonl';
if(!fs.existsSync(raw))throw Error('NO ORIGINAL B56-F1d1k-G1 NATIVE CIL JSONL');
const lines=fs.readFileSync(raw,'utf8').trimEnd().split(/\r?\n/);
let pos=0;
const compare=(expected,why)=>{
 if(pos>=lines.length)throw Error('original native evidence truncated '+why);
 const actual=JSON.parse(lines[pos++]);
 if(!isDeepStrictEqual(expected,actual))throw Error(
  'B56-F1d1k-G1 FIRST NATIVE DIVERGENCE at source line '+pos+': '+why+
  '\nEXPECTED '+JSON.stringify(expected)+'\nACTUAL   '+JSON.stringify(actual));
};
const meta=JSON.parse(lines[pos++]);
assert.deepEqual(meta.slice(0,3),['meta','B56-F1d1k-G1',1]);
if(SHA)assert.equal(meta[3],SHA);
assert.equal(meta[4],sha256(protocolBuffer));
assert.equal(meta[5],protocolBuffer.length);
assert.equal(meta[6],sha256(frozenBuffer));
assert.equal(meta[7],frozenBuffer.length);
assert.deepEqual(meta.slice(8),[
 160,4000,16,180,300,178,
 L.metaOpcodeFetch,L.metaCheckpoint,L.metaModeWrite,
 L.taskFirstOpcodeFetch,L.taskSecondOpcodeFetch,L.taskAcc1,L.taskAcc2,
 L.taskWrite52,L.taskWrite53,L.serviceWrite,L.feedbackRead,L.scoreWrite,
 L.metaBaseWrite,L.metaOffsetReset,L.shadowBaseWrite,L.shadowOffsetWrite,
 L.failedOffsetWrite,L.fallbackOffsetWrite,L.heldoutStageWrite
]);
const value=(s,k)=>s.data.has(k)?s.data.get(k):null;
const cells=s=>KEYS.map(k=>value(s,k));
const snap=s=>[s.pc,[...s.reg],s.credits,s.stopped,178,codehash];
const occupied=s=>179+s.data.size;
const taskCode=s=>[value(s,52),value(s,53)];
const oracle=(task,x)=>task==='NEGATE'?-x:
 task==='COMPOSED'?-x+2:task==='IDENTITY'?x:x+4;
const totals={
 paid:0,external:0,feedback:0,score:0,metaFetch:0,metaCheckpoint:0,
 metaMode:0,taskFetch:0,taskCheckpoint:0,
 metaDescriptorWrites:0,failedOffsetEdits:0,shadowWrites:0,
 taskWrites:0,outputs:0,fallbacks:0
};
let peak=0,firstDivergence=null;
const results=[];
for(const seed of spec.seeds){
 for(let sid=0;sid<spec.schedules.length;sid++){
  const tasks=spec.schedules[sid];
  for(const regime of spec.feedback_regimes){
   for(const control of spec.gate_controls){
    const gate=control.value,id=results.length;
    const s={
     pc:0,reg:[0,0,0,0],data:new Map(),tick:0,credits:4000,
     steps:0,stopped:false,peak:179,externalCount:0,
     services:[],metaFetches:[],metaPrograms:[],metaCheckpoints:[],
     metaBaseEdits:[],metaOffsetEdits:[],metaShadow:[],
     taskWrites:[],taskFetches:0,taskCheckpoints:0,
     paidFeedback:0,paidScore:0,fallbacks:0,heldoutPaid:[]
    };
    compare(['case',id,seed.id,sid,regime,gate,tasks,seed.probes,
      seed.heldout,code,codehash,occupied(s),4000,16,180,300],
      'immutable PID1 founder and original strict world '+id);
    function external(k,v,why){
     assert.ok([7,20,21].includes(k),'unauthorized host data-program edit');
     s.data.set(k,v);s.tick++;s.externalCount++;totals.external++;
     assert.ok(s.data.size<=16&&occupied(s)<=300);
     s.peak=Math.max(s.peak,occupied(s));peak=Math.max(peak,s.peak);
     compare(['ext',id,s.tick,k,v,why,snap(s),occupied(s),cells(s)],
      'original allowed host '+id+'/'+why);
    }
    external(21,gate,'initial_immutable_rule_control');
    external(7,seed.probes[0],'first_task_probe');
    while(!s.stopped){
     const pc=s.pc,rawOp=code[pc];
     assert.notEqual(rawOp,undefined,'unaccounted original CIL branch');
     assert.ok(s.credits>0,'process ran out of finite credits');
     const kind=rawOp==='Halt'?'Halt':Object.keys(rawOp)[0],
      a=rawOp==='Halt'?{}:rawOp[kind];
     const r=i=>{assert.ok(i>=0&&i<=3);return s.reg[i]};
     s.steps++;s.tick++;s.credits--;totals.paid++;
     let event='Executed',probe=null,ended=null;
     switch(kind){
      case 'Const':s.reg[a.dst]=a.value;s.pc=a.next;break;
      case 'Add':s.reg[a.dst]=r(a.lhs)+r(a.rhs);s.pc=a.next;break;
      case 'Sub':s.reg[a.dst]=r(a.lhs)-r(a.rhs);s.pc=a.next;break;
      case 'Branch':s.pc=r(a.test)!==0?a.nonzero:a.zero;break;
      case 'Read':{
       const k=r(a.address),v=s.data.get(k)??0;
       s.reg[a.dst]=v;s.pc=a.next;event=op('CellRead',{key:k,value:v});
       if(pc===L.metaOpcodeFetch){
        const mp=value(s,50);assert.ok(mp===0||mp===1);
        assert.equal(k,62+mp);
        assert.ok(v>=0&&v<=2,'actor-written invalid meta instruction reached interpreter');
        s.metaFetches.push([value(s,31),mp,k,v,s.tick]);totals.metaFetch++;
       }
       if(pc===L.taskFirstOpcodeFetch||pc===L.taskSecondOpcodeFetch){
        assert.equal(k,pc===L.taskFirstOpcodeFetch?52:53);
        s.taskFetches++;totals.taskFetch++;
       }
       if(pc===L.feedbackRead){
        assert.equal(k,20);assert.equal(v,s.data.get(20)??0);
        s.paidFeedback++;totals.feedback++;
       }
       break;
      }
      case 'Write':{
       const k=r(a.address),v=r(a.value);
       s.data.set(k,v);s.pc=a.next;event=op('CellWritten',{key:k,value:v});
       if(pc===L.metaCheckpoint){
        assert.equal(k,56);
        s.metaCheckpoints.push([value(s,31),value(s,50),v,s.tick]);
        totals.metaCheckpoint++;
       }
       if(pc===L.metaModeWrite){
        assert.equal(k,33);assert.ok(v>=0&&v<=2);
        s.metaPrograms.push([value(s,31),value(s,62),value(s,63),v,s.tick]);
        totals.metaMode++;
       }
       if(pc===L.taskAcc1||pc===L.taskAcc2){
        assert.equal(k,56);s.taskCheckpoints++;totals.taskCheckpoint++;
       }
       if(pc===L.metaBaseWrite||pc===L.metaOffsetReset){
        assert.equal(gate,1);
        assert.equal(k,pc===L.metaBaseWrite?62:63);
        s.metaBaseEdits.push([value(s,31),value(s,33),k,v,s.tick]);
        totals.metaDescriptorWrites++;
       }
       if(pc===L.shadowBaseWrite||pc===L.shadowOffsetWrite){
        assert.notEqual(gate,1);assert.equal(k,56);
        s.metaShadow.push([value(s,31),value(s,33),v,s.tick]);
        totals.shadowWrites++;
       }
       if(pc===L.failedOffsetWrite||pc===L.fallbackOffsetWrite){
        assert.equal(k,63);
        s.metaOffsetEdits.push([value(s,31),v,pc,s.tick]);
        totals.failedOffsetEdits++;
        if(pc===L.fallbackOffsetWrite){s.fallbacks++;totals.fallbacks++;}
       }
       if(pc===L.taskWrite52||pc===L.taskWrite53){
        assert.equal(k,pc===L.taskWrite52?52:53);
        s.taskWrites.push([value(s,31),value(s,33),k,v,s.tick]);
        totals.taskWrites++;
       }
       if(pc===L.scoreWrite){
        assert.equal(k,61);s.paidScore++;totals.score++;
       }
       if(k===11){
        assert.equal(pc,L.serviceWrite);
        const phase=value(s,31),stage=value(s,32),mode=value(s,33);
        assert.ok(phase===0||phase===1);
        assert.ok(stage>=0&&stage<=2&&mode>=0&&mode<=2);
        s.services.push([phase,stage,mode,v,taskCode(s),value(s,62),value(s,63)]);
        totals.outputs++;
        if(stage<2)probe=[phase,stage,mode,v];
        else{ended=phase;s.heldoutPaid.push(s.steps);}
       }
       break;
      }
      case 'Halt':s.stopped=true;event='Halted';break;
      default:throw Error('unsupported native CIL op '+kind);
     }
     assert.equal(s.credits,4000-s.steps,'compute not paid exactly once per CIL');
     assert.ok(s.data.size<=16&&occupied(s)<=300);
     assert.ok(s.steps<=4000);
     s.peak=Math.max(s.peak,occupied(s));peak=Math.max(peak,s.peak);
     compare(['step',id,s.steps,s.tick,1,pc,event,snap(s),occupied(s),cells(s)],
       'actual charge PID1 original instruction '+id+'/'+s.steps+' at native PC'+pc);
     if(probe){
      const [phase,stage,mode,actual]=probe;
      const correct=Number(actual===oracle(tasks[phase],seed.probes[stage]));
      const bit=regime==='TRUTH'?correct:
       regime==='INVERTED'?1-correct:
       regime==='BLIND_SUCCESS'?1:
       regime==='ABSENT'?null:
       (phase===0&&mode===1)?1:correct;
      if(bit!==null)external(20,bit,'after_paid_probe_feedback');
      external(7,stage===0?seed.probes[1]:seed.probes[0],'next_probe_input');
     }
     if(pc===L.heldoutStageWrite){
      assert.equal(kind,'Write');assert.equal(value(s,32),2);
      external(7,seed.heldout,'actor_requested_heldout');
     }
     if(ended===0)external(7,seed.probes[0],'next_hidden_task_phase');
    }
    assert.equal(value(s,31),2);
    assert.equal(s.heldoutPaid.length,2);
    assert.equal(s.metaFetches.length,s.metaCheckpoints.length);
    assert.equal(s.metaPrograms.length*2,s.metaFetches.length);
    assert.equal(s.taskFetches,s.services.length*2);
    assert.equal(s.taskCheckpoints,s.services.length*2);
    if(gate===1)assert.equal(s.metaShadow.length,0);
    else{
     assert.equal(s.metaBaseEdits.length,0);
     assert.equal(value(s,62),gate===2?2:gate===3?0:1);
    }
    const phases=s.services.filter(x=>x[1]===2).map(([phase,stage,mode,output,program,base,offset])=>({
     phase,task:tasks[phase],selected_mode:mode,output,
     expected:oracle(tasks[phase],seed.heldout),
     correct:output===oracle(tasks[phase],seed.heldout),
     task_bytecode:program,meta_base:base,meta_offset:offset
    }));
    assert.equal(phases.length,2);
    const good=phases.filter(x=>x.correct).length;
    if(regime==='TRUTH'){
     assert.equal(good,sid===3?1:2);
     if(sid===3){assert.equal(phases[1].correct,false);
      assert.equal(phases[1].expected,seed.heldout+4);
      assert.ok(s.fallbacks>0);}
    }
    const result={
     id,seed:seed.id,schedule:sid,regime,gate,
     original_PID1_real_service_events:s.services,
     heldout_by_phase:phases,heldout_correct:good,
     native_paid_meta_opcode_fetches:s.metaFetches,
     native_paid_meta_intermediate_checkpoints:s.metaCheckpoints,
     native_composed_meta_program_results:s.metaPrograms,
     native_actor_meta_descriptor_writes:s.metaBaseEdits,
     native_actor_failed_probe_meta_offset_writes:s.metaOffsetEdits,
     native_paid_meta_write_suppression_shadows:s.metaShadow,
     native_task_program_writes:s.taskWrites,
     native_task_opcode_reads:s.taskFetches,
     native_task_intermediate_checkpoints:s.taskCheckpoints,
     real_feedback_reads:s.paidFeedback,real_candidate_score_writes:s.paidScore,
     source_paid_policy_fallbacks:s.fallbacks,
     phase_costs_at_real_heldout:[s.heldoutPaid[0],s.heldoutPaid[1]-s.heldoutPaid[0]],
     actual_paid_CIL_instructions:s.steps,
     actual_external_interventions:s.externalCount,
     maximum_occupied_storage:s.peak,
     final_meta_bytecode:[value(s,62),value(s,63)],
     final_process:snap(s),final_cells:cells(s),credits_left:s.credits,
     native_source_unchanged:true,original_PID1_only:true,
     authored_meta_opcode_ISA:true,generic_unbounded_rule_synthesis:false,
     b3:false,b4:false,cyberlife:false
    };
    compare(['end',id,result],'exact original-native complete experiment '+id);
    results.push(result);
   }
  }
 }
}
assert.equal(results.length,160);
assert.equal(pos,lines.length,'original native JSONL trailing events unchecked');
const pick=(seed,sid,regime,gate)=>results.find(
 x=>x.seed===seed&&x.schedule===sid&&x.regime===regime&&x.gate===gate);
const transfer=[];
for(let seed=0;seed<2;seed++){
 const truth=pick(seed,0,'TRUTH',1),frozen=pick(seed,0,'TRUTH',0),
  privileged=pick(seed,0,'TRUTH',2),decoy=pick(seed,0,'PHASE0_C_DECOY',1);
 assert.ok(truth.phase_costs_at_real_heldout[1]<frozen.phase_costs_at_real_heldout[1]);
 assert.ok(privileged.actual_paid_CIL_instructions<truth.actual_paid_CIL_instructions);
 assert.equal(truth.native_composed_meta_program_results.find(x=>x[0]===1)[1],2);
 assert.equal(decoy.native_composed_meta_program_results.find(x=>x[0]===1)[1],1);
 assert.equal(truth.native_composed_meta_program_results.find(x=>x[0]===1)[3],2);
 assert.equal(decoy.native_composed_meta_program_results.find(x=>x[0]===1)[3],1);
 assert.equal(decoy.heldout_by_phase[0].correct,false);
 assert.equal(truth.heldout_by_phase[0].correct,true);
 const shift=pick(seed,2,'TRUTH',1),noLearn=pick(seed,2,'TRUTH',0);
 assert.ok(shift.phase_costs_at_real_heldout[1]>noLearn.phase_costs_at_real_heldout[1]);
 const identity=pick(seed,3,'TRUTH',1);
 assert.deepEqual(identity.native_composed_meta_program_results
  .filter(x=>x[0]===0).map(x=>x[3]),[1,2,0]);
 assert.equal(identity.heldout_by_phase[1].correct,false);
 transfer.push({seed,
  repeated_truth_phase1:truth.phase_costs_at_real_heldout[1],
  repeated_frozenC_phase1:frozen.phase_costs_at_real_heldout[1],
  prior_optimal_frozenB_total:privileged.actual_paid_CIL_instructions,
  task_shift_learned_B_phase1:shift.phase_costs_at_real_heldout[1],
  task_shift_frozenC_phase1:noLearn.phase_costs_at_real_heldout[1],
  decoy_learned_initial_C_and_first_phase_fails:true});
}
const prior={};let effectiveBase=0,effectiveOffset=0,newPairs=new Set(),
 virtualTaskPairs=new Set(),checkpointByWorld=[];
for(const r of results){
 for(const e of r.native_composed_meta_program_results)
  newPairs.add([e[1],e[2]].join(','));
 for(const service of r.original_PID1_real_service_events)
  virtualTaskPairs.add(service[4].join(','));
 let oldBase=r.gate===2?2:r.gate===3?0:1;
 for(const edit of r.native_actor_meta_descriptor_writes){
  if(edit[2]===62){if(edit[3]!==oldBase)effectiveBase++;oldBase=edit[3];}
 }
 const priorOffset=[0,0]; // used only for reporting paid edits separately
 checkpointByWorld.push(r.native_paid_meta_intermediate_checkpoints.length);
}
const counts=results.map(r=>r.actual_paid_CIL_instructions);
const accTruth=results.filter(r=>r.regime==='TRUTH'&&r.gate===1)
 .reduce((n,r)=>n+r.heldout_correct,0);
console.log(JSON.stringify({
 study:'B56-F1d1k-G1',tested_source_sha:meta[3],
 frozen_protocol_sha256:sha256(protocolBuffer),
 frozen_exact_178_node_graph_sha256:sha256(frozenBuffer),
 original_Rust_source_sha256:sha256(Buffer.from(native)),
 reconstructed_native_CIL_node_count:178,
 original_worlds_entirely_reexecuted:160,
 original_actor_paid_CIL_instructions_independently_reexecuted:totals.paid,
 original_external_interventions_reexecuted:totals.external,
 actual_pid1_paid_meta_opcode_fetches:totals.metaFetch,
 actual_pid1_paid_meta_accumulator_checkpoints:totals.metaCheckpoint,
 actor_paid_composed_meta_program_executions:totals.metaMode,
 actor_paid_rule_slot62_63_writes_on_validated_candidates:totals.metaDescriptorWrites,
 actor_paid_effective_base_opcode_changes:effectiveBase,
 actor_paid_temporary_meta_instruction_offset_and_fallback_edits:totals.failedOffsetEdits,
 actor_paid_shadow_rule_writes:totals.shadowWrites,
 actor_paid_actual_task_program_slot_writes:totals.taskWrites,
 actor_paid_task_operator_fetches:totals.taskFetch,
 actor_paid_task_accumulator_writes:totals.taskCheckpoint,
 actor_paid_feedback_reads:totals.feedback,
 actor_paid_score_writes:totals.score,
 actual_original_pid1_service_outputs:totals.outputs,
 actor_paid_failure_fallbacks:totals.fallbacks,
 distinct_executed_two_opcode_rule_programs:[...newPairs].sort(),
 distinct_executed_task_bytecode_programs:[...virtualTaskPairs].sort(),
 exact_instruction_count_range:[Math.min(...counts),Math.max(...counts)],
 peak_storage_units:peak,
 first_divergence:null,
 original_PID1_continuous_in_each_world:true,
 original_source_unchanged:true,
 true_feedback_correct_heldout_per_16:accTruth,
 source_identical_truthteller_v_decoy_changes_future_meta_program:true,
 unchanged_native_CIL_vm:true,
 checked_invalid_sentinel3_branch_not_injected_as_control:true,
 controlled_unbounded_invalid_meta_opcodes:false,
 true_yoked_learned_rule_not_tested:true,
 all_meta_instruction_meanings_researcher_authored:true,
 full_endogenous_rule_synthesis_Hk:false,
 B53_authenticated_truth:false,N56_A_B_C:false,B3:false,B4:false,cyberlife:false,
 measured_transfer_by_seed:transfer,
 cases:results.map(({
  original_PID1_real_service_events,native_paid_meta_opcode_fetches,
  native_paid_meta_intermediate_checkpoints,
  native_task_program_writes,final_process,final_cells,...keep
 })=>keep)
},null,2));
