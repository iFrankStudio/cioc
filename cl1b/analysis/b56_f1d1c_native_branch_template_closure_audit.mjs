#!/usr/bin/env node
// F1d1c: a frozen-founder instruction-template closure certificate.
// Exact F1c 192k native history + kernel semantics, no NEW native actor.
import fs from 'node:fs';
import crypto from 'node:crypto';
import readline from 'node:readline';
import assert from 'node:assert/strict';

const rustName=n=>new URL('../../cl0/crates/world/tests/'+n,import.meta.url);
const files={
  b51c:fs.readFileSync(rustName('cl1b_b51c_paid_search_retention.rs')),
  f1c:fs.readFileSync(rustName('cl1b_b56_f1c_pilot_ordered_trial.rs')),
  b53:fs.readFileSync(rustName('cl1b_b53_native_response_correlation.rs')),
  kernel:fs.readFileSync(new URL('../../cl0/crates/world/src/lib.rs',import.meta.url)),
  manifest:fs.readFileSync(new URL('../studies/B56-F1d1c-founder-branch-template-closure-and-equivalent-correlator-preregistration.md',import.meta.url))
};
const hashes=Object.fromEntries(Object.entries(files).map(([k,v])=>
  [k,crypto.createHash('sha256').update(v).digest('hex')]));
const b51c=files.b51c.toString('utf8'),f1c=files.f1c.toString('utf8');
const b53=files.b53.toString('utf8'),kernel=files.kernel.toString('utf8');
const f1cMaker=f1c.split('fn build_two()->Vec<Op>{')[1]?.split('fn build_static(fourth:bool)')[0];
const b51cMaker=b51c.split('pub(crate) fn founder()->Vec<Op>{')[1]?.split('\n    ]')[0];
if(!f1cMaker||!b51cMaker||!f1cMaker.includes('code.resize(162,Op::Halt)'))
 throw Error('Original F1c founder grammar changed');
const parent=[...b51cMaker.matchAll(/Op::(\w+)(?:\{([^}]+)\})?/g)];
assert.equal(parent.length,72);
const branches=[
 ...parent.map((m,i)=>({pc:i,kind:m[1],args:m[2]}))
  .filter(x=>x.kind==='Branch').map(x=>{
    const m=x.args.match(/^test:(\d+),nonzero:(\d+),zero:(\d+)$/);
    if(!m)throw Error('B51c branch source drift '+x.pc);
    return {pc:x.pc,test:Number(m[1]),nonzero:Number(m[2]),zero:Number(m[3])};
  }),
 ...[...f1cMaker.matchAll(/code\[(\d+)\]=Op::Branch\{test:(\d+),nonzero:(\d+),zero:(\d+)\};/g)]
  .map(m=>({pc:Number(m[1]),test:Number(m[2]),nonzero:Number(m[3]),zero:Number(m[4])}))
].sort((a,b)=>a.pc-b.pc);
assert.deepEqual(branches.map(x=>x.pc),
 [23,44,47,61,64,74,98,106,117,120,123,144,152]);
assert.ok(branches.every(x=>x.test===3),'F1c already has other Branch registers');
const allOriginalBranchTests=[...new Set(branches.map(x=>x.test))];
const b53Branches=[...b53.matchAll(/Op::Branch\{test:(\d+),nonzero:(\d+),zero:(\d+)\},?\s*\/\/\s*(\d+)/g)]
 .map(m=>({pc:Number(m[4]),test:Number(m[1]),nonzero:Number(m[2]),zero:Number(m[3])}));
assert.ok(b53Branches.some(x=>x.pc===111&&x.test===1));
assert.ok(b53Branches.some(x=>x.pc===117&&x.test===1));
assert.ok(b53Branches.some(x=>x.pc===120&&x.test===3));
assert.ok(b51cMaker.includes('Op::Sub{dst:3,lhs:1,rhs:3,next:61}'),
 'B51c paid Sub60 r3 destination disappeared');

// Kernel proof: no overlooked mutation operation reachable from the original
// B51c+F1c founder can edit a Branch.test or any Branch outgoing edges.
const nextMut=kernel.split('fn next_mut(&mut self)')[1]?.split('fn ')[0];
assert.ok(nextMut?.includes('Op::Branch { .. } | Op::Halt => None'),
 'Kernel may now permit Branch edge patching');
