#!/usr/bin/env node
// B56-F1d1b: source + same-SHA native-history feasibility audit.
// Proves a DIRECT, VERBATIM B53 append to F1c is illegal. It does NOT prove
// that all actor-paid rewiring/compression schemes are impossible or viable.
// This is an analytic observer, not an additional native actor experiment.
import fs from 'node:fs';
import crypto from 'node:crypto';
import readline from 'node:readline';
import assert from 'node:assert/strict';

const file=n=>new URL('../../cl0/crates/world/tests/'+n,import.meta.url);
const f1c=fs.readFileSync(file('cl1b_b56_f1c_pilot_ordered_trial.rs'),'utf8');
const b53=fs.readFileSync(file('cl1b_b53_native_response_correlation.rs'),'utf8');
const b51c=fs.readFileSync(file('cl1b_b51c_paid_search_retention.rs'),'utf8');
const manifest=fs.readFileSync(new URL('../studies/B56-F1d1b-strict-founder-source-and-cell-liveness-capacity-preregistration.md',import.meta.url));
const sha256=s=>crypto.createHash('sha256').update(s).digest('hex');
const expectedSHA=process.env.GITHUB_SHA||'local';
const pilot=JSON.parse(fs.readFileSync('cl1b-ci-evidence/b56_f1c_pilot_ordered_trial_replay.json','utf8'));
if(expectedSHA!=='local')assert.equal(pilot.source_sha,expectedSHA,'same-source F1c independent replay');
assert.equal(pilot.first_divergence,null);
assert.equal(pilot.native_worlds_replayed,48);
assert.equal(pilot.by_arm.A_TRUTH_TWO.worlds,8);
assert.equal(pilot.by_arm.A_TRUTH_TWO.clones,48,'six real source clones per truthful F1c world');

const maker=f1c.split('fn build_two()->Vec<Op>{')[1]?.split('fn build_static(fourth:bool)')[0];
const base=b51c.split('pub(crate) fn founder()->Vec<Op>{')[1]?.split('\n    ]')[0];
if(!maker||!base||!maker.includes('code.resize(162,Op::Halt)')||
 !maker.includes('assert_eq!(code.len(),162)')||
 !f1c.includes('World::with_strict_capacity(16,180,300)'))
 throw Error('F1c actual native founder/capacity source grammar changed');
const founderOps=[...base.matchAll(/Op::(\w+)(?:\{([^}]+)\})?/g)].map((m,i)=>({pc:i,kind:m[1],args:m[2]||''}));
assert.equal(founderOps.length,72);
assert.equal(founderOps[35].kind,'Const');
assert.equal(founderOps[36].kind,'Read');
assert.equal(founderOps[71].kind,'Halt');
const f1graph=Array.from({length:162},(_,pc)=>pc<72?founderOps[pc]:{pc,kind:'Halt',args:''});
const f1assign=[...maker.matchAll(/code\[(\d+)\]=Op::(\w+)\{([^}]+)\};/g)];
assert.equal(f1assign.length,90,'original F1c source assignments');
for(const m of f1assign){
 const pc=Number(m[1]);
 assert.ok(pc<162,'F1c source assignment exceeds frozen source length');
 f1graph[pc]={pc,kind:m[2],args:m[3]};
}
assert.deepEqual(f1graph.filter(op=>op.kind==='Halt').map(op=>op.pc),[71,143]);
assert.equal(f1graph[110].kind,'Const','live F1c periodic/selector node');
assert.equal(f1graph[136].kind,'Const','live F1c FOURTH counter entry');
assert.equal(f1graph[2].kind,'Write');
assert.equal(f1graph[50].kind,'PatchNext');
const b53nodes=new Map();
for(const m of b53.matchAll(/Op::(\w+)\{([^}\n]+)\},?\s*\/\/\s*(\d+)/g)){
 const pc=Number(m[3]);b53nodes.set(pc,{pc,kind:m[1],args:m[2]});
}
assert.equal(b53nodes.size,32);
const nonce=Array.from({length:7},(_,i)=>99+i);
const checker=Array.from({length:18},(_,i)=>106+i);
for(const pc of [...nonce,...checker])assert.ok(b53nodes.has(pc));
const overlaps=[...nonce,...checker].filter(pc=>f1graph[pc].kind!==b53nodes.get(pc).kind)
 .map(pc=>({pc,f1c:f1graph[pc].kind,b53:b53nodes.get(pc).kind}));
