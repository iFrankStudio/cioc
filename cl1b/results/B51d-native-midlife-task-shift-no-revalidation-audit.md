# CL-1B B51d — unchanged retained-source CIL fails after a hidden midlife task shift

**B51d no-revalidation negative falsifier CONFIRMED; full cross-regime transfer NOT MET.** This study does not reauthor the B51c founder graph and must not be interpreted as life, autonomous search-policy invention, or B3/B4 success.

**Additional FULL 52-target source-pinned regression (2026-11-17):** [run44040441081](https://github.com/iFrankStudio/cioc/actions/runs/44040441081), tested SHA `544750393a0315c9f088235efada80c02b1244b0`, changed only a workflow explanatory comment from the previously successful B51d source, **scope=full; 52/52 named Rust native integration targets PASS, 39/39 independent observer programs PASS, B31/B34 ABI PASS, both failure counters zero, evidence publication PASS**. [Original complete full CI summary](https://github.com/iFrankStudio/cioc/blob/cl1b-ci-evidence/run-records/544750393a0315c9f088235efada80c02b1244b0-44040441081-1/summary.txt) · [exact SHA and conclusion](https://github.com/iFrankStudio/cioc/blob/cl1b-ci-evidence/run-records/544750393a0315c9f088235efada80c02b1244b0-44040441081-1/result.txt) · [B51d independent 11,728-step parity in full run](https://github.com/iFrankStudio/cioc/blob/cl1b-ci-evidence/run-records/544750393a0315c9f088235efada80c02b1244b0-44040441081-1/b51d_midlife_regime_replay.json). The original B51d primary [scoped run44039928740](https://github.com/iFrankStudio/cioc/actions/runs/44039928740) remains separately archived; both exact revisions agree in scientific result. **B52 remains preregistered, not executed.**

## Registered test and exact evidence

- The [prospective B51d protocol](../studies/B51d-midlife-regime-shift-and-revalidation.md) was registered *before* implementation and before all B51d native runs.
- **Exact tested source SHA:** `b7c396349f3f1b26bb644fb536841d5498384276`.
- [GitHub Actions run44039928740](https://github.com/iFrankStudio/cioc/actions/runs/44039928740): **scope=changed-only; 5/5 selected Rust native targets PASS** (B49, B50, B51b, B51c, B51d), **39/39 independent observers PASS**, B31/B34 native library ABI PASS; both failure counters zero; evidence publishing success. **This is NOT a full 52/52 native target run**.
- [Original CI summary](https://github.com/iFrankStudio/cioc/blob/cl1b-ci-evidence/run-records/b7c396349f3f1b26bb644fb536841d5498384276-44039928740-1/summary.txt) · [source-identified job result](https://github.com/iFrankStudio/cioc/blob/cl1b-ci-evidence/run-records/b7c396349f3f1b26bb644fb536841d5498384276-44039928740-1/result.txt) · [independent source-parsed event replay](https://github.com/iFrankStudio/cioc/blob/cl1b-ci-evidence/run-records/b7c396349f3f1b26bb644fb536841d5498384276-44039928740-1/b51d_midlife_regime_replay.json) · [native 16-world JSONL](https://github.com/iFrankStudio/cioc/blob/cl1b-ci-evidence/run-records/b7c396349f3f1b26bb644fb536841d5498384276-44039928740-1/b51d_native_steps.jsonl) · [native Rust log](https://github.com/iFrankStudio/cioc/blob/cl1b-ci-evidence/run-records/b7c396349f3f1b26bb644fb536841d5498384276-44039928740-1/cl1b_b51d_midlife_regime_shift.log).
- [Native code](../../cl0/crates/world/tests/cl1b_b51d_midlife_regime_shift.rs) imports and executes precisely the unchanged CIL `founder()` from [B51c Rust source](../../cl0/crates/world/tests/cl1b_b51c_paid_search_retention.rs). The latter gained only `pub(crate)` visibility to permit exact reuse; no CIL op or next edge was modified. [Independent B51d observer](../analysis/b51d_midlife_regime_replay.mjs) reuses [B51c independent CIL instruction semantics](../analysis/b51c_native_paid_search_replay.mjs) without sourcing native outcomes as state inputs.

## Actual negative causal contrast

**Two hidden task transitions:** A `x-2 → -x+3`; B `-x-3 → x+2`. Four arms per transition (`STATIONARY`, `SHIFT_NO_FEEDBACK`, `SHIFT_WITH_FEEDBACK`, `SHIFT_RETURN`), plus two deterministic input-interleaving phases: external input arriving before or after **one actually paid, task-input-unobserving initialization CIL instruction**, preserving exactly the same original code and actual charge opportunities.

One continuous **PID1** per native world, **1,300 local paid credits**, strict 16-cell/120-node/192-storage caps, no compute refill or process reset. It first pays to construct and select the native source using actual B51c two-probe feedback, then executes **24 original-mode writes; 24 changed-mode writes; 24 further changed-mode writes or original-mode writes (RETURN)**. Every matched 4-arm block used the **same actual paid instruction total**: 601 per world for transition A, 865 for transition B, for both phases.

| Arm | Window 1 correct/wrong | Window 2 correct/wrong | Window 3 correct/wrong | Paid instructions across 4 matched worlds |
|---|---:|---:|---:|---:|
| STATIONARY | 96/0 | 96/0 | 96/0 | 2,932 |
| SHIFT_NO_FEEDBACK | 96/0 | **0/96** | **0/96** | 2,932 |
| SHIFT_WITH_FEEDBACK | 96/0 | **0/96** | **0/96** | 2,932 |
| SHIFT_RETURN | 96/0 | **0/96** | 96/0 | 2,932 |

The lab performed **192 extra external-origin feedback-bit writes** after the mode changes in SHIFT_WITH_FEEDBACK. Yet across all **16 continuous native worlds**, **zero CIL `Read` of feedback cell20 after initial candidate retention**, **zero subsequent executable edge patches**, and **zero subsequent clone allocations** occurred. The new task is fully incorrect even though its external feedback is available because the retained CIL writer is redirected directly to a paid input/calculation loop, not to feedback sensing or candidate search.

SHIFT_RETURN's 96 correct outputs after f1's return are only **old-code functional persistence**, not novel re-adaptation. A separate privileged compact per-mode oracle, freshly seeded with the current task parameters on each mode, generated **24/24 correct outputs per native segment** across 12 such baseline segments, requiring144 paid steps per segment. This privileged comparator must not be characterized as a continuous source-blind individual.

## Independent replay and limits

Source-parsed separate JS CIL replay independently reconstructed all **11,728 genuine paid native instructions**, **1,680 external cell interventions**, **32 laboratory-only stage transition records**, and the 12 privileged static comparator summaries in their exact source order. **First divergence = null**; 16/16 worlds with actual PC, registers, code edges, storage, input/feedback and credits agreed. All source-pinned run evidence was published.

**Historical failed run retained:** [run44039764958](https://github.com/iFrankStudio/cioc/actions/runs/44039764958) failed B51d Rust compilation due moving a non-Copy `serde_json::Value` out of an indexed array. The B51d observer correctly failed closed because the native trace did not exist. [Corrected-source run44039928740](https://github.com/iFrankStudio/cioc/actions/runs/44039928740) passed both.

Scientific caveats: exact initial source and recheck omission are deliberately fixed, so the failure is expected but **quantitatively causally verified**, not evidence no other digital individual could recover. The two-phase control uses one **task-input-unobserving paid initialization instruction**, not a literal `NOP` opcode: this is a documented implementation clarification of the frozen intended charged phase-offset control. Task modes belong to one known affine family, and current source has no intrinsic task objective or maintenance rule.

## Next stage

**B52** may add researcher-authored but fully **paid CIL** post-write feedback rechecking plus a CIL-only ability to reopen and reuse the existing candidate grammar after mode shift. Strong native controls must show actual CIL feedback reads and source edits, recovery within the same PID, and evaluate old-task return. The comparison should include `no feedback`, forced-zero/stale-success feedback, compact static oracles and exact source-pinned independent replay. If B52 succeeds it demonstrates a better **engineered closed-loop adaptive controller**, not endogenous invention of that controller; B3/B4 remain NOT MET.

**B3 NOT MET; B4 NOT MET; cyberlife NOT ESTABLISHED.**