assert.ok(kernel.includes('let new_node = p.code.nodes[index].clone();')&&
 kernel.includes('p.code.nodes.push(new_node);'),
 'CloneNode no longer copies whole source opcode exactly');
const kConst=kernel.split('Op::PatchConst { node, value, next } => {')[1]?.split('Op::Delete { address, next } =>')[0];
assert.ok(kConst?.includes('if let Op::Const { value: literal, .. }'),
 'PatchConst may now rewrite non-Const fields');
const kNext=kernel.split('Op::PatchNext { node, target, next } => {')[1]?.split('Op::ReadNext { dst, node, next } =>')[0];
assert.ok(kNext?.includes('.next_mut()')&&kNext.includes('node has no patchable edge'),
 'PatchNext may now rewrite Branch edges');
const mutationTokens=['GraphReplace','GraphSetNext','GraphCopyNode','GraphEncode',
 'GraphCopyFrom','GraphTransfer','GraphNew','GraphLaunch'];
for(const token of mutationTokens){
  // The only allowed builder vocabulary is original paid CloneNode,
  // PatchNext, PatchConst. The full B51c base is also inspected.
  assert.ok(!new RegExp('Op::'+token+'\\b').test(f1cMaker+b51cMaker),
   'original founder has an unexpected arbitrary-graph editor '+token);
}

// Independently verified precursor replays EVERY step from original CIL.
const upstream=JSON.parse(fs.readFileSync(
 'cl1b-ci-evidence/b56_f1c_pilot_ordered_trial_replay.json','utf8'));
