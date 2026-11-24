#!/usr/bin/env node
// Retrospective engineering prototype, no new native worlds.
import fs from "node:fs";
import assert from "node:assert/strict";
const manifest=JSON.parse(fs.readFileSync(new URL("../studies/CL1B-C27-PAID-CREDIT-REMEASURE-ROLLBACK-frozen-8-worlds-v1.json",import.meta.url),"utf8"));
export function source(kind){
 const g=structuredClone(manifest.original_full_frozen_170_node_CIL_source);
 if(kind==="old")return g;
 if(kind==="compact"){
  const n=(p,x)=>g[p]=x;
  n(0,{kind:"SenseOwnCredits",dst:0,next:3});
  n(30,{kind:"SenseOwnCredits",dst:3,next:31});
  n(31,{kind:"Sub",dst:0,lhs:0,rhs:3,next:32});
  n(32,{kind:"Const",dst:2,value:41,next:33});
  n(33,{kind:"Write",address:2,value:0,next:34});
  n(34,{kind:"SenseOwnCredits",dst:0,next:35});
  n(35,{kind:"Const",dst:2,value:50,next:36});
  n(36,{kind:"Read",dst:3,address:2,next:37});
  n(37,{kind:"Branch",test:3,nonzero:40,zero:50});
  n(60,{kind:"SenseOwnCredits",dst:3,next:61});
  n(61,{kind:"Sub",dst:1,lhs:0,rhs:3,next:62});
  n(62,{kind:"Const",dst:2,value:41,next:63});
  n(63,{kind:"Read",dst:0,address:2,next:73});
 }
 g[73]={kind:"Sub",dst:3,lhs:0,rhs:1,next:74};
 g[74]={kind:"Sub",dst:1,lhs:1,rhs:0,next:75};
 g[75]={kind:"Const",dst:2,value:1,next:76};
 g[76]={kind:"Branch",test:3,nonzero:170,zero:100};
 for(let i=0;i<8;i++){
  const p=170+4*i;
  g.push({kind:"Sub",dst:3,lhs:3,rhs:2,next:p+1},
    {kind:"Branch",test:3,nonzero:p+2,zero:90},
    {kind:"Sub",dst:1,lhs:1,rhs:2,next:p+3},
    {kind:"Branch",test:1,nonzero:i===7?100:p+4,zero:80});
 }
 return g;
}
function compare(a,b){
 const g=source("compact"),v=[a,b,0,0];let pc=73,c=0;
 while(![80,90,100].includes(pc)&&c++<100){
  const n=g[pc];pc=n.next;
  if(n.kind==="Sub")v[n.dst]=v[n.lhs]-v[n.rhs];
  else if(n.kind==="Const")v[n.dst]=n.value;
  else if(n.kind==="Branch")pc=v[n.test]!==0?n.nonzero:n.zero;
  else throw Error(n.kind);
 }
 assert([80,90,100].includes(pc));return pc;
}
for(let a=1;a<=32;a++)for(let b=1;b<=32;b++){
 const diff=a-b;
 assert.equal(compare(a,b),diff>0&&diff<=8?90:diff<0&&diff>=-8?80:100);
}
function run(a,kind,oneShot=false){
 const g=source(kind);if(oneShot)g[162].zero=100;
 const v=[0,0,0,0],d=new Map([[50,a.initial_mode]]);
 let pc=0,c=0,prod=0,trials=0;const edits=[],measures=[];
 while(c<3000){
  const n=g[pc],old=pc;assert(n);c++;pc=n.next;
  if(old===73)measures.push([v[0],v[1]]);
  switch(n.kind){
   case "SenseOwnCredits":v[n.dst]=3000-c;break;
   case "Const":v[n.dst]=n.value;break;
   case "Read":v[n.dst]=d.get(v[n.address])??0;break;
   case "Write":{
    const key=v[n.address];d.set(key,v[n.value]);
    if(key===11){assert.equal(v[n.value],1);if(old===12||old===42)trials++;else prod++;}
    if(key===45&&v[n.value]===12&&a.mode_after_stage12!==null)d.set(50,a.mode_after_stage12);
    break;
   }
   case "Sub":v[n.dst]=v[n.lhs]-v[n.rhs];break;
   case "Add":v[n.dst]=v[n.lhs]+v[n.rhs];break;
   case "Branch":pc=v[n.test]!==0?n.nonzero:n.zero;break;
   case "ReadNext":v[n.dst]=g[v[n.node]].next;break;
   case "PatchNext":
    if(!a.patchnext_suppressed){edits.push([old,g[v[n.node]].next,v[n.target]]);g[v[n.node]].next=v[n.target];}
    break;
   case "Halt":
    assert.equal(prod,24);assert.equal(d.get(45),24);assert.equal(trials,oneShot?2:4);
    return {cost:c,prod,trials,edits,measures};
   default:throw Error(n.kind);
  }
 }
 throw Error("budget");
}
const original=[625,673,627,673,626,626,624,720],bounded=[552,600,554,600,556,556,548,644],compact=[534,582,536,582,538,538,530,626],once=[532,532,528,528,484,484,480,576];
const rows=manifest.arms.map((a,i)=>{
 const o=run(a,"old"),b=run(a,"bounded"),c=run(a,"compact"),s=run(a,"compact",true);
 assert.equal(o.cost,original[i]);assert.equal(b.cost,bounded[i]);assert.equal(c.cost,compact[i]);assert.equal(s.cost,once[i]);
 assert.deepEqual(c.edits,o.edits);assert.deepEqual(b.edits,o.edits);
 assert.deepEqual(c.measures[0],a.initial_mode===0?[7,11]:[11,7]);
 const mode=a.mode_after_stage12??a.initial_mode;
 assert.deepEqual(c.measures[1],mode===0?[7,11]:[11,7]);
 return {id:a.id,old:o,bounded:b,compact:c,compactOneShot:s};
});
console.log(JSON.stringify({tier:"NON_NATIVE_RETROSPECTIVE_MODEL",nodes:source("compact").length,comparatorCases:1024,rows},null,2));
