// B55a methodological negative control. NOT native CIL evidence.
// A policy selected on a trial ecology can lose after an unseen shift.
// Frozen deterministic toy counts are deliberately NOT empirical CL-1B measurements.
import assert from 'node:assert/strict';
const modes = ['stationary', 'shift'];
const policies = ['never', 'every', 'fourth'];
// [correct, wrong, missing, paid]; each period has eight mandatory outputs.
const table = {
  stationary: {never:[8,0,0,8],every:[8,0,0,24],fourth:[8,0,0,12]},
  shift: {never:[2,6,0,8],every:[7,1,0,24],fourth:[5,3,0,12]},
};
function utility([C,W,M,P]) {
  assert.equal(C+W+M,8);
  return (C-2*W-M)/Math.max(1,P);
}
function choose(mode) {
  return policies.reduce((best,p)=>
    utility(table[mode][p])>utility(table[mode][best])?p:best,policies[0]);
}
const outcomes=[];
for(const trial of modes) for(const heldout of modes) {
  const chosen=choose(trial);
  const oracle=choose(heldout);
  outcomes.push({trial,heldout,chosen,oracle,
    score:utility(table[heldout][chosen]),
    oracleScore:utility(table[heldout][oracle])});
}
assert.equal(choose('stationary'),'never');
assert.equal(choose('shift'),'every');
for(const [trial,heldout] of [['stationary','shift'],['shift','stationary']]) {
  const x=outcomes.find(x=>x.trial===trial&&x.heldout===heldout);
  assert(x.score<x.oracleScore,'heldout reversal required');
}
console.log(JSON.stringify({scope:'abstract protocol counterexample only',outcomes},null,2));