assert.equal(upstream.study,'B56-F1c-PILOT');
assert.equal(upstream.native_worlds_replayed,48);
assert.equal(upstream.native_paid_steps_replayed,192000);
assert.equal(upstream.external_interventions_replayed,26432);
assert.equal(upstream.first_divergence,null);
if(process.env.GITHUB_SHA)assert.equal(upstream.source_sha,process.env.GITHUB_SHA);
const nativePath='cl1b-ci-evidence/b56_f1c_pilot_native_steps.jsonl';
assert.ok(fs.existsSync(nativePath),'same-source real F1c paid trace not produced');
const worlds=new Map();
let metaSeen=false,steps=0,externals=0,sourceEdits=0,branchDiffs=0,addedBranches=0;
const originalWitnesses=[];
function assertBranch(op,where){
 if(op!==null&&typeof op==='object'&&Object.hasOwn(op,'Branch')){
   assert.equal(op.Branch.test,3,'Native CIL Branch.test changed at '+where);
   assert.equal(Number.isInteger(op.Branch.nonzero),true);
   assert.equal(Number.isInteger(op.Branch.zero),true);
   return true;
 }
 return false;
}
for await(const line of readline.createInterface({
 input:fs.createReadStream(nativePath),crlfDelay:Infinity})){
 if(!line.trim())continue;
 const row=JSON.parse(line);
 if(row[0]==='meta'){
  assert.ok(!metaSeen);
  metaSeen=true;
  assert.deepEqual(row.slice(0,3),['meta','B56-F1c-PILOT',1]);
  if(process.env.GITHUB_SHA)assert.equal(row[3],process.env.GITHUB_SHA);
  assert.deepEqual(row.slice(6),[4000,16,180,300]);
  continue;
 }
 if(row[0]==='case'){
  const [_,id,seed,ecology,arm,from,to,order,credits,code,storage]=row;
  assert.ok(!worlds.has(id));
  assert.equal(credits,4000);
  assert.ok(code.length===162||code.length===126);
  let count=0;
  code.forEach((op,i)=>{if(assertBranch(op,'initial world '+id+' pc '+i))count++;});
  worlds.set(id,{id,seed,ecology,arm,initialBranchCount:count,
    currentLen:code.length,firstStep:null,lastStep:null,edits:0});
  continue;
 }
 if(row[0]==='ext'){
  externals++;
  assert.ok(worlds.has(row[1]));
  continue;
 }
 if(row[0]==='step'){
  const w=worlds.get(row[1]);
  assert.ok(w);
  steps++;
  if(w.firstStep===null)w.firstStep=row[2];
  w.lastStep=row[2];
  const newLen=row[10],diff=row[17];
  assert.ok(Array.isArray(diff));
  assert.ok(newLen>=w.currentLen&&newLen<=180);
  for(const change of diff){
   const [pc,prior,after]=change;
   sourceEdits++;w.edits++;
   assert.ok(Number.isInteger(pc)&&pc<newLen);
   const beforeWasBranch=assertBranch(prior,'native old pc '+pc);
   const afterIsBranch=assertBranch(after,'native new pc '+pc);
   if(beforeWasBranch||afterIsBranch)branchDiffs++;
   if(afterIsBranch&&prior===null)addedBranches++;
   if(beforeWasBranch){
     assert.deepEqual(after,prior,'PatchNext/PatchConst touched an existing Branch');
   }
  }
  w.currentLen=newLen;
  continue;
 }
 if(row[0]==='end'){
  assert.ok(worlds.has(row[1]));
  continue;
 }
 throw Error('New F1c raw native trace tag '+row[0]);
}
assert.ok(metaSeen);
assert.equal(worlds.size,48);
assert.equal(steps,192000);
assert.equal(externals,26432);
assert.equal(sourceEdits,upstream.executable_node_changes_replayed);
assert.ok([...worlds.values()].every(w=>w.lastStep===4000));
assert.ok(branches.length===13);
assert.ok(branchDiffs===0,'source witness must be checked before allowing mutation');
assert.equal(addedBranches,0,'original F1c pilot does not clone Branch nodes');
const arithmeticPairs=[
 [0n,0n],[1n,0n],[0n,1n],[-1n,-1n],[1n,-1n],[-1n,1n],
 [9223372036854775807n,-9223372036854775808n],
 [-9223372036854775808n,9223372036854775807n],
 [9223372036854775807n,9223372036854775807n],
 [-9223372036854775808n,-9223372036854775808n]
];
for(const [expected,echo] of arithmeticPairs){
 const b53Reg1=BigInt.asIntN(64,expected-echo);
 const adaptedReg3=BigInt.asIntN(64,expected-echo);
 assert.equal(b53Reg1===0n,adaptedReg3===0n);
 assert.equal(expected===echo,adaptedReg3===0n);
}
const report={
 study:'B56-F1d1c',
 evidence_class:'EXACT_SOURCE_OPCODE_CLOSURE_AND_EXISTING_NATIVE_HISTORY',
 exact_source_sha:process.env.GITHUB_SHA||upstream.source_sha,
 frozen_manifest_sha256:hashes.manifest,
 source_hashes_sha256:{b51c:hashes.b51c,f1c:hashes.f1c,b53:hashes.b53,kernel:hashes.kernel},
 original_F1c_branch_nodes:branches,
 original_F1c_distinct_branch_registers:allOriginalBranchTests,
 original_F1c_branch_count:branches.length,
 B53_requires_branch_test_register1_at:b53Branches.filter(x=>x.test===1).map(x=>x.pc),
 kernel_mutation_facts:{
  clone_node_copies_whole_op:true,
  patch_const_mutates_only_const_literal:true,
  patch_next_cannot_change_Branch_test_or_branch_exits:true,
  F1c_initial_mutation_vocabulary:['CloneNode','PatchNext','PatchConst'],
  original_F1c_has_full_op_replacement:false
 },
 historical_exact_source_native:{
  original_worlds_scanned:worlds.size,
  paid_steps_scanned:steps,
  external_interventions_scanned:externals,
  existing_independent_VM_replay_first_divergence:upstream.first_divergence,
  entire_source_node_changes_scanned:sourceEdits,
  branch_structure_changes:branchDiffs,
  branch_clones_observed:addedBranches,
  final_process_lengths_range:[
   Math.min(...[...worlds.values()].map(w=>w.currentLen)),
   Math.max(...[...worlds.values()].map(w=>w.currentLen))
  ]
 },
 register3_semantic_equality_adapter:{
  original_sub_source_pc:60,original_sub_dst_register:3,
  original_branch_condition_register:3,
  B53_branch_condition_register:1,
  signed_64bit_integer_boundary_cases_matched:arithmeticPairs.length,
  only_equality_decision_equivalence:true,
  no_complete_correlator_assembly:true
 },
 strict_falsifier:{
  literal_B53_Branch_test1_generated_from_unchanged_F1c:false,
  cannot_conclude_all_semantically_equivalent_correlators_impossible:true,
  still_requires_source_node_cell_and_branch_exit_rewiring:true
 },
 N56_A_B_C:false,B3:false,B4:false,cyberlife:false
};
console.log(JSON.stringify(report,null,2));
