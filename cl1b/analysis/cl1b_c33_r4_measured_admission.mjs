#!/usr/bin/env node
// C33-R4 paid measured-span admission with no extra trial computation.
import fs from "node:fs";
import assert from "node:assert/strict";
function buildRec(policy="evolve"){
 const g=[],l={};const label=k=>{if(k in l)throw Error(k);l[k]=g.length};const op=(kind,args)=>g.push({kind,...args});
 const C=(dst,value,next)=>op("Const",{dst,value,next}),R=(dst,address,next)=>op("Read",{dst,address,next}),W=(address,value,next)=>op("Write",{address,value,next}),S=(dst,lhs,rhs,next)=>op("Sub",{dst,lhs,rhs,next}),A=(dst,lhs,rhs,next)=>op("Add",{dst,lhs,rhs,next}),B=(test,nonzero,zero)=>op("Branch",{test,nonzero,zero}),Q=(dst,next)=>op("SenseOwnCredits",{dst,next}),P=(node,target,next)=>op("PatchNext",{node,target,next}),K=(node,value,next)=>op("PatchConst",{node,value,next});
 const set=(key,value,next)=>{C(3,key);C(0,value);W(3,0,next)};
 const pe=(at,to,next)=>{C(1,"@"+at);C(0,"@"+to);P(1,0,next)};
 label("init");C(0,0,policy==="plain"?"prod_init":"generate_init");
 label("generate_init");C(2,8);label("double_src");A(2,2,2);C(1,"@parent_add");op("CloneNode",{dst:3,source:1});C(2,"@out");P(3,2);
 C(2,1);S(2,3,2);C(1,"@base");K(1,2);C(2,0);A(0,3,2);C(2,7);A(1,0,2);
 label("generate_even");C(2,"@double_src");op("CloneNode",{dst:3,source:2});P(3,0);S(2,0,1);B(2,"generate_odd","generate_done");
 label("generate_odd");C(2,"@parent_add");op("CloneNode",{dst:2,source:2});P(2,3);C(2,1);A(0,0,2,"generate_even");
 label("generate_done");pe("kernel_dispatch","base","prod_init");


 label("prod_init");C(0,0,"batch");
 label("batch");pe("done","first_result");C(3,50,"prepare");
 label("second");pe("done","second_result");C(3,52,"prepare");
 label("prepare");R(1,3);C(0,1);A(3,3,0);R(2,3);C(3,43);W(3,1);C(3,44);W(3,2);C(3,41);W(3,2);C(3,58);R(0,3);C(3,42);W(3,0,"loop");
 label("loop");C(3,43);R(1,3);C(3,41);R(2,3,"kernel_dispatch");
 label("kernel_dispatch");C(0,0,"parent");
 label("parent");C(3,1);
 label("parent_add");A(0,0,2);S(1,1,3);B(1,"parent_add","out");
 label("base");C(3,0);A(1,1,3);C(3,"@special_entry");P(3,1);
 label("special_entry");C(0,0,"parent_add");
 label("out");C(3,44);R(1,3);A(0,0,1);C(3,41);W(3,0);C(3,42);R(1,3);C(2,1);S(1,1,2);W(3,1);B(1,"loop","done");
 label("done");C(3,41);R(0,3);label("return_route");C(1,0,"first_result");
 label("first_result");C(3,11);W(3,0,"decision");
 label("decision");C(0,0,policy==="evolve"?"probe":policy==="static"?"gate":"second");
 const equal=(name,yes,no)=>{label(name);C(3,50);R(0,3);C(3,52);R(1,3);S(0,0,1);B(0,no,name+"_b");label(name+"_b");C(3,51);R(0,3);C(3,53);R(1,3);S(0,0,1);B(0,no,yes);};
 label("probe");C(0,0,"observe");equal("observe","install","second");
 label("install");pe("decision","gate");C(1,"@flag");C(0,1);label("activate");K(1,0,"flag");label("flag");C(0,0);B(0,"compile","gate");label("compile");pe("gate","check","gate");
 label("gate");C(0,0,policy==="static"?"check":"second");
 equal("check","reuse","second");
 label("reuse");C(3,11);R(0,3,"second_result");
 label("second_result");C(3,12);W(3,0,"batch");
 for(let i=0;i<g.length;i++){const n=g[i];if(n.kind==="Branch"){n.nonzero=l[n.nonzero];n.zero=l[n.zero]}else n.next=n.next===undefined?i+1:l[n.next];if(n.kind==="Const"&&typeof n.value==="string")n.value=l[n.value.slice(1)];}
 // call return edges are on the last instruction of done, not the read sequence.
 for(const n of g)if(n.kind==="Const"&&n.value===l.done&&n.dst===1)n.value=l.return_route;
 return {g,l};
}
function depth(x){let n=0;while(x){n+=x%2;if(x>1)n++;x=Math.floor(x/2);}return n;}
function recurrence(a,b,k){let z=b;for(let i=0;i<k;i++)z=a*z+b;return z;}
function runRec(s,policy="evolve",off=false){
 const {g,l}=buildRec(policy),initial=g.length,d=new Map([...s.xy.map((v,i)=>[50+i,v]),[58,s.k]]),v=[0,0,0,0];let pc=0,paid=0,prod=0,peak=g.length+1+d.size,clones=0,kernels=0,serviceKernels=0,serviceClones=0,cloneSteps=0,firstExposure=null,reused=false,firstOutput=null,expectedEntry=null,probeSteps=0;const outputs=[],activations=[],prefix=[],services=[],patches=[];
 const inputs=()=>[50,51,52,53].map(k=>d.get(k));
 if(![s.xy,...s.changes.map(a=>a.slice(1))].every(a=>a.length===4&&a.every(x=>Number.isInteger(x)&&x>=1&&x<=16))||![1,2,4,8].includes(s.k))throw Error("domain");
 while(paid<200000){
  const p=pc,n={...g[p]};if(!n.kind)throw Error("pc");
  if(p===l.activate&&firstExposure===null)firstExposure=paid+1;
  if(firstExposure===null&&policy==="evolve")prefix.push([p,[...v],[...d],g.length,n]);
  if(p>=l.probe&&p<l.gate)probeSteps++;
  if(p===l.batch){reused=false;firstOutput=null;}
  if(p===l.reuse)reused=true;
  if(p===l.kernel_dispatch){kernels++;serviceKernels++;expectedEntry=initial+v[1]-1;const [a,b,c,e]=inputs(),factor=firstOutput===null?a:c,bias=firstOutput===null?b:e,iter=(serviceKernels-1)%s.k;if(v[1]!==factor||v[2]!==recurrence(factor,bias,iter))throw Error("kernel input");}
  if(p>=initial){serviceClones++;cloneSteps++;}
  paid++;pc=n.next;
  switch(n.kind){
   case "Const":v[n.dst]=n.value;break;
   case "Read":v[n.dst]=d.get(v[n.address])??0;break;
   case "Write":{
    const k=v[n.address],z=v[n.value];d.set(k,z);const [a,b,c,e]=inputs();
    if(k===11){if(z!==recurrence(a,b,s.k)||firstOutput!==null||serviceKernels!==s.k)throw Error("first output");firstOutput=z;}
    if(k===12){
     if(z!==recurrence(c,e,s.k)||firstOutput!==recurrence(a,b,s.k))throw Error("second output "+s.id+" "+prod);
     const active=g[l.gate].next===l.check,choice=active&&a===c&&b===e;
     if(reused!==choice||serviceKernels!==s.k*(choice?1:2))throw Error("reuse");
     if(serviceClones!==(policy==="plain"?0:s.k*(depth(a)+(choice?0:depth(c)))))throw Error("clone count");
     prod++;outputs.push([firstOutput,z]);services.push({at:prod,reused,paid,serviceKernels,serviceClones,active});serviceKernels=serviceClones=0;
     if(prod===s.n)return {paid,outputs,activations,services,clones,kernels,cloneSteps,nodes:g.length,peak,firstExposure,prefix,probeSteps,patches};
     for(const [at,...q] of s.changes)if(at===prod)for(let i=0;i<4;i++)d.set(50+i,q[i]);
    }break;}
   case "Add":v[n.dst]=v[n.lhs]+v[n.rhs];break;
   case "Sub":v[n.dst]=v[n.lhs]-v[n.rhs];break;
   case "Branch":pc=v[n.test]!==0?n.nonzero:n.zero;break;
   case "CloneNode":{const from=v[n.source],idx=g.length,x=idx-initial+1;if(from!==(x%2?l.parent_add:l.double_src))throw Error("template");g.push({...g[from]});v[n.dst]=idx;clones++;break;}
   case "PatchConst":{const at=v[n.node];if(at===l.flag){const [a,b,c,e]=inputs();if(p!==l.activate||a!==c||b!==e||v[n.value]!==1||activations.length)throw Error("activation");activations.push({at:prod,paid,suppressed:off});if(!off)g[at].value=1;}else{if(at!==l.base||g[at].kind!=="Const"||v[n.value]!==initial-1)throw Error("base");g[at].value=v[n.value];}break;}
   case "PatchNext":{
    const at=v[n.node],to=v[n.target];if(!g[to]||g[at].next===undefined)throw Error("edge");
    if(at>=initial){const x=at-initial+1;if(to!==(x===1?l.out:x%2?at-1:initial+x/2-1))throw Error("generated edge");}
    else if(at===l.special_entry){if(to!==expectedEntry)throw Error("entry");}
    else if(at===l.kernel_dispatch){if(to!==l.base)throw Error("kernel");}
    else if(at===l.return_route){if(to!==(firstOutput===null?l.first_result:l.second_result))throw Error("return");}
    else if(at===l.decision){if(to!==l.gate)throw Error("probe retire");}
    else if(at===l.gate){const [a,b,c,e]=inputs();if(a!==c||b!==e||p!==l.compile+2||to!==l.check||activations.length!==1||off)throw Error("compile");}
    else throw Error("target "+at);
    const suppress=false;patches.push([paid,p,at,g[at].next,to,suppress]);if(!suppress)g[at].next=to;break;}
   default:throw Error(n.kind);
  }
  if(!v.every(Number.isSafeInteger))throw Error("integer");
  peak=Math.max(peak,g.length+1+d.size);if(g.length>200||d.size>16||peak>220)throw Error("capacity "+g.length+" "+peak);
 }
 throw Error("budget");
}

