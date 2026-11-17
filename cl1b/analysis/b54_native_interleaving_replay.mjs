// B54 independent whole-run CIL interpreter. Native data is never used as
// a simulated state input: the B51c/B52 and B54 executable Rust source
// literals are parsed, then 80k CIL events independently reexecuted.
// Source-pinned true feedback/replay/forgery/torn controls, NOT cyberlife.
import fs from 'node:fs';
import {isDeepStrictEqual} from 'node:util';
import {CIL,founder as b51c} from './b51c_native_paid_search_replay.mjs';

const read=(name)=>fs.readFileSync(
  new URL('../../cl0/crates/world/tests/'+name,import.meta.url),'utf8');
const b52=read('cl1b_b52_native_paid_revalidation.rs');
const b53=read('cl1b_b53_native_response_correlation.rs');
const b54=read('cl1b_b54_native_interleaving.rs');
for(const needle of ['b53::founder()', 'late_bit_flip_after_paid_commit_check',
  'late_commit_flip_after_paid_commit_read', 'fn b54_native_paid_interleaving_falsifiers']){
  if(!b54.includes(needle))throw Error('B54 native timing/source drift '+needle);
}
for(const needle of [
  'b52::founder()', 'const BUDGET:usize=4000;',
  'const WINDOW:usize=24;', 'with_strict_capacity(16,180,300)',
  'proc.pc=92;', 'pub(crate) fn founder()->Vec<Op>{'
]){
  const src=needle==='pub(crate) fn founder()->Vec<Op>{'?b52:b53;
  if(!src.includes(needle))throw Error('B54 actual native founder/protocol drift: '+needle);
}
function appended(rust,expected){
  const block=rust.match(/\bcode\.extend\(\[([\s\S]*?)\]\);/)?.[1];
  if(!block)throw Error('B54 native CIL append block missing');
  const nodes=[...block.matchAll(/Op::([A-Za-z]+)\s*\{([^}]+)\}/g)].map(m=>{
    const op={kind:m[1]};
    for(const field of m[2].matchAll(/(\w+)\s*:\s*(-?\d+)/g))
      op[field[1]]=Number(field[2]);
    return op;
  });
  if(nodes.length!==expected)throw Error('B54 code literal count drift '+nodes.length+' expected '+expected);
  return nodes;
}
const b52op=appended(b52,20),b53op=appended(b53,32);
const graph=[...b51c.map(op=>({...op})),...b52op,...b53op];
if(graph.length!==124||graph[34].next!==2||graph[49].next!==50||
   graph[76].next!==50||graph[92].next!==93||
   graph[97].next!==50||graph[120].zero!==80||
   graph[123].next!==30)
  throw Error('B54 sourced exact executable graph topology mismatch');

const path='cl1b-ci-evidence/b54_native_steps.jsonl';
if(!fs.existsSync(path))throw Error('B54 same-revision native trace missing (fail closed)');
const lines=fs.readFileSync(path,'utf8').trimEnd().split(/\r?\n/);
let n=0,paidCount=0,externalCount=0,labStageCount=0,nativePatches=0;
function next(){
  if(n>=lines.length)throw Error('B54 truncated native trace after row '+n);
  try{return JSON.parse(lines[n++]);}
  catch(e){throw Error('B54 unparsable native JSONL row '+n+': '+e)}
}
function compare(expected,tag){
  const observed=next();
  if(!isDeepStrictEqual(expected,observed))
    throw Error('B54 FIRST DIVERGENCE '+tag+' row '+n+
      '\nindependently expected='+JSON.stringify(expected)+
      '\nactual native='+JSON.stringify(observed));
}
const meta=next();
if(meta.type!=='meta'||meta.study!=='B54'||meta.version!==1||
   meta.founder_source!=='cl1b_b53_native_response_correlation.rs'||
   meta.serial_protocol!=='native_inc26_echo27_commit28_bit20'||
   !meta.source_sha||(process.env.GITHUB_SHA&&meta.source_sha!==process.env.GITHUB_SHA))
  throw Error('B54 native source SHA / metadata mismatch');
