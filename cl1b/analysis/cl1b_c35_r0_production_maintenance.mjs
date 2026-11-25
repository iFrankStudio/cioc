#!/usr/bin/env node
// C35-R0: guest production-feedback-triggered structural maintenance.
import assert from "node:assert/strict";

function build(policy="reactive"){
 const g=[],l={};const label=k=>{if(k in l)throw Error(k);l[k]=g.length};const op=(kind,args)=>g.push({kind,...args});
 const C=(dst,value,next)=>op("Const",{dst,value,next}),R=(dst,address,next)=>op("Read",{dst,address,next}),W=(address,value,next)=>op("Write",{address,value,next}),S=(dst,lhs,rhs,next)=>op("Sub",{dst,lhs,rhs,next}),A=(dst,lhs,rhs,next)=>op("Add",{dst,lhs,rhs,next}),B=(test,nonzero,zero)=>op("Branch",{test,nonzero,zero}),Q=(dst,next)=>op("SenseOwnCredits",{dst,next}),P=(node,target,next)=>op("PatchNext",{node,target,next}),K=(node,value,next)=>op("PatchConst",{node,value,next});
 const set=(key,value,next)=>{C(3,key);C(0,value);W(3,0,next)};
 const pe=(at,to,next)=>{C(1,"@"+at);C(0,"@"+to);P(1,0,next)};
 label("init");C(0,0,policy==="direct"?"prod_init":"generate_init");
 label("generate_init");C(2,8);label("double_src");A(2,2,2);C(1,"@parent_add");op("CloneNode",{dst:3,source:1});C(2,"@out");P(3,2);
 C(2,1);S(2,3,2);C(1,"@base");K(1,2);C(1,"@repair_anchor");K(1,2);C(2,0);A(0,3,2);C(2,7);A(1,0,2);
 label("generate_even");C(2,"@double_src");op("CloneNode",{dst:3,source:2});P(3,0);S(2,0,1);B(2,"generate_odd","generate_done");
 label("generate_odd");C(2,"@parent_add");op("CloneNode",{dst:2,source:2});P(2,3);C(2,1);A(0,0,2,"generate_even");
 label("generate_done");C(1,"@repair_last");K(1,3);pe("kernel_dispatch","base","prod_init");








 const N=(dst,node,next)=>op("ReadNext",{dst,node,next});
 const pk=(at,val,next)=>{C(1,"@"+at);C(0,val);K(1,0,next)};
 label("prod_init");C(0,0,"batch");
 label("batch");C(0,0,policy==="reactive"?"reset_alarm":policy==="sweep"?"repair_anchor":"pair");label("reset_alarm");pe("alarm","repair_anchor","pair");
 label("pair");pe("return_route","first");C(3,50);R(1,3);C(3,51);R(2,3,"kernel_dispatch");
 label("second_call");pe("return_route","second");C(3,50);R(1,3);C(0,1);A(1,1,0);C(3,51);R(2,3,"kernel_dispatch");
 label("kernel_dispatch");C(0,0,"parent");
 label("parent");C(3,1);
 label("parent_add");A(0,0,2);S(1,1,3);B(1,"parent_add","out");
 label("base");C(3,0);A(1,1,3);C(3,"@special_entry");P(3,1);
 label("special_entry");C(0,0,"parent_add");
 label("out");label("return_route");C(1,0,"first");
 label("first");C(3,11);W(3,0,"second_call");
 label("second");C(3,12);W(3,0,["direct","unchecked"].includes(policy)?"publish":"check");
 label("check");C(3,11);R(1,3);C(3,51);R(2,3);A(1,1,2);S(2,0,1);B(2,"alarm","publish");
 label("alarm");C(0,0,policy==="reactive"?"repair_anchor":policy==="fallback"?"fallback":"publish");
 // fallback redoes both required products on parent, then restores shared dispatch.
 label("fallback");pe("kernel_dispatch","parent");pe("second_tail","restore","pair");
 label("restore");pe("kernel_dispatch","base");pe("second_tail","batch","batch");
 // Full paid structural scan, actual returned clone addresses, edge-only repair.
 label("repair_anchor");C(0,0);C(2,1);A(0,0,2);C(1,"@out");N(2,0);S(2,2,1);B(2,"fix_first","init_even");
 label("fix_first");P(0,1,"init_even");
 label("init_even");C(2,0);A(3,0,2);C(2,1);A(0,0,2,"even");
 label("even");C(2,0);A(1,3,2);N(2,0);S(2,2,1);B(2,"fix_even","bound");
 label("fix_even");P(0,1,"bound");
 label("bound");label("repair_last");C(2,0);S(2,0,2);B(2,"odd","finish");
 label("odd");C(2,0);A(1,0,2);C(2,1);A(0,0,2);N(2,0);S(2,2,1);B(2,"fix_odd","advance");
 label("fix_odd");P(0,1,"advance");
 label("advance");C(2,1);A(0,0,2);A(3,3,2,"even");
 label("finish");C(0,0,policy==="reactive"?"bound_retry":"pair");label("bound_retry");pe("alarm","publish","pair");
 label("publish");C(3,12);R(0,3);C(3,13);W(3,0,"second_tail");
 label("second_tail");C(0,0,"batch");
 for(let i=0;i<g.length;i++){const n=g[i];if(n.kind==="Branch"){n.nonzero=l[n.nonzero];n.zero=l[n.zero]}else n.next=n.next===undefined?i+1:l[n.next];if(n.kind==="Const"&&typeof n.value==="string")n.value=l[n.value.slice(1)];}
 return {g,l};
}
function expectedEdge(i,initial,l){return i===1?l.out:i%2===0?initial+i/2-1:initial+i-2;}
function runWorld(s,policy="reactive",off=false){
 const {g,l}=build(policy),initial=g.length,d=new Map(),v=[0,0,0,0];const load=i=>{d.set(50,s.requests[i][0]);d.set(51,s.requests[i][1]);};load(0);
 let pc=0,paid=0,prod=0,peak=initial+1+d.size,clones=0,steps=0,kernels=0,repairs=0,reads=0,sweeps=0,alarms=0,exposure=null,repairAttempts=0,requestStart=0,repairPaid=0,sk=0,firstPair=null;const outputs=[],services=[],events=[],prefix=[],injected=[];
 const input=()=>[d.get(50),d.get(51)];
 const inject=()=>{for(const f of s.faults.filter(f=>f.at===prod)){if(policy==="direct")continue;const at=initial+f.node-1;if(!g[at]||at>=g.length)throw Error("fault target");const old=g[at].next,to=f.to===0?l.out:initial+f.to-1;if(to===old||f.to>=f.node)throw Error("fault ineffective");g[at].next=to;injected.push({at:prod,node:f.node,old,to});}};
 while(paid<200000){
 const p=pc,n={...g[p]};if(!n.kind)throw Error("pc");
 if(p===l.batch){requestStart=paid;sk=0;firstPair=null;repairAttempts=0;inject();}
 if(p===l.repair_anchor){sweeps++;repairAttempts++;if(repairAttempts>1)throw Error("repeat repair");}
 if(p===l.alarm)alarms++;
 if([l.fix_first,l.fix_even,l.fix_odd].includes(p)&&exposure===null)exposure=paid+1;
 if(exposure===null&&policy==="reactive")prefix.push([p,[...v],[...d],g.length,n]);
 if(p>=l.repair_anchor&&p<l.publish)repairPaid++;
 if(p===l.kernel_dispatch){const [x,y]=input();const a=sk%2===0?x:x+1;if(v[1]!==a||v[2]!==y)throw Error("kernel input");kernels++;sk++;}
 if(p>=initial)steps++;paid++;pc=n.next;
 switch(n.kind){
 case "Const":v[n.dst]=n.value;break;
 case "Read":v[n.dst]=d.get(v[n.address])??0;break;
 case "Write":{const k=v[n.address],z=v[n.value];d.set(k,z);
 if(k===12&&firstPair===null)firstPair=[d.get(11),z];
 if(k===13){const [x,y]=input(),a=d.get(11),b=z,correct=a===x*y&&b===(x+1)*y,relation=b-a===y;
 outputs.push([a,b]);services.push({at:prod,correct,relation,initialPair:firstPair,paid:paid-requestStart,kernels:sk,sweeps:repairAttempts});
 prod++;if(prod===s.requests.length)return {paid,outputs,services,clones,steps,kernels,repairs,reads,sweeps,alarms,exposure,repairPaid,peak,nodes:g.length,events,prefix,injected};
 load(prod);}break;}
 case "Add":v[n.dst]=v[n.lhs]+v[n.rhs];break;
 case "Sub":v[n.dst]=v[n.lhs]-v[n.rhs];break;
 case "Branch":pc=v[n.test]!==0?n.nonzero:n.zero;break;
 case "ReadNext":{const at=v[n.node];if(at<initial||at>=g.length||g[at].next===undefined)throw Error("read edge");v[n.dst]=g[at].next;reads++;break;}
 case "CloneNode":{const from=v[n.source],idx=g.length,x=idx-initial+1;if(from!==(x%2?l.parent_add:l.double_src))throw Error("clone");g.push({...g[from]});v[n.dst]=idx;clones++;break;}
 case "PatchConst":{const at=v[n.node],to=v[n.value];if(at===l.base||at===l.repair_anchor){if(to!==initial-1)throw Error("anchor");}else if(at===l.repair_last){if(to!==initial+15)throw Error("last");}else throw Error("const");g[at].value=to;break;}
 case "PatchNext":{const at=v[n.node],to=v[n.target];if(!g[to]||g[at].next===undefined)throw Error("edge");
 if(at>=initial){if(to!==expectedEdge(at-initial+1,initial,l))throw Error("wrong intended edge");const repair=p>=l.repair_anchor&&p<l.publish;if(repair){if(g[at].next===to)throw Error("spurious repair");events.push({at:prod,node:at-initial+1,paid,old:g[at].next,to,suppressed:off});if(!off){g[at].next=to;repairs++;}break;}}
 else if(at===l.special_entry){const [x]=input();if(to!==initial+(sk%2===1?x:x+1)-1)throw Error("entry");}
 else if(at===l.kernel_dispatch){if(![l.base,l.parent].includes(to))throw Error("dispatch");}
 else if(at===l.return_route){if(![l.first,l.second].includes(to))throw Error("return");}
 else if(at===l.second_tail){if(![l.restore,l.batch].includes(to))throw Error("tail");}
 else if(at===l.alarm){if(![l.publish,l.repair_anchor].includes(to))throw Error("retry");}
 else throw Error("patch "+at);g[at].next=to;break;}
 default:throw Error(n.kind);
 }
 peak=Math.max(peak,g.length+1+d.size);if(g.length>200||d.size>16||peak>220||!v.every(Number.isSafeInteger))throw Error("capacity");
 }throw Error("budget");
}

