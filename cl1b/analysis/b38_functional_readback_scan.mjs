// B38 independent Rust-source-parsed native CIL semantics.
// Performs all 96 charged attempts; changes external task input at step48.
// No fitness oracle, no host repair, no claim of autonomous goal discovery.
import {readFileSync} from "node:fs";
const src=readFileSync(new URL("../../cl0/crates/world/tests/cl1b_b38_functional_readback.rs",import.meta.url),"utf8");
const body=src.match(/fn founder\(arm:Arm\)->Vec<Op>\{([\s\S]*?)\n\}/)?.[1];
if(!body)throw Error("B38 Rust native CIL founder missing");
const parse=text=>[...text.matchAll(/Op::([A-Za-z]+)\s*\{([^}]*)\}/g)].map(m=>{
 const op={kind:m[1]};
 for(const item of m[2].matchAll(/(\w+)\s*:\s*(-?\d+)/g))
  op[item[1]]=Number(item[2]);
 return op;
});
const staticSource=body.match(/if arm==Arm::StaticCorrect\{([\s\S]*?)\n \}/)?.[1];
const staticCode=parse(staticSource||"");
const source=parse(body.slice(body.lastIndexOf("\n vec![")));
if(staticCode.length!==4||source.length!==14||
 source[6].kind!=="Sub"||source[7].kind!=="Const"||source[8].kind!=="Sub"||
 source[12].kind!=="PatchNext"||source[13].kind!=="Write")
 throw Error("B38 native Rust CIL founder source layout drift");
function founder(arm){
 if(arm==="StaticCorrect")return staticCode.map(o=>({...o}));
 const comparatorIndex=arm==="AssumeOk"?8:arm==="ForceFix"?7:6;
 return [...source.slice(0,6),source[comparatorIndex],...source.slice(9)]
  .map(op=>({...op}));
}
function trial(arm,history){
 const code=founder(arm),regs=[0,0,0,0],cells=new Map();
 let pc=0,credits=96,charged=0,edits=0,suppressed=0,traps=0,stopped=false;
 let firstEdge=null;
 const accurate=[],inaccurate=[],totalWrites=[],reported=[];
 const reg=id=>{if(id<0||id>=4||!Number.isInteger(id))throw Error("B38 invalid register");return regs[id]};
 for(let phase=0;phase<2;phase++){
  const expected=history[phase];
  cells.set(7,expected);
  let correct=0,bad=0;
  for(let step=0;step<48;step++){
   if(stopped||credits<=0)throw Error("B38 native source prematurely non-runnable");
   const op=code[pc],was=credits;
   credits--;charged++;
   switch(op.kind){
    case"Const":regs[op.dst]=op.value;pc=op.next;break;
    case"Read":regs[op.dst]=cells.get(reg(op.address))??0;pc=op.next;break;
    case"Write":{
     const addr=reg(op.address),value=reg(op.value);
     cells.set(addr,value);
     if(addr===11){if(value===expected)correct++;else bad++;}
     pc=op.next;break;
    }
    case"Sub":regs[op.dst]=reg(op.lhs)-reg(op.rhs);pc=op.next;break;
    case"Branch":pc=reg(op.test)!==0?op.nonzero:op.zero;break;
    case"PatchNext":{
     const node=reg(op.node),target=reg(op.target);
     if(!code[node]||target<0||target>=code.length)throw Error("B38 invalid native patch target");
     if(arm==="PatchDisabled")suppressed++;
     else{if(code[node].next!==target)edits++;code[node].next=target;}
     pc=op.next;break;
    }
    default:throw Error("B38 unknown native CIL instruction "+op.kind);
   }
   if(credits!==was-1)throw Error("B38 charged compute violation");
  }
  accurate.push(correct);inaccurate.push(bad);totalWrites.push(correct+bad);
  if(phase===0)firstEdge=code[arm==="StaticCorrect"?2:3].next;
  reported.push({phase,input:expected,pc,credits,output:cells.get(11)??null});
 }
 if(charged!==96||credits!==0)throw Error("B38 charge conservation failure");
 return {arm,history,accurate,inaccurate,totalWrites,edits,suppressed,
  firstEdge,edge:code[arm==="StaticCorrect"?2:3].next,
  pc,credits,charged,stopped,traps,reported};
}
const histories=[[0,17],[17,0],[23,-9],[-9,23]];
const arms=["Readback","AssumeOk","ForceFix","PatchDisabled","StaticCorrect"];
const cases=[];
for(const history of histories)for(const arm of arms){
 cases.push(trial(arm,history));
}
if(cases.length!==20)throw Error("B38 primary denominator must be20 worlds");
for(let i=0;i<4;i++){
 const [sensed,assumed,forced,disabled,compact]=cases.slice(i*5,i*5+5);
 if(sensed.edits!==1||sensed.edge!==11||assumed.edits!==0||
  assumed.edge!==4||forced.edits!==1||disabled.edits!==0||
  disabled.suppressed<=0||compact.edge!==3)
  throw Error("B38 real functional self-edit differential broken");
 if(sensed.history[0]===0&&sensed.firstEdge!==4)
  throw Error("B38 repaired before any actual mismatch on initially correct input");
 if(sensed.history[0]!==0&&sensed.firstEdge!==11)
  throw Error("B38 failed to patch at first actual mismatch");
 if(compact.accurate.reduce((a,b)=>a+b,0)<
   sensed.accurate.reduce((a,b)=>a+b,0))
  throw Error("B38 compact immutable performance advantage lost");
}
process.stdout.write(JSON.stringify({
 evidence:"independent Rust-source-parsed CIL VM, NOT native CI",
 origin:"cl0/crates/world/tests/cl1b_b38_functional_readback.rs",
 worlds:20,chargePerWorld:96,phasesPerWorld:2,cases
},null,2)+"\n");
