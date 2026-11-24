#!/usr/bin/env node
// Retrospective model only; no Rust or independent native replay.
import fs from "node:fs";
import assert from "node:assert/strict";
const m=JSON.parse(fs.readFileSync(new URL("../studies/CL1B-C27-PAID-CREDIT-REMEASURE-ROLLBACK-frozen-8-worlds-v1.json",import.meta.url),"utf8"));
const C=(dst,value,next)=>({kind:"Const",dst,value,next}), S=(dst,lhs,rhs,next)=>({kind:"Sub",dst,lhs,rhs,next}), B=(test,nonzero,zero)=>({kind:"Branch",test,nonzero,zero}), R=(dst,address,next)=>({kind:"Read",dst,address,next}), W=(address,value,next)=>({kind:"Write",address,value,next}), Q=(dst,next)=>({kind:"SenseOwnCredits",dst,next});
function source(policy,interval){
 const g=JSON.parse(JSON.stringify(m.original_full_frozen_170_node_CIL_source));
 Object.assign(g,{0:Q(0,3),30:Q(3,31),31:S(0,0,3,32),32:C(2,41,33),33:W(2,0,34),34:Q(0,35),35:C(2,50,36),36:R(3,2,37),37:B(3,40,50),60:Q(3,61),61:S(1,0,3,62),62:C(2,41,63),63:R(0,2,73),73:S(3,0,1,74),74:S(1,1,0,75),75:C(2,1,76),76:B(3,170,100)});
 for(let i=0;i<8;i++){const p=170+4*i;g.push(S(3,3,2,p+1),B(3,p+2,90),S(1,1,2,p+3),B(1,i===7?100:p+4,80));}
 if(policy==="once"){g[155].next=100;return g;}
 if(policy==="fixed12"){g[155].next=160;return g;}
 g[76].zero=202;g[201].nonzero=202;g[95].next=202;g[167].zero=202;g[169].next=202;
 if(policy==="monitor3"){g[155].next=160;g[160]=C(2,1,161);g[161]=S(0,0,2,162);g[162]=B(0,100,209);}
 else {g[153].next=156;g[156]=S(0,0,2,154);g[155].next=157;g[157]=B(0,100,209);}
 g.push(C(2,47,203),C(1,0,204),W(2,1,205),C(0,interval,206),Q(3,207),C(2,46,208),W(2,3,100),Q(3,210),C(2,46,211),R(1,2,212),S(1,1,3,213),C(2,47,214),R(3,2,215),B(3,216,218),S(3,3,1,217),B(3,0,205),W(2,1,205));
 return g;
}
function run(s,policy,interval,suppress=false){
 const g=source(policy,interval),v=[0,0,0,0],d=new Map([[50,s.mode]]);let pc=0,c=0,prod=0,trials=0,stage=0;const probes=[],observations=[],edits=[];
 while(c<10000){const n=g[pc],old=pc;assert(n);c++;pc=n.next;
 if(old===0)probes.push(stage);if(old===215)observations.push([stage,v[1],v[3]]);
 switch(n.kind){case"SenseOwnCredits":v[n.dst]=10000-c;break;case"Const":v[n.dst]=n.value;break;case"Read":v[n.dst]=d.get(v[n.address])??0;break;case"Write":{const key=v[n.address];d.set(key,v[n.value]);if(key===11){assert.equal(v[n.value],1);if(old===12||old===42)trials++;else prod++;}if(key===45){stage=v[n.value];if(stage===s.n){assert.equal(prod,s.n);return {cost:c,prod,trials,probes,observations,edits};}for(const [at,mode] of s.changes)if(stage===at)d.set(50,mode);}break;}case"Sub":v[n.dst]=v[n.lhs]-v[n.rhs];break;case"Add":v[n.dst]=v[n.lhs]+v[n.rhs];break;case"Branch":pc=v[n.test]!==0?n.nonzero:n.zero;break;case"ReadNext":v[n.dst]=g[v[n.node]].next;break;case"PatchNext":if(!suppress){edits.push([stage,g[v[n.node]].next,v[n.target]]);g[v[n.node]].next=v[n.target];}break;default:throw Error(n.kind)}
 }throw Error("budget");
}
const cases=[{id:"early",mode:0,n:48,changes:[[2,1]]},{id:"middle",mode:0,n:48,changes:[[17,1]]},{id:"reverse",mode:1,n:48,changes:[[17,0]]},{id:"late",mode:0,n:48,changes:[[43,1]]},{id:"stable0",mode:0,n:48,changes:[]},{id:"stable1",mode:1,n:48,changes:[]},{id:"transient",mode:0,n:48,changes:[[19,1],[21,0]]},{id:"twice",mode:1,n:96,changes:[[19,0],[55,1]]},{id:"long",mode:0,n:96,changes:[[17,1]]},{id:"longer",mode:0,n:240,changes:[[17,1]]}];
const expected=[[814,861,960,914,910,928],[754,949,924,878,850,868],[750,941,926,880,852,864],[650,845,888,842,790,764],[630,825,868,822,770,744],[626,817,864,818,766,740],[638,833,932,886,834,752],[1346,1681,1838,1744,1640,1776],[1522,1861,1748,1654,1574,1618],[3826,4597,4220,3982,3746,3712]];
const rows=cases.map((s,i)=>{
 const once=run(s,"once",6),fixed12=run(s,"fixed12",6),monitor3=run(s,"monitor3",6),intervals=[6,12,24].map(k=>({k,on:run(s,"monitor2",k),off:run(s,"monitor2",k,true)}));
 assert.deepEqual([once.cost,fixed12.cost,monitor3.cost,...intervals.map(x=>x.on.cost)],expected[i]);
 return {scenario:s,once,fixed12,monitor3,intervals};
});
console.log(JSON.stringify({tier:"RETROSPECTIVE_NON_NATIVE_MODEL",nodes:219,rows},null,2));
