#!/usr/bin/env node
// C29 R4 non-native paid affine inference; no production or physical edit gate.
import assert from "node:assert/strict";

function build(profile){
 const g=[],l={};const label=k=>{if(k in l)throw Error(k);l[k]=g.length};
 const op=(kind,args)=>g.push({kind,...args});
 const C=(dst,value,next)=>op("Const",{dst,value,next}),R=(dst,address,next)=>op("Read",{dst,address,next}),W=(address,value,next)=>op("Write",{address,value,next}),S=(dst,lhs,rhs,next)=>op("Sub",{dst,lhs,rhs,next}),A=(dst,lhs,rhs,next)=>op("Add",{dst,lhs,rhs,next}),B=(test,nonzero,zero)=>op("Branch",{test,nonzero,zero}),Q=(dst,next)=>op("SenseOwnCredits",{dst,next});
 const set=(key,value,next)=>{C(3,key);C(0,value);W(3,0,next)};
 const pair=(x,y,next)=>{set(40,x);set(41,y,next)};
 label("init");set(47,0);pair(1,1,"trial_a");
 for(const name of ["a","b"]){
  const cross=name==="b"&&profile==="cross",unroll=name==="b"&&profile==="unroll";
  label("trial_"+name);Q(0);C(3,42);W(3,0);C(3,cross?41:40);R(1,3);C(3,cross?40:41);R(2,3);C(0,0);C(3,1);
  if(unroll)A(2,2,2);
  label("loop_"+name);
  if(unroll){
   S(1,1,3);B(1,"pair_b","odd_b");
   label("pair_b");A(0,0,2);S(1,1,3);B(1,"loop_b","output_b");
   label("odd_b");C(3,41);R(2,3);A(0,0,2,"output_b");
  }else{A(0,0,2);S(1,1,3);B(1,"loop_"+name,"output_"+name);}
  label("output_"+name);C(3,name==="a"?43:44);W(3,0);Q(0);C(3,42);R(1,3);S(1,1,0);C(3,name==="a"?45:46);W(3,1,name==="a"?"trial_b":"compare");
 }
 label("compare");C(3,43);R(0,3);C(3,44);R(1,3);S(0,0,1);B(0,"bad","diff");
 label("bad");op("Halt",{});
 label("diff");C(3,45);R(0,3);C(3,46);R(1,3);S(0,0,1);C(3,47);R(1,3);B(1,"phase1test","train0");
 label("phase1test");C(2,1);S(1,1,2);B(1,"phase2test","train1");
 label("phase2test");S(1,1,2);B(1,"audit","train2");
 label("train0");C(3,48);W(3,0);set(47,1);pair(2,1,"trial_a");
 label("train1");C(3,48);R(1,3);S(0,0,1);C(3,49);W(3,0);set(47,2);pair(1,2,"trial_a");
 label("train2");C(3,48);R(1,3);S(0,0,1);C(3,52);W(3,0);set(47,3,"predict");
 label("predict");C(3,50);R(0,3);C(3,40);W(3,0);C(3,51);R(0,3);C(3,41);W(3,0);
 C(3,48);R(0,3);
 for(const [name,addr,coef] of [["x",40,49],["y",41,52]]){
  C(3,addr);R(1,3);C(3,coef);R(2,3);C(3,1);S(1,1,3);B(1,"pred_loop_"+name,"pred_end_"+name);
  label("pred_loop_"+name);A(0,0,2);S(1,1,3);B(1,"pred_loop_"+name,"pred_end_"+name);
  label("pred_end_"+name);
 }
 C(3,53);W(3,0,"trial_a");
 label("audit");C(3,54);W(3,0,"predict");
 for(let i=0;i<g.length;i++){const n=g[i];if(n.kind==="Branch"){n.nonzero=l[n.nonzero];n.zero=l[n.zero]}else if(n.kind!=="Halt")n.next=n.next===undefined?i+1:l[n.next];}
 return {g,l};
}
function run(profile,batch){
 const {g,l}=build(profile),d=new Map([[50,batch[0][0]],[51,batch[0][1]]]),v=[0,0,0,0];
 let pc=0,paid=0,peak=g.length+3,trained=null,predStart=0,predCost=0,trainCost=0,trainPairs=[],rows=[],predAt=0;
 while(paid<20000){
  const n=g[pc],p=pc;if(!n||n.kind==="Halt")throw Error("bad execution");if(p===l.predict){predStart=paid;if(trained===null){trained=[d.get(48),d.get(49),d.get(52)];trainCost=paid;}}
  if(p===l.compare){
   const x=d.get(40),y=d.get(41);if(d.get(43)!==x*y||d.get(44)!==x*y)throw Error("product");
   if(d.get(47)<3)trainPairs.push([x,y,d.get(45),d.get(46)]);
  }
  paid++;pc=n.next;
  switch(n.kind){
   case "Const":v[n.dst]=n.value;break;
   case "Read":v[n.dst]=d.get(v[n.address])??0;break;
   case "Write":{
    const k=v[n.address],a=v[n.value];d.set(k,a);
    if(k===53){predCost=paid-predStart;predAt=paid;}
    if(k===54){
     const x=d.get(40),y=d.get(41),prediction=d.get(53),actual=a;
     if(prediction!==trained[0]+(x-1)*trained[1]+(y-1)*trained[2])throw Error("affine");
     if(actual!==d.get(45)-d.get(46))throw Error("diff");
     rows.push({x,y,prediction,actual,correct:Math.sign(prediction)===Math.sign(actual),predictionCost:predCost,auditCost:paid-predAt,trialA:d.get(45),trialB:d.get(46)});
     peak=Math.max(peak,g.length+1+d.size);
     if(rows.length===batch.length)return {profile,paid,peak,nodes:g.length,trainCost,trained,trainPairs,rows};
     const [nx,ny]=batch[rows.length];d.set(50,nx);d.set(51,ny);
    }break;
   }
   case "SenseOwnCredits":v[n.dst]=20000-paid;break;
   case "Add":v[n.dst]=v[n.lhs]+v[n.rhs];break;
   case "Sub":v[n.dst]=v[n.lhs]-v[n.rhs];break;
   case "Branch":pc=v[n.test]!==0?n.nonzero:n.zero;break;
   default:throw Error(n.kind);
  }
  peak=Math.max(peak,g.length+1+d.size);
 }
 throw Error("budget");
}
const batches=[];
for(let x=1;x<=16;x++){const a=[];for(let y=1;y<=16;y++)if(!((x===1&&y<=2)||(x===2&&y===1)))a.push([x,y]);batches.push(a);}
const worlds=["cross","unroll"].flatMap(profile=>batches.map(b=>run(profile,b)));
const summary=Object.fromEntries(["cross","unroll"].map(profile=>{
 const w=worlds.filter(x=>x.profile===profile),r=w.flatMap(x=>x.rows);
 return [profile,{worlds:w.length,n:r.length,nodes:w[0].nodes,peak:Math.max(...w.map(x=>x.peak)),trained:w[0].trained,trainPairs:w[0].trainPairs,exact:r.filter(x=>x.actual===x.prediction).length,signCorrect:r.filter(x=>x.correct).length,meanAbsError:r.reduce((s,x)=>s+Math.abs(x.prediction-x.actual),0)/r.length,maxAbsError:Math.max(...r.map(x=>Math.abs(x.prediction-x.actual))),paid:w.reduce((s,x)=>s+x.paid,0),training:w.reduce((s,x)=>s+x.trainCost,0),prediction:r.reduce((s,x)=>s+x.predictionCost,0),audit:r.reduce((s,x)=>s+x.auditCost,0),worst:r.filter(x=>!x.correct).slice(0,5)}]
}));

