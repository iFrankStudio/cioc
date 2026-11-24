#!/usr/bin/env node
// C30-R5 non-native partial and incremental executable construction.
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

function buildIncremental(k=8,extend=true){
 const g=[],l={};const label=x=>{if(x in l)throw Error(x);l[x]=g.length};
 const op=(kind,args)=>g.push({kind,...args});
 const C=(dst,value,next)=>op("Const",{dst,value,next}),R=(dst,address,next)=>op("Read",{dst,address,next}),W=(address,value,next)=>op("Write",{address,value,next}),S=(dst,lhs,rhs,next)=>op("Sub",{dst,lhs,rhs,next}),A=(dst,lhs,rhs,next)=>op("Add",{dst,lhs,rhs,next}),B=(test,nonzero,zero)=>op("Branch",{test,nonzero,zero}),Q=(dst,next)=>op("SenseOwnCredits",{dst,next}),P=(node,target,next)=>op("PatchNext",{node,target,next}),K=(node,value,next)=>op("PatchConst",{node,value,next}),N=(dst,node,next)=>op("ReadNext",{dst,node,next});
 const set=(key,value,next)=>{C(3,key);C(0,value);W(3,0,next)};
 const pc=(at,value,next)=>{C(1,"@"+at);C(0,value);K(1,0,next)};
 const pe=(at,target,next)=>{C(1,"@"+at);C(0,"@"+target);P(1,0,next)};
 label("init");set(40,k);set(49,0);C(0,k);C(1,"@out","generate");
 label("generate");C(2,"@parent_add");op("CloneNode",{dst:3,source:2});P(3,1);C(2,0);A(1,3,2);C(2,"table_delta");A(2,1,2);P(2,1);C(2,1);S(0,0,2);B(0,"generate","link");
 label("link");C(3,48);W(3,1);C(3,49);R(0,3);B(0,"link_full","setup_trial");
 label("link_full");C(2,16);S(0,1,2);C(1,"@base");K(1,0,"setup_trial");
 label("setup_trial");set(47,0,"measure");
 label("measure");Q(0);C(3,42);W(3,0,"entry");
 label("entry");label("input_x");C(3,40);label("dispatch");R(1,3,"parent");
 label("parent");C(3,51);R(2,3);C(0,0);C(3,1);
 label("parent_add");A(0,0,2);S(1,1,3);B(1,"parent_add","out");
 label("table_path");C(3,"table_base");A(3,1,3);N(3,3);C(0,"@special_entry");P(0,3,"read_y");
 label("base");C(3,0);A(1,1,3);C(3,"@special_entry");P(3,1);
 label("read_y");C(3,51);R(2,3);
 label("special_entry");C(0,0,"parent_add");
 label("out");C(3,0,"trial_out");
 label("trial_out");C(3,44);W(3,0);Q(0);C(3,42);R(1,3);S(0,1,0);C(3,46);W(3,0);
 C(3,47);R(1,3);B(1,"trial_compare","save_parent");
 label("save_parent");C(3,45);W(3,0);C(3,44);R(0,3);C(3,43);W(3,0);set(47,1);
 C(3,49);R(0,3);B(0,"trial_full","trial_partial");
 label("trial_full");pe("dispatch","base","measure");
 label("trial_partial");pe("dispatch","table_path","measure");
 label("trial_compare");pe("dispatch","parent");
 C(3,43);R(0,3);C(3,44);R(1,3);S(0,0,1);B(0,"finish","compare_cost");
 label("compare_cost");C(3,45);R(0,3);C(3,46);R(1,3);S(2,0,1);S(1,1,0);C(3,1);B(2,"cmp","finish");
 label("cmp");S(2,2,3);B(2,"cmp_neg","install");label("cmp_neg");S(1,1,3);B(1,"cmp","finish");
 label("install");C(1,"@flag");C(0,1);label("activation");K(1,0,"finish");
 label("finish");pc("input_x",50);pe("out","prod_out");
 label("flag");C(0,0);B(0,"compile","entry");
 label("compile");C(3,49);R(0,3);B(0,"compile_full","compile_partial");
 label("compile_full");pe("dispatch","base","entry");
 label("compile_partial");pe("dispatch","table_path","entry");
 label("prod_out");C(3,11);W(3,0,"entry");
 if(extend){
  label("miss");pc("flag",0);pc("input_x",40);pe("out","trial_out");pe("dispatch","parent");set(40,16);set(49,1);C(3,48);R(1,3);C(0,16-k,"generate");
 }
 label("table");for(let i=1;i<=16;i++)C(0,0,extend?"miss":"parent");
 for(let i=0;i<g.length;i++){const n=g[i];if(n.kind==="Branch"){n.nonzero=l[n.nonzero];n.zero=l[n.zero]}else n.next=n.next===undefined?i+1:l[n.next];if(n.kind==="Const"&&typeof n.value==="string"){n.value=n.value==="table_delta"?l.table-g.length:n.value==="table_base"?l.table-1:l[n.value.slice(1)];}}
 return {g,l};
}
function runIncremental(s,k=8,extend=true,off=false){
 const {g,l}=buildIncremental(k,extend),initial=g.length,d=new Map([[50,s.xy[0]],[51,s.xy[1]]]),v=[0,0,0,0];let pc=0,paid=0,peak=g.length+3,prod=0,serviceClones=0,cloneProductionSteps=0,cloneTrialSteps=0,firstExposure=null,extensions=0;const clones=[],patches=[],trials=[],outputs=[],prefix=[];
 if(![s.xy,...(s.changes??[]).map(z=>z.slice(1))].every(z=>z.every(x=>Number.isInteger(x)&&x>=1&&x<=16)))throw Error("domain");
 while(paid<20000){
 const p=pc,n={...g[p]};if(!n.kind)throw Error("pc");
 if(p===l.activation&&firstExposure===null)firstExposure=paid+1;
 if(firstExposure===null)prefix.push([p,[...v],[...d.entries()]]);
 if(p===l.miss)extensions++;
 if(p>=initial){if(g[l.out].next===l.prod_out){serviceClones++;cloneProductionSteps++;}else cloneTrialSteps++;}
 if(p>=l.table&&p<initial)throw Error("metadata executed");
 paid++;pc=n.next;
 switch(n.kind){
 case "Const":v[n.dst]=n.value;break;
 case "Read":v[n.dst]=d.get(v[n.address])??0;break;
 case "ReadNext":v[n.dst]=g[v[n.node]].next;if(v[n.dst]===undefined)throw Error("read edge");break;
 case "Write":{
 const key=v[n.address],value=v[n.value];d.set(key,value);
 if(key===46)trials.push({stage:d.get(49),x:d.get(40),y:d.get(51),cost:value,output:d.get(44)});
 if(key===11){
 const x=d.get(50),y=d.get(51);if(value!==x*y)throw Error("product "+s.id+" "+k+" "+x+" "+y+" "+value);
 const mode=g[l.dispatch].next;
 if(mode===l.base&&serviceClones!==x||mode===l.table_path&&serviceClones!==(x<=clones.length?x:0)||mode===l.parent&&serviceClones!==0)throw Error("mode");
 serviceClones=0;prod++;outputs.push(value);peak=Math.max(peak,g.length+1+d.size);
 if(prod===s.n)return {paid,initial,nodes:g.length,peak,clones:clones.length,cloneProductionSteps,cloneTrialSteps,extensions,trials,firstExposure,prefix,outputs,patches};
 for(const [at,nx,ny] of s.changes??[])if(at===prod){d.set(50,nx);d.set(51,ny);}
 }break;}
 case "SenseOwnCredits":v[n.dst]=20000-paid;break;
 case "Add":v[n.dst]=v[n.lhs]+v[n.rhs];break;
 case "Sub":v[n.dst]=v[n.lhs]-v[n.rhs];break;
 case "Branch":pc=v[n.test]!==0?n.nonzero:n.zero;break;
 case "CloneNode":{const from=v[n.source],idx=g.length;if(from!==l.parent_add)throw Error("clone source");g.push({...g[from]});v[n.dst]=idx;clones.push(idx);break;}
 case "PatchNext":{const at=v[n.node],to=v[n.target];if(!g[to]||g[at].next===undefined)throw Error("edge");
 if(at>=l.table&&at<initial&&to!==initial+at-l.table)throw Error("table mapping");
 if(at===l.special_entry){const x=d.get(g[l.input_x].value),expected=x<=clones.length?initial+x-1:extend?l.miss:l.parent;if(to!==expected)throw Error("entry mapping "+to+" "+expected);}
 patches.push([paid,at,g[at].next,to]);g[at].next=to;break;}
 case "PatchConst":{const at=v[n.node];if(g[at].kind!=="Const")throw Error("const");if(!(off&&p===l.activation))g[at].value=v[n.value];break;}
 default:throw Error(n.kind);
 }
 peak=Math.max(peak,g.length+1+d.size);if(g.length>200||d.size>16||peak>220)throw Error("capacity "+g.length+" "+peak);
 }throw Error("budget");
}
const cases=JSON.parse(fs.readFileSync(new URL("../studies/CL1B-C30-R3-COMPILED-ENTRY-frozen-51-worlds-v1.json",import.meta.url),"utf8")).cases;

