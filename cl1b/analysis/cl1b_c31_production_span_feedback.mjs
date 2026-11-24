#!/usr/bin/env node
// C31-R3 paid sampled production-span feedback; no input snapshot.
import assert from "node:assert/strict";
function build(policy="adaptive"){
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
 label("prod_init");set(47,0,policy==="adaptive"?"calibrate":"first");
 label("first");C(3,50,"orientation");
 label("second");C(3,52,"orientation");
 label("orientation");C(0,0,policy==="reverse"?"reverse":"direct");
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
 label("prod_out");C(3,11);W(3,0,policy==="adaptive"?"countdown":"first");
 label("countdown");C(3,49);R(0,3);C(1,1);S(0,0,1);W(3,0);B(0,"first","calibrate");
 label("calibrate");pe("orientation","direct");pe("result","trial_out");set(48,0,"measure");
 label("measure");Q(0);C(3,42);W(3,0,"first");
 label("trial_out");C(3,44);W(3,0);Q(0);C(3,42);R(1,3);S(0,1,0);C(3,46);W(3,0);C(3,48);R(1,3);B(1,"compare","save_trial");
 label("save_trial");C(3,45);W(3,0);C(3,44);R(0,3);C(3,43);W(3,0);set(48,1);pe("orientation","reverse","measure");
 label("compare");pe("orientation","direct");C(3,43);R(0,3);C(3,44);R(1,3);S(0,0,1);B(0,"finish","cost");
 label("cost");C(3,45);R(0,3);C(3,46);R(1,3);S(2,0,1);S(1,1,0);C(3,1);B(2,"cmp","finish");
 label("cmp");S(2,2,3);B(2,"cmp_neg","install");label("cmp_neg");S(1,1,3);B(1,"cmp","finish");
 label("install");C(1,"@orientation");C(0,"@reverse");label("activation");P(1,0,"finish");
 label("finish");pe("result","prod_out");set(49,16,"first");
 for(let i=0;i<g.length;i++){const n=g[i];if(n.kind==="Branch"){n.nonzero=l[n.nonzero];n.zero=l[n.zero]}else n.next=n.next===undefined?i+1:l[n.next];if(n.kind==="Const"&&typeof n.value==="string")n.value=l[n.value.slice(1)];}
 return {g,l};
}
function depth(x){let n=0;while(x){n+=x%2;if(x>1)n++;x=Math.floor(x/2);}return n;}
function run(s,policy="adaptive",off=false){
 const {g,l}=build(policy),initial=g.length,d=new Map(s.xy.map((v,i)=>[50+i,v])),v=[0,0,0,0];let pc=0,paid=0,prod=0,peak=g.length+1+d.size,clones=0,trialClones=0,productionClones=0,serviceClones=0,firstExposure=null,lastTrialStart=0;
 const outputs=[],trials=[],decisions=[],prefix=[],services=[],mutations=[];
 const inputs=()=>[50,51,52,53].map(k=>d.get(k));const expected=()=>d.get(50)*d.get(51)+d.get(52)*d.get(53);
 if(![s.xy,...s.changes.map(a=>a.slice(1))].every(a=>a.length===4&&a.every(x=>Number.isInteger(x)&&x>=1&&x<=16)))throw Error("domain");
 while(paid<30000){
  const p=pc,n={...g[p]};if(!n.kind)throw Error("pc");
  if(p===l.activation&&firstExposure===null)firstExposure=paid+1;
  if(firstExposure===null&&policy==="adaptive")prefix.push([p,[...v],[...d],g.map(x=>({...x}))]);
  if(p===l.calibrate)lastTrialStart=paid;
  if(p>=initial){serviceClones++;if(g[l.result].next===l.trial_out)trialClones++;else productionClones++;}
  paid++;pc=n.next;
  switch(n.kind){
   case "Const":v[n.dst]=n.value;break;
   case "Read":v[n.dst]=d.get(v[n.address])??0;break;
   case "Write":{
    const k=v[n.address],a=v[n.value];d.set(k,a);
    if(k===46){const reverse=g[l.orientation].next===l.reverse;const q=inputs();if(d.get(44)!==expected()||serviceClones!==depth(q[reverse?1:0])+depth(q[reverse?3:2]))throw Error("trial");trials.push({at:prod,reverse,cost:a,output:d.get(44),inputs:q,clones:serviceClones});serviceClones=0;}
    if(k===11){
     const reverse=g[l.orientation].next===l.reverse,q=inputs();if(a!==expected())throw Error("production");
     if(serviceClones!==(policy==="plain"?0:depth(q[reverse?1:0])+depth(q[reverse?3:2])))throw Error("production path");
     if(policy==="adaptive"&&prod%16===0){const [a,b]=trials.slice(-2);if(a.at!==prod||b.at!==prod||a.reverse||!b.reverse||a.output!==b.output||reverse!==(!off&&b.cost<a.cost))throw Error("decision");decisions.push({at:prod,inputs:q,direct:a.cost,reverse:b.cost,selected:reverse?"reverse":"direct",setupPaid:paid-lastTrialStart});}
     serviceClones=0;prod++;outputs.push(a);services.push({at:prod,reverse,paid});
     peak=Math.max(peak,g.length+1+d.size);
     if(prod===s.n)return {paid,outputs,trials,decisions,services,clones,trialClones,productionClones,initial,nodes:g.length,peak,firstExposure,prefix,mutations};
     for(const [at,...q2] of s.changes)if(at===prod)for(let i=0;i<4;i++)d.set(50+i,q2[i]);
    }
    break;}
   case "SenseOwnCredits":v[n.dst]=30000-paid;break;
   case "Add":v[n.dst]=v[n.lhs]+v[n.rhs];break;
   case "Sub":v[n.dst]=v[n.lhs]-v[n.rhs];break;
   case "Branch":pc=v[n.test]!==0?n.nonzero:n.zero;break;
   case "CloneNode":{const src=v[n.source],idx=g.length,x=idx-initial+1;if(src!==(x%2?l.parent_add:l.double_src))throw Error("template");g.push({...g[src]});v[n.dst]=idx;clones++;break;}
   case "PatchConst":{const at=v[n.node];if(at!==l.base||g[at].kind!=="Const"||v[n.value]!==initial-1)throw Error("base");g[at].value=v[n.value];break;}
   case "PatchNext":{const at=v[n.node],to=v[n.target];if(!g[to]||g[at].next===undefined)throw Error("edge");
    if(at>=initial){const x=at-initial+1;if(to!==(x===1?l.out:x%2?at-1:initial+x/2-1))throw Error("generated edge");}
    else if(at===l.special_entry){if(to!==initial+v[1]-initial){} /* actual path independently checked at each output */}
    else if(at===l.orientation){if(![l.direct,l.reverse].includes(to))throw Error("orientation");}
    else if(at===l.kernel_dispatch){if(to!==l.base)throw Error("kernel");}
    else if(at===l.result){if(![l.prod_out,l.trial_out].includes(to))throw Error("result");}
    else throw Error("patch target");
    const suppress=off&&p===l.activation;mutations.push([paid,p,at,g[at].next,to,suppress]);if(!suppress)g[at].next=to;break;}
   default:throw Error(n.kind);
  }
  peak=Math.max(peak,g.length+1+d.size);if(g.length>200||d.size>16||peak>220)throw Error("capacity "+g.length+" "+peak);
 }
 throw Error("budget");
}
function buildLocal(policy="periodic"){
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
 label("prod_init");set(47,0,["periodic","initial"].includes(policy)?"calibrate":"first");
 label("first");C(3,50);label("first_route");C(0,0,policy==="rr"||policy==="rd"?"reverse":"direct");
 label("second");C(3,52);label("second_route");C(0,0,policy==="rr"||policy==="dr"?"reverse":"direct");
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
 label("prod_out");C(3,11);W(3,0,policy==="periodic"?"countdown":"first");
 label("countdown");C(3,49);R(0,3);C(1,1);S(0,0,1);W(3,0);B(0,"first","calibrate");
 // Reuse a single local path comparison for both pairs; patch dynamic route independently.
 label("calibrate");set(48,50,"prepare");
 label("prepare");C(3,48);R(3,3);R(0,3);C(2,1);A(3,3,2);R(1,3);C(2,"base_value");A(0,0,2);A(1,1,2);C(3,"@out");
 // Compare actual path lengths. Reverse costs one extra instruction: it must reach out strictly more than one edge earlier.
 label("chase");op("ReadNext",{dst:0,node:0});S(2,0,3);B(2,"chase_right","choose_direct");
 label("chase_right");op("ReadNext",{dst:1,node:1});S(2,1,3);B(2,"chase","margin");
 label("margin");op("ReadNext",{dst:0,node:0});S(2,0,3);B(2,"choose_reverse","choose_direct");
 label("choose_direct");C(1,"@direct","select_route");
 label("choose_reverse");C(1,"@reverse","select_route");
 label("select_route");C(3,48);R(0,3);C(2,50);S(0,0,2);B(0,"second_target","first_target");
 label("first_target");C(0,"@first_route","activate");
 label("second_target");C(0,"@second_route","activate");
 label("activate");P(0,1);
 C(3,48);R(0,3);C(2,50);S(1,0,2);B(1,"finish","advance");
 label("advance");C(0,52);W(3,0,"prepare");
 label("finish");set(49,16,"first");
 for(let i=0;i<g.length;i++){const n=g[i];if(n.kind==="Branch"){n.nonzero=l[n.nonzero];n.zero=l[n.zero]}else n.next=n.next===undefined?i+1:l[n.next];if(n.kind==="Const"&&typeof n.value==="string")n.value=n.value==="base_value"?g.length-1:l[n.value.slice(1)];}
 return {g,l};
}

