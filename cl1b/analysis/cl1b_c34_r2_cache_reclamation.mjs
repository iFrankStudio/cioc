#!/usr/bin/env node
// C34-R2 actual paid Delete and one-shot shrink, non-native model.
import assert from "node:assert/strict";
function buildShrink(policy="shrink"){
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




 label("prod_init");set(60,0);if(["two","shrink"].includes(policy))set(64,0);C(0,0,"entry");
 label("entry");C(0,0,["shared","plain"].includes(policy)?"compute":"lookup1");
 const compare=(name,slot,yes,no)=>{label(name);for(const [i,key] of [[0,50],[1,51],[2,58]]){C(3,slot+i);R(0,3);C(3,key);R(1,3);S(0,0,1);B(0,no,i===2?yes:name+"_"+(i+1));if(i<2)label(name+"_"+(i+1));}};
 compare("lookup1",60,"hit1","miss_route");
 label("miss_route");C(0,0,["two","shrink"].includes(policy)?"lookup2":"compute");
 compare("lookup2",64,"second_match","miss_budget");
 label("second_match");C(0,0,"hit2");
 label("hit1");C(3,63);R(0,3,"publish");
 label("hit2");C(3,67);R(0,3,"publish");
 label("miss_budget");C(0,0,policy==="shrink"?"budget":"compute");
 label("budget");C(0,8);C(1,1);S(0,0,1);C(1,"@budget");K(1,0);B(0,"compute","retire");
 label("retire");pe("miss_budget","compute");C(1,"@flag");C(0,1);label("activate");K(1,0,"flag");
 label("flag");C(0,0);B(0,"release","compute");
 label("release");pe("miss_route","compute");pe("after_store","reload");C(1,"@store_slot");C(0,63);K(1,0);
 for(const key of [64,65,66,67]){C(3,key);op("Delete",{address:3,next:key===67?"compute":undefined});}
 label("compute");C(3,50);R(1,3);C(3,51);R(2,3);C(3,43);W(3,1);C(3,44);W(3,2);C(3,41);W(3,2);C(3,58);R(0,3);C(3,42);W(3,0,"loop");
 label("loop");C(3,43);R(1,3);C(3,41);R(2,3,"kernel_dispatch");
 label("kernel_dispatch");C(0,0,"parent");
 label("parent");C(3,1);
 label("parent_add");A(0,0,2);S(1,1,3);B(1,"parent_add","out");
 label("base");C(3,0);A(1,1,3);C(3,"@special_entry");P(3,1);
 label("special_entry");C(0,0,"parent_add");
 label("out");C(3,44);R(1,3);A(0,0,1);C(3,41);W(3,0);C(3,42);R(1,3);C(2,1);S(1,1,2);W(3,1);B(1,"loop","done");
 label("done");C(3,41);R(0,3,["plain","shared"].includes(policy)?"publish":"store_route");
 label("store_route");C(0,0,"store_slot");
 label("store_slot");C(2,63);C(1,1);for(const [i,key] of [[3,41],[2,58],[1,44],[0,43]]){C(3,key);R(0,3);W(2,0);if(i)S(2,2,1);}
 label("after_store");C(0,0,["two","shrink"].includes(policy)?"rotate":"reload");
 label("rotate");C(1,127);S(0,1,2);C(1,"@store_slot");K(1,0,"reload");
 label("reload");C(3,41);R(0,3,"publish");
 label("publish");C(3,11);W(3,0,"entry");
 for(let i=0;i<g.length;i++){const n=g[i];if(n.kind==="Branch"){n.nonzero=l[n.nonzero];n.zero=l[n.zero]}else n.next=n.next===undefined?i+1:l[n.next];if(n.kind==="Const"&&typeof n.value==="string")n.value=l[n.value.slice(1)];}
 return {g,l};
}
function depth(x){return x.toString(2).length-1+x.toString(2).replace(/0/g,"").length;}
function rec(a,b,k){let z=b;for(let i=0;i<k;i++)z=a*z+b;return z;}
function runShrink(s,policy="shrink",off=false){
 const {g,l}=buildShrink(policy),initial=g.length,d=new Map([[50,s.requests[0][0]],[51,s.requests[0][1]],[58,s.requests[0][2]]]),v=[0,0,0,0];let pc=0,paid=0,prod=0,peak=g.length+1+d.size,clones=0,kernels=0,sk=0,sc=0,entry=null,hit=null,stores=0,evictions=0,activation=null,exposure=null,deleted=0,misses=0,reclaimedAt=null,cellTime=0;const outputs=[],services=[],prefix=[],records=[null,null];
 const input=()=>[d.get(50),d.get(51),d.get(58)];
 while(paid<200000){
  const p=pc,n={...g[p]};if(!n.kind)throw Error("pc");
  if(p===l.activate&&exposure===null)exposure=paid+1;
  if(exposure===null&&policy==="shrink")prefix.push([p,[...v],[...d],g.length,n]);
  if(p===l.entry)hit=null;if(p===l.budget)misses++;if(p===l.hit1||p===l.hit2)hit=p===l.hit1?0:1;
  if(p===l.kernel_dispatch){const [a,b,k]=input();if(v[1]!==a||v[2]!==rec(a,b,sk)||sk>=k)throw Error("kernel inputs");entry=initial+a-1;sk++;kernels++;}
  if(p>=initial)sc++;
  paid++;pc=n.next;
  switch(n.kind){
   case "Const":v[n.dst]=n.value;break;
   case "Read":v[n.dst]=d.get(v[n.address])??0;break;
   case "Write":{const key=v[n.address],z=v[n.value];d.set(key,z);
    if(p>=l.store_slot&&p<l.after_store&&[60,64].includes(key)){const slot=key===60?0:1,q=input();if([0,1,2].some(i=>d.get(key+i)!==q[i])||d.get(key+3)!==rec(...q)||sk!==q[2])throw Error("store");if(records[slot])evictions++;records[slot]=[...q,rec(...q)];stores++;}
    if(key===11){const q=input();if(z!==rec(...q))throw Error("output "+s.id+" "+prod);const reused=sk===0;if(reused){if(hit===null||!records[hit]||records[hit].slice(0,3).join()!==q.join()||records[hit][3]!==z)throw Error("stale");}else if(sk!==q[2])throw Error("kernel count");if(sc!==(policy==="plain"?0:sk*depth(q[0])))throw Error("clone count");
     prod++;cellTime+=d.size;outputs.push(z);services.push({at:prod,paid,reused,hit,slots:records.filter(Boolean).length,cells:d.size});sk=sc=0;if(prod===s.requests.length)return {paid,outputs,services,clones,kernels,stores,evictions,peak,nodes:g.length,activation,exposure,prefix,deleted,misses,reclaimedAt,cellTime};
     const q2=s.requests[prod];[50,51,58].forEach((k,i)=>d.set(k,q2[i]));}break;}
   case "Delete":{const key=v[n.address];if(!activation||off||key<64||key>67)throw Error("delete");if(d.delete(key))deleted++;if(key===67){records[1]=null;reclaimedAt=prod;if([64,65,66,67].some(k=>d.has(k)))throw Error("not reclaimed");}break;}
   case "Add":v[n.dst]=v[n.lhs]+v[n.rhs];break;
   case "Sub":v[n.dst]=v[n.lhs]-v[n.rhs];break;
   case "Branch":pc=v[n.test]!==0?n.nonzero:n.zero;break;
   case "CloneNode":{const from=v[n.source],idx=g.length,x=idx-initial+1;if(from!==(x%2?l.parent_add:l.double_src))throw Error("template");g.push({...g[from]});v[n.dst]=idx;clones++;break;}
   case "PatchConst":{const at=v[n.node],to=v[n.value];if(at===l.flag){if(p!==l.activate||to!==1||activation||misses!==8||g[l.budget].value!==0)throw Error("shrink evidence");activation={at:prod,paid,request:input(),occupiedBefore:g.length+1+d.size,slot2Present:[64,65,66,67].filter(k=>d.has(k)).length};if(!off)g[at].value=to;}else if(at===l.budget){if(to!==8-misses||to!==g[at].value-1||to<0)throw Error("budget");g[at].value=to;}else if(at===l.store_slot){if(![63,67].includes(to)||!["two","shrink"].includes(policy))throw Error("rotation");if(!activation||off||p!==l.release+8){if(to===g[at].value)throw Error("rotate no change");}g[at].value=to;}else{if(at!==l.base||to!==initial-1)throw Error("base");g[at].value=to;}break;}
   case "PatchNext":{const at=v[n.node],to=v[n.target];if(!g[to]||g[at].next===undefined)throw Error("edge");if(at>=initial){const x=at-initial+1;if(to!==(x===1?l.out:x%2?at-1:initial+x/2-1))throw Error("generated");}else if(at===l.special_entry){if(to!==entry)throw Error("entry");}else if(at===l.kernel_dispatch){if(to!==l.base)throw Error("kernel");}else if(at===l.miss_budget){if(to!==l.compute||misses!==8||activation)throw Error("retire monitoring");}else if(at===l.after_store){if(off||to!==l.reload||!activation)throw Error("stop rotation");}else if(at===l.miss_route){if(off||to!==l.compute||!activation)throw Error("disable");}else throw Error("patch");g[at].next=to;break;}
   default:throw Error(n.kind);
  }
  peak=Math.max(peak,g.length+1+d.size);if(g.length>200||d.size>16||peak>220)throw Error("capacity "+g.length+" "+peak+" "+d.size);
 }
 throw Error("budget");
}
const scenarios=[];
const patterns=["stable","alternating","cycle3","burst","drift","rare","k_change","mixed"];
for(const k of [1,2,4,8])for(const n of [24,96])for(const pattern of patterns){
 const A=[7,3,k],B=[11,5,k],C=[13,2,k];
 const requests=Array.from({length:n},(_,i)=>{
  if(pattern==="stable")return A;
  if(pattern==="alternating")return i%2?B:A;
  if(pattern==="cycle3")return [A,B,C][i%3];
  if(pattern==="burst")return [A,B,C][Math.floor(i/8)%3];
  if(pattern==="drift")return [1+(i*3)%16,1+(i*5+Math.floor(i/16))%16,k];
  if(pattern==="rare")return i===Math.floor(n/2)?[1+((i-1)*3)%16,1+((i-1)*5+Math.floor((i-1)/16))%16,k]:[1+(i*3)%16,1+(i*5+Math.floor(i/16))%16,k];
  if(pattern==="k_change")return [7,3,[1,2,4,8][Math.floor(i/4)%4]];
  return i%5===0?A:i%7===0?B:[1+(i*3)%16,1+(i*5+Math.floor(i/16))%16,k];
 });scenarios.push({id:k+"_"+n+"_"+pattern,k,n,pattern,requests});
}


