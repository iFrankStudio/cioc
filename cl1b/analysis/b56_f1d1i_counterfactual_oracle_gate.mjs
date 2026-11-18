// B56-F1d1i prospective falsifier. Model-level gate, NOT native CIL evidence.
import assert from 'node:assert/strict';
const tasks={IDENTITY:x=>x,NEGATE:x=>-x,COMPOSED:x=>-x+2};
const candidates={A:x=>x,B:x=>-x,C:x=>-x+2};
const probes=[-7,5],heldout=11;
const outcomes=[];
for(const [taskName,oracle] of Object.entries(tasks)){
  const vectors=Object.entries(candidates).map(([candidate,fn])=>({candidate,outputs:probes.map(fn),heldout:fn(heldout),correct:probes.every(x=>fn(x)===oracle(x))}));
  assert.equal(vectors.filter(v=>v.correct).length,1);
  assert.equal(new Set(vectors.map(v=>JSON.stringify(v.outputs))).size,3);
  outcomes.push({task:taskName,vectors});
}
// Only the post-service feedback bit changes in this matched counterfactual.
const proposal=(feedback,old)=>feedback===1?old:(old==='A'?'B':old==='B'?'C':'A');
for(const old of Object.keys(candidates))
  assert.notEqual(proposal(1,old),proposal(0,old),'feedback must change next mutation');
// Reject a fixed-order controller as an explanation for feedback-conditioned mutation.
const fixed=old=>old==='A'?'B':old==='B'?'C':'A';
assert.equal(fixed('A'),proposal(0,'A'));
assert.notEqual(fixed('A'),proposal(1,'A'));
// Without an independent provenance signal, missing feedback aliases genuine zero.
assert.equal(proposal(0,'A'),proposal(Number(undefined??0),'A'));
const shifts=[['IDENTITY','COMPOSED'],['COMPOSED','NEGATE']];
for(const [before,after] of shifts){
  assert.notEqual(tasks[before](heldout),tasks[after](heldout));
  assert.ok(probes.some(x=>tasks[before](x)!==tasks[after](x)));
}
console.log(JSON.stringify({study:'B56-F1d1i',classification:'PROSPECTIVE_COUNTERFACTUAL_ORACLE_NOT_NATIVE',
 tasks:outcomes,shifts,matchedFeedbackCounterfactuals:Object.keys(candidates).map(old=>({old,nextOnSuccess:proposal(1,old),nextOnFailure:proposal(0,old)})),
 rejectionCriteria:['Native next mutation identical under matched feedback 0/1','Externally selected candidate','Native source/PID change','Uncharged edit','Capacity overflow','Missing required probes/heldout','Unjustified authenticated absence claim'],
 openObligations:['Freeze exact native CIL graph and live-cell layout','Prove charged PID1 next-slot edits and rollback under both shifts','Independently replay all native steps and interventions']},null,2));
