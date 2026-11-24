#!/usr/bin/env node
// C30-R4 non-native one-shot admission using charged actual production observations.
import fs from "node:fs";
import assert from "node:assert/strict";
function build(kind="generated"){
 const high=kind==="high"||kind==="selective",selective=kind==="selective",generic=kind==="generic"||high;const g=[],l={};const label=k=>{if(k in l)throw Error(k);l[k]=g.length};
 const op=(kind,args)=>g.push({kind,...args});
 const C=(dst,value,next)=>op("Const",{dst,value,next}),R=(dst,address,next)=>op("Read",{dst,address,next}),W=(address,value,next)=>op("Write",{address,value,next}),S=(dst,lhs,rhs,next)=>op("Sub",{dst,lhs,rhs,next}),A=(dst,lhs,rhs,next)=>op("Add",{dst,lhs,rhs,next}),B=(test,nonzero,zero)=>op("Branch",{test,nonzero,zero}),Q=(dst,next)=>op("SenseOwnCredits",{dst,next}),P=(node,target,next)=>op("PatchNext",{node,target,next}),K=(node,value,next)=>op("PatchConst",{node,value,next});
 const set=(key,value,next)=>{C(3,key);C(0,value);W(3,0,next)};
 label("init");
 if(kind==="plain"||kind==="guard_only"){C(3,0,"entry");}
 else{
  if(high){set(40,16);}else{C(3,50);R(0,3);C(3,40);W(3,0);}
  if(generic)C(0,16);if(kind==="fault"){C(2,1);S(0,0,2);}
  C(1,"@out");label("generate");C(2,"@parent_add");op("CloneNode",{dst:3,source:2});P(3,1);
  C(2,0);A(1,3,2);C(2,1);S(0,0,2);B(0,"generate","link");
  label("link");if(generic){C(2,16);S(0,1,2);C(1,"@base");K(1,0);}else{C(3,"@special_entry");P(3,1);}set(47,0,"measure");
  label("measure");Q(0);C(3,42);W(3,0,"entry");
 }
 label("entry");label("input_x");C(3,high?40:50);R(1,3);C(3,51);R(2,3);C(0,0);
 if(kind!=="plain"){
  label("guard");C(3,0);if(generic){B(3,selective?"mode":"base","parent");if(selective){label("mode");C(0,1);S(3,3,0);C(0,0);B(3,"threshold","base");label("threshold");C(0,0);C(3,1);label("gate_loop");S(1,1,3);B(1,"gate_count","restore_parent");label("gate_count");S(0,0,3);B(0,"gate_loop","restore_base");label("restore_parent");C(3,50);R(1,3);C(0,0,"parent");label("restore_base");C(3,50);R(1,3);C(0,0,"base");}label("base");C(3,0);A(1,1,3);C(3,"@special_entry");P(3,1);}else{S(3,1,3);B(3,"parent","special_entry");}
  label("special_entry");C(3,1,"parent_add");
 }
 label("parent");C(3,1);
 label("parent_add");A(0,0,2);S(1,1,3);B(1,"parent_add","out");
 label("out");C(3,0,kind==="plain"||kind==="guard_only"?"prod_out":"trial_out");
 if(kind!=="plain"&&kind!=="guard_only"){
  label("trial_out");C(3,44);W(3,0);Q(0);C(3,42);R(1,3);S(0,1,0);C(3,46);W(3,0);
  C(3,47);R(1,3);B(1,"trial_compare","save_parent");
  label("save_parent");C(3,45);W(3,0);C(3,44);R(0,3);C(3,43);W(3,0);
  set(47,1);if(generic)C(0,1);else{C(3,40);R(0,3);}C(1,"@guard");K(1,0,"measure");
  label("trial_compare");C(1,"@guard");C(0,0);K(1,0);
  C(3,43);R(0,3);C(3,44);R(1,3);S(0,0,1);B(0,"finish","compare_cost");
  label("compare_cost");C(3,45);R(0,3);C(3,46);R(1,3);S(2,0,1);S(1,1,0);C(3,1);B(2,"cmp","finish");
  label("cmp");S(2,2,3);B(2,"cmp_neg","install");
  label("cmp_neg");S(1,1,3);B(1,"cmp","finish");
  label("install");if(selective){C(3,45);R(0,3);C(3,46);R(1,3);S(0,0,1);C(1,16);C(3,1);label("learn_cutoff");S(0,0,3);S(1,1,3);B(0,"learn_second","store_cutoff");label("learn_second");S(0,0,3);B(0,"learn_cutoff","store_cutoff");label("store_cutoff");C(3,"@threshold");K(3,1);C(0,2);}else if(generic)C(0,1);else{C(3,40);R(0,3);}C(1,"@guard");label("activation");K(1,0,"finish");
  label("finish");if(high){C(1,"@input_x");C(0,50);K(1,0);}C(1,"@out");C(0,"@prod_out");P(1,0,"entry");
 }
 label("prod_out");C(3,11);W(3,0,"entry");
 for(let i=0;i<g.length;i++){const n=g[i];if(n.kind==="Branch"){n.nonzero=l[n.nonzero];n.zero=l[n.zero]}else n.next=n.next===undefined?i+1:l[n.next];if(n.kind==="Const"&&typeof n.value==="string")n.value=l[n.value.slice(1)];}
 return {g,l};
}
function legacyRun(s,kind="generated",off=false){
 if(![s.xy,...(s.changes??[]).map(z=>z.slice(1))].every(z=>z.every(x=>Number.isInteger(x)&&x>=1&&x<=16)))throw Error("domain");
 if(kind==="fault"&&s.xy[0]<2)throw Error("fault domain");
 const {g,l}=build(kind),initial=g.length,d=new Map([[50,s.xy[0]],[51,s.xy[1]]]),v=[0,0,0,0];let pc=0,paid=0,peak=g.length+3,prod=0,firstProd=null,cloneTrialSteps=0,cloneProductionSteps=0,firstExposure=null,serviceClones=0;const clones=[],patches=[],trials=[],output=[],prefix=[];
 while(paid<20000){
 const p=pc,n=g[pc];if(!n)throw Error("pc");if(p===l.activation)firstExposure=paid+1;if(kind==="selective"&&firstExposure===null&&prod===0)prefix.push([p,[...v],Array.from(d.entries()),g.map(x=>({...x}))]);
 paid++;pc=n.next;
 if(p>=initial){if(g[l.out].next===l.prod_out){cloneProductionSteps++;serviceClones++;}else cloneTrialSteps++;}
 switch(n.kind){
 case "Const":v[n.dst]=n.value;break;
 case "Read":v[n.dst]=d.get(v[n.address])??0;break;
 case "Write":{
 const k=v[n.address],a=v[n.value];d.set(k,a);
 if(k===46){trials.push({output:d.get(44),cost:a,x:kind==="high"||kind==="selective"?16:d.get(50),y:d.get(51)});}
 if(k===11){
 if(a!==d.get(50)*d.get(51))throw Error("bad production");
 if(["generic","high","selective"].includes(kind)){const mode=g[l.guard].value;const expected=mode===1||mode===2&&d.get(50)>g[l.threshold].value?d.get(50):0;if(serviceClones!==expected)throw Error("wrong generated path length");}serviceClones=0;prod++;output.push(a);if(firstProd===null)firstProd=paid;
 peak=Math.max(peak,g.length+1+d.size);
 if(prod===s.n)return {paid,initial,nodes:g.length,peak,firstProd,clones,patches,trials,output,active:g[l.guard]?.value??null,cutoff:g[l.threshold]?.value??null,finalSource:g,cloneTrialSteps,cloneProductionSteps,firstExposure,prefix};
 for(const [at,x,y] of s.changes??[])if(at===prod){d.set(50,x);d.set(51,y);}
 }break;}
 case "SenseOwnCredits":v[n.dst]=20000-paid;break;
 case "Add":v[n.dst]=v[n.lhs]+v[n.rhs];break;
 case "Sub":v[n.dst]=v[n.lhs]-v[n.rhs];break;
 case "Branch":pc=v[n.test]!==0?n.nonzero:n.zero;break;
 case "CloneNode":{const src=v[n.source],idx=g.length;g.push({...g[src]});v[n.dst]=idx;clones.push([paid,src,idx]);break;}
 case "PatchNext":{const target=v[n.node],before=g[target].next,after=v[n.target];if(["generic","high","selective"].includes(kind)&&target===l.special_entry&&after!==initial+d.get(g[l.input_x].value)-1)throw Error("wrong generated address");if(!g[after]||g[target].next===undefined)throw Error("edge");g[target].next=after;patches.push([paid,p,target,before,after,false]);break;}
 case "PatchConst":{const target=v[n.node],before=g[target].value,after=v[n.value],suppress=off&&p===l.activation;if(g[target].kind!=="Const")throw Error("const");if(!suppress)g[target].value=after;patches.push([paid,p,target,before,after,suppress]);break;}
 default:throw Error(n.kind);}
 peak=Math.max(peak,g.length+1+d.size);if(g.length>200||d.size>16||peak>220)throw Error("capacity");
 }throw Error("budget");
}

