// B33 independently parses canonical B31 native CIL founder, then checks
// the same 24 worlds with an explicit source-step-indexed neutral pool packet.
// The interpreter is independent of Rust. It does NOT prove learning.
import { readFileSync } from "node:fs";
const source=readFileSync(new URL("../../cl0/crates/world/tests/cl1b_b31_pool_sensing.rs",import.meta.url),"utf8");
const body=source.match(/fn founder\(arm:Arm\)->Vec<Op>\{([\s\S]*?)\n\}/)?.[1];
if(!body)throw Error("B33: canonical native B31 founder missing");
const branches=[...body.matchAll(/Arm::(Sense|BlindThree|BlindFour)=>Op::(SenseComputePool|Const)\{([^}]*)\}/g)];
const codeNodes=[...body.matchAll(/Op::([A-Za-z]+)\s*\{([^}]*)\}/g)].map(m=>{
 const op={kind:m[1]};
 for(const c of m[2].matchAll(/(\w+)\s*:\s*(-?\d+)/g))op[c[1]]=Number(c[2]);
 return op;
});
if(codeNodes.length!==15||branches.length!==3||codeNodes[9].kind!=="PatchConst"||
 codeNodes[11].kind!=="DrawCompute")throw Error("B33 B31 founder ABI changed");
function founder(arm){
 const alt=branches.find(x=>x[1]===arm);
 const first={kind:alt[2]};
 for(const c of alt[3].matchAll(/(\w+)\s*:\s*(-?\d+)/g))first[c[1]]=Number(c[2]);
 return [first,...codeNodes.slice(3).map(o=>({...o}))];
}
function simulate(arm,arrival,packet){
 const code=founder(arm),regs=[0,0,0,0];
 let pool=0,local=8,pc=0,attempts=0,services=0,edits=0;
 const observations=[],events=[];
 const reg=i=>{if(!Number.isInteger(i)||i<0||i>=4)throw Error("B33 register OOB");return regs[i];};
 const pre={EARLY:0,AFTER_SENSE:1,AFTER_COMMIT:8}[arrival];
 if(pre===undefined)throw Error("invalid B33 arrival");
 function step(){
  const op=code[pc];
  const poolPaid=local===0&&op.kind==="DrawCompute"&&pool>=reg(op.amount)+1;
  if(local===0&&!poolPaid)return false;
  const before=local+pool;
  if(poolPaid)pool--;else local--;
  const at=pc;
  attempts++;
  switch(op.kind){
   case"SenseComputePool":regs[op.dst]=pool;observations.push(pool);pc=op.next;break;
   case"Const":regs[op.dst]=op.value;pc=op.next;break;
   case"Sub":regs[op.dst]=reg(op.lhs)-reg(op.rhs);pc=op.next;break;
   case"Branch":pc=reg(op.test)!==0?op.nonzero:op.zero;break;
   case"PatchConst":{
    const node=reg(op.node);
    if(code[node].kind!=="Const")throw Error("bad native patch");
    code[node].value=reg(op.value);edits++;pc=op.next;break;
   }
   case"DrawCompute":{
    const amount=reg(op.amount);
    if(pool>=amount){pool-=amount;local+=amount;pc=op.next;}
    break;
   }
   case"Write":if(reg(op.address)!==9||reg(op.value)!==1)throw Error("not a native service");
    services++;pc=op.next;break;
   default:throw Error("unknown native opcode "+op.kind);
  }
  if(local+pool!==before-1)throw Error("B33 computation not conserved");
  events.push({pc:at,kind:op.kind});
  return true;
 }
 for(let i=0;i<pre;i++)if(!step())throw Error("B33 unexpectedly blocked before packet");
 if(arrival==="AFTER_SENSE"&&pc!==1)throw Error("native sensor step not paid");
 if(arrival==="AFTER_COMMIT"&&(pc!==9||local!==0||edits!==1))
  throw Error("native B31 source commitment did not precede packet");
 pool+=packet;
 while(step()){if(attempts>=64)throw Error("B33 finite source should block");}
 if(local!==0||pc!==9||edits!==1||attempts+pool!==8+packet)
  throw Error("B33 native source paid-loop invariant violated");
 return {arm,arrival,packet,sensed:observations,request:code[8].value,
  services,attempts,remainingPool:pool,prepaid:pre};
}
const cases=[];
for(const packet of [0,4,5,6])for(const arrival of ["EARLY","AFTER_SENSE","AFTER_COMMIT"])
 for(const arm of ["Sense","BlindThree"]){
  const x=simulate(arm,arrival,packet);cases.push(x);
  const sensed=arrival==="EARLY"?packet:0;
  if(JSON.stringify(x.sensed)!==JSON.stringify(arm==="Sense"?[sensed]:[]))
   throw Error("B33 wrongly observed packet after reading state");
  if(x.request!==(arm==="BlindThree"||arm==="Sense"&&sensed===4?3:4))
   throw Error("B33 native source literal changed unexpectedly");
  const service=(arm==="BlindThree"||x.request===3)
   ?Number(packet>=4):Number(packet>=5);
  if(x.services!==service)throw Error("B33 service endpoint disagreement");
  if(x.remainingPool!==packet-(service?(x.request+1):0))
   throw Error("B33 resource inventory disagreement");
 }
if(cases.length!==24)throw Error("B33 wrong 24-world denominator");
process.stdout.write(JSON.stringify({
 evidence:"independent source-parsed B31 CIL interpreter; NOT native Rust",
 worlds:cases.length,source:"cl0/crates/world/tests/cl1b_b31_pool_sensing.rs",
 cases
},null,2)+"\n");
