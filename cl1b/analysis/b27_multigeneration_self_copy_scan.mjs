// CL-1B B27: source-parsed restricted deterministic CIL VM, no Rust execution.
// Audits exact full-program copy, child genotype equality, charged attempts,
// lineage ancestry, energy supply and copy-disabled negative control.
import { readFileSync } from "node:fs";
const source=readFileSync(new URL("../../cl0/crates/world/tests/cl1b_b27_multigeneration_self_copy.rs",import.meta.url),"utf8");
const body=source.match(/fn founder\(\) -> Vec<Op> \{[\s\S]*?vec!\[([\s\S]*?)\n    \]/)?.[1];
if(!body)throw Error("B27 native founder not found");
const genome=[...body.matchAll(/Op::([A-Za-z]+)\s*\{([^}]*)\}/g)].map(m=>{
 const op={kind:m[1]};
 for(const v of m[2].matchAll(/(\w+)\s*:\s*(-?\d+)/g))op[v[1]]=Number(v[2]);
 return op;
});
if(genome.length!==12||genome[4].kind!=="GraphCopyNode"||
   genome[9].kind!=="Branch"||genome[11].kind!=="GraphLaunch")
 throw Error("B27 genome source ABI changed");
function equalGenome(a,b){return JSON.stringify(a)===JSON.stringify(b);}
function run(arm,packets){
 const disabled=arm==="NATIVE_COPY_DISABLED";
 const processes=[{id:1,code:genome.map(x=>({...x})),r:[0,0,0,0],pc:0,credits:2,stopped:false}];
 const graphObjects=new Map(),events=[],externals=["load1"];
 let nextGraph=1,nextPid=2,cursor=null,pool=0,attempts=0;
 const inventory=()=>pool+processes.reduce((sum,p)=>sum+p.credits,0);
 const reg=(p,i)=>{if(!Number.isInteger(i)||i<0||i>=4)throw Error("invalid register");return p.r[i];};
 const pcnext=(p,i)=>{if(!Number.isInteger(i)||i<0||i>=p.code.length)throw Error("invalid PC");return i;};
 function step(){
  const eligible=processes.filter(p=>!p.stopped&&(p.credits>0||(
   p.code[p.pc].kind==="DrawCompute"&&pool>=reg(p,p.code[p.pc].amount)+1
  )));
  if(!eligible.length)return false;
  const p=eligible.find(x=>cursor===null||x.id>cursor)??eligible[0];cursor=p.id;
  const before=inventory(),at=p.pc,o=p.code[at];let kind="Executed";
  if(p.credits>0)p.credits--;else pool--;
  try{
   switch(o.kind){
    case"Const":p.r[o.dst]=o.value;p.pc=pcnext(p,o.next);break;
    case"DrawCompute":{
     const request=reg(p,o.amount);
     if(pool<request){kind="DrawDeferred";break;}
     pool-=request;p.credits+=request;p.pc=pcnext(p,o.next);kind="DrawCompute";break;
    }
    case"GraphNew":{
     const id=nextGraph++;graphObjects.set(id,[]);
     p.r[o.dst]=id;p.pc=pcnext(p,o.next);kind="GraphCreated";break;
    }
    case"GraphCopyNode":{
     const id=reg(p,o.graph),node=reg(p,o.source),arr=graphObjects.get(id);
     if(!arr||node<0||node>=p.code.length)throw Error("invalid graph clone");
     if(!disabled){arr.push({...p.code[node]});kind="GraphNodeCopied";}
     else kind="NativeCodeEditSuppressed";
     p.r[o.dst]=arr.length-1;p.pc=pcnext(p,o.next);break;
    }
    case"Add":p.r[o.dst]=reg(p,o.lhs)+reg(p,o.rhs);p.pc=pcnext(p,o.next);break;
    case"Sub":p.r[o.dst]=reg(p,o.lhs)-reg(p,o.rhs);p.pc=pcnext(p,o.next);break;
    case"Branch":p.pc=pcnext(p,reg(p,o.test)!==0?o.nonzero:o.zero);break;
    case"GraphLaunch":{
     const id=reg(p,o.graph),amount=reg(p,o.credits),code=graphObjects.get(id);
     if(!code||!code.length||p.credits<amount)throw Error("cannot launch empty or unfunded graph");
     graphObjects.delete(id);p.credits-=amount;
     const childId=nextPid++;
     p.r[o.dst]=childId;p.pc=pcnext(p,o.next);
     processes.push({id:childId,code,r:[0,0,0,0],pc:0,credits:amount,stopped:false});
     kind="GraphLaunched";break;
    }
    default:throw Error("B27 unsupported native opcode "+o.kind);
   }
  }catch(err){p.stopped=true;kind="Trapped"; }
  attempts++;
  if(inventory()!==before-1)throw Error("CIL created compute energy");
  events.push({process:p.id,pc:at,kind});
  return true;
 }
 const drain=()=>{let n=0;while(step()){n++;if(n>=110)throw Error("finite window exceeded");}return n;};
 const bootstrap=drain();
 if(bootstrap!==2||inventory()!==0||processes.length!==1)throw Error("B27 wrong initial debit");
 const windows=[];
 for(const packet of packets){
  const before=inventory();pool+=packet;externals.push("pool+"+packet);
  if(inventory()!==before+packet)throw Error("external supply mismatch");
  const paid=drain();
  windows.push({packet,paid,births:events.filter(e=>e.kind==="GraphLaunched").length,energy:inventory(),pool});
 }
 const lineage=events.filter(e=>e.kind==="GraphLaunched").map((e,i)=>({parent:e.process,child:i+2}));
 const copies=events.filter(e=>e.kind==="GraphNodeCopied");
 const originals=processes.filter(p=>!equalGenome(p.code,genome));
 return {arm,packets,bootstrap,windows,
  attempts,births:lineage.length,lineage,copies:copies.length,created:events.filter(e=>e.kind==="GraphCreated").length,
  suppressed:events.filter(e=>e.kind==="NativeCodeEditSuppressed").length,
  traps:events.filter(e=>e.kind==="Trapped").length,genomeMismatches:originals.length,
  pool,inventory:inventory(),processes:processes.map(p=>({id:p.id,pc:p.pc,credits:p.credits,stopped:p.stopped,codeNodes:p.code.length})),
  graphCount:graphObjects.size,externalEvents:externals.length};
}
const frozen=[
 ["THREE_GENERATIONS",[79,79,79],3],
 ["SPLIT_THRESHOLD",[78,1,79,79],3],
 ["SINGLE_PACKET",[79],1],
 ["NO_INFLUX",[],0],
 ["NATIVE_COPY_DISABLED",[79,79,79],0]
];
const worlds=frozen.map(([arm,packets,expected])=>{
 const o=run(arm,packets);if(o.births!==expected)throw Error("B27 "+arm+" unexpected number of births");
 if(arm!=="NATIVE_COPY_DISABLED"){
  if(o.genomeMismatches!==0||o.copies!==expected*12||o.created!==expected||o.traps)
   throw Error("B27 native heredity model failed "+arm);
  if(o.lineage.some((e,i)=>e.parent!==i+1||e.child!==i+2))
   throw Error("B27 not a valid ancestor→descendant chain");
 }else if(o.births!==0||o.copies!==0||o.traps!==1)throw Error("B27 copy control failed");
 return o;
});
process.stdout.write(JSON.stringify({
 evidence:"source-parsed restricted CIL world model, NOT native Rust execution",
 source:"cl0/crates/world/tests/cl1b_b27_multigeneration_self_copy.rs",
 worlds:worlds.length,cases:worlds
},null,2)+"\n");
