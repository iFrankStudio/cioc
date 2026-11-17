// B51c independent static feasibility falsifier; NOT native CIL replay.
// B51b's two authored candidate functions are x+2 and 2-x.
import assert from 'node:assert/strict';
const xs=[0,5,-3];
const modes=[-1,1].flatMap(s=>[-3,-2,2,3].map(k=>({s,k})));
const candidates=[x=>x+2,x=>2-x];
const rows=modes.map(({s,k})=>{
  const target=x=>s*x+k;
  const bits=xs.map(x=>candidates.map(f=>Number(f(x)===target(x))));
  const exact=candidates.map(f=>xs.every(x=>f(x)===target(x)));
  return {s,k,bits,exact};
});
assert.equal(rows.length,8);
assert.equal(rows.filter(r=>r.exact.some(Boolean)).length,2);
assert.deepEqual(rows.find(r=>r.s===1&&r.k===2).bits[0],[1,1]);
assert.deepEqual(rows.find(r=>r.s===-1&&r.k===2).bits[0],[1,1]);
for(const row of rows) if(row.exact.some(Boolean)) assert.equal(row.exact.filter(Boolean).length,1);
console.log(JSON.stringify({study:'B51c',kind:'static feasibility only',native:false,
  probes:xs,fully_covered:2,uncovered:6,rows},null,2));
