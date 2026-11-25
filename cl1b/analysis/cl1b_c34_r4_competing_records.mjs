#!/usr/bin/env node
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
const rows=scenarios.map(s=>{
 const arms={shared:run(s,"shared"),primary2:run(s,"primary2"),split:run(s,"split"),adaptive:run(s),off:run(s,"adaptive",true)};
 for(const q of Object.values(arms))if(q.outputs.join()!==arms.shared.outputs.join())throw Error("outputs");
 if(JSON.stringify(arms.adaptive.prefix)!==JSON.stringify(arms.off.prefix)||arms.adaptive.exposure!==arms.off.exposure)throw Error("prefix");
 if(arms.off.deleted||arms.off.transfers||arms.off.services.some(x=>x.owner!==null))throw Error("off");
 const compact=q=>({paid:q.paid,kernels:q.kernels,nodes:q.nodes,peak:q.peak,stores:q.stores,deleted:q.deleted,transfers:q.transfers,events:q.events,reused:q.services.filter(x=>x.reused).length,pHits:q.services.filter(x=>x.reused&&x.stream===0).length,sHits:q.services.filter(x=>x.reused&&x.stream===1).length,cellTime:q.cellTime});
 return {id:s.id,k:s.k,n:s.n,pattern:s.pattern,...Object.fromEntries(Object.entries(arms).map(([k,q])=>[k,compact(q)]))};
});
function agg(a){return Object.fromEntries(["primary2","split","adaptive","off"].map(k=>[k,{net:a.reduce((s,r)=>s+r.shared.paid-r[k].paid,0),wins:a.filter(r=>r.shared.paid>r[k].paid).length,paid:a.reduce((s,r)=>s+r[k].paid,0),peak:Math.max(...a.map(r=>r[k].peak)),nodes:Math.max(...a.map(r=>r[k].nodes)),pHits:a.reduce((s,r)=>s+r[k].pHits,0),sHits:a.reduce((s,r)=>s+r[k].sHits,0),transfers:a.reduce((s,r)=>s+r[k].transfers,0),deleted:a.reduce((s,r)=>s+r[k].deleted,0),admissions:a.reduce((s,r)=>s+r[k].events.length,0),cellTime:a.reduce((s,r)=>s+r[k].cellTime,0)}]))}
const summary={all:agg(rows),patterns:Object.fromEntries(patterns.map(k=>[k,agg(rows.filter(r=>r.pattern===k))])),scales:Object.fromEntries([1,4,8].map(k=>[k,agg(rows.filter(r=>r.k===k))])),bothDirections:rows.filter(r=>r.adaptive.events.some(e=>e.owner===0&&e.stream===1)&&r.adaptive.events.some(e=>e.owner===1&&e.stream===0)).map(r=>r.id),worlds:rows.length*5};

