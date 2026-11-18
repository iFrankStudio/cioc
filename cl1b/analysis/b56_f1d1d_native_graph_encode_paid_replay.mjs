#!/usr/bin/env node
// B56-F1d1d: independently reconstruct the actual native 3-world paid source,
// dynamic graph descriptors, two process states, credit and storage accounting.
// This is a narrow GENERIC ENCODER feasibility replay, NOT autonomous invention.
import fs from 'node:fs';
import crypto from 'node:crypto';
import assert from 'node:assert/strict';
import {isDeepStrictEqual} from 'node:util';

const src=fs.readFileSync(new URL(
 '../../cl0/crates/world/tests/cl1b_b56_f1d1d_paid_graph_encode_preflight.rs',import.meta.url),'utf8');
const kernel=fs.readFileSync(new URL('../../cl0/crates/world/src/lib.rs',import.meta.url),'utf8');
const manifest=fs.readFileSync(new URL(
 '../studies/B56-F1d1d-paid-graph-encode-novel-branch-structural-feasibility-manifest-v1.md',import.meta.url));
const hash=b=>crypto.createHash('sha256').update(b).digest('hex');
const contract=src.split('const DESCRIPTORS:[[i64;5];7]=[')[1]?.split('];')[0];
if(!contract||!src.includes('let mut c=vec![Op::GraphNew{dst:0,next:1}];')||
 !src.includes('c[17]=Op::Halt')||
 !src.includes('desc[0][2]=arm.bit()')||
 !kernel.includes('let candidate = Op::decode_instruction(words)?;')||
 !/self\.processes\s*\.insert\(child_id, Process::new\(child_id, object.nodes, requested\)\)/.test(kernel))
 throw Error('F1d1d source/kernel graph encoder or identity semantics changed');
const rows=[...contract.matchAll(/\[\s*(-?\d+)\s*,\s*(-?\d+)\s*,\s*(-?\d+)\s*,\s*(-?\d+)\s*,\s*(-?\d+)\s*\]/g)]
 .map(m=>m.slice(1).map(Number));
