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
function run(s,policy,interval,suppress=false,variant="compact"){
 const original=variant==='shared'?shared(policy,interval):source(policy,interval), pack=compact(original), use=variant!=='raw', g=use?pack.nodes:original, norm=p=>use?pack.old[p]:p,v=[0,0,0,0],d=new Map([[50,s.mode]]);let pc=0,c=0,prod=0,trials=0,stage=0;const probes=[],observations=[],edits=[],trace=[];
 while(c<20000){const n=g[pc],old=norm(pc);assert(n);c++;trace.push(old);pc=n.next;
 if(old===0)probes.push(stage);if(old===215)observations.push([stage,v[1],v[3]]);
 switch(n.kind){case"SenseOwnCredits":v[n.dst]=20000-c;break;case"Const":v[n.dst]=n.value;break;case"Read":v[n.dst]=d.get(v[n.address])??0;break;case"Write":{const key=v[n.address];d.set(key,v[n.value]);if(key===11){assert.equal(v[n.value],1);if(old===12||old===42)trials++;else prod++;}if(key===45){stage=v[n.value];if(stage===s.n){assert.equal(prod,s.n);return {cost:c,prod,trials,probes,observations,edits,nodes:g.length,trace};}for(const [at,mode] of s.changes)if(stage===at)d.set(50,mode);}break;}case"Sub":v[n.dst]=v[n.lhs]-v[n.rhs];break;case"Add":v[n.dst]=v[n.lhs]+v[n.rhs];break;case"Branch":pc=v[n.test]!==0?n.nonzero:n.zero;break;case"ReadNext":v[n.dst]=g[v[n.node]].next;break;case"PatchNext":if(!suppress){edits.push([stage,norm(g[v[n.node]].next),norm(v[n.target])]);g[v[n.node]].next=v[n.target];}break;default:throw Error(n.kind)}
 }throw Error("budget");
}
const cases=[{id:"early",mode:0,n:48,changes:[[2,1]]},{id:"middle",mode:0,n:48,changes:[[17,1]]},{id:"reverse",mode:1,n:48,changes:[[17,0]]},{id:"late",mode:0,n:48,changes:[[43,1]]},{id:"stable0",mode:0,n:48,changes:[]},{id:"stable1",mode:1,n:48,changes:[]},{id:"transient",mode:0,n:48,changes:[[19,1],[21,0]]},{id:"twice",mode:1,n:96,changes:[[19,0],[55,1]]},{id:"long",mode:0,n:96,changes:[[17,1]]},{id:"longer",mode:0,n:240,changes:[[17,1]]}];

const rows=cases.map(s=>({id:s.id,intervals:[6,12,24].map(k=>{
 const raw=run(s,"monitor",k,false,"raw"),c=run(s,"monitor",k),off=run(s,"monitor",k,true),rawOff=run(s,"monitor",k,true,"raw"),h=run(s,"monitor",k,false,"shared"),ho=run(s,"monitor",k,true,"shared"),base=run(s,"once",k);
 for(const [a,b] of [[raw,c],[rawOff,off]])for(const key of ["cost","prod","trials","probes","observations","edits","trace"])assert.deepEqual(a[key],b[key]);
 assert.deepEqual(h.probes,c.probes);assert.deepEqual(h.edits,c.edits);
 assert.deepEqual(ho.probes,off.probes);assert.deepEqual(ho.edits,off.edits);
 assert.equal(raw.nodes,215+32*k);assert.equal(c.nodes,97+32*k);assert.equal(h.nodes,100+17*k);
 return {k,raw:raw.nodes,compact:c.nodes,shared:h.nodes,cost:c.cost,sharedCost:h.cost,sharedOff:ho.cost,compactOff:off.cost,net:base.cost-h.cost,compactNet:base.cost-c.cost,base:base.cost,baseNodes:base.nodes,probes:h.probes,edits:h.edits};
})}));
console.log(JSON.stringify({tier:"RETROSPECTIVE_NON_NATIVE_MODEL",rows},null,2));
