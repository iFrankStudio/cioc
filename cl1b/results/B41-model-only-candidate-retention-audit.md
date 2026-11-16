# CL-1B B41 — independent model checkpoint (native CI not yet verified)

**Evidence status: source-parsed CIL interpreter only.** The published Rust `Op` graph is the source of the independent JavaScript execution, but executing the JS model is **not** Rust native CI. Until an exact-commit native success is inspected, the scientific-native result remains **UNVERIFIED**.

[Protocol](../studies/B41-native-candidate-feedback-retention.md) · [native test](../../cl0/crates/world/tests/cl1b_b41_candidate_feedback_retention.rs) · [independent model](../analysis/b41_candidate_feedback_retention_scan.mjs).

## Model-only 48-world matrix

| Arm (12 worlds each) | Training correct / wrong | Holdout correct / wrong | Total CIL charges | Halts |
| --- | ---: | ---: | ---: | ---: |
| FEEDBACK | 68 / 4 | 91 / 21 | 1,152 | 0 |
| ASSUME_PASS | 56 / 28 | 54 / 54 | 1,152 | 0 |
| FORCE_FALLBACK | 16 / 8 | 0 / 0 | 420 | 12 |
| STATIC_CORRECT | 144 / 0 | 144 / 0 | 1,152 | 0 |

The twelve FEEDBACK worlds include **eight informative-training worlds** (two candidate orders × training17/23 × two holdouts), where the actor's own paid functional observation is sufficient in the model to retain the *correct*, already-present candidate15. The remaining four worlds train on zero, two of which have candidate14 first and retain the wrong constant-zero implementation. These two then each produce **0 correct / 9 incorrect** holdout writes. Zero was not hidden or excluded as a degenerate case: the observed output really equals the zero training input, so a single true observation cannot prove cross-input generalization.

For four nonzero-training worlds with candidate14 first, the actor reads real output0, detects mismatch, writes an actual fallback flag at cell22, performs paid PC8 edge modification to candidate15, then changes the selected candidate's own `Write.next` from9 to5. With candidate15 first, the actor retains it without fallback. The same-code ASSUME_PASS and FORCE_FALLBACK comparators expose the role of actual task evidence instead of allowing the observer to choose the winning branch. The four-opcode STATIC_CORRECT remains substantially more compute-efficient than the larger trial controller.

**Temporal limitation:** an input update at a fixed number of paid instructions can intervene between source-read and service-write, causing stale output even after the right candidate was retained. Count those outputs as wrong; do not treat the final edge alone as an output success witness.

**Cost limitation:** forced-rejection programs stop early, spending 420 charges total; they do not have the same 96 actual charged instructions as continuing programs, even though each starts with the same96 credit. Reporting zero output without this denominator would be misleading.

**Disposition:** model-generated evidence of a possible paid native candidate comparison plus an important zero-alias overfitting counterexample. It is **not** autonomous generation of candidate code, internally invented goals, cross-distribution policy improvement, trustworthy mutable evaluation or cyberlife. **B3/B4: NOT MET.**
