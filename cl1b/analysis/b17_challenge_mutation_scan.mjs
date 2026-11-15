// CL-1B B17: exhaustive source-parsed CIL checker robustness screen.
// All outcomes are independent simplified interpreter results, NOT native Rust.
// Reads actual B16 founder source; no hardcoded alternate founder or host repair.
import { readFileSync } from "node:fs";

const source = readFileSync(
  new URL("../../cl0/crates/world/tests/cl1b_b16_functional_challenge.rs", import.meta.url),
  "utf8"
);
const vector = source.match(/fn founder\(\) -> Vec<Op> \{[\s\S]*?vec!\[([\s\S]*?)\n    \]/)?.[1];
if (!vector) throw Error("B17 cannot locate B16 founder");
const program = [...vector.matchAll(/Op::([A-Za-z]+)(?:\s*\{([^}]*)\})?/g)].map(m => {
  const x = { op: m[1] };
  for(const a of m[2]?.matchAll(/(\w+)\s*:\s*(-?\d+)/g) || []) x[a[1]] = Number(a[2]);
  return x;
});
const allowed = new Set(["Const","Read","Write","Delete","Sub","Branch","ReadNext","PatchNext","Halt"]);
if(program.length!==32 || program.filter(o=>o.next!==undefined).length!==29
   ||program.some(o=>!allowed.has(o.op)))throw Error("B17 source schema drift");

function simulate(input, mutation) {
  const nodes=program.map(x=>({...x})), r=[0,0,0,0], cells=new Map([[7,input]]);
  if(mutation)nodes[mutation.node].next=mutation.to;
  let pc=0,attempts=0,trap=null,halted=false,repairs=0;
  let challengeInEpoch=0,pendingReadStage=0,pc13ReadBypass=0;
  let challengeWrites=0,ordinaryWrites=0,wrongWrites=0,outputDeletions=0;
  let fp=0,fn=0,accepts=0,rejects=0,checkerReached=0,branchNoRead=0;
  const reg=i=>{if(!Number.isInteger(i)||i<0||i>=4)throw Error("invalid register");return r[i];};
  const edge=i=>{if(!Number.isInteger(i)||i<0||i>=32)throw Error("invalid edge");return i;};
  while(attempts<120 && !halted) {
    const at=pc, op=nodes[at];
    if(!op){trap="invalid PC";break;}
    // The normal read→decision pipeline is PC13 Read, PC14 Const,
    // PC15 Sub, then PC16 Branch (not an immediate PC13→PC16 jump).
    if(pendingReadStage!==0){
      if(at===pendingReadStage){
        pendingReadStage=pendingReadStage===16?0:pendingReadStage+1;
      }else{
        pc13ReadBypass++;
        pendingReadStage=0;
      }
    }
    if(at===0)challengeInEpoch=0;
    attempts++;
    try{switch(op.op){
      case "Const": if(op.dst<0||op.dst>=4)throw Error("invalid dest");r[op.dst]=op.value;pc=edge(op.next);break;
      case "Read": {
        const key=reg(op.address),value=cells.get(key)??0;
        if(op.dst<0||op.dst>=4)throw Error("invalid dest");
        r[op.dst]=value;pc=edge(op.next);
        if(at===13)pendingReadStage=14;
        break;
      }
      case "Write": {
        const key=reg(op.address),value=reg(op.value);
        if(!cells.has(key)&&(cells.size>=16||cells.size+33>128))
          throw Error("storage capacity");
        cells.set(key,value);pc=edge(op.next);
        if(at===11&&key===11&&value===17){challengeInEpoch++;challengeWrites++;}
        if(at===31&&key===11&&value===input)ordinaryWrites++;
        if((at===11&&!(key===11&&value===17))||(at===31&&!(key===11&&value===input)))wrongWrites++;
        break;
      }
      case "Delete": {
        const key=reg(op.address);
        cells.delete(key);pc=edge(op.next);
        if(at===7&&key===11)outputDeletions++;
        break;
      }
      case "Sub": if(op.dst<0||op.dst>=4)throw Error("invalid dest");r[op.dst]=reg(op.lhs)-reg(op.rhs);pc=edge(op.next);break;
      case "Branch": {
        const nonzero=reg(op.test)!==0, next=edge(nonzero?op.nonzero:op.zero);
        if(at===16){
          checkerReached++;
          if(at===16 && pendingReadStage!==0)branchNoRead++;
          if(next===24){accepts++;if(challengeInEpoch===0)fp++;}
          else if(next===17){rejects++;if(challengeInEpoch>0)fn++;}
        }
        pc=next;break;
      }
      case "ReadNext": {
        const node=edge(reg(op.node));
        if(nodes[node].next===undefined)throw Error("no readable edge");
        if(op.dst<0||op.dst>=4)throw Error("invalid dest");
        r[op.dst]=nodes[node].next;pc=edge(op.next);break;
      }
      case "PatchNext": {
        const node=edge(reg(op.node)), target=edge(reg(op.target));
        if(nodes[node].next===undefined)throw Error("not patchable");
        if(nodes[node].next!==target)repairs++;
        nodes[node].next=target;pc=edge(op.next);break;
      }
      case "Halt":halted=true;break;
      default:throw Error("unsupported opcode");
    }}catch(e){trap=e.message;halted=true;}
  }
  return {attempts,trap,halted,checkerReached,fp,fn,accepts,rejects,
    challengeWrites,ordinaryWrites,wrongWrites,outputDeletions,repairs,
    branchNoRead,pc13ReadBypass};
}

