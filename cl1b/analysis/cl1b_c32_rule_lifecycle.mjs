#!/usr/bin/env node
// C32-R1: sampled admission, paid miss-budget dormancy and reactivation.
import assert from "node:assert/strict";
function buildFactor(policy="evolve"){
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
 label("prod_init");set(47,0,"entry");
 label("first");C(3,50);label("first_route");C(0,0,"direct");
 label("second");C(3,52);label("second_route");C(0,0,"direct");
 label("direct");R(1,3);C(0,1);A(3,3,0);R(2,3,"kernel_dispatch");
 label("reverse");C(0,1);A(3,3,0);R(1,3);S(3,3,0);R(2,3,"kernel_dispatch");
 label("kernel_dispatch");C(0,0,"parent");
 label("parent");C(3,1);
 label("parent_add");A(0,0,2);S(1,1,3);B(1,"parent_add","out");
 label("base");C(3,0);A(1,1,3);C(3,"@special_entry");P(3,1);
 label("special_entry");C(0,0,"parent_add");
 label("out");C(3,47);R(1,3);B(1,"sum","save_first");
 label("save_first");C(3,41);W(3,0);set(47,1,"second");
 label("sum");C(3,41);R(1,3);A(0,0,1);C(3,47);C(1,0);W(3,1);
 label("result");C(3,0,"prod_out");
 label("prod_out");C(3,11);W(3,0,"entry");
 label("entry");C(0,0,policy==="evolve"?"probe_x":policy==="plain"||policy==="shared"?"first":"x_gate");
 label("probe_x");C(3,50,"observe_x");
 label("observe_x");R(1,3);C(3,52);R(2,3);S(0,1,2);B(0,"probe_y","install_x");
 label("install_x");pe("probe_x","probe_y");C(1,"@x_gate");C(0,"@check_x");label("activate_x");P(1,0,"probe_y");
 label("probe_y");C(3,51,"observe_y");
 label("observe_y");R(1,3);C(3,53);R(2,3);S(0,1,2);B(0,"x_gate","install_y");
 label("install_y");pe("probe_y","x_gate");C(1,"@y_gate");C(0,"@check_y");label("activate_y");P(1,0,"x_gate");
 label("x_gate");C(3,50,["x","both"].includes(policy)?"check_x":"y_gate");
 label("check_x");R(1,3);C(3,52);R(2,3);S(0,1,2);B(0,"y_gate","factor_x");
 label("y_gate");C(3,51,["y","both"].includes(policy)?"check_y":"first");
 label("check_y");R(1,3);C(3,53);R(2,3);S(0,1,2);B(0,"first","factor_y");
 label("factor_x");C(3,51);R(2,3);C(3,53);R(0,3);A(2,2,0,"factor_phase");
 label("factor_y");C(3,50);R(2,3);C(3,52);R(0,3);A(2,2,0,"factor_phase");
 // phase 1 plus zero partial routes the single multiplication through existing sum/production tail.
 label("factor_phase");C(3,41);C(0,0);W(3,0);C(3,47);C(0,1);W(3,0,"kernel_dispatch");
 for(let i=0;i<g.length;i++){const n=g[i];if(n.kind==="Branch"){n.nonzero=l[n.nonzero];n.zero=l[n.zero]}else n.next=n.next===undefined?i+1:l[n.next];if(n.kind==="Const"&&typeof n.value==="string")n.value=l[n.value.slice(1)];}
 return {g,l};
}


