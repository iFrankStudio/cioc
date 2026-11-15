// CL-1B B21: independent source-parsed resource relay simulator.
// Uses the EXACT B21 Rust founder opcodes and deterministic round-robin scheduler.
// This is NOT CL-0 native Rust evidence or an emergent organism.
import { readFileSync } from "node:fs";
const source = readFileSync(
  new URL("../../cl0/crates/world/tests/cl1b_b21_cooperative_compute_relay.rs", import.meta.url),
  "utf8"
);
function parse(name) {
  const m=source.match(new RegExp("fn "+name+"\\([^)]*\\) -> Vec<Op> \\{[\\s\\S]*?vec!\\[([\\s\\S]*?)\\n    \\]"));
  if(!m)throw Error("missing frozen B21 "+name+" founder");
  return [...m[1].matchAll(/Op::([A-Za-z]+)\s*\{([^}]*)\}/g)].map(found=>{
    const o={op:found[1]};
    for(const part of found[2].matchAll(/(\w+)\s*:\s*(-?\d+)/g))o[part[1]]=Number(part[2]);
    return o;
  });
}
const worker = parse("worker"), helper=parse("relay");
if(worker.length!==5||helper.length!==6||helper[4].op!=="TransferCompute")
  throw Error("frozen B21 graph changed");

function simulate(arm) {
  const processes=[{id:1,code:worker.map(o=>({...o})),pc:0,credits:2,r:[0,0,0,0]}];
  if(arm!=="NO_RELAY"){
    const code=[helper[0],helper[1],helper[2],helper[3],arm==="RELAY"?helper[4]:helper[5]];
    processes.push({id:2,code:code.map(o=>({...o})),pc:0,credits:2,r:[0,0,0,0]});
  }
  let pool=0,cursor=null,nativeAttempts=0,serviceWrites=0,relayTransfers=0;
  const trace=[],externalActions=[{type:"load",id:1},...(arm==="NO_RELAY"?[]:[{type:"load",id:2}])];
  const energy=()=>pool+processes.reduce((t,p)=>t+p.credits,0);
  const reg=(p,i)=>{if(!Number.isInteger(i)||i<0||i>=4)throw Error("bad reg");return p.r[i]};
  function step(){
    const runnable=processes.filter(p=>{
      const op=p.code[p.pc];
      return p.credits>0||(op.op==="DrawCompute"&&pool>=reg(p,op.amount)+1);
    });
    if(!runnable.length)return false;
    const p=runnable.find(p=>cursor===null||p.id>cursor)??runnable[0];
    cursor=p.id;
    const at=p.pc,op=p.code[at],before=energy();
    const poolPaid=p.credits===0; if(poolPaid)pool--;else p.credits--;
    let kind="Executed";
    switch(op.op){
      case "Const":p.r[op.dst]=op.value;p.pc=op.next;break;
      case "DrawCompute":{
        const request=reg(p,op.amount);
        if(request>pool){kind="DrawDeferred";break;}
        pool-=request;p.credits+=request;p.pc=op.next;kind="DrawCompute";break;
      }
      case "TransferCompute":{
        const id=reg(p,op.recipient),to=processes.find(other=>other.id===id);
        const quantity=reg(p,op.amount);
        if(!to||p.credits<quantity)throw Error("invalid transfer");
        p.credits-=quantity;to.credits+=quantity;p.pc=op.next;
        relayTransfers++;kind="TransferCompute";break;
      }
      case "Write":if(reg(p,op.address)!==9||reg(p,op.value)!==1)throw Error("wrong service write");
        serviceWrites++;p.pc=op.next;kind="CellWritten";break;
      default:throw Error("unknown native opcode "+op.op);
    }
    nativeAttempts++;
    if(energy()!==before-1)throw Error("native attempt minted compute");
    trace.push({id:p.id,pc:at,kind,pool,credits:processes.map(q=>q.credits),poolPaid});
    return true;
  }
  function run(){
    let n=0;
    while(step()){
      n++;if(n>=50)throw Error("unexpected unbounded work");
    }
    return n;
  }
  const startup=run();
  for(const packet of [3,5]){
    const before=energy();pool+=packet;
    if(energy()!==before+packet)throw Error("external supply must be explicit");
    externalActions.push({type:"world_pool_packet",packet});
    const steps=run();
    if(packet===3)var afterFirst=steps;
    else var afterSecond=steps;
  }
  return {arm,startup,afterFirst,afterSecond,nativeAttempts,serviceWrites,relayTransfers,
    pool,processes:processes.map(p=>({id:p.id,pc:p.pc,credits:p.credits})),
    externalActions:externalActions.length,trace};
}
const outcomes=["RELAY","NO_TRANSFER","NO_RELAY"].map(simulate);
const expected=[
 {startup:4,afterFirst:3,afterSecond:5,nativeAttempts:12,serviceWrites:1,relayTransfers:1,pool:0},
 {startup:4,afterFirst:3,afterSecond:5,nativeAttempts:12,serviceWrites:0,relayTransfers:0,pool:0},
 {startup:2,afterFirst:3,afterSecond:0,nativeAttempts:5,serviceWrites:0,relayTransfers:0,pool:5},
];
for(let i=0;i<outcomes.length;i++){
 for(const [k,v] of Object.entries(expected[i])){
  if(outcomes[i][k]!==v)throw Error(`B21 source/scheduler mismatch ${outcomes[i].arm} ${k}`);
 }
}
process.stdout.write(JSON.stringify({
  source:"cl0/crates/world/tests/cl1b_b21_cooperative_compute_relay.rs",
  evidence:"independent simplified deterministic scheduler; NOT native Rust CI",
  worlds:outcomes.length,outcomes,
},null,2)+"\n");
