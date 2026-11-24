#!/usr/bin/env node
// C29 R3: non-native paid experience-cache model; no new native evidence.
import fs from "node:fs";
import assert from "node:assert/strict";
function build({snapshot=true,repeat=true,freshness='none',admission='none',period=24,cache=0,direct=false}={}){
 const g=[],labels={};
 const label=k=>{if(k in labels)throw Error(k);labels[k]=g.length};
 const emit=(kind,args)=>g.push({kind,...args});
 const C=(dst,value,next)=>emit("Const",{dst,value,next});
 const R=(dst,address,next)=>emit("Read",{dst,address,next});
 const W=(address,value,next)=>emit("Write",{address,value,next});
 const S=(dst,lhs,rhs,next)=>emit("Sub",{dst,lhs,rhs,next});
 const A=(dst,lhs,rhs,next)=>emit("Add",{dst,lhs,rhs,next});
 const B=(test,nonzero,zero)=>emit("Branch",{test,nonzero,zero});
 const Q=(dst,next)=>emit("SenseOwnCredits",{dst,next});
 label("eval");
 if(admission==="periodic"){C(3,60);R(0,3);C(1,period);A(0,0,1);C(3,48);W(3,0);}
 if(freshness==="retry"){C(3,47);C(0,1);W(3,0);}
 label("capture");
 if(snapshot){C(3,50);R(0,3);C(3,40);W(3,0);C(3,51);R(0,3);C(3,41);W(3,0);}
 if(cache){
  for(let i=0;i<cache;i++){
   const key=70+3*i,next=i+1<cache?"lookup_"+(i+1):"trial_a";
   label("lookup_"+i);C(3,40);R(0,3);C(3,key);R(1,3);S(0,0,1);B(0,next,"lookup_y_"+i);
   label("lookup_y_"+i);C(3,41);R(0,3);C(3,key+1);R(1,3);S(0,0,1);B(0,next,"hit_"+i);
   label("hit_"+i);C(3,key+2);R(0,3);B(0,"choose_b","choose_a");
  }
 }
 for(const [name,x,y,out,cost,next] of [["a",snapshot?40:50,snapshot?41:51,43,45,"trial_b"],["b",snapshot?41:51,snapshot?40:50,44,46,"compare"]]){
  label("trial_"+name);Q(0);C(3,42);W(3,0);C(3,x);R(1,3);C(3,y);R(2,3);C(0,0);C(3,1);
  label("loop_"+name);A(0,0,2);S(1,1,3);B(1,"loop_"+name,"output_"+name);
  label("output_"+name);C(3,out);W(3,0);Q(0);C(3,42);R(1,3);S(1,1,0);C(3,cost);W(3,1,next);
 }
 label("compare");
 if(freshness==="guard"||freshness==="retry"){
  C(3,50);R(0,3);C(3,40);R(1,3);S(0,0,1);B(0,"stale","fresh_y");
  label("fresh_y");C(3,51);R(0,3);C(3,41);R(1,3);S(0,0,1);B(0,"stale","matched");
  label("stale");
  if(freshness==="retry"){C(3,47);R(0,3);B(0,"retry","dispatch");label("retry");C(0,0);W(3,0,"capture");}
  else C(3,0,"dispatch");
 }
 label("matched");C(3,43);R(0,3);C(3,44);R(1,3);S(0,0,1);B(0,"dispatch","costs");
 label("costs");C(3,45);R(0,3);C(3,46);R(1,3);S(2,0,1);S(1,1,0);C(3,3);B(2,"cmp1","dispatch");
 for(let i=1;i<=15;i++){label("cmp"+i);S(2,2,3);B(2,"cmpB"+i,"choose_b");label("cmpB"+i);S(1,1,3);B(1,i===15?"dispatch":"cmp"+(i+1),"choose_a");}
 for(const name of ["a","b"]){label("choose_"+name);C(1,"@dispatch");emit("ReadNext",{dst:0,node:1});C(2,"@prod_"+name);S(0,0,2);B(0,"patch_"+name,"dispatch");label("patch_"+name);
 if(freshness==="selective"){
  C(3,50);R(0,3);C(3,40);R(1,3);S(0,0,1);B(0,"dispatch","fresh_y_"+name);
  label("fresh_y_"+name);C(3,51);R(0,3);C(3,41);R(1,3);S(0,0,1);B(0,"dispatch","fresh_patch_"+name);
  label("fresh_patch_"+name);C(1,"@dispatch");
 }
 C(0,"@prod_"+name);emit("PatchNext",{node:1,target:0,next:"dispatch"});}
 if(cache){
  for(const name of ["a","b"]){
   const choice=labels["choose_"+name],target="store_"+name;
   // Cache only validated fresh trial preferences. Hits do not refresh FIFO.
   for(let i=0;i<choice;i++)if(g[i].kind==="Branch"){
    if(g[i].zero==="choose_"+name && i>=labels.costs)g[i].zero=target;
   }
   label(target);
   if(cache===2){for(let j=0;j<3;j++){C(3,70+j);R(0,3);C(3,73+j);W(3,0);}}
   for(const [from,to] of [[40,70],[41,71]]){C(3,from);R(0,3);C(3,to);W(3,0);}
   C(0,name==="b"?1:0);C(3,72);W(3,0,"choose_"+name);
  }
 }
 label("dispatch");C(3,0,"prod_a");
 for(const [name,x,y] of [["a",50,51],["b",51,50]]){
  label("prod_"+name);C(3,x);R(1,3);C(3,y);R(2,3);C(0,0);C(3,1);label("prod_loop_"+name);A(0,0,2);S(1,1,3);B(1,"prod_loop_"+name,"prod_out_"+name);
  label("prod_out_"+name);C(3,11);W(3,0,"stage");
 }
 label("stage");C(3,60);R(0,3);C(1,1);A(0,0,1);W(3,0,repeat?"gate":"dispatch");
 if(repeat){
 label("gate");
 if(admission==="periodic"){C(3,48);R(1,3);S(0,0,1);B(0,"dispatch","advance");label("advance");C(0,period);A(1,1,0);W(3,1,"admit");}
 else{C(1,period);S(0,0,1);B(0,"dispatch",admission==="once"?"admit":"eval");}
 if(admission!=="none"){
 label("admit");C(3,50);R(0,3);C(3,40);R(1,3);S(0,0,1);B(0,"eval","admit_y");
 label("admit_y");C(3,51);R(0,3);C(3,41);R(1,3);S(0,0,1);B(0,"eval","dispatch");
 }
}
 if(direct){
  // Task-aware baseline: exploit the known operand-cost relation.
  const entry=labels.trial_a;
  g[entry]={kind:"Const",dst:3,value:40,next:"direct_x"};
  label("direct_x");R(0,3);C(3,41);R(1,3);S(2,0,1);S(1,1,0);C(3,1);B(2,"cmp1","dispatch");
 }
 for(let i=0;i<g.length;i++){const n=g[i];if(n.kind==="Branch"){n.nonzero=labels[n.nonzero];n.zero=labels[n.zero];}else n.next=n.next===undefined?i+1:labels[n.next];if(n.kind==="Const"&&typeof n.value==="string")n.value=labels[n.value.slice(1)];}
 return {g,labels};
}
function run(s,opts,off=false){
 const {g,labels:l}=build(opts),v=[0,0,0,0],d=new Map([[50,s.xy[0]],[51,s.xy[1]]]);let pc=0,paid=0,stage=0,prod=0,trial=0,paired=0,invalid=0,peak=g.length+3;
 const decisions=[],edits=[],probes=[],measurements=[],hits=[];let changed=false;
 while(paid<20000){
  const n=g[pc],old=pc;if(!n)throw Error("invalid pc");paid++;pc=n.next;
  if(old===l.eval)probes.push(stage);if(old===l.hit_0||old===l.hit_1)hits.push(stage);
  if(old===l.compare){paired++;if(d.get(43)!==d.get(44))invalid++;measurements.push({stage,a:d.get(45),b:d.get(46),outputs:[d.get(43),d.get(44)],snapshot:[d.get(40)??null,d.get(41)??null],live:[d.get(50),d.get(51)]});}
  if(old===l.choose_a||old===l.choose_b)decisions.push([stage,old===l.choose_a?"a":"b"]);
  switch(n.kind){
   case "Const":v[n.dst]=n.value;break;
   case "Read":v[n.dst]=d.get(v[n.address])??0;break;
   case "Write":{
    const key=v[n.address],value=v[n.value];d.set(key,value);
    if(key===43||key===44){trial++;if(key===43&&s.midTrial&&!changed){d.set(50,s.midTrial[0]);d.set(51,s.midTrial[1]);changed=true;}}
    if(key===11){prod++;if(value!==d.get(50)*d.get(51))throw Error("wrong product");}
    if(key===60){stage=value;if(prod!==stage)throw Error("count");peak=Math.max(peak,g.length+1+d.size);if(stage===s.n)return {paid,prod,trial,paired,invalid,peak,nodes:g.length,probes,measurements,decisions,edits,hits};for(const [at,x,y] of s.changes??[])if(at===stage){d.set(50,x);d.set(51,y);}}
    break;
   }
   case "SenseOwnCredits":v[n.dst]=20000-paid;break;
   case "Add":v[n.dst]=v[n.lhs]+v[n.rhs];break;
   case "Sub":v[n.dst]=v[n.lhs]-v[n.rhs];break;
   case "Branch":pc=v[n.test]!==0?n.nonzero:n.zero;break;
   case "ReadNext":v[n.dst]=g[v[n.node]].next;break;
   case "PatchNext":if(v[n.node]!==l.dispatch||![l.prod_a,l.prod_b].includes(v[n.target]))throw Error("edit");edits.push([stage,g[v[n.node]].next,v[n.target],off]);if(!off)g[v[n.node]].next=v[n.target];break;
   default:throw Error(n.kind);
  }
  peak=Math.max(peak,g.length+1+d.size);
 }
 throw Error("budget");
}
const cases=JSON.parse(fs.readFileSync(new URL("../studies/CL1B-C29-PAIRED-ARITHMETIC-frozen-80-worlds-v1.json",import.meta.url),"utf8")).cases;

