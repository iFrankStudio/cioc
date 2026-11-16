// CL-1B B43: independent, paid CIL interpretation of the *exact* B42
// Rust founder under laboratory interventions at distinct causal times.
// Host SetCell is external and never scored as a native task-output write.
// This is an independent model prediction, NOT native Rust verification.
import { readFileSync } from 'node:fs';
import { fileURLToPath } from 'node:url';

const src = readFileSync(
  new URL('../../cl0/crates/world/tests/cl1b_b42_self_assembled_probe.rs', import.meta.url), 'utf8'
);
const founderText = src.match(/let mut code = vec!\[([\s\S]*?)\n    \];/)?.[1];
const compactText = src.match(
  /if arm == Arm::StaticCorrect \{\s*return vec!\[([\s\S]*?)\n        \];/
)?.[1];
if (!founderText || !compactText) throw Error('B43 B42 founder could not be parsed');
const parse = raw => [...raw.matchAll(/Op::([A-Za-z]+)\s*\{([^}]*)\}|Op::Halt\b/g)].map(m => {
  const op = { kind: m[1] ?? 'Halt' };
  for (const a of (m[2] ?? '').matchAll(/(\w+)\s*:\s*(-?\d+)/g))
    op[a[1]] = Number(a[2]);
  return op;
});
const founder = parse(founderText), compact = parse(compactText);
if (founder.length !== 38 || compact.length !== 4
    || founder[7].kind !== 'Write' || founder[10].kind !== 'Write'
    || founder[11].kind !== 'Read' || founder[12].kind !== 'Sub'
    || founder[13].kind !== 'Branch' || founder[18].kind !== 'CloneNode'
    || founder[9].next !== 10 || founder[10].value !== 3)
  throw Error('B43 exact imported source shape drifted');

const arms = [
  'Genuine', 'ForgedBeforeRead', 'SameValueBeforeRead',
  'ForgedAfterRead', 'StaticCorrect'
];
function run(arm, train, holdout) {
  const isStatic = arm === 'StaticCorrect';
  const code = (isStatic ? compact : founder).map(op => ({ ...op }));
  const r = [0, 0, 0, 0], cells = new Map([[7, train]]);
  let pc = 0, credits = 128, stopped = false, restored = false;
  let probe = null, firstNativeOutput = null, firstFeedbackRead = null;
  let firstBranchPc = null, externalSpoofs = 0, diagnosticWrites = 0;
  const service = [[0, 0], [0, 0]];
  const reg = i => {
    if (!Number.isInteger(i) || i < 0 || i > 3)
      throw Error('B43 invalid native register ' + i);
    return r[i];
  };
  for (const [phase, expected] of [train, holdout].entries()) {
    if (phase === 1) cells.set(7, expected); // external environment update
    for (let attempt = 0; attempt < 64 && !stopped && credits > 0; attempt++) {
      const at = pc, op = code[pc];
      if (!op) throw Error('B43 native CIL missing executable node ' + pc);
      credits--;
      let evt = { kind: 'Executed', pc: at };
      switch (op.kind) {
        case 'Const': r[op.dst] = op.value; pc = op.next; break;
        case 'Add': r[op.dst] = reg(op.lhs) + reg(op.rhs); pc = op.next; break;
        case 'Sub': r[op.dst] = reg(op.lhs) - reg(op.rhs); pc = op.next; break;
        case 'Read': {
          const key = reg(op.address), value = cells.get(key) ?? 0;
          r[op.dst] = value; pc = op.next;
          evt = { kind: 'CellRead', pc: at, key, value };
          break;
        }
        case 'Write': {
          const key = reg(op.address), value = reg(op.value);
          cells.set(key, value); pc = op.next;
          evt = { kind: 'CellWritten', pc: at, key, value };
          break;
        }
        case 'Branch': pc = reg(op.test) !== 0 ? op.nonzero : op.zero; break;
        case 'CloneNode': {
          const node = reg(op.source);
          if (!code[node] || code.length >= 80)
            throw Error('B43 paid CloneNode impossible in this world');
          const nextIndex = code.length;
          code.push({ ...code[node] }); r[op.dst] = nextIndex; pc = op.next;
          break;
        }
        case 'ReadNext': {
          const node = reg(op.node);
          if (!code[node] || !Object.hasOwn(code[node], 'next'))
            throw Error('B43 invalid native ReadNext');
          r[op.dst] = code[node].next; pc = op.next; break;
        }
        case 'PatchNext': {
          const node = reg(op.node), target = reg(op.target);
          if (!code[node] || !Object.hasOwn(code[node], 'next') || !code[target])
            throw Error('B43 invalid native PatchNext');
          code[node].next = target; pc = op.next; break;
        }
        case 'Halt': stopped = true; break;
        default: throw Error('B43 unsupported native CIL operation ' + op.kind);
      }
      if (phase === 0 && !isStatic) {
        if (evt.kind === 'CellWritten' && evt.key === 7 && at === 7)
          probe = evt.value;
        if (evt.kind === 'CellWritten' && evt.key === 11
            && at === 10 && firstNativeOutput === null) {
          firstNativeOutput = evt.value;
          if (pc !== 11) throw Error('B43 intervention moved from pre-Read interval');
          if (arm === 'ForgedBeforeRead' || arm === 'SameValueBeforeRead') {
            cells.set(11, arm === 'ForgedBeforeRead' ? train + 1 : 0);
            externalSpoofs++;
          }
        }
        if (evt.kind === 'CellRead' && evt.key === 11
            && at === 11 && firstFeedbackRead === null) {
          firstFeedbackRead = evt.value;
          if (arm === 'ForgedAfterRead') {
            cells.set(11, train + 1); // strictly AFTER the value entered CIL reg1
            externalSpoofs++;
          }
        }
        if (evt.kind === 'CellWritten' && evt.key === 7 && at === 33) {
          if (evt.value !== train) throw Error('B43 task input not restored');
          restored = true;
        }
        if (at === 13 && firstBranchPc === null) firstBranchPc = pc;
      }
      if (evt.kind === 'CellWritten' && evt.key === 11) {
        if (phase === 0 && !restored && !isStatic) {
          diagnosticWrites++;
        } else {
          service[phase][Number(evt.value !== expected)]++;
        }
      }
    }
  }
  const router = isStatic ? 3 : code[9].next;
  const wrongAccept = !isStatic && firstBranchPc === 26
    && firstNativeOutput !== probe;
  return {
    arm, train, holdout, probe, firstNativeOutput, firstFeedbackRead,
    firstBranchPc, externalSpoofs, diagnosticWrites,
    trainCorrect: service[0][0], trainWrong: service[0][1],
    holdoutCorrect: service[1][0], holdoutWrong: service[1][1],
    paid: 128 - credits, finalRouter: router, sourceNodes: code.length,
    restored, stopped, wrongAccept, trueNativeClones: Number(code.length === 39)
  };
}