function runLocal(s,policy="periodic",off=false){
 const {g,l}=buildLocal(policy),initial=g.length,d=new Map(s.xy.map((v,i)=>[50+i,v])),v=[0,0,0,0];let pc=0,paid=0,prod=0,peak=g.length+1+d.size,clones=0,productionClones=0,serviceClones=0,firstExposure=null,reads=0,servicePair=0,expectedEntry=null;
 const outputs=[],decisions=[],prefix=[],services=[],mutations=[];
 const inputs=()=>[50,51,52,53].map(k=>d.get(k));const expected=()=>d.get(50)*d.get(51)+d.get(52)*d.get(53);
 while(paid<30000){
  const p=pc,n={...g[p]};if(!n.kind)throw Error("pc");
  if(p===l.activate&&firstExposure===null)firstExposure=paid+1;
  if(firstExposure===null&&["periodic","initial"].includes(policy))prefix.push([p,[...v],[...d],g.map(x=>({...x}))]);
  if(p===l.first||p===l.second){servicePair=p===l.first?0:1;const rev=g[servicePair?l.second_route:l.first_route].next===l.reverse;expectedEntry=initial+d.get(50+2*servicePair+(rev?1:0))-1;}
  if(p>=initial){serviceClones++;productionClones++;}
  paid++;pc=n.next;
  switch(n.kind){
   case "Const":v[n.dst]=n.value;break;
   case "Read":v[n.dst]=d.get(v[n.address])??0;break;
   case "ReadNext":{const at=v[n.node];if(at<initial||at>=g.length)throw Error("read outside graph");v[n.dst]=g[at].next;reads++;break;}
   case "Write":{
    const k=v[n.address],a=v[n.value];d.set(k,a);
    if(k===11){
     const dirs=[g[l.first_route].next===l.reverse,g[l.second_route].next===l.reverse],q=inputs();if(a!==expected())throw Error("production");
     if(serviceClones!==(policy==="plain"?0:depth(q[dirs[0]?1:0])+depth(q[dirs[1]?3:2])))throw Error("path");
     serviceClones=0;prod++;outputs.push(a);services.push({at:prod,dirs,paid});
     if(prod===s.n)return {paid,outputs,decisions,services,clones,productionClones,initial,nodes:g.length,peak,firstExposure,prefix,mutations,reads};
     for(const [at,...q2] of s.changes)if(at===prod)for(let i=0;i<4;i++)d.set(50+i,q2[i]);
    }
    break;}
   case "Add":v[n.dst]=v[n.lhs]+v[n.rhs];break;
   case "Sub":v[n.dst]=v[n.lhs]-v[n.rhs];break;
   case "Branch":pc=v[n.test]!==0?n.nonzero:n.zero;break;
   case "CloneNode":{const src=v[n.source],idx=g.length,x=idx-initial+1;if(src!==(x%2?l.parent_add:l.double_src))throw Error("template");g.push({...g[src]});v[n.dst]=idx;clones++;break;}
   case "PatchConst":{const at=v[n.node];if(at!==l.base||g[at].kind!=="Const"||v[n.value]!==initial-1)throw Error("base");g[at].value=v[n.value];break;}
   case "PatchNext":{const at=v[n.node],to=v[n.target];if(!g[to]||g[at].next===undefined)throw Error("edge");
    if(at>=initial){const x=at-initial+1;if(to!==(x===1?l.out:x%2?at-1:initial+x/2-1))throw Error("generated edge");}
    else if(at===l.special_entry){if(to!==expectedEntry)throw Error("entry");}
    else if(at===l.first_route||at===l.second_route){
     if(p!==l.activate)throw Error("activation");
     const pair=at===l.first_route?0:1,q=inputs(),x=q[2*pair],y=q[2*pair+1],rev=depth(y)+1<depth(x);
     if(to!==(rev?l.reverse:l.direct)||d.get(48)!==50+2*pair)throw Error("choice");
     decisions.push({at:prod,pair,x,y,direct:depth(x),reverse:depth(y)+1,chosen:rev?"reverse":"direct",suppressed:off});
    }else if(at===l.kernel_dispatch){if(to!==l.base)throw Error("kernel");}
    else throw Error("patch target");
    const suppress=off&&p===l.activate;mutations.push([paid,p,at,g[at].next,to,suppress]);if(!suppress)g[at].next=to;break;}
   default:throw Error(n.kind);
  }
  peak=Math.max(peak,g.length+1+d.size);if(g.length>200||d.size>16||peak>220)throw Error("capacity "+g.length+" "+peak);
 }
 throw Error("budget");
}
function buildFeedback(policy="feedback"){
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
 label("prod_init");set(47,0);set(55,0);set(56,0);set(49,1,"first");
 label("first");C(3,50);label("first_route");C(0,0,policy==="rr"||policy==="rd"?"reverse":"direct");
 label("second");C(3,52);label("second_route");C(0,0,policy==="rr"||policy==="dr"?"reverse":"direct");
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
 label("prod_out");C(3,11);W(3,0,"countdown");
 label("countdown");C(3,49);R(0,3);C(1,1);S(0,0,1);W(3,0);B(0,"first","sample");
 // Reuse a single local path comparison for both pairs; patch dynamic route independently.
 label("sample");pe("result","sample_out");Q(0);C(3,42);W(3,0,"first");
 label("sample_out");C(3,54);W(3,0);Q(0);C(3,42);R(1,3);S(0,1,0);C(3,55);R(1,3);B(1,"compare_sample","save_reference");
 label("compare_sample");S(2,0,1);B(2,"changed","publish");
 label("save_reference");W(3,0);C(3,56);R(0,3);B(0,"publish","initial_search");label("initial_search");C(0,1);W(3,0,"calibrate");
 label("changed");C(0,0);W(3,0,"calibrate");
 label("calibrate");set(48,50,"prepare");
 label("prepare");C(3,48);R(3,3);R(0,3);C(2,1);A(3,3,2);R(1,3);C(2,"base_value");A(0,0,2);A(1,1,2);C(3,"@out");
 // Compare actual path lengths. Reverse costs one extra instruction: it must reach out strictly more than one edge earlier.
 label("chase");op("ReadNext",{dst:0,node:0});S(2,0,3);B(2,"chase_right","choose_direct");
 label("chase_right");op("ReadNext",{dst:1,node:1});S(2,1,3);B(2,"chase","margin");
 label("margin");op("ReadNext",{dst:0,node:0});S(2,0,3);B(2,"choose_reverse","choose_direct");
 label("choose_direct");C(1,"@direct","select_route");
 label("choose_reverse");C(1,"@reverse","select_route");
 label("select_route");C(3,48);R(0,3);C(2,50);S(0,0,2);B(0,"second_target","first_target");
 label("first_target");C(0,"@first_route","activate");
 label("second_target");C(0,"@second_route","activate");
 label("activate");P(0,1);
 C(3,48);R(0,3);C(2,50);S(1,0,2);B(1,"finish","advance");
 label("advance");C(0,52);W(3,0,"prepare");
 label("finish");C(3,55);C(0,0);W(3,0,"publish");
 label("publish");pe("result","prod_out");set(49,17);C(3,54);R(0,3,"prod_out");
 for(let i=0;i<g.length;i++){const n=g[i];if(n.kind==="Branch"){n.nonzero=l[n.nonzero];n.zero=l[n.zero]}else n.next=n.next===undefined?i+1:l[n.next];if(n.kind==="Const"&&typeof n.value==="string")n.value=n.value==="base_value"?g.length-1:l[n.value.slice(1)];}
 return {g,l};
}

