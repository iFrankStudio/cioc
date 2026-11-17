// B51c independent CIL interpreter: executes the literal Rust founder and
// reconstructs EVERY actual native paid instruction and external feedback.
// No trace => no pass; no native outcome used as independent state input.
import fs from 'node:fs';
import nodePath from 'node:path';
import {fileURLToPath} from 'node:url';
import {isDeepStrictEqual} from 'node:util';

const rustPath=new URL('../../cl0/crates/world/tests/cl1b_b51c_paid_search_retention.rs',import.meta.url);
const rust=fs.readFileSync(rustPath,'utf8');
const file='cl1b-ci-evidence/b51c_native_steps.jsonl';

const source=rust.match(/fn founder\(\)->Vec<Op>\{[\s\S]*?\bvec!\[([\s\S]*?)\n    \]/)?.[1];
if(!source)throw Error('B51c cannot parse native founder instruction literal');
const parse=raw=>[...raw.matchAll(/Op::([A-Za-z]+)\s*\{([^}]*)\}|Op::Halt\b/g)].map(m=>{
  const instr={kind:m[1]??'Halt'};
  for(const kv of (m[2]??'').matchAll(/(\w+)\s*:\s*(-?\d+)/g))
    instr[kv[1]]=Number(kv[2]);
  return instr;
});
const founder=parse(source);
if(founder.length!==72||founder[0].kind!=='Add'||
   founder[1].kind!=='Sub'||founder[2].kind!=='Write'||
   founder[33].kind!=='Read'||founder[33].next!==33||
   founder[50].kind!=='PatchNext'||founder[71].kind!=='Halt')
  throw Error('B51c actual source geometry does not match frozen preregistration');
for(const clause of [
  'const BUDGET:usize=1300','const CAP:usize=192',
  'const TRAIN:[i64;2]=[5,-3]','const HELD:[i64;3]=[11,-7,0]',
  'process.pc=3;', 'with_native_executable_edit_trace()',
  'with_native_patchnext_disabled()'
])if(!rust.includes(clause))throw Error('B51c unregistered native protocol change '+clause);

