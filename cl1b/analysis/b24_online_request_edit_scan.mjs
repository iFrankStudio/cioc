// CL-1B B24 independent CIL subset VM: CIL-originated policy literal edits,
// bounded resource pulses, no hidden host top-ups. NOT native Rust.
import { readFileSync } from "node:fs";
const source=readFileSync(new URL("../../cl0/crates/world/tests/cl1b_b24_online_request_edit.rs",import.meta.url),"utf8");
const body=source.match(/fn founder\(initial: i64\) -> Vec<Op> \{[\s\S]*?vec!\[([\s\S]*?)\n    \]/)?.[1];
if(!body) throw Error("B24 founder body absent");
const ops=[...body.matchAll(/Op::([A-Za-z]+)\s*\{([^}]*)\}/g)].map(m=>{
 const z={op:m[1]};for(const k of m[2].matchAll(/(\w+)\s*:\s*(-?\d+|initial)/g))
  z[k[1]]=k[2]==="initial"?"initial":Number(k[2]);
 return z;
});
if(ops.length!==8||ops[3].value!=="initial"||ops[7].op!=="PatchConst")throw Error("B24 source schema drift");
function simulate(arm,schedule) {
 const enabled=arm==="ENABLED", initial=arm==="STATIC3"?3:4;
 const code=ops.map(q=>({...q,value:q.value==="initial"?initial:q.value}));
 const r=[0,0,0,0],cells=new Map();let pc=0,local=29,pool=0,attempts=0,output=0,attemptedPatches=0,suppressed=0;
 const policyChanges=[],trace=[];
 function step() {
  const op=code[pc],amount=op.op==="DrawCompute"?r[op.amount]:0;
  const poolPaid=op.op==="DrawCompute"&&local===0&&pool>=amount+1;
  if(local===0&&!poolPaid)return false;
  const before=pool+local,at=pc;
  if(poolPaid)pool--;else local--;
  attempts++;
  switch(op.op){
   case"Const":r[op.dst]=op.value;pc=op.next;break;
   case"DrawCompute":if(pool>=amount){pool-=amount;local+=amount;pc=op.next;}break;
   case"Write":{const k=r[op.address],v=r[op.value];cells.set(k,v);if(k===9&&v===1&&at===5)output++;pc=op.next;break;}
   case"Sub":r[op.dst]=r[op.lhs]-r[op.rhs];pc=op.next;break;
   case"PatchConst":{attemptedPatches++;const node=r[op.node],value=r[op.value];if(code[node].op!=="Const")throw Error("invalid patch");
    if(enabled){const beforeValue=code[node].value;code[node].value=value;if(beforeValue!==value)policyChanges.push([beforeValue,value]);}
    else suppressed++;
    pc=op.next;break;
   }
   default:throw Error("unknown source opcode "+op.op);
  }
  if(before-pool-local!==1)throw Error("credit conservation");
  trace.push({attempt:attempts,pc:at,kind:op.op,local,pool,nextPc:pc,request:code[3].value});
  return true;
 }
 function run(){let n=0;while(step()){if(++n>200)throw Error("unbounded");}return n;}
 const startup=run();if(startup!==29||pc!==4||local!==0||output!==0)throw Error("initial native budget drift");
 const windows=[];
 for(const packet of schedule){const before=pool+local;pool+=packet;if(pool+local-before!==packet)throw Error("external packet");windows.push({packet,attempts:run(),outputs:output,pc,local,pool,sourceLiteral:code[3].value});}
 const total=29+schedule.reduce((s,v)=>s+v,0)-attempts;
 if(total!==pool+local)throw Error("total budget drift");
 return {arm,schedule,output,policyChanges,attemptedPatches,suppressed,pc,local,pool,attempts,sourceLiteral:code[3].value,windows};
}
const schedules=[[5,4,4],[5,5,5],[4,4,4],[3,3,3,3]];
const cases=schedules.flatMap(schedule=>["ENABLED","DISABLED","STATIC3"].map(arm=>simulate(arm,schedule)));
if(cases.length!==12)throw Error("wrong B24 denominator");
for(const trial of cases){
 if(trial.arm==="ENABLED"&&(!trial.policyChanges.length||trial.policyChanges[0][0]!==4||trial.policyChanges[0][1]!==3||trial.pc!==3))
  throw Error("B24 online policy edit or debt invariant changed");
 if(trial.arm==="DISABLED"&&(trial.sourceLiteral!==4||trial.suppressed!==trial.attemptedPatches))
  throw Error("B24 suppressed native edit failed");
 if(trial.arm==="STATIC3"&&(trial.sourceLiteral!==3||trial.pc!==3))
  throw Error("B24 static low request cost invariant changed");
}
process.stdout.write(JSON.stringify({
 evidence:"independent source-parsed limited CIL VM; NOT native Rust",
 worlds:cases.length,source:"cl0/crates/world/tests/cl1b_b24_online_request_edit.rs",
 cases,
},null,2)+"\n");