assert.deepEqual(summary,{"all":{"primary2":{"net":122480,"wins":24,"paid":516276,"peak":200,"nodes":185,"pHits":2040,"sHits":0,"transfers":0,"deleted":0,"admissions":0,"cellTime":61800},"split":{"net":120763,"wins":26,"paid":517993,"peak":200,"nodes":185,"pHits":252,"sHits":1884,"transfers":0,"deleted":564,"admissions":0,"cellTime":61116},"adaptive":{"net":310050,"wins":38,"paid":328706,"peak":200,"nodes":185,"pHits":1839,"sHits":1674,"transfers":369,"deleted":738,"admissions":411,"cellTime":61932},"off":{"net":-199385,"wins":2,"paid":838141,"peak":198,"nodes":185,"pHits":252,"sHits":0,"transfers":0,"deleted":0,"admissions":3672,"cellTime":54048}},"patterns":{"primary_only":{"primary2":{"net":56552,"wins":6,"paid":22828,"peak":200,"nodes":185,"pHits":564,"sHits":0,"transfers":0,"deleted":0,"admissions":0,"cellTime":8052},"split":{"net":-24192,"wins":0,"paid":103572,"peak":198,"nodes":185,"pHits":0,"sHits":0,"transfers":0,"deleted":0,"admissions":0,"cellTime":6912},"adaptive":{"net":55588,"wins":6,"paid":23792,"peak":200,"nodes":185,"pHits":558,"sHits":0,"transfers":0,"deleted":0,"admissions":6,"cellTime":8040},"off":{"net":-35040,"wins":0,"paid":114420,"peak":198,"nodes":185,"pHits":0,"sHits":0,"transfers":0,"deleted":0,"admissions":564,"cellTime":6912}},"secondary_only":{"primary2":{"net":-17280,"wins":0,"paid":98484,"peak":196,"nodes":185,"pHits":0,"sHits":0,"transfers":0,"deleted":0,"admissions":0,"cellTime":5760},"split":{"net":60144,"wins":6,"paid":21060,"peak":198,"nodes":185,"pHits":0,"sHits":570,"transfers":0,"deleted":0,"admissions":0,"cellTime":6912},"adaptive":{"net":59226,"wins":6,"paid":21978,"peak":198,"nodes":185,"pHits":0,"sHits":564,"transfers":0,"deleted":0,"admissions":6,"cellTime":6900},"off":{"net":-25884,"wins":0,"paid":107088,"peak":196,"nodes":185,"pHits":0,"sHits":0,"transfers":0,"deleted":0,"admissions":570,"cellTime":5760}},"alternating_streams":{"primary2":{"net":31336,"wins":4,"paid":48652,"peak":200,"nodes":185,"pHits":372,"sHits":0,"transfers":0,"deleted":0,"admissions":0,"cellTime":8028},"split":{"net":31312,"wins":4,"paid":48676,"peak":200,"nodes":185,"pHits":192,"sHits":186,"transfers":0,"deleted":0,"admissions":0,"cellTime":8040},"adaptive":{"net":27292,"wins":4,"paid":52696,"peak":200,"nodes":185,"pHits":282,"sHits":90,"transfers":180,"deleted":360,"admissions":186,"cellTime":8004},"off":{"net":-284,"wins":2,"paid":80272,"peak":198,"nodes":185,"pHits":192,"sHits":0,"transfers":0,"deleted":0,"admissions":366,"cellTime":6912}},"primary_heavy":{"primary2":{"net":48277,"wins":6,"paid":31309,"peak":200,"nodes":185,"pHits":501,"sHits":0,"transfers":0,"deleted":0,"admissions":0,"cellTime":8052},"split":{"net":-7017,"wins":2,"paid":86603,"peak":200,"nodes":185,"pHits":60,"sHits":57,"transfers":0,"deleted":0,"admissions":0,"cellTime":7968},"adaptive":{"net":37554,"wins":4,"paid":42032,"peak":200,"nodes":185,"pHits":441,"sHits":0,"transfers":111,"deleted":222,"admissions":117,"cellTime":8040},"off":{"net":-24093,"wins":0,"paid":103679,"peak":198,"nodes":185,"pHits":60,"sHits":0,"transfers":0,"deleted":0,"admissions":498,"cellTime":6912}},"secondary_heavy":{"primary2":{"net":-10821,"wins":0,"paid":91819,"peak":200,"nodes":185,"pHits":51,"sHits":0,"transfers":0,"deleted":0,"admissions":0,"cellTime":7764},"split":{"net":50820,"wins":6,"paid":30178,"peak":200,"nodes":185,"pHits":0,"sHits":507,"transfers":0,"deleted":0,"admissions":0,"cellTime":7968},"adaptive":{"net":48570,"wins":6,"paid":32428,"peak":200,"nodes":185,"pHits":24,"sHits":474,"transfers":54,"deleted":108,"admissions":60,"cellTime":7956},"off":{"net":-26796,"wins":0,"paid":107794,"peak":198,"nodes":185,"pHits":0,"sHits":0,"transfers":0,"deleted":0,"admissions":558,"cellTime":6816}},"phase_shift":{"primary2":{"net":18728,"wins":4,"paid":61564,"peak":200,"nodes":185,"pHits":276,"sHits":0,"transfers":0,"deleted":0,"admissions":0,"cellTime":8052},"split":{"net":17520,"wins":4,"paid":62772,"peak":200,"nodes":185,"pHits":0,"sHits":282,"transfers":0,"deleted":0,"admissions":0,"cellTime":7488},"adaptive":{"net":55102,"wins":6,"paid":25190,"peak":200,"nodes":185,"pHits":270,"sHits":276,"transfers":6,"deleted":12,"admissions":12,"cellTime":8040},"off":{"net":-30396,"wins":0,"paid":110688,"peak":198,"nodes":185,"pHits":0,"sHits":0,"transfers":0,"deleted":0,"admissions":558,"cellTime":6912}},"return":{"primary2":{"net":18728,"wins":4,"paid":61564,"peak":200,"nodes":185,"pHits":276,"sHits":0,"transfers":0,"deleted":0,"admissions":0,"cellTime":8052},"split":{"net":17520,"wins":4,"paid":62772,"peak":200,"nodes":185,"pHits":0,"sHits":282,"transfers":0,"deleted":0,"admissions":0,"cellTime":7776},"adaptive":{"net":53214,"wins":6,"paid":27078,"peak":200,"nodes":185,"pHits":264,"sHits":270,"transfers":18,"deleted":36,"admissions":24,"cellTime":8040},"off":{"net":-30396,"wins":0,"paid":110688,"peak":198,"nodes":185,"pHits":0,"sHits":0,"transfers":0,"deleted":0,"admissions":558,"cellTime":6912}},"no_locality":{"primary2":{"net":-23040,"wins":0,"paid":100056,"peak":200,"nodes":185,"pHits":0,"sHits":0,"transfers":0,"deleted":0,"admissions":0,"cellTime":8040},"split":{"net":-25344,"wins":0,"paid":102360,"peak":200,"nodes":185,"pHits":0,"sHits":0,"transfers":0,"deleted":564,"admissions":0,"cellTime":8052},"adaptive":{"net":-26496,"wins":0,"paid":103512,"peak":198,"nodes":185,"pHits":0,"sHits":0,"transfers":0,"deleted":0,"admissions":0,"cellTime":6912},"off":{"net":-26496,"wins":0,"paid":103512,"peak":198,"nodes":185,"pHits":0,"sHits":0,"transfers":0,"deleted":0,"admissions":0,"cellTime":6912}}},"scales":{"1":{"primary2":{"net":-19240,"wins":4,"paid":96092,"peak":200,"nodes":185,"pHits":680,"sHits":0,"transfers":0,"deleted":0,"admissions":0,"cellTime":20600},"split":{"net":-23689,"wins":4,"paid":100541,"peak":200,"nodes":185,"pHits":84,"sHits":628,"transfers":0,"deleted":188,"admissions":0,"cellTime":20372},"adaptive":{"net":-1130,"wins":10,"paid":77982,"peak":200,"nodes":185,"pHits":613,"sHits":558,"transfers":123,"deleted":246,"admissions":137,"cellTime":20644},"off":{"net":-73885,"wins":0,"paid":150737,"peak":198,"nodes":185,"pHits":84,"sHits":0,"transfers":0,"deleted":0,"admissions":1224,"cellTime":18016}},"4":{"primary2":{"net":34820,"wins":10,"paid":164492,"peak":200,"nodes":185,"pHits":680,"sHits":0,"transfers":0,"deleted":0,"admissions":0,"cellTime":20600},"split":{"net":33860,"wins":10,"paid":165452,"peak":200,"nodes":185,"pHits":84,"sHits":628,"transfers":0,"deleted":188,"admissions":0,"cellTime":20372},"adaptive":{"net":92902,"wins":14,"paid":106410,"peak":200,"nodes":185,"pHits":613,"sHits":558,"transfers":123,"deleted":246,"admissions":137,"cellTime":20644},"off":{"net":-67204,"wins":0,"paid":266516,"peak":198,"nodes":185,"pHits":84,"sHits":0,"transfers":0,"deleted":0,"admissions":1224,"cellTime":18016}},"8":{"primary2":{"net":106900,"wins":10,"paid":255692,"peak":200,"nodes":185,"pHits":680,"sHits":0,"transfers":0,"deleted":0,"admissions":0,"cellTime":20600},"split":{"net":110592,"wins":12,"paid":252000,"peak":200,"nodes":185,"pHits":84,"sHits":628,"transfers":0,"deleted":188,"admissions":0,"cellTime":20372},"adaptive":{"net":218278,"wins":14,"paid":144314,"peak":200,"nodes":185,"pHits":613,"sHits":558,"transfers":123,"deleted":246,"admissions":137,"cellTime":20644},"off":{"net":-58296,"wins":2,"paid":420888,"peak":198,"nodes":185,"pHits":84,"sHits":0,"transfers":0,"deleted":0,"admissions":1224,"cellTime":18016}}},"bothDirections":["1_48_alternating_streams","1_48_primary_heavy","1_48_secondary_heavy","1_144_alternating_streams","1_144_primary_heavy","1_144_secondary_heavy","1_144_return","4_48_alternating_streams","4_48_primary_heavy","4_48_secondary_heavy","4_144_alternating_streams","4_144_primary_heavy","4_144_secondary_heavy","4_144_return","8_48_alternating_streams","8_48_primary_heavy","8_48_secondary_heavy","8_144_alternating_streams","8_144_primary_heavy","8_144_secondary_heavy","8_144_return"],"worlds":240});
console.log(JSON.stringify({tier:"NON_NATIVE_COMPETING_RECORD_ALLOCATION",summary,rows,scenarios,sources:Object.fromEntries(["shared","primary2","split","adaptive"].map(k=>[k,buildCompete(k)]))},null,2));
