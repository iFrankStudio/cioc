// CL-1B B42 independent interpreter of the exact Rust-native founder CIL.
// Mechanism model; NOT native verification. All candidate code is parsed from
// the original Rust source and CloneNode is interpreted with a paid storage unit.
import {readFileSync} from 'node:fs';
import {fileURLToPath} from 'node:url';

const src=readFileSync(new URL('../../cl0/crates/world/tests/cl1b_b42_self_assembled_probe.rs',import.meta.url),'utf8');
const full=src.match(/let mut code = vec!\[([\s\S]*?)\n    \];/)?.[1];
const cheap=src.match(/if arm == Arm::StaticCorrect \{\s*return vec!\[([\s\S]*?)\n        \];/)?.[1];
if(!full||!cheap)throw Error('cannot parse native Rust CIL sources');
const parse=text=>[...text.matchAll(/Op::([A-Za-z]+)\s*\{([^}]*)\}|Op::Halt\b/g)].map(m=>{
 const op={kind:m[1]??'Halt'};
 for(const v of (m[2]??'').matchAll(/(\w+)\s*:\s*(-?\d+)/g))op[v[1]]=Number(v[2]);
 return op;
});
const base=parse(full),baseline=parse(cheap);
if(base.length!==38||baseline.length!==4||base[7].kind!=='Write'||base[18].kind!=='CloneNode'||base[9].next!==10||base[13].nonzero!==14||base[13].zero!==26)
 throw Error('native source semantics changed');
if(!src.includes('Arm::ProbeMinus => code[4] = Op::Const { dst: 1, value: -1, next: 5 }')||
 !src.includes('Arm::NoProbe => code[5] = Op::Add { dst: 0, lhs: 0, rhs: 3, next: 6 }')||
 !src.includes('Arm::IgnoreFeedback => code[12] = Op::Sub { dst: 1, lhs: 0, rhs: 0, next: 13 }'))
 throw Error('native counterfactual source drifted');