function buildProof(policy="evolve",fault="none"){
 const g=[],l={};const label=k=>{if(k in l)throw Error(k);l[k]=g.length};const op=(kind,args)=>g.push({kind,...args});
 const C=(dst,value,next)=>op("Const",{dst,value,next}),R=(dst,address,next)=>op("Read",{dst,address,next}),W=(address,value,next)=>op("Write",{address,value,next}),S=(dst,lhs,rhs,next)=>op("Sub",{dst,lhs,rhs,next}),A=(dst,lhs,rhs,next)=>op("Add",{dst,lhs,rhs,next}),B=(test,nonzero,zero)=>op("Branch",{test,nonzero,zero}),Q=(dst,next)=>op("SenseOwnCredits",{dst,next}),P=(node,target,next)=>op("PatchNext",{node,target,next}),K=(node,value,next)=>op("PatchConst",{node,value,next});
 const set=(key,value,next)=>{C(3,key);C(0,value);W(3,0,next)};
 const pe=(at,to,next)=>{C(1,"@"+at);C(0,"@"+to);P(1,0,next)};
 label("init");C(0,0,policy==="plain"?"prod_init":"generate_init");
 label("generate_init");C(2,8);label("double_src");A(2,2,2);C(1,"@parent_add");op("CloneNode",{dst:3,source:1});C(2,"@out");P(3,2);
 C(2,1);S(2,3,2);C(1,"@base");K(1,2);C(2,0);A(0,3,2);C(2,7);A(1,0,2);
 label("generate_even");C(2,"@double_src");op("CloneNode",{dst:3,source:2});P(3,0);S(2,0,1);B(2,"generate_odd","generate_done");
 label("generate_odd");C(2,"@parent_add");op("CloneNode",{dst:2,source:2});P(2,3);C(2,1);A(0,0,2,"generate_even");
 label("generate_done");pe("kernel_dispatch","base","prod_init");


 label("prod_init");C(0,0,"batch");
 label("batch");pe("done","first_result");C(3,50,"prepare");
 label("second");pe("done","second_result");C(3,52,"prepare");
 label("prepare");R(1,3);C(0,1);A(3,3,0);R(2,3);C(3,43);W(3,1);C(3,44);W(3,2);C(3,41);W(3,2);C(3,58);R(0,3);C(3,42);W(3,0,"loop");
 label("loop");C(3,43);R(1,3);C(3,41);R(2,3,"kernel_dispatch");
 label("kernel_dispatch");C(0,0,"parent");
 label("parent");C(3,1);
 label("parent_add");A(0,0,2);S(1,1,3);B(1,"parent_add","out");
 label("base");C(3,0);A(1,1,3);C(3,"@special_entry");P(3,1);
 label("special_entry");C(0,0,"parent_add");
 label("out");C(3,44);R(1,3);A(0,0,1);C(3,41);W(3,0);C(3,42);R(1,3);C(2,1);S(1,1,2);W(3,1);B(1,"loop","done");
 label("done");C(3,41);R(0,3);label("return_route");C(1,0,"first_result");
 label("first_result");C(3,11);W(3,0,"decision");
 label("decision");C(0,0,policy==="evolve"?"probe":policy==="static"?"gate":"second");
 const equal=(name,yes,no)=>{label(name);C(3,50);R(0,3);C(3,52);R(1,3);S(0,0,1);B(0,no,name+"_b");label(name+"_b");C(3,51);R(0,3);C(3,53);R(1,3);S(0,0,1);B(0,no,yes);};
 label("probe");C(0,0,"observe");equal("observe","install","second");
 label("install");pe("decision","gate");C(1,"@flag");C(0,1);label("activate");K(1,0,"flag");label("flag");C(0,0);B(0,"compile","gate");label("compile");pe("gate","check","gate");
 label("gate");C(0,0,policy==="static"?"check":"second");
 equal("check","verify_a","second");
 // Use the actual retained first invocation parameters and completed loop state.
 label("verify_a");C(3,43);R(0,3);C(3,52);R(1,3);S(0,0,1);B(0,"reject","verify_b");
 label("verify_b");C(3,44);R(0,3);C(3,53);R(1,3);S(0,0,1);B(0,"reject","verify_done");
 label("verify_done");C(3,42);R(0,3);B(0,"reject","candidate");
 label("candidate");C(3,fault==="wrong_cache_cell"?44:11);R(0,3);C(3,41);R(1,3);S(2,0,1);B(2,"reject","reuse");
 label("reuse");C(3,12,"second_result");
 label("reject");C(0,0,"second");
 label("second_result");C(3,12);W(3,0,"batch");
 for(let i=0;i<g.length;i++){const n=g[i];if(n.kind==="Branch"){n.nonzero=l[n.nonzero];n.zero=l[n.zero]}else n.next=n.next===undefined?i+1:l[n.next];if(n.kind==="Const"&&typeof n.value==="string")n.value=l[n.value.slice(1)];}
 // call return edges are on the last instruction of done, not the read sequence.
 for(const n of g)if(n.kind==="Const"&&n.value===l.done&&n.dst===1)n.value=l.return_route;
 if(fault==="skip_bias_guard")g[l.check+5].zero=l.verify_a;if(fault==="post_check")g[l.reuse]={kind:"Const",dst:0,value:0,next:l.second_result};return {g,l};
}
function depth(x){let n=0;while(x){n+=x%2;if(x>1)n++;x=Math.floor(x/2);}return n;}
function recurrence(a,b,k){let z=b;for(let i=0;i<k;i++)z=a*z+b;return z;}
function runProof(s,policy="evolve",off=false,fault="none"){
 const {g,l}=buildProof(policy,fault),initial=g.length,d=new Map([...s.xy.map((v,i)=>[50+i,v]),[58,s.k]]),v=[0,0,0,0];let pc=0,paid=0,prod=0,peak=g.length+1+d.size,clones=0,kernels=0,serviceKernels=0,serviceClones=0,cloneSteps=0,firstExposure=null,reused=false,firstOutput=null,expectedEntry=null,probeSteps=0,rejections=0,proofSteps=0,wrong=null;const outputs=[],activations=[],prefix=[],services=[],patches=[];
 const inputs=()=>[50,51,52,53].map(k=>d.get(k));
 if(![s.xy,...s.changes.map(a=>a.slice(1))].every(a=>a.length===4&&a.every(x=>Number.isInteger(x)&&x>=1&&x<=16))||![1,2,4,8].includes(s.k))throw Error("domain");
 while(paid<200000){
  const p=pc,n={...g[p]};if(!n.kind)throw Error("pc");
  if(p===l.activate&&firstExposure===null)firstExposure=paid+1;
  if(firstExposure===null&&policy==="evolve")prefix.push([p,[...v],[...d],g.length,n]);
  if(p>=l.probe&&p<l.gate)probeSteps++;
  if(p===l.batch){reused=false;firstOutput=null;}
  if(p===l.reuse)reused=true;if(p===l.reject)rejections++;if(p>=l.verify_a&&p<l.reuse)proofSteps++;
  if(p===l.kernel_dispatch){kernels++;serviceKernels++;expectedEntry=initial+v[1]-1;const [a,b,c,e]=inputs(),factor=firstOutput===null?a:c,bias=firstOutput===null?b:e,iter=(serviceKernels-1)%s.k;if(v[1]!==factor||v[2]!==recurrence(factor,bias,iter))throw Error("kernel input");}
  if(p>=initial){serviceClones++;cloneSteps++;}
  paid++;pc=n.next;
  switch(n.kind){
   case "Const":v[n.dst]=n.value;break;
   case "Read":v[n.dst]=d.get(v[n.address])??0;break;
   case "Write":{
    const k=v[n.address],z=v[n.value];d.set(k,z);const [a,b,c,e]=inputs();
    if(k===11){if(z!==recurrence(a,b,s.k)||firstOutput!==null||serviceKernels!==s.k)throw Error("first output");firstOutput=z;}
    if(k===12){
     if(z!==recurrence(c,e,s.k)||firstOutput!==recurrence(a,b,s.k)){if(fault==="post_check")wrong={actual:z,expected:recurrence(c,e,s.k)};else throw Error("second output "+s.id+" "+prod);}
     const active=g[l.gate].next===l.check,choice=active&&a===c&&b===e;
     if(reused!==(choice&&fault!=="wrong_cache_cell")||serviceKernels!==s.k*(reused?1:2))throw Error("reuse");
     if(serviceClones!==(policy==="plain"?0:s.k*(depth(a)+(reused?0:depth(c)))))throw Error("clone count");
     prod++;outputs.push([firstOutput,z]);services.push({at:prod,reused,paid,serviceKernels,serviceClones,active});serviceKernels=serviceClones=0;
     if(prod===s.n)return {paid,outputs,activations,services,clones,kernels,cloneSteps,nodes:g.length,peak,firstExposure,prefix,probeSteps,patches,rejections,proofSteps,wrong};
     for(const [at,...q] of s.changes)if(at===prod)for(let i=0;i<4;i++)d.set(50+i,q[i]);
    }break;}
   case "Add":v[n.dst]=v[n.lhs]+v[n.rhs];break;
   case "Sub":v[n.dst]=v[n.lhs]-v[n.rhs];break;
   case "Branch":pc=v[n.test]!==0?n.nonzero:n.zero;break;
   case "CloneNode":{const from=v[n.source],idx=g.length,x=idx-initial+1;if(from!==(x%2?l.parent_add:l.double_src))throw Error("template");g.push({...g[from]});v[n.dst]=idx;clones++;break;}
   case "PatchConst":{const at=v[n.node];if(at===l.flag){const [a,b,c,e]=inputs();if(p!==l.activate||a!==c||b!==e||v[n.value]!==1||activations.length)throw Error("activation");activations.push({at:prod,paid,suppressed:off});if(!off)g[at].value=1;}else{if(at!==l.base||g[at].kind!=="Const"||v[n.value]!==initial-1)throw Error("base");g[at].value=v[n.value];}break;}
   case "PatchNext":{
    const at=v[n.node],to=v[n.target];if(!g[to]||g[at].next===undefined)throw Error("edge");
    if(at>=initial){const x=at-initial+1;if(to!==(x===1?l.out:x%2?at-1:initial+x/2-1))throw Error("generated edge");}
    else if(at===l.special_entry){if(to!==expectedEntry)throw Error("entry");}
    else if(at===l.kernel_dispatch){if(to!==l.base)throw Error("kernel");}
    else if(at===l.return_route){if(to!==(firstOutput===null?l.first_result:l.second_result))throw Error("return");}
    else if(at===l.decision){if(to!==l.gate)throw Error("probe retire");}
    else if(at===l.gate){const [a,b,c,e]=inputs();if(a!==c||b!==e||p!==l.compile+2||to!==l.check||activations.length!==1||off)throw Error("compile");}
    else throw Error("target "+at);
    const suppress=false;patches.push([paid,p,at,g[at].next,to,suppress]);if(!suppress)g[at].next=to;break;}
   default:throw Error(n.kind);
  }
  if(!v.every(Number.isSafeInteger))throw Error("integer");
  peak=Math.max(peak,g.length+1+d.size);if(g.length>200||d.size>16||peak>220)throw Error("capacity "+g.length+" "+peak);
 }
 throw Error("budget");
}


