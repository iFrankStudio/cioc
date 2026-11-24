#!/usr/bin/env node
// C30-R2 non-native cost-model calibration and selective dispatch.
import fs from "node:fs";
import assert from "node:assert/strict";
function build(kind="generated"){
 const high=kind==="high"||kind==="selective",selective=kind==="selective",generic=kind==="generic"||high;const g=[],l={};const label=k=>{if(k in l)throw Error(k);l[k]=g.length};
 const op=(kind,args)=>g.push({kind,...args});
 const C=(dst,value,next)=>op("Const",{dst,value,next}),R=(dst,address,next)=>op("Read",{dst,address,next}),W=(address,value,next)=>op("Write",{address,value,next}),S=(dst,lhs,rhs,next)=>op("Sub",{dst,lhs,rhs,next}),A=(dst,lhs,rhs,next)=>op("Add",{dst,lhs,rhs,next}),B=(test,nonzero,zero)=>op("Branch",{test,nonzero,zero}),Q=(dst,next)=>op("SenseOwnCredits",{dst,next}),P=(node,target,next)=>op("PatchNext",{node,target,next}),K=(node,value,next)=>op("PatchConst",{node,value,next});
 const set=(key,value,next)=>{C(3,key);C(0,value);W(3,0,next)};
 label("init");
 if(kind==="plain"||kind==="guard_only"){C(3,0,"entry");}
 else{
  if(high){set(40,16);}else{C(3,50);R(0,3);C(3,40);W(3,0);}
  if(generic)C(0,16);if(kind==="fault"){C(2,1);S(0,0,2);}
  C(1,"@out");label("generate");C(2,"@parent_add");op("CloneNode",{dst:3,source:2});P(3,1);
  C(2,0);A(1,3,2);C(2,1);S(0,0,2);B(0,"generate","link");
  label("link");if(generic){C(2,16);S(0,1,2);C(1,"@base");K(1,0);}else{C(3,"@special_entry");P(3,1);}set(47,0,"measure");
  label("measure");Q(0);C(3,42);W(3,0,"entry");
 }
 label("entry");label("input_x");C(3,high?40:50);R(1,3);C(3,51);R(2,3);C(0,0);
 if(kind!=="plain"){
  label("guard");C(3,0);if(generic){B(3,selective?"mode":"base","parent");if(selective){label("mode");C(0,1);S(3,3,0);C(0,0);B(3,"threshold","base");label("threshold");C(0,0);C(3,1);label("gate_loop");S(1,1,3);B(1,"gate_count","restore_parent");label("gate_count");S(0,0,3);B(0,"gate_loop","restore_base");label("restore_parent");C(3,50);R(1,3);C(0,0,"parent");label("restore_base");C(3,50);R(1,3);C(0,0,"base");}label("base");C(3,0);A(1,1,3);C(3,"@special_entry");P(3,1);}else{S(3,1,3);B(3,"parent","special_entry");}
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
  label("install");if(selective){C(3,45);R(0,3);C(3,46);R(1,3);S(0,0,1);C(1,16);C(3,1);label("learn_cutoff");S(0,0,3);S(1,1,3);B(0,"learn_second","store_cutoff");label("learn_second");S(0,0,3);B(0,"learn_cutoff","store_cutoff");label("store_cutoff");C(3,"@threshold");K(3,1);C(0,2);}else if(generic)C(0,1);else{C(3,40);R(0,3);}C(1,"@guard");label("activation");K(1,0,"finish");
  label("finish");if(high){C(1,"@input_x");C(0,50);K(1,0);}C(1,"@out");C(0,"@prod_out");P(1,0,"entry");
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
 const p=pc,n=g[pc];if(!n)throw Error("pc");if(p===l.activation)firstExposure=paid+1;if(kind==="selective"&&firstExposure===null&&prod===0)prefix.push([p,[...v],Array.from(d.entries()),g.map(x=>({...x}))]);
 paid++;pc=n.next;
 if(p>=initial){if(g[l.out].next===l.prod_out){cloneProductionSteps++;serviceClones++;}else cloneTrialSteps++;}
 switch(n.kind){
 case "Const":v[n.dst]=n.value;break;
 case "Read":v[n.dst]=d.get(v[n.address])??0;break;
 case "Write":{
 const k=v[n.address],a=v[n.value];d.set(k,a);
 if(k===46){trials.push({output:d.get(44),cost:a,x:kind==="high"||kind==="selective"?16:d.get(50),y:d.get(51)});}
 if(k===11){
 if(a!==d.get(50)*d.get(51))throw Error("bad production");
 if(["generic","high","selective"].includes(kind)){const mode=g[l.guard].value;const expected=mode===1||mode===2&&d.get(50)>g[l.threshold].value?d.get(50):0;if(serviceClones!==expected)throw Error("wrong generated path length");}serviceClones=0;prod++;output.push(a);if(firstProd===null)firstProd=paid;
 peak=Math.max(peak,g.length+1+d.size);
 if(prod===s.n)return {paid,initial,nodes:g.length,peak,firstProd,clones,patches,trials,output,active:g[l.guard]?.value??null,cutoff:g[l.threshold]?.value??null,finalSource:g,cloneTrialSteps,cloneProductionSteps,firstExposure,prefix};
 for(const [at,x,y] of s.changes??[])if(at===prod){d.set(50,x);d.set(51,y);}
 }break;}
 case "SenseOwnCredits":v[n.dst]=20000-paid;break;
 case "Add":v[n.dst]=v[n.lhs]+v[n.rhs];break;
 case "Sub":v[n.dst]=v[n.lhs]-v[n.rhs];break;
 case "Branch":pc=v[n.test]!==0?n.nonzero:n.zero;break;
 case "CloneNode":{const src=v[n.source],idx=g.length;g.push({...g[src]});v[n.dst]=idx;clones.push([paid,src,idx]);break;}
 case "PatchNext":{const target=v[n.node],before=g[target].next,after=v[n.target];if(["generic","high","selective"].includes(kind)&&target===l.special_entry&&after!==initial+d.get(g[l.input_x].value)-1)throw Error("wrong generated address");if(!g[after]||g[target].next===undefined)throw Error("edge");g[target].next=after;patches.push([paid,p,target,before,after,false]);break;}
 case "PatchConst":{const target=v[n.node],before=g[target].value,after=v[n.value],suppress=off&&p===l.activation;if(g[target].kind!=="Const")throw Error("const");if(!suppress)g[target].value=after;patches.push([paid,p,target,before,after,suppress]);break;}
 default:throw Error(n.kind);}
 peak=Math.max(peak,g.length+1+d.size);if(g.length>200||d.size>16||peak>220)throw Error("capacity");
 }throw Error("budget");
}
const cases=JSON.parse(fs.readFileSync(new URL("../studies/CL1B-C30-EXECUTABLE-SPECIALIZATION-frozen-67-worlds-v1.json",import.meta.url),"utf8")).cases;

