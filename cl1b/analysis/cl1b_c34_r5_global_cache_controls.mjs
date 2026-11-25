#!/usr/bin/env node
// C34-R5: global paid FIFO/LRU and global two-observation admission controls.
import assert from "node:assert/strict";
function buildCompete(policy="adaptive"){
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
 // key=(32a+b)*32+2k+stream; exact, positive, disjoint namespaces.
 label("encode");C(3,50);R(0,3);for(let i=0;i<5;i++)A(0,0,0);C(3,51);R(1,3);A(0,0,1);for(let i=0;i<4;i++)A(0,0,0);C(3,58);R(1,3);A(0,0,1);A(0,0,0);C(3,57);R(1,3);A(0,0,1);C(3,59);W(3,0);B(1,"secondary","primary");
 label("primary");C(3,60);R(1,3);S(2,0,1);B(2,"p_miss","hit1");
 label("hit1");C(3,61);R(0,3,"publish");
 label("p_miss");C(1,0,policy==="split"?"p_default":"p_lookup2");
 label("p_lookup2");C(3,64);R(1,3);S(2,0,1);B(2,policy==="primary2"?"p_fifo":"p_ghost","hit2");
 label("secondary");C(1,0,policy==="primary2"?"s_default":"s_lookup2");
 label("s_lookup2");C(3,64);R(1,3);S(2,0,1);B(2,policy==="split"?"allocate":"s_ghost","hit2");
 label("hit2");C(3,65);R(0,3,"publish");
 label("p_ghost");C(1,0);S(2,0,1);B(2,"p_remember","admit");
 label("p_remember");C(3,60);R(0,3);C(1,"@p_ghost");K(1,0,"p_default");
 label("s_ghost");C(1,0);S(2,0,1);B(2,"s_remember","admit");
 label("s_remember");C(1,"@s_ghost");K(1,0,"s_default");
 label("admit");C(1,"@flag");C(0,1);label("activate");K(1,0);
 label("flag");C(0,0);B(0,"allocate","denied");
 label("denied");C(3,57);R(0,3);B(0,"s_default","p_remember");
 label("allocate");for(const key of [64,65]){C(3,key);op("Delete",{address:3});}pk("slot",64,"compute");
 label("p_default");pk("slot",60,"compute");
 label("s_default");pk("slot",0,"compute");
 label("p_fifo");C(0,60);C(1,"@slot");K(1,0);C(1,124);S(0,1,0);C(1,"@p_fifo");K(1,0,"compute");
 label("compute");C(3,50);R(1,3);C(3,51);R(2,3);C(3,43);W(3,1);C(3,44);W(3,2);C(3,57);R(0,3);B(0,"zero_seed","seed");
 label("zero_seed");C(2,0,"seed");
 label("seed");C(3,41);W(3,2);C(3,58);R(0,3);C(3,42);W(3,0,"loop");
 label("loop");C(3,43);R(1,3);C(3,41);R(2,3,"kernel_dispatch");
 label("kernel_dispatch");C(0,0,"parent");
 label("parent");C(3,1);
 label("parent_add");A(0,0,2);S(1,1,3);B(1,"parent_add","out");
 label("base");C(3,0);A(1,1,3);C(3,"@special_entry");P(3,1);
 label("special_entry");C(0,0,"parent_add");
 label("out");C(3,44);R(1,3);A(0,0,1);C(3,41);W(3,0);C(3,42);R(1,3);C(2,1);S(1,1,2);W(3,1);B(1,"loop","done");
 label("done");C(3,41);R(0,3,policy==="shared"?"publish":"slot");
 label("slot");C(2,0);B(2,"store","publish");
 label("store");C(1,1);A(3,2,1);W(3,0);C(3,59);R(0,3);W(2,0);C(3,41);R(0,3,"publish");
 label("publish");C(3,11);W(3,0,"entry");
 for(let i=0;i<g.length;i++){const n=g[i];if(n.kind==="Branch"){n.nonzero=l[n.nonzero];n.zero=l[n.zero]}else n.next=n.next===undefined?i+1:l[n.next];if(n.kind==="Const"&&typeof n.value==="string")n.value=l[n.value.slice(1)];}
 return {g,l};
}
function rec(a,b,k,t){let z=t?0:b;for(let i=0;i<k;i++)z=a*z+b;return z;}
function key(a,b,k,t){return (32*a+b)*32+2*k+t;}
function depth(x){return x.toString(2).length-1+x.toString(2).replace(/0/g,"").length;}
function run(s,policy="adaptive",off=false){
 const {g,l}=buildCompete(policy),initial=g.length,d=new Map(),v=[0,0,0,0];const load=q=>[50,51,58,57].forEach((k,i)=>d.set(k,q[i]));load(s.requests[0]);
 let pc=0,paid=0,prod=0,kernels=0,sk=0,sc=0,clones=0,peak=initial+1+d.size,entry=null,hit=null,exposure=null,deleted=0,cellTime=0,stores=0,transfers=0,oldOwner=null;const outputs=[],services=[],events=[],prefix=[],records=[null,null];
 const input=()=>[50,51,58,57].map(k=>d.get(k));
 while(paid<200000){
 const p=pc,n={...g[p]};if(!n.kind)throw Error("pc");if(p===l.activate&&exposure===null)exposure=paid+1;
 if(exposure===null&&policy==="adaptive")prefix.push([p,[...v],[...d],g.length,n]);
 if(p===l.entry){hit=null;oldOwner=null;}
 if(p===l.hit1||p===l.hit2)hit=p===l.hit1?0:1;
 if(p===l.kernel_dispatch){const [a,b,k,t]=input();if(v[1]!==a||v[2]!==rec(a,b,sk,t)||sk>=k)throw Error("kernel");entry=initial+a-1;sk++;kernels++;}
 if(p>=initial)sc++;paid++;pc=n.next;
 switch(n.kind){
 case "Const":v[n.dst]=n.value;break;
 case "Read":v[n.dst]=d.get(v[n.address])??0;break;
 case "Write":{const k=v[n.address],z=v[n.value];d.set(k,z);const q=input();
 if(k===59&&z!==key(...q))throw Error("key");
 if(k===60||k===64){const slot=k===60?0:1;if(z!==key(...q)||d.get(k+1)!==rec(...q)||sk!==q[2]||slot===0&&q[3])throw Error("record");
 if(slot===1&&oldOwner!==null&&oldOwner!==q[3]){transfers++;if(policy==="adaptive"&&!events.some(e=>e.at===prod&&!e.suppressed))throw Error("unpaid transfer");}
 records[slot]=[...q,rec(...q)];stores++;}
 if(k===11){if(z!==rec(...q))throw Error("output "+s.id+" "+prod+" "+policy);const reused=sk===0;if(reused){if(hit===null||!records[hit]||records[hit].slice(0,4).join()!==q.join()||records[hit][4]!==z)throw Error("stale");}else if(sk!==q[2])throw Error("count");if(sc!==sk*depth(q[0]))throw Error("clone count");
 outputs.push(z);prod++;cellTime+=d.size;services.push({at:prod,paid,reused,stream:q[3],cells:d.size,owner:records[1]?.[3]??null});sk=sc=0;
 if(prod===s.requests.length)return {paid,outputs,services,events,exposure,prefix,peak,nodes:g.length,kernels,clones,stores,deleted,transfers,cellTime};load(s.requests[prod]);}break;}
 case "Delete":{const k=v[n.address];if(![64,65].includes(k))throw Error("delete");if(k===64)oldOwner=records[1]?.[3]??null;if(d.delete(k))deleted++;if(k===65)records[1]=null;break;}
 case "Add":v[n.dst]=v[n.lhs]+v[n.rhs];break;
 case "Sub":v[n.dst]=v[n.lhs]-v[n.rhs];break;
 case "Branch":pc=v[n.test]!==0?n.nonzero:n.zero;break;
 case "CloneNode":{const from=v[n.source],idx=g.length,x=idx-initial+1;if(from!==(x%2?l.parent_add:l.double_src))throw Error("clone");g.push({...g[from]});v[n.dst]=idx;clones++;break;}
 case "PatchConst":{const at=v[n.node],to=v[n.value];if(at===l.flag){const q=input();if(p!==l.activate||to!==1||g[q[3]?l.s_ghost:l.p_ghost].value!==key(...q))throw Error("admit");events.push({at:prod,stream:q[3],owner:records[1]?.[3]??null,paid,suppressed:off});if(!off)g[at].value=1;}
 else if(at===l.slot){if(![0,60,64].includes(to))throw Error("slot");g[at].value=to;}
 else if(at===l.p_fifo){if(![60,64].includes(to))throw Error("fifo");g[at].value=to;}
 else if(at===l.p_ghost){if(to!==(d.get(60)??0))throw Error("ghost p");g[at].value=to;}
 else if(at===l.s_ghost){if(to!==key(...input())||!input()[3])throw Error("ghost s");g[at].value=to;}
 else{if(at!==l.base||to!==initial-1)throw Error("base");g[at].value=to;}break;}
 case "PatchNext":{const at=v[n.node],to=v[n.target];if(!g[to]||g[at].next===undefined)throw Error("edge");if(at>=initial){const x=at-initial+1;if(to!==(x===1?l.out:x%2?at-1:initial+x/2-1))throw Error("generated");}else if(at===l.special_entry){if(to!==entry)throw Error("entry");}else if(at===l.kernel_dispatch){if(to!==l.base)throw Error("kernel");}else throw Error("patch");g[at].next=to;break;}
 default:throw Error(n.kind);
 }
 peak=Math.max(peak,g.length+1+d.size);if(g.length>200||d.size>14||peak>220||!v.every(Number.isSafeInteger))throw Error("capacity "+g.length+" "+d.size+" "+peak);
 }throw Error("budget");
}


