// B32 independent, Rust-source-parsed restricted CIL scheduler.
// Checks native PC4 Write.next mutation, charged attempts and zero-credit gating.
// NOT native Rust execution or evidence of endogenous learning.
import { readFileSync } from "node:fs";
const source=readFileSync(new URL("../../cl0/crates/world/tests/cl1b_b32_prune_metabolic_loop.rs",import.meta.url),"utf8");
function parse(name){
 const body=source.match(new RegExp("fn "+name+"\\(\\)->Vec<Op>\\{([\\s\\S]*?)\\n\\}"))?.[1];
 if(!body)throw Error("B32 Rust founder source missing "+name);
 return [...body.matchAll(/Op::([A-Za-z]+)\s*\{([^}]*)\}/g)].map(m=>{
  const o={kind:m[1]};
  for(const a of m[2].matchAll(/(\w+)\s*:\s*(-?\d+)/g))o[a[1]]=Number(a[2]);
  return o;
 });
}
const full=parse("pruning_founder"),staticNodes=parse("static_founder");
if(full.length!==10||staticNodes.length!==5||full[8].kind!=="PatchNext"||
   full[4].kind!=="Write"||staticNodes[4].next!==1)
 throw Error("B32 native CIL source structure changed");

function trial(arm,packets) {
 const code=(arm==="StaticThree"?staticNodes:full).map(x=>({...x}));
 const r=[0,0,0,0];let local=2,pool=0,pc=0,steps=0,writes=0,edits=0,suppressed=0;
 const reg=i=>{if(!Number.isInteger(i)||i<0||i>=4)throw Error("bad register");return r[i];};
 function step(){
  const op=code[pc],payPool=!local&&op.kind==="DrawCompute"&&pool>=reg(op.amount)+1;
  if(!local&&!payPool)return false;
  const before=local+pool;
  if(payPool)pool--;else local--;
  steps++;
  switch(op.kind){
   case"Const":r[op.dst]=op.value;pc=op.next;break;
   case"DrawCompute":{
    const amount=reg(op.amount);
    if(pool>=amount){pool-=amount;local+=amount;pc=op.next;}
    break;
   }
   case"Write":
    if(reg(op.address)!==9||reg(op.value)!==1)throw Error("B32 nonservice output");
    writes++;pc=op.next;break;
   case"PatchNext":{
    const node=reg(op.node),target=reg(op.target);
    if(!Number.isInteger(target)||target<0||target>=code.length||!code[node])
     throw Error("invalid patch");
    if(arm==="PatchSuppressed")suppressed++;else{code[node].next=target;edits++;}
    pc=op.next;break;
   }
   default:throw Error("unsupported B32 CIL native opcode");
  }
  if(local+pool!==before-1)throw Error("B32 compute conservation violated");
  return true;
 }
 const drain=()=>{let n=0;while(step()){if(++n>100)throw Error("unbounded B32 computation");}return n;};
 const bootstrap=drain();if(bootstrap!==2||pc!==1||local!==0)throw Error("B32 bad initial native state");
 const windows=[];
 for(const packet of packets){
  const before=local+pool;pool+=packet;if(local+pool!==before+packet)throw Error("bad pool intervention");
  windows.push({packet,paid:drain(),services:writes,pc,local,pool});
 }
 if(steps+local+pool!==2+packets.reduce((a,b)=>a+b,0))throw Error("invalid total compute");
 return {arm,packets,bootstrap,writes,steps,local,pool,pc,edge:code[4].next,
  edits,suppressed,windows};
}
const schedules=[[10,4,4,4],[10,3,3,3],[10,5,5,5],[9,4,4,4]];
const worlds=[];
for(const packets of schedules)for(const arm of ["Prune","PatchSuppressed","StaticThree"]){
 worlds.push(trial(arm,packets));
}
if(worlds.length!==12)throw Error("B32 wrong world denominator");
const expected=[
 {services:[4,2,5],pools:[0,8,2]},
 {services:[3,2,4],pools:[1,5,3]},
 {services:[4,2,6],pools:[3,11,1]},
 {services:[4,2,5],pools:[0,8,1]},
];
for(let i=0;i<4;i++){
 const group=worlds.slice(i*3,i*3+3);
 if(JSON.stringify(group.map(x=>x.writes))!==JSON.stringify(expected[i].services))
  throw Error("B32 frozen 12-world exact service counts drifted");
 if(JSON.stringify(group.map(x=>x.pool))!==JSON.stringify(expected[i].pools))
  throw Error("B32 frozen 12-world residual pool resource accounting drifted");
}
for(const w of worlds){
 if(w.arm==="Prune"&&w.edits>0&&(w.edge!==1||w.pc!==1))
  throw Error("B32 native-source pruned loop not pool-reachable");
 if(w.arm==="PatchSuppressed"&&w.edge!==5)
  throw Error("B32 native patch suppression did not preserve original source");
}
const noFlux=trial("Prune",[]);
if(noFlux.writes||noFlux.edits||noFlux.pc!==1)
 throw Error("B32 zero neutral input control invalid");
process.stdout.write(JSON.stringify({
 evidence:"source-parsed independent CIL simulator; NOT native Rust",
 source:"cl0/crates/world/tests/cl1b_b32_prune_metabolic_loop.rs",
 worlds:worlds.length,controls:{noFlux},cases:worlds
},null,2)+"\n");