function test(s){const r={id:s.id,group:s.group??s.cohort,plain:run(s,"plain"),generic:run(s,"generic"),high:run(s,"high"),selective:run(s,"selective"),off:run(s,"selective",true)};for(const k of ["generic","high","selective","off"]){if(r[k].output.join()!==r.plain.output.join())throw Error("prod");}
if(JSON.stringify(r.selective.prefix)!==JSON.stringify(r.off.prefix)||r.selective.firstExposure!==r.off.firstExposure)throw Error("prefix");
if(r.selective.cutoff!==4||r.selective.active!==2||r.off.active!==0)throw Error("cutoff "+r.selective.cutoff);
for(const k of ["plain","generic","high","selective","off"]){delete r[k].prefix;delete r[k].finalSource;delete r[k].output;delete r[k].patches;}return r;}
const rows=cases.map(test);
const family=[];
for(let x=1;x<=16;x++)for(const n of [32,96,144]){
 family.push({id:"low_"+x+"_"+n,group:"low",xy:[x,5],n,changes:[[1,1,7],[Math.floor(n/2),2,9]]});
 family.push({id:"high_"+x+"_"+n,group:"high",xy:[x,5],n,changes:[[1,16,7],[Math.floor(n/2),12,9]]});
 family.push({id:"mixed_"+x+"_"+n,group:"mixed",xy:[x,5],n,changes:Array.from({length:n-1},(_,i)=>[i+1,1+((x+5*i)%16),1+(7*i%16)])});
}
const grid=family.map(test);
function agg(a){return Object.fromEntries(["generic","high","selective","off"].map(k=>[k,{n:a.length,net:a.reduce((s,r)=>s+r.plain.paid-r[k].paid,0),wins:a.filter(r=>r[k].paid<r.plain.paid).length,paid:a.reduce((s,r)=>s+r[k].paid,0),nodes:Math.max(...a.map(r=>r[k].nodes)),peak:Math.max(...a.map(r=>r[k].peak)),clones:a.reduce((s,r)=>s+r[k].cloneProductionSteps,0)}]))}
const summary={known:agg(rows.filter(r=>r.group==="prospective")),grid:agg(grid),groups:Object.fromEntries(["low","high","mixed"].map(k=>[k,agg(grid.filter(r=>r.group===k))]))};

