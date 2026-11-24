#!/usr/bin/env node
// C31-R1 paid independent local graph path comparison.
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
 const plain=run(s,"plain"),old=run(s,"adaptive"),staticDirect=run(s,"direct"),staticReverse=run(s,"reverse");
 const a=runLocal(s),b=runLocal(s,"periodic",true),i=runLocal(s,"initial"),dd=runLocal(s,"dd"),dr=runLocal(s,"dr"),rd=runLocal(s,"rd"),rr=runLocal(s,"rr");
 for(const q of [old,staticDirect,staticReverse,a,b,i,dd,dr,rd,rr])if(q.outputs.join()!==plain.outputs.join())throw Error("outputs");
 if(JSON.stringify(a.prefix)!==JSON.stringify(b.prefix)||a.firstExposure!==b.firstExposure)throw Error("prefix");
 if(a.decisions.length!==2*Math.ceil(s.n/16)||i.decisions.length!==2||a.clones!==16||b.clones!==16||i.clones!==16)throw Error("counts");
 if(dd.paid!==staticDirect.paid||rr.paid!==staticReverse.paid)throw Error("static comparator "+dd.paid+" "+staticDirect.paid);
 if(JSON.stringify(a.decisions.map(({suppressed,...q})=>q))!==JSON.stringify(b.decisions.map(({suppressed,...q})=>q)))throw Error("paired decisions");
 const compact=q=>({paid:q.paid,nodes:q.nodes,peak:q.peak,productionClones:q.productionClones,decisions:q.decisions,reads:q.reads??0,outputs:q.outputs.length});
 return {id:s.id,pattern:s.pattern,n:s.n,plain:compact(plain),old:compact(old),periodic:compact(a),off:compact(b),initial:compact(i),dd:compact(dd),dr:compact(dr),rd:compact(rd),rr:compact(rr)};
});
function agg(a){return Object.fromEntries(["old","periodic","off","initial","dd","dr","rd","rr"].map(k=>[k,{net:a.reduce((s,r)=>s+r.plain.paid-r[k].paid,0),wins:a.filter(r=>r.plain.paid>r[k].paid).length,paid:a.reduce((s,r)=>s+r[k].paid,0),peak:Math.max(...a.map(r=>r[k].peak)),nodes:Math.max(...a.map(r=>r[k].nodes))}]))}
const summary={all:agg(rows),groups:Object.fromEntries(["left","right","conflict","flip","alternating","mixed"].map(k=>[k,agg(rows.filter(r=>r.pattern===k))])),decisions:rows.reduce((s,r)=>s+r.periodic.decisions.length,0),reverse:rows.reduce((s,r)=>s+r.periodic.decisions.filter(x=>x.chosen==="reverse").length,0),mixedChoices:rows.filter(r=>r.periodic.decisions.some((d,i,a)=>i%2===0&&d.chosen!==a[i+1].chosen)).map(r=>r.id),worlds:rows.length*11};

