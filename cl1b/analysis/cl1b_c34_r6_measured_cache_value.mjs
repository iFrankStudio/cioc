#!/usr/bin/env node
// C34-R6 measured-cost admission, all guest instructions paid.
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
 const arms={shared:run(s,"shared"),adaptive:run(s),fifo:runGlobal(s,"fifo"),lru:runGlobal(s,"lru"),measure:runValue(s,"measure"),value:runValue(s),bounded:runValue(s,"bounded"),off:runValue(s,"value",true)};
 for(const q of Object.values(arms))if(q.outputs.join()!==arms.shared.outputs.join())throw Error("outputs");
 if(JSON.stringify(arms.value.prefix)!==JSON.stringify(arms.off.prefix)||arms.value.exposure!==arms.off.exposure)throw Error("prefix");
 if(arms.off.stores!==arms.measure.stores||arms.off.kernels!==arms.measure.kernels||arms.measure.kernels!==arms.lru.kernels)throw Error("off schedule");
 const compact=q=>({paid:q.paid,kernels:q.kernels,nodes:q.nodes,peak:q.peak,stores:q.stores,transfers:q.transfers,reused:q.services.filter(x=>x.reused).length,measured:q.measured??0,comparePaid:q.comparePaid??0,measurePaid:q.measurePaid??0,metadataPaid:q.metadataPaid??0,vetoes:q.events.length,cellTime:q.cellTime});
 return {id:s.id,k:s.k,n:s.n,pattern:s.pattern,...Object.fromEntries(Object.entries(arms).map(([k,q])=>[k,compact(q)]))};
});
const names=["adaptive","fifo","lru","measure","value","bounded","off"];
function agg(a){return Object.fromEntries(names.map(k=>[k,{net:a.reduce((s,r)=>s+r.shared.paid-r[k].paid,0),vsLru:a.reduce((s,r)=>s+r.lru.paid-r[k].paid,0),wins:a.filter(r=>r.shared.paid>r[k].paid).length,paid:a.reduce((s,r)=>s+r[k].paid,0),peak:Math.max(...a.map(r=>r[k].peak)),nodes:Math.max(...a.map(r=>r[k].nodes)),reused:a.reduce((s,r)=>s+r[k].reused,0),kernels:a.reduce((s,r)=>s+r[k].kernels,0),measured:a.reduce((s,r)=>s+r[k].measured,0),comparePaid:a.reduce((s,r)=>s+r[k].comparePaid,0),measurePaid:a.reduce((s,r)=>s+r[k].measurePaid,0),metadataPaid:a.reduce((s,r)=>s+r[k].metadataPaid,0),vetoes:a.reduce((s,r)=>s+r[k].vetoes,0)}]))}
const summary={all:agg(rows),original:agg(rows.filter(r=>patterns.includes(r.pattern))),novel:agg(rows.filter(r=>novel.includes(r.pattern))),patterns:Object.fromEntries([...patterns,...novel].map(k=>[k,agg(rows.filter(r=>r.pattern===k))])),worlds:rows.length*8};

