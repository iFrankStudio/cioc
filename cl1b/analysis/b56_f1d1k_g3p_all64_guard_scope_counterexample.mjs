#!/usr/bin/env node
// B56-F1d1k G3P GUARD-SCOPE ABSTRACT FALSIFIER, NOT AN ORIGINAL CIL RUN.
// This exhausts all 4^3 authored meta-opcode descriptors; only source-admitted
// forms were previously validated by original PID1-paid Rust/JS 120-world CI.
import assert from 'node:assert/strict';
const mode=code=>code.reduce((acc,op)=>op===3?0:(acc+op)%3,0);
function analyze(code,guard){
 let [base,middle,tail]=code,realModes=[],repairs=0,repairEvents=[],cycle=false;
 for(let actualTrial=0;actualTrial<3;actualTrial++){
  let proposed=mode([base,middle,tail]),attempts=0;
  if(guard&&actualTrial>0){
   while(proposed===base){
    attempts++;repairs++;
    if(attempts>4){cycle=true;break;}
    middle=(middle+1)%4;
    proposed=mode([base,middle,tail]);
    repairEvents.push({trial:actualTrial,middle_after_paid_style_edit:middle,mode:proposed});
   }
   if(cycle)break;
  }
  realModes.push(proposed);
  if(actualTrial<2)middle=(middle+1)%4;
 }
 return {realModes,repairs,repairEvents,cycle,
  all_three_modes:!cycle&&new Set(realModes).size===3};
}
const rows=[];
for(let first=0;first<4;first++)for(let second=0;second<4;second++)
 for(let third=0;third<4;third++){
  const descriptor=[first,second,third];
  rows.push({descriptor,guard_off:analyze(descriptor,false),
   guard_on:analyze(descriptor,true)});
 }
const qty=pred=>rows.filter(pred).length;
const result={
 study:'B56-F1d1k-G3P-ABSTRACT-64-SCOPE',
 status:'pure finite exhaustive model; no new original PID1 world tested here',
 syntactic_descriptors:rows.length,
 meta_primitive_alphabet:[0,1,2,3],
 per_word_description:['ADD0 mod3','ADD1 mod3','ADD2 mod3','RESET0'],
 real_candidate_budget:3,
 guard_checks:'if i>0 and mode==base-word, increment middle modulo4 and reevaluate before real probe',
 unguarded_full_cover:qty(r=>r.guard_off.all_three_modes),
 guarded_full_cover:qty(r=>r.guard_on.all_three_modes),
 previously_covering_broken_by_guard:qty(r=>r.guard_off.all_three_modes&&!r.guard_on.all_three_modes),
 previously_incomplete_fixed_by_guard:qty(r=>!r.guard_off.all_three_modes&&r.guard_on.all_three_modes),
 guarded_nonterminating_repair_cycle:qty(r=>r.guard_on.cycle),
 guarded_finite_but_incomplete:qty(r=>!r.guard_on.cycle&&!r.guard_on.all_three_modes),
 source_admitted_shapes:[[1,0,0],[0,3,0],[1,3,1],[2,3,2]],
 lost_coverage_example:[2,2,2],
 nontermination_example:[0,0,3],
 new_original_native_source_execution:false,
 generalized_coverage_safety_proven:false,
 rows
};
assert.equal(result.syntactic_descriptors,64);
assert.equal(result.unguarded_full_cover,24);
assert.equal(result.guarded_full_cover,16);
assert.equal(result.previously_covering_broken_by_guard,12);
assert.equal(result.previously_incomplete_fixed_by_guard,4);
assert.equal(result.guarded_nonterminating_repair_cycle,4);
assert.equal(result.guarded_finite_but_incomplete,44);
assert.equal(result.unguarded_full_cover-
 result.previously_covering_broken_by_guard+
 result.previously_incomplete_fixed_by_guard,
 result.guarded_full_cover);
const select=x=>rows.find(r=>r.descriptor.join(',')===x.join(','));
for(const x of result.source_admitted_shapes)
 assert.equal(select(x).guard_on.all_three_modes,true);
assert.deepEqual(select([2,2,2]).guard_off.realModes,[0,2,1]);
assert.deepEqual(select([2,2,2]).guard_on.realModes,[0,1,0]);
assert.equal(select([0,0,3]).guard_on.cycle,true);
assert.deepEqual(select([2,3,2]).guard_off.realModes,[2,1,2]);
assert.deepEqual(select([2,3,2]).guard_on.realModes,[2,1,0]);
assert.deepEqual(select([0,3,0]).guard_on.realModes,[0,1,2]);
console.log(JSON.stringify(result,null,2));
