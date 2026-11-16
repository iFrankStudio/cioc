// B30: preregistered eight-world adjacent-packet threshold falsification.
// Reuses the independently source-parsed B29 CIL interpreter, rather than
// making a second potentially divergent resource-accounting VM.
import { simulateB29 } from "./b29_mutation_rule_siblings_scan.mjs";
const heldout=[
 {third:155,flipBirths:2,noFlipBirths:2,thirdParent:null,remaining:155},
 {third:156,flipBirths:3,noFlipBirths:2,thirdParent:3,remaining:0},
 {third:157,flipBirths:3,noFlipBirths:3,thirdParent:3,remaining:1},
 {third:158,flipBirths:3,noFlipBirths:3,thirdParent:2,remaining:0}
];
const cases=[];
for(const {third,flipBirths,noFlipBirths,thirdParent,remaining} of heldout){
 const schedule=[157,157,third];
 const flip=simulateB29("FLIP",schedule);
 const noFlip=simulateB29("NO_FLIP",schedule);
 if(flip.births.length!==flipBirths||noFlip.births.length!==noFlipBirths)
  throw Error(`B30 predeclared count contradicted at packet ${third}: ${flip.births.length}/${noFlip.births.length}`);
 if(JSON.stringify(flip.births.slice(0,2))!==JSON.stringify([[1,2],[1,3]])||
    JSON.stringify(noFlip.births.slice(0,2))!==JSON.stringify([[1,2],[1,3]]))
  throw Error(`B30 source-parent identities differ before heldout packet ${third}`);
 if(JSON.stringify(flip.windows.slice(0,2).map(w=>w.paid))!==JSON.stringify([157,157]) ||
    JSON.stringify(noFlip.windows.slice(0,2).map(w=>w.paid))!==JSON.stringify([157,157]))
  throw Error("B30 first two supply windows were not resource matched");
 if((flip.births[2]?.[0]??null)!==thirdParent)
  throw Error(`B30 native scheduler changed actual third parent for packet ${third}`);
 if(flip.pool!==remaining||noFlip.pool!==(third<=156?third:0))
  throw Error(`B30 neutral-pool inventory prediction changed for ${third}`);
 if(flip.lineageGenotypes[1][1]!==157||flip.lineageGenotypes[2][1]!==155 ||
    noFlip.lineageGenotypes[1][1]!==157||noFlip.lineageGenotypes[2][1]!==157)
  throw Error("B30 sibling candidate genotype labels no longer match copied native source");
 if(noFlip.edits.length!==0)
  throw Error("B30 no-flip program unexpectedly modified its own executable delta");
 cases.push({third,flip:{births:flip.births,sourceEdits:flip.edits,
    finalGenotypes:flip.lineageGenotypes,paidThird:flip.windows[2].paid,pool:flip.pool},
   noFlip:{births:noFlip.births,sourceEdits:noFlip.edits,
    finalGenotypes:noFlip.lineageGenotypes,paidThird:noFlip.windows[2].paid,pool:noFlip.pool},
   birthCountAdvantage:flip.births.length-noFlip.births.length,
   bothConserved:flip.energy+flip.attempts===2+schedule.reduce((a,b)=>a+b,0)&&
     noFlip.energy+noFlip.attempts===2+schedule.reduce((a,b)=>a+b,0)});
}
if(cases.length!==4||cases.some(c=>!c.bothConserved))
 throw Error("B30 eight native-equal-energy comparison worlds not conserved");
process.stdout.write(JSON.stringify({
 status:"independent JS observer; NOT native Rust CI and NOT endogenous fitness",
 source:"cl0/crates/world/tests/cl1b_b29_mutation_rule_siblings.rs",
 experimentalWorlds:cases.length*2,
 claim:"advantage exists only at one frozen narrow packet threshold among four",
 cases
},null,2)+"\n");