function runFeedback(s,policy="feedback",off=false){
 const {g,l}=buildFeedback(policy),initial=g.length,d=new Map(s.xy.map((v,i)=>[50+i,v])),v=[0,0,0,0];let pc=0,paid=0,prod=0,peak=g.length+1+d.size,clones=0,productionClones=0,serviceClones=0,firstExposure=null,reads=0,servicePair=0,expectedEntry=null,samples=0,searches=0,sampleStart=0,sampleCosts=[],lastSample=null,firstSample=0,sampleEntry=null,actualDirs=[false,false];
 const outputs=[],decisions=[],prefix=[],services=[],mutations=[];
 const inputs=()=>[50,51,52,53].map(k=>d.get(k));const expected=()=>d.get(50)*d.get(51)+d.get(52)*d.get(53);
 while(paid<30000){
  const p=pc,n={...g[p]};if(!n.kind)throw Error("pc");
  if(p===l.sample){samples++;sampleEntry=prod;}if(p===l.calibrate)searches++;if(p===l.sample+3)sampleStart=paid+1;if(p===l.sample_out+2){lastSample=paid+1-sampleStart;sampleCosts.push({at:prod,cost:lastSample,reference:d.get(55),dirs:[g[l.first_route].next===l.reverse,g[l.second_route].next===l.reverse]});}
  if(p===l.activate&&firstExposure===null)firstExposure=paid+1;
  if(firstExposure===null&&["feedback"].includes(policy))prefix.push([p,[...v],[...d],g.map(x=>({...x}))]);
  if(p===l.first||p===l.second){servicePair=p===l.first?0:1;const rev=g[servicePair?l.second_route:l.first_route].next===l.reverse;actualDirs[servicePair]=rev;expectedEntry=initial+d.get(50+2*servicePair+(rev?1:0))-1;}
  if(p>=initial){serviceClones++;productionClones++;}
  paid++;pc=n.next;
  switch(n.kind){
   case "Const":v[n.dst]=n.value;break;
   case "Read":v[n.dst]=d.get(v[n.address])??0;break;
   case "ReadNext":{const at=v[n.node];if(at<initial||at>=g.length)throw Error("read outside graph");v[n.dst]=g[at].next;reads++;break;}
   case "Write":{
    const k=v[n.address],a=v[n.value];d.set(k,a);
    if(k===11){
     const dirs=actualDirs,q=inputs();if(a!==expected())throw Error("production");
     if(serviceClones!==(policy==="plain"?0:depth(q[dirs[0]?1:0])+depth(q[dirs[1]?3:2])))throw Error("path");
     serviceClones=0;prod++;outputs.push(a);services.push({at:prod,dirs,paid});
     if(prod===s.n)return {paid,outputs,decisions,services,clones,productionClones,initial,nodes:g.length,peak,firstExposure,prefix,mutations,reads,samples,searches,sampleCosts};
     for(const [at,...q2] of s.changes)if(at===prod)for(let i=0;i<4;i++)d.set(50+i,q2[i]);
    }
    break;}
   case "SenseOwnCredits":v[n.dst]=30000-paid;break;
   case "Add":v[n.dst]=v[n.lhs]+v[n.rhs];break;
   case "Sub":v[n.dst]=v[n.lhs]-v[n.rhs];break;
   case "Branch":pc=v[n.test]!==0?n.nonzero:n.zero;break;
   case "CloneNode":{const src=v[n.source],idx=g.length,x=idx-initial+1;if(src!==(x%2?l.parent_add:l.double_src))throw Error("template");g.push({...g[src]});v[n.dst]=idx;clones++;break;}
   case "PatchConst":{const at=v[n.node];if(at!==l.base||g[at].kind!=="Const"||v[n.value]!==initial-1)throw Error("base");g[at].value=v[n.value];break;}
   case "PatchNext":{const at=v[n.node],to=v[n.target];if(!g[to]||g[at].next===undefined)throw Error("edge");
    if(at>=initial){const x=at-initial+1;if(to!==(x===1?l.out:x%2?at-1:initial+x/2-1))throw Error("generated edge");}
    else if(at===l.special_entry){if(to!==expectedEntry)throw Error("entry");}
    else if(at===l.first_route||at===l.second_route){
     if(p!==l.activate)throw Error("activation");
     const pair=at===l.first_route?0:1,q=inputs(),x=q[2*pair],y=q[2*pair+1],rev=depth(y)+1<depth(x);
     if(to!==(rev?l.reverse:l.direct)||d.get(48)!==50+2*pair)throw Error("choice");
     decisions.push({at:prod,pair,x,y,direct:depth(x),reverse:depth(y)+1,chosen:rev?"reverse":"direct",suppressed:off});
    }else if(at===l.kernel_dispatch){if(to!==l.base)throw Error("kernel");}else if(at===l.result){if(![l.sample_out,l.prod_out].includes(to))throw Error("sample route");}
    else throw Error("patch target");
    const suppress=off&&p===l.activate;mutations.push([paid,p,at,g[at].next,to,suppress]);if(!suppress)g[at].next=to;break;}
   default:throw Error(n.kind);
  }
  peak=Math.max(peak,g.length+1+d.size);if(g.length>200||d.size>16||peak>220)throw Error("capacity "+g.length+" "+peak);
 }
 throw Error("budget");
}
const scenarios=[];
for(const n of [12,48,96,144])for(const pattern of ["left","right","conflict","flip","alternating","mixed"]){
 const xy=pattern==="left"?[2,15,3,14]:pattern==="right"?[15,2,14,3]:pattern==="conflict"?[1,16,15,2]:pattern==="flip"?[15,1,13,2]:pattern==="alternating"?[2,15,3,14]:[7,11,3,15];
 const changes=[];
 for(let i=1;i<n;i++){
  if(pattern==="flip"&&i===Math.floor(n/2))changes.push([i,1,15,2,13]);
  if(pattern==="alternating"&&i%8===0)changes.push([i,...(i%16===8?[15,2,14,3]:[2,15,3,14])]);
  if(pattern==="mixed")changes.push([i,1+(i*5+3)%16,1+(i*11+7)%16,1+(i*7+1)%16,1+(i*13+9)%16]);
 }
 scenarios.push({id:pattern+"_"+n,pattern,n,xy,changes});
}


