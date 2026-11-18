// B56-F1c-PILOT: independent Rust-source-parsed CIL VM for two paid-assembled
// native monitor paths. Narrow fixed-order feasibility only, not full F1.
import fs from 'node:fs';
import crypto from 'node:crypto';
import assert from 'node:assert/strict';
import {isDeepStrictEqual} from 'node:util';
import {CIL,founder as b51c} from './b51c_native_paid_search_replay.mjs';
const rust=fs.readFileSync(new URL('../../cl0/crates/world/tests/cl1b_b56_f1c_pilot_ordered_trial.rs',import.meta.url),'utf8');
const manifest=fs.readFileSync(new URL('../studies/B56-F1c-pilot-native-order-balanced-discriminating-trials-manifest-v1.md',import.meta.url));
const manifestSha=crypto.createHash('sha256').update(manifest).digest('hex');
const file='cl1b-ci-evidence/b56_f1c_pilot_native_steps.jsonl';
if(!fs.existsSync(file))throw Error('B56 F1c pilot native trace missing');
const lines=fs.readFileSync(file,'utf8').trimEnd().split(/\r?\n/);
let cursor=0,paidSteps=0,externals=0,sourceEdges=0,sourceChanges=0;
function take(expected,where){
  if(cursor>=lines.length)throw Error('B56 F1c pilot native evidence truncated at '+where);
  const got=JSON.parse(lines[cursor++]);
  if(!isDeepStrictEqual(got,expected))throw Error('B56 F1c pilot first divergence: '+where+
    ' row '+cursor+'\nexpected '+JSON.stringify(expected)+'\nactual '+JSON.stringify(got));
}
const meta=JSON.parse(lines[cursor++]);
if(meta[0]!=='meta'||meta[1]!=='B56-F1c-PILOT'||meta[2]!==1||
   meta[4]!==manifestSha||meta[5]!==manifest.length||
   meta[6]!==4000||meta[7]!==16||meta[8]!==180||meta[9]!==300||
   (process.env.GITHUB_SHA&&meta[3]!==process.env.GITHUB_SHA))
  throw Error('B56 F1c pilot manifest/source SHA or native-world budget differs');
function slice(a,b){
  const i=rust.indexOf(a),j=rust.indexOf(b,i+a.length);
  if(i<0||j<0)throw Error('Cannot source-parse Rust '+a+'/'+b);
  return rust.slice(i,j);
}
const maker=slice('fn build_two()->Vec<Op>','fn build_static(fourth:bool)');
const staticMaker=slice('fn build_static(fourth:bool)','#[derive(Copy,Clone)]');
function parse(s){
  return [...s.matchAll(/code\[(\d+)\]=Op::([A-Za-z]+)\{([^}]+)\};/g)]
    .map(m=>{
      const op={kind:m[2]};
      for(const x of m[3].matchAll(/(\w+)\s*:\s*(-?\d+)/g))op[x[1]]=Number(x[2]);
      return [Number(m[1]),op];
    });
}
const actorLiteral=parse(maker),fixedLiteral=parse(staticMaker);
if(!maker.includes('code.resize(162,Op::Halt)')||
   !maker.includes('code[50]=Op::PatchNext{node:2,target:3,next:72}')||
   !staticMaker.includes('code.resize(126,Op::Halt)')||
   actorLiteral.length!==90 ||fixedLiteral.length!==18)
  throw Error('B56 F1c pilot source compiler grammar inventory changed');
