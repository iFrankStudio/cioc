#!/usr/bin/env node
// B56-F1d1k-G3Q: finite falsifier of G3P's first-mode/base-word shortcut.
// Pure independent grammar enumeration; NOT Rust-native PID1 evidence.
import assert from 'node:assert/strict';
import fs from 'node:fs';
const oracle=JSON.parse(fs.readFileSync(new URL('../studies/B56-F1d1k-G2-frozen-64-descriptor-three-trial-meta-oracle-v1.json',import.meta.url),'utf8'));
assert.equal(oracle.total_syntactic_meta_programs,64);
assert.equal(oracle.mutable_instruction_slot_index,1);
const execute=(words,trial)=>{
 let acc=0;
 for(let i=0;i<3;i++){
  const op=i===1?(words[i]+trial)%4:words[i];
  assert.ok(op>=0&&op<=3);
  acc=op===3?0:(acc+op)%3;
 }
 return acc;
};
const rows=[];
for(let base=0;base<4;base++)for(let middle=0;middle<4;middle++)for(let tail=0;tail<4;tail++){
 const descriptor=[base,middle,tail];
 const modes=[0,1,2].map(trial=>execute(descriptor,trial));
 const falsePositive=[1,2].filter(t=>modes[t]===base&&modes[t]!==modes[0]);
 const falseNegative=[1,2].filter(t=>modes[t]===modes[0]&&modes[t]!==base);
 rows.push({descriptor,modes,falsePositive,falseNegative});
}
assert.equal(rows.length,64);
const classes=new Map();
for(const r of rows){const key=r.modes.join('');classes.set(key,(classes.get(key)||0)+1);}
assert.equal(classes.size,oracle.distinct_resulting_candidate_sequences);
for(const cls of oracle.equivalence_classes)
 assert.equal(classes.get(cls.three_trial_modes.join('')),cls.descriptor_count);
const mismatch=rows.filter(r=>r.modes[0]!==r.descriptor[0]).length;
const fp=rows.reduce((s,r)=>s+r.falsePositive.length,0);
const fn=rows.reduce((s,r)=>s+r.falseNegative.length,0);
assert.equal(mismatch,48);
assert.equal(fp,20);
assert.equal(fn,35);
assert.deepEqual(rows.find(r=>r.descriptor.join(',')==='0,0,1').modes,[1,2,0]);
assert.deepEqual(rows.find(r=>r.descriptor.join(',')==='0,3,1').modes,[1,1,2]);
for(const d of [[1,0,0],[0,3,0],[1,3,1],[2,3,2]]){
 const r=rows.find(r=>r.descriptor.join()===d.join());
 assert.equal(r.modes[0],d[0],'G3P admitted source shape violates shortcut');
}
console.log(JSON.stringify({study:'B56-F1d1k-G3Q',evidence:'abstract exhaustive grammar only',descriptors:64,sequenceClasses:classes.size,firstModeNotBase:mismatch,falsePositiveChecks:fp,falseNegativeChecks:fn,examples:{falsePositive:rows.find(r=>r.descriptor.join()==='0,0,1'),falseNegative:rows.find(r=>r.descriptor.join()==='0,3,1')}},null,2));