function buildCompiled(window=0,threshold=0){
 const g=[],l={};const label=k=>{if(k in l)throw Error(k);l[k]=g.length};
 const op=(kind,args)=>g.push({kind,...args});
 const C=(dst,value,next)=>op("Const",{dst,value,next}),R=(dst,address,next)=>op("Read",{dst,address,next}),W=(address,value,next)=>op("Write",{address,value,next}),S=(dst,lhs,rhs,next)=>op("Sub",{dst,lhs,rhs,next}),A=(dst,lhs,rhs,next)=>op("Add",{dst,lhs,rhs,next}),B=(test,nonzero,zero)=>op("Branch",{test,nonzero,zero}),Q=(dst,next)=>op("SenseOwnCredits",{dst,next}),P=(node,target,next)=>op("PatchNext",{node,target,next}),K=(node,value,next)=>op("PatchConst",{node,value,next});
 const set=(key,value,next)=>{C(3,key);C(0,value);W(3,0,next)};
 label("init");if(window){set(48,window);set(49,0,"entry");}label("start_generation");set(40,16);C(0,16);C(1,"@out");
 label("generate");C(2,"@parent_add");op("CloneNode",{dst:3,source:2});P(3,1);C(2,0);A(1,3,2);C(2,1);S(0,0,2);B(0,"generate","link");
 label("link");C(2,16);S(0,1,2);C(1,"@base");K(1,0);set(47,0,"measure");
 label("measure");Q(0);C(3,42);W(3,0,"entry");
 label("entry");label("input_x");C(3,window?50:40);
 label("dispatch");R(1,3,window?"observe":"parent");
 if(window){label("observe");C(3,49);R(0,3);A(0,0,1);W(3,0,"parent");}
 label("parent");C(3,51);R(2,3);C(0,0);C(3,1);
 label("parent_add");A(0,0,2);S(1,1,3);B(1,"parent_add","out");
 label("base");C(3,0);A(1,1,3);C(3,"@special_entry");P(3,1);C(3,51);R(2,3);
 label("special_entry");C(0,0,"parent_add");
 label("out");C(3,0,window?"prod_out":"trial_out");
 label("trial_out");C(3,44);W(3,0);Q(0);C(3,42);R(1,3);S(0,1,0);C(3,46);W(3,0);
 C(3,47);R(1,3);B(1,"trial_compare","save_parent");
 label("save_parent");C(3,45);W(3,0);C(3,44);R(0,3);C(3,43);W(3,0);set(47,1);
 C(1,"@dispatch");C(0,"@base");P(1,0,"measure");
 label("trial_compare");C(1,"@dispatch");C(0,"@parent");P(1,0);
 C(3,43);R(0,3);C(3,44);R(1,3);S(0,0,1);B(0,"finish","compare_cost");
 label("compare_cost");C(3,45);R(0,3);C(3,46);R(1,3);S(2,0,1);S(1,1,0);C(3,1);B(2,"cmp","finish");
 label("cmp");S(2,2,3);B(2,"cmp_neg","install");label("cmp_neg");S(1,1,3);B(1,"cmp","finish");
 label("install");C(1,"@flag");C(0,1);label("activation");K(1,0,"finish");
 label("finish");C(1,"@input_x");C(0,50);K(1,0);C(1,"@out");C(0,"@prod_out");P(1,0);
 label("flag");C(0,0);B(0,"compile","entry");
 label("compile");C(1,"@dispatch");C(0,"@base");P(1,0,"entry");
 label("prod_out");C(3,11);label("prod_write");W(3,0,window?"gate":"entry");
 if(window){
  label("gate");C(3,48);R(0,3);C(1,1);S(0,0,1);W(3,0);B(0,"entry","retire");
  label("retire");C(1,"@dispatch");C(0,"@parent");P(1,0);C(1,"@prod_write");C(0,"@entry");P(1,0);
  if(threshold===0){C(0,0,"admit");}else{
   C(3,49);R(0,3);C(1,threshold);C(3,1);
   label("admission_compare");S(0,0,3);B(0,"admission_count","entry");
   label("admission_count");S(1,1,3);B(1,"admission_compare","admit");
  }
  label("admit");C(1,"@input_x");C(0,40);K(1,0);C(1,"@out");C(0,"@trial_out");P(1,0,"start_generation");
 }
 for(let i=0;i<g.length;i++){const n=g[i];if(n.kind==="Branch"){n.nonzero=l[n.nonzero];n.zero=l[n.zero]}else n.next=n.next===undefined?i+1:l[n.next];if(n.kind==="Const"&&typeof n.value==="string")n.value=l[n.value.slice(1)];}
 return {g,l};
}
function runCompiled(s,off=false,window=0,threshold=0){
 if(![s.xy,...(s.changes??[]).map(z=>z.slice(1))].every(z=>z.every(x=>Number.isInteger(x)&&x>=1&&x<=16)))throw Error("domain");
 const {g,l}=buildCompiled(window,threshold),initial=g.length,d=new Map([[50,s.xy[0]],[51,s.xy[1]]]),v=[0,0,0,0];let pc=0,paid=0,peak=g.length+3,prod=0,cloneTrialSteps=0,cloneProductionSteps=0,serviceClones=0,firstExposure=null,firstProd=null;const clones=[],patches=[],trials=[],output=[],prefix=[];
 while(paid<20000){
 const p=pc,n=g[p];if(!n)throw Error("pc");if(p===l.activation)firstExposure=paid+1;
 if(firstExposure===null)prefix.push([p,[...v],Array.from(d.entries())]);
 paid++;pc=n.next;
 if(p>=initial){if(g[l.out].next===l.prod_out){cloneProductionSteps++;serviceClones++;}else cloneTrialSteps++;}
 switch(n.kind){
 case "Const":v[n.dst]=n.value;break;
 case "Read":v[n.dst]=d.get(v[n.address])??0;break;
 case "Write":{
 const k=v[n.address],a=v[n.value];d.set(k,a);
 if(k===46)trials.push({output:d.get(44),cost:a,x:d.get(40),y:d.get(51)});
 if(k===11){
 if(a!==d.get(50)*d.get(51))throw Error("bad production");
 const active=g[l.dispatch].next===l.base;
 if(serviceClones!==(active?d.get(50):0))throw Error("clone count");
 serviceClones=0;prod++;output.push(a);if(firstProd===null)firstProd=paid;
 peak=Math.max(peak,g.length+1+d.size);
 if(prod===s.n)return {admitted:clones.length>0,observed:d.get(49)??null,paid,initial,nodes:g.length,peak,firstProd,clones,patches,trials,output,active,flag:g[l.flag].value,cloneTrialSteps,cloneProductionSteps,firstExposure,prefix};
 for(const [at,x,y] of s.changes??[])if(at===prod){d.set(50,x);d.set(51,y);}
 }break;}
 case "SenseOwnCredits":v[n.dst]=20000-paid;break;
 case "Add":v[n.dst]=v[n.lhs]+v[n.rhs];break;
 case "Sub":v[n.dst]=v[n.lhs]-v[n.rhs];break;
 case "Branch":pc=v[n.test]!==0?n.nonzero:n.zero;break;
 case "CloneNode":{const src=v[n.source],idx=g.length;if(src!==l.parent_add)throw Error("clone source");g.push({...g[src]});v[n.dst]=idx;clones.push([paid,src,idx]);break;}
 case "PatchNext":{const target=v[n.node],before=g[target].next,after=v[n.target];if(!g[after]||g[target].next===undefined)throw Error("edge");
 if(target===l.special_entry&&after!==initial+d.get(g[l.input_x].value)-1)throw Error("address");
 if(target===l.dispatch&&![l.base,l.parent,l.observe].includes(after))throw Error("dispatch target");
 g[target].next=after;patches.push([paid,p,target,before,after,false]);break;}
 case "PatchConst":{const target=v[n.node],before=g[target].value,after=v[n.value],suppress=off&&p===l.activation;if(g[target].kind!=="Const")throw Error("const");if(!suppress)g[target].value=after;patches.push([paid,p,target,before,after,suppress]);break;}
 default:throw Error(n.kind);}
 peak=Math.max(peak,g.length+1+d.size);if(g.length>200||d.size>16||peak>220)throw Error("capacity");
 }throw Error("budget");
}
const cases=JSON.parse(fs.readFileSync(new URL("../studies/CL1B-C30-R3-COMPILED-ENTRY-frozen-51-worlds-v1.json",import.meta.url),"utf8")).cases;

