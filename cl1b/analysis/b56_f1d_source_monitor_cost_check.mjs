#!/usr/bin/env node
// B56-F1d prospective source-extracted cost falsifier.
// Static path verification only; NOT a native world execution or replay.
import fs from 'node:fs';
import assert from 'node:assert/strict';

const file = new URL('../../cl0/crates/world/tests/cl1b_b56_f1c_pilot_ordered_trial.rs', import.meta.url);
const src = fs.readFileSync(file, 'utf8');
const section = src.split('fn build_two()->Vec<Op>{')[1]?.split('fn build_static(')[0];
assert.ok(section, 'Missing original F1c builder');
const ops = new Map();
for (const m of section.matchAll(/code\[(\d+)\]=Op::(\w+)\{([^}]+)\};/g)) {
  const attrs = Object.fromEntries([...m[3].matchAll(/(\w+):(-?\d+)/g)].map(a => [a[1], Number(a[2])]));
  ops.set(Number(m[1]), { op:m[2], ...attrs });
}
for (const n of [98,99,100,101,110,136,137,138,139,140,141,142,144,145,146,147,148]) {
  assert.ok(ops.has(n), `Missing source node ${n}`);
}
assert.match(section, /code\[94\]=Op::Const\{dst:1,value:2,next:95\}/);
assert.match(section, /code\[95\]=Op::PatchNext\{node:1,target:3,next:102\}/);
// Native CloneNode copies founder nodes35 (Const20) and36 (Read20),
// and paid PatchNext connects both cloned Reads to the shared branch98.
const founder = fs.readFileSync(new URL('../../cl0/crates/world/tests/cl1b_b51c_paid_search_retention.rs', import.meta.url), 'utf8');
assert.match(founder, /Op::Const\s*\{\s*dst:\s*2,\s*value:\s*20/);
assert.match(founder, /Op::Read\s*\{\s*dst:\s*3,\s*address:\s*2/);
ops.set(164, {op:'Const',dst:2,value:20,next:165});
ops.set(165, {op:'Read',dst:3,address:2,next:98});
ops.set(166, {op:'Const',dst:2,value:20,next:167});
ops.set(167, {op:'Read',dst:3,address:2,next:98});
// Paid PatchNext at node95 changes Const148.next from30 to cloned node166.
ops.set(148, {...ops.get(148), next:166});
function path(start, sampled, success) {
  const visited = [];
  let pc=start;
  for(let i=0;i<40;i++){
    if(pc===110) return visited;
    const x=ops.get(pc);
    assert.ok(x, `Missing node ${pc}`);
    visited.push(pc);
    if(x.op==='Branch') {
      if(pc===144) pc=sampled ? x.zero : x.nonzero;
      else if(pc===98) pc=success ? x.nonzero : x.zero;
      else throw new Error(`Unexpected branch ${pc}`);
    } else pc=x.next;
  }
  throw new Error('Unbounded path');
}
const cases={
  every_success:path(164,true,true),
  every_failure:path(164,true,false),
  fourth_skip:path(136,false,true),
  fourth_sample_success:path(136,true,true),
  fourth_sample_failure:path(136,true,false)
};
const costs=Object.fromEntries(Object.entries(cases).map(([k,v])=>[k,v.length]));
assert.deepEqual(costs,{every_success:3,every_failure:6,fourth_skip:8,fourth_sample_success:15,fourth_sample_failure:18});
const fourSuccessful={every:4*costs.every_success,fourth:3*costs.fourth_skip+costs.fourth_sample_success};
assert.deepEqual(fourSuccessful,{every:12,fourth:39});
const worstEvery=4*costs.every_failure;
const minimumFourth=fourSuccessful.fourth;
assert.ok(worstEvery<minimumFourth,'Even four EVERY failures must cost less than four FOURTH successes');
const report={status:'static_source_path_check_only',native_experiment:false,paths:cases,costs,four_successful:fourSuccessful,worst_every_four_failures:worstEvery,minimum_fourth_four_successes:minimumFourth,dominance:'EVERY is cheaper for all feedback patterns over four services; this does not account for later shared control or differential benefit'};
console.log(JSON.stringify(report,null,2));