for(const k of [1,2,4,8])for(const n of [48,96])for(const pattern of ["alternating_to_drift","alternating_to_stable","return","stable_to_alternating"]){
 const A=[7,3,k],B=[11,5,k];const requests=Array.from({length:n},(_,i)=>{
 const alt=i%2?B:A,drift=[1+(i*3)%16,1+(i*5+Math.floor(i/16))%16,k];
 return pattern==="alternating_to_drift"?(i<n/3?alt:drift):pattern==="alternating_to_stable"?(i<n/3?alt:A):pattern==="return"?(i<n/3||i>=2*n/3?alt:drift):i<n/3?A:alt;
 });scenarios.push({id:"shift_"+k+"_"+n+"_"+pattern,k,n,pattern,requests});}
const rows=scenarios.map(s=>{
 const arms={shared:runShrink(s,"shared"),one:runShrink(s,"one"),two:runShrink(s,"two"),shrink:runShrink(s),off:runShrink(s,"shrink",true)};
 for(const q of Object.values(arms))if(q.outputs.join()!==arms.shared.outputs.join())throw Error("outputs");
 if(JSON.stringify(arms.shrink.prefix)!==JSON.stringify(arms.off.prefix)||JSON.stringify(arms.shrink.activation)!==JSON.stringify(arms.off.activation)||arms.shrink.exposure!==arms.off.exposure)throw Error("prefix");
 if(arms.off.deleted||arms.off.reclaimedAt!==null||arms.off.kernels!==arms.two.kernels)throw Error("off");
 if(arms.shrink.activation){if(arms.shrink.deleted!==arms.shrink.activation.slot2Present||arms.shrink.deleted!==4||arms.shrink.reclaimedAt!==arms.shrink.activation.at||arms.shrink.services.slice(arms.shrink.reclaimedAt).some(x=>x.slots>1||x.cells>12))throw Error("reclamation");}
 const compact=q=>({paid:q.paid,kernels:q.kernels,nodes:q.nodes,peak:q.peak,stores:q.stores,evictions:q.evictions,reused:q.services.filter(x=>x.reused).length,activation:q.activation,deleted:q.deleted,misses:q.misses,reclaimedAt:q.reclaimedAt,cellTime:q.cellTime,finalCells:q.services.at(-1).cells});
 return {id:s.id,k:s.k,n:s.n,pattern:s.pattern,...Object.fromEntries(Object.entries(arms).map(([k,q])=>[k,compact(q)]))};
});
function agg(a){return Object.fromEntries(["one","two","shrink","off"].map(k=>[k,{net:a.reduce((s,r)=>s+r.shared.paid-r[k].paid,0),vsTwo:a.reduce((s,r)=>s+r.two.paid-r[k].paid,0),wins:a.filter(r=>r.shared.paid>r[k].paid).length,paid:a.reduce((s,r)=>s+r[k].paid,0),peak:Math.max(...a.map(r=>r[k].peak)),nodes:Math.max(...a.map(r=>r[k].nodes)),reused:a.reduce((s,r)=>s+r[k].reused,0),kernels:a.reduce((s,r)=>s+r[k].kernels,0),retirements:a.filter(r=>r[k].activation).length,deleted:a.reduce((s,r)=>s+r[k].deleted,0),misses:a.reduce((s,r)=>s+r[k].misses,0),cellTime:a.reduce((s,r)=>s+r[k].cellTime,0)}]))}
const summary={all:agg(rows),original:agg(rows.filter(r=>patterns.includes(r.pattern))),shifts:agg(rows.filter(r=>!patterns.includes(r.pattern))),patterns:Object.fromEntries([...patterns,"alternating_to_drift","alternating_to_stable","return","stable_to_alternating"].map(k=>[k,agg(rows.filter(r=>r.pattern===k))])),worlds:rows.length*5};

