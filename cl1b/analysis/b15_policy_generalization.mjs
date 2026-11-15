// CL-1B B15: exhaustively compare authored in-actor policy step switch
// against an equal-opportunity one-literal no-switch counterfactual.
// Observer-side simplified CIL interpreter: NOT native Rust evidence.
import { readFileSync } from "node:fs";
const src = readFileSync(new URL("../../cl0/crates/world/tests/cl1b_b14_search_policy_mutation.rs",import.meta.url),"utf8");
const body=src.match(/fn founder\(\) -> Vec<Op> \{[\s\S]*?vec!\[([\s\S]*?)\n    \]/)?.[1];
if (!body) throw Error("missing original B14 founder");
const original=[...body.matchAll(/Op::([A-Za-z]+)(?:\s*\{([^}]*)\})?/g)].map(m=>{
 const o={op:m[1]};
 for(const f of m[2]?.matchAll(/(\w+)\s*:\s*(-?\d+)/g)||[])o[f[1]]=Number(f[2]);
 return o;
});
if(original.length!==35||original[21].op!=="Const"||original[21].value!==1||original[29].value!==2||original[6].next!==7)
 throw Error("frozen B14 source invariants changed");
function simulate(input,next,policySwitch){
 const code=original.map(o=>({...o})),reg=[0,0,0,0],cells=new Map([[7,input]]);
 code[6].next=next; if(!policySwitch)code[21].value=2;
 let pc=0,n=0,trap=null,stopped=false,service=0,marker=0,side=0,policyEdits=0,policyEffectiveChanges=0,candidatePolicyChanges=0,candidatePolicyUses=0,edgeEdits=0;
 const r=i=>{if(!Number.isInteger(i)||i<0||i>=4)throw Error("register");return reg[i]};
 const e=i=>{if(!Number.isInteger(i)||i<0||i>=35)throw Error("edge");return i};
 while(n<120&&!stopped){
  let o=code[pc]; if(!o){trap="missing instruction";break;}
  n++;
  try{switch(o.op){
   case"Const":if(pc===29&&o.value!==2)candidatePolicyUses++;reg[o.dst]=o.value;pc=e(o.next);break;
   case"Delete":cells.delete(r(o.address));pc=e(o.next);break;
   case"Read":reg[o.dst]=cells.get(r(o.address))??0;pc=e(o.next);break;
   case"Write":{let key=r(o.address),value=r(o.value);if(!cells.has(key)&&(cells.size>=16||cells.size+36>128))throw Error("storage");cells.set(key,value);pc=e(o.next);if(key===11&&value===input)service++;if(key===13&&value===1)marker++;if(key===1)side++;break;}
   case"Branch":pc=e(r(o.test)!==0?o.nonzero:o.zero);break;
   case"ReadNext":{let node=e(r(o.node));if(code[node].next===undefined)throw Error("read unpatchable");reg[o.dst]=code[node].next;pc=e(o.next);break;}
   case"PatchConst":{let node=e(r(o.node));if(code[node].op!=="Const")throw Error("not Const");if(code[node].value!==r(o.value)){policyEffectiveChanges++;if(node===29)candidatePolicyChanges++;}code[node].value=r(o.value);policyEdits++;pc=e(o.next);break;}
   case"Sub":reg[o.dst]=r(o.lhs)-r(o.rhs);pc=e(o.next);break;
   case"PatchNext":{let node=e(r(o.node)),val=e(r(o.target));if(code[node].next===undefined)throw Error("unpatchable");if(code[node].next!==val)edgeEdits++;code[node].next=val;pc=e(o.next);break;}
   case"Halt":stopped=true;break;
   default:throw Error("unknown opcode "+o.op);
  }}catch(err){trap=String(err.message);stopped=true;}
 }
 return {input,next,policySwitch,attempts:n,stopped,trap,pc,services:service,markers:marker,side,policyEdits,policyEffectiveChanges,candidatePolicyChanges,candidatePolicyUses,edgeEdits,finalEdge:code[6].next,finalStep:code[29].value};
}
const inputs=[-13,0,7,23,101],states=[];
for(const input of inputs)for(let next=0;next<35;next++){
 const adaptive=simulate(input,next,true),locked=simulate(input,next,false);
 states.push({input,next,group:next===7?"healthy":next===10||next===11?"development":"holdout",adaptive,locked,delta:adaptive.services-locked.services});
}
if(states.length!==175)throw Error("B15 175 matched-pair denominator");
function summarize(rows){
 const sum=key=>rows.reduce((v,s)=>v+s[key],0);
 return {pairs:rows.length,win:rows.filter(s=>s.delta>0).length,loss:rows.filter(s=>s.delta<0).length,tie:rows.filter(s=>s.delta===0).length,
  totalServiceDelta:sum("delta"),
  adaptiveTraps:rows.filter(s=>s.adaptive.trap!==null).length,lockedTraps:rows.filter(s=>s.locked.trap!==null).length,
  adaptiveStopped:rows.filter(s=>s.adaptive.stopped).length,lockedStopped:rows.filter(s=>s.locked.stopped).length,
  unequalAttempts:rows.filter(s=>s.adaptive.attempts!==s.locked.attempts).length};
}
const groups={healthy:summarize(states.filter(s=>s.group==="healthy")),development:summarize(states.filter(s=>s.group==="development")),holdout:summarize(states.filter(s=>s.group==="holdout")),overall:summarize(states)};
const byLesion=Array.from({length:35},(_,next)=>({next,...summarize(states.filter(s=>s.next===next))}));
const uniqueHoldoutEdges=byLesion.filter(x=>x.next!==7&&x.next!==10&&x.next!==11);
const policyActiveHoldout=states.filter(s=>s.group==="holdout"&&(s.adaptive.policyEdits>0||s.locked.policyEdits>0));
const policyInactiveHoldout=states.filter(s=>s.group==="holdout"&&s.adaptive.policyEdits===0&&s.locked.policyEdits===0);
const effectivePolicyHoldout=states.filter(s=>s.group==="holdout"&&s.adaptive.candidatePolicyChanges>0&&s.adaptive.candidatePolicyUses>0);
const opcodeReachedButNotEffective=policyActiveHoldout.filter(s=>s.adaptive.candidatePolicyChanges===0||s.adaptive.candidatePolicyUses===0);
const postHocUnits={
  policyChangedAndUsedUniqueEdges:[...new Set(effectivePolicyHoldout.map(s=>s.next))],
  policyChangedAndUsedMatchedPairs:summarize(effectivePolicyHoldout),
  opcodeReachedButPolicyNotChangedUsedPairs:summarize(opcodeReachedButNotEffective),
  uniqueHoldoutEdgeTypes:uniqueHoldoutEdges.length,
  distinctHeldoutWins:uniqueHoldoutEdges.filter(s=>s.win>0).length,
  distinctHeldoutLosses:uniqueHoldoutEdges.filter(s=>s.loss>0).length,
  distinctHeldoutTies:uniqueHoldoutEdges.filter(s=>s.tie===5).length,
  policyActiveUniqueEdges:[...new Set(policyActiveHoldout.map(s=>s.next))],
  policyInactiveUniqueEdges:[...new Set(policyInactiveHoldout.map(s=>s.next))],
  policyActiveMatchedPairs:summarize(policyActiveHoldout),
  policyInactiveMatchedPairs:summarize(policyInactiveHoldout),
  caveat:"Post-hoc B15b stratification: five labels are not independent structural lesions; different numeric inputs CAN cause different internal code paths under mutation, even when the service-count win/loss category is repeated."
};
if(postHocUnits.uniqueHoldoutEdgeTypes!==32||
  postHocUnits.policyActiveUniqueEdges.length!==13||
  postHocUnits.policyInactiveUniqueEdges.length!==32 ||
  postHocUnits.policyActiveMatchedPairs.pairs!==53 ||
  postHocUnits.policyInactiveMatchedPairs.pairs!==107 ||
  postHocUnits.policyChangedAndUsedUniqueEdges.length!==7 ||
  postHocUnits.policyChangedAndUsedMatchedPairs.pairs!==31 ||
  postHocUnits.opcodeReachedButPolicyNotChangedUsedPairs.pairs!==22)
 throw Error("B15b active-policy lesion denominator changed");

const output={source:"B14 exact founder parsed from checked-out Rust file",status:"independent simplified interpreter; NOT CL-0 Rust native evidence",initialCredits:240,maxAttempts:120,worlds:350,matchedPairs:175,groups,byLesion,postHocUnits,
 development:states.filter(s=>s.group==="development"),
 holdoutWinExamples:states.filter(s=>s.group==="holdout"&&s.delta>0).slice(0,10),
 holdoutLossExamples:states.filter(s=>s.group==="holdout"&&s.delta<0).slice(0,10)};
process.stdout.write(JSON.stringify(output,null,2)+"\n");
