// B56-F1 founder-origin preflight: executable negative control.
// Run: node cl1b/analysis/b56_f1_founder_origin_gate.mjs
// This is a source-structure falsifier, NOT native CIL evidence.
import fs from 'node:fs';
import assert from 'node:assert/strict';
const b51=fs.readFileSync(new URL('../../cl0/crates/world/tests/cl1b_b51c_paid_search_retention.rs',import.meta.url),'utf8');
const f0=fs.readFileSync(new URL('../../cl0/crates/world/tests/cl1b_b56_f0_paid_monitor_assembly.rs',import.meta.url),'utf8');
const original=b51.slice(b51.indexOf('pub(crate) fn founder()'),b51.indexOf('fn native_kind('));
const assembly=f0.slice(f0.indexOf('fn build_assemble_founder()'),f0.indexOf('fn build_static_founder('));
assert(original.startsWith('pub(crate) fn founder()'));
const inherited={
  output_to_feedback_address:/Op::Write\{address:2,value:0,next:35\}/.test(original),
  address_to_feedback_read:/Op::Const\{dst:2,value:20,next:36\}/.test(original),
  paid_feedback_read:/Op::Read\{dst:3,address:2,next:37\}/.test(original),
  f0_reuses_original:/let mut code=b51c::founder\(\)/.test(assembly),
  f0_does_not_replace_circuit:![2,35,36].some(i=>new RegExp('code\\['+i+'\\]\\s*=').test(assembly))
};
assert(Object.values(inherited).every(Boolean),'Founder changed; rerun structural provenance audit before using this result');
const control={
  // Counterfactual removal breaks the complete three-node chain.
  cut_writer:!(/Op::Write\{address:2,value:0,next:35\}/.test(original.replace('value:0,next:35','value:0,next:30'))),
  cut_read:!(/Op::Read\{dst:3,address:2,next:37\}/.test(original.replace('dst:3,address:2,next:37','dst:3,address:2,next:30')))
};
assert(Object.values(control).every(Boolean));
console.log(JSON.stringify({
 study:'B56-F1',kind:'source_provenance_preflight_not_native',
 inherited,negative_controls:control,
 strict_no_preexisting_monitor_gate_with_unchanged_b51c_founder:false,
 consequence:'F1 must replace or causally disable the inherited training feedback circuit and revalidate original task-search viability before claiming de-novo monitor origin',
 b3_met:false,b4_met:false
},null,2));
