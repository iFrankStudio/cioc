#!/usr/bin/env node
// C33-R0: within-batch cross-task common-subexpression reuse; paid CIL model.
import assert from "node:assert/strict";

function buildReuse(policy="evolve"){
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
 const call=(name,addr,ret)=>{label(name);pe("out",ret);C(3,addr);R(1,3);C(0,1);A(3,3,0);R(2,3,"kernel_dispatch");};
 label("batch");C(0,0,"call1");
 call("call1",50,"save1");
 label("save1");C(3,41);W(3,0,"call2");
 call("call2",52,"save2");
 label("save2");C(3,41);R(1,3);A(0,0,1);C(3,11);W(3,0,"decision");
 label("decision");C(0,0,policy==="evolve"?"probe_d":policy==="plain"||policy==="shared"?"call3":"gate_d");
 // Compare pair 1 and pair 3; direct and commuted relations, separately installed.
 const equal=(name,a,b,c,d,yes,no)=>{label(name);C(3,a);R(0,3);C(3,b);R(1,3);S(0,0,1);B(0,no,name+"_second");label(name+"_second");C(3,c);R(0,3);C(3,d);R(1,3);S(0,0,1);B(0,no,yes);};
 label("probe_d");C(0,0,"observe_d");
 equal("observe_d",50,54,51,55,"install_d","probe_s");
 label("install_d");pe("probe_d","probe_s");C(1,"@gate_d");C(0,"@check_d");label("activate_d");P(1,0,"probe_s");
 label("probe_s");C(0,0,"observe_s");
 equal("observe_s",50,55,51,54,"install_s","gate_d");
 label("install_s");pe("probe_s","gate_d");C(1,"@gate_s");C(0,"@check_s");label("activate_s");P(1,0,"gate_d");
 label("gate_d");C(0,0,["direct","both"].includes(policy)?"check_d":"gate_s");
 equal("check_d",50,54,51,55,"reuse_d","gate_s");
 label("gate_s");C(0,0,["swap","both"].includes(policy)?"check_s":"call3");
 equal("check_s",50,55,51,54,"reuse_s","call3");
 label("reuse_d");C(0,0,"call4");
 label("reuse_s");C(0,0,"call4");
 call("call3",54,"save3");
 label("save3");C(3,41);W(3,0,"call4");
 call("call4",56,"save4");
 label("save4");C(3,41);R(1,3);A(0,0,1);C(3,12);W(3,0,"batch");
 label("kernel_dispatch");C(0,0,"parent");
 label("parent");C(3,1);
 label("parent_add");A(0,0,2);S(1,1,3);B(1,"parent_add","out");
 label("base");C(3,0);A(1,1,3);C(3,"@special_entry");P(3,1);
 label("special_entry");C(0,0,"parent_add");
 label("out");C(3,0,"save1");
 for(let i=0;i<g.length;i++){const n=g[i];if(n.kind==="Branch"){n.nonzero=l[n.nonzero];n.zero=l[n.zero]}else n.next=n.next===undefined?i+1:l[n.next];if(n.kind==="Const"&&typeof n.value==="string")n.value=l[n.value.slice(1)];}
 return {g,l};
}
function depth(x){let n=0;while(x){n+=x%2;if(x>1)n++;x=Math.floor(x/2);}return n;}
function runReuse(s,policy="evolve",off=false){
 const {g,l}=buildReuse(policy),initial=g.length,d=new Map(s.xy.map((v,i)=>[50+i,v])),v=[0,0,0,0];let pc=0,paid=0,prod=0,peak=g.length+1+d.size,clones=0,kernels=0,serviceKernels=0,serviceClones=0,cloneSteps=0,firstExposure=null,selected="none",expectedEntry=null,probeSteps=0,firstOutput=null,callIndex=0;
 const outputs=[],activations=[],prefix=[],services=[],patches=[];
 const inputs=()=>Array.from({length:8},(_,i)=>d.get(50+i));
 if(![s.xy,...s.changes.map(a=>a.slice(1))].every(a=>a.length===8&&a.every(x=>Number.isInteger(x)&&x>=1&&x<=16)))throw Error("domain");
 while(paid<100000){
  const p=pc,n={...g[p]};if(!n.kind)throw Error("pc");
  if([l.activate_d,l.activate_s].includes(p)&&firstExposure===null)firstExposure=paid+1;
  if(firstExposure===null&&policy==="evolve"&&prod===0)prefix.push([p,[...v],[...d],g.map(x=>({...x}))]);
  if(p>=l.probe_d&&p<l.gate_d)probeSteps++;
  if(p===l.batch){selected="none";firstOutput=null;callIndex=0;}
  if(p===l.reuse_d)selected="direct";if(p===l.reuse_s)selected="swap";
  if(p===l.kernel_dispatch){kernels++;serviceKernels++;expectedEntry=initial+v[1]-1;const q=inputs();const i=callIndex===2&&selected!=="none"?3:callIndex; if(v[1]!==q[2*i]||v[2]!==q[2*i+1])throw Error("kernel inputs");callIndex=i+1;}
  if(p>=initial){serviceClones++;cloneSteps++;}
  paid++;pc=n.next;
  switch(n.kind){
   case "Const":v[n.dst]=n.value;break;
   case "Read":v[n.dst]=d.get(v[n.address])??0;break;
   case "Write":{
    const k=v[n.address],z=v[n.value];d.set(k,z);
    const [a,b,c,e,f,h,i,j]=inputs();
    if(k===11){if(z!==a*b+c*e||firstOutput!==null||serviceKernels!==2)throw Error("first output");firstOutput=z;}
    if(k===12){
     if(z!==f*h+i*j||firstOutput!==a*b+c*e)throw Error("second output");
     const direct=g[l.gate_d].next===l.check_d,swap=g[l.gate_s].next===l.check_s;
     const choice=direct&&a===f&&b===h?"direct":swap&&a===h&&b===f?"swap":"none";
     if(selected!==choice||serviceKernels!==(choice==="none"?4:3)||callIndex!==4)throw Error("reuse selection");
     if(serviceClones!==(policy==="plain"?0:depth(a)+depth(c)+depth(i)+(choice==="none"?depth(f):0)))throw Error("path count");
     prod++;outputs.push([firstOutput,z]);services.push({at:prod,selected,paid,serviceKernels,serviceClones,direct,swap});serviceKernels=serviceClones=0;
     if(prod===s.n)return {paid,outputs,activations,services,clones,kernels,cloneSteps,nodes:g.length,peak,firstExposure,prefix,probeSteps,patches};
     for(const [at,...q] of s.changes)if(at===prod)for(let i=0;i<8;i++)d.set(50+i,q[i]);
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
    else if(at===l.out){if(![l.save1,l.save2,l.save3,l.save4].includes(to))throw Error("continuation");}
    else if(at===l.probe_d){if(to!==l.probe_s)throw Error("probe retire");}
    else if(at===l.probe_s){if(to!==l.gate_d)throw Error("probe retire");}
    else if(at===l.gate_d||at===l.gate_s){
     const axis=at===l.gate_d?"d":"s",[a,b,,,f,h]=inputs(),match=axis==="d"?a===f&&b===h:a===h&&b===f;
     if(!match||p!==l["activate_"+axis]||to!==l["check_"+axis]||activations.some(e=>e.axis===axis))throw Error("activation");
     activations.push({axis,at:prod,paid,suppressed:off});
    }else throw Error("patch target");
    const suppress=off&&[l.activate_d,l.activate_s].includes(p);patches.push([paid,p,at,g[at].next,to,suppress]);if(!suppress)g[at].next=to;break;}
   default:throw Error(n.kind);
  }
  peak=Math.max(peak,g.length+1+d.size);if(g.length>200||d.size>16||peak>220)throw Error("capacity "+g.length+" "+peak);
 }
 throw Error("budget");
}
const scenarios=[];
const patterns=["direct","swap","none","both","d_to_s","s_to_d","rare","alternating","mixed"];
function tuple(pattern,i,n){const a=1+(i*3+10)%16,b=1+(i*5+3)%16,c=1+(i*7+6)%16,e=1+(i*11+1)%16,j=1+(i*13+2)%16,k=1+(i*9+9)%16;let f=a%16+1,h=b%16+1;
 const mode=pattern==="d_to_s"?(i<n/2?"direct":"swap"):pattern==="s_to_d"?(i<n/2?"swap":"direct"):pattern==="rare"?(i===Math.floor(n/2)?"direct":"none"):pattern==="alternating"?(Math.floor(i/8)%2?"swap":"direct"):pattern==="mixed"?(i%5===0?"direct":i%7===0?"swap":"none"):pattern;
 if(mode==="both")return [a,a,c,e,a,a,j,k];
 if(mode==="direct"){f=a;h=b;}if(mode==="swap"){f=b;h=a;}
 // ensure none really neither relation, avoid accidental commuted equality
 if(mode==="none"&&f===b&&h===a)h=h%16+1;
 return [a,b,c,e,f,h,j,k];}
for(const n of [16,64,128,256])for(const pattern of patterns){const xy=tuple(pattern,0,n),changes=Array.from({length:n-1},(_,i)=>[i+1,...tuple(pattern,i+1,n)]);scenarios.push({id:pattern+"_"+n,pattern,n,xy,changes});}
const rows=scenarios.map(s=>{
 const arms={plain:runReuse(s,"plain"),shared:runReuse(s,"shared"),direct:runReuse(s,"direct"),swap:runReuse(s,"swap"),both:runReuse(s,"both"),evolve:runReuse(s),off:runReuse(s,"evolve",true)};
 for(const q of Object.values(arms))if(JSON.stringify(q.outputs)!==JSON.stringify(arms.plain.outputs))throw Error("outputs");
 if(JSON.stringify(arms.evolve.prefix)!==JSON.stringify(arms.off.prefix)||arms.evolve.firstExposure!==arms.off.firstExposure)throw Error("prefix");
 if(JSON.stringify(arms.evolve.activations.map(({paid,suppressed,...x})=>x))!==JSON.stringify(arms.off.activations.map(({paid,suppressed,...x})=>x)))throw Error("activation mismatch");
 for(const [k,q] of Object.entries(arms))if(q.clones!==(k==="plain"?0:16))throw Error("clone count");
 const compact=q=>({paid:q.paid,nodes:q.nodes,peak:q.peak,kernels:q.kernels,cloneSteps:q.cloneSteps,activations:q.activations,probeSteps:q.probeSteps,reused:q.services.filter(x=>x.selected!=="none").length,outputs:q.outputs.length*2});
 return {id:s.id,pattern:s.pattern,n:s.n,...Object.fromEntries(Object.entries(arms).map(([k,q])=>[k,compact(q)]))};
});
function agg(a){return Object.fromEntries(["shared","direct","swap","both","evolve","off"].map(k=>[k,{net:a.reduce((s,r)=>s+r.plain.paid-r[k].paid,0),vsShared:a.reduce((s,r)=>s+r.shared.paid-r[k].paid,0),winsShared:a.filter(r=>r.shared.paid>r[k].paid).length,paid:a.reduce((s,r)=>s+r[k].paid,0),peak:Math.max(...a.map(r=>r[k].peak)),nodes:Math.max(...a.map(r=>r[k].nodes)),reused:a.reduce((s,r)=>s+r[k].reused,0),kernels:a.reduce((s,r)=>s+r[k].kernels,0),activations:a.reduce((s,r)=>s+r[k].activations.length,0),probeSteps:a.reduce((s,r)=>s+r[k].probeSteps,0)}]))}
const summary={all:agg(rows),groups:Object.fromEntries(patterns.map(k=>[k,agg(rows.filter(r=>r.pattern===k))])),twoStage:rows.filter(r=>r.evolve.activations.length===2&&r.evolve.activations[0].at!==r.evolve.activations[1].at).map(r=>r.id),worlds:rows.length*7};

assert.deepEqual(summary,{"all":{"shared":{"net":283119,"vsShared":0,"winsShared":0,"paid":395004,"peak":182,"nodes":170,"reused":0,"kernels":16704,"activations":0,"probeSteps":0},"direct":{"net":276354,"vsShared":-6765,"winsShared":8,"paid":401769,"peak":182,"nodes":170,"reused":1723,"kernels":14981,"activations":0,"probeSteps":0},"swap":{"net":273041,"vsShared":-10078,"winsShared":8,"paid":405082,"peak":182,"nodes":170,"reused":1678,"kernels":15026,"activations":0,"probeSteps":0},"both":{"net":278267,"vsShared":-4852,"winsShared":24,"paid":399856,"peak":182,"nodes":170,"reused":2937,"kernels":13767,"activations":0,"probeSteps":0},"evolve":{"net":264539,"vsShared":-18580,"winsShared":10,"paid":413584,"peak":182,"nodes":170,"reused":2937,"kernels":13767,"activations":52,"probeSteps":28464},"off":{"net":246303,"vsShared":-36816,"winsShared":0,"paid":431820,"peak":182,"nodes":170,"reused":0,"kernels":16704,"activations":52,"probeSteps":28464}},"groups":{"direct":{"shared":{"net":31300,"vsShared":0,"winsShared":0,"paid":43876,"peak":182,"nodes":170,"reused":0,"kernels":1856,"activations":0,"probeSteps":0},"direct":{"net":34751,"vsShared":3451,"winsShared":4,"paid":40425,"peak":182,"nodes":170,"reused":464,"kernels":1392,"activations":0,"probeSteps":0},"swap":{"net":27588,"vsShared":-3712,"winsShared":0,"paid":47588,"peak":182,"nodes":170,"reused":0,"kernels":1856,"activations":0,"probeSteps":0},"both":{"net":34751,"vsShared":3451,"winsShared":4,"paid":40425,"peak":182,"nodes":170,"reused":464,"kernels":1392,"activations":0,"probeSteps":0},"evolve":{"net":30967,"vsShared":-333,"winsShared":0,"paid":44209,"peak":182,"nodes":170,"reused":464,"kernels":1392,"activations":4,"probeSteps":3784},"off":{"net":26588,"vsShared":-4712,"winsShared":0,"paid":48588,"peak":182,"nodes":170,"reused":0,"kernels":1856,"activations":4,"probeSteps":3784}},"swap":{"shared":{"net":31300,"vsShared":0,"winsShared":0,"paid":43876,"peak":182,"nodes":170,"reused":0,"kernels":1856,"activations":0,"probeSteps":0},"direct":{"net":27588,"vsShared":-3712,"winsShared":0,"paid":47588,"peak":182,"nodes":170,"reused":0,"kernels":1856,"activations":0,"probeSteps":0},"swap":{"net":34287,"vsShared":2987,"winsShared":4,"paid":40889,"peak":182,"nodes":170,"reused":464,"kernels":1392,"activations":0,"probeSteps":0},"both":{"net":31503,"vsShared":203,"winsShared":4,"paid":43673,"peak":182,"nodes":170,"reused":464,"kernels":1392,"activations":0,"probeSteps":0},"evolve":{"net":30503,"vsShared":-797,"winsShared":0,"paid":44673,"peak":182,"nodes":170,"reused":464,"kernels":1392,"activations":4,"probeSteps":3784},"off":{"net":26588,"vsShared":-4712,"winsShared":0,"paid":48588,"peak":182,"nodes":170,"reused":0,"kernels":1856,"activations":4,"probeSteps":3784}},"none":{"shared":{"net":31300,"vsShared":0,"winsShared":0,"paid":43876,"peak":182,"nodes":170,"reused":0,"kernels":1856,"activations":0,"probeSteps":0},"direct":{"net":27588,"vsShared":-3712,"winsShared":0,"paid":47588,"peak":182,"nodes":170,"reused":0,"kernels":1856,"activations":0,"probeSteps":0},"swap":{"net":27240,"vsShared":-4060,"winsShared":0,"paid":47936,"peak":182,"nodes":170,"reused":0,"kernels":1856,"activations":0,"probeSteps":0},"both":{"net":24456,"vsShared":-6844,"winsShared":0,"paid":50720,"peak":182,"nodes":170,"reused":0,"kernels":1856,"activations":0,"probeSteps":0},"evolve":{"net":23528,"vsShared":-7772,"winsShared":0,"paid":51648,"peak":182,"nodes":170,"reused":0,"kernels":1856,"activations":0,"probeSteps":6844},"off":{"net":23528,"vsShared":-7772,"winsShared":0,"paid":51648,"peak":182,"nodes":170,"reused":0,"kernels":1856,"activations":0,"probeSteps":6844}},"both":{"shared":{"net":31300,"vsShared":0,"winsShared":0,"paid":43876,"peak":182,"nodes":170,"reused":0,"kernels":1856,"activations":0,"probeSteps":0},"direct":{"net":34751,"vsShared":3451,"winsShared":4,"paid":40425,"peak":182,"nodes":170,"reused":464,"kernels":1392,"activations":0,"probeSteps":0},"swap":{"net":34287,"vsShared":2987,"winsShared":4,"paid":40889,"peak":182,"nodes":170,"reused":464,"kernels":1392,"activations":0,"probeSteps":0},"both":{"net":34751,"vsShared":3451,"winsShared":4,"paid":40425,"peak":182,"nodes":170,"reused":464,"kernels":1392,"activations":0,"probeSteps":0},"evolve":{"net":33679,"vsShared":2379,"winsShared":4,"paid":41497,"peak":182,"nodes":170,"reused":464,"kernels":1392,"activations":8,"probeSteps":1072},"off":{"net":29300,"vsShared":-2000,"winsShared":0,"paid":45876,"peak":182,"nodes":170,"reused":0,"kernels":1856,"activations":8,"probeSteps":1072}},"d_to_s":{"shared":{"net":31345,"vsShared":0,"winsShared":0,"paid":43879,"peak":182,"nodes":170,"reused":0,"kernels":1856,"activations":0,"probeSteps":0},"direct":{"net":31216,"vsShared":-129,"winsShared":0,"paid":44008,"peak":182,"nodes":170,"reused":232,"kernels":1624,"activations":0,"probeSteps":0},"swap":{"net":30984,"vsShared":-361,"winsShared":0,"paid":44240,"peak":182,"nodes":170,"reused":232,"kernels":1624,"activations":0,"probeSteps":0},"both":{"net":33175,"vsShared":1830,"winsShared":4,"paid":42049,"peak":182,"nodes":170,"reused":464,"kernels":1392,"activations":0,"probeSteps":0},"evolve":{"net":30711,"vsShared":-634,"winsShared":0,"paid":44513,"peak":182,"nodes":170,"reused":464,"kernels":1392,"activations":8,"probeSteps":2464},"off":{"net":27953,"vsShared":-3392,"winsShared":0,"paid":47271,"peak":182,"nodes":170,"reused":0,"kernels":1856,"activations":8,"probeSteps":2464}},"s_to_d":{"shared":{"net":31255,"vsShared":0,"winsShared":0,"paid":43873,"peak":182,"nodes":170,"reused":0,"kernels":1856,"activations":0,"probeSteps":0},"direct":{"net":31123,"vsShared":-132,"winsShared":0,"paid":44005,"peak":182,"nodes":170,"reused":232,"kernels":1624,"activations":0,"probeSteps":0},"swap":{"net":30891,"vsShared":-364,"winsShared":0,"paid":44237,"peak":182,"nodes":170,"reused":232,"kernels":1624,"activations":0,"probeSteps":0},"both":{"net":33079,"vsShared":1824,"winsShared":4,"paid":42049,"peak":182,"nodes":170,"reused":464,"kernels":1392,"activations":0,"probeSteps":0},"evolve":{"net":32007,"vsShared":752,"winsShared":3,"paid":43121,"peak":182,"nodes":170,"reused":464,"kernels":1392,"activations":8,"probeSteps":2464},"off":{"net":27863,"vsShared":-3392,"winsShared":0,"paid":47265,"peak":182,"nodes":170,"reused":0,"kernels":1856,"activations":8,"probeSteps":2464}},"rare":{"shared":{"net":31285,"vsShared":0,"winsShared":0,"paid":43879,"peak":182,"nodes":170,"reused":0,"kernels":1856,"activations":0,"probeSteps":0},"direct":{"net":27638,"vsShared":-3647,"winsShared":0,"paid":47526,"peak":182,"nodes":170,"reused":4,"kernels":1852,"activations":0,"probeSteps":0},"swap":{"net":27225,"vsShared":-4060,"winsShared":0,"paid":47939,"peak":182,"nodes":170,"reused":0,"kernels":1856,"activations":0,"probeSteps":0},"both":{"net":24530,"vsShared":-6755,"winsShared":0,"paid":50634,"peak":182,"nodes":170,"reused":4,"kernels":1852,"activations":0,"probeSteps":0},"evolve":{"net":23506,"vsShared":-7779,"winsShared":0,"paid":51658,"peak":182,"nodes":170,"reused":4,"kernels":1852,"activations":4,"probeSteps":5524},"off":{"net":24833,"vsShared":-6452,"winsShared":0,"paid":50331,"peak":182,"nodes":170,"reused":0,"kernels":1856,"activations":4,"probeSteps":5524}},"alternating":{"shared":{"net":32605,"vsShared":0,"winsShared":0,"paid":43963,"peak":182,"nodes":170,"reused":0,"kernels":1856,"activations":0,"probeSteps":0},"direct":{"net":32518,"vsShared":-87,"winsShared":0,"paid":44050,"peak":182,"nodes":170,"reused":232,"kernels":1624,"activations":0,"probeSteps":0},"swap":{"net":32286,"vsShared":-319,"winsShared":0,"paid":44282,"peak":182,"nodes":170,"reused":232,"kernels":1624,"activations":0,"probeSteps":0},"both":{"net":34519,"vsShared":1914,"winsShared":4,"paid":42049,"peak":182,"nodes":170,"reused":464,"kernels":1392,"activations":0,"probeSteps":0},"evolve":{"net":33255,"vsShared":650,"winsShared":3,"paid":43313,"peak":182,"nodes":170,"reused":464,"kernels":1392,"activations":8,"probeSteps":1264},"off":{"net":30413,"vsShared":-2192,"winsShared":0,"paid":46155,"peak":182,"nodes":170,"reused":0,"kernels":1856,"activations":8,"probeSteps":1264}},"mixed":{"shared":{"net":31429,"vsShared":0,"winsShared":0,"paid":43906,"peak":182,"nodes":170,"reused":0,"kernels":1856,"activations":0,"probeSteps":0},"direct":{"net":29181,"vsShared":-2248,"winsShared":0,"paid":46154,"peak":182,"nodes":170,"reused":95,"kernels":1761,"activations":0,"probeSteps":0},"swap":{"net":28253,"vsShared":-3176,"winsShared":0,"paid":47082,"peak":182,"nodes":170,"reused":54,"kernels":1802,"activations":0,"probeSteps":0},"both":{"net":27503,"vsShared":-3926,"winsShared":0,"paid":47832,"peak":182,"nodes":170,"reused":149,"kernels":1707,"activations":0,"probeSteps":0},"evolve":{"net":26383,"vsShared":-5046,"winsShared":0,"paid":48952,"peak":182,"nodes":170,"reused":149,"kernels":1707,"activations":8,"probeSteps":1264},"off":{"net":29237,"vsShared":-2192,"winsShared":0,"paid":46098,"peak":182,"nodes":170,"reused":0,"kernels":1856,"activations":8,"probeSteps":1264}}},"twoStage":["d_to_s_16","s_to_d_16","alternating_16","mixed_16","d_to_s_64","s_to_d_64","alternating_64","mixed_64","d_to_s_128","s_to_d_128","alternating_128","mixed_128","d_to_s_256","s_to_d_256","alternating_256","mixed_256"],"worlds":252});
console.log(JSON.stringify({tier:"NON_NATIVE_CROSS_TASK_REUSE",summary,rows,scenarios,sources:Object.fromEntries(["plain","shared","direct","swap","both","evolve"].map(k=>[k,buildReuse(k)]))},null,2));