function buildGlobal(policy="fifo"){
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
 const ghost=policy.startsWith("ghost"),lru=policy.endsWith("lru");
 label("prod_init");C(0,0,"entry");
 label("entry");C(0,0,"encode");
 label("encode");C(3,50);R(0,3);for(let i=0;i<5;i++)A(0,0,0);C(3,51);R(1,3);A(0,0,1);for(let i=0;i<4;i++)A(0,0,0);C(3,58);R(1,3);A(0,0,1);A(0,0,0);C(3,57);R(1,3);A(0,0,1);C(3,59);W(3,0);
 label("lookup1");C(3,60);R(1,3);S(2,0,1);B(2,"lookup2","hit1");
 label("lookup2");C(3,64);R(1,3);S(2,0,1);B(2,ghost?"ghost":"compute","hit2");
 label("hit1");C(0,0,lru?"recent1":"result1");label("recent1");pk("slot",64,"result1");
 label("result1");C(3,61);R(0,3,"publish");
 label("hit2");C(0,0,lru?"recent2":"result2");label("recent2");pk("slot",60,"result2");
 label("result2");C(3,65);R(0,3,"publish");
 label("ghost");C(1,0);S(2,0,1);B(2,"remember","admit");
 label("remember");C(3,59);R(0,3);C(1,"@ghost");K(1,0);pe("store_gate","publish","compute");
 label("admit");C(1,"@flag");C(0,1);label("activate");K(1,0);
 label("flag");C(0,0);B(0,"allow","remember");
 label("allow");pe("store_gate","slot","compute");
 label("compute");C(3,50);R(1,3);C(3,51);R(2,3);C(3,43);W(3,1);C(3,44);W(3,2);C(3,57);R(0,3);B(0,"zero_seed","seed");
 label("zero_seed");C(2,0,"seed");
 label("seed");C(3,41);W(3,2);C(3,58);R(0,3);C(3,42);W(3,0,"loop");
 label("loop");C(3,43);R(1,3);C(3,41);R(2,3,"kernel_dispatch");
 label("kernel_dispatch");C(0,0,"parent");
 label("parent");C(3,1);
 label("parent_add");A(0,0,2);S(1,1,3);B(1,"parent_add","out");
 label("base");C(3,0);A(1,1,3);C(3,"@special_entry");P(3,1);
 label("special_entry");C(0,0,"parent_add");
 label("out");C(3,44);R(1,3);A(0,0,1);C(3,41);W(3,0);C(3,42);R(1,3);C(2,1);S(1,1,2);W(3,1);B(1,"loop","done");

 label("done");C(3,41);R(0,3,"store_gate");
 label("store_gate");C(1,0,ghost?"publish":"slot");
 label("slot");C(2,60);C(1,1);A(3,2,1);W(3,0);C(3,59);R(0,3);W(2,0);
 label("rotate");C(1,124);S(0,1,2);C(1,"@slot");K(1,0);
 label("reload");C(3,41);R(0,3,"publish");
 label("publish");C(3,11);W(3,0,"entry");
 for(let i=0;i<g.length;i++){const n=g[i];if(n.kind==="Branch"){n.nonzero=l[n.nonzero];n.zero=l[n.zero]}else n.next=n.next===undefined?i+1:l[n.next];if(n.kind==="Const"&&typeof n.value==="string")n.value=l[n.value.slice(1)];}
 return {g,l};
}
function runGlobal(s,policy="fifo",off=false){
 const {g,l}=buildGlobal(policy),initial=g.length,d=new Map(),v=[0,0,0,0];const load=q=>[50,51,58,57].forEach((k,i)=>d.set(k,q[i]));load(s.requests[0]);
 const ghost=policy.startsWith("ghost"),lru=policy.endsWith("lru");
 let pc=0,paid=0,prod=0,kernels=0,sk=0,sc=0,clones=0,peak=initial+1+d.size,entry=null,hit=null,exposure=null,stores=0,transfers=0,cellTime=0,expectedHit=null,expectedStore=null,expectedVictim=60,ghostKey=0,actualStore=null;const outputs=[],services=[],events=[],prefix=[],records=[null,null];
 const input=()=>[50,51,58,57].map(k=>d.get(k));
 while(paid<200000){
 const p=pc,n={...g[p]};if(!n.kind)throw Error("pc");
 if(p===l.activate&&exposure===null)exposure=paid+1;
 if(exposure===null&&ghost)prefix.push([p,[...v],[...d],g.length,n]);
 if(p===l.entry){
  hit=null;actualStore=null;const q=input(),k=key(...q);
  const found=records.findIndex(x=>x&&x.slice(0,4).join()===q.join());expectedHit=found<0?null:found;
  expectedStore=expectedHit===null&&(!ghost||(!off&&ghostKey===k))?expectedVictim:null;
  if(expectedHit===null&&ghost&&(ghostKey!==k||off))ghostKey=k;
 }
 if(p===l.hit1||p===l.hit2)hit=p===l.hit1?0:1;
 if(p===l.kernel_dispatch){const [a,b,k,t]=input();if(v[1]!==a||v[2]!==rec(a,b,sk,t)||sk>=k)throw Error("kernel");entry=initial+a-1;sk++;kernels++;}
 if(p>=initial)sc++;paid++;pc=n.next;
 switch(n.kind){
 case "Const":v[n.dst]=n.value;break;
 case "Read":v[n.dst]=d.get(v[n.address])??0;break;
 case "Write":{const k=v[n.address],z=v[n.value];d.set(k,z);const q=input();
 if(k===59&&z!==key(...q))throw Error("key");
 if(k===60||k===64){const slot=k===60?0:1;if(k!==expectedStore||actualStore!==null||z!==key(...q)||d.get(k+1)!==rec(...q)||sk!==q[2])throw Error("record/victim");
 if(records[slot]&&records[slot][3]!==q[3])transfers++;records[slot]=[...q,rec(...q)];stores++;actualStore=k;expectedVictim=124-k;}
 if(k===11){if(z!==rec(...q))throw Error("output "+s.id+" "+prod+" "+policy);
 const reused=sk===0;if(reused!==(expectedHit!==null)||hit!==expectedHit||actualStore!==expectedStore)throw Error("policy");
 if(reused){if(!records[hit]||records[hit].slice(0,4).join()!==q.join()||records[hit][4]!==z)throw Error("stale");if(lru)expectedVictim=hit===0?64:60;}
 else if(sk!==q[2])throw Error("count");if(sc!==sk*depth(q[0]))throw Error("clone count");
 if(g[l.slot].value!==expectedVictim||ghost&&g[l.ghost].value!==ghostKey)throw Error("policy state");
 outputs.push(z);prod++;cellTime+=d.size;services.push({at:prod,paid,reused,stream:q[3],cells:d.size});sk=sc=0;
 if(prod===s.requests.length)return {paid,outputs,services,events,exposure,prefix,peak,nodes:g.length,kernels,clones,stores,deleted:0,transfers,cellTime};load(s.requests[prod]);}break;}
 case "Add":v[n.dst]=v[n.lhs]+v[n.rhs];break;
 case "Sub":v[n.dst]=v[n.lhs]-v[n.rhs];break;
 case "Branch":pc=v[n.test]!==0?n.nonzero:n.zero;break;
 case "CloneNode":{const from=v[n.source],idx=g.length,x=idx-initial+1;if(from!==(x%2?l.parent_add:l.double_src))throw Error("clone");g.push({...g[from]});v[n.dst]=idx;clones++;break;}
 case "PatchConst":{const at=v[n.node],to=v[n.value];if(at===l.flag){if(!ghost||p!==l.activate||to!==1||g[l.ghost].value!==key(...input())||expectedHit!==null)throw Error("admit");events.push({at:prod,stream:input()[3],paid,suppressed:off});if(!off)g[at].value=1;}
 else if(at===l.slot){if(![60,64].includes(to))throw Error("slot");g[at].value=to;}
 else if(at===l.ghost){if(to!==key(...input()))throw Error("ghost");g[at].value=to;}
 else{if(at!==l.base||to!==initial-1)throw Error("base");g[at].value=to;}break;}
 case "PatchNext":{const at=v[n.node],to=v[n.target];if(!g[to]||g[at].next===undefined)throw Error("edge");if(at>=initial){const x=at-initial+1;if(to!==(x===1?l.out:x%2?at-1:initial+x/2-1))throw Error("generated");}else if(at===l.special_entry){if(to!==entry)throw Error("entry");}else if(at===l.kernel_dispatch){if(to!==l.base)throw Error("kernel");}else if(at===l.store_gate){if(!ghost||![l.slot,l.publish].includes(to))throw Error("gate");}else throw Error("patch");g[at].next=to;break;}
 default:throw Error(n.kind);
 }
 peak=Math.max(peak,g.length+1+d.size);if(g.length>200||d.size>14||peak>220||!v.every(Number.isSafeInteger))throw Error("capacity "+g.length+" "+d.size+" "+peak);
 }throw Error("budget");
}
const patterns=["primary_only","secondary_only","alternating_streams","primary_heavy","secondary_heavy","phase_shift","return","no_locality"];
const scenarios=[];
for(const k of [1,4,8])for(const n of [48,144])for(const pattern of patterns){
 const A=[7,3,k,0],B=[11,5,k,0],S=[13,2,k,1];const requests=Array.from({length:n},(_,i)=>{
 const primary=i%2?B:A,secondary=S,unique=[1+i*3%16,1+(i*5+Math.floor(i/16))%16,k,i%2];
 if(pattern==="primary_only")return primary;if(pattern==="secondary_only")return secondary;
 if(pattern==="alternating_streams")return i%3===2?secondary:(Math.floor(i/3)%2?B:A);
 if(pattern==="primary_heavy")return i%9===8?secondary:primary;
 if(pattern==="secondary_heavy")return i%9===8?primary:secondary;
 if(pattern==="phase_shift")return i<n/2?primary:secondary;
 if(pattern==="return")return Math.floor(i/24)%2?secondary:primary;
 return unique;
 });scenarios.push({id:k+"_"+n+"_"+pattern,k,n,pattern,requests});}

