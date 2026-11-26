#!/usr/bin/env node
// C36-R1: horizon-blind online amortization; abstract operation model.
const assert=(x,m)=>{if(!x)throw Error(m)};
const modes=["static","always","delay4","adaptive"];
const pow=[32,16,8,4,2,1];
function run(s,mode){
 const cost={},events=[],regs=[0,0,0,0];
 let paid=0,correct=0,wrong=0,peak=0,prior=null,masks=[],first=0,graph=[],ready=false,age=0,rent=0,can=false,need=0,save=0,installs=0,denied=0,training=0,staticRequests=0,compiledRequests=0;
 const base={static:42,always:50,delay4:54,adaptive:56}[mode],state=mode==="static"?3:mode==="always"?4:6;
 const pay=(k,n=1)=>{assert(Number.isSafeInteger(n)&&n>=0,"cost");paid+=n;cost[k]=(cost[k]??0)+n;};
 const size=()=>base+4+2+state+masks.length+graph.length;
 const space=()=>{peak=Math.max(peak,size());assert(size()<=s.cap,"capacity");};
 space();
 function retire(){if(ready){pay("retire",graph.length);graph=[];ready=false;}}
 function observe(e,x,y){training++;pay("observe");return e.a*x+e.b*y;}
 function learn(e){
  const a=observe(e,1,0),b=observe(e,0,1);pay("data",2);pay("branch",2);
  if(prior&&a===prior[0]&&b===prior[1])return;
  retire();prior=[a,b];masks=[];age=0;rent=0;
  let aa=a,bb=b,ops=0;
  for(const p of pow){
   pay("branch",2);let m=0;
   if(aa>=p){pay("alu",2);aa-=p;m+=1;}
   if(bb>=p){pay("alu",2);bb-=p;m+=2;}
   pay("data");masks.push(m);
  }
  first=0;while(first<5){pay("data");pay("branch");if(masks[first])break;pay("alu");first++;}
  space();
  if(mode!=="static"){
   for(let j=first;j<6;j++){pay("data");pay("branch",2);if(j>first){ops++;pay("alu");}if(masks[j]){ops++;pay("alu");}}
   // Scan, emit, activate, retire. Estimate using only learned masks and resource tariff.
   pay("policy",8);need=2*(6-first)+ops*(s.install+2)+1;save=2*(6-first);
   can=size()+ops<=s.cap;if(!can)denied++;
   if(mode==="always"&&can)install(e.id);
  }
 }
 function install(epoch){
  assert(can&&!ready,"install");
  const before=paid;
  function emit(op){pay("install",s.install);pay("data");graph.push(op);space();}
  for(let j=first;j<6;j++){
   pay("data");pay("branch");
   if(j>first)emit(0);
   if(masks[j])emit(masks[j]);
  }
  pay("install");ready=true;installs++;
  assert(paid-before+graph.length===need,"estimated cost");
  events.push({epoch,age,need,save,length:graph.length,paid});
 }
 function execute(op){pay("dispatch");pay("alu");if(op===0)regs[0]+=regs[0];else regs[0]+=regs[op];}
 function produce(x,y,epoch){
  if((mode==="adaptive"||mode==="delay4")&&!ready&&can){
   pay("policy",2);if(mode==="adaptive"?rent>=need:age>=4)install(epoch);
  }
  pay("data",3);regs[0]=0;regs[1]=x;regs[2]=y;pay("dispatch");pay("alu");regs[3]=x+y;
  pay("dispatch");
  if(ready){compiledRequests++;for(const op of graph)execute(op);}
  else {
   staticRequests++;
   // Direct preinstalled mask-indexed stage paths, not an opcode interpreter.
   for(let j=first;j<6;j++){pay("data");pay("branch");const m=masks[j];if(j>first)execute(0);if(m)execute(m);}
   if((mode==="adaptive"||mode==="delay4")&&can){pay("policy");rent+=save;age++;}
  }
  pay("publish");return regs[0];
 }
 for(const e of s.epochs){
  learn(e);
  for(const [x,y] of e.requests){const result=produce(x,y,e.id);if(result===e.a*x+e.b*y)correct++;else wrong++;}
 }
 retire();
 assert(Object.values(cost).reduce((a,b)=>a+b,0)===paid,"ledger");
 return {paid,wrong,correct,peak,installs,denied,training,staticRequests,compiledRequests,cost,events};
}
function world(seed,regime,cap,install){
 let z=seed;const rnd=n=>{z=(Math.imul(z,1664525)+1013904223)>>>0;return Math.floor(z/4294967296*n)};
 const epochs=[];let a,b;
 for(let e=0;e<12;e++){
  if(e===0||regime!=="sticky"||e%4===0){a=rnd(64);b=rnd(64);if(!a&&!b)a=1;}
  const n=regime==="short"?1+rnd(3):regime==="long"?16+rnd(33):regime==="mixed"?[1,2,4,8,24,48][rnd(6)]:1+rnd(6);
  const requests=Array.from({length:n},()=>[rnd(129)-64,rnd(129)-64]);
  for(const p of requests)if((p[0]===1&&p[1]===0)||(p[0]===0&&p[1]===1))p[0]+=2;
  epochs.push({id:e,a,b,requests});
 }
 return {id:[seed,regime,cap,install].join("_"),seed,regime,cap,install,epochs};
}
const scenarios=[];
for(const seed of [17,43,97,139,191,271,337,433])for(const regime of ["short","long","mixed","sticky"])for(const cap of [80,96])for(const install of [1,4])scenarios.push(world(seed,regime,cap,install));
const rows=scenarios.map(s=>{
 const arms=Object.fromEntries(modes.map(m=>[m,run(s,m)]));
 const n=s.epochs.reduce((a,e)=>a+e.requests.length,0);
 for(const a of Object.values(arms)){assert(a.wrong===0&&a.correct===n,"correct");assert(a.training===24,"observations");}
 for(const ev of arms.adaptive.events)assert(ev.age*ev.save>=ev.need,"online trigger");
 return {id:s.id,regime:s.regime,cap:s.cap,install:s.install,arms};
});
function agg(rs){return Object.fromEntries(modes.map(m=>[m,{paid:rs.reduce((a,r)=>a+r.arms[m].paid,0),correct:rs.reduce((a,r)=>a+r.arms[m].correct,0),wrong:rs.reduce((a,r)=>a+r.arms[m].wrong,0),peak:Math.max(...rs.map(r=>r.arms[m].peak)),installs:rs.reduce((a,r)=>a+r.arms[m].installs,0),denied:rs.reduce((a,r)=>a+r.arms[m].denied,0),compiledRequests:rs.reduce((a,r)=>a+r.arms[m].compiledRequests,0)}]))}
const summary={worlds:rows.length*modes.length,paired:rows.length,all:agg(rows),groups:Object.fromEntries(["short","long","mixed","sticky"].map(k=>[k,agg(rows.filter(r=>r.regime===k))])),sensitivity:Object.fromEntries([80,96].flatMap(c=>[1,4].map(i=>[c+"_"+i,agg(rows.filter(r=>r.cap===c&&r.install===i))]))),wins:Object.fromEntries(modes.map(m=>[m,rows.filter(r=>r.arms[m].paid===Math.min(...modes.map(k=>r.arms[k].paid))).length]))};
const report={tier:"NON_NATIVE_ONLINE_AMORTIZATION",summary,rows,scenarios};

console.log(JSON.stringify(report,null,2));
