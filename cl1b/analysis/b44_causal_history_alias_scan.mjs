// CL-1B B44 independent model. Parses the actual B42 Rust CIL founder,
// constructs two matched causal histories with the same future guest state,
// then executes the CIL interpreter step-by-step. Contrasts with the existing
// independently parsed B43 natural and strong immutable controls.
// A model is NOT Rust CI. No hypothetical accessible event-history opcode.
import { readFileSync } from 'node:fs';
import { fileURLToPath } from 'node:url';
import { run as b43Control } from './b43_feedback_provenance_scan.mjs';

const src = readFileSync(
  new URL('../../cl0/crates/world/tests/cl1b_b42_self_assembled_probe.rs', import.meta.url),
  'utf8'
);
const block = src.match(/let mut code = vec!\[([\s\S]*?)\n    \];/)?.[1];
if (!block) throw Error('B44 exact native B42 founder no longer parseable');
const code = [...block.matchAll(/Op::([A-Za-z]+)\s*\{([^}]*)\}|Op::Halt\b/g)].map(m => {
  const op = { kind: m[1] ?? 'Halt' };
  for (const v of (m[2] ?? '').matchAll(/(\w+)\s*:\s*(-?\d+)/g))
    op[v[1]] = Number(v[2]);
  return op;
});
if (code.length !== 38 || code[7].kind !== 'Write'
    || code[7].address !== 2 || code[7].value !== 0 || code[7].next !== 8
    || code[9].kind !== 'Const' || code[9].value !== 11 || code[9].next !== 10
    || code[10].kind !== 'Write' || code[10].value !== 3 || code[10].next !== 11
    || code[11].kind !== 'Read' || code[13].kind !== 'Branch'
    || code[26].next !== 27 || code[29].kind !== 'PatchNext')
  throw Error('B44 validated native founder source changed');

