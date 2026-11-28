#!/usr/bin/env node
// C36-R9: same-policy register branch baseline. Abstract interpreter, not Rust.
const p=[["Const",0,10,1],["Const",1,1,2],["Const",2,2,3],["Const",3,6,4],["TransferCompute",2,3,5],["Write",0,1,6],["Read",1,0,7],["Branch",1,6,8],["Const",0,11,9],["Read",1,0,10],["Const",0,15,11],["Write",0,1,12],["Const",0,15,13],["Read",1,0,14],["Const",0,51,15],["Read",2,0,16],["Add",1,1,2,17],["Const",0,12,18],["Write",0,1,19],["Const",0,10,20],["Const",1,2,21],["Write",0,1,22],["Read",1,0,23],["Branch",1,22,24],["Const",0,11,25],["Read",1,0,26],["Branch",1,27,0],["Const",0,12,28],["Read",1,0,29],["Const",0,13,30],["Write",0,1,31],["Const",0,14,32],["Const",1,1,33],["Write",0,1,12],["Const",0,14,35],["Const",1,0,36],["Write",0,1,12]], extra=[["Write",0,1,38],["Sub",3,1,2,39],["Branch",3,40,41],["Const",1,0,42],["Const",1,12,42],["Const",0,33,43],["ReadNext",2,0,44],["Sub",3,2,1,45],["Branch",3,46,12],["PatchNext",0,1,12]], b=[["Const",0,10,1],["Read",1,0,2],["Branch",1,3,1],["Const",2,1,4],["Sub",2,1,2,5],["Branch",2,10,6],["Const",0,50,7],["Read",1,0,8],["Const",0,11,9],["Write",0,1,26],["Const",0,51,11],["Read",1,0,12],["Const",0,50,13],["Read",2,0,14],["Add",1,1,2,15],["Const",0,12,16],["Read",2,0,17],["Sub",1,1,2,18],["Branch",1,19,22],["Const",1,0,20],["Const",0,11,21],["Write",0,1,26],["Const",2,1,23],["Const",3,100,24],["TransferCompute",2,3,25],["Const",1,1,20],["Const",0,10,27],["Const",1,0,28],["Write",0,1,1]];
const modes=["fresh","cache","adaptive","edit_off","reg_branch"];
const assert=(x,m)=>{if(!x)throw Error(m)};
function producer(mode){
 const g=p.map(x=>[...x]);
 if(mode!=="cache")g[33][3]=0;
 if(mode==="adaptive"||mode==="edit_off"){g[11]=["Read",2,0,37];g.push(...extra.map(x=>[...x]));}
 if(mode==="reg_branch"){
  // r3 holds signed new-old difference; untouched during production.
  // Probe payment clobbers r3, but probe exits recompute it.
  g[11]=["Read",2,0,37];g[33][3]=39;
  g.push(["Write",0,1,38],["Sub",3,1,2,12],["Branch",3,0,12]);
  while(g.length<47)g.push(["Const",0,0,12]);
 }
 return g;
}
function run(mode,s){
 const pg=producer(mode),bg=b.map(a=>[...a]);bg[23][2]=s.reward;
 const ps=[{pc:0,r:[0,0,0,0],c:s.initial,code:pg,paid:0},{pc:0,r:[0,0,0,0],c:15000,code:bg,paid:0}];
 const offers=s.offers,d=new Map([[10,0],[11,0],[12,0],[13,0],[14,0],[15,0],[50,offers[0][0]],[51,offers[0][1]]]);
 const peak=pg.length+bg.length+d.size+2;assert(peak<=100,"storage");
 let cursor=1,done=0,steps=0,probes=0,sales=0,invalid=0,attempts=0,edits=0,wrong=0;
 const probeAt=[],rejectAt=[],decisions=[];
 while(done<offers.length&&ps[0].c>0&&steps<30000){
  const id=ps[(cursor+1)%2].c>0?(cursor+1)%2:cursor;cursor=id;const a=ps[id],v=a.r,pc=a.pc,[op,x,y,z,n]=a.code[pc];a.c--;a.paid++;steps++;
  switch(op){case"Const":v[x]=y;a.pc=z;break;case"Read":v[x]=d.get(v[y])??0;a.pc=z;break;case"Write":d.set(v[x],v[y]);a.pc=z;break;case"Branch":a.pc=v[x]?y:z;break;case"Add":v[x]=v[y]+v[z];a.pc=n;break;case"Sub":v[x]=v[y]-v[z];a.pc=n;break;case"TransferCompute":{const to=v[x]-1,q=v[y];assert(a.c>=q,"transfer");a.c-=q;ps[to].c+=q;a.pc=z;if(id===0){probes++;probeAt.push(done);}else sales++;break;}case"ReadNext":v[x]=a.code[v[y]].at(-1);a.pc=z;break;case"PatchNext":{attempts++;const node=v[x],target=v[y],old=a.code[node].at(-1);assert(node===33&&[0,12].includes(target)&&old!==target,"patch");if(mode!=="edit_off"){a.code[node][a.code[node].length-1]=target;edits++;}a.pc=z;break;}default:throw Error(op)}
  assert(ps[0].c+ps[1].c===s.initial+15000-steps,"inventory");
  if(id===1&&pc===18&&a.pc===19){invalid++;rejectAt.push(done);}
  if(id===0&&pc===30){if(d.get(13)!==offers[done][0]+offers[done][1]||d.get(11)!==1)wrong++;assert(wrong===0,"wrong publication");}
  if(id===0&&pc===33){decisions.push(mode==="reg_branch"?(v[3]===0?12:0):a.code[33][3]);done++;if(done<offers.length){d.set(50,offers[done][0]);d.set(51,offers[done][1]);}}
 }
 return {completed:done,steps,probes,sales,invalid_candidates:invalid,patch_attempts:attempts,edits,producer_credit:ps[0].c,buyer_credit:ps[1].c,producer_paid:ps[0].paid,buyer_paid:ps[1].paid,peak,wrong,probeAt,rejectAt,decisions};
}
const scenarios=[];
const old={stable:Array.from({length:24},(_,i)=>3+(i>=12?4:0)),mixed:Array.from({length:24},(_,i)=>[3,8,2,9,4,11][Math.floor(i/4)]),fast:Array.from({length:24},(_,i)=>1+i%11),switch:Array.from({length:24},(_,i)=>i<12?1+i%11:7),reverse:Array.from({length:24},(_,i)=>i<12?7:1+i%11)};
for(const [reg,v] of Object.entries(old))scenarios.push({id:"r8_"+reg,reg,initial:220,reward:100,offers:v.map((a,i)=>[a,1+(i*7)%19])});
for(const seed of [3407,3511,3631,3761,3889,4001])for(const reg of ["stable","mixed","fast","switch","reverse"])for(const [initial,reward] of [[220,100],[500,85]]){
 let z=seed;const rnd=n=>{z=(Math.imul(z,1664525)+1013904223)>>>0;return Math.floor(z/4294967296*n)};
 let a=1,remain=0;const offers=[];
 for(let i=0;i<48;i++){
  if(i===24&&(reg==="switch"||reg==="reverse"))remain=0;
  if(!remain){const old=a;do{a=1+rnd(31);}while(a===old);
   const fast=reg==="fast"||(reg==="switch"&&i<24)||(reg==="reverse"&&i>=24);
   remain=fast?1:reg==="mixed"?1+rnd(8):8+rnd(17);
  }
  remain--;offers.push([a,1+rnd(49)]);
 }
 scenarios.push({id:[seed,reg,initial,reward].join("_"),reg,initial,reward,offers});
}
const rows=scenarios.map(s=>{
 const arms=Object.fromEntries(modes.map(m=>[m,run(m,s)])),a=arms.adaptive,b=arms.reg_branch;
 const n=Math.min(a.completed,b.completed);
 assert(JSON.stringify(a.decisions.slice(0,n))===JSON.stringify(b.decisions.slice(0,n)),"decision mismatch "+s.id);
 for(const k of ["probeAt","rejectAt"])assert(JSON.stringify(a[k].filter(i=>i<n))===JSON.stringify(b[k].filter(i=>i<n)),"observation mismatch "+s.id+" "+k);
 return {id:s.id,reg:s.reg,initial:s.initial,reward:s.reward,n:s.offers.length,arms};
});
function agg(rs){return Object.fromEntries(modes.map(m=>[m,{paid:rs.reduce((n,r)=>n+r.arms[m].steps,0),complete:rs.filter(r=>r.arms[m].completed===r.n).length,correct:rs.reduce((n,r)=>n+r.arms[m].completed,0),wrong:rs.reduce((n,r)=>n+r.arms[m].wrong,0),credit:rs.reduce((n,r)=>n+r.arms[m].producer_credit,0),probes:rs.reduce((n,r)=>n+r.arms[m].probes,0),edits:rs.reduce((n,r)=>n+r.arms[m].edits,0),peak:Math.max(...rs.map(r=>r.arms[m].peak))}]))}
const full=rows.filter(r=>r.arms.adaptive.completed===r.n&&r.arms.reg_branch.completed===r.n);
const summary={worlds:rows.length*modes.length,paired:rows.length,offersPerArm:rows.reduce((n,r)=>n+r.n,0),all:agg(rows),old:agg(rows.slice(0,5)),new:agg(rows.slice(5)),fullPair:full.length,full:agg(full),wins:{edit:full.filter(r=>r.arms.adaptive.steps<r.arms.reg_branch.steps).length,branch:full.filter(r=>r.arms.adaptive.steps>r.arms.reg_branch.steps).length,tie:full.filter(r=>r.arms.adaptive.steps===r.arms.reg_branch.steps).length},groups:Object.fromEntries(["stable","mixed","fast","switch","reverse"].map(k=>[k,agg(rows.filter(r=>r.reg===k))]))};
const report={tier:"NON_NATIVE_STRONG_BRANCH_CONTROL",summary,rows,scenarios};

console.log(JSON.stringify(report,null,2));
