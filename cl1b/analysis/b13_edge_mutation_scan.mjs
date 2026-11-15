// CL-1B/B13: observer-side single-edge mutant screen. No host selected winner.
// Data are simplified CIL semantics, NOT original Rust VM runs.
import {readFileSync} from "node:fs";
function parseProgram(src) {
 const body=src.match(/fn founder\(\) -> Vec<Op> \{[\s\S]*?vec!\[([\s\S]*?)\n    \]/)?.[1]; if(!body)throw Error("missing founder");
 const ops=[...body.matchAll(/Op::([A-Za-z]+)\s*\{([^}]*)\}/g)].map(m=>{const o={op:m[1]};for(const f of m[2].matchAll(/(\w+)\s*:\s*(-?\d+)/g))o[f[1]]=Number(f[2]);return o;});
 if(ops.length!==20||ops.filter(o=>o.next!==undefined).length!==19)throw Error("unexpected founder shape");
 if(ops.some(o=>!["Const","Delete","Read","Write","Branch","ReadNext","Sub","PatchNext"].includes(o.op)))throw Error("unexpected opcode");
 return ops;
}
function simulate(ops,input,mutation) {
 const code=ops.map(o=>({...o})),r=[0,0,0,0],cells=new Map([[7,input]]);if(mutation)code[mutation.node].next=mutation.target;
 let pc=0,n=0,trap=null,correct=0,wrong=0,markers=0,accepts=0,rejects=0,fp=0,fn=0,repairs=0,side=0,serviceSinceReset=0;
 const reg=k=>{if(!Number.isInteger(k)||k<0||k>=4)throw Error("register");return r[k];};
 const edge=k=>{if(!Number.isInteger(k)||k<0||k>=20)throw Error("edge");return k;};
 while(n<80){
  if(!Number.isInteger(pc)||pc<0||pc>=20){trap="pc";break;}
  const o=code[pc],at=pc;n++;
  try {switch(o.op){
    case "Const":r[o.dst]=o.value;pc=edge(o.next);break;
    case "Delete":cells.delete(reg(o.address));pc=edge(o.next);if(at===2)serviceSinceReset=0;break;
    case "Read":{let key=reg(o.address),v=cells.get(key)??0;r[o.dst]=v;pc=edge(o.next);if(at===11&&key===13){if(v!==0){accepts++;if(!serviceSinceReset)fp++;}else{rejects++;if(serviceSinceReset)fn++;}}break;}
    case "Write":{let key=reg(o.address),v=reg(o.value);pc=edge(o.next);if(!cells.has(key)&&cells.size+20>=64)throw Error("storage");cells.set(key,v);if(key===11){if(v===input){correct++;serviceSinceReset++;}else wrong++;}else if(key===13&&v===1)markers++;else if(key!==7)side++;break;}
    case "Branch":pc=edge(reg(o.test)!==0?o.nonzero:o.zero);break;
    case "ReadNext":{let target=edge(reg(o.node));if(code[target].next===undefined)throw Error("no readable edge");r[o.dst]=code[target].next;pc=edge(o.next);break;}
    case "Sub":r[o.dst]=reg(o.lhs)-reg(o.rhs);pc=edge(o.next);break;
    case "PatchNext":{let targetNode=edge(reg(o.node)),target=edge(reg(o.target));if(code[targetNode].next===undefined)throw Error("no patchable edge");pc=edge(o.next);if(code[targetNode].next!==target)repairs++;code[targetNode].next=target;break;}
    default:throw Error("op");
  }}catch(e){trap=String(e.message);break;}
 }
 return {n,trap,correct,wrong,markers,accepts,rejects,fp,fn,repairs,side};
}
function scan(ops) {
 const inputs=[-13,0,7,23,101],groups=[],byNode=[];
 for(const input of inputs){
   const outcomes=[];
   for(let node=0;node<20;node++)if(ops[node].next!==undefined)
    for(let target=0;target<20;target++)if(target!==ops[node].next)
     outcomes.push({node,target,...simulate(ops,input,{node,target})});
   if(outcomes.length!==361)throw Error("unexpected mutant count");
   const count=k=>outcomes.filter(x=>x[k]>0).length;
   groups.push({input,baseline:simulate(ops,input,null),worlds:outcomes.length,
    trapped:outcomes.filter(x=>x.trap!==null).length,
    falsePositiveWorlds:count("fp"),falseNegativeWorlds:count("fn"),
    markerWithoutAnyService:outcomes.filter(x=>x.markers>0&&x.correct===0).length,
    serviceWithoutAnyMarker:outcomes.filter(x=>x.correct>0&&x.markers===0).length,
    falsePositiveDecisions:outcomes.reduce((a,x)=>a+x.fp,0),
    falseNegativeDecisions:outcomes.reduce((a,x)=>a+x.fn,0),
    repairs:count("repairs")});
   if(input===23){for(let node=0;node<20;node++)if(ops[node].next!==undefined){
    const subset=outcomes.filter(x=>x.node===node);
    byNode.push({node,total:subset.length,fp:subset.filter(x=>x.fp>0).length,fn:subset.filter(x=>x.fn>0).length,trapped:subset.filter(x=>x.trap!==null).length,
    sampleFalsePositive:subset.find(x=>x.fp>0)?.target??null});
   }}
 }
 const sum=k=>groups.reduce((a,x)=>a+x[k],0);
 return {name:"B13 bounded single-edge mutation scan",status:"independent simplified interpreter NOT Rust CI",source:"cl0/crates/world/tests/cl1b_b10_freshness.rs",worlds:sum("worlds"),groups,totals:{trapped:sum("trapped"),falsePositiveWorlds:sum("falsePositiveWorlds"),falseNegativeWorlds:sum("falseNegativeWorlds"),markerWithoutAnyService:sum("markerWithoutAnyService"),serviceWithoutAnyMarker:sum("serviceWithoutAnyMarker"),falsePositiveDecisions:sum("falsePositiveDecisions"),falseNegativeDecisions:sum("falseNegativeDecisions"),repairWorlds:sum("repairs")},byNode};
}
const src=readFileSync(new URL("../../cl0/crates/world/tests/cl1b_b10_freshness.rs",import.meta.url),"utf8");
process.stdout.write(JSON.stringify(scan(parseProgram(src)),null,2)+"\n");
