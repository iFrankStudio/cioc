#!/usr/bin/env node
// C30-R11 paid graph-local path search, not a certification replacement.
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
function optimalDepth(x){const a=[0,1];for(let i=2;i<=x;i++)a[i]=1+(i%2?a[i-1]:Math.min(a[i-1],a[i/2]));return a[x];}
function pathDepth(g,p,out){let n=0;while(p!==out){if(++n>16)throw Error("cycle");p=g[p].next;}return n;}
function buildSearch(tieHalf=false){
 const g=[],l={};const label=k=>{if(k in l)throw Error(k);l[k]=g.length};
 const op=(kind,args)=>g.push({kind,...args});
 const C=(dst,value,next)=>op("Const",{dst,value,next}),R=(dst,address,next)=>op("Read",{dst,address,next}),W=(address,value,next)=>op("Write",{address,value,next}),S=(dst,lhs,rhs,next)=>op("Sub",{dst,lhs,rhs,next}),A=(dst,lhs,rhs,next)=>op("Add",{dst,lhs,rhs,next}),B=(test,nonzero,zero)=>op("Branch",{test,nonzero,zero}),Q=(dst,next)=>op("SenseOwnCredits",{dst,next}),P=(node,target,next)=>op("PatchNext",{node,target,next}),K=(node,value,next)=>op("PatchConst",{node,value,next});
 const set=(key,value,next)=>{C(3,key);C(0,value);W(3,0,next)};
 label("init");C(2,8);label("double_src");A(2,2,2);C(3,40);W(3,2);
 C(1,"@parent_add");op("CloneNode",{dst:3,source:1});C(2,"@out");P(3,2);
 C(2,1);S(2,3,2);C(1,"@base");K(1,2);
 C(2,0);A(0,3,2);A(1,3,2);
 label("search_start");C(3,48);W(3,0);C(3,49);W(3,1);C(3,"@out");
 label("chase");op("ReadNext",{dst:0,node:0});op("ReadNext",{dst:1,node:1});
 if(tieHalf){S(2,0,3);B(2,"check_pred","pick_half");label("check_pred");S(2,1,3);B(2,"chase","pick_pred");}
 else{S(2,1,3);B(2,"check_half","pick_pred");label("check_half");S(2,0,3);B(2,"chase","pick_half");}
 label("pick_pred");C(3,49);R(0,3);C(2,"@parent_add","make_even");
 label("pick_half");C(3,48);R(0,3);C(2,"@double_src","make_even");
 label("make_even");op("CloneNode",{dst:3,source:2});P(3,0);
 C(2,48);R(0,2);C(1,"eighth_clone");S(2,0,1);B(2,"generate_odd","setup_trial");
 label("generate_odd");C(2,"@parent_add");op("CloneNode",{dst:1,source:2});P(1,3);C(2,1);A(0,0,2,"search_start");
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
 for(let i=0;i<g.length;i++){const n=g[i];if(n.kind==="Branch"){n.nonzero=l[n.nonzero];n.zero=l[n.zero]}else n.next=n.next===undefined?i+1:l[n.next];if(n.kind==="Const"&&typeof n.value==="string")n.value=n.value==="eighth_clone"?g.length+7:l[n.value.slice(1)];}
 return {g,l};
}
function runSearch(s,off=false,tieHalf=false){
 if(![s.xy,...(s.changes??[]).map(z=>z.slice(1))].every(z=>z.every(x=>Number.isInteger(x)&&x>=1&&x<=16)))throw Error("domain");
 const {g,l}=buildSearch(tieHalf),initial=g.length,d=new Map([[50,s.xy[0]],[51,s.xy[1]]]),v=[0,0,0,0];let pc=0,paid=0,peak=g.length+3,prod=0,cloneTrialSteps=0,cloneProductionSteps=0,serviceClones=0,firstExposure=null,firstProd=null,searchReads=0;const choices=[],clones=[],patches=[],trials=[],output=[],prefix=[];
 while(paid<20000){
 const p=pc,n=g[p];if(!n)throw Error("pc");if(p===l.activation)firstExposure=paid+1;
 if(firstExposure===null&&prod===0)prefix.push([p,[...v],Array.from(d.entries()),g.map(x=>({...x}))]);
 paid++;pc=n.next;
 if(p>=initial){if(g[l.out].next===l.prod_out){cloneProductionSteps++;serviceClones++;}else cloneTrialSteps++;}
 switch(n.kind){
 case "Const":v[n.dst]=n.value;break;
 case "Read":v[n.dst]=d.get(v[n.address])??0;break;
 case "ReadNext":{const at=v[n.node];if(at<initial||at>=g.length)throw Error("search outside existing generated DAG");v[n.dst]=g[at].next;searchReads++;break;}
 case "Write":{
 const k=v[n.address],a=v[n.value];d.set(k,a);
 if(k===46)trials.push({output:d.get(44),cost:a,x:d.get(40),y:d.get(51)});
 if(k===11){
 if(a!==d.get(50)*d.get(51))throw Error("bad production");
 const active=g[l.dispatch].next===l.base;
 if(serviceClones!==(active?optimalDepth(d.get(50)):0))throw Error("clone count");
 serviceClones=0;prod++;output.push(a);if(firstProd===null)firstProd=paid;
 peak=Math.max(peak,g.length+1+d.size);
 if(prod===s.n)return {paid,choices,searchReads,initial,nodes:g.length,peak,firstProd,clones,patches,trials,output,active,flag:g[l.flag].value,cloneTrialSteps,cloneProductionSteps,firstExposure,prefix};
 for(const [at,x,y] of s.changes??[])if(at===prod){d.set(50,x);d.set(51,y);}
 }break;}
 case "SenseOwnCredits":v[n.dst]=20000-paid;break;
 case "Add":v[n.dst]=v[n.lhs]+v[n.rhs];break;
 case "Sub":v[n.dst]=v[n.lhs]-v[n.rhs];break;
 case "Branch":pc=v[n.test]!==0?n.nonzero:n.zero;break;
 case "CloneNode":{const src=v[n.source],idx=g.length;if(src!==l.parent_add&&src!==l.double_src)throw Error("clone source");if((idx-initial+1)%2&&src!==l.parent_add)throw Error("odd layout");g.push({...g[src]});v[n.dst]=idx;clones.push([paid,src,idx]);break;}
 case "PatchNext":{const target=v[n.node],before=g[target].next,after=v[n.target];if(!g[after]||g[target].next===undefined)throw Error("edge");
 if(target===l.special_entry&&after!==initial+d.get(g[l.input_x].value)-1)throw Error("address");
 if(target===l.dispatch&&![l.base,l.parent].includes(after))throw Error("dispatch target");if(target>=initial){
 const x=target-initial+1;if(x===1){if(after!==l.out)throw Error("first edge");}
 else if(x%2){if(after!==target-1)throw Error("odd edge");}
 else{const pred=target-1,half=initial+x/2-1,dp=pathDepth(g,pred,l.out),dh=pathDepth(g,half,l.out);
 const useHalf=dh<dp||(tieHalf&&dh===dp),expected=useHalf?half:pred;
 if(after!==expected||g[target].dst!==(useHalf?2:0)||g[target].lhs!==(useHalf?2:0)||g[target].rhs!==2)throw Error("search selection");
 choices.push({x,predDepth:dp,halfDepth:dh,selected:useHalf?"half":"pred",readCost:2*Math.min(dp,dh)});}
 }
 g[target].next=after;patches.push([paid,p,target,before,after,false]);break;}
 case "PatchConst":{const target=v[n.node],before=g[target].value,after=v[n.value],suppress=off&&p===l.activation;if(g[target].kind!=="Const")throw Error("const");if(!suppress)g[target].value=after;patches.push([paid,p,target,before,after,suppress]);break;}
 default:throw Error(n.kind);}
 peak=Math.max(peak,g.length+1+d.size);if(g.length>200||d.size>16||peak>220)throw Error("capacity");
 }throw Error("budget");
}
const cases=JSON.parse(fs.readFileSync(new URL("../studies/CL1B-C30-R10-CONSTRUCTION-PROVENANCE-frozen-77-worlds-v1.json",import.meta.url),"utf8")).cases;

