// CL-1B B51b: independent source-parsed step-by-step interpreter for real
// native Rust trace. H51b/H51c learning and transfer are NOT tested here.
import fs from 'node:fs';
import {isDeepStrictEqual} from 'node:util';

const source=new URL('../../cl0/crates/world/tests/cl1b_b51b_native_candidate_composition.rs',import.meta.url);
const rust=fs.readFileSync(source,'utf8');
const path='cl1b-ci-evidence/b51b_native_steps.jsonl';
if(!fs.existsSync(path))throw Error('B51b missing native same-CI trace; fail closed');
const raw=rust.match(/fn founder\(\)->Vec<Op>\{[\s\S]*?\bvec!\[([\s\S]*?)\n    \]/)?.[1];
if(!raw)throw Error('B51b cannot parse actual CIL Rust founder source');
const parse=s=>[...s.matchAll(/Op::([A-Za-z]+)\s*\{([^}]*)\}|Op::Halt\b/g)].map(m=>{
  const op={kind:m[1]??'Halt'};
  for(const kv of (m[2]??'').matchAll(/(\w+)\s*:\s*(-?\d+)/g)){
    op[kv[1]]=Number(kv[2]);
  }
  return op;
});
const template=parse(raw);
if(template.length!==26||template[0].kind!=='Add'||
   template[1].kind!=='Sub'||template[2].kind!=='Write'||
   template[4].kind!=='CloneNode'||template[14].kind!=='CloneNode'||
   template[25].kind!=='Halt')
  throw Error('B51b source layout drift');
const numeric=n=>{
  if(!Number.isSafeInteger(n))throw Error('B51b unsafe integer '+n);
  return n;
};
class CIL{
  constructor(input){
    this.input=input;this.code=template.map(op=>({...op}));
    this.pc=3;this.regs=[0,0,0,0];this.credits=64;
    this.cells=new Map([[7,input]]);
    this.paid=0;this.tick=1;this.stopped=false;
    this.writes=[];this.patches=[];this.peakStorage=this.storage();
  }
  storage(){return 1+this.code.length+this.cells.size;}
  get(i){
    if(!Number.isInteger(i)||i<0||i>=this.code.length)
      throw Error('B51b invalid PC/node '+i);
    return this.code[i];
  }
  reg(i){
    if(!Number.isInteger(i)||i<0||i>=4)
      throw Error('B51b invalid register '+i);
    return this.regs[i];
  }
  edge(i){const op=this.code[i];return op&&Object.hasOwn(op,'next')?op.next:null;}
  cell(k){return this.cells.has(k)?this.cells.get(k):null;}
  step(caseId){
    if(this.stopped||this.credits===0)throw Error('B51b unpayable instruction');
    const oldPC=this.pc,op=this.get(this.pc),oldLen=this.code.length;
    const r=i=>this.reg(i),go=i=>{this.get(i);return i;};
    this.credits--;this.paid++;this.tick++;
    let kind='executed';
    switch(op.kind){
      case 'Const':{
        this.regs[op.dst]=numeric(op.value);
        this.pc=go(op.next);break;
      }
      case 'Add':{
        this.regs[op.dst]=numeric(r(op.lhs)+r(op.rhs));
        this.pc=go(op.next);break;
      }
      case 'Sub':{
        this.regs[op.dst]=numeric(r(op.lhs)-r(op.rhs));
        this.pc=go(op.next);break;
      }
      case 'Read':{
        const addr=r(op.address);
        this.regs[op.dst]=this.cells.get(addr)??0;
        this.pc=go(op.next);kind='read';break;
      }
      case 'Write':{
        const addr=r(op.address),val=r(op.value);
        if(!this.cells.has(addr)&&this.cells.size>=16)
          throw Error('B51b cell cap violated');
        this.cells.set(addr,val);
        this.pc=go(op.next);kind='write';
        if(addr===11)this.writes.push(val);
        break;
      }
      case 'CloneNode':{
        const sourceIdx=r(op.source);
        const sourceNode={...this.get(sourceIdx)};
        if(this.code.length>=80||this.storage()+1>192)
          throw Error('B51b node cap violated');
        this.regs[op.dst]=this.code.length;
        this.code.push(sourceNode);this.pc=go(op.next);break;
      }
      case 'PatchNext':{
        const sourceIdx=r(op.node),target=go(r(op.target));
        const dest=this.get(sourceIdx);
        if(!Object.hasOwn(dest,'next'))
          throw Error('B51b unpatchable node');
        const before=dest.next;dest.next=target;
        this.pc=go(op.next);
        if(before!==target){
          this.patches.push([sourceIdx,before,target]);kind='patch';
        }
        break;
      }
      case 'Halt':
        this.stopped=true;kind='halt';break;
      default:throw Error('B51b unsupported actual CIL op '+op.kind);
    }
    this.peakStorage=Math.max(this.peakStorage,this.storage());
    if(this.peakStorage>192||this.code.length>80)
      throw Error('B51b strict storage overflow');
    const desc=i=>this.code[i]?{
      opcode:this.code[i].kind==='Add'?'Add':this.code[i].kind==='Sub'?'Sub':'other',
      next:this.edge(i)
    }:null;
    return{
      type:'step',case:caseId,step:this.paid,
      tick:this.tick,pc:oldPC,pc_after:this.pc,
      kind,credits:this.credits,regs:[...this.regs],
      stopped:this.stopped,code_len:this.code.length,
      code_growth:this.code.length-oldLen,
      edge2:this.edge(2),edge11:this.edge(11),edge24:this.edge(24),
      candidate26:desc(26),candidate27:desc(27),
      cells:[this.cell(7),this.cell(11)],storage:this.storage()
    };
  }
  end(caseId){
    return{type:'end',case:caseId,paid:this.paid,
      writes:this.writes,patches:this.patches,
      peak_storage:this.peakStorage,stopped:this.stopped,
      final_code_len:this.code.length,credits:this.credits};
  }
}
const rawLines=fs.readFileSync(path,'utf8').trimEnd().split(/\r?\n/);
if(rawLines.length!==91)throw Error('B51b expected 91 exact native JSONL records, found '+rawLines.length);
const records=rawLines.map((line,i)=>{
  try{return JSON.parse(line);}catch(e){
    throw Error('B51b malformed native JSONL line '+(i+1)+': '+e);
  }
});
const meta=records.shift();
if(meta.type!=='meta'||meta.study!=='B51b'||meta.version!==1||!meta.source_sha)
  throw Error('B51b bad native source metadata');