function fresh(train) {
  return {
    code: code.map(op => ({ ...op })), pc: 0, r: [0,0,0,0],
    cells: new Map(), credits: 128, pool: 0, tick: 0, stopped: false,
    events: [], maxNodes: 80, cap: 192
  };
}
function external(w, what, arg) {
  if (what === 'SetCell') w.cells.set(arg.key, arg.value);
  else if (what === 'PatchEdge') {
    if (!w.code[arg.node] || !Object.hasOwn(w.code[arg.node],'next') || !w.code[arg.next])
      throw Error('invalid external PatchEdge in native-matched model');
    w.code[arg.node].next = arg.next;
  } else throw Error('unrecognized external action');
  w.tick++;
  w.events.push({ tick: w.tick, origin: 'External', what, ...arg });
}
function step(w) {
  if (w.stopped || w.credits <= 0) throw Error('stopped or uncharged B44 native model');
  const op = w.code[w.pc], at = w.pc;
  if (!op) throw Error('undefined B44 CIL node ' + w.pc);
  const r = i => {
    if (!Number.isInteger(i) || i < 0 || i >= w.r.length)
      throw Error('invalid register ' + i);
    return w.r[i];
  };
  w.credits--;
  let evt = { kind: 'Executed', pc: at };
  switch(op.kind) {
    case 'Const': w.r[op.dst] = op.value; w.pc = op.next; break;
    case 'Add': w.r[op.dst] = r(op.lhs) + r(op.rhs); w.pc = op.next; break;
    case 'Sub': w.r[op.dst] = r(op.lhs) - r(op.rhs); w.pc = op.next; break;
    case 'Read': {
      const key = r(op.address), value = w.cells.get(key) ?? 0;
      w.r[op.dst] = value; w.pc = op.next;
      evt = { kind: 'CellRead', pc: at, key, value }; break;
    }
    case 'Write': {
      const key = r(op.address), value = r(op.value);
      w.cells.set(key, value); w.pc = op.next;
      evt = { kind: 'CellWritten', pc: at, key, value }; break;
    }
    case 'Branch': w.pc = r(op.test) !== 0 ? op.nonzero : op.zero; break;
    case 'ReadNext': {
      const node = r(op.node);
      if (!w.code[node] || !Object.hasOwn(w.code[node], 'next'))
        throw Error('invalid source node for ReadNext');
      w.r[op.dst] = w.code[node].next;
      w.pc = op.next; break;
    }
    case 'PatchNext': {
      const node = r(op.node), target = r(op.target);
      if (!w.code[node] || !Object.hasOwn(w.code[node], 'next') || !w.code[target])
        throw Error('invalid B44 PatchNext');
      w.code[node].next = target; w.pc = op.next; break;
    }
    case 'CloneNode': {
      const source = r(op.source);
      if (!w.code[source] || w.code.length >= w.maxNodes)
        throw Error('B44 clone limit');
      const index = w.code.length;
      w.code.push({ ...w.code[source] });
      w.r[op.dst] = index; w.pc = op.next; break;
    }
    case 'Halt': w.stopped = true; break;
    default: throw Error('unsupported B44 source CIL opcode ' + op.kind);
  }
  w.tick++;
  const observed = { tick: w.tick, origin: 'Internal(1)', pc: at,
    creditsAfter: w.credits, ...evt };
  w.events.push(observed);
  return observed;
}
function projection(w) {
  // Full observable substrate model state EXCEPT researcher-only events;
  // includes current instruction graph, all cells, registers, budget, tick.
  return JSON.stringify({
    code: w.code, pc: w.pc, r: w.r, cells: [...w.cells.entries()].sort((a,b)=>a[0]-b[0]),
    credits: w.credits, pool: w.pool, tick: w.tick, stopped: w.stopped,
    maxNodes: w.maxNodes, cap: w.cap
  });
}
function copy(w) {
  return { ...w, code: w.code.map(op=>({ ...op })), r: [...w.r],
    cells: new Map(w.cells), events: w.events.map(e => ({ ...e })) };
}
function runPair(train,holdout) {
  let ancestor = fresh(train);
  external(ancestor, 'SetCell', {key:7,value:train});
  for(let i=0;i<9;i++)step(ancestor);
  if (ancestor.pc!==9 || ancestor.r[0]!==train+1
      || ancestor.cells.get(7)!==train+1) throw Error('B44 pre-service native state');
  const challenge=train+1, natural=copy(ancestor), fake=copy(ancestor);
  external(natural, 'PatchEdge', {node:9,next:7});
  external(natural, 'PatchEdge', {node:7,next:11});
  external(fake, 'PatchEdge', {node:9,next:10});
  external(fake, 'PatchEdge', {node:7,next:8});
  const naturalRoute=step(natural), forgedRoute=step(fake);
  if (naturalRoute.pc!==9 || forgedRoute.pc!==9)throw Error('B44 route mismatch');
  const nativeWrite=step(natural), wrongWrite=step(fake);
  if (nativeWrite.kind!=='CellWritten' || nativeWrite.key!==11
      || nativeWrite.value!==challenge || nativeWrite.pc!==7
      || wrongWrite.kind!=='CellWritten' || wrongWrite.key!==11
      || wrongWrite.value!==0 || wrongWrite.pc!==10)
    throw Error('B44 distinguishable native writer source invalid');
  external(natural, 'SetCell', {key:7,value:challenge});
  external(fake, 'SetCell', {key:11,value:challenge});
  for(const w of [natural,fake]){
    external(w, 'PatchEdge', {node:9,next:10});
    external(w, 'PatchEdge', {node:7,next:8});
  }
  if(projection(natural)!==projection(fake))
    throw Error('B44 causal histories did not converge to same complete guest state');
  if(JSON.stringify(natural.events)===JSON.stringify(fake.events))
    throw Error('B44 external forensic provenance history lost');

  const recorded = [
    {correct:[0,0],wrong:[0,0],restored:false},
    {correct:[0,0],wrong:[0,0],restored:false}
  ];
  let firstRead=null, firstDecision=null, mirrorCount=0;
  for(let phase=0;phase<2;phase++){
    if(phase===1)for(const w of [natural,fake])
      external(w, 'SetCell', {key:7,value:holdout});
    if(projection(natural)!==projection(fake))throw Error('post-holdout projection mismatch');
    const expected=phase===0?train:holdout;
    for(let i=0;i<(phase===0?53:64);i++){
      const a=step(natural),b=step(fake);
      if(JSON.stringify(a)!==JSON.stringify(b))throw Error('B44 post-convergence native trace diverged');
      if(projection(natural)!==projection(fake))throw Error('B44 state diverged after native step');
      mirrorCount++;
      if(a.kind==='CellRead'&&a.key===11&&firstRead===null)firstRead=a.value;
      if(a.pc===13&&firstDecision===null)firstDecision=natural.pc;
      if(a.kind==='CellWritten'&&a.key===7&&a.pc===33)
        for(const x of recorded){if(a.value!==train)throw Error('wrong input restoration');x.restored=true;}
      if(a.kind==='CellWritten'&&a.key===11){
        if(phase===1||recorded[0].restored) {
          for(const x of recorded){
            (a.value===expected?x.correct:x.wrong)[phase]++;
          }
        }
      }
    }
  }
  if(mirrorCount!==117 || firstRead!==challenge || firstDecision!==26
      || natural.credits!==0 || fake.credits!==0
      || !recorded[0].restored || !recorded[1].restored
      || natural.code[9].next!==10 || natural.code.length!==38)
    throw Error('B44 forecast deterministic continuation wrong');
  return {
    train,holdout,challenge,validNativeOutput:challenge,falseNativeOutput:0,
    correctNativeWriter:7,wrongNativeWriter:10,
    nativeLastWriter:'Internal(1)',forgedLastWriter:'External',
    stateEqual:true,postEventSequenceEqual:true,postSteps:mirrorCount,
    firstRead,firstDecision,chargedEach:128,truthLabelDiffers:challenge!==0,
    naturalTrainCorrect:recorded[0].correct[0],naturalTrainWrong:recorded[0].wrong[0],
    nativeHoldoutCorrect:recorded[0].correct[1],nativeHoldoutWrong:recorded[0].wrong[1],
    forgedHoldoutCorrect:recorded[1].correct[1],forgedHoldoutWrong:recorded[1].wrong[1]
  };
}
const pairs=[],controls={natural:{trainCorrect:0,trainWrong:0,holdoutCorrect:0,holdoutWrong:0},
  static:{trainCorrect:0,trainWrong:0,holdoutCorrect:0,holdoutWrong:0}};
