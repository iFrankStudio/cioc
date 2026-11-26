#!/usr/bin/env node
// C36-R3: unannounced coefficient changes, metered observations and finite credit ledger.
// Host-executed abstract model. No production answer or change signal delivered to policy.
const assert=(x,m)=>{if(!x)throw Error(m)};
const modes=["probe_all","feedback","periodic4","feedback_compiled","periodic4_compiled","no_feedback","compile_off"];
const pow=[32,16,8,4,2,1], STOP=Symbol("credit exhaustion");
function run(s,mode){
 const ledger={},rows=[],events=[];
 let credit=s.initial,paid=0,earned=0,wrong=0,correct=0,abstain=0,deadAt=null,prior=null,masks=[],graph=[],first=0,dirty=true,age=0,probes=0,installs=0,denied=0,peak=0;
 const compiled=mode.endsWith("_compiled")||mode==="compile_off";
 const periodic=mode.startsWith("periodic4");
 const base=78; // equal reserved footprint for all arms, includes registers and masks
 function space(){peak=Math.max(peak,base+graph.length);assert(peak<=s.cap,"storage");}
 space();
 function pay(k,n=1){assert(Number.isSafeInteger(n)&&n>=0,"charge");if(credit<n)throw STOP;credit-=n;paid+=n;ledger[k]=(ledger[k]??0)+n;}
 function sense(a,b,x,y){pay("observe",6);return a*x+b*y;}
 function learn(q,t){
  // Access to hidden coefficients only inside the two charged basis observations.
  const a=sense(q.a,q.b,1,0),b=sense(q.a,q.b,0,1);probes+=2;pay("data",2);pay("branch",2);
  dirty=false;age=0;
  if(prior&&prior[0]===a&&prior[1]===b)return;
  if(graph.length){pay("retire",graph.length);graph=[];}
  prior=[a,b];masks=[];let aa=a,bb=b;
  for(const p of pow){pay("branch",2);let m=0;if(aa>=p){pay("alu",2);aa-=p;m+=1;}if(bb>=p){pay("alu",2);bb-=p;m+=2;}pay("data");masks.push(m);}
  first=0;while(first<5){pay("data");pay("branch");if(masks[first])break;pay("alu");first++;}
  if(compiled){
   // Cheap conservative admission BEFORE exact planning. Maximum eleven ops.
   pay("policy",3);const reserve=50,upper=12+11*(s.tariff+2)+1;
   if(base+11>s.cap||credit<reserve+upper){denied++;return;}
   const before=paid;const p=[];for(let j=first;j<6;j++){pay("data");pay("branch");if(j>first){pay("synth");p.push(0);}if(masks[j]){pay("synth");p.push(masks[j]);}}
   for(const op of p){pay("install",s.tariff);pay("data");graph.push(op);space();}
   pay("install");assert(paid-before<=upper,"admission upper bound");installs++;events.push({t,kind:"install",length:graph.length});
  }
 }
 function produce(x,y){
  pay("data",3);pay("dispatch");pay("alu");const regs=[0,x,y,x+y];pay("dispatch");
  function op(k){pay("dispatch");pay("alu");regs[0]+=k===0?regs[0]:regs[k];}
  if(graph.length&&mode!=="compile_off"){for(const k of graph)op(k);}
  else for(let j=first;j<6;j++){pay("data");pay("branch");if(j>first)op(0);if(masks[j])op(masks[j]);}
  return regs[0];
 }
 for(let t=0;t<s.requests.length;t++){
  const q=s.requests[t],start=credit;
  try{
   pay("policy",2);
   const check=mode==="probe_all"||!prior||dirty||(periodic&&age>=4);
   if(check){
    // If even two observations cannot be afforded, decline rather than publish stale data.
    pay("policy");if(credit<16){pay("abstain");abstain++;rows.push({t,outcome:"abstain",credit});continue;}
    learn(q,t);
   }
   pay("policy");if(credit<8){pay("abstain");abstain++;rows.push({t,outcome:"abstain",credit});continue;}
   const y=produce(q.x,q.y);pay("publish");
   // Reward and a one-bit accept/reject are the only environment feedback.
   const ok=y===q.a*q.x+q.b*q.y;
   if(ok){correct++;earned+=s.reward;credit+=s.reward;}else wrong++;
   pay("feedback");if(mode!=="no_feedback"&&!ok)dirty=true;
   if(periodic){pay("policy");age++;}
   rows.push({t,outcome:ok?"correct":"wrong",credit,start,probed:check,compiled:graph.length>0&&mode!=="compile_off"});
  }catch(e){if(e!==STOP)throw e;deadAt=t;break;}
 }
 assert(credit===s.initial+earned-paid&&credit>=0,"credit identity");
 assert(Object.values(ledger).reduce((a,b)=>a+b,0)===paid,"ledger");
 return {credit,paid,earned,correct,wrong,abstain,deadAt,probes,installs,denied,peak,ledger,rows,events};
}
function world(seed,regime,initial,reward,cap,tariff){
 let z=seed;const rnd=n=>{z=(Math.imul(z,1664525)+1013904223)>>>0;return Math.floor(z/4294967296*n)};
 let a=1,b=1,remain=0;const requests=[],changes=[];
 for(let t=0;t<192;t++){
  if(!remain){const old=[a,b];do{a=rnd(64);b=rnd(64);}while((a===old[0]&&b===old[1])||(!a&&!b));remain=regime==="fast"?1+rnd(4):regime==="slow"?24+rnd(25):regime==="switch"?(t<96?1+rnd(4):24+rnd(25)):[1,4,12,36][rnd(4)];changes.push(t);}
  remain--;
  let x=rnd(129)-64,y=rnd(129)-64;if((x===1&&y===0)||(x===0&&y===1))x+=2;
  requests.push({a,b,x,y});
 }
 return {id:[seed,regime,initial,reward,cap,tariff].join("_"),seed,regime,initial,reward,cap,tariff,requests,changes};
}
const scenarios=[];
for(const seed of [1301,1423,1553,1667,1783,1901])for(const regime of ["fast","slow","mixed","switch"])for(const initial of [100,300])for(const reward of [40,65])for(const cap of [84,96])for(const tariff of [1,4])scenarios.push(world(seed,regime,initial,reward,cap,tariff));
const rows=scenarios.map(s=>{
 const arms=Object.fromEntries(modes.map(m=>[m,run(s,m)]));
 for(const a of Object.values(arms)){assert(a.correct+a.wrong+a.abstain<=s.requests.length,"accounting");assert(a.peak<=s.cap,"peak");}
 assert(arms.probe_all.wrong===0,"fresh probes");
 return {id:s.id,regime:s.regime,initial:s.initial,reward:s.reward,cap:s.cap,tariff:s.tariff,arms};
});
function agg(rs){return Object.fromEntries(modes.map(m=>[m,{paid:rs.reduce((a,r)=>a+r.arms[m].paid,0),correct:rs.reduce((a,r)=>a+r.arms[m].correct,0),wrong:rs.reduce((a,r)=>a+r.arms[m].wrong,0),abstain:rs.reduce((a,r)=>a+r.arms[m].abstain,0),survive:rs.filter(r=>r.arms[m].deadAt===null&&r.arms[m].credit>=16).length,dead:rs.filter(r=>r.arms[m].deadAt!==null).length,credit:rs.reduce((a,r)=>a+r.arms[m].credit,0),probes:rs.reduce((a,r)=>a+r.arms[m].probes,0),installs:rs.reduce((a,r)=>a+r.arms[m].installs,0),denied:rs.reduce((a,r)=>a+r.arms[m].denied,0)}]))}
const summary={worlds:rows.length*modes.length,paired:rows.length,offeredPerArm:rows.length*192,all:agg(rows),groups:Object.fromEntries(["fast","slow","mixed","switch"].map(k=>[k,agg(rows.filter(r=>r.regime===k))])),sensitivity:Object.fromEntries([84,96].flatMap(c=>[40,65].map(r=>[c+"_"+r,agg(rows.filter(x=>x.cap===c&&x.reward===r))])))};
const report={tier:"NON_NATIVE_HIDDEN_CHANGE_CREDIT_LEDGER",summary,rows,scenarios};

console.log(JSON.stringify(report,null,2));