assert.equal(overlaps.length,18,'verbatim B53 index collisions changed');
assert.ok(overlaps.some(x=>x.pc===110&&x.f1c==='Const'&&x.b53==='Sub'));
assert.ok(overlaps.some(x=>x.pc===111&&x.f1c==='Read'&&x.b53==='Branch'));
for(const marker of ['Op::PatchNext','Op::PatchConst','Op::CloneNode']){
 assert.ok(maker.includes(marker),'F1c mutation vocabulary changed');
}
assert.ok(!maker.includes('Op::Delete')&&!base.includes('Op::Delete'),
 'F1c original source unexpectedly has native paid cell deletion');
const originalClones=6;
const afterClones=162+originalClones;
const appendRoom=180-afterClones;
const b53Distinct=nonce.length+checker.length;
assert.deepEqual([afterClones,appendRoom,b53Distinct-afterClones],[168,12,-143]); // protects against off-by-one
assert.equal(b53Distinct-appendRoom,13,'verbatim B53 append node deficit');
const parsedKeys=f1c.match(/const KEYS:\[i64;16\]=\[([^\]]+)\]/)?.[1]
 .split(',').map(x=>Number(x.trim()));
assert.ok(parsedKeys&&parsedKeys.length===16);
assert.equal(new Set(parsedKeys).size,16);
assert.deepEqual(parsedKeys,[7,11,20,21,22,24,25,27,28,29,30,31,40,41,42,43]);
const borrowed=[20,26,27,28,29];
assert.ok(borrowed.every(k=>b53.includes('value:'+k)));
const union=[...new Set([...parsedKeys,...borrowed])].sort((a,b)=>a-b);
assert.equal(union.length,17,'registered-key union changed');
const conflicts={
  27:'F1c paid constructor route guard / B53 packet echo',
  28:'F1c paid FOURTH counter / B53 response commit',
  29:'F1c paid choice counter / B53 reject marker',
  40:'F1c monitor clone address / proposed relocated FOURTH counter',
  110:'F1c executable Const source node / B53 executable Sub source node'
};
for(const [pc,needle] of [[72,'value:27'],[136,'value:28'],[110,'value:29'],[77,'value:40']])
 assert.ok(f1graph[pc].args.includes(needle),'source conflict anchor drift '+pc);

