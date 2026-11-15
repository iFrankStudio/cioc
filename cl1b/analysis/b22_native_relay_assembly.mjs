// CL-1B B22: independent source-parsed restricted deterministic CIL model.
// Reads the exact Rust founder, including the inert helper template, and
// models native GraphNew/GraphCopyNodeLocal/GraphSetNext/GraphLaunch with charged
// execution. OBSERVER-ONLY, not a substitute for verified native Rust tests.
import { readFileSync } from "node:fs";
const source=readFileSync(new URL("../../cl0/crates/world/tests/cl1b_b22_native_relay_assembly.rs",import.meta.url),"utf8");
const body=source.match(/fn founder\(transfer: bool\) -> Vec<Op> \{[\s\S]*?vec!\[([\s\S]*?)\n    \]/)?.[1];
if(!body)throw Error("B22 founder source changed");
const extracted=[...body.matchAll(/Op::([A-Za-z]+)\s*\{([^}]*)\}/g)].map(m=>{
 const op={kind:m[1]};for(const a of m[2].matchAll(/(\w+)\s*:\s*(-?\d+)/g))op[a[1]]=Number(a[2]);return op;
});
if(extracted.length!==36||extracted[24].kind!=="GraphLaunch"||
   extracted[34].kind!=="TransferCompute"||extracted[35].kind!=="Const")
 throw Error("B22 source-shape drift");