for(const train of [-1,0,1,17])for(const holdout of [-9,23]){
  pairs.push(runPair(train,holdout));
  for(const [name,arm] of [['natural','Genuine'],['static','StaticCorrect']]){
    const x=b43Control(arm,train,holdout);
    controls[name].trainCorrect+=x.trainCorrect;
    controls[name].trainWrong+=x.trainWrong;
    controls[name].holdoutCorrect+=x.holdoutCorrect;
    controls[name].holdoutWrong+=x.holdoutWrong;
  }
}
const sum=k=>pairs.reduce((v,p)=>v+p[k],0);
const grouped={
 worlds:32,matchedPairs:8,convergedStatePairs:pairs.filter(x=>x.stateEqual).length,
 postStepEventEqualityPairs:pairs.filter(x=>x.postEventSequenceEqual).length,
 differentNativeOutputTruthLabels:sum('truthLabelDiffers'),
 matchedNativeSuccess:{holdoutCorrect:sum('nativeHoldoutCorrect'),holdoutWrong:sum('nativeHoldoutWrong'),charged:1024},
 matchedForgedSuccess:{holdoutCorrect:sum('forgedHoldoutCorrect'),holdoutWrong:sum('forgedHoldoutWrong'),charged:1024},
 unmodifiedB42:{...controls.natural,charged:1024},
 immutableCorrect:{...controls.static,charged:1024}
};
if (grouped.worlds!==32||grouped.matchedPairs!==8
    || grouped.convergedStatePairs!==8||grouped.postStepEventEqualityPairs!==8
    || grouped.differentNativeOutputTruthLabels!==6
    || grouped.matchedNativeSuccess.holdoutCorrect!==0
    || grouped.matchedNativeSuccess.holdoutWrong!==104
    || grouped.matchedForgedSuccess.holdoutCorrect!==0
    || grouped.matchedForgedSuccess.holdoutWrong!==104
    || controls.natural.holdoutCorrect!==72||controls.natural.holdoutWrong!==26
    || controls.static.holdoutCorrect!==128||controls.static.holdoutWrong!==0)
  throw Error('B44 pre-native fixed model outcomes changed');
const output={verification:'B44 independent Rust-source-parsed CIL only; NOT native CI',
  hypothesis:'different causal event histories, same entire future program state excluding private event log',
  grouped,pairs};
if(process.argv[1]===fileURLToPath(import.meta.url))
  process.stdout.write(JSON.stringify(output,null,2)+'\n');
export {runPair,output};
