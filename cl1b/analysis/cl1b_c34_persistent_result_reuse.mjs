#!/usr/bin/env node
// C34-R0: paid persistent cross-request exact-key result reuse.
import assert from "node:assert/strict";

function buildCache(policy="one"){
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



 label("prod_init");C(3,60);C(0,0);W(3,0);C(3,64,policy==="two"?"init2":"entry");label("init2");W(3,0,"entry");
 label("entry");C(0,0,policy==="shared"||policy==="plain"?"compute":"lookup1");
 const compare=(name,slot,yes,no)=>{label(name);
 for(const [i,key] of [[0,50],[1,51],[2,58]]){C(3,slot+i);R(0,3);C(3,key);R(1,3);S(0,0,1);B(0,no,i===2?yes:name+"_"+(i+1));if(i<2)label(name+"_"+(i+1));}};
 compare("lookup1",60,"hit1",policy==="two"?"lookup2":"compute");
 compare("lookup2",64,"hit2","compute");
 label("hit1");C(3,63);R(0,3,policy==="learn"?"learn_gate":"publish");
 label("hit2");C(3,67);R(0,3,"publish");
 // Learner initially performs a paid lookup but does not reuse; first exact hit authorizes the permanent entry.
 label("learn_gate");C(1,0,"activate_prepare");
 label("activate_prepare");pe("learn_gate","inactive");C(1,"@flag");C(0,1);label("activate");K(1,0,"flag");
 label("flag");C(0,0);B(0,"compile","inactive");
 label("compile");pe("learn_gate","cached","cached");
 label("cached");C(3,63);R(0,3,"publish");
 label("inactive");C(0,0,"compute");
 label("compute");C(3,50);R(1,3);C(3,51);R(2,3);C(3,43);W(3,1);C(3,44);W(3,2);C(3,41);W(3,2);C(3,58);R(0,3);C(3,42);W(3,0,"loop");
 label("loop");C(3,43);R(1,3);C(3,41);R(2,3,"kernel_dispatch");
 label("kernel_dispatch");C(0,0,"parent");
 label("parent");C(3,1);
 label("parent_add");A(0,0,2);S(1,1,3);B(1,"parent_add","out");
 label("base");C(3,0);A(1,1,3);C(3,"@special_entry");P(3,1);
 label("special_entry");C(0,0,"parent_add");
 label("out");C(3,44);R(1,3);A(0,0,1);C(3,41);W(3,0);C(3,42);R(1,3);C(2,1);S(1,1,2);W(3,1);B(1,"loop","done");
 label("done");C(3,41);R(0,3,policy==="plain"||policy==="shared"?"publish":"store_route");
 label("store_route");C(0,0,"store1");
 const store=(name,slot,next)=>{label(name);for(const [i,key] of [[3,41],[2,58],[1,44],[0,43]]){C(3,key);R(0,3);C(3,slot+i);W(3,0);}if(policy==="two"){pe("store_route",next,"reload");}else C(0,0,"reload");};
 store("store1",60,"store2");store("store2",64,"store1");
 label("reload");C(3,41);R(0,3,"publish");
 label("publish");C(3,11);W(3,0,"entry");
 for(let i=0;i<g.length;i++){const n=g[i];if(n.kind==="Branch"){n.nonzero=l[n.nonzero];n.zero=l[n.zero]}else n.next=n.next===undefined?i+1:l[n.next];if(n.kind==="Const"&&typeof n.value==="string")n.value=l[n.value.slice(1)];}
 return {g,l};
}
function depth(x){return x.toString(2).length-1+x.toString(2).replace(/0/g,"").length;}
function rec(a,b,k){let z=b;for(let i=0;i<k;i++)z=a*z+b;return z;}
function runCache(s,policy="one",off=false){
 const {g,l}=buildCache(policy),initial=g.length,d=new Map([[50,s.requests[0][0]],[51,s.requests[0][1]],[58,s.requests[0][2]]]),v=[0,0,0,0];
 let pc=0,paid=0,prod=0,peak=g.length+1+d.size,clones=0,kernels=0,serviceKernels=0,serviceClones=0,cloneSteps=0,expectedEntry=null,hit=null,stores=0,evictions=0,firstExposure=null,activation=0,lookupPaid=0,storePaid=0;const outputs=[],services=[],prefix=[],records=[null,null];
 const input=()=>[d.get(50),d.get(51),d.get(58)];
 while(paid<200000){
  const p=pc,n={...g[p]};if(!n.kind)throw Error("pc "+p);
  if(p===l.activate&&firstExposure===null)firstExposure=paid+1;
  if(firstExposure===null&&policy==="learn")prefix.push([p,[...v],[...d],g.length,n]);
  if(p===l.entry)hit=null;
  if(p===l.hit1||p===l.hit2)hit=p===l.hit1?0:1;
  if(p>=l.lookup1&&p<l.compute)lookupPaid++;
  if(p>=l.store_route&&p<l.publish)storePaid++;
  if(p===l.kernel_dispatch){const [a,b,k]=input();if(v[1]!==a||v[2]!==rec(a,b,serviceKernels)||serviceKernels>=k)throw Error("kernel input");expectedEntry=initial+a-1;serviceKernels++;kernels++;}
  if(p>=initial){serviceClones++;cloneSteps++;}
  paid++;pc=n.next;
  switch(n.kind){
   case "Const":v[n.dst]=n.value;break;
   case "Read":v[n.dst]=d.get(v[n.address])??0;break;
   case "Write":{
    const k=v[n.address],z=v[n.value];d.set(k,z);
    if(p>l.store1&&p<l.reload&&(k===60||k===64)){let slot=k===60?0:1;const q=input();if(d.get(k)!==q[0]||d.get(k+1)!==q[1]||d.get(k+2)!==q[2]||d.get(k+3)!==rec(...q)||serviceKernels!==q[2])throw Error("record");if(records[slot])evictions++;records[slot]=[...q,d.get(k+3)];stores++;}
    if(k===11){
     const q=input();if(z!==rec(...q))throw Error("output "+s.id+" "+prod+" "+z+" "+rec(...q));
     const reused=serviceKernels===0;if(reused){if(hit===null||!records[hit]||records[hit].slice(0,3).join()!==q.join()||records[hit][3]!==z)throw Error("stale");}else if(serviceKernels!==q[2])throw Error("kernel count");
     if(serviceClones!==(policy==="plain"?0:serviceKernels*depth(q[0])))throw Error("clone count");
     prod++;outputs.push(z);services.push({at:prod,paid,reused,hit,serviceKernels,serviceClones});serviceKernels=serviceClones=0;
     if(prod===s.requests.length)return {paid,outputs,services,clones,kernels,cloneSteps,nodes:g.length,peak,stores,evictions,firstExposure,activation,prefix,lookupPaid,storePaid};
     const next=s.requests[prod];d.set(50,next[0]);d.set(51,next[1]);d.set(58,next[2]);
    }break;}
   case "Add":v[n.dst]=v[n.lhs]+v[n.rhs];break;
   case "Sub":v[n.dst]=v[n.lhs]-v[n.rhs];break;
   case "Branch":pc=v[n.test]!==0?n.nonzero:n.zero;break;
   case "CloneNode":{const from=v[n.source],idx=g.length,x=idx-initial+1;if(from!==(x%2?l.parent_add:l.double_src))throw Error("template");g.push({...g[from]});v[n.dst]=idx;clones++;break;}
   case "PatchConst":{const at=v[n.node];if(at===l.flag){if(p!==l.activate||v[n.value]!==1||activation||!records[0]||records[0].slice(0,3).join()!==input().join())throw Error("activation");activation++;if(!off)g[at].value=1;}else{if(at!==l.base||g[at].kind!=="Const"||v[n.value]!==initial-1)throw Error("base");g[at].value=v[n.value];}break;}
   case "PatchNext":{
    const at=v[n.node],to=v[n.target];if(!g[to]||g[at].next===undefined)throw Error("edge");
    if(at>=initial){const x=at-initial+1;if(to!==(x===1?l.out:x%2?at-1:initial+x/2-1))throw Error("generated edge");}
    else if(at===l.special_entry){if(to!==expectedEntry)throw Error("entry");}
    else if(at===l.kernel_dispatch){if(to!==l.base)throw Error("kernel");}
    else if(at===l.store_route){if(policy!=="two"||to!== (g[at].next===l.store1?l.store2:l.store1))throw Error("fifo");}
    else if(at===l.learn_gate){if(![l.inactive,l.cached].includes(to)||to===l.cached&&(off||!activation))throw Error("compile");}
    else throw Error("target");
    if(!(off&&p===l.activate))g[at].next=to;break;}
   default:throw Error(n.kind);
  }
  if(!v.every(Number.isSafeInteger))throw Error("integer");
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
 const arms={plain:runCache(s,"plain"),shared:runCache(s,"shared"),one:runCache(s),two:runCache(s,"two"),learn:runCache(s,"learn"),off:runCache(s,"learn",true)};
 for(const q of Object.values(arms))if(q.outputs.join()!==arms.shared.outputs.join())throw Error("outputs");
 if(JSON.stringify(arms.learn.prefix)!==JSON.stringify(arms.off.prefix)||arms.learn.firstExposure!==arms.off.firstExposure||arms.learn.activation!==arms.off.activation)throw Error("prefix");
 if(arms.off.services.some(x=>x.reused))throw Error("off reuse");
 const compact=q=>({paid:q.paid,kernels:q.kernels,nodes:q.nodes,peak:q.peak,stores:q.stores,evictions:q.evictions,reused:q.services.filter(x=>x.reused).length,activation:q.activation,lookupPaid:q.lookupPaid,storePaid:q.storePaid});
 return {id:s.id,k:s.k,n:s.n,pattern:s.pattern,...Object.fromEntries(Object.entries(arms).map(([k,q])=>[k,compact(q)]))};
});
function agg(a){return Object.fromEntries(["shared","one","two","learn","off"].map(k=>[k,{net:a.reduce((s,r)=>s+r.plain.paid-r[k].paid,0),vsShared:a.reduce((s,r)=>s+r.shared.paid-r[k].paid,0),wins:a.filter(r=>r.shared.paid>r[k].paid).length,paid:a.reduce((s,r)=>s+r[k].paid,0),peak:Math.max(...a.map(r=>r[k].peak)),nodes:Math.max(...a.map(r=>r[k].nodes)),reused:a.reduce((s,r)=>s+r[k].reused,0),kernels:a.reduce((s,r)=>s+r[k].kernels,0),stores:a.reduce((s,r)=>s+r[k].stores,0),evictions:a.reduce((s,r)=>s+r[k].evictions,0),activation:a.reduce((s,r)=>s+r[k].activation,0),lookupPaid:a.reduce((s,r)=>s+r[k].lookupPaid,0),storePaid:a.reduce((s,r)=>s+r[k].storePaid,0)}]))}
const summary={all:agg(rows),scales:Object.fromEntries([1,2,4,8].map(k=>[k,agg(rows.filter(r=>r.k===k))])),patterns:Object.fromEntries(patterns.map(k=>[k,agg(rows.filter(r=>r.pattern===k))])),worlds:rows.length*6};

