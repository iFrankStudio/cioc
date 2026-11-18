#!/usr/bin/env node
// B56-F1d1 source-derived correlated-receipt cost gate.
// Static feasibility analysis, NOT a native execution or organism result.
import fs from 'node:fs';
import assert from 'node:assert/strict';
const source=fs.readFileSync(new URL('../../cl0/crates/world/tests/cl1b_b53_native_response_correlation.rs',import.meta.url),'utf8');
const ops=new Map();
for(const match of source.matchAll(/Op::(\w+)\{([^}\n]+)\},?\s*\/\/\s*(\d+)/g)){
  const attrs=Object.fromEntries([...match[2].matchAll(/(\w+):(-?\d+)/g)].map(m=>[m[1],Number(m[2])]));
  ops.set(Number(match[3]),{op:match[1],...attrs});
}
assert.equal(ops.size,32,'B53 source op extraction drift');
function walk(start,choice,terminal){
  let pc=start,steps=[],readBit=false;
  while(!terminal.includes(pc)){
    assert.ok(steps.length<30,'cycle in sensor');
    const op=ops.get(pc);assert.ok(op,'missing node '+pc);
    steps.push(pc);
    if(op.op==='Read' && pc===119)readBit=true;
    if(op.op==='Branch'){
      if(pc===111)pc=choice==='echo_mismatch'?op.nonzero:op.zero;
      else if(pc===117)pc=choice==='commit_mismatch'?op.nonzero:op.zero;
      else if(pc===120)pc=op.nonzero;
      else throw Error('unexpected branch '+pc);
    }else pc=op.next;
  }
  return {steps:steps.length,pcs:steps,readBit};
}
const fresh=walk(106,'fresh',[30,80]);
const echo=walk(106,'echo_mismatch',[30,80]);
const torn=walk(106,'commit_mismatch',[30,80]);
const nonce=walk(99,'fresh',[2]);
assert.deepEqual([fresh.steps,echo.steps,torn.steps,nonce.steps],[15,9,15,7]);
assert.equal(fresh.readBit,true);
assert.equal(echo.readBit,false);
assert.equal(torn.readBit,false);
const base={every:12,fourth:39};
const perFour={
  allFresh:{every:base.every+4*fresh.steps-4*3,fourth:base.fourth+fresh.steps-3},
  allEchoMismatch:{every:base.every+4*echo.steps-4*3,fourth:base.fourth+echo.steps-3},
  allCommitMismatch:{every:base.every+4*torn.steps-4*3,fourth:base.fourth+torn.steps-3}
};
// Assumes replacing the existing 3-op raw read+branch with the complete sensor.
// The 7-op nonce generator is shared, 4*7=28, and not hidden.
assert.deepEqual(perFour,{allFresh:{every:60,fourth:51},allEchoMismatch:{every:36,fourth:45},allCommitMismatch:{every:60,fourth:51}});
console.log(JSON.stringify({status:'static_source_gate_only',native:false,source:'B53 nodes99-123',nonce,fresh,echo,torn,perFour,sharedNonceFourOutputs:28,limits:'Not a full CIL integration: F1c cell28 is also a counter; success and rejection branches have different control continuations; no policy selection or native result.'},null,2));
