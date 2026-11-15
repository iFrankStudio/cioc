// CL-1B B16: restricted source-parsed CIL interpreter, not the Rust VM.
// Reads the canonical founder from cl0/crates/world/tests/; no changed founder.
import { readFileSync } from "node:fs";
const source = readFileSync(new URL("../../cl0/crates/world/tests/cl1b_b16_functional_challenge.rs", import.meta.url), "utf8");
const body = source.match(/fn founder\(\) -> Vec<Op> \{[\s\S]*?vec!\[([\s\S]*?)\n    \]/)?.[1];
if (!body) throw Error("B16 canonical founder missing");
const ops = [...body.matchAll(/Op::([A-Za-z]+)(?:\s*\{([^}]*)\})?/g)].map(m => {
  const x = { op:m[1] };
  for(const a of m[2]?.matchAll(/(\w+)\s*:\s*(-?\d+)/g) || []) x[a[1]] = Number(a[2]);
  return x;
});
if(ops.length !== 32 || ops[10].next !== 11) throw Error("B16 founder source changed");
const allowed = new Set(["Const","Read","Write","Delete","Sub","Branch","ReadNext","PatchNext","Halt"]);
if(ops.some(x=>!allowed.has(x.op))) throw Error("unknown B16 CIL opcode");

function simulate(input, arm) {
  const code = ops.map(o=>({...o})), r=[0,0,0,0], cells=new Map([[7,input]]);
  let pc=0,attempts=0,trap=null,deleted=0,saves=0,suppressed=0;
  const probes=[],ordinary=[],reads=[],patches=[],external=[];
  external.push("load-actor","SetCell(7,input)");
  if(arm==="STALE") { cells.set(11,17); external.push("SetCell(11,17)"); }
  if(arm!=="INTACT") {
    code[10].next=arm==="CHECKER_BYPASS"?13:12;
    external.push("PatchEdge(10,"+code[10].next+")");
  }
  const reg=k=>{if(!Number.isInteger(k)||k<0||k>=4)throw Error("bad register");return r[k];};
  const edge=k=>{if(!Number.isInteger(k)||k<0||k>=32)throw Error("bad edge");return k;};
  while(attempts<120){
    const at=pc, op=code[at]; if(!op){trap="missing instruction";break;}
    attempts++;
    try{switch(op.op){
      case"Const":r[op.dst]=op.value;pc=edge(op.next);break;
      case"Read":{const key=reg(op.address),value=cells.get(key)??0;r[op.dst]=value;pc=edge(op.next);
        if(at===13)reads.push({attempt:attempts,key,value});break;}
      case"Write":{const key=reg(op.address),value=reg(op.value);
        if(!cells.has(key)&&(cells.size>=16||cells.size+33>128))throw Error("storage exhausted");
        cells.set(key,value);pc=edge(op.next);
        if(at===11)probes.push({attempt:attempts,key,value});
        if(at===31)ordinary.push({attempt:attempts,key,value});
        if(at===3)saves++;
        break;}
      case"Delete":cells.delete(reg(op.address));pc=edge(op.next);if(at===7)deleted++;break;
      case"Sub":r[op.dst]=reg(op.lhs)-reg(op.rhs);pc=edge(op.next);break;
      case"Branch":pc=edge(reg(op.test)!==0?op.nonzero:op.zero);break;
      case"ReadNext":{const node=edge(reg(op.node));if(code[node].next===undefined)throw Error("invalid readnext");
        r[op.dst]=code[node].next;pc=edge(op.next);break;}
      case"PatchNext":{const node=edge(reg(op.node)),target=edge(reg(op.target)),before=code[node].next;
        if(before===undefined)throw Error("invalid patchnext");
        if(arm==="DISABLED")suppressed++;
        else {code[node].next=target;if(before!==target)patches.push({attempt:attempts,node,before,after:target});}
        pc=edge(op.next);break;}
      default:throw Error("unexpected halt or opcode");
    }
    if(arm==="TIMED"&&attempts===12){
      if(pc!==13||cells.has(11))throw Error("timed-forgery barrier failed");
      cells.set(11,17);external.push("post-reset SetCell(11,17)");
    }
    }catch(e){trap=e.message;break;}
  }
  return {input,arm,attempts,pc,remainingCredits:240-attempts,trap,edge10:code[10].next,
    challengeWrites:probes.length,ordinaryWrites:ordinary.length,sourceSaves:saves,
    resets:deleted,firstRead:reads[0],reads,patches,suppressed,
    allChallengeOutputsCorrect:probes.every(x=>x.key===11&&x.value===17),
    allOrdinaryOutputsCorrect:ordinary.every(x=>x.key===11&&x.value===input),
    externals:external.length};
}
const arms=["INTACT","SKIP","STALE","DISABLED","CHECKER_BYPASS","TIMED"];
const expected={
 INTACT:{challengeWrites:5,ordinaryWrites:4,edge10:11,patchCount:0,suppressed:0,firstKey:11,firstValue:17},
 SKIP:{challengeWrites:4,ordinaryWrites:4,edge10:11,patchCount:1,suppressed:0,firstKey:11,firstValue:0},
 STALE:{challengeWrites:4,ordinaryWrites:4,edge10:11,patchCount:1,suppressed:0,firstKey:11,firstValue:0},
 DISABLED:{challengeWrites:0,ordinaryWrites:4,edge10:12,patchCount:0,suppressed:4,firstKey:11,firstValue:0},
 CHECKER_BYPASS:{challengeWrites:0,ordinaryWrites:5,edge10:13,patchCount:0,suppressed:0,firstKey:7,firstValue:17},
 TIMED:{challengeWrites:3,ordinaryWrites:4,edge10:11,patchCount:1,suppressed:0,firstKey:11,firstValue:17}
};
const worlds=[-13,0,7,23,101].flatMap(input=>arms.map(arm=>simulate(input,arm)));
for(const w of worlds){
 const x=expected[w.arm];
 for(const k of ["challengeWrites","ordinaryWrites","edge10","suppressed"]) {
   if(w[k]!==x[k])throw Error(`B16 mismatch input=${w.input} arm=${w.arm} field=${k} got=${w[k]} expected=${x[k]}`);
 }
 if(w.patches.length!==x.patchCount||w.firstRead.key!==x.firstKey||w.firstRead.value!==x.firstValue
   ||w.trap!==null||w.attempts!==120||!w.allChallengeOutputsCorrect||!w.allOrdinaryOutputsCorrect)
   throw Error("B16 failed source-level expected invariant");
}
process.stdout.write(JSON.stringify({
  source:"cl0/crates/world/tests/cl1b_b16_functional_challenge.rs",
  evidence:"independent simplified CIL interpreter; NOT native Rust",
  expectedWorlds:30,executedWorlds:worlds.length,
  worlds,
},null,2)+"\n");
