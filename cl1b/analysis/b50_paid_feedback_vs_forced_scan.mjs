// CL-1B B50: source-parsed independent CIL replay of native case aggregates.
// No native artifact => no green observer; NOT a cyberlife or B3 claim.
import fs from 'node:fs';
import {isDeepStrictEqual} from 'node:util';
const tests=new URL('../../cl0/crates/world/tests/',import.meta.url);
const b42=fs.readFileSync(new URL('cl1b_b42_self_assembled_probe.rs',tests),'utf8');
const b50=fs.readFileSync(new URL('cl1b_b50_feedback_vs_forced_clone.rs',tests),'utf8');
const evidence='cl1b-ci-evidence/b50_native_cases.jsonl';
if(!fs.existsSync(evidence))throw Error('B50 native evidence missing; fail closed');
const founderText=b42.match(/let mut code = vec!\[([\s\S]*?)\n    \];/)?.[1];
const staticText=b42.match(/if arm == Arm::StaticCorrect \{\s*return vec!\[([\s\S]*?)\n        \];/)?.[1];
if(!founderText||!staticText)throw Error('B50 cannot extract actual B42 CIL code');
const parse=s=>[...s.matchAll(/Op::([A-Za-z]+)\s*\{([^}]*)\}|Op::Halt\b/g)].map(m=>{
  const o={kind:m[1]??'Halt'};
  for(const pair of (m[2]??'').matchAll(/(\w+)\s*:\s*(-?\d+)/g))
    o[pair[1]]=Number(pair[2]);
  return o;
});
const original=parse(founderText),compact=parse(staticText);
if(original.length!==38||compact.length!==4||original[12].kind!=='Sub'||
   original[16].kind!=='Branch'||original[18].kind!=='CloneNode'||
   original[29].kind!=='PatchNext'||original[37].kind!=='Halt')
  throw Error('B50 B42 founder shape changed');
for(const rule of [
  'nodes[12]=Op::Sub{dst:1,lhs:0,rhs:0,next:13}',
  'nodes[12]=Op::Const{dst:1,value:1,next:13}',
  'nodes[16]=Op::Branch{test:1,nonzero:26,zero:17}',
  'World::with_strict_capacity(16,80,192)','for _ in 0..48'
])if(!b50.includes(rule))throw Error('B50 native arm/schedule changed: '+rule);
const sequences=[[0,2,-1],[-1,-1,3],[5,0,5],[-7,4,0],[1,1,-2],[19,-3,-1]];
const arms=['True','BlindNever','ForcedClone','Static'];
const plans=[];
for(let sequence=0;sequence<6;sequence++)
  for(const shift of [false,true])for(const arm of arms)
    plans.push({sequence,shift,arm,inputs:sequences[sequence]});
