#!/usr/bin/env node
// Retrospective model only; no Rust or independent native replay.
import fs from "node:fs";
import assert from "node:assert/strict";
const m=JSON.parse(fs.readFileSync(new URL("../studies/CL1B-C27-PAID-CREDIT-REMEASURE-ROLLBACK-frozen-8-worlds-v1.json",import.meta.url),"utf8"));
const C=(dst,value,next)=>({kind:"Const",dst,value,next}), S=(dst,lhs,rhs,next)=>({kind:"Sub",dst,lhs,rhs,next}), B=(test,nonzero,zero)=>({kind:"Branch",test,nonzero,zero}), R=(dst,address,next)=>({kind:"Read",dst,address,next}), W=(address,value,next)=>({kind:"Write",address,value,next}), Q=(dst,next)=>({kind:"SenseOwnCredits",dst,next});
const A=(dst,lhs,rhs,next)=>({kind:"Add",dst,lhs,rhs,next});
function source(policy,k){
 const g=JSON.parse(JSON.stringify(m.original_full_frozen_170_node_CIL_source));
 Object.assign(g,{0:Q(0,3),30:Q(3,31),31:S(0,0,3,32),32:C(2,41,33),33:W(2,0,34),34:Q(0,35),35:C(2,50,36),36:R(3,2,37),37:B(3,40,50),60:Q(3,61),61:S(1,0,3,62),62:C(2,41,63),63:R(0,2,73),73:S(3,0,1,74),74:S(1,1,0,75),75:C(2,1,76),76:B(3,170,100)});
 for(let i=0;i<8;i++){const p=170+4*i;g.push(S(3,3,2,p+1),B(3,p+2,90),S(1,1,2,p+3),B(1,i===7?100:p+4,80));}
 g.push(C(2,47,203),C(1,0,204),W(2,1,206),{kind:"Halt"},Q(3,207),C(2,46,208),W(2,3,100),Q(3,210),C(2,46,211),R(1,2,212),S(1,1,3,213),C(2,47,214),R(3,2,215),B(3,216,218),S(3,3,1,217),B(3,0,206),W(2,1,206));
 if(policy==="monitor"){g[76].zero=202;g[201].nonzero=202;g[95].next=202;g[167].zero=202;g[169].next=202;}
 const add=n=>(g.push(n),g.length-1);
 for(const mode of [0,1]){
  let next=policy==="monitor"?209:100;
  for(let i=k-1;i>=0;i--){
   let p=add(W(2,1,next));p=add(C(2,45,p));p=add(A(1,1,2,p));p=add(C(2,1,p));p=add(R(1,2,p));p=add(C(2,45,p));
   p=add(W(2,1,p));p=add(C(1,1,p));const cheap=add(C(2,11,p));
   let costly=cheap;for(let j=0;j<4;j++)costly=add(C(2,0,costly));
   p=add(mode===0?B(3,costly,cheap):B(3,cheap,costly));
   const read=R(3,2,p);
   if(i===0)g[mode===0?101:131]=read;else next=add(C(2,50,add(read)));
  }
 }
 return g;
}

function shared(policy,k){
 const g=source(policy,1).slice(0,219);
 const add=n=>(g.push(n),g.length-1);
 let next=policy==="monitor"?209:100;
 for(let i=k-1;i>=0;i--){
  let p=add(W(2,1,next));p=add(C(2,45,p));p=add(A(1,1,2,p));p=add(C(2,1,p));p=add(R(1,2,p));p=add(C(2,45,p));
  p=add(W(2,1,p));p=add(C(1,1,p));const cheap=add(C(2,11,p));
  let costly=cheap;for(let j=0;j<4;j++)costly=add(C(2,0,costly));
  p=add(B(3,costly,cheap));p=add(S(3,3,0,p));p=add(R(3,2,p));
  if(i===0){g[101]=C(0,0,p);g[131]=C(0,1,p);}else next=add(C(2,50,p));
 }
 return g;
}
// Source-specific conservative reachability: PC100 may be edited to either root.
// Only these five Const instructions encode source addresses. Do not apply as
// a general CIL optimizer to arbitrary reflective/self-modifying graphs.
function compact(g){
 const seen=new Set(),stack=[0,101,131];
 while(stack.length){const p=stack.pop();if(seen.has(p))continue;seen.add(p);const n=g[p];if(n.kind==="Branch")stack.push(n.nonzero,n.zero);else if(n.kind!=="Halt")stack.push(n.next);}
 const old=[...seen].sort((a,b)=>a-b),map=new Map(old.map((p,i)=>[p,i]));
 const nodes=old.map(p=>{const n={...g[p]};if(n.kind==="Branch"){n.nonzero=map.get(n.nonzero);n.zero=map.get(n.zero);}else if(n.kind!=="Halt")n.next=map.get(n.next);
 if([93,94,163,165,168].includes(p))n.value=map.get(n.value);
 return n;});
 return {nodes,old,map};
}

