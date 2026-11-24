#!/usr/bin/env node
// Retrospective non-native model. Known C29 cases, no new native evidence.
import fs from "node:fs";
import assert from "node:assert/strict";
function build({snapshot=true,repeat=true,freshness='none'}={}){
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
 if(repeat){label("gate");C(1,24);S(0,0,1);B(0,"dispatch","eval");}
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
const rows=cases.map(s=>{const original=run(s,{snapshot:true,repeat:true});const once=run(s,{snapshot:true,repeat:false});const variants=Object.fromEntries(["guard","retry","selective"].flatMap(k=>[[k,run(s,{snapshot:true,repeat:true,freshness:k})],[k+"Off",run(s,{snapshot:true,repeat:true,freshness:k},true)]]));return {id:s.id,cohort:s.cohort,original,once,...variants};});
const summary=Object.fromEntries(["guard","retry","selective"].map(k=>[k,{nodes:rows[0][k].nodes,peak:Math.max(...rows.map(r=>r[k].peak)),net:rows.filter(r=>r.cohort==="prospective").reduce((t,r)=>t+r.once.paid-r[k].paid,0),physical:rows.filter(r=>r.cohort==="prospective").reduce((t,r)=>t+r[k+"Off"].paid-r[k].paid,0),wins:rows.filter(r=>r.cohort==="prospective"&&r.once.paid>r[k].paid).length}]));
assert.deepEqual(summary,{"guard":{"nodes":180,"peak":192,"net":878,"physical":2644,"wins":3},"retry":{"nodes":187,"peak":200,"net":351,"physical":2718,"wins":3},"selective":{"nodes":193,"peak":205,"net":770,"physical":2631,"wins":3}});
console.log(JSON.stringify({tier:"RETROSPECTIVE_NON_NATIVE_MODEL",summary,rows},null,2));
