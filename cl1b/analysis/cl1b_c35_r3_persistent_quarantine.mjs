#!/usr/bin/env node
import {readFileSync} from "node:fs";
let s=readFileSync(new URL("./cl1b_c35_r2_unit_path_witness.mjs",import.meta.url),"utf8");
s=s.slice(0,s.indexOf("const assert="));
function r(a,b){if(!s.includes(a))throw Error(a);s=s.replace(a,b)}
r('cadence=0,witness=false){','cadence=0,witness=false,quarantine=false){');
r('label("second");C(3,12);W','label("second");C(3,12);label("second_write");W');
r('pe("alarm","publish","pair");','pe("alarm",quarantine?"quarantine":"publish","pair");if(quarantine){label("quarantine");pe("kernel_dispatch","parent");pe("second_write","publish");pe("batch","pair","pair");}');
r('cadence=0,witness=false){','cadence=0,witness=false,quarantine=false,routeOff=false){');
r('build(policy,cadence,witness),initial','build(policy,cadence,witness,quarantine),initial');
r('audits=0,sa=0;','audits=0,sa=0,quarantines=0,quarantineAt=null,routeExposure=null;');
r('prefix=[],injected=[]','prefix=[],routePrefix=[],injected=[]');
r('if(p===l.alarm)alarms++;','if(p===l.alarm)alarms++;if(p===l.quarantine){quarantines++;quarantineAt=prod;}if(quarantine&&routeExposure===null)routePrefix.push([p,[...v],[...d],g.length,n]);');
r('audits:sa,sweeps:repairAttempts','audits:sa,sweeps:repairAttempts,isolated:g[l.kernel_dispatch].next===l.parent');
r('events,prefix,injected};','events,prefix,injected,quarantines,quarantineAt,routeExposure,routePrefix};');
r('throw Error("dispatch");}','throw Error("dispatch");if(quarantine&&to===l.parent){routeExposure=paid;if(routeOff)break;}}');
r('[l.publish,l.repair_anchor].includes','[l.publish,l.repair_anchor,l.quarantine].includes');
r('else throw Error("patch "+at);','else if(at===l.second_write&&quarantine&&to===l.publish){}else if(at===l.batch&&quarantine&&to===l.pair){}else throw Error("patch "+at);');
const tail=`
for(let node=1;node<=16;node++){const n=32,requests=Array.from({length:n},(_,i)=>[i<8?1:Math.min(node,15),1+i%16]);scenarios.push({id:"opcode_"+node,n,pattern:"opcode_sweep",requests,faults:[{at:4,node,kind:"opcode"}]});}
const configs=[["direct","direct",0,false,false,false,false],["unit1","reactive",1,false,true,false,false],["quarantine","reactive",1,false,true,true,false],["route_off","reactive",1,false,true,true,true],["repair_off","reactive",1,true,true,true,false]];
const rows=scenarios.map(s=>{const a=Object.fromEntries(configs.map(([name,p,k,off,w,q,ro])=>[name,runWorld(s,p,off,k,w,q,ro)]));
for(const k of ["direct","quarantine","repair_off"])if(a[k].services.some(x=>!x.correct))throw Error(k+s.id);
if(JSON.stringify(a.quarantine.routePrefix)!==JSON.stringify(a.route_off.routePrefix)||a.quarantine.routeExposure!==a.route_off.routeExposure)throw Error("prefix");
if(a.quarantine.quarantines>1||a.quarantine.quarantineAt!==null&&!a.quarantine.services.slice(a.quarantine.quarantineAt).every(x=>x.isolated&&x.correct))throw Error("isolation");
return Object.fromEntries(configs.map(([k])=>[k,{paid:a[k].paid,wrong:a[k].services.filter(x=>!x.correct).length,repairs:a[k].repairs,quarantines:a[k].quarantines,isolated:a[k].services.filter(x=>x.isolated).length}]))});
function agg(rr){return Object.fromEntries(configs.map(([k])=>[k,Object.fromEntries(Object.keys(rr[0][k]).map(f=>[f,rr.reduce((a,r)=>a+r[k][f],0)]))]))}
return {repairable:agg(rows.slice(0,24)),opcode:agg(rows.slice(24)),worlds:rows.length*configs.length,requestsPerArm:scenarios.reduce((a,r)=>a+r.n,0)};
`;
console.log(JSON.stringify(new Function(s.replace(/^#![^\n]*\n/,"")+tail)(),null,2));