const cases = [], falseAccepts = Object.fromEntries(arms.map(a => [a, 0]));
for (const train of [-1, 0, 1, 17]) for (const holdout of [-9, 23]) {
  const results = arms.map(arm => run(arm, train, holdout));
  cases.push(...results);
  const [natural, forged, echo, late, compact] = results;
  for (const p of results) {
    if (p.paid !== 128 || p.stopped)
      throw Error('B43 unexpected changed CIL compute/stop status');
    if (p.wrongAccept) falseAccepts[p.arm]++;
  }
  for (const p of [natural, forged, echo, late]) {
    if (p.probe !== train + 1 || p.firstNativeOutput !== 0 || !p.restored)
      throw Error('B43 B42 imported founder diverged on paid probe generation');
    if (p.firstBranchPc !== (p.firstFeedbackRead === p.probe ? 26 : 14))
      throw Error('B43 functional decision register mismatched');
  }
  if (forged.firstFeedbackRead !== train + 1 || forged.finalRouter !== 10
      || forged.holdoutCorrect !== 0 || forged.holdoutWrong <= 0
      || forged.trueNativeClones !== 0)
    throw Error('B43 early fake was not causally retained');
  for (const p of [natural, echo, late]) {
    if (p.firstFeedbackRead !== 0 || p.finalRouter !== (train === -1 ? 10 : 38)
      || p.holdoutCorrect !== natural.holdoutCorrect
      || p.holdoutWrong !== natural.holdoutWrong)
      throw Error('B43 matched genuine/echo/late discrepancy');
  }
  if (compact.holdoutCorrect !== 16 || compact.holdoutWrong !== 0)
    throw Error('B43 compact baseline not preserved');
}
if (cases.length !== 40) throw Error('B43 denominator must be exactly 40 worlds');
if (JSON.stringify(falseAccepts) !== JSON.stringify({
  Genuine: 0, ForgedBeforeRead: 6, SameValueBeforeRead: 0,
  ForgedAfterRead: 0, StaticCorrect: 0
})) throw Error('B43 wrong host-spoof false acceptance denominator');
const grouped = Object.fromEntries(arms.map(arm => {
  const xs = cases.filter(c => c.arm === arm);
  const sum = key => xs.reduce((a, x) => a + x[key], 0);
  return [arm, {
    worlds: xs.length, trainCorrect: sum('trainCorrect'), trainWrong: sum('trainWrong'),
    holdoutCorrect: sum('holdoutCorrect'), holdoutWrong: sum('holdoutWrong'),
    paid: sum('paid'), clones: sum('trueNativeClones'),
    falseAccepts: xs.filter(c => c.wrongAccept).length,
    externalInterventions: sum('externalSpoofs')
  }];
}));
const output = { evidence: 'independent B42 Rust-source-parsed CIL prediction, NOT native Rust CI',
  worlds: cases.length, grouped, cases };
if (process.argv[1] === fileURLToPath(import.meta.url))
  process.stdout.write(JSON.stringify(output, null, 2) + '\n');
export { run, output };
