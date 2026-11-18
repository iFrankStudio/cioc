#!/usr/bin/env node
// F1d1g independent native event-by-event reexecution of a same-PID
// 77-node CIL interpreter and multi-slot virtual instruction program.
// Does not confer algorithm-origin novelty on human-authored primitives.
import fs from 'node:fs';
import crypto from 'node:crypto';
import assert from 'node:assert/strict';
import {isDeepStrictEqual} from 'node:util';

const nativeSource=fs.readFileSync(new URL(
 '../../cl0/crates/world/tests/cl1b_b56_f1d1g_same_pid_multi_bytecode.rs',import.meta.url),'utf8');
const manifest=fs.readFileSync(new URL(
 '../studies/B56-F1d1g-multi-op-bytecode-exact-layout-and-edit-suppression-v1.md',import.meta.url));
const hash=b=>crypto.createHash('sha256').update(b).digest('hex');
const maker=nativeSource.split('fn founder()->Vec<Op>{')[1]?.split('#[derive(Copy,Clone)]')[0];
assert.ok(maker?.includes('vec!['),'original Rust CIL source changed unexpectedly');
const sourceNodes=[...maker.matchAll(/Op::(\w+)(?:\{([^}]+)\})?,\s*\/\/ pc (\d+)/g)];
assert.equal(sourceNodes.length,77,'expected original 77-node native script');
const op=(kind,props={})=>kind==='Halt'?'Halt':{[kind]:props};
const code=sourceNodes.map((m,i)=>{
 assert.equal(Number(m[3]),i,'source numbering drift: '+i);
 const args=Object.fromEntries([...(m[2]||'').matchAll(/(\w+):(-?\d+)/g)]
  .map(x=>[x[1],Number(x[2])]));
 assert.ok(m[1]==='Halt'||Object.hasOwn(args,'next')||m[1]==='Branch',
  'malformed CIL source node '+i);
 return op(m[1],args);
});
assert.ok(code.every(n=>!n.GraphLaunch&&!n.GraphNew&&!n.GraphEncode),
 'source now has an unexpected graph launch/encode primitive');
assert.deepEqual(code[26],op('Read',{dst:3,address:2,next:27}));
assert.deepEqual(code[54],op('Read',{dst:3,address:2,next:55}));
assert.deepEqual(code[49],op('Write',{address:2,value:1,next:50}));
assert.deepEqual(code[59],op('Const',{dst:2,value:52,next:60}));
assert.deepEqual(code[65],op('Const',{dst:2,value:60,next:66}));
assert.deepEqual(code[75],op('Write',{address:2,value:0,next:22}));

