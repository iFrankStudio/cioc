// CL-1B B49: independently EXECUTES every native CIL step, including
// dynamic CloneNode/PatchNext and real shared-cell output. Compares each
// native per-event record to this distinct JS implementation, fail closed at
// the FIRST divergence. It does NOT infer fitness from source patterns.
import fs from 'node:fs';
import {isDeepStrictEqual} from 'node:util';

const root=new URL('../../cl0/crates/world/tests/',import.meta.url);
const b42=fs.readFileSync(new URL('cl1b_b42_self_assembled_probe.rs',root),'utf8');
const b49=fs.readFileSync(new URL('cl1b_b49_dynamic_replay.rs',root),'utf8');
const tracePath='cl1b-ci-evidence/b49_native_steps.jsonl';
if(!fs.existsSync(tracePath))
  throw Error('B49 missing same-CI native trace artifact, cannot claim independent replay');

const founderRaw=b42.match(/let mut code = vec!\[([\s\S]*?)\n    \];/)?.[1];
const staticRaw=b42.match(/if arm == Arm::StaticCorrect \{\s*return vec!\[([\s\S]*?)\n        \];/)?.[1];
if(!founderRaw||!staticRaw)throw Error('B49 cannot parse actual B42 Rust CIL founder');
const parse=raw=>[...raw.matchAll(/Op::([A-Za-z]+)\s*\{([^}]*)\}|Op::Halt\b/g)]
  .map(m=>{
    const op={kind:m[1]??'Halt'};
    for(const pair of (m[2]??'').matchAll(/(\w+)\s*:\s*(-?\d+)/g))
      op[pair[1]]=Number(pair[2]);
    return op;
  });
const b42Graph=parse(founderRaw),staticGraph=parse(staticRaw);
if(b42Graph.length!==38||staticGraph.length!==4||
   b42Graph[7].kind!=='Write'||b42Graph[9].next!==10||
   b42Graph[10].kind!=='Write'||b42Graph[11].kind!=='Read'||
   b42Graph[13].kind!=='Branch'||b42Graph[18].kind!=='CloneNode'||
   b42Graph[27].kind!=='ReadNext'||b42Graph[29].kind!=='PatchNext'||
   b42Graph[33].kind!=='Write')
  throw Error('B49 B42 source/ISA shape changed');
for(const pattern of [
  'Arm::BlindSelf => nodes[12]=Op::Sub{dst:1,lhs:0,rhs:0,next:13}',
  'Arm::BlindZero => nodes[12]=Op::Sub{dst:1,lhs:0,rhs:3,next:13}',
  'if shift { nodes.push(Op::Const{dst:3,value:0,next:0}); }',
  'World::with_strict_capacity(16,80,192)',
  'for attempt in 0..48',
  'EventKind::ExecutableEdgePatched{node,before,after}',
]){
  if(!b49.includes(pattern))throw Error('B49 registered CIL/tracing policy drift: '+pattern);
}
const OLD=[
  [23,0,-1],[-9,17,23],[1,-9,0],[0,23,17],
  [17,-9,-9],[-1,1,23],[23,23,0],[-9,0,1],
];
const NEW=[[2,0,-7],[-4,31,0],[0,7,-4],[31,2,7]];
const arms=['True','BlindSelf','BlindZero','Static'];
const expectedCases=[];
for(const [block,seqs]of [[0,OLD],[1,NEW]])for(let seed=0;seed<seqs.length;seed++)
  for(const shift of [false,true])for(const arm of arms)
    expectedCases.push({block,seed,shift,arm,inputs:seqs[seed]});
if(expectedCases.length!==96)throw Error('B49 case-plan denominator');

