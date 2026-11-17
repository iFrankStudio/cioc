// CL-1B B55a-PREFLIGHT independent instruction-by-instruction native replay.
// Narrow feasibility pilot only: 5 arms A/B/C/D/E × 2 ecologies E1/E2 ×
// 4 of 8 frozen seeds, NOT the full frozen H55a acceptance experiment.
// All actual paid CIL instructions + external writes are reexecuted from
// SOURCE-parsed B51c and B52 Rust, plus B55a native Rust code[idx] literals.
import fs from 'node:fs';
import crypto from 'node:crypto';
import assert from 'node:assert/strict';
import {isDeepStrictEqual} from 'node:util';
import {CIL,founder as b51c} from './b51c_native_paid_search_replay.mjs';
const rust=(name)=>fs.readFileSync(new URL('../../cl0/crates/world/tests/'+name,import.meta.url),'utf8');
const b52=rust('cl1b_b52_native_paid_revalidation.rs');
const pre=rust('cl1b_b55a_paid_monitor_choice_preflight.rs');
const online=rust('cl1b_b55b_online_monitor_rate_pilot.rs');
const main=fs.readFileSync('cl1b-ci-evidence/b55b_pilot_native_steps.jsonl','utf8')
  .trimEnd().split(/\r?\n/);
const manifestBytes=fs.readFileSync(
  new URL('../studies/B55b-pilot-executable-manifest-v1.md',import.meta.url));
const manifestSha=crypto.createHash('sha256').update(manifestBytes).digest('hex');
function parse(raw, regex){
  return [...raw.matchAll(regex)].map(m=>{
    const op={kind:m[2]};
    for(const field of m[3].matchAll(/(\w+)\s*:\s*(-?\d+)/g))
      op[field[1]]=Number(field[2]);
    return {index:Number(m[1]),op};
  });
}
const b52section=b52.match(/code\.extend\(\[([\s\S]*?)\]\);/)?.[1];
if(!b52section)throw Error('B55a missing live B52 Rust code');
const extra52=parse(b52section,/(?:\/\/\s*(\d+)\n)?Op::([A-Za-z]+)\s*\{([^}]+)\}/g);
// B52 no dense index is preceding opcode; validate 20 distinct ordinary CIL ops.
if(extra52.length!==20)throw Error('B55a B52 Rust dynamic source length mismatch');
const graph=[...b51c.map(op=>({...op})),...extra52.map(x=>x.op)];
if(graph.length!==92)throw Error('B55a inherited founder not92');
const added=parse(pre,/\bcode\[(\d+)\]=Op::([A-Za-z]+)\{([^}]+)\};/g);
if(!pre.includes('code.resize(178,Op::Halt)')||added.length!==51)
  throw Error('B55a pilot Rust source topology drift, fail closed');
while(graph.length<178)graph.push({kind:'Halt'});
for(const row of added){
  if(row.index<92||row.index>=178||graph[row.index].kind!=='Halt')
    throw Error('B55a wrong appended graph node '+row.index);
  graph[row.index]=row.op;
}
// Source-parse exactly the 2 modified and 31 newly activated CIL nodes.
const onlineAdded=parse(online,/\bcode\[(\d+)\]=Op::([A-Za-z]+)\{([^}]+)\};/g);
if(!online.includes('b55a::founder()')||onlineAdded.length!==33)
  throw Error('B55b source 2+31 CIL native literal count mismatch');
for(const {index,op} of onlineAdded){
  if(index<0||index>=178)throw Error('B55b source node out of bound');
  if(index!==79&&index!==155&&graph[index].kind!=='Halt')
    throw Error('B55b modified occupied founder node '+index);
  graph[index]=op;
}
if(graph[79].nonzero!==95||graph[79].zero!==103||
   graph[155].value!==160||graph[119].kind!=='PatchNext'||
   graph[158].kind!=='PatchNext')
  throw Error('B55b source topology precheck failed');
for(const [node,kind]of [[92,'Const'],[94,'PatchConst'],[120,'Const'],[138,'Read'],
  [145,'Const'],[175,'PatchConst'],[177,'PatchNext']]){
  if(graph[node].kind!==kind)throw Error('B55a wrong exact policy opcode '+node);
}
const meta=JSON.parse(main[0]);let row=1;
const sourceSha=process.env.GITHUB_SHA||meta[3];
assert.deepEqual(meta,['meta','B55b-PILOT',1,sourceSha,
  manifestSha,manifestBytes.length,178,4000]);