// The native JSONL is independently replayed by the earlier F1c observer.
// Here independently scan SOURCE-PINNED live-cell and occupancy snapshots,
// not a guessed 16/16 memory fullness from the KEYS instrumentation array.
const native='cl1b-ci-evidence/b56_f1c_pilot_native_steps.jsonl';
if(!fs.existsSync(native))throw Error('F1d1b needs same-source native F1c CIL event history');
const worlds=new Map();
let scannedSteps=0,scannedExternal=0,rows=0,headerSeen=false;
for await(const line of readline.createInterface({input:fs.createReadStream(native),crlfDelay:Infinity})){
 if(!line.trim())continue;
 const row=JSON.parse(line);rows++;
 if(row[0]==='meta'){
   assert.ok(!headerSeen);headerSeen=true;
   assert.deepEqual(row.slice(0,3),['meta','B56-F1c-PILOT',1]);
   if(expectedSHA!=='local')assert.equal(row[3],expectedSHA);
   assert.deepEqual(row.slice(6),[4000,16,180,300]);continue;
 }
 if(row[0]==='case'){
   const id=row[1];assert.ok(!worlds.has(id));
   const isTwo=['A_TRUTH_TWO','B_BLIND_TWO','C_INVERTED_TWO','D_NO_ASSEMBLY'].includes(row[4]);
   const opLength=row[9].length;
   if(isTwo)assert.equal(opLength,162);
   worlds.set(id,{id,arm:row[4],two:isTwo,peakNamed:0,peakStorage:row[10],
       peakSource:opLength,finalSource:opLength,steps:0,externals:0,
       peakAfterRetentionNamed:0});
   continue;
 }
 if(row[0]==='step'||row[0]==='ext'){
   const w=worlds.get(row[1]);if(!w)throw Error('unmatched native event world '+row[1]);
   let data,storage,sourceLength;
   if(row[0]==='step'){
     scannedSteps++;w.steps++;
     sourceLength=row[10];storage=row[15];data=row[16];
   }else{
     scannedExternal++;w.externals++;
     sourceLength=w.finalSource;storage=row[7];data=row[8];
   }
   assert.ok(Array.isArray(data)&&data.length===16,'F1c observed named cell data shape drift');
   const named=data.filter(v=>v!==null).length;
   w.peakNamed=Math.max(w.peakNamed,named);
   w.peakStorage=Math.max(w.peakStorage,storage);
   w.peakSource=Math.max(w.peakSource,sourceLength);
   w.finalSource=sourceLength;
   assert.ok(sourceLength<=180&&storage<=300&&named<=16);
   if(w.two&&sourceLength>=168)w.peakAfterRetentionNamed=Math.max(w.peakAfterRetentionNamed,named);
   continue;
 }
 if(row[0]==='end'){const w=worlds.get(row[1]);assert.ok(w);continue;}
 throw Error('F1c native history row grammar drift at row '+rows+' tag '+row[0]);
}
assert.ok(headerSeen);assert.equal(worlds.size,48);
assert.equal(scannedSteps,192000);
assert.equal(scannedExternal,26432);
const chosen=[...worlds.values()].filter(w=>w.arm==='A_TRUTH_TWO');
assert.equal(chosen.length,8);
assert.ok(chosen.every(w=>w.peakSource===168),'observed actor-paid 6 source clones');
const peak=(field,arr)=>Math.max(...arr.map(x=>x[field]));
const min=(field,arr)=>Math.min(...arr.map(x=>x[field]));
const all=[...worlds.values()];
const report={
 study:'B56-F1d1b',classification:'same_source_native_history_plus_source_capacity_rejection',
 exact_source_sha:expectedSHA,
 preregistration_sha256:sha256(manifest),
 source_hashes_sha256:{
  f1c:sha256(f1c),b53:sha256(b53),b51c:sha256(b51c)
 },
 existing_f1c_initial_nodes:162,original_actor_paid_clones_per_truth_world:6,
 native_observed_max_source_nodes_truth:peak('peakSource',chosen),
 strict_code_node_cap:180,remaining_append_nodes_after_original_clones:appendRoom,
 exact_b53_nonce_generator_nodes:nonce.length,
 exact_b53_checker_nodes:checker.length,
 full_verbatim_b53_additional_nodes:b53Distinct,
 direct_append_nodes_required:afterClones+b53Distinct,
 direct_append_deficit_nodes:b53Distinct-appendRoom,
 exact_pc_opcode_kind_conflicts_count:overlaps.length,
 opcode_conflicts:overlaps,
 original_f1c_halt_pc_slots:[71,143],
 halt_slots_cannot_be_retyped_by_existing_F1c_PatchNext_PatchConst:true,
 key_inventory_instrumented_original_f1c:parsedKeys,
 b53_concurrent_key_requirements:borrowed,
 union_address_keys:union,union_count:union.length,
 explicit_key_and_pc_conflicts:conflicts,
 native_f1c_source_pinned_history:{worlds:worlds.size,paid_instructions:scannedSteps,
    external_interventions:scannedExternal,first_divergence_in_upstream_observer:pilot.first_divergence,
    truthful_8_worlds_peak_named_live_cells_range:
     [min('peakNamed',chosen),peak('peakNamed',chosen)],
    truthful_8_worlds_peak_named_cells_at_or_after_168_nodes:
     [min('peakAfterRetentionNamed',chosen),peak('peakAfterRetentionNamed',chosen)],
    truthful_8_worlds_peak_total_storage_range:
     [min('peakStorage',chosen),peak('peakStorage',chosen)],
    entire_48_worlds_peak_total_storage:peak('peakStorage',all)
 },
 conclusions:{
   direct_verbatim_append_feasible:false,
   pinned_b53_PCs_can_replace_F1c_PCs_by_paid_next_and_const_patches:false,
   17_union_keys_are_all_simultaneously_occupied:'NOT_ESTABLISHED',
   alternate_actor_paid_source_reuse_feasible:'NOT_ESTABLISHED',
   retained_B51c_training_feedback_present:true,
   complete_F1d1_actor_paid_experiment_done:false,
   strict_N56A_B_C:false,b3_met:false,b4_met:false,cyberlife:false
 }
};
console.log(JSON.stringify(report,null,2));
