// B56-F1d1h independent prospective oracle; NOT native execution.
// Run: node cl1b/analysis/b56_f1d1h_frozen_oracle_preflight.mjs
import assert from 'node:assert/strict';
const probes=[-7,5],held=11;
const candidates=[x=>x,x=>-x,x=>-x+2];
const tasks=['IDENTITY','NEGATE','COMPOSED'];
const regimes=['TRUTH','BLIND_SUCCESS','INVERTED','ABSENT'];
const expected={TRUTH:[0,1,2],BLIND_SUCCESS:[0,0,0],INVERTED:[1,0,0],ABSENT:[0,0,0]};
const rows=[];
for(let task=0;task<3;task++) for(const regime of regimes){
  const scores=[0,0,0],trials=[];
  for(let candidate=0;candidate<3;candidate++) for(const x of probes){
    const output=candidates[candidate](x);
    const truth=Number(output===candidates[task](x));
    const bit=regime==='TRUTH'?truth:regime==='BLIND_SUCCESS'?1:regime==='INVERTED'?1-truth:0;
    scores[candidate]+=bit;
    trials.push({candidate,x,output,truth,delivered:regime==='ABSENT'?null:bit,readDefault:bit});
  }
  assert.equal(trials.length,6);
  assert.equal(new Set(trials.map(t=>t.candidate+':'+t.x)).size,6);
  const winner=scores.indexOf(Math.max(...scores));
  assert.equal(winner,expected[regime][task]);
  const correct=candidates[winner](held)===candidates[task](held);
  assert.equal(correct,regime==='TRUTH'||(task===0&&(regime==='BLIND_SUCCESS'||regime==='ABSENT')));
  if(regime==='TRUTH') assert.deepEqual(scores,scores.map((_,i)=>i===task?2:0));
  if(regime==='BLIND_SUCCESS') assert.deepEqual(scores,[2,2,2]);
  if(regime==='ABSENT') assert.deepEqual(scores,[0,0,0]);
  rows.push({task:tasks[task],regime,scores,winner,heldoutCorrect:correct});
}
assert.equal(rows.length,12);
const frozen=tasks.map((task,i)=>({task,heldoutCorrect:candidates[0](held)===candidates[i](held)}));
assert.deepEqual(frozen.map(x=>x.heldoutCorrect),[true,false,false]);
console.log(JSON.stringify({study:'B56-F1d1h',classification:'PROSPECTIVE_ORACLE_ONLY_NOT_NATIVE',worlds:12,probeEvaluations:72,heldoutEvaluations:12,rows,frozen},null,2));