if(process.env.GITHUB_SHA&&meta.source_sha!==process.env.GITHUB_SHA)
  throw Error('B51b stale or cross-source native trace');
const compare=(expected,actual,label)=>{
  if(!isDeepStrictEqual(expected,actual))
    throw Error('B51b first independent native divergence at '+label+
      '\nexpected='+JSON.stringify(expected)+
      '\nactual='+JSON.stringify(actual));
};
let compared=0,clones=0,changedEdges=0;const cases=[];
for(const [id,input]of [5,-3,0].entries()){
  const model=new CIL(input);
  compare({type:'case',case:id,input,initial_pc:3,
    initial_code_len:26,initial_credits:64,
    after_external_storage:model.storage()},records.shift(),'case '+id);
  for(let step=0;step<28;step++){
    const prediction=model.step(id);
    compare(prediction,records.shift(),'case '+id+' step '+(step+1));
    compared++;clones+=prediction.code_growth;
    changedEdges+=Number(prediction.kind==='patch');
  }
  const end=model.end(id);
  compare(end,records.shift(),'case '+id+' end');
  if(end.paid!==28||!end.stopped||end.final_code_len!==28||
     end.writes[0]!==input+2||end.writes[1]!==2-input||
     end.patches.length!==5||model.storage()!==31)
    throw Error('B51b candidate/charge acceptance invariant failed');
  cases.push({input,paid:end.paid,outputs:end.writes,source_mutations:end.patches,
    new_nodes:end.final_code_len-26,peak_storage:end.peak_storage});
}
if(records.length!==0||compared!==84||clones!==6||changedEdges!==15)
  throw Error('B51b truncated trace or missed candidate provenance');
process.stdout.write(JSON.stringify({
  study:'B51b',native_source_sha:meta.source_sha,
  native_worlds:3,independent_paid_events:compared,
  native_cloned_nodes:clones,patched_edges:changedEdges,
  first_divergence:null,cases,
  candidate_composition_engineering:true,
  b51b_selection_utility:false,b51c_transfer:false,
  b3_met:false,b4_met:false,cyberlife_established:false
},null,2)+'\n');