function predicted(k,variant){
 const g=variant==="shared"?shared("monitor",k):source("monitor",k);
 const add=n=>(g.push(n),g.length-1);
 let p=add(W(2,1,73));p=add(C(2,42,p));g[63].next=p;
 const offset=variant==="shared"?76:63;
 p=add(W(2,1,206));p=add(C(2,47,p));p=add(A(1,1,2,p));p=add(C(2,offset,p));
 p=add(A(1,1,3,p));p=add(A(1,3,3,p));p=add(A(3,1,1,p));p=add(A(1,1,1,p));
 const load=add(R(1,2,p));const aa=add(C(2,41,load)),bb=add(C(2,42,load));
 p=add(B(3,bb,aa));p=add(S(3,3,2,p));p=add(C(2,101,p));p=add({kind:"ReadNext",dst:3,node:1,next:p});p=add(C(1,100,p));
 g[202]={...g[202],next:p};
 return g;
}
function sim(g,s,off){
 const v=[0,0,0,0],d=new Map([[50,s.mode]]);let pc=0,c=0,prod=0,trials=0,stage=0;const probes=[],observations=[],edits=[];
 while(c<20000){const n=g[pc],old=pc;assert(n);c++;pc=n.next;
 if(old===0)probes.push(stage);if(old===215)observations.push([stage,v[1],v[3]]);
 switch(n.kind){case"SenseOwnCredits":v[n.dst]=20000-c;break;case"Const":v[n.dst]=n.value;break;case"Read":v[n.dst]=d.get(v[n.address])??0;break;case"Write":{const key=v[n.address];d.set(key,v[n.value]);if(key===11){assert.equal(v[n.value],1);if(old===12||old===42)trials++;else prod++;}if(key===45){stage=v[n.value];if(stage===s.n){assert.equal(prod,s.n);return {cost:c,prod,trials,probes,observations,edits};}for(const [at,mode] of s.changes)if(stage===at)d.set(50,mode);}break;}case"Sub":v[n.dst]=v[n.lhs]-v[n.rhs];break;case"Add":v[n.dst]=v[n.lhs]+v[n.rhs];break;case"Branch":pc=v[n.test]!==0?n.nonzero:n.zero;break;case"ReadNext":v[n.dst]=g[v[n.node]].next;break;case"PatchNext":if(!off){edits.push([stage,g[v[n.node]].next,v[n.target]]);g[v[n.node]].next=v[n.target];}break;default:throw Error(n.kind)}
 }throw Error("budget");
}
const frozen=JSON.parse(fs.readFileSync(new URL("../studies/CL1B-C28-PAID-BLOCK-MONITOR-frozen-60-worlds-v1.json",import.meta.url),"utf8"));
const expected=[[783,835,879],[785,837,823],[1407,1511,1695],[687,739,687],[1097,1175,1087],[1563,1680,1851],[1179,1270,1459],[1775,1918,2303],[1259,1350,1259],[2113,2269,2407],[2943,3177,3663],[1503,1607,1503]];
const rows=frozen.cases.map((s,i)=>{
 const once=sim(source("once",1),s,false),duplicate=sim(predicted(12,"duplicate"),s,false),shared=sim(predicted(12,"shared"),s,false),duplicateOff=sim(predicted(12,"duplicate"),s,true);
 assert.deepEqual([duplicate.cost,shared.cost,duplicateOff.cost],expected[i]);
 return {id:s.id,cohort:s.cohort,once,duplicate,shared,duplicateOff};
});
console.log(JSON.stringify({tier:"RETROSPECTIVE_NON_NATIVE_MODEL",rows},null,2));
