// Independent B52 whole-CIL event interpreter. It parses the actual
// B51c base from its own source and B52 appended CIL instructions from Rust,
// never using a native trace as initial state or future state input.
// Narrow engineered causal test: NOT B3/B4 or a cyberlife result.
import fs from 'node:fs';
import {isDeepStrictEqual} from 'node:util';
import assert from 'node:assert/strict';
import {CIL,founder as base} from './b51c_native_paid_search_replay.mjs';

const path=new URL('../../cl0/crates/world/tests/cl1b_b52_native_paid_revalidation.rs',import.meta.url);
const rust=fs.readFileSync(path,'utf8');
for(const needle of [
  'b51c::founder()', 'const BUDGET:usize=3000;',
  'const WINDOW:usize=24;', 'with_native_executable_edit_trace()',
  'with_strict_capacity(16,160,256)', 'p.pc=72;',
  'post_retention_patches','stale_false_successes',
]){
  if(!rust.includes(needle))throw Error('B52 native source protocol drift: '+needle);
}
const appendedSrc=rust.match(/\bcode\.extend\(\[([\s\S]*?)\]\);/)?.[1];
if(!appendedSrc)throw Error('B52 actual Rust appended opcode block missing');
const parsed=[...appendedSrc.matchAll(/Op::([A-Za-z]+)\s*\{([^}]+)\}/g)]
  .map(m=>{
    const op={kind:m[1]};
    for(const field of m[2].matchAll(/(\w+)\s*:\s*(-?\d+)/g))
      op[field[1]]=Number(field[2]);
    return op;
  });
if(base.length!==72||parsed.length!==20 ||
   parsed[2].kind!=='PatchNext'||parsed[2].next!==75||
   parsed[5].kind!=='Const'||parsed[6].kind!=='Read'||
   parsed[7].kind!=='Branch'||parsed[19].kind!=='PatchNext')
  throw Error('B52 frozen base/appended executable structure mismatch');
const init=[...base.map(n=>({...n})),...parsed];
assert.equal(init.length,92);
assert.equal(init[49].next,50);
assert.equal(init[2].next,35);

const file='cl1b-ci-evidence/b52_native_steps.jsonl';
if(!fs.existsSync(file))throw Error('B52 current-CI native trace missing');
const records=fs.readFileSync(file,'utf8').trimEnd().split(/\r?\n/);
let index=0,paidEvents=0,externals=0,sourceEdits=0;
function take(){
  if(index>=records.length)throw Error('B52 native trace truncated at '+index);
  try{return JSON.parse(records[index++]);}
  catch(e){throw Error('B52 unparseable evidence row '+index+': '+e);}
}
function match(expected,tag){
  const found=take();
  if(!isDeepStrictEqual(expected,found))
    throw Error('B52 independent first divergence '+tag+' row '+index+
      '\nexpected '+JSON.stringify(expected)+
      '\nfound '+JSON.stringify(found));
}
const meta=take();
if(meta.study!=='B52'||meta.type!=='meta'||meta.version!==1||
   meta.base_founder!=='cl1b_b51c_paid_search_retention.rs'||!meta.source_sha)
  throw Error('B52 missing source-pin/real-native provenance');
if(process.env.GITHUB_SHA&&meta.source_sha!==process.env.GITHUB_SHA)
  throw Error('B52 wrong-source native trace; fail closed');