const novel=["cost_asymmetry_primary","cost_asymmetry_secondary","scan_pollution","bursty_secondary","alternating_distinct_cost","return_asymmetry"];
for(const n of [48,144])for(const pattern of novel){
 const A=[7,3,8,0],B=[11,5,8,0],S=[13,2,1,1],P=[7,3,1,0],Q=[11,5,1,0],T=[13,2,8,1];
 const requests=Array.from({length:n},(_,i)=>{
  if(pattern==="cost_asymmetry_primary")return i%3===2?S:(Math.floor(i/3)%2?A:B);
  if(pattern==="cost_asymmetry_secondary")return i%3===2?T:(Math.floor(i/3)%2?P:Q);
  if(pattern==="scan_pollution")return i%4===0?[1+(i*3)%16,1+(i*5+Math.floor(i/16))%16,1,1]:i%2?A:B;
  if(pattern==="bursty_secondary")return Math.floor(i/12)%3===1?T:i%2?P:Q;
  if(pattern==="alternating_distinct_cost")return i%2?T:A;
  return Math.floor(i/24)%2?S:i%2?A:B;
 });scenarios.push({id:"new_"+n+"_"+pattern,k:"mixed",n,pattern,requests});
}
const allKeys=new Set();for(let a=1;a<=16;a++)for(let b=1;b<=16;b++)for(const k of [1,4,8,2])for(const t of [0,1]){const x=key(a,b,k,t);if(allKeys.has(x))throw Error("collision");allKeys.add(x);}
const rows=scenarios.map(s=>{
 const arms={shared:run(s,"shared"),primary2:run(s,"primary2"),split:run(s,"split"),adaptive:run(s),fifo:runGlobal(s,"fifo"),lru:runGlobal(s,"lru"),ghost_fifo:runGlobal(s,"ghost_fifo"),ghost_lru:runGlobal(s,"ghost_lru"),ghost_off:runGlobal(s,"ghost_lru",true)};
 for(const q of Object.values(arms))if(q.outputs.join()!==arms.shared.outputs.join())throw Error("outputs");
 if(JSON.stringify(arms.ghost_lru.prefix)!==JSON.stringify(arms.ghost_off.prefix)||arms.ghost_lru.exposure!==arms.ghost_off.exposure)throw Error("prefix");
 if(arms.ghost_off.stores||arms.ghost_off.services.some(x=>x.reused))throw Error("off");
 const compact=q=>({paid:q.paid,kernels:q.kernels,nodes:q.nodes,peak:q.peak,stores:q.stores,deleted:q.deleted,transfers:q.transfers,admissions:q.events.length,reused:q.services.filter(x=>x.reused).length,pHits:q.services.filter(x=>x.reused&&x.stream===0).length,sHits:q.services.filter(x=>x.reused&&x.stream===1).length,cellTime:q.cellTime});
 return {id:s.id,k:s.k,n:s.n,pattern:s.pattern,...Object.fromEntries(Object.entries(arms).map(([k,q])=>[k,compact(q)]))};
});
const names=["primary2","split","adaptive","fifo","lru","ghost_fifo","ghost_lru","ghost_off"];
function agg(a){return Object.fromEntries(names.map(k=>[k,{net:a.reduce((s,r)=>s+r.shared.paid-r[k].paid,0),wins:a.filter(r=>r.shared.paid>r[k].paid).length,paid:a.reduce((s,r)=>s+r[k].paid,0),peak:Math.max(...a.map(r=>r[k].peak)),nodes:Math.max(...a.map(r=>r[k].nodes)),pHits:a.reduce((s,r)=>s+r[k].pHits,0),sHits:a.reduce((s,r)=>s+r[k].sHits,0),transfers:a.reduce((s,r)=>s+r[k].transfers,0),admissions:a.reduce((s,r)=>s+r[k].admissions,0),cellTime:a.reduce((s,r)=>s+r[k].cellTime,0)}]))}
const summary={all:agg(rows),original:agg(rows.filter(r=>patterns.includes(r.pattern))),novel:agg(rows.filter(r=>novel.includes(r.pattern))),patterns:Object.fromEntries([...patterns,...novel].map(k=>[k,agg(rows.filter(r=>r.pattern===k))])),keyDomain:allKeys.size,worlds:rows.length*9};