const TRAIN=[5,-3],HELD=[11,-7,0],WINDOW=24,BUDGET=4000;
const keys=[7,11,20,21,22,24,25,26,27,28,29,30,31];
const state=m=>keys.map(k=>m.cell(k));
const arms=['FRESH_ATOMIC','LATE_BIT_FLIP','LATE_COMMIT_FLIP',
  'STABLE_FORGED_CURRENT','OLD_NONCE_REPLAY'];
function task(a,b,stage){return stage===1?b:a}
function ext(m,id,key,value,reason){
  m.external(id,key,value,reason);
  externalCount++;
  compare({type:'external',case:id,reason,key,value,tick:m.tick,
    credits:m.credits,storage:m.storage(),cells:state(m)},
    'external '+id+'/'+reason+'/'+m.tick);
}
function pay(m,id){
  const r=m.step(id);
  paidCount++;
  nativePatches+=Number(r.event.type==='patch');
  compare({type:'step',case:id,step:r.step,tick:r.tick,
    pc:r.pc,pc_after:r.pc_after,event:r.event,regs:r.regs,
    credits:r.credits,stopped:r.stopped,code_len:r.code_len,
    growth:r.growth,edge2:m.edge(2),edge34:m.edge(34),
    edge49:m.edge(49),edge76:m.edge(76),
    edge124:m.edge(124),edge125:m.edge(125),
    storage:m.storage(),cells:state(m)},
    'native paid instruction '+id+'/'+m.paid);
  return r.event;
}
function packet(m,id,bit,echo,commit,reason){
  ext(m,id,20,bit,reason+'_bit');
  ext(m,id,27,echo,reason+'_echo');
  ext(m,id,28,commit,reason+'_commit');
}
const transitions=[
  ['A',[1,-2],[-1,3]],
  ['B',[-1,-3],[1,2]]
];
const outcomes=[];
let id=0;
for(const [transition,a,b]of transitions){
  for(const phase of [0,1]){
    for(const arm of arms){
      const caseId=id++;
      const m=new CIL(a[0],a[1],'TRUE');
      m.code=graph.map(op=>({...op}));
      m.pc=92;m.credits=BUDGET;m.maxCodeNodes=180;
      m.maxStorage=300;m.maxDataCells=16;m.peak=m.storage();
      compare({type:'case',case:caseId,transition,
        from:a,to:b,arm,phase,initial_len:124,
        initial_credits:BUDGET,initial_storage:m.storage()},
        'B54 initial actor '+caseId);
      if(phase===1){
        const e=pay(m,caseId);
        if(e.type!=='executed'||m.pc!==93)
          throw Error('B54 charged task-unobserving phase control drift');
      }
      let input=TRAIN[0];
      ext(m,caseId,7,input,'initial_training');
      let trial=true,trialOutputs=0,retained=0,reopened=0;
      let choices=[],serviceCount=0;
      const correct=[0,0,0],wrong=[0,0,0];
      let rejected=0,feedbackReads=0,echoReads=0,commitReads=0;
      let acceptedLies=0,serialWrites=0,lastSerial=0;
      let priorPacket=[0,0],successfulPacket=[0,1];
      let previousTruth=true,postRetentionPatches=0,bootstrapPatches=0;
      let trainingOutputsTotal=0,trainingWrong=0,serviceFeedbackPackets=0;
      let lateBitFlips=0,lateCommitFlips=0,lastLateBitSerial=0;
      while(m.paid<BUDGET&&!m.stopped){
        if(!trial&&arm==='LATE_BIT_FLIP'&&!previousTruth&&
           m.pc===119&&lastLateBitSerial!==lastSerial){
          ext(m,caseId,20,1,'late_bit_flip_after_paid_commit_check');
          lateBitFlips++;lastLateBitSerial=lastSerial;
        }
        const e=pay(m,caseId);
        if(retained>0&&e.type==='patch')postRetentionPatches++;
        if(e.type==='patch'&&(
             (e.node===76&&e.before===50&&e.after===97)||
             (e.node===34&&e.before===2&&e.after===99)||
             (e.node===49&&e.before===50&&e.after===76)))
          bootstrapPatches++;
        if(e.type==='write'&&e.key===26){
          if(e.value!==lastSerial+1)throw Error('B54 actor native serial not monotonic');
          lastSerial=e.value;serialWrites++;
        }else if(e.type==='write'&&e.key===11){
          if(lastSerial<=0||m.cell(26)!==lastSerial)
            throw Error('B54 output not bound to actor-written serial');
          const stage=Math.min(2,Math.floor(serviceCount/WINDOW));
          const [s,k]=task(a,b,stage);
          const good=e.value===s*input+k;
          const nonce=lastSerial;
          if(trial){
            trialOutputs++;trainingOutputsTotal++;
            if(!good)trainingWrong++;
            packet(m,caseId,Number(good),nonce,nonce,'training_truth');
            priorPacket=[nonce,Number(good)];
            if(good)successfulPacket=[nonce,1];
            input=TRAIN[trialOutputs%TRAIN.length];
            ext(m,caseId,7,input,'training_next');
          }else{
            if(serviceCount<3*WINDOW){
              if(good)correct[stage]++;else wrong[stage]++;
              serviceCount++;
              if(serviceCount%WINDOW===0&&serviceCount<3*WINDOW){
                compare({type:'lab_transition',case:caseId,
                  stage:serviceCount/WINDOW,after_service_outputs:serviceCount,
                  charged:m.paid,tick:m.tick},
                  'lab-only hidden task switch '+caseId+'/'+serviceCount);
                labStageCount++;
              }
            }
            let echo,bit,commit;
            switch(arm){
              case 'FRESH_ATOMIC':
              case 'LATE_BIT_FLIP':
              case 'LATE_COMMIT_FLIP':
                [echo,bit,commit]=[nonce,Number(good),nonce];break;
              case 'STABLE_FORGED_CURRENT':
                [echo,bit,commit]=[nonce,1,nonce];break;
              case 'OLD_NONCE_REPLAY':
                [echo,bit,commit]=[successfulPacket[0],successfulPacket[1],successfulPacket[0]];break;
              default:throw Error('Unknown B54 frozen arm');
            }
            packet(m,caseId,bit,echo,commit,'service_packet');
            serviceFeedbackPackets++;
            priorPacket=[nonce,Number(good)];
            previousTruth=good;
            input=HELD[((serviceCount%WINDOW)+phase)%HELD.length];
            ext(m,caseId,7,input,'service_next');
          }
        }else if(e.type==='patch'&&e.node===2&&e.before===42&&e.after===106){
          if(!trial||trialOutputs<2||trialOutputs%2!==0)
            throw Error('B54 selection without two native paid trial probes');
          trial=false;retained++;
          choices.push([m.cell(22),m.cell(21)]);
          input=HELD[phase];
          ext(m,caseId,7,input,'retained_start');
        }else if(e.type==='write'&&e.key===25&&e.value===1){
          if(trial||retained<=0)throw Error('B54 illegitimate restart request');
          trial=true;reopened++;trialOutputs=0;input=TRAIN[0];
          ext(m,caseId,7,input,'native_reopen_request');
        }else if(e.type==='write'&&e.key===29&&e.value===1){
          if(trial||retained<=0)throw Error('B54 rejection outside service');
          rejected++;
        }else if(e.type==='read'&&e.key===27&&!trial){
          echoReads++;
        }else if(e.type==='read'&&e.key===28&&!trial){
          commitReads++;
          if(arm==='LATE_COMMIT_FLIP'){
            ext(m,caseId,28,lastSerial-1,
              'late_commit_flip_after_paid_commit_read');
            lateCommitFlips++;
          }
        }else if(e.type==='read'&&e.key===20&&!trial){
          feedbackReads++;
          if(e.value===1&&!previousTruth)acceptedLies++;
        }
      }
      if(m.paid!==BUDGET||bootstrapPatches!==3)
        throw Error('B54 failed source-identical finite-budget bootstrap');
      const end={type:'end',case:caseId,transition,
        from:a,to:b,arm,phase,paid:m.paid,credits:m.credits,
        stopped:m.stopped,retentions:retained,reopens:reopened,
        choices,service_writes:serviceCount,
        window_correct:correct,window_wrong:wrong,
        nonce_writes:serialWrites,final_nonce:lastSerial,
        response_echo_reads:echoReads,response_commit_reads:commitReads,
        service_feedback_reads:feedbackReads,rejections:rejected,
        false_accepted_success:acceptedLies,
        late_bit_flips:lateBitFlips,late_commit_flips:lateCommitFlips,
        post_retention_patches:postRetentionPatches,
        bootstrap_patches:bootstrapPatches,
        training_outputs:trainingOutputsTotal,training_wrong:trainingWrong,
        service_feedback_packets:serviceFeedbackPackets,
        final_pc:m.pc,final_len:m.code.length,peak_storage:m.peak,
        final_regs:[...m.regs],final_cells:state(m),
        final_edge2:m.edge(2),final_edge34:m.edge(34),
        final_edge76:m.edge(76)};
      compare(end,'native B54 final world summary '+caseId);
      outcomes.push(end);
    }
  }
}
if(n!==lines.length)throw Error('B54 trailing unverified native events '+(lines.length-n));
const byArm=Object.fromEntries(arms.map(arm=>{
  const rows=outcomes.filter(r=>r.arm===arm);
  return[arm,{
    worlds:rows.length,paid:rows.reduce((s,r)=>s+r.paid,0),
    service_writes:rows.reduce((s,r)=>s+r.service_writes,0),
    correct_by_stage:[0,1,2].map(i=>rows.reduce((s,r)=>s+r.window_correct[i],0)),
    wrong_by_stage:[0,1,2].map(i=>rows.reduce((s,r)=>s+r.window_wrong[i],0)),
    retentions:rows.reduce((s,r)=>s+r.retentions,0),
    reopens:rows.reduce((s,r)=>s+r.reopens,0),
    rejected_packets:rows.reduce((s,r)=>s+r.rejections,0),
    accepted_false_successes:rows.reduce((s,r)=>s+r.false_accepted_success,0),
    feedback_reads:rows.reduce((s,r)=>s+r.service_feedback_reads,0),
    nonce_writes:rows.reduce((s,r)=>s+r.nonce_writes,0),
    late_bit_flips:rows.reduce((s,r)=>s+r.late_bit_flips,0),
    late_commit_flips:rows.reduce((s,r)=>s+r.late_commit_flips,0),
    per_world:rows.map(r=>({
      transition:r.transition,phase:r.phase,chosen:r.choices,
      service:r.service_writes,correct:r.window_correct,wrong:r.window_wrong,
      rejected:r.rejections,false_success:r.false_accepted_success,
      late_bit_flips:r.late_bit_flips,late_commit_flips:r.late_commit_flips,
      retentions:r.retentions,reopens:r.reopens,paid:r.paid
    }))
  }];
}));
console.log(JSON.stringify({
  study:'B54',source_sha:meta.source_sha,
  independently_replayed_worlds:outcomes.length,
  independently_replayed_paid_native_instructions:paidCount,
  independently_replayed_external_interventions:externalCount,
  verified_lab_only_task_transitions:labStageCount,
  verified_native_executable_patches:nativePatches,
  first_divergence:null,
  fresh_control_reacquired:byArm.FRESH_ATOMIC.reopens>=8 &&
    byArm.FRESH_ATOMIC.accepted_false_successes===0,
  timed_bit_toctou_falsifier_confirmed:byArm.LATE_BIT_FLIP.late_bit_flips>0&&
    byArm.LATE_BIT_FLIP.accepted_false_successes>0 &&
    byArm.LATE_BIT_FLIP.reopens<byArm.FRESH_ATOMIC.reopens,
  commit_read_snapshot_not_atomic:byArm.LATE_COMMIT_FLIP.late_commit_flips>0 &&
    byArm.LATE_COMMIT_FLIP.feedback_reads>0,
  current_nonce_content_forgery_detected:false,
  original_old_nonce_replay_rejected:byArm.OLD_NONCE_REPLAY.accepted_false_successes===0,
  source_blind_endogenous_policy_formation:false,
  b3_met:false,b4_met:false,cyberlife_established:false,
  by_arm:byArm
},null,2));
