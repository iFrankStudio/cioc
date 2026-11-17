// B56-F0 independently replays every native paid CIL event and mutation.
// Novelty is bounded actor-paid assembly from authored generic templates.
import fs from 'node:fs';
import crypto from 'node:crypto';
import assert from 'node:assert/strict';
import {isDeepStrictEqual} from 'node:util';
import {CIL,founder as b51c} from './b51c_native_paid_search_replay.mjs';

const rust=fs.readFileSync(new URL('../../cl0/crates/world/tests/cl1b_b56_f0_paid_monitor_assembly.rs',import.meta.url),'utf8');
const manifest=fs.readFileSync(new URL('../studies/B56-F0-source-generation-native-graph-manifest-v1.md',import.meta.url));
const digest=crypto.createHash('sha256').update(manifest).digest('hex');
const file='cl1b-ci-evidence/b56_f0_native_steps.jsonl';
if(!fs.existsSync(file))throw Error('B56 F0 native evidence absent');
const lines=fs.readFileSync(file,'utf8').trimEnd().split(/\r?\n/);
let cursor=0,paid=0,externalCount=0,changedEdges=0,changedNodes=0;
function cmp(x,why){
  if(cursor>=lines.length)throw Error('B56 F0 truncated at '+why);
  const y=JSON.parse(lines[cursor++]);
  if(!isDeepStrictEqual(x,y))throw Error('B56 F0 first divergence '+why+
    ' row='+cursor+'\nexpected '+JSON.stringify(x)+'\nnative '+JSON.stringify(y));
}
const meta=JSON.parse(lines[cursor++]);
if(meta[0]!=='meta'||meta[1]!=='B56-F0'||meta[2]!==1||
   meta[4]!==digest||meta[5]!==manifest.length||meta[6]!==4000||
   meta[7]!==16||meta[8]!==180||meta[9]!==300||
   (process.env.GITHUB_SHA&&meta[3]!==process.env.GITHUB_SHA))
  throw Error('B56 F0 source SHA or frozen manifest mismatch');
const section=(a,b)=>{
  const i=rust.indexOf(a),j=rust.indexOf(b,i+a.length);
  if(i<0||j<0)throw Error('B56 F0 actual Rust source section not found');
  return rust.slice(i,j);
};
const src=section('fn build_assemble_founder()','fn build_static_founder(');
const pre=section('fn build_static_founder(','#[derive(Clone,Copy,Debug)]');
function assignments(s){
  return [...s.matchAll(/code\[(\d+)\]=Op::([A-Za-z]+)\{([^}]+)\};/g)]
    .map(m=>{
      const op={kind:m[2]};
      for(const a of m[3].matchAll(/(\w+)\s*:\s*(-?\d+)/g))
        op[a[1]]=Number(a[2]);
      return [Number(m[1]),op];
    });
}
const assign=assignments(src),staticAssign=assignments(pre);
if(!src.includes('code.resize(102,Op::Halt)')||
   !pre.includes('code.resize(126,Op::Halt)')||
   assign.length!==29||staticAssign.length!==18)
  throw Error('B56 F0 founder source inventory drift');
const serial=op=>op.kind==='Halt'?'Halt':{
  [op.kind]:Object.fromEntries(Object.entries(op).filter(([key])=>key!=='kind'))
};
const graphJSON=code=>code.map(serial);
const actorGraph=()=>{
  const x=b51c.map(o=>({...o}));
  while(x.length<102)x.push({kind:'Halt'});
  for(const [i,o] of assign)x[i]=o;
  if(x[50].next!==72||x[2].next!==35||
     x[72].value!==27||x[76].kind!=='CloneNode'||
     x[80].kind!=='CloneNode'||x[95].kind!=='PatchNext'||
     x[96].kind!=='Halt'||x[98].kind!=='Branch')
    throw Error('B56 F0 initial no-monitor graph topology drift');
  return x;
};
const staticGraph=periodic=>{
  const x=b51c.map(o=>({...o}));
  while(x.length<126)x.push({kind:'Halt'});
  for(const [i,o] of staticAssign)if(i!==49)x[i]=o;
  const exp=periodic?
    /if periodic\{\s*code\[49\]=Op::Const\{([^}]+)\};/:
    /\}else\{\s*code\[49\]=Op::Const\{([^}]+)\};/;
  const m=pre.match(exp);
  if(!m)throw Error('B56 source literal static branch missing');
  const op={kind:'Const'};
  for(const a of m[1].matchAll(/(\w+)\s*:\s*(-?\d+)/g))
    op[a[1]]=Number(a[2]);
  x[49]=op;
  if(op.value!==(periodic?110:120))throw Error('B56 static baseline drift');
  return x;
};
const arms=['A_ACTOR_ASSEMBLE','B_BLIND_ASSEMBLE','C_EDITS_DISABLED',
  'D_NO_ASSEMBLY','E_STATIC_EVERY','F_STATIC_FOURTH'];