const BUDGET=3000,WINDOW=24,TRAIN=[5,-3],HELD=[11,-7,0];
const transitions=[
  ['A',[1,-2],[-1,3]],
  ['B',[-1,-3],[1,2]]
];
const arms=['TRUE','STALE_SUCCESS','FORCED_ZERO','STATIONARY_TRUE','STALE_DELAYED'];
const mode=(a,b,arm,stage)=>stage===0||arm==='STATIONARY_TRUE'||stage===2?a:b;
const cellKeys=[7,11,20,21,22,24,25,30,31];
const cells=m=>cellKeys.map(k=>m.cell(k));
const xwrite=(m,caseId,key,value,reason)=>{
  // Real World::intervene emits one external tick, no computation charged.
  m.external(caseId,key,value,reason);
  externals++;
  return {type:'external',case:caseId,tick:m.tick,key,value,reason,
    credits:m.credits,storage:m.storage(),cells:cells(m)};
};
const step=(m,caseId)=>{
  const r=m.step(caseId);
  paidEvents++;
  sourceEdits+=Number(r.event.type==='patch');
  return {type:'step',case:caseId,step:r.step,
    tick:r.tick,pc:r.pc,pc_after:r.pc_after,
    event:r.event,regs:r.regs,credits:r.credits,stopped:r.stopped,
    code_len:r.code_len,growth:r.growth,
    edge2:r.edge2,edge33:r.edge33,
    edge49:m.edge(49),edge72:m.edge(92),edge73:m.edge(93),
    cells:cells(m),storage:r.storage};
};
const outcomes=[];
let caseId=0;
for(const [tag,a,b] of transitions){
  for(const phase of [0,1]){
    for(const arm of arms){
      const id=caseId++;
      const m=new CIL(a[0],a[1],'TRUE');
      m.code=init.map(n=>({...n}));
      m.pc=72;m.credits=BUDGET;m.peak=m.storage();
      match({type:'case',case:id,transition:tag,from:a,to:b,arm,phase,
        initial_len:92,initial_credits:BUDGET,
        initial_storage:m.storage()},'case header '+id);
      if(phase===1){
        const r=step(m,id);match(r,'phase paid initializer '+id);
        assert.equal(r.event.type,'executed');
        assert.equal(m.pc,73);
      }
      let input=TRAIN[0];
      match(xwrite(m,id,7,input,'initial_training'),'initial x '+id);
      let trial=true,trialOutputs=0,retained=0,reopened=0;
      const choices=[],correct=[0,0,0],wrong=[0,0,0];
      let serviceTotal=0,serviceFeedbackReads=0;
      let postRetentionPatches=0,prevTrueFeedback=1;
      let staleFalseSuccesses=0,bootstraps=0,allFeedbackReads=0;
      let trainingOutputsTotal=0,trainingCorrect=0,trainingWrong=0;
      while(m.paid<BUDGET&&!m.stopped){
        const row=step(m,id);
        match(row,'paid CIL event '+id+'/'+m.paid);
        const event=row.event;
        if(event.type==='read'&&event.key===20)allFeedbackReads++;
        if(event.type==='patch'&&event.node===49&&
           event.before===50&&event.after===76)bootstraps++;
        if(retained>0 && event.type==='patch')postRetentionPatches++;
        if(event.type==='write'&&event.key===11){
          const stage=Math.min(2,Math.floor(serviceTotal/WINDOW));
          const [s,k]=mode(a,b,arm,stage);
          const good=event.value===s*input+k;
          if(trial){
            trialOutputs++;trainingOutputsTotal++;
            if(good)trainingCorrect++;else trainingWrong++;
            match(xwrite(m,id,20,Number(good),'trial_feedback'),
              'trial post-output feedback '+id+'/'+m.paid);
            input=TRAIN[trialOutputs%TRAIN.length];
            match(xwrite(m,id,7,input,'training_next'),
              'trial next probe '+id+'/'+m.paid);
          }else{
            if(serviceTotal<3*WINDOW){
              if(good)correct[stage]++;else wrong[stage]++;
              serviceTotal++;
            }
            const fb=arm==='TRUE'||arm==='STATIONARY_TRUE'?Number(good):
              arm==='FORCED_ZERO'?0:
              arm==='STALE_SUCCESS'?1:prevTrueFeedback;
            if(arm!=='STALE_SUCCESS')
              match(xwrite(m,id,20,fb,'service_feedback'),
                'service post-output feedback '+id+'/'+m.paid);
            if(fb===1&&!good)staleFalseSuccesses++;
            prevTrueFeedback=Number(good);
            input=HELD[((serviceTotal%WINDOW)+phase)%HELD.length];
            match(xwrite(m,id,7,input,'service_next'),
              'service next input '+id+'/'+m.paid);
          }
        }else if(event.type==='patch'&&event.node===2&&
                 event.before===42&&event.after===77){
          if(!trial||trialOutputs<2||trialOutputs%2!==0)
            throw Error('B52 unearned or incorrectly grouped retention');
          trial=false;retained++;
          choices.push([m.cell(22),m.cell(21)]);
          input=HELD[phase];
          match(xwrite(m,id,7,input,'retained_start'),
            'native paid source retention '+id+'/'+m.paid);
          prevTrueFeedback=1;
        }else if(event.type==='write'&&event.key===25&&event.value===1){
          if(trial||retained<=0)
            throw Error('B52 illegitimate trial restart without retention');
          trial=true;reopened++;trialOutputs=0;input=TRAIN[0];
          match(xwrite(m,id,7,input,'native_reopen_request'),
            'native paid source restart signal '+id+'/'+m.paid);
        }else if(event.type==='read'&&event.key===20&&!trial){
          serviceFeedbackReads++;
        }
      }
      if(m.paid!==BUDGET)throw Error('B52 stopped without budget match '+id);
      if(bootstraps!==1)throw Error('B52 missing paid bootstrap edge '+id);
      const end={type:'end',case:id,transition:tag,from:a,to:b,arm,phase,
        paid:m.paid,stopped:m.stopped,credits:m.credits,
        retentions:retained,reopens:reopened,choices,
        service_writes:serviceTotal,window_correct:correct,
        window_wrong:wrong,
        service_feedback_reads:serviceFeedbackReads,
        all_feedback_reads:allFeedbackReads,
        post_retention_patches:postRetentionPatches,
        stale_false_successes:staleFalseSuccesses,
        training_outputs:trainingOutputsTotal,
        training_correct:trainingCorrect,training_wrong:trainingWrong,
        bootstrapped_edges:bootstraps,peak_storage:m.peak,
        final_pc:m.pc,final_len:m.code.length,
        final_regs:[...m.regs],final_cells:cells(m),
        final_edge2:m.edge(2),final_edge49:m.edge(49)};
      match(end,'end of B52 native world '+id);
      outcomes.push(end);
    }
  }
}
if(index!==records.length)
  throw Error('B52 native trace contains '+(records.length-index)+' unchecked events');
