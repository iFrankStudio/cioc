#!/usr/bin/env node
// C29-R8 non-native model; actor paid indirect addressing and recent-feature verification.
import assert from "node:assert/strict";
function build(profile,policy='validated',period=0,coverage=false){
 const g=[],l={};const label=k=>{if(k in l)throw Error(k);l[k]=g.length};
 const op=(kind,args)=>g.push({kind,...args});
 const C=(dst,value,next)=>op("Const",{dst,value,next}),R=(dst,address,next)=>op("Read",{dst,address,next}),W=(address,value,next)=>op("Write",{address,value,next}),S=(dst,lhs,rhs,next)=>op("Sub",{dst,lhs,rhs,next}),A=(dst,lhs,rhs,next)=>op("Add",{dst,lhs,rhs,next}),B=(test,nonzero,zero)=>op("Branch",{test,nonzero,zero}),Q=(dst,next)=>op("SenseOwnCredits",{dst,next});
 const set=(key,value,next)=>{C(3,key);C(0,value);W(3,0,next)};
 const pair=(x,y,next)=>{set(40,x);set(41,y,next)};
 label("init");if(policy==="paired"){set(55,0);set(47,5,"predict");}else{set(55,1);set(47,0);pair(1,1,"trial_a");}
 for(const name of ["a","b"]){
  const special=name==="b"&&(profile.startsWith("switch")||profile.startsWith("yswitch")),cross=name==="b"&&(profile==="cross"||special),unroll=name==="b"&&profile==="unroll";
  label("trial_"+name);Q(0);C(3,42);W(3,0);
  if(special){C(3,profile.startsWith("yswitch")?41:40);R(1,3);C(3,Number(profile.replace("yswitch","").replace("switch","")));S(1,1,3);B(1,"normal_b","special_b");label("special_b");C(3,40);R(1,3);C(3,41);R(2,3,"setup_b");label("normal_b");}
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
 label("predict");C(3,50);R(0,3);C(3,40);W(3,0);C(3,51);R(0,3);C(3,41);W(3,0);C(3,55);R(0,3);B(0,"predict_math","trial_a");
 label("predict_math");C(3,48);R(0,3);
 for(const [name,addr,coef] of [["x",40,49],["y",41,52]]){
  C(3,addr);R(1,3);C(3,coef);R(2,3);C(3,1);S(1,1,3);B(1,"pred_loop_"+name,"pred_end_"+name);
  label("pred_loop_"+name);A(0,0,2);S(1,1,3);B(1,"pred_loop_"+name,"pred_end_"+name);
  label("pred_end_"+name);
 }
 C(3,53);W(3,0);
 C(3,47);R(1,3);
 if(coverage){
  C(2,3);S(1,1,2);B(1,"cover_four","trial_a");
  label("cover_four");C(2,1);S(1,1,2);B(1,"cover_x","trial_a");
  label("cover_x");
  if(coverage==="indexed"){C(3,40);R(1,3);C(2,70);A(1,1,2);R(2,1);B(2,"predicted","trial_a");}
  else{C(3,40);R(2,3);S(1,1,2);B(1,coverage==="recent"?"trial_a":"predicted",coverage==="recent"?"predicted":"trial_a");}
 }else if(period){
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
 label("validation_done");set(47,coverage==="recent"?21:period?4+period:5,"predict");
 label("audit");
 if(coverage||period){
  C(3,55);R(1,3);B(1,"audit_residual","emit_measured");
  label("audit_residual");C(3,53);R(1,3);S(2,0,1);B(2,"revoke","reset_audit");
  label("revoke");C(3,55);C(1,0);W(3,1,"emit_measured");
  label("reset_audit");C(3,47);if(coverage==="indexed"){C(3,40);R(1,3);C(2,70);A(1,1,2);C(2,1);W(1,2);}
  else if(coverage==="recent"){C(3,40);R(1,3);C(2,4);A(1,1,2);C(3,47);W(3,1);}
  else if(coverage){R(1,3);C(2,1);A(1,1,2);C(2,21);S(2,1,2);B(2,"cover_store","cover_wrap");label("cover_wrap");C(1,5);label("cover_store");W(3,1);}else{C(1,4+period);W(3,1);}
  label("emit_measured");
 }
 C(3,54);W(3,0,"predict");
 for(let i=0;i<g.length;i++){const n=g[i];if(n.kind==="Branch"){n.nonzero=l[n.nonzero];n.zero=l[n.zero]}else if(n.kind!=="Halt")n.next=n.next===undefined?i+1:l[n.next];}
 return {g,l};
}

function run(profile,batch,policy="validated",period=0,coverage=false){
 const {g,l}=build(profile,policy,period,coverage),d=new Map([[50,batch[0][0]],[51,batch[0][1]]]),v=[0,0,0,0];let pc=0,paid=0,peak=g.length+3,setup=null,begin=0,rows=[],validation=[],trials=0,audits=[],queryTrial=false;
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
 const costA=11+3*x,costB=profile==="cross"?11+3*y:(profile.startsWith("switch")||profile.startsWith("yswitch"))?16+3*((profile.startsWith("yswitch")?y:x)===Number(profile.replace("yswitch","").replace("switch",""))?x:y):12+5*Math.floor(x/2)+(x%2?5:0);
 const actual=costA-costB,trust=d.get(55);
 if(!trust&&a!==actual)throw Error("reference "+profile+" "+x+" "+y+" "+a+" "+actual);
 rows.push({x,y,emitted:a,actual,trust,measured:queryTrial,verifiedCells:[...d.keys()].filter(k=>k>=71&&k<=86).length,paid:paid-begin,exact:a===actual,signCorrect:Math.sign(a)===Math.sign(actual)});
 peak=Math.max(peak,g.length+1+d.size);
 if(rows.length===batch.length)return {profile,policy,period,coverage,nodes:g.length,peak,paid,setup,validation,rows,trials,audits};
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


const profiles=["cross","unroll","switch5","switch11","switch16","yswitch5","yswitch11"];
const policies=[["paired",0,false],["validated",4,false],["validated",0,true],["validated",0,"recent"],["validated",0,"indexed"]];
const batches=[];
for(const order of ["ascending","descending","permuted"])for(let shift=0;shift<4;shift++){
 const pairs=[];
 for(let i=0;i<64;i++){
  let x=1+((order==="ascending"?i:order==="descending"?63-i:13*i+shift*5)%16);
  let y=1+((Math.floor(i/16)*4+shift+5*i)%16);
  if(x===1&&y<=3||y===1&&x<=3)y=4;
  pairs.push([x,y]);
 }
 batches.push({id:order+"_"+shift,group:"grid",pairs});
}
function stress(profile){
 const target=Number(profile.replace("yswitch","").replace("switch",""));const axis=profile.startsWith("yswitch")?1:0;
 return ["missing_first","clustered","rare_first","missing_middle","feature_alias"].map(type=>({id:type,group:"stress",pairs:Array.from({length:64},(_,i)=>{
  if(type==="feature_alias")return axis===1?[8,i<32?4:target]:[i<32?4:target,8];
  let x=type==="missing_first"?(i%3===0?target:2+i%14):type==="clustered"?(i<32?target:1+(i-32)%16):type==="rare_first"?(i===60?1:target):(i===0?1:i%2===0?target:3);
  return axis===1?[4,x]:[x,4];
 })}));
}
const worlds=profiles.flatMap(p=>[...batches,...(p.includes("switch")?stress(p):[])].flatMap(b=>policies.map(([q,k,c])=>({id:b.id,group:b.group,...run(p,b.pairs,q,k,c)}))));
function name(q,k,c){return c===true?"ordered":c||q+"_"+k}
function aggregate(group,p,q,k,c){
 const w=worlds.filter(z=>z.group===group&&z.profile===p&&z.policy===q&&z.period===k&&z.coverage===c),r=w.flatMap(z=>z.rows);
 const paid=w.reduce((s,z)=>s+z.paid,0),base=worlds.filter(z=>z.group===group&&z.profile===p&&z.policy==="paired").reduce((s,z)=>s+z.paid,0);
 return {worlds:w.length,queries:r.length,nodes:w[0].nodes,peak:Math.max(...w.map(z=>z.peak)),paid,net:base-paid,errors:r.filter(z=>!z.exact).length,signErrors:r.filter(z=>!z.signCorrect).length,audits:w.reduce((s,z)=>s+z.audits.length,0),revoked:w.filter(z=>z.setup.trust===1&&z.rows.some(a=>!a.trust)).length,verifiedCells:Math.max(...r.map(z=>z.verifiedCells))};
}
const summary=Object.fromEntries(["grid","stress"].map(group=>[group,Object.fromEntries(profiles.filter(p=>group==="grid"||p.includes("switch")).map(p=>[p,Object.fromEntries(policies.map(([q,k,c])=>[name(q,k,c),aggregate(group,p,q,k,c)]))]))]));
for(const w of worlds){const paired=worlds.find(z=>z.profile===w.profile&&z.id===w.id&&z.policy==="paired");for(let i=0;i<w.rows.length;i++)if(w.rows[i].actual!==paired.rows[i].emitted)throw Error("paired mismatch");}

assert.equal(worlds.length,545);
assert.deepEqual(summary,{"grid":{"cross":{"paired_0":{"worlds":12,"queries":768,"nodes":177,"peak":190,"paid":93108,"net":0,"errors":0,"signErrors":0,"audits":0,"revoked":0,"verifiedCells":0},"validated_4":{"worlds":12,"queries":768,"nodes":205,"peak":222,"paid":96783,"net":-3675,"errors":0,"signErrors":0,"audits":192,"revoked":0,"verifiedCells":0},"ordered":{"worlds":12,"queries":768,"nodes":209,"peak":226,"paid":112021,"net":-18913,"errors":0,"signErrors":0,"audits":319,"revoked":0,"verifiedCells":0},"recent":{"worlds":12,"queries":768,"nodes":207,"peak":224,"paid":166944,"net":-73836,"errors":0,"signErrors":0,"audits":768,"revoked":0,"verifiedCells":0},"indexed":{"worlds":12,"queries":768,"nodes":209,"peak":242,"paid":98127,"net":-5019,"errors":0,"signErrors":0,"audits":192,"revoked":0,"verifiedCells":16}},"unroll":{"paired_0":{"worlds":12,"queries":768,"nodes":183,"peak":196,"paid":91464,"net":0,"errors":0,"signErrors":0,"audits":0,"revoked":0,"verifiedCells":0},"validated_4":{"worlds":12,"queries":768,"nodes":211,"peak":228,"paid":99312,"net":-7848,"errors":0,"signErrors":0,"audits":0,"revoked":0,"verifiedCells":0},"ordered":{"worlds":12,"queries":768,"nodes":215,"peak":232,"paid":99312,"net":-7848,"errors":0,"signErrors":0,"audits":0,"revoked":0,"verifiedCells":0},"recent":{"worlds":12,"queries":768,"nodes":213,"peak":230,"paid":99312,"net":-7848,"errors":0,"signErrors":0,"audits":0,"revoked":0,"verifiedCells":0},"indexed":{"worlds":12,"queries":768,"nodes":215,"peak":232,"paid":99312,"net":-7848,"errors":0,"signErrors":0,"audits":0,"revoked":0,"verifiedCells":0}},"switch5":{"paired_0":{"worlds":12,"queries":768,"nodes":186,"peak":199,"paid":96516,"net":0,"errors":0,"signErrors":0,"audits":0,"revoked":0,"verifiedCells":0},"validated_4":{"worlds":12,"queries":768,"nodes":214,"peak":231,"paid":100503,"net":-3987,"errors":24,"signErrors":6,"audits":127,"revoked":5,"verifiedCells":0},"ordered":{"worlds":12,"queries":768,"nodes":218,"peak":235,"paid":95429,"net":1087,"errors":22,"signErrors":5,"audits":84,"revoked":6,"verifiedCells":0},"recent":{"worlds":12,"queries":768,"nodes":216,"peak":233,"paid":115391,"net":-18875,"errors":0,"signErrors":0,"audits":130,"revoked":12,"verifiedCells":0},"indexed":{"worlds":12,"queries":768,"nodes":218,"peak":251,"paid":111173,"net":-14657,"errors":6,"signErrors":2,"audits":120,"revoked":10,"verifiedCells":16}},"switch11":{"paired_0":{"worlds":12,"queries":768,"nodes":186,"peak":199,"paid":97284,"net":0,"errors":0,"signErrors":0,"audits":0,"revoked":0,"verifiedCells":0},"validated_4":{"worlds":12,"queries":768,"nodes":214,"peak":231,"paid":98505,"net":-1221,"errors":39,"signErrors":24,"audits":178,"revoked":1,"verifiedCells":0},"ordered":{"worlds":12,"queries":768,"nodes":218,"peak":235,"paid":95558,"net":1726,"errors":25,"signErrors":15,"audits":104,"revoked":6,"verifiedCells":0},"recent":{"worlds":12,"queries":768,"nodes":216,"peak":233,"paid":115699,"net":-18415,"errors":0,"signErrors":0,"audits":122,"revoked":12,"verifiedCells":0},"indexed":{"worlds":12,"queries":768,"nodes":218,"peak":251,"paid":111226,"net":-13942,"errors":6,"signErrors":4,"audits":113,"revoked":10,"verifiedCells":16}},"switch16":{"paired_0":{"worlds":12,"queries":768,"nodes":186,"peak":199,"paid":97956,"net":0,"errors":0,"signErrors":0,"audits":0,"revoked":0,"verifiedCells":0},"validated_4":{"worlds":12,"queries":768,"nodes":214,"peak":231,"paid":100350,"net":-2394,"errors":24,"signErrors":23,"audits":131,"revoked":5,"verifiedCells":0},"ordered":{"worlds":12,"queries":768,"nodes":218,"peak":235,"paid":97518,"net":438,"errors":27,"signErrors":26,"audits":127,"revoked":4,"verifiedCells":0},"recent":{"worlds":12,"queries":768,"nodes":216,"peak":233,"paid":115399,"net":-17443,"errors":0,"signErrors":0,"audits":110,"revoked":12,"verifiedCells":0},"indexed":{"worlds":12,"queries":768,"nodes":218,"peak":251,"paid":112787,"net":-14831,"errors":3,"signErrors":3,"audits":100,"revoked":11,"verifiedCells":16}},"yswitch5":{"paired_0":{"worlds":12,"queries":768,"nodes":186,"peak":199,"paid":97380,"net":0,"errors":0,"signErrors":0,"audits":0,"revoked":0,"verifiedCells":0},"validated_4":{"worlds":12,"queries":768,"nodes":214,"peak":231,"paid":99825,"net":-2445,"errors":32,"signErrors":21,"audits":150,"revoked":3,"verifiedCells":0},"ordered":{"worlds":12,"queries":768,"nodes":218,"peak":235,"paid":96749,"net":631,"errors":24,"signErrors":16,"audits":102,"revoked":5,"verifiedCells":0},"recent":{"worlds":12,"queries":768,"nodes":216,"peak":233,"paid":117003,"net":-19623,"errors":0,"signErrors":0,"audits":138,"revoked":12,"verifiedCells":0},"indexed":{"worlds":12,"queries":768,"nodes":218,"peak":251,"paid":113385,"net":-16005,"errors":6,"signErrors":4,"audits":136,"revoked":10,"verifiedCells":16}},"yswitch11":{"paired_0":{"worlds":12,"queries":768,"nodes":186,"peak":199,"paid":96612,"net":0,"errors":0,"signErrors":0,"audits":0,"revoked":0,"verifiedCells":0},"validated_4":{"worlds":12,"queries":768,"nodes":214,"peak":231,"paid":99246,"net":-2634,"errors":31,"signErrors":9,"audits":153,"revoked":3,"verifiedCells":0},"ordered":{"worlds":12,"queries":768,"nodes":218,"peak":235,"paid":97110,"net":-498,"errors":14,"signErrors":6,"audits":70,"revoked":10,"verifiedCells":0},"recent":{"worlds":12,"queries":768,"nodes":216,"peak":233,"paid":115355,"net":-18743,"errors":0,"signErrors":0,"audits":130,"revoked":12,"verifiedCells":0},"indexed":{"worlds":12,"queries":768,"nodes":218,"peak":251,"paid":110398,"net":-13786,"errors":6,"signErrors":2,"audits":113,"revoked":10,"verifiedCells":16}}},"stress":{"switch5":{"paired_0":{"worlds":5,"queries":320,"nodes":186,"peak":199,"paid":34032,"net":0,"errors":0,"signErrors":0,"audits":0,"revoked":0,"verifiedCells":0},"validated_4":{"worlds":5,"queries":320,"nodes":214,"peak":231,"paid":35578,"net":-1546,"errors":41,"signErrors":0,"audits":28,"revoked":4,"verifiedCells":0},"ordered":{"worlds":5,"queries":320,"nodes":218,"peak":235,"paid":25848,"net":8184,"errors":183,"signErrors":0,"audits":7,"revoked":1,"verifiedCells":0},"recent":{"worlds":5,"queries":320,"nodes":216,"peak":233,"paid":36565,"net":-2533,"errors":0,"signErrors":0,"audits":8,"revoked":5,"verifiedCells":0},"indexed":{"worlds":5,"queries":320,"nodes":218,"peak":237,"paid":36643,"net":-2611,"errors":0,"signErrors":0,"audits":8,"revoked":5,"verifiedCells":2}},"switch11":{"paired_0":{"worlds":5,"queries":320,"nodes":186,"peak":199,"paid":40581,"net":0,"errors":0,"signErrors":0,"audits":0,"revoked":0,"verifiedCells":0},"validated_4":{"worlds":5,"queries":320,"nodes":214,"peak":231,"paid":41461,"net":-880,"errors":41,"signErrors":41,"audits":28,"revoked":4,"verifiedCells":0},"ordered":{"worlds":5,"queries":320,"nodes":218,"peak":235,"paid":29565,"net":11016,"errors":182,"signErrors":182,"audits":13,"revoked":1,"verifiedCells":0},"recent":{"worlds":5,"queries":320,"nodes":216,"peak":233,"paid":43204,"net":-2623,"errors":0,"signErrors":0,"audits":8,"revoked":5,"verifiedCells":0},"indexed":{"worlds":5,"queries":320,"nodes":218,"peak":237,"paid":43282,"net":-2701,"errors":0,"signErrors":0,"audits":8,"revoked":5,"verifiedCells":2}},"switch16":{"paired_0":{"worlds":5,"queries":320,"nodes":186,"peak":199,"paid":45969,"net":0,"errors":0,"signErrors":0,"audits":0,"revoked":0,"verifiedCells":0},"validated_4":{"worlds":5,"queries":320,"nodes":214,"peak":231,"paid":46294,"net":-325,"errors":41,"signErrors":41,"audits":28,"revoked":4,"verifiedCells":0},"ordered":{"worlds":5,"queries":320,"nodes":218,"peak":235,"paid":32745,"net":13224,"errors":180,"signErrors":180,"audits":18,"revoked":1,"verifiedCells":0},"recent":{"worlds":5,"queries":320,"nodes":216,"peak":233,"paid":48667,"net":-2698,"errors":0,"signErrors":0,"audits":8,"revoked":5,"verifiedCells":0},"indexed":{"worlds":5,"queries":320,"nodes":218,"peak":237,"paid":48745,"net":-2776,"errors":0,"signErrors":0,"audits":8,"revoked":5,"verifiedCells":2}},"yswitch5":{"paired_0":{"worlds":5,"queries":320,"nodes":186,"peak":199,"paid":33690,"net":0,"errors":0,"signErrors":0,"audits":0,"revoked":0,"verifiedCells":0},"validated_4":{"worlds":5,"queries":320,"nodes":214,"peak":231,"paid":35410,"net":-1720,"errors":41,"signErrors":3,"audits":28,"revoked":4,"verifiedCells":0},"ordered":{"worlds":5,"queries":320,"nodes":218,"peak":235,"paid":24066,"net":9624,"errors":185,"signErrors":32,"audits":0,"revoked":0,"verifiedCells":0},"recent":{"worlds":5,"queries":320,"nodes":216,"peak":233,"paid":31949,"net":1741,"errors":63,"signErrors":32,"audits":5,"revoked":3,"verifiedCells":0},"indexed":{"worlds":5,"queries":320,"nodes":218,"peak":236,"paid":32211,"net":1479,"errors":63,"signErrors":32,"audits":5,"revoked":3,"verifiedCells":1}},"yswitch11":{"paired_0":{"worlds":5,"queries":320,"nodes":186,"peak":199,"paid":33621,"net":0,"errors":0,"signErrors":0,"audits":0,"revoked":0,"verifiedCells":0},"validated_4":{"worlds":5,"queries":320,"nodes":214,"peak":231,"paid":36151,"net":-2530,"errors":41,"signErrors":0,"audits":28,"revoked":4,"verifiedCells":0},"ordered":{"worlds":5,"queries":320,"nodes":218,"peak":235,"paid":27306,"net":6315,"errors":184,"signErrors":0,"audits":0,"revoked":0,"verifiedCells":0},"recent":{"worlds":5,"queries":320,"nodes":216,"peak":233,"paid":33068,"net":553,"errors":63,"signErrors":0,"audits":5,"revoked":3,"verifiedCells":0},"indexed":{"worlds":5,"queries":320,"nodes":218,"peak":236,"paid":33330,"net":291,"errors":63,"signErrors":0,"audits":5,"revoked":3,"verifiedCells":1}}}});
console.log(JSON.stringify({tier:"NON_NATIVE_LEARNING_COMPONENT_MODEL",summary,worlds},null,2));