const inputs=[-13,0,7,23,101], rows=[];
for(const input of inputs) {
 const control=simulate(input,null);
 if(control.trap||control.halted||control.attempts!==120||
    control.challengeWrites<1||control.accepts<1||control.fp||control.fn)
   throw Error("B17 intact control did not meet registered positive criterion");
 for(let node=0;node<32;node++)if(program[node].next!==undefined) {
  for(let to=0;to<32;to++)if(program[node].next!==to){
    rows.push({input,node,to,...simulate(input,{node,to})});
  }
 }
}
if(rows.length!==4495)throw Error("B17 mutation denominator changed");
function summary(xs) {
 const count=fn=>xs.filter(fn).length, sum=k=>xs.reduce((a,x)=>a+x[k],0);
 return {worlds:xs.length,
  falsePositiveWorlds:count(x=>x.fp>0),falseNegativeWorlds:count(x=>x.fn>0),
  evaluatorReachedWorlds:count(x=>x.checkerReached>0),
  falsePositiveAmongReached:count(x=>x.fp>0&&x.checkerReached>0),
  falseNegativeAmongReached:count(x=>x.fn>0&&x.checkerReached>0),
  trappedWorlds:count(x=>x.trap!==null),haltedWorlds:count(x=>x.halted),
  truncatedWorlds:count(x=>x.attempts<120),branchWithoutReadWorlds:count(x=>x.branchNoRead>0),
  pc13ReadBypassedWorlds:count(x=>x.pc13ReadBypass>0),
  noRealChallengeWorlds:count(x=>x.challengeWrites===0),
  actualChallengeAndNoAcceptWorlds:count(x=>x.challengeWrites>0&&x.accepts===0),
  falsePositiveDecisions:sum("fp"),falseNegativeDecisions:sum("fn")};
}
const groups=inputs.map(input=>({input,...summary(rows.filter(x=>x.input===input))}));
const byNode=Array.from({length:32},(_,node)=>{
 const xs=rows.filter(x=>x.node===node);
 return xs.length?{node,...summary(xs)}:null;
}).filter(Boolean);
const unique=rows.filter(x=>x.fp>0).map(x=>x.node+":"+x.to);
const distinctFalsePositiveEdges=[...new Set(unique)];
const distinctFalseNegativeEdges=[...new Set(rows.filter(x=>x.fn>0).map(x=>x.node+":"+x.to))];
const result={
  source:"cl0/crates/world/tests/cl1b_b16_functional_challenge.rs",
  evidence:"independent simplified interpreter NOT native Rust",sourceNodes:32,
  underlyingMutations:899,worlds:4495,controlInputs:inputs,
  totals:summary(rows),groups,byNode,
  distinctFalsePositiveMutations:distinctFalsePositiveEdges.length,
  distinctFalseNegativeMutations:distinctFalseNegativeEdges.length,
  examples:rows.filter(x=>x.fp>0).slice(0,20),
  knownCheckerBypass:rows.filter(x=>x.node===10&&x.to===13).map(x=>({input:x.input,fp:x.fp,challengeWrites:x.challengeWrites,ordinaryWrites:x.ordinaryWrites})),
};
process.stdout.write(JSON.stringify(result,null,2)+"\n");