const scenarios=[];
for(const n of [24,96])for(const pattern of ["clean","single_visible","repeat_visible","latent_then_visible","silent_correlated","mixed"]){
 const requests=Array.from({length:n},(_,i)=>[3+(i*7)%12,1+(i*5)%16]);const faults=[];
 const put=(at,node,to,x)=>{requests[at]=[x,7];faults.push({at,node,to});};
 if(pattern==="single_visible")put(4,7,0,6);
 if(pattern==="repeat_visible"){for(let i=4;i<n;i+=12){const node=i>=88?15:2+2*(Math.floor(i/12)%7);put(i,node,0,node-1);}}
 if(pattern==="latent_then_visible"){put(4,16,0,3);requests[8]=[15,7];}
 if(pattern==="silent_correlated"){for(let i=4;i<n;i++)requests[i]=[2,7];faults.push({at:4,node:2,to:0});}
 if(pattern==="mixed"){put(4,2,0,2);requests[5]=[3,7];put(n-4,15,0,14);}
 scenarios.push({id:n+"_"+pattern,n,pattern,requests,faults});
}
const rows=scenarios.map(s=>{
 const arms={direct:runWorld(s,"direct"),unchecked:runWorld(s,"unchecked"),sweep:runWorld(s,"sweep"),fallback:runWorld(s,"fallback"),reactive:runWorld(s),off:runWorld(s,"reactive",true)};
 if(!arms.direct.services.every(x=>x.correct)||!arms.sweep.services.every(x=>x.correct))throw Error("direct/sweep");
 if(JSON.stringify(arms.reactive.prefix)!==JSON.stringify(arms.off.prefix)||arms.reactive.exposure!==arms.off.exposure)throw Error("prefix");
 if(arms.off.repairs)throw Error("off");
 if(arms.reactive.services.some(x=>!x.correct&&x.relation===false))throw Error("visible not repaired");
 if(s.pattern==="silent_correlated"&&!arms.reactive.services.some(x=>!x.correct&&x.relation))throw Error("silent falsifier");
 const compact=q=>({paid:q.paid,kernels:q.kernels,nodes:q.nodes,peak:q.peak,reads:q.reads,repairs:q.repairs,alarms:q.alarms,sweeps:q.sweeps,repairPaid:q.repairPaid,wrong:q.services.filter(x=>!x.correct).length,silent:q.services.filter(x=>!x.correct&&x.relation).length,retries:q.services.filter(x=>x.kernels>2).length,events:q.events,services:q.services});
 return {id:s.id,n:s.n,pattern:s.pattern,...Object.fromEntries(Object.entries(arms).map(([k,q])=>[k,compact(q)]))};
});
function agg(a){return Object.fromEntries(["direct","unchecked","sweep","fallback","reactive","off"].map(k=>[k,{paid:a.reduce((s,r)=>s+r[k].paid,0),net:a.reduce((s,r)=>s+r.direct.paid-r[k].paid,0),wrong:a.reduce((s,r)=>s+r[k].wrong,0),silent:a.reduce((s,r)=>s+r[k].silent,0),repairs:a.reduce((s,r)=>s+r[k].repairs,0),reads:a.reduce((s,r)=>s+r[k].reads,0),alarms:a.reduce((s,r)=>s+r[k].alarms,0),sweeps:a.reduce((s,r)=>s+r[k].sweeps,0),retries:a.reduce((s,r)=>s+r[k].retries,0),repairPaid:a.reduce((s,r)=>s+r[k].repairPaid,0),nodes:Math.max(...a.map(r=>r[k].nodes)),peak:Math.max(...a.map(r=>r[k].peak))}]))}
const summary={all:agg(rows),patterns:Object.fromEntries([...new Set(rows.map(r=>r.pattern))].map(k=>[k,agg(rows.filter(r=>r.pattern===k))])),worlds:rows.length*6,requests:rows.reduce((s,r)=>s+r.n,0)};

