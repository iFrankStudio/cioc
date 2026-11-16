// B29 independent source-parsed deterministic CIL substrate simulation.
// This reproduces native instruction charges and full-genome origin; NOT Rust CI,
// automatic discovery, endogenous evolution, or biological fitness.
import { readFileSync } from "node:fs";
const source=readFileSync(new URL("../../cl0/crates/world/tests/cl1b_b29_mutation_rule_siblings.rs",import.meta.url),"utf8");
const body=source.match(/fn founder\(initial: i64,delta: i64, flip: bool\) -> Vec<Op> \{[\s\S]*?vec!\[([\s\S]*?)\n    \]/)?.[1];
if(!body)throw Error("B29 native founder missing");
const all=[...body.matchAll(/Op::([A-Za-z]+)\s*\{([^}]*)\}/g)].map(m=>{
 const o={kind:m[1]};
 for(const a of m[2].matchAll(/(\w+)\s*:\s*(-?\d+|initial|delta)/g))
  o[a[1]]=a[2]==="initial"||a[2]==="delta"?a[2]:Number(a[2]);
 return o;
});
if(all.length!==24||all[22].kind!=="PatchConst"||all[23].kind!=="Const"||
   all[7].value!==23||all[18].next!==19)throw Error("B29 23-opcode source ABI changed");