const num=x=>{
  if(!Number.isSafeInteger(x))throw Error('Unsafe B49 native number: '+x);
  return x;
};
class IndependentCIL {
  constructor(plan){
    this.plan=plan;
    this.code=(plan.arm==='Static'?staticGraph:b42Graph).map(op=>({...op}));
    if(plan.arm==='BlindSelf')
      this.code[12]={kind:'Sub',dst:1,lhs:0,rhs:0,next:13};
    else if(plan.arm==='BlindZero')
      this.code[12]={kind:'Sub',dst:1,lhs:0,rhs:3,next:13};
    const preamble=this.code.length;
    if(plan.shift)this.code.push({kind:'Const',dst:3,value:0,next:0});
    this.initial_len=this.code.length;
    this.pc=plan.shift?preamble:0;
    this.regs=[0,0,0,0];
    this.credits=144;
    this.tick=0;
    this.stopped=false;
    this.cells=new Map();
    this.edits=0;
    this.paid=0;
    this.peakStorage=1+this.code.length;
    this.correct=[0,0,0];this.wrong=[0,0,0];
    this.lastWindow=-1;
  }
  storage(){return 1+this.code.length+this.cells.size;}
  edge(index){
    const op=this.code[index];
    return op&&Object.hasOwn(op,'next')?op.next:null;
  }
  cell(key){return this.cells.has(key)?this.cells.get(key):null;}
  read(key){return this.cells.get(key)??0;}
  reg(i){
    if(!Number.isInteger(i)||i<0||i>3)throw Error('B49 invalid register '+i);
    return this.regs[i];
  }
  setInput(input,window){
    if(window!==this.lastWindow+1)throw Error('B49 nonconsecutive world input window');
    this.lastWindow=window;
    const isNew=!this.cells.has(7);
    if(isNew&&this.cells.size>=16)throw Error('B49 strict data cell limit reached');
    this.cells.set(7,input);this.tick++;
    this.peakStorage=Math.max(this.peakStorage,this.storage());
    return{
      type:'input',window,input,tick:this.tick,cell7:this.cell(7),
      credits:this.credits,storage:this.storage()
    };
  }
  step(window,attempt){
    if(this.stopped||this.credits===0)throw Error('B49 attempted unpayable CIL instruction');
    const at=this.pc,op=this.code[at];
    if(!op)throw Error('B49 undefined CIL PC '+at);
    this.credits--;
    this.paid++;
    const r=i=>this.reg(i);
    const go=i=>{
      if(!Number.isInteger(i)||i<0||i>=this.code.length)
        throw Error('B49 invalid next instruction '+i);
      return i;
    };
    let event={type:'executed'};
    switch(op.kind){
      case 'Const':
        this.regs[op.dst]=num(op.value);this.pc=go(op.next);break;
      case 'Add':
        this.regs[op.dst]=num(r(op.lhs)+r(op.rhs));this.pc=go(op.next);break;
      case 'Sub':
        this.regs[op.dst]=num(r(op.lhs)-r(op.rhs));this.pc=go(op.next);break;
      case 'Read':{
        const key=r(op.address),value=this.read(key);
        this.regs[op.dst]=value;this.pc=go(op.next);
        event={type:'read',key,value};break;
      }
      case 'Write':{
        const key=r(op.address),value=r(op.value);
        const newCell=!this.cells.has(key);
        if(newCell&&(this.cells.size>=16||this.storage()+1>192))
          throw Error('B49 capacity would reject CIL Write');
        this.cells.set(key,value);this.pc=go(op.next);
        event={type:'write',key,value};
        if(key===11){
          if(value===this.read(7))this.correct[window]++;
          else this.wrong[window]++;
        }
        break;
      }
      case 'Branch':
        this.pc=go(r(op.test)!==0?op.nonzero:op.zero);break;
      case 'CloneNode':{
        const source=go(r(op.source));
        if(this.code.length>=80||this.storage()+1>192)
          throw Error('B49 CloneNode storage exhausted');
        const clone=structuredClone(this.code[source]);
        this.regs[op.dst]=this.code.length;
        this.code.push(clone);
        this.pc=go(op.next);break;
      }
      case 'PatchNext':{
        const node=go(r(op.node)),target=go(r(op.target));
        const old=this.edge(node);
        if(old===null)throw Error('B49 unpatchable CIL node '+node);
        this.code[node].next=target;this.pc=go(op.next);
        if(old!==target){
          event={type:'patch',node,before:old,after:target};
          this.edits++;
        }
        break;
      }
      case 'ReadNext':{
        const node=go(r(op.node)),edge=this.edge(node);
        if(edge===null)throw Error('B49 ReadNext without one successor');
        this.regs[op.dst]=edge;this.pc=go(op.next);break;
      }
      case 'Halt':
        this.stopped=true;event={type:'halted'};break;
      default:throw Error('B49 independently unsupported opcode '+op.kind);
    }
    this.tick++;
    this.peakStorage=Math.max(this.peakStorage,this.storage());
    const created=this.code.length-this.initial_len;
    return{
      type:'step',window,attempt,
      tick:this.tick,pc:at,pc_after:this.pc,
      event,regs:[...this.regs],credits:this.credits,
      stopped:this.stopped,
      cells:[this.cell(7),this.cell(11),this.cell(22),this.cell(30)],
      code_len:this.code.length,
      route9:this.edge(9),edge7:this.edge(7),edge10:this.edge(10),
      edge11:this.edge(11),clone_count:created,
      new_edge:created>0?this.edge(this.code.length-1):null,
      storage:this.storage()
    };
  }
  result(){
    return{
      type:'end',paid:this.paid,correct:this.correct,wrong:this.wrong,
      edits:this.edits,stopped:this.stopped,
      peak_storage:this.peakStorage,
      clone_count:this.code.length-this.initial_len
    };
  }
}

const lines=fs.readFileSync(tracePath,'utf8').trimEnd().split(/\r?\n/);
let current=null,caseId=-1,comparedSteps=0,comparedInputs=0,clones=0,patches=0;
let stoppedWorlds=0;
const modelAggregate={};
const discrepancies=[];
const b48=[[[[0,0],[0,0]],[[0,0],[0,0]],[[0,0],[0,0]],[[0,0],[0,0]]],
           [[[0,0],[0,0]],[[0,0],[0,0]],[[0,0],[0,0]],[[0,0],[0,0]]]];
