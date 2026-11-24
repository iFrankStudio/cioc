#!/usr/bin/env node
// C29 R2 non-native model; selected engineering family, no blind validation.
import fs from "node:fs";
import assert from "node:assert/strict";
function build({snapshot=true,repeat=true,freshness='none',admission='none',period=24}={}){
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
 for(let i=0;i<g.length;i++){const n=g[i];if(n.kind==="Branch"){n.nonzero=labels[n.nonzero];n.zero=labels[n.zero];}else n.next=n.next===undefined?i+1:labels[n.next];if(n.kind==="Const"&&typeof n.value==="string")n.value=labels[n.value.slice(1)];}
 return {g,labels};
}
function run(s,opts,off=false){
 const {g,labels:l}=build(opts),v=[0,0,0,0],d=new Map([[50,s.xy[0]],[51,s.xy[1]]]);let pc=0,paid=0,stage=0,prod=0,trial=0,paired=0,invalid=0,peak=g.length+3;
 const decisions=[],edits=[],probes=[],measurements=[];let changed=false;
 while(paid<20000){
  const n=g[pc],old=pc;if(!n)throw Error("invalid pc");paid++;pc=n.next;
  if(old===l.eval)probes.push(stage);
  if(old===l.compare){paired++;if(d.get(43)!==d.get(44))invalid++;measurements.push({stage,a:d.get(45),b:d.get(46),outputs:[d.get(43),d.get(44)],snapshot:[d.get(40)??null,d.get(41)??null],live:[d.get(50),d.get(51)]});}
  if(old===l.choose_a||old===l.choose_b)decisions.push([stage,old===l.choose_a?"a":"b"]);
  switch(n.kind){
   case "Const":v[n.dst]=n.value;break;
   case "Read":v[n.dst]=d.get(v[n.address])??0;break;
   case "Write":{
    const key=v[n.address],value=v[n.value];d.set(key,value);
    if(key===43||key===44){trial++;if(key===43&&s.midTrial&&!changed){d.set(50,s.midTrial[0]);d.set(51,s.midTrial[1]);changed=true;}}
    if(key===11){prod++;if(value!==d.get(50)*d.get(51))throw Error("wrong product");}
    if(key===60){stage=value;if(prod!==stage)throw Error("count");peak=Math.max(peak,g.length+1+d.size);if(stage===s.n)return {paid,prod,trial,paired,invalid,peak,nodes:g.length,probes,measurements,decisions,edits};for(const [at,x,y] of s.changes??[])if(at===stage){d.set(50,x);d.set(51,y);}}
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

const variants={initial:{repeat:false,freshness:"guard"},fixed:{freshness:"guard"},admit:{freshness:"guard",admission:"once"},periodic:{freshness:"guard",admission:"periodic"},periodic48:{freshness:"guard",admission:"periodic",period:48}};
function test(s){return {id:s.id,cohort:s.cohort,group:s.group,results:Object.fromEntries(Object.entries(variants).map(([k,o])=>[k,{on:run(s,o),off:run(s,o,true)}]))}}
const rows=cases.map(test);
const family=[];
for(const [x,y] of [[2,3],[2,8],[2,16],[16,2],[7,8],[8,7],[9,9]])for(const n of [48,96,144]){
 family.push({id:"stable_"+x+"_"+y+"_"+n,group:"stable",xy:[x,y],n,changes:[]});
 if(x!==y)for(const t of [5,23,37,73])if(t<n){
 family.push({id:"flip_"+x+"_"+y+"_"+n+"_"+t,group:"flip",xy:[x,y],n,changes:[[t,y,x]]});
 for(const w of [4,18,36])if(t+w<n)family.push({id:"transient_"+x+"_"+y+"_"+n+"_"+t+"_"+w,group:"transient",xy:[x,y],n,changes:[[t,y,x],[t+w,x,y]]});
 }
}
const grid=family.map(test);
function aggregate(rows){return Object.fromEntries(Object.keys(variants).map(k=>{const nets=rows.map(r=>r.results.initial.on.paid-r.results[k].on.paid);return [k,{n:rows.length,net:nets.reduce((a,b)=>a+b,0),wins:nets.filter(x=>x>0).length,losses:nets.filter(x=>x<0).length,worst:Math.min(...nets),trials:rows.reduce((a,r)=>a+r.results[k].on.trial,0),nodes:rows[0].results[k].on.nodes,peak:Math.max(...rows.map(r=>r.results[k].on.peak)),physical:rows.reduce((a,r)=>a+r.results[k].off.paid-r.results[k].on.paid,0)}]}))}
const summary={known:aggregate(rows.filter(r=>r.cohort==="prospective")),grid:aggregate(grid),groups:Object.fromEntries(["stable","flip","transient"].map(k=>[k,aggregate(grid.filter(r=>r.group===k))]))};
assert.equal(grid.length,261);
assert.deepEqual(summary,{"known":{"initial":{"n":12,"net":0,"wins":0,"losses":0,"worst":0,"trials":24,"nodes":177,"peak":189,"physical":576},"fixed":{"n":12,"net":-2354,"wins":1,"losses":11,"worst":-1422,"trials":48,"nodes":180,"peak":192,"physical":2644},"admit":{"n":12,"net":-1803,"wins":1,"losses":11,"worst":-1428,"trials":40,"nodes":192,"peak":204,"physical":2642},"periodic":{"n":12,"net":-451,"wins":3,"losses":9,"worst":-1108,"trials":46,"nodes":202,"peak":215,"physical":5772},"periodic48":{"n":12,"net":-1991,"wins":2,"losses":10,"worst":-1297,"trials":40,"nodes":202,"peak":215,"physical":3518}},"grid":{"initial":{"n":261,"net":0,"wins":0,"losses":0,"worst":0,"trials":522,"nodes":177,"peak":189,"physical":86130},"fixed":{"n":261,"net":-121516,"wins":22,"losses":239,"worst":-5402,"trials":1044,"nodes":180,"peak":192,"physical":81060},"admit":{"n":261,"net":-102220,"wins":22,"losses":239,"worst":-5408,"trials":726,"nodes":192,"peak":204,"physical":80956},"periodic":{"n":261,"net":-79424,"wins":33,"losses":228,"worst":-1777,"trials":966,"nodes":202,"peak":215,"physical":159194},"periodic48":{"n":261,"net":-89566,"wins":21,"losses":240,"worst":-2404,"trials":726,"nodes":202,"peak":215,"physical":124012}},"groups":{"stable":{"initial":{"n":21,"net":0,"wins":0,"losses":0,"worst":0,"trials":42,"nodes":177,"peak":189,"physical":12960},"fixed":{"n":21,"net":-8826,"wins":0,"losses":21,"worst":-612,"trials":84,"nodes":180,"peak":192,"physical":12972},"admit":{"n":21,"net":-6237,"wins":0,"losses":21,"worst":-441,"trials":42,"nodes":192,"peak":204,"physical":12960},"periodic":{"n":21,"net":-9051,"wins":0,"losses":21,"worst":-653,"trials":42,"nodes":202,"peak":215,"physical":12960},"periodic48":{"n":21,"net":-8421,"wins":0,"losses":21,"worst":-608,"trials":42,"nodes":202,"peak":215,"physical":12960}},"flip":{"initial":{"n":66,"net":0,"wins":0,"losses":0,"worst":0,"trials":132,"nodes":177,"peak":189,"physical":-18990},"fixed":{"n":66,"net":19155,"wins":18,"losses":48,"worst":-612,"trials":264,"nodes":180,"peak":192,"physical":28958},"admit":{"n":66,"net":22704,"wins":18,"losses":48,"worst":-441,"trials":204,"nodes":192,"peak":204,"physical":28938},"periodic":{"n":66,"net":31989,"wins":27,"losses":39,"worst":-633,"trials":240,"nodes":202,"peak":215,"physical":50238},"periodic48":{"n":66,"net":19741,"wins":21,"losses":45,"worst":-588,"trials":216,"nodes":202,"peak":215,"physical":34262}},"transient":{"initial":{"n":174,"net":0,"wins":0,"losses":0,"worst":0,"trials":348,"nodes":177,"peak":189,"physical":92160},"fixed":{"n":174,"net":-131845,"wins":4,"losses":170,"worst":-5402,"trials":696,"nodes":180,"peak":192,"physical":39130},"admit":{"n":174,"net":-118687,"wins":4,"losses":170,"worst":-5408,"trials":480,"nodes":192,"peak":204,"physical":39058},"periodic":{"n":174,"net":-102362,"wins":6,"losses":168,"worst":-1777,"trials":684,"nodes":202,"peak":215,"physical":95996},"periodic48":{"n":174,"net":-100886,"wins":0,"losses":174,"worst":-2404,"trials":468,"nodes":202,"peak":215,"physical":76790}}}});
console.log(JSON.stringify({tier:"NON_NATIVE_ENGINEERING_MODEL",summary,rows,grid},null,2));