assert.deepEqual(summary,{"all":{"adaptive":{"net":420710,"vsLru":7470,"wins":50,"paid":402668,"peak":200,"nodes":185,"reused":4437,"kernels":5638,"measured":0,"comparePaid":0,"measurePaid":0,"metadataPaid":0,"vetoes":559},"fifo":{"net":378998,"vsLru":-34242,"wins":50,"paid":444380,"peak":183,"nodes":168,"reused":4162,"kernels":7151,"measured":0,"comparePaid":0,"measurePaid":0,"metadataPaid":0,"vetoes":0},"lru":{"net":413240,"vsLru":0,"wins":50,"paid":410138,"peak":183,"nodes":168,"reused":4445,"kernels":5746,"measured":0,"comparePaid":0,"measurePaid":0,"metadataPaid":0,"vetoes":0},"measure":{"net":388755,"vsLru":-24485,"wins":50,"paid":434623,"peak":204,"nodes":189,"reused":4445,"kernels":5746,"measured":1315,"comparePaid":0,"measurePaid":10520,"metadataPaid":14465,"vetoes":0},"value":{"net":-477796,"vsLru":-891036,"wins":29,"paid":1301174,"peak":204,"nodes":189,"reused":4212,"kernels":6102,"measured":1548,"comparePaid":864286,"measurePaid":12384,"metadataPaid":2970,"vetoes":1278},"bounded":{"net":-210019,"vsLru":-623259,"wins":36,"paid":1033397,"peak":204,"nodes":189,"reused":4264,"kernels":6423,"measured":1496,"comparePaid":575484,"measurePaid":11968,"metadataPaid":11264,"vetoes":472},"off":{"net":-348712,"vsLru":-761952,"wins":33,"paid":1172090,"peak":204,"nodes":189,"reused":4445,"kernels":5746,"measured":1315,"comparePaid":734907,"measurePaid":10520,"metadataPaid":14465,"vetoes":512}},"original":{"adaptive":{"net":310050,"vsLru":-3123,"wins":38,"paid":328706,"peak":200,"nodes":185,"reused":3513,"kernels":4745,"measured":0,"comparePaid":0,"measurePaid":0,"metadataPaid":0,"vetoes":411},"fifo":{"net":296400,"vsLru":-16773,"wins":38,"paid":342356,"peak":183,"nodes":168,"reused":3381,"kernels":5317,"measured":0,"comparePaid":0,"measurePaid":0,"metadataPaid":0,"vetoes":0},"lru":{"net":313173,"vsLru":0,"wins":38,"paid":325583,"peak":183,"nodes":168,"reused":3558,"kernels":4550,"measured":0,"comparePaid":0,"measurePaid":0,"metadataPaid":0,"vetoes":0},"measure":{"net":293631,"vsLru":-19542,"wins":38,"paid":345125,"peak":204,"nodes":189,"reused":3558,"kernels":4550,"measured":1050,"comparePaid":0,"measurePaid":8400,"metadataPaid":11550,"vetoes":0},"value":{"net":-444490,"vsLru":-757663,"wins":25,"paid":1083246,"peak":204,"nodes":189,"reused":3414,"kernels":5174,"measured":1194,"comparePaid":728498,"measurePaid":9552,"metadataPaid":2640,"vetoes":954},"bounded":{"net":-197202,"vsLru":-510375,"wins":28,"paid":835958,"peak":204,"nodes":189,"reused":3426,"kernels":5122,"measured":1182,"comparePaid":471993,"measurePaid":9456,"metadataPaid":8877,"vetoes":375},"off":{"net":-329072,"vsLru":-642245,"wins":27,"paid":967828,"peak":204,"nodes":189,"reused":3558,"kernels":4550,"measured":1050,"comparePaid":620738,"measurePaid":8400,"metadataPaid":11550,"vetoes":393}},"novel":{"adaptive":{"net":110660,"vsLru":10593,"wins":12,"paid":73962,"peak":200,"nodes":185,"reused":924,"kernels":893,"measured":0,"comparePaid":0,"measurePaid":0,"metadataPaid":0,"vetoes":148},"fifo":{"net":82598,"vsLru":-17469,"wins":12,"paid":102024,"peak":183,"nodes":168,"reused":781,"kernels":1834,"measured":0,"comparePaid":0,"measurePaid":0,"metadataPaid":0,"vetoes":0},"lru":{"net":100067,"vsLru":0,"wins":12,"paid":84555,"peak":183,"nodes":168,"reused":887,"kernels":1196,"measured":0,"comparePaid":0,"measurePaid":0,"metadataPaid":0,"vetoes":0},"measure":{"net":95124,"vsLru":-4943,"wins":12,"paid":89498,"peak":204,"nodes":189,"reused":887,"kernels":1196,"measured":265,"comparePaid":0,"measurePaid":2120,"metadataPaid":2915,"vetoes":0},"value":{"net":-33306,"vsLru":-133373,"wins":4,"paid":217928,"peak":204,"nodes":189,"reused":798,"kernels":928,"measured":354,"comparePaid":135788,"measurePaid":2832,"metadataPaid":330,"vetoes":324},"bounded":{"net":-12817,"vsLru":-112884,"wins":8,"paid":197439,"peak":204,"nodes":189,"reused":838,"kernels":1301,"measured":314,"comparePaid":103491,"measurePaid":2512,"metadataPaid":2387,"vetoes":97},"off":{"net":-19640,"vsLru":-119707,"wins":6,"paid":204262,"peak":204,"nodes":189,"reused":887,"kernels":1196,"measured":265,"comparePaid":114169,"measurePaid":2120,"metadataPaid":2915,"vetoes":119}},"patterns":{"primary_only":{"adaptive":{"net":55588,"vsLru":386,"wins":6,"paid":23792,"peak":200,"nodes":185,"reused":558,"kernels":78,"measured":0,"comparePaid":0,"measurePaid":0,"metadataPaid":0,"vetoes":6},"fifo":{"net":56894,"vsLru":1692,"wins":6,"paid":22486,"peak":183,"nodes":168,"reused":564,"kernels":52,"measured":0,"comparePaid":0,"measurePaid":0,"metadataPaid":0,"vetoes":0},"lru":{"net":55202,"vsLru":0,"wins":6,"paid":24178,"peak":183,"nodes":168,"reused":564,"kernels":52,"measured":0,"comparePaid":0,"measurePaid":0,"metadataPaid":0,"vetoes":0},"measure":{"net":55502,"vsLru":300,"wins":6,"paid":23878,"peak":204,"nodes":189,"reused":564,"kernels":52,"measured":12,"comparePaid":0,"measurePaid":96,"metadataPaid":132,"vetoes":0},"value":{"net":55490,"vsLru":288,"wins":6,"paid":23890,"peak":204,"nodes":189,"reused":564,"kernels":52,"measured":12,"comparePaid":12,"measurePaid":96,"metadataPaid":132,"vetoes":0},"bounded":{"net":55466,"vsLru":264,"wins":6,"paid":23914,"peak":204,"nodes":189,"reused":564,"kernels":52,"measured":12,"comparePaid":12,"measurePaid":96,"metadataPaid":132,"vetoes":0},"off":{"net":55490,"vsLru":288,"wins":6,"paid":23890,"peak":204,"nodes":189,"reused":564,"kernels":52,"measured":12,"comparePaid":12,"measurePaid":96,"metadataPaid":132,"vetoes":0}},"secondary_only":{"adaptive":{"net":59226,"vsLru":216,"wins":6,"paid":21978,"peak":198,"nodes":185,"reused":564,"kernels":52,"measured":0,"comparePaid":0,"measurePaid":0,"metadataPaid":0,"vetoes":6},"fifo":{"net":60720,"vsLru":1710,"wins":6,"paid":20484,"peak":181,"nodes":168,"reused":570,"kernels":26,"measured":0,"comparePaid":0,"measurePaid":0,"metadataPaid":0,"vetoes":0},"lru":{"net":59010,"vsLru":0,"wins":6,"paid":22194,"peak":181,"nodes":168,"reused":570,"kernels":26,"measured":0,"comparePaid":0,"measurePaid":0,"metadataPaid":0,"vetoes":0},"measure":{"net":59448,"vsLru":438,"wins":6,"paid":21756,"peak":202,"nodes":189,"reused":570,"kernels":26,"measured":6,"comparePaid":0,"measurePaid":48,"metadataPaid":66,"vetoes":0},"value":{"net":59442,"vsLru":432,"wins":6,"paid":21762,"peak":202,"nodes":189,"reused":570,"kernels":26,"measured":6,"comparePaid":6,"measurePaid":48,"metadataPaid":66,"vetoes":0},"bounded":{"net":59430,"vsLru":420,"wins":6,"paid":21774,"peak":202,"nodes":189,"reused":570,"kernels":26,"measured":6,"comparePaid":6,"measurePaid":48,"metadataPaid":66,"vetoes":0},"off":{"net":59442,"vsLru":432,"wins":6,"paid":21762,"peak":202,"nodes":189,"reused":570,"kernels":26,"measured":6,"comparePaid":6,"measurePaid":48,"metadataPaid":66,"vetoes":0}},"alternating_streams":{"adaptive":{"net":27292,"vsLru":-1758,"wins":4,"paid":52696,"peak":200,"nodes":185,"reused":372,"kernels":884,"measured":0,"comparePaid":0,"measurePaid":0,"metadataPaid":0,"vetoes":186},"fifo":{"net":16672,"vsLru":-12378,"wins":4,"paid":63316,"peak":183,"nodes":168,"reused":288,"kernels":1248,"measured":0,"comparePaid":0,"measurePaid":0,"metadataPaid":0,"vetoes":0},"lru":{"net":29050,"vsLru":0,"wins":4,"paid":50938,"peak":183,"nodes":168,"reused":378,"kernels":858,"measured":0,"comparePaid":0,"measurePaid":0,"metadataPaid":0,"vetoes":0},"measure":{"net":25072,"vsLru":-3978,"wins":4,"paid":54916,"peak":204,"nodes":189,"reused":378,"kernels":858,"measured":198,"comparePaid":0,"measurePaid":1584,"metadataPaid":2178,"vetoes":0},"value":{"net":-95776,"vsLru":-124826,"wins":0,"paid":175764,"peak":204,"nodes":189,"reused":378,"kernels":858,"measured":198,"comparePaid":123758,"measurePaid":1584,"metadataPaid":198,"vetoes":180},"bounded":{"net":-112912,"vsLru":-141962,"wins":0,"paid":192900,"peak":204,"nodes":189,"reused":288,"kernels":1248,"measured":288,"comparePaid":123668,"measurePaid":2304,"metadataPaid":2178,"vetoes":90},"off":{"net":-99046,"vsLru":-128096,"wins":0,"paid":179034,"peak":204,"nodes":189,"reused":378,"kernels":858,"measured":198,"comparePaid":123668,"measurePaid":1584,"metadataPaid":2178,"vetoes":90}},"primary_heavy":{"adaptive":{"net":37554,"vsLru":42,"wins":4,"paid":42032,"peak":200,"nodes":185,"reused":441,"kernels":585,"measured":0,"comparePaid":0,"measurePaid":0,"metadataPaid":0,"vetoes":117},"fifo":{"net":29904,"vsLru":-7608,"wins":4,"paid":49682,"peak":183,"nodes":168,"reused":381,"kernels":845,"measured":0,"comparePaid":0,"measurePaid":0,"metadataPaid":0,"vetoes":0},"lru":{"net":37512,"vsLru":0,"wins":4,"paid":42074,"peak":183,"nodes":168,"reused":441,"kernels":585,"measured":0,"comparePaid":0,"measurePaid":0,"metadataPaid":0,"vetoes":0},"measure":{"net":34983,"vsLru":-2529,"wins":4,"paid":44603,"peak":204,"nodes":189,"reused":441,"kernels":585,"measured":135,"comparePaid":0,"measurePaid":1080,"metadataPaid":1485,"vetoes":0},"value":{"net":-138160,"vsLru":-175672,"wins":0,"paid":217746,"peak":204,"nodes":189,"reused":327,"kernels":1079,"measured":249,"comparePaid":157690,"measurePaid":1992,"metadataPaid":198,"vetoes":231},"bounded":{"net":-57090,"vsLru":-94602,"wins":0,"paid":136676,"peak":204,"nodes":189,"reused":384,"kernels":832,"measured":192,"comparePaid":83020,"measurePaid":1536,"metadataPaid":1452,"vetoes":60},"off":{"net":-48337,"vsLru":-85849,"wins":0,"paid":127923,"peak":204,"nodes":189,"reused":441,"kernels":585,"measured":135,"comparePaid":83020,"measurePaid":1080,"metadataPaid":1485,"vetoes":60}},"secondary_heavy":{"adaptive":{"net":48570,"vsLru":-1053,"wins":6,"paid":32428,"peak":200,"nodes":185,"reused":498,"kernels":338,"measured":0,"comparePaid":0,"measurePaid":0,"metadataPaid":0,"vetoes":60},"fifo":{"net":46140,"vsLru":-3483,"wins":6,"paid":34858,"peak":183,"nodes":168,"reused":480,"kernels":416,"measured":0,"comparePaid":0,"measurePaid":0,"metadataPaid":0,"vetoes":0},"lru":{"net":49623,"vsLru":0,"wins":6,"paid":31375,"peak":183,"nodes":168,"reused":507,"kernels":299,"measured":0,"comparePaid":0,"measurePaid":0,"metadataPaid":0,"vetoes":0},"measure":{"net":48612,"vsLru":-1011,"wins":6,"paid":32386,"peak":204,"nodes":189,"reused":507,"kernels":299,"measured":69,"comparePaid":0,"measurePaid":552,"metadataPaid":759,"vetoes":0},"value":{"net":31036,"vsLru":-18587,"wins":4,"paid":49962,"peak":204,"nodes":189,"reused":531,"kernels":195,"measured":45,"comparePaid":21962,"measurePaid":360,"metadataPaid":198,"vetoes":27},"bounded":{"net":31262,"vsLru":-18361,"wins":4,"paid":49736,"peak":204,"nodes":189,"reused":522,"kernels":234,"measured":54,"comparePaid":19957,"measurePaid":432,"metadataPaid":429,"vetoes":15},"off":{"net":10571,"vsLru":-39052,"wins":4,"paid":70427,"peak":204,"nodes":189,"reused":507,"kernels":299,"measured":69,"comparePaid":37906,"measurePaid":552,"metadataPaid":759,"vetoes":27}},"phase_shift":{"adaptive":{"net":55102,"vsLru":-694,"wins":6,"paid":25190,"peak":200,"nodes":185,"reused":546,"kernels":130,"measured":0,"comparePaid":0,"measurePaid":0,"metadataPaid":0,"vetoes":12},"fifo":{"net":57470,"vsLru":1674,"wins":6,"paid":22822,"peak":183,"nodes":168,"reused":558,"kernels":78,"measured":0,"comparePaid":0,"measurePaid":0,"metadataPaid":0,"vetoes":0},"lru":{"net":55796,"vsLru":0,"wins":6,"paid":24496,"peak":183,"nodes":168,"reused":558,"kernels":78,"measured":0,"comparePaid":0,"measurePaid":0,"metadataPaid":0,"vetoes":0},"measure":{"net":55958,"vsLru":162,"wins":6,"paid":24334,"peak":204,"nodes":189,"reused":558,"kernels":78,"measured":18,"comparePaid":0,"measurePaid":144,"metadataPaid":198,"vetoes":0},"value":{"net":51960,"vsLru":-3836,"wins":6,"paid":28332,"peak":204,"nodes":189,"reused":558,"kernels":78,"measured":18,"comparePaid":3998,"measurePaid":144,"metadataPaid":198,"vetoes":0},"bounded":{"net":51924,"vsLru":-3872,"wins":6,"paid":28368,"peak":204,"nodes":189,"reused":558,"kernels":78,"measured":18,"comparePaid":3998,"measurePaid":144,"metadataPaid":198,"vetoes":0},"off":{"net":51960,"vsLru":-3836,"wins":6,"paid":28332,"peak":204,"nodes":189,"reused":558,"kernels":78,"measured":18,"comparePaid":3998,"measurePaid":144,"metadataPaid":198,"vetoes":0}},"return":{"adaptive":{"net":53214,"vsLru":314,"wins":6,"paid":27078,"peak":200,"nodes":185,"reused":534,"kernels":182,"measured":0,"comparePaid":0,"measurePaid":0,"metadataPaid":0,"vetoes":24},"fifo":{"net":54520,"vsLru":1620,"wins":6,"paid":25772,"peak":183,"nodes":168,"reused":540,"kernels":156,"measured":0,"comparePaid":0,"measurePaid":0,"metadataPaid":0,"vetoes":0},"lru":{"net":52900,"vsLru":0,"wins":6,"paid":27392,"peak":183,"nodes":168,"reused":540,"kernels":156,"measured":0,"comparePaid":0,"measurePaid":0,"metadataPaid":0,"vetoes":0},"measure":{"net":52648,"vsLru":-252,"wins":6,"paid":27644,"peak":204,"nodes":189,"reused":540,"kernels":156,"measured":36,"comparePaid":0,"measurePaid":288,"metadataPaid":396,"vetoes":0},"value":{"net":-6936,"vsLru":-59836,"wins":3,"paid":87228,"peak":204,"nodes":189,"reused":486,"kernels":390,"measured":90,"comparePaid":51902,"measurePaid":720,"metadataPaid":198,"vetoes":72},"bounded":{"net":40962,"vsLru":-11938,"wins":6,"paid":39330,"peak":204,"nodes":189,"reused":540,"kernels":156,"measured":36,"comparePaid":11976,"measurePaid":288,"metadataPaid":330,"vetoes":6},"off":{"net":36490,"vsLru":-16410,"wins":5,"paid":43802,"peak":204,"nodes":189,"reused":540,"kernels":156,"measured":36,"comparePaid":16098,"measurePaid":288,"metadataPaid":396,"vetoes":12}},"no_locality":{"adaptive":{"net":-26496,"vsLru":-576,"wins":0,"paid":103512,"peak":198,"nodes":185,"reused":0,"kernels":2496,"measured":0,"comparePaid":0,"measurePaid":0,"metadataPaid":0,"vetoes":0},"fifo":{"net":-25920,"vsLru":0,"wins":0,"paid":102936,"peak":183,"nodes":168,"reused":0,"kernels":2496,"measured":0,"comparePaid":0,"measurePaid":0,"metadataPaid":0,"vetoes":0},"lru":{"net":-25920,"vsLru":0,"wins":0,"paid":102936,"peak":183,"nodes":168,"reused":0,"kernels":2496,"measured":0,"comparePaid":0,"measurePaid":0,"metadataPaid":0,"vetoes":0},"measure":{"net":-38592,"vsLru":-12672,"wins":0,"paid":115608,"peak":204,"nodes":189,"reused":0,"kernels":2496,"measured":576,"comparePaid":0,"measurePaid":4608,"metadataPaid":6336,"vetoes":0},"value":{"net":-401546,"vsLru":-375626,"wins":0,"paid":478562,"peak":204,"nodes":189,"reused":0,"kernels":2496,"measured":576,"comparePaid":369170,"measurePaid":4608,"metadataPaid":1452,"vetoes":444},"bounded":{"net":-266244,"vsLru":-240324,"wins":0,"paid":343260,"peak":204,"nodes":189,"reused":0,"kernels":2496,"measured":576,"comparePaid":229356,"measurePaid":4608,"metadataPaid":4092,"vetoes":204},"off":{"net":-395642,"vsLru":-369722,"wins":0,"paid":472658,"peak":204,"nodes":189,"reused":0,"kernels":2496,"measured":576,"comparePaid":356030,"measurePaid":4608,"metadataPaid":6336,"vetoes":204}},"cost_asymmetry_primary":{"adaptive":{"net":15346,"vsLru":5024,"wins":2,"paid":17994,"peak":200,"nodes":185,"reused":124,"kernels":306,"measured":0,"comparePaid":0,"measurePaid":0,"metadataPaid":0,"vetoes":62},"fifo":{"net":8896,"vsLru":-1426,"wins":2,"paid":24444,"peak":183,"nodes":168,"reused":96,"kernels":544,"measured":0,"comparePaid":0,"measurePaid":0,"metadataPaid":0,"vetoes":0},"lru":{"net":10322,"vsLru":0,"wins":2,"paid":23018,"peak":183,"nodes":168,"reused":126,"kernels":514,"measured":0,"comparePaid":0,"measurePaid":0,"metadataPaid":0,"vetoes":0},"measure":{"net":8996,"vsLru":-1326,"wins":2,"paid":24344,"peak":204,"nodes":189,"reused":126,"kernels":514,"measured":66,"comparePaid":0,"measurePaid":528,"metadataPaid":726,"vetoes":0},"value":{"net":-63484,"vsLru":-73806,"wins":0,"paid":96824,"peak":204,"nodes":189,"reused":124,"kernels":530,"measured":68,"comparePaid":73092,"measurePaid":544,"metadataPaid":44,"vetoes":64},"bounded":{"net":-69914,"vsLru":-80236,"wins":0,"paid":103254,"peak":204,"nodes":189,"reused":94,"kernels":770,"measured":98,"comparePaid":70778,"measurePaid":784,"metadataPaid":726,"vetoes":32},"off":{"net":-61942,"vsLru":-72264,"wins":0,"paid":95282,"peak":204,"nodes":189,"reused":126,"kernels":514,"measured":66,"comparePaid":70778,"measurePaid":528,"metadataPaid":726,"vetoes":32}},"cost_asymmetry_secondary":{"adaptive":{"net":3684,"vsLru":-6484,"wins":2,"paid":18008,"peak":200,"nodes":185,"reused":124,"kernels":306,"measured":0,"comparePaid":0,"measurePaid":0,"metadataPaid":0,"vetoes":62},"fifo":{"net":3072,"vsLru":-7096,"wins":2,"paid":18620,"peak":183,"nodes":168,"reused":96,"kernels":320,"measured":0,"comparePaid":0,"measurePaid":0,"metadataPaid":0,"vetoes":0},"lru":{"net":10168,"vsLru":0,"wins":2,"paid":11524,"peak":183,"nodes":168,"reused":126,"kernels":80,"measured":0,"comparePaid":0,"measurePaid":0,"metadataPaid":0,"vetoes":0},"measure":{"net":8842,"vsLru":-1326,"wins":2,"paid":12850,"peak":204,"nodes":189,"reused":126,"kernels":80,"measured":66,"comparePaid":0,"measurePaid":528,"metadataPaid":726,"vetoes":0},"value":{"net":-5244,"vsLru":-15412,"wins":0,"paid":26936,"peak":204,"nodes":189,"reused":124,"kernels":82,"measured":68,"comparePaid":14852,"measurePaid":544,"metadataPaid":44,"vetoes":64},"bounded":{"net":-7824,"vsLru":-17992,"wins":0,"paid":29516,"peak":204,"nodes":189,"reused":94,"kernels":112,"measured":98,"comparePaid":14358,"measurePaid":784,"metadataPaid":726,"vetoes":32},"off":{"net":-5676,"vsLru":-15844,"wins":0,"paid":27368,"peak":204,"nodes":189,"reused":126,"kernels":80,"measured":66,"comparePaid":14358,"measurePaid":528,"metadataPaid":726,"vetoes":32}},"scan_pollution":{"adaptive":{"net":24894,"vsLru":11444,"wins":2,"paid":11102,"peak":200,"nodes":185,"reused":138,"kernels":96,"measured":0,"comparePaid":0,"measurePaid":0,"metadataPaid":0,"vetoes":2},"fifo":{"net":2880,"vsLru":-10570,"wins":2,"paid":33116,"peak":183,"nodes":168,"reused":48,"kernels":816,"measured":0,"comparePaid":0,"measurePaid":0,"metadataPaid":0,"vetoes":0},"lru":{"net":13450,"vsLru":0,"wins":2,"paid":22546,"peak":183,"nodes":168,"reused":94,"kernels":448,"measured":0,"comparePaid":0,"measurePaid":0,"metadataPaid":0,"vetoes":0},"measure":{"net":11388,"vsLru":-2062,"wins":2,"paid":24608,"peak":204,"nodes":189,"reused":94,"kernels":448,"measured":98,"comparePaid":0,"measurePaid":784,"metadataPaid":1078,"vetoes":0},"value":{"net":13314,"vsLru":-136,"wins":2,"paid":22682,"peak":204,"nodes":189,"reused":140,"kernels":80,"measured":52,"comparePaid":11138,"measurePaid":416,"metadataPaid":66,"vetoes":46},"bounded":{"net":6940,"vsLru":-6510,"wins":2,"paid":29056,"peak":204,"nodes":189,"reused":118,"kernels":256,"measured":74,"comparePaid":11116,"measurePaid":592,"metadataPaid":550,"vetoes":24},"off":{"net":-10636,"vsLru":-24086,"wins":0,"paid":46632,"peak":204,"nodes":189,"reused":94,"kernels":448,"measured":98,"comparePaid":21794,"measurePaid":784,"metadataPaid":1078,"vetoes":46}},"bursty_secondary":{"adaptive":{"net":11486,"vsLru":25,"wins":2,"paid":9444,"peak":200,"nodes":185,"reused":174,"kernels":67,"measured":0,"comparePaid":0,"measurePaid":0,"metadataPaid":0,"vetoes":12},"fifo":{"net":11980,"vsLru":519,"wins":2,"paid":8950,"peak":183,"nodes":168,"reused":173,"kernels":54,"measured":0,"comparePaid":0,"measurePaid":0,"metadataPaid":0,"vetoes":0},"lru":{"net":11461,"vsLru":0,"wins":2,"paid":9469,"peak":183,"nodes":168,"reused":173,"kernels":54,"measured":0,"comparePaid":0,"measurePaid":0,"metadataPaid":0,"vetoes":0},"measure":{"net":11216,"vsLru":-245,"wins":2,"paid":9714,"peak":204,"nodes":189,"reused":173,"kernels":54,"measured":19,"comparePaid":0,"measurePaid":152,"metadataPaid":209,"vetoes":0},"value":{"net":-4282,"vsLru":-15743,"wins":0,"paid":25212,"peak":204,"nodes":189,"reused":130,"kernels":76,"measured":62,"comparePaid":13466,"measurePaid":496,"metadataPaid":88,"vetoes":54},"bounded":{"net":7347,"vsLru":-4114,"wins":2,"paid":13583,"peak":204,"nodes":189,"reused":168,"kernels":59,"measured":24,"comparePaid":3499,"measurePaid":192,"metadataPaid":209,"vetoes":5},"off":{"net":7692,"vsLru":-3769,"wins":2,"paid":13238,"peak":204,"nodes":189,"reused":173,"kernels":54,"measured":19,"comparePaid":3499,"measurePaid":152,"metadataPaid":209,"vetoes":5}},"alternating_distinct_cost":{"adaptive":{"net":37086,"vsLru":324,"wins":2,"paid":8126,"peak":200,"nodes":185,"reused":186,"kernels":48,"measured":0,"comparePaid":0,"measurePaid":0,"metadataPaid":0,"vetoes":2},"fifo":{"net":37326,"vsLru":564,"wins":2,"paid":7886,"peak":183,"nodes":168,"reused":188,"kernels":32,"measured":0,"comparePaid":0,"measurePaid":0,"metadataPaid":0,"vetoes":0},"lru":{"net":36762,"vsLru":0,"wins":2,"paid":8450,"peak":183,"nodes":168,"reused":188,"kernels":32,"measured":0,"comparePaid":0,"measurePaid":0,"metadataPaid":0,"vetoes":0},"measure":{"net":36862,"vsLru":100,"wins":2,"paid":8350,"peak":204,"nodes":189,"reused":188,"kernels":32,"measured":4,"comparePaid":0,"measurePaid":32,"metadataPaid":44,"vetoes":0},"value":{"net":36858,"vsLru":96,"wins":2,"paid":8354,"peak":204,"nodes":189,"reused":188,"kernels":32,"measured":4,"comparePaid":4,"measurePaid":32,"metadataPaid":44,"vetoes":0},"bounded":{"net":36850,"vsLru":88,"wins":2,"paid":8362,"peak":204,"nodes":189,"reused":188,"kernels":32,"measured":4,"comparePaid":4,"measurePaid":32,"metadataPaid":44,"vetoes":0},"off":{"net":36858,"vsLru":96,"wins":2,"paid":8354,"peak":204,"nodes":189,"reused":188,"kernels":32,"measured":4,"comparePaid":4,"measurePaid":32,"metadataPaid":44,"vetoes":0}},"return_asymmetry":{"adaptive":{"net":18164,"vsLru":260,"wins":2,"paid":9288,"peak":200,"nodes":185,"reused":178,"kernels":70,"measured":0,"comparePaid":0,"measurePaid":0,"metadataPaid":0,"vetoes":8},"fifo":{"net":18444,"vsLru":540,"wins":2,"paid":9008,"peak":183,"nodes":168,"reused":180,"kernels":68,"measured":0,"comparePaid":0,"measurePaid":0,"metadataPaid":0,"vetoes":0},"lru":{"net":17904,"vsLru":0,"wins":2,"paid":9548,"peak":183,"nodes":168,"reused":180,"kernels":68,"measured":0,"comparePaid":0,"measurePaid":0,"metadataPaid":0,"vetoes":0},"measure":{"net":17820,"vsLru":-84,"wins":2,"paid":9632,"peak":204,"nodes":189,"reused":180,"kernels":68,"measured":12,"comparePaid":0,"measurePaid":96,"metadataPaid":132,"vetoes":0},"value":{"net":-10468,"vsLru":-28372,"wins":0,"paid":37920,"peak":204,"nodes":189,"reused":92,"kernels":128,"measured":100,"comparePaid":23236,"measurePaid":800,"metadataPaid":44,"vetoes":96},"bounded":{"net":13784,"vsLru":-4120,"wins":2,"paid":13668,"peak":204,"nodes":189,"reused":176,"kernels":72,"measured":16,"comparePaid":3736,"measurePaid":128,"metadataPaid":132,"vetoes":4},"off":{"net":14064,"vsLru":-3840,"wins":2,"paid":13388,"peak":204,"nodes":189,"reused":180,"kernels":68,"measured":12,"comparePaid":3736,"measurePaid":96,"metadataPaid":132,"vetoes":4}}},"worlds":480});
console.log(JSON.stringify({tier:"NON_NATIVE_MEASURED_CACHE_VALUE",summary,rows,scenarios,sources:Object.fromEntries(["measure","value","bounded"].map(k=>[k,buildValue(k)]))},null,2));