const compare=(expected,actual,lineNo)=>{
  if(!isDeepStrictEqual(expected,actual)){
    const error={line:lineNo,case:caseId,expected,actual};
    discrepancies.push(error);
    throw Error('B49 FIRST NATIVE-vs-INDEPENDENT divergence: '+JSON.stringify(error).slice(0,3500));
  }
};
let meta=null;
for(let i=0;i<lines.length;i++){
  let row;
  try{row=JSON.parse(lines[i]);}catch(e){throw Error('B49 malformed native JSONL at line '+(i+1)+': '+e);}
  if(!meta){
    if(row.type!=='meta'||row.study!=='B49'||row.version!==1)
      throw Error('B49 source trace metadata missing');
    if(process.env.GITHUB_SHA&&row.source_sha!==process.env.GITHUB_SHA)
      throw Error('B49 native trace SHA does not match this CI source SHA');
    meta=row;continue;
  }
  if(row.type==='case'){
    if(current!==null)throw Error('B49 previous world not concluded');
    caseId++;
    const p=expectedCases[caseId];
    if(!p)throw Error('B49 surplus native case');
    for(const [key,value]of Object.entries(p))
      compare(value,row[key],i+1);
    if(row.case!==caseId||row.initial_credits!==144||row.initial_tick!==0)
      throw Error('B49 native case lacks paid/clock invariants');
    current=new IndependentCIL(p);
    const expected={
      type:'case',case:caseId,block:p.block,seed:p.seed,shift:p.shift,
      arm:p.arm,inputs:p.inputs,initial_len:current.initial_len,
      initial_pc:current.pc,initial_tick:0,initial_credits:144,
      initial_storage:current.storage()
    };
    compare(expected,row,i+1);
    continue;
  }
  if(current===null)throw Error('B49 orphan native trace row '+i);
  if(row.case!==caseId)throw Error('B49 native case identity unexpectedly changed');
  const plan=expectedCases[caseId];
  if(row.type==='input'){
    const nativeCaseRow=current.setInput(plan.inputs[row.window],row.window);
    compare({case:caseId,...nativeCaseRow},row,i+1);
    comparedInputs++;
  } else if(row.type==='step'){
    if(row.window!==current.lastWindow||row.attempt>=48)
      throw Error('B49 native execution outside registered phase');
    const expected=current.step(row.window,row.attempt);
    compare({case:caseId,...expected},row,i+1);
    comparedSteps++;
    clones+=expected.code_len-expected.clone_count === current.initial_len ? 0 : 0;
    patches+=Number(expected.event.type==='patch');
  } else if(row.type==='end'){
    const expected=current.result();
    compare({case:caseId,...expected},row,i+1);
    stoppedWorlds+=Number(expected.stopped);
    const bucket=modelAggregate[plan.block]??={};
    const key=plan.arm+(plan.shift?'+1':'base');
    const sum=bucket[key]??={worlds:0,paid:0,correct:[0,0,0],wrong:[0,0,0],edits:0,stopped:0};
    sum.worlds++;sum.paid+=expected.paid;sum.edits+=expected.edits;
    sum.stopped+=Number(expected.stopped);
    for(let w=0;w<3;w++){
      sum.correct[w]+=expected.correct[w];sum.wrong[w]+=expected.wrong[w];
    }
    if(plan.block===0){
      const a=arms.indexOf(plan.arm),shift=Number(plan.shift);
      for(let h=0;h<2;h++){
        b48[shift][a][h][0]+=expected.correct[h+1];
        b48[shift][a][h][1]+=expected.wrong[h+1];
      }
    }
    current=null;
  } else throw Error('B49 unrecognized native trace record '+row.type);
}
if(current||caseId+1!==96||comparedInputs!==96*3)
  throw Error('B49 incomplete world or intervention coverage');
if(comparedSteps<3000)throw Error('B49 suspiciously few paid CIL events');
const expectedB48=[
  [[[64,15],[64,16]],[[18,54],[20,60]],[[0,9],[0,10]],[[96,0],[96,0]]],
  [[[64,15],[63,10]],[[18,54],[20,60]],[[0,9],[0,10]],[[89,7],[89,7]]]
];
compare(expectedB48,b48,'B48 frozen unseen replication aggregate');
for(const block of [0,1]){
  for(const arm of ['True','BlindSelf','Static']){
    for(const phase of ['base','+1']){
      const row=modelAggregate[block][arm+phase];
      if(row.paid!==row.worlds*144||row.stopped!==0)
        throw Error('B49 claimed equal-exposure control actually stopped: '+arm+phase);
    }
  }
}
const result={
  study:'B49',class:'actual independent SOURCE-PARSED dynamic CIL event-by-event parity',
  native_source_sha:meta.source_sha,
  worlds:caseId+1,input_interventions:comparedInputs,
  matched_native_paid_events:comparedSteps,
  matched_dynamic_code_patches:patches,
  stopped_worlds:stoppedWorlds,
  first_divergence:null,
  b48_unseen_replication:b48,
  compared_origin:'real charged CIL native process events, no host output writes scored',
  new_prospective_sequences:NEW,
  aggregate:modelAggregate,
  claim:'validation of native measurement and CIL mechanics only; B48 H48 not rescued; B3/B4 NOT MET'
};
process.stdout.write(JSON.stringify(result,null,2)+'\n');