assert.deepEqual(summary,{"all":{"direct":{"paid":57330,"net":0,"wrong":0,"silent":0,"repairs":0,"reads":0,"alarms":0,"sweeps":0,"retries":0,"repairPaid":0,"nodes":129,"peak":135},"unchecked":{"paid":35181,"net":22149,"wrong":378,"silent":292,"repairs":0,"reads":0,"alarms":0,"sweeps":0,"retries":0,"repairPaid":0,"nodes":145,"peak":151},"sweep":{"paid":146545,"net":-89215,"wrong":0,"silent":0,"repairs":20,"reads":11520,"alarms":0,"sweeps":720,"retries":0,"repairPaid":105140,"nodes":145,"peak":151},"fallback":{"paid":48227,"net":9103,"wrong":292,"silent":292,"repairs":0,"reads":0,"alarms":86,"sweeps":0,"retries":86,"repairPaid":0,"nodes":145,"peak":151},"reactive":{"paid":46891,"net":10439,"wrong":114,"silent":114,"repairs":18,"reads":288,"alarms":18,"sweeps":18,"retries":18,"repairPaid":2700,"nodes":145,"peak":151},"off":{"paid":59519,"net":-2189,"wrong":378,"silent":292,"repairs":0,"reads":1376,"alarms":172,"sweeps":86,"retries":86,"repairPaid":13027,"nodes":145,"peak":151}},"patterns":{"clean":{"direct":{"paid":10322,"net":0,"wrong":0,"silent":0,"repairs":0,"reads":0,"alarms":0,"sweeps":0,"retries":0,"repairPaid":0,"nodes":129,"peak":135},"unchecked":{"paid":6154,"net":4168,"wrong":0,"silent":0,"repairs":0,"reads":0,"alarms":0,"sweeps":0,"retries":0,"repairPaid":0,"nodes":145,"peak":151},"sweep":{"paid":24514,"net":-14192,"wrong":0,"silent":0,"repairs":0,"reads":1920,"alarms":0,"sweeps":120,"retries":0,"repairPaid":17520,"nodes":145,"peak":151},"fallback":{"paid":6994,"net":3328,"wrong":0,"silent":0,"repairs":0,"reads":0,"alarms":0,"sweeps":0,"retries":0,"repairPaid":0,"nodes":145,"peak":151},"reactive":{"paid":7354,"net":2968,"wrong":0,"silent":0,"repairs":0,"reads":0,"alarms":0,"sweeps":0,"retries":0,"repairPaid":0,"nodes":145,"peak":151},"off":{"paid":7354,"net":2968,"wrong":0,"silent":0,"repairs":0,"reads":0,"alarms":0,"sweeps":0,"retries":0,"repairPaid":0,"nodes":145,"peak":151}},"single_visible":{"direct":{"paid":10310,"net":0,"wrong":0,"silent":0,"repairs":0,"reads":0,"alarms":0,"sweeps":0,"retries":0,"repairPaid":0,"nodes":129,"peak":135},"unchecked":{"paid":5954,"net":4356,"wrong":40,"silent":10,"repairs":0,"reads":0,"alarms":0,"sweeps":0,"retries":0,"repairPaid":0,"nodes":145,"peak":151},"sweep":{"paid":24516,"net":-14206,"wrong":0,"silent":0,"repairs":2,"reads":1920,"alarms":0,"sweeps":120,"retries":0,"repairPaid":17522,"nodes":145,"peak":151},"fallback":{"paid":9812,"net":498,"wrong":10,"silent":10,"repairs":0,"reads":0,"alarms":30,"sweeps":0,"retries":30,"repairPaid":0,"nodes":145,"peak":151},"reactive":{"paid":7748,"net":2562,"wrong":0,"silent":0,"repairs":2,"reads":32,"alarms":2,"sweeps":2,"retries":2,"repairPaid":300,"nodes":145,"peak":151},"off":{"paid":13124,"net":-2814,"wrong":40,"silent":10,"repairs":0,"reads":480,"alarms":60,"sweeps":30,"retries":30,"repairPaid":4500,"nodes":145,"peak":151}},"repeat_visible":{"direct":{"paid":10304,"net":0,"wrong":0,"silent":0,"repairs":0,"reads":0,"alarms":0,"sweeps":0,"retries":0,"repairPaid":0,"nodes":129,"peak":135},"unchecked":{"paid":5627,"net":4677,"wrong":112,"silent":78,"repairs":0,"reads":0,"alarms":0,"sweeps":0,"retries":0,"repairPaid":0,"nodes":145,"peak":151},"sweep":{"paid":24515,"net":-14211,"wrong":0,"silent":0,"repairs":10,"reads":1920,"alarms":0,"sweeps":120,"retries":0,"repairPaid":17530,"nodes":145,"peak":151},"fallback":{"paid":9513,"net":791,"wrong":78,"silent":78,"repairs":0,"reads":0,"alarms":34,"sweeps":0,"retries":34,"repairPaid":0,"nodes":145,"peak":151},"reactive":{"paid":9315,"net":989,"wrong":0,"silent":0,"repairs":10,"reads":160,"alarms":10,"sweeps":10,"retries":10,"repairPaid":1500,"nodes":145,"peak":151},"off":{"paid":13623,"net":-3319,"wrong":112,"silent":78,"repairs":0,"reads":544,"alarms":68,"sweeps":34,"retries":34,"repairPaid":5225,"nodes":145,"peak":151}},"latent_then_visible":{"direct":{"paid":10322,"net":0,"wrong":0,"silent":0,"repairs":0,"reads":0,"alarms":0,"sweeps":0,"retries":0,"repairPaid":0,"nodes":129,"peak":135},"unchecked":{"paid":6142,"net":4180,"wrong":2,"silent":0,"repairs":0,"reads":0,"alarms":0,"sweeps":0,"retries":0,"repairPaid":0,"nodes":145,"peak":151},"sweep":{"paid":24512,"net":-14190,"wrong":0,"silent":0,"repairs":2,"reads":1920,"alarms":0,"sweeps":120,"retries":0,"repairPaid":17522,"nodes":145,"peak":151},"fallback":{"paid":7260,"net":3062,"wrong":0,"silent":0,"repairs":0,"reads":0,"alarms":2,"sweeps":0,"retries":2,"repairPaid":0,"nodes":145,"peak":151},"reactive":{"paid":7750,"net":2572,"wrong":0,"silent":0,"repairs":2,"reads":32,"alarms":2,"sweeps":2,"retries":2,"repairPaid":300,"nodes":145,"peak":151},"off":{"paid":7744,"net":2578,"wrong":2,"silent":0,"repairs":0,"reads":32,"alarms":4,"sweeps":2,"retries":2,"repairPaid":300,"nodes":145,"peak":151}},"silent_correlated":{"direct":{"paid":5906,"net":0,"wrong":0,"silent":0,"repairs":0,"reads":0,"alarms":0,"sweeps":0,"retries":0,"repairPaid":0,"nodes":129,"peak":135},"unchecked":{"paid":5402,"net":504,"wrong":112,"silent":112,"repairs":0,"reads":0,"alarms":0,"sweeps":0,"retries":0,"repairPaid":0,"nodes":145,"peak":151},"sweep":{"paid":23988,"net":-18082,"wrong":0,"silent":0,"repairs":2,"reads":1920,"alarms":0,"sweeps":120,"retries":0,"repairPaid":17522,"nodes":145,"peak":151},"fallback":{"paid":6242,"net":-336,"wrong":112,"silent":112,"repairs":0,"reads":0,"alarms":0,"sweeps":0,"retries":0,"repairPaid":0,"nodes":145,"peak":151},"reactive":{"paid":6602,"net":-696,"wrong":112,"silent":112,"repairs":0,"reads":0,"alarms":0,"sweeps":0,"retries":0,"repairPaid":0,"nodes":145,"peak":151},"off":{"paid":6602,"net":-696,"wrong":112,"silent":112,"repairs":0,"reads":0,"alarms":0,"sweeps":0,"retries":0,"repairPaid":0,"nodes":145,"peak":151}},"mixed":{"direct":{"paid":10166,"net":0,"wrong":0,"silent":0,"repairs":0,"reads":0,"alarms":0,"sweeps":0,"retries":0,"repairPaid":0,"nodes":129,"peak":135},"unchecked":{"paid":5902,"net":4264,"wrong":112,"silent":92,"repairs":0,"reads":0,"alarms":0,"sweeps":0,"retries":0,"repairPaid":0,"nodes":145,"peak":151},"sweep":{"paid":24500,"net":-14334,"wrong":0,"silent":0,"repairs":4,"reads":1920,"alarms":0,"sweeps":120,"retries":0,"repairPaid":17524,"nodes":145,"peak":151},"fallback":{"paid":8406,"net":1760,"wrong":92,"silent":92,"repairs":0,"reads":0,"alarms":20,"sweeps":0,"retries":20,"repairPaid":0,"nodes":145,"peak":151},"reactive":{"paid":8122,"net":2044,"wrong":2,"silent":2,"repairs":4,"reads":64,"alarms":4,"sweeps":4,"retries":4,"repairPaid":600,"nodes":145,"peak":151},"off":{"paid":11072,"net":-906,"wrong":112,"silent":92,"repairs":0,"reads":320,"alarms":40,"sweeps":20,"retries":20,"repairPaid":3002,"nodes":145,"peak":151}}},"worlds":72,"requests":720});
console.log(JSON.stringify({tier:"NON_NATIVE_PRODUCTION_TRIGGERED_MAINTENANCE",summary,rows,scenarios,sources:Object.fromEntries(["direct","unchecked","sweep","fallback","reactive"].map(k=>[k,build(k)]))},null,2));
