# CL-1B B50 — does real paid feedback outperform blind forced construction?

**Prospective registration (2026-11-17; before B50 native execution).** B3/B4 remain NOT MET. B49's accepted hypothesis was event-level measurement fidelity, not learning. B47 failed its proposed retention claim, and B48's second sham halted early and did not provide an equal-exposure contrast. B50 tests a new *causal policy* alternative, not a retroactive repair of H48.

## Falsifiable causal question

B42's 38-op researcher-authored founder reads real service output and conditionally clones a generic Write into executable code. TRUE beats a policy that never clones, but that does not show that real feedback is *needed* to retain a good candidate. Would a **blind, unconditional, paid clone-and-retain** actor produce at least as many correct actual native task writes? This null is plausible because the candidate itself is already prescribed by the designer.

**H50 (strong feedback-necessity criterion):** at equal realized native instruction exposure and identical initial resource/storage budgets, TRUE must produce **strictly more correct writes and no more wrong writes** than BOTH nonhalting BLIND_NEVER and nonhalting FORCED_CLONE, on each of the two phase offsets' aggregate heldout windows, while outperforming a compact task-correct static baseline to qualify as engineering efficiency progress. All clauses are conjunctive; a single failure rejects H50. This is an intentionally demanding test, not an acceptance threshold for B3.

**Alternative A50:** FORCED_CLONE ties or exceeds TRUE, particularly on zero-valued diagnostic aliases. If A50 occurs, observed success may be caused by the preauthored candidate, not informative selection by actor-local task feedback. Even H50 success would still be *engineered*, not endogenous evolutionary adaptation.

## Frozen worlds and policies

Six **new** three-window sequences, not chosen after B50 result:
`[0,2,-1]`, `[-1,-1,3]`, `[5,0,5]`, `[-7,4,0]`, `[1,1,-2]`, `[19,-3,-1]`.
For each sequence, cross **2** program-phase offsets (no preamble; one charged Const preamble) and **4** source arms (48 distinct native worlds). Preserve **all** windows and actual writes, including the initial diagnostic stage; never discard early losses or stopped worlds.

- **TRUE:** the unmodified B42 `ProbePlus` CIL source.
- **BLIND_NEVER:** identical graph except node12 is `Sub {dst:1,lhs:0,rhs:0,next:13}` (always reports equality); feedback read still costs a CIL instruction.
- **FORCED_CLONE:** identical source except node12 is `Const {dst:1,value:1,next:13}` (same charged opcode opportunity, ignores read value), and node16's retry-marker `Branch` continues to node26 instead of halting on the second mismatch. It pays for construction, two trials, and retaining the synthesized clone on every viable first diagnostic. Neither feedback equality nor actual output determines retention. **This is an engineered alternative policy, not an independent learned solution**.
- **STATIC:** same B42 compact 4-op correct copier, no executable edits or diagnostic work.

All arms: one PID1, 144 initial local CIL instruction credits, 16 cells, <=80 code nodes, <=192 strict storage units, no compute replenishment, no other actor, no special life/fitness/reward opcode. Program-phase shift changes only the charged entry preamble, not the task. Externally set input cell7 at each window; attribute actor output only to native `CellWritten(key=11)` events. Score correct/wrong against **cell7 at the instant of that output Write**, including diagnostics, the same *raw* criterion as B49. No arbitrary time/phase censoring. Enforce exact resource conservation, process origin, storage caps, 48 planned worlds and full per-world paid exposure.

## Independent prediction and CI evidence

1. Source-pinned native Rust target `cl1b_b50_feedback_vs_forced_clone.rs` must emit one case-end JSONL row per actual world in `cl0/cl1b-ci-evidence/b50_native_cases.jsonl`, with source SHA, arm, task sequence, offset, actual charged instructions, correct/wrong arrays, edit/clone counts, halt state, peak storage and final code route.
2. Independently authored Node.js CIL interpreter `cl1b/analysis/b50_paid_feedback_vs_forced_scan.mjs` must parse the *actual* B42 Rust founder, re-execute all 48 planned policies under the same CIL subset and full three-window schedule, and compare every native **case aggregate** exactly. It must fail closed on missing/truncated native data, wrong source SHA in CI, unknown opcodes, missing cases, unknown policies or divergent state. B50's source-level prediction is NOT a native result until Rust CI and this independent comparison pass together.
3. Report paid attempts and halts per arm first; equal *starting* credits alone is not sufficient. Compare TRUE with both nonhalting controls and STATIC separately. Treat source-modification differences in the FORCED policy as a limitation: equal exposure is **not** matched control-flow path or equal design complexity.

## Scientific interpretation rules

- H50 failure is a valid result; do not rewrite the metric, exclude problematic `-1` inputs, move the cutoff, recode FORCED as an invalid arm merely because it wins, or equate observer-only forecast with native execution.
- If FORCED wins, the result is stronger evidence **against feedback necessity in this particular authored task**, not proof that feedback is generally useless.
- Neither H50 nor A50 establishes **actor-origin candidate invention**, multi-task generalization, cumulative unseen adaptation, autonomous goal formation, B3/B4 or cyberlife. A subsequent study must change the generator/retention strategy itself and test true out-of-family tasks.

**Registered status:** PENDING NATIVE + INDEPENDENT REPLAY; B3/B4 NOT MET.