assert.deepEqual(summary,{"known":{"generic":{"n":12,"net":4440,"wins":8,"paid":23682,"nodes":109,"peak":120,"clones":6571},"high":{"n":12,"net":3224,"wins":7,"paid":24898,"nodes":111,"peak":122,"clones":6694},"selective":{"n":12,"net":-16842,"wins":0,"paid":44964,"nodes":141,"peak":152,"clones":6007},"off":{"n":12,"net":-7280,"wins":0,"paid":35402,"nodes":141,"peak":152,"clones":0}},"grid":{"generic":{"n":144,"net":66666,"wins":80,"paid":364326,"nodes":109,"peak":120,"clones":91548},"high":{"n":144,"net":72816,"wins":80,"paid":358176,"nodes":111,"peak":122,"clones":104448},"selective":{"n":144,"net":-198200,"wins":0,"paid":629192,"nodes":141,"peak":152,"clones":95188},"off":{"n":144,"net":-92064,"wins":0,"paid":523056,"nodes":141,"peak":152,"clones":0}},"groups":{"low":{"generic":{"n":48,"net":-26672,"wins":0,"paid":86552,"nodes":109,"peak":120,"clones":6069},"high":{"n":48,"net":-31584,"wins":0,"paid":91464,"nodes":111,"peak":122,"clones":6888},"selective":{"n":48,"net":-87228,"wins":0,"paid":147108,"nodes":141,"peak":152,"clones":378},"off":{"n":48,"net":-30688,"wins":0,"paid":90568,"nodes":141,"peak":152,"clones":0}},"high":{"generic":{"n":48,"net":67268,"wins":42,"paid":153652,"nodes":109,"peak":120,"clones":53039},"high":{"n":48,"net":75776,"wins":48,"paid":145144,"nodes":111,"peak":122,"clones":60568},"selective":{"n":48,"net":-35676,"wins":0,"paid":256596,"nodes":141,"peak":152,"clones":60538},"off":{"n":48,"net":-30688,"wins":0,"paid":251608,"nodes":141,"peak":152,"clones":0}},"mixed":{"generic":{"n":48,"net":26070,"wins":38,"paid":124122,"nodes":109,"peak":120,"clones":32440},"high":{"n":48,"net":28624,"wins":32,"paid":121568,"nodes":111,"peak":122,"clones":36992},"selective":{"n":48,"net":-75296,"wins":0,"paid":225488,"nodes":141,"peak":152,"clones":34272},"off":{"n":48,"net":-30688,"wins":0,"paid":180880,"nodes":141,"peak":152,"clones":0}}}});
console.log(JSON.stringify({tier:"NON_NATIVE_SELECTIVE_DISPATCH_MODEL",summary,rows,grid},null,2));
