# CL-1B B41 — native paid candidate selection, with real zero-input overfitting

**FULL NATIVE VERIFICATION PASS**: [GitHub Actions run 43831945114](https://github.com/iFrankStudio/cioc/actions/runs/43831945114), exact tested source SHA `f8a363d9c10436df0a135884a740f8d3c3aab35b`. [SHA-pinned summary](https://github.com/iFrankStudio/cioc/blob/cl1b-ci-evidence/run-records/f8a363d9c10436df0a135884a740f8d3c3aab35b-43831945114-1/summary.txt) · [B41 native trace](https://github.com/iFrankStudio/cioc/blob/cl1b-ci-evidence/run-records/f8a363d9c10436df0a135884a740f8d3c3aab35b-43831945114-1/cl1b_b41_candidate_feedback_retention.log) · [source-parsed independent 48-world model](https://github.com/iFrankStudio/cioc/blob/cl1b-ci-evidence/run-records/f8a363d9c10436df0a135884a740f8d3c3aab35b-43831945114-1/b41_candidate_feedback_retention_scan.json).

**Scope:** full **41/41 native Rust integration targets PASS**, **28/28 independent models PASS**, B31/B34 private ABI checks PASS, published result `native_job_result=success`, no omitted native targets. Previous B00–B40 regressions were rechecked. One B41 harmless `dead_code` warning on a debug-only observation field was emitted; it does not affect behavior, but is not a strict warnings-as-errors formatting/lint certification.

## Native source, conditions and causal path

[Study protocol](../studies/B41-native-candidate-feedback-retention.md) · [Rust-native 48-world test](../../cl0/crates/world/tests/cl1b_b41_candidate_feedback_retention.rs) · [independent Rust-source-parsed interpreter](../analysis/b41_candidate_feedback_retention_scan.mjs) · [pre-native model-only checkpoint](B41-model-only-candidate-retention-audit.md).

The same *single* PID1 uses charged CIL `Read` to obtain environment-provided candidate edge addresses in cells20/21. Candidate14 is a constant-zero Writer; candidate15 writes current actual task input from cell7 to output cell11. A genuine output mismatch changes its PC8 routing with native `PatchNext`, after which CIL produces an actual flag22 to distinguish a second failed trial. A genuine match triggers native `ReadNext` and a different `PatchNext` of the selected Write's `next:9→5`, retaining that source within the same process. No host side selection, repair instruction, synthetic fitness signal, or external compute refill occurs. Each world starts with96 credits and uses 48 training then up to48 heldout charged instruction attempts.

Exact design: **two candidate orders × three training inputs × two heldout inputs × four arms = 48 native worlds**. No cases dropped; stopped actors keep their unused compute. Model forecasts were frozen before native CI, and native B41 event traces matched the forecast.

## Exact native aggregate outcomes

| Arm (12 worlds) | Training true / wrong task writes | Heldout true / wrong | Total paid instructions | Halted worlds |
| --- | ---: | ---: | ---: | ---: |
| **FEEDBACK** | **68 / 4** | **91 / 21** | **1,152** | 0 |
| ASSUME_PASS | 56 / 28 | 54 / 54 | 1,152 | 0 |
| FORCE_FALLBACK | 16 / 8 | 0 / 0 | **420** | 12 |
| **STATIC_CORRECT** | **144 / 0** | **144 / 0** | **1,152** | 0 |

Native evidence genuinely supports *a narrow individual-level mechanism*: under nonzero training input17 or23, feedback-selected candidate15 becomes the retained source in both candidate orders (eight native worlds), with a paid fallback in four worlds starting with candidate14. An actual internally originated output read precedes the mismatch decision, and the selected Write's control edge changes only through native instructions. Such a result is stronger than B38's one known preset code repair, but the two alternatives remain externally supplied, the functional task remains known, and the comparison policy remains hand-written.

**Crucial negative control:** when input0 is the only training observation and candidate14 is tried first, it returns the correct value *for that one input*. In the **two** registered heldout worlds (input -9 or17), the actor retains candidate14, writes **0 correct and9 wrong** outputs per world, and never repairs itself despite continuing to execute. A truthful single feedback datum is not a sufficient test of generalized functionality.

**Temporal cost:** even with the correct candidate15 retained, the 48-step external input change may happen between a native input read and its subsequent Write, producing a wrong service result. Code correctness and feedback correctness are not guaranteed atomic. **Resource cost:** the full feedback actor produces substantially fewer true heldout writes than the four-opcode immutable correct copier. FORCE_FALLBACK's 420 total executed charges reflect an early halt, not free 96-step allocation usage.

## Scientific disposition

**Mechanism: verified. Cumulative endogenous adaptation B3: NOT MET. Endogenous organization B4: NOT MET. Cyberlife: NOT ESTABLISHED.** The candidate code, task and objective were seeded externally; a real selection event is not proof of self-generated alternatives, a tamper-proof evaluator, unseen-environment robustness or continuous improvement. This observation redirects subsequent experiments toward multiple internally paid functional probes, their compute/storage tradeoffs and performance under unseen contexts, with a strong static baseline and traceable failure cases. Do not conceal negative trials or reframe externally authored choice as emergent evolution.