assert.deepEqual(summary,{"all":{"one":{"net":48246,"vsTwo":-146952,"wins":36,"paid":676470,"peak":204,"nodes":191,"reused":1828,"kernels":16167,"retirements":0,"deleted":0,"misses":0,"cellTime":73728},"two":{"net":195198,"vsTwo":0,"wins":58,"paid":529518,"peak":211,"nodes":194,"reused":3372,"kernels":10377,"retirements":0,"deleted":0,"misses":0,"cellTime":95784},"shrink":{"net":198060,"vsTwo":2862,"wins":56,"paid":526656,"peak":211,"nodes":194,"reused":3196,"kernels":11037,"retirements":56,"deleted":224,"misses":540,"cellTime":84520},"off":{"net":191510,"vsTwo":-3688,"wins":57,"paid":533206,"peak":211,"nodes":194,"reused":3372,"kernels":10377,"retirements":56,"deleted":0,"misses":540,"cellTime":95784}},"original":{"one":{"net":43746,"vsTwo":-25322,"wins":24,"paid":408505,"peak":204,"nodes":191,"reused":1260,"kernels":9657,"retirements":0,"deleted":0,"misses":0,"cellTime":46080},"two":{"net":69068,"vsTwo":0,"wins":32,"paid":383183,"peak":211,"nodes":194,"reused":1724,"kernels":7917,"retirements":0,"deleted":0,"misses":0,"cellTime":59592},"shrink":{"net":88780,"vsTwo":19712,"wins":32,"paid":363471,"peak":211,"nodes":194,"reused":1724,"kernels":7917,"retirements":40,"deleted":160,"misses":380,"cellTime":51080},"off":{"net":66468,"vsTwo":-2600,"wins":32,"paid":385783,"peak":211,"nodes":194,"reused":1724,"kernels":7917,"retirements":40,"deleted":0,"misses":380,"cellTime":59592}},"shifts":{"one":{"net":4500,"vsTwo":-121630,"wins":12,"paid":267965,"peak":204,"nodes":191,"reused":568,"kernels":6510,"retirements":0,"deleted":0,"misses":0,"cellTime":27648},"two":{"net":126130,"vsTwo":0,"wins":26,"paid":146335,"peak":211,"nodes":194,"reused":1648,"kernels":2460,"retirements":0,"deleted":0,"misses":0,"cellTime":36192},"shrink":{"net":109280,"vsTwo":-16850,"wins":24,"paid":163185,"peak":211,"nodes":194,"reused":1472,"kernels":3120,"retirements":16,"deleted":64,"misses":160,"cellTime":33440},"off":{"net":125042,"vsTwo":-1088,"wins":25,"paid":147423,"peak":211,"nodes":194,"reused":1648,"kernels":2460,"retirements":16,"deleted":0,"misses":160,"cellTime":36192}},"patterns":{"stable":{"one":{"net":43908,"vsTwo":112,"wins":8,"paid":12788,"peak":204,"nodes":191,"reused":472,"kernels":30,"retirements":0,"deleted":0,"misses":0,"cellTime":5760},"two":{"net":43796,"vsTwo":0,"wins":8,"paid":12900,"peak":208,"nodes":194,"reused":472,"kernels":30,"retirements":0,"deleted":0,"misses":0,"cellTime":6240},"shrink":{"net":43748,"vsTwo":-48,"wins":8,"paid":12948,"peak":208,"nodes":194,"reused":472,"kernels":30,"retirements":0,"deleted":0,"misses":8,"cellTime":6240},"off":{"net":43748,"vsTwo":-48,"wins":8,"paid":12948,"peak":208,"nodes":194,"reused":472,"kernels":30,"retirements":0,"deleted":0,"misses":8,"cellTime":6240}},"alternating":{"one":{"net":-13440,"vsTwo":-55190,"wins":0,"paid":71036,"peak":204,"nodes":191,"reused":0,"kernels":1800,"retirements":0,"deleted":0,"misses":0,"cellTime":5760},"two":{"net":41750,"vsTwo":0,"wins":8,"paid":15846,"peak":211,"nodes":194,"reused":464,"kernels":60,"retirements":0,"deleted":0,"misses":0,"cellTime":7656},"shrink":{"net":41654,"vsTwo":-96,"wins":8,"paid":15942,"peak":211,"nodes":194,"reused":464,"kernels":60,"retirements":0,"deleted":0,"misses":16,"cellTime":7656},"off":{"net":41654,"vsTwo":-96,"wins":8,"paid":15942,"peak":211,"nodes":194,"reused":464,"kernels":60,"retirements":0,"deleted":0,"misses":16,"cellTime":7656}},"cycle3":{"one":{"net":-13440,"vsTwo":5304,"wins":0,"paid":71336,"peak":204,"nodes":191,"reused":0,"kernels":1800,"retirements":0,"deleted":0,"misses":0,"cellTime":5760},"two":{"net":-18744,"vsTwo":0,"wins":0,"paid":76640,"peak":211,"nodes":194,"reused":0,"kernels":1800,"retirements":0,"deleted":0,"misses":0,"cellTime":7656},"shrink":{"net":-14720,"vsTwo":4024,"wins":0,"paid":72616,"peak":211,"nodes":194,"reused":0,"kernels":1800,"retirements":8,"deleted":32,"misses":64,"cellTime":5960},"off":{"net":-19192,"vsTwo":-448,"wins":0,"paid":77088,"peak":211,"nodes":194,"reused":0,"kernels":1800,"retirements":8,"deleted":0,"misses":64,"cellTime":7656}},"burst":{"one":{"net":38640,"vsTwo":2252,"wins":8,"paid":19256,"peak":204,"nodes":191,"reused":420,"kernels":225,"retirements":0,"deleted":0,"misses":0,"cellTime":5760},"two":{"net":36388,"vsTwo":0,"wins":8,"paid":21508,"peak":211,"nodes":194,"reused":420,"kernels":225,"retirements":0,"deleted":0,"misses":0,"cellTime":7488},"shrink":{"net":36888,"vsTwo":500,"wins":8,"paid":21008,"peak":211,"nodes":194,"reused":420,"kernels":225,"retirements":4,"deleted":16,"misses":44,"cellTime":6848},"off":{"net":36092,"vsTwo":-296,"wins":8,"paid":21804,"peak":211,"nodes":194,"reused":420,"kernels":225,"retirements":4,"deleted":0,"misses":44,"cellTime":7488}},"drift":{"one":{"net":-13440,"vsTwo":5304,"wins":0,"paid":69071,"peak":204,"nodes":191,"reused":0,"kernels":1800,"retirements":0,"deleted":0,"misses":0,"cellTime":5760},"two":{"net":-18744,"vsTwo":0,"wins":0,"paid":74375,"peak":211,"nodes":194,"reused":0,"kernels":1800,"retirements":0,"deleted":0,"misses":0,"cellTime":7656},"shrink":{"net":-14720,"vsTwo":4024,"wins":0,"paid":70351,"peak":211,"nodes":194,"reused":0,"kernels":1800,"retirements":8,"deleted":32,"misses":64,"cellTime":5960},"off":{"net":-19192,"vsTwo":-448,"wins":0,"paid":74823,"peak":211,"nodes":194,"reused":0,"kernels":1800,"retirements":8,"deleted":0,"misses":64,"cellTime":7656}},"rare":{"one":{"net":-12498,"vsTwo":5280,"wins":0,"paid":68174,"peak":204,"nodes":191,"reused":8,"kernels":1770,"retirements":0,"deleted":0,"misses":0,"cellTime":5760},"two":{"net":-17778,"vsTwo":0,"wins":0,"paid":73454,"peak":211,"nodes":194,"reused":8,"kernels":1770,"retirements":0,"deleted":0,"misses":0,"cellTime":7656},"shrink":{"net":-13778,"vsTwo":4000,"wins":0,"paid":69454,"peak":211,"nodes":194,"reused":8,"kernels":1770,"retirements":8,"deleted":32,"misses":64,"cellTime":5960},"off":{"net":-18226,"vsTwo":-448,"wins":0,"paid":73902,"peak":211,"nodes":194,"reused":8,"kernels":1770,"retirements":8,"deleted":0,"misses":64,"cellTime":7656}},"k_change":{"one":{"net":27552,"vsTwo":6192,"wins":8,"paid":27272,"peak":204,"nodes":191,"reused":360,"kernels":432,"retirements":0,"deleted":0,"misses":0,"cellTime":5760},"two":{"net":21360,"vsTwo":0,"wins":8,"paid":33464,"peak":211,"nodes":194,"reused":360,"kernels":432,"retirements":0,"deleted":0,"misses":0,"cellTime":7584},"shrink":{"net":24572,"vsTwo":3212,"wins":8,"paid":30252,"peak":211,"nodes":194,"reused":360,"kernels":432,"retirements":4,"deleted":16,"misses":56,"cellTime":6496},"off":{"net":20992,"vsTwo":-368,"wins":8,"paid":33832,"peak":211,"nodes":194,"reused":360,"kernels":432,"retirements":4,"deleted":0,"misses":56,"cellTime":7584}},"mixed":{"one":{"net":-13536,"vsTwo":5424,"wins":0,"paid":69572,"peak":204,"nodes":191,"reused":0,"kernels":1800,"retirements":0,"deleted":0,"misses":0,"cellTime":5760},"two":{"net":-18960,"vsTwo":0,"wins":0,"paid":74996,"peak":211,"nodes":194,"reused":0,"kernels":1800,"retirements":0,"deleted":0,"misses":0,"cellTime":7656},"shrink":{"net":-14864,"vsTwo":4096,"wins":0,"paid":70900,"peak":211,"nodes":194,"reused":0,"kernels":1800,"retirements":8,"deleted":32,"misses":64,"cellTime":5960},"off":{"net":-19408,"vsTwo":-448,"wins":0,"paid":75444,"peak":211,"nodes":194,"reused":0,"kernels":1800,"retirements":8,"deleted":0,"misses":64,"cellTime":7656}},"alternating_to_drift":{"one":{"net":-16128,"vsTwo":-16586,"wins":0,"paid":83558,"peak":204,"nodes":191,"reused":0,"kernels":2160,"retirements":0,"deleted":0,"misses":0,"cellTime":6912},"two":{"net":458,"vsTwo":0,"wins":4,"paid":66972,"peak":211,"nodes":194,"reused":176,"kernels":1500,"retirements":0,"deleted":0,"misses":0,"cellTime":9192},"shrink":{"net":3602,"vsTwo":3144,"wins":4,"paid":63828,"peak":211,"nodes":194,"reused":176,"kernels":1500,"retirements":8,"deleted":32,"misses":64,"cellTime":7816},"off":{"net":10,"vsTwo":-448,"wins":3,"paid":67420,"peak":211,"nodes":194,"reused":176,"kernels":1500,"retirements":8,"deleted":0,"misses":64,"cellTime":9192}},"alternating_to_stable":{"one":{"net":29556,"vsTwo":-21782,"wins":8,"paid":38684,"peak":204,"nodes":191,"reused":376,"kernels":750,"retirements":0,"deleted":0,"misses":0,"cellTime":6912},"two":{"net":51338,"vsTwo":0,"wins":8,"paid":16902,"peak":211,"nodes":194,"reused":560,"kernels":60,"retirements":0,"deleted":0,"misses":0,"cellTime":9192},"shrink":{"net":51242,"vsTwo":-96,"wins":8,"paid":16998,"peak":211,"nodes":194,"reused":560,"kernels":60,"retirements":0,"deleted":0,"misses":16,"cellTime":9192},"off":{"net":51242,"vsTwo":-96,"wins":8,"paid":16998,"peak":211,"nodes":194,"reused":560,"kernels":60,"retirements":0,"deleted":0,"misses":16,"cellTime":9192}},"return":{"one":{"net":-16128,"vsTwo":-39532,"wins":0,"paid":84323,"peak":204,"nodes":191,"reused":0,"kernels":2160,"retirements":0,"deleted":0,"misses":0,"cellTime":6912},"two":{"net":23404,"vsTwo":0,"wins":6,"paid":44791,"peak":211,"nodes":194,"reused":352,"kernels":840,"retirements":0,"deleted":0,"misses":0,"cellTime":9192},"shrink":{"net":3602,"vsTwo":-19802,"wins":4,"paid":64593,"peak":211,"nodes":194,"reused":176,"kernels":1500,"retirements":8,"deleted":32,"misses":64,"cellTime":7816},"off":{"net":22956,"vsTwo":-448,"wins":6,"paid":45239,"peak":211,"nodes":194,"reused":352,"kernels":840,"retirements":8,"deleted":0,"misses":64,"cellTime":9192}},"stable_to_alternating":{"one":{"net":7200,"vsTwo":-43730,"wins":4,"paid":61400,"peak":204,"nodes":191,"reused":192,"kernels":1440,"retirements":0,"deleted":0,"misses":0,"cellTime":6912},"two":{"net":50930,"vsTwo":0,"wins":8,"paid":17670,"peak":211,"nodes":194,"reused":560,"kernels":60,"retirements":0,"deleted":0,"misses":0,"cellTime":8616},"shrink":{"net":50834,"vsTwo":-96,"wins":8,"paid":17766,"peak":211,"nodes":194,"reused":560,"kernels":60,"retirements":0,"deleted":0,"misses":16,"cellTime":8616},"off":{"net":50834,"vsTwo":-96,"wins":8,"paid":17766,"peak":211,"nodes":194,"reused":560,"kernels":60,"retirements":0,"deleted":0,"misses":16,"cellTime":8616}}},"worlds":480});
console.log(JSON.stringify({tier:"NON_NATIVE_CACHE_RECLAMATION",summary,rows,scenarios,sources:Object.fromEntries(["shared","one","two","shrink"].map(k=>[k,buildShrink(k)]))},null,2));
