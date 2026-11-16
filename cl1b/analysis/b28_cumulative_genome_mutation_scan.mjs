// CL-1B B28: independent source-parsed CIL VM for multigeneration
// cumulative inherited request mutation. NO native Rust execution and NO
// spontaneous evolutionary interpretation. Full genotype, costs and traps.
import { readFileSync } from "node:fs";
const source=readFileSync(new URL("../../cl0/crates/world/tests/cl1b_b28_cumulative_genome_mutation.rs",import.meta.url),"utf8");
const body=source.match(/fn founder\(initial: i64,delta: i64\) -> Vec<Op> \{[\s\S]*?vec!\[([\s\S]*?)\n    \]/)?.[1];
if(!body)throw Error("B28 frozen Rust founder unavailable");
const base=[...body.matchAll(/Op::([A-Za-z]+)\s*\{([^}]*)\}/g)].map(m=>{
 const op={kind:m[1]};
 for(const f of m[2].matchAll(/(\w+)\s*:\s*(-?\d+|initial|delta)/g))
  op[f[1]]=["initial","delta"].includes(f[2])?f[2]:Number(f[2]);
 return op;
});
if(base.length!==19||base[4].kind!=="GraphCopyNode"||
  base[14].kind!=="GraphSetConst"||base[16].kind!=="GraphSetConst"||
  base[18].kind!=="GraphLaunch")throw Error("B28 native 19-node ABI drift");