function run(arm){
 const transfer=arm!=="NO_TRANSFER";
 const founder=extracted.slice(0,34).concat([transfer?extracted[34]:extracted[35]]);
 if(founder.length!==35)throw Error("B22 expected 35 nodes");
 const processes=[{id:1,code:founder.map(o=>({...o})),r:[0,0,0,0],credits:29,pc:0,stopped:false}];
 let pool=0,stepCount=0,roundRobin=null,nextGraphId=1,nextPid=2,writes=0,transfers=0;
 const objects=new Map(),events=[],externals=["load1"];
 const inventory=()=>pool+processes.reduce((s,p)=>s+p.credits,0);
 const reg=(p,i)=>{if(!Number.isInteger(i)||i<0||i>=4)throw Error("bad reg");return p.r[i];};
 const next=(p,i)=>{if(!Number.isInteger(i)||i<0||i>=p.code.length)throw Error("bad edge "+i);return i;};
 function step(){
  const eligible=processes.filter(p=>{
   if(p.stopped)return false;
   if(p.credits>0)return true;
   const o=p.code[p.pc];
   return o.kind==="DrawCompute"&&pool>=reg(p,o.amount)+1;
  });
  if(!eligible.length)return false;
  const p=eligible.find(x=>roundRobin===null||x.id>roundRobin)??eligible[0];
  roundRobin=p.id;
  const at=p.pc,o=p.code[at],before=inventory();
  if(p.credits>0)p.credits--;else pool--;
  let kind="Executed";
  switch(o.kind){
   case"Const":p.r[o.dst]=o.value;p.pc=next(p,o.next);break;
   case"GraphNew":p.r[o.dst]=nextGraphId;objects.set(nextGraphId++,[]);p.pc=next(p,o.next);kind="GraphCreated";break;
   case"GraphCopyNodeLocal":{
    const id=reg(p,o.graph),index=reg(p,o.source),arr=objects.get(id);
    if(!arr||index<0||index>=p.code.length||arr.length>=64)throw Error("bad graph copy");
    const copied={...p.code[index]},target=arr.length;
    if(copied.next!==undefined)
     copied.next=((target+(copied.next-index))%(target+1)+(target+1))%(target+1);
    arr.push(copied);p.r[o.dst]=target;p.pc=next(p,o.next);
    kind="GraphNodeCopiedLocal";break;
   }
   case"GraphSetNext":{
    const arr=objects.get(reg(p,o.graph)),index=reg(p,o.node),target=reg(p,o.target);
    if(!arr||index<0||index>=arr.length||target<0||target>=arr.length||arr[index].next===undefined)
      throw Error("bad graph set next");
    arr[index].next=target;p.pc=next(p,o.next);kind="GraphNodeEdited";break;
   }
   case"GraphLaunch":{
    const graphId=reg(p,o.graph),credits=reg(p,o.credits),child=objects.get(graphId);
    if(!child||!child.length||credits>p.credits)throw Error("bad launch");
    const newPid=nextPid++;objects.delete(graphId);p.credits-=credits;p.r[o.dst]=newPid;p.pc=next(p,o.next);
    processes.push({id:newPid,code:child,r:[0,0,0,0],credits,pc:0,stopped:false});
    kind="GraphLaunched";break;
   }
   case"DrawCompute":{
    const req=reg(p,o.amount);
    if(pool<req){kind="DrawDeferred";break;}
    pool-=req;p.credits+=req;p.pc=next(p,o.next);kind="DrawCompute";break;
   }
   case"Write":{
    if(reg(p,o.address)===9&&reg(p,o.value)===1&&p.id===1)writes++;
    p.pc=next(p,o.next);kind="CellWritten";break;
   }
   case"TransferCompute":{
    const to=processes.find(q=>q.id===reg(p,o.recipient)),q=reg(p,o.amount);
    if(!to||p.credits<q)throw Error("bad transfer");
    to.credits+=q;p.credits-=q;transfers++;p.pc=next(p,o.next);
    kind="TransferCompute";break;
   }
   default:throw Error("unsupported B22 op "+o.kind);
  }
  stepCount++;
  if(inventory()!==before-1)throw Error("native charge was not conserved");
  events.push({process:p.id,pc:at,kind,remaining:inventory()});
  return true;
 }
 function runToBlocked(){let n=0;while(step()){n++;if(n>100)throw Error("unbounded loop");}return n;}
 const bootstrap=runToBlocked();
 const packets=arm==="NO_SECOND_PACKET"?[3]:[3,5],windows=[];
 for(const packet of packets){
  const before=inventory();pool+=packet;externals.push("pool+"+packet);
  if(inventory()!==before+packet)throw Error("nonconserving host packet");
  windows.push({packet,steps:runToBlocked(),writes,transfers,pool,pcs:processes.map(p=>p.pc)});
 }
 return {arm,bootstrap,windows,writes,transfers,steps:stepCount,pool,processes:processes.map(p=>({
   id:p.id,pc:p.pc,credits:p.credits,codeLength:p.code.length,
   lastNode:p.code[p.code.length-1]})),
   created:events.filter(e=>e.kind==="GraphCreated").length,
   copied:events.filter(e=>e.kind==="GraphNodeCopiedLocal").length,
   rewired:events.filter(e=>e.kind==="GraphNodeEdited").length,
   launched:events.filter(e=>e.kind==="GraphLaunched").length,
   externalActions:externals.length,graphObjectsRemaining:objects.size,
   trace:events};
}
const outcomes=["BUILD_TRANSFER","NO_TRANSFER","NO_SECOND_PACKET"].map(name=>
 run(name==="BUILD_TRANSFER"?"TRANSFER":name));
const expected=[
 {bootstrap:29,writes:1,transfers:1,steps:37,pool:0,launched:1},
 {bootstrap:29,writes:0,transfers:0,steps:37,pool:0,launched:1},
 {bootstrap:29,writes:0,transfers:0,steps:32,pool:0,launched:1},
];
for(let i=0;i<outcomes.length;i++){
 const o=outcomes[i],x=expected[i];
 for(const [key,val] of Object.entries(x))if(o[key]!==val)
  throw Error(`B22 source-shaped result mismatch arm=${o.arm} ${key}: ${o[key]} vs ${val}`);
 if(o.created!==1||o.copied!==5||o.rewired!==4||o.processes.length!==2||
    o.graphObjectsRemaining!==0)throw Error("B22 graph assembly invariant failed");
}
process.stdout.write(JSON.stringify({
 evidence:"restricted independent CIL simulator; NOT native Rust",
 source:"cl0/crates/world/tests/cl1b_b22_native_relay_assembly.rs",
 outcomes,
},null,2)+"\n");