function buildFused(policy="evolve",fault="none"){
 const g=[],l={};const label=k=>{if(k in l)throw Error(k);l[k]=g.length};const op=(kind,args)=>g.push({kind,...args});
 const C=(dst,value,next)=>op("Const",{dst,value,next}),R=(dst,address,next)=>op("Read",{dst,address,next}),W=(address,value,next)=>op("Write",{address,value,next}),S=(dst,lhs,rhs,next)=>op("Sub",{dst,lhs,rhs,next}),A=(dst,lhs,rhs,next)=>op("Add",{dst,lhs,rhs,next}),B=(test,nonzero,zero)=>op("Branch",{test,nonzero,zero}),Q=(dst,next)=>op("SenseOwnCredits",{dst,next}),P=(node,target,next)=>op("PatchNext",{node,target,next}),K=(node,value,next)=>op("PatchConst",{node,value,next});
 const set=(key,value,next)=>{C(3,key);C(0,value);W(3,0,next)};
 const pe=(at,to,next)=>{C(1,"@"+at);C(0,"@"+to);P(1,0,next)};
 label("init");C(0,0,policy==="plain"?"prod_init":"generate_init");
 label("generate_init");C(2,8);label("double_src");A(2,2,2);C(1,"@parent_add");op("CloneNode",{dst:3,source:1});C(2,"@out");P(3,2);
 C(2,1);S(2,3,2);C(1,"@base");K(1,2);C(2,0);A(0,3,2);C(2,7);A(1,0,2);
 label("generate_even");C(2,"@double_src");op("CloneNode",{dst:3,source:2});P(3,0);S(2,0,1);B(2,"generate_odd","generate_done");
 label("generate_odd");C(2,"@parent_add");op("CloneNode",{dst:2,source:2});P(2,3);C(2,1);A(0,0,2,"generate_even");
 label("generate_done");pe("kernel_dispatch","base","prod_init");


 label("prod_init");C(0,0,"batch");
 label("batch");pe("done","first_result");C(3,50,"prepare");
 label("second");pe("done","second_result");C(3,52,"prepare");
 label("prepare");R(1,3);C(0,1);A(3,3,0);R(2,3);C(3,43);W(3,1);C(3,44);W(3,2);C(3,41);W(3,2);C(3,58);R(0,3);C(3,42);W(3,0,"loop");
 label("loop");C(3,43);R(1,3);C(3,41);R(2,3,"kernel_dispatch");
 label("kernel_dispatch");C(0,0,"parent");
 label("parent");C(3,1);
 label("parent_add");A(0,0,2);S(1,1,3);B(1,"parent_add","out");
 label("base");C(3,0);A(1,1,3);C(3,"@special_entry");P(3,1);
 label("special_entry");C(0,0,"parent_add");
 label("out");C(3,44);R(1,3);A(0,0,1);C(3,41);W(3,0);C(3,42);R(1,3);C(2,1);S(1,1,2);W(3,1);B(1,"loop","done");
 label("done");C(3,41);R(0,3);label("return_route");C(1,0,"first_result");
 label("first_result");C(3,11);W(3,0,"decision");
 label("decision");C(0,0,policy==="evolve"?"probe":policy==="static"?"gate":"second");
 const equal=(name,yes,no)=>{label(name);C(3,50);R(0,3);C(3,52);R(1,3);S(0,0,1);B(0,no,name+"_b");label(name+"_b");C(3,51);R(0,3);C(3,53);R(1,3);S(0,0,1);B(0,no,yes);};
 label("probe");C(0,0,"observe");equal("observe","install","second");
 label("install");pe("decision","gate");C(1,"@flag");C(0,1);label("activate");K(1,0,"flag");label("flag");C(0,0);B(0,"compile","gate");label("compile");pe("gate","check","gate");
 label("gate");C(0,0,policy==="static"?"check":"second");
 label("check");C(0,0,"verify_a");
 // Use the actual retained first invocation parameters and completed loop state.
 label("verify_a");C(3,43);R(0,3);C(3,52);R(1,3);S(0,0,1);B(0,"reject","verify_b");
 label("verify_b");C(3,44);R(0,3);C(3,53);R(1,3);S(0,0,1);B(0,"reject","verify_done");
 label("verify_done");C(3,42);R(0,3);B(0,"reject","candidate");
 label("candidate");C(3,fault==="wrong_cache_cell"?44:11);R(0,3);C(3,41);R(1,3);S(2,0,1);B(2,"reject","reuse");
 label("reuse");C(3,12,"second_result");
 label("reject");C(0,0,"second");
 label("second_result");C(3,12);W(3,0,"batch");
 for(let i=0;i<g.length;i++){const n=g[i];if(n.kind==="Branch"){n.nonzero=l[n.nonzero];n.zero=l[n.zero]}else n.next=n.next===undefined?i+1:l[n.next];if(n.kind==="Const"&&typeof n.value==="string")n.value=l[n.value.slice(1)];}
 // call return edges are on the last instruction of done, not the read sequence.
 for(const n of g)if(n.kind==="Const"&&n.value===l.done&&n.dst===1)n.value=l.return_route;
 if(fault==="skip_probe_bias")g[l.observe+5].zero=l.install;if(fault==="skip_verify_bias")g[l.verify_a+5].zero=l.verify_done;if(fault==="post_check")g[l.reuse]={kind:"Const",dst:0,value:0,next:l.second_result};return {g,l};
}
function depth(x){let n=0;while(x){n+=x%2;if(x>1)n++;x=Math.floor(x/2);}return n;}
function recurrence(a,b,k){let z=b;for(let i=0;i<k;i++)z=a*z+b;return z;}
function runFused(s,policy="evolve",off=false,fault="none"){
 const {g,l}=buildFused(policy,fault),initial=g.length,d=new Map([...s.xy.map((v,i)=>[50+i,v]),[58,s.k]]),v=[0,0,0,0];let pc=0,paid=0,prod=0,peak=g.length+1+d.size,clones=0,kernels=0,serviceKernels=0,serviceClones=0,cloneSteps=0,firstExposure=null,reused=false,firstOutput=null,expectedEntry=null,probeSteps=0,rejections=0,proofSteps=0,wrong=null;const outputs=[],activations=[],prefix=[],services=[],patches=[];
 const inputs=()=>[50,51,52,53].map(k=>d.get(k));
 if(![s.xy,...s.changes.map(a=>a.slice(1))].every(a=>a.length===4&&a.every(x=>Number.isInteger(x)&&x>=1&&x<=16))||![1,2,4,8].includes(s.k))throw Error("domain");
 while(paid<200000){
  const p=pc,n={...g[p]};if(!n.kind)throw Error("pc");
  if(p===l.activate&&firstExposure===null)firstExposure=paid+1;
  if(firstExposure===null&&policy==="evolve")prefix.push([p,[...v],[...d],g.length,n]);
  if(p>=l.probe&&p<l.gate)probeSteps++;
  if(p===l.batch){reused=false;firstOutput=null;}
  if(p===l.reuse)reused=true;if(p===l.reject)rejections++;if(p>=l.verify_a&&p<l.reuse)proofSteps++;
  if(p===l.kernel_dispatch){kernels++;serviceKernels++;expectedEntry=initial+v[1]-1;const [a,b,c,e]=inputs(),factor=firstOutput===null?a:c,bias=firstOutput===null?b:e,iter=(serviceKernels-1)%s.k;if(v[1]!==factor||v[2]!==recurrence(factor,bias,iter))throw Error("kernel input");}
  if(p>=initial){serviceClones++;cloneSteps++;}
  paid++;pc=n.next;
  switch(n.kind){
   case "Const":v[n.dst]=n.value;break;
   case "Read":v[n.dst]=d.get(v[n.address])??0;break;
   case "Write":{
    const k=v[n.address],z=v[n.value];d.set(k,z);const [a,b,c,e]=inputs();
    if(k===11){if(z!==recurrence(a,b,s.k)||firstOutput!==null||serviceKernels!==s.k)throw Error("first output");firstOutput=z;}
    if(k===12){
     if(z!==recurrence(c,e,s.k)||firstOutput!==recurrence(a,b,s.k)){if(["post_check","skip_verify_bias"].includes(fault))wrong={actual:z,expected:recurrence(c,e,s.k)};else throw Error("second output "+s.id+" "+prod);}
     const active=g[l.gate].next===l.check,choice=active&&a===c&&b===e;
     if(reused!==((choice&&fault!=="wrong_cache_cell")||fault==="skip_verify_bias")||serviceKernels!==s.k*(reused?1:2))throw Error("reuse");
     if(serviceClones!==(policy==="plain"?0:s.k*(depth(a)+(reused?0:depth(c)))))throw Error("clone count");
     prod++;outputs.push([firstOutput,z]);services.push({at:prod,reused,paid,serviceKernels,serviceClones,active});serviceKernels=serviceClones=0;
     if(prod===s.n)return {paid,outputs,activations,services,clones,kernels,cloneSteps,nodes:g.length,peak,firstExposure,prefix,probeSteps,patches,rejections,proofSteps,wrong};
     for(const [at,...q] of s.changes)if(at===prod)for(let i=0;i<4;i++)d.set(50+i,q[i]);
    }break;}
   case "Add":v[n.dst]=v[n.lhs]+v[n.rhs];break;
   case "Sub":v[n.dst]=v[n.lhs]-v[n.rhs];break;
   case "Branch":pc=v[n.test]!==0?n.nonzero:n.zero;break;
   case "CloneNode":{const from=v[n.source],idx=g.length,x=idx-initial+1;if(from!==(x%2?l.parent_add:l.double_src))throw Error("template");g.push({...g[from]});v[n.dst]=idx;clones++;break;}
   case "PatchConst":{const at=v[n.node];if(at===l.flag){const [a,b,c,e]=inputs();if(p!==l.activate||a!==c||(b!==e&&fault!=="skip_probe_bias")||v[n.value]!==1||activations.length)throw Error("activation");activations.push({at:prod,paid,suppressed:off});if(!off)g[at].value=1;}else{if(at!==l.base||g[at].kind!=="Const"||v[n.value]!==initial-1)throw Error("base");g[at].value=v[n.value];}break;}
   case "PatchNext":{
    const at=v[n.node],to=v[n.target];if(!g[to]||g[at].next===undefined)throw Error("edge");
    if(at>=initial){const x=at-initial+1;if(to!==(x===1?l.out:x%2?at-1:initial+x/2-1))throw Error("generated edge");}
    else if(at===l.special_entry){if(to!==expectedEntry)throw Error("entry");}
    else if(at===l.kernel_dispatch){if(to!==l.base)throw Error("kernel");}
    else if(at===l.return_route){if(to!==(firstOutput===null?l.first_result:l.second_result))throw Error("return");}
    else if(at===l.decision){if(to!==l.gate)throw Error("probe retire");}
    else if(at===l.gate){const [a,b,c,e]=inputs();if(a!==c||(b!==e&&fault!=="skip_probe_bias")||p!==l.compile+2||to!==l.check||activations.length!==1||off)throw Error("compile");}
    else throw Error("target "+at);
    const suppress=false;patches.push([paid,p,at,g[at].next,to,suppress]);if(!suppress)g[at].next=to;break;}
   default:throw Error(n.kind);
  }
  if(!v.every(Number.isSafeInteger))throw Error("integer");
  peak=Math.max(peak,g.length+1+d.size);if(g.length>200||d.size>16||peak>220)throw Error("capacity "+g.length+" "+peak);
 }
 throw Error("budget");
}



