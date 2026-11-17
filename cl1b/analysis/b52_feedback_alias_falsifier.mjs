// B52c abstract observational alias falsifier. NOT native CIL evidence.
import assert from 'node:assert/strict';
const inputs=[11,-7,0];
const oldTask=x=>x-2;
const shiftedTask=x=>-x+3;
const pairs=[];
for(const x of inputs){
  const y=oldTask(x);
  const oldCorrect=Number(y===oldTask(x));
  const newCorrect=Number(y===shiftedTask(x));
  const fresh={input:x,output:y,feedback:oldCorrect};
  const stale={input:x,output:y,feedback:1};
  assert.equal(oldCorrect,1);
  assert.equal(newCorrect,0);
  assert.deepEqual(fresh,stale);
  pairs.push({input:x,output:y,visible:fresh,freshTruth:oldCorrect,staleTruth:newCorrect});
}
assert.equal(pairs.length,3);
console.log(JSON.stringify({
  study:'B52',tier:'abstract-observation-impossibility',
  nativeVerified:false,identicalVisiblePairs:pairs.length,pairs,
  conclusion:'A single correctness bit with no actor-visible freshness provenance cannot distinguish a fresh success from a stale success in these matched histories. A paid Read alone does not establish freshness.',
  scope:'Only feedback-bit observation, conditioned on matched inputs, submissions and internal state. Additional task information or generic paid provenance sensing can break the alias. Not a native CIL result or H52c pass.'
},null,2));