const program=(initial,delta,flip)=>all.slice(0,22).concat([all[flip?22:23]]).map(op=>{
 const next={...op};
 for(const key in next){if(next[key]==="initial")next[key]=initial;
  else if(next[key]==="delta")next[key]=delta;}
 return next;
});
const sameGenome=(a,b)=>JSON.stringify(a)===JSON.stringify(b);
function run(name,packets) {
 const flip=name!=="NO_FLIP",disabled=name==="COPY_DISABLED";
 const processes=[{id:1,code:program(156,1,flip),registers:[0,0,0,0],pc:0,credits:2,stopped:false}];
 let pool=0,lastId=null,graphs=new Map(),nextGraph=1,nextId=2,attempts=0;
 const events=[],births=[],edits=[];
 const energy=()=>pool+processes.reduce((n,p)=>n+p.credits,0);
 const reg=(p,index)=>{if(!Number.isInteger(index)||index<0||index>=4)throw Error("invalid register");return p.registers[index];};
 const goto=(p,i)=>{if(!Number.isInteger(i)||i<0||i>=p.code.length)throw Error("invalid edge");p.pc=i;};
 function step(){
  const elig=processes.filter(p=>!p.stopped&&(
   p.credits>0||(p.code[p.pc].kind==="DrawCompute"&&pool>=reg(p,p.code[p.pc].amount)+1)
  ));
  if(!elig.length)return false;
  const p=elig.find(p=>lastId===null||p.id>lastId)??elig[0];lastId=p.id;
  const op=p.code[p.pc],at=p.pc,before=energy(),priorDelta=p.code[12].value;
  if(p.credits>0)p.credits--;else pool--;
  let kind="Executed";
  try {
   switch(op.kind){
    case "Const":p.registers[op.dst]=op.value;goto(p,op.next);break;
    case "Add":p.registers[op.dst]=reg(p,op.lhs)+reg(p,op.rhs);goto(p,op.next);break;
    case "Sub":p.registers[op.dst]=reg(p,op.lhs)-reg(p,op.rhs);goto(p,op.next);break;
    case "Branch":goto(p,reg(p,op.test)!==0?op.nonzero:op.zero);break;
    case "DrawCompute":{
     const quantity=reg(p,op.amount);
     if(pool<quantity){kind="DrawDeferred";break;}
     pool-=quantity;p.credits+=quantity;goto(p,op.next);kind="DrawCompute";break;
    }
    case "GraphNew":{
     const id=nextGraph++;graphs.set(id,[]);
     p.registers[op.dst]=id;goto(p,op.next);kind="GraphCreated";break;
    }
    case "GraphCopyNode":{
     const graph=graphs.get(reg(p,op.graph)),idx=reg(p,op.source);
     if(!graph||idx<0||idx>=p.code.length)throw Error("invalid native graph copy");
     if(!disabled){graph.push({...p.code[idx]});kind="GraphNodeCopied";}
     else kind="NativeCodeEditSuppressed";
     p.registers[op.dst]=graph.length-1;goto(p,op.next);break;
    }
    case "GraphSetConst":{
     const graph=graphs.get(reg(p,op.graph)),idx=reg(p,op.node);
     if(!graph||idx<0||idx>=graph.length||graph[idx].kind!=="Const")throw Error("invalid native graph edit");
     if(!disabled){graph[idx].value=reg(p,op.value);kind="GraphNodeEdited";}
     else kind="NativeCodeEditSuppressed";
     goto(p,op.next);break;
    }
    case "GraphLaunch":{
     const id=reg(p,op.graph),credits=reg(p,op.credits),code=graphs.get(id);
     if(!code||!code.length||p.credits<credits)throw Error("invalid launch or nonconserving child budget");
     graphs.delete(id);p.credits-=credits;
     const child=nextId++;p.registers[op.dst]=child;goto(p,op.next);
     processes.push({id:child,code,registers:[0,0,0,0],pc:0,credits,stopped:false});
     births.push([p.id,child]);kind="GraphLaunched";break;
    }
    case "PatchConst":{
     const idx=reg(p,op.node),value=reg(p,op.value);
     if(!p.code[idx]||p.code[idx].kind!=="Const")throw Error("bad executable patch target");
     if(!disabled){p.code[idx].value=value;kind="ExecutableConstPatched";}
     else kind="NativeCodeEditSuppressed";
     goto(p,op.next);break;
    }
    default:throw Error("unsupported native opcode "+op.kind);
   }
  } catch(error){p.stopped=true;kind="Trapped";}
  attempts++;
  if(energy()!==before-1)throw Error("B29 energy conservation violated");
  if(p.code[12].value!==priorDelta){
    if(at!==22||kind!=="ExecutableConstPatched"||p.code[12].value!==-priorDelta)
      throw Error("mutation-rule change cannot be attributed to native PC22");
    edits.push([p.id,priorDelta,p.code[12].value]);
  }
  events.push({pid:p.id,pc:at,kind});
  return true;
 }
 function drain(){
  let n=0;while(step()){if(++n>220)throw Error("B29 bounded pool exceeded");}return n;
 }
 const initial=drain();if(initial!==2||energy()!==0)throw Error("B29 initial state mismatch");
 const windows=[];
 for(const packet of packets){
  const before=energy();pool+=packet;
  if(energy()!==before+packet)throw Error("external energy packet mismatch");
  windows.push({packet,paid:drain(),births:births.length,pool});
 }
 if(energy()+attempts!==2+packets.reduce((s,x)=>s+x,0))
  throw Error("B29 budget not conserved");
 const lineageGenotypes=processes.map(p=>[p.id,p.code[0].value,p.code[12].value]);
 return {name,packets,initial,windows,births,edits,lineageGenotypes,attempts,pool,
  energy:energy(),copies:events.filter(e=>e.kind==="GraphNodeCopied").length,
  graphEdits:events.filter(e=>e.kind==="GraphNodeEdited").length,
  traps:events.filter(e=>e.kind==="Trapped").length,
  final:processes.map(p=>({id:p.id,pc:p.pc,credits:p.credits,stopped:p.stopped,codeLength:p.code.length})),
  allFull23:processes.every(p=>p.code.length===23),
  graphObjects:graphs.size};
}
const packets=[157,157,156];
const cases=[run("FLIP",packets),run("NO_FLIP",packets),run("NO_FLUX",[]),run("COPY_DISABLED",packets)];
if(JSON.stringify(cases[0].births)!==JSON.stringify([[1,2],[1,3],[3,4]]))
 throw Error("B29 native sibling ancestry prediction falsified");
if(JSON.stringify(cases[0].edits)!==JSON.stringify([[1,1,-1],[1,-1,1],[3,-1,1]]))
 throw Error("B29 in-parent mutation-rule edits not independently demonstrated");
if(JSON.stringify(cases[0].lineageGenotypes)!==JSON.stringify([[1,156,1],[2,157,1],[3,155,1],[4,154,-1]]))
 throw Error("B29 inherited sibling genotypes not source-correct");
if(JSON.stringify(cases[1].births)!==JSON.stringify([[1,2],[1,3]])||cases[1].edits.length)
 throw Error("B29 no-flip matched control invalid");
if(cases[2].births.length||cases[3].births.length||cases[3].traps!==1)
 throw Error("B29 absence-of-compute/edits controls invalid");
if(!cases.every(c=>c.allFull23))
 throw Error("B29 failed to reproduce whole native executable");
process.stdout.write(JSON.stringify({
  level:"independent source-parsed restricted deterministic CIL VM, not native Rust verification",
  source:"cl0/crates/world/tests/cl1b_b29_mutation_rule_siblings.rs",
  experimentalWorlds:cases.length,cases
},null,2)+"\n");