const policies=[["r3",0,0],["delay4",4,0],["gate4",4,24],["gate8",8,48],["gate16",16,96]];
function test(s){
 const plain=legacyRun(s,"plain");const arms={};
 for(const [name,w,t] of policies){
  const a=runCompiled(s,false,w,t),b=runCompiled(s,true,w,t);
  if(a.output.join()!==plain.output.join()||b.output.join()!==plain.output.join())throw Error("outputs");
  if(JSON.stringify(a.prefix)!==JSON.stringify(b.prefix)||a.firstExposure!==b.firstExposure)throw Error("prefix");
  if(a.admitted!==b.admitted||a.active!==a.admitted||b.active||b.cloneProductionSteps!==0)throw Error("activation");
  if(a.admitted&&(a.trials.length!==2||a.trials[0].output!==a.trials[1].output||a.trials[0].cost!==60||a.trials[1].cost!==31||a.clones.length!==16))throw Error("trial");
  if(!a.admitted&&(a.clones.length||a.trials.length))throw Error("nonadmit");
  if(w&&s.n>w){let x=s.xy[0],sum=0;for(let i=0;i<w;i++){sum+=x;for(const [at,nx] of s.changes??[])if(at===i+1)x=nx;}if(sum!==a.observed||a.admitted!==(sum>t))throw Error("admission");}
  arms[name]={paid:a.paid,off:b.paid,admitted:a.admitted,observed:a.observed,nodes:a.nodes,peak:a.peak,exposure:a.firstExposure,clones:a.cloneProductionSteps};
 }
 return {id:s.id,group:s.group??s.cohort,plain:plain.paid,arms};
}
const known=cases.map(test);
const family=[];
for(let x of [1,4,8,12,16])for(const n of [6,24,72,144]){
 family.push({id:"stable_"+x+"_"+n,group:"stable",xy:[x,5],n,changes:[[3,x,9]]});
 family.push({id:"rise_"+x+"_"+n,group:"rise",xy:[x,5],n,changes:[[Math.min(n-1,12),16,9]]});
 family.push({id:"fall_"+x+"_"+n,group:"fall",xy:[x,5],n,changes:[[Math.min(n-1,12),1,9]]});
 family.push({id:"mixed_"+x+"_"+n,group:"mixed",xy:[x,5],n,changes:Array.from({length:n-1},(_,i)=>[i+1,1+(x+5*i)%16,1+(i*7)%16])});
}
const grid=family.map(test);
function agg(rows){return Object.fromEntries(policies.map(([k])=>[k,{n:rows.length,net:rows.reduce((s,r)=>s+r.plain-r.arms[k].paid,0),physical:rows.reduce((s,r)=>s+r.arms[k].off-r.arms[k].paid,0),wins:rows.filter(r=>r.arms[k].paid<r.plain).length,admitted:rows.filter(r=>r.arms[k].admitted).length,maxNodes:Math.max(...rows.map(r=>r.arms[k].nodes)),peak:Math.max(...rows.map(r=>r.arms[k].peak))}]))}
const summary={known:agg(known.filter(r=>r.group==="prospective")),grid:agg(grid),groups:Object.fromEntries(["stable","rise","fall","mixed"].map(k=>[k,agg(grid.filter(r=>r.group===k))]))};

