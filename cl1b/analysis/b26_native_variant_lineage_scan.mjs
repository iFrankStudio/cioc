// CL-1B B26: independent restricted deterministic VM for native CIL
// first-generation worker copy, computed candidate genotype and competition.
// Parses actual B26 Rust source and enforces per-attempt compute conservation.
// NOT native Rust and NOT spontaneous self-discovered evolution.
import { readFileSync } from "node:fs";
const source=readFileSync(new URL("../../cl0/crates/world/tests/cl1b_b26_native_variant_lineage.rs",import.meta.url),"utf8");
const body=source.match(/fn founder\(base: i64, delta: i64\) -> Vec<Op> \{[\s\S]*?vec!\[([\s\S]*?)\n    \]/)?.[1];
if(!body)throw Error("B26 frozen Rust founder not found");
const parse=[...body.matchAll(/Op::([A-Za-z]+)\s*\{([^}]*)\}/g)].map(m=>{
 const op={kind:m[1]};
 for(const v of m[2].matchAll(/(\w+)\s*:\s*(-?\d+|base|delta)/g))
  op[v[1]]=["base","delta"].includes(v[2])?v[2]:Number(v[2]);
 return op;
});
if(parse.length!==35||parse[26].kind!=="Add"||
   parse[27].kind!=="GraphSetConst"||parse[29].kind!=="GraphLaunch")
 throw Error("B26 35-node CIL source shape changed");