function depth(x){let n=0;while(x){n+=x%2;if(x>1)n++;x=Math.floor(x/2);}return n;}
function runFactor(s,policy="evolve",off=false){
 const {g,l}=buildFactor(policy),initial=g.length,d=new Map(s.xy.map((v,i)=>[50+i,v])),v=[0,0,0,0];let pc=0,paid=0,prod=0,peak=g.length+1+d.size,clones=0,kernels=0,serviceKernels=0,serviceClones=0,cloneSteps=0,firstExposure=null,selected=null,expectedEntry=null,probeSteps=0;const outputs=[],activations=[],prefix=[],services=[],patches=[];
 const inputs=()=>[50,51,52,53].map(k=>d.get(k));
 if(![s.xy,...s.changes.map(a=>a.slice(1))].every(a=>a.length===4&&a.every(x=>Number.isInteger(x)&&x>=1&&x<=16)))throw Error("domain");
 while(paid<50000){
  const p=pc,n={...g[p]};if(!n.kind)throw Error("pc");
  if([l.activate_x,l.activate_y].includes(p)&&firstExposure===null)firstExposure=paid+1;
  if(firstExposure===null&&policy==="evolve")prefix.push([p,[...v],[...d],g.map(x=>({...x}))]);
  if(p>=l.probe_x&&p<l.x_gate)probeSteps++;
  if(p===l.entry)selected="normal";
  if(p===l.factor_x)selected="x";if(p===l.factor_y)selected="y";
  if(p===l.kernel_dispatch){kernels++;serviceKernels++;expectedEntry=initial+v[1]-1;}
  if(p>=initial){serviceClones++;cloneSteps++;}
  paid++;pc=n.next;
  switch(n.kind){
   case "Const":v[n.dst]=n.value;break;
   case "Read":v[n.dst]=d.get(v[n.address])??0;break;
   case "Write":{
    const k=v[n.address],z=v[n.value];d.set(k,z);
    if(k===11){
     const [x,y,a,b]=inputs(),expected=x*y+a*b;
     if(z!==expected)throw Error("wrong output "+s.id+" "+prod+" "+selected+" "+z+" "+expected);
     const xActive=g[l.x_gate].next===l.check_x,yActive=g[l.y_gate].next===l.check_y;
     const choice=xActive&&x===a?"x":yActive&&y===b?"y":"normal";
     if(selected!==choice||serviceKernels!==(choice==="normal"?2:1))throw Error("factor selection");
     if(serviceClones!==(policy==="plain"?0:choice==="x"?depth(x):choice==="y"?depth(y):depth(x)+depth(a)))throw Error("path count");
     if(d.get(47)!==0)throw Error("phase");
     prod++;outputs.push(z);services.push({at:prod,selected,paid,serviceKernels,serviceClones,xActive,yActive});serviceKernels=serviceClones=0;
     if(prod===s.n)return {paid,outputs,activations,services,clones,kernels,cloneSteps,nodes:g.length,peak,firstExposure,prefix,probeSteps,patches};
     for(const [at,...q] of s.changes)if(at===prod)for(let i=0;i<4;i++)d.set(50+i,q[i]);
    }break;}
   case "Add":v[n.dst]=v[n.lhs]+v[n.rhs];break;
   case "Sub":v[n.dst]=v[n.lhs]-v[n.rhs];break;
   case "Branch":pc=v[n.test]!==0?n.nonzero:n.zero;break;
   case "CloneNode":{const from=v[n.source],idx=g.length,x=idx-initial+1;if(from!==(x%2?l.parent_add:l.double_src))throw Error("template");g.push({...g[from]});v[n.dst]=idx;clones++;break;}
   case "PatchConst":{const at=v[n.node];if(at!==l.base||g[at].kind!=="Const"||v[n.value]!==initial-1)throw Error("base");g[at].value=v[n.value];break;}
   case "PatchNext":{
    const at=v[n.node],to=v[n.target];if(!g[to]||g[at].next===undefined)throw Error("edge");
    if(at>=initial){const x=at-initial+1;if(to!==(x===1?l.out:x%2?at-1:initial+x/2-1))throw Error("generated edge");}
    else if(at===l.special_entry){if(to!==expectedEntry)throw Error("entry");}
    else if(at===l.kernel_dispatch){if(to!==l.base)throw Error("kernel");}
    else if(at===l.probe_x){if(to!==l.probe_y)throw Error("probe retire");}
    else if(at===l.probe_y){if(to!==l.x_gate)throw Error("probe retire");}
    else if(at===l.x_gate||at===l.y_gate){
     const axis=at===l.x_gate?"x":"y",q=inputs(),match=axis==="x"?q[0]===q[2]:q[1]===q[3];
     if(!match||p!==l["activate_"+axis]||to!==l["check_"+axis]||activations.some(e=>e.axis===axis))throw Error("activation");
     activations.push({axis,at:prod,inputs:q,paid,suppressed:off});
    }else throw Error("patch target");
    const suppress=off&&[l.activate_x,l.activate_y].includes(p);patches.push([paid,p,at,g[at].next,to,suppress]);if(!suppress)g[at].next=to;break;}
   default:throw Error(n.kind);
  }
  peak=Math.max(peak,g.length+1+d.size);if(g.length>200||d.size>16||peak>220)throw Error("capacity "+g.length+" "+peak);
 }
 throw Error("budget");
}
function buildLife(policy="life"){
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
 label("prod_init");set(47,0);set(49,1,"entry");
 label("first");C(3,50);label("first_route");C(0,0,"direct");
 label("second");C(3,52);label("second_route");C(0,0,"direct");
 label("direct");R(1,3);C(0,1);A(3,3,0);R(2,3,"kernel_dispatch");
 label("reverse");C(0,1);A(3,3,0);R(1,3);S(3,3,0);R(2,3,"kernel_dispatch");
 label("kernel_dispatch");C(0,0,"parent");
 label("parent");C(3,1);
 label("parent_add");A(0,0,2);S(1,1,3);B(1,"parent_add","out");
 label("base");C(3,0);A(1,1,3);C(3,"@special_entry");P(3,1);
 label("special_entry");C(0,0,"parent_add");
 label("out");C(3,47);R(1,3);B(1,"sum","save_first");
 label("save_first");C(3,41);W(3,0);set(47,1,"second");
 label("sum");C(3,41);R(1,3);A(0,0,1);C(3,47);C(1,0);W(3,1);
 label("result");C(3,0,"prod_out");
 label("prod_out");C(3,11);W(3,0,"entry");
 label("entry");C(3,49);R(0,3);C(1,1);S(0,0,1);W(3,0);B(0,"x_gate","wake");
 label("wake");set(49,32,"probe_x");
 label("probe_x");C(3,50,"observe_x");
 label("observe_x");R(1,3);C(3,52);R(2,3);S(0,1,2);B(0,"probe_y","install_x");
 label("install_x");set(54,4);pe("probe_x","probe_y");C(1,"@x_gate");C(0,"@check_x");label("activate_x");P(1,0,"probe_y");
 label("probe_y");C(3,51,"observe_y");
 label("observe_y");R(1,3);C(3,53);R(2,3);S(0,1,2);B(0,"x_gate","install_y");
 label("install_y");set(55,4);pe("probe_y","x_gate");C(1,"@y_gate");C(0,"@check_y");label("activate_y");P(1,0,"x_gate");
 label("x_gate");C(3,50,"y_gate");
 label("check_x");R(1,3);C(3,52);R(2,3);S(0,1,2);B(0,policy==="life"?"miss_x":"y_gate","factor_x");
 label("y_gate");C(3,51,"first");
 label("check_y");R(1,3);C(3,53);R(2,3);S(0,1,2);B(0,policy==="life"?"miss_y":"first","factor_y");
 label("miss_x");C(3,54);R(0,3);C(1,1);S(0,0,1);W(3,0);B(0,"y_gate","retire_x");
 label("retire_x");pe("probe_x","observe_x");C(1,"@x_gate");C(0,"@y_gate");label("deactivate_x");P(1,0,"y_gate");
 label("miss_y");C(3,55);R(0,3);C(1,1);S(0,0,1);W(3,0);B(0,"first","retire_y");
 label("retire_y");pe("probe_y","observe_y");C(1,"@y_gate");C(0,"@first");label("deactivate_y");P(1,0,"first");
 label("factor_x");C(3,51);R(2,3);C(3,53);R(0,3);A(2,2,0,"factor_phase");
 label("factor_y");C(3,50);R(2,3);C(3,52);R(0,3);A(2,2,0,"factor_phase");
 // phase 1 plus zero partial routes the single multiplication through existing sum/production tail.
 label("factor_phase");C(3,41);C(0,0);W(3,0);C(3,47);C(0,1);W(3,0,"kernel_dispatch");
 for(let i=0;i<g.length;i++){const n=g[i];if(n.kind==="Branch"){n.nonzero=l[n.nonzero];n.zero=l[n.zero]}else n.next=n.next===undefined?i+1:l[n.next];if(n.kind==="Const"&&typeof n.value==="string")n.value=l[n.value.slice(1)];}
 return {g,l};
}


