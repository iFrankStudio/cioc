#!/usr/bin/env node
// Retrospective narrow CIL cost model, NOT an independent native replay.
import fs from "node:fs";
import assert from "node:assert/strict";
const manifest = JSON.parse(fs.readFileSync(new URL("../studies/CL1B-C27-PAID-CREDIT-REMEASURE-ROLLBACK-frozen-8-worlds-v1.json", import.meta.url), "utf8"));
function run(arm, policy) {
  const g = structuredClone(manifest.original_full_frozen_170_node_CIL_source);
  const v = [0,0,0,0], d = new Map([[50,arm.initial_mode]]);
  let pc = policy.startsWith("fixed") ? 100 : 0, credits = 3000, stages = 0, prod = 0, trials = 0;
  const probes = [], edges = [];
  if (policy === "fixedB") g[100].next = 131;
  if (policy !== "c27") g[162].zero = 100;
  while (credits > 0) {
    const n = g[pc], old = pc;
    assert(n, "invalid PC");
    credits--; pc = n.next;
    switch (n.kind) {
      case "SenseOwnCredits": v[n.dst] = credits; if(old===0) probes.push(3000-credits); break;
      case "Const": v[n.dst] = n.value; break;
      case "Read": v[n.dst] = d.get(v[n.address]) ?? 0; break;
      case "Write": {
        const a = v[n.address]; d.set(a,v[n.value]);
        if(a===11) { assert.equal(v[n.value],1); if(old===12||old===42) trials++; else prod++; }
        if(a===45) { stages=v[n.value]; if(stages===12 && arm.mode_after_stage12!==null) d.set(50,arm.mode_after_stage12); }
        break;
      }
      case "Sub": v[n.dst]=v[n.lhs]-v[n.rhs]; break;
      case "Add": v[n.dst]=v[n.lhs]+v[n.rhs]; break;
      case "Branch": pc=v[n.test]!==0?n.nonzero:n.zero; break;
      case "ReadNext": v[n.dst]=g[v[n.node]].next; break;
      case "PatchNext":
        if(!arm.patchnext_suppressed) {
          edges.push([old,3000-credits,g[v[n.node]].next,v[n.target]]);
          g[v[n.node]].next=v[n.target];
        }
        break;
      case "Halt":
        assert.equal(stages,24); assert.equal(prod,24);
        assert.equal(trials,policy==="c27"?4:policy==="oneShot"?2:0);
        return {cost:3000-credits,prod,trials,probes,edges};
      default: throw Error("unsupported opcode "+n.kind);
    }
  }
  throw Error("out of credits");
}
const expected = [625,673,627,673,626,626,624,720];
const expectedOne = [576,576,575,575,528,528,527,623];
const rows = manifest.arms.map((arm,i)=>{
  const c27=run(arm,"c27"), oneShot=run(arm,"oneShot");
  assert.equal(c27.cost,expected[i]); assert.equal(oneShot.cost,expectedOne[i]);
  const row={id:arm.id,c27,oneShot};
  if(!arm.patchnext_suppressed){
    row.fixedA=run(arm,"fixedA").cost; row.fixedB=run(arm,"fixedB").cost;
    const switched=arm.mode_after_stage12!==null && arm.mode_after_stage12!==arm.initial_mode;
    row.secondProbeOverhead=c27.cost-oneShot.cost+(switched?48:0);
    row.netExtraVsOneShot=c27.cost-oneShot.cost;
    row.netExtraVsBestFixed=c27.cost-Math.min(row.fixedA,row.fixedB);
  }
  return row;
});
console.log(JSON.stringify({tier:"RETROSPECTIVE_NON_NATIVE_COST_MODEL",rows},null,2));