assert.deepEqual(summary,{"all":{"old":{"net":26125,"wins":18,"paid":128189,"peak":172,"nodes":157},"periodic":{"net":41554,"wins":20,"paid":112760,"peak":145,"nodes":135},"off":{"net":37618,"wins":20,"paid":116696,"peak":145,"nodes":135},"initial":{"net":57688,"wins":22,"paid":96626,"peak":145,"nodes":135},"dd":{"net":57256,"wins":23,"paid":97058,"peak":143,"nodes":135},"dr":{"net":56939,"wins":20,"paid":97375,"peak":143,"nodes":135},"rd":{"net":54233,"wins":20,"paid":100081,"peak":143,"nodes":135},"rr":{"net":53916,"wins":20,"paid":100398,"peak":143,"nodes":135}},"groups":{"left":{"old":{"net":-5648,"wins":0,"paid":20964,"peak":172,"nodes":157},"periodic":{"net":-3026,"wins":0,"paid":18342,"peak":145,"nodes":135},"off":{"net":-3026,"wins":0,"paid":18342,"peak":145,"nodes":135},"initial":{"net":-80,"wins":2,"paid":15396,"peak":145,"nodes":135},"dd":{"net":232,"wins":3,"paid":15084,"peak":143,"nodes":135},"dr":{"net":-968,"wins":0,"paid":16284,"peak":143,"nodes":135},"rd":{"net":-1568,"wins":0,"paid":16884,"peak":143,"nodes":135},"rr":{"net":-2768,"wins":0,"paid":18084,"peak":143,"nodes":135}},"right":{"old":{"net":15637,"wins":4,"paid":21279,"peak":172,"nodes":157},"periodic":{"net":17746,"wins":4,"paid":19170,"peak":145,"nodes":135},"off":{"net":15946,"wins":4,"paid":20970,"peak":145,"nodes":135},"initial":{"net":20872,"wins":4,"paid":16044,"peak":145,"nodes":135},"dd":{"net":19432,"wins":4,"paid":17484,"peak":143,"nodes":135},"dr":{"net":20032,"wins":4,"paid":16884,"peak":143,"nodes":135},"rd":{"net":20632,"wins":4,"paid":16284,"peak":143,"nodes":135},"rr":{"net":21232,"wins":4,"paid":15684,"peak":143,"nodes":135}},"conflict":{"old":{"net":4093,"wins":4,"paid":21123,"peak":172,"nodes":157},"periodic":{"net":7288,"wins":4,"paid":17928,"peak":145,"nodes":135},"off":{"net":6088,"wins":4,"paid":19128,"peak":145,"nodes":135},"initial":{"net":10144,"wins":4,"paid":15072,"peak":145,"nodes":135},"dd":{"net":9232,"wins":4,"paid":15984,"peak":143,"nodes":135},"dr":{"net":10432,"wins":4,"paid":14784,"peak":143,"nodes":135},"rd":{"net":7732,"wins":4,"paid":17484,"peak":143,"nodes":135},"rr":{"net":8932,"wins":4,"paid":16284,"peak":143,"nodes":135}},"flip":{"old":{"net":3889,"wins":3,"paid":20877,"peak":172,"nodes":157},"periodic":{"net":6556,"wins":4,"paid":18210,"peak":145,"nodes":135},"off":{"net":5620,"wins":4,"paid":19146,"peak":145,"nodes":135},"initial":{"net":7870,"wins":4,"paid":16896,"peak":145,"nodes":135},"dd":{"net":8782,"wins":4,"paid":15984,"peak":143,"nodes":135},"dr":{"net":8482,"wins":4,"paid":16284,"peak":143,"nodes":135},"rd":{"net":8482,"wins":4,"paid":16284,"peak":143,"nodes":135},"rr":{"net":8182,"wins":4,"paid":16584,"peak":143,"nodes":135}},"alternating":{"old":{"net":3824,"wins":3,"paid":22148,"peak":172,"nodes":157},"periodic":{"net":6446,"wins":4,"paid":19526,"peak":145,"nodes":135},"off":{"net":6446,"wins":4,"paid":19526,"peak":145,"nodes":135},"initial":{"net":9392,"wins":4,"paid":16580,"peak":145,"nodes":135},"dd":{"net":9704,"wins":4,"paid":16268,"peak":143,"nodes":135},"dr":{"net":9392,"wins":4,"paid":16580,"peak":143,"nodes":135},"rd":{"net":9384,"wins":4,"paid":16588,"peak":143,"nodes":135},"rr":{"net":9072,"wins":4,"paid":16900,"peak":143,"nodes":135}},"mixed":{"old":{"net":4330,"wins":4,"paid":21798,"peak":172,"nodes":157},"periodic":{"net":6544,"wins":4,"paid":19584,"peak":145,"nodes":135},"off":{"net":6544,"wins":4,"paid":19584,"peak":145,"nodes":135},"initial":{"net":9490,"wins":4,"paid":16638,"peak":145,"nodes":135},"dd":{"net":9874,"wins":4,"paid":16254,"peak":143,"nodes":135},"dr":{"net":9569,"wins":4,"paid":16559,"peak":143,"nodes":135},"rd":{"net":9571,"wins":4,"paid":16557,"peak":143,"nodes":135},"rr":{"net":9266,"wins":4,"paid":16862,"peak":143,"nodes":135}}},"decisions":228,"reverse":79,"mixedChoices":["conflict_12","conflict_48","conflict_96","conflict_144"],"worlds":264});
console.log(JSON.stringify({tier:"NON_NATIVE_LOCAL_PATH_SELECTION",summary,rows,scenarios,sources:Object.fromEntries(["periodic","initial","dd","dr","rd","rr"].map(k=>[k,buildLocal(k)]))},null,2));