function depth(x){let n=0;while(x){n+=x%2;if(x>1)n++;x=Math.floor(x/2);}return n;}
function runLife(s,policy="life",off=false){
 const {g,l}=buildLife(policy),initial=g.length,d=new Map(s.xy.map((v,i)=>[50+i,v])),v=[0,0,0,0];let pc=0,paid=0,prod=0,peak=g.length+1+d.size,clones=0,kernels=0,serviceKernels=0,serviceClones=0,cloneSteps=0,firstExposure=null,selected=null,expectedEntry=null,probeSteps=0,wakes=0;const outputs=[],activations=[],deactivations=[],prefix=[],services=[],patches=[];
 const inputs=()=>[50,51,52,53].map(k=>d.get(k));
 if(![s.xy,...s.changes.map(a=>a.slice(1))].every(a=>a.length===4&&a.every(x=>Number.isInteger(x)&&x>=1&&x<=16)))throw Error("domain");
 while(paid<50000){
  const p=pc,n={...g[p]};if(!n.kind)throw Error("pc");
  if([l.activate_x,l.activate_y].includes(p)&&firstExposure===null)firstExposure=paid+1;
  if(firstExposure===null&&["life","sampled"].includes(policy))prefix.push([p,[...v],[...d],g.map(x=>({...x}))]);
  if(p>=l.probe_x&&p<l.x_gate)probeSteps++;
  if(p===l.entry)selected="normal";if(p===l.wake)wakes++;
  if(p===l.factor_x)selected="x";if(p===l.factor_y)selected="y";
  if(p===l.kernel_dispatch){kernels++;serviceKernels++;expectedEntry=initial+v[1]-1;}
  if(p>=initial){serviceClones++;cloneSteps++;}
  paid++;pc=n.next;
  switch(n.kind){
   case "Const":v[n.dst]=n.value;break;
   case "Read":v[n.dst]=d.get(v[n.address])??0;break;
   case "Write":{
    const k=v[n.address],z=v[n.value];d.set(k,z);
    if(k===11){
     const [x,y,a,b]=inputs(),expected=x*y+a*b;
     if(z!==expected)throw Error("wrong output "+s.id+" "+prod+" "+selected+" "+z+" "+expected);
     const xActive=g[l.x_gate].next===l.check_x,yActive=g[l.y_gate].next===l.check_y;
     const choice=xActive&&x===a?"x":yActive&&y===b?"y":"normal";
     if(selected!==choice||serviceKernels!==(choice==="normal"?2:1))throw Error("factor selection");
     if(serviceClones!==(policy==="plain"?0:choice==="x"?depth(x):choice==="y"?depth(y):depth(x)+depth(a)))throw Error("path count");
     if(d.get(47)!==0)throw Error("phase");
     prod++;outputs.push(z);services.push({at:prod,selected,paid,serviceKernels,serviceClones,xActive,yActive});serviceKernels=serviceClones=0;
     if(prod===s.n)return {paid,outputs,activations,services,clones,kernels,cloneSteps,nodes:g.length,peak,firstExposure,prefix,probeSteps,patches,deactivations,wakes};
     for(const [at,...q] of s.changes)if(at===prod)for(let i=0;i<4;i++)d.set(50+i,q[i]);
    }break;}
   case "Add":v[n.dst]=v[n.lhs]+v[n.rhs];break;
   case "Sub":v[n.dst]=v[n.lhs]-v[n.rhs];break;
   case "Branch":pc=v[n.test]!==0?n.nonzero:n.zero;break;
   case "CloneNode":{const from=v[n.source],idx=g.length,x=idx-initial+1;if(from!==(x%2?l.parent_add:l.double_src))throw Error("template");g.push({...g[from]});v[n.dst]=idx;clones++;break;}
   case "PatchConst":{const at=v[n.node];if(at!==l.base||g[at].kind!=="Const"||v[n.value]!==initial-1)throw Error("base");g[at].value=v[n.value];break;}
   case "PatchNext":{
    const at=v[n.node],to=v[n.target];if(!g[to]||g[at].next===undefined)throw Error("edge");
    if(at>=initial){const x=at-initial+1;if(to!==(x===1?l.out:x%2?at-1:initial+x/2-1))throw Error("generated edge");}
    else if(at===l.special_entry){if(to!==expectedEntry)throw Error("entry");}
    else if(at===l.kernel_dispatch){if(to!==l.base)throw Error("kernel");}
    else if(at===l.probe_x){if(![l.probe_y,l.observe_x].includes(to))throw Error("probe change");}
    else if(at===l.probe_y){if(![l.x_gate,l.observe_y].includes(to))throw Error("probe change");}
    else if(at===l.x_gate||at===l.y_gate){
     const axis=at===l.x_gate?"x":"y",q=inputs(),match=axis==="x"?q[0]===q[2]:q[1]===q[3];
     if(p===l["activate_"+axis]){
      if(!match||to!==l["check_"+axis]||g[at].next===to||d.get(axis==="x"?54:55)!==4)throw Error("activation");
      activations.push({axis,at:prod,inputs:q,paid,suppressed:off});
     }else if(p===l["deactivate_"+axis]){
      if(match||d.get(axis==="x"?54:55)!==0||g[at].next!==l["check_"+axis]||to!==(axis==="x"?l.y_gate:l.first))throw Error("deactivation");
      deactivations.push({axis,at:prod,paid});
     }else throw Error("gate change");
    }else throw Error("patch target");
    const suppress=off&&[l.activate_x,l.activate_y].includes(p);patches.push([paid,p,at,g[at].next,to,suppress]);if(!suppress)g[at].next=to;break;}
   default:throw Error(n.kind);
  }
  peak=Math.max(peak,g.length+1+d.size);if(g.length>200||d.size>16||peak>220)throw Error("capacity "+g.length+" "+peak);
 }
 throw Error("budget");
}
const scenarios=[];
const patterns=["x_stable","y_stable","none","both","x_to_y","y_to_x","rare","alternating","mixed"];
for(const n of [16,64,128,256])for(const pattern of patterns){
 const initial={x_stable:[8,11,8,15],y_stable:[13,4,15,4],none:[7,3,11,5],both:[9,13,9,13],x_to_y:[12,5,12,9],y_to_x:[13,4,15,4],rare:[7,3,11,5],alternating:[12,5,12,9],mixed:[7,3,11,5]}[pattern];
 const changes=[];
 for(let i=1;i<n;i++){
  if(pattern==="x_stable"&&i%7===0){let x=1+(i*3)%16;changes.push([i,x,1+(i*5)%16,x,1+(i*7)%16]);}
  if(pattern==="y_stable"&&i%7===0){let y=1+(i*3)%16;changes.push([i,1+(i*5)%16,y,1+(i*7)%16,y]);}
  if(pattern==="both"&&i%7===0){let x=1+(i*3)%16,y=1+(i*5)%16;changes.push([i,x,y,x,y]);}
  if(pattern==="x_to_y"&&i===Math.floor(n/2))changes.push([i,13,4,15,4]);
  if(pattern==="y_to_x"&&i===Math.floor(n/2))changes.push([i,12,5,12,9]);
  if(pattern==="rare"&&i===Math.floor(n/2))changes.push([i,12,5,12,9]);
  if(pattern==="rare"&&i===Math.floor(n/2)+1)changes.push([i,7,3,11,5]);
  if(pattern==="alternating"&&i%8===0)changes.push([i,...(i%16===8?[13,4,15,4]:[12,5,12,9])]);
  if(pattern==="mixed"){let x=1+(i*3)%16,y=1+(i*5)%16,a=1+(i*7+1)%16,b=1+(i*11+3)%16;if(i%5===0)a=x;if(i%7===0)b=y;changes.push([i,x,y,a,b]);}
 }
 scenarios.push({id:pattern+"_"+n,pattern,n,xy:initial,changes});
}