assert.deepEqual(summary,{"all":{"primary2":{"net":200928,"wins":32,"paid":622450,"peak":200,"nodes":185,"pHits":2742,"sHits":0,"transfers":0,"admissions":0,"cellTime":77500},"split":{"net":186745,"wins":36,"paid":636633,"peak":200,"nodes":185,"pHits":520,"sHits":2254,"transfers":0,"admissions":0,"cellTime":77076},"adaptive":{"net":420710,"wins":50,"paid":402668,"peak":200,"nodes":185,"pHits":2468,"sHits":1969,"transfers":505,"admissions":559,"cellTime":77976},"fifo":{"net":378998,"wins":50,"paid":444380,"peak":183,"nodes":168,"pHits":2153,"sHits":2009,"transfers":614,"admissions":0,"cellTime":79276},"lru":{"net":413240,"wins":50,"paid":410138,"peak":183,"nodes":168,"pHits":2259,"sHits":2186,"transfers":257,"admissions":0,"cellTime":79276},"ghost_fifo":{"net":83509,"wins":30,"paid":739869,"peak":183,"nodes":168,"pHits":320,"sHits":1915,"transfers":150,"admissions":268,"cellTime":65652},"ghost_lru":{"net":52748,"wins":26,"paid":770630,"peak":183,"nodes":168,"pHits":20,"sHits":2055,"transfers":10,"admissions":348,"cellTime":65652},"ghost_off":{"net":-252130,"wins":0,"paid":1075508,"peak":179,"nodes":168,"pHits":0,"sHits":0,"transfers":0,"admissions":2042,"cellTime":57600}},"original":{"primary2":{"net":122480,"wins":24,"paid":516276,"peak":200,"nodes":185,"pHits":2040,"sHits":0,"transfers":0,"admissions":0,"cellTime":61800},"split":{"net":120763,"wins":26,"paid":517993,"peak":200,"nodes":185,"pHits":252,"sHits":1884,"transfers":0,"admissions":0,"cellTime":61116},"adaptive":{"net":310050,"wins":38,"paid":328706,"peak":200,"nodes":185,"pHits":1839,"sHits":1674,"transfers":369,"admissions":411,"cellTime":61932},"fifo":{"net":296400,"wins":38,"paid":342356,"peak":183,"nodes":168,"pHits":1677,"sHits":1704,"transfers":384,"admissions":0,"cellTime":63180},"lru":{"net":313173,"wins":38,"paid":325583,"peak":183,"nodes":168,"pHits":1737,"sHits":1821,"transfers":147,"admissions":0,"cellTime":63180},"ghost_fifo":{"net":85903,"wins":24,"paid":552853,"peak":183,"nodes":168,"pHits":192,"sHits":1707,"transfers":90,"admissions":168,"cellTime":52020},"ghost_lru":{"net":65918,"wins":22,"paid":572838,"peak":183,"nodes":168,"pHits":12,"sHits":1791,"transfers":6,"admissions":216,"cellTime":52020},"ghost_off":{"net":-202371,"wins":0,"paid":841127,"peak":179,"nodes":168,"pHits":0,"sHits":0,"transfers":0,"admissions":1767,"cellTime":46080}},"novel":{"primary2":{"net":78448,"wins":8,"paid":106174,"peak":200,"nodes":185,"pHits":702,"sHits":0,"transfers":0,"admissions":0,"cellTime":15700},"split":{"net":65982,"wins":10,"paid":118640,"peak":200,"nodes":185,"pHits":268,"sHits":370,"transfers":0,"admissions":0,"cellTime":15960},"adaptive":{"net":110660,"wins":12,"paid":73962,"peak":200,"nodes":185,"pHits":629,"sHits":295,"transfers":136,"admissions":148,"cellTime":16044},"fifo":{"net":82598,"wins":12,"paid":102024,"peak":183,"nodes":168,"pHits":476,"sHits":305,"transfers":230,"admissions":0,"cellTime":16096},"lru":{"net":100067,"wins":12,"paid":84555,"peak":183,"nodes":168,"pHits":522,"sHits":365,"transfers":110,"admissions":0,"cellTime":16096},"ghost_fifo":{"net":-2394,"wins":6,"paid":187016,"peak":183,"nodes":168,"pHits":128,"sHits":208,"transfers":60,"admissions":100,"cellTime":13632},"ghost_lru":{"net":-13170,"wins":4,"paid":197792,"peak":183,"nodes":168,"pHits":8,"sHits":264,"transfers":4,"admissions":132,"cellTime":13632},"ghost_off":{"net":-49759,"wins":0,"paid":234381,"peak":179,"nodes":168,"pHits":0,"sHits":0,"transfers":0,"admissions":275,"cellTime":11520}},"patterns":{"primary_only":{"primary2":{"net":56552,"wins":6,"paid":22828,"peak":200,"nodes":185,"pHits":564,"sHits":0,"transfers":0,"admissions":0,"cellTime":8052},"split":{"net":-24192,"wins":0,"paid":103572,"peak":198,"nodes":185,"pHits":0,"sHits":0,"transfers":0,"admissions":0,"cellTime":6912},"adaptive":{"net":55588,"wins":6,"paid":23792,"peak":200,"nodes":185,"pHits":558,"sHits":0,"transfers":0,"admissions":6,"cellTime":8040},"fifo":{"net":56894,"wins":6,"paid":22486,"peak":183,"nodes":168,"pHits":564,"sHits":0,"transfers":0,"admissions":0,"cellTime":8052},"lru":{"net":55202,"wins":6,"paid":24178,"peak":183,"nodes":168,"pHits":564,"sHits":0,"transfers":0,"admissions":0,"cellTime":8052},"ghost_fifo":{"net":-24192,"wins":0,"paid":103572,"peak":179,"nodes":168,"pHits":0,"sHits":0,"transfers":0,"admissions":0,"cellTime":5760},"ghost_lru":{"net":-24192,"wins":0,"paid":103572,"peak":179,"nodes":168,"pHits":0,"sHits":0,"transfers":0,"admissions":0,"cellTime":5760},"ghost_off":{"net":-24192,"wins":0,"paid":103572,"peak":179,"nodes":168,"pHits":0,"sHits":0,"transfers":0,"admissions":0,"cellTime":5760}},"secondary_only":{"primary2":{"net":-17280,"wins":0,"paid":98484,"peak":196,"nodes":185,"pHits":0,"sHits":0,"transfers":0,"admissions":0,"cellTime":5760},"split":{"net":60144,"wins":6,"paid":21060,"peak":198,"nodes":185,"pHits":0,"sHits":570,"transfers":0,"admissions":0,"cellTime":6912},"adaptive":{"net":59226,"wins":6,"paid":21978,"peak":198,"nodes":185,"pHits":0,"sHits":564,"transfers":0,"admissions":6,"cellTime":6900},"fifo":{"net":60720,"wins":6,"paid":20484,"peak":181,"nodes":168,"pHits":0,"sHits":570,"transfers":0,"admissions":0,"cellTime":6912},"lru":{"net":59010,"wins":6,"paid":22194,"peak":181,"nodes":168,"pHits":0,"sHits":570,"transfers":0,"admissions":0,"cellTime":6912},"ghost_fifo":{"net":59760,"wins":6,"paid":21444,"peak":181,"nodes":168,"pHits":0,"sHits":564,"transfers":0,"admissions":6,"cellTime":6900},"ghost_lru":{"net":58068,"wins":6,"paid":23136,"peak":181,"nodes":168,"pHits":0,"sHits":564,"transfers":0,"admissions":6,"cellTime":6900},"ghost_off":{"net":-27042,"wins":0,"paid":108246,"peak":179,"nodes":168,"pHits":0,"sHits":0,"transfers":0,"admissions":570,"cellTime":5760}},"alternating_streams":{"primary2":{"net":31336,"wins":4,"paid":48652,"peak":200,"nodes":185,"pHits":372,"sHits":0,"transfers":0,"admissions":0,"cellTime":8028},"split":{"net":31312,"wins":4,"paid":48676,"peak":200,"nodes":185,"pHits":192,"sHits":186,"transfers":0,"admissions":0,"cellTime":8040},"adaptive":{"net":27292,"wins":4,"paid":52696,"peak":200,"nodes":185,"pHits":282,"sHits":90,"transfers":180,"admissions":186,"cellTime":8004},"fifo":{"net":16672,"wins":4,"paid":63316,"peak":183,"nodes":168,"pHits":192,"sHits":96,"transfers":180,"admissions":0,"cellTime":8040},"lru":{"net":29050,"wins":4,"paid":50938,"peak":183,"nodes":168,"pHits":192,"sHits":186,"transfers":0,"admissions":0,"cellTime":8040},"ghost_fifo":{"net":14650,"wins":4,"paid":65338,"peak":183,"nodes":168,"pHits":192,"sHits":90,"transfers":90,"admissions":144,"cellTime":8004},"ghost_lru":{"net":-484,"wins":2,"paid":80472,"peak":183,"nodes":168,"pHits":12,"sHits":174,"transfers":6,"admissions":192,"cellTime":8004},"ghost_off":{"net":-25152,"wins":0,"paid":105140,"peak":179,"nodes":168,"pHits":0,"sHits":0,"transfers":0,"admissions":192,"cellTime":5760}},"primary_heavy":{"primary2":{"net":48277,"wins":6,"paid":31309,"peak":200,"nodes":185,"pHits":501,"sHits":0,"transfers":0,"admissions":0,"cellTime":8052},"split":{"net":-7017,"wins":2,"paid":86603,"peak":200,"nodes":185,"pHits":60,"sHits":57,"transfers":0,"admissions":0,"cellTime":7968},"adaptive":{"net":37554,"wins":4,"paid":42032,"peak":200,"nodes":185,"pHits":441,"sHits":0,"transfers":111,"admissions":117,"cellTime":8040},"fifo":{"net":29904,"wins":4,"paid":49682,"peak":183,"nodes":168,"pHits":381,"sHits":0,"transfers":123,"admissions":0,"cellTime":8052},"lru":{"net":37512,"wins":4,"paid":42074,"peak":183,"nodes":168,"pHits":441,"sHits":0,"transfers":123,"admissions":0,"cellTime":8052},"ghost_fifo":{"net":-24192,"wins":0,"paid":103778,"peak":179,"nodes":168,"pHits":0,"sHits":0,"transfers":0,"admissions":0,"cellTime":5760},"ghost_lru":{"net":-24192,"wins":0,"paid":103778,"peak":179,"nodes":168,"pHits":0,"sHits":0,"transfers":0,"admissions":0,"cellTime":5760},"ghost_off":{"net":-24192,"wins":0,"paid":103778,"peak":179,"nodes":168,"pHits":0,"sHits":0,"transfers":0,"admissions":0,"cellTime":5760}},"secondary_heavy":{"primary2":{"net":-10821,"wins":0,"paid":91819,"peak":200,"nodes":185,"pHits":51,"sHits":0,"transfers":0,"admissions":0,"cellTime":7764},"split":{"net":50820,"wins":6,"paid":30178,"peak":200,"nodes":185,"pHits":0,"sHits":507,"transfers":0,"admissions":0,"cellTime":7968},"adaptive":{"net":48570,"wins":6,"paid":32428,"peak":200,"nodes":185,"pHits":24,"sHits":474,"transfers":54,"admissions":60,"cellTime":7956},"fifo":{"net":46140,"wins":6,"paid":34858,"peak":183,"nodes":168,"pHits":0,"sHits":480,"transfers":57,"admissions":0,"cellTime":7968},"lru":{"net":49623,"wins":6,"paid":31375,"peak":183,"nodes":168,"pHits":0,"sHits":507,"transfers":0,"admissions":0,"cellTime":7968},"ghost_fifo":{"net":50373,"wins":6,"paid":30625,"peak":181,"nodes":168,"pHits":0,"sHits":501,"transfers":0,"admissions":6,"cellTime":6900},"ghost_lru":{"net":48870,"wins":6,"paid":32128,"peak":181,"nodes":168,"pHits":0,"sHits":501,"transfers":0,"admissions":6,"cellTime":6900},"ghost_off":{"net":-26427,"wins":0,"paid":107425,"peak":179,"nodes":168,"pHits":0,"sHits":0,"transfers":0,"admissions":447,"cellTime":5760}},"phase_shift":{"primary2":{"net":18728,"wins":4,"paid":61564,"peak":200,"nodes":185,"pHits":276,"sHits":0,"transfers":0,"admissions":0,"cellTime":8052},"split":{"net":17520,"wins":4,"paid":62772,"peak":200,"nodes":185,"pHits":0,"sHits":282,"transfers":0,"admissions":0,"cellTime":7488},"adaptive":{"net":55102,"wins":6,"paid":25190,"peak":200,"nodes":185,"pHits":270,"sHits":276,"transfers":6,"admissions":12,"cellTime":8040},"fifo":{"net":57470,"wins":6,"paid":22822,"peak":183,"nodes":168,"pHits":276,"sHits":282,"transfers":6,"admissions":0,"cellTime":8052},"lru":{"net":55796,"wins":6,"paid":24496,"peak":183,"nodes":168,"pHits":276,"sHits":282,"transfers":6,"admissions":0,"cellTime":8052},"ghost_fifo":{"net":16848,"wins":4,"paid":63444,"peak":181,"nodes":168,"pHits":0,"sHits":276,"transfers":0,"admissions":6,"cellTime":6324},"ghost_lru":{"net":16020,"wins":4,"paid":64272,"peak":181,"nodes":168,"pHits":0,"sHits":276,"transfers":0,"admissions":6,"cellTime":6324},"ghost_off":{"net":-25602,"wins":0,"paid":105894,"peak":179,"nodes":168,"pHits":0,"sHits":0,"transfers":0,"admissions":282,"cellTime":5760}},"return":{"primary2":{"net":18728,"wins":4,"paid":61564,"peak":200,"nodes":185,"pHits":276,"sHits":0,"transfers":0,"admissions":0,"cellTime":8052},"split":{"net":17520,"wins":4,"paid":62772,"peak":200,"nodes":185,"pHits":0,"sHits":282,"transfers":0,"admissions":0,"cellTime":7776},"adaptive":{"net":53214,"wins":6,"paid":27078,"peak":200,"nodes":185,"pHits":264,"sHits":270,"transfers":18,"admissions":24,"cellTime":8040},"fifo":{"net":54520,"wins":6,"paid":25772,"peak":183,"nodes":168,"pHits":264,"sHits":276,"transfers":18,"admissions":0,"cellTime":8052},"lru":{"net":52900,"wins":6,"paid":27392,"peak":183,"nodes":168,"pHits":264,"sHits":276,"transfers":18,"admissions":0,"cellTime":8052},"ghost_fifo":{"net":16848,"wins":4,"paid":63444,"peak":181,"nodes":168,"pHits":0,"sHits":276,"transfers":0,"admissions":6,"cellTime":6612},"ghost_lru":{"net":16020,"wins":4,"paid":64272,"peak":181,"nodes":168,"pHits":0,"sHits":276,"transfers":0,"admissions":6,"cellTime":6612},"ghost_off":{"net":-25572,"wins":0,"paid":105864,"peak":179,"nodes":168,"pHits":0,"sHits":0,"transfers":0,"admissions":276,"cellTime":5760}},"no_locality":{"primary2":{"net":-23040,"wins":0,"paid":100056,"peak":200,"nodes":185,"pHits":0,"sHits":0,"transfers":0,"admissions":0,"cellTime":8040},"split":{"net":-25344,"wins":0,"paid":102360,"peak":200,"nodes":185,"pHits":0,"sHits":0,"transfers":0,"admissions":0,"cellTime":8052},"adaptive":{"net":-26496,"wins":0,"paid":103512,"peak":198,"nodes":185,"pHits":0,"sHits":0,"transfers":0,"admissions":0,"cellTime":6912},"fifo":{"net":-25920,"wins":0,"paid":102936,"peak":183,"nodes":168,"pHits":0,"sHits":0,"transfers":0,"admissions":0,"cellTime":8052},"lru":{"net":-25920,"wins":0,"paid":102936,"peak":183,"nodes":168,"pHits":0,"sHits":0,"transfers":0,"admissions":0,"cellTime":8052},"ghost_fifo":{"net":-24192,"wins":0,"paid":101208,"peak":179,"nodes":168,"pHits":0,"sHits":0,"transfers":0,"admissions":0,"cellTime":5760},"ghost_lru":{"net":-24192,"wins":0,"paid":101208,"peak":179,"nodes":168,"pHits":0,"sHits":0,"transfers":0,"admissions":0,"cellTime":5760},"ghost_off":{"net":-24192,"wins":0,"paid":101208,"peak":179,"nodes":168,"pHits":0,"sHits":0,"transfers":0,"admissions":0,"cellTime":5760}},"cost_asymmetry_primary":{"primary2":{"net":22494,"wins":2,"paid":10846,"peak":200,"nodes":185,"pHits":124,"sHits":0,"transfers":0,"admissions":0,"cellTime":2676},"split":{"net":11076,"wins":2,"paid":22264,"peak":200,"nodes":185,"pHits":64,"sHits":62,"transfers":0,"admissions":0,"cellTime":2680},"adaptive":{"net":15346,"wins":2,"paid":17994,"peak":200,"nodes":185,"pHits":94,"sHits":30,"transfers":60,"admissions":62,"cellTime":2668},"fifo":{"net":8896,"wins":2,"paid":24444,"peak":183,"nodes":168,"pHits":64,"sHits":32,"transfers":60,"admissions":0,"cellTime":2680},"lru":{"net":10322,"wins":2,"paid":23018,"peak":183,"nodes":168,"pHits":64,"sHits":62,"transfers":0,"admissions":0,"cellTime":2680},"ghost_fifo":{"net":8402,"wins":2,"paid":24938,"peak":183,"nodes":168,"pHits":64,"sHits":30,"transfers":30,"admissions":48,"cellTime":2668},"ghost_lru":{"net":-4968,"wins":0,"paid":38308,"peak":183,"nodes":168,"pHits":4,"sHits":58,"transfers":2,"admissions":64,"cellTime":2668},"ghost_off":{"net":-8384,"wins":0,"paid":41724,"peak":179,"nodes":168,"pHits":0,"sHits":0,"transfers":0,"admissions":64,"cellTime":1920}},"cost_asymmetry_secondary":{"primary2":{"net":-508,"wins":0,"paid":22200,"peak":200,"nodes":185,"pHits":124,"sHits":0,"transfers":0,"admissions":0,"cellTime":2676},"split":{"net":10922,"wins":2,"paid":10770,"peak":200,"nodes":185,"pHits":64,"sHits":62,"transfers":0,"admissions":0,"cellTime":2680},"adaptive":{"net":3684,"wins":2,"paid":18008,"peak":200,"nodes":185,"pHits":94,"sHits":30,"transfers":60,"admissions":62,"cellTime":2668},"fifo":{"net":3072,"wins":2,"paid":18620,"peak":183,"nodes":168,"pHits":64,"sHits":32,"transfers":60,"admissions":0,"cellTime":2680},"lru":{"net":10168,"wins":2,"paid":11524,"peak":183,"nodes":168,"pHits":64,"sHits":62,"transfers":0,"admissions":0,"cellTime":2680},"ghost_fifo":{"net":2200,"wins":2,"paid":19492,"peak":183,"nodes":168,"pHits":64,"sHits":30,"transfers":30,"admissions":48,"cellTime":2668},"ghost_lru":{"net":5238,"wins":2,"paid":16454,"peak":183,"nodes":168,"pHits":4,"sHits":58,"transfers":2,"admissions":64,"cellTime":2668},"ghost_off":{"net":-8384,"wins":0,"paid":30076,"peak":179,"nodes":168,"pHits":0,"sHits":0,"transfers":0,"admissions":64,"cellTime":1920}},"scan_pollution":{"primary2":{"net":26078,"wins":2,"paid":9918,"peak":200,"nodes":185,"pHits":140,"sHits":0,"transfers":0,"admissions":0,"cellTime":2676},"split":{"net":2738,"wins":2,"paid":33258,"peak":200,"nodes":185,"pHits":46,"sHits":0,"transfers":0,"admissions":0,"cellTime":2684},"adaptive":{"net":24894,"wins":2,"paid":11102,"peak":200,"nodes":185,"pHits":138,"sHits":0,"transfers":0,"admissions":2,"cellTime":2672},"fifo":{"net":2880,"wins":2,"paid":33116,"peak":183,"nodes":168,"pHits":48,"sHits":0,"transfers":94,"admissions":0,"cellTime":2684},"lru":{"net":13450,"wins":2,"paid":22546,"peak":183,"nodes":168,"pHits":94,"sHits":0,"transfers":94,"admissions":0,"cellTime":2684},"ghost_fifo":{"net":-8064,"wins":0,"paid":44060,"peak":179,"nodes":168,"pHits":0,"sHits":0,"transfers":0,"admissions":0,"cellTime":1920},"ghost_lru":{"net":-8064,"wins":0,"paid":44060,"peak":179,"nodes":168,"pHits":0,"sHits":0,"transfers":0,"admissions":0,"cellTime":1920},"ghost_off":{"net":-8064,"wins":0,"paid":44060,"peak":179,"nodes":168,"pHits":0,"sHits":0,"transfers":0,"admissions":0,"cellTime":1920}},"bursty_secondary":{"primary2":{"net":-336,"wins":0,"paid":21266,"peak":200,"nodes":185,"pHits":128,"sHits":0,"transfers":0,"admissions":0,"cellTime":2684},"split":{"net":6254,"wins":2,"paid":14676,"peak":200,"nodes":185,"pHits":0,"sHits":58,"transfers":0,"admissions":0,"cellTime":2640},"adaptive":{"net":11486,"wins":2,"paid":9444,"peak":200,"nodes":185,"pHits":121,"sHits":53,"transfers":10,"admissions":12,"cellTime":2680},"fifo":{"net":11980,"wins":2,"paid":8950,"peak":183,"nodes":168,"pHits":118,"sHits":55,"transfers":10,"admissions":0,"cellTime":2684},"lru":{"net":11461,"wins":2,"paid":9469,"peak":183,"nodes":168,"pHits":118,"sHits":55,"transfers":10,"admissions":0,"cellTime":2684},"ghost_fifo":{"net":5796,"wins":2,"paid":15134,"peak":181,"nodes":168,"pHits":0,"sHits":56,"transfers":0,"admissions":2,"cellTime":2252},"ghost_lru":{"net":5628,"wins":2,"paid":15302,"peak":181,"nodes":168,"pHits":0,"sHits":56,"transfers":0,"admissions":2,"cellTime":2252},"ghost_off":{"net":-8339,"wins":0,"paid":29269,"peak":179,"nodes":168,"pHits":0,"sHits":0,"transfers":0,"admissions":55,"cellTime":1920}},"alternating_distinct_cost":{"primary2":{"net":15538,"wins":2,"paid":29674,"peak":198,"nodes":185,"pHits":94,"sHits":0,"transfers":0,"admissions":0,"cellTime":2304},"split":{"net":37612,"wins":2,"paid":7600,"peak":200,"nodes":185,"pHits":94,"sHits":94,"transfers":0,"admissions":0,"cellTime":2684},"adaptive":{"net":37086,"wins":2,"paid":8126,"peak":200,"nodes":185,"pHits":94,"sHits":92,"transfers":0,"admissions":2,"cellTime":2676},"fifo":{"net":37326,"wins":2,"paid":7886,"peak":183,"nodes":168,"pHits":94,"sHits":94,"transfers":0,"admissions":0,"cellTime":2684},"lru":{"net":36762,"wins":2,"paid":8450,"peak":183,"nodes":168,"pHits":94,"sHits":94,"transfers":0,"admissions":0,"cellTime":2684},"ghost_fifo":{"net":-8064,"wins":0,"paid":53276,"peak":179,"nodes":168,"pHits":0,"sHits":0,"transfers":0,"admissions":0,"cellTime":1920},"ghost_lru":{"net":-8064,"wins":0,"paid":53276,"peak":179,"nodes":168,"pHits":0,"sHits":0,"transfers":0,"admissions":0,"cellTime":1920},"ghost_off":{"net":-8064,"wins":0,"paid":53276,"peak":179,"nodes":168,"pHits":0,"sHits":0,"transfers":0,"admissions":0,"cellTime":1920}},"return_asymmetry":{"primary2":{"net":15182,"wins":2,"paid":12270,"peak":200,"nodes":185,"pHits":92,"sHits":0,"transfers":0,"admissions":0,"cellTime":2684},"split":{"net":-2620,"wins":0,"paid":30072,"peak":200,"nodes":185,"pHits":0,"sHits":94,"transfers":0,"admissions":0,"cellTime":2592},"adaptive":{"net":18164,"wins":2,"paid":9288,"peak":200,"nodes":185,"pHits":88,"sHits":90,"transfers":6,"admissions":8,"cellTime":2680},"fifo":{"net":18444,"wins":2,"paid":9008,"peak":183,"nodes":168,"pHits":88,"sHits":92,"transfers":6,"admissions":0,"cellTime":2684},"lru":{"net":17904,"wins":2,"paid":9548,"peak":183,"nodes":168,"pHits":88,"sHits":92,"transfers":6,"admissions":0,"cellTime":2684},"ghost_fifo":{"net":-2664,"wins":0,"paid":30116,"peak":181,"nodes":168,"pHits":0,"sHits":92,"transfers":0,"admissions":2,"cellTime":2204},"ghost_lru":{"net":-2940,"wins":0,"paid":30392,"peak":181,"nodes":168,"pHits":0,"sHits":92,"transfers":0,"admissions":2,"cellTime":2204},"ghost_off":{"net":-8524,"wins":0,"paid":35976,"peak":179,"nodes":168,"pHits":0,"sHits":0,"transfers":0,"admissions":92,"cellTime":1920}}},"keyDomain":2048,"worlds":540});
console.log(JSON.stringify({tier:"NON_NATIVE_GLOBAL_CACHE_CONTROLS",summary,rows,scenarios,sources:Object.fromEntries(["fifo","lru","ghost_fifo","ghost_lru"].map(k=>[k,buildGlobal(k)]))},null,2));