function actorGraph(){
  const code=b51c.map(op=>({...op}));
  while(code.length<162)code.push({kind:'Halt'});
  for(const [i,op]of actorLiteral)code[i]=op;
  if(code[50].next!==72||code[2].next!==35||
    code[75].kind!=='Const'||code[76].kind!=='CloneNode'||
    code[80].kind!=='CloneNode'||code[94].value!==2||
    code[95].kind!=='PatchNext'||code[95].next!==102||code[103].next!==130||
    code[117].zero!==104||code[152].zero!==149||
    code[148].next!==30||code[161].next!==121||
    code[127].kind!=='PatchNext'||code[158].kind!=='PatchNext')
    throw Error('B56 F1c pilot inventoried actor-program provenance drift');
  return code;
}
function staticGraph(fourth){
  const code=b51c.map(op=>({...op}));
  while(code.length<126)code.push({kind:'Halt'});
  for(const [i,op]of fixedLiteral)code[i]=op;
  const value=fourth?110:120;
  const exact=fourth?
    'if fourth{code[49]=Op::Const{dst:3,value:110,next:50};}':
    'else{code[49]=Op::Const{dst:3,value:120,next:50};}';
  if(!staticMaker.includes(exact))throw Error('Source-authored static comparator drift');
  code[49]={kind:'Const',dst:3,value,next:50};
  return code;
}
const opJSON=o=>o.kind==='Halt'?'Halt':{
  [o.kind]:Object.fromEntries(Object.entries(o).filter(([k])=>k!=='kind'))
};
const programJSON=code=>code.map(opJSON);
const keys=[7,11,20,21,22,24,25,27,28,29,30,31,40,41,42,43];
const cells=m=>keys.map(k=>m.cell(k));
const seeds=[101,211,307,401],train=[5,-3],held=[11,-7,0],WINDOW=24;
const arms=['A_TRUTH_TWO','B_BLIND_TWO','C_INVERTED_TWO','D_NO_ASSEMBLY','E_STATIC_EVERY','F_STATIC_FOURTH'];
const ecologies=['S_STATIONARY','D_EARLY_SHIFT_RETURN'];
function task(i,serviceCount,eco){
  const base=i%2===0?[1,-2]:[-1,-3],shift=i%2===0?[-1,3]:[1,2];
  return eco==='D_EARLY_SHIFT_RETURN'&&serviceCount>=2&&serviceCount<24?shift:base;
}
function kind(e){
  if(e.type==='executed')return['exec'];
  if(e.type==='read')return['read',e.key,e.value];
  if(e.type==='write')return['write',e.key,e.value];
  if(e.type==='patch')return['patch',e.node,e.before,e.after];
  if(e.type==='suppressed')return['suppressed',e.tag];
  if(e.type==='trap')return['trap',e.message];
  if(e.type==='halt')return['halt'];
  throw Error('B56 F1c pilot unsupported CIL event '+JSON.stringify(e));
}
function ext(m,id,key,value,reason){
  m.external(id,key,value,reason);externals++;
  take(['ext',id,m.tick,key,value,reason,m.credits,m.storage(),cells(m)],
    'externally origin event '+id+'/'+reason+'/'+m.tick);
}
function edge(m,node){return m.code[node]&&Object.hasOwn(m.code[node],'next')?
  m.code[node].next:null}
