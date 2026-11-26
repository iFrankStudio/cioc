#!/usr/bin/env node
// C36-R0: unknown coefficient epochs; explicit paid operation model, not native CIL.
const assert=(ok,msg)=>{if(!ok)throw Error(msg)};
const modes=["generic","plan_interp","compiled","compiled_off","static_table"];
const CAP=96, MAX=63;
const costKeys=["observe","learn","synth","install","retire","dispatch","data","alu","branch","publish"];
function run(s,mode){
 let paid=0, peak=0, occupied=0, wrong=0, correct=0, rejected=0, training=0, installed=0, prior=null, plan=[], graph=[], flags=[], regs=[0,0,0,0], changes=[], rows=[];
 const cost=Object.fromEntries(costKeys.map(k=>[k,0]));
 const pay=(k,n=1)=>{assert(Number.isSafeInteger(n)&&n>=0,"charge");paid+=n;cost[k]+=n;};
 // Explicit storage units: one register, scalar data cell, or encoded operation.
 // Static code footprint is modeled below, not JS byte size; no free all-pairs table.
 const boot={generic:24,plan_interp:30,compiled:32,compiled_off:32,static_table:50}[mode];
 function space(extra=0){occupied=boot+4+2+flags.length+plan.length+graph.length+extra;peak=Math.max(peak,occupied);assert(occupied<=CAP,"capacity");}
 space();
 const read=(a,i)=>{pay("data");return a[i]};
 const store=(a,i,v)=>{pay("data");a[i]=v;};
 const add=(a,b)=>{pay("alu");return a+b};
 const branch=b=>{pay("branch");return b};
 // All learners get exactly two labelled basis samples, including zeros.
 function learn(epoch){
  const old=prior;
  const sample=(x,y)=>{pay("observe");training++;return epoch.a*x+epoch.b*y;};
  const a=sample(1,0),b=sample(0,1);pay("learn",2); // store two observed coefficients
  prior=[a,b];
  return old===null||a!==old[0]||b!==old[1];
 }
 // Paid bit extraction by compare/subtract against fixed powers, no native division.
 function bits(a,b){
  const f=[];
  for(const p of [32,16,8,4,2,1]){
   for(const v of [0,1]){
    const q=v===0?a:b;pay("learn");const on=branch(q>=p);
    store(f,f.length,on?1:0);if(on){pay("alu");if(v===0)a-=p;else b-=p;}
   }
  }
  return f;
 }
 // Fixed legal instruction alphabet: zero R0, double R0, add input X or Y.
 // No target-specific executable graph is present at boot. Recipe is learned.
 function makePlan(f){
  const p=[];let live=false;
  for(let j=0;j<6;j++){
   pay("synth");if(branch(live)){pay("synth");p.push(0);}
   for(let k=0;k<2;k++){
    const on=read(f,2*j+k);if(branch(on)){pay("synth");p.push(k+1);live=true;}
   }
  }
  return p;
 }
 function init(epoch){
  const changed=learn(epoch);pay("branch");
  if(!changed)return false;
  pay("retire",flags.length+plan.length+graph.length);flags=[];plan=[];graph=[];
  flags=bits(prior[0],prior[1]);space();
  if(mode==="plan_interp"||mode==="compiled"||mode==="compiled_off"){
   plan=makePlan(flags);space();
   if(mode!=="plan_interp"){
    for(let i=0;i<plan.length;i++){
     const op=read(plan,i);pay("install",2); // allocate and write executable instruction/edge
     if(mode==="compiled")graph.push({op,next:i+1});
     space();
    }
    if(mode==="compiled"){pay("install");installed++;}
    // Off retains recipe and all compilation work, uses interpreter.
    if(mode==="compiled"){pay("retire",plan.length);plan=[];}
   }
  }
  changes.push({epoch:epoch.id,a:prior[0],b:prior[1],plan:plan.slice(),graph:graph.map(n=>n.op),paid});
  return true;
 }
 function execOp(op,x,y){if(op===0)regs[0]=add(regs[0],regs[0]);else if(op===1)regs[0]=add(regs[0],x);else if(op===2)regs[0]=add(regs[0],y);else throw Error("op");}
 function produce(x,y){
  pay("data",3);regs[0]=0;regs[1]=x;regs[2]=y;
  if(mode==="compiled"){
   let pc=0;
   while(pc<graph.length){pay("dispatch");const n=graph[pc];execOp(n.op,x,y);pc=n.next;}
  }else if(mode==="plan_interp"||mode==="compiled_off"){
   for(let i=0;branch(i<plan.length);i++){
    const op=read(plan,i);pay("dispatch");if(!branch(op===0))branch(op===1);
    execOp(op,x,y);pay("alu"); // interpreter index increment
   }
  }else if(mode==="static_table"){
   // Unrolled fixed control, reads learned flag pair per stage, dispatches fixed paths.
   // No loop/index overhead; always doubles from stage 2, including leading zeros.
   for(let j=0;j<6;j++){
    if(j){pay("dispatch");execOp(0,x,y);}
    for(let k=0;k<2;k++){const on=read(flags,2*j+k);pay("dispatch");if(branch(on))execOp(k+1,x,y);}
   }
  }else{
   // Generic loop over bit flags, no recipe construction.
   for(let j=0;branch(j<6);j++){
    if(branch(j>0)){pay("dispatch");execOp(0,x,y);}
    for(let k=0;k<2;k++){const on=read(flags,2*j+k);if(branch(on)){pay("dispatch");execOp(k+1,x,y);}}
    pay("alu");
   }
  }
  pay("publish");return regs[0];
 }
 for(const epoch of s.epochs){
  const start=paid;init(epoch);const before=paid;let ew=0,ec=0;
  for(const [x,y] of epoch.requests){
   // Production data never supplied with expected answer; observer only validates.
   const result=produce(x,y);if(result===epoch.a*x+epoch.b*y){correct++;ec++;}else{wrong++;ew++;}
  }
  rows.push({id:epoch.id,setup:before-start,production:paid-before,wrong:ew,correct:ec});
 }
 assert(Object.values(cost).reduce((a,b)=>a+b,0)===paid,"cost accounting");
 return {paid,wrong,correct,rejected,training,installed,peak,cost,rows,changes};
}
// Deterministic held-out coefficient pairs and signed production inputs; no training points.
function world(seed,horizon,regime){
 let z=seed;const rnd=n=>{z=(Math.imul(z,1664525)+1013904223)>>>0;return Math.floor(z/4294967296*n)};
 const epochs=[];let a=0,b=0;
 for(let e=0;e<8;e++){
  if(e===0||regime==="drift"||e%4===0){a=rnd(MAX+1);b=rnd(MAX+1);if(a===0&&b===0)a=1;}
  const requests=Array.from({length:horizon},()=>{let x=rnd(129)-64,y=rnd(129)-64;if((x===1&&y===0)||(x===0&&y===1))x+=2;return [x,y]});
  epochs.push({id:e,a,b,requests});
 }
 return {id:seed+"_"+horizon+"_"+regime,seed,horizon,regime,epochs};
}
const scenarios=[];
for(const seed of [19,47,83,131,199,251])for(const h of [1,4,16,64])for(const regime of ["drift","stable"])scenarios.push(world(seed,h,regime));
const rows=scenarios.map(s=>{
 const arms=Object.fromEntries(modes.map(m=>[m,run(s,m)]));
 for(const a of Object.values(arms)){assert(a.wrong===0,"wrong "+s.id);assert(a.training===16,"training");}
 assert(JSON.stringify(arms.compiled.rows.map(x=>x.correct))===JSON.stringify(arms.compiled_off.rows.map(x=>x.correct)),"matched correctness");
 return {id:s.id,horizon:s.horizon,regime:s.regime,arms};
});
function agg(rs){return Object.fromEntries(modes.map(m=>[m,{paid:rs.reduce((n,r)=>n+r.arms[m].paid,0),correct:rs.reduce((n,r)=>n+r.arms[m].correct,0),wrong:rs.reduce((n,r)=>n+r.arms[m].wrong,0),peak:Math.max(...rs.map(r=>r.arms[m].peak)),setup:rs.reduce((n,r)=>n+r.arms[m].rows.reduce((s,e)=>s+e.setup,0),0)}]))}
const groups=Object.fromEntries([1,4,16,64].flatMap(h=>["drift","stable"].map(regime=>[h+"_"+regime,agg(rows.filter(r=>r.horizon===h&&r.regime===regime))])));
const summary={worlds:rows.length*modes.length,requestsPerArm:rows.reduce((n,r)=>n+r.arms.compiled.correct,0),all:agg(rows),groups,compiledWins:rows.filter(r=>r.arms.compiled.paid<Math.min(r.arms.generic.paid,r.arms.plan_interp.paid,r.arms.static_table.paid)).length};
const report={tier:"NON_NATIVE_PAID_OPERATION_MODEL",summary,rows,scenarios};

console.log(JSON.stringify(report,null,2));
