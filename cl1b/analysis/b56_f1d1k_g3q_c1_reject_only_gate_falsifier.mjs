#!/usr/bin/env node
// G3Q-C1 prospective abstract deployment falsifier; NOT Rust-native PID1 evidence.
import assert from 'node:assert/strict';
const op=(acc,word)=>word===3?0:(acc+word)%3;
const modes=(words)=>[0,1,2].map(t=>words.reduce((acc,w,i)=>op(acc,i===1?(w+t)%4:w),0));
const rows=[];
for(let a=0;a<4;a++)for(let b=0;b<4;b++)for(let c=0;c<4;c++){
 const words=[a,b,c],seq=modes(words),safe=new Set(seq).size===3;
 rows.push({words,seq,safe});
}
assert.equal(rows.length,64);
assert.equal(rows.filter(r=>r.safe).length,24);
const safe=rows.find(r=>r.words.join()==='2,2,2');
const unsafe=rows.find(r=>r.words.join()==='0,0,3');
assert.deepEqual(safe.seq,[0,2,1]);
assert.deepEqual(unsafe.seq,[0,0,0]);
for(const r of rows.filter(x=>x.safe))for(let target=0;target<3;target++)assert.ok(r.seq.includes(target));
for(const r of rows.filter(x=>!x.safe))assert.ok([0,1,2].some(t=>!r.seq.includes(t)));
const compare=(rule,target,gate)=>{
 const deployed=gate?(rule.safe?rule.seq:null):rule.seq;
 return {deployed,success:deployed!==null&&deployed.includes(target)};
};
let discriminating=0,indistinguishable=0;
for(const r of rows)for(let target=0;target<4;target++){
 const on=compare(r,target,true),off=compare(r,target,false);
 if(on.success!==off.success)discriminating++;else indistinguishable++;
 if(target===3)assert.equal(on.success,false);
}
assert.equal(discriminating,rows.filter(r=>!r.safe).reduce((n,r)=>n+new Set(r.seq).size,0));
assert.ok(rows.every(r=>[0,1,2,3].every(t=>!(compare(r,t,true).success&&!compare(r,t,false).success))));
console.log(JSON.stringify({study:'B56-F1d1k-G3Q-C1',level:'ABSTRACT_ORACLE_ONLY',rules:64,certified:24,rejected:40,safe_example:safe,unsafe_example:unsafe,gate_vs_bypass:{discriminating,indistinguishable},unreachable_target_3_always_fails:true,conclusion:'Reject-only gate cannot improve binary task success without paid repair or a safe-bank fallback; matched native C1 must include that causal mechanism.'},null,2));
