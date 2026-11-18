// B56-F1d1h: prospective, independently executable cell-allocation feasibility gate.
// This is NOT a Rust/CIL native execution or a proof that a <=180-node founder exists.
// Run: node cl1b/analysis/b56_f1d1h_cell_allocation_gate.mjs
import assert from 'node:assert/strict';

const cap=16;
const roles={
  input:7, serviceOutput:11, feedback:20, editEnable:21, stage:31,
  virtualPC:50, op0:51, op1:52, op2:53, op3:54,
  literal:55, accumulatorCheckpoint:56,
  scoreA:60, scoreB:61, scoreC:62, bestCandidate:63,
};
const entries=Object.entries(roles);
assert.equal(new Set(entries.map(([,k])=>k)).size,entries.length,'aliasing must be declared explicitly');
assert.equal(entries.length,cap,'prospective full-key allocation must fit exactly 16');
const scoreKeys=[roles.scoreA,roles.scoreB,roles.scoreC];
const phases=[
  {name:'initialize',live:entries.map(([,k])=>k),scoreWrites:[60,61,62],bestWrite:63},
  ...[0,1,2].flatMap((candidate)=>[-7,5].map((input)=>({
    name:`candidate_${candidate}_probe_${input}`,live:entries.map(([,k])=>k),
    input,candidate,scoreWrite:scoreKeys[candidate],
  }))),
  {name:'select_and_restore',live:entries.map(([,k])=>k),bestRead:63,programEdits:[52,53]},
  {name:'heldout',live:entries.map(([,k])=>k),input:11},
];
for(const p of phases){
  assert.equal(new Set(p.live).size,p.live.length,`duplicate live key in ${p.name}`);
  assert.ok(p.live.length<=cap,`capacity violation in ${p.name}`);
  for(const k of [...(p.scoreWrites??[]),...(p.programEdits??[]),...(p.scoreWrite===undefined?[]:[p.scoreWrite])])
    assert.ok(p.live.includes(k),`undeclared key ${k} in ${p.name}`);
}
assert.deepEqual(phases.filter(p=>p.candidate!==undefined).map(p=>[p.candidate,p.input]),
  [[0,-7],[0,5],[1,-7],[1,5],[2,-7],[2,5]]);
const forbiddenExtra=Object.values(roles).concat(64);
assert.equal(new Set(forbiddenExtra).size,17);
assert.ok(forbiddenExtra.length>cap,'adding persistent candidate-index cell64 violates cap');
const result={
  study:'B56-F1d1h',classification:'PROSPECTIVE_STATIC_ALLOCATION_ONLY',
  keyRoles:roles,simultaneousKeys:entries.length,capacity:cap,
  phases:phases.map(p=>({name:p.name,peakKeys:p.live.length})),
  rejectedAlternative:{persistentCandidateIndexKey:64,simultaneousKeys:17,reason:'exceeds 16-cell limit'},
  openObligations:[
    'Native source graph <=180 nodes, initial credits <=4000 and storage <=300',
    'Candidate index must be in native registers/control flow; no hidden persistent cell64',
    'CIL must actually initialize scores and best, execute all six probes, and charge writes',
    'Live occupancy and data-cell existence must be independently replayed from native events',
    'Edit-suppressed arm must charge shadow writes without clobbering live score/program cells',
    'This maximal 16-key allocation leaves no spare cell for temporary shadow writes',
    'ABSENT feedback must not be silently treated as authenticated failure',
  ],
};
console.log(JSON.stringify(result,null,2));
