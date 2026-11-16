// B39 independently executes the exact B38 Rust-source-parsed CIL programs,
// with a real task input change between Read(key7) and output verification.
// No privileged reward, host code repair or synthetic success marker.
import {b38Source} from "./b38_functional_readback_scan.mjs";
import {fileURLToPath} from "node:url";
export function simulateB39(early,oldValue,newValue,readback,recheck=false){
 const arm=readback?(recheck?"Recheck":"Readback"):"StaticCorrect";
 const code=b38Source(readback?"Readback":"StaticCorrect");
 if(recheck){
  code[5].next=12;
  code.push(
   {kind:"Const",dst:2,value:7,next:13},
   {kind:"Read",dst:0,address:2,next:14},
   {kind:"Const",dst:2,value:11,next:6},
  );
 }
 const r=[0,0,0,0],cells=new Map([[7,oldValue]]);
 let pc=0,charged=0,credits=96,edits=0,sampled=null,checkpoint=null,updated=false;
 let correct=0,wrong=0;
 const taskReads=[];
 const reg=n=>{if(!Number.isInteger(n)||n<0||n>3)throw Error("B39 invalid CIL register");return r[n];};
 if(early){cells.set(7,newValue);updated=true;}
 function step(){
  if(credits<=0)throw Error("B39 source spent too much compute");
  const op=code[pc],at=pc;
  credits--;charged++;
  let event={pc:at,kind:op.kind};
  switch(op.kind){
   case"Const":r[op.dst]=op.value;pc=op.next;break;
   case"Read":{
    const key=reg(op.address),value=cells.get(key)??0;
    r[op.dst]=value;pc=op.next;
    if(key===7)taskReads.push(value);
    event={pc:at,kind:"CellRead",key,value};
    break;
   }
   case"Write":{
    const key=reg(op.address),value=reg(op.value);
    cells.set(key,value);pc=op.next;
    event={pc:at,kind:"CellWritten",key,value};
    if(updated&&key===11){
     if(value===newValue)correct++;else wrong++;
    }
    break;
   }
   case"Sub":r[op.dst]=reg(op.lhs)-reg(op.rhs);pc=op.next;break;
   case"Branch":pc=reg(op.test)!==0?op.nonzero:op.zero;break;
   case"PatchNext":{
    const node=reg(op.node),target=reg(op.target);
    if(!code[node]||target<0||target>=code.length)throw Error("B39 invalid native patch");
    if(code[node].next!==target)edits++;
    code[node].next=target;pc=op.next;break;
   }
   default:throw Error("B39 unsupported native opcode "+op.kind);
  }
  return event;
 }
 while(true){
  const e=step();
  if(e.kind==="CellRead"&&e.key===7){
   sampled=e.value;
   if(!early){cells.set(7,newValue);updated=true;}
   break;
  }
  if(charged>8)throw Error("B39 never sampled task");
 }
 while(true){
  const e=step();
  if((readback&&e.pc===7)||(!readback&&e.kind==="CellWritten"&&e.key===11)){
   checkpoint={pc,firstWritten:cells.get(11),currentInput:cells.get(7),
    firstOutputCurrentCorrect:cells.get(11)===newValue,
    sourceEdge:code[readback?3:2].next};
   checkpoint.firstVerdictCorrect=readback?
    ((pc===1)===checkpoint.firstOutputCurrentCorrect):null;
   break;
  }
  if(charged>30)throw Error("B39 missed first verification checkpoint");
 }
 while(charged<96)step();
 if(credits!==0||charged!==96)throw Error("B39 compute conservation");
 return {old:oldValue,new:newValue,arrival:early?"BEFORE_READ":"AFTER_READ",
  arm,sampled,checkpoint,edits,finalEdge:code[readback?3:2].next,
  readsAtFirstCheck:taskReads.slice(0,readback?(recheck?2:1):1),
  correct,wrong,charged};
}
if(process.argv[1]===fileURLToPath(import.meta.url)) {
const cases=[];
for(const [old,newValue] of [[0,17],[17,0]])
 for(const early of [true,false])
  for(const readback of [true,false]){
   const out=simulateB39(early,old,newValue,readback);
   cases.push(out);
   if(readback){
    if(out.checkpoint.firstVerdictCorrect!==early)
     throw Error("B39 false/true native verdict failed to match temporal causal history");
    if(out.checkpoint.sourceEdge!==4)throw Error("B39 repaired before first feedback checkpoint");
    if(!early&&out.checkpoint.pc!==(old===0?1:8))
     throw Error("B39 stale expected input should produce false success or false failure");
   }else{
    if(out.checkpoint.firstOutputCurrentCorrect!==early)
     throw Error("B39 static copier did not exhibit same asynchronous sampling window");
    if(out.edits!==0||out.finalEdge!==3)
     throw Error("B39 immutable static copier changed source");
   }
  }
if(cases.length!==8)throw Error("B39 total world denominator != 8");
process.stdout.write(JSON.stringify({
 evidence:"independent Rust-source-parsed CIL native-order model, NOT Rust CI",
 worlds:8,cases
},null,2)+"\n");

}
