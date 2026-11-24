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
 if([93,94,163,165,168,...(g.extraAddressConsts??[])].includes(p))n.value=map.get(n.value);
 return n;});
 return {nodes,old,map};
}
function confirm(k,variant){
 assert.equal(k,12); // framing constant below is specific to this topology.
 const g=source("monitor",k);
 const add=n=>(g.push(n),g.length-1);
 if(variant==="full"){
  const reset=add(C(0,0,206));g[204].next=reset;g[218].next=reset;
  const pending=add(C(0,1,206)),decide=add(B(0,0,pending));g[217]=B(3,decide,reset);
 }else{
  // Paid single ordinary production service, selected using live executable edge.
  let p=add(B(3,0,206));p=add(S(3,3,1,p));p=add(R(3,2,p));p=add(C(2,47,p));
  p=add(A(1,1,2,p));p=add(C(2,-45,p)); // sample span 16/20; normalized block cost 12*t-45
  p=add(A(1,1,3,p));p=add(A(1,3,3,p));p=add(A(3,1,1,p));p=add(A(1,1,1,p));
  p=add(S(1,1,3,p));p=add(R(1,2,p));p=add(C(2,46,p));const end=add(Q(3,p));
  p=add(W(2,1,end));p=add(C(2,45,p));p=add(A(1,1,2,p));p=add(C(2,1,p));p=add(R(1,2,p));p=add(C(2,45,p));
  p=add(W(2,1,p));p=add(C(1,1,p));const cheap=add(C(2,11,p));let costly=cheap;for(let j=0;j<4;j++)costly=add(C(2,0,costly));
  p=add(B(3,costly,cheap));p=add(S(3,3,0,p));p=add(R(3,2,p));p=add(C(2,50,p));p=add(W(2,3,p));p=add(C(2,46,p));p=add(Q(3,p));
  const aa=add(C(0,0,p)),bb=add(C(0,1,p));p=add(B(3,bb,aa));p=add(S(3,3,2,p));const addr=add(C(2,101,p));p=add({kind:"ReadNext",dst:3,node:1,next:addr});const dispatch=add(C(1,100,p));
  g[217]=B(3,dispatch,206);g.extraAddressConsts=[addr,dispatch];
 }
 return g;
}
function run(s,policy,interval,suppress=false,variant="compact"){
 const original=variant==='full'||variant==='short'?confirm(interval,variant):source(policy,interval), pack=compact(original), use=variant!=='raw', g=use?pack.nodes:original, norm=p=>use?pack.old[p]:p,v=[0,0,0,0],d=new Map([[50,s.mode]]);let pc=0,c=0,prod=0,trials=0,stage=0;const probes=[],observations=[],edits=[],trace=[];
 while(c<20000){const n=g[pc],old=norm(pc);assert(n);c++;trace.push(old);pc=n.next;
 if(old===0)probes.push(stage);if(old===215)observations.push([stage,v[1],v[3]]);
 switch(n.kind){case"SenseOwnCredits":v[n.dst]=20000-c;break;case"Const":v[n.dst]=n.value;break;case"Read":v[n.dst]=d.get(v[n.address])??0;break;case"Write":{const key=v[n.address];d.set(key,v[n.value]);if(key===11){assert.equal(v[n.value],1);if(old===12||old===42)trials++;else prod++;}if(key===45){stage=v[n.value];if(stage===s.n){assert.equal(prod,s.n);return {cost:c,prod,trials,probes,observations,edits,nodes:g.length};}for(const [at,mode] of s.changes)if(stage===at)d.set(50,mode);}break;}case"Sub":v[n.dst]=v[n.lhs]-v[n.rhs];break;case"Add":v[n.dst]=v[n.lhs]+v[n.rhs];break;case"Branch":pc=v[n.test]!==0?n.nonzero:n.zero;break;case"ReadNext":v[n.dst]=g[v[n.node]].next;break;case"PatchNext":if(!suppress){edits.push([stage,norm(g[v[n.node]].next),norm(v[n.target])]);g[v[n.node]].next=v[n.target];}break;default:throw Error(n.kind)}
 }throw Error("budget");
}
const cases=JSON.parse(fs.readFileSync(new URL("../studies/CL1B-C28-PAID-BLOCK-MONITOR-frozen-60-worlds-v1.json",import.meta.url),"utf8")).cases;
const expected=[[754,751,806,779,871],[750,753,808,781,815],[1522,1375,1433,1403,1687],[630,671,675,671,671],[1182,1113,1169,1141,1079],[1666,1531,1590,1559,1843],[1086,1163,1170,1163,1443],[1634,1759,1770,1759,2287],[1094,1227,1179,1195,1195],[2018,2065,2177,2125,2407],[3370,2911,2979,2939,3655],[1254,1455,1410,1447,1447]];
const rows=cases.map((s,i)=>{
 const once=run(s,"once",1),original=run(s,"monitor",12),full=run(s,"monitor",12,false,"full"),fullOff=run(s,"monitor",12,true,"full"),short=run(s,"monitor",12,false,"short"),shortOff=run(s,"monitor",12,true,"short");
 assert.deepEqual([once.cost,original.cost,full.cost,short.cost,shortOff.cost],expected[i]);
 assert.equal(full.nodes,484);assert.equal(short.nodes,522);
 return {id:s.id,cohort:s.cohort,once,original,full,fullOff,short,shortOff};
});
const totals=Object.fromEntries(["original","full","short"].map(k=>[k,rows.filter(r=>r.cohort==="prospective").reduce((t,r)=>t+r.once.cost-r[k].cost,0)]));
assert.deepEqual(totals,{original:80,full:-140,short:-24});
console.log(JSON.stringify({tier:"RETROSPECTIVE_NON_NATIVE_MODEL",totals,rows},null,2));