const seeds=[101,211,307,401],held=[11,-7,0],train=[5,-3];
const keys=[7,11,20,21,22,24,25,27,28,30,31,40,41];
const cells=m=>keys.map(k=>m.cell(k));
const outputEvent=e=>{
  if(e.type==='executed')return['exec'];
  if(e.type==='read')return['read',e.key,e.value];
  if(e.type==='write')return['write',e.key,e.value];
  if(e.type==='patch')return['patch',e.node,e.before,e.after];
  if(e.type==='suppressed')return['suppressed',e.tag];
  if(e.type==='halt')return['halt'];
  if(e.type==='trap')return['trap',e.message];
  throw Error('Unknown independent CIL event '+e.type);
};
function inject(m,id,key,value,why){
  m.external(id,key,value,why);externalCount++;
  cmp(['ext',id,m.tick,key,value,why,m.credits,m.storage(),cells(m)],
    'external '+id+'/'+why+'/'+m.tick);
}
function step(m,id){
  const pc=m.pc,op=m.code[pc],oldLen=m.code.length;
  const old=['CloneNode','PatchNext','PatchConst'].includes(op.kind)?
    m.code.map(o=>({...o})):null;
  let e;
  if(m.editDisabled&&old){
    m.pc=op.next;m.paid++;m.tick++;m.credits--;
    e={type:'suppressed',tag:{CloneNode:8,PatchNext:6,PatchConst:9}[op.kind]};
  }else e=m.step(id).event;
  paid++;
  const diff=[];
  if(old)for(let i=0;i<m.code.length;i++){
    if(!isDeepStrictEqual(old[i],m.code[i])){
      diff.push([i,old[i]?serial(old[i]):null,serial(m.code[i])]);
    }
  }
  changedNodes+=diff.length;changedEdges+=Number(e.type==='patch');
  m.peak=Math.max(m.peak,m.storage());
  cmp(['step',id,m.paid,m.tick,pc,m.pc,outputEvent(e),
    [...m.regs],m.credits,m.stopped,m.code.length,m.storage(),cells(m),diff],
    'native paid '+id+'/'+m.paid+' PC'+pc);
  return [e,oldLen];
}
function task(idx,phase){
  return phase===1?(idx%2===0?[-1,3]:[1,2]):
    (idx%2===0?[1,-2]:[-1,-3]);
}
const result=[];
let cid=0;
for(const seed of seeds){
  const idx=seeds.indexOf(seed);
  for(const arm of arms){
    const id=cid++;
    const code=arm==='E_STATIC_EVERY'?staticGraph(false):
      arm==='F_STATIC_FOURTH'?staticGraph(true):actorGraph();
    const m=new CIL(task(idx,0)[0],task(idx,0)[1],'TRUE');
    m.code=code.map(o=>({...o}));
    m.pc=3;m.credits=4000;m.maxCodeNodes=180;
    m.maxStorage=300;m.maxDataCells=16;
    m.editDisabled=arm==='C_EDITS_DISABLED';m.peak=m.storage();
    cmp(['case',id,seed,arm,task(idx,0),task(idx,1),
      code.length,4000,m.editDisabled,graphJSON(code),m.storage()],
      'initial founder graph '+id);
    inject(m,id,27,arm==='D_NO_ASSEMBLY'?0:1,'initial_arm_selector');
    inject(m,id,7,train[0],'first_training_probe');
    let input=train[0],training=true,trainCount=0,serviceCount=0;
    const right=[0,0,0],wrong=[0,0,0];
    let rightAll=0,wrongAll=0,first72=null,sourceEdits=0,clones=0,
      reads=0,markers=0,created=0,connected=0,
      clone1=null,clone2=null,accepted=null,chosen=null,
      trainGood=0,trainWrong=0,peak=m.storage();
    while(m.paid<4000&&!m.stopped){
      const [e,oldLen]=step(m,id);
      peak=Math.max(peak,m.storage());
      if(m.code.length>oldLen){
        clones+=m.code.length-oldLen;
        if(m.code.length===105)clone1=104;
        if(m.code.length===106)clone2=105;
      }
      sourceEdits+=Number(e.type==='patch');
      if(e.type==='write'&&e.key===11){
        const stage=Math.min(2,Math.floor(serviceCount/24));
        const [sign,k]=task(idx,stage);
        const good=e.value===sign*input+k;
        if(training){
          trainCount++;
          if(good)trainGood++;else trainWrong++;
          inject(m,id,20,arm==='B_BLIND_ASSEMBLE'?1:Number(good),
            'candidate_training_feedback');
          input=train[trainCount%2];
          inject(m,id,7,input,'next_training_probe');
        }else{
          if(serviceCount<72){
            if(good)right[stage]++;else wrong[stage]++;
          }
          if(good)rightAll++;else wrongAll++;
          serviceCount++;if(serviceCount===72)first72=m.paid;
          inject(m,id,20,arm==='B_BLIND_ASSEMBLE'?1:Number(good),
            'service_feedback');
          input=held[((serviceCount%24)+idx%held.length)%held.length];
          inject(m,id,7,input,'next_service_input');
        }
      }else if(e.type==='patch'&&e.node===2&&e.before===42&&
       ((e.after===30&&['A_ACTOR_ASSEMBLE','B_BLIND_ASSEMBLE',
         'C_EDITS_DISABLED','D_NO_ASSEMBLY'].includes(arm))||
        (e.after===120&&arm==='E_STATIC_EVERY')||
        (e.after===110&&arm==='F_STATIC_FOURTH'))){
        if(!training||trainCount<2||trainCount%2)
          throw Error('B56 first task retention not observed causally');
        training=false;accepted=m.paid;
        chosen=[m.cell(22),m.cell(21)];
        input=held[idx%held.length];
        inject(m,id,7,input,'retained_service_start');
      }else if(e.type==='patch'&&e.node===2&&e.before===30&&e.after===104){
        connected++;
        if(m.code[104]?.kind!=='Const'||m.code[104].value!==20||
           m.code[104].next!==105||
           m.code[105]?.kind!=='Read'||m.code[105].next!==98)
          throw Error('B56 inventoried original founder could not contain this new control chain');
        created++;
      }else if(e.type==='read'&&e.key===20&&!training)reads++;
      else if(e.type==='write'&&e.key===25&&e.value===1&&!training)markers++;
    }
    const missing=72-Math.min(72,serviceCount);
    const C=right.reduce((a,b)=>a+b,0),W=wrong.reduce((a,b)=>a+b,0);
    assert.equal(C+W+missing,72);
    const row={
      case:id,seed,arm,paid:m.paid,stopped:m.stopped,
      initial_nodes:code.length,final_nodes:m.code.length,
      initial_credits:4000,peak_storage:peak,training_outputs:trainCount,
      train_correct:trainGood,train_wrong:trainWrong,retained_at:accepted,
      chosen,candidate_clones_and_monitor_clones:clones,
      native_executable_edge_edits:sourceEdits,
      assembly_clone_first:clone1,assembly_clone_second:clone2,
      created_monitor:created,writer_connected:connected,
      paid_service_feedback_reads:reads,failure_detected_marker:markers,
      stage_correct:right,stage_wrong:wrong,missing,
      service_writes:serviceCount,total_correct:rightAll,total_wrong:wrongAll,
      paid_to_first72:first72,
      historical_u:(C-2*W-missing)/Math.max(1,m.paid),
      final_pc:m.pc,final_regs:[...m.regs],final_graph:graphJSON(m.code),
      final_cells:cells(m)
    };
    cmp(['end',id,row],'native source final world '+id);
    result.push(row);
  }
}
if(cursor!==lines.length)throw Error('B56 unused native evidence rows '+(lines.length-cursor));
const byArm=Object.fromEntries(arms.map(arm=>{
  const a=result.filter(x=>x.arm===arm);
  return [arm,{
    worlds:a.length,paid:a.reduce((x,r)=>x+r.paid,0),
    correct:a.reduce((x,r)=>x+r.stage_correct.reduce((u,v)=>u+v,0),0),
    wrong:a.reduce((x,r)=>x+r.stage_wrong.reduce((u,v)=>u+v,0),0),
    missing:a.reduce((x,r)=>x+r.missing,0),
    all_correct:a.reduce((x,r)=>x+r.total_correct,0),
    service:a.reduce((x,r)=>x+r.service_writes,0),
    paid_feedback_reads:a.reduce((x,r)=>x+r.paid_service_feedback_reads,0),
    actor_source_monitor_links:a.reduce((x,r)=>x+r.created_monitor,0),
    detected_failure_markers:a.reduce((x,r)=>x+r.failure_detected_marker,0),
    per_seed:a.map(r=>({seed:r.seed,paid:r.paid,correct:r.stage_correct,
      wrong:r.stage_wrong,missing:r.missing,chosen:r.chosen,
      source_links:r.created_monitor,clones:r.candidate_clones_and_monitor_clones,
      service_feedback_reads:r.paid_service_feedback_reads}))
  }];
}));
const a=byArm.A_ACTOR_ASSEMBLE,c=byArm.C_EDITS_DISABLED,
  d=byArm.D_NO_ASSEMBLY;
const noPreseededServiceMonitor=actorGraph()[2].next===35&&
  actorGraph()[50].next===72&&actorGraph()[96].kind==='Halt'&&
  actorGraph().length===102;
const paidAssembly=noPreseededServiceMonitor&&a.actor_source_monitor_links===4&&
  a.paid_feedback_reads>0&&c.actor_source_monitor_links===0&&
  d.actor_source_monitor_links===0;
console.log(JSON.stringify({
  study:'B56-F0',source_sha:meta[3],manifest_sha256:digest,
  native_worlds_replayed:result.length,
  native_paid_steps_replayed:paid,
  native_external_interventions_replayed:externalCount,
  native_changed_source_edges_replayed:changedEdges,
  native_full_graph_node_mutations_replayed:changedNodes,
  first_divergence:null,
  complete_service_monitor_absent_in_founder:noPreseededServiceMonitor,
  actor_paid_monitor_assembly_feasible:paidAssembly,
  authored_assembly_not_self_originated_policy:paidAssembly,
  n56b_policy_selection_and_useful_advantage_not_tested:true,
  n56c_generator_second_order_revision_not_tested:true,
  b3_met:false,b4_met:false,cyberlife_established:false,
  by_arm:byArm
},null,2));