const byArm=Object.fromEntries(arms.map(a=>{
  const rows=outcomes.filter(r=>r.arm===a);
  return[a,{
    worlds:rows.length,paid:rows.reduce((s,r)=>s+r.paid,0),
    retentions:rows.reduce((s,r)=>s+r.retentions,0),
    reopens:rows.reduce((s,r)=>s+r.reopens,0),
    correct:rows.reduce((s,r)=>s+r.window_correct.reduce((u,v)=>u+v,0),0),
    wrong:rows.reduce((s,r)=>s+r.window_wrong.reduce((u,v)=>u+v,0),0),
    correct_stages:[0,1,2].map(i=>rows.reduce((n,r)=>n+r.window_correct[i],0)),
    wrong_stages:[0,1,2].map(i=>rows.reduce((n,r)=>n+r.window_wrong[i],0)),
    feedback_reads:rows.reduce((s,r)=>s+r.service_feedback_reads,0),
    source_edits:rows.reduce((s,r)=>s+r.post_retention_patches,0),
    stale_false_successes:rows.reduce((s,r)=>s+r.stale_false_successes,0),
    service_writes:rows.reduce((s,r)=>s+r.service_writes,0),
    winners:rows.map(r=>({transition:r.transition,phase:r.phase,
      sequence:r.choices,serviceByStage:r.window_correct,
      wrongByStage:r.window_wrong,retentions:r.retentions,reopens:r.reopens}))
  }];
}));
const trueArm=byArm.TRUE;
const causalUseful=trueArm.correct-trueArm.wrong >
  byArm.STALE_SUCCESS.correct-byArm.STALE_SUCCESS.wrong &&
  trueArm.correct-trueArm.wrong >
  byArm.FORCED_ZERO.correct-byArm.FORCED_ZERO.wrong;
const watchdogEvidence=outcomes.filter(r=>r.arm==='TRUE').every(r=>
  r.reopens>=1 && r.retentions>=2&&r.service_feedback_reads>0&&
  r.post_retention_patches>0);
const allReacquired=outcomes.filter(r=>r.arm==='TRUE').every(r=>
  r.window_correct[1]===24&&r.window_wrong[1]===0&&
  r.window_correct[2]===24&&r.window_wrong[2]===0);
const staleCannotBeProvedFresh=byArm.STALE_SUCCESS.stale_false_successes>0||
  byArm.STALE_DELAYED.stale_false_successes>0;
console.log(JSON.stringify({
  study:'B52',source_sha:meta.source_sha,
  independently_replayed_native_worlds:outcomes.length,
  independently_replayed_paid_events:paidEvents,
  independently_replayed_external_interventions:externals,
  native_actor_executable_edits:sourceEdits,
  first_divergence:null,
  authored_watchdog_actually_acts:watchdogEvidence,
  feedback_recovery_causal_utility:causalUseful,
  all_shift_and_return_stages_reacquired:allReacquired,
  stale_replay_causally_indistinguishable_without_freshness:staleCannotBeProvedFresh,
  b52_freshness_certified:false,
  b3_met:false,b4_met:false,cyberlife_established:false,
  by_arm:byArm
},null,2));
