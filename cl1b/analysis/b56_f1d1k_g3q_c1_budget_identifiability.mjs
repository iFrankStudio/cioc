#!/usr/bin/env node
import assert from 'node:assert/strict';
// G3Q-C1 identifiability falsifier; abstract finite oracle, NOT original native CIL.
const step=(acc,word)=>word===3?0:(acc+word)%3;
const sequence=words=>[0,1,2].map(t=>words.reduce((a,w,i)=>step(a,i===1?(w+t)%4:w),0));
const certified=[];
for(let a=0;a<4;a++)for(let b=0;b<4;b++)for(let c=0;c<4;c++){
  const words=[a,b,c],seq=sequence(words);
  if(new Set(seq).size===3)certified.push({words,seq});
}
assert.equal(certified.length,24);
const bank=sequence([1,3,2]);
const earnedAlternate=sequence([2,2,2]);
assert.deepEqual(bank,[2,0,1]);
assert.deepEqual(earnedAlternate,[0,2,1]);
const countForBudget={};
const firstModeHistogram=[0,0,0];
for(const r of certified)firstModeHistogram[r.seq[0]]++;
assert.deepEqual(firstModeHistogram,[8,8,8]);
for(let k=1;k<=3;k++){
  const perRule=certified.map(r=>[0,1,2].filter(t=>r.seq.slice(0,k).includes(t)).length);
  assert.ok(perRule.every(n=>n===k));
  countForBudget[k]={successesAcross72RuleTargetPairs:perRule.reduce((a,b)=>a+b,0),meanPerRule:k};
}
assert.deepEqual([1,2,3].map(k=>countForBudget[k].successesAcross72RuleTargetPairs),[24,48,72]);
const witnessTargets=[0,2].map(target=>({target,bankFirst:bank[0]===target,alternateFirst:earnedAlternate[0]===target}));
assert.deepEqual(witnessTargets,[{target:0,bankFirst:false,alternateFirst:true},{target:2,bankFirst:true,alternateFirst:false}]);
console.log(JSON.stringify({study:'B56-F1d1k-G3Q-C1-budget-identifiability',level:'ABSTRACT_ORACLE_ONLY',certifiedCount:24,firstModeHistogram,countForBudget,witnessTargets,conclusion:'Complete coverage alone does not establish task-conditioned adaptation. Under uniform targets all certified rules tie for k=1,2,3; target-conditioned ordering and actual charged computation must distinguish adaptive learning from a fixed safe bank.',limits:'No original PID, charged native CIL, task service, authenticated feedback, or GitHub CI evidence.'},null,2));
