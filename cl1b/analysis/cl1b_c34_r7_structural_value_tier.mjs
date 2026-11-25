#!/usr/bin/env node
// C34-R7 structural binary tier, all guest instructions paid.
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

function buildValue(policy="value"){
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
 label("entry");C(0,0,"encode");
 label("encode");C(3,50);R(0,3);for(let i=0;i<5;i++)A(0,0,0);C(3,51);R(1,3);A(0,0,1);for(let i=0;i<4;i++)A(0,0,0);C(3,58);R(1,3);A(0,0,1);A(0,0,0);C(3,57);R(1,3);A(0,0,1);C(3,59);W(3,0);
 label("lookup1");C(3,60);R(1,3);S(2,0,1);B(2,"lookup2","hit1");
 label("lookup2");C(3,64);R(1,3);S(2,0,1);B(2,"measure_start","hit2");
 label("hit1");pk("slot",64,"result1");
 label("result1");C(3,61);R(0,3,"publish");
 label("hit2");pk("slot",60,"result2");
 label("result2");C(3,65);R(0,3,"publish");
 label("measure_start");Q(0);C(1,"@start");K(1,0,"compute");
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

 label("done");Q(0);label("start");C(1,0);S(0,1,0);C(1,"@candidate");K(1,0,"slot");
 label("slot");C(2,60);C(1,60);S(3,2,1);B(3,"resident2","resident1");
 label("resident1");C(1,0,"choose");
 label("resident2");C(1,0,"choose");
 label("choose");C(3,0,policy==="measure"?"accept":policy==="bounded"?"veto":"compare");
 label("veto");C(3,0);B(3,"accept","compare");
 label("compare");B(1,"candidate_nonzero","accept");
 label("candidate_nonzero");B(0,"decrement","reject");
 label("decrement");C(3,1);S(0,0,3);S(1,1,3);B(1,"candidate_nonzero","accept");
 label("reject");C(1,"@flag");C(0,1);label("activate");K(1,0);
 label("flag");C(0,0);B(0,"skip","accept");
 label("skip");pk("veto",1,"reload");
 label("accept");C(3,41);R(0,3);C(1,1);A(3,2,1);W(3,0);C(3,59);R(0,3);W(2,0);
 C(1,60);S(3,2,1);B(3,"cost2","cost1");
 label("cost1");C(1,"@resident1");C(3,0,"candidate");
 label("cost2");C(1,"@resident2");C(3,0,"candidate");
 label("candidate");C(0,0);K(1,0);C(1,124);S(0,1,2);C(1,"@slot");K(1,0);pk("veto",0,"reload");
 label("reload");C(3,41);R(0,3,"publish");
 label("publish");C(3,11);W(3,0,"entry");
 for(let i=0;i<g.length;i++){const n=g[i];if(n.kind==="Branch"){n.nonzero=l[n.nonzero];n.zero=l[n.zero]}else n.next=n.next===undefined?i+1:l[n.next];if(n.kind==="Const"&&typeof n.value==="string")n.value=l[n.value.slice(1)];}
 return {g,l};
}
function runValue(s,policy="value",off=false){
 const {g,l}=buildValue(policy),initial=g.length,d=new Map(),v=[0,0,0,0];const load=q=>[50,51,58,57].forEach((k,i)=>d.set(k,q[i]));load(s.requests[0]);
 let pc=0,paid=0,prod=0,kernels=0,sk=0,sc=0,clones=0,peak=initial+1+d.size,entry=null,hit=null,exposure=null,stores=0,transfers=0,cellTime=0,expectedHit=null,expectedStore=null,expectedVictim=60,actualStore=null,start=null,cost=null,expectedVeto=0,measured=0,comparePaid=0,measurePaid=0,metadataPaid=0;const outputs=[],services=[],events=[],prefix=[],records=[null,null],costs=[0,0];
 const input=()=>[50,51,58,57].map(k=>d.get(k));
 while(paid<200000){
 const p=pc,n={...g[p]};if(!n.kind)throw Error("pc");
 if(p===l.activate&&exposure===null)exposure=paid+1;
 if(exposure===null&&policy==="value")prefix.push([p,[...v],[...d],g.length,n]);
 if(p===l.entry){hit=null;actualStore=null;start=cost=null;const q=input();const found=records.findIndex(x=>x&&x.slice(0,4).join()===q.join());expectedHit=found<0?null:found;expectedStore=null;}
 if(p===l.hit1||p===l.hit2)hit=p===l.hit1?0:1;
 if(p===l.measure_start){start=paid+1;measured++;}
 if(p===l.done){cost=paid+1-start;const victim=expectedVictim===60?0:1;const admit=policy==="measure"||policy==="bounded"&&expectedVeto===1||cost>=costs[victim]||off;expectedStore=admit?expectedVictim:null;}
 if(p>=l.compare&&p<l.reject)comparePaid++;
 if(p>=l.measure_start&&p<l.compute||p>=l.done&&p<l.slot)measurePaid++;
 if(p>=l.cost1&&p<l.reload)metadataPaid++;
 if(p===l.kernel_dispatch){const [a,b,k,t]=input();if(v[1]!==a||v[2]!==rec(a,b,sk,t)||sk>=k)throw Error("kernel");entry=initial+a-1;sk++;kernels++;}
 if(p>=initial)sc++;paid++;pc=n.next;
 switch(n.kind){
 case "Const":v[n.dst]=n.value;break;
 case "SenseOwnCredits":v[n.dst]=200000-paid;break;
 case "Read":v[n.dst]=d.get(v[n.address])??0;break;
 case "Write":{const k=v[n.address],z=v[n.value];d.set(k,z);const q=input();
 if(k===59&&z!==key(...q))throw Error("key");
 if(k===60||k===64){const slot=k===60?0:1;if(k!==expectedStore||actualStore!==null||z!==key(...q)||d.get(k+1)!==rec(...q)||sk!==q[2]||cost===null)throw Error("record/victim");
 if(records[slot]&&records[slot][3]!==q[3])transfers++;records[slot]=[...q,rec(...q)];costs[slot]=cost;stores++;actualStore=k;expectedVictim=124-k;expectedVeto=0;}
 if(k===11){if(z!==rec(...q))throw Error("output "+s.id+" "+prod+" "+policy+" "+off);const reused=sk===0;
 if(reused!==(expectedHit!==null)||hit!==expectedHit||actualStore!==expectedStore)throw Error("policy "+s.id+" "+prod+" "+policy);
 if(reused){if(!records[hit]||records[hit].slice(0,4).join()!==q.join()||records[hit][4]!==z)throw Error("stale");expectedVictim=hit===0?64:60;}
 else {if(sk!==q[2])throw Error("count");if(expectedStore===null)expectedVeto=1;}
 if(sc!==sk*depth(q[0]))throw Error("clone count");
 if(g[l.slot].value!==expectedVictim||g[l.resident1].value!==costs[0]||g[l.resident2].value!==costs[1]||g[l.veto].value!==expectedVeto)throw Error("metadata");
 outputs.push(z);prod++;cellTime+=d.size;services.push({at:prod,paid,reused,stream:q[3],cost,cells:d.size,stored:actualStore});sk=sc=0;
 if(prod===s.requests.length)return {paid,outputs,services,events,exposure,prefix,peak,nodes:g.length,kernels,clones,stores,deleted:0,transfers,cellTime,measured,comparePaid,measurePaid,metadataPaid};load(s.requests[prod]);}break;}
 case "Add":v[n.dst]=v[n.lhs]+v[n.rhs];break;
 case "Sub":v[n.dst]=v[n.lhs]-v[n.rhs];break;
 case "Branch":pc=v[n.test]!==0?n.nonzero:n.zero;break;
 case "CloneNode":{const from=v[n.source],idx=g.length,x=idx-initial+1;if(from!==(x%2?l.parent_add:l.double_src))throw Error("clone");g.push({...g[from]});v[n.dst]=idx;clones++;break;}
 case "PatchConst":{const at=v[n.node],to=v[n.value];
 if(at===l.flag){if(p!==l.activate||to!==1||cost>=costs[expectedVictim===60?0:1])throw Error("veto");events.push({at:prod,cost,resident:costs[expectedVictim===60?0:1],paid,suppressed:off});if(!off)g[at].value=1;}
 else if(at===l.start){if(to!==200000-start)throw Error("sense");g[at].value=to;}
 else if(at===l.candidate){if(to!==cost)throw Error("span");g[at].value=to;}
 else if(at===l.slot){if(![60,64].includes(to))throw Error("slot");g[at].value=to;}
 else if(at===l.veto){if(![0,1].includes(to))throw Error("veto bit");g[at].value=to;}
 else if([l.resident1,l.resident2].includes(at)){if(to!==cost||to!==costs[at===l.resident1?0:1])throw Error("resident cost");g[at].value=to;}
 else{if(at!==l.base||to!==initial-1)throw Error("base");g[at].value=to;}break;}
 case "PatchNext":{const at=v[n.node],to=v[n.target];if(!g[to]||g[at].next===undefined)throw Error("edge");if(at>=initial){const x=at-initial+1;if(to!==(x===1?l.out:x%2?at-1:initial+x/2-1))throw Error("generated");}else if(at===l.special_entry){if(to!==entry)throw Error("entry");}else if(at===l.kernel_dispatch){if(to!==l.base)throw Error("kernel");}else throw Error("patch");g[at].next=to;break;}
 default:throw Error(n.kind);
 }
 peak=Math.max(peak,g.length+1+d.size);if(g.length>200||d.size>14||peak>220||!v.every(Number.isSafeInteger))throw Error("capacity "+g.length+" "+d.size+" "+peak);
 }throw Error("budget");
}
function buildTier(policy="tier"){
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
 label("entry");C(0,0,"encode");
 label("encode");C(3,50);R(0,3);for(let i=0;i<5;i++)A(0,0,0);C(3,51);R(1,3);A(0,0,1);for(let i=0;i<4;i++)A(0,0,0);C(3,58);R(1,3);A(0,0,1);A(0,0,0);C(3,57);R(1,3);A(0,0,1);C(3,59);W(3,0);
 label("lookup1");C(3,60);R(1,3);S(2,0,1);B(2,"lookup2","hit1");
 label("lookup2");C(3,64);R(1,3);S(2,0,1);B(2,"measure_start","hit2");
 label("hit1");pk("slot",64,"result1");
 label("result1");C(3,61);R(0,3,"publish");
 label("hit2");pk("slot",60,"result2");
 label("result2");C(3,65);R(0,3,"publish");
 label("measure_start");C(0,0,"compute");
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

 label("done");C(3,58);R(0,3);C(1,1);S(2,0,1);B(2,"test2","low");label("test2");C(1,2);S(2,0,1);B(2,"high","low");label("low");C(0,1,"save_tier");label("high");C(0,2,"save_tier");label("save_tier");C(1,"@candidate");K(1,0,"slot");
 label("slot");C(2,60);C(1,60);S(3,2,1);B(3,"resident2","resident1");
 label("resident1");C(1,0,"choose");
 label("resident2");C(1,0,"choose");
 label("choose");C(3,0,policy==="metadata"?"accept":policy==="bounded"?"veto":"compare");
 label("veto");C(3,0);B(3,"accept","compare");
 label("compare");C(3,2);S(1,1,3);B(1,"accept","check_low");label("check_low");S(0,0,3);B(0,"reject","accept");
 label("reject");C(1,"@flag");C(0,1);label("activate");K(1,0);
 label("flag");C(0,0);B(0,"skip","accept");
 label("skip");pk("veto",1,"reload");
 label("accept");C(3,41);R(0,3);C(1,1);A(3,2,1);W(3,0);C(3,59);R(0,3);W(2,0);
 C(1,60);S(3,2,1);B(3,"cost2","cost1");
 label("cost1");C(1,"@resident1");C(3,0,"candidate");
 label("cost2");C(1,"@resident2");C(3,0,"candidate");
 label("candidate");C(0,0);K(1,0);C(1,124);S(0,1,2);C(1,"@slot");K(1,0);pk("veto",0,"reload");
 label("reload");C(3,41);R(0,3,"publish");
 label("publish");C(3,11);W(3,0,"entry");
 for(let i=0;i<g.length;i++){const n=g[i];if(n.kind==="Branch"){n.nonzero=l[n.nonzero];n.zero=l[n.zero]}else n.next=n.next===undefined?i+1:l[n.next];if(n.kind==="Const"&&typeof n.value==="string")n.value=l[n.value.slice(1)];}
 return {g,l};
}
function runTier(s,policy="tier",off=false){
 const {g,l}=buildTier(policy),initial=g.length,d=new Map(),v=[0,0,0,0];const load=q=>[50,51,58,57].forEach((k,i)=>d.set(k,q[i]));load(s.requests[0]);
 let pc=0,paid=0,prod=0,kernels=0,sk=0,sc=0,clones=0,peak=initial+1+d.size,entry=null,hit=null,exposure=null,stores=0,transfers=0,cellTime=0,expectedHit=null,expectedStore=null,expectedVictim=60,actualStore=null,start=null,cost=null,expectedVeto=0,measured=0,comparePaid=0,measurePaid=0,metadataPaid=0;const outputs=[],services=[],events=[],prefix=[],records=[null,null],costs=[0,0];
 const input=()=>[50,51,58,57].map(k=>d.get(k));
 while(paid<200000){
 const p=pc,n={...g[p]};if(!n.kind)throw Error("pc");
 if(p===l.activate&&exposure===null)exposure=paid+1;
 if(exposure===null&&policy==="tier")prefix.push([p,[...v],[...d],g.length,n]);
 if(p===l.entry){hit=null;actualStore=null;start=cost=null;const q=input();const found=records.findIndex(x=>x&&x.slice(0,4).join()===q.join());expectedHit=found<0?null:found;expectedStore=null;}
 if(p===l.hit1||p===l.hit2)hit=p===l.hit1?0:1;
 if(p===l.measure_start){measured++;}
 if(p===l.done){cost=input()[2]<=2?1:2;const victim=expectedVictim===60?0:1;const admit=policy==="metadata"||policy==="bounded"&&expectedVeto===1||cost>=costs[victim]||off;expectedStore=admit?expectedVictim:null;}
 if(p>=l.compare&&p<l.reject)comparePaid++;
 if(p>=l.measure_start&&p<l.compute||p>=l.done&&p<l.slot)measurePaid++;
 if(p>=l.cost1&&p<l.reload)metadataPaid++;
 if(p===l.kernel_dispatch){const [a,b,k,t]=input();if(v[1]!==a||v[2]!==rec(a,b,sk,t)||sk>=k)throw Error("kernel");entry=initial+a-1;sk++;kernels++;}
 if(p>=initial)sc++;paid++;pc=n.next;
 switch(n.kind){
 case "Const":v[n.dst]=n.value;break;
 case "SenseOwnCredits":v[n.dst]=200000-paid;break;
 case "Read":v[n.dst]=d.get(v[n.address])??0;break;
 case "Write":{const k=v[n.address],z=v[n.value];d.set(k,z);const q=input();
 if(k===59&&z!==key(...q))throw Error("key");
 if(k===60||k===64){const slot=k===60?0:1;if(k!==expectedStore||actualStore!==null||z!==key(...q)||d.get(k+1)!==rec(...q)||sk!==q[2]||cost===null)throw Error("record/victim");
 if(records[slot]&&records[slot][3]!==q[3])transfers++;records[slot]=[...q,rec(...q)];costs[slot]=cost;stores++;actualStore=k;expectedVictim=124-k;expectedVeto=0;}
 if(k===11){if(z!==rec(...q))throw Error("output "+s.id+" "+prod+" "+policy+" "+off);const reused=sk===0;
 if(reused!==(expectedHit!==null)||hit!==expectedHit||actualStore!==expectedStore)throw Error("policy "+s.id+" "+prod+" "+policy);
 if(reused){if(!records[hit]||records[hit].slice(0,4).join()!==q.join()||records[hit][4]!==z)throw Error("stale");expectedVictim=hit===0?64:60;}
 else {if(sk!==q[2])throw Error("count");if(expectedStore===null)expectedVeto=1;}
 if(sc!==sk*depth(q[0]))throw Error("clone count");
 if(g[l.slot].value!==expectedVictim||g[l.resident1].value!==costs[0]||g[l.resident2].value!==costs[1]||g[l.veto].value!==expectedVeto)throw Error("metadata");
 outputs.push(z);prod++;cellTime+=d.size;services.push({at:prod,paid,reused,stream:q[3],cost,cells:d.size,stored:actualStore});sk=sc=0;
 if(prod===s.requests.length)return {paid,outputs,services,events,exposure,prefix,peak,nodes:g.length,kernels,clones,stores,deleted:0,transfers,cellTime,measured,comparePaid,measurePaid,metadataPaid};load(s.requests[prod]);}break;}
 case "Add":v[n.dst]=v[n.lhs]+v[n.rhs];break;
 case "Sub":v[n.dst]=v[n.lhs]-v[n.rhs];break;
 case "Branch":pc=v[n.test]!==0?n.nonzero:n.zero;break;
 case "CloneNode":{const from=v[n.source],idx=g.length,x=idx-initial+1;if(from!==(x%2?l.parent_add:l.double_src))throw Error("clone");g.push({...g[from]});v[n.dst]=idx;clones++;break;}
 case "PatchConst":{const at=v[n.node],to=v[n.value];
 if(at===l.flag){if(p!==l.activate||to!==1||cost>=costs[expectedVictim===60?0:1])throw Error("veto");events.push({at:prod,cost,resident:costs[expectedVictim===60?0:1],paid,suppressed:off});if(!off)g[at].value=1;}
 
 else if(at===l.candidate){if(to!==cost)throw Error("tier");g[at].value=to;}
 else if(at===l.slot){if(![60,64].includes(to))throw Error("slot");g[at].value=to;}
 else if(at===l.veto){if(![0,1].includes(to))throw Error("veto bit");g[at].value=to;}
 else if([l.resident1,l.resident2].includes(at)){if(to!==cost||to!==costs[at===l.resident1?0:1])throw Error("resident cost");g[at].value=to;}
 else{if(at!==l.base||to!==initial-1)throw Error("base");g[at].value=to;}break;}
 case "PatchNext":{const at=v[n.node],to=v[n.target];if(!g[to]||g[at].next===undefined)throw Error("edge");if(at>=initial){const x=at-initial+1;if(to!==(x===1?l.out:x%2?at-1:initial+x/2-1))throw Error("generated");}else if(at===l.special_entry){if(to!==entry)throw Error("entry");}else if(at===l.kernel_dispatch){if(to!==l.base)throw Error("kernel");}else throw Error("patch");g[at].next=to;break;}
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


const rows=scenarios.map(s=>{
 const arms={shared:run(s,"shared"),adaptive:run(s),fifo:runGlobal(s,"fifo"),lru:runGlobal(s,"lru"),metadata:runTier(s,"metadata"),tier:runTier(s),bounded:runTier(s,"bounded"),off:runTier(s,"tier",true)};
 for(const q of Object.values(arms))if(q.outputs.join()!==arms.shared.outputs.join())throw Error("outputs");
 if(JSON.stringify(arms.tier.prefix)!==JSON.stringify(arms.off.prefix)||arms.tier.exposure!==arms.off.exposure)throw Error("prefix");
 if(arms.off.stores!==arms.metadata.stores||arms.off.kernels!==arms.metadata.kernels||arms.metadata.kernels!==arms.lru.kernels)throw Error("off schedule");
 const compact=q=>({paid:q.paid,kernels:q.kernels,nodes:q.nodes,peak:q.peak,stores:q.stores,transfers:q.transfers,reused:q.services.filter(x=>x.reused).length,classified:q.measured??0,comparePaid:q.comparePaid??0,classPaid:q.measurePaid??0,metadataPaid:q.metadataPaid??0,vetoes:q.events.length,cellTime:q.cellTime});
 return {id:s.id,k:s.k,n:s.n,pattern:s.pattern,...Object.fromEntries(Object.entries(arms).map(([k,q])=>[k,compact(q)]))};
});
const names=["adaptive","fifo","lru","metadata","tier","bounded","off"];
function agg(a){return Object.fromEntries(names.map(k=>[k,{net:a.reduce((s,r)=>s+r.shared.paid-r[k].paid,0),vsLru:a.reduce((s,r)=>s+r.lru.paid-r[k].paid,0),wins:a.filter(r=>r.shared.paid>r[k].paid).length,paid:a.reduce((s,r)=>s+r[k].paid,0),peak:Math.max(...a.map(r=>r[k].peak)),nodes:Math.max(...a.map(r=>r[k].nodes)),reused:a.reduce((s,r)=>s+r[k].reused,0),kernels:a.reduce((s,r)=>s+r[k].kernels,0),classified:a.reduce((s,r)=>s+r[k].classified,0),comparePaid:a.reduce((s,r)=>s+r[k].comparePaid,0),classPaid:a.reduce((s,r)=>s+r[k].classPaid,0),metadataPaid:a.reduce((s,r)=>s+r[k].metadataPaid,0),vetoes:a.reduce((s,r)=>s+r[k].vetoes,0)}]))}
const summary={all:agg(rows),original:agg(rows.filter(r=>patterns.includes(r.pattern))),novel:agg(rows.filter(r=>novel.includes(r.pattern))),patterns:Object.fromEntries([...patterns,...novel].map(k=>[k,agg(rows.filter(r=>r.pattern===k))])),worlds:rows.length*8};

assert.deepEqual(summary,{"all":{"adaptive":{"net":420710,"vsLru":7470,"wins":50,"paid":402668,"peak":200,"nodes":185,"reused":4437,"kernels":5638,"classified":0,"comparePaid":0,"classPaid":0,"metadataPaid":0,"vetoes":559},"fifo":{"net":378998,"vsLru":-34242,"wins":50,"paid":444380,"peak":183,"nodes":168,"reused":4162,"kernels":7151,"classified":0,"comparePaid":0,"classPaid":0,"metadataPaid":0,"vetoes":0},"lru":{"net":413240,"vsLru":0,"wins":50,"paid":410138,"peak":183,"nodes":168,"reused":4445,"kernels":5746,"classified":0,"comparePaid":0,"classPaid":0,"metadataPaid":0,"vetoes":0},"metadata":{"net":384941,"vsLru":-28299,"wins":50,"paid":438437,"peak":208,"nodes":193,"reused":4445,"kernels":5746,"classified":1315,"comparePaid":0,"classPaid":14334,"metadataPaid":14465,"vetoes":0},"tier":{"net":384999,"vsLru":-28241,"wins":50,"paid":438379,"peak":208,"nodes":193,"reused":4360,"kernels":5460,"classified":1400,"comparePaid":5996,"classPaid":14940,"metadataPaid":13244,"vetoes":196},"bounded":{"net":383373,"vsLru":-29867,"wins":50,"paid":440005,"peak":208,"nodes":193,"reused":4460,"kernels":5563,"classified":1300,"comparePaid":5281,"classPaid":14127,"metadataPaid":13937,"vetoes":33},"off":{"net":379203,"vsLru":-34037,"wins":50,"paid":444175,"peak":208,"nodes":193,"reused":4445,"kernels":5746,"classified":1315,"comparePaid":5463,"classPaid":14334,"metadataPaid":14465,"vetoes":55}},"original":{"adaptive":{"net":310050,"vsLru":-3123,"wins":38,"paid":328706,"peak":200,"nodes":185,"reused":3513,"kernels":4745,"classified":0,"comparePaid":0,"classPaid":0,"metadataPaid":0,"vetoes":411},"fifo":{"net":296400,"vsLru":-16773,"wins":38,"paid":342356,"peak":183,"nodes":168,"reused":3381,"kernels":5317,"classified":0,"comparePaid":0,"classPaid":0,"metadataPaid":0,"vetoes":0},"lru":{"net":313173,"vsLru":0,"wins":38,"paid":325583,"peak":183,"nodes":168,"reused":3558,"kernels":4550,"classified":0,"comparePaid":0,"classPaid":0,"metadataPaid":0,"vetoes":0},"metadata":{"net":290481,"vsLru":-22692,"wins":38,"paid":348275,"peak":208,"nodes":193,"reused":3558,"kernels":4550,"classified":1050,"comparePaid":0,"classPaid":11550,"metadataPaid":11550,"vetoes":0},"tier":{"net":286051,"vsLru":-27122,"wins":38,"paid":352705,"peak":208,"nodes":193,"reused":3558,"kernels":4550,"classified":1050,"comparePaid":4430,"classPaid":11550,"metadataPaid":11550,"vetoes":0},"bounded":{"net":283951,"vsLru":-29222,"wins":38,"paid":354805,"peak":208,"nodes":193,"reused":3558,"kernels":4550,"classified":1050,"comparePaid":4430,"classPaid":11550,"metadataPaid":11550,"vetoes":0},"off":{"net":286051,"vsLru":-27122,"wins":38,"paid":352705,"peak":208,"nodes":193,"reused":3558,"kernels":4550,"classified":1050,"comparePaid":4430,"classPaid":11550,"metadataPaid":11550,"vetoes":0}},"novel":{"adaptive":{"net":110660,"vsLru":10593,"wins":12,"paid":73962,"peak":200,"nodes":185,"reused":924,"kernels":893,"classified":0,"comparePaid":0,"classPaid":0,"metadataPaid":0,"vetoes":148},"fifo":{"net":82598,"vsLru":-17469,"wins":12,"paid":102024,"peak":183,"nodes":168,"reused":781,"kernels":1834,"classified":0,"comparePaid":0,"classPaid":0,"metadataPaid":0,"vetoes":0},"lru":{"net":100067,"vsLru":0,"wins":12,"paid":84555,"peak":183,"nodes":168,"reused":887,"kernels":1196,"classified":0,"comparePaid":0,"classPaid":0,"metadataPaid":0,"vetoes":0},"metadata":{"net":94460,"vsLru":-5607,"wins":12,"paid":90162,"peak":208,"nodes":193,"reused":887,"kernels":1196,"classified":265,"comparePaid":0,"classPaid":2784,"metadataPaid":2915,"vetoes":0},"tier":{"net":98948,"vsLru":-1119,"wins":12,"paid":85674,"peak":208,"nodes":193,"reused":802,"kernels":910,"classified":350,"comparePaid":1566,"classPaid":3390,"metadataPaid":1694,"vetoes":196},"bounded":{"net":99422,"vsLru":-645,"wins":12,"paid":85200,"peak":208,"nodes":193,"reused":902,"kernels":1013,"classified":250,"comparePaid":851,"classPaid":2577,"metadataPaid":2387,"vetoes":33},"off":{"net":93152,"vsLru":-6915,"wins":12,"paid":91470,"peak":208,"nodes":193,"reused":887,"kernels":1196,"classified":265,"comparePaid":1033,"classPaid":2784,"metadataPaid":2915,"vetoes":55}},"patterns":{"primary_only":{"adaptive":{"net":55588,"vsLru":386,"wins":6,"paid":23792,"peak":200,"nodes":185,"reused":558,"kernels":78,"classified":0,"comparePaid":0,"classPaid":0,"metadataPaid":0,"vetoes":6},"fifo":{"net":56894,"vsLru":1692,"wins":6,"paid":22486,"peak":183,"nodes":168,"reused":564,"kernels":52,"classified":0,"comparePaid":0,"classPaid":0,"metadataPaid":0,"vetoes":0},"lru":{"net":55202,"vsLru":0,"wins":6,"paid":24178,"peak":183,"nodes":168,"reused":564,"kernels":52,"classified":0,"comparePaid":0,"classPaid":0,"metadataPaid":0,"vetoes":0},"metadata":{"net":55466,"vsLru":264,"wins":6,"paid":23914,"peak":208,"nodes":193,"reused":564,"kernels":52,"classified":12,"comparePaid":0,"classPaid":132,"metadataPaid":132,"vetoes":0},"tier":{"net":55430,"vsLru":228,"wins":6,"paid":23950,"peak":208,"nodes":193,"reused":564,"kernels":52,"classified":12,"comparePaid":36,"classPaid":132,"metadataPaid":132,"vetoes":0},"bounded":{"net":55406,"vsLru":204,"wins":6,"paid":23974,"peak":208,"nodes":193,"reused":564,"kernels":52,"classified":12,"comparePaid":36,"classPaid":132,"metadataPaid":132,"vetoes":0},"off":{"net":55430,"vsLru":228,"wins":6,"paid":23950,"peak":208,"nodes":193,"reused":564,"kernels":52,"classified":12,"comparePaid":36,"classPaid":132,"metadataPaid":132,"vetoes":0}},"secondary_only":{"adaptive":{"net":59226,"vsLru":216,"wins":6,"paid":21978,"peak":198,"nodes":185,"reused":564,"kernels":52,"classified":0,"comparePaid":0,"classPaid":0,"metadataPaid":0,"vetoes":6},"fifo":{"net":60720,"vsLru":1710,"wins":6,"paid":20484,"peak":181,"nodes":168,"reused":570,"kernels":26,"classified":0,"comparePaid":0,"classPaid":0,"metadataPaid":0,"vetoes":0},"lru":{"net":59010,"vsLru":0,"wins":6,"paid":22194,"peak":181,"nodes":168,"reused":570,"kernels":26,"classified":0,"comparePaid":0,"classPaid":0,"metadataPaid":0,"vetoes":0},"metadata":{"net":59430,"vsLru":420,"wins":6,"paid":21774,"peak":206,"nodes":193,"reused":570,"kernels":26,"classified":6,"comparePaid":0,"classPaid":66,"metadataPaid":66,"vetoes":0},"tier":{"net":59412,"vsLru":402,"wins":6,"paid":21792,"peak":206,"nodes":193,"reused":570,"kernels":26,"classified":6,"comparePaid":18,"classPaid":66,"metadataPaid":66,"vetoes":0},"bounded":{"net":59400,"vsLru":390,"wins":6,"paid":21804,"peak":206,"nodes":193,"reused":570,"kernels":26,"classified":6,"comparePaid":18,"classPaid":66,"metadataPaid":66,"vetoes":0},"off":{"net":59412,"vsLru":402,"wins":6,"paid":21792,"peak":206,"nodes":193,"reused":570,"kernels":26,"classified":6,"comparePaid":18,"classPaid":66,"metadataPaid":66,"vetoes":0}},"alternating_streams":{"adaptive":{"net":27292,"vsLru":-1758,"wins":4,"paid":52696,"peak":200,"nodes":185,"reused":372,"kernels":884,"classified":0,"comparePaid":0,"classPaid":0,"metadataPaid":0,"vetoes":186},"fifo":{"net":16672,"vsLru":-12378,"wins":4,"paid":63316,"peak":183,"nodes":168,"reused":288,"kernels":1248,"classified":0,"comparePaid":0,"classPaid":0,"metadataPaid":0,"vetoes":0},"lru":{"net":29050,"vsLru":0,"wins":4,"paid":50938,"peak":183,"nodes":168,"reused":378,"kernels":858,"classified":0,"comparePaid":0,"classPaid":0,"metadataPaid":0,"vetoes":0},"metadata":{"net":24478,"vsLru":-4572,"wins":4,"paid":55510,"peak":208,"nodes":193,"reused":378,"kernels":858,"classified":198,"comparePaid":0,"classPaid":2178,"metadataPaid":2178,"vetoes":0},"tier":{"net":23636,"vsLru":-5414,"wins":4,"paid":56352,"peak":208,"nodes":193,"reused":378,"kernels":858,"classified":198,"comparePaid":842,"classPaid":2178,"metadataPaid":2178,"vetoes":0},"bounded":{"net":23240,"vsLru":-5810,"wins":4,"paid":56748,"peak":208,"nodes":193,"reused":378,"kernels":858,"classified":198,"comparePaid":842,"classPaid":2178,"metadataPaid":2178,"vetoes":0},"off":{"net":23636,"vsLru":-5414,"wins":4,"paid":56352,"peak":208,"nodes":193,"reused":378,"kernels":858,"classified":198,"comparePaid":842,"classPaid":2178,"metadataPaid":2178,"vetoes":0}},"primary_heavy":{"adaptive":{"net":37554,"vsLru":42,"wins":4,"paid":42032,"peak":200,"nodes":185,"reused":441,"kernels":585,"classified":0,"comparePaid":0,"classPaid":0,"metadataPaid":0,"vetoes":117},"fifo":{"net":29904,"vsLru":-7608,"wins":4,"paid":49682,"peak":183,"nodes":168,"reused":381,"kernels":845,"classified":0,"comparePaid":0,"classPaid":0,"metadataPaid":0,"vetoes":0},"lru":{"net":37512,"vsLru":0,"wins":4,"paid":42074,"peak":183,"nodes":168,"reused":441,"kernels":585,"classified":0,"comparePaid":0,"classPaid":0,"metadataPaid":0,"vetoes":0},"metadata":{"net":34578,"vsLru":-2934,"wins":4,"paid":45008,"peak":208,"nodes":193,"reused":441,"kernels":585,"classified":135,"comparePaid":0,"classPaid":1485,"metadataPaid":1485,"vetoes":0},"tier":{"net":34009,"vsLru":-3503,"wins":4,"paid":45577,"peak":208,"nodes":193,"reused":441,"kernels":585,"classified":135,"comparePaid":569,"classPaid":1485,"metadataPaid":1485,"vetoes":0},"bounded":{"net":33739,"vsLru":-3773,"wins":4,"paid":45847,"peak":208,"nodes":193,"reused":441,"kernels":585,"classified":135,"comparePaid":569,"classPaid":1485,"metadataPaid":1485,"vetoes":0},"off":{"net":34009,"vsLru":-3503,"wins":4,"paid":45577,"peak":208,"nodes":193,"reused":441,"kernels":585,"classified":135,"comparePaid":569,"classPaid":1485,"metadataPaid":1485,"vetoes":0}},"secondary_heavy":{"adaptive":{"net":48570,"vsLru":-1053,"wins":6,"paid":32428,"peak":200,"nodes":185,"reused":498,"kernels":338,"classified":0,"comparePaid":0,"classPaid":0,"metadataPaid":0,"vetoes":60},"fifo":{"net":46140,"vsLru":-3483,"wins":6,"paid":34858,"peak":183,"nodes":168,"reused":480,"kernels":416,"classified":0,"comparePaid":0,"classPaid":0,"metadataPaid":0,"vetoes":0},"lru":{"net":49623,"vsLru":0,"wins":6,"paid":31375,"peak":183,"nodes":168,"reused":507,"kernels":299,"classified":0,"comparePaid":0,"classPaid":0,"metadataPaid":0,"vetoes":0},"metadata":{"net":48405,"vsLru":-1218,"wins":6,"paid":32593,"peak":208,"nodes":193,"reused":507,"kernels":299,"classified":69,"comparePaid":0,"classPaid":759,"metadataPaid":759,"vetoes":0},"tier":{"net":48122,"vsLru":-1501,"wins":6,"paid":32876,"peak":208,"nodes":193,"reused":507,"kernels":299,"classified":69,"comparePaid":283,"classPaid":759,"metadataPaid":759,"vetoes":0},"bounded":{"net":47984,"vsLru":-1639,"wins":6,"paid":33014,"peak":208,"nodes":193,"reused":507,"kernels":299,"classified":69,"comparePaid":283,"classPaid":759,"metadataPaid":759,"vetoes":0},"off":{"net":48122,"vsLru":-1501,"wins":6,"paid":32876,"peak":208,"nodes":193,"reused":507,"kernels":299,"classified":69,"comparePaid":283,"classPaid":759,"metadataPaid":759,"vetoes":0}},"phase_shift":{"adaptive":{"net":55102,"vsLru":-694,"wins":6,"paid":25190,"peak":200,"nodes":185,"reused":546,"kernels":130,"classified":0,"comparePaid":0,"classPaid":0,"metadataPaid":0,"vetoes":12},"fifo":{"net":57470,"vsLru":1674,"wins":6,"paid":22822,"peak":183,"nodes":168,"reused":558,"kernels":78,"classified":0,"comparePaid":0,"classPaid":0,"metadataPaid":0,"vetoes":0},"lru":{"net":55796,"vsLru":0,"wins":6,"paid":24496,"peak":183,"nodes":168,"reused":558,"kernels":78,"classified":0,"comparePaid":0,"classPaid":0,"metadataPaid":0,"vetoes":0},"metadata":{"net":55904,"vsLru":108,"wins":6,"paid":24388,"peak":208,"nodes":193,"reused":558,"kernels":78,"classified":18,"comparePaid":0,"classPaid":198,"metadataPaid":198,"vetoes":0},"tier":{"net":55842,"vsLru":46,"wins":6,"paid":24450,"peak":208,"nodes":193,"reused":558,"kernels":78,"classified":18,"comparePaid":62,"classPaid":198,"metadataPaid":198,"vetoes":0},"bounded":{"net":55806,"vsLru":10,"wins":6,"paid":24486,"peak":208,"nodes":193,"reused":558,"kernels":78,"classified":18,"comparePaid":62,"classPaid":198,"metadataPaid":198,"vetoes":0},"off":{"net":55842,"vsLru":46,"wins":6,"paid":24450,"peak":208,"nodes":193,"reused":558,"kernels":78,"classified":18,"comparePaid":62,"classPaid":198,"metadataPaid":198,"vetoes":0}},"return":{"adaptive":{"net":53214,"vsLru":314,"wins":6,"paid":27078,"peak":200,"nodes":185,"reused":534,"kernels":182,"classified":0,"comparePaid":0,"classPaid":0,"metadataPaid":0,"vetoes":24},"fifo":{"net":54520,"vsLru":1620,"wins":6,"paid":25772,"peak":183,"nodes":168,"reused":540,"kernels":156,"classified":0,"comparePaid":0,"classPaid":0,"metadataPaid":0,"vetoes":0},"lru":{"net":52900,"vsLru":0,"wins":6,"paid":27392,"peak":183,"nodes":168,"reused":540,"kernels":156,"classified":0,"comparePaid":0,"classPaid":0,"metadataPaid":0,"vetoes":0},"metadata":{"net":52540,"vsLru":-360,"wins":6,"paid":27752,"peak":208,"nodes":193,"reused":540,"kernels":156,"classified":36,"comparePaid":0,"classPaid":396,"metadataPaid":396,"vetoes":0},"tier":{"net":52400,"vsLru":-500,"wins":6,"paid":27892,"peak":208,"nodes":193,"reused":540,"kernels":156,"classified":36,"comparePaid":140,"classPaid":396,"metadataPaid":396,"vetoes":0},"bounded":{"net":52328,"vsLru":-572,"wins":6,"paid":27964,"peak":208,"nodes":193,"reused":540,"kernels":156,"classified":36,"comparePaid":140,"classPaid":396,"metadataPaid":396,"vetoes":0},"off":{"net":52400,"vsLru":-500,"wins":6,"paid":27892,"peak":208,"nodes":193,"reused":540,"kernels":156,"classified":36,"comparePaid":140,"classPaid":396,"metadataPaid":396,"vetoes":0}},"no_locality":{"adaptive":{"net":-26496,"vsLru":-576,"wins":0,"paid":103512,"peak":198,"nodes":185,"reused":0,"kernels":2496,"classified":0,"comparePaid":0,"classPaid":0,"metadataPaid":0,"vetoes":0},"fifo":{"net":-25920,"vsLru":0,"wins":0,"paid":102936,"peak":183,"nodes":168,"reused":0,"kernels":2496,"classified":0,"comparePaid":0,"classPaid":0,"metadataPaid":0,"vetoes":0},"lru":{"net":-25920,"vsLru":0,"wins":0,"paid":102936,"peak":183,"nodes":168,"reused":0,"kernels":2496,"classified":0,"comparePaid":0,"classPaid":0,"metadataPaid":0,"vetoes":0},"metadata":{"net":-40320,"vsLru":-14400,"wins":0,"paid":117336,"peak":208,"nodes":193,"reused":0,"kernels":2496,"classified":576,"comparePaid":0,"classPaid":6336,"metadataPaid":6336,"vetoes":0},"tier":{"net":-42800,"vsLru":-16880,"wins":0,"paid":119816,"peak":208,"nodes":193,"reused":0,"kernels":2496,"classified":576,"comparePaid":2480,"classPaid":6336,"metadataPaid":6336,"vetoes":0},"bounded":{"net":-43952,"vsLru":-18032,"wins":0,"paid":120968,"peak":208,"nodes":193,"reused":0,"kernels":2496,"classified":576,"comparePaid":2480,"classPaid":6336,"metadataPaid":6336,"vetoes":0},"off":{"net":-42800,"vsLru":-16880,"wins":0,"paid":119816,"peak":208,"nodes":193,"reused":0,"kernels":2496,"classified":576,"comparePaid":2480,"classPaid":6336,"metadataPaid":6336,"vetoes":0}},"cost_asymmetry_primary":{"adaptive":{"net":15346,"vsLru":5024,"wins":2,"paid":17994,"peak":200,"nodes":185,"reused":124,"kernels":306,"classified":0,"comparePaid":0,"classPaid":0,"metadataPaid":0,"vetoes":62},"fifo":{"net":8896,"vsLru":-1426,"wins":2,"paid":24444,"peak":183,"nodes":168,"reused":96,"kernels":544,"classified":0,"comparePaid":0,"classPaid":0,"metadataPaid":0,"vetoes":0},"lru":{"net":10322,"vsLru":0,"wins":2,"paid":23018,"peak":183,"nodes":168,"reused":126,"kernels":514,"classified":0,"comparePaid":0,"classPaid":0,"metadataPaid":0,"vetoes":0},"metadata":{"net":8738,"vsLru":-1584,"wins":2,"paid":24602,"peak":208,"nodes":193,"reused":126,"kernels":514,"classified":66,"comparePaid":0,"classPaid":786,"metadataPaid":726,"vetoes":0},"tier":{"net":8416,"vsLru":-1906,"wins":2,"paid":24924,"peak":208,"nodes":193,"reused":126,"kernels":514,"classified":66,"comparePaid":322,"classPaid":786,"metadataPaid":726,"vetoes":0},"bounded":{"net":8284,"vsLru":-2038,"wins":2,"paid":25056,"peak":208,"nodes":193,"reused":126,"kernels":514,"classified":66,"comparePaid":322,"classPaid":786,"metadataPaid":726,"vetoes":0},"off":{"net":8416,"vsLru":-1906,"wins":2,"paid":24924,"peak":208,"nodes":193,"reused":126,"kernels":514,"classified":66,"comparePaid":322,"classPaid":786,"metadataPaid":726,"vetoes":0}},"cost_asymmetry_secondary":{"adaptive":{"net":3684,"vsLru":-6484,"wins":2,"paid":18008,"peak":200,"nodes":185,"reused":124,"kernels":306,"classified":0,"comparePaid":0,"classPaid":0,"metadataPaid":0,"vetoes":62},"fifo":{"net":3072,"vsLru":-7096,"wins":2,"paid":18620,"peak":183,"nodes":168,"reused":96,"kernels":320,"classified":0,"comparePaid":0,"classPaid":0,"metadataPaid":0,"vetoes":0},"lru":{"net":10168,"vsLru":0,"wins":2,"paid":11524,"peak":183,"nodes":168,"reused":126,"kernels":80,"classified":0,"comparePaid":0,"classPaid":0,"metadataPaid":0,"vetoes":0},"metadata":{"net":8770,"vsLru":-1398,"wins":2,"paid":12922,"peak":208,"nodes":193,"reused":126,"kernels":80,"classified":66,"comparePaid":0,"classPaid":600,"metadataPaid":726,"vetoes":0},"tier":{"net":8572,"vsLru":-1596,"wins":2,"paid":13120,"peak":208,"nodes":193,"reused":126,"kernels":80,"classified":66,"comparePaid":198,"classPaid":600,"metadataPaid":726,"vetoes":0},"bounded":{"net":8440,"vsLru":-1728,"wins":2,"paid":13252,"peak":208,"nodes":193,"reused":126,"kernels":80,"classified":66,"comparePaid":198,"classPaid":600,"metadataPaid":726,"vetoes":0},"off":{"net":8572,"vsLru":-1596,"wins":2,"paid":13120,"peak":208,"nodes":193,"reused":126,"kernels":80,"classified":66,"comparePaid":198,"classPaid":600,"metadataPaid":726,"vetoes":0}},"scan_pollution":{"adaptive":{"net":24894,"vsLru":11444,"wins":2,"paid":11102,"peak":200,"nodes":185,"reused":138,"kernels":96,"classified":0,"comparePaid":0,"classPaid":0,"metadataPaid":0,"vetoes":2},"fifo":{"net":2880,"vsLru":-10570,"wins":2,"paid":33116,"peak":183,"nodes":168,"reused":48,"kernels":816,"classified":0,"comparePaid":0,"classPaid":0,"metadataPaid":0,"vetoes":0},"lru":{"net":13450,"vsLru":0,"wins":2,"paid":22546,"peak":183,"nodes":168,"reused":94,"kernels":448,"classified":0,"comparePaid":0,"classPaid":0,"metadataPaid":0,"vetoes":0},"metadata":{"net":11140,"vsLru":-2310,"wins":2,"paid":24856,"peak":208,"nodes":193,"reused":94,"kernels":448,"classified":98,"comparePaid":0,"classPaid":1032,"metadataPaid":1078,"vetoes":0},"tier":{"net":24140,"vsLru":10690,"wins":2,"paid":11856,"peak":208,"nodes":193,"reused":140,"kernels":80,"classified":52,"comparePaid":248,"classPaid":480,"metadataPaid":66,"vetoes":46},"bounded":{"net":17700,"vsLru":4250,"wins":2,"paid":18296,"peak":208,"nodes":193,"reused":118,"kernels":256,"classified":74,"comparePaid":204,"classPaid":744,"metadataPaid":550,"vetoes":24},"off":{"net":10524,"vsLru":-2926,"wins":2,"paid":25472,"peak":208,"nodes":193,"reused":94,"kernels":448,"classified":98,"comparePaid":386,"classPaid":1032,"metadataPaid":1078,"vetoes":46}},"bursty_secondary":{"adaptive":{"net":11486,"vsLru":25,"wins":2,"paid":9444,"peak":200,"nodes":185,"reused":174,"kernels":67,"classified":0,"comparePaid":0,"classPaid":0,"metadataPaid":0,"vetoes":12},"fifo":{"net":11980,"vsLru":519,"wins":2,"paid":8950,"peak":183,"nodes":168,"reused":173,"kernels":54,"classified":0,"comparePaid":0,"classPaid":0,"metadataPaid":0,"vetoes":0},"lru":{"net":11461,"vsLru":0,"wins":2,"paid":9469,"peak":183,"nodes":168,"reused":173,"kernels":54,"classified":0,"comparePaid":0,"classPaid":0,"metadataPaid":0,"vetoes":0},"metadata":{"net":11182,"vsLru":-279,"wins":2,"paid":9748,"peak":208,"nodes":193,"reused":173,"kernels":54,"classified":19,"comparePaid":0,"classPaid":186,"metadataPaid":209,"vetoes":0},"tier":{"net":8822,"vsLru":-2639,"wins":2,"paid":12108,"peak":208,"nodes":193,"reused":130,"kernels":76,"classified":62,"comparePaid":294,"classPaid":564,"metadataPaid":88,"vetoes":54},"bounded":{"net":10740,"vsLru":-721,"wins":2,"paid":10190,"peak":208,"nodes":193,"reused":168,"kernels":59,"classified":24,"comparePaid":67,"classPaid":231,"metadataPaid":209,"vetoes":5},"off":{"net":11090,"vsLru":-371,"wins":2,"paid":9840,"peak":208,"nodes":193,"reused":173,"kernels":54,"classified":19,"comparePaid":67,"classPaid":186,"metadataPaid":209,"vetoes":5}},"alternating_distinct_cost":{"adaptive":{"net":37086,"vsLru":324,"wins":2,"paid":8126,"peak":200,"nodes":185,"reused":186,"kernels":48,"classified":0,"comparePaid":0,"classPaid":0,"metadataPaid":0,"vetoes":2},"fifo":{"net":37326,"vsLru":564,"wins":2,"paid":7886,"peak":183,"nodes":168,"reused":188,"kernels":32,"classified":0,"comparePaid":0,"classPaid":0,"metadataPaid":0,"vetoes":0},"lru":{"net":36762,"vsLru":0,"wins":2,"paid":8450,"peak":183,"nodes":168,"reused":188,"kernels":32,"classified":0,"comparePaid":0,"classPaid":0,"metadataPaid":0,"vetoes":0},"metadata":{"net":36846,"vsLru":84,"wins":2,"paid":8366,"peak":208,"nodes":193,"reused":188,"kernels":32,"classified":4,"comparePaid":0,"classPaid":48,"metadataPaid":44,"vetoes":0},"tier":{"net":36834,"vsLru":72,"wins":2,"paid":8378,"peak":208,"nodes":193,"reused":188,"kernels":32,"classified":4,"comparePaid":12,"classPaid":48,"metadataPaid":44,"vetoes":0},"bounded":{"net":36826,"vsLru":64,"wins":2,"paid":8386,"peak":208,"nodes":193,"reused":188,"kernels":32,"classified":4,"comparePaid":12,"classPaid":48,"metadataPaid":44,"vetoes":0},"off":{"net":36834,"vsLru":72,"wins":2,"paid":8378,"peak":208,"nodes":193,"reused":188,"kernels":32,"classified":4,"comparePaid":12,"classPaid":48,"metadataPaid":44,"vetoes":0}},"return_asymmetry":{"adaptive":{"net":18164,"vsLru":260,"wins":2,"paid":9288,"peak":200,"nodes":185,"reused":178,"kernels":70,"classified":0,"comparePaid":0,"classPaid":0,"metadataPaid":0,"vetoes":8},"fifo":{"net":18444,"vsLru":540,"wins":2,"paid":9008,"peak":183,"nodes":168,"reused":180,"kernels":68,"classified":0,"comparePaid":0,"classPaid":0,"metadataPaid":0,"vetoes":0},"lru":{"net":17904,"vsLru":0,"wins":2,"paid":9548,"peak":183,"nodes":168,"reused":180,"kernels":68,"classified":0,"comparePaid":0,"classPaid":0,"metadataPaid":0,"vetoes":0},"metadata":{"net":17784,"vsLru":-120,"wins":2,"paid":9668,"peak":208,"nodes":193,"reused":180,"kernels":68,"classified":12,"comparePaid":0,"classPaid":132,"metadataPaid":132,"vetoes":0},"tier":{"net":12164,"vsLru":-5740,"wins":2,"paid":15288,"peak":208,"nodes":193,"reused":92,"kernels":128,"classified":100,"comparePaid":492,"classPaid":912,"metadataPaid":44,"vetoes":96},"bounded":{"net":17432,"vsLru":-472,"wins":2,"paid":10020,"peak":208,"nodes":193,"reused":176,"kernels":72,"classified":16,"comparePaid":48,"classPaid":168,"metadataPaid":132,"vetoes":4},"off":{"net":17716,"vsLru":-188,"wins":2,"paid":9736,"peak":208,"nodes":193,"reused":180,"kernels":68,"classified":12,"comparePaid":48,"classPaid":132,"metadataPaid":132,"vetoes":4}}},"worlds":480});
console.log(JSON.stringify({tier:"NON_NATIVE_STRUCTURAL_CACHE_VALUE",summary,rows,scenarios,sources:Object.fromEntries(["metadata","tier","bounded"].map(k=>[k,buildTier(k)]))},null,2));
