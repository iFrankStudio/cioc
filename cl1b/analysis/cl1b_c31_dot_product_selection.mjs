#!/usr/bin/env node
// C31-R0: multi-invocation dot-product benchmark, paid recurrent orientation choice.
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
 const plain=run(s,"plain"),direct=run(s,"direct"),reverse=run(s,"reverse"),on=run(s),off=run(s,"adaptive",true);
 for(const q of [direct,reverse,on,off])if(q.outputs.join()!==plain.outputs.join())throw Error("same output");
 if(JSON.stringify(on.prefix)!==JSON.stringify(off.prefix)||on.firstExposure!==off.firstExposure)throw Error("matched prefix");
 if(on.clones!==16||off.clones!==16||direct.clones!==16||reverse.clones!==16||plain.clones!==0)throw Error("clone totals");
 const compact=q=>({paid:q.paid,initial:q.initial,nodes:q.nodes,peak:q.peak,productionClones:q.productionClones,trialClones:q.trialClones,decisions:q.decisions,outputs:q.outputs.length,mutations:q.mutations.filter(x=>x[1]===build("adaptive").l.activation)});
 return {id:s.id,pattern:s.pattern,n:s.n,plain:compact(plain),direct:compact(direct),reverse:compact(reverse),adaptive:compact(on),off:compact(off)};
});
function agg(a){return Object.fromEntries(["direct","reverse","adaptive","off"].map(k=>[k,{net:a.reduce((s,r)=>s+r.plain.paid-r[k].paid,0),wins:a.filter(r=>r.plain.paid>r[k].paid).length,paid:a.reduce((s,r)=>s+r[k].paid,0),peak:Math.max(...a.map(r=>r[k].peak)),nodes:Math.max(...a.map(r=>r[k].nodes))}]))}
const summary={all:agg(rows),groups:Object.fromEntries(["left","right","conflict","flip","alternating","mixed"].map(k=>[k,agg(rows.filter(r=>r.pattern===k))])),reverseSelections:rows.reduce((s,r)=>s+r.adaptive.decisions.filter(x=>x.selected==="reverse").length,0),decisions:rows.reduce((s,r)=>s+r.adaptive.decisions.length,0),changed:rows.filter(r=>new Set(r.adaptive.decisions.map(x=>x.selected)).size>1).map(r=>r.id),worlds:rows.length*5};

assert.deepEqual(summary,{"all":{"direct":{"net":57256,"wins":23,"paid":97058,"peak":165,"nodes":157},"reverse":{"net":53916,"wins":20,"paid":100398,"peak":165,"nodes":157},"adaptive":{"net":26125,"wins":18,"paid":128189,"peak":172,"nodes":157},"off":{"net":23389,"wins":19,"paid":130925,"peak":172,"nodes":157}},"groups":{"left":{"direct":{"net":232,"wins":3,"paid":15084,"peak":165,"nodes":157},"reverse":{"net":-2768,"wins":0,"paid":18084,"peak":165,"nodes":157},"adaptive":{"net":-5648,"wins":0,"paid":20964,"peak":172,"nodes":157},"off":{"net":-5648,"wins":0,"paid":20964,"peak":172,"nodes":157}},"right":{"direct":{"net":19432,"wins":4,"paid":17484,"peak":165,"nodes":157},"reverse":{"net":21232,"wins":4,"paid":15684,"peak":165,"nodes":157},"adaptive":{"net":15637,"wins":4,"paid":21279,"peak":172,"nodes":157},"off":{"net":13837,"wins":4,"paid":23079,"peak":172,"nodes":157}},"conflict":{"direct":{"net":9232,"wins":4,"paid":15984,"peak":165,"nodes":157},"reverse":{"net":8932,"wins":4,"paid":16284,"peak":165,"nodes":157},"adaptive":{"net":4093,"wins":4,"paid":21123,"peak":172,"nodes":157},"off":{"net":4093,"wins":4,"paid":21123,"peak":172,"nodes":157}},"flip":{"direct":{"net":8782,"wins":4,"paid":15984,"peak":165,"nodes":157},"reverse":{"net":8182,"wins":4,"paid":16584,"peak":165,"nodes":157},"adaptive":{"net":3889,"wins":3,"paid":20877,"peak":172,"nodes":157},"off":{"net":2953,"wins":4,"paid":21813,"peak":172,"nodes":157}},"alternating":{"direct":{"net":9704,"wins":4,"paid":16268,"peak":165,"nodes":157},"reverse":{"net":9072,"wins":4,"paid":16900,"peak":165,"nodes":157},"adaptive":{"net":3824,"wins":3,"paid":22148,"peak":172,"nodes":157},"off":{"net":3824,"wins":3,"paid":22148,"peak":172,"nodes":157}},"mixed":{"direct":{"net":9874,"wins":4,"paid":16254,"peak":165,"nodes":157},"reverse":{"net":9266,"wins":4,"paid":16862,"peak":165,"nodes":157},"adaptive":{"net":4330,"wins":4,"paid":21798,"peak":172,"nodes":157},"off":{"net":4330,"wins":4,"paid":21798,"peak":172,"nodes":157}}},"reverseSelections":30,"decisions":114,"changed":["flip_48","flip_96","flip_144"],"worlds":120});
console.log(JSON.stringify({tier:"NON_NATIVE_DOT_PRODUCT_RECURRENT_SELECTION",summary,rows,scenarios,sources:Object.fromEntries(["plain","direct","reverse","adaptive"].map(k=>[k,build(k)]))},null,2));
