import assert from 'node:assert/strict';
import fs from 'node:fs';

// Structural gate only: no claim about dynamic reachability or native execution.
const path = process.argv[2] ?? 'cl1b/studies/B56-F1d1k-G1-frozen-exact-178-node-source-v1.json';
const manifest = JSON.parse(fs.readFileSync(path, 'utf8'));
const nodes = manifest.code;
const cap = manifest.resources;
assert.equal(nodes.length, 178, 'G1 frozen founder changed');
assert.equal(cap.max_native_nodes, 180);
assert.equal(cap.max_cells, 16);
assert.equal(cap.max_total_storage, 300);
assert.equal(cap.one_PID_only, true);
const edges = n => n.kind === 'Branch' ? [n.zero,n.nonzero] : n.kind === 'Halt' ? [] : [n.next];
for (const [i,n] of nodes.entries()) for (const target of edges(n)) {
  assert.ok(Number.isInteger(target) && target >= 0 && target < nodes.length,
    'invalid control edge ' + i + '->' + target);
}
const seen = new Set([0]), stack = [0];
while (stack.length) for (const t of edges(nodes[stack.pop()])) if (!seen.has(t)) { seen.add(t); stack.push(t); }
assert.equal(seen.size, nodes.length, 'unreachable frozen source nodes changed');
const keys = Object.values(manifest.keys);
assert.equal(new Set(keys).size, keys.length, 'named data keys alias');
assert.equal(keys.length, 14, 'G1 key inventory changed');
assert.equal(nodes.filter(n=>n.kind==='Halt').length, 1);
const meta = [manifest.keys.meta_rule_first_opcode,manifest.keys.meta_rule_second_opcode];
assert.deepEqual(meta,[62,63]);
const availableSource = cap.max_native_nodes - nodes.length;
const availableKeys = cap.max_cells - keys.length;
assert.equal(availableSource,2);
assert.equal(availableKeys,2);
// G2 requires three separately addressable persistent words, hence at least one new key.
// The remaining source budget is NOT a proof of impossibility: source refactoring is allowed.
const result = {study:'B56-F1d1k-G2',level:'static G1 graph inventory only',
 source_nodes:nodes.length,graph_reachable_nodes:seen.size,source_slack:availableSource,
 named_data_keys:keys.length,data_key_slack:availableKeys,
 third_meta_word_requires_new_key:true,refactoring_feasible:'not determined',
 native_verified:false};
console.log(JSON.stringify(result));