if(plans.length!==48)throw Error('B50 case denominator changed');
const safe=x=>{
  if(!Number.isSafeInteger(x))throw Error('B50 out-of-range arithmetic '+x);
  return x;
};
class CIL {
  constructor(plan){
    this.code=(plan.arm==='Static'?compact:original).map(o=>({...o}));
    if(plan.arm==='BlindNever')
      this.code[12]={kind:'Sub',dst:1,lhs:0,rhs:0,next:13};
    if(plan.arm==='ForcedClone'){
      this.code[12]={kind:'Const',dst:1,value:1,next:13};
      this.code[16]={kind:'Branch',test:1,nonzero:26,zero:17};
    }
    const preamble=this.code.length;
    if(plan.shift)this.code.push({kind:'Const',dst:3,value:0,next:0});
    this.initial_len=this.code.length;
    this.pc=plan.shift?preamble:0;
    this.regs=[0,0,0,0];this.cells=new Map();
    this.credits=144;this.charged=0;this.edits=0;this.stopped=false;
    this.correct=[0,0,0];this.wrong=[0,0,0];
    this.peak_storage=this.storage();
  }
  storage(){return 1+this.code.length+this.cells.size;}
  cell(k){return this.cells.has(k)?this.cells.get(k):null;}
  read(k){return this.cells.get(k)??0;}
  reg(i){
    if(!Number.isInteger(i)||i<0||i>3)throw Error('B50 bad register index');
    return this.regs[i];
  }
  node(i){
    if(!Number.isInteger(i)||i<0||i>=this.code.length)
      throw Error('B50 invalid executable index '+i);
    return this.code[i];
  }
  next(i){this.node(i);return i;}
  peak(){
    this.peak_storage=Math.max(this.peak_storage,this.storage());
    if(this.peak_storage>192||this.code.length>80||this.cells.size>16)
      throw Error('B50 storage invariant failed');
  }
  step(window){
    if(this.stopped||this.credits<=0)throw Error('B50 unpaid execution');
    const op=this.node(this.pc),r=i=>this.reg(i);
    this.charged++;this.credits--;
    switch(op.kind){
      case 'Const':
        this.regs[op.dst]=safe(op.value);this.pc=this.next(op.next);break;
      case 'Add':
        this.regs[op.dst]=safe(r(op.lhs)+r(op.rhs));
        this.pc=this.next(op.next);break;
      case 'Sub':
        this.regs[op.dst]=safe(r(op.lhs)-r(op.rhs));
        this.pc=this.next(op.next);break;
      case 'Read':
        this.regs[op.dst]=this.read(r(op.address));
        this.pc=this.next(op.next);break;
      case 'Write':{
        const key=r(op.address),value=r(op.value);
        if(!this.cells.has(key)&&(this.cells.size>=16||this.storage()+1>192))
          throw Error('B50 Write storage overflow');
        this.cells.set(key,value);this.pc=this.next(op.next);
        if(key===11){
          if(value===this.read(7))this.correct[window]++;
          else this.wrong[window]++;
        }
        break;
      }
      case 'Branch':
        this.pc=this.next(r(op.test)!==0?op.nonzero:op.zero);break;
      case 'CloneNode':{
        const template=this.node(r(op.source));
        if(this.code.length>=80||this.storage()+1>192)
          throw Error('B50 CloneNode storage overflow');
        this.regs[op.dst]=this.code.length;
        this.code.push({...template});this.pc=this.next(op.next);break;
      }
      case 'ReadNext':{
        const node=this.node(r(op.node));
        if(!Object.hasOwn(node,'next'))throw Error('B50 ReadNext nonedge');
        this.regs[op.dst]=node.next;this.pc=this.next(op.next);break;
      }
      case 'PatchNext':{
        const node=this.node(r(op.node)),target=this.next(r(op.target));
        if(!Object.hasOwn(node,'next'))throw Error('B50 PatchNext nonedge');
        if(node.next!==target)this.edits++;
        node.next=target;this.pc=this.next(op.next);break;
      }
      case 'Halt':
        this.stopped=true;break;
      default:throw Error('B50 unknown CIL op '+op.kind);
    }
    this.peak();
  }
  run(plan,id){
    for(let w=0;w<3;w++){
      const input=plan.inputs[w];
      if(!this.cells.has(7)&&this.cells.size>=16)
        throw Error('B50 external SetCell failed');
      this.cells.set(7,input);this.peak();
      for(let attempt=0;attempt<48;attempt++){
        if(this.stopped)break;
        this.step(w);
      }
    }
    return {
      type:'case',case:id,sequence:plan.sequence,shift:plan.shift,
      arm:plan.arm,inputs:plan.inputs,initial_len:this.initial_len,
      charged:this.charged,correct:this.correct,wrong:this.wrong,
      edits:this.edits,stopped:this.stopped,peak_storage:this.peak_storage,
      clones:this.code.length-this.initial_len,
      route9:plan.arm==='Static'?null:this.node(9).next,
      final_code_len:this.code.length,final_pc:this.pc,
      final_registers:this.regs,
      final_cells:[7,11,22,30].map(key=>this.cell(key))
    };
  }
}
const lines=fs.readFileSync(evidence,'utf8').trimEnd().split(/\r?\n/);
if(lines.length!==49)throw Error('B50 expected 1 header + 48 native cases; got '+lines.length);
const native=lines.map((line,i)=>{
  try{return JSON.parse(line);}catch(e){throw Error('B50 invalid native line '+(i+1)+': '+e);}
});
const meta=native.shift();
if(meta.type!=='meta'||meta.study!=='B50'||meta.version!==1||!meta.source_sha)
  throw Error('B50 malformed native source metadata');
if(process.env.GITHUB_SHA&&meta.source_sha!==process.env.GITHUB_SHA)
  throw Error('B50 native trace SHA mismatch with CI');
const models=[];
for(let i=0;i<plans.length;i++){
  const observed=native[i],predicted=new CIL(plans[i]).run(plans[i],i);
  if(!isDeepStrictEqual(predicted,observed)){
    throw Error('B50 native/model divergence case '+i+
      '\\npredicted '+JSON.stringify(predicted)+
      '\\nnative '+JSON.stringify(observed));
  }
  models.push(predicted);
}
const cohorts=[];
for(const shift of [false,true])for(const arm of arms){
  const rows=models.filter(r=>r.shift===shift&&r.arm===arm);
  if(rows.length!==6)throw Error('B50 unmatched cohort');
  const correct=[0,0,0],wrong=[0,0,0];
  let charged=0,edits=0,clones=0,stops=0;
  for(const row of rows){
    charged+=row.charged;edits+=row.edits;clones+=row.clones;
    stops+=Number(row.stopped);
    for(let j=0;j<3;j++){
      correct[j]+=row.correct[j];wrong[j]+=row.wrong[j];
    }
  }
  cohorts.push({shift,arm,charged,correct,wrong,edits,clones,stops,
    holdout_correct:correct[1]+correct[2],holdout_wrong:wrong[1]+wrong[2]});
}
if(cohorts.some(c=>c.charged!==864||c.stops!==0))
  throw Error('B50 equal-realized-exposure preregistration gate failed');
const h50= [false,true].every(shift=>{
  const same=cohorts.filter(c=>c.shift===shift);
  const t=same.find(c=>c.arm==='True');
  const better=arm=>{
    const x=same.find(c=>c.arm===arm);
    return t.holdout_correct>x.holdout_correct &&
      t.holdout_wrong<=x.holdout_wrong;
  };
  return better('BlindNever')&&better('ForcedClone')&&better('Static');
});
process.stdout.write(JSON.stringify({
  study:'B50',source_sha:meta.source_sha,
  native_worlds:48,independent_replay_worlds:models.length,
  paid_steps:models.reduce((s,r)=>s+r.charged,0),
  first_divergence:null,cohorts,h50_accepted:h50,
  b3_accepted:false,b4_accepted:false,cyberlife_established:false
},null,2)+'\n');