function literal94(m){return m.code[94]?.kind==='Const'?m.code[94].value:null}
function step(m,id){
  const pc=m.pc,op=m.code[pc],oldLen=m.code.length;
  const original=['CloneNode','PatchNext','PatchConst'].includes(op.kind)?
    m.code.map(o=>({...o})):null;
  const r=m.step(id),e=r.event;paidSteps++;
  const diff=[];
  if(original){
    for(let i=0;i<m.code.length;i++){
      if(!isDeepStrictEqual(original[i],m.code[i])){
        diff.push([i,original[i]?opJSON(original[i]):null,opJSON(m.code[i])]);
      }
    }
  }
  sourceEdges+=Number(e.type==='patch');
  sourceChanges+=diff.length;
  m.peak=Math.max(m.peak,m.storage());
  take(['step',id,m.paid,m.tick,pc,m.pc,kind(e),[...m.regs],
    m.credits,m.stopped,m.code.length,edge(m,2),edge(m,95),edge(m,148),
    literal94(m),m.storage(),cells(m),diff],
    'paid native instruction '+id+'/'+m.paid+' pc='+pc);
  return {pc,oldLen,e};
}
let id=0;
const outcomes=[];
for(const seed of seeds){
  const idx=seeds.indexOf(seed);
  for(const ecology of ecologies){
    for(const arm of arms){
    const caseId=id++;
    const code=arm==='E_STATIC_EVERY'?staticGraph(false):
      arm==='F_STATIC_FOURTH'?staticGraph(true):actorGraph();
    const m=new CIL(task(idx,0,ecology)[0],task(idx,0,ecology)[1],'TRUE');
    m.code=code.map(x=>({...x}));m.pc=3;m.credits=4000;
    m.maxCodeNodes=180;m.maxDataCells=16;m.maxStorage=300;
    m.peak=m.storage();
    const orderAFirst=idx<2;
    take(['case',caseId,seed,ecology,arm,task(idx,0,ecology),
      idx%2===0?[-1,3]:[1,2],orderAFirst?'A_THEN_B':'B_THEN_A',
      4000,programJSON(code),m.storage()],'source inventoried case '+caseId);
    ext(m,caseId,27,arm==='D_NO_ASSEMBLY'?0:1,'prestart_authorized_assembly_gate');
    ext(m,caseId,43,orderAFirst?1:0,'prestart_balanced_trial_order');
    let input=train[0];
    ext(m,caseId,7,input,'initial_training_probe');
    let training=true,trainingWrites=0,service=0,allCorrect=0,allWrong=0;
    const C=[0,0,0],W=[0,0,0];
    let first72=null,retained=false,clones=0,edges=0,editAttempts=0;
    let aLinked=false,bLinked=false,aTrials=0,bTrials=0,aReads=0,bReads=0;
    let firstRoute=null,afterFourRoute=null,trialReadsA=0,trialReadsB=0;
    let finalChoice=null,selectPaid=null,shiftWrong=0,markers=0,peak=m.storage();
    while(m.paid<4000&&!m.stopped){
      const {pc,oldLen,e}=step(m,caseId);
      const newLen=m.code.length;
      clones+=newLen-oldLen;peak=Math.max(peak,m.storage());
      if(e.type==='patch')edges++;
      if([132,135,87,91,95,161,156].includes(pc))editAttempts++;
      if((pc===156||pc===97)&&m.cell(29)===8){
        finalChoice=edge(m,2);selectPaid=m.paid;
      }
      if(pc===127&&m.cell(29)===0)firstRoute=edge(m,2);
      if([109,158].includes(pc)&&m.cell(29)===4)
        afterFourRoute=edge(m,2);
      if(e.type==='write'&&e.key===11){
        const phase=Math.min(2,Math.floor(service/WINDOW));
        const [sign,k]=task(idx,service,ecology);
        const good=e.value===sign*input+k;
        if(training){
          trainingWrites++;
          ext(m,caseId,20,Number(good),'truthful_training_feedback');
          input=train[trainingWrites%train.length];
          ext(m,caseId,7,input,'next_training_probe');
        }else{
          if(service<3*WINDOW){if(good)C[phase]++;else W[phase]++;}
          if(good)allCorrect++;else allWrong++;
          service++;
          if(service===72)first72=m.paid;
          const fb=arm==='B_BLIND_TWO'?1:
            arm==='C_INVERTED_TWO'?Number(!good):Number(good);
          const why=arm==='B_BLIND_TWO'?'service_constant_false_success':
            arm==='C_INVERTED_TWO'?'service_inverted_feedback':'truthful_service_feedback';
          ext(m,caseId,20,fb,why);
          input=held[((service%WINDOW)+idx%held.length)%held.length];
          ext(m,caseId,7,input,'next_service_probe');
          if(service<=8){
            if((service<=4)===orderAFirst)aTrials++;
            else bTrials++;
          }
          if(!good&&phase===1)shiftWrong++;
        }
      }else if(e.type==='patch'&&e.node===2&&e.before===42&&
               !retained&&[30,110,120].includes(e.after)){
        assert.ok(training&&trainingWrites>=2&&trainingWrites%2===0);
        training=false;retained=true;input=held[idx%held.length];
        ext(m,caseId,7,input,'paid_source_retained_service');
      }else if(e.type==='patch'&&e.node===2&&e.before===30&&e.after===164){
        assert.ok(retained&&!aLinked);aLinked=true;assert.equal(newLen,166);
      }else if(pc===161&&e.type==='patch'&&e.node===2&&e.before===164&&e.after===136){
        assert.ok(retained&&aLinked&&!bLinked);bLinked=true;assert.equal(newLen,168);
      }else if(e.type==='read'&&e.key===20&&!training){
        if(edge(m,2)===164){aReads++;if(service<=8)trialReadsA++;}
        else if(edge(m,2)===136){bReads++;if(service<=8)trialReadsB++;}
      }else if(e.type==='write'&&e.key===25&&e.value===1&&!training)markers++;
    }
    const correct=C.reduce((a,b)=>a+b,0),wrong=W.reduce((a,b)=>a+b,0);
    const missing=72-Math.min(72,service);
    assert.equal(correct+wrong+missing,72);
    const result={id:caseId,seed,ecology,
      order:orderAFirst?'A_THEN_B':'B_THEN_A',arm,paid:m.paid,retained,
      training_writes:trainingWrites,service_writes:service,
      stage_correct:C,stage_wrong:W,correct,wrong,missing,
      all_correct:allCorrect,all_wrong:allWrong,first72_paid:first72,
      native_clone_count:clones,native_changed_edges:edges,
      paid_builder_edit_attempts:editAttempts,
      monitor_a_linked:aLinked,monitor_b_linked:bLinked,
      a_trial_outputs:aTrials,b_trial_outputs:bTrials,
      a_feedback_reads:aReads,b_feedback_reads:bReads,
      trial_a_reads:trialReadsA,trial_b_reads:trialReadsB,
      first_route:firstRoute,after_four_route:afterFourRoute,
      selected_service_edge:finalChoice,selection_paid:selectPaid,
      shifted_wrong_outputs:shiftWrong,failure_marker_events:markers,
      peak_storage:peak,final_nodes:m.code.length,final_pc:m.pc,
      final_regs:[...m.regs],final_graph:programJSON(m.code),
      final_cells:cells(m),stopped:m.stopped,
      historical_u:(correct-2*wrong-missing)/Math.max(1,m.paid)};
    take(['end',caseId,result],'native outcome world '+caseId);
    outcomes.push(result);
    }
  }
}
if(cursor!==lines.length)throw Error('B56 F1c pilot unchecked extra native evidence lines');
const aggregate=Object.fromEntries(arms.map(arm=>{
  const rows=outcomes.filter(r=>r.arm===arm);
  return [arm,{worlds:rows.length,
    correct:rows.reduce((n,r)=>n+r.correct,0),
    wrong:rows.reduce((n,r)=>n+r.wrong,0),
    missing:rows.reduce((n,r)=>n+r.missing,0),
    all_correct:rows.reduce((n,r)=>n+r.all_correct,0),
    all_wrong:rows.reduce((n,r)=>n+r.all_wrong,0),
    clones:rows.reduce((n,r)=>n+r.native_clone_count,0),
    first_monitor_links:rows.filter(r=>r.monitor_a_linked).length,
    second_monitor_links:rows.filter(r=>r.monitor_b_linked).length,
    paid_monitor_reads:rows.reduce((n,r)=>n+r.a_feedback_reads+r.b_feedback_reads,0),
    selected:rows.map(r=>r.selected_service_edge),
    per_seed:rows.map(r=>({seed:r.seed,ecology:r.ecology,order:r.order,
      training:r.training_writes,
      a_link:r.monitor_a_linked,b_link:r.monitor_b_linked,
      a_trial:r.a_trial_outputs,b_trial:r.b_trial_outputs,
      reads_a:r.a_feedback_reads,reads_b:r.b_feedback_reads,
      first:r.first_route,after4:r.after_four_route,
      selected:r.selected_service_edge,correct:r.correct,wrong:r.wrong,
      missing:r.missing,all_correct:r.all_correct}))
  }];
}));
const a=outcomes.filter(r=>r.arm==='A_TRUTH_TWO');
const b=outcomes.filter(r=>r.arm==='B_BLIND_TWO');
const trueWithinPIDTwoDistinctPaths=a.every(r=>
  r.monitor_a_linked&&r.monitor_b_linked&&r.a_trial_outputs>=4&&
  r.b_trial_outputs>=4&&r.trial_a_reads>=4&&r.trial_b_reads>=1&&
  r.native_clone_count===6);