assert.equal(worlds.length,32);
assert.deepEqual(summary,{"cross":{"worlds":16,"n":253,"nodes":136,"peak":152,"trained":[0,3,-3],"trainPairs":[[1,1,14,14],[2,1,17,14],[1,2,14,17]],"exact":253,"signCorrect":253,"meanAbsError":0,"maxAbsError":0,"paid":48527,"training":3488,"prediction":18092,"audit":26947,"worst":[]},"unroll":{"worlds":16,"n":253,"nodes":142,"peak":158,"trained":[-3,3,0],"trainPairs":[[1,1,14,17],[2,1,17,17],[1,2,14,17]],"exact":29,"signCorrect":205,"meanAbsError":17.707509881422926,"maxAbsError":35,"paid":48105,"training":3584,"prediction":18092,"audit":26429,"worst":[{"x":3,"y":1,"prediction":3,"actual":-2,"correct":false,"predictionCost":32,"auditCost":75,"trialA":20,"trialB":22},{"x":3,"y":2,"prediction":3,"actual":-2,"correct":false,"predictionCost":35,"auditCost":75,"trialA":20,"trialB":22},{"x":3,"y":3,"prediction":3,"actual":-2,"correct":false,"predictionCost":38,"auditCost":75,"trialA":20,"trialB":22},{"x":3,"y":4,"prediction":3,"actual":-2,"correct":false,"predictionCost":41,"auditCost":75,"trialA":20,"trialB":22},{"x":3,"y":5,"prediction":3,"actual":-2,"correct":false,"predictionCost":44,"auditCost":75,"trialA":20,"trialB":22}]}});
console.log(JSON.stringify({tier:"NON_NATIVE_LEARNING_COMPONENT_MODEL",summary,worlds,sources:{cross:build("cross"),unroll:build("unroll")}},null,2));