function buildAdmission(policy="evolve",fault="none"){
 const g=[],l={};const label=k=>{if(k in l)throw Error(k);l[k]=g.length};const op=(kind,args)=>g.push({kind,...args});
 const C=(dst,value,next)=>op("Const",{dst,value,next}),R=(dst,address,next)=>op("Read",{dst,address,next}),W=(address,value,next)=>op("Write",{address,value,next}),S=(dst,lhs,rhs,next)=>op("Sub",{dst,lhs,rhs,next}),A=(dst,lhs,rhs,next)=>op("Add",{dst,lhs,rhs,next}),B=(test,nonzero,zero)=>op("Branch",{test,nonzero,zero}),Q=(dst,next)=>op("SenseOwnCredits",{dst,next}),P=(node,target,next)=>op("PatchNext",{node,target,next}),K=(node,value,next)=>op("PatchConst",{node,value,next});
 const set=(key,value,next)=>{C(3,key);C(0,value);W(3,0,next)};
 const pe=(at,to,next)=>{C(1,"@"+at);C(0,"@"+to);P(1,0,next)};
 label("init");C(0,0,policy==="plain"?"prod_init":"generate_init");
 label("generate_init");C(2,8);label("double_src");A(2,2,2);C(1,"@parent_add");op("CloneNode",{dst:3,source:1});C(2,"@out");P(3,2);
 C(2,1);S(2,3,2);C(1,"@base");K(1,2);C(2,0);A(0,3,2);C(2,7);A(1,0,2);
 label("generate_even");C(2,"@double_src");op("CloneNode",{dst:3,source:2});P(3,0);S(2,0,1);B(2,"generate_odd","generate_done");
 label("generate_odd");C(2,"@parent_add");op("CloneNode",{dst:2,source:2});P(2,3);C(2,1);A(0,0,2,"generate_even");
 label("generate_done");pe("kernel_dispatch","base","prod_init");


 label("prod_init");C(0,0,"batch");
 label("batch");pe("done","first_result");C(3,50,"prepare");
 label("second");pe("done","second_result");C(3,52,"prepare");
 label("prepare");R(1,3);C(0,1);A(3,3,0);R(2,3);C(3,43);W(3,1);C(3,44);W(3,2);C(3,41);W(3,2);C(3,58);R(0,3);C(3,42);W(3,0,"loop");
 label("loop");C(3,43);R(1,3);C(3,41);R(2,3,"kernel_dispatch");
 label("kernel_dispatch");C(0,0,"parent");
 label("parent");C(3,1);
 label("parent_add");A(0,0,2);S(1,1,3);B(1,"parent_add","out");
 label("base");C(3,0);A(1,1,3);C(3,"@special_entry");P(3,1);
 label("special_entry");C(0,0,"parent_add");
 label("out");C(3,44);R(1,3);A(0,0,1);C(3,41);W(3,0);C(3,42);R(1,3);C(2,1);S(1,1,2);W(3,1);B(1,"loop","done");
 label("done");C(3,41);R(0,3);label("return_route");C(1,0,"first_result");
 label("first_result");C(3,11);W(3,0,"decision");
 label("decision");C(0,0,["evolve","high"].includes(policy)?"probe":policy==="static"?"gate":"second");
 const equal=(name,yes,no)=>{label(name);C(3,50);R(0,3);C(3,52);R(1,3);S(0,0,1);B(0,no,name+"_b");label(name+"_b");C(3,51);R(0,3);C(3,53);R(1,3);S(0,0,1);B(0,no,yes);};
 label("probe");C(0,0,"observe");equal("observe","measure","second");
 label("install");pe("decision","gate");C(1,"@flag");C(0,1);label("activate");K(1,0,"flag");label("flag");C(0,0);B(0,"compile","publish");label("compile");pe("gate","check","publish");
 label("measure");Q(0);C(3,59);W(3,0);pe("done","measure_end");C(3,52,"prepare");
 label("measure_end");Q(0);C(3,59);R(1,3);S(0,1,0);C(1,policy==="high"?96:48);C(2,1);
 label("compare");S(0,0,2);B(0,"compare_threshold","deny");
 label("compare_threshold");S(1,1,2);B(1,"compare","install");
 label("deny");pe("decision","gate","publish");
 label("publish");C(3,41);R(0,3,"second_result");
 label("gate");C(0,0,policy==="static"?"check":"second");
 label("check");C(0,0,"verify_a");
 // Use the actual retained first invocation parameters and completed loop state.
 label("verify_a");C(3,43);R(0,3);C(3,52);R(1,3);S(0,0,1);B(0,"reject","verify_b");
 label("verify_b");C(3,44);R(0,3);C(3,53);R(1,3);S(0,0,1);B(0,"reject","verify_done");
 label("verify_done");C(3,42);R(0,3);B(0,"reject","candidate");
 label("candidate");C(3,fault==="wrong_cache_cell"?44:11);R(0,3);C(3,41);R(1,3);S(2,0,1);B(2,"reject","reuse");
 label("reuse");C(3,12,"second_result");
 label("reject");C(0,0,"second");
 label("second_result");C(3,12);W(3,0,"batch");
 for(let i=0;i<g.length;i++){const n=g[i];if(n.kind==="Branch"){n.nonzero=l[n.nonzero];n.zero=l[n.zero]}else n.next=n.next===undefined?i+1:l[n.next];if(n.kind==="Const"&&typeof n.value==="string")n.value=l[n.value.slice(1)];}
 // call return edges are on the last instruction of done, not the read sequence.
 for(const n of g)if(n.kind==="Const"&&n.value===l.done&&n.dst===1)n.value=l.return_route;
 if(fault==="skip_probe_bias")g[l.observe+5].zero=l.install;if(fault==="skip_verify_bias")g[l.verify_a+5].zero=l.verify_done;if(fault==="post_check")g[l.reuse]={kind:"Const",dst:0,value:0,next:l.second_result};return {g,l};
}
function depth(x){let n=0;while(x){n+=x%2;if(x>1)n++;x=Math.floor(x/2);}return n;}
function recurrence(a,b,k){let z=b;for(let i=0;i<k;i++)z=a*z+b;return z;}
function runAdmission(s,policy="evolve",off=false,fault="none"){
 const {g,l}=buildAdmission(policy,fault),initial=g.length,d=new Map([...s.xy.map((v,i)=>[50+i,v]),[58,s.k]]),v=[0,0,0,0];let pc=0,paid=0,prod=0,peak=g.length+1+d.size,clones=0,kernels=0,serviceKernels=0,serviceClones=0,cloneSteps=0,firstExposure=null,reused=false,firstOutput=null,expectedEntry=null,probeSteps=0,rejections=0,proofSteps=0,wrong=null,measureStart=null,measurement=null,calibrating=false,comparePaid=0;const outputs=[],activations=[],prefix=[],services=[],patches=[];
 const inputs=()=>[50,51,52,53].map(k=>d.get(k));
 if(![s.xy,...s.changes.map(a=>a.slice(1))].every(a=>a.length===4&&a.every(x=>Number.isInteger(x)&&x>=1&&x<=16))||![1,2,4,8].includes(s.k))throw Error("domain");
 while(paid<200000){
  const p=pc,n={...g[p]};if(!n.kind)throw Error("pc");
  if(p===l.activate&&firstExposure===null)firstExposure=paid+1;
  if(firstExposure===null&&["evolve","high"].includes(policy))prefix.push([p,[...v],[...d],g.length,n]);
  if(p>=l.probe&&p<l.gate)probeSteps++;
  if(p===l.batch){reused=false;firstOutput=null;calibrating=false;}if(p===l.measure){calibrating=true;measureStart=paid+1;}if(p===l.measure_end){measurement={at:prod,cost:paid+1-measureStart,admitted:null};}if(p>=l.compare&&p<l.deny)comparePaid++;if(p===l.install){measurement.admitted=true;}if(p===l.deny){measurement.admitted=false;}
  if(p===l.reuse)reused=true;if(p===l.reject)rejections++;if(p>=l.verify_a&&p<l.reuse)proofSteps++;
  if(p===l.kernel_dispatch){kernels++;serviceKernels++;expectedEntry=initial+v[1]-1;const [a,b,c,e]=inputs(),factor=firstOutput===null?a:c,bias=firstOutput===null?b:e,iter=(serviceKernels-1)%s.k;if(v[1]!==factor||v[2]!==recurrence(factor,bias,iter))throw Error("kernel input");}
  if(p>=initial){serviceClones++;cloneSteps++;}
  paid++;pc=n.next;
  switch(n.kind){
   case "Const":v[n.dst]=n.value;break;
   case "Read":v[n.dst]=d.get(v[n.address])??0;break;
   case "Write":{
    const k=v[n.address],z=v[n.value];d.set(k,z);const [a,b,c,e]=inputs();
    if(k===11){if(z!==recurrence(a,b,s.k)||firstOutput!==null||serviceKernels!==s.k)throw Error("first output");firstOutput=z;}
    if(k===12){
     if(z!==recurrence(c,e,s.k)||firstOutput!==recurrence(a,b,s.k)){if(["post_check","skip_verify_bias"].includes(fault))wrong={actual:z,expected:recurrence(c,e,s.k)};else throw Error("second output "+s.id+" "+prod);}
     const active=g[l.gate].next===l.check,choice=active&&!calibrating&&a===c&&b===e;
     if(reused!==((choice&&fault!=="wrong_cache_cell")||fault==="skip_verify_bias")||serviceKernels!==s.k*(reused?1:2))throw Error("reuse");
     if(serviceClones!==(policy==="plain"?0:s.k*(depth(a)+(reused?0:depth(c)))))throw Error("clone count");
     prod++;outputs.push([firstOutput,z]);services.push({at:prod,reused,paid,serviceKernels,serviceClones,active});serviceKernels=serviceClones=0;
     if(prod===s.n)return {paid,outputs,activations,services,clones,kernels,cloneSteps,nodes:g.length,peak,firstExposure,prefix,probeSteps,patches,rejections,proofSteps,wrong,measurement,comparePaid};
     for(const [at,...q] of s.changes)if(at===prod)for(let i=0;i<4;i++)d.set(50+i,q[i]);
    }break;}
   case "SenseOwnCredits":v[n.dst]=200000-paid;break;
   case "Add":v[n.dst]=v[n.lhs]+v[n.rhs];break;
   case "Sub":v[n.dst]=v[n.lhs]-v[n.rhs];break;
   case "Branch":pc=v[n.test]!==0?n.nonzero:n.zero;break;
   case "CloneNode":{const from=v[n.source],idx=g.length,x=idx-initial+1;if(from!==(x%2?l.parent_add:l.double_src))throw Error("template");g.push({...g[from]});v[n.dst]=idx;clones++;break;}
   case "PatchConst":{const at=v[n.node];if(at===l.flag){const [a,b,c,e]=inputs();if(!measurement?.admitted||p!==l.activate||a!==c||(b!==e&&fault!=="skip_probe_bias")||v[n.value]!==1||activations.length)throw Error("activation");activations.push({at:prod,paid,suppressed:off});if(!off)g[at].value=1;}else{if(at!==l.base||g[at].kind!=="Const"||v[n.value]!==initial-1)throw Error("base");g[at].value=v[n.value];}break;}
   case "PatchNext":{
    const at=v[n.node],to=v[n.target];if(!g[to]||g[at].next===undefined)throw Error("edge");
    if(at>=initial){const x=at-initial+1;if(to!==(x===1?l.out:x%2?at-1:initial+x/2-1))throw Error("generated edge");}
    else if(at===l.special_entry){if(to!==expectedEntry)throw Error("entry");}
    else if(at===l.kernel_dispatch){if(to!==l.base)throw Error("kernel");}
    else if(at===l.return_route){if(to!==(firstOutput===null?l.first_result:calibrating?l.measure_end:l.second_result))throw Error("return");}
    else if(at===l.decision){if(to!==l.gate)throw Error("probe retire");}
    else if(at===l.gate){const [a,b,c,e]=inputs();if(a!==c||(b!==e&&fault!=="skip_probe_bias")||p!==l.compile+2||to!==l.check||activations.length!==1||off)throw Error("compile");}
    else throw Error("target "+at);
    const suppress=false;patches.push([paid,p,at,g[at].next,to,suppress]);if(!suppress)g[at].next=to;break;}
   default:throw Error(n.kind);
  }
  if(p===l.measure_end+3&&v[0]!==measurement.cost)throw Error("measured cost");if((p===l.install||p===l.deny)&&measurement.admitted!==(measurement.cost>(policy==="high"?96:48)))throw Error("threshold");
  if(!v.every(Number.isSafeInteger))throw Error("integer");
  peak=Math.max(peak,g.length+1+d.size);if(g.length>200||d.size>16||peak>220)throw Error("capacity "+g.length+" "+peak);
 }
 throw Error("budget");
}