const blindCanStillAssemble=b.every(r=>r.monitor_a_linked&&r.monitor_b_linked);
const correctOrderSwitch=a.every(r=>
  (r.order==='A_THEN_B'?
    r.first_route===164&&r.after_four_route===136:
    r.first_route===null&&r.after_four_route===164));
const distinctTruthfulChoiceVsBlindD=a.filter(r=>r.ecology==='D_EARLY_SHIFT_RETURN')
  .every(r=>{
    const counterpart=b.find(x=>x.seed===r.seed&&x.ecology===r.ecology);
    return r.selected_service_edge===164&&counterpart?.selected_service_edge===136;
  });
const stationaryTruthfulChoosesFour=a.filter(r=>r.ecology==='S_STATIONARY')
  .every(r=>r.selected_service_edge===136);
const invertedWrongSignal=outcomes.filter(r=>r.arm==='C_INVERTED_TWO'&&r.ecology==='S_STATIONARY')
  .every(r=>r.selected_service_edge===164);
console.log(JSON.stringify({
  study:'B56-F1c-PILOT',source_sha:meta[3],manifest_sha256:manifestSha,
  native_worlds_replayed:outcomes.length,native_paid_steps_replayed:paidSteps,
  external_interventions_replayed:externals,
  executable_edge_changes_replayed:sourceEdges,
  executable_node_changes_replayed:sourceChanges,
  first_divergence:null,
  two_different_actor_paid_monitor_graphs_both_executed:trueWithinPIDTwoDistinctPaths,
  blind_source_assembled_same_topologies:blindCanStillAssemble,
  source_order_balanced_across_both_task_families:correctOrderSwitch,
  conditional_truth_vs_blind_choice_changed_in_early_shift:distinctTruthfulChoiceVsBlindD,
  stationary_true_chooses_fourth:stationaryTruthfulChoosesFour,
  stationary_inverted_feedback_causes_false_every:invertedWrongSignal,
  order_counterbalanced:true,monitor_only_edit_ablation:false,
  narrow_paid_feedback_sensitive_selection:distinctTruthfulChoiceVsBlindD,
  full_B56_F1_registered_controls_complete:false,
  n56a_strict_de_novo_monitor_origin_met:false,n56b_met:false,
  n56c_met:false,b3_met:false,b4_met:false,cyberlife_established:false,
  by_arm:aggregate
},null,2));
