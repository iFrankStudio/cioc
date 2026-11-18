#!/usr/bin/env node
// C1 finite grammar oracle: NOT native CIL or measured costs.
import assert from 'node:assert/strict';
const op=(a,w)=>w===3?0:(a+w)%3;
const modes=w=>[0,1,2].map(t=>w.reduce((a,v,i)=>op(a,i===1?(v+t)%4:v),0));
const rules=[];
for(let a=0;a<4;a++)for(let b=0;b<4;b++)for(let c=0;c<4;c++){
 const words=[a,b,c],seq=modes(words);
 rules.push({words,seq,safe:new Set(seq).size===3});
}
const bank=[1,3,2],bankSeq=modes(bank);
assert.deepEqual(bankSeq,[2,0,1]);
assert.equal(rules.filter(r=>r.safe).length,24);
const deploy=(r,arm)=>arm==='bypass'||arm==='monitor_only'?r.seq:
 arm==='reject_only'?(r.safe?r.seq:null):
 arm==='certified_fallback'?(r.safe?r.seq:bankSeq):
 arm==='privileged_static'?bankSeq:assert.fail('unknown arm');
const arms=['bypass','monitor_only','reject_only','certified_fallback','privileged_static'];
const successes=Object.fromEntries(arms.map(a=>[a,0]));
let fallbackRescues=0,rejectOnlyLosses=0;
for(const r of rules)for(let target=0;target<4;target++){
 const v=Object.fromEntries(arms.map(a=>[a,deploy(r,a)?.includes(target)??false]));
 for(const a of arms)successes[a]+=Number(v[a]);
 assert.equal(v.bypass,v.monitor_only);
 assert.ok(!v.bypass||v.certified_fallback);
 if(target===3)assert.ok(arms.every(a=>!v[a]));
 if(v.certified_fallback&&!v.bypass)fallbackRescues++;
 if(v.bypass&&!v.reject_only)rejectOnlyLosses++;
}
assert.deepEqual(successes,{bypass:136,monitor_only:136,reject_only:72,certified_fallback:192,privileged_static:192});
assert.equal(fallbackRescues,56);
assert.equal(rejectOnlyLosses,64);
const safe=rules.find(r=>r.words.join()==='2,2,2');
const unsafe=rules.find(r=>r.words.join()==='0,0,3');
assert.deepEqual(safe.seq,[0,2,1]);
assert.deepEqual(unsafe.seq,[0,0,0]);
assert.deepEqual(deploy(unsafe,'certified_fallback'),[2,0,1]);
console.log(JSON.stringify({
 study:'B56-F1d1k-G3Q-C1',level:'ABSTRACT_ORACLE_ONLY',
 rules:64,certified:24,rejected:40,targets:[0,1,2,3],bank,bankSeq,
 successesAcross256RuleTargetPairs:successes,fallbackRescues,rejectOnlyLosses,
 interpretation:'A certified safe-bank fallback can rescue 56 reachable cases, but a privileged static bank has identical structural reachability; certification provenance and paid deployment must be demonstrated by native trace.',
 limits:'No original PID, paid CIL, actual service, feedback, instruction costs, heldout learning, native trace or CI result is established here.'
},null,2));
