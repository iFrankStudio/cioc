#!/usr/bin/env node
// C32-R0: cumulative guarded factorization, shared generated DAG, paid CIL model.
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
const rows=scenarios.map(s=>{
 const arms={plain:runFactor(s,"plain"),shared:runFactor(s,"shared"),x:runFactor(s,"x"),y:runFactor(s,"y"),both:runFactor(s,"both"),evolve:runFactor(s),off:runFactor(s,"evolve",true)};
 for(const q of Object.values(arms))if(q.outputs.join()!==arms.plain.outputs.join())throw Error("outputs");
 if(JSON.stringify(arms.evolve.prefix)!==JSON.stringify(arms.off.prefix)||arms.evolve.firstExposure!==arms.off.firstExposure)throw Error("prefix");
 if(JSON.stringify(arms.evolve.activations.map(({paid,suppressed,...x})=>x))!==JSON.stringify(arms.off.activations.map(({paid,suppressed,...x})=>x)))throw Error("activation mismatch");
 const compact=q=>({paid:q.paid,nodes:q.nodes,peak:q.peak,kernels:q.kernels,cloneSteps:q.cloneSteps,activations:q.activations,probeSteps:q.probeSteps,factored:q.services.filter(x=>x.selected!=="normal").length,outputs:q.outputs.length});
 return {id:s.id,pattern:s.pattern,n:s.n,...Object.fromEntries(Object.entries(arms).map(([k,q])=>[k,compact(q)]))};
});
function agg(a){return Object.fromEntries(["shared","x","y","both","evolve","off"].map(k=>[k,{net:a.reduce((s,r)=>s+r.plain.paid-r[k].paid,0),vsShared:a.reduce((s,r)=>s+r.shared.paid-r[k].paid,0),winsShared:a.filter(r=>r.shared.paid>r[k].paid).length,paid:a.reduce((s,r)=>s+r[k].paid,0),peak:Math.max(...a.map(r=>r[k].peak)),nodes:Math.max(...a.map(r=>r[k].nodes)),factored:a.reduce((s,r)=>s+r[k].factored,0),kernels:a.reduce((s,r)=>s+r[k].kernels,0),activations:a.reduce((s,r)=>s+r[k].activations.length,0),probeSteps:a.reduce((s,r)=>s+r[k].probeSteps,0)}]))}
const summary={all:agg(rows),groups:Object.fromEntries(patterns.map(k=>[k,agg(rows.filter(r=>r.pattern===k))])),twoStage:rows.filter(r=>r.evolve.activations.length===2&&r.evolve.activations[0].at<r.evolve.activations[1].at).map(r=>({id:r.id,activations:r.evolve.activations})),worlds:rows.length*7};