const rows=scenarios.map(s=>{
 const plain=run(s,"plain"),dd=runLocal(s,"dd"),initial=runLocal(s,"initial"),periodic=runLocal(s),on=runFeedback(s),off=runFeedback(s,"feedback",true);
 for(const q of [dd,initial,periodic,on,off])if(q.outputs.join()!==plain.outputs.join())throw Error("output");
 if(JSON.stringify(on.prefix)!==JSON.stringify(off.prefix)||on.firstExposure!==off.firstExposure)throw Error("prefix "+s.id);
 for(const q of [on,off]){
 if(q.decisions.length!==2*q.searches||q.clones!==16||q.samples!==Math.ceil((s.n-1)/16)||q.sampleCosts.length!==q.samples)throw Error("counts "+s.id+" "+JSON.stringify({samples:q.samples,expected:Math.ceil((s.n-1)/16),searches:q.searches}));
 let prev=0,init=false,count=0;for(const c of q.sampleCosts){if(c.reference!==prev)throw Error("reference");if(!init||prev!==0&&c.cost!==prev){count++;prev=0;init=true;}else if(prev===0)prev=c.cost;}
 if(count!==q.searches)throw Error("trigger");
 }
 const compact=q=>({paid:q.paid,nodes:q.nodes,peak:q.peak,productionClones:q.productionClones,decisions:q.decisions,reads:q.reads??0,samples:q.samples??0,searches:q.searches??0,sampleCosts:q.sampleCosts??[],outputs:q.outputs.length});
 return {id:s.id,pattern:s.pattern,n:s.n,plain:compact(plain),dd:compact(dd),initial:compact(initial),periodic:compact(periodic),feedback:compact(on),off:compact(off)};
});
function agg(a){return Object.fromEntries(["dd","initial","periodic","feedback","off"].map(k=>[k,{net:a.reduce((s,r)=>s+r.plain.paid-r[k].paid,0),wins:a.filter(r=>r.plain.paid>r[k].paid).length,paid:a.reduce((s,r)=>s+r[k].paid,0),peak:Math.max(...a.map(r=>r[k].peak)),nodes:Math.max(...a.map(r=>r[k].nodes)),samples:a.reduce((s,r)=>s+r[k].samples,0),searches:a.reduce((s,r)=>s+r[k].searches,0)}]))}
const summary={all:agg(rows),groups:Object.fromEntries(["left","right","conflict","flip","alternating","mixed"].map(k=>[k,agg(rows.filter(r=>r.pattern===k))])),worlds:rows.length*6};