for(const n of [128,256]){
 const xy=[12,5,12,9],changes=[[16,7,3,11,5],[64,12,5,12,9],[96,7,3,11,5],[112,12,5,12,9]];
 if(n===256)changes.push([144,7,3,11,5],[192,12,5,12,9]);
 scenarios.push({id:"return_"+n,pattern:"return",n,xy,changes});
}

const rows=scenarios.map(s=>{
 const arms={plain:runFactor(s,"plain"),shared:runFactor(s,"shared"),both:runFactor(s,"both"),old:runFactor(s),sampled:runLife(s,"sampled"),life:runLife(s),off:runLife(s,"life",true)};
 for(const q of Object.values(arms))if(q.outputs.join()!==arms.plain.outputs.join())throw Error("outputs");
 if(JSON.stringify(arms.life.prefix)!==JSON.stringify(arms.off.prefix)||arms.life.firstExposure!==arms.off.firstExposure)throw Error("prefix");
 if(arms.life.wakes!==Math.ceil(s.n/32)||arms.sampled.wakes!==arms.life.wakes||arms.off.wakes!==arms.life.wakes||arms.off.deactivations.length||arms.sampled.deactivations.length)throw Error("wake counts");
 for(const q of [arms.life,arms.sampled,arms.off])if(q.clones!==16)throw Error("clones");
 for(const e of arms.life.deactivations){const a=arms.life.activations.filter(x=>x.axis===e.axis&&x.at<e.at).at(-1);if(!a)throw Error("retire without activation");}
 const compact=q=>({paid:q.paid,nodes:q.nodes,peak:q.peak,kernels:q.kernels,cloneSteps:q.cloneSteps,activations:q.activations,deactivations:q.deactivations??[],wakes:q.wakes??0,factored:q.services.filter(x=>x.selected!=="normal").length,outputs:q.outputs.length});
 return {id:s.id,pattern:s.pattern,n:s.n,...Object.fromEntries(Object.entries(arms).map(([k,q])=>[k,compact(q)]))};
});
function agg(a){return Object.fromEntries(["shared","both","old","sampled","life","off"].map(k=>[k,{net:a.reduce((s,r)=>s+r.plain.paid-r[k].paid,0),vsShared:a.reduce((s,r)=>s+r.shared.paid-r[k].paid,0),winsShared:a.filter(r=>r.shared.paid>r[k].paid).length,paid:a.reduce((s,r)=>s+r[k].paid,0),peak:Math.max(...a.map(r=>r[k].peak)),nodes:Math.max(...a.map(r=>r[k].nodes)),factored:a.reduce((s,r)=>s+r[k].factored,0),kernels:a.reduce((s,r)=>s+r[k].kernels,0),activations:a.reduce((s,r)=>s+r[k].activations.length,0),deactivations:a.reduce((s,r)=>s+r[k].deactivations.length,0),wakes:a.reduce((s,r)=>s+r[k].wakes,0)}]))}
const summary={all:agg(rows),revealed:agg(rows.filter(r=>r.pattern!=="return")),groups:Object.fromEntries([...patterns,"return"].map(k=>[k,agg(rows.filter(r=>r.pattern===k))])),reactivations:rows.flatMap(r=>r.life.activations.filter(e=>r.life.deactivations.some(d=>d.axis===e.axis&&d.at<e.at)).map(e=>({id:r.id,...e}))),worlds:rows.length*7};

