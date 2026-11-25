import fs from 'node:fs';
function buildRec(policy="evolve"){
 const g=[],l={};const label=k=>{if(k in l)throw Error(k);l[k]=g.length};const op=(kind,args)=>g.push({kind,...args});
 const C=(dst,value,next)=>op("Const",{dst,value,next}),R=(dst,address,next)=>op("Read",{dst,address,next}),W=(address,value,next)=>op("Write",{address,value,next}),S=(dst,lhs,rhs,next)=>op("Sub",{dst,lhs,rhs,next}),A=(dst,lhs,rhs,next)=>op("Add",{dst,lhs,rhs,next}),B=(test,nonzero,zero)=>op("Branch",{test,nonzero,zero}),Q=(dst,next)=>op("SenseOwnCredits",{dst,next}),P=(node,target,next)=>op("PatchNext",{node,target,next}),K=(node,value,next)=>op("PatchConst",{node,value,next});
 const set=(key,value,next)=>{C(3,key);C(0,value);W(3,0,next)};
 const pe=(at,to,next)=>{C(1,"@"+at);C(0,"@"+to);P(1,0,next)};
 label("init");C(0,0,policy==="plain"?"prod_init":"generate_init");
 label("generate_init");C(2,8);label("double_src");A(2,2,2);C(1,"@parent_add");op("CloneNode",{dst:3,source:1});C(2,"@out");P(3,2);
 C(2,1);S(2,3,2);C(1,"@base");K(1,2);C(2,0);A(0,3,2);C(2,7);A(1,0,2);
 label("generate_even");C(2,"@double_src");op("CloneNode",{dst:3,source:2});P(3,0);S(2,0,1);B(2,"generate_odd","generate_done");
 label("generate_odd");C(2,"@parent_add");op("CloneNode",{dst:2,source:2});P(2,3);C(2,1);A(0,0,2,"generate_even");
 label("generate_done");pe("kernel_dispatch","base","prod_init");


 label("prod_init");C(0,0,"batch");
 label("batch");pe("done","first_result");C(3,50,"prepare");
 label("second");pe("done","second_result");C(3,52,"prepare");
 label("prepare");R(1,3);C(0,1);A(3,3,0);R(2,3);C(3,43);W(3,1);C(3,44);W(3,2);C(3,41);W(3,2);C(3,58);R(0,3);C(3,42);W(3,0,"loop");
 label("loop");C(3,43);R(1,3);C(3,41);R(2,3,"kernel_dispatch");
 label("kernel_dispatch");C(0,0,"parent");
 label("parent");C(3,1);
 label("parent_add");A(0,0,2);S(1,1,3);B(1,"parent_add","out");
 label("base");C(3,0);A(1,1,3);C(3,"@special_entry");P(3,1);
 label("special_entry");C(0,0,"parent_add");
 label("out");C(3,44);R(1,3);A(0,0,1);C(3,41);W(3,0);C(3,42);R(1,3);C(2,1);S(1,1,2);W(3,1);B(1,"loop","done");
 label("done");C(3,41);R(0,3);label("return_route");C(1,0,"first_result");
 label("first_result");C(3,11);W(3,0,"decision");
 label("decision");C(0,0,policy==="evolve"?"probe":policy==="static"?"gate":"second");
 const equal=(name,yes,no)=>{label(name);C(3,50);R(0,3);C(3,52);R(1,3);S(0,0,1);B(0,no,name+"_b");label(name+"_b");C(3,51);R(0,3);C(3,53);R(1,3);S(0,0,1);B(0,no,yes);};
 label("probe");C(0,0,"observe");equal("observe","install","second");
 label("install");pe("decision","gate");C(1,"@flag");C(0,1);label("activate");K(1,0,"flag");label("flag");C(0,0);B(0,"compile","gate");label("compile");pe("gate","check","gate");
 label("gate");C(0,0,policy==="static"?"check":"second");
 equal("check","reuse","second");
 label("reuse");C(3,11);R(0,3,"second_result");
 label("second_result");C(3,12);W(3,0,"batch");
 for(let i=0;i<g.length;i++){const n=g[i];if(n.kind==="Branch"){n.nonzero=l[n.nonzero];n.zero=l[n.zero]}else n.next=n.next===undefined?i+1:l[n.next];if(n.kind==="Const"&&typeof n.value==="string")n.value=l[n.value.slice(1)];}
 // call return edges are on the last instruction of done, not the read sequence.
 for(const n of g)if(n.kind==="Const"&&n.value===l.done&&n.dst===1)n.value=l.return_route;
 return {g,l};
}
function depth(x){let n=0;while(x){n+=x%2;if(x>1)n++;x=Math.floor(x/2);}return n;}
function recurrence(a,b,k){let z=b;for(let i=0;i<k;i++)z=a*z+b;return z;}
function runRec(s,policy="evolve",off=false){
 const {g,l}=buildRec(policy),initial=g.length,d=new Map([...s.xy.map((v,i)=>[50+i,v]),[58,s.k]]),v=[0,0,0,0];let pc=0,paid=0,prod=0,peak=g.length+1+d.size,clones=0,kernels=0,serviceKernels=0,serviceClones=0,cloneSteps=0,firstExposure=null,reused=false,firstOutput=null,expectedEntry=null,probeSteps=0;const outputs=[],activations=[],prefix=[],services=[],patches=[];
 const inputs=()=>[50,51,52,53].map(k=>d.get(k));
 if(![s.xy,...s.changes.map(a=>a.slice(1))].every(a=>a.length===4&&a.every(x=>Number.isInteger(x)&&x>=1&&x<=16))||![1,2,4,8].includes(s.k))throw Error("domain");
 while(paid<200000){
  const p=pc,n={...g[p]};if(!n.kind)throw Error("pc");
  if(p===l.activate&&firstExposure===null)firstExposure=paid+1;
  if(firstExposure===null&&policy==="evolve")prefix.push([p,[...v],[...d],g.length,n]);
  if(p>=l.probe&&p<l.gate)probeSteps++;
  if(p===l.batch){reused=false;firstOutput=null;}
  if(p===l.reuse)reused=true;
  if(p===l.kernel_dispatch){kernels++;serviceKernels++;expectedEntry=initial+v[1]-1;const [a,b,c,e]=inputs(),factor=firstOutput===null?a:c,bias=firstOutput===null?b:e,iter=(serviceKernels-1)%s.k;if(v[1]!==factor||v[2]!==recurrence(factor,bias,iter))throw Error("kernel input");}
  if(p>=initial){serviceClones++;cloneSteps++;}
  paid++;pc=n.next;
  switch(n.kind){
   case "Const":v[n.dst]=n.value;break;
   case "Read":v[n.dst]=d.get(v[n.address])??0;break;
   case "Write":{
    const k=v[n.address],z=v[n.value];d.set(k,z);const [a,b,c,e]=inputs();
    if(k===11){if(z!==recurrence(a,b,s.k)||firstOutput!==null||serviceKernels!==s.k)throw Error("first output");firstOutput=z;}
    if(k===12){
     if(z!==recurrence(c,e,s.k)||firstOutput!==recurrence(a,b,s.k))throw Error("second output "+s.id+" "+prod);
     const active=g[l.gate].next===l.check,choice=active&&a===c&&b===e;
     if(reused!==choice||serviceKernels!==s.k*(choice?1:2))throw Error("reuse");
     if(serviceClones!==(policy==="plain"?0:s.k*(depth(a)+(choice?0:depth(c)))))throw Error("clone count");
     prod++;outputs.push([firstOutput,z]);services.push({at:prod,reused,paid,serviceKernels,serviceClones,active});serviceKernels=serviceClones=0;
     if(prod===s.n)return {paid,outputs,activations,services,clones,kernels,cloneSteps,nodes:g.length,peak,firstExposure,prefix,probeSteps,patches};
     for(const [at,...q] of s.changes)if(at===prod)for(let i=0;i<4;i++)d.set(50+i,q[i]);
    }break;}
   case "Add":v[n.dst]=v[n.lhs]+v[n.rhs];break;
   case "Sub":v[n.dst]=v[n.lhs]-v[n.rhs];break;
   case "Branch":pc=v[n.test]!==0?n.nonzero:n.zero;break;
   case "CloneNode":{const from=v[n.source],idx=g.length,x=idx-initial+1;if(from!==(x%2?l.parent_add:l.double_src))throw Error("template");g.push({...g[from]});v[n.dst]=idx;clones++;break;}
   case "PatchConst":{const at=v[n.node];if(at===l.flag){const [a,b,c,e]=inputs();if(p!==l.activate||a!==c||b!==e||v[n.value]!==1||activations.length)throw Error("activation");activations.push({at:prod,paid,suppressed:off});if(!off)g[at].value=1;}else{if(at!==l.base||g[at].kind!=="Const"||v[n.value]!==initial-1)throw Error("base");g[at].value=v[n.value];}break;}
   case "PatchNext":{
    const at=v[n.node],to=v[n.target];if(!g[to]||g[at].next===undefined)throw Error("edge");
    if(at>=initial){const x=at-initial+1;if(to!==(x===1?l.out:x%2?at-1:initial+x/2-1))throw Error("generated edge");}
    else if(at===l.special_entry){if(to!==expectedEntry)throw Error("entry");}
    else if(at===l.kernel_dispatch){if(to!==l.base)throw Error("kernel");}
    else if(at===l.return_route){if(to!==(firstOutput===null?l.first_result:l.second_result))throw Error("return");}
    else if(at===l.decision){if(to!==l.gate)throw Error("probe retire");}
    else if(at===l.gate){const [a,b,c,e]=inputs();if(a!==c||b!==e||p!==l.compile+2||to!==l.check||activations.length!==1||off)throw Error("compile");}
    else throw Error("target "+at);
    const suppress=false;patches.push([paid,p,at,g[at].next,to,suppress]);if(!suppress)g[at].next=to;break;}
   default:throw Error(n.kind);
  }
  if(!v.every(Number.isSafeInteger))throw Error("integer");
  peak=Math.max(peak,g.length+1+d.size);if(g.length>200||d.size>16||peak>220)throw Error("capacity "+g.length+" "+peak);
 }
 throw Error("budget");
}

const m=JSON.parse(fs.readFileSync(new URL('../studies/CL1B-C33-R1-NATIVE-REUSE-frozen-82-worlds-v1.json',import.meta.url)));for(const c of m.cases.filter(c=>c.cohort==='calibration'))for(const a of m.arms){const paid=runRec(c,a.source,a.off).paid;if(paid!==m.calibration_expected[c.id][a.id])throw Error('calibration');}console.log('calibration verified');
