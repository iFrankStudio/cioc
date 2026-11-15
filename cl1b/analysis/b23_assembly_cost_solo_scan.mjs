// CL-1B B23: source-parsed 18-world deterministic energy comparison.
// The B22 assembled-organization arm uses the SAME independent restricted
// graph VM as the B22 audit; solo policy uses the actual Rust B23 five-node
// founder parsed from source. NO native Rust or spontaneous adaptive claim.
import { readFileSync } from "node:fs";
import { simulateB22 } from "./b22_native_relay_assembly.mjs";

const source=readFileSync(new URL("../../cl0/crates/world/tests/cl1b_b23_assembly_cost_vs_solo.rs",import.meta.url),"utf8");
const body=source.match(/fn solo\(request: i64\) -> Vec<Op> \{[\s\S]*?vec!\[([\s\S]*?)\n    \]/)?.[1];
if(!body)throw Error("B23 Rust solo source changed");
const base=[...body.matchAll(/Op::([A-Za-z]+)\s*\{([^}]*)\}/g)].map(m=>{
  const op={kind:m[1]};
  for(const f of m[2].matchAll(/(\w+)\s*:\s*(-?\d+)/g))op[f[1]]=Number(f[2]);
  return op;
});
if(base.length!==5||base[0].kind!=="Const"||base[1].kind!=="DrawCompute"
   ||base[4].kind!=="Write")throw Error("B23 Rust program source invariants changed");

function simulateSolo(request,schedule){
 const code=base.map(o=>({...o}));code[0].value=request;
 const r=[0,0,0,0];let pc=0,credits=29,pool=0,attempts=0,writes=0;
 const reg=k=>{if(!Number.isInteger(k)||k<0||k>=4)throw Error("bad register");return r[k];};
 const next=k=>{if(!Number.isInteger(k)||k<0||k>=5)throw Error("bad edge");return k;};
 function step(){
  const o=code[pc];
  const amount=o.kind==="DrawCompute"?reg(o.amount):0;
  const poolPaid=credits===0&&o.kind==="DrawCompute"&&pool>=amount+1;
  if(credits<=0&&!poolPaid)return false;
  const total=pool+credits;
  if(poolPaid)pool--;else credits--;
  attempts++;
  switch(o.kind){
   case"Const":r[o.dst]=o.value;pc=next(o.next);break;
   case"DrawCompute":
     if(pool>=amount){pool-=amount;credits+=amount;pc=next(o.next);}
     // Shortage is charged but no CIL branch observes an exception.
     break;
   case"Write":if(reg(o.address)!==9||reg(o.value)!==1)throw Error("invalid service");
     writes++;pc=next(o.next);break;
   default:throw Error("unexpected B23 native instruction");
  }
  if(credits+pool!==total-1)throw Error("resource conservation failure");
  return true;
 }
 function run(){
  let n=0;
  while(step()){n++;if(n>100)throw Error("bounded energy must not fund unbounded work");}
  return n;
 }
 const bootstrap=run();if(bootstrap!==29||writes!==0||credits!==0)
   throw Error("B23 equal-energy bootstrap not matched");
 const afterPacketAttempts=[];
 for(const packet of schedule){
  const before=credits+pool;pool+=packet;
  if(credits+pool!==before+packet)throw Error("external energy mismatch");
  afterPacketAttempts.push(run());
 }
 return {type:"SOLO",request,schedule,codeNodes:5,bootstrap,afterPacketAttempts,
         serviceWrites:writes,remainingPool:pool,remainingLocalCredits:credits,
         totalNativeAttempts:attempts,remainingPC:pc};
}

const schedules=[[3,5],[4,4]];
const comparisons=[];
for(const schedule of schedules){
 const built=simulateB22("TRANSFER",schedule);
 const noTransfer=simulateB22("NO_TRANSFER",schedule);
 const solo=Array.from({length:7},(_,i)=>simulateSolo(i+1,schedule));
 const solo3=solo.find(x=>x.request===3);
 if(built.bootstrap!==29||noTransfer.bootstrap!==29||
    built.writes!==1||noTransfer.writes!==0||solo3.serviceWrites!==2||
    built.steps+0!==37||solo3.totalNativeAttempts!==37)
   throw Error("B23 independent efficiency counterexample not reproduced");
 if(solo.some(x=>x.bootstrap!==29))throw Error("B23 unequal initial cost");
 comparisons.push({schedule,
    assembled:{initialNodes:35,attempts:built.steps,serviceWrites:built.writes,
      postSupplyAttempts:built.windows.map(w=>w.steps),residualPool:built.pool},
    noTransfer:{initialNodes:35,attempts:noTransfer.steps,
      serviceWrites:noTransfer.writes,residualPool:noTransfer.pool},
    solo});
}
if(comparisons.reduce((s,x)=>s+2+x.solo.length,0)!==18)
 throw Error("expected eighteen frozen B23 worlds");
process.stdout.write(JSON.stringify({
  evidence:"source-parsed independent CIL resource model, NOT native Rust",
  sources:[
    "cl0/crates/world/tests/cl1b_b22_native_relay_assembly.rs",
    "cl0/crates/world/tests/cl1b_b23_assembly_cost_vs_solo.rs",
  ],
  design:"2 neutral pulse schedules × (2 assembled source variants + 7 solo fixed policies)",
  comparedWorlds:18,comparisons,
},null,2)+"\n");
