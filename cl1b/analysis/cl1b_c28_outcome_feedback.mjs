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
function adaptive(k,cap,fixed){
 const g=source("monitor",k),add=n=>(g.push(n),g.length-1);
 const gate=g.length;g.push(B(0,gate+1,209),C(2,1,gate+2),S(0,0,2,gate+3),B(0,100,206));
 for(let i=219;i<gate;i++)if(g[i].next===209)g[i].next=gate;
 const reset=add(C(0,0,206)),init=add(W(2,0,reset)),addr=add(C(2,48,init)),zero=add(C(0,0,addr));
 g[204].next=zero;
 let eq;
 if(fixed!==null){eq=add(C(0,fixed,fixed?100:206));}
 else{
  const write=add(W(2,0,100)),key=add(C(2,48,write)),inc=add(C(0,1,key));
  const dbl=add(A(0,0,0,key)),limit=add(C(0,cap,key));
  const cmp=add(B(3,dbl,limit)),sub=add(S(3,0,3,cmp)),c=add(C(3,cap,sub));
  const test=add(B(0,c,inc)),read=add(R(0,2,test));eq=add(C(2,48,read));
 }
 g[217]=B(3,0,eq);
 return g;
}

function feedback(k,initial,success,failure,criterion){
 const g=source("monitor",k),add=n=>(g.push(n),g.length-1);
 const gate=g.length;g.push(B(0,gate+1,209),C(2,1,gate+2),S(0,0,2,gate+3),B(0,100,206));
 for(let i=219;i<gate;i++)if(g[i].next===209)g[i].next=gate;
 g[204].next=add(C(0,0,206));
 const save=add(W(2,1,0)),key=add(C(2,49,save));
 const decide=add(B(0,100,206)),read=add(R(0,2,decide)),readkey=add(C(2,48,read));
 g[217]=B(3,key,readkey);
 const clear=add(C(0,0,206)),store=add(W(2,0,clear)),policykey=add(C(2,48,store));
 const good=add(C(0,success,policykey)),bad=add(C(0,failure,policykey)),first=add(C(0,initial,policykey));
 let scan=bad;
 for(let i=12;i>=1;i--){
  let br=add(B(3,scan,i>=criterion?good:bad));scan=add(S(3,3,2,br));
 }
 const diff=add(S(3,3,1,add(C(2,4,scan))));
 const prior=add(B(3,diff,first)),load=add(R(3,2,prior)),addr=add(C(2,49,load));
 g[218].next=addr;
 return g;
}
function run(s,policy,interval,suppress=false,variant="compact",cap=4,fixed=null,criterion=1){
 const original=variant==='feedback'?feedback(interval,1,cap,fixed,criterion):variant==='adaptive'?adaptive(interval,cap,fixed):source(policy,interval), pack=compact(original), use=variant!=='raw', g=use?pack.nodes:original, norm=p=>use?pack.old[p]:p,v=[0,0,0,0],d=new Map([[50,s.mode]]);let pc=0,c=0,prod=0,trials=0,stage=0;const probes=[],observations=[],edits=[],outcomes=[];
 while(c<20000){const n=g[pc],old=norm(pc);assert(n);c++;pc=n.next;
 if(old===0)probes.push(stage);if(old===215)observations.push([stage,v[1],v[3]]);
 switch(n.kind){case"SenseOwnCredits":v[n.dst]=20000-c;break;case"Const":v[n.dst]=n.value;break;case"Read":v[n.dst]=d.get(v[n.address])??0;break;case"Write":{const key=v[n.address];d.set(key,v[n.value]);if(key===48&&variant==='feedback')outcomes.push({stage,prior:d.get(49)??0,post:d.get(47),skip:v[n.value],edge:norm(g[pack.map.get(100)].next)});if(key===11){assert.equal(v[n.value],1);if(old===12||old===42)trials++;else prod++;}if(key===45){stage=v[n.value];if(stage===s.n){assert.equal(prod,s.n);return {cost:c,prod,trials,probes,observations,edits,outcomes,nodes:g.length};}for(const [at,mode] of s.changes)if(stage===at)d.set(50,mode);}break;}case"Sub":v[n.dst]=v[n.lhs]-v[n.rhs];break;case"Add":v[n.dst]=v[n.lhs]+v[n.rhs];break;case"Branch":pc=v[n.test]!==0?n.nonzero:n.zero;break;case"ReadNext":v[n.dst]=g[v[n.node]].next;break;case"PatchNext":if(!suppress){edits.push([stage,norm(g[v[n.node]].next),norm(v[n.target])]);g[v[n.node]].next=v[n.target];}break;default:throw Error(n.kind)}
 }throw Error("budget");
}
const cases=JSON.parse(fs.readFileSync(new URL("../studies/CL1B-C28-PAID-BLOCK-MONITOR-frozen-60-worlds-v1.json",import.meta.url),"utf8")).cases;

