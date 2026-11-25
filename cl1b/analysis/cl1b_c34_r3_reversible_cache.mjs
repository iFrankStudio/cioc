#!/usr/bin/env node
import assert from "node:assert/strict";

function buildCycle(policy="cycle"){
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





 const pk=(at,value,next)=>{C(1,"@"+at);C(0,value);K(1,0,next)};
 label("prod_init");C(0,0,"entry");
 label("entry");C(0,0,policy==="shared"?"compute":"encode");
 // key = (32*a+b)*16+k, positive and injective on a,b=1..16,k=1,2,4,8.
 label("encode");C(3,50);R(0,3);for(let i=0;i<5;i++)A(0,0,0);C(3,51);R(1,3);A(0,0,1);for(let i=0;i<4;i++)A(0,0,0);C(3,58);R(1,3);A(0,0,1);C(3,59);W(3,0);
 label("lookup1");C(3,60);R(1,3);S(2,0,1);B(2,"miss_route","hit1");
 label("hit1");C(3,61);R(0,3,"publish");
 label("miss_route");C(2,0,policy==="one"?"compute":policy==="two"?"lookup2":"ghost");
 label("ghost");C(1,0);S(2,0,1);B(2,"remember","grow");
 label("remember");C(3,60);R(0,3);C(1,"@ghost");K(1,0,"compute");
 label("grow");C(1,"@grow_flag");C(0,1);label("activate_grow");K(1,0);
 label("grow_flag");C(0,0);B(0,"expand","remember");
 label("expand");pe("miss_route","lookup2");pe("after_store","rotate");pk("slot",64);pk("budget",8,"compute");
 label("lookup2");C(3,64);R(1,3);S(2,0,1);B(2,"miss2","hit2");
 label("hit2");C(3,65);R(0,3,"publish");
 label("miss2");C(0,0,policy==="cycle"?"budget":"compute");
 label("budget");C(0,8);C(1,1);S(0,0,1);C(1,"@budget");K(1,0);B(0,"compute","shrink");
 label("shrink");C(1,"@shrink_flag");C(0,1);label("activate_shrink");K(1,0);
 label("shrink_flag");C(0,0);B(0,"release","retire_monitor");
 label("retire_monitor");pe("miss2","compute","compute");
 label("release");pe("miss_route","ghost");pe("after_store","reload");pk("slot",60);
 for(const key of [64,65]){C(3,key);op("Delete",{address:3,next:key===65?"remember":undefined});}
 label("compute");C(3,50);R(1,3);C(3,51);R(2,3);C(3,43);W(3,1);C(3,44);W(3,2);C(3,41);W(3,2);C(3,58);R(0,3);C(3,42);W(3,0,"loop");
 label("loop");C(3,43);R(1,3);C(3,41);R(2,3,"kernel_dispatch");
 label("kernel_dispatch");C(0,0,"parent");
 label("parent");C(3,1);
 label("parent_add");A(0,0,2);S(1,1,3);B(1,"parent_add","out");
 label("base");C(3,0);A(1,1,3);C(3,"@special_entry");P(3,1);
 label("special_entry");C(0,0,"parent_add");
 label("out");C(3,44);R(1,3);A(0,0,1);C(3,41);W(3,0);C(3,42);R(1,3);C(2,1);S(1,1,2);W(3,1);B(1,"loop","done");
 label("done");C(3,41);R(0,3,policy==="shared"?"publish":"slot");
 label("slot");C(2,60);C(1,1);A(3,2,1);W(3,0);C(3,59);R(0,3);W(2,0);
 label("after_store");C(0,0,policy==="two"?"rotate":"reload");
 label("rotate");C(1,124);S(0,1,2);C(1,"@slot");K(1,0,"reload");
 label("reload");C(3,41);R(0,3,"publish");
 label("publish");C(3,11);W(3,0,"entry");
 for(let i=0;i<g.length;i++){const n=g[i];if(n.kind==="Branch"){n.nonzero=l[n.nonzero];n.zero=l[n.zero]}else n.next=n.next===undefined?i+1:l[n.next];if(n.kind==="Const"&&typeof n.value==="string")n.value=l[n.value.slice(1)];}
 return {g,l};
}
function depth(x){return x.toString(2).length-1+x.toString(2).replace(/0/g,"").length;}
function rec(a,b,k){let z=b;for(let i=0;i<k;i++)z=a*z+b;return z;}
function key(a,b,k){return (32*a+b)*16+k;}
function runCycle(s,policy="cycle",off="none"){
 const {g,l}=buildCycle(policy),initial=g.length,d=new Map([[50,s.requests[0][0]],[51,s.requests[0][1]],[58,s.requests[0][2]]]),v=[0,0,0,0];let pc=0,paid=0,prod=0,peak=g.length+1+d.size,clones=0,kernels=0,sk=0,sc=0,entry=null,hit=null,stores=0,evictions=0,deleted=0,cellTime=0,misses=0,exposure=null;const outputs=[],services=[],prefix=[],events=[],records=[null,null];let expanded=policy==="two";
 const input=()=>[d.get(50),d.get(51),d.get(58)];
 while(paid<200000){
  const p=pc,n={...g[p]};if(!n.kind)throw Error("pc");
  if(exposure===null&&[l.activate_grow,l.activate_shrink].includes(p))exposure=paid+1;
  if(exposure===null&&policy==="cycle")prefix.push([p,[...v],[...d],g.length,n]);
  if(p===l.entry)hit=null;if(p===l.hit1||p===l.hit2)hit=p===l.hit1?0:1;
  if(p===l.budget)misses++;
  if(p===l.kernel_dispatch){const [a,b,k]=input();if(v[1]!==a||v[2]!==rec(a,b,sk)||sk>=k)throw Error("kernel inputs");entry=initial+a-1;sk++;kernels++;}
  if(p>=initial)sc++;
  paid++;pc=n.next;
  switch(n.kind){
   case "Const":v[n.dst]=n.value;break;
   case "Read":v[n.dst]=d.get(v[n.address])??0;break;
   case "Write":{const k=v[n.address],z=v[n.value];d.set(k,z);
    if(k===59&&z!==key(...input()))throw Error("encoding");
    if([60,64].includes(k)){const q=input(),slot=k===60?0:1;if(z!==key(...q)||d.get(k+1)!==rec(...q)||sk!==q[2]||slot===1&&!expanded)throw Error("store");if(records[slot])evictions++;records[slot]=[...q,rec(...q)];stores++;}
    if(k===11){const q=input();if(z!==rec(...q))throw Error("output "+s.id+" "+prod+" "+policy+" "+off+" "+z);const reused=sk===0;if(reused){if(hit===null||!records[hit]||records[hit].slice(0,3).join()!==q.join()||records[hit][3]!==z)throw Error("stale");}else if(sk!==q[2])throw Error("kernel count");if(sc!==sk*depth(q[0]))throw Error("clone count");if(!expanded&&d.has(64))throw Error("slot leaked");
     prod++;cellTime+=d.size;outputs.push(z);services.push({at:prod,paid,reused,hit,expanded,cells:d.size});sk=sc=0;if(prod===s.requests.length)return {paid,outputs,services,clones,kernels,stores,evictions,peak,nodes:g.length,events,exposure,prefix,deleted,misses,cellTime};
     const q2=s.requests[prod];[50,51,58].forEach((k,i)=>d.set(k,q2[i]));}break;}
   case "Delete":{const k=v[n.address];if(expanded||![64,65].includes(k)||!d.has(k))throw Error("delete");d.delete(k);deleted++;if(k===65)records[1]=null;break;}
   case "Add":v[n.dst]=v[n.lhs]+v[n.rhs];break;
   case "Sub":v[n.dst]=v[n.lhs]-v[n.rhs];break;
   case "Branch":pc=v[n.test]!==0?n.nonzero:n.zero;break;
   case "CloneNode":{const from=v[n.source],idx=g.length,x=idx-initial+1;if(from!==(x%2?l.parent_add:l.double_src))throw Error("template");g.push({...g[from]});v[n.dst]=idx;clones++;break;}
   case "PatchConst":{const at=v[n.node],to=v[n.value];if(at===l.grow_flag){if(p!==l.activate_grow||expanded||g[l.ghost].value!==key(...input())||to!==1||d.has(64))throw Error("grow");events.push({kind:"grow",at:prod,paid,suppressed:off==="grow"});if(off!=="grow"){g[at].value=1;expanded=true;}}
    else if(at===l.shrink_flag){if(p!==l.activate_shrink||!expanded||g[l.budget].value!==0||to!==1||!d.has(64)||!d.has(65))throw Error("shrink");events.push({kind:"shrink",at:prod,paid,suppressed:off==="shrink"});if(off!=="shrink"){g[at].value=1;expanded=false;}}
    else if(at===l.ghost){if(to!==0&&to!==(d.get(60)??0))throw Error("ghost");g[at].value=to;}
    else if(at===l.budget){if(!(to===8||to===g[at].value-1&&to>=0))throw Error("budget");g[at].value=to;}
    else if(at===l.slot){if(![60,64].includes(to)||to===64&&!expanded)throw Error("slot");g[at].value=to;}
    else{if(at!==l.base||to!==initial-1)throw Error("base");g[at].value=to;}break;}
   case "PatchNext":{const at=v[n.node],to=v[n.target];if(!g[to]||g[at].next===undefined)throw Error("edge");if(at>=initial){const x=at-initial+1;if(to!==(x===1?l.out:x%2?at-1:initial+x/2-1))throw Error("generated");}else if(at===l.special_entry){if(to!==entry)throw Error("entry");}else if(at===l.kernel_dispatch){if(to!==l.base)throw Error("kernel");}else if(at===l.miss_route){if(to!==(expanded?l.lookup2:l.ghost))throw Error("route");}else if(at===l.after_store){if(to!==(expanded?l.rotate:l.reload))throw Error("rotation");}else if(at===l.miss2){if(off!=="shrink"||to!==l.compute)throw Error("monitor");}else throw Error("patch");g[at].next=to;break;}
   default:throw Error(n.kind);
  }
  if(!v.every(Number.isSafeInteger))throw Error("integer");peak=Math.max(peak,g.length+1+d.size);if(g.length>200||d.size>16||peak>220)throw Error("capacity "+g.length+" "+peak+" "+d.size);
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

for(const k of [1,4,8])for(const n of [96,192]){const A=[7,3,k],B=[11,5,k];const requests=Array.from({length:n},(_,i)=>Math.floor(i/24)%2===0?(i%2?B:A):[1+(i*3)%16,1+(i*5+Math.floor(i/16))%16,k]);scenarios.push({id:"repeated_"+k+"_"+n,k,n,pattern:"repeated",requests});}
const rows=scenarios.map(s=>{
 const arms={shared:runCycle(s,"shared"),one:runCycle(s,"one"),two:runCycle(s,"two"),grow:runCycle(s,"grow"),cycle:runCycle(s),offGrow:runCycle(s,"cycle","grow"),offShrink:runCycle(s,"cycle","shrink")};
 for(const q of Object.values(arms))if(q.outputs.join()!==arms.shared.outputs.join())throw Error("outputs");
 for(const q of [arms.offGrow,arms.offShrink])if(JSON.stringify(q.prefix)!==JSON.stringify(arms.cycle.prefix)||q.exposure!==arms.cycle.exposure)throw Error("prefix");
 if(arms.offGrow.deleted||arms.offGrow.services.some(x=>x.expanded)||arms.offShrink.deleted)throw Error("off");
 const compact=q=>({paid:q.paid,kernels:q.kernels,nodes:q.nodes,peak:q.peak,stores:q.stores,evictions:q.evictions,reused:q.services.filter(x=>x.reused).length,events:q.events,deleted:q.deleted,misses:q.misses,cellTime:q.cellTime,finalCells:q.services.at(-1).cells});
 return {id:s.id,k:s.k,n:s.n,pattern:s.pattern,...Object.fromEntries(Object.entries(arms).map(([k,q])=>[k,compact(q)]))};
});
function agg(a){return Object.fromEntries(["one","two","grow","cycle","offGrow","offShrink"].map(k=>[k,{net:a.reduce((s,r)=>s+r.shared.paid-r[k].paid,0),wins:a.filter(r=>r.shared.paid>r[k].paid).length,paid:a.reduce((s,r)=>s+r[k].paid,0),peak:Math.max(...a.map(r=>r[k].peak)),nodes:Math.max(...a.map(r=>r[k].nodes)),reused:a.reduce((s,r)=>s+r[k].reused,0),kernels:a.reduce((s,r)=>s+r[k].kernels,0),grow:a.reduce((s,r)=>s+r[k].events.filter(e=>e.kind==="grow"&&!e.suppressed).length,0),shrink:a.reduce((s,r)=>s+r[k].events.filter(e=>e.kind==="shrink"&&!e.suppressed).length,0),deleted:a.reduce((s,r)=>s+r[k].deleted,0),cellTime:a.reduce((s,r)=>s+r[k].cellTime,0)}]))}
const summary={all:agg(rows),patterns:Object.fromEntries([...patterns,"alternating_to_drift","alternating_to_stable","return","stable_to_alternating","repeated"].map(k=>[k,agg(rows.filter(r=>r.pattern===k))])),regrowth:rows.filter(r=>r.cycle.events.filter(e=>e.kind==="grow"&&!e.suppressed).length>1).map(r=>r.id),worlds:rows.length*7};

assert.deepEqual(summary,{"all":{"one":{"net":-14726,"wins":34,"paid":853634,"peak":199,"nodes":187,"reused":1828,"kernels":19911,"grow":0,"shrink":0,"deleted":0,"cellTime":77088},"two":{"net":196758,"wins":60,"paid":642150,"peak":201,"nodes":187,"reused":3768,"kernels":12405,"grow":0,"shrink":0,"deleted":0,"cellTime":89412},"grow":{"net":194542,"wins":60,"paid":644366,"peak":201,"nodes":187,"reused":3722,"kernels":12581,"grow":46,"shrink":0,"deleted":0,"cellTime":83816},"cycle":{"net":190838,"wins":60,"paid":648070,"peak":201,"nodes":187,"reused":3702,"kernels":12663,"grow":66,"shrink":34,"deleted":68,"cellTime":82196},"offGrow":{"net":-60686,"wins":34,"paid":899594,"peak":199,"nodes":187,"reused":1828,"kernels":19911,"grow":0,"shrink":0,"deleted":0,"cellTime":77088},"offShrink":{"net":193310,"wins":60,"paid":645598,"peak":201,"nodes":187,"reused":3722,"kernels":12581,"grow":46,"shrink":0,"deleted":0,"cellTime":83816}},"patterns":{"stable":{"one":{"net":41500,"wins":8,"paid":15172,"peak":199,"nodes":187,"reused":472,"kernels":30,"grow":0,"shrink":0,"deleted":0,"cellTime":5280},"two":{"net":41428,"wins":8,"paid":15244,"peak":199,"nodes":187,"reused":472,"kernels":30,"grow":0,"shrink":0,"deleted":0,"cellTime":5280},"grow":{"net":41444,"wins":8,"paid":15228,"peak":199,"nodes":187,"reused":472,"kernels":30,"grow":0,"shrink":0,"deleted":0,"cellTime":5280},"cycle":{"net":41444,"wins":8,"paid":15228,"peak":199,"nodes":187,"reused":472,"kernels":30,"grow":0,"shrink":0,"deleted":0,"cellTime":5280},"offGrow":{"net":41444,"wins":8,"paid":15228,"peak":199,"nodes":187,"reused":472,"kernels":30,"grow":0,"shrink":0,"deleted":0,"cellTime":5280},"offShrink":{"net":41444,"wins":8,"paid":15228,"peak":199,"nodes":187,"reused":472,"kernels":30,"grow":0,"shrink":0,"deleted":0,"cellTime":5280}},"alternating":{"one":{"net":-16320,"wins":0,"paid":73892,"peak":199,"nodes":187,"reused":0,"kernels":1800,"grow":0,"shrink":0,"deleted":0,"cellTime":5280},"two":{"net":40086,"wins":8,"paid":17486,"peak":201,"nodes":187,"reused":464,"kernels":60,"grow":0,"shrink":0,"deleted":0,"cellTime":6224},"grow":{"net":38986,"wins":8,"paid":18586,"peak":201,"nodes":187,"reused":456,"kernels":90,"grow":8,"shrink":0,"deleted":0,"cellTime":6208},"cycle":{"net":38986,"wins":8,"paid":18586,"peak":201,"nodes":187,"reused":456,"kernels":90,"grow":8,"shrink":0,"deleted":0,"cellTime":6208},"offGrow":{"net":-22000,"wins":0,"paid":79572,"peak":199,"nodes":187,"reused":0,"kernels":1800,"grow":0,"shrink":0,"deleted":0,"cellTime":5280},"offShrink":{"net":38986,"wins":8,"paid":18586,"peak":201,"nodes":187,"reused":456,"kernels":90,"grow":8,"shrink":0,"deleted":0,"cellTime":6208}},"cycle3":{"one":{"net":-16320,"wins":0,"paid":74192,"peak":199,"nodes":187,"reused":0,"kernels":1800,"grow":0,"shrink":0,"deleted":0,"cellTime":5280},"two":{"net":-20640,"wins":0,"paid":78512,"peak":201,"nodes":187,"reused":0,"kernels":1800,"grow":0,"shrink":0,"deleted":0,"cellTime":6224},"grow":{"net":-19680,"wins":0,"paid":77552,"peak":199,"nodes":187,"reused":0,"kernels":1800,"grow":0,"shrink":0,"deleted":0,"cellTime":5280},"cycle":{"net":-19680,"wins":0,"paid":77552,"peak":199,"nodes":187,"reused":0,"kernels":1800,"grow":0,"shrink":0,"deleted":0,"cellTime":5280},"offGrow":{"net":-19680,"wins":0,"paid":77552,"peak":199,"nodes":187,"reused":0,"kernels":1800,"grow":0,"shrink":0,"deleted":0,"cellTime":5280},"offShrink":{"net":-19680,"wins":0,"paid":77552,"peak":199,"nodes":187,"reused":0,"kernels":1800,"grow":0,"shrink":0,"deleted":0,"cellTime":5280}},"burst":{"one":{"net":36180,"wins":8,"paid":21692,"peak":199,"nodes":187,"reused":420,"kernels":225,"grow":0,"shrink":0,"deleted":0,"cellTime":5280},"two":{"net":34660,"wins":8,"paid":23212,"peak":201,"nodes":187,"reused":420,"kernels":225,"grow":0,"shrink":0,"deleted":0,"cellTime":6112},"grow":{"net":35760,"wins":8,"paid":22112,"peak":199,"nodes":187,"reused":420,"kernels":225,"grow":0,"shrink":0,"deleted":0,"cellTime":5280},"cycle":{"net":35760,"wins":8,"paid":22112,"peak":199,"nodes":187,"reused":420,"kernels":225,"grow":0,"shrink":0,"deleted":0,"cellTime":5280},"offGrow":{"net":35760,"wins":8,"paid":22112,"peak":199,"nodes":187,"reused":420,"kernels":225,"grow":0,"shrink":0,"deleted":0,"cellTime":5280},"offShrink":{"net":35760,"wins":8,"paid":22112,"peak":199,"nodes":187,"reused":420,"kernels":225,"grow":0,"shrink":0,"deleted":0,"cellTime":5280}},"drift":{"one":{"net":-16320,"wins":0,"paid":71927,"peak":199,"nodes":187,"reused":0,"kernels":1800,"grow":0,"shrink":0,"deleted":0,"cellTime":5280},"two":{"net":-20640,"wins":0,"paid":76247,"peak":201,"nodes":187,"reused":0,"kernels":1800,"grow":0,"shrink":0,"deleted":0,"cellTime":6224},"grow":{"net":-19680,"wins":0,"paid":75287,"peak":199,"nodes":187,"reused":0,"kernels":1800,"grow":0,"shrink":0,"deleted":0,"cellTime":5280},"cycle":{"net":-19680,"wins":0,"paid":75287,"peak":199,"nodes":187,"reused":0,"kernels":1800,"grow":0,"shrink":0,"deleted":0,"cellTime":5280},"offGrow":{"net":-19680,"wins":0,"paid":75287,"peak":199,"nodes":187,"reused":0,"kernels":1800,"grow":0,"shrink":0,"deleted":0,"cellTime":5280},"offShrink":{"net":-19680,"wins":0,"paid":75287,"peak":199,"nodes":187,"reused":0,"kernels":1800,"grow":0,"shrink":0,"deleted":0,"cellTime":5280}},"rare":{"one":{"net":-15370,"wins":0,"paid":71022,"peak":199,"nodes":187,"reused":8,"kernels":1770,"grow":0,"shrink":0,"deleted":0,"cellTime":5280},"two":{"net":-19658,"wins":0,"paid":75310,"peak":201,"nodes":187,"reused":8,"kernels":1770,"grow":0,"shrink":0,"deleted":0,"cellTime":6224},"grow":{"net":-18674,"wins":0,"paid":74326,"peak":199,"nodes":187,"reused":8,"kernels":1770,"grow":0,"shrink":0,"deleted":0,"cellTime":5280},"cycle":{"net":-18674,"wins":0,"paid":74326,"peak":199,"nodes":187,"reused":8,"kernels":1770,"grow":0,"shrink":0,"deleted":0,"cellTime":5280},"offGrow":{"net":-18674,"wins":0,"paid":74326,"peak":199,"nodes":187,"reused":8,"kernels":1770,"grow":0,"shrink":0,"deleted":0,"cellTime":5280},"offShrink":{"net":-18674,"wins":0,"paid":74326,"peak":199,"nodes":187,"reused":8,"kernels":1770,"grow":0,"shrink":0,"deleted":0,"cellTime":5280}},"k_change":{"one":{"net":26376,"wins":8,"paid":28424,"peak":199,"nodes":187,"reused":360,"kernels":432,"grow":0,"shrink":0,"deleted":0,"cellTime":5280},"two":{"net":24396,"wins":8,"paid":30404,"peak":201,"nodes":187,"reused":360,"kernels":432,"grow":0,"shrink":0,"deleted":0,"cellTime":6176},"grow":{"net":25536,"wins":8,"paid":29264,"peak":199,"nodes":187,"reused":360,"kernels":432,"grow":0,"shrink":0,"deleted":0,"cellTime":5280},"cycle":{"net":25536,"wins":8,"paid":29264,"peak":199,"nodes":187,"reused":360,"kernels":432,"grow":0,"shrink":0,"deleted":0,"cellTime":5280},"offGrow":{"net":25536,"wins":8,"paid":29264,"peak":199,"nodes":187,"reused":360,"kernels":432,"grow":0,"shrink":0,"deleted":0,"cellTime":5280},"offShrink":{"net":25536,"wins":8,"paid":29264,"peak":199,"nodes":187,"reused":360,"kernels":432,"grow":0,"shrink":0,"deleted":0,"cellTime":5280}},"mixed":{"one":{"net":-16320,"wins":0,"paid":72332,"peak":199,"nodes":187,"reused":0,"kernels":1800,"grow":0,"shrink":0,"deleted":0,"cellTime":5280},"two":{"net":-20640,"wins":0,"paid":76652,"peak":201,"nodes":187,"reused":0,"kernels":1800,"grow":0,"shrink":0,"deleted":0,"cellTime":6224},"grow":{"net":-19680,"wins":0,"paid":75692,"peak":199,"nodes":187,"reused":0,"kernels":1800,"grow":0,"shrink":0,"deleted":0,"cellTime":5280},"cycle":{"net":-19680,"wins":0,"paid":75692,"peak":199,"nodes":187,"reused":0,"kernels":1800,"grow":0,"shrink":0,"deleted":0,"cellTime":5280},"offGrow":{"net":-19680,"wins":0,"paid":75692,"peak":199,"nodes":187,"reused":0,"kernels":1800,"grow":0,"shrink":0,"deleted":0,"cellTime":5280},"offShrink":{"net":-19680,"wins":0,"paid":75692,"peak":199,"nodes":187,"reused":0,"kernels":1800,"grow":0,"shrink":0,"deleted":0,"cellTime":5280}},"alternating_to_drift":{"one":{"net":-19584,"wins":0,"paid":86990,"peak":199,"nodes":187,"reused":0,"kernels":2160,"grow":0,"shrink":0,"deleted":0,"cellTime":6336},"two":{"net":-1734,"wins":2,"paid":69140,"peak":201,"nodes":187,"reused":176,"kernels":1500,"grow":0,"shrink":0,"deleted":0,"cellTime":7472},"grow":{"net":-2834,"wins":2,"paid":70240,"peak":201,"nodes":187,"reused":168,"kernels":1530,"grow":8,"shrink":0,"deleted":0,"cellTime":7456},"cycle":{"net":-2722,"wins":2,"paid":70128,"peak":201,"nodes":187,"reused":168,"kernels":1530,"grow":8,"shrink":8,"deleted":16,"cellTime":6800},"offGrow":{"net":-24496,"wins":0,"paid":91902,"peak":199,"nodes":187,"reused":0,"kernels":2160,"grow":0,"shrink":0,"deleted":0,"cellTime":6336},"offShrink":{"net":-3282,"wins":2,"paid":70688,"peak":201,"nodes":187,"reused":168,"kernels":1530,"grow":8,"shrink":0,"deleted":0,"cellTime":7456}},"alternating_to_stable":{"one":{"net":26476,"wins":6,"paid":41740,"peak":199,"nodes":187,"reused":376,"kernels":750,"grow":0,"shrink":0,"deleted":0,"cellTime":6336},"two":{"net":48762,"wins":8,"paid":19454,"peak":201,"nodes":187,"reused":560,"kernels":60,"grow":0,"shrink":0,"deleted":0,"cellTime":7472},"grow":{"net":45742,"wins":8,"paid":22474,"peak":201,"nodes":187,"reused":552,"kernels":90,"grow":8,"shrink":0,"deleted":0,"cellTime":7456},"cycle":{"net":45742,"wins":8,"paid":22474,"peak":201,"nodes":187,"reused":552,"kernels":90,"grow":8,"shrink":0,"deleted":0,"cellTime":7456},"offGrow":{"net":24156,"wins":6,"paid":44060,"peak":199,"nodes":187,"reused":376,"kernels":750,"grow":0,"shrink":0,"deleted":0,"cellTime":6336},"offShrink":{"net":45742,"wins":8,"paid":22474,"peak":201,"nodes":187,"reused":552,"kernels":90,"grow":8,"shrink":0,"deleted":0,"cellTime":7456}},"return":{"one":{"net":-19584,"wins":0,"paid":87755,"peak":199,"nodes":187,"reused":0,"kernels":2160,"grow":0,"shrink":0,"deleted":0,"cellTime":6336},"two":{"net":21300,"wins":6,"paid":46871,"peak":201,"nodes":187,"reused":352,"kernels":840,"grow":0,"shrink":0,"deleted":0,"cellTime":7472},"grow":{"net":20200,"wins":6,"paid":47971,"peak":201,"nodes":187,"reused":344,"kernels":870,"grow":8,"shrink":0,"deleted":0,"cellTime":7456},"cycle":{"net":18828,"wins":6,"paid":49343,"peak":201,"nodes":187,"reused":336,"kernels":900,"grow":16,"shrink":8,"deleted":16,"cellTime":7152},"offGrow":{"net":-25376,"wins":0,"paid":93547,"peak":199,"nodes":187,"reused":0,"kernels":2160,"grow":0,"shrink":0,"deleted":0,"cellTime":6336},"offShrink":{"net":19752,"wins":6,"paid":48419,"peak":201,"nodes":187,"reused":344,"kernels":870,"grow":8,"shrink":0,"deleted":0,"cellTime":7456}},"stable_to_alternating":{"one":{"net":3936,"wins":4,"paid":64640,"peak":199,"nodes":187,"reused":192,"kernels":1440,"grow":0,"shrink":0,"deleted":0,"cellTime":6336},"two":{"net":48642,"wins":8,"paid":19934,"peak":201,"nodes":187,"reused":560,"kernels":60,"grow":0,"shrink":0,"deleted":0,"cellTime":7088},"grow":{"net":47542,"wins":8,"paid":21034,"peak":201,"nodes":187,"reused":552,"kernels":90,"grow":8,"shrink":0,"deleted":0,"cellTime":7072},"cycle":{"net":47542,"wins":8,"paid":21034,"peak":201,"nodes":187,"reused":552,"kernels":90,"grow":8,"shrink":0,"deleted":0,"cellTime":7072},"offGrow":{"net":-592,"wins":4,"paid":69168,"peak":199,"nodes":187,"reused":192,"kernels":1440,"grow":0,"shrink":0,"deleted":0,"cellTime":6336},"offShrink":{"net":47542,"wins":8,"paid":21034,"peak":201,"nodes":187,"reused":552,"kernels":90,"grow":8,"shrink":0,"deleted":0,"cellTime":7072}},"repeated":{"one":{"net":-29376,"wins":0,"paid":143856,"peak":199,"nodes":187,"reused":0,"kernels":3744,"grow":0,"shrink":0,"deleted":0,"cellTime":9504},"two":{"net":20796,"wins":4,"paid":93684,"peak":201,"nodes":187,"reused":396,"kernels":2028,"grow":0,"shrink":0,"deleted":0,"cellTime":11220},"grow":{"net":19880,"wins":4,"paid":94600,"peak":201,"nodes":187,"reused":390,"kernels":2054,"grow":6,"shrink":0,"deleted":0,"cellTime":11208},"cycle":{"net":17436,"wins":4,"paid":97044,"peak":201,"nodes":187,"reused":378,"kernels":2106,"grow":18,"shrink":18,"deleted":36,"cellTime":10548},"offGrow":{"net":-37404,"wins":0,"paid":151884,"peak":199,"nodes":187,"reused":0,"kernels":3744,"grow":0,"shrink":0,"deleted":0,"cellTime":9504},"offShrink":{"net":19544,"wins":4,"paid":94936,"peak":201,"nodes":187,"reused":390,"kernels":2054,"grow":6,"shrink":0,"deleted":0,"cellTime":11208}}},"regrowth":["shift_1_48_return","shift_1_96_return","shift_2_48_return","shift_2_96_return","shift_4_48_return","shift_4_96_return","shift_8_48_return","shift_8_96_return","repeated_1_96","repeated_1_192","repeated_4_96","repeated_4_192","repeated_8_96","repeated_8_192"],"worlds":714});
console.log(JSON.stringify({tier:"NON_NATIVE_REVERSIBLE_CACHE",summary,rows,scenarios,sources:Object.fromEntries(["shared","one","two","grow","cycle"].map(k=>[k,buildCycle(k)]))},null,2));
