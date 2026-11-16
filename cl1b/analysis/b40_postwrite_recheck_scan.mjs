// B40: reuse native B38 Rust-source-parsed CIL and the independent B39
// interpreter. New RECHECK is a 3-instruction explicit program extension
// taken from the frozen B40 Rust founder (not a new VM/fitness primitive).
import {readFileSync} from "node:fs";
import {simulateB39} from "./b39_temporal_task_evidence_scan.mjs";
const source=readFileSync(new URL("../../cl0/crates/world/tests/cl1b_b40_postwrite_recheck.rs",import.meta.url),"utf8");
const required=[
 "Op::Const{dst:2,value:7,next:13}",
 "Op::Read{dst:0,address:2,next:14}",
 "Op::Const{dst:2,value:11,next:6}",
 "*next=12",
];
for(const term of required)if(!source.includes(term))
 throw Error("B40 native source's post-write recheck CIL extension changed: "+term);
const cases=[];
for(const [old,newValue] of [[0,17],[17,0]])
 for(const early of [true,false]){
  const oldFeedback=simulateB39(early,old,newValue,true,false);
  const recheck=simulateB39(early,old,newValue,true,true);
  const compact=simulateB39(early,old,newValue,false,false);
  cases.push(oldFeedback,recheck,compact);
  if(oldFeedback.checkpoint.firstVerdictCorrect!==early)
   throw Error("B40 historical B39 stale verdict regression");
  if(recheck.checkpoint.firstVerdictCorrect!==true)
   throw Error("B40 paid second input read did not repair stale verdict");
  if(JSON.stringify(recheck.readsAtFirstCheck)!==
     JSON.stringify(early?[newValue,newValue]:[old,newValue]))
   throw Error("B40 source actual first and second input reads are not distinct");
  if(recheck.edits!==Number(newValue!==0)||recheck.finalEdge!==(newValue!==0?11:4))
   throw Error("B40 refreshed true functional feedback did not condition source retention");
  if(compact.correct<recheck.correct)
   throw Error("B40 simpler static baseline is not retained");
  if(recheck.charged!==96||compact.charged!==96||oldFeedback.charged!==96)
   throw Error("B40 native paid opportunity not equal");
 }
if(cases.length!==12)throw Error("B40 wrong 12-world study denominator");
process.stdout.write(JSON.stringify({
 evidence:"independent Rust-source-parsed CIL with explicit B40 Rust source check; not native CI",
 worlds:cases.length,source:"cl0/crates/world/tests/cl1b_b40_postwrite_recheck.rs",cases
},null,2)+"\n");
