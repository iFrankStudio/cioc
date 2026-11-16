// B35 deterministic independent parser/interpreter of native Rust CIL source.
// No native Rust execution, no oracle score or discovered fitness.
import { readFileSync } from "node:fs";
const source=readFileSync(new URL("../../cl0/crates/world/tests/cl1b_b35_multitrial_retention.rs",import.meta.url),"utf8");
const founderBody=source.match(/fn founder\(arm:Arm\)->Vec<Op> \{([\s\S]*?)\n\}/)?.[1];
if(!founderBody)throw Error("B35 native founder Rust body missing");
function parse(s){
 return [...s.matchAll(/Op::([A-Za-z]+)\s*\{([^}]*)\}/g)].map(m=>{
  const o={kind:m[1]};
  for(const k of m[2].matchAll(/(\w+)\s*:\s*(-?\d+)/g))o[k[1]]=Number(k[2]);
  return o;
 });
}
const first=founderBody.match(/if arm==Arm::StaticThree \{([\s\S]*?)\n \}/)?.[1];
const staticCode=parse(first??"").slice(0,5);
const adaptiveLayout=parse(founderBody.slice(founderBody.lastIndexOf("\n vec![")));
if(staticCode.length!==5||adaptiveLayout.length!==13||adaptiveLayout[3].kind!=="TryDrawCompute"||
   adaptiveLayout[4].kind!=="Const"||adaptiveLayout[7].kind!=="PatchConst"||
   adaptiveLayout[12].kind!=="DrawCompute")throw Error("B35 source program layout changed");
function make(arm){
 if(arm==="StaticThree")return staticCode.map(o=>({...o}));
 const chosen=arm==="Adaptive"?3:4;
 return [...adaptiveLayout.slice(0,3),adaptiveLayout[chosen],...adaptiveLayout.slice(5)].map(o=>({...o}));
}
function run(arm,schedule){
 const code=make(arm),r=[0,0,0,0];
 let pc=0,local=14,pool=0,charged=0,writes=0,edits=0,stopped=false,traps=0;
 const trials=[],perPacketWrites=[],perPacketPaid=[],attempts=[];
 const reg=i=>{if(!Number.isInteger(i)||i<0||i>=4)throw Error("B35 invalid CIL register");return r[i];};
 function step(){
  if(stopped)return false;
  const op=code[pc],zeroLocal=local===0;
  const poolPaid=zeroLocal&&op.kind==="DrawCompute"&&pool>=reg(op.amount)+1;
  if(zeroLocal&&!poolPaid)return false;
  const old=local+pool,at=pc;
  if(poolPaid)pool--;else local--;
  charged++;
  let kind=op.kind;
  try{
   switch(op.kind){
    case"Const":r[op.dst]=op.value;pc=op.next;break;
    case"Sub":r[op.dst]=reg(op.lhs)-reg(op.rhs);pc=op.next;break;
    case"Branch":pc=reg(op.test)!==0?op.nonzero:op.zero;break;
    case"TryDrawCompute":{
     const req=reg(op.amount);
     if(req<0)throw Error("negative request");
     const available=pool,success=available>=req;
     if(success){pool-=req;local+=req;}
     r[op.dst]=success?1:0;pc=op.next;
     trials.push({req,available,success});break;
    }
    case"PatchConst":{
     const node=reg(op.node);
     if(code[node]?.kind!=="Const")throw Error("cannot patch non-Const");
     code[node].value=reg(op.value);pc=op.next;edits++;break;
    }
    case"Write":{
     if(reg(op.address)!==9||reg(op.value)!==1)throw Error("invalid real service");
     writes++;pc=op.next;break;
    }
    case"DrawCompute":{
     const req=reg(op.amount);
     if(req<0)throw Error("negative draw");
     if(pool>=req){pool-=req;local+=req;pc=op.next;}
     else kind="DrawDeferred";
     break;
    }
    default:throw Error("unsupported B35 native CIL opcode "+op.kind);
   }
  }catch(e){stopped=true;traps++;kind="Trapped";}
  if(old!==local+pool+1)throw Error("B35 compute minting");
  attempts.push({pc:at,kind,pool,local});
  return true;
 }
 for(const packet of schedule){
  const before=local+pool;pool+=packet;
  if(before+packet!==local+pool)throw Error("B35 exogenous packet failed accounting");
  let work=0;
  while(step()){if(++work>=160)throw Error("B35 unbounded finite pool");}
  perPacketPaid.push(work);perPacketWrites.push(writes);
 }
 if(charged+pool+local!==14+schedule.reduce((a,b)=>a+b,0))
  throw Error("B35 total budget incorrect");
 return {arm,schedule,totalWrites:writes,perPacketWrites,perPacketPaid,charged,
  edits,trials,sourceRequest:code[0].value,finalPC:pc,local,pool,stopped,traps};
}
const schedules=[[4,5,5],[4,12,12],[5,5,5],[6,8,12]];
const cases=[];
for(const schedule of schedules)for(const arm of ["Adaptive","ForcedFail","StaticThree"])
 cases.push(run(arm,schedule));
if(cases.length!==12)throw Error("B35 wrong world denominator");
for(let i=0;i<4;i++){
 const [adaptive,forced,static3]=cases.slice(i*3,i*3+3);
 if(static3.totalWrites<adaptive.totalWrites)
  throw Error("B35 fixed baseline falsified in source-shaped model; retain, don't rewrite");
 if(adaptive.edits>adaptive.trials.filter(t=>!t.success).length)
  throw Error("B35 native self-edit not caused by an actual failed resource try");
 if(forced.trials.length||static3.trials.length)throw Error("B35 no-resource-info control read results");
}
process.stdout.write(JSON.stringify({
 evidence:"independent source-parsed CIL interpreter; NOT native Rust verification",
 source:"cl0/crates/world/tests/cl1b_b35_multitrial_retention.rs",
 worlds:cases.length,cases
},null,2)+"\n");
