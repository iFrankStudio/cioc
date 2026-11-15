// CL-1B B19 observer-side resource model. NOT the CL-0 Rust VM.
// Extracts the explicit five-instruction candidate and no-draw control from
// the committed Rust founder source and simulates conserved compute transfers.
import { readFileSync } from "node:fs";
const source = readFileSync(
  new URL("../../cl0/crates/world/tests/cl1b_b19_neutral_compute_flux.rs", import.meta.url), "utf8"
);
const body=source.match(/fn founder\(draw: bool\) -> Vec<Op> \{[\s\S]*?vec!\[([\s\S]*?)\n    \]/)?.[1];
if(!body)throw Error("B19 Rust founder missing");
const ops=[...body.matchAll(/Op::([A-Za-z]+)\s*\{([^}]*)\}/g)].map(m=>{
 const x={op:m[1]};for(const f of m[2].matchAll(/(\w+)\s*:\s*(-?\d+)/g))x[f[1]]=Number(f[2]);
 return x;
});
if(ops.length!==6||ops[0].op!=="Const"||ops[1].op!=="DrawCompute"
   ||ops[2].op!=="Const"||ops[5].op!=="Write")
 throw Error("B19 native source shape drift");
function simulate(arm){
 const active=arm!=="NO_DRAW_OPCODE";
 const code=[ops[0],active?ops[1]:ops[2],ops[3],ops[4],ops[5]];
 const poolPaid=arm!=="DRAW_NOT_POOL_PAID";
 const packets=arm==="RENEWABLE_ACCESS"||arm==="NO_DRAW_OPCODE"||arm==="DRAW_NOT_POOL_PAID"
    ?[5,5,5,5,5,5]:arm==="SUBTHRESHOLD_THEN_CROSSING"?[4,1]:[];
 const r=[0,0,0,0],cells=new Map();let pc=0,credits=2,pool=0,attempts=0,writes=0,paid=0,deferred=0,stopped=false;
 const externals=[{type:"load",credit:2}],windows=[];
 function attempt(){
  const op=code[pc],request=op.op==="DrawCompute"?r[op.amount]:0;
  const canPayPool=poolPaid&&op.op==="DrawCompute"&&credits===0&&pool>=request+1;
  if(credits===0&&!canPayPool)return false;
  const before=credits+pool;
  if(canPayPool){pool--;paid++;}else credits--;
  attempts++;
  switch(op.op){
   case"Const":r[op.dst]=op.value;pc=op.next;break;
   case"DrawCompute":
     if(request>pool){deferred++;break;}
     pool-=request;credits+=request;pc=op.next;break;
   case"Write":cells.set(r[op.address],r[op.value]);if(r[op.address]===9&&r[op.value]===1)writes++;pc=op.next;break;
   default:throw Error("unknown opcode");
  }
  if(credits+pool!==before-1)throw Error("resource conservation broken");
  return true;
 }
 function run(){let n=0;while(attempt()){n++;if(n>100)throw Error("unbounded packet");}return n;}
 const boot=run();
 if(boot!==2)throw Error("expected two initial instruction attempts");
 for(const packet of packets){
  const before=credits+pool;pool+=packet;
  if(credits+pool!==before+packet)throw Error("external inflow mismatch");
  externals.push({type:"pool_packet",amount:packet});
  const work=run();windows.push({packet,work,credits,pool,pc,writes});
 }
 return {arm,boot,packets,windows,attempts,writes,pool,credits,pc,paid,deferred,stopped,externalActions:externals.length};
}
const arms=["RENEWABLE_ACCESS","NO_DRAW_OPCODE","DRAW_NOT_POOL_PAID","SUBTHRESHOLD_THEN_CROSSING","NO_INFLUX"];
const outcomes=arms.map(simulate);
const expected={
 RENEWABLE_ACCESS:{attempts:32,writes:6,pool:0,paid:6,deferred:7},
 NO_DRAW_OPCODE:{attempts:2,writes:0,pool:30,paid:0,deferred:0},
 DRAW_NOT_POOL_PAID:{attempts:2,writes:0,pool:30,paid:0,deferred:1},
 SUBTHRESHOLD_THEN_CROSSING:{attempts:7,writes:1,pool:0,paid:1,deferred:2},
 NO_INFLUX:{attempts:2,writes:0,pool:0,paid:0,deferred:1},
};
for(const actual of outcomes){
 for(const [key,value] of Object.entries(expected[actual.arm])) {
  if(actual[key]!==value)throw Error(`observer B19 ${actual.arm} ${key}=${actual[key]} not ${value}`);
 }
}
process.stdout.write(JSON.stringify({
  source:"cl0/crates/world/tests/cl1b_b19_neutral_compute_flux.rs",
  evidence:"independent simplified compute model; NOT native Rust CI",
  outcomes,
},null,2)+"\n");