function take(expected,label){
  if(row>=main.length)throw Error('B55a native trace ends early at '+label);
  let found;
  try{found=JSON.parse(main[row++]);}catch(e){throw Error('B55a malformed trace row '+row+': '+e)}
  if(!isDeepStrictEqual(expected,found)){
    throw Error('B55a independent FIRST DIVERGENCE '+label+' row '+row+
    '\nexpected='+JSON.stringify(expected)+'\nfound='+JSON.stringify(found));
  }
}
function event(e){
  switch(e.type){
    case 'executed':return ['exec'];
    case 'read':return ['read',e.key,e.value];
    case 'write':return ['write',e.key,e.value];
    case 'patch':return ['patch',e.node,e.before,e.after];
    case 'halt':return ['halt'];
    case 'trap':return ['trap',e.message];
    default:throw Error('B55a unsupported independent CIL event '+e.type)
  }
}
const keys=[7,11,20,21,22,23,24,25,26,27,28,30,31];
const state=m=>keys.map(k=>m.cell(k));
const edge2=m=>m.edge(2);
function external(m,id,key,value,reason){
  m.external(id,key,value,reason);
  interventions++;
  take(['ext',id,m.tick,key,value,reason,m.credits,m.storage(),state(m)],
    'external '+id+' '+reason+' tick '+m.tick);
}
function pay(m,id){
  const r=m.step(id);paid++;
  take(['step',id,m.paid,m.tick,r.pc,m.pc,
    event(r.event),[...m.regs],m.credits,m.stopped,m.code.length,
    edge2(m),m.code[76].value,m.storage(),state(m)],
    'paid instruction '+id+'/'+m.paid);
  return r.event;
}
const SEEDS=[101,211,307,401],WINDOW=24,HELD=[11,-7,0],TRAIN=[5,-3];
let paid=0,interventions=0,patches=0;
const outputs=[],armNames=['A','B','C','D','E'];
function task(base,shift,ecology,stage){return ecology==='E2'&&stage===1?shift:base}
let id=0;
for(const ecology of ['E1','E2']){
  for(const seed of SEEDS){
    const index=SEEDS.indexOf(seed),tag=index%2===0?'A':'B';
    const base=tag==='A'?[1,-2]:[-1,-3];
    const shift=tag==='A'?[-1,3]:[1,2];
    for(const arm of armNames){
      const caseId=id++,fixed={A:0,B:77,C:0,D:30,E:160}[arm];
      const m=new CIL(base[0],base[1],'TRUE');
      m.code=graph.map(op=>({...op}));m.pc=92;m.credits=4000;
      m.maxCodeNodes=180;m.maxStorage=300;m.maxDataCells=16;
      m.peak=m.storage();
      take(['case',caseId,seed,arm,ecology,tag,base,shift,
        4000,178,m.storage()], 'initial world '+caseId);
      external(m,caseId,27,fixed,'fixed_arm_value_0_means_actor_choice');
      let input=TRAIN[0];
      external(m,caseId,7,input,'start_training');
      let stage='INITIAL_TRAIN',training=0,trial=0,service=0;
      let allServiceCorrect=0,allServiceWrong=0,first72Paid=null;
      const rateChanges=[];let paidConstRewrites=0;
      let reopened=0,retentions=0,trialObservations=0,monitorReads=0;
      let falseTrials=0,edits=0,chosen=0;
      const correct=[0,0,0],wrong=[0,0,0],recovery=[];
      let shiftStartStep=0,firstShiftCorrect=null;
      while(m.paid<4000&&!m.stopped){
        const pcBefore=m.pc;
        const e=pay(m,caseId);
        if(pcBefore===116||pcBefore===136){
          paidConstRewrites++;
          assert.ok([77,160].includes(m.code[76].value));
        }
        if(e.type==='patch'){edits++;patches++;}
        if(e.type==='write'&&e.key===11){
          const phase=Math.min(2,Math.floor(service/WINDOW));
          const [s,k]=task(base,shift,ecology,phase);
          const good=e.value===s*input+k;
          if(stage==='INITIAL_TRAIN'||stage==='REOPEN_TRAIN'){
            training++;
            external(m,caseId,20,Number(good),'task_training_truth');
            input=TRAIN[training%2];
            external(m,caseId,7,input,'next_training_probe');
          }else if(stage==='TRIAL'){
            trial++;
            const bit=arm==='C'?1:Number(good);
            if(!good)falseTrials++;
            external(m,caseId,20,bit,'trial_actor_visible_feedback');
            input=TRAIN[trial%2];
            external(m,caseId,7,input,'next_trial_probe');
          }else if(stage==='SERVICE'){
            if(good)allServiceCorrect++;else allServiceWrong++;
            if(service<3*WINDOW){
              if(good)correct[phase]++;else wrong[phase]++;
              if(ecology==='E2'&&phase===1&&good&&firstShiftCorrect===null)
                firstShiftCorrect=m.paid;
            }
            service++;
            if(ecology==='E2'&&service===WINDOW)shiftStartStep=m.paid;
            if(service===3*WINDOW)first72Paid=m.paid;
            external(m,caseId,20,arm==='C'?1:Number(good),
              arm==='C'?'service_blind_success':'service_feedback_truth');
            input=HELD[((service%WINDOW)+index%3)%HELD.length];
            external(m,caseId,7,input,'next_heldout_input');
          }else throw Error('B55a independent stage drift');
        }else if(e.type==='patch'&&e.node===2&&e.before===42&&e.after===120){
          assert.equal(stage,'INITIAL_TRAIN');
          retentions++;stage='TRIAL';input=TRAIN[0];
          external(m,caseId,7,input,'post_retention_trial_start');
        }else if(e.type==='patch'&&e.node===2&&e.before===120){
          assert.equal(stage,'TRIAL');assert.equal(trial,4);
          assert.ok([30,77,160].includes(e.after));
          chosen=e.after;retentions++;stage='SERVICE';input=HELD[index%3];
          external(m,caseId,7,input,'actor_source_choice_enters_service');
        }else if(e.type==='patch'&&e.node===2&&stage==='SERVICE'&&
                 ((e.before===160&&e.after===77)||
                  (e.before===77&&e.after===160))){
          assert.equal(m.code[76].value,e.after);
          chosen=e.after;
          rateChanges.push({before:e.before,after:e.after,paid:m.paid,
            service_writes_before:service,pc:pcBefore});
        }else if(e.type==='write'&&e.key===25&&e.value===1){
          assert.equal(stage,'SERVICE');
          reopened++;training=0;stage='REOPEN_TRAIN';input=TRAIN[0];
          external(m,caseId,7,input,'native_watchdog_reopens_search');
        }else if(e.type==='patch'&&e.node===2&&e.before===42&&
                 stage==='REOPEN_TRAIN'&&e.after===chosen){
          assert.equal(stage,'REOPEN_TRAIN');assert.equal(e.after,chosen);
          retentions++;stage='SERVICE';
          input=HELD[((service%WINDOW)+index%3)%HELD.length];
          external(m,caseId,7,input,'repaired_source_returns_service');
          recovery.push(m.paid);
        }else if(e.type==='read'&&e.key===20&&stage==='TRIAL'){
          trialObservations++;
        }else if(e.type==='read'&&e.key===20&&stage==='SERVICE'){
          monitorReads++;
        }
      }
      const missing=3*WINDOW-Math.min(service,3*WINDOW);
      const C=correct.reduce((a,b)=>a+b,0),W=wrong.reduce((a,b)=>a+b,0);
      assert.equal(C+W+missing,72);
      const end={
        id:caseId,seed,ecology,arm,task:tag,paid:m.paid,stopped:m.stopped,
        trial_outputs:trial,trial_observations:trialObservations,
        trial_wrong:falseTrials,monitor_choice:chosen,
        correct_by_stage:correct,wrong_by_stage:wrong,
        correct:C,wrong:W,missing,utility:(C-2*W-missing)/Math.max(1,m.paid),
        service_outputs:Math.min(service,72),
        all_service_outputs:service,all_correct:allServiceCorrect,
        all_wrong:allServiceWrong,first72_paid:first72Paid,
        feedback_reads:monitorReads,rate_changes:rateChanges,
        paid_rate_const_rewrites:paidConstRewrites,
        retentions,reopens:reopened,actor_patch_events:edits,
        program_len:m.code.length,storage:m.storage(),
        shift_start_paid:shiftStartStep,first_shift_correct_paid:firstShiftCorrect,
        recovery_patches_paid:recovery,initial_cap:[16,180,300],
        initial_budget:4000,final_cells:state(m)
      };
      take(['end',caseId,end], 'final native PID1 world '+caseId);
      outputs.push(end);
    }
  }
}
if(row!==main.length)
  throw Error('B55a has '+(main.length-row)+' extra unchecked native evidence rows');
