#!/usr/bin/env node
// C29-R6: paid runtime validation; no native or productive self-edit claim.
import assert from "node:assert/strict";
function build(profile,policy='validated',period=0){
 const g=[],l={};const label=k=>{if(k in l)throw Error(k);l[k]=g.length};
 const op=(kind,args)=>g.push({kind,...args});
 const C=(dst,value,next)=>op("Const",{dst,value,next}),R=(dst,address,next)=>op("Read",{dst,address,next}),W=(address,value,next)=>op("Write",{address,value,next}),S=(dst,lhs,rhs,next)=>op("Sub",{dst,lhs,rhs,next}),A=(dst,lhs,rhs,next)=>op("Add",{dst,lhs,rhs,next}),B=(test,nonzero,zero)=>op("Branch",{test,nonzero,zero}),Q=(dst,next)=>op("SenseOwnCredits",{dst,next});
 const set=(key,value,next)=>{C(3,key);C(0,value);W(3,0,next)};
 const pair=(x,y,next)=>{set(40,x);set(41,y,next)};
 label("init");if(policy==="paired"){set(55,0);set(47,5,"predict");}else{set(55,1);set(47,0);pair(1,1,"trial_a");}
 for(const name of ["a","b"]){
  const special=name==="b"&&profile.startsWith("switch"),cross=name==="b"&&(profile==="cross"||special),unroll=name==="b"&&profile==="unroll";
  label("trial_"+name);Q(0);C(3,42);W(3,0);
  if(special){C(3,40);R(1,3);C(3,Number(profile.slice(6)));S(1,1,3);B(1,"normal_b","special_b");label("special_b");C(3,40);R(1,3);C(3,41);R(2,3,"setup_b");label("normal_b");}
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
 C(3,47);R(1,3);
 if(period){
  C(2,3);S(2,1,2);B(2,"phase_four","trial_a");
  label("phase_four");C(2,4);S(2,1,2);B(2,"runtime_check","trial_a");
  label("runtime_check");C(2,5);S(2,1,2);B(2,"runtime_skip","trial_a");
  label("runtime_skip");C(2,1);S(1,1,2);W(3,1,"predicted");
 }else{C(2,5);S(1,1,2);B(1,"trial_a","predicted");}
 label("predicted");C(3,54);W(3,0,"predict");
 label("validate");C(3,53);R(1,3);S(0,0,1);B(0,"reject","validation_next");
 label("reject");set(55,0,"validation_next");
 label("validation_next");C(3,47);R(1,3);C(2,3);S(1,1,2);B(1,"validation_done","validation_second");
 label("validation_second");set(47,4);pair(1,3,"predict_math");
 label("validation_done");set(47,period?4+period:5,"predict");
 label("audit");
 if(period){
  C(3,55);R(1,3);B(1,"audit_residual","emit_measured");
  label("audit_residual");C(3,53);R(1,3);S(2,0,1);B(2,"revoke","reset_audit");
  label("revoke");C(3,55);C(1,0);W(3,1,"emit_measured");
  label("reset_audit");C(3,47);C(1,4+period);W(3,1);
  label("emit_measured");
 }
 C(3,54);W(3,0,"predict");
 for(let i=0;i<g.length;i++){const n=g[i];if(n.kind==="Branch"){n.nonzero=l[n.nonzero];n.zero=l[n.zero]}else if(n.kind!=="Halt")n.next=n.next===undefined?i+1:l[n.next];}
 return {g,l};
}

function run(profile,batch,policy="validated",period=0){
 const {g,l}=build(profile,policy,period),d=new Map([[50,batch[0][0]],[51,batch[0][1]]]),v=[0,0,0,0];let pc=0,paid=0,peak=g.length+3,setup=null,begin=0,rows=[],validation=[],trials=0,audits=[],queryTrial=false;
 while(paid<20000){
 const p=pc,n=g[p];if(!n||n.kind==="Halt")throw Error("bad execution");
 if(p===l.predict){begin=paid;queryTrial=false;if(setup===null)setup={paid,trust:d.get(55),params:[d.get(48)??null,d.get(49)??null,d.get(52)??null]};}
 if(p===l.compare){const x=d.get(40),y=d.get(41);if(d.get(43)!==x*y||d.get(44)!==x*y)throw Error("product");trials+=2;if(setup!==null){queryTrial=true;if(d.get(55))audits.push({i:rows.length,x,y,pred:d.get(53),actual:d.get(45)-d.get(46)});}
 if(d.get(47)===3||d.get(47)===4)validation.push({x,y,pred:d.get(53),actual:d.get(45)-d.get(46)});}
 paid++;pc=n.next;
 switch(n.kind){
 case "Const":v[n.dst]=n.value;break;
 case "Read":v[n.dst]=d.get(v[n.address])??0;break;
 case "Write":{const k=v[n.address],a=v[n.value];d.set(k,a);if(k===54){
 const [x,y]=batch[rows.length];if(x!==d.get(40)||y!==d.get(41))throw Error("input");
 const costA=11+3*x,costB=profile==="cross"?11+3*y:profile.startsWith("switch")?16+3*(x===Number(profile.slice(6))?x:y):12+5*Math.floor(x/2)+(x%2?5:0);
 const actual=costA-costB,trust=d.get(55);
 if(!trust&&a!==actual)throw Error("reference "+profile+" "+x+" "+y+" "+a+" "+actual);
 rows.push({x,y,emitted:a,actual,trust,measured:queryTrial,paid:paid-begin,exact:a===actual,signCorrect:Math.sign(a)===Math.sign(actual)});
 peak=Math.max(peak,g.length+1+d.size);
 if(rows.length===batch.length)return {profile,policy,period,nodes:g.length,peak,paid,setup,validation,rows,trials,audits};
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

const profiles=["cross","unroll","switch5","switch11","switch16"];
const policies=[["paired",0],["validated",0],["validated",4],["validated",8],["validated",16]];
const batches=[];
for(const order of ["ascending","descending","permuted"])for(let shift=0;shift<4;shift++){
 const pairs=[];
 for(let i=0;i<64;i++){
  let x=1+((order==="ascending"?i:order==="descending"?63-i:13*i+shift*5)%16);
  let y=1+((Math.floor(i/16)*4+shift+5*i)%16);
  if(x===1&&y<=3||y===1&&x<=3)y=4;
  pairs.push([x,y]);
 }
 batches.push({id:order+"_"+shift,pairs});
}
const worlds=profiles.flatMap(p=>batches.flatMap(b=>policies.map(([q,k])=>({id:b.id,...run(p,b.pairs,q,k)}))));
function aggregate(p,q,k){
 const w=worlds.filter(z=>z.profile===p&&z.policy===q&&z.period===k),r=w.flatMap(z=>z.rows);
 const paid=w.reduce((s,z)=>s+z.paid,0),base=worlds.filter(z=>z.profile===p&&z.policy==="paired").reduce((s,z)=>s+z.paid,0);
 return {worlds:w.length,queries:r.length,nodes:w[0].nodes,peak:Math.max(...w.map(z=>z.peak)),paid,net:base-paid,exact:r.filter(z=>z.exact).length,signCorrect:r.filter(z=>z.signCorrect).length,measured:r.filter(z=>z.measured).length,audits:w.reduce((s,z)=>s+z.audits.length,0),revoked:w.filter(z=>z.setup.trust===1&&z.rows.some(a=>a.trust===0)).length,badBeforeRevoke:r.filter(z=>!z.exact).length};
}
const summary=Object.fromEntries(profiles.map(p=>[p,Object.fromEntries(policies.map(([q,k])=>[q+"_"+k,aggregate(p,q,k)]))]));
for(const w of worlds){const paired=worlds.find(z=>z.profile===w.profile&&z.id===w.id&&z.policy==="paired");for(let i=0;i<w.rows.length;i++){if(w.rows[i].actual!==paired.rows[i].emitted)throw Error("paired mismatch");}}

assert.equal(worlds.length,300);
assert.deepEqual(summary,{"cross":{"paired_0":{"worlds":12,"queries":768,"nodes":185,"peak":198,"paid":93108,"net":0,"exact":768,"signCorrect":768,"measured":768,"audits":0,"revoked":0,"badBeforeRevoke":0},"validated_0":{"worlds":12,"queries":768,"nodes":191,"peak":208,"paid":67728,"net":25380,"exact":768,"signCorrect":768,"measured":0,"audits":0,"revoked":0,"badBeforeRevoke":0},"validated_4":{"worlds":12,"queries":768,"nodes":213,"peak":230,"paid":96783,"net":-3675,"exact":768,"signCorrect":768,"measured":192,"audits":192,"revoked":0,"badBeforeRevoke":0},"validated_8":{"worlds":12,"queries":768,"nodes":213,"peak":230,"paid":85647,"net":7461,"exact":768,"signCorrect":768,"measured":96,"audits":96,"revoked":0,"badBeforeRevoke":0},"validated_16":{"worlds":12,"queries":768,"nodes":213,"peak":230,"paid":80166,"net":12942,"exact":768,"signCorrect":768,"measured":48,"audits":48,"revoked":0,"badBeforeRevoke":0}},"unroll":{"paired_0":{"worlds":12,"queries":768,"nodes":191,"peak":204,"paid":91464,"net":0,"exact":768,"signCorrect":768,"measured":768,"audits":0,"revoked":0,"badBeforeRevoke":0},"validated_0":{"worlds":12,"queries":768,"nodes":197,"peak":214,"paid":96972,"net":-5508,"exact":768,"signCorrect":768,"measured":768,"audits":0,"revoked":0,"badBeforeRevoke":0},"validated_4":{"worlds":12,"queries":768,"nodes":219,"peak":236,"paid":99312,"net":-7848,"exact":768,"signCorrect":768,"measured":768,"audits":0,"revoked":0,"badBeforeRevoke":0},"validated_8":{"worlds":12,"queries":768,"nodes":219,"peak":236,"paid":99312,"net":-7848,"exact":768,"signCorrect":768,"measured":768,"audits":0,"revoked":0,"badBeforeRevoke":0},"validated_16":{"worlds":12,"queries":768,"nodes":219,"peak":236,"paid":99312,"net":-7848,"exact":768,"signCorrect":768,"measured":768,"audits":0,"revoked":0,"badBeforeRevoke":0}},"switch5":{"paired_0":{"worlds":12,"queries":768,"nodes":194,"peak":207,"paid":96516,"net":0,"exact":768,"signCorrect":768,"measured":768,"audits":0,"revoked":0,"badBeforeRevoke":0},"validated_0":{"worlds":12,"queries":768,"nodes":200,"peak":217,"paid":68028,"net":28488,"exact":726,"signCorrect":758,"measured":0,"audits":0,"revoked":0,"badBeforeRevoke":42},"validated_4":{"worlds":12,"queries":768,"nodes":222,"peak":239,"paid":100503,"net":-3987,"exact":744,"signCorrect":762,"measured":387,"audits":127,"revoked":5,"badBeforeRevoke":24},"validated_8":{"worlds":12,"queries":768,"nodes":222,"peak":239,"paid":86427,"net":10089,"exact":726,"signCorrect":758,"measured":96,"audits":96,"revoked":0,"badBeforeRevoke":42},"validated_16":{"worlds":12,"queries":768,"nodes":222,"peak":239,"paid":80706,"net":15810,"exact":726,"signCorrect":758,"measured":48,"audits":48,"revoked":0,"badBeforeRevoke":42}},"switch11":{"paired_0":{"worlds":12,"queries":768,"nodes":194,"peak":207,"paid":97284,"net":0,"exact":768,"signCorrect":768,"measured":768,"audits":0,"revoked":0,"badBeforeRevoke":0},"validated_0":{"worlds":12,"queries":768,"nodes":200,"peak":217,"paid":68028,"net":29256,"exact":726,"signCorrect":742,"measured":0,"audits":0,"revoked":0,"badBeforeRevoke":42},"validated_4":{"worlds":12,"queries":768,"nodes":222,"peak":239,"paid":98505,"net":-1221,"exact":729,"signCorrect":744,"measured":234,"audits":178,"revoked":1,"badBeforeRevoke":39},"validated_8":{"worlds":12,"queries":768,"nodes":222,"peak":239,"paid":87822,"net":9462,"exact":729,"signCorrect":744,"measured":145,"audits":89,"revoked":1,"badBeforeRevoke":39},"validated_16":{"worlds":12,"queries":768,"nodes":222,"peak":239,"paid":80706,"net":16578,"exact":726,"signCorrect":742,"measured":48,"audits":48,"revoked":0,"badBeforeRevoke":42}},"switch16":{"paired_0":{"worlds":12,"queries":768,"nodes":194,"peak":207,"paid":97956,"net":0,"exact":768,"signCorrect":768,"measured":768,"audits":0,"revoked":0,"badBeforeRevoke":0},"validated_0":{"worlds":12,"queries":768,"nodes":200,"peak":217,"paid":68028,"net":29928,"exact":726,"signCorrect":728,"measured":0,"audits":0,"revoked":0,"badBeforeRevoke":42},"validated_4":{"worlds":12,"queries":768,"nodes":222,"peak":239,"paid":100350,"net":-2394,"exact":744,"signCorrect":745,"measured":375,"audits":131,"revoked":5,"badBeforeRevoke":24},"validated_8":{"worlds":12,"queries":768,"nodes":222,"peak":239,"paid":91167,"net":6789,"exact":741,"signCorrect":742,"measured":264,"audits":72,"revoked":4,"badBeforeRevoke":27},"validated_16":{"worlds":12,"queries":768,"nodes":222,"peak":239,"paid":86904,"net":11052,"exact":741,"signCorrect":742,"measured":228,"audits":36,"revoked":4,"badBeforeRevoke":27}}});
console.log(JSON.stringify({tier:"NON_NATIVE_LEARNING_COMPONENT_MODEL",summary,worlds},null,2));
