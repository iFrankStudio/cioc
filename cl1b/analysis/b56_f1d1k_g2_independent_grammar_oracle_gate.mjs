import assert from 'node:assert/strict';
import fs from 'node:fs';

// Independent prospective G2 grammar check; never interprets this as native evidence.
const file = process.argv[2] ?? 'cl1b/studies/B56-F1d1k-G2-frozen-64-descriptor-three-trial-meta-oracle-v1.json';
assert.ok(fs.existsSync(file), 'frozen oracle missing: ' + file);
const oracle = JSON.parse(fs.readFileSync(file, 'utf8'));
function execute(words) {
  let acc = 0;
  for (const op of words) {
    if (op === 0) continue;
    if (op === 1 || op === 2) acc = (acc + op) % 3;
    else if (op === 3) acc = 0;
    else throw Error('invalid opcode ' + op);
  }
  return acc;
}
function sequence(initial) {
  const words = [...initial], out = [];
  for (let trial = 0; trial < 3; trial++) {
    out.push(execute(words));
    words[1] = (words[1] + 1) % 4;
  }
  return out;
}
const classes = new Map();
let n = 0;
for (let a = 0; a < 4; a++) for (let b = 0; b < 4; b++) for (let c = 0; c < 4; c++) {
  const descriptor = [a,b,c], modes = sequence(descriptor), key = modes.join('');
  n++;
  classes.set(key, (classes.get(key) ?? 0) + 1);
  if (oracle) {
    const frozen = oracle.exact_full_enumeration.find(x => x.descriptor.join(',') === descriptor.join(','));
    assert.ok(frozen, 'missing descriptor ' + descriptor);
    assert.deepEqual(frozen.proposed_modes_on_three_failures, modes, 'descriptor ' + descriptor);
  }
}
if (oracle) assert.equal(oracle.exact_full_enumeration.length, 64);
assert.equal(n, 64);
assert.equal(classes.size, 19);
assert.equal([...classes.values()].reduce((a,b) => a+b, 0), 64);
for (const item of oracle?.equivalence_classes ?? []) {
  const key = item.three_trial_modes.join('');
  assert.equal(classes.get(key), item.descriptor_count, 'class ' + key);
  assert.deepEqual(sequence(item.example), item.three_trial_modes);
}
if (oracle) assert.equal(oracle.equivalence_classes.length, 19);
const old = new Set(['012','120','201']);
const novel = [...classes.keys()].filter(k => !old.has(k)).sort();
if (oracle) assert.deepEqual(novel, oracle.new_sequence_classes.slice().sort());
assert.equal(novel.length, 16);
assert.deepEqual(sequence([0,3,0]), [0,0,1]);
assert.deepEqual(sequence([2,1,0]), [0,1,0]);
assert.deepEqual(sequence([2,0,3]), [0,0,0]);
for (const bad of [-1,4,99,1.5]) assert.throws(() => execute([0,bad,0]));
console.log(JSON.stringify({study:'B56-F1d1k-G2',level:'abstract oracle only',
  descriptors:n,classes:classes.size,novel:novel.length,
  largest_class:Math.max(...classes.values()),native_verified:false}));