assert.deepEqual(summary,{"all":{"shared":{"net":174667,"vsShared":0,"winsShared":0,"paid":234260,"peak":148,"nodes":140,"factored":0,"kernels":8352,"activations":0,"probeSteps":0},"x":{"net":182009,"vsShared":7342,"winsShared":20,"paid":226918,"peak":148,"nodes":140,"factored":1768,"kernels":6584,"activations":0,"probeSteps":0},"y":{"net":183098,"vsShared":8431,"winsShared":20,"paid":225829,"peak":148,"nodes":140,"factored":1738,"kernels":6614,"activations":0,"probeSteps":0},"both":{"net":196547,"vsShared":21880,"winsShared":24,"paid":212380,"peak":148,"nodes":140,"factored":2933,"kernels":5419,"activations":0,"probeSteps":0},"evolve":{"net":185277,"vsShared":10610,"winsShared":24,"paid":223650,"peak":148,"nodes":140,"factored":2933,"kernels":5419,"activations":58,"probeSteps":21670},"off":{"net":144645,"vsShared":-30022,"winsShared":0,"paid":264282,"peak":148,"nodes":140,"factored":0,"kernels":8352,"activations":58,"probeSteps":21670}},"groups":{"x_stable":{"shared":{"net":15784,"vsShared":0,"winsShared":0,"paid":25430,"peak":148,"nodes":140,"factored":0,"kernels":928,"activations":0,"probeSteps":0},"x":{"net":22043,"vsShared":6259,"winsShared":4,"paid":19171,"peak":148,"nodes":140,"factored":464,"kernels":464,"activations":0,"probeSteps":0},"y":{"net":13432,"vsShared":-2352,"winsShared":0,"paid":27782,"peak":148,"nodes":140,"factored":49,"kernels":879,"activations":0,"probeSteps":0},"both":{"net":22043,"vsShared":6259,"winsShared":4,"paid":19171,"peak":148,"nodes":140,"factored":464,"kernels":464,"activations":0,"probeSteps":0},"evolve":{"net":20118,"vsShared":4334,"winsShared":4,"paid":21096,"peak":148,"nodes":140,"factored":464,"kernels":464,"activations":7,"probeSteps":1925},"off":{"net":12931,"vsShared":-2853,"winsShared":0,"paid":28283,"peak":148,"nodes":140,"factored":0,"kernels":928,"activations":7,"probeSteps":1925}},"y_stable":{"shared":{"net":14937,"vsShared":0,"winsShared":0,"paid":25371,"peak":148,"nodes":140,"factored":0,"kernels":928,"activations":0,"probeSteps":0},"x":{"net":12634,"vsShared":-2303,"winsShared":0,"paid":27674,"peak":148,"nodes":140,"factored":49,"kernels":879,"activations":0,"probeSteps":0},"y":{"net":20701,"vsShared":5764,"winsShared":4,"paid":19607,"peak":148,"nodes":140,"factored":464,"kernels":464,"activations":0,"probeSteps":0},"both":{"net":18675,"vsShared":3738,"winsShared":4,"paid":21633,"peak":148,"nodes":140,"factored":464,"kernels":464,"activations":0,"probeSteps":0},"evolve":{"net":17670,"vsShared":2733,"winsShared":4,"paid":22638,"peak":148,"nodes":140,"factored":464,"kernels":464,"activations":7,"probeSteps":1925},"off":{"net":12084,"vsShared":-2853,"winsShared":0,"paid":28224,"peak":148,"nodes":140,"factored":0,"kernels":928,"activations":7,"probeSteps":1925}},"none":{"shared":{"net":15872,"vsShared":0,"winsShared":0,"paid":26368,"peak":148,"nodes":140,"factored":0,"kernels":928,"activations":0,"probeSteps":0},"x":{"net":12624,"vsShared":-3248,"winsShared":0,"paid":29616,"peak":148,"nodes":140,"factored":0,"kernels":928,"activations":0,"probeSteps":0},"y":{"net":12624,"vsShared":-3248,"winsShared":0,"paid":29616,"peak":148,"nodes":140,"factored":0,"kernels":928,"activations":0,"probeSteps":0},"both":{"net":10304,"vsShared":-5568,"winsShared":0,"paid":31936,"peak":148,"nodes":140,"factored":0,"kernels":928,"activations":0,"probeSteps":0},"evolve":{"net":9376,"vsShared":-6496,"winsShared":0,"paid":32864,"peak":148,"nodes":140,"factored":0,"kernels":928,"activations":0,"probeSteps":5568},"off":{"net":9376,"vsShared":-6496,"winsShared":0,"paid":32864,"peak":148,"nodes":140,"factored":0,"kernels":928,"activations":0,"probeSteps":5568}},"both":{"shared":{"net":15896,"vsShared":0,"winsShared":0,"paid":25486,"peak":148,"nodes":140,"factored":0,"kernels":928,"activations":0,"probeSteps":0},"x":{"net":22183,"vsShared":6287,"winsShared":4,"paid":19199,"peak":148,"nodes":140,"factored":464,"kernels":464,"activations":0,"probeSteps":0},"y":{"net":21737,"vsShared":5841,"winsShared":4,"paid":19645,"peak":148,"nodes":140,"factored":464,"kernels":464,"activations":0,"probeSteps":0},"both":{"net":22183,"vsShared":6287,"winsShared":4,"paid":19199,"peak":148,"nodes":140,"factored":464,"kernels":464,"activations":0,"probeSteps":0},"evolve":{"net":21167,"vsShared":5271,"winsShared":4,"paid":20215,"peak":148,"nodes":140,"factored":464,"kernels":464,"activations":8,"probeSteps":1016},"off":{"net":13952,"vsShared":-1944,"winsShared":0,"paid":27430,"peak":148,"nodes":140,"factored":0,"kernels":928,"activations":8,"probeSteps":1016}},"x_to_y":{"shared":{"net":26776,"vsShared":0,"winsShared":0,"paid":26600,"peak":148,"nodes":140,"factored":0,"kernels":928,"activations":0,"probeSteps":0},"x":{"net":28400,"vsShared":1624,"winsShared":4,"paid":24976,"peak":148,"nodes":140,"factored":232,"kernels":696,"activations":0,"probeSteps":0},"y":{"net":29328,"vsShared":2552,"winsShared":4,"paid":24048,"peak":148,"nodes":140,"factored":232,"kernels":696,"activations":0,"probeSteps":0},"both":{"net":33040,"vsShared":6264,"winsShared":4,"paid":20336,"peak":148,"nodes":140,"factored":464,"kernels":464,"activations":0,"probeSteps":0},"evolve":{"net":30864,"vsShared":4088,"winsShared":4,"paid":22512,"peak":148,"nodes":140,"factored":464,"kernels":464,"activations":8,"probeSteps":2176},"off":{"net":23672,"vsShared":-3104,"winsShared":0,"paid":29704,"peak":148,"nodes":140,"factored":0,"kernels":928,"activations":8,"probeSteps":2176}},"y_to_x":{"shared":{"net":26776,"vsShared":0,"winsShared":0,"paid":26600,"peak":148,"nodes":140,"factored":0,"kernels":928,"activations":0,"probeSteps":0},"x":{"net":28400,"vsShared":1624,"winsShared":4,"paid":24976,"peak":148,"nodes":140,"factored":232,"kernels":696,"activations":0,"probeSteps":0},"y":{"net":29328,"vsShared":2552,"winsShared":4,"paid":24048,"peak":148,"nodes":140,"factored":232,"kernels":696,"activations":0,"probeSteps":0},"both":{"net":33040,"vsShared":6264,"winsShared":4,"paid":20336,"peak":148,"nodes":140,"factored":464,"kernels":464,"activations":0,"probeSteps":0},"evolve":{"net":32024,"vsShared":5248,"winsShared":4,"paid":21352,"peak":148,"nodes":140,"factored":464,"kernels":464,"activations":8,"probeSteps":2176},"off":{"net":23672,"vsShared":-3104,"winsShared":0,"paid":29704,"peak":148,"nodes":140,"factored":0,"kernels":928,"activations":8,"probeSteps":2176}},"rare":{"shared":{"net":15948,"vsShared":0,"winsShared":0,"paid":26364,"peak":148,"nodes":140,"factored":0,"kernels":928,"activations":0,"probeSteps":0},"x":{"net":12784,"vsShared":-3164,"winsShared":0,"paid":29528,"peak":148,"nodes":140,"factored":4,"kernels":924,"activations":0,"probeSteps":0},"y":{"net":12700,"vsShared":-3248,"winsShared":0,"paid":29612,"peak":148,"nodes":140,"factored":0,"kernels":928,"activations":0,"probeSteps":0},"both":{"net":10484,"vsShared":-5464,"winsShared":0,"paid":31828,"peak":148,"nodes":140,"factored":4,"kernels":924,"activations":0,"probeSteps":0},"evolve":{"net":9492,"vsShared":-6456,"winsShared":0,"paid":32820,"peak":148,"nodes":140,"factored":4,"kernels":924,"activations":4,"probeSteps":4452},"off":{"net":10568,"vsShared":-5380,"winsShared":0,"paid":31744,"peak":148,"nodes":140,"factored":0,"kernels":928,"activations":4,"probeSteps":4452}},"alternating":{"shared":{"net":26776,"vsShared":0,"winsShared":0,"paid":26600,"peak":148,"nodes":140,"factored":0,"kernels":928,"activations":0,"probeSteps":0},"x":{"net":28400,"vsShared":1624,"winsShared":4,"paid":24976,"peak":148,"nodes":140,"factored":232,"kernels":696,"activations":0,"probeSteps":0},"y":{"net":29328,"vsShared":2552,"winsShared":4,"paid":24048,"peak":148,"nodes":140,"factored":232,"kernels":696,"activations":0,"probeSteps":0},"both":{"net":33040,"vsShared":6264,"winsShared":4,"paid":20336,"peak":148,"nodes":140,"factored":464,"kernels":464,"activations":0,"probeSteps":0},"evolve":{"net":31864,"vsShared":5088,"winsShared":4,"paid":21512,"peak":148,"nodes":140,"factored":464,"kernels":464,"activations":8,"probeSteps":1176},"off":{"net":24672,"vsShared":-2104,"winsShared":0,"paid":28704,"peak":148,"nodes":140,"factored":0,"kernels":928,"activations":8,"probeSteps":1176}},"mixed":{"shared":{"net":15902,"vsShared":0,"winsShared":0,"paid":25441,"peak":148,"nodes":140,"factored":0,"kernels":928,"activations":0,"probeSteps":0},"x":{"net":14541,"vsShared":-1361,"winsShared":0,"paid":26802,"peak":148,"nodes":140,"factored":91,"kernels":837,"activations":0,"probeSteps":0},"y":{"net":13920,"vsShared":-1982,"winsShared":0,"paid":27423,"peak":148,"nodes":140,"factored":65,"kernels":863,"activations":0,"probeSteps":0},"both":{"net":13738,"vsShared":-2164,"winsShared":0,"paid":27605,"peak":148,"nodes":140,"factored":145,"kernels":783,"activations":0,"probeSteps":0},"evolve":{"net":12702,"vsShared":-3200,"winsShared":0,"paid":28641,"peak":148,"nodes":140,"factored":145,"kernels":783,"activations":8,"probeSteps":1256},"off":{"net":13718,"vsShared":-2184,"winsShared":0,"paid":27625,"peak":148,"nodes":140,"factored":0,"kernels":928,"activations":8,"probeSteps":1256}}},"twoStage":[{"id":"x_to_y_16","activations":[{"axis":"x","at":0,"inputs":[12,5,12,9],"paid":109,"suppressed":false},{"axis":"y","at":8,"inputs":[13,4,15,4],"paid":505,"suppressed":false}]},{"id":"y_to_x_16","activations":[{"axis":"y","at":0,"inputs":[13,4,15,4],"paid":115,"suppressed":false},{"axis":"x","at":8,"inputs":[12,5,12,9],"paid":496,"suppressed":false}]},{"id":"alternating_16","activations":[{"axis":"x","at":0,"inputs":[12,5,12,9],"paid":109,"suppressed":false},{"axis":"y","at":8,"inputs":[13,4,15,4],"paid":505,"suppressed":false}]},{"id":"mixed_16","activations":[{"axis":"x","at":5,"inputs":[16,10,16,11],"paid":455,"suppressed":false},{"axis":"y","at":7,"inputs":[6,4,3,4],"paid":582,"suppressed":false}]},{"id":"x_stable_64","activations":[{"axis":"x","at":0,"inputs":[8,11,8,15],"paid":109,"suppressed":false},{"axis":"y","at":56,"inputs":[9,9,9,9],"paid":2795,"suppressed":false}]},{"id":"y_stable_64","activations":[{"axis":"y","at":0,"inputs":[13,4,15,4],"paid":115,"suppressed":false},{"axis":"x","at":56,"inputs":[9,9,9,9],"paid":2843,"suppressed":false}]},{"id":"x_to_y_64","activations":[{"axis":"x","at":0,"inputs":[12,5,12,9],"paid":109,"suppressed":false},{"axis":"y","at":32,"inputs":[13,4,15,4],"paid":1657,"suppressed":false}]},{"id":"y_to_x_64","activations":[{"axis":"y","at":0,"inputs":[13,4,15,4],"paid":115,"suppressed":false},{"axis":"x","at":32,"inputs":[12,5,12,9],"paid":1624,"suppressed":false}]},{"id":"alternating_64","activations":[{"axis":"x","at":0,"inputs":[12,5,12,9],"paid":109,"suppressed":false},{"axis":"y","at":8,"inputs":[13,4,15,4],"paid":505,"suppressed":false}]},{"id":"mixed_64","activations":[{"axis":"x","at":5,"inputs":[16,10,16,11],"paid":455,"suppressed":false},{"axis":"y","at":7,"inputs":[6,4,3,4],"paid":582,"suppressed":false}]},{"id":"x_stable_128","activations":[{"axis":"x","at":0,"inputs":[8,11,8,15],"paid":109,"suppressed":false},{"axis":"y","at":56,"inputs":[9,9,9,9],"paid":2795,"suppressed":false}]},{"id":"y_stable_128","activations":[{"axis":"y","at":0,"inputs":[13,4,15,4],"paid":115,"suppressed":false},{"axis":"x","at":56,"inputs":[9,9,9,9],"paid":2843,"suppressed":false}]},{"id":"x_to_y_128","activations":[{"axis":"x","at":0,"inputs":[12,5,12,9],"paid":109,"suppressed":false},{"axis":"y","at":64,"inputs":[13,4,15,4],"paid":3193,"suppressed":false}]},{"id":"y_to_x_128","activations":[{"axis":"y","at":0,"inputs":[13,4,15,4],"paid":115,"suppressed":false},{"axis":"x","at":64,"inputs":[12,5,12,9],"paid":3128,"suppressed":false}]},{"id":"alternating_128","activations":[{"axis":"x","at":0,"inputs":[12,5,12,9],"paid":109,"suppressed":false},{"axis":"y","at":8,"inputs":[13,4,15,4],"paid":505,"suppressed":false}]},{"id":"mixed_128","activations":[{"axis":"x","at":5,"inputs":[16,10,16,11],"paid":455,"suppressed":false},{"axis":"y","at":7,"inputs":[6,4,3,4],"paid":582,"suppressed":false}]},{"id":"x_stable_256","activations":[{"axis":"x","at":0,"inputs":[8,11,8,15],"paid":109,"suppressed":false},{"axis":"y","at":56,"inputs":[9,9,9,9],"paid":2795,"suppressed":false}]},{"id":"y_stable_256","activations":[{"axis":"y","at":0,"inputs":[13,4,15,4],"paid":115,"suppressed":false},{"axis":"x","at":56,"inputs":[9,9,9,9],"paid":2843,"suppressed":false}]},{"id":"x_to_y_256","activations":[{"axis":"x","at":0,"inputs":[12,5,12,9],"paid":109,"suppressed":false},{"axis":"y","at":128,"inputs":[13,4,15,4],"paid":6265,"suppressed":false}]},{"id":"y_to_x_256","activations":[{"axis":"y","at":0,"inputs":[13,4,15,4],"paid":115,"suppressed":false},{"axis":"x","at":128,"inputs":[12,5,12,9],"paid":6136,"suppressed":false}]},{"id":"alternating_256","activations":[{"axis":"x","at":0,"inputs":[12,5,12,9],"paid":109,"suppressed":false},{"axis":"y","at":8,"inputs":[13,4,15,4],"paid":505,"suppressed":false}]},{"id":"mixed_256","activations":[{"axis":"x","at":5,"inputs":[16,10,16,11],"paid":455,"suppressed":false},{"axis":"y","at":7,"inputs":[6,4,3,4],"paid":582,"suppressed":false}]}],"worlds":252});
console.log(JSON.stringify({tier:"NON_NATIVE_CUMULATIVE_FACTORIZATION",summary,rows,scenarios,sources:Object.fromEntries(["plain","shared","x","y","both","evolve"].map(k=>[k,buildFactor(k)]))},null,2));
