// B54 prospective, independent CIL-level interleaving model; NOT native evidence.
// Reads the actual B53 source and checks the frozen paid Read/Branch ordering.
// Run: node cl1b/analysis/b54_toctou_source_model.mjs
import fs from 'node:fs';
import assert from 'node:assert/strict';
const source=fs.readFileSync(new URL('../../cl0/crates/world/tests/cl1b_b53_native_response_correlation.rs',import.meta.url),'utf8');
const entries=[...source.matchAll(/^\s*(Op::\w+\{[^\n]+\}),\s*\/\/\s*(\d+)(?:\s|:)/gm)]
 .map(m=>({op:m[1],pc:Number(m[2])}));
const code=new Map(entries.map(x=>[x.pc,x.op]));
for(let pc=106;pc<=123;pc++)assert.ok(code.has(pc),'missing CIL node '+pc);
assert.match(code.get(117),/^Op::Branch\{test:1,nonzero:121,zero:118\}/);
assert.match(code.get(118),/^Op::Const\{dst:2,value:20,next:119\}/);
assert.match(code.get(119),/^Op::Read\{dst:3,address:2,next:120\}/);
assert.match(code.get(120),/^Op::Branch\{test:3,nonzero:30,zero:80\}/);
function execute({lateFlip=false,forged=false,oldNonce=false}){
 const serial=7,cell=new Map([[26,serial],[27,oldNonce?serial-1:serial],[28,oldNonce?serial-1:serial],[20,forged?1:0]]);
 let pc=106,regs=[0,0,0,0],paid=0,reads=[],interventions=[];
 while(paid<20){
  if(pc===119&&lateFlip){cell.set(20,1);interventions.push({afterPaid:paid,key:20,value:1,origin:'external'});}
  const op=code.get(pc);assert.ok(op,'unknown PC '+pc);paid++;
  const kind=op.match(/^Op::(\w+)/)[1], field=k=>{const m=op.match(new RegExp('(?:\\{|,)'+k+':(-?\\d+)'));assert.ok(m,op+' '+k);return Number(m[1]);};
  if(kind==='Const'){regs[field('dst')]=field('value');pc=field('next');}
  else if(kind==='Read'){const addr=regs[field('address')],v=cell.get(addr)??0;regs[field('dst')]=v;reads.push({pc,key:addr,value:v});pc=field('next');}
  else if(kind==='Sub'){regs[field('dst')]=regs[field('lhs')]-regs[field('rhs')];pc=field('next');}
  else if(kind==='Branch'){pc=regs[field('test')]!==0?field('nonzero'):field('zero');}
  else throw Error('unsupported '+kind);
  if(pc===30||pc===80||pc===121)break;
 }
 return {destination:pc,paid,reads,interventions,truthfulOutput:false};
}
const baseline=execute({});
const flipped=execute({lateFlip:true});
const forged=execute({forged:true});
const old=execute({oldNonce:true});
assert.equal(baseline.destination,80);
assert.equal(flipped.destination,30);
assert.equal(forged.destination,30);
assert.equal(old.destination,121);
assert.equal(flipped.reads.at(-1).pc,119);
assert.equal(old.reads.some(x=>x.key===20),false);
console.log(JSON.stringify({study:'B54',kind:'source-parsed deterministic CIL model, NOT native execution',source:'cl1b_b53_native_response_correlation.rs',baseline,flipped,forged,old,conclusion:'Late external bit flip after serial checks changes the modeled branch from reopen to retain; native step-boundary test remains required.'},null,2));