function test(s){
 const plain=legacyRun(s,"plain"),full=runCompiled(s),binary=runBinary(s),on=runSearch(s),off=runSearch(s,true),half=runSearch(s,false,true);
 for(const q of [full,binary,on,off,half])if(q.output.join()!==plain.output.join())throw Error("output");
 if(JSON.stringify(on.prefix)!==JSON.stringify(off.prefix)||on.firstExposure!==off.firstExposure)throw Error("prefix");
 for(const q of [on,off,half])if(q.clones.length!==16||q.choices.length!==8||q.searchReads!==q.choices.reduce((a,b)=>a+b.readCost,0)||q.cloneTrialSteps!==optimalDepth(16)||q.trials[0].output!==q.trials[1].output||q.trials[0].cost!==60||q.trials[1].cost!==15+optimalDepth(16))throw Error("construction");
 if(!on.active||off.active||off.cloneProductionSteps!==0||!half.active)throw Error("activation");
 const compact=x=>({paid:x.paid,nodes:x.nodes,peak:x.peak,trial:x.trials,cloneProductionSteps:x.cloneProductionSteps,searchReads:x.searchReads??0});
 return {id:s.id,group:s.group??s.cohort,plain:compact(plain),full:compact(full),binary:compact(binary),search:compact(on),off:compact(off),tieHalf:compact(half)};
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
const domain=runSearch({id:"domain",xy:[16,16],n:257,changes:pairs.map(([x,y],i)=>[i+1,x,y])});
const tie=runSearch({id:"domain",xy:[16,16],n:257,changes:pairs.map(([x,y],i)=>[i+1,x,y])},false,true);
if(domain.output.length!==257||domain.output.join()!==tie.output.join())throw Error("domain");
function agg(rows){return Object.fromEntries(["full","binary","search","off","tieHalf"].map(k=>[k,{n:rows.length,net:rows.reduce((s,r)=>s+r.plain.paid-r[k].paid,0),wins:rows.filter(r=>r.plain.paid>r[k].paid).length,paid:rows.reduce((s,r)=>s+r[k].paid,0),nodes:Math.max(...rows.map(r=>r[k].nodes)),peak:Math.max(...rows.map(r=>r[k].peak))}]))}
const summary={known:agg(known.filter(r=>r.group==="prospective")),grid:agg(grid),groups:Object.fromEntries(["stable","rise","fall","mixed"].map(k=>[k,agg(grid.filter(r=>r.group===k))])),domain:{pairs:256,paid:domain.paid,production:domain.output.length,cloneProductionSteps:domain.cloneProductionSteps,nodes:domain.nodes,peak:domain.peak,choices:domain.choices,searchReads:domain.searchReads},tieHalf:{paid:tie.paid,choices:tie.choices,searchReads:tie.searchReads,cloneProductionSteps:tie.cloneProductionSteps},overheadVsR6:known[0].search.paid-known[0].binary.paid,trial:known[0].search.trial,depths:Array.from({length:16},(_,i)=>({x:i+1,optimal:optimalDepth(i+1),binary:binarySteps(i+1)}))};

assert.deepEqual(summary,{"known":{"full":{"n":12,"net":9818,"wins":10,"paid":29788,"nodes":116,"peak":127},"binary":{"n":12,"net":14311,"wins":10,"paid":25295,"nodes":125,"peak":136},"search":{"n":12,"net":11515,"wins":9,"paid":28091,"nodes":143,"peak":156},"off":{"n":12,"net":-7464,"wins":0,"paid":47070,"nodes":143,"peak":156},"tieHalf":{"n":12,"net":11683,"wins":9,"paid":27923,"nodes":143,"peak":156}},"grid":{"full":{"n":80,"net":35380,"wins":33,"paid":132790,"nodes":116,"peak":127},"binary":{"n":80,"net":60207,"wins":43,"paid":107963,"nodes":125,"peak":136},"search":{"n":80,"net":41567,"wins":32,"paid":126603,"nodes":143,"peak":156},"off":{"n":80,"net":-49760,"wins":0,"paid":217930,"nodes":143,"peak":156},"tieHalf":{"n":80,"net":42687,"wins":32,"paid":125483,"nodes":143,"peak":156}},"groups":{"stable":{"full":{"n":20,"net":8382,"wins":9,"paid":32966,"nodes":116,"peak":127},"binary":{"n":20,"net":14300,"wins":11,"paid":27048,"nodes":125,"peak":136},"search":{"n":20,"net":9640,"wins":9,"paid":31708,"nodes":143,"peak":156},"off":{"n":20,"net":-12440,"wins":0,"paid":53788,"nodes":143,"peak":156},"tieHalf":{"n":20,"net":9920,"wins":9,"paid":31428,"nodes":143,"peak":156}},"rise":{"full":{"n":20,"net":24372,"wins":14,"paid":40961,"nodes":116,"peak":127},"binary":{"n":20,"net":36850,"wins":15,"paid":28483,"nodes":125,"peak":136},"search":{"n":20,"net":32190,"wins":13,"paid":33143,"nodes":143,"peak":156},"off":{"n":20,"net":-12440,"wins":0,"paid":77773,"nodes":143,"peak":156},"tieHalf":{"n":20,"net":32470,"wins":13,"paid":32863,"nodes":143,"peak":156}},"fall":{"full":{"n":20,"net":-6378,"wins":0,"paid":25586,"nodes":116,"peak":127},"binary":{"n":20,"net":-5175,"wins":2,"paid":24383,"nodes":125,"peak":136},"search":{"n":20,"net":-9835,"wins":0,"paid":29043,"nodes":143,"peak":156},"off":{"n":20,"net":-12440,"wins":0,"paid":31648,"nodes":143,"peak":156},"tieHalf":{"n":20,"net":-9555,"wins":0,"paid":28763,"nodes":143,"peak":156}},"mixed":{"full":{"n":20,"net":9004,"wins":10,"paid":33277,"nodes":116,"peak":127},"binary":{"n":20,"net":14232,"wins":15,"paid":28049,"nodes":125,"peak":136},"search":{"n":20,"net":9572,"wins":10,"paid":32709,"nodes":143,"peak":156},"off":{"n":20,"net":-12440,"wins":0,"paid":54721,"nodes":143,"peak":156},"tieHalf":{"n":20,"net":9852,"wins":10,"paid":32429,"nodes":143,"peak":156}}},"domain":{"pairs":256,"paid":4851,"production":257,"cloneProductionSteps":1141,"nodes":143,"peak":156,"choices":[{"x":2,"predDepth":1,"halfDepth":1,"selected":"pred","readCost":2},{"x":4,"predDepth":3,"halfDepth":2,"selected":"half","readCost":4},{"x":6,"predDepth":4,"halfDepth":3,"selected":"half","readCost":6},{"x":8,"predDepth":5,"halfDepth":3,"selected":"half","readCost":6},{"x":10,"predDepth":5,"halfDepth":4,"selected":"half","readCost":8},{"x":12,"predDepth":6,"halfDepth":4,"selected":"half","readCost":8},{"x":14,"predDepth":6,"halfDepth":5,"selected":"half","readCost":10},{"x":16,"predDepth":7,"halfDepth":4,"selected":"half","readCost":8}],"searchReads":52},"tieHalf":{"paid":4837,"choices":[{"x":2,"predDepth":1,"halfDepth":1,"selected":"half","readCost":2},{"x":4,"predDepth":3,"halfDepth":2,"selected":"half","readCost":4},{"x":6,"predDepth":4,"halfDepth":3,"selected":"half","readCost":6},{"x":8,"predDepth":5,"halfDepth":3,"selected":"half","readCost":6},{"x":10,"predDepth":5,"halfDepth":4,"selected":"half","readCost":8},{"x":12,"predDepth":6,"halfDepth":4,"selected":"half","readCost":8},{"x":14,"predDepth":6,"halfDepth":5,"selected":"half","readCost":10},{"x":16,"predDepth":7,"halfDepth":4,"selected":"half","readCost":8}],"searchReads":52,"cloneProductionSteps":1141},"overheadVsR6":233,"trial":[{"output":48,"cost":60,"x":16,"y":3},{"output":48,"cost":20,"x":16,"y":3}],"depths":[{"x":1,"optimal":1,"binary":1},{"x":2,"optimal":2,"binary":2},{"x":3,"optimal":3,"binary":3},{"x":4,"optimal":3,"binary":3},{"x":5,"optimal":4,"binary":4},{"x":6,"optimal":4,"binary":4},{"x":7,"optimal":5,"binary":5},{"x":8,"optimal":4,"binary":4},{"x":9,"optimal":5,"binary":5},{"x":10,"optimal":5,"binary":5},{"x":11,"optimal":6,"binary":6},{"x":12,"optimal":5,"binary":5},{"x":13,"optimal":6,"binary":6},{"x":14,"optimal":6,"binary":6},{"x":15,"optimal":7,"binary":7},{"x":16,"optimal":5,"binary":5}]});
console.log(JSON.stringify({tier:"NON_NATIVE_PAID_GRAPH_LOCAL_SEARCH",summary,known,grid,sources:{predTie:buildSearch(false),halfTie:buildSearch(true)}},null,2));