const TRAIN=[5,-3],HELD=[11,-7,0],BUDGET=1300;
const safe=v=>{
  if(!Number.isSafeInteger(v))throw Error('B51c arithmetic overflow/unsafe integer '+v);
  return v;
};
class CIL{
  constructor(s,k,arm){
    this.s=s;this.k=k;this.arm=arm;
    this.code=founder.map(op=>({...op}));
    this.maxCodeNodes=120;this.maxStorage=192;this.maxDataCells=16;
    this.pc=3;this.regs=[0,0,0,0];this.credits=BUDGET;
    this.cells=new Map();this.tick=0;this.paid=0;this.stopped=false;
    this.input=TRAIN[0];this.trainingOutputs=0;
    this.trainingCorrect=0;this.trainingWrong=0;this.feedbackCount=0;
    this.accepted=false;this.acceptStep=null;this.chosen=null;
    this.heldoutOutputs=0;this.correct=0;this.wrong=0;
    this.clones=0;this.patches=0;this.peak=0;
  }
  storage(){return 1+this.code.length+this.cells.size;}
  cell(key){return this.cells.has(key)?this.cells.get(key):null;}
  read(key){return this.cells.get(key)??0;}
  r(i){
    if(!Number.isInteger(i)||i<0||i>3)throw Error('B51c invalid register '+i);
    return this.regs[i];
  }
  node(i){
    if(!Number.isInteger(i)||i<0||i>=this.code.length)
      throw Error('B51c invalid executable node '+i);
    return this.code[i];
  }
  target(i){this.node(i);return i;}
  edge(i){const n=this.code[i];return n&&Object.hasOwn(n,'next')?n.next:null;}
  stateCells(){return [7,11,20,21,22,24,30,31].map(key=>this.cell(key));}
  external(caseId,key,value,reason){
    if(!Number.isSafeInteger(value))throw Error('B51c invalid external integer');
    const isNew=!this.cells.has(key);
    if(isNew&&this.cells.size>=this.maxDataCells)throw Error('B51c data-cell capacity exceeded');
    this.cells.set(key,value);
    this.tick++;
    if(this.storage()>this.maxStorage)throw Error('B51c external storage cap exceeded');
    return {type:'external',case:caseId,reason,key,value,tick:this.tick,
      credits:this.credits,storage:this.storage(),cells:this.stateCells()};
  }
  step(caseId){
    if(this.stopped||this.credits<=0)
      throw Error('B51c attempted unpaid or stopped instruction');
    const oldPC=this.pc,op=this.node(this.pc),oldLen=this.code.length;
    const r=i=>this.r(i),go=i=>this.target(i);
    this.paid++;this.tick++;this.credits--;
    let event={type:'executed'};
    switch(op.kind){
      case 'Const':
        this.regs[op.dst]=safe(op.value);
        this.pc=go(op.next);break;
      case 'Add':
        this.regs[op.dst]=safe(r(op.lhs)+r(op.rhs));
        this.pc=go(op.next);break;
      case 'Sub':
        this.regs[op.dst]=safe(r(op.lhs)-r(op.rhs));
        this.pc=go(op.next);break;
      case 'Read':{
        const key=r(op.address),value=this.read(key);
        this.regs[op.dst]=value;this.pc=go(op.next);
        event={type:'read',key,value};break;
      }
      case 'Write':{
        const key=r(op.address),value=r(op.value);
        if(!this.cells.has(key)&&(this.cells.size>=this.maxDataCells||this.storage()+1>this.maxStorage))
          throw Error('B51c CIL Write cell capacity exceeded');
        this.cells.set(key,value);this.pc=go(op.next);
        event={type:'write',key,value};break;
      }
      case 'Branch':
        this.pc=go(r(op.test)!==0?op.nonzero:op.zero);break;
      case 'CloneNode':{
        const template={...this.node(r(op.source))};
        if(this.code.length>=this.maxCodeNodes||this.storage()+1>this.maxStorage)
          throw Error('B51c CloneNode exceeds capacity');
        this.regs[op.dst]=this.code.length;
        this.code.push(template);this.pc=go(op.next);
        break;
      }
      case 'PatchConst':{
        const sourceNode=this.node(r(op.node));
        if(sourceNode.kind!=='Const')throw Error('B55a PatchConst target not Const');
        sourceNode.value=r(op.value);this.pc=go(op.next);
        break;
      }
      case 'PatchNext':{
        const sourceNode=this.node(r(op.node)),target=go(r(op.target));
        if(!Object.hasOwn(sourceNode,'next'))
          throw Error('B51c PatchNext target has no next field');
        const before=sourceNode.next;
        sourceNode.next=target;this.pc=go(op.next);
        if(before!==target)event={type:'patch',node:r(op.node),before,after:target};
        break;
      }
      case 'Halt':
        this.stopped=true;event={type:'halt'};break;
      default:throw Error('B51c unexpected actual Rust CIL opcode '+op.kind);
    }
    const grown=this.code.length-oldLen;
    this.clones+=grown;
    this.patches+=Number(event.type==='patch');
    this.peak=Math.max(this.peak,this.storage());
    if(this.peak>this.maxStorage||this.code.length>this.maxCodeNodes)
      throw Error('B51c strict native storage bound broken');
    return {
      type:'step',case:caseId,step:this.paid,tick:this.tick,
      pc:oldPC,pc_after:this.pc,event,regs:[...this.regs],
      credits:this.credits,stopped:this.stopped,
      code_len:this.code.length,growth:grown,
      edge2:this.edge(2),edge33:this.edge(33),
      edge72:this.edge(72),edge73:this.edge(73),
      storage:this.storage(),cells:this.stateCells()
    };
  }
  transition(caseId,event){
    const result=[];
    if(event.type==='write'&&event.key===11){
      const correct=event.value===this.s*this.input+this.k;
      if(this.accepted){
        this.heldoutOutputs++;
        if(correct)this.correct++;else this.wrong++;
        this.input=HELD[this.heldoutOutputs%HELD.length];
        result.push(this.external(caseId,7,this.input,'heldout_next'));
      }else{
        this.trainingOutputs++;
        if(correct)this.trainingCorrect++;else this.trainingWrong++;
        const feedback=this.arm==='TRUE'?Number(correct):1;
        result.push(this.external(caseId,20,feedback,'post_output_bit'));
        this.feedbackCount++;
        this.input=TRAIN[this.trainingOutputs%TRAIN.length];
        result.push(this.external(caseId,7,this.input,'training_next'));
      }
    }else if(event.type==='patch'&&event.node===2&&
             event.before===42&&event.after===30){
      if(this.accepted||this.trainingOutputs<2||this.trainingOutputs%2)
        throw Error('B51c native retention occurred outside paired evidence');
      this.accepted=true;this.acceptStep=this.paid;
      this.chosen=[this.cell(22),this.cell(21)];
      this.input=HELD[0];
      result.push(this.external(caseId,7,this.input,'retained_holdout_start'));
    }
    return result;
  }
  end(caseId){
    return{type:'end',case:caseId,s:this.s,k:this.k,arm:this.arm,
      paid:this.paid,stopped:this.stopped,credits:this.credits,
      accepted:this.accepted,accept_step:this.acceptStep,
      chosen:this.chosen,training_outputs:this.trainingOutputs,
      feedback_count:this.feedbackCount,training_correct:this.trainingCorrect,
      training_wrong:this.trainingWrong,holdout_outputs:this.heldoutOutputs,
      holdout_correct:this.correct,holdout_wrong:this.wrong,
      clones:this.clones,patches:this.patches,peak_storage:this.peak,
      final_pc:this.pc,final_regs:[...this.regs],
      final_len:this.code.length,final_cells:this.stateCells(),
      edge2:this.edge(2),edge33:this.edge(33)};
  }
}
// Expose the actual independent CIL interpreter for B51d's unchanged founder.
export {CIL,founder};
if(process.argv[1]&&nodePath.resolve(process.argv[1])===fileURLToPath(import.meta.url)){
if(!fs.existsSync(file))throw Error('B51c native JSONL missing; fail closed');
const nativeLines=fs.readFileSync(file,'utf8').trimEnd().split(/\r?\n/);
let cursor=0,comparedPaid=0,comparedExternal=0,totalPatches=0;
const native=()=>{
  if(cursor>=nativeLines.length)throw Error('B51c truncated native JSONL at '+cursor);
  const line=nativeLines[cursor++];
  try{return JSON.parse(line);}catch(e){
    throw Error('B51c malformed native JSONL line '+cursor+': '+e);
  }
};
const cmp=(expected,detail)=>{
  const observed=native();
  if(!isDeepStrictEqual(expected,observed))
    throw Error('B51c first native-vs-independent divergence: '+detail+
      ' line='+cursor+'\nexpected='+JSON.stringify(expected)+
      '\nobserved='+JSON.stringify(observed));
};
const meta=native();
if(meta.type!=='meta'||meta.study!=='B51c'||meta.version!==1||!meta.source_sha)
  throw Error('B51c missing native experiment/source metadata');
if(process.env.GITHUB_SHA&&meta.source_sha!==process.env.GITHUB_SHA)
  throw Error('B51c stale native trace: wrong commit SHA');
const reports=[];
for(const s of [-1,1])for(const k of [-3,-2,2,3])
  for(const arm of ['TRUE','BLIND_SUCCESS']){
    const id=reports.length,world=new CIL(s,k,arm);
    cmp({type:'case',case:id,s,k,arm,
      initial_len:72,initial_credits:BUDGET,initial_storage:73},
      'case header '+id);
    cmp(world.external(id,7,TRAIN[0],'initial_training'),'initial input '+id);
    world.peak=world.storage();
    while(world.paid<BUDGET&&!world.stopped){
      const row=world.step(id);
      cmp(row,'case '+id+' paid step '+world.paid);
      comparedPaid++;totalPatches+=Number(row.event.type==='patch');
      for(const extra of world.transition(id,row.event)){
        cmp(extra,'case '+id+' external after step '+world.paid);
        comparedExternal++;
      }
    }
    const end=world.end(id);
    cmp(end,'end case '+id);
    reports.push(end);
  }
for(const s of [-1,1])for(const k of [-3,-2,2,3]){
  // Re-evaluate the separately seeded compact per-mode exact oracle.
  // The actual six-op loop returns 1 correct output every 6 paid steps.
  cmp({type:'static',result:{s,k,paid:BUDGET,
    correct:Math.floor(BUDGET/6),wrong:0,stopped:false}},
  'six-instruction task-aware static oracle');
}
cmp({type:'disabled',result:{paid:BUDGET,writes:0,
  stopped:false,remaining_credits:0}},
  'same-founder patch-disabled native negative control');
if(cursor!==nativeLines.length)throw Error('B51c surplus, potentially unverified native rows');
const byArm={};
for(const arm of ['TRUE','BLIND_SUCCESS']){
  const group=reports.filter(r=>r.arm===arm);
  byArm[arm]={
    worlds:group.length,
    paid:group.reduce((n,r)=>n+r.paid,0),
    accepted:group.filter(r=>r.accepted).length,
    holdout_correct:group.reduce((n,r)=>n+r.holdout_correct,0),
    holdout_wrong:group.reduce((n,r)=>n+r.holdout_wrong,0),
    heldout_events:group.reduce((n,r)=>n+r.holdout_outputs,0),
    chosen:group.map(r=>({s:r.s,k:r.k,selected:r.chosen,
      trials:r.training_outputs/2,holdout_correct:r.holdout_correct,
      holdout_wrong:r.holdout_wrong,charged:r.paid}))
  };
}
const truthful=byArm.TRUE,blind=byArm.BLIND_SUCCESS;
const allModesSolved=truthful.chosen.every(r=>
  r.selected?.[0]===Number(r.s===-1)&&r.selected?.[1]===r.k&&
  r.holdout_wrong===0&&r.holdout_correct>0);
const selectionBetter=
  truthful.holdout_correct-truthful.holdout_wrong >
  blind.holdout_correct-blind.holdout_wrong;
process.stdout.write(JSON.stringify({
  study:'B51c',native_source_sha:meta.source_sha,
  independent_native_worlds:reports.length,
  independent_paid_steps:comparedPaid,
  independent_external_after_step_rows:comparedExternal,
  native_edge_changes_reproduced:totalPatches,
  first_divergence:null,
  candidate_coverage_all_eight:allModesSolved,
  feedback_selection_beats_blind:selectionBetter,
  hypotheses_engineering_accepted:allModesSolved&&selectionBetter,
  truthful,blind,
  static_oracle_per_mode_correct:Math.floor(BUDGET/6),
  b51d_transfer_not_tested:true,
  b3_met:false,b4_met:false,cyberlife_established:false
},null,2)+'\n');

}
