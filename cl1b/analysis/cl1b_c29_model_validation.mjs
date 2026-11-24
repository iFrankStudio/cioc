#!/usr/bin/env node
// Non-native component model. No physical source edit or productive service claim.
import assert from "node:assert/strict";
function build(profile,policy='validated'){
 const g=[],l={};const label=k=>{if(k in l)throw Error(k);l[k]=g.length};
 const op=(kind,args)=>g.push({kind,...args});
 const C=(dst,value,next)=>op("Const",{dst,value,next}),R=(dst,address,next)=>op("Read",{dst,address,next}),W=(address,value,next)=>op("Write",{address,value,next}),S=(dst,lhs,rhs,next)=>op("Sub",{dst,lhs,rhs,next}),A=(dst,lhs,rhs,next)=>op("Add",{dst,lhs,rhs,next}),B=(test,nonzero,zero)=>op("Branch",{test,nonzero,zero}),Q=(dst,next)=>op("SenseOwnCredits",{dst,next});
 const set=(key,value,next)=>{C(3,key);C(0,value);W(3,0,next)};
 const pair=(x,y,next)=>{set(40,x);set(41,y,next)};
 label("init");if(policy==="paired"){set(55,0);set(47,5,"predict");}else{set(55,1);set(47,0);pair(1,1,"trial_a");}
 for(const name of ["a","b"]){
  const special=name==="b"&&profile==="switch8",cross=name==="b"&&(profile==="cross"||special),unroll=name==="b"&&profile==="unroll";
  label("trial_"+name);Q(0);C(3,42);W(3,0);
  if(special){C(3,40);R(1,3);C(3,8);S(1,1,3);B(1,"normal_b","special_b");label("special_b");C(3,40);R(1,3);C(3,41);R(2,3,"setup_b");label("normal_b");}
  C(3,cross?41:40);R(1,3);C(3,cross?40:41);R(2,3);if(special)label("setup_b");C(0,0);C(3,1);
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
 label("phase2test");S(1,1,2);B(1,"phase3test","train2");
 label("phase3test");S(1,1,2);B(1,"phase4test","validate");
 label("phase4test");S(1,1,2);B(1,"audit","validate");
 label("train0");C(3,48);W(3,0);set(47,1);pair(2,1,"trial_a");
 label("train1");C(3,48);R(1,3);S(0,0,1);C(3,49);W(3,0);set(47,2);pair(1,2,"trial_a");
 label("train2");C(3,48);R(1,3);S(0,0,1);C(3,52);W(3,0);if(policy==="unchecked"){set(47,5,"predict");}else{set(47,3);pair(3,1,"predict_math");}
 label("predict");C(3,55);R(0,3);B(0,"capture_live","fallback");
 label("fallback");C(3,50);R(0,3);C(3,40);W(3,0);C(3,51);R(0,3);C(3,41);W(3,0,"trial_a");
 label("capture_live");C(3,50);R(0,3);C(3,40);W(3,0);C(3,51);R(0,3);C(3,41);W(3,0);
 label("predict_math");C(3,48);R(0,3);
 for(const [name,addr,coef] of [["x",40,49],["y",41,52]]){
  C(3,addr);R(1,3);C(3,coef);R(2,3);C(3,1);S(1,1,3);B(1,"pred_loop_"+name,"pred_end_"+name);
  label("pred_loop_"+name);A(0,0,2);S(1,1,3);B(1,"pred_loop_"+name,"pred_end_"+name);
  label("pred_end_"+name);
 }
 C(3,53);W(3,0);
 C(3,47);R(1,3);C(2,5);S(1,1,2);B(1,"trial_a","predicted");
 label("predicted");C(3,54);W(3,0,"predict");
 label("validate");C(3,53);R(1,3);S(0,0,1);B(0,"reject","validation_next");
 label("reject");set(55,0,"validation_next");
 label("validation_next");C(3,47);R(1,3);C(2,3);S(1,1,2);B(1,"validation_done","validation_second");
 label("validation_second");set(47,4);pair(1,3,"predict_math");
 label("validation_done");set(47,5,"predict");
 label("audit");C(3,54);W(3,0,"predict");
 for(let i=0;i<g.length;i++){const n=g[i];if(n.kind==="Branch"){n.nonzero=l[n.nonzero];n.zero=l[n.zero]}else if(n.kind!=="Halt")n.next=n.next===undefined?i+1:l[n.next];}
 return {g,l};
}

function run(profile,batch,policy="validated"){
 const {g,l}=build(profile,policy),d=new Map([[50,batch[0][0]],[51,batch[0][1]]]),v=[0,0,0,0];let pc=0,paid=0,peak=g.length+3,setup=null,begin=0,rows=[],validation=[],trials=0;
 while(paid<20000){
 const p=pc,n=g[p];if(!n||n.kind==="Halt")throw Error("bad execution");
 if(p===l.predict){begin=paid;if(setup===null)setup={paid,trust:d.get(55),params:[d.get(48)??null,d.get(49)??null,d.get(52)??null]};}
 if(p===l.compare){const x=d.get(40),y=d.get(41);if(d.get(43)!==x*y||d.get(44)!==x*y)throw Error("product");trials+=2;
 if(d.get(47)===3||d.get(47)===4)validation.push({x,y,pred:d.get(53),actual:d.get(45)-d.get(46)});}
 paid++;pc=n.next;
 switch(n.kind){
 case "Const":v[n.dst]=n.value;break;
 case "Read":v[n.dst]=d.get(v[n.address])??0;break;
 case "Write":{const k=v[n.address],a=v[n.value];d.set(k,a);if(k===54){
 const [x,y]=batch[rows.length];if(x!==d.get(40)||y!==d.get(41))throw Error("input");
 const costA=11+3*x,costB=profile==="cross"?11+3*y:profile==="switch8"?16+3*(x===8?x:y):12+5*Math.floor(x/2)+(x%2?5:0);
 const actual=costA-costB,trust=d.get(55);
 if(!trust&&a!==actual)throw Error("reference "+profile+" "+x+" "+y+" "+a+" "+actual);
 rows.push({x,y,emitted:a,actual,trust,paid:paid-begin,exact:a===actual,signCorrect:Math.sign(a)===Math.sign(actual)});
 peak=Math.max(peak,g.length+1+d.size);
 if(rows.length===batch.length)return {profile,policy,nodes:g.length,peak,paid,setup,validation,rows,trials};
 const [nx,ny]=batch[rows.length];d.set(50,nx);d.set(51,ny);
 }break;}
 case "SenseOwnCredits":v[n.dst]=20000-paid;break;
 case "Add":v[n.dst]=v[n.lhs]+v[n.rhs];break;
 case "Sub":v[n.dst]=v[n.lhs]-v[n.rhs];break;
 case "Branch":pc=v[n.test]!==0?n.nonzero:n.zero;break;
 default:throw Error(n.kind);}
 peak=Math.max(peak,g.length+1+d.size);
 }throw Error("budget");
}
const batches=[];for(let x=1;x<=16;x++){const a=[];for(let y=1;y<=16;y++)if(![[1,1],[2,1],[1,2],[3,1],[1,3]].some(z=>z[0]===x&&z[1]===y))a.push([x,y]);batches.push(a);}
const profiles=["cross","unroll","switch8"],policies=["validated","unchecked","paired"];
const worlds=profiles.flatMap(p=>batches.flatMap(b=>policies.map(q=>run(p,b,q))));
for(const p of profiles)for(const b of batches){
 const w=worlds.filter(z=>z.profile===p&&z.rows[0].x===b[0][0]);const measured=w.find(z=>z.policy==="paired");
 for(const z of w)for(let i=0;i<z.rows.length;i++)if(z.rows[i].actual!==measured.rows[i].emitted)throw Error("paired reference mismatch");
}
const summary=Object.fromEntries(profiles.map(p=>[p,Object.fromEntries(policies.map(q=>{const w=worlds.filter(z=>z.profile===p&&z.policy===q),r=w.flatMap(z=>z.rows);return [q,{n:r.length,nodes:w[0].nodes,peak:Math.max(...w.map(z=>z.peak)),setup:w[0].setup,validation:w[0].validation,accepted:r.filter(z=>z.trust).length,exact:r.filter(z=>z.exact).length,signCorrect:r.filter(z=>z.signCorrect).length,paid:w.reduce((s,z)=>s+z.paid,0),setupTotal:w.reduce((s,z)=>s+z.setup.paid,0),trials:w.reduce((s,z)=>s+z.trials,0),failures:r.filter(z=>!z.signCorrect).slice(0,5)}]}))]));

assert.equal(worlds.length,144);
assert.deepEqual(summary,{"cross":{"validated":{"n":251,"nodes":191,"peak":208,"setup":{"paid":451,"trust":1,"params":[0,3,-3]},"validation":[{"x":3,"y":1,"pred":6,"actual":6},{"x":1,"y":3,"pred":-6,"actual":-6}],"accepted":251,"exact":251,"signCorrect":251,"paid":27754,"setupTotal":7216,"trials":160,"failures":[]},"unchecked":{"n":251,"nodes":185,"peak":202,"setup":{"paid":221,"trust":1,"params":[0,3,-3]},"validation":[],"accepted":251,"exact":251,"signCorrect":251,"paid":24074,"setupTotal":3536,"trials":96,"failures":[]},"paired":{"n":251,"nodes":185,"peak":198,"setup":{"paid":6,"trust":0,"params":[null,null,null]},"validation":[],"accepted":0,"exact":251,"signCorrect":251,"paid":30674,"setupTotal":96,"trials":502,"failures":[]}},"unroll":{"validated":{"n":251,"nodes":197,"peak":214,"setup":{"paid":465,"trust":0,"params":[-3,3,0]},"validation":[{"x":3,"y":1,"pred":3,"actual":-2},{"x":1,"y":3,"pred":-3,"actual":-3}],"accepted":0,"exact":251,"signCorrect":251,"paid":37495,"setupTotal":7440,"trials":662,"failures":[]},"unchecked":{"n":251,"nodes":191,"peak":208,"setup":{"paid":227,"trust":1,"params":[-3,3,0]},"validation":[],"accepted":251,"exact":28,"signCorrect":204,"paid":24170,"setupTotal":3632,"trials":96,"failures":[{"x":3,"y":2,"emitted":3,"actual":-2,"trust":1,"paid":45,"exact":false,"signCorrect":false},{"x":3,"y":3,"emitted":3,"actual":-2,"trust":1,"paid":48,"exact":false,"signCorrect":false},{"x":3,"y":4,"emitted":3,"actual":-2,"trust":1,"paid":51,"exact":false,"signCorrect":false},{"x":3,"y":5,"emitted":3,"actual":-2,"trust":1,"paid":54,"exact":false,"signCorrect":false},{"x":3,"y":6,"emitted":3,"actual":-2,"trust":1,"paid":57,"exact":false,"signCorrect":false}]},"paired":{"n":251,"nodes":191,"peak":204,"setup":{"paid":6,"trust":0,"params":[null,null,null]},"validation":[],"accepted":0,"exact":251,"signCorrect":251,"paid":30151,"setupTotal":96,"trials":502,"failures":[]}},"switch8":{"validated":{"n":251,"nodes":200,"peak":217,"setup":{"paid":476,"trust":1,"params":[-5,3,-3]},"validation":[{"x":3,"y":1,"pred":1,"actual":1},{"x":1,"y":3,"pred":-11,"actual":-11}],"accepted":251,"exact":236,"signCorrect":245,"paid":28154,"setupTotal":7616,"trials":160,"failures":[{"x":8,"y":1,"emitted":16,"actual":-5,"trust":1,"paid":57,"exact":false,"signCorrect":false},{"x":8,"y":2,"emitted":13,"actual":-5,"trust":1,"paid":60,"exact":false,"signCorrect":false},{"x":8,"y":3,"emitted":10,"actual":-5,"trust":1,"paid":63,"exact":false,"signCorrect":false},{"x":8,"y":4,"emitted":7,"actual":-5,"trust":1,"paid":66,"exact":false,"signCorrect":false},{"x":8,"y":5,"emitted":4,"actual":-5,"trust":1,"paid":69,"exact":false,"signCorrect":false}]},"unchecked":{"n":251,"nodes":194,"peak":211,"setup":{"paid":236,"trust":1,"params":[-5,3,-3]},"validation":[],"accepted":251,"exact":236,"signCorrect":245,"paid":24314,"setupTotal":3776,"trials":96,"failures":[{"x":8,"y":1,"emitted":16,"actual":-5,"trust":1,"paid":57,"exact":false,"signCorrect":false},{"x":8,"y":2,"emitted":13,"actual":-5,"trust":1,"paid":60,"exact":false,"signCorrect":false},{"x":8,"y":3,"emitted":10,"actual":-5,"trust":1,"paid":63,"exact":false,"signCorrect":false},{"x":8,"y":4,"emitted":7,"actual":-5,"trust":1,"paid":66,"exact":false,"signCorrect":false},{"x":8,"y":5,"emitted":4,"actual":-5,"trust":1,"paid":69,"exact":false,"signCorrect":false}]},"paired":{"n":251,"nodes":194,"peak":207,"setup":{"paid":6,"trust":0,"params":[null,null,null]},"validation":[],"accepted":0,"exact":251,"signCorrect":251,"paid":31905,"setupTotal":96,"trials":502,"failures":[]}}});
console.log(JSON.stringify({tier:"NON_NATIVE_LEARNING_COMPONENT_MODEL",summary,worlds,sources:Object.fromEntries(profiles.map(p=>[p,Object.fromEntries(policies.map(q=>[q,build(p,q)]))]))},null,2));
