// B37 independent Rust-source-parsed deterministic CIL interpreter.
// Measures shared neutral compute wasted on an authored nonservice loop,
// with role IDs swapped. NO native Rust execution or organism fitness claim.
import { readFileSync } from "node:fs";
const source=readFileSync(new URL("../../cl0/crates/world/tests/cl1b_b37_direct_pool_contention.rs",import.meta.url),"utf8");
function parse(name){
 const body=source.match(new RegExp("fn "+name+"\\(\\)->Vec<Op>\\{([\\s\\S]*?)\\n\\}"))?.[1];
 if(!body)throw Error("B37 canonical native source missing "+name);
 return [...body.matchAll(/Op::([A-Za-z]+)\s*\{([^}]*)\}/g)].map(m=>{
  const o={kind:m[1]};
  for(const part of m[2].matchAll(/(\w+)\s*:\s*(-?\d+)/g))
   o[part[1]]=Number(part[2]);
  return o;
 });
}
const worker=parse("worker"),burner=parse("burner");
if(worker.length!==3||burner.length!==2||
 worker[2].kind!=="Write"||burner[1].kind!=="Branch")
 throw Error("B37 native source graph unexpectedly changed");
function run(composition,supply,direct){
 const variants={
  SoloWorker:[["Worker"]],
  BurnerFirst:[["Burner"],["Worker"]],
  WorkerFirst:[["Worker"],["Burner"]],
  DualWorker:[["Worker"],["Worker"]]
 };
 const assigned=variants[composition];if(!assigned)throw Error("B37 invalid composition");
 const actors=assigned.map(([type],index)=>({id:index+1,type,
  code:(type==="Worker"?worker:burner).map(op=>({...op})),
  r:[0,0,0,0],pc:0,credits:0,stopped:false}));
 let cursor=null,pool=0,steps=0,productive=0,workerSteps=0,burnerSteps=0,poolPaid=0;
 const windowResults=[];
 for(const packet of supply){
  const old=pool;pool+=packet;if(pool!==old+packet)throw Error("B37 resource intervention");
  let executed=0;
  while(pool>0&&direct){
   const eligible=actors.filter(p=>!p.stopped);
   if(eligible.length===0)break;
   const p=eligible.find(x=>cursor===null||x.id>cursor)??eligible[0];
   cursor=p.id;
   const op=p.code[p.pc],before=pool;pool--;steps++;executed++;poolPaid++;
   switch(op.kind){
    case"Const":p.r[op.dst]=op.value;p.pc=op.next;break;
    case"Branch":p.pc=p.r[op.test]!==0?op.nonzero:op.zero;break;
    case"Write":
     if(p.r[op.address]!==9||p.r[op.value]!==1)throw Error("B37 not genuine work");
     if(p.type!=="Worker")throw Error("B37 burner emitted work");
     productive++;p.pc=op.next;break;
    default:throw Error("B37 unknown CIL source opcode "+op.kind);
   }
   if(p.type==="Worker")workerSteps++;else burnerSteps++;
   if(pool!==before-1)throw Error("B37 native step minted compute");
   if(executed>18)throw Error("B37 finite resource overspent");
  }
  windowResults.push({packet,executed,productive,pool});
 }
 if(steps+pool!==supply.reduce((a,b)=>a+b,0))
  throw Error("B37 global resource conservation violated");
 return {composition,supply,direct,steps,productive,workerSteps,burnerSteps,
   poolPaid,pool,windowResults};
}
const schedules=[[3,6,9],[6,6,6],[1,2,3,4,5,3]];
const compositions=["SoloWorker","BurnerFirst","WorkerFirst","DualWorker"];
const cases=[];
for(const supply of schedules)for(const composition of compositions){
 const result=run(composition,supply,true);cases.push(result);
 if(result.steps!==18||result.poolPaid!==18||result.pool!==0)
  throw Error("B37 every direct law world must spend exactly18 native credits");
 if(result.productive!==(composition==="BurnerFirst"||composition==="WorkerFirst"?3:6))
  throw Error("B37 true worker output diluted by native CIL burner differs from prereg");
 if(result.productive*3!==result.workerSteps)
  throw Error("B37 useful event requires precisely three actual worker opcodes");
}
for(const composition of ["SoloWorker","BurnerFirst"]){
 const x=run(composition,[18],false);
 if(x.steps!==0||x.pool!==18)throw Error("B37 Draw-only world unexpectedly ran nonDraw opcode");
 cases.push(x);
}
if(cases.length!==14)throw Error("B37 total world denominator must be14");
process.stdout.write(JSON.stringify({
 evidence:"independent source-parsed generic CIL world model; NOT native Rust",
 source:"cl0/crates/world/tests/cl1b_b37_direct_pool_contention.rs",
 worlds:cases.length,cases
},null,2)+"\n");