const byEcology=Object.fromEntries(['E1','E2'].map(ecology=>[
  ecology,Object.fromEntries(armNames.map(arm=>{
    const worlds=outputs.filter(x=>x.ecology===ecology&&x.arm===arm);
    return[arm,{worlds:worlds.length,
      paid:worlds.reduce((n,x)=>n+x.paid,0),
      correct:worlds.reduce((n,x)=>n+x.correct,0),
      wrong:worlds.reduce((n,x)=>n+x.wrong,0),
      missing:worlds.reduce((n,x)=>n+x.missing,0),
      all_correct:worlds.reduce((n,x)=>n+x.all_correct,0),
      all_wrong:worlds.reduce((n,x)=>n+x.all_wrong,0),
      all_service:worlds.reduce((n,x)=>n+x.all_service_outputs,0),
      rate_up:worlds.reduce((n,x)=>n+x.rate_changes.filter(y=>y.before===160&&y.after===77).length,0),
      rate_down:worlds.reduce((n,x)=>n+x.rate_changes.filter(y=>y.before===77&&y.after===160).length,0),
      rate_const_rewrites:worlds.reduce((n,x)=>n+x.paid_rate_const_rewrites,0),
      mean_paid_to_first72:worlds.reduce((n,x)=>n+(x.first72_paid??4000),0)/worlds.length,
      reopens:worlds.reduce((n,x)=>n+x.reopens,0),
      choices:worlds.map(x=>x.monitor_choice),
      mean_utility:worlds.reduce((n,x)=>n+x.utility,0)/worlds.length,
      per_seed:worlds.map(x=>({seed:x.seed,C:x.correct,W:x.wrong,
        M:x.missing,P:x.paid,U:x.utility,chosen:x.monitor_choice,
        source_edits:x.actor_patch_events,reads:x.feedback_reads,
        all_correct:x.all_correct,all_service:x.all_service_outputs,
        first72_paid:x.first72_paid,rate_changes:x.rate_changes}))
    }];
  }))
]));
const initialHistoryAlias=outputs.filter(x=>x.arm==='A').every(x=>{
  const paired=outputs.find(y=>y.seed===x.seed&&y.arm==='A'&&y.ecology!==x.ecology);
  return paired&&x.trial_outputs===paired.trial_outputs&&
    x.trial_observations===paired.trial_observations&&
    x.monitor_choice===paired.monitor_choice;
});
const e2=outputs.filter(x=>x.ecology==='E2');
const A=e2.filter(x=>x.arm==='A'),B=e2.filter(x=>x.arm==='B'),
  C=e2.filter(x=>x.arm==='C');