const sourceEvidence='cl1b-ci-evidence/b56_f1d1g_same_pid_multi_bytecode_native_steps.jsonl';
assert.ok(fs.existsSync(sourceEvidence),'missing ORIGINAL same-SHA native F1d1g JSONL evidence');
const lines=fs.readFileSync(sourceEvidence,'utf8').trimEnd().split(/\r?\n/);
let position=0,paid=0,external=0,outputWrites=0,feedbackReads=0;
let actualProgramWrites=0,virtualFetches=0,actualAccWrites=0,shadowWrites=0;
function take(expected,why){
 if(position===lines.length)throw Error('truncated original native event stream '+why);
 const actual=JSON.parse(lines[position++]);
 if(!isDeepStrictEqual(actual,expected))
  throw Error('F1d1g FIRST NATIVE DIVERGENCE '+why+' at original line '+position+
   '\nEXPECTED '+JSON.stringify(expected)+'\nACTUAL '+JSON.stringify(actual));
}
const meta=JSON.parse(lines[position++]);
assert.deepEqual(meta.slice(0,3),['meta','B56-F1d1g',1]);
assert.equal(meta[4],hash(manifest));
assert.equal(meta[5],manifest.length);
assert.deepEqual(meta.slice(6),[24,1000,16,180,300,26,54,49]);
if(process.env.GITHUB_SHA)assert.equal(meta[3],process.env.GITHUB_SHA);
const keys=[7,11,20,21,31,50,51,52,53,54,55,56,60];
function snapshot(s){return [s.pc,[...s.regs],s.credits,s.stopped,code]}
function cell(s,k){return s.data.has(k)?s.data.get(k):null}
function cells(s){return keys.map(k=>cell(s,k))}
function occupied(s){return 1+code.length+s.data.size}
const cases=[];
for(const seed of [0,1]){
 for(const task of ['IDENTITY','COMPOSED']){
  for(const regime of ['TRUTH','BLIND_SUCCESS','INVERTED']){
   for(const enabled of [true,false]){
    const id=cases.length,initial=[5,7][seed],held=[11,-7][seed];
    const s={
     pc:0,regs:[0,0,0,0],credits:1000,stopped:false,
     data:new Map(),tick:0,outputs:[],firstTruth:0,delivered:null,
     opcodeFetches:[],accumulatorWrites:[],feedbackReads:0,
     programEdits:0,shadowWrites:0,maxStorage:78,n:0
    };
    assert.equal(occupied(s),78);
    take(['case',id,seed,task,regime,enabled,initial,held,
       1000,16,180,300,code,occupied(s)],'initial source case'+id);
    function ext(key,value,reason){
     const before=occupied(s);external++;s.tick++;s.data.set(key,value);
     assert.ok(s.data.size<=16&&occupied(s)<=300);
     take(['ext',id,s.tick,key,value,reason,before,snapshot(s),occupied(s),cells(s)],
        'external paid-independent environment '+id+'/'+reason);
     s.maxStorage=Math.max(s.maxStorage,occupied(s));
    }
    ext(21,Number(enabled),'authorized_edit_control');
    ext(7,initial,'first_environment_input');
    while(!s.stopped){
     const pc=s.pc,raw=code[pc];
     assert.ok(raw!==undefined,'unrepresented CIL PC '+pc);
     assert.ok(s.credits>0);
     const kind=raw==='Halt'?'Halt':Object.keys(raw)[0];
     const attrs=raw==='Halt'?{}:raw[kind];
     const reg=i=>{assert.ok(i>=0&&i<=3);return s.regs[i]};
     s.n++;paid++;s.tick++;s.credits--;
     let event='Executed';
     switch(kind){
      case 'Const':
       s.regs[attrs.dst]=attrs.value;s.pc=attrs.next;break;
      case 'Read':{
       const key=reg(attrs.address),value=s.data.get(key)??0;
       s.regs[attrs.dst]=value;s.pc=attrs.next;
       event=op('CellRead',{key,value});
       if(pc===26){
        const virtualPC=s.data.get(50);
        assert.ok(Number.isInteger(virtualPC)&&virtualPC>=0&&virtualPC<=3);
        assert.equal(key,51+virtualPC);
        assert.equal(value,s.data.get(key));
        s.opcodeFetches.push(value);virtualFetches++;
       }
       if(pc===54){
        assert.equal(key,20);feedbackReads++;s.feedbackReads++;
       }
       break;
      }
      case 'Write':{
       const key=reg(attrs.address),value=reg(attrs.value);
       s.data.set(key,value);s.pc=attrs.next;
       event=op('CellWritten',{key,value});
       if((key===52&&value===1)||(key===53&&value===2)){
        assert.ok(enabled);
        s.programEdits++;actualProgramWrites++;
       }
       if(key===60){assert.equal(enabled,false);s.shadowWrites++;shadowWrites++;}
       if(key===56){s.accumulatorWrites.push(value);actualAccWrites++;}
       if(key===11){
        assert.equal(pc,49);
        s.outputs.push(value);outputWrites++;
        assert.ok(s.outputs.length<=2);
       }
       break;
      }
      case 'Add':
       s.regs[attrs.dst]=reg(attrs.lhs)+reg(attrs.rhs);
       s.pc=attrs.next;break;
      case 'Sub':
       s.regs[attrs.dst]=reg(attrs.lhs)-reg(attrs.rhs);
       s.pc=attrs.next;break;
      case 'Branch':
       s.pc=reg(attrs.test)!==0?attrs.nonzero:attrs.zero;break;
      case 'Halt':s.stopped=true;event='Halted';break;
      default:throw Error('new original CIL opcode in source '+kind);
     }
     s.maxStorage=Math.max(s.maxStorage,occupied(s));
     assert.ok(occupied(s)<=300&&s.data.size<=16);
     assert.equal(s.credits,1000-s.n);
     take(['step',id,s.n,s.tick,1,pc,event,snapshot(s),occupied(s),cells(s)],
      'real PID1 CIL instruction '+id+'/'+s.n+' pc'+pc);
     if(s.outputs.length===1&&!s.data.has(20)){
      s.firstTruth=Number(s.outputs[0]===(task==='IDENTITY'?initial:-initial+2));
      s.delivered=regime==='TRUTH'?s.firstTruth:
       regime==='BLIND_SUCCESS'?1:1-s.firstTruth;
      ext(20,s.delivered,'feedback_after_first_paid_return');
      ext(7,held,'heldout_input_after_real_service');
     }
     assert.ok(s.n<=500);
    }
    assert.equal(s.outputs.length,2);
    assert.equal(s.feedbackReads,1);
    const composed=enabled&&s.delivered===0;
    assert.deepEqual(s.opcodeFetches,
      composed?[0,3,0,1,2,3]:[0,3,0,3]);
    assert.deepEqual(s.accumulatorWrites,
      composed?[initial,held,-held,-held+2]:[initial,held]);
    assert.equal(s.programEdits,composed?2:0);
    assert.equal(s.shadowWrites,!enabled&&s.delivered===0?2:0);
    assert.equal(s.data.get(51),0);
    assert.equal(s.data.get(52),composed?1:3);
    assert.equal(s.data.get(53),composed?2:3);
    assert.equal(s.data.get(54),3);
    assert.equal(s.data.get(55),2);
    assert.equal(s.data.get(31),1);
    const secondWanted=task==='IDENTITY'?held:-held+2;
    const correct=s.outputs[1]===secondWanted;
    const result={
     id,seed,task,feedback_regime:regime,edit_enabled:enabled,
     first_output:s.outputs[0],second_output:s.outputs[1],
     first_truth:s.firstTruth,delivered_feedback:s.delivered,
     heldout_correct:correct,
     actual_virtual_opcode_fetches:s.opcodeFetches,
     actual_accumulator_writes:s.accumulatorWrites,
     paid_feedback_reads:s.feedbackReads,
     real_program_slot_edits:s.programEdits,
     shadow_write_events:s.shadowWrites,
     final_program_slots:[51,52,53,54].map(k=>cell(s,k)),
     charged_steps:s.n,max_storage:s.maxStorage,
     remaining_credits:s.credits,
     final_process:snapshot(s),same_PID1:true,
     native_source_unchanged:true,self_generated_opcode_semantics:false,
     b3_met:false,b4_met:false,cyberlife:false
    };
    take(['end',id,result],'final program state '+id);
    cases.push(result);
   }
  }
 }
}
assert.equal(position,lines.length,'unchecked original native events');
assert.equal(cases.length,24);
assert.equal(external,96);
assert.equal(outputWrites,48);
assert.equal(feedbackReads,24);
const acc=(regime,edit)=>cases.filter(x=>x.feedback_regime===regime&&x.edit_enabled===edit&&x.heldout_correct).length;
assert.deepEqual([
 acc('TRUTH',true),acc('BLIND_SUCCESS',true),acc('INVERTED',true),
 acc('TRUTH',false),acc('BLIND_SUCCESS',false),acc('INVERTED',false)
],[4,2,0,2,2,2]);
const minPaid=Math.min(...cases.map(x=>x.charged_steps));
const maxPaid=Math.max(...cases.map(x=>x.charged_steps));
const maxStorage=Math.max(...cases.map(x=>x.max_storage));
console.log(JSON.stringify({
 study:'B56-F1d1g',source_sha:meta[3],
 frozen_detailed_manifest_sha256:hash(manifest),
 rust_native_test_source_sha256:hash(Buffer.from(nativeSource)),
 source_parsed_CIL_node_count:code.length,
 original_native_worlds_entirely_replayed:cases.length,
 actual_paid_CIL_instructions_independently_reexecuted:paid,
 external_interventions_replayed:external,
 dynamic_virtual_opcode_fetches:virtualFetches,
 native_paid_feedback_reads:feedbackReads,
 native_PID1_real_program_slot_edits:actualProgramWrites,
 paid_shadow_write_events:shadowWrites,
 native_accumulator_checkpoint_writes:actualAccWrites,
 real_PID1_service_writes:outputWrites,
 charged_steps_per_world_range:[minPaid,maxPaid],
 max_total_storage:maxStorage,
 first_divergence:null,
 exact_PID1_continuity:true,
 native_executable_code_unchanged:true,
 real_multi_opcode_composition_in_enabled_truth_composed_cases:true,
 same_source_edit_suppression_control:true,
 heldout_correct_by_feedback:{
  truth_enabled:acc('TRUTH',true),
  blind_enabled:acc('BLIND_SUCCESS',true),
  inverted_enabled:acc('INVERTED',true),
  truth_suppressed:acc('TRUTH',false),
  blind_suppressed:acc('BLIND_SUCCESS',false),
  inverted_suppressed:acc('INVERTED',false),
 },
 primitive_semantics_researcher_authored:true,
 full_new_algorithm_invention:false,
 strict_N56_A_B_C:false,B3:false,B4:false,cyberlife:false,
 by_case:cases.map(({final_process,...x})=>x)
},null,2));
