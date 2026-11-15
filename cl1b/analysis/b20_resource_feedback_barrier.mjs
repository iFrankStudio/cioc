// CL-1B B20: restricted model of the actual B19 five-node resource actor.
// Source shape is read from B19 native test, never silently redesigned.
// This is an independent observer-side model, NOT native Rust execution.
import { readFileSync } from "node:fs";
const source = readFileSync(
  new URL("../../cl0/crates/world/tests/cl1b_b19_neutral_compute_flux.rs", import.meta.url),
  "utf8"
);
const body = source.match(/fn founder\(draw: bool\) -> Vec<Op> \{[\s\S]*?vec!\[([\s\S]*?)\n    \]/)?.[1];
if (!body) throw Error("B19 founder source missing");
const ops = [...body.matchAll(/Op::([A-Za-z]+)\s*\{([^}]*)\}/g)].map(m => {
  const o = {op:m[1]};
  for (const k of m[2].matchAll(/(\w+)\s*:\s*(-?\d+)/g)) o[k[1]] = Number(k[2]);
  return o;
});
if(ops.length!==6 || ops[1].op!=="DrawCompute" || ops[5].op!=="Write")
  throw Error("frozen five-node source changed");
const founder = [ops[0],ops[1],ops[3],ops[4],ops[5]];
function simulate(request, packet){
  const code = founder.map(o=>({...o}));code[0].value=request;
  const r=[0,0,0,0];
  let credits=2,pool=0,pc=0,attempts=0,service=0,paidDraws=0,deferred=0;
  const windows=[];
  const step=()=>{
    const op=code[pc],req=op.op==="DrawCompute"?r[op.amount]:0;
    const poolPaid=credits===0&&op.op==="DrawCompute"&&pool>=req+1;
    if(credits===0&&!poolPaid) return false;
    const before=credits+pool;
    if(poolPaid)pool--;else credits--;
    attempts++;
    switch(op.op){
      case "Const":r[op.dst]=op.value;pc=op.next;break;
      case "DrawCompute":
        if(pool<req){deferred++;break;}
        pool-=req;credits+=req;pc=op.next;
        if(poolPaid)paidDraws++;
        break;
      case "Write":
        if(r[op.address]!==9||r[op.value]!==1)throw Error("unexpected output");
        service++;pc=op.next;break;
      default:throw Error("unexpected opcode");
    }
    if(credits+pool!==before-1) throw Error("native charge conservation");
    return true;
  };
  const run=()=>{let n=0;while(step()){n++;if(n>100)throw Error("unbounded packet work");}return n;};
  const initial=run();
  if(initial!==2||credits!==0||pc!==1)throw Error("initial exhaustion changed");
  const pulses=packet===0?0:4;
  for(let i=0;i<pulses;i++){
    const before=credits+pool;
    pool+=packet;
    if(credits+pool!==before+packet)throw Error("neutral supply conservation");
    const count=run();
    windows.push({packet,attempts:count,serviceWrites:service,draws:paidDraws,credits,pool,pc});
  }
  return {request,packet,initial,windows,attempts,service,paidDraws,deferred,credits,pool,pc};
}
const cases=[[2,4],[3,4],[4,4],[2,5],[3,5],[4,5],[4,0]];
const expected=[
 {attempts:5,service:0,paidDraws:1,pool:13,pc:4,windows:[3,0,0,0]},
 {attempts:18,service:4,paidDraws:4,pool:0,pc:1,windows:[4,4,4,4]},
 {attempts:17,service:3,paidDraws:3,pool:1,pc:1,windows:[0,5,5,5]},
 {attempts:5,service:0,paidDraws:1,pool:17,pc:4,windows:[3,0,0,0]},
 {attempts:22,service:5,paidDraws:5,pool:0,pc:1,windows:[4,4,4,8]},
 {attempts:22,service:4,paidDraws:4,pool:0,pc:1,windows:[5,5,5,5]},
 {attempts:2,service:0,paidDraws:0,pool:0,pc:1,windows:[]},
];
const outcomes=cases.map(([r,p],i)=>{
  const x=simulate(r,p), exp=expected[i];
  for(const k of ["attempts","service","paidDraws","pool","pc"]){
    if(x[k]!==exp[k])throw Error(`B20 observer ${r}/${p}: unexpected ${k}`);
  }
  if(JSON.stringify(x.windows.map(w=>w.attempts))!==JSON.stringify(exp.windows))
    throw Error("B20 per-packet pattern changed");
  return x;
});
process.stdout.write(JSON.stringify({
  source:"cl0/crates/world/tests/cl1b_b19_neutral_compute_flux.rs",
  evidence:"independent source-shaped resource model, NOT native Rust",
  matchedWorlds:outcomes.length,
  outcomes,
},null,2)+"\n");