const policies=[["partial4",4,false],["partial8",8,false],["grow4",4,true],["grow8",8,true],["grow12",12,true]];
function test(s){
 const plain=legacyRun(s,"plain"),full=runCompiled(s);const arms={full:{paid:full.paid,clones:16,peak:full.peak,nodes:full.nodes}};
 for(const [name,k,extend] of policies){
 const a=runIncremental(s,k,extend),b=runIncremental(s,k,extend,true);
 if(a.outputs.join()!==plain.output.join()||b.outputs.join()!==plain.output.join())throw Error("outputs");
 if(a.firstExposure!==b.firstExposure||JSON.stringify(a.prefix)!==JSON.stringify(b.prefix))throw Error("prefix");
 if(b.clones!==k||b.cloneProductionSteps!==0||b.extensions!==0)throw Error("off");
 if(a.clones!==(a.extensions?16:k)||a.extensions>1||a.trials.length!==2*(1+a.extensions))throw Error("growth");
 for(let i=0;i<a.trials.length;i+=2){const p=a.trials[i],q=a.trials[i+1];if(p.output!==p.x*p.y||p.output!==q.output||p.cost<=q.cost)throw Error("trial "+JSON.stringify([p,q]));}
 arms[name]={paid:a.paid,off:b.paid,clones:a.clones,extensions:a.extensions,peak:a.peak,nodes:a.nodes,trial:a.trials,cloneProductionSteps:a.cloneProductionSteps};
 }
 return {id:s.id,group:s.group??s.cohort,plain:plain.paid,arms};
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
function agg(a){return Object.fromEntries(["full",...policies.map(x=>x[0])].map(k=>[k,{n:a.length,net:a.reduce((s,r)=>s+r.plain-r.arms[k].paid,0),wins:a.filter(r=>r.plain>r.arms[k].paid).length,extended:a.filter(r=>r.arms[k].extensions>0).length,physical:k==="full"?null:a.reduce((s,r)=>s+r.arms[k].off-r.arms[k].paid,0),nodes:Math.max(...a.map(r=>r.arms[k].nodes)),peak:Math.max(...a.map(r=>r.arms[k].peak))}]))}
const summary={known:agg(known.filter(r=>r.group==="prospective")),grid:agg(grid),groups:Object.fromEntries(["stable","rise","fall","mixed"].map(k=>[k,agg(grid.filter(r=>r.group===k))]))};

assert.deepEqual(summary,{"known":{"full":{"n":12,"net":7332,"wins":8,"extended":0,"physical":null,"nodes":116,"peak":127},"partial4":{"n":12,"net":-6818,"wins":0,"extended":0,"physical":-4682,"nodes":148,"peak":161},"partial8":{"n":12,"net":-6748,"wins":0,"extended":0,"physical":-3508,"nodes":152,"peak":165},"grow4":{"n":12,"net":5224,"wins":7,"extended":10,"physical":7360,"nodes":181,"peak":194},"grow8":{"n":12,"net":4538,"wins":7,"extended":10,"physical":7778,"nodes":181,"peak":194},"grow12":{"n":12,"net":3845,"wins":7,"extended":10,"physical":8189,"nodes":181,"peak":194}},"grid":{"full":{"n":80,"net":35380,"wins":33,"extended":0,"physical":null,"nodes":116,"peak":127},"partial4":{"n":80,"net":-39242,"wins":2,"extended":0,"physical":-25002,"nodes":148,"peak":161},"partial8":{"n":80,"net":-34806,"wins":5,"extended":0,"physical":-13206,"nodes":152,"peak":165},"grow4":{"n":80,"net":23374,"wins":30,"extended":64,"physical":37614,"nodes":181,"peak":194},"grow8":{"n":80,"net":21509,"wins":31,"extended":56,"physical":43109,"nodes":181,"peak":194},"grow12":{"n":80,"net":19284,"wins":28,"extended":47,"physical":48244,"nodes":181,"peak":194}},"groups":{"stable":{"full":{"n":20,"net":8382,"wins":9,"extended":0,"physical":null,"nodes":116,"peak":127},"partial4":{"n":20,"net":-9092,"wins":2,"extended":0,"physical":-5532,"nodes":148,"peak":161},"partial8":{"n":20,"net":-6012,"wins":5,"extended":0,"physical":-612,"nodes":152,"peak":165},"grow4":{"n":20,"net":6946,"wins":9,"extended":12,"physical":10506,"nodes":181,"peak":194},"grow8":{"n":20,"net":7000,"wins":10,"extended":8,"physical":12400,"nodes":181,"peak":194},"grow12":{"n":20,"net":6702,"wins":8,"extended":4,"physical":13942,"nodes":181,"peak":194}},"rise":{"full":{"n":20,"net":24372,"wins":14,"extended":0,"physical":null,"nodes":116,"peak":127},"partial4":{"n":20,"net":-12782,"wins":0,"extended":0,"physical":-9222,"nodes":148,"peak":161},"partial8":{"n":20,"net":-13802,"wins":0,"extended":0,"physical":-8402,"nodes":152,"peak":165},"grow4":{"n":20,"net":19770,"wins":11,"extended":20,"physical":23330,"nodes":181,"peak":194},"grow8":{"n":20,"net":18769,"wins":11,"extended":20,"physical":24169,"nodes":181,"peak":194},"grow12":{"n":20,"net":17768,"wins":10,"extended":20,"physical":25008,"nodes":181,"peak":194}},"fall":{"full":{"n":20,"net":-6378,"wins":0,"extended":0,"physical":null,"nodes":116,"peak":127},"partial4":{"n":20,"net":-6632,"wins":0,"extended":0,"physical":-3072,"nodes":148,"peak":161},"partial8":{"n":20,"net":-7652,"wins":0,"extended":0,"physical":-2252,"nodes":152,"peak":165},"grow4":{"n":20,"net":-7814,"wins":0,"extended":12,"physical":-4254,"nodes":181,"peak":194},"grow8":{"n":20,"net":-7760,"wins":0,"extended":8,"physical":-2360,"nodes":181,"peak":194},"grow12":{"n":20,"net":-8058,"wins":0,"extended":4,"physical":-818,"nodes":181,"peak":194}},"mixed":{"full":{"n":20,"net":9004,"wins":10,"extended":0,"physical":null,"nodes":116,"peak":127},"partial4":{"n":20,"net":-10736,"wins":0,"extended":0,"physical":-7176,"nodes":148,"peak":161},"partial8":{"n":20,"net":-7340,"wins":0,"extended":0,"physical":-1940,"nodes":152,"peak":165},"grow4":{"n":20,"net":4472,"wins":10,"extended":20,"physical":8032,"nodes":181,"peak":194},"grow8":{"n":20,"net":3500,"wins":10,"extended":20,"physical":8900,"nodes":181,"peak":194},"grow12":{"n":20,"net":2872,"wins":10,"extended":19,"physical":10112,"nodes":181,"peak":194}}}});
console.log(JSON.stringify({tier:"NON_NATIVE_INCREMENTAL_GENERATION_MODEL",summary,known,grid},null,2));