const arms=['ProbePlus','ProbeMinus','NoProbe','IgnoreFeedback','PatchDisabled','StaticCorrect'];
function run(arm,train,holdout){
 const code=(arm==='StaticCorrect'?baseline:base).map(o=>({...o}));
 if(arm==='ProbeMinus')code[4]={kind:'Const',dst:1,value:-1,next:5};
 if(arm==='NoProbe')code[5]={kind:'Add',dst:0,lhs:0,rhs:3,next:6};
 if(arm==='IgnoreFeedback')code[12]={kind:'Sub',dst:1,lhs:0,rhs:0,next:13};
 const cells=new Map([[7,train]]),r=[0,0,0,0],events=[];
 let credits=128,pc=0,stopped=false,restored=false,probe=null;
 const diagnostic=[0,0],workTrain=[0,0],workHoldout=[0,0],patches=[],suppressed=[];
 const reg=i=>{if(!Number.isInteger(i)||i<0||i>3)throw Error('invalid register '+i);return r[i];};
 for(const [phase,input] of [train,holdout].entries()){
  if(phase===1)cells.set(7,input);
  for(let k=0;k<64&&credits>0&&!stopped;k++){
   const op=code[pc],at=pc;
   if(!op)throw Error('nonexisting executable node '+pc);
   credits--;
   const target=code.length;
   switch(op.kind){
    case 'Const':r[op.dst]=op.value;pc=op.next;break;
    case 'Add':r[op.dst]=reg(op.lhs)+reg(op.rhs);pc=op.next;break;
    case 'Sub':r[op.dst]=reg(op.lhs)-reg(op.rhs);pc=op.next;break;
    case 'Read':r[op.dst]=cells.get(reg(op.address))??0;pc=op.next;break;
    case 'Write':{
     const addr=reg(op.address),value=reg(op.value);
     cells.set(addr,value);pc=op.next;
     if(addr===7 && at===7)probe=value;
     if(addr===7 && at===33){
      if(value!==train)throw Error('incorrect native input restoration');
      restored=true;
     }
     if(addr===11){
      const diag=phase===0&&!restored&&arm!=='StaticCorrect';
      const bucket=diag?diagnostic:phase===0?workTrain:workHoldout;
      const expected=diag?probe:input;
      bucket[Number(expected!==value)]++;
     }
     break;
    }
    case 'Branch':pc=reg(op.test)!==0?op.nonzero:op.zero;break;
    case 'CloneNode':{
     const node=reg(op.source);
     if(!code[node]||code.length>=80)throw Error('native CloneNode would trap');
     code.push({...code[node]});r[op.dst]=target;pc=op.next;break;
    }
    case 'ReadNext':{
     const node=reg(op.node);if(!code[node]||!Object.hasOwn(code[node],'next'))throw Error('native ReadNext trap');
     r[op.dst]=code[node].next;pc=op.next;break;
    }
    case 'PatchNext':{
     const node=reg(op.node),after=reg(op.target);
     if(!code[node]||!Object.hasOwn(code[node],'next')||!code[after])throw Error('native PatchNext trap');
     const before=code[node].next;
     if(arm==='PatchDisabled')suppressed.push(at);
     else {code[node].next=after;if(before!==after)patches.push({node,before,after,at});}
     pc=op.next;break;
    }
    case 'Halt':stopped=true;break;
    default:throw Error('unknown opcode '+op.kind);
   }
   events.push({phase,pc:at,credits});
  }
 }
 const edge=arm==='StaticCorrect'?3:code[9].next;
 return {arm,train,holdout,probe,
  diagnosticCorrect:diagnostic[0],diagnosticWrong:diagnostic[1],
  trainCorrect:workTrain[0],trainWrong:workTrain[1],
  holdoutCorrect:workHoldout[0],holdoutWrong:workHoldout[1],
  charged:128-credits,stopped,restored,codeNodes:code.length,
  finalEdge:edge,clonedWriterNext:code[38]?.next??null,
  cloneEvents:Number(code.length>38),
  routeEdits:patches.filter(p=>p.node===9).length,
  retentionEdits:patches.filter(p=>(p.node===10||p.node===38)&&p.before===11&&p.after===34).length,
  suppressed:suppressed.length,
  originalInputPreserved:cells.get(7)===holdout,
  patches};
}
const cases=[];
for(const train of [-1,0,1,17])for(const holdout of [-9,23])for(const arm of arms){
 const p=run(arm,train,holdout);cases.push(p);
 if(arm==='ProbePlus'&&p.finalEdge!==(train===-1?10:38))throw Error('probe+ source retained unexpected '+JSON.stringify(p));
 if(arm==='ProbeMinus'&&p.finalEdge!==(train===1?10:38))throw Error('probe- source retained unexpected '+JSON.stringify(p));
 if(arm==='NoProbe'&&p.finalEdge!==(train===0?10:38))throw Error('no-probe route differs');
 if(arm==='IgnoreFeedback'&&(p.finalEdge!==10||p.cloneEvents!==0))throw Error('ignore feedback unexpectedly constructs');
 if(arm==='PatchDisabled'&&(p.finalEdge!==10||p.routeEdits!==0))throw Error('patch disabled route modified');
 if(arm==='StaticCorrect'&&(p.holdoutWrong!==0||p.codeNodes!==4||p.charged!==128))throw Error('static comparator broken');
}
if(cases.length!==48)throw Error('wrong frozen world count');
const grouped=Object.fromEntries(arms.map(a=>{
 const xs=cases.filter(c=>c.arm===a);
 const sum=k=>xs.reduce((acc,x)=>acc+x[k],0);
 return [a,{n:xs.length,probeCorrect:sum('diagnosticCorrect'),probeWrong:sum('diagnosticWrong'),
   trainCorrect:sum('trainCorrect'),trainWrong:sum('trainWrong'),holdoutCorrect:sum('holdoutCorrect'),
   holdoutWrong:sum('holdoutWrong'),charged:sum('charged'),clones:sum('cloneEvents'),
   restored:xs.filter(c=>c.restored).length,halted:xs.filter(c=>c.stopped).length}];
}));
const output={kind:'independent Rust-source-parsed CIL only; not native CI',worlds:cases.length,grouped,cases};
if(process.argv[1]===fileURLToPath(import.meta.url))process.stdout.write(JSON.stringify(output,null,2)+'\n');
export {run,output};