assert.deepEqual(summary,{"all":{"shared":{"net":233803,"vsShared":0,"wins":0,"paid":452251,"peak":195,"nodes":185,"reused":0,"kernels":14328,"stores":0,"evictions":0,"activation":0,"lookupPaid":0,"storePaid":0},"one":{"net":282709,"vsShared":48906,"wins":24,"paid":403345,"peak":198,"nodes":185,"reused":1260,"kernels":9657,"stores":2580,"evictions":2516,"activation":0,"lookupPaid":42120,"storePaid":51600},"two":{"net":314811,"vsShared":81008,"wins":32,"paid":371243,"peak":206,"nodes":189,"reused":1724,"kernels":7917,"stores":2116,"evictions":1996,"activation":0,"lookupPaid":68536,"storePaid":46552},"learn":{"net":278577,"vsShared":44774,"wins":24,"paid":407477,"peak":198,"nodes":185,"reused":1260,"kernels":9657,"stores":2580,"evictions":2516,"activation":32,"lookupPaid":46252,"storePaid":51600},"off":{"net":112107,"vsShared":-121696,"wins":0,"paid":573947,"peak":198,"nodes":185,"reused":0,"kernels":14328,"stores":3840,"evictions":3776,"activation":32,"lookupPaid":44896,"storePaid":76800}},"scales":{"1":{"shared":{"net":18309,"vsShared":0,"wins":0,"paid":52901,"peak":195,"nodes":185,"reused":0,"kernels":1272,"stores":0,"evictions":0,"activation":0,"lookupPaid":0,"storePaid":0},"one":{"net":14261,"vsShared":-4048,"wins":6,"paid":56949,"peak":198,"nodes":185,"reused":315,"kernels":723,"stores":645,"evictions":629,"activation":0,"lookupPaid":10530,"storePaid":12900},"two":{"net":13833,"vsShared":-4476,"wins":8,"paid":57377,"peak":206,"nodes":189,"reused":431,"kernels":607,"stores":529,"evictions":499,"activation":0,"lookupPaid":17134,"storePaid":11638},"learn":{"net":13228,"vsShared":-5081,"wins":6,"paid":57982,"peak":198,"nodes":185,"reused":315,"kernels":723,"stores":645,"evictions":629,"activation":8,"lookupPaid":11563,"storePaid":12900},"off":{"net":-12115,"vsShared":-30424,"wins":0,"paid":83325,"peak":198,"nodes":185,"reused":0,"kernels":1272,"stores":960,"evictions":944,"activation":8,"lookupPaid":11224,"storePaid":19200}},"2":{"shared":{"net":32906,"vsShared":0,"wins":0,"paid":74778,"peak":195,"nodes":185,"reused":0,"kernels":2112,"stores":0,"evictions":0,"activation":0,"lookupPaid":0,"storePaid":0},"one":{"net":34776,"vsShared":1870,"wins":6,"paid":72908,"peak":198,"nodes":185,"reused":315,"kernels":1338,"stores":645,"evictions":629,"activation":0,"lookupPaid":10530,"storePaid":12900},"two":{"net":37422,"vsShared":4516,"wins":8,"paid":70262,"peak":206,"nodes":189,"reused":431,"kernels":1106,"stores":529,"evictions":499,"activation":0,"lookupPaid":17134,"storePaid":11638},"learn":{"net":33743,"vsShared":837,"wins":6,"paid":73941,"peak":198,"nodes":185,"reused":315,"kernels":1338,"stores":645,"evictions":629,"activation":8,"lookupPaid":11563,"storePaid":12900},"off":{"net":2482,"vsShared":-30424,"wins":0,"paid":105202,"peak":198,"nodes":185,"reused":0,"kernels":2112,"stores":960,"evictions":944,"activation":8,"lookupPaid":11224,"storePaid":19200}},"4":{"shared":{"net":62100,"vsShared":0,"wins":0,"paid":118532,"peak":195,"nodes":185,"reused":0,"kernels":3792,"stores":0,"evictions":0,"activation":0,"lookupPaid":0,"storePaid":0},"one":{"net":75806,"vsShared":13706,"wins":6,"paid":104826,"peak":198,"nodes":185,"reused":315,"kernels":2568,"stores":645,"evictions":629,"activation":0,"lookupPaid":10530,"storePaid":12900},"two":{"net":84600,"vsShared":22500,"wins":8,"paid":96032,"peak":206,"nodes":189,"reused":431,"kernels":2104,"stores":529,"evictions":499,"activation":0,"lookupPaid":17134,"storePaid":11638},"learn":{"net":74773,"vsShared":12673,"wins":6,"paid":105859,"peak":198,"nodes":185,"reused":315,"kernels":2568,"stores":645,"evictions":629,"activation":8,"lookupPaid":11563,"storePaid":12900},"off":{"net":31676,"vsShared":-30424,"wins":0,"paid":148956,"peak":198,"nodes":185,"reused":0,"kernels":3792,"stores":960,"evictions":944,"activation":8,"lookupPaid":11224,"storePaid":19200}},"8":{"shared":{"net":120488,"vsShared":0,"wins":0,"paid":206040,"peak":195,"nodes":185,"reused":0,"kernels":7152,"stores":0,"evictions":0,"activation":0,"lookupPaid":0,"storePaid":0},"one":{"net":157866,"vsShared":37378,"wins":6,"paid":168662,"peak":198,"nodes":185,"reused":315,"kernels":5028,"stores":645,"evictions":629,"activation":0,"lookupPaid":10530,"storePaid":12900},"two":{"net":178956,"vsShared":58468,"wins":8,"paid":147572,"peak":206,"nodes":189,"reused":431,"kernels":4100,"stores":529,"evictions":499,"activation":0,"lookupPaid":17134,"storePaid":11638},"learn":{"net":156833,"vsShared":36345,"wins":6,"paid":169695,"peak":198,"nodes":185,"reused":315,"kernels":5028,"stores":645,"evictions":629,"activation":8,"lookupPaid":11563,"storePaid":12900},"off":{"net":90064,"vsShared":-30424,"wins":0,"paid":236464,"peak":198,"nodes":185,"reused":0,"kernels":7152,"stores":960,"evictions":944,"activation":8,"lookupPaid":11224,"storePaid":19200}}},"patterns":{"stable":{"shared":{"net":20864,"vsShared":0,"wins":0,"paid":56696,"peak":195,"nodes":185,"reused":0,"kernels":1800,"stores":0,"evictions":0,"activation":0,"lookupPaid":0,"storePaid":0},"one":{"net":64788,"vsShared":43924,"wins":8,"paid":12772,"peak":198,"nodes":185,"reused":472,"kernels":30,"stores":8,"evictions":0,"activation":0,"lookupPaid":9488,"storePaid":160},"two":{"net":64716,"vsShared":43852,"wins":8,"paid":12844,"peak":203,"nodes":189,"reused":472,"kernels":30,"stores":8,"evictions":0,"activation":0,"lookupPaid":9536,"storePaid":176},"learn":{"net":63284,"vsShared":42420,"wins":8,"paid":14276,"peak":198,"nodes":185,"reused":472,"kernels":30,"stores":8,"evictions":0,"activation":8,"lookupPaid":10992,"storePaid":160},"off":{"net":768,"vsShared":-20096,"wins":0,"paid":76792,"peak":198,"nodes":185,"reused":0,"kernels":1800,"stores":480,"evictions":472,"activation":8,"lookupPaid":10496,"storePaid":9600}},"alternating":{"shared":{"net":30764,"vsShared":0,"wins":0,"paid":57596,"peak":195,"nodes":185,"reused":0,"kernels":1800,"stores":0,"evictions":0,"activation":0,"lookupPaid":0,"storePaid":0},"one":{"net":18284,"vsShared":-12480,"wins":0,"paid":70076,"peak":198,"nodes":185,"reused":0,"kernels":1800,"stores":480,"evictions":472,"activation":0,"lookupPaid":2880,"storePaid":9600},"two":{"net":73074,"vsShared":42310,"wins":8,"paid":15286,"peak":206,"nodes":189,"reused":464,"kernels":60,"stores":16,"evictions":0,"activation":0,"lookupPaid":10864,"storePaid":352},"learn":{"net":18284,"vsShared":-12480,"wins":0,"paid":70076,"peak":198,"nodes":185,"reused":0,"kernels":1800,"stores":480,"evictions":472,"activation":0,"lookupPaid":2880,"storePaid":9600},"off":{"net":18284,"vsShared":-12480,"wins":0,"paid":70076,"peak":198,"nodes":185,"reused":0,"kernels":1800,"stores":480,"evictions":472,"activation":0,"lookupPaid":2880,"storePaid":9600}},"cycle3":{"shared":{"net":37664,"vsShared":0,"wins":0,"paid":57896,"peak":195,"nodes":185,"reused":0,"kernels":1800,"stores":0,"evictions":0,"activation":0,"lookupPaid":0,"storePaid":0},"one":{"net":25184,"vsShared":-12480,"wins":0,"paid":70376,"peak":198,"nodes":185,"reused":0,"kernels":1800,"stores":480,"evictions":472,"activation":0,"lookupPaid":2880,"storePaid":9600},"two":{"net":21336,"vsShared":-16328,"wins":0,"paid":74224,"peak":206,"nodes":189,"reused":0,"kernels":1800,"stores":480,"evictions":464,"activation":0,"lookupPaid":5760,"storePaid":10560},"learn":{"net":25184,"vsShared":-12480,"wins":0,"paid":70376,"peak":198,"nodes":185,"reused":0,"kernels":1800,"stores":480,"evictions":472,"activation":0,"lookupPaid":2880,"storePaid":9600},"off":{"net":25184,"vsShared":-12480,"wins":0,"paid":70376,"peak":198,"nodes":185,"reused":0,"kernels":1800,"stores":480,"evictions":472,"activation":0,"lookupPaid":2880,"storePaid":9600}},"burst":{"shared":{"net":37664,"vsShared":0,"wins":0,"paid":57896,"peak":195,"nodes":185,"reused":0,"kernels":1800,"stores":0,"evictions":0,"activation":0,"lookupPaid":0,"storePaid":0},"one":{"net":76424,"vsShared":38760,"wins":8,"paid":19136,"peak":198,"nodes":185,"reused":420,"kernels":225,"stores":60,"evictions":52,"activation":0,"lookupPaid":8760,"storePaid":1200},"two":{"net":74760,"vsShared":37096,"wins":8,"paid":20800,"peak":206,"nodes":189,"reused":420,"kernels":225,"stores":60,"evictions":44,"activation":0,"lookupPaid":10296,"storePaid":1320},"learn":{"net":75076,"vsShared":37412,"wins":8,"paid":20484,"peak":198,"nodes":185,"reused":420,"kernels":225,"stores":60,"evictions":52,"activation":8,"lookupPaid":10108,"storePaid":1200},"off":{"net":18400,"vsShared":-19264,"wins":0,"paid":77160,"peak":198,"nodes":185,"reused":0,"kernels":1800,"stores":480,"evictions":472,"activation":8,"lookupPaid":9664,"storePaid":9600}},"drift":{"shared":{"net":29669,"vsShared":0,"wins":0,"paid":55631,"peak":195,"nodes":185,"reused":0,"kernels":1800,"stores":0,"evictions":0,"activation":0,"lookupPaid":0,"storePaid":0},"one":{"net":17189,"vsShared":-12480,"wins":0,"paid":68111,"peak":198,"nodes":185,"reused":0,"kernels":1800,"stores":480,"evictions":472,"activation":0,"lookupPaid":2880,"storePaid":9600},"two":{"net":13341,"vsShared":-16328,"wins":0,"paid":71959,"peak":206,"nodes":189,"reused":0,"kernels":1800,"stores":480,"evictions":464,"activation":0,"lookupPaid":5760,"storePaid":10560},"learn":{"net":17189,"vsShared":-12480,"wins":0,"paid":68111,"peak":198,"nodes":185,"reused":0,"kernels":1800,"stores":480,"evictions":472,"activation":0,"lookupPaid":2880,"storePaid":9600},"off":{"net":17189,"vsShared":-12480,"wins":0,"paid":68111,"peak":198,"nodes":185,"reused":0,"kernels":1800,"stores":480,"evictions":472,"activation":0,"lookupPaid":2880,"storePaid":9600}},"rare":{"shared":{"net":30074,"vsShared":0,"wins":0,"paid":55676,"peak":195,"nodes":185,"reused":0,"kernels":1800,"stores":0,"evictions":0,"activation":0,"lookupPaid":0,"storePaid":0},"one":{"net":18520,"vsShared":-11554,"wins":0,"paid":67230,"peak":198,"nodes":185,"reused":8,"kernels":1770,"stores":472,"evictions":464,"activation":0,"lookupPaid":2992,"storePaid":9440},"two":{"net":14688,"vsShared":-15386,"wins":0,"paid":71062,"peak":206,"nodes":189,"reused":8,"kernels":1770,"stores":472,"evictions":456,"activation":0,"lookupPaid":5872,"storePaid":10384},"learn":{"net":18408,"vsShared":-11666,"wins":0,"paid":67342,"peak":198,"nodes":185,"reused":8,"kernels":1770,"stores":472,"evictions":464,"activation":8,"lookupPaid":3104,"storePaid":9440},"off":{"net":17402,"vsShared":-12672,"wins":0,"paid":68348,"peak":198,"nodes":185,"reused":0,"kernels":1800,"stores":480,"evictions":472,"activation":8,"lookupPaid":3072,"storePaid":9600}},"k_change":{"shared":{"net":20000,"vsShared":0,"wins":0,"paid":54824,"peak":195,"nodes":185,"reused":0,"kernels":1728,"stores":0,"evictions":0,"activation":0,"lookupPaid":0,"storePaid":0},"one":{"net":47792,"vsShared":27792,"wins":8,"paid":27032,"peak":198,"nodes":185,"reused":360,"kernels":432,"stores":120,"evictions":112,"activation":0,"lookupPaid":9264,"storePaid":2400},"two":{"net":42336,"vsShared":22336,"wins":8,"paid":32488,"peak":206,"nodes":189,"reused":360,"kernels":432,"stores":120,"evictions":104,"activation":0,"lookupPaid":14472,"storePaid":2640},"learn":{"net":46624,"vsShared":26624,"wins":8,"paid":28200,"peak":198,"nodes":185,"reused":360,"kernels":432,"stores":120,"evictions":112,"activation":8,"lookupPaid":10432,"storePaid":2400},"off":{"net":352,"vsShared":-19648,"wins":0,"paid":74472,"peak":198,"nodes":185,"reused":0,"kernels":1728,"stores":480,"evictions":472,"activation":8,"lookupPaid":10048,"storePaid":9600}},"mixed":{"shared":{"net":27104,"vsShared":0,"wins":0,"paid":56036,"peak":195,"nodes":185,"reused":0,"kernels":1800,"stores":0,"evictions":0,"activation":0,"lookupPaid":0,"storePaid":0},"one":{"net":14528,"vsShared":-12576,"wins":0,"paid":68612,"peak":198,"nodes":185,"reused":0,"kernels":1800,"stores":480,"evictions":472,"activation":0,"lookupPaid":2976,"storePaid":9600},"two":{"net":10560,"vsShared":-16544,"wins":0,"paid":72580,"peak":206,"nodes":189,"reused":0,"kernels":1800,"stores":480,"evictions":464,"activation":0,"lookupPaid":5976,"storePaid":10560},"learn":{"net":14528,"vsShared":-12576,"wins":0,"paid":68612,"peak":198,"nodes":185,"reused":0,"kernels":1800,"stores":480,"evictions":472,"activation":0,"lookupPaid":2976,"storePaid":9600},"off":{"net":14528,"vsShared":-12576,"wins":0,"paid":68612,"peak":198,"nodes":185,"reused":0,"kernels":1800,"stores":480,"evictions":472,"activation":0,"lookupPaid":2976,"storePaid":9600}}},"worlds":384});
console.log(JSON.stringify({tier:"NON_NATIVE_PERSISTENT_RESULT_REUSE",summary,rows,scenarios,sources:Object.fromEntries(["plain","shared","one","two","learn"].map(k=>[k,buildCache(k)]))},null,2));
