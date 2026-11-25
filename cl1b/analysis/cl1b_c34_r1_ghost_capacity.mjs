#!/usr/bin/env node
import assert from "node:assert/strict";

function buildExpand(policy="ghost"){
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




 label("prod_init");set(60,0);if(["two","ghost"].includes(policy))set(64,0);C(0,0,"entry");
 label("entry");C(0,0,["shared","plain"].includes(policy)?"compute":"lookup1");
 const compare=(name,slot,yes,no)=>{label(name);for(const [i,key] of [[0,50],[1,51],[2,58]]){C(3,slot+i);R(0,3);C(3,key);R(1,3);S(0,0,1);B(0,no,i===2?yes:name+"_"+(i+1));if(i<2)label(name+"_"+(i+1));}};
 compare("lookup1",60,"hit1","miss_route");
 label("miss_route");C(0,0,["two","ghost"].includes(policy)?"lookup2":"compute");
 compare("lookup2",64,"second_match","compute");
 label("second_match");C(0,0,policy==="ghost"?"promote":"hit2");
 label("hit1");C(3,63);R(0,3,"publish");
 label("hit2");C(3,67);R(0,3,"publish");
 label("promote");C(1,"@flag");C(0,1);label("activate");K(1,0,"flag");
 label("flag");C(0,0);B(0,"enable","disable");
 label("enable");pe("second_match","hit2");pe("store_route","store_slot");pe("after_store","rotate");C(1,"@store_slot");C(0,67);K(1,0,"compute");
 label("disable");pe("miss_route","compute");pe("store_route","store_slot","compute");
 label("compute");C(3,50);R(1,3);C(3,51);R(2,3);C(3,43);W(3,1);C(3,44);W(3,2);C(3,41);W(3,2);C(3,58);R(0,3);C(3,42);W(3,0,"loop");
 label("loop");C(3,43);R(1,3);C(3,41);R(2,3,"kernel_dispatch");
 label("kernel_dispatch");C(0,0,"parent");
 label("parent");C(3,1);
 label("parent_add");A(0,0,2);S(1,1,3);B(1,"parent_add","out");
 label("base");C(3,0);A(1,1,3);C(3,"@special_entry");P(3,1);
 label("special_entry");C(0,0,"parent_add");
 label("out");C(3,44);R(1,3);A(0,0,1);C(3,41);W(3,0);C(3,42);R(1,3);C(2,1);S(1,1,2);W(3,1);B(1,"loop","done");
 label("done");C(3,41);R(0,3,["plain","shared"].includes(policy)?"publish":"store_route");
 label("store_route");C(0,0,policy==="ghost"?"ghost_copy":"store_slot");
 label("ghost_copy");for(const i of [2,1,0]){C(3,60+i);R(0,3);C(3,64+i);W(3,0);}
 label("store_slot");C(2,63);C(1,1);for(const [i,key] of [[3,41],[2,58],[1,44],[0,43]]){C(3,key);R(0,3);W(2,0);if(i)S(2,2,1);}
 label("after_store");C(0,0,policy==="two"?"rotate":"reload");
 label("rotate");C(1,127);S(0,1,2);C(1,"@store_slot");K(1,0,"reload");
 label("reload");C(3,41);R(0,3,"publish");
 label("publish");C(3,11);W(3,0,"entry");
 for(let i=0;i<g.length;i++){const n=g[i];if(n.kind==="Branch"){n.nonzero=l[n.nonzero];n.zero=l[n.zero]}else n.next=n.next===undefined?i+1:l[n.next];if(n.kind==="Const"&&typeof n.value==="string")n.value=l[n.value.slice(1)];}
 return {g,l};
}
function depth(x){return x.toString(2).length-1+x.toString(2).replace(/0/g,"").length;}
function rec(a,b,k){let z=b;for(let i=0;i<k;i++)z=a*z+b;return z;}
function runExpand(s,policy="ghost",off=false){
 const {g,l}=buildExpand(policy),initial=g.length,d=new Map([[50,s.requests[0][0]],[51,s.requests[0][1]],[58,s.requests[0][2]]]),v=[0,0,0,0];let pc=0,paid=0,prod=0,peak=g.length+1+d.size,clones=0,kernels=0,sk=0,sc=0,entry=null,hit=null,stores=0,evictions=0,activation=null,exposure=null,ghostCopies=0;const outputs=[],services=[],prefix=[],records=[null,null];
 const input=()=>[d.get(50),d.get(51),d.get(58)];
 while(paid<200000){
  const p=pc,n={...g[p]};if(!n.kind)throw Error("pc");
  if(p===l.activate&&exposure===null)exposure=paid+1;
  if(exposure===null&&policy==="ghost")prefix.push([p,[...v],[...d],g.length,n]);
  if(p===l.entry)hit=null;if(p===l.hit1||p===l.hit2)hit=p===l.hit1?0:1;
  if(p===l.kernel_dispatch){const [a,b,k]=input();if(v[1]!==a||v[2]!==rec(a,b,sk)||sk>=k)throw Error("kernel inputs");entry=initial+a-1;sk++;kernels++;}
  if(p>=initial)sc++;
  paid++;pc=n.next;
  switch(n.kind){
   case "Const":v[n.dst]=n.value;break;
   case "Read":v[n.dst]=d.get(v[n.address])??0;break;
   case "Write":{const key=v[n.address],z=v[n.value];d.set(key,z);
    if(p>=l.ghost_copy&&p<l.store_slot&&key===64){ghostCopies++;if(d.has(67)||[0,1,2].some(i=>(d.get(64+i)??0)!==(d.get(60+i)??0)))throw Error("ghost copy");}
    if(p>=l.store_slot&&p<l.after_store&&[60,64].includes(key)){const slot=key===60?0:1,q=input();if([0,1,2].some(i=>d.get(key+i)!==q[i])||d.get(key+3)!==rec(...q)||sk!==q[2])throw Error("store");if(records[slot])evictions++;records[slot]=[...q,rec(...q)];stores++;}
    if(key===11){const q=input();if(z!==rec(...q))throw Error("output "+s.id+" "+prod);const reused=sk===0;if(reused){if(hit===null||!records[hit]||records[hit].slice(0,3).join()!==q.join()||records[hit][3]!==z)throw Error("stale");}else if(sk!==q[2])throw Error("kernel count");if(sc!==(policy==="plain"?0:sk*depth(q[0])))throw Error("clone count");
     prod++;outputs.push(z);services.push({at:prod,paid,reused,hit,slots:records.filter(Boolean).length,cells:d.size});sk=sc=0;if(prod===s.requests.length)return {paid,outputs,services,clones,kernels,stores,evictions,peak,nodes:g.length,activation,exposure,prefix,ghostCopies};
     const q2=s.requests[prod];[50,51,58].forEach((k,i)=>d.set(k,q2[i]));}break;}
   case "Add":v[n.dst]=v[n.lhs]+v[n.rhs];break;
   case "Sub":v[n.dst]=v[n.lhs]-v[n.rhs];break;
   case "Branch":pc=v[n.test]!==0?n.nonzero:n.zero;break;
   case "CloneNode":{const from=v[n.source],idx=g.length,x=idx-initial+1;if(from!==(x%2?l.parent_add:l.double_src))throw Error("template");g.push({...g[from]});v[n.dst]=idx;clones++;break;}
   case "PatchConst":{const at=v[n.node],to=v[n.value];if(at===l.flag){if(p!==l.activate||to!==1||activation||d.has(67)||[0,1,2].some(i=>d.get(64+i)!==input()[i]))throw Error("promotion");activation={at:prod,paid,request:input()};if(!off)g[at].value=to;}else if(at===l.store_slot){if(![63,67].includes(to)||!(policy==="two"||activation&&!off)||to===g[at].value)throw Error("rotation");g[at].value=to;}else{if(at!==l.base||to!==initial-1)throw Error("base");g[at].value=to;}break;}
   case "PatchNext":{const at=v[n.node],to=v[n.target];if(!g[to]||g[at].next===undefined)throw Error("edge");if(at>=initial){const x=at-initial+1;if(to!==(x===1?l.out:x%2?at-1:initial+x/2-1))throw Error("generated");}else if(at===l.special_entry){if(to!==entry)throw Error("entry");}else if(at===l.kernel_dispatch){if(to!==l.base)throw Error("kernel");}else if(at===l.store_route){if(to!==l.store_slot||!activation)throw Error("store route");}else if(at===l.second_match){if(to!==l.hit2||off||!activation)throw Error("promote edge");}else if(at===l.after_store){if(off||to!==l.rotate||!activation)throw Error("flip");}else if(at===l.miss_route){if(!off||to!==l.compute||!activation)throw Error("disable");}else throw Error("patch");g[at].next=to;break;}
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

const rows=scenarios.map(s=>{
 const arms={shared:runExpand(s,"shared"),one:runExpand(s,"one"),two:runExpand(s,"two"),ghost:runExpand(s),off:runExpand(s,"ghost",true)};
 for(const q of Object.values(arms))if(q.outputs.join()!==arms.shared.outputs.join())throw Error("outputs");
 if(JSON.stringify(arms.ghost.prefix)!==JSON.stringify(arms.off.prefix)||JSON.stringify(arms.ghost.activation)!==JSON.stringify(arms.off.activation))throw Error("prefix");
 if(arms.off.services.some(x=>x.slots>1))throw Error("off slots");
 const compact=q=>({paid:q.paid,kernels:q.kernels,nodes:q.nodes,peak:q.peak,stores:q.stores,evictions:q.evictions,reused:q.services.filter(x=>x.reused).length,activation:q.activation,ghostCopies:q.ghostCopies,maxSlots:Math.max(...q.services.map(x=>x.slots))});
 return {id:s.id,k:s.k,n:s.n,pattern:s.pattern,...Object.fromEntries(Object.entries(arms).map(([k,q])=>[k,compact(q)]))};
});
function agg(a){return Object.fromEntries(["one","two","ghost","off"].map(k=>[k,{net:a.reduce((s,r)=>s+r.shared.paid-r[k].paid,0),wins:a.filter(r=>r.shared.paid>r[k].paid).length,paid:a.reduce((s,r)=>s+r[k].paid,0),peak:Math.max(...a.map(r=>r[k].peak)),nodes:Math.max(...a.map(r=>r[k].nodes)),reused:a.reduce((s,r)=>s+r[k].reused,0),kernels:a.reduce((s,r)=>s+r[k].kernels,0),expanded:a.filter(r=>r[k].activation).length,ghostCopies:a.reduce((s,r)=>s+r[k].ghostCopies,0)}]))}
const summary={all:agg(rows),patterns:Object.fromEntries(patterns.map(k=>[k,agg(rows.filter(r=>r.pattern===k))])),scales:Object.fromEntries([1,2,4,8].map(k=>[k,agg(rows.filter(r=>r.k===k))])),worlds:rows.length*5};

assert.deepEqual(summary,{"all":{"one":{"net":43746,"wins":24,"paid":408505,"peak":207,"nodes":194,"reused":1260,"kernels":9657,"expanded":0,"ghostCopies":0},"two":{"net":71184,"wins":32,"paid":381067,"peak":214,"nodes":197,"reused":1724,"kernels":7917,"expanded":0,"ghostCopies":0},"ghost":{"net":58260,"wins":32,"paid":393991,"peak":214,"nodes":197,"reused":1716,"kernels":7947,"expanded":8,"ghostCopies":2116},"off":{"net":3858,"wins":24,"paid":448393,"peak":213,"nodes":197,"reused":1260,"kernels":9657,"expanded":8,"ghostCopies":2116}},"patterns":{"stable":{"one":{"net":43908,"wins":8,"paid":12788,"peak":207,"nodes":194,"reused":472,"kernels":30,"expanded":0,"ghostCopies":0},"two":{"net":43804,"wins":8,"paid":12892,"peak":211,"nodes":197,"reused":472,"kernels":30,"expanded":0,"ghostCopies":0},"ghost":{"net":43740,"wins":8,"paid":12956,"peak":213,"nodes":197,"reused":472,"kernels":30,"expanded":0,"ghostCopies":8},"off":{"net":43740,"wins":8,"paid":12956,"peak":213,"nodes":197,"reused":472,"kernels":30,"expanded":0,"ghostCopies":8}},"alternating":{"one":{"net":-13440,"wins":0,"paid":71036,"peak":207,"nodes":194,"reused":0,"kernels":1800,"expanded":0,"ghostCopies":0},"two":{"net":41766,"wins":8,"paid":15830,"peak":214,"nodes":197,"reused":464,"kernels":60,"expanded":0,"ghostCopies":0},"ghost":{"net":40410,"wins":8,"paid":17186,"peak":214,"nodes":197,"reused":456,"kernels":90,"expanded":8,"ghostCopies":16},"off":{"net":-13992,"wins":0,"paid":71588,"peak":213,"nodes":197,"reused":0,"kernels":1800,"expanded":8,"ghostCopies":16}},"cycle3":{"one":{"net":-13440,"wins":0,"paid":71336,"peak":207,"nodes":194,"reused":0,"kernels":1800,"expanded":0,"ghostCopies":0},"two":{"net":-18264,"wins":0,"paid":76160,"peak":214,"nodes":197,"reused":0,"kernels":1800,"expanded":0,"ghostCopies":0},"ghost":{"net":-22104,"wins":0,"paid":80000,"peak":213,"nodes":197,"reused":0,"kernels":1800,"expanded":0,"ghostCopies":480},"off":{"net":-22104,"wins":0,"paid":80000,"peak":213,"nodes":197,"reused":0,"kernels":1800,"expanded":0,"ghostCopies":480}},"burst":{"one":{"net":38640,"wins":8,"paid":19256,"peak":207,"nodes":194,"reused":420,"kernels":225,"expanded":0,"ghostCopies":0},"two":{"net":36448,"wins":8,"paid":21448,"peak":214,"nodes":197,"reused":420,"kernels":225,"expanded":0,"ghostCopies":0},"ghost":{"net":37536,"wins":8,"paid":20360,"peak":213,"nodes":197,"reused":420,"kernels":225,"expanded":0,"ghostCopies":60},"off":{"net":37536,"wins":8,"paid":20360,"peak":213,"nodes":197,"reused":420,"kernels":225,"expanded":0,"ghostCopies":60}},"drift":{"one":{"net":-13440,"wins":0,"paid":69071,"peak":207,"nodes":194,"reused":0,"kernels":1800,"expanded":0,"ghostCopies":0},"two":{"net":-18264,"wins":0,"paid":73895,"peak":214,"nodes":197,"reused":0,"kernels":1800,"expanded":0,"ghostCopies":0},"ghost":{"net":-22104,"wins":0,"paid":77735,"peak":213,"nodes":197,"reused":0,"kernels":1800,"expanded":0,"ghostCopies":480},"off":{"net":-22104,"wins":0,"paid":77735,"peak":213,"nodes":197,"reused":0,"kernels":1800,"expanded":0,"ghostCopies":480}},"rare":{"one":{"net":-12498,"wins":0,"paid":68174,"peak":207,"nodes":194,"reused":8,"kernels":1770,"expanded":0,"ghostCopies":0},"two":{"net":-17306,"wins":0,"paid":72982,"peak":214,"nodes":197,"reused":8,"kernels":1770,"expanded":0,"ghostCopies":0},"ghost":{"net":-21018,"wins":0,"paid":76694,"peak":213,"nodes":197,"reused":8,"kernels":1770,"expanded":0,"ghostCopies":472},"off":{"net":-21018,"wins":0,"paid":76694,"peak":213,"nodes":197,"reused":8,"kernels":1770,"expanded":0,"ghostCopies":472}},"k_change":{"one":{"net":27552,"wins":8,"paid":27272,"peak":207,"nodes":194,"reused":360,"kernels":432,"expanded":0,"ghostCopies":0},"two":{"net":21480,"wins":8,"paid":33344,"peak":214,"nodes":197,"reused":360,"kernels":432,"expanded":0,"ghostCopies":0},"ghost":{"net":24120,"wins":8,"paid":30704,"peak":213,"nodes":197,"reused":360,"kernels":432,"expanded":0,"ghostCopies":120},"off":{"net":24120,"wins":8,"paid":30704,"peak":213,"nodes":197,"reused":360,"kernels":432,"expanded":0,"ghostCopies":120}},"mixed":{"one":{"net":-13536,"wins":0,"paid":69572,"peak":207,"nodes":194,"reused":0,"kernels":1800,"expanded":0,"ghostCopies":0},"two":{"net":-18480,"wins":0,"paid":74516,"peak":214,"nodes":197,"reused":0,"kernels":1800,"expanded":0,"ghostCopies":0},"ghost":{"net":-22320,"wins":0,"paid":78356,"peak":213,"nodes":197,"reused":0,"kernels":1800,"expanded":0,"ghostCopies":480},"off":{"net":-22320,"wins":0,"paid":78356,"peak":213,"nodes":197,"reused":0,"kernels":1800,"expanded":0,"ghostCopies":480}}},"scales":{"1":{"one":{"net":-5338,"wins":6,"paid":58239,"peak":207,"nodes":194,"reused":315,"kernels":723,"expanded":0,"ghostCopies":0},"two":{"net":-6932,"wins":8,"paid":59833,"peak":214,"nodes":197,"reused":431,"kernels":607,"expanded":0,"ghostCopies":0},"ghost":{"net":-10020,"wins":8,"paid":62921,"peak":214,"nodes":197,"reused":429,"kernels":609,"expanded":2,"ghostCopies":529},"off":{"net":-15310,"wins":6,"paid":68211,"peak":213,"nodes":197,"reused":315,"kernels":723,"expanded":2,"ghostCopies":529}},"2":{"one":{"net":580,"wins":6,"paid":74198,"peak":207,"nodes":194,"reused":315,"kernels":1338,"expanded":0,"ghostCopies":0},"two":{"net":2060,"wins":8,"paid":72718,"peak":214,"nodes":197,"reused":431,"kernels":1106,"expanded":0,"ghostCopies":0},"ghost":{"net":-1080,"wins":8,"paid":75858,"peak":214,"nodes":197,"reused":429,"kernels":1110,"expanded":2,"ghostCopies":529},"off":{"net":-9392,"wins":6,"paid":84170,"peak":213,"nodes":197,"reused":315,"kernels":1338,"expanded":2,"ghostCopies":529}},"4":{"one":{"net":12416,"wins":6,"paid":106116,"peak":207,"nodes":194,"reused":315,"kernels":2568,"expanded":0,"ghostCopies":0},"two":{"net":20044,"wins":8,"paid":98488,"peak":214,"nodes":197,"reused":431,"kernels":2104,"expanded":0,"ghostCopies":0},"ghost":{"net":16800,"wins":8,"paid":101732,"peak":214,"nodes":197,"reused":429,"kernels":2112,"expanded":2,"ghostCopies":529},"off":{"net":2444,"wins":6,"paid":116088,"peak":213,"nodes":197,"reused":315,"kernels":2568,"expanded":2,"ghostCopies":529}},"8":{"one":{"net":36088,"wins":6,"paid":169952,"peak":207,"nodes":194,"reused":315,"kernels":5028,"expanded":0,"ghostCopies":0},"two":{"net":56012,"wins":8,"paid":150028,"peak":214,"nodes":197,"reused":431,"kernels":4100,"expanded":0,"ghostCopies":0},"ghost":{"net":52560,"wins":8,"paid":153480,"peak":214,"nodes":197,"reused":429,"kernels":4116,"expanded":2,"ghostCopies":529},"off":{"net":26116,"wins":6,"paid":179924,"peak":213,"nodes":197,"reused":315,"kernels":5028,"expanded":2,"ghostCopies":529}}},"worlds":320});
console.log(JSON.stringify({summary,rows,scenarios,sources:Object.fromEntries(["shared","one","two","ghost"].map(k=>[k,buildExpand(k)]))},null,2));