assert.deepEqual(summary,{"known":{"r3":{"n":12,"net":7332,"physical":12156,"wins":8,"admitted":12,"maxNodes":116,"peak":127},"delay4":{"n":12,"net":6130,"physical":11662,"wins":7,"admitted":12,"maxNodes":145,"peak":158},"gate4":{"n":12,"net":3426,"physical":7324,"wins":4,"admitted":6,"maxNodes":152,"peak":165},"gate8":{"n":12,"net":3200,"physical":8354,"wins":5,"admitted":6,"maxNodes":152,"peak":165},"gate16":{"n":12,"net":-797,"physical":7504,"wins":3,"admitted":7,"maxNodes":152,"peak":165}},"grid":{"r3":{"n":80,"net":35380,"physical":67540,"wins":33,"admitted":80,"maxNodes":116,"peak":127},"delay4":{"n":80,"net":26420,"physical":63300,"wins":30,"admitted":80,"maxNodes":145,"peak":158},"gate4":{"n":80,"net":15668,"physical":47584,"wins":22,"admitted":52,"maxNodes":152,"peak":165},"gate8":{"n":80,"net":13724,"physical":47250,"wins":22,"admitted":42,"maxNodes":152,"peak":165},"gate16":{"n":80,"net":-2926,"physical":47097,"wins":18,"admitted":45,"maxNodes":152,"peak":165}},"groups":{"stable":{"r3":{"n":20,"net":8382,"physical":16422,"wins":9,"admitted":20,"maxNodes":116,"peak":127},"delay4":{"n":20,"net":6130,"physical":15350,"wins":8,"admitted":20,"maxNodes":145,"peak":158},"gate4":{"n":20,"net":6990,"physical":14454,"wins":7,"admitted":12,"maxNodes":152,"peak":165},"gate8":{"n":20,"net":5984,"physical":13581,"wins":6,"admitted":9,"maxNodes":152,"peak":165},"gate16":{"n":20,"net":1064,"physical":12069,"wins":5,"admitted":9,"maxNodes":152,"peak":165}},"rise":{"r3":{"n":20,"net":24372,"physical":32412,"wins":14,"admitted":20,"maxNodes":116,"peak":127},"delay4":{"n":20,"net":22120,"physical":31340,"wins":12,"admitted":20,"maxNodes":145,"peak":158},"gate4":{"n":20,"net":11910,"physical":19374,"wins":7,"admitted":12,"maxNodes":152,"peak":165},"gate8":{"n":20,"net":10880,"physical":18477,"wins":6,"admitted":9,"maxNodes":152,"peak":165},"gate16":{"n":20,"net":8894,"physical":22236,"wins":8,"admitted":12,"maxNodes":152,"peak":165}},"fall":{"r3":{"n":20,"net":-6378,"physical":1662,"wins":0,"admitted":20,"maxNodes":116,"peak":127},"delay4":{"n":20,"net":-8630,"physical":590,"wins":0,"admitted":20,"maxNodes":145,"peak":158},"gate4":{"n":20,"net":-6540,"physical":924,"wins":0,"admitted":12,"maxNodes":152,"peak":165},"gate8":{"n":20,"net":-7480,"physical":117,"wins":0,"admitted":9,"maxNodes":152,"peak":165},"gate16":{"n":20,"net":-11464,"physical":-603,"wins":0,"admitted":9,"maxNodes":152,"peak":165}},"mixed":{"r3":{"n":20,"net":9004,"physical":17044,"wins":10,"admitted":20,"maxNodes":116,"peak":127},"delay4":{"n":20,"net":6800,"physical":16020,"wins":10,"admitted":20,"maxNodes":145,"peak":158},"gate4":{"n":20,"net":3308,"physical":12832,"wins":8,"admitted":16,"maxNodes":152,"peak":165},"gate8":{"n":20,"net":4340,"physical":15075,"wins":10,"admitted":15,"maxNodes":152,"peak":165},"gate16":{"n":20,"net":-1420,"physical":13395,"wins":5,"admitted":15,"maxNodes":152,"peak":165}}}});
console.log(JSON.stringify({tier:"NON_NATIVE_GENERATION_ADMISSION_MODEL",summary,known,grid},null,2));