function simulate(arm,supply){
 const [base,delta]=arm==="PARENT4_CHILD3"?[4,-1]:
  arm==="PARENT3_CHILD4"?[3,1]:[4,0];
 const founder=parse.map(op=>{
  const c={...op};
  for(const k of Object.keys(c))if(c[k]==="base")c[k]=base;
    else if(c[k]==="delta")c[k]=delta;
  return c;
 });
 const processes=[{id:1,code:founder,pc:0,r:[0,0,0,0],credits:34}];
 const objects=new Map(),events=[],external=["load1"];
 let pool=0,lastId=null,nextGraph=1,nextPid=2,attempts=0;
 const energy=()=>pool+processes.reduce((sum,p)=>sum+p.credits,0);
 const reg=(p,i)=>{if(!Number.isInteger(i)||i<0||i>=4)throw Error("bad register");return p.r[i];};
 const e=(p,i)=>{if(!Number.isInteger(i)||i<0||i>=p.code.length)throw Error("bad next");return i;};
 function step(){
  const eligible=processes.filter(p=>p.credits>0||(
   p.code[p.pc].kind==="DrawCompute"&&pool>=reg(p,p.code[p.pc].amount)+1
  ));
  if(!eligible.length)return false;
  const p=eligible.find(p=>lastId===null||p.id>lastId)??eligible[0];
  lastId=p.id;
  const at=p.pc,o=p.code[at],before=energy();
  if(p.credits===0)pool--;else p.credits--;
  let kind="Executed";
  switch(o.kind){
   case"Const":p.r[o.dst]=o.value;p.pc=e(p,o.next);break;
   case"Add":p.r[o.dst]=reg(p,o.lhs)+reg(p,o.rhs);p.pc=e(p,o.next);break;
   case"GraphNew":{const graphId=nextGraph++;objects.set(graphId,[]);p.r[o.dst]=graphId;p.pc=e(p,o.next);kind="GraphCreated";break;}
   case"GraphCopyNodeLocal":{
     const graphId=reg(p,o.graph),srcIndex=reg(p,o.source),arr=objects.get(graphId);
     if(!arr||srcIndex<0||srcIndex>=p.code.length)throw Error("bad graph clone");
     const cloned={...p.code[srcIndex]},dstIndex=arr.length;
     if(cloned.next!==undefined){
       const modulus=dstIndex+1;
       cloned.next=((dstIndex+cloned.next-srcIndex)%modulus+modulus)%modulus;
     }
     arr.push(cloned);p.r[o.dst]=dstIndex;p.pc=e(p,o.next);
     kind="GraphNodeCopiedLocal";break;
   }
   case"GraphSetNext":{
     const arr=objects.get(reg(p,o.graph)),node=reg(p,o.node),target=reg(p,o.target);
     if(!arr||node<0||node>=arr.length||target<0||target>=arr.length||arr[node].next===undefined)
       throw Error("bad source graph next edit");
     arr[node].next=target;p.pc=e(p,o.next);kind="GraphNodeEditedNext";break;
   }
   case"GraphSetConst":{
     const arr=objects.get(reg(p,o.graph)),node=reg(p,o.node),value=reg(p,o.value);
     if(!arr||node<0||node>=arr.length||arr[node].kind!=="Const")
       throw Error("bad source graph const edit");
     arr[node].value=value;p.pc=e(p,o.next);kind="GraphNodeEditedConst";break;
   }
   case"GraphLaunch":{
     const graphId=reg(p,o.graph),amount=reg(p,o.credits),childCode=objects.get(graphId);
     if(!childCode||!childCode.length||p.credits<amount)throw Error("invalid child launch");
     objects.delete(graphId);p.credits-=amount;
     const id=nextPid++;p.r[o.dst]=id;p.pc=e(p,o.next);
     processes.push({id,code:childCode,r:[0,0,0,0],pc:0,credits:amount});
     kind="GraphLaunched";break;
   }
   case"DrawCompute":{
     const amount=reg(p,o.amount);
     if(pool<amount){kind="DrawDeferred";break;}
     pool-=amount;p.credits+=amount;p.pc=e(p,o.next);kind="DrawCompute";break;
   }
   case"Write":{
     if(reg(p,o.address)!==9||reg(p,o.value)!==1)throw Error("invalid service output");
     p.pc=e(p,o.next);kind="CellWritten";break;
   }
   default:throw Error("unsupported CIL opcode "+o.kind);
  }
  attempts++;
  if(energy()!==before-1)throw Error("native compute inventory not conserved");
  events.push({process:p.id,pc:at,kind});
  return true;
 }
 function drain(){let n=0;while(step()){n++;if(n>=100)throw Error("unbounded pool consumption");}return n;}
 const bootstrap=drain();
 if(bootstrap!==34||processes.length!==2||energy()!==0||objects.size!==0)
  throw Error("unexpected B26 native actor lineage bootstrap");
 const child=processes[1],parent=processes[0];
 if(parent.code[30].value!==base||child.code[0].value!==base+delta||
   child.code.length!==5||parent.pc!==31||child.pc!==1)
  throw Error("native CIL child worker variation was not realized");
 const counts=pid=>events.filter(ev=>ev.process===pid&&ev.kind==="CellWritten"&&ev.pc===(pid===1?34:4)).length;
 const byPacket=[];
 let lastParent=0,lastChild=0;
 for(const quantity of supply){
  const before=energy();pool+=quantity;external.push("pool+"+quantity);
  if(energy()!==before+quantity)throw Error("world pool external packet accounting");
  const attempted=drain(),p=counts(1),c=counts(2);
  byPacket.push({quantity,attempted,serviceParent:p-lastParent,serviceChild:c-lastChild,pool});
  lastParent=p;lastChild=c;
 }
 if(attempts+energy()!==34+supply.reduce((a,b)=>a+b,0))
  throw Error("B26 final compute conservation broken");
 return {arm,base,delta,childRequest:child.code[0].value,supply,bootstrap,
  parentWrites:lastParent,childWrites:lastChild,byPacket,
  attempts,pool,created:events.filter(e=>e.kind==="GraphCreated").length,
  copies:events.filter(e=>e.kind==="GraphNodeCopiedLocal").length,
  edgeEdits:events.filter(e=>e.kind==="GraphNodeEditedNext").length,
  candidateEdits:events.filter(e=>e.kind==="GraphNodeEditedConst").length,
  children:events.filter(e=>e.kind==="GraphLaunched").length,
  final:processes.map(p=>({id:p.id,pc:p.pc,credits:p.credits,codeNodes:p.code.length}))};
}

const schedules=[[4,4,4,4,4,4],[5,5,5,5,5,5],[4,5,4,5,4,5],[5,4,5,4,5,4]];
const arms=["PARENT4_CHILD3","PARENT3_CHILD4","PARENT4_CHILD4"];
const worlds=schedules.flatMap(supply=>arms.map(arm=>simulate(arm,supply)));
if(worlds.length!==12)throw Error("B26 wrong world denominator");
for(const w of worlds) {
 if(w.created!==1||w.copies!==5||w.edgeEdits!==4||
   w.candidateEdits!==1||w.children!==1||w.bootstrap!==34)
  throw Error("B26 CIL lineage source classification changed");
 if(w.supply.every(x=>x===4)){
  if(w.arm==="PARENT4_CHILD3"&&(w.parentWrites!==0||w.childWrites!==6))
   throw Error("B26 scarce pool not selecting CIL-derived child3");
  if(w.arm==="PARENT3_CHILD4"&&(w.parentWrites!==6||w.childWrites!==0))
   throw Error("B26 scarce pool not selecting parent3 after ID swap");
 }
}
process.stdout.write(JSON.stringify({
  evidence:"restricted source-parsed CIL interpreter, not native Rust",
  source:"cl0/crates/world/tests/cl1b_b26_native_variant_lineage.rs",
  worlds:worlds.length,cases:worlds
},null,2)+"\n");