assert.deepEqual(summary,{"all":{"shared":{"net":191875,"vsShared":0,"winsShared":0,"paid":255748,"peak":148,"nodes":140,"factored":0,"kernels":9120,"activations":0,"deactivations":0,"wakes":0},"both":{"net":214555,"vsShared":22680,"winsShared":26,"paid":233068,"peak":148,"nodes":140,"factored":3141,"kernels":5979,"activations":0,"deactivations":0,"wakes":0},"old":{"net":201455,"vsShared":9580,"winsShared":24,"paid":246168,"peak":148,"nodes":140,"factored":3141,"kernels":5979,"activations":60,"deactivations":0,"wakes":0},"sampled":{"net":197766,"vsShared":5891,"winsShared":18,"paid":249857,"peak":192,"nodes":181,"factored":2771,"kernels":6349,"activations":43,"deactivations":0,"wakes":147},"life":{"net":198254,"vsShared":6379,"winsShared":18,"paid":249369,"peak":192,"nodes":181,"factored":2605,"kernels":6515,"activations":58,"deactivations":31,"wakes":147},"off":{"net":157754,"vsShared":-34121,"winsShared":0,"paid":289869,"peak":192,"nodes":181,"factored":0,"kernels":9120,"activations":43,"deactivations":0,"wakes":147}},"revealed":{"shared":{"net":174667,"vsShared":0,"winsShared":0,"paid":234260,"peak":148,"nodes":140,"factored":0,"kernels":8352,"activations":0,"deactivations":0,"wakes":0},"both":{"net":196547,"vsShared":21880,"winsShared":24,"paid":212380,"peak":148,"nodes":140,"factored":2933,"kernels":5419,"activations":0,"deactivations":0,"wakes":0},"old":{"net":185277,"vsShared":10610,"winsShared":24,"paid":223650,"peak":148,"nodes":140,"factored":2933,"kernels":5419,"activations":58,"deactivations":0,"wakes":0},"sampled":{"net":180952,"vsShared":6285,"winsShared":18,"paid":227975,"peak":192,"nodes":181,"factored":2563,"kernels":5789,"activations":41,"deactivations":0,"wakes":135},"life":{"net":181393,"vsShared":6726,"winsShared":18,"paid":227534,"peak":192,"nodes":181,"factored":2429,"kernels":5923,"activations":52,"deactivations":26,"wakes":135},"off":{"net":143388,"vsShared":-31279,"winsShared":0,"paid":265539,"peak":192,"nodes":181,"factored":0,"kernels":8352,"activations":41,"deactivations":0,"wakes":135}},"groups":{"x_stable":{"shared":{"net":15784,"vsShared":0,"winsShared":0,"paid":25430,"peak":148,"nodes":140,"factored":0,"kernels":928,"activations":0,"deactivations":0,"wakes":0},"both":{"net":22043,"vsShared":6259,"winsShared":4,"paid":19171,"peak":148,"nodes":140,"factored":464,"kernels":464,"activations":0,"deactivations":0,"wakes":0},"old":{"net":20118,"vsShared":4334,"winsShared":4,"paid":21096,"peak":148,"nodes":140,"factored":464,"kernels":464,"activations":7,"deactivations":0,"wakes":0},"sampled":{"net":19496,"vsShared":3712,"winsShared":4,"paid":21718,"peak":192,"nodes":181,"factored":464,"kernels":464,"activations":5,"deactivations":0,"wakes":15},"life":{"net":19496,"vsShared":3712,"winsShared":4,"paid":21718,"peak":192,"nodes":181,"factored":464,"kernels":464,"activations":5,"deactivations":0,"wakes":15},"off":{"net":12309,"vsShared":-3475,"winsShared":0,"paid":28905,"peak":192,"nodes":181,"factored":0,"kernels":928,"activations":5,"deactivations":0,"wakes":15}},"y_stable":{"shared":{"net":14937,"vsShared":0,"winsShared":0,"paid":25371,"peak":148,"nodes":140,"factored":0,"kernels":928,"activations":0,"deactivations":0,"wakes":0},"both":{"net":18675,"vsShared":3738,"winsShared":4,"paid":21633,"peak":148,"nodes":140,"factored":464,"kernels":464,"activations":0,"deactivations":0,"wakes":0},"old":{"net":17670,"vsShared":2733,"winsShared":4,"paid":22638,"peak":148,"nodes":140,"factored":464,"kernels":464,"activations":7,"deactivations":0,"wakes":0},"sampled":{"net":18036,"vsShared":3099,"winsShared":4,"paid":22272,"peak":192,"nodes":181,"factored":464,"kernels":464,"activations":5,"deactivations":0,"wakes":15},"life":{"net":18111,"vsShared":3174,"winsShared":4,"paid":22197,"peak":192,"nodes":181,"factored":464,"kernels":464,"activations":5,"deactivations":1,"wakes":15},"off":{"net":11462,"vsShared":-3475,"winsShared":0,"paid":28846,"peak":192,"nodes":181,"factored":0,"kernels":928,"activations":5,"deactivations":0,"wakes":15}},"none":{"shared":{"net":15872,"vsShared":0,"winsShared":0,"paid":26368,"peak":148,"nodes":140,"factored":0,"kernels":928,"activations":0,"deactivations":0,"wakes":0},"both":{"net":10304,"vsShared":-5568,"winsShared":0,"paid":31936,"peak":148,"nodes":140,"factored":0,"kernels":928,"activations":0,"deactivations":0,"wakes":0},"old":{"net":9376,"vsShared":-6496,"winsShared":0,"paid":32864,"peak":148,"nodes":140,"factored":0,"kernels":928,"activations":0,"deactivations":0,"wakes":0},"sampled":{"net":12387,"vsShared":-3485,"winsShared":0,"paid":29853,"peak":190,"nodes":181,"factored":0,"kernels":928,"activations":0,"deactivations":0,"wakes":15},"life":{"net":12387,"vsShared":-3485,"winsShared":0,"paid":29853,"peak":190,"nodes":181,"factored":0,"kernels":928,"activations":0,"deactivations":0,"wakes":15},"off":{"net":12387,"vsShared":-3485,"winsShared":0,"paid":29853,"peak":190,"nodes":181,"factored":0,"kernels":928,"activations":0,"deactivations":0,"wakes":15}},"both":{"shared":{"net":15896,"vsShared":0,"winsShared":0,"paid":25486,"peak":148,"nodes":140,"factored":0,"kernels":928,"activations":0,"deactivations":0,"wakes":0},"both":{"net":22183,"vsShared":6287,"winsShared":4,"paid":19199,"peak":148,"nodes":140,"factored":464,"kernels":464,"activations":0,"deactivations":0,"wakes":0},"old":{"net":21167,"vsShared":5271,"winsShared":4,"paid":20215,"peak":148,"nodes":140,"factored":464,"kernels":464,"activations":8,"deactivations":0,"wakes":0},"sampled":{"net":19664,"vsShared":3768,"winsShared":4,"paid":21718,"peak":192,"nodes":181,"factored":464,"kernels":464,"activations":8,"deactivations":0,"wakes":15},"life":{"net":19664,"vsShared":3768,"winsShared":4,"paid":21718,"peak":192,"nodes":181,"factored":464,"kernels":464,"activations":8,"deactivations":0,"wakes":15},"off":{"net":12449,"vsShared":-3447,"winsShared":0,"paid":28933,"peak":192,"nodes":181,"factored":0,"kernels":928,"activations":8,"deactivations":0,"wakes":15}},"x_to_y":{"shared":{"net":26776,"vsShared":0,"winsShared":0,"paid":26600,"peak":148,"nodes":140,"factored":0,"kernels":928,"activations":0,"deactivations":0,"wakes":0},"both":{"net":33040,"vsShared":6264,"winsShared":4,"paid":20336,"peak":148,"nodes":140,"factored":464,"kernels":464,"activations":0,"deactivations":0,"wakes":0},"old":{"net":30864,"vsShared":4088,"winsShared":4,"paid":22512,"peak":148,"nodes":140,"factored":464,"kernels":464,"activations":8,"deactivations":0,"wakes":0},"sampled":{"net":30335,"vsShared":3559,"winsShared":3,"paid":23041,"peak":192,"nodes":181,"factored":456,"kernels":472,"activations":7,"deactivations":0,"wakes":15},"life":{"net":31275,"vsShared":4499,"winsShared":3,"paid":22101,"peak":192,"nodes":181,"factored":456,"kernels":472,"activations":7,"deactivations":4,"wakes":15},"off":{"net":23303,"vsShared":-3473,"winsShared":0,"paid":30073,"peak":192,"nodes":181,"factored":0,"kernels":928,"activations":7,"deactivations":0,"wakes":15}},"y_to_x":{"shared":{"net":26776,"vsShared":0,"winsShared":0,"paid":26600,"peak":148,"nodes":140,"factored":0,"kernels":928,"activations":0,"deactivations":0,"wakes":0},"both":{"net":33040,"vsShared":6264,"winsShared":4,"paid":20336,"peak":148,"nodes":140,"factored":464,"kernels":464,"activations":0,"deactivations":0,"wakes":0},"old":{"net":32024,"vsShared":5248,"winsShared":4,"paid":21352,"peak":148,"nodes":140,"factored":464,"kernels":464,"activations":8,"deactivations":0,"wakes":0},"sampled":{"net":31487,"vsShared":4711,"winsShared":3,"paid":21889,"peak":192,"nodes":181,"factored":456,"kernels":472,"activations":7,"deactivations":0,"wakes":15},"life":{"net":31477,"vsShared":4701,"winsShared":3,"paid":21899,"peak":192,"nodes":181,"factored":456,"kernels":472,"activations":7,"deactivations":1,"wakes":15},"off":{"net":23303,"vsShared":-3473,"winsShared":0,"paid":30073,"peak":192,"nodes":181,"factored":0,"kernels":928,"activations":7,"deactivations":0,"wakes":15}},"rare":{"shared":{"net":15948,"vsShared":0,"winsShared":0,"paid":26364,"peak":148,"nodes":140,"factored":0,"kernels":928,"activations":0,"deactivations":0,"wakes":0},"both":{"net":10484,"vsShared":-5464,"winsShared":0,"paid":31828,"peak":148,"nodes":140,"factored":4,"kernels":924,"activations":0,"deactivations":0,"wakes":0},"old":{"net":9492,"vsShared":-6456,"winsShared":0,"paid":32820,"peak":148,"nodes":140,"factored":4,"kernels":924,"activations":4,"deactivations":0,"wakes":0},"sampled":{"net":11399,"vsShared":-4549,"winsShared":0,"paid":30913,"peak":191,"nodes":181,"factored":3,"kernels":925,"activations":3,"deactivations":0,"wakes":15},"life":{"net":12334,"vsShared":-3614,"winsShared":0,"paid":29978,"peak":191,"nodes":181,"factored":3,"kernels":925,"activations":3,"deactivations":3,"wakes":15},"off":{"net":12456,"vsShared":-3492,"winsShared":0,"paid":29856,"peak":191,"nodes":181,"factored":0,"kernels":928,"activations":3,"deactivations":0,"wakes":15}},"alternating":{"shared":{"net":26776,"vsShared":0,"winsShared":0,"paid":26600,"peak":148,"nodes":140,"factored":0,"kernels":928,"activations":0,"deactivations":0,"wakes":0},"both":{"net":33040,"vsShared":6264,"winsShared":4,"paid":20336,"peak":148,"nodes":140,"factored":464,"kernels":464,"activations":0,"deactivations":0,"wakes":0},"old":{"net":31864,"vsShared":5088,"winsShared":4,"paid":21512,"peak":148,"nodes":140,"factored":464,"kernels":464,"activations":8,"deactivations":0,"wakes":0},"sampled":{"net":25862,"vsShared":-914,"winsShared":0,"paid":27514,"peak":191,"nodes":181,"factored":232,"kernels":696,"activations":4,"deactivations":0,"wakes":15},"life":{"net":24326,"vsShared":-2450,"winsShared":0,"paid":29050,"peak":191,"nodes":181,"factored":120,"kernels":808,"activations":15,"deactivations":15,"wakes":15},"off":{"net":23310,"vsShared":-3466,"winsShared":0,"paid":30066,"peak":191,"nodes":181,"factored":0,"kernels":928,"activations":4,"deactivations":0,"wakes":15}},"mixed":{"shared":{"net":15902,"vsShared":0,"winsShared":0,"paid":25441,"peak":148,"nodes":140,"factored":0,"kernels":928,"activations":0,"deactivations":0,"wakes":0},"both":{"net":13738,"vsShared":-2164,"winsShared":0,"paid":27605,"peak":148,"nodes":140,"factored":145,"kernels":783,"activations":0,"deactivations":0,"wakes":0},"old":{"net":12702,"vsShared":-3200,"winsShared":0,"paid":28641,"peak":148,"nodes":140,"factored":145,"kernels":783,"activations":8,"deactivations":0,"wakes":0},"sampled":{"net":12286,"vsShared":-3616,"winsShared":0,"paid":29057,"peak":192,"nodes":181,"factored":24,"kernels":904,"activations":2,"deactivations":0,"wakes":15},"life":{"net":12323,"vsShared":-3579,"winsShared":0,"paid":29020,"peak":192,"nodes":181,"factored":2,"kernels":926,"activations":2,"deactivations":2,"wakes":15},"off":{"net":12409,"vsShared":-3493,"winsShared":0,"paid":28934,"peak":192,"nodes":181,"factored":0,"kernels":928,"activations":2,"deactivations":0,"wakes":15}},"return":{"shared":{"net":17208,"vsShared":0,"winsShared":0,"paid":21488,"peak":148,"nodes":140,"factored":0,"kernels":768,"activations":0,"deactivations":0,"wakes":0},"both":{"net":18008,"vsShared":800,"winsShared":2,"paid":20688,"peak":148,"nodes":140,"factored":208,"kernels":560,"activations":0,"deactivations":0,"wakes":0},"old":{"net":16178,"vsShared":-1030,"winsShared":0,"paid":22518,"peak":148,"nodes":140,"factored":208,"kernels":560,"activations":2,"deactivations":0,"wakes":0},"sampled":{"net":16814,"vsShared":-394,"winsShared":0,"paid":21882,"peak":191,"nodes":181,"factored":208,"kernels":560,"activations":2,"deactivations":0,"wakes":12},"life":{"net":16861,"vsShared":-347,"winsShared":0,"paid":21835,"peak":191,"nodes":181,"factored":176,"kernels":592,"activations":6,"deactivations":5,"wakes":12},"off":{"net":14366,"vsShared":-2842,"winsShared":0,"paid":24330,"peak":191,"nodes":181,"factored":0,"kernels":768,"activations":2,"deactivations":0,"wakes":12}}},"reactivations":[{"id":"alternating_64","axis":"x","at":32,"inputs":[12,5,12,9],"paid":2101,"suppressed":false},{"id":"alternating_128","axis":"x","at":32,"inputs":[12,5,12,9],"paid":2101,"suppressed":false},{"id":"alternating_128","axis":"x","at":64,"inputs":[12,5,12,9],"paid":4079,"suppressed":false},{"id":"alternating_128","axis":"x","at":96,"inputs":[12,5,12,9],"paid":6057,"suppressed":false},{"id":"alternating_256","axis":"x","at":32,"inputs":[12,5,12,9],"paid":2101,"suppressed":false},{"id":"alternating_256","axis":"x","at":64,"inputs":[12,5,12,9],"paid":4079,"suppressed":false},{"id":"alternating_256","axis":"x","at":96,"inputs":[12,5,12,9],"paid":6057,"suppressed":false},{"id":"alternating_256","axis":"x","at":128,"inputs":[12,5,12,9],"paid":8035,"suppressed":false},{"id":"alternating_256","axis":"x","at":160,"inputs":[12,5,12,9],"paid":10013,"suppressed":false},{"id":"alternating_256","axis":"x","at":192,"inputs":[12,5,12,9],"paid":11991,"suppressed":false},{"id":"alternating_256","axis":"x","at":224,"inputs":[12,5,12,9],"paid":13969,"suppressed":false},{"id":"return_128","axis":"x","at":64,"inputs":[12,5,12,9],"paid":3972,"suppressed":false},{"id":"return_256","axis":"x","at":64,"inputs":[12,5,12,9],"paid":3972,"suppressed":false},{"id":"return_256","axis":"x","at":128,"inputs":[12,5,12,9],"paid":7528,"suppressed":false},{"id":"return_256","axis":"x","at":192,"inputs":[12,5,12,9],"paid":11377,"suppressed":false}],"worlds":266});
console.log(JSON.stringify({tier:"NON_NATIVE_RULE_LIFECYCLE",summary,rows,scenarios,sources:{life:buildLife(),sampled:buildLife("sampled")}},null,2));
