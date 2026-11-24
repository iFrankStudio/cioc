#!/usr/bin/env node
// C30-R1: non-native shared generated suffix chain; domain fixed in authored grammar.
import fs from "node:fs";
import assert from "node:assert/strict";
function build(kind="generated"){
 const generic=kind==="generic";const g=[],l={};const label=k=>{if(k in l)throw Error(k);l[k]=g.length};
 const op=(kind,args)=>g.push({kind,...args});
 const C=(dst,value,next)=>op("Const",{dst,value,next}),R=(dst,address,next)=>op("Read",{dst,address,next}),W=(address,value,next)=>op("Write",{address,value,next}),S=(dst,lhs,rhs,next)=>op("Sub",{dst,lhs,rhs,next}),A=(dst,lhs,rhs,next)=>op("Add",{dst,lhs,rhs,next}),B=(test,nonzero,zero)=>op("Branch",{test,nonzero,zero}),Q=(dst,next)=>op("SenseOwnCredits",{dst,next}),P=(node,target,next)=>op("PatchNext",{node,target,next}),K=(node,value,next)=>op("PatchConst",{node,value,next});
 const set=(key,value,next)=>{C(3,key);C(0,value);W(3,0,next)};
 label("init");
 if(kind==="plain"||kind==="guard_only"){C(3,0,"entry");}
 else{
  C(3,50);R(0,3);C(3,40);W(3,0);
  if(generic)C(0,16);if(kind==="fault"){C(2,1);S(0,0,2);}
  C(1,"@out");label("generate");C(2,"@parent_add");op("CloneNode",{dst:3,source:2});P(3,1);
  C(2,0);A(1,3,2);C(2,1);S(0,0,2);B(0,"generate","link");
  label("link");if(generic){C(2,16);S(0,1,2);C(1,"@base");K(1,0);}else{C(3,"@special_entry");P(3,1);}set(47,0,"measure");
  label("measure");Q(0);C(3,42);W(3,0,"entry");
 }
 label("entry");C(3,50);R(1,3);C(3,51);R(2,3);C(0,0);
 if(kind!=="plain"){
  label("guard");C(3,0);if(generic){B(3,"base","parent");label("base");C(3,0);A(1,1,3);C(3,"@special_entry");P(3,1);}else{S(3,1,3);B(3,"parent","special_entry");}
  label("special_entry");C(3,1,"parent_add");
 }
 label("parent");C(3,1);
 label("parent_add");A(0,0,2);S(1,1,3);B(1,"parent_add","out");
 label("out");C(3,0,kind==="plain"||kind==="guard_only"?"prod_out":"trial_out");
 if(kind!=="plain"&&kind!=="guard_only"){
  label("trial_out");C(3,44);W(3,0);Q(0);C(3,42);R(1,3);S(0,1,0);C(3,46);W(3,0);
  C(3,47);R(1,3);B(1,"trial_compare","save_parent");
  label("save_parent");C(3,45);W(3,0);C(3,44);R(0,3);C(3,43);W(3,0);
  set(47,1);if(generic)C(0,1);else{C(3,40);R(0,3);}C(1,"@guard");K(1,0,"measure");
  label("trial_compare");C(1,"@guard");C(0,0);K(1,0);
  C(3,43);R(0,3);C(3,44);R(1,3);S(0,0,1);B(0,"finish","compare_cost");
  label("compare_cost");C(3,45);R(0,3);C(3,46);R(1,3);S(2,0,1);S(1,1,0);C(3,1);B(2,"cmp","finish");
  label("cmp");S(2,2,3);B(2,"cmp_neg","install");
  label("cmp_neg");S(1,1,3);B(1,"cmp","finish");
  label("install");if(generic)C(0,1);else{C(3,40);R(0,3);}C(1,"@guard");label("activation");K(1,0,"finish");
  label("finish");C(1,"@out");C(0,"@prod_out");P(1,0,"entry");
 }
 label("prod_out");C(3,11);W(3,0,"entry");
 for(let i=0;i<g.length;i++){const n=g[i];if(n.kind==="Branch"){n.nonzero=l[n.nonzero];n.zero=l[n.zero]}else n.next=n.next===undefined?i+1:l[n.next];if(n.kind==="Const"&&typeof n.value==="string")n.value=l[n.value.slice(1)];}
 return {g,l};
}
function run(s,kind="generated",off=false){
 if(![s.xy,...(s.changes??[]).map(z=>z.slice(1))].every(z=>z.every(x=>Number.isInteger(x)&&x>=1&&x<=16)))throw Error("domain");
 if(kind==="fault"&&s.xy[0]<2)throw Error("fault domain");
 const {g,l}=build(kind),initial=g.length,d=new Map([[50,s.xy[0]],[51,s.xy[1]]]),v=[0,0,0,0];let pc=0,paid=0,peak=g.length+3,prod=0,firstProd=null,cloneTrialSteps=0,cloneProductionSteps=0,firstExposure=null,serviceClones=0;const clones=[],patches=[],trials=[],output=[],prefix=[];
 while(paid<20000){
 const p=pc,n=g[pc];if(!n)throw Error("pc");if(p===l.activation)firstExposure=paid+1;if((kind==="generated"||kind==="generic")&&firstExposure===null&&prod===0)prefix.push([p,[...v],Array.from(d.entries()),g.map(x=>({...x}))]);
 paid++;pc=n.next;
 if(p>=initial){if(g[l.out].next===l.prod_out){cloneProductionSteps++;serviceClones++;}else cloneTrialSteps++;}
 switch(n.kind){
 case "Const":v[n.dst]=n.value;break;
 case "Read":v[n.dst]=d.get(v[n.address])??0;break;
 case "Write":{
 const k=v[n.address],a=v[n.value];d.set(k,a);
 if(k===46){trials.push({output:d.get(44),cost:a,x:d.get(50),y:d.get(51)});}
 if(k===11){
 if(a!==d.get(50)*d.get(51))throw Error("bad production");
 if(kind==="generic"){if(serviceClones!==(g[l.guard].value===1?d.get(50):0))throw Error("wrong generated path length");}serviceClones=0;prod++;output.push(a);if(firstProd===null)firstProd=paid;
 peak=Math.max(peak,g.length+1+d.size);
 if(prod===s.n)return {paid,initial,nodes:g.length,peak,firstProd,clones,patches,trials,output,active:g[l.guard]?.value??null,finalSource:g,cloneTrialSteps,cloneProductionSteps,firstExposure,prefix};
 for(const [at,x,y] of s.changes??[])if(at===prod){d.set(50,x);d.set(51,y);}
 }break;}
 case "SenseOwnCredits":v[n.dst]=20000-paid;break;
 case "Add":v[n.dst]=v[n.lhs]+v[n.rhs];break;
 case "Sub":v[n.dst]=v[n.lhs]-v[n.rhs];break;
 case "Branch":pc=v[n.test]!==0?n.nonzero:n.zero;break;
 case "CloneNode":{const src=v[n.source],idx=g.length;g.push({...g[src]});v[n.dst]=idx;clones.push([paid,src,idx]);break;}
 case "PatchNext":{const target=v[n.node],before=g[target].next,after=v[n.target];if(kind==="generic"&&target===l.special_entry&&after!==initial+d.get(50)-1)throw Error("wrong generated address");if(!g[after]||g[target].next===undefined)throw Error("edge");g[target].next=after;patches.push([paid,p,target,before,after,false]);break;}
 case "PatchConst":{const target=v[n.node],before=g[target].value,after=v[n.value],suppress=off&&p===l.activation;if(g[target].kind!=="Const")throw Error("const");if(!suppress)g[target].value=after;patches.push([paid,p,target,before,after,suppress]);break;}
 default:throw Error(n.kind);}
 peak=Math.max(peak,g.length+1+d.size);if(g.length>200||d.size>16||peak>220)throw Error("capacity");
 }throw Error("budget");
}
const cases=JSON.parse(fs.readFileSync(new URL("../studies/CL1B-C30-EXECUTABLE-SPECIALIZATION-frozen-67-worlds-v1.json",import.meta.url),"utf8")).cases;

