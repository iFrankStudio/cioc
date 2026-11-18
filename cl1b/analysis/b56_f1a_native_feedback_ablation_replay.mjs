// CL-1B B56-F1a: source-parsed, independent CIL VM replay of all five
// native founder-feedback route/truth ablations. NOT two-monitor B56-F1.
import fs from 'node:fs';
import assert from 'node:assert/strict';
import {isDeepStrictEqual} from 'node:util';
import {CIL,founder as b51} from './b51c_native_paid_search_replay.mjs';

const rust=fs.readFileSync(new URL('../../cl0/crates/world/tests/cl1b_b56_f1_founder_feedback_cut.rs',import.meta.url),'utf8');
const original=fs.readFileSync(new URL('../../cl0/crates/world/tests/cl1b_b51c_paid_search_retention.rs',import.meta.url),'utf8');
const trace='cl1b-ci-evidence/b56_f1a_native_feedback_ablation_steps.jsonl';
if(!fs.existsSync(trace))throw Error('B56 F1a original native JSONL missing');
const data=fs.readFileSync(trace,'utf8').trimEnd().split(/\r?\n/);
let position=0, paid=0, interventions=0,patches=0;
function take(expected,label){
  if(position>=data.length)throw Error('B56 F1a trace unexpectedly ended '+label);
  const found=JSON.parse(data[position++]);
  if(!isDeepStrictEqual(found,expected))
    throw Error('B56 F1a FIRST DIVERGENCE at '+label+' row '+position+
      '\nexpected '+JSON.stringify(expected)+'\nfound '+JSON.stringify(found));
}
const meta=JSON.parse(data[position++]);
if(meta[0]!=='meta'||meta[1]!=='B56-F1a'||meta[2]!==1||
   meta[4]!==1300||meta[5]!==16||meta[6]!==180||meta[7]!==300||
   (process.env.GITHUB_SHA&&process.env.GITHUB_SHA!==meta[3]))
  throw Error('B56 F1a missing/mismatched exact-source native meta');

const cs=b51.map(op=>({...op}));
if(cs.length!==72||!original.includes('pub(crate) fn founder()')||
  !rust.includes('let mut code=b51c::founder()')||
  !rust.includes('Arm::BlindSuccess=>1')||
  !rust.includes('Arm::InvertedFeedback=>i64::from(!good)')||
  !rust.includes('code[2]=Op::Write{address:2,value:0,next:30}')||
  !rust.includes('code[36]=Op::Read{dst:3,address:2,next:30}'))
  throw Error('B56 F1a actual Rust source ablation definition drift');
const names=['intact','cut_output_to_feedback','cut_feedback_read_to_search',
  'intact_routing_blind_success','intact_routing_inverted_feedback'];
const kinds=['Intact','CutOutput','CutFeedbackRead','BlindSuccess','InvertedFeedback'];
const keys=[7,11,20,21,22,24,30,31];
const cells=m=>keys.map(k=>m.cell(k));
const serialize=op=>op.kind==='Halt'?'Halt':{
  [op.kind]:Object.fromEntries(Object.entries(op).filter(([name])=>name!=='kind'))
};
const coded=m=>m.code.map(serialize);
function ev(e){
  if(e.type==='executed')return ['exec'];
  if(e.type==='read')return ['read',e.key,e.value];
  if(e.type==='write')return ['write',e.key,e.value];
  if(e.type==='patch')return ['patch',e.node,e.before,e.after];
  if(e.type==='halt')return ['halt'];
  if(e.type==='trap')return ['trap',e.message];
  if(e.type==='suppressed')return ['suppressed',e.tag];
  throw Error('unsupported B56 F1a native event '+JSON.stringify(e));
}
function external(m,id,k,value,why){
  m.external(id,k,value,why);interventions++;
  take(['ext',id,m.tick,k,value,why,m.credits,m.storage(),cells(m)],
    'external '+id+' tick '+m.tick+' '+why);
}
const reports=[];
for(let id=0;id<5;id++){
  const name=names[id];
  const m=new CIL(1,-2,'TRUE');
  m.code=cs.map(op=>({...op}));
  if(id===1)m.code[2]={kind:'Write',address:2,value:0,next:30};
  if(id===2)m.code[36]={kind:'Read',dst:3,address:2,next:30};
  m.pc=3;m.credits=1300;m.maxCodeNodes=180;m.maxDataCells=16;m.maxStorage=300;
  m.peak=m.storage();
  take(['case',id,name,coded(m),1300,m.storage()],'founder '+name);
  external(m,id,7,5,'initial_training_input');
  let writes=0,reads=0,accepted=0,correct=0,wrong=0,otherPatches=0,input=5;
  while(m.paid<1300&&!m.stopped){
    const row=m.step(id);paid++;
    const e=row.event;
    take(['step',id,m.paid,m.tick,row.pc,m.pc,ev(e),[...m.regs],
      m.credits,m.stopped,m.code.length,m.edge(2),m.edge(33),
      m.edge(72),m.edge(73),m.storage(),cells(m)],
      'paid native step '+id+'/'+m.paid);
    if(e.type==='write'&&e.key===11){
      writes++;
      const good=e.value===input-2;
      if(good)correct++;else wrong++;
      const bit=id===3?1:id===4?Number(!good):Number(good);
      external(m,id,20,bit,'post_output_feedback');
      input=writes%2===0?5:-3;
      external(m,id,7,input,'next_training_or_service_input');
    }else if(e.type==='read'&&e.key===20){reads++;
    }else if(e.type==='patch'&&e.node===2&&e.before===42&&e.after===30){
      accepted++;patches++;
    }else if(e.type==='patch'){
      otherPatches++;patches++;
    }
  }
  const result={arm:name,paid:m.paid,writes,feedback_reads:reads,
    accepted,correct,wrong,other_patches:otherPatches,
    final_pc:m.pc,final_nodes:m.code.length,peak_storage_at_end:m.storage(),
    native_trace_sample_len:Math.min(16,m.paid)};
  take(['end',id,result,coded(m),cells(m)],'complete source and result '+id);
  reports.push(result);
}
if(position!==data.length)throw Error('B56 F1a unverified extra native rows '+(data.length-position));
console.log(JSON.stringify({
  study:'B56-F1a',source_sha:meta[3],
  genuine_native_worlds_replayed:reports.length,
  genuine_paid_native_instructions_replayed:paid,
  external_interventions_replayed:interventions,
  changed_native_executable_edges_replayed:patches,
  first_divergence:null,
  intact_source_feedback_routes:reports[0],
  feedback_route_cut_controls:reports.slice(1,3),
  source_identical_feedback_corruption:reports.slice(3),
  strict_b56_n56a_de_novo_monitor_origin:false,
  two_generated_service_policies_tested:false,
  b3_met:false,b4_met:false,cyberlife_established:false
},null,2));