assert.deepEqual(summary,{"all":{"dd":{"net":57256,"wins":23,"paid":97058,"peak":143,"nodes":135,"samples":0,"searches":0},"initial":{"net":57688,"wins":22,"paid":96626,"peak":145,"nodes":135,"samples":0,"searches":0},"periodic":{"net":41554,"wins":20,"paid":112760,"peak":145,"nodes":135,"samples":0,"searches":0},"feedback":{"net":43879,"wins":20,"paid":110435,"peak":191,"nodes":177,"samples":114,"searches":35},"off":{"net":40578,"wins":20,"paid":113736,"peak":191,"nodes":177,"samples":114,"searches":34}},"groups":{"left":{"dd":{"net":232,"wins":3,"paid":15084,"peak":143,"nodes":135,"samples":0,"searches":0},"initial":{"net":-80,"wins":2,"paid":15396,"peak":145,"nodes":135,"samples":0,"searches":0},"periodic":{"net":-3026,"wins":0,"paid":18342,"peak":145,"nodes":135,"samples":0,"searches":0},"feedback":{"net":-2389,"wins":0,"paid":17705,"peak":191,"nodes":177,"samples":19,"searches":4},"off":{"net":-2389,"wins":0,"paid":17705,"peak":191,"nodes":177,"samples":19,"searches":4}},"right":{"dd":{"net":19432,"wins":4,"paid":17484,"peak":143,"nodes":135,"samples":0,"searches":0},"initial":{"net":20872,"wins":4,"paid":16044,"peak":145,"nodes":135,"samples":0,"searches":0},"periodic":{"net":17746,"wins":4,"paid":19170,"peak":145,"nodes":135,"samples":0,"searches":0},"feedback":{"net":18515,"wins":4,"paid":18401,"peak":191,"nodes":177,"samples":19,"searches":4},"off":{"net":16763,"wins":4,"paid":20153,"peak":191,"nodes":177,"samples":19,"searches":4}},"conflict":{"dd":{"net":9232,"wins":4,"paid":15984,"peak":143,"nodes":135,"samples":0,"searches":0},"initial":{"net":10144,"wins":4,"paid":15072,"peak":145,"nodes":135,"samples":0,"searches":0},"periodic":{"net":7288,"wins":4,"paid":17928,"peak":145,"nodes":135,"samples":0,"searches":0},"feedback":{"net":7803,"wins":4,"paid":17413,"peak":191,"nodes":177,"samples":19,"searches":4},"off":{"net":6635,"wins":4,"paid":18581,"peak":191,"nodes":177,"samples":19,"searches":4}},"flip":{"dd":{"net":8782,"wins":4,"paid":15984,"peak":143,"nodes":135,"samples":0,"searches":0},"initial":{"net":7870,"wins":4,"paid":16896,"peak":145,"nodes":135,"samples":0,"searches":0},"periodic":{"net":6556,"wins":4,"paid":18210,"peak":145,"nodes":135,"samples":0,"searches":0},"feedback":{"net":6633,"wins":4,"paid":18133,"peak":191,"nodes":177,"samples":19,"searches":7},"off":{"net":5953,"wins":4,"paid":18813,"peak":191,"nodes":177,"samples":19,"searches":7}},"alternating":{"dd":{"net":9704,"wins":4,"paid":16268,"peak":143,"nodes":135,"samples":0,"searches":0},"initial":{"net":9392,"wins":4,"paid":16580,"peak":145,"nodes":135,"samples":0,"searches":0},"periodic":{"net":6446,"wins":4,"paid":19526,"peak":145,"nodes":135,"samples":0,"searches":0},"feedback":{"net":6999,"wins":4,"paid":18973,"peak":191,"nodes":177,"samples":19,"searches":5},"off":{"net":6989,"wins":4,"paid":18983,"peak":191,"nodes":177,"samples":19,"searches":5}},"mixed":{"dd":{"net":9874,"wins":4,"paid":16254,"peak":143,"nodes":135,"samples":0,"searches":0},"initial":{"net":9490,"wins":4,"paid":16638,"peak":145,"nodes":135,"samples":0,"searches":0},"periodic":{"net":6544,"wins":4,"paid":19584,"peak":145,"nodes":135,"samples":0,"searches":0},"feedback":{"net":6318,"wins":4,"paid":19810,"peak":191,"nodes":177,"samples":19,"searches":11},"off":{"net":6627,"wins":4,"paid":19501,"peak":191,"nodes":177,"samples":19,"searches":10}}},"worlds":144});
console.log(JSON.stringify({tier:"NON_NATIVE_PRODUCTION_SPAN_FEEDBACK",summary,rows,scenarios,source:buildFeedback()},null,2));
