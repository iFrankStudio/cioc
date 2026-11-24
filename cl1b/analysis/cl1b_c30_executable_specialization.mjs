#!/usr/bin/env node
// C30-R0: non-native model of paid executable specialization.
import assert from "node:assert/strict";

function build(kind="generated"){
 const g=[],l={};const label=k=>{if(k in l)throw Error(k);l[k]=g.length};
 const op=(kind,args)=>g.push({kind,...args});
 const C=(dst,value,next)=>op("Const",{dst,value,next}),R=(dst,address,next)=>op("Read",{dst,address,next}),W=(address,value,next)=>op("Write",{address,value,next}),S=(dst,lhs,rhs,next)=>op("Sub",{dst,lhs,rhs,next}),A=(dst,lhs,rhs,next)=>op("Add",{dst,lhs,rhs,next}),B=(test,nonzero,zero)=>op("Branch",{test,nonzero,zero}),Q=(dst,next)=>op("SenseOwnCredits",{dst,next}),P=(node,target,next)=>op("PatchNext",{node,target,next}),K=(node,value,next)=>op("PatchConst",{node,value,next});
 const set=(key,value,next)=>{C(3,key);C(0,value);W(3,0,next)};
 label("init");
 if(kind==="plain"||kind==="guard_only"){C(3,0,"entry");}
 else{
  C(3,50);R(0,3);C(3,40);W(3,0);
  if(kind==="fault"){C(2,1);S(0,0,2);}
  C(1,"@out");label("generate");C(2,"@parent_add");op("CloneNode",{dst:3,source:2});P(3,1);
  C(2,0);A(1,3,2);C(2,1);S(0,0,2);B(0,"generate","link");
  label("link");C(3,"@special_entry");P(3,1);set(47,0,"measure");
  label("measure");Q(0);C(3,42);W(3,0,"entry");
 }
 label("entry");C(3,50);R(1,3);C(3,51);R(2,3);C(0,0);
 if(kind!=="plain"){
  label("guard");C(3,0);S(3,1,3);B(3,"parent","special_entry");
  label("special_entry");C(3,1,"parent_add");
 }
 label("parent");C(3,1);
 label("parent_add");A(0,0,2);S(1,1,3);B(1,"parent_add","out");
 label("out");C(3,0,kind==="plain"||kind==="guard_only"?"prod_out":"trial_out");
 if(kind!=="plain"&&kind!=="guard_only"){
  label("trial_out");C(3,44);W(3,0);Q(0);C(3,42);R(1,3);S(0,1,0);C(3,46);W(3,0);
  C(3,47);R(1,3);B(1,"trial_compare","save_parent");
  label("save_parent");C(3,45);W(3,0);C(3,44);R(0,3);C(3,43);W(3,0);
  set(47,1);C(3,40);R(0,3);C(1,"@guard");K(1,0,"measure");
  label("trial_compare");C(1,"@guard");C(0,0);K(1,0);
  C(3,43);R(0,3);C(3,44);R(1,3);S(0,0,1);B(0,"finish","compare_cost");
  label("compare_cost");C(3,45);R(0,3);C(3,46);R(1,3);S(2,0,1);S(1,1,0);C(3,1);B(2,"cmp","finish");
  label("cmp");S(2,2,3);B(2,"cmp_neg","install");
  label("cmp_neg");S(1,1,3);B(1,"cmp","finish");
  label("install");C(3,40);R(0,3);C(1,"@guard");label("activation");K(1,0,"finish");
  label("finish");C(1,"@out");C(0,"@prod_out");P(1,0,"entry");
 }
 label("prod_out");C(3,11);W(3,0,"entry");
 for(let i=0;i<g.length;i++){const n=g[i];if(n.kind==="Branch"){n.nonzero=l[n.nonzero];n.zero=l[n.zero]}else n.next=n.next===undefined?i+1:l[n.next];if(n.kind==="Const"&&typeof n.value==="string")n.value=l[n.value.slice(1)];}
 return {g,l};
}
function run(s,kind="generated",off=false){
 if(![s.xy,...(s.changes??[]).map(z=>z.slice(1))].every(z=>z.every(x=>Number.isInteger(x)&&x>=1&&x<=16)))throw Error("domain");
 if(kind==="fault"&&s.xy[0]<2)throw Error("fault domain");
 const {g,l}=build(kind),initial=g.length,d=new Map([[50,s.xy[0]],[51,s.xy[1]]]),v=[0,0,0,0];let pc=0,paid=0,peak=g.length+3,prod=0,firstProd=null,cloneTrialSteps=0,cloneProductionSteps=0,firstExposure=null;const clones=[],patches=[],trials=[],output=[],prefix=[];
 while(paid<20000){
 const p=pc,n=g[pc];if(!n)throw Error("pc");if(p===l.activation)firstExposure=paid+1;if(kind==="generated"&&firstExposure===null)prefix.push([p,[...v],Array.from(d.entries()),g.map(x=>({...x}))]);
 paid++;pc=n.next;
 if(p>=initial){if(g[l.out].next===l.prod_out)cloneProductionSteps++;else cloneTrialSteps++;}
 switch(n.kind){
 case "Const":v[n.dst]=n.value;break;
 case "Read":v[n.dst]=d.get(v[n.address])??0;break;
 case "Write":{
 const k=v[n.address],a=v[n.value];d.set(k,a);
 if(k===46){trials.push({output:d.get(44),cost:a,x:d.get(50),y:d.get(51)});}
 if(k===11){
 if(a!==d.get(50)*d.get(51))throw Error("bad production");
 prod++;output.push(a);if(firstProd===null)firstProd=paid;
 peak=Math.max(peak,g.length+1+d.size);
 if(prod===s.n)return {paid,initial,nodes:g.length,peak,firstProd,clones,patches,trials,output,active:g[l.guard]?.value??null,finalSource:g,cloneTrialSteps,cloneProductionSteps,firstExposure,prefix};
 for(const [at,x,y] of s.changes??[])if(at===prod){d.set(50,x);d.set(51,y);}
 }break;}
 case "SenseOwnCredits":v[n.dst]=20000-paid;break;
 case "Add":v[n.dst]=v[n.lhs]+v[n.rhs];break;
 case "Sub":v[n.dst]=v[n.lhs]-v[n.rhs];break;
 case "Branch":pc=v[n.test]!==0?n.nonzero:n.zero;break;
 case "CloneNode":{const src=v[n.source],idx=g.length;g.push({...g[src]});v[n.dst]=idx;clones.push([paid,src,idx]);break;}
 case "PatchNext":{const target=v[n.node],before=g[target].next,after=v[n.target];if(!g[after]||g[target].next===undefined)throw Error("edge");g[target].next=after;patches.push([paid,p,target,before,after,false]);break;}
 case "PatchConst":{const target=v[n.node],before=g[target].value,after=v[n.value],suppress=off&&p===l.activation;if(g[target].kind!=="Const")throw Error("const");if(!suppress)g[target].value=after;patches.push([paid,p,target,before,after,suppress]);break;}
 default:throw Error(n.kind);}
 peak=Math.max(peak,g.length+1+d.size);if(g.length>200||d.size>16||peak>220)throw Error("capacity");
 }throw Error("budget");
}
const cases=[];
for(const x of [1,2,3,5,8,12,16])for(const n of [12,48,96]){
 cases.push({id:"stable_"+x+"_"+n,group:"stable",xy:[x,3],n,changes:[[4,x,7]]});
 cases.push({id:"shift_"+x+"_"+n,group:"shift",xy:[x,3],n,changes:[[4,x===16?2:16,7]]});
 if(n>12)cases.push({id:"return_"+x+"_"+n,group:"return",xy:[x,3],n,changes:[[4,x===16?2:16,7],[24,x,9]]});
}
const rows=cases.map(s=>({id:s.id,group:s.group,on:run(s),off:run(s,"generated",true),plain:run(s,"plain"),guard:run(s,"guard_only")}));
const faults=[2,5,16].map(x=>{const s={xy:[x,3],n:24,changes:[[4,x,8]]};return {x,run:run(s,"fault")};});
for(const r of rows){
 if(r.on.output.join()!==r.off.output.join()||r.on.output.join()!==r.plain.output.join()||r.on.output.join()!==r.guard.output.join())throw Error("output match");
 if(r.on.trials[0].output!==r.on.trials[1].output)throw Error("correct trial");
 if(r.on.nodes!==r.on.initial+r.on.clones.length)throw Error("nodes");
 if(r.on.firstExposure!==r.off.firstExposure||JSON.stringify(r.on.prefix)!==JSON.stringify(r.off.prefix))throw Error("causal prefix");
 if(r.on.cloneProductionSteps===0||r.off.cloneProductionSteps!==0||r.on.cloneTrialSteps!==r.off.cloneTrialSteps)throw Error("actual generated execution");
}
for(const f of faults){if(f.run.trials[0].output===f.run.trials[1].output||f.run.active!==0||f.run.cloneProductionSteps!==0)throw Error("fault acceptance");}
const summary=Object.fromEntries(["all","stable","shift","return"].map(k=>{const a=rows.filter(r=>k==="all"||r.group===k);return [k,{n:a.length,physical:a.reduce((s,r)=>s+r.off.paid-r.on.paid,0),netPlain:a.reduce((s,r)=>s+r.plain.paid-r.on.paid,0),netGuard:a.reduce((s,r)=>s+r.guard.paid-r.on.paid,0),winsPlain:a.filter(r=>r.plain.paid>r.on.paid).length,activated:a.filter(r=>r.on.active>0).length,peak:Math.max(...a.map(r=>r.on.peak)),maxNodes:Math.max(...a.map(r=>r.on.nodes))}]}));
const totals={worlds:rows.length*4+faults.length,paid:rows.reduce((s,r)=>s+r.on.paid+r.off.paid+r.plain.paid+r.guard.paid,0)+faults.reduce((s,f)=>s+f.run.paid,0),services:rows.reduce((s,r)=>s+4*r.on.output.length,0)+faults.reduce((s,f)=>s+f.run.output.length,0),newNodes:rows.reduce((s,r)=>s+r.on.clones.length+r.off.clones.length,0)+faults.reduce((s,f)=>s+f.run.clones.length,0),productionCloneSteps:rows.reduce((s,r)=>s+r.on.cloneProductionSteps,0),matchedPrefixes:rows.length};
for(const r of rows)for(const k of ["on","off","plain","guard"])delete r[k].prefix;
for(const f of faults)delete f.run.prefix;

assert.deepEqual(summary,{"all":{"n":56,"physical":25568,"netPlain":3320,"netGuard":12896,"winsPlain":17,"activated":56,"peak":116,"maxNodes":105},"stable":{"n":21,"physical":14664,"netPlain":6636,"netGuard":9912,"winsPlain":9,"activated":21,"peak":116,"maxNodes":105},"shift":{"n":21,"physical":1128,"netPlain":-6900,"netGuard":-3624,"winsPlain":0,"activated":21,"peak":116,"maxNodes":105},"return":{"n":14,"physical":9776,"netPlain":3584,"netGuard":6608,"winsPlain":8,"activated":14,"peak":116,"maxNodes":105}});
assert.deepEqual(totals,{"worlds":227,"paid":516456,"services":12840,"newNodes":772,"productionCloneSteps":12784,"matchedPrefixes":56});
console.log(JSON.stringify({tier:"NON_NATIVE_EXECUTABLE_CONSTRUCTION_MODEL",summary,totals,cases,rows,faults,sources:Object.fromEntries(["generated","plain","guard_only","fault"].map(k=>[k,build(k)]))},null,2));
