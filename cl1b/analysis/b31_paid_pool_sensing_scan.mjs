// B31 independent source-parsed CIL subset VM for paid neutral compute-pool
// sensor and conditional executable mutation. NOT a native Rust result.
import { readFileSync } from "node:fs";
const text=readFileSync(new URL("../../cl0/crates/world/tests/cl1b_b31_pool_sensing.rs",import.meta.url),"utf8");
const body=text.match(/fn founder\(arm:Arm\)->Vec<Op>\{([\s\S]*?)\n\}/)?.[1];
if(!body)throw Error("B31 Rust native founder not found");
const branches=[...body.matchAll(/Arm::(Sense|BlindThree|BlindFour)=>Op::(SenseComputePool|Const)\{([^}]*)\}/g)];
const all=[...body.matchAll(/Op::([A-Za-z]+)\s*\{([^}]*)\}/g)].map(m=>{
 const op={kind:m[1]};
 for(const pair of m[2].matchAll(/(\w+)\s*:\s*(-?\d+)/g))op[pair[1]]=Number(pair[2]);
 return op;
});
if(branches.length!==3||all.length!==15||
   all[0].kind!=="SenseComputePool"||all[3].kind!=="Const"||
   all[9].kind!=="PatchConst"||all[11].kind!=="DrawCompute")throw Error("B31 CIL source ABI drift");
const from=(arm)=>{
 const found=branches.find(x=>x[1]===arm);
 if(!found)throw Error("B31 missing "+arm);
 const op={kind:found[2]};
 for(const pair of found[3].matchAll(/(\w+)\s*:\s*(-?\d+)/g))op[pair[1]]=Number(pair[2]);
 return [op,...all.slice(3)];
};
function trial(arm,packet,initial=8){
 const code=from(arm),r=[0,0,0,0];let pc=0,credits=initial,pool=packet,paid=0,services=0,patch=0;
 const sensed=[],events=[];
 const reg=i=>{if(!Number.isInteger(i)||i<0||i>=4)throw Error("invalid reg");return r[i];};
 function step(){
  const op=code[pc],local=credits>0;
  const paidFromPool=!local&&op.kind==="DrawCompute"&&pool>=reg(op.amount)+1;
  if(!local&&!paidFromPool)return false;
  const before=credits+pool,at=pc;
  if(local)credits--;else pool--;
  paid++;
  switch(op.kind){
   case"SenseComputePool":r[op.dst]=pool;sensed.push(pool);pc=op.next;break;
   case"Const":r[op.dst]=op.value;pc=op.next;break;
   case"Sub":r[op.dst]=reg(op.lhs)-reg(op.rhs);pc=op.next;break;
   case"Branch":pc=reg(op.test)!==0?op.nonzero:op.zero;break;
   case"PatchConst":{
    const idx=reg(op.node);if(code[idx]?.kind!=="Const")throw Error("invalid source literal target");
    code[idx].value=reg(op.value);patch++;pc=op.next;break;
   }
   case"DrawCompute":{
    const amount=reg(op.amount);
    if(pool>=amount){pool-=amount;credits+=amount;pc=op.next;}
    // A shortage consumes its charge and preserves the Draw PC.
    break;
   }
   case"Write":if(reg(op.address)!==9||reg(op.value)!==1)throw Error("not real service");
    services++;pc=op.next;break;
   default:throw Error("unknown B31 CIL instruction "+op.kind);
  }
  if(before!==credits+pool+1)throw Error("compute inventory not conserved");
  events.push({pc:at,kind:op.kind,remainingPool:pool,local:credits});
  return true;
 }
 while(step()){if(paid>60)throw Error("unbounded computation");}
 return {arm,packet,initial,sensed,paid,services,patch,pc,credits,pool,
   literal:code[8].value,eventCount:events.length};
}
const cases=[];
for(const packet of [0,4,5,6])for(const arm of ["Sense","BlindThree","BlindFour"]){
 const x=trial(arm,packet);cases.push(x);
 if(x.patch!==1||x.pc!==9||x.credits!==0)throw Error("B31 source mutation or blocked PC changed");
 if(x.literal!==(arm==="BlindThree"||arm==="Sense"&&packet===4?3:4))
  throw Error("B31 true native source variant did not match sensed packet");
 if(x.services!==Number(packet>=4&&(arm!=="BlindFour"||packet>=5)))
  throw Error("B31 work service mismatch");
 if(arm==="Sense"&&(x.sensed.length!==1||x.sensed[0]!==packet))
  throw Error("B31 paid environment observation failed");
 if(arm!=="Sense"&&x.sensed.length)throw Error("blind founder accessed pool");
}
const dead=trial("Sense",100,0);
if(dead.paid!==0||dead.services!==0||dead.pc!==0||dead.sensed.length)
 throw Error("B31 sensor bypassed zero-credit Draw-only physics");
if(cases.length!==12)throw Error("B31 wrong primary trial count");
process.stdout.write(JSON.stringify({
 evidence:"independent source-parsed deterministic CIL interpreter, NOT native Rust",
 source:"cl0/crates/world/tests/cl1b_b31_pool_sensing.rs",
 worlds:cases.length,zeroCreditDeadlock:dead,cases
},null,2)+"\n");
