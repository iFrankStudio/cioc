// B34 independent native-source-parsed restricted CIL interpreter.
// Charged nonblocking resource outcome is CIL-visible, but the trial and
// fallback policy are investigator-authored. NOT native Rust validation.
import { readFileSync } from "node:fs";
const source=readFileSync(new URL("../../cl0/crates/world/tests/cl1b_b34_nonblocking_resource_feedback.rs",import.meta.url),"utf8");
const body=source.match(/fn founder\(arm:Arm\)->Vec<Op>\{([\s\S]*?)\n\}/)?.[1];
if(!body)throw Error("B34 native founder body missing");
const matchOps=part=>[...part.matchAll(/Op::([A-Za-z]+)\s*\{([^}]*)\}/g)].map(m=>{
 const o={kind:m[1]};
 for(const x of m[2].matchAll(/(\w+)\s*:\s*(-?\d+)/g))o[x[1]]=Number(x[2]);
 return o;
});
const staticPart=body.match(/if arm==Arm::StaticThree\{([\s\S]*?)\n \}/)?.[1];
const afterStatic=body.slice(body.lastIndexOf("\n vec!["));
const staticCode=matchOps(staticPart||"").slice(0,5);
const rest=matchOps(afterStatic);
if(staticCode.length!==5||rest.length!==11||rest[2].kind!=="TryDrawCompute"||
 rest[3].kind!=="Const")throw Error("B34 Rust source ABI or policy geometry drift");
function program(arm){
 if(arm==="StaticThree")return staticCode.map(o=>({...o}));
 const clone=rest.map(x=>({...x}));
 return [...clone.slice(0,2),clone[arm==="Try"?2:3],...clone.slice(4)];
}
function run(arm,packet){
 const code=program(arm),r=[0,0,0,0];let pc=0,local=6,pool=packet,attempts=0,writes=0,edits=0;
 const trials=[];
 const reg=i=>{if(!Number.isInteger(i)||i<0||i>=4)throw Error("invalid register");return r[i]};
 function step(){
  const op=code[pc];
  const canPool=local===0&&op.kind==="DrawCompute"&&pool>=reg(op.amount)+1;
  if(local===0&&!canPool)return false;
  const before=local+pool,at=pc;
  if(canPool)pool--;else local--;
  attempts++;
  switch(op.kind){
   case"Const":r[op.dst]=op.value;pc=op.next;break;
   case"TryDrawCompute":{
    const req=reg(op.amount),available=pool,success=available>=req;
    if(success){pool-=req;local+=req;}
    r[op.dst]=success?1:0;pc=op.next;
    trials.push({request:req,available,success});break;
   }
   case"Branch":pc=reg(op.test)?op.nonzero:op.zero;break;
   case"PatchConst":{
    const idx=reg(op.node);
    if(code[idx]?.kind!=="Const")throw Error("invalid self source patch");
    code[idx].value=reg(op.value);edits++;pc=op.next;break;
   }
   case"DrawCompute":{
    const amount=reg(op.amount);
    if(pool>=amount){pool-=amount;local+=amount;pc=op.next;}
    break;
   }
   case"Write":{
    if(reg(op.address)!==9||reg(op.value)!==1)throw Error("not native real work");
    pc=op.next;writes++;break;
   }
   default:throw Error("unrecognized CIL opcode "+op.kind);
  }
  if(before!==local+pool+1)throw Error("B34 compute created from nowhere");
  return true;
 }
 while(step()){if(attempts>70)throw Error("finite packet unbounded");}
 if(attempts+local+pool!==6+packet)throw Error("B34 global energy conservation drift");
 return {arm,packet,attempts,writes,edits,trials,sourceRequest:code[0].value,pc,local,pool};
}
const cases=[];
for(const packet of [0,4,5,6])for(const arm of ["Try","ForcedFail","StaticThree"]){
 const result=run(arm,packet);cases.push(result);
 if(result.sourceRequest!==(arm==="Try"&&packet>=5?5:3))
  throw Error("native B34 code-patch outcome disagrees with source model");
 if(arm==="Try"&&(result.trials.length!==1||result.trials[0].success!==(packet>=5)))
  throw Error("B34 real nonblocking draw outcome not recorded");
}
if(cases.length!==12)throw Error("B34 world denominator mismatch");
process.stdout.write(JSON.stringify({
 evidence:"independent CIL source-parsed interpreter (NOT native Rust)",
 source:"cl0/crates/world/tests/cl1b_b34_nonblocking_resource_feedback.rs",
 worlds:cases.length,cases
},null,2)+"\n");