const variants={initial:{repeat:false,freshness:"guard"},periodic:{freshness:"guard",admission:"periodic"},cache1:{freshness:"guard",admission:"periodic",cache:1},cache2:{freshness:"guard",admission:"periodic",cache:2},direct:{freshness:"guard",admission:"periodic",direct:true}};
function test(s){return {id:s.id,group:s.group??s.cohort,results:Object.fromEntries(Object.entries(variants).map(([k,o])=>[k,{on:run(s,o),off:run(s,o,true)}]))}}
const family=[];
for(const [x,y] of [[2,3],[2,8],[2,16],[16,2],[7,8],[8,7]])for(const n of [96,144,192]){
 for(const step of [18,30,48]){
  let changes=[];for(let t=step;t<n;t+=step)changes.push([t,...(changes.length%2?[x,y]:[y,x])]);
  family.push({id:"recur_"+x+"_"+y+"_"+n+"_"+step,group:"recurrent",xy:[x,y],n,changes});
  changes=[];for(let t=step;t<n;t+=step){const z=changes.length+1;changes.push([t,1+(x+z)%16,1+(y+2*z)%16]);}
  family.push({id:"novel_"+x+"_"+y+"_"+n+"_"+step,group:"novel",xy:[x,y],n,changes});
 }
}
function aggregate(rows){return Object.fromEntries(Object.keys(variants).map(k=>{const nets=rows.map(r=>r.results.initial.on.paid-r.results[k].on.paid);const against=rows.map(r=>r.results.periodic.on.paid-r.results[k].on.paid);return [k,{n:rows.length,net:nets.reduce((a,b)=>a+b,0),vsPeriodic:against.reduce((a,b)=>a+b,0),wins:nets.filter(x=>x>0).length,trials:rows.reduce((a,r)=>a+r.results[k].on.trial,0),hits:rows.reduce((a,r)=>a+r.results[k].on.hits.length,0),nodes:rows[0].results[k].on.nodes,peak:Math.max(...rows.map(r=>r.results[k].on.peak)),physical:rows.reduce((a,r)=>a+r.results[k].off.paid-r.results[k].on.paid,0)}]}))}
const known=cases.map(test),grid=family.map(test),summary={known:aggregate(known.filter(r=>r.group==="prospective")),grid:aggregate(grid),recurrent:aggregate(grid.filter(r=>r.group==="recurrent")),novel:aggregate(grid.filter(r=>r.group==="novel"))};
assert.equal(grid.length,108);
assert.deepEqual(summary,{"known":{"initial":{"n":12,"net":0,"vsPeriodic":451,"wins":0,"trials":24,"hits":0,"nodes":177,"peak":189,"physical":576},"periodic":{"n":12,"net":-451,"vsPeriodic":0,"wins":3,"trials":46,"hits":0,"nodes":202,"peak":215,"physical":5772},"cache1":{"n":12,"net":-787,"vsPeriodic":-336,"wins":3,"trials":46,"hits":0,"nodes":239,"peak":255,"physical":5772},"cache2":{"n":12,"net":-820,"vsPeriodic":-369,"wins":3,"trials":42,"hits":2,"nodes":278,"peak":297,"physical":5772},"direct":{"n":12,"net":3143,"vsPeriodic":3594,"wins":5,"trials":0,"hits":0,"nodes":209,"peak":217,"physical":12504}},"grid":{"initial":{"n":108,"net":0,"vsPeriodic":65395,"wins":0,"trials":216,"hits":0,"nodes":177,"peak":189,"physical":9216},"periodic":{"n":108,"net":-65395,"vsPeriodic":0,"wins":16,"trials":936,"hits":0,"nodes":202,"peak":215,"physical":63996},"cache1":{"n":108,"net":-73164,"vsPeriodic":-7769,"wins":16,"trials":936,"hits":0,"nodes":239,"peak":255,"physical":63996},"cache2":{"n":108,"net":-65371,"vsPeriodic":24,"wins":15,"trials":708,"hits":114,"nodes":278,"peak":297,"physical":63996},"direct":{"n":108,"net":-20653,"vsPeriodic":44742,"wins":30,"trials":0,"hits":0,"nodes":209,"peak":217,"physical":63996}},"recurrent":{"initial":{"n":54,"net":0,"vsPeriodic":31994,"wins":0,"trials":108,"hits":0,"nodes":177,"peak":189,"physical":5940},"periodic":{"n":54,"net":-31994,"vsPeriodic":0,"wins":8,"trials":444,"hits":0,"nodes":202,"peak":215,"physical":32420},"cache1":{"n":54,"net":-35768,"vsPeriodic":-3774,"wins":8,"trials":444,"hits":0,"nodes":239,"peak":255,"physical":32420},"cache2":{"n":54,"net":-23751,"vsPeriodic":8243,"wins":8,"trials":216,"hits":114,"nodes":278,"peak":297,"physical":32420},"direct":{"n":54,"net":-11459,"vsPeriodic":20535,"wins":11,"trials":0,"hits":0,"nodes":209,"peak":217,"physical":32420}},"novel":{"initial":{"n":54,"net":0,"vsPeriodic":33401,"wins":0,"trials":108,"hits":0,"nodes":177,"peak":189,"physical":3276},"periodic":{"n":54,"net":-33401,"vsPeriodic":0,"wins":8,"trials":492,"hits":0,"nodes":202,"peak":215,"physical":31576},"cache1":{"n":54,"net":-37396,"vsPeriodic":-3995,"wins":8,"trials":492,"hits":0,"nodes":239,"peak":255,"physical":31576},"cache2":{"n":54,"net":-41620,"vsPeriodic":-8219,"wins":7,"trials":492,"hits":0,"nodes":278,"peak":297,"physical":31576},"direct":{"n":54,"net":-9194,"vsPeriodic":24207,"wins":19,"trials":0,"hits":0,"nodes":209,"peak":217,"physical":31576}}});
console.log(JSON.stringify({tier:"NON_NATIVE_ENGINEERING_MODEL",summary,known,grid},null,2));
