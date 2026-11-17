// B52 information-limit control: multiple probes versus stale success.
// Abstract analysis only; never native CIL evidence.
import assert from 'node:assert/strict';
const tasks=[
 {name:'A',old:x=>x-2,next:x=>-x+3},
 {name:'B',old:x=>-x-3,next:x=>x+2}
];
const probes=[5,-3,11,-7,0];
const cases=[];
for(const t of tasks){
 const records=probes.map(x=>{
  const y=t.old(x);
  const freshOld=Number(y===t.old(x));
  const actualNew=Number(y===t.next(x));
  const staleSuccess=1;
  assert.equal(freshOld,1);
  assert.equal(staleSuccess,freshOld);
  return {x,y,observedBit:staleSuccess,actualNew};
 });
 const falseSuccesses=records.filter(r=>r.actualNew===0).length;
 assert.ok(falseSuccesses>=2);
 for(let n=1;n<=records.length;n++)
  assert.deepEqual(records.slice(0,n).map(r=>r.observedBit),Array(n).fill(1));
 cases.push({transition:t.name,probes:records,falseSuccesses});
}
assert.equal(cases.length,2);
console.log(JSON.stringify({
 study:'B52',tier:'abstract-finite-probe-indistinguishability',nativeVerified:false,
 cases,
 conclusion:'Two or five distinct probes cannot certify feedback freshness when the external channel may replay success=1. Provenance or an explicit freshness assumption is necessary.',
 caveat:'Not native CIL evidence. Paid provenance sensing or independent environmental signals may break this alias.'
},null,2));
