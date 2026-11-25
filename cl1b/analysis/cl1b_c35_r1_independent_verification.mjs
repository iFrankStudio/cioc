#!/usr/bin/env node
// C35-R1: paid independent parent verification with guest-owned cadence.


function build(policy="reactive",cadence=0){
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
 label("prod_init");if(cadence)set(14,cadence,"batch");else C(0,0,"batch");
 label("batch");C(0,0,cadence?"audit_tick":policy==="reactive"?"reset_alarm":policy==="sweep"?"repair_anchor":"pair");
 if(cadence){
 label("audit_tick");C(3,14);R(1,3);B(1,"audit_dec","audit_reset");
 label("audit_reset");C(1,cadence,"audit_dec");
 label("audit_dec");C(0,1);S(1,1,0);W(3,1,"reset_alarm");
 }
 label("reset_alarm");pe("alarm","repair_anchor","pair");
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
 label("check");C(3,11);R(1,3);C(3,51);R(2,3);A(1,1,2);S(2,0,1);B(2,"alarm",cadence?"audit_gate":"publish");
 label("alarm");C(0,0,policy==="reactive"?"repair_anchor":policy==="fallback"?"fallback":"publish");
 // fallback redoes both required products on parent, then restores shared dispatch.
 label("fallback");pe("kernel_dispatch","parent");pe("second_tail","restore","pair");
 label("restore");pe("kernel_dispatch","base");pe("second_tail","batch","batch");
 if(cadence){
 label("audit_gate");C(3,14);R(1,3);B(1,"publish","audit_call");
 label("audit_call");pe("return_route","audit_result");C(3,50);R(1,3);C(3,51);R(2,3);C(0,0,"parent");
 label("audit_result");C(3,11);R(1,3);S(2,0,1);B(2,"alarm","publish");
 }
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
function runWorld(s,policy="reactive",off=false,cadence=0){
 const {g,l}=build(policy,cadence),initial=g.length,d=new Map(),v=[0,0,0,0];const load=i=>{d.set(50,s.requests[i][0]);d.set(51,s.requests[i][1]);};load(0);
 let pc=0,paid=0,prod=0,peak=initial+1+d.size,clones=0,steps=0,kernels=0,repairs=0,reads=0,sweeps=0,alarms=0,exposure=null,repairAttempts=0,requestStart=0,repairPaid=0,sk=0,firstPair=null,audits=0,sa=0;const outputs=[],services=[],events=[],prefix=[],injected=[];
 const input=()=>[d.get(50),d.get(51)];
 const inject=()=>{for(const f of s.faults.filter(f=>f.at===prod)){if(policy==="direct")continue;const at=initial+f.node-1;if(!g[at]||at>=g.length)throw Error("fault target");const old=g[at].next,to=f.to===0?l.out:initial+f.to-1;if(to===old||f.to>=f.node)throw Error("fault ineffective");g[at].next=to;injected.push({at:prod,node:f.node,old,to});}};
 while(paid<200000){
 const p=pc,n={...g[p]};if(!n.kind)throw Error("pc");
 if(p===l.batch){requestStart=paid;sk=0;sa=0;firstPair=null;repairAttempts=0;inject();}
 if(p===l.repair_anchor){sweeps++;repairAttempts++;if(repairAttempts>1)throw Error("repeat repair");}
 if(p===l.alarm)alarms++;if(p===l.audit_call){audits++;sa++;}
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
 outputs.push([a,b]);services.push({at:prod,correct,relation,initialPair:firstPair,paid:paid-requestStart,kernels:sk,audits:sa,sweeps:repairAttempts});
 prod++;if(prod===s.requests.length)return {paid,outputs,services,clones,steps,kernels,audits,repairs,reads,sweeps,alarms,exposure,repairPaid,peak,nodes:g.length,events,prefix,injected};
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
 else if(at===l.return_route){if(![l.first,l.second,l.audit_result].includes(to))throw Error("return");}
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

const assert=(ok,msg)=>{if(!ok)throw Error(msg)};
const configs=[["direct","direct",0,false],["reactive","reactive",0,false],...([1,2,4,8,16].map(k=>["audit"+k,"reactive",k,false])),["audit4_off","reactive",4,true]];
const rows=scenarios.map(s=>{
 const arms=Object.fromEntries(configs.map(([name,p,k,off])=>[name,runWorld(s,p,off,k)]));
 assert(arms.direct.services.every(x=>x.correct),"direct");
 assert(arms.audit1.services.every(x=>x.correct),"every-request audit must catch scoped faults");
 assert(JSON.stringify(arms.audit4.prefix)===JSON.stringify(arms.audit4_off.prefix)&&arms.audit4.exposure===arms.audit4_off.exposure,"matched prefix");
 assert(arms.audit4_off.repairs===0,"off repairs");
 for(const [name,p,k,off] of configs)if(k&&!off){
  const a=arms[name];
  assert(a.services.every(x=>x.correct||x.relation),"visible wrong");
  assert(a.services.every(x=>!x.audits||((x.at+1)%k===0)),"audit phase");
  assert(a.services.every(x=>x.correct||(x.at+1)%k!==0),"wrong on audit request");
 }
 const compact=q=>({paid:q.paid,wrong:q.services.filter(x=>!x.correct).length,silent:q.services.filter(x=>!x.correct&&x.relation).length,audits:q.audits,repairs:q.repairs,reads:q.reads,alarms:q.alarms,sweeps:q.sweeps,repairPaid:q.repairPaid,nodes:q.nodes,peak:q.peak,services:q.services,events:q.events});
 return {id:s.id,pattern:s.pattern,n:s.n,...Object.fromEntries(Object.entries(arms).map(([k,q])=>[k,compact(q)]))};
});
function aggregate(rr){return Object.fromEntries(configs.map(([k])=>[k,{paid:rr.reduce((a,r)=>a+r[k].paid,0),rawNet:rr.reduce((a,r)=>a+r.direct.paid-r[k].paid,0),wrong:rr.reduce((a,r)=>a+r[k].wrong,0),audits:rr.reduce((a,r)=>a+r[k].audits,0),repairs:rr.reduce((a,r)=>a+r[k].repairs,0),reads:rr.reduce((a,r)=>a+r[k].reads,0),repairPaid:rr.reduce((a,r)=>a+r[k].repairPaid,0),peak:Math.max(...rr.map(r=>r[k].peak))}]))}
const summary={all:aggregate(rows),patterns:Object.fromEntries([...new Set(rows.map(r=>r.pattern))].map(p=>[p,aggregate(rows.filter(r=>r.pattern===p))])),worlds:rows.length*configs.length,requestsPerArm:rows.reduce((a,r)=>a+r.n,0)};
const report={tier:"NON_NATIVE_PAID_INDEPENDENT_VERIFICATION",summary,rows,scenarios,configs,sources:Object.fromEntries(configs.filter(c=>!c[3]).map(([n,p,k])=>[n,build(p,k)]))};

console.log(JSON.stringify(report,null,2));
