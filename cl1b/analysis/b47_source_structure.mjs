// B47 independent source-structure audit. Does NOT execute Rust or assert native outcomes.
// Freeze predicted direction before native CI; fail if experiment/control wiring changes.
import fs from 'node:fs';
const s=fs.readFileSync(new URL('../../cl0/crates/world/tests/cl1b_b47_retention_phase.rs',import.meta.url),'utf8');
const checks={
 six_arms:/enum Arm \{ Feedback, Blind, Disabled, Phase, Compact, Oracle \}/.test(s),
 blind_ablates_feedback:/Arm::Blind=>b42::founder\(b42::Arm::IgnoreFeedback\)/.test(s),
 disabled_charges_patch:/with_native_patchnext_disabled\(\)/.test(s),
 phase_adds_paid_instruction:/c\.push\(Op::Const\{dst:3,value:0,next:13\}\)/.test(s),
 immutable_baseline:/Arm::Compact\|Arm::Oracle=>b42::founder\(b42::Arm::StaticCorrect\)/.test(s),
 actual_write_score:/EventKind::CellWritten\{key:11,value\}/.test(s),
 three_regimes:/const SEQUENCES:\[\[i64;3\];8\]/.test(s),
 finite_budget:/Process::new\(1,founder\(arm\),144\)/.test(s),
 event_origin:/assert_eq!\(e.origin,Origin::Internal\(1\)\)/.test(s)
};
console.log(JSON.stringify({study:'B47',kind:'independent source-structure audit, NOT independent dynamic CIL interpreter',checks,forecast:{H47:'FAIL likely',phase:'some timing differences',compact:'higher correct writes per charged instruction'}},null,2));
if(Object.values(checks).some(x=>!x))process.exitCode=1;