assert.equal(rows.length,7);
assert.deepEqual(rows[1],[5,1,2,3,0],'exact Branch(test1) data descriptor drift');
const line='cl1b-ci-evidence/b56_f1d1d_native_graph_encoded_branch.jsonl';
if(!fs.existsSync(line))throw Error('F1d1d original same-SHA native trace is missing');
const all=fs.readFileSync(line,'utf8').trimEnd().split(/\r?\n/);
let pos=0,nativeSteps=0,chargedGraphEncodes=0,paidDescriptorWrites=0;
function check(expected,where){
 if(pos>=all.length)throw Error('F1d1d truncated '+where);
 const got=JSON.parse(all[pos++]);
 if(!isDeepStrictEqual(expected,got))
  throw Error('F1d1d FIRST NATIVE DIVERGENCE '+where+' line '+pos+
   '\nexpected '+JSON.stringify(expected)+'\nactual '+JSON.stringify(got));
}
const meta=JSON.parse(all[pos++]);
assert.deepEqual(meta.slice(0,3),['meta','B56-F1d1d',1]);
assert.equal(meta[4],hash(manifest));
assert.equal(meta[5],manifest.length);
assert.deepEqual(meta.slice(6),[3,1000,16,180,300]);
if(process.env.GITHUB_SHA)assert.equal(meta[3],process.env.GITHUB_SHA);
function op(kind,attrs={}){
 return kind==='Halt'?'Halt':{[kind]:attrs};
}
function graphOp(w){
 const [tag,a,b,c,d]=w;
 switch(tag){
  case 0:return op('Const',{dst:a,value:b,next:c});
  case 4:return op('Write',{address:a,value:b,next:c});
  case 5:return op('Branch',{test:a,nonzero:b,zero:c});
  case 19:return 'Halt';
  default:throw Error('F1d1d native encoder descriptor added a new opcode tag '+tag);
 }
}
function parentSource(arm,ds){
 const p=[op('GraphNew',{dst:0,next:1})];
 for(const words of ds){
  for(let i=0;i<words.length;i++){
   const pc=p.length;
   p.push(op('Const',{dst:2,value:40+i,next:pc+1}));
   p.push(op('Const',{dst:3,value:words[i],next:pc+2}));
   p.push(op('Write',{address:2,value:3,next:pc+3}));
  }
  const pc=p.length;
  p.push(op('Const',{dst:1,value:40,next:pc+1}));
  p.push(op('GraphEncode',{graph:0,address:1,dst:2,next:pc+2}));
 }
 const pc=p.length;
 p.push(op('Const',{dst:1,value:100,next:pc+1}));
 p.push(op('GraphLaunch',{graph:0,credits:1,dst:2,next:pc+2}));
 p.push('Halt');
 assert.equal(p.length,123);
 if(arm==='NO_ENCODE')p[17]='Halt';
 assert.ok(p.every(o=>!o.Branch),'parent already contained a Branch');
 return p;
}
const CKEYS=[11,40,41,42,43,44];
function snap(p){
 return p?[p.pc,[...p.regs],p.credits,p.stopped,p.code.length]:null;
}
function occupied(state){
 let z=state.cells.size+state.parent.code.length+1;
 if(state.child)z+=state.child.code.length+1;
 if(state.graph)z+=1+state.graph.nodes.length;
 return z;
}
const outcomes=[];
for(const arm of ['BRANCH_ZERO','BRANCH_ONE','NO_ENCODE']){
 const id=outcomes.length,bit=arm==='BRANCH_ONE'?1:0;
 const ds=rows.map(x=>[...x]);ds[0][2]=bit;
 const initial=parentSource(arm,ds);
 const state={
  parent:{pc:0,regs:[0,0,0,0],credits:1000,stopped:false,code:initial},
  child:null,graph:null,cells:new Map(),paid:0,tick:null,
  encodes:0,writes:0,launches:0,branchExecutions:0,serviceWrites:0,
  graphCreations:0,peak:124
 };
 assert.equal(occupied(state),124);
 check(['case',id,arm,1000,16,180,300,initial,ds,occupied(state),1000],
  'initial source '+arm);
 let n=0;
 while(pos<all.length && JSON.parse(all[pos])[0]==='step'){
  const actual=JSON.parse(all[pos]),pid=actual[4],w=pid===1?state.parent:state.child;
  assert.ok(w&&!w.stopped,'cannot pay instruction from missing or stopped PID');
  assert.ok(w.credits>0,'not enough native CIL credits');
  const pc=w.pc,raw=w.code[pc];
  assert.ok(raw!==undefined,'missing executable PC '+pc);
  const kind=raw==='Halt'?'Halt':Object.keys(raw)[0];
  const o=raw==='Halt'?{}:raw[kind];
  let event='Executed';
  w.credits--;
  if(kind==='Const'){
   w.regs[o.dst]=o.value;w.pc=o.next;
  }else if(kind==='Write'){
   const key=w.regs[o.address],value=w.regs[o.value];
   state.cells.set(key,value);w.pc=o.next;
   event=op('CellWritten',{key,value});
   if(key>=40&&key<=44){assert.equal(pid,1);state.writes++;}
   if(key===11){assert.equal(pid,2);state.serviceWrites++;}
  }else if(kind==='GraphNew'){
   assert.equal(pid,1);assert.equal(state.graph,null);
   state.graph={owner:1,nodes:[]};
   w.regs[o.dst]=1;w.pc=o.next;
   event=op('GraphCreated',{graph:1,owner:1});state.graphCreations++;
  }else if(kind==='GraphEncode'){
   assert.equal(pid,1);assert.ok(state.graph);
   const base=w.regs[o.address];
   const desc=Array.from({length:5},(_,i)=>{
    const k=base+i;assert.ok(state.cells.has(k),'incomplete descriptor');
    return state.cells.get(k);
   });
   const nextNode=graphOp(desc);
   const index=state.graph.nodes.length;
   state.graph.nodes.push(nextNode);
   w.regs[o.dst]=index;w.pc=o.next;
   event=op('GraphEncoded',{
    graph:1,owner:1,new_node:index,descriptor_base:base,descriptor:desc
   });
   state.encodes++;chargedGraphEncodes++;
  }else if(kind==='GraphLaunch'){
   assert.equal(pid,1);assert.ok(state.graph);
   assert.equal(state.child,null);
   const transfer=w.regs[o.credits],nodes=state.graph.nodes;
   assert.equal(transfer,100);assert.equal(nodes.length,7);
   w.credits-=transfer;w.regs[o.dst]=2;w.pc=o.next;
   state.child={pc:0,regs:[0,0,0,0],credits:transfer,
    stopped:false,code:nodes.map(x=>x)};
   state.graph=null;
   event=op('GraphLaunched',{
    source_graph:1,parent_process:1,child_process:2,
    transferred_credits:transfer,moved_nodes:nodes.length
   });
   assert.deepEqual(state.child.code[1],op('Branch',{test:1,nonzero:2,zero:3}));
   state.launches++;
  }else if(kind==='Branch'){
   assert.equal(pid,2,'parent must not have Branch');
   assert.equal(o.test,1,'must execute generated Branch on r1');
   w.pc=w.regs[o.test]!==0?o.nonzero:o.zero;
   state.branchExecutions++;
  }else if(kind==='Halt'){
   w.stopped=true;
   event='Halted';
  }else{
   throw Error('unsupported opcode in frozen paid-native replay '+kind);
  }
  state.paid++;n++;nativeSteps++;
  if(pid===1&&kind==='Write'&&state.cells.has(40))paidDescriptorWrites++;
  state.peak=Math.max(state.peak,occupied(state));
  assert.ok(state.peak<=300);
  const total=state.parent.credits+(state.child?.credits||0);
  assert.equal(total,1000-n,'no graph credit creation');
  const gotTick=actual[3];
  if(state.tick!==null)assert.equal(gotTick,state.tick+1,'native event tick sequence');
  state.tick=gotTick;
  check(['step',id,n,gotTick,pid,pc,event,
   snap(state.parent),snap(state.child),total,occupied(state),
   CKEYS.map(k=>state.cells.has(k)?state.cells.get(k):null),
   state.graph?state.graph.nodes:null],
   'native actor-paid '+arm+' instruction '+n+' PC '+pc);
 }
 const result={
  id,arm,
  initial_parent_contains_Branch:false,
  graph_created_by_pid1:state.graphCreations,
  descriptor_cell_writes_paid_by_pid1:state.writes,
  graph_encoded_nodes_paid_by_pid1:state.encodes,
  graph_launches_by_pid1:state.launches,
  novel_Branch_test1_executed_by_pid2:state.branchExecutions,
  actual_pid2_service_writes:state.serviceWrites,
  service_output11:state.cells.has(11)?state.cells.get(11):null,
  paid_steps:n,
  parent_paid:1000-state.parent.credits-(state.child?100:0),
  child_paid:state.child?100-state.child.credits:0,
  peak_total_storage:state.peak,
  final_total_credits:state.parent.credits+(state.child?.credits||0),
  PID1:snap(state.parent),PID2:snap(state.child),
  same_pid_continuous_individual:false,
  b3_met:false,b4_met:false,cyberlife:false
 };
 // Native controls are charged CIL; don't invent same-PID continuity.
 check(['end',id,result],'final source-pinned world '+arm);
 if(arm==='NO_ENCODE'){
  assert.equal(state.encodes,0);assert.equal(state.launches,0);
  assert.equal(state.serviceWrites,0);assert.equal(state.child,null);
 }else{
  assert.equal(state.encodes,7);assert.equal(state.launches,1);
  assert.equal(state.branchExecutions,1);assert.equal(state.serviceWrites,1);
  assert.equal(state.cells.get(11),bit);
 }
 outcomes.push(result);
}
assert.equal(pos,all.length,'unchecked original native events');
assert.equal(outcomes.length,3);
console.log(JSON.stringify({
 study:'B56-F1d1d',source_sha:meta[3],
 frozen_manifest_sha256:hash(manifest),
 rust_source_sha256:hash(Buffer.from(src)),
 real_native_worlds_reexecuted:outcomes.length,
 real_paid_CIL_instructions_reexecuted:nativeSteps,
 real_actor_paid_GraphEncode_events_reexecuted:chargedGraphEncodes,
 native_cil_descriptor_writes_reexecuted:outcomes.reduce((a,b)=>a+b.descriptor_cell_writes_paid_by_pid1,0),
 first_divergence:null,
 generated_Branch_test1_not_present_in_parent_source:true,
 registered_no_encode_negative_control:true,
 strict_finite_resources:true,
 actor_generated_semantic_intent:false,
 new_pid_instead_of_same_pid:true,
 strict_N56_A_B_C:false,B3:false,B4:false,cyberlife:false,
 by_case:outcomes
},null,2));