const genome=(q,delta)=>base.map(x=>{
 const v={...x};
 for(const k of Object.keys(v))if(v[k]==="initial")v[k]=q;
  else if(v[k]==="delta")v[k]=delta;
 return v;
});
const sameGenome=(a,b)=>JSON.stringify(a)===JSON.stringify(b);
function simulate(arm,supply){
 const delta=arm==="MINUS_ONE"?-1:arm==="IDENTICAL"?0:1;
 const suppress=arm==="COPY_DISABLED";
 const processes=[{id:1,code:genome(127,delta),r:[0,0,0,0],pc:0,credits:2,stopped:false}];
 const graphs=new Map(),events=[];let pool=0,cursor=null,nextGraph=1,nextPid=2,attempts=0;
 const inventory=()=>pool+processes.reduce((sum,p)=>sum+p.credits,0);
 const reg=(p,i)=>{if(!Number.isInteger(i)||i<0||i>=4)throw Error("bad register");return p.r[i];};
 const edge=(p,n)=>{if(!Number.isInteger(n)||n<0||n>=p.code.length)throw Error("bad PC");return n;};
 function step(){
  const ready=processes.filter(p=>!p.stopped&&(p.credits>0||
    (p.code[p.pc].kind==="DrawCompute"&&pool>=reg(p,p.code[p.pc].amount)+1)));
  if(!ready.length)return false;
  const p=ready.find(x=>cursor===null||x.id>cursor)??ready[0];
  cursor=p.id;
  const prev=inventory(),at=p.pc,o=p.code[at];let kind="Executed";
  if(p.credits>0)p.credits--;else pool--;
  try {
   switch(o.kind) {
    case "Const":p.r[o.dst]=o.value;p.pc=edge(p,o.next);break;
    case "DrawCompute":{
      const req=reg(p,o.amount);
      if(pool<req){kind="DrawDeferred";break;}
      pool-=req;p.credits+=req;p.pc=edge(p,o.next);kind="DrawCompute";break;
    }
    case "GraphNew":{const id=nextGraph++;graphs.set(id,[]);
      p.r[o.dst]=id;p.pc=edge(p,o.next);kind="GraphCreated";break;}
    case "GraphCopyNode":{
      const id=reg(p,o.graph),idx=reg(p,o.source),arr=graphs.get(id);
      if(!arr||idx<0||idx>=p.code.length)throw Error("bad graph node source");
      if(!suppress){arr.push({...p.code[idx]});kind="GraphNodeCopied";}
      else kind="NativeCodeEditSuppressed";
      p.r[o.dst]=arr.length-1;p.pc=edge(p,o.next);break;
    }
    case "Add":p.r[o.dst]=reg(p,o.lhs)+reg(p,o.rhs);p.pc=edge(p,o.next);break;
    case "Sub":p.r[o.dst]=reg(p,o.lhs)-reg(p,o.rhs);p.pc=edge(p,o.next);break;
    case "Branch":p.pc=edge(p,reg(p,o.test)!==0?o.nonzero:o.zero);break;
    case "GraphSetConst":{
      const arr=graphs.get(reg(p,o.graph)),idx=reg(p,o.node),value=reg(p,o.value);
      if(!arr||idx<0||idx>=arr.length||arr[idx].kind!=="Const")throw Error("invalid CIL graph const edit");
      if(!suppress){arr[idx].value=value;kind="GraphNodeEdited";}
      else kind="NativeCodeEditSuppressed";
      p.pc=edge(p,o.next);break;
    }
    case "GraphLaunch":{
      const id=reg(p,o.graph),amount=reg(p,o.credits),childCode=graphs.get(id);
      if(!childCode||!childCode.length||p.credits<amount)throw Error("invalid child launch / credit shortage");
      graphs.delete(id);p.credits-=amount;
      const childId=nextPid++;p.r[o.dst]=childId;p.pc=edge(p,o.next);
      processes.push({id:childId,code:childCode,pc:0,r:[0,0,0,0],credits:amount,stopped:false});
      kind="GraphLaunched";break;
    }
    default:throw Error("unsupported B28 native opcode "+o.kind);
   }
  }catch(err){p.stopped=true;kind="Trapped";}
  attempts++;
  if(inventory()!==prev-1)throw Error("B28 minted compute");
  events.push({process:p.id,pc:at,kind});
  return true;
 }
 function drain(){let n=0;while(step()){n++;if(n>=180)throw Error("bounded packet overrun");}return n;}
 const startup=drain();if(startup!==2||inventory()!==0)throw Error("B28 start phase changed");
 const windows=[];
 for(const packet of supply){
  const prev=inventory();pool+=packet;if(inventory()!==prev+packet)throw Error("world supply accounting");
  const work=drain();windows.push({packet,work,births:events.filter(e=>e.kind==="GraphLaunched").length,pool,energy:inventory()});
 }
 const births=events.filter(e=>e.kind==="GraphLaunched").map((e,i)=>[e.process,i+2]);
 const errors=[];
 for(let i=0;i<processes.length;i++){
  const p=processes[i],request=127+delta*i;
  if(!sameGenome(p.code,genome(request,delta)))errors.push({id:p.id,expect:request,actual:p.code[0]?.value});
 }
 const made=events.filter(e=>e.kind==="GraphCreated").length;
 const copies=events.filter(e=>e.kind==="GraphNodeCopied").length;
 const edits=events.filter(e=>e.kind==="GraphNodeEdited").length;
 const traps=events.filter(e=>e.kind==="Trapped").length;
 if(attempts+inventory()!==2+supply.reduce((s,k)=>s+k,0))throw Error("B28 final energy drift");
 return {arm,delta,supply,startup,windows,births,genotypes:processes.map(p=>p.code[0].value),
  genotypeMismatches:errors,attempts,pool,inventory:inventory(),made,copies,edits,traps,
  processes:processes.map(p=>({id:p.id,pc:p.pc,credits:p.credits,stopped:p.stopped,programSize:p.code.length})),
  graphObjects:graphs.size};
}
const sequences={
 "INCREASING":[128,129,130],
 "FLAT":[128,128,128],
 "DECREASING":[130,129,128]
};
const cases=[];
for(const arm of ["PLUS_ONE","IDENTICAL","MINUS_ONE"])for(const [name,supply] of Object.entries(sequences)){
 cases.push({...simulate(arm,supply),scheduleName:name});
}
cases.push(simulate("NO_INFLUX",[]));
cases.push(simulate("COPY_DISABLED",sequences.INCREASING));
if(cases.length!==11)throw Error("B28 wrong frozen denominator");
for(const c of cases){
 if(c.genotypeMismatches.length)throw Error("B28 cumulative source genome not inherited");
 if(c.copies%19!==0||c.edits%2!==0)throw Error("B28 partial graph edits");
}
if(JSON.stringify(cases[0].genotypes)!==JSON.stringify([127,128,129,130]))
 throw Error("B28 increased native request not inherited cumulatively");
if(JSON.stringify(cases[1].genotypes)!==JSON.stringify([127,128,129]))
 throw Error("B28 flat supply delay not observed");
if(JSON.stringify(cases[4].genotypes)!==JSON.stringify([127,127,127,127]))
 throw Error("B28 same-genotype control lost reproducible lineage");
if(JSON.stringify(cases[6].genotypes)!==JSON.stringify([127,126])||cases[6].traps!==1)
 throw Error("B28 low-energy deleterious mutant did not strand");
if(cases[10].births.length!==0||cases[10].traps!==1)
 throw Error("B28 edit-disabled native control ineffective");
process.stdout.write(JSON.stringify({
 evidence:"independent source-parsed restricted CIL VM; NOT native Rust execution",
 source:"cl0/crates/world/tests/cl1b_b28_cumulative_genome_mutation.rs",
 worlds:cases.length,cases
},null,2)+"\n");
