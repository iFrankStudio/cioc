#!/usr/bin/env node
// C30-R9 fused single-pass descriptor checks; no omitted fields.
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
function legacyRun(s,kind="generated",off=false){
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

function buildCompiled(){
 const g=[],l={};const label=k=>{if(k in l)throw Error(k);l[k]=g.length};
 const op=(kind,args)=>g.push({kind,...args});
 const C=(dst,value,next)=>op("Const",{dst,value,next}),R=(dst,address,next)=>op("Read",{dst,address,next}),W=(address,value,next)=>op("Write",{address,value,next}),S=(dst,lhs,rhs,next)=>op("Sub",{dst,lhs,rhs,next}),A=(dst,lhs,rhs,next)=>op("Add",{dst,lhs,rhs,next}),B=(test,nonzero,zero)=>op("Branch",{test,nonzero,zero}),Q=(dst,next)=>op("SenseOwnCredits",{dst,next}),P=(node,target,next)=>op("PatchNext",{node,target,next}),K=(node,value,next)=>op("PatchConst",{node,value,next});
 const set=(key,value,next)=>{C(3,key);C(0,value);W(3,0,next)};
 label("init");set(40,16);C(0,16);C(1,"@out");
 label("generate");C(2,"@parent_add");op("CloneNode",{dst:3,source:2});P(3,1);C(2,0);A(1,3,2);C(2,1);S(0,0,2);B(0,"generate","link");
 label("link");C(2,16);S(0,1,2);C(1,"@base");K(1,0);set(47,0,"measure");
 label("measure");Q(0);C(3,42);W(3,0,"entry");
 label("entry");label("input_x");C(3,40);
 label("dispatch");R(1,3,"parent");
 label("parent");C(3,51);R(2,3);C(0,0);C(3,1);
 label("parent_add");A(0,0,2);S(1,1,3);B(1,"parent_add","out");
 label("base");C(3,0);A(1,1,3);C(3,"@special_entry");P(3,1);C(3,51);R(2,3);
 label("special_entry");C(0,0,"parent_add");
 label("out");C(3,0,"trial_out");
 label("trial_out");C(3,44);W(3,0);Q(0);C(3,42);R(1,3);S(0,1,0);C(3,46);W(3,0);
 C(3,47);R(1,3);B(1,"trial_compare","save_parent");
 label("save_parent");C(3,45);W(3,0);C(3,44);R(0,3);C(3,43);W(3,0);set(47,1);
 C(1,"@dispatch");C(0,"@base");P(1,0,"measure");
 label("trial_compare");C(1,"@dispatch");C(0,"@parent");P(1,0);
 C(3,43);R(0,3);C(3,44);R(1,3);S(0,0,1);B(0,"finish","compare_cost");
 label("compare_cost");C(3,45);R(0,3);C(3,46);R(1,3);S(2,0,1);S(1,1,0);C(3,1);B(2,"cmp","finish");
 label("cmp");S(2,2,3);B(2,"cmp_neg","install");label("cmp_neg");S(1,1,3);B(1,"cmp","finish");
 label("install");C(1,"@flag");C(0,1);label("activation");K(1,0,"finish");
 label("finish");C(1,"@input_x");C(0,50);K(1,0);C(1,"@out");C(0,"@prod_out");P(1,0);
 label("flag");C(0,0);B(0,"compile","entry");
 label("compile");C(1,"@dispatch");C(0,"@base");P(1,0,"entry");
 label("prod_out");C(3,11);W(3,0,"entry");
 for(let i=0;i<g.length;i++){const n=g[i];if(n.kind==="Branch"){n.nonzero=l[n.nonzero];n.zero=l[n.zero]}else n.next=n.next===undefined?i+1:l[n.next];if(n.kind==="Const"&&typeof n.value==="string")n.value=l[n.value.slice(1)];}
 return {g,l};
}
function runCompiled(s,off=false){
 if(![s.xy,...(s.changes??[]).map(z=>z.slice(1))].every(z=>z.every(x=>Number.isInteger(x)&&x>=1&&x<=16)))throw Error("domain");
 const {g,l}=buildCompiled(),initial=g.length,d=new Map([[50,s.xy[0]],[51,s.xy[1]]]),v=[0,0,0,0];let pc=0,paid=0,peak=g.length+3,prod=0,cloneTrialSteps=0,cloneProductionSteps=0,serviceClones=0,firstExposure=null,firstProd=null;const clones=[],patches=[],trials=[],output=[],prefix=[];
 while(paid<20000){
 const p=pc,n=g[p];if(!n)throw Error("pc");if(p===l.activation)firstExposure=paid+1;
 if(firstExposure===null&&prod===0)prefix.push([p,[...v],Array.from(d.entries()),g.map(x=>({...x}))]);
 paid++;pc=n.next;
 if(p>=initial){if(g[l.out].next===l.prod_out){cloneProductionSteps++;serviceClones++;}else cloneTrialSteps++;}
 switch(n.kind){
 case "Const":v[n.dst]=n.value;break;
 case "Read":v[n.dst]=d.get(v[n.address])??0;break;
 case "Write":{
 const k=v[n.address],a=v[n.value];d.set(k,a);
 if(k===46)trials.push({output:d.get(44),cost:a,x:d.get(40),y:d.get(51)});
 if(k===11){
 if(a!==d.get(50)*d.get(51))throw Error("bad production");
 const active=g[l.dispatch].next===l.base;
 if(serviceClones!==(active?d.get(50):0))throw Error("clone count");
 serviceClones=0;prod++;output.push(a);if(firstProd===null)firstProd=paid;
 peak=Math.max(peak,g.length+1+d.size);
 if(prod===s.n)return {paid,initial,nodes:g.length,peak,firstProd,clones,patches,trials,output,active,flag:g[l.flag].value,cloneTrialSteps,cloneProductionSteps,firstExposure,prefix};
 for(const [at,x,y] of s.changes??[])if(at===prod){d.set(50,x);d.set(51,y);}
 }break;}
 case "SenseOwnCredits":v[n.dst]=20000-paid;break;
 case "Add":v[n.dst]=v[n.lhs]+v[n.rhs];break;
 case "Sub":v[n.dst]=v[n.lhs]-v[n.rhs];break;
 case "Branch":pc=v[n.test]!==0?n.nonzero:n.zero;break;
 case "CloneNode":{const src=v[n.source],idx=g.length;if(src!==l.parent_add)throw Error("clone source");g.push({...g[src]});v[n.dst]=idx;clones.push([paid,src,idx]);break;}
 case "PatchNext":{const target=v[n.node],before=g[target].next,after=v[n.target];if(!g[after]||g[target].next===undefined)throw Error("edge");
 if(target===l.special_entry&&after!==initial+d.get(g[l.input_x].value)-1)throw Error("address");
 if(target===l.dispatch&&![l.base,l.parent].includes(after))throw Error("dispatch target");
 g[target].next=after;patches.push([paid,p,target,before,after,false]);break;}
 case "PatchConst":{const target=v[n.node],before=g[target].value,after=v[n.value],suppress=off&&p===l.activation;if(g[target].kind!=="Const")throw Error("const");if(!suppress)g[target].value=after;patches.push([paid,p,target,before,after,suppress]);break;}
 default:throw Error(n.kind);}
 peak=Math.max(peak,g.length+1+d.size);if(g.length>200||d.size>16||peak>220)throw Error("capacity");
 }throw Error("budget");
}
function binarySteps(x){let n=0;while(x){n+=x%2;if(x>1)n++;x=Math.floor(x/2);}return n;}
function buildBinary(fault=false){
 const g=[],l={};const label=k=>{if(k in l)throw Error(k);l[k]=g.length};
 const op=(kind,args)=>g.push({kind,...args});
 const C=(dst,value,next)=>op("Const",{dst,value,next}),R=(dst,address,next)=>op("Read",{dst,address,next}),W=(address,value,next)=>op("Write",{address,value,next}),S=(dst,lhs,rhs,next)=>op("Sub",{dst,lhs,rhs,next}),A=(dst,lhs,rhs,next)=>op("Add",{dst,lhs,rhs,next}),B=(test,nonzero,zero)=>op("Branch",{test,nonzero,zero}),Q=(dst,next)=>op("SenseOwnCredits",{dst,next}),P=(node,target,next)=>op("PatchNext",{node,target,next}),K=(node,value,next)=>op("PatchConst",{node,value,next});
 const set=(key,value,next)=>{C(3,key);C(0,value);W(3,0,next)};
 label("init");C(2,8);label("double_src");A(2,2,2);C(3,40);W(3,2);
 C(1,"@parent_add");op("CloneNode",{dst:3,source:1});C(2,"@out");P(3,2);
 C(2,1);S(2,3,2);C(1,"@base");K(1,2);
 C(2,0);A(0,3,2);C(2,7);A(1,0,2);
 label("generate_even");C(2,"@double_src");op("CloneNode",{dst:3,source:2});P(3,0);
 S(2,0,1);B(2,"generate_odd","setup_trial");
 label("generate_odd");C(2,"@parent_add");op("CloneNode",{dst:2,source:2});P(2,fault?0:3);C(2,1);A(0,0,2,"generate_even");
 label("setup_trial");set(47,0,"measure");
 label("measure");Q(0);C(3,42);W(3,0,"entry");
 label("entry");label("input_x");C(3,40);
 label("dispatch");R(1,3,"parent");
 label("parent");C(3,51);R(2,3);C(0,0);C(3,1);
 label("parent_add");A(0,0,2);S(1,1,3);B(1,"parent_add","out");
 label("base");C(3,0);A(1,1,3);C(3,"@special_entry");P(3,1);C(3,51);R(2,3);
 label("special_entry");C(0,0,"parent_add");
 label("out");C(3,0,"trial_out");
 label("trial_out");C(3,44);W(3,0);Q(0);C(3,42);R(1,3);S(0,1,0);C(3,46);W(3,0);
 C(3,47);R(1,3);B(1,"trial_compare","save_parent");
 label("save_parent");C(3,45);W(3,0);C(3,44);R(0,3);C(3,43);W(3,0);set(47,1);
 C(1,"@dispatch");C(0,"@base");P(1,0,"measure");
 label("trial_compare");C(1,"@dispatch");C(0,"@parent");P(1,0);
 C(3,43);R(0,3);C(3,44);R(1,3);S(0,0,1);B(0,"finish","compare_cost");
 label("compare_cost");C(3,45);R(0,3);C(3,46);R(1,3);S(2,0,1);S(1,1,0);C(3,1);B(2,"cmp","finish");
 label("cmp");S(2,2,3);B(2,"cmp_neg","install");label("cmp_neg");S(1,1,3);B(1,"cmp","finish");
 label("install");C(1,"@flag");C(0,1);label("activation");K(1,0,"finish");
 label("finish");C(1,"@input_x");C(0,50);K(1,0);C(1,"@out");C(0,"@prod_out");P(1,0);
 label("flag");C(0,0);B(0,"compile","entry");
 label("compile");C(1,"@dispatch");C(0,"@base");P(1,0,"entry");
 label("prod_out");C(3,11);W(3,0,"entry");
 for(let i=0;i<g.length;i++){const n=g[i];if(n.kind==="Branch"){n.nonzero=l[n.nonzero];n.zero=l[n.zero]}else n.next=n.next===undefined?i+1:l[n.next];if(n.kind==="Const"&&typeof n.value==="string")n.value=l[n.value.slice(1)];}
 return {g,l};
}
function runBinary(s,off=false,fault=false){
 if(![s.xy,...(s.changes??[]).map(z=>z.slice(1))].every(z=>z.every(x=>Number.isInteger(x)&&x>=1&&x<=16)))throw Error("domain");
 const {g,l}=buildBinary(fault),initial=g.length,d=new Map([[50,s.xy[0]],[51,s.xy[1]]]),v=[0,0,0,0];let pc=0,paid=0,peak=g.length+3,prod=0,cloneTrialSteps=0,cloneProductionSteps=0,serviceClones=0,firstExposure=null,firstProd=null;const clones=[],patches=[],trials=[],output=[],prefix=[];
 while(paid<20000){
 const p=pc,n=g[p];if(!n)throw Error("pc");if(p===l.activation)firstExposure=paid+1;
 if(firstExposure===null&&prod===0)prefix.push([p,[...v],Array.from(d.entries()),g.map(x=>({...x}))]);
 paid++;pc=n.next;
 if(p>=initial){if(g[l.out].next===l.prod_out){cloneProductionSteps++;serviceClones++;}else cloneTrialSteps++;}
 switch(n.kind){
 case "Const":v[n.dst]=n.value;break;
 case "Read":v[n.dst]=d.get(v[n.address])??0;break;
 case "Write":{
 const k=v[n.address],a=v[n.value];d.set(k,a);
 if(k===46)trials.push({output:d.get(44),cost:a,x:d.get(40),y:d.get(51)});
 if(k===11){
 if(a!==d.get(50)*d.get(51)){if(fault)return {wrong:true,x:d.get(50),y:d.get(51),actual:a,expected:d.get(50)*d.get(51),paid,trials,clones:clones.length,flag:g[l.flag].value};throw Error("bad production");}
 const active=g[l.dispatch].next===l.base;
 if(serviceClones!==(active?binarySteps(d.get(50)):0))throw Error("clone count");
 serviceClones=0;prod++;output.push(a);if(firstProd===null)firstProd=paid;
 peak=Math.max(peak,g.length+1+d.size);
 if(prod===s.n)return {paid,initial,nodes:g.length,peak,firstProd,clones,patches,trials,output,active,flag:g[l.flag].value,cloneTrialSteps,cloneProductionSteps,firstExposure,prefix};
 for(const [at,x,y] of s.changes??[])if(at===prod){d.set(50,x);d.set(51,y);}
 }break;}
 case "SenseOwnCredits":v[n.dst]=20000-paid;break;
 case "Add":v[n.dst]=v[n.lhs]+v[n.rhs];break;
 case "Sub":v[n.dst]=v[n.lhs]-v[n.rhs];break;
 case "Branch":pc=v[n.test]!==0?n.nonzero:n.zero;break;
 case "CloneNode":{const src=v[n.source],idx=g.length;if(src!==l.parent_add&&src!==l.double_src)throw Error("clone source");if(src!==((idx-initial+1)%2?l.parent_add:l.double_src))throw Error("binary layout");g.push({...g[src]});v[n.dst]=idx;clones.push([paid,src,idx]);break;}
 case "PatchNext":{const target=v[n.node],before=g[target].next,after=v[n.target];if(!g[after]||g[target].next===undefined)throw Error("edge");
 if(target===l.special_entry&&after!==initial+d.get(g[l.input_x].value)-1)throw Error("address");
 if(target===l.dispatch&&![l.base,l.parent].includes(after))throw Error("dispatch target");if(target>=initial){const x=target-initial+1,expected=x===1?l.out:x%2?target-1:initial+x/2-1;if(!fault&&after!==expected)throw Error("binary edge");}
 g[target].next=after;patches.push([paid,p,target,before,after,false]);break;}
 case "PatchConst":{const target=v[n.node],before=g[target].value,after=v[n.value],suppress=off&&p===l.activation;if(g[target].kind!=="Const")throw Error("const");if(!suppress)g[target].value=after;patches.push([paid,p,target,before,after,suppress]);break;}
 default:throw Error(n.kind);}
 peak=Math.max(peak,g.length+1+d.size);if(g.length>200||d.size>16||peak>220)throw Error("capacity");
 }throw Error("budget");
}
function buildVerified(fault="none"){
 const g=[],l={};const label=k=>{if(k in l)throw Error(k);l[k]=g.length};
 const op=(kind,args)=>g.push({kind,...args});
 const C=(dst,value,next)=>op("Const",{dst,value,next}),R=(dst,address,next)=>op("Read",{dst,address,next}),W=(address,value,next)=>op("Write",{address,value,next}),S=(dst,lhs,rhs,next)=>op("Sub",{dst,lhs,rhs,next}),A=(dst,lhs,rhs,next)=>op("Add",{dst,lhs,rhs,next}),B=(test,nonzero,zero)=>op("Branch",{test,nonzero,zero}),Q=(dst,next)=>op("SenseOwnCredits",{dst,next}),P=(node,target,next)=>op("PatchNext",{node,target,next}),K=(node,value,next)=>op("PatchConst",{node,value,next});
 const set=(key,value,next)=>{C(3,key);C(0,value);W(3,0,next)};
 label("init");C(2,8);label("double_src");A(2,2,2);C(3,40);W(3,2);
 C(1,"@parent_add");op("CloneNode",{dst:3,source:1});C(2,fault==="first_edge"?"@parent_add":"@out");P(3,2);
 C(2,1);S(2,3,2);C(1,"@base");K(1,2);
 C(2,0);A(0,3,2);C(2,7);A(1,0,2);
 label("generate_even");C(2,"@double_src");op("CloneNode",{dst:3,source:2});P(3,fault==="even_edge"?3:0);
 S(2,0,1);B(2,"generate_odd","verify");
 label("generate_odd");C(2,fault==="odd_opcode"?"@double_src":"@parent_add");op("CloneNode",{dst:2,source:2});P(2,fault==="odd_edge"?0:3);C(2,1);A(0,0,2,"generate_even");
 label("verify");C(1,"first_clone");op("ReadNext",{dst:2,node:1});C(3,"@out");S(2,2,3);B(2,"finish","verify_init");
 label("verify_init");C(0,"first_clone");C(1,"second_clone");
 label("verify_even");op("ReadNext",{dst:2,node:1});S(2,2,0);B(2,"finish","verify_end");
 label("verify_end");C(3,"last_clone");S(2,1,3);B(2,"verify_odd","setup_trial");
 label("verify_odd");C(3,1);A(1,1,3);op("ReadNext",{dst:2,node:1});S(2,2,1);A(2,2,3);B(2,"finish","verify_advance");
 label("verify_advance");A(1,1,3);A(0,0,3,"verify_even");
 label("setup_trial");set(47,0,"measure");
 label("measure");Q(0);C(3,42);W(3,0,"entry");
 label("entry");label("input_x");C(3,40);
 label("dispatch");R(1,3,"parent");
 label("parent");C(3,51);R(2,3);C(0,0);C(3,1);
 label("parent_add");A(0,0,2);S(1,1,3);B(1,"parent_add","out");
 label("base");C(3,0);A(1,1,3);C(3,"@special_entry");P(3,1);C(3,51);R(2,3);
 label("special_entry");C(0,0,"parent_add");
 label("out");C(3,0,"trial_out");
 label("trial_out");C(3,44);W(3,0);Q(0);C(3,42);R(1,3);S(0,1,0);C(3,46);W(3,0);
 C(3,47);R(1,3);B(1,"trial_compare","save_parent");
 label("save_parent");C(3,45);W(3,0);C(3,44);R(0,3);C(3,43);W(3,0);set(47,1);
 C(1,"@dispatch");C(0,"@base");P(1,0,"measure");
 label("trial_compare");C(1,"@dispatch");C(0,"@parent");P(1,0);
 C(3,43);R(0,3);C(3,44);R(1,3);S(0,0,1);B(0,"finish","compare_cost");
 label("compare_cost");C(3,45);R(0,3);C(3,46);R(1,3);S(2,0,1);S(1,1,0);C(3,1);B(2,"cmp","finish");
 label("cmp");S(2,2,3);B(2,"cmp_neg","install");label("cmp_neg");S(1,1,3);B(1,"cmp","finish");
 label("install");C(1,"@flag");C(0,1);label("activation");K(1,0,"finish");
 label("finish");C(1,"@input_x");C(0,50);K(1,0);C(1,"@out");C(0,"@prod_out");P(1,0);
 label("flag");C(0,0);B(0,"compile","entry");
 label("compile");C(1,"@dispatch");C(0,"@base");P(1,0,"entry");
 label("prod_out");C(3,11);W(3,0,"entry");
 for(let i=0;i<g.length;i++){const n=g[i];if(n.kind==="Branch"){n.nonzero=l[n.nonzero];n.zero=l[n.zero]}else n.next=n.next===undefined?i+1:l[n.next];if(n.kind==="Const"&&typeof n.value==="string")n.value=n.value==="first_clone"?g.length:n.value==="second_clone"?g.length+1:n.value==="last_clone"?g.length+15:l[n.value.slice(1)];}
 return {g,l};
}
function runVerified(s,off=false,fault="none"){
 if(![s.xy,...(s.changes??[]).map(z=>z.slice(1))].every(z=>z.every(x=>Number.isInteger(x)&&x>=1&&x<=16)))throw Error("domain");
 const {g,l}=buildVerified(fault),initial=g.length,d=new Map([[50,s.xy[0]],[51,s.xy[1]]]),v=[0,0,0,0];let pc=0,paid=0,peak=g.length+3,prod=0,cloneTrialSteps=0,cloneProductionSteps=0,serviceClones=0,firstExposure=null,firstProd=null,verifiedReads=0;const clones=[],patches=[],trials=[],output=[],prefix=[];
 while(paid<20000){
 const p=pc,n=g[p];if(!n)throw Error("pc");if(p===l.activation)firstExposure=paid+1;
 if(firstExposure===null&&prod===0)prefix.push([p,[...v],Array.from(d.entries()),g.map(x=>({...x}))]);
 paid++;pc=n.next;
 if(p>=initial){if(g[l.out].next===l.prod_out){cloneProductionSteps++;serviceClones++;}else cloneTrialSteps++;}
 switch(n.kind){
 case "Const":v[n.dst]=n.value;break;
 case "Read":v[n.dst]=d.get(v[n.address])??0;break;
 case "ReadNext":v[n.dst]=g[v[n.node]].next;verifiedReads++;break;
 case "Write":{
 const k=v[n.address],a=v[n.value];d.set(k,a);
 if(k===46)trials.push({output:d.get(44),cost:a,x:d.get(40),y:d.get(51)});
 if(k===11){
 if(a!==d.get(50)*d.get(51)){if(fault!=="none")return {wrong:true,verifiedReads,x:d.get(50),y:d.get(51),actual:a,expected:d.get(50)*d.get(51),paid,trials,clones:clones.length,flag:g[l.flag].value};throw Error("bad production");}
 const active=g[l.dispatch].next===l.base;
 if(serviceClones!==(active?binarySteps(d.get(50)):0))throw Error("clone count");
 serviceClones=0;prod++;output.push(a);if(firstProd===null)firstProd=paid;
 peak=Math.max(peak,g.length+1+d.size);
 if(prod===s.n)return {paid,verifiedReads,initial,nodes:g.length,peak,firstProd,clones,patches,trials,output,active,flag:g[l.flag].value,cloneTrialSteps,cloneProductionSteps,firstExposure,prefix};
 for(const [at,x,y] of s.changes??[])if(at===prod){d.set(50,x);d.set(51,y);}
 }break;}
 case "SenseOwnCredits":v[n.dst]=20000-paid;break;
 case "Add":v[n.dst]=v[n.lhs]+v[n.rhs];break;
 case "Sub":v[n.dst]=v[n.lhs]-v[n.rhs];break;
 case "Branch":pc=v[n.test]!==0?n.nonzero:n.zero;break;
 case "CloneNode":{const src=v[n.source],idx=g.length;if(src!==l.parent_add&&src!==l.double_src)throw Error("clone source");if(fault==="none"&&src!==((idx-initial+1)%2?l.parent_add:l.double_src))throw Error("binary layout");g.push({...g[src]});v[n.dst]=idx;clones.push([paid,src,idx]);break;}
 case "PatchNext":{const target=v[n.node],before=g[target].next,after=v[n.target];if(!g[after]||g[target].next===undefined)throw Error("edge");
 if(target===l.special_entry&&after!==initial+d.get(g[l.input_x].value)-1)throw Error("address");
 if(target===l.dispatch&&![l.base,l.parent].includes(after))throw Error("dispatch target");if(target>=initial){const x=target-initial+1,expected=x===1?l.out:x%2?target-1:initial+x/2-1;if(fault==="none"&&after!==expected)throw Error("binary edge");}
 g[target].next=after;patches.push([paid,p,target,before,after,false]);break;}
 case "PatchConst":{const target=v[n.node],before=g[target].value,after=v[n.value],suppress=off&&p===l.activation;if(g[target].kind!=="Const")throw Error("const");if(!suppress)g[target].value=after;patches.push([paid,p,target,before,after,suppress]);break;}
 default:throw Error(n.kind);}
 peak=Math.max(peak,g.length+1+d.size);if(g.length>200||d.size>16||peak>220)throw Error("capacity");
 }throw Error("budget");
}
function buildCertified(fault="none"){
 const g=[],l={};const label=k=>{if(k in l)throw Error(k);l[k]=g.length};
 const op=(kind,args)=>g.push({kind,...args});
 const C=(dst,value,next)=>op("Const",{dst,value,next}),R=(dst,address,next)=>op("Read",{dst,address,next}),W=(address,value,next)=>op("Write",{address,value,next}),S=(dst,lhs,rhs,next)=>op("Sub",{dst,lhs,rhs,next}),A=(dst,lhs,rhs,next)=>op("Add",{dst,lhs,rhs,next}),B=(test,nonzero,zero)=>op("Branch",{test,nonzero,zero}),Q=(dst,next)=>op("SenseOwnCredits",{dst,next}),P=(node,target,next)=>op("PatchNext",{node,target,next}),K=(node,value,next)=>op("PatchConst",{node,value,next});
 const set=(key,value,next)=>{C(3,key);C(0,value);W(3,0,next)};
 label("init");C(2,8);label("double_src");A(2,2,2);C(3,40);W(3,2);
 C(1,"@parent_add");op("CloneNode",{dst:3,source:1});C(2,fault==="first_edge"?"@parent_add":"@out");P(3,2);
 C(2,1);S(2,3,2);C(1,"@base");K(1,2);
 C(2,0);A(0,3,2);C(2,7);A(1,0,2);
 label("generate_even");C(2,fault==="even_operand"?"@parent_add":fault==="even_sub"?"@bad_sub":"@double_src");op("CloneNode",{dst:3,source:2});P(3,fault==="even_edge"?3:0);
 S(2,0,1);B(2,"generate_odd","verify");
 label("generate_odd");C(2,fault==="odd_opcode"?"@double_src":"@parent_add");op("CloneNode",{dst:2,source:2});P(2,fault==="odd_edge"?0:3);C(2,1);A(0,0,2,"generate_even");
 label("verify");C(1,"first_clone");op("ReadNext",{dst:2,node:1});C(3,"@out");S(2,2,3);B(2,"finish","verify_init");
 label("verify_init");C(0,"first_clone");C(1,"second_clone");
 label("verify_even");op("ReadNext",{dst:2,node:1});S(2,2,0);B(2,"finish","verify_end");
 label("verify_end");C(3,"last_clone");S(2,1,3);B(2,"verify_odd","semantic");
 label("verify_odd");C(3,1);A(1,1,3);op("ReadNext",{dst:2,node:1});S(2,2,1);A(2,2,3);B(2,"finish","verify_advance");
 label("verify_advance");A(1,1,3);A(0,0,3,"verify_even");
 label("semantic");op("GraphNew",{dst:0});C(1,"first_clone");
 label("copy");op("GraphCopyNode",{graph:0,source:1,dst:3});C(2,"last_clone");S(2,1,2);B(2,"copy_advance","tag_init");
 label("copy_advance");C(2,1);A(1,1,2,"copy");
 label("tag_init");C(1,0);
 label("tag_loop");C(2,0);op("GraphReadField",{graph:0,node:1,field:2});C(3,1);S(2,2,3);B(2,"semantic_reject","rhs");
 label("rhs");C(2,3);op("GraphReadField",{graph:0,node:1,field:2});C(3,2);S(2,2,3);B(2,"semantic_reject","tag_end");
 label("tag_end");C(3,15);label("bad_sub");S(2,1,3);B(2,"tag_advance","operand_init");
 label("tag_advance");C(3,1);A(1,1,3,"tag_loop");
 label("operand_init");C(1,0);C(3,0);
 label("operand_loop");C(2,1);op("GraphReadField",{graph:0,node:1,field:2});S(2,2,3);B(2,"semantic_reject","lhs");
 label("lhs");C(2,2);op("GraphReadField",{graph:0,node:1,field:2});S(2,2,3);B(2,"semantic_reject","operand_end");
 label("operand_end");C(2,15);S(2,1,2);B(2,"operand_advance","semantic_ok");
 label("operand_advance");C(2,1);A(1,1,2);C(2,2);S(3,2,3,"operand_loop");
 label("semantic_reject");op("GraphDrop",{graph:0,next:"finish"});
 label("semantic_ok");op("GraphDrop",{graph:0,next:"setup_trial"});
 label("setup_trial");set(47,0,"measure");
 label("measure");Q(0);C(3,42);W(3,0,"entry");
 label("entry");label("input_x");C(3,40);
 label("dispatch");R(1,3,"parent");
 label("parent");C(3,51);R(2,3);C(0,0);C(3,1);
 label("parent_add");A(0,0,2);S(1,1,3);B(1,"parent_add","out");
 label("base");C(3,0);A(1,1,3);C(3,"@special_entry");P(3,1);C(3,51);R(2,3);
 label("special_entry");C(0,0,"parent_add");
 label("out");C(3,0,"trial_out");
 label("trial_out");C(3,44);W(3,0);Q(0);C(3,42);R(1,3);S(0,1,0);C(3,46);W(3,0);
 C(3,47);R(1,3);B(1,"trial_compare","save_parent");
 label("save_parent");C(3,45);W(3,0);C(3,44);R(0,3);C(3,43);W(3,0);set(47,1);
 C(1,"@dispatch");C(0,"@base");P(1,0,"measure");
 label("trial_compare");C(1,"@dispatch");C(0,"@parent");P(1,0);
 C(3,43);R(0,3);C(3,44);R(1,3);S(0,0,1);B(0,"finish","compare_cost");
 label("compare_cost");C(3,45);R(0,3);C(3,46);R(1,3);S(2,0,1);S(1,1,0);C(3,1);B(2,"cmp","finish");
 label("cmp");S(2,2,3);B(2,"cmp_neg","install");label("cmp_neg");S(1,1,3);B(1,"cmp","finish");
 label("install");C(1,"@flag");C(0,1);label("activation");K(1,0,"finish");
 label("finish");C(1,"@input_x");C(0,50);K(1,0);C(1,"@out");C(0,"@prod_out");P(1,0);
 label("flag");C(0,0);B(0,"compile","entry");
 label("compile");C(1,"@dispatch");C(0,"@base");P(1,0,"entry");
 label("prod_out");C(3,11);W(3,0,"entry");
 for(let i=0;i<g.length;i++){const n=g[i];if(n.kind==="Branch"){n.nonzero=l[n.nonzero];n.zero=l[n.zero]}else n.next=n.next===undefined?i+1:l[n.next];if(n.kind==="Const"&&typeof n.value==="string")n.value=n.value==="first_clone"?g.length:n.value==="second_clone"?g.length+1:n.value==="last_clone"?g.length+15:l[n.value.slice(1)];}
 return {g,l};
}
function runCertified(s,off=false,fault="none"){
 if(![s.xy,...(s.changes??[]).map(z=>z.slice(1))].every(z=>z.every(x=>Number.isInteger(x)&&x>=1&&x<=16)))throw Error("domain");
 const {g,l}=buildCertified(fault),initial=g.length,d=new Map([[50,s.xy[0]],[51,s.xy[1]]]),v=[0,0,0,0];let pc=0,paid=0,peak=g.length+3,prod=0,cloneTrialSteps=0,cloneProductionSteps=0,serviceClones=0,firstExposure=null,firstProd=null,verifiedReads=0,fieldReads=0,graphCopies=0,graphPeak=0,graph=null;const clones=[],patches=[],trials=[],output=[],prefix=[];
 while(paid<20000){
 const p=pc,n=g[p];if(!n)throw Error("pc");if(p===l.activation)firstExposure=paid+1;
 if(firstExposure===null&&prod===0)prefix.push([p,[...v],Array.from(d.entries()),g.map(x=>({...x})),graph?.map(x=>({...x}))??null]);
 paid++;pc=n.next;
 if(p>=initial){if(g[l.out].next===l.prod_out){cloneProductionSteps++;serviceClones++;}else cloneTrialSteps++;}
 switch(n.kind){
 case "Const":v[n.dst]=n.value;break;
 case "Read":v[n.dst]=d.get(v[n.address])??0;break;
 case "ReadNext":v[n.dst]=g[v[n.node]].next;verifiedReads++;break;
 case "GraphNew":if(graph)throw Error("graph exists");graph=[];v[n.dst]=1;break;
 case "GraphCopyNode":if(v[n.graph]!==1||!graph||!g[v[n.source]])throw Error("graph copy");v[n.dst]=graph.length;graph.push({...g[v[n.source]]});graphCopies++;graphPeak=Math.max(graphPeak,1+graph.length);break;
 case "GraphReadField":{if(v[n.graph]!==1||!graph)throw Error("graph handle");const q=graph[v[n.node]],f=v[n.field];if(!q||f<0||f>=5)throw Error("field");const desc=q.kind==="Add"?[1,q.dst,q.lhs,q.rhs,q.next]:q.kind==="Sub"?[2,q.dst,q.lhs,q.rhs,q.next]:(()=>{throw Error("unsupported descriptor "+q.kind)})();v[n.field]=desc[f];fieldReads++;break;}
 case "GraphDrop":if(v[n.graph]!==1||!graph)throw Error("drop");graph=null;break;
 case "Write":{
 const k=v[n.address],a=v[n.value];d.set(k,a);
 if(k===46)trials.push({output:d.get(44),cost:a,x:d.get(40),y:d.get(51)});
 if(k===11){
 if(a!==d.get(50)*d.get(51)){if(fault!=="none")return {wrong:true,verifiedReads,x:d.get(50),y:d.get(51),actual:a,expected:d.get(50)*d.get(51),paid,trials,clones:clones.length,flag:g[l.flag].value};throw Error("bad production");}
 const active=g[l.dispatch].next===l.base;
 if(serviceClones!==(active?binarySteps(d.get(50)):0))throw Error("clone count");
 serviceClones=0;prod++;output.push(a);if(firstProd===null)firstProd=paid;
 peak=Math.max(peak,g.length+1+d.size+(graph?1+graph.length:0));
 if(prod===s.n)return {paid,verifiedReads,fieldReads,graphCopies,graphPeak,graphAlive:graph!==null,initial,nodes:g.length,peak,firstProd,clones,patches,trials,output,active,flag:g[l.flag].value,cloneTrialSteps,cloneProductionSteps,firstExposure,prefix};
 for(const [at,x,y] of s.changes??[])if(at===prod){d.set(50,x);d.set(51,y);}
 }break;}
 case "SenseOwnCredits":v[n.dst]=20000-paid;break;
 case "Add":v[n.dst]=v[n.lhs]+v[n.rhs];break;
 case "Sub":v[n.dst]=v[n.lhs]-v[n.rhs];break;
 case "Branch":pc=v[n.test]!==0?n.nonzero:n.zero;break;
 case "CloneNode":{const src=v[n.source],idx=g.length;if(src!==l.parent_add&&src!==l.double_src&&src!==l.bad_sub)throw Error("clone source");if(fault==="none"&&src!==((idx-initial+1)%2?l.parent_add:l.double_src))throw Error("binary layout");g.push({...g[src]});v[n.dst]=idx;clones.push([paid,src,idx]);break;}
 case "PatchNext":{const target=v[n.node],before=g[target].next,after=v[n.target];if(!g[after]||g[target].next===undefined)throw Error("edge");
 if(target===l.special_entry&&after!==initial+d.get(g[l.input_x].value)-1)throw Error("address");
 if(target===l.dispatch&&![l.base,l.parent].includes(after))throw Error("dispatch target");if(target>=initial){const x=target-initial+1,expected=x===1?l.out:x%2?target-1:initial+x/2-1;if(fault==="none"&&after!==expected)throw Error("binary edge");}
 g[target].next=after;patches.push([paid,p,target,before,after,false]);break;}
 case "PatchConst":{const target=v[n.node],before=g[target].value,after=v[n.value],suppress=off&&p===l.activation;if(g[target].kind!=="Const")throw Error("const");if(!suppress)g[target].value=after;patches.push([paid,p,target,before,after,suppress]);break;}
 default:throw Error(n.kind);}
 peak=Math.max(peak,g.length+1+d.size+(graph?1+graph.length:0));if(g.length>200||d.size>16||peak>220)throw Error("capacity");
 }throw Error("budget");
}
function buildCompact(fault="none"){
 const g=[],l={};const label=k=>{if(k in l)throw Error(k);l[k]=g.length};
 const op=(kind,args)=>g.push({kind,...args});
 const C=(dst,value,next)=>op("Const",{dst,value,next}),R=(dst,address,next)=>op("Read",{dst,address,next}),W=(address,value,next)=>op("Write",{address,value,next}),S=(dst,lhs,rhs,next)=>op("Sub",{dst,lhs,rhs,next}),A=(dst,lhs,rhs,next)=>op("Add",{dst,lhs,rhs,next}),B=(test,nonzero,zero)=>op("Branch",{test,nonzero,zero}),Q=(dst,next)=>op("SenseOwnCredits",{dst,next}),P=(node,target,next)=>op("PatchNext",{node,target,next}),K=(node,value,next)=>op("PatchConst",{node,value,next});
 const set=(key,value,next)=>{C(3,key);C(0,value);W(3,0,next)};
 label("init");C(2,8);label("double_src");A(2,2,2);C(3,40);W(3,2);
 C(1,"@parent_add");op("CloneNode",{dst:3,source:1});C(2,fault==="first_edge"?"@parent_add":"@out");P(3,2);
 C(2,1);S(2,3,2);C(1,"@base");K(1,2);
 C(2,0);A(0,3,2);C(2,7);A(1,0,2);
 label("generate_even");C(2,fault==="even_operand"?"@parent_add":fault==="even_sub"?"@bad_sub":"@double_src");op("CloneNode",{dst:3,source:2});P(3,fault==="even_edge"?3:0);
 S(2,0,1);B(2,"generate_odd","verify");
 label("generate_odd");C(2,fault==="odd_opcode"?"@double_src":"@parent_add");op("CloneNode",{dst:2,source:2});P(2,fault==="odd_edge"?0:3);C(2,1);A(0,0,2,"generate_even");
 label("verify");C(1,"first_clone");op("ReadNext",{dst:2,node:1});C(3,"@out");S(2,2,3);B(2,"finish","verify_init");
 label("verify_init");C(0,"first_clone");C(1,"second_clone");
 label("verify_even");op("ReadNext",{dst:2,node:1});S(2,2,0);B(2,"finish","verify_end");
 label("verify_end");C(3,"last_clone");S(2,1,3);B(2,"verify_odd","semantic");
 label("verify_odd");C(3,1);A(1,1,3);op("ReadNext",{dst:2,node:1});S(2,2,1);A(2,2,3);B(2,"finish","verify_advance");
 label("verify_advance");A(1,1,3);A(0,0,3,"verify_even");
 label("semantic");op("GraphNew",{dst:0});C(1,1);S(2,0,1);B(2,"semantic_reject","copy_init");
 label("copy_init");C(1,"first_clone");
 label("copy");op("GraphCopyNode",{graph:0,source:1,dst:3});C(2,"last_clone");S(2,1,2);B(2,"copy_advance","field_init");
 label("copy_advance");A(1,1,0,"copy");
 label("field_init");C(1,0);C(3,0);
 label("field_loop");C(2,0);op("GraphReadField",{graph:0,node:1,field:2});S(2,2,0);B(2,"semantic_reject","dst");
 label("dst");C(2,1);op("GraphReadField",{graph:0,node:1,field:2});S(2,2,3);B(2,"semantic_reject","lhs");
 label("lhs");C(2,2);op("GraphReadField",{graph:0,node:1,field:2});S(2,2,3);B(2,"semantic_reject","rhs");
 label("rhs");C(2,3);op("GraphReadField",{graph:0,node:1,field:2});S(2,2,0);S(2,2,0);B(2,"semantic_reject","field_end");
 label("field_end");C(2,15);label("bad_sub");S(2,1,2);B(2,"field_advance","semantic_ok");
 label("field_advance");A(1,1,0);C(2,2);S(3,2,3,"field_loop");
 label("semantic_reject");op("GraphDrop",{graph:0,next:"finish"});
 label("semantic_ok");op("GraphDrop",{graph:0,next:"setup_trial"});
 label("setup_trial");set(47,0,"measure");
 label("measure");Q(0);C(3,42);W(3,0,"entry");
 label("entry");label("input_x");C(3,40);
 label("dispatch");R(1,3,"parent");
 label("parent");C(3,51);R(2,3);C(0,0);C(3,1);
 label("parent_add");A(0,0,2);S(1,1,3);B(1,"parent_add","out");
 label("base");C(3,0);A(1,1,3);C(3,"@special_entry");P(3,1);C(3,51);R(2,3);
 label("special_entry");C(0,0,"parent_add");
 label("out");C(3,0,"trial_out");
 label("trial_out");C(3,44);W(3,0);Q(0);C(3,42);R(1,3);S(0,1,0);C(3,46);W(3,0);
 C(3,47);R(1,3);B(1,"trial_compare","save_parent");
 label("save_parent");C(3,45);W(3,0);C(3,44);R(0,3);C(3,43);W(3,0);set(47,1);
 C(1,"@dispatch");C(0,"@base");P(1,0,"measure");
 label("trial_compare");C(1,"@dispatch");C(0,"@parent");P(1,0);
 C(3,43);R(0,3);C(3,44);R(1,3);S(0,0,1);B(0,"finish","compare_cost");
 label("compare_cost");C(3,45);R(0,3);C(3,46);R(1,3);S(2,0,1);S(1,1,0);C(3,1);B(2,"cmp","finish");
 label("cmp");S(2,2,3);B(2,"cmp_neg","install");label("cmp_neg");S(1,1,3);B(1,"cmp","finish");
 label("install");C(1,"@flag");C(0,1);label("activation");K(1,0,"finish");
 label("finish");C(1,"@input_x");C(0,50);K(1,0);C(1,"@out");C(0,"@prod_out");P(1,0);
 label("flag");C(0,0);B(0,"compile","entry");
 label("compile");C(1,"@dispatch");C(0,"@base");P(1,0,"entry");
 label("prod_out");C(3,11);W(3,0,"entry");
 for(let i=0;i<g.length;i++){const n=g[i];if(n.kind==="Branch"){n.nonzero=l[n.nonzero];n.zero=l[n.zero]}else n.next=n.next===undefined?i+1:l[n.next];if(n.kind==="Const"&&typeof n.value==="string")n.value=n.value==="first_clone"?g.length:n.value==="second_clone"?g.length+1:n.value==="last_clone"?g.length+15:l[n.value.slice(1)];}
 return {g,l};
}
function runCompact(s,off=false,fault="none"){
 if(![s.xy,...(s.changes??[]).map(z=>z.slice(1))].every(z=>z.every(x=>Number.isInteger(x)&&x>=1&&x<=16)))throw Error("domain");
 const {g,l}=buildCompact(fault),initial=g.length,d=new Map([[50,s.xy[0]],[51,s.xy[1]]]),v=[0,0,0,0];let pc=0,paid=0,peak=g.length+3,prod=0,cloneTrialSteps=0,cloneProductionSteps=0,serviceClones=0,firstExposure=null,firstProd=null,verifiedReads=0,fieldReads=0,graphCopies=0,graphPeak=0,graph=null;const clones=[],patches=[],trials=[],output=[],prefix=[];
 while(paid<20000){
 const p=pc,n=g[p];if(!n)throw Error("pc");if(p===l.activation)firstExposure=paid+1;
 if(firstExposure===null&&prod===0)prefix.push([p,[...v],Array.from(d.entries()),g.map(x=>({...x})),graph?.map(x=>({...x}))??null]);
 paid++;pc=n.next;
 if(p>=initial){if(g[l.out].next===l.prod_out){cloneProductionSteps++;serviceClones++;}else cloneTrialSteps++;}
 switch(n.kind){
 case "Const":v[n.dst]=n.value;break;
 case "Read":v[n.dst]=d.get(v[n.address])??0;break;
 case "ReadNext":v[n.dst]=g[v[n.node]].next;verifiedReads++;break;
 case "GraphNew":if(graph)throw Error("graph exists");graph=[];v[n.dst]=1;break;
 case "GraphCopyNode":if(v[n.graph]!==1||!graph||!g[v[n.source]])throw Error("graph copy");v[n.dst]=graph.length;graph.push({...g[v[n.source]]});graphCopies++;graphPeak=Math.max(graphPeak,1+graph.length);break;
 case "GraphReadField":{if(v[n.graph]!==1||!graph)throw Error("graph handle");const q=graph[v[n.node]],f=v[n.field];if(!q||f<0||f>=5)throw Error("field");const desc=q.kind==="Add"?[1,q.dst,q.lhs,q.rhs,q.next]:q.kind==="Sub"?[2,q.dst,q.lhs,q.rhs,q.next]:(()=>{throw Error("unsupported descriptor "+q.kind)})();v[n.field]=desc[f];fieldReads++;break;}
 case "GraphDrop":if(v[n.graph]!==1||!graph)throw Error("drop");graph=null;break;
 case "Write":{
 const k=v[n.address],a=v[n.value];d.set(k,a);
 if(k===46)trials.push({output:d.get(44),cost:a,x:d.get(40),y:d.get(51)});
 if(k===11){
 if(a!==d.get(50)*d.get(51)){if(fault!=="none")return {wrong:true,verifiedReads,x:d.get(50),y:d.get(51),actual:a,expected:d.get(50)*d.get(51),paid,trials,clones:clones.length,flag:g[l.flag].value};throw Error("bad production");}
 const active=g[l.dispatch].next===l.base;
 if(serviceClones!==(active?binarySteps(d.get(50)):0))throw Error("clone count");
 serviceClones=0;prod++;output.push(a);if(firstProd===null)firstProd=paid;
 peak=Math.max(peak,g.length+1+d.size+(graph?1+graph.length:0));
 if(prod===s.n)return {paid,verifiedReads,fieldReads,graphCopies,graphPeak,graphAlive:graph!==null,initial,nodes:g.length,peak,firstProd,clones,patches,trials,output,active,flag:g[l.flag].value,cloneTrialSteps,cloneProductionSteps,firstExposure,prefix};
 for(const [at,x,y] of s.changes??[])if(at===prod){d.set(50,x);d.set(51,y);}
 }break;}
 case "SenseOwnCredits":v[n.dst]=20000-paid;break;
 case "Add":v[n.dst]=v[n.lhs]+v[n.rhs];break;
 case "Sub":v[n.dst]=v[n.lhs]-v[n.rhs];break;
 case "Branch":pc=v[n.test]!==0?n.nonzero:n.zero;break;
 case "CloneNode":{const src=v[n.source],idx=g.length;if(src!==l.parent_add&&src!==l.double_src&&src!==l.bad_sub)throw Error("clone source");if(fault==="none"&&src!==((idx-initial+1)%2?l.parent_add:l.double_src))throw Error("binary layout");g.push({...g[src]});v[n.dst]=idx;clones.push([paid,src,idx]);break;}
 case "PatchNext":{const target=v[n.node],before=g[target].next,after=v[n.target];if(!g[after]||g[target].next===undefined)throw Error("edge");
 if(target===l.special_entry&&after!==initial+d.get(g[l.input_x].value)-1)throw Error("address");
 if(target===l.dispatch&&![l.base,l.parent].includes(after))throw Error("dispatch target");if(target>=initial){const x=target-initial+1,expected=x===1?l.out:x%2?target-1:initial+x/2-1;if(fault==="none"&&after!==expected)throw Error("binary edge");}
 g[target].next=after;patches.push([paid,p,target,before,after,false]);break;}
 case "PatchConst":{const target=v[n.node],before=g[target].value,after=v[n.value],suppress=off&&p===l.activation;if(g[target].kind!=="Const")throw Error("const");if(!suppress)g[target].value=after;patches.push([paid,p,target,before,after,suppress]);break;}
 default:throw Error(n.kind);}
 peak=Math.max(peak,g.length+1+d.size+(graph?1+graph.length:0));if(g.length>200||d.size>16||peak>220)throw Error("capacity");
 }throw Error("budget");
}
const cases=JSON.parse(fs.readFileSync(new URL("../studies/CL1B-C30-R3-COMPILED-ENTRY-frozen-51-worlds-v1.json",import.meta.url),"utf8")).cases;

function test(s){
 const plain=legacyRun(s,"plain"),full=runCompiled(s),binary=runBinary(s),old=runCertified(s),on=runCompact(s),off=runCompact(s,true);
 for(const q of [full,binary,old,on,off])if(q.output.join()!==plain.output.join())throw Error("output");
 if(JSON.stringify(on.prefix)!==JSON.stringify(off.prefix)||on.firstExposure!==off.firstExposure)throw Error("prefix");
 if(on.clones.length!==16||on.verifiedReads!==16||on.fieldReads!==64||off.fieldReads!==64||on.graphCopies!==16||on.graphAlive||off.graphAlive||on.cloneTrialSteps!==5||off.cloneTrialSteps!==5||on.trials[0].cost!==60||on.trials[1].cost!==20||on.trials[0].output!==on.trials[1].output||!on.active||off.active||off.cloneProductionSteps!==0)throw Error("normal");
 if(old.cloneProductionSteps!==on.cloneProductionSteps||old.paid-on.paid!==old.firstExposure-on.firstExposure)throw Error("not one-time saving");
 const compact=x=>({paid:x.paid,nodes:x.nodes,peak:x.peak,trial:x.trials,cloneProductionSteps:x.cloneProductionSteps,verifiedReads:x.verifiedReads??0,fieldReads:x.fieldReads??0,graphCopies:x.graphCopies??0,graphPeak:x.graphPeak??0});
 return {id:s.id,group:s.group??s.cohort,plain:compact(plain),full:compact(full),binary:compact(binary),old:compact(old),compact:compact(on),off:compact(off)};
}
const known=cases.map(test);
const family=[];
for(let x of [1,4,8,12,16])for(const n of [6,24,72,144]){
 family.push({id:"stable_"+x+"_"+n,group:"stable",xy:[x,5],n,changes:[[3,x,9]]});
 family.push({id:"rise_"+x+"_"+n,group:"rise",xy:[x,5],n,changes:[[Math.min(n-1,12),16,9]]});
 family.push({id:"fall_"+x+"_"+n,group:"fall",xy:[x,5],n,changes:[[Math.min(n-1,12),1,9]]});
 family.push({id:"mixed_"+x+"_"+n,group:"mixed",xy:[x,5],n,changes:Array.from({length:n-1},(_,i)=>[i+1,1+(x+5*i)%16,1+(i*7)%16])});
}
const grid=family.map(test);
const pairs=[];for(let x=1;x<=16;x++)for(let y=1;y<=16;y++)pairs.push([x,y]);
const domain=runCompact({id:"domain",xy:[16,16],n:257,changes:pairs.map(([x,y],i)=>[i+1,x,y])});
if(domain.output.length!==257||domain.cloneProductionSteps!==1141||domain.fieldReads!==64||domain.graphAlive)throw Error("domain");
const faults=[];
for(const f of ["first_edge","even_edge","odd_edge","odd_opcode","even_operand","even_sub"]){
 const r=runCompact({id:f,xy:[3,7],n:12,changes:[[4,16,2],[8,5,9]]},false,f);
 if(r.wrong||r.active||r.firstExposure!==null||r.trials.length||r.cloneProductionSteps||r.output.length!==12||r.graphAlive)throw Error("rejection "+f);
 faults.push({fault:f,paid:r.paid,verifiedReads:r.verifiedReads,fieldReads:r.fieldReads,graphCopies:r.graphCopies,active:r.flag,output:r.output,cloneProductionSteps:r.cloneProductionSteps});
}
function agg(rows){return Object.fromEntries(["full","binary","old","compact","off"].map(k=>[k,{n:rows.length,net:rows.reduce((s,r)=>s+r.plain.paid-r[k].paid,0),wins:rows.filter(r=>r.plain.paid>r[k].paid).length,paid:rows.reduce((s,r)=>s+r[k].paid,0),nodes:Math.max(...rows.map(r=>r[k].nodes)),peak:Math.max(...rows.map(r=>r[k].peak))}]))}
const summary={known:agg(known.filter(r=>r.group==="prospective")),grid:agg(grid),groups:Object.fromEntries(["stable","rise","fall","mixed"].map(k=>[k,agg(grid.filter(r=>r.group===k))])),domain:{pairs:256,paid:domain.paid,production:domain.output.length,cloneProductionSteps:domain.cloneProductionSteps,nodes:domain.nodes,peak:domain.peak,graphPeak:domain.graphPeak},faults,oneTimeSaving:known[0].old.paid-known[0].compact.paid,verificationCost:known[0].compact.paid-known[0].binary.paid,trial:known[0].compact.trial};

assert.deepEqual(summary,{"known":{"full":{"n":12,"net":7332,"wins":8,"paid":26034,"nodes":116,"peak":127},"binary":{"n":12,"net":11359,"wins":9,"paid":22007,"nodes":125,"peak":136},"old":{"n":12,"net":3139,"wins":7,"paid":30227,"nodes":189,"peak":210},"compact":{"n":12,"net":4603,"wins":7,"paid":28763,"nodes":183,"peak":204},"off":{"n":12,"net":-11424,"wins":0,"paid":44790,"nodes":183,"peak":204}},"grid":{"full":{"n":80,"net":35380,"wins":33,"paid":132790,"nodes":116,"peak":127},"binary":{"n":80,"net":60207,"wins":43,"paid":107963,"nodes":125,"peak":136},"old":{"n":80,"net":5407,"wins":26,"paid":162763,"nodes":189,"peak":210},"compact":{"n":80,"net":15167,"wins":28,"paid":153003,"nodes":183,"peak":204},"off":{"n":80,"net":-76160,"wins":0,"paid":244330,"nodes":183,"peak":204}},"groups":{"stable":{"full":{"n":20,"net":8382,"wins":9,"paid":32966,"nodes":116,"peak":127},"binary":{"n":20,"net":14300,"wins":11,"paid":27048,"nodes":125,"peak":136},"old":{"n":20,"net":600,"wins":6,"paid":40748,"nodes":189,"peak":210},"compact":{"n":20,"net":3040,"wins":7,"paid":38308,"nodes":183,"peak":204},"off":{"n":20,"net":-19040,"wins":0,"paid":60388,"nodes":183,"peak":204}},"rise":{"full":{"n":20,"net":24372,"wins":14,"paid":40961,"nodes":116,"peak":127},"binary":{"n":20,"net":36850,"wins":15,"paid":28483,"nodes":125,"peak":136},"old":{"n":20,"net":23150,"wins":10,"paid":42183,"nodes":189,"peak":210},"compact":{"n":20,"net":25590,"wins":11,"paid":39743,"nodes":183,"peak":204},"off":{"n":20,"net":-19040,"wins":0,"paid":84373,"nodes":183,"peak":204}},"fall":{"full":{"n":20,"net":-6378,"wins":0,"paid":25586,"nodes":116,"peak":127},"binary":{"n":20,"net":-5175,"wins":2,"paid":24383,"nodes":125,"peak":136},"old":{"n":20,"net":-18875,"wins":0,"paid":38083,"nodes":189,"peak":210},"compact":{"n":20,"net":-16435,"wins":0,"paid":35643,"nodes":183,"peak":204},"off":{"n":20,"net":-19040,"wins":0,"paid":38248,"nodes":183,"peak":204}},"mixed":{"full":{"n":20,"net":9004,"wins":10,"paid":33277,"nodes":116,"peak":127},"binary":{"n":20,"net":14232,"wins":15,"paid":28049,"nodes":125,"peak":136},"old":{"n":20,"net":532,"wins":10,"paid":41749,"nodes":189,"peak":210},"compact":{"n":20,"net":2972,"wins":10,"paid":39309,"nodes":183,"peak":204},"off":{"n":20,"net":-19040,"wins":0,"paid":61321,"nodes":183,"peak":204}}},"domain":{"pairs":256,"paid":5181,"production":257,"cloneProductionSteps":1141,"nodes":183,"peak":204,"graphPeak":17},"faults":[{"fault":"first_edge","paid":500,"verifiedReads":1,"fieldReads":0,"graphCopies":0,"active":0,"output":[21,21,21,21,32,32,32,32,45,45,45,45],"cloneProductionSteps":0},{"fault":"even_edge","paid":505,"verifiedReads":2,"fieldReads":0,"graphCopies":0,"active":0,"output":[21,21,21,21,32,32,32,32,45,45,45,45],"cloneProductionSteps":0},{"fault":"odd_edge","paid":514,"verifiedReads":3,"fieldReads":0,"graphCopies":0,"active":0,"output":[21,21,21,21,32,32,32,32,45,45,45,45],"cloneProductionSteps":0},{"fault":"odd_opcode","paid":747,"verifiedReads":16,"fieldReads":10,"graphCopies":16,"active":0,"output":[21,21,21,21,32,32,32,32,45,45,45,45],"cloneProductionSteps":0},{"fault":"even_operand","paid":724,"verifiedReads":16,"fieldReads":6,"graphCopies":16,"active":0,"output":[21,21,21,21,32,32,32,32,45,45,45,45],"cloneProductionSteps":0},{"fault":"even_sub","paid":720,"verifiedReads":16,"fieldReads":5,"graphCopies":16,"active":0,"output":[21,21,21,21,32,32,32,32,45,45,45,45],"cloneProductionSteps":0}],"oneTimeSaving":122,"verificationCost":563,"trial":[{"output":48,"cost":60,"x":16,"y":3},{"output":48,"cost":20,"x":16,"y":3}]});
console.log(JSON.stringify({tier:"NON_NATIVE_COMPACT_DESCRIPTOR_CERTIFICATION",summary,known,grid,sources:Object.fromEntries(["none","first_edge","even_edge","odd_edge","odd_opcode","even_operand","even_sub"].map(f=>[f,buildCompact(f)]))},null,2));
