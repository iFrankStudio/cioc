#!/usr/bin/env node
import {readFileSync} from "node:fs";
let s=readFileSync(new URL("./cl1b_c36_r3_hidden_change_credit_ledger.mjs",import.meta.url),"utf8");

s=s.slice(0,s.indexOf("const scenarios=[]"));
function r(a,b){if(!s.includes(a))throw Error(a);s=s.replace(a,b)}
r('const modes=["probe_all","feedback","periodic4","feedback_compiled","periodic4_compiled","no_feedback","compile_off"];','const modes=["probe_all","feedback","feedback_compiled","fused_compiled","gated_compiled","gated_off"];');
r('const compiled=mode.endsWith("_compiled")||mode==="compile_off";','const compiled=mode.endsWith("_compiled")||mode==="gated_off",fused=mode==="fused_compiled"||mode.startsWith("gated"),gated=mode.startsWith("gated");let count=0,streak=0,pending=false;');
r('prior=[a,b];masks=[];let aa=a,bb=b;','prior=[a,b];masks=[];count=0;streak=0;pending=false;let aa=a,bb=b;');
r('pay("data");masks.push(m);','pay("data");masks.push(m);if(fused){pay("branch");if(m){pay("alu");count++;}}');
const start=s.indexOf('   // Cheap conservative admission'),end=s.indexOf('\n  }\n }\n function produce',start);
if(start<0||end<0)throw Error("install block");
s=s.slice(0,start)+`   if(fused){pay("alu");count+=5-first;}
   if(gated){pay("policy");pending=true;}else install(t);
  }
 }
 function install(t){
   pay("policy",3);const reserve=50,n=fused?count:11,stages=fused?6-first:6;
   const upper=2*stages+n*(s.tariff+2)+1;
   if(base+n>s.cap||credit<reserve+upper){denied++;return;}
   const before=paid;
   function emit(op){pay("synth");pay("install",s.tariff);pay("data");graph.push(op);space();}
   for(let j=first;j<6;j++){pay("data");pay("branch");if(j>first)emit(0);if(masks[j])emit(masks[j]);}
   pay("install");assert(paid-before<=upper,"admission upper");if(fused)assert(paid-before===upper&&graph.length===count,"exact");installs++;events.push({t,kind:"install",length:graph.length});
`+s.slice(end+'\n  }'.length);
r('if(graph.length&&mode!=="compile_off")','if(graph.length&&mode!=="gated_off")');
r('compiled:graph.length>0&&mode!=="compile_off"','compiled:graph.length>0&&mode!=="gated_off"');
r('   const y=produce(q.x,q.y);','   if(gated&&pending){pay("policy",2);if(streak>=4){pending=false;install(t);}}\n   const y=produce(q.x,q.y);');
r('if(mode!=="no_feedback"&&!ok)dirty=true;','if(mode!=="no_feedback"&&!ok)dirty=true;if(gated&&pending){pay("policy",2);if(ok)streak++;else streak=0;}');
r('for(let t=0;t<192;t++){','for(let t=0;t<512;t++){');
r('  if(!remain){','  if(regime==="sandwich"&&(t===160||t===224))remain=0;\n  if(!remain){');
r('regime==="switch"?(t<96?1+rnd(4):24+rnd(25)):[1,4,12,36][rnd(4)]','regime==="sandwich"?(t>=160&&t<224?1+rnd(4):24+rnd(25)):[1,4,12,36][rnd(4)]');

const tail="\nconst regimes=[\"fast\",\"slow\",\"sandwich\",\"mixed\"],scenarios=[];\nfor(const seed of [2741,2851,2963,3079,3187,3301])for(const regime of regimes)for(const initial of [300,1500])for(const reward of [65,90])for(const cap of [84,96])scenarios.push(world(seed,regime,initial,reward,cap,1));\nconst rows=scenarios.map(s=>{\n const arms=Object.fromEntries(modes.map(m=>[m,run(s,m)]));\n for(const a of Object.values(arms)){\n  assert(a.correct+a.wrong+a.abstain<=512&&a.peak<=s.cap,\"accounting\");\n  const at=t=>a.rows.find(r=>r.t===t-1)?.credit??null;\n  const pre=at(160),mid=at(224),half=at(256),end=at(512);\n  const complete=end!==null&&a.deadAt===null&&end>=16;\n  const loss=pre!==null&&mid!==null&&mid<pre;\n  const restored=loss&&complete&&end>=pre;\n  const sustainable=complete&&end>=s.initial&&end>half;\n  const quality=complete&&a.correct>=Math.ceil(.95*512);\n  a.metrics={complete,loss,restored,sustainable,quality,strict:sustainable&&quality,net:a.credit-s.initial};\n }\n assert(arms.probe_all.wrong===0,\"fresh probes\");\n return {id:s.id,regime:s.regime,initial:s.initial,reward:s.reward,cap:s.cap,arms};\n});\nfunction agg(rs){return Object.fromEntries(modes.map(m=>[m,{correct:rs.reduce((a,r)=>a+r.arms[m].correct,0),wrong:rs.reduce((a,r)=>a+r.arms[m].wrong,0),complete:rs.filter(r=>r.arms[m].metrics.complete).length,loss:rs.filter(r=>r.arms[m].metrics.loss).length,restored:rs.filter(r=>r.arms[m].metrics.restored).length,sustainable:rs.filter(r=>r.arms[m].metrics.sustainable).length,strict:rs.filter(r=>r.arms[m].metrics.strict).length,net:rs.reduce((a,r)=>a+r.arms[m].metrics.net,0),installs:rs.reduce((a,r)=>a+r.arms[m].installs,0),denied:rs.reduce((a,r)=>a+r.arms[m].denied,0)}]))}\nconst summary={worlds:rows.length*modes.length,paired:rows.length,offeredPerArm:rows.length*512,all:agg(rows),regimes:Object.fromEntries(regimes.map(k=>[k,agg(rows.filter(r=>r.regime===k))])),capacity:Object.fromEntries([84,96].map(c=>[c,agg(rows.filter(x=>x.cap===c))]))};\nconst report={tier:\"NON_NATIVE_STRICT_SUSTAINABILITY\",summary,rows,scenarios};\n";
console.log(JSON.stringify(new Function(s.replace(/^#![^\n]*\n/,"")+tail+"\nreturn report;")(),null,2));