const policies=[["once",1,"compact",4,null,1],["original",12,"compact",4,null,1],["skip1",12,"adaptive",4,1,1],["skip3",12,"adaptive",4,3,1],["feedback1",12,"feedback",0,3,1],["feedback6",12,"feedback",0,3,6],["feedback_sparse",12,"feedback",1,3,1]];
function test(s){return {scenario:s,results:Object.fromEntries(policies.map(([id,k,v,cap,f,crit])=>{const on=run(s,id==="once"?"once":"monitor",k,false,v,cap,f,crit),off=id==="once"?null:run(s,"monitor",k,true,v,cap,f,crit);return [id,{on:on.cost,off:off?.cost??null,nodes:on.nodes,probes:on.probes,observations:on.observations,edits:on.edits,outcomes:on.outcomes,offOutcomes:off?.outcomes??[]}]}))}}
function family(){
 const out=[];
 for(const mode of [0,1]){
  const n=144;
  out.push({id:"stable_"+mode,group:"stable",mode,n,changes:[]});
  for(let t=1;t<n;t++)out.push({id:"single_"+mode+"_"+t,group:"single",mode,n,changes:[[t,1-mode]]});
  for(let t=6;t<=126;t+=6)for(const w of [1,4,12])out.push({id:"transient_"+mode+"_"+t+"_"+w,group:"transient",mode,n,changes:[[t,1-mode],[t+w,mode]]});
  for(let t=12;t<=72;t+=12)for(const w of [24,48])out.push({id:"reverse_"+mode+"_"+t+"_"+w,group:"reverse",mode,n,changes:[[t,1-mode],[t+w,mode]]});
 }
 return out;
}

function aggregate(rows){return Object.fromEntries(policies.map(([id])=>{const data=rows.map(r=>{const q=r.results[id],base=r.results.once.on;return {net:base-q.on,physical:q.off===null?0:q.off-q.on}});return [id,{n:data.length,net:data.reduce((t,q)=>t+q.net,0),wins:data.filter(q=>q.net>0).length,losses:data.filter(q=>q.net<0).length,worst:Math.min(...data.map(q=>q.net)),physical:data.reduce((t,q)=>t+q.physical,0),nodes:rows[0].results[id].nodes}]}))}
const known=cases.map(test),grid=family().map(test);
const summary={known:aggregate(known.filter(r=>r.scenario.cohort==="prospective")),grid:aggregate(grid),groups:Object.fromEntries(["stable","single","transient","reverse"].map(k=>[k,aggregate(grid.filter(r=>r.scenario.group===k))]))};
// These are actor-visible cost decreases, not verified causal gains.
const falseCredit=grid.flatMap(r=>r.results.feedback1.offOutcomes.filter(x=>x.prior>0&&x.skip===0).map(x=>({case:r.scenario.id,...x})));
const knownAlias=known.find(r=>r.scenario.id==="new_alias").results.feedback1;

assert.equal(grid.length,438);
assert.equal(summary.grid.feedback1.net,-26984);
assert.equal(summary.grid.feedback6.net,-23852);
assert.equal(summary.grid.feedback_sparse.net,-19480);
console.log(JSON.stringify({tier:"RETROSPECTIVE_NON_NATIVE_MODEL",summary,knownAlias,falseCredit,known,grid},null,2));
