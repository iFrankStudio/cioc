#!/usr/bin/env node
// Exact frozen C0 source-structure capacity preflight, not a native C1 test.
import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
const url=new URL('../studies/B56-F1d1k-G3Q-C0-frozen-exact-164-node-original-CIL-founder-v1.json',import.meta.url);
const f=JSON.parse(readFileSync(url,'utf8'));
const op=f.exact_native_founder_code,lim=f.strict_world;
assert.equal(op.length,164);
assert.equal(lim.max_source_nodes,180);
assert.equal(lim.allowed_cells,16);
assert.equal(lim.max_storage_units,300);
assert.deepEqual(f.allowed_cells,[7,11,20,21,32,33,50,51,52,53,56,61,62,63,64,65]);
const edge=(node)=>node.kind==='Branch'?[node.zero,node.nonzero]:node.kind==='Halt'?[]:[node.next];
for(let i=0;i<op.length;i++){
  const node=op[i];
  for(const to of edge(node))assert.ok(Number.isInteger(to)&&to>=0&&to<op.length,`bad edge ${i}->${to}`);
  for(const key of node.kind==='Branch'?['test']:['dst','lhs','rhs','address','value']){
    if(key in node&&key!=='value')assert.ok(Number.isInteger(node[key]),`bad register operand ${i}.${key}`);
  }
}
const visited=new Set([0]),pending=[0];
while(pending.length){for(const t of edge(op[pending.pop()]))if(!visited.has(t)){visited.add(t);pending.push(t);}}
assert.equal(visited.size,164);
assert.equal(op[163].kind,'Halt');
const exitIncoming=op.flatMap((n,i)=>edge(n).includes(163)?[i]:[]);
assert.deepEqual(exitIncoming,[154]);
assert.equal(op.filter(n=>n.kind==='Branch').length,15);
assert.equal(f.allowed_cells.length,16);
assert.equal(f.native_expected.max_observed_occupied_units,181);
// These counts constrain *append-only* attempts; do not prove no smaller refactor exists.
const freeNodes=lim.max_source_nodes-op.length;
const copyThreeWordsBasic=12; // (Const address,Read,Const destination,Write) x 3
assert.equal(freeNodes,16);
assert.equal(freeNodes-copyThreeWordsBasic,4);
const report={
 study:'B56-F1d1k-G3Q-C1',
 level:'FROZEN_C0_STATIC_SOURCE_PREFLIGHT_NOT_NATIVE_C1',
 originalSourceNodes:op.length,
 reachableNodes:visited.size,
 branchNodes:15,
 exitIncoming,
 existingAllowedDataAddresses:f.allowed_cells,
 remainingSourceNodes:freeNodes,
 freeDistinctDataAddresses:lim.allowed_cells-f.allowed_cells.length,
 observedOriginalPeakStorage:f.native_expected.max_observed_occupied_units,
 upperBoundPeakIfAll16NodesAppendedWithoutNewData:197,
 trivialThreeWordBankToActiveCopyLowerBoundNodes:copyThreeWordsBasic,
 remainingNodesAfterTrivialCopy:freeNodes-copyThreeWordsBasic,
 conclusion:'Append-only C0 plus triple bank-to-active copy leaves four source nodes; genuine charged task evaluation, feedback handling, regime shift and causal controls require a prospective source refactor or a documented feasibility failure.',
 caveats:'The 12-node count is for one naive register-safe load/store pattern, not an ISA-wide mathematical minimum. Static CFG reachability is not evidence of runtime coverage; no native Rust C1, PID1 replay, or CI was executed.'
};
console.log(JSON.stringify(report,null,2));