const cases=JSON.parse(fs.readFileSync(new URL("../studies/CL1B-C33-R1-NATIVE-REUSE-frozen-82-worlds-v1.json",import.meta.url),"utf8")).cases;

for(const k of [1,2,4,8])for(const n of [16,80])for(const pattern of ["dense","rare","none","fall"]){
 const tuple=i=>{const a=1+(i*5+3)%16,b=1+(i*7+2)%16;const hit=pattern==="dense"?i%3!==2:pattern==="rare"?i===Math.floor(n/2):pattern==="fall"?i<n/4:false;return hit?[a,b,a,b]:[a,b,a,b%16+1];};
 cases.push({id:"grid_"+k+"_"+n+"_"+pattern,cohort:"designed_grid",k,n,pattern,xy:tuple(0),changes:Array.from({length:n-1},(_,i)=>[i+1,...tuple(i+1)])});
}
const rows=cases.map(s=>{
 const shared=runRec(s,"shared"),r3=runFused(s),low=runAdmission(s),high=runAdmission(s,"high"),lowOff=runAdmission(s,"evolve",true),highOff=runAdmission(s,"high",true);
 for(const q of [r3,low,high,lowOff,highOff])if(JSON.stringify(q.outputs)!==JSON.stringify(shared.outputs))throw Error("outputs");
 for(const [on,off] of [[low,lowOff],[high,highOff]]){
 if(JSON.stringify(on.prefix)!==JSON.stringify(off.prefix)||on.firstExposure!==off.firstExposure||JSON.stringify(on.measurement)!==JSON.stringify(off.measurement))throw Error("pair");
 if(on.measurement&&on.measurement.admitted!==Boolean(on.activations.length))throw Error("admit");
 if(off.services.some(x=>x.reused)||on.clones!==16||off.clones!==16)throw Error("counts");
 }
 const compact=q=>({paid:q.paid,kernels:q.kernels,nodes:q.nodes,peak:q.peak,reused:q.services.filter(x=>x.reused).length,proofSteps:q.proofSteps??0,measurement:q.measurement??null,comparePaid:q.comparePaid??0});
 return {id:s.id,cohort:s.cohort,k:s.k,n:s.n,pattern:s.pattern??null,shared:compact(shared),r3:compact(r3),low:compact(low),high:compact(high),lowOff:compact(lowOff),highOff:compact(highOff)};
});
function agg(rows){return Object.fromEntries(["r3","low","high","lowOff","highOff"].map(k=>[k,{net:rows.reduce((s,r)=>s+r.shared.paid-r[k].paid,0),wins:rows.filter(r=>r.shared.paid>r[k].paid).length,paid:rows.reduce((s,r)=>s+r[k].paid,0),measured:rows.filter(r=>r[k].measurement).length,admitted:rows.filter(r=>r[k].measurement?.admitted).length,comparePaid:rows.reduce((s,r)=>s+r[k].comparePaid,0),reused:rows.reduce((s,r)=>s+r[k].reused,0),peak:Math.max(...rows.map(r=>r[k].peak)),nodes:Math.max(...rows.map(r=>r[k].nodes))}]))}
const summary={all:agg(rows),cohorts:Object.fromEntries(["calibration","prospective","designed_grid"].map(k=>[k,agg(rows.filter(r=>r.cohort===k))])),scales:Object.fromEntries([1,2,4,8].map(k=>[k,agg(rows.filter(r=>r.k===k))])),patterns:Object.fromEntries(["dense","rare","none","fall"].map(k=>[k,agg(rows.filter(r=>r.pattern===k))])),worlds:rows.length*6};

