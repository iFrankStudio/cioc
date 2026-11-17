// B52 source-graph preflight: static structural falsifier, NOT native evidence.
// Run: node cl1b/analysis/b52_retained_watchdog_topology.mjs
import fs from 'node:fs';
import assert from 'node:assert/strict';
const src=fs.readFileSync('cl0/crates/world/tests/cl1b_b51c_paid_search_retention.rs','utf8');
const founder=src.split('pub(crate) fn founder()->Vec<Op>{')[1]?.split('\n    ]')[0];
assert.ok(founder,'missing B51c founder');
const nodes=[...founder.matchAll(/Op::([A-Za-z]+)(?:\{([^}]*)\})?,\s*\/\/\s*(\d+)/g)]
  .map(m=>({op:m[1],fields:m[2]??'',id:Number(m[3])}));
assert.equal(nodes.length,72,'B51c graph changed: re-audit');
for(let i=0;i<nodes.length;i++)assert.equal(nodes[i].id,i);
const field=(id,name)=>{
  const m=nodes[id].fields.match(new RegExp('(?:^|,)\\s*'+name+':(-?\\d+)(?:,|$)'));
  return m?Number(m[1]):null;
};
const patches=nodes.filter(n=>n.op==='PatchNext'&&field(n.id,'node')===2)
  .map(n=>({pc:n.id,targetRegister:field(n.id,'target'),next:field(n.id,'next')}));
const feedbackReads=nodes.filter(n=>n.op==='Read'&&field(n.id,'address')===2)
  .map(n=>({pc:n.id,addressRegister:field(n.id,'address')}));
assert.equal(nodes[2].op,'Write');
assert.equal(field(2,'next'),35,'original output continuation changed');
assert.deepEqual(patches.map(x=>x.pc),[29,41,50,53],'candidate/retention patches changed');
assert.deepEqual(feedbackReads.map(x=>x.pc),[22,25,27,31,33,36,43,46,55,63],
  'read topology changed');
assert.match(src,/EventKind::ExecutableEdgePatched\{node:2,before:42,after:30\}/,
  'retention event contract changed');
const result={
  study:'B52',tier:'static-source-topology-only',nativeVerified:false,
  founderNodes:nodes.length,outputNode:2,seedOutputContinuation:35,
  retentionPatch:'node2 edge42->30',feedbackReadPC:36,
  necessaryCondition:'A B52 watchdog must reconnect the post-retention output route to a paid feedback Read, and a failed check must reopen the candidate search by actor-paid edits; source topology alone cannot prove either occurs.',
  patches,feedbackReads
};
console.log(JSON.stringify(result,null,2));
