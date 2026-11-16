// B36: reuse the SAME B35 Rust-source-parsed CIL interpreter to compare
// two generic compute-energy laws. Source / supply / actor remain identical.
// This JS result is model-level until separately verified by native Rust CI.
import { simulateB35 } from "./b35_multitrial_retention_scan.mjs";
const schedules=[[4,5,5],[4,12,12],[5,5,5],[6,8,12]];
const cases=[];
for(const schedule of schedules){
 for(const arm of ["Adaptive","StaticThree"]){
  const restricted=simulateB35(arm,schedule,false);
  const direct=simulateB35(arm,schedule,true);
  if(restricted.poolPaidNonDraw!==0)
   throw Error("B36 draw-only default mistakenly funded non-Draw instructions");
  if(restricted.charged+restricted.local+restricted.pool!==14+schedule.reduce((a,b)=>a+b,0)||
     direct.charged+direct.local+direct.pool!==14+schedule.reduce((a,b)=>a+b,0))
   throw Error("B36 compute resource conservation violation");
  cases.push({schedule,arm,drawOnly:restricted,allNative:direct,
   outputDelta:direct.totalWrites-restricted.totalWrites});
 }
}
if(cases.length!==8)throw Error("B36 requires 8 actor/law pairs = 16 worlds");
const original=[[1,3,1,1],[3,7,4,7]];
for(let i=0;i<4;i++){
 if(cases[2*i].drawOnly.totalWrites!==original[0][i]||
    cases[2*i+1].drawOnly.totalWrites!==original[1][i])
  throw Error("B36 legacy draw-only native-source outcomes must remain unchanged");
}
if(!cases.some(x=>x.allNative.poolPaidNonDraw>0))
 throw Error("B36 alternative did not ever fund an ordinary non-Draw CIL opcode");
process.stdout.write(JSON.stringify({
 evidence:"independent Rust-source-parsed CIL model, NOT native Rust confirmation",
 source:"cl0/crates/world/tests/cl1b_b35_multitrial_retention.rs",
 programVariants:2,energyPhysicsVariants:2,resourceSchedules:4,worlds:16,
 cases
},null,2)+"\n");