assert.deepEqual(summary,{"all":{"r3":{"net":24871,"wins":23,"paid":494192,"measured":0,"admitted":0,"comparePaid":0,"reused":532,"peak":161,"nodes":149},"low":{"net":13634,"wins":18,"paid":505429,"measured":38,"admitted":32,"comparePaid":7280,"reused":392,"peak":184,"nodes":171},"high":{"net":6444,"wins":13,"paid":512619,"measured":38,"admitted":19,"comparePaid":11890,"reused":241,"peak":184,"nodes":171},"lowOff":{"net":-18560,"wins":0,"paid":537623,"measured":38,"admitted":32,"comparePaid":7280,"reused":0,"peak":184,"nodes":171},"highOff":{"net":-23105,"wins":0,"paid":542168,"measured":38,"admitted":19,"comparePaid":11890,"reused":0,"peak":184,"nodes":171}},"cohorts":{"calibration":{"r3":{"net":1068,"wins":2,"paid":22050,"measured":0,"admitted":0,"comparePaid":0,"reused":36,"peak":161,"nodes":149},"low":{"net":543,"wins":2,"paid":22575,"measured":2,"admitted":2,"comparePaid":384,"reused":34,"peak":184,"nodes":171},"high":{"net":-180,"wins":1,"paid":23298,"measured":2,"admitted":1,"comparePaid":578,"reused":11,"peak":184,"nodes":171},"lowOff":{"net":-974,"wins":0,"paid":24092,"measured":2,"admitted":2,"comparePaid":384,"reused":0,"peak":184,"nodes":171},"highOff":{"net":-1163,"wins":0,"paid":24281,"measured":2,"admitted":1,"comparePaid":578,"reused":0,"peak":184,"nodes":171}},"prospective":{"r3":{"net":7354,"wins":8,"paid":120351,"measured":0,"admitted":0,"comparePaid":0,"reused":132,"peak":161,"nodes":149},"low":{"net":4165,"wins":7,"paid":123540,"measured":12,"admitted":10,"comparePaid":2296,"reused":103,"peak":184,"nodes":171},"high":{"net":1956,"wins":5,"paid":125749,"measured":12,"admitted":6,"comparePaid":3800,"reused":60,"peak":184,"nodes":171},"lowOff":{"net":-3814,"wins":0,"paid":131519,"measured":12,"admitted":10,"comparePaid":2296,"reused":0,"peak":184,"nodes":171},"highOff":{"net":-5298,"wins":0,"paid":133003,"measured":12,"admitted":6,"comparePaid":3800,"reused":0,"peak":184,"nodes":171}},"designed_grid":{"r3":{"net":16449,"wins":13,"paid":351791,"measured":0,"admitted":0,"comparePaid":0,"reused":364,"peak":161,"nodes":149},"low":{"net":8926,"wins":9,"paid":359314,"measured":24,"admitted":20,"comparePaid":4600,"reused":255,"peak":184,"nodes":171},"high":{"net":4668,"wins":7,"paid":363572,"measured":24,"admitted":12,"comparePaid":7512,"reused":170,"peak":184,"nodes":171},"lowOff":{"net":-13772,"wins":0,"paid":382012,"measured":24,"admitted":20,"comparePaid":4600,"reused":0,"peak":184,"nodes":171},"highOff":{"net":-16644,"wins":0,"paid":384884,"measured":24,"admitted":12,"comparePaid":7512,"reused":0,"peak":184,"nodes":171}}},"scales":{"1":{"r3":{"net":-2196,"wins":4,"paid":56250,"measured":0,"admitted":0,"comparePaid":0,"reused":148,"peak":161,"nodes":149},"low":{"net":-4581,"wins":1,"paid":58635,"measured":10,"admitted":4,"comparePaid":1904,"reused":36,"peak":184,"nodes":171},"high":{"net":-4609,"wins":0,"paid":58663,"measured":10,"admitted":0,"comparePaid":1924,"reused":0,"peak":184,"nodes":171},"lowOff":{"net":-4609,"wins":0,"paid":58663,"measured":10,"admitted":4,"comparePaid":1904,"reused":0,"peak":184,"nodes":171},"highOff":{"net":-4609,"wins":0,"paid":58663,"measured":10,"admitted":0,"comparePaid":1924,"reused":0,"peak":184,"nodes":171}},"2":{"r3":{"net":297,"wins":5,"paid":81350,"measured":0,"admitted":0,"comparePaid":0,"reused":124,"peak":161,"nodes":149},"low":{"net":-1959,"wins":4,"paid":83606,"measured":9,"admitted":9,"comparePaid":1728,"reused":115,"peak":184,"nodes":171},"high":{"net":-5473,"wins":0,"paid":87120,"measured":9,"admitted":0,"comparePaid":2670,"reused":0,"peak":184,"nodes":171},"lowOff":{"net":-4576,"wins":0,"paid":86223,"measured":9,"admitted":9,"comparePaid":1728,"reused":0,"peak":184,"nodes":171},"highOff":{"net":-5473,"wins":0,"paid":87120,"measured":9,"admitted":0,"comparePaid":2670,"reused":0,"peak":184,"nodes":171}},"4":{"r3":{"net":7729,"wins":7,"paid":128814,"measured":0,"admitted":0,"comparePaid":0,"reused":136,"peak":161,"nodes":149},"low":{"net":4741,"wins":6,"paid":131802,"measured":10,"admitted":10,"comparePaid":1920,"reused":126,"peak":184,"nodes":171},"high":{"net":2821,"wins":6,"paid":133722,"measured":10,"admitted":10,"comparePaid":3840,"reused":126,"peak":184,"nodes":171},"lowOff":{"net":-4655,"wins":0,"paid":141198,"measured":10,"admitted":10,"comparePaid":1920,"reused":0,"peak":184,"nodes":171},"highOff":{"net":-6575,"wins":0,"paid":143118,"measured":10,"admitted":10,"comparePaid":3840,"reused":0,"peak":184,"nodes":171}},"8":{"r3":{"net":19041,"wins":7,"paid":227778,"measured":0,"admitted":0,"comparePaid":0,"reused":124,"peak":161,"nodes":149},"low":{"net":15433,"wins":7,"paid":231386,"measured":9,"admitted":9,"comparePaid":1728,"reused":115,"peak":184,"nodes":171},"high":{"net":13705,"wins":7,"paid":233114,"measured":9,"admitted":9,"comparePaid":3456,"reused":115,"peak":184,"nodes":171},"lowOff":{"net":-4720,"wins":0,"paid":251539,"measured":9,"admitted":9,"comparePaid":1728,"reused":0,"peak":184,"nodes":171},"highOff":{"net":-6448,"wins":0,"paid":253267,"measured":9,"admitted":9,"comparePaid":3456,"reused":0,"peak":184,"nodes":171}}},"patterns":{"dense":{"r3":{"net":21798,"wins":8,"paid":70262,"measured":0,"admitted":0,"comparePaid":0,"reused":260,"peak":161,"nodes":149},"low":{"net":18464,"wins":6,"paid":73596,"measured":8,"admitted":6,"comparePaid":1532,"reused":189,"peak":184,"nodes":171},"high":{"net":14896,"wins":4,"paid":77164,"measured":8,"admitted":4,"comparePaid":2488,"reused":126,"peak":184,"nodes":171},"lowOff":{"net":-2154,"wins":0,"paid":94214,"measured":8,"admitted":6,"comparePaid":1532,"reused":0,"peak":184,"nodes":171},"highOff":{"net":-3100,"wins":0,"paid":95160,"measured":8,"admitted":4,"comparePaid":2488,"reused":0,"peak":184,"nodes":171}},"rare":{"r3":{"net":-4692,"wins":0,"paid":96752,"measured":0,"admitted":0,"comparePaid":0,"reused":8,"peak":161,"nodes":149},"low":{"net":-7072,"wins":0,"paid":99132,"measured":8,"admitted":8,"comparePaid":1536,"reused":0,"peak":184,"nodes":171},"high":{"net":-6752,"wins":0,"paid":98812,"measured":8,"admitted":4,"comparePaid":2536,"reused":0,"peak":184,"nodes":171},"lowOff":{"net":-4472,"wins":0,"paid":96532,"measured":8,"admitted":8,"comparePaid":1536,"reused":0,"peak":184,"nodes":171},"highOff":{"net":-5452,"wins":0,"paid":97512,"measured":8,"admitted":4,"comparePaid":2536,"reused":0,"peak":184,"nodes":171}},"none":{"r3":{"net":-5160,"wins":0,"paid":100880,"measured":0,"admitted":0,"comparePaid":0,"reused":0,"peak":161,"nodes":149},"low":{"net":-5160,"wins":0,"paid":100880,"measured":0,"admitted":0,"comparePaid":0,"reused":0,"peak":183,"nodes":171},"high":{"net":-5160,"wins":0,"paid":100880,"measured":0,"admitted":0,"comparePaid":0,"reused":0,"peak":183,"nodes":171},"lowOff":{"net":-5160,"wins":0,"paid":100880,"measured":0,"admitted":0,"comparePaid":0,"reused":0,"peak":183,"nodes":171},"highOff":{"net":-5160,"wins":0,"paid":100880,"measured":0,"admitted":0,"comparePaid":0,"reused":0,"peak":183,"nodes":171}},"fall":{"r3":{"net":4335,"wins":5,"paid":87725,"measured":0,"admitted":0,"comparePaid":0,"reused":96,"peak":161,"nodes":149},"low":{"net":2526,"wins":3,"paid":89534,"measured":8,"admitted":6,"comparePaid":1532,"reused":66,"peak":184,"nodes":171},"high":{"net":1516,"wins":3,"paid":90544,"measured":8,"admitted":4,"comparePaid":2488,"reused":44,"peak":184,"nodes":171},"lowOff":{"net":-2154,"wins":0,"paid":94214,"measured":8,"admitted":6,"comparePaid":1532,"reused":0,"peak":184,"nodes":171},"highOff":{"net":-3100,"wins":0,"paid":95160,"measured":8,"admitted":4,"comparePaid":2488,"reused":0,"peak":184,"nodes":171}}},"worlds":288});
console.log(JSON.stringify({tier:"NON_NATIVE_MEASURED_ADMISSION",summary,rows,sources:{low:buildAdmission(),high:buildAdmission("high")}},null,2));