const pilotConditionalE2Advantage=A.every(x=>{
  const b=B.find(y=>y.seed===x.seed),c=C.find(y=>y.seed===x.seed);
  return x.utility>b.utility && x.utility>c.utility;
});
const revisedAfterChange=A.every(x=>
  x.rate_changes.some(z=>z.before===160&&z.after===77&&z.paid>x.shift_start_paid)
  &&x.rate_changes.some(z=>z.before===77&&z.after===160&&z.paid>x.shift_start_paid));
const fixedUnchanged=outputs.filter(x=>['B','D','E'].includes(x.arm))
  .every(x=>x.rate_changes.length===0);
const recovered=A.every(x=>x.correct_by_stage[1]>0&&x.reopens>=1);
console.log(JSON.stringify({
  study:'B55b-pilot',source_sha:sourceSha,manifest_sha256:manifestSha,
  native_worlds_replayed:outputs.length,native_paid_steps_replayed:paid,
  native_external_interventions_replayed:interventions,
  native_source_edge_patches_replayed:patches,
  first_divergence:null,
  stage_policy_recommendation_same_before_hidden_shift:initialHistoryAlias,
  pilot_E2_utility_outperforms_B_and_C:pilotConditionalE2Advantage,
  actor_paid_rate_up_down_after_hidden_shift:revisedAfterChange,
  fixed_policy_routes_unchanged:fixedUnchanged,
  task_reacquisition_after_paid_source_edit:recovered,
  full_B55b_E3_mutation_oracle_controls_completed:false,
  H55b_full_mechanism_accepted:false,H55b_full_utility_accepted:false,
  tier2_endogenous_policy_formation:false,
  b3_met:false,b4_met:false,cyberlife_established:false,
  by_ecology:byEcology
},null,2));
