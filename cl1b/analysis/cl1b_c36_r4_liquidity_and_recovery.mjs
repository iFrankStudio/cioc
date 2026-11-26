#!/usr/bin/env node
import {readFileSync} from "node:fs";
let s=readFileSync(new URL("./cl1b_c36_r3_hidden_change_credit_ledger.mjs",import.meta.url),"utf8");

s=s.slice(0,s.indexOf("const scenarios=[]"));
function r(a,b){if(!s.includes(a))throw Error(a);s=s.replace(a,b)}
r('const modes=["probe_all","feedback","periodic4","feedback_compiled","periodic4_compiled","no_feedback","compile_off"];','const modes=["probe_all","feedback","feedback_compiled","exact_compiled","exact_off","periodic4_exact_compiled"];');
r('const compiled=mode.endsWith("_compiled")||mode==="compile_off";','const compiled=mode.endsWith("_compiled")||mode==="exact_off",exact=mode.startsWith("exact")||mode.startsWith("periodic4_exact");');
r('if(graph.length&&mode!=="compile_off")','if(graph.length&&mode!=="exact_off")');
r('compiled:graph.length>0&&mode!=="compile_off"','compiled:graph.length>0&&mode!=="exact_off"');
const start=s.indexOf('   // Cheap conservative admission'),end=s.indexOf('\n  }\n }\n function produce',start);
if(start<0||end<0)throw Error("install block");
s=s.slice(0,start)+`   // Same direct emission for every compilation arm; no unmetered temporary plan.
   pay("policy",3);const reserve=50;
   let count=11,stages=6;
   if(exact){
    // Cheap lower bound before exact scan; skip if no possible graph fits.
    pay("policy",2);if(base+1>s.cap||credit<reserve+3){denied++;return;}
    count=0;stages=6-first;
    for(let j=first;j<6;j++){pay("data");pay("branch",2);if(j>first){pay("alu");count++;}if(masks[j]){pay("alu");count++;}}
   }
   const upper=2*stages+count*(s.tariff+2)+1;
   if(base+count>s.cap||credit<reserve+upper){denied++;return;}
   const before=paid;
   function emit(op){pay("synth");pay("install",s.tariff);pay("data");graph.push(op);space();}
   for(let j=first;j<6;j++){pay("data");pay("branch");if(j>first)emit(0);if(masks[j])emit(masks[j]);}
   pay("install");assert(paid-before<=upper,"admission upper bound");if(exact)assert(paid-before===upper&&graph.length===count,"exact admission");installs++;events.push({t,kind:"install",length:graph.length});
`+s.slice(end);
r('for(let t=0;t<192;t++){','for(let t=0;t<384;t++){');
r('regime==="switch"?(t<96?1+rnd(4):24+rnd(25)):[1,4,12,36][rnd(4)]','regime==="switch"?(t<48?1+rnd(4):24+rnd(25)):regime==="sandwich"?(t>=128&&t<176?1+rnd(4):24+rnd(25)):[1,4,12,36][rnd(4)]');

const tail="\nconst regimes=[\"fast\",\"slow\",\"switch\",\"sandwich\"];\nconst scenarios=[];\nfor(const seed of [2039,2143,2251,2371,2503,2617])for(const regime of regimes)for(const initial of [100,300,1500,6000])for(const reward of [40,65,90])for(const cap of [84,96])scenarios.push(world(seed,regime,initial,reward,cap,1));\nconst rows=scenarios.map(s=>{\n const arms=Object.fromEntries(modes.map(m=>[m,run(s,m)]));\n for(const a of Object.values(arms)){assert(a.correct+a.wrong+a.abstain<=s.requests.length,\"accounting\");assert(a.peak<=s.cap,\"peak\");}\n assert(arms.probe_all.wrong===0,\"fresh probes\");\n const phaseEnd=s.regime===\"switch\"?48:s.regime===\"sandwich\"?176:192;\n const phaseStart=s.regime===\"sandwich\"?128:0;\n for(const a of Object.values(arms)){\n  const at=t=>t===0?s.initial:a.rows.find(r=>r.t===t-1)?.credit??null;\n  const pre=at(phaseStart),mid=at(phaseEnd),end=at(384);\n  a.phase={pre,mid,end,reach:mid!==null,recover:mid!==null&&end!==null&&end>mid,restore:mid!==null&&end!==null&&pre!==null&&end>=pre,delta:mid===null||end===null?null:end-mid};\n }\n return {id:s.id,regime:s.regime,initial:s.initial,reward:s.reward,cap:s.cap,arms};\n});\nfunction agg(rs){return Object.fromEntries(modes.map(m=>[m,{correct:rs.reduce((a,r)=>a+r.arms[m].correct,0),wrong:rs.reduce((a,r)=>a+r.arms[m].wrong,0),survive:rs.filter(r=>r.arms[m].deadAt===null&&r.arms[m].credit>=16).length,reach:rs.filter(r=>r.arms[m].phase.reach).length,recover:rs.filter(r=>r.arms[m].phase.recover).length,restore:rs.filter(r=>r.arms[m].phase.restore).length,credit:rs.reduce((a,r)=>a+r.arms[m].credit,0),net:rs.reduce((a,r)=>a+r.arms[m].credit-r.initial,0),installs:rs.reduce((a,r)=>a+r.arms[m].installs,0),denied:rs.reduce((a,r)=>a+r.arms[m].denied,0)}]))}\nconst summary={worlds:rows.length*modes.length,paired:rows.length,offeredPerArm:rows.length*384,all:agg(rows),regimes:Object.fromEntries(regimes.map(k=>[k,agg(rows.filter(r=>r.regime===k))])),fastLiquidity:Object.fromEntries([100,300,1500,6000].flatMap(i=>[40,65,90].map(r=>[i+\"_\"+r,agg(rows.filter(x=>x.regime===\"fast\"&&x.initial===i&&x.reward===r))]))),capacity:Object.fromEntries([84,96].map(c=>[c,agg(rows.filter(x=>x.cap===c))]))};\nconst report={tier:\"NON_NATIVE_LIQUIDITY_AND_RECOVERY\",summary,rows,scenarios};\n";
console.log(JSON.stringify(new Function(s.replace(/^#![^\n]*\n/,"")+tail+"\nreturn report;")(),null,2));
