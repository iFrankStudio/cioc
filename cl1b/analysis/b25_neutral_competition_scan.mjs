// CL-1B B25 independent deterministic CIL scheduler, source-parsed from
// canonical Rust founder. Neutral world compute pool, two fixed requests,
// ID-swap and sole-actor controls. This is NOT native Rust CI.
import { readFileSync } from "node:fs";
const source=readFileSync(new URL("../../cl0/crates/world/tests/cl1b_b25_neutral_compute_competition.rs",import.meta.url),"utf8");
const body=source.match(/fn founder\(request: i64\) -> Vec<Op> \{[\s\S]*?vec!\[([\s\S]*?)\n    \]/)?.[1];
if(!body)throw Error("B25 native founder absent");
const base=[...body.matchAll(/Op::([A-Za-z]+)\s*\{([^}]*)\}/g)].map(m=>{
 const v={op:m[1]};
 for(const x of m[2].matchAll(/(\w+)\s*:\s*(-?\d+|request)/g))
  v[x[1]]=x[2]==="request"?"request":Number(x[2]);
 return v;
});
if(base.length!==5||base[0].value!=="request"||base[1].op!=="DrawCompute"||base[4].op!=="Write")
 throw Error("B25 Rust program ABI drift");

function simulate(assignment,schedule) {
 const policies=assignment==="ORDER34"?[[1,3],[2,4]]:
   assignment==="ORDER43"?[[1,4],[2,3]]:
   assignment==="SOLO3"?[[1,3]]:[[1,4]];
 const processes=policies.map(([id,request])=>({
   id,request,credits:2,pc:0,r:[0,0,0,0],
   code:base.map(x=>({...x,value:x.value==="request"?request:x.value}))
 }));
 let pool=0,cursor=null,totalAttempts=0,events=[],cell9=null;
 const energy=()=>pool+processes.reduce((t,p)=>t+p.credits,0);
 const reg=(p,i)=>{if(!Number.isInteger(i)||i<0||i>=4)throw Error("bad reg");return p.r[i];};
 function step() {
  const eligible=processes.filter(p=>{
    if(p.credits>0)return true;
    const op=p.code[p.pc];
    return op.op==="DrawCompute"&&pool>=reg(p,op.amount)+1;
  });
  if(!eligible.length)return false;
  const p=eligible.find(x=>cursor===null||x.id>cursor)??eligible[0];
  cursor=p.id;
  const at=p.pc,op=p.code[p.pc],before=energy();
  if(p.credits===0)pool--;else p.credits--;
  totalAttempts++;
  let event="Executed";
  switch(op.op){
   case"Const":p.r[op.dst]=op.value;p.pc=op.next;break;
   case"DrawCompute":{
    const req=reg(p,op.amount);
    if(pool>=req){pool-=req;p.credits+=req;p.pc=op.next;event="Draw";}
    else event="DrawDeferred";
    break;
   }
   case"Write":{
    const addr=reg(p,op.address),value=reg(p,op.value);
    if(addr!==9||value!==1)throw Error("noncanonical service");
    cell9=value;p.pc=op.next;event="Service";
    break;
   }
   default:throw Error("unsupported op "+op.op);
  }
  if(energy()!==before-1)throw Error("compute minted");
  events.push({id:p.id,request:p.request,pc:at,event,pool,credits:p.credits});
  return true;
 }
 function run(){
  let n=0;while(step()){n++;if(n>100)throw Error("unbounded work");}return n;
 }
 const startup=run();if(startup!==processes.length*2||energy()!==0)
  throw Error("B25 bootstrap changed");
 const windows=[],externals=processes.length;
 for(const packet of schedule){
  const before=energy();pool+=packet;
  if(energy()!==before+packet)throw Error("bad environment");
  const beforeEvents=events.length,steps=run();
  const windowEvents=events.slice(beforeEvents);
  windows.push({packet,steps,writes3:windowEvents.filter(x=>x.event==="Service"&&x.request===3).length,
    writes4:windowEvents.filter(x=>x.event==="Service"&&x.request===4).length,
    firstNative:windowEvents[0]?.id??null,endingPool:pool});
 }
 const service3=events.filter(x=>x.event==="Service"&&x.request===3).length;
 const service4=events.filter(x=>x.event==="Service"&&x.request===4).length;
 const conserved=totalAttempts+energy()===processes.length*2+schedule.reduce((s,x)=>s+x,0);
 if(!conserved)throw Error("global energy conservation");
 return {assignment,schedule,service3,service4,attempts:totalAttempts,
   pool,positions:processes.map(p=>({request:p.request,id:p.id,pc:p.pc,credits:p.credits})),
   windows};
}
const schedules=[[4,4,4,4,4,4],[5,5,5,5,5,5],[4,5,4,5,4,5],[5,4,5,4,5,4]];
const assignments=["ORDER34","ORDER43","SOLO3","SOLO4"];
const worlds=schedules.flatMap(schedule=>assignments.map(a=>simulate(a,schedule)));
if(worlds.length!==16)throw Error("wrong B25 denominator");
for(const world of worlds){
 if(world.schedule[0]===4&&world.schedule.every(n=>n===4)){
  if(["ORDER34","ORDER43"].includes(world.assignment) &&
    (world.service3!==6||world.service4!==0))
    throw Error("B25 ecological threshold result not ID-invariant");
 }
}
process.stdout.write(JSON.stringify({
 evidence:"independent source-parsed deterministic CL-0-like CIL scheduler; NOT native Rust",
 source:"cl0/crates/world/tests/cl1b_b25_neutral_compute_competition.rs",
 worlds:worlds.length,
 cases:worlds
},null,2)+"\n");