function test(s){return {id:s.id,group:s.group??s.cohort,original:run(s),generic:run(s,"generic"),off:run(s,"generic",true),plain:run(s,"plain")}}
function check(r){
 if(r.generic.output.join()!==r.plain.output.join()||r.off.output.join()!==r.plain.output.join())throw Error("product");
 if(r.generic.firstExposure!==r.off.firstExposure||JSON.stringify(r.generic.prefix)!==JSON.stringify(r.off.prefix))throw Error("prefix");
 if(r.generic.trials[0].output!==r.generic.trials[1].output||r.generic.clones.length!==16)throw Error("trial");
 if(r.generic.active===1&&r.generic.cloneProductionSteps===0||r.off.cloneProductionSteps!==0)throw Error("activation");
}
const rows=cases.map(test);
function agg(a){return {n:a.length,original:a.reduce((s,r)=>s+r.plain.paid-r.original.paid,0),generic:a.reduce((s,r)=>s+r.plain.paid-r.generic.paid,0),physical:a.reduce((s,r)=>s+r.off.paid-r.generic.paid,0),wins:a.filter(r=>r.generic.paid<r.plain.paid).length,accepted:a.filter(r=>r.generic.active===1).length,peak:Math.max(...a.map(r=>r.generic.peak)),nodes:Math.max(...a.map(r=>r.generic.nodes))}}
const family=[];
for(let x=1;x<=16;x++)for(const n of [24,96,144]){
 family.push({id:"stable_"+x+"_"+n,group:"stable",xy:[x,2],n,changes:[[5,x,13]]});
 family.push({id:"shift_"+x+"_"+n,group:"shift",xy:[x,2],n,changes:[[3,17-x,7]]});
 family.push({id:"cycle_"+x+"_"+n,group:"cycle",xy:[x,2],n,changes:Array.from({length:Math.floor((n-1)/4)},(_,i)=>[4*(i+1),1+(x+7*(i+1)-1)%16,1+i%16])});
}
const grid=family.map(test);
for(const r of [...rows,...grid])check(r);
const pairs=[];for(let x=1;x<=16;x++)for(let y=1;y<=16;y++)pairs.push([x,y]);
const exhaustive={id:"exhaustive_domain",xy:[16,16],n:257,changes:pairs.map(([x,y],i)=>[i+1,x,y])};
const full=run(exhaustive,"generic");
if(full.output.length!==257||full.active!==1||full.cloneProductionSteps!==2192)throw Error("domain check");
const summary={known:agg(rows.filter(r=>r.group==="prospective")),grid:agg(grid),groups:Object.fromEntries(["stable","shift","cycle"].map(k=>[k,agg(grid.filter(r=>r.group===k))])),exhaustive:{n:full.output.length,distinctPairs:256,paid:full.paid,nodes:full.nodes,peak:full.peak,cloneProductionSteps:full.cloneProductionSteps}};
for(const r of [...rows,...grid])for(const k of ["original","generic","off","plain"]){delete r[k].prefix;delete r[k].finalSource;delete r[k].output;}

assert.deepEqual(summary,{"known":{"n":12,"original":3870,"generic":4440,"physical":10030,"wins":8,"accepted":10,"peak":120,"nodes":109},"grid":{"n":144,"original":5040,"generic":74794,"physical":144760,"wins":87,"accepted":126,"peak":120,"nodes":109},"groups":{"stable":{"n":48,"original":46560,"generic":32118,"physical":55440,"wins":32,"accepted":42,"peak":120,"nodes":109},"shift":{"n":48,"original":-22800,"generic":17838,"physical":41160,"wins":25,"accepted":42,"peak":120,"nodes":109},"cycle":{"n":48,"original":-18720,"generic":24838,"physical":48160,"wins":30,"accepted":42,"peak":120,"nodes":109}},"exhaustive":{"n":257,"distinctPairs":256,"paid":6447,"nodes":109,"peak":120,"cloneProductionSteps":2192}});
console.log(JSON.stringify({tier:"NON_NATIVE_SHARED_EXECUTABLE_CHAIN_MODEL",summary,rows,grid,sources:Object.fromEntries(["generated","generic","plain"].map(k=>[k,build(k)]))},null,2));
