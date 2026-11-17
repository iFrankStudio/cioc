# CL-1B B50 — native-verified: forced candidate construction beats real feedback in an authored task

**H50 feedback-necessity gate: REJECTED.** This negative result is source-pinned, fully native, and independently re-executed at the **per-world aggregate** level. It is **not** a discovery of cyberlife, a claim that all feedback is useless, or evidence for CL-1B B3/B4.

**Exact source:** `245944c30506c59b0724cb23fd0e9f3df82dd01d`. [Full-scope native GitHub Actions run 43997664085](https://github.com/iFrankStudio/cioc/actions/runs/43997664085): **49/49 named Rust integration targets PASS, 36/36 independent observers PASS, B31/B34 private native ABI checks PASS; zero failed targets and zero failed observers.** [Original preserved CI summary](https://github.com/iFrankStudio/cioc/blob/cl1b-ci-evidence/run-records/245944c30506c59b0724cb23fd0e9f3df82dd01d-43997664085-1/summary.txt) · [original result and source SHA](https://github.com/iFrankStudio/cioc/blob/cl1b-ci-evidence/run-records/245944c30506c59b0724cb23fd0e9f3df82dd01d-43997664085-1/result.txt).

**Registered before the first B50 native run:** [B50 prospective hypothesis/controls](../studies/B50-paid-feedback-vs-forced-construction.md). Code: [native Rust](../../cl0/crates/world/tests/cl1b_b50_feedback_vs_forced_clone.rs) · [independent JS CIL execution](../analysis/b50_paid_feedback_vs_forced_scan.mjs). Original machine-readable results: [48 native case rows](https://github.com/iFrankStudio/cioc/blob/cl1b-ci-evidence/run-records/245944c30506c59b0724cb23fd0e9f3df82dd01d-43997664085-1/b50_native_cases.jsonl) · [independent exact case replay](https://github.com/iFrankStudio/cioc/blob/cl1b-ci-evidence/run-records/245944c30506c59b0724cb23fd0e9f3df82dd01d-43997664085-1/b50_paid_feedback_vs_forced_scan.json) · [native Rust log](https://github.com/iFrankStudio/cioc/blob/cl1b-ci-evidence/run-records/245944c30506c59b0724cb23fd0e9f3df82dd01d-43997664085-1/cl1b_b50_feedback_vs_forced_clone.log).

## Actual native design, not a selected model-only forecast

- Six **new**, preregistered three-window task sequences, crossed with two entry-program phase shifts and four source policies = **48 native worlds**.
- Each world starts with exactly **144 local CIL credits**, strictly capacity-bounded 16 data cells /80 code nodes /192 storage units. One PID1, no replenishment, extra labor process or host repair.
- Source arms: original B42 TRUE feedback; nonhalting BLIND_NEVER (its comparator always says zero discrepancy); nonhalting FORCED_CLONE (paid source construction and retention without using real output equality); compact STATIC correct copier.
- Every actual actor-origin output cell11 `Write` is judged against the **then-current cell7**, including diagnostic writes and the possibility that the actor earlier modified cell7.
- Native test consumes **6,912 /6,912** scheduled and actually charged CIL steps across all 48 worlds. **Zero** source processes halted early; each of the 8 phase-by-arm cohorts spends exactly **864** native CIL instructions over six worlds. Thus the specific B48 early-stop exposure objection does **not** apply here.
- Separately written Node.js interpreter parses the **actual 38-node Rust B42 founder CIL source**, simulates native execution and construction for all 48 worlds, and matches **48/48** complete per-case aggregate/state projections: correct/wrong by window, charged budget, source edits, clone count, final PC, registers, key cells, route and strict storage. `first_divergence: null`. B50 is not a repeat of B49's full **per-instruction event** independent replay; its audit granularity is **per-world**.

## Verified results

Six worlds in each phase-arm cohort, 144 paid CIL instructions per world. The table counts **only the two subsequent/heldout input windows** (`correct[1]+correct[2]` and `wrong[1]+wrong[2]`); the underlying native JSONL and independent observer preserve the entire first window and all diagnostic writes.

| Phase | Arm | Heldout correct | Heldout wrong | Actual paid | New clones | Source edge edits | Halted |
| --- | --- | ---: | ---: | ---: | ---: | ---: | ---: |
| baseline | TRUE | **91** | 28 | 864 | 5 | 16 | 0 |
| baseline | BLIND_NEVER | 19 | 95 | 864 | 0 | 6 | 0 |
| baseline | **FORCED_CLONE** | **108** | **6** | 864 | 6 | 18 | 0 |
| baseline | STATIC | **144** | **0** | 864 | 0 | 0 | 0 |
| +1 charged entry | TRUE | **91** | 23 | 864 | 5 | 16 | 0 |
| +1 charged entry | BLIND_NEVER | 19 | 95 | 864 | 0 | 6 | 0 |
| +1 charged entry | **FORCED_CLONE** | **108** | **6** | 864 | 6 | 18 | 0 |
| +1 charged entry | STATIC | **134** | **10** | 864 | 0 | 0 | 0 |

First-window scores were also preserved: baseline TRUE **11/10** correct/wrong; BLIND_NEVER **6/30**; FORCED_CLONE **13/5**; STATIC **72/0**. Phase+1 had the same TRUE/BLIND/FORCED first-window scores, but STATIC **66/0**. No diagnostic stage, zero-input alias, stopped-world loss or wrong writes were removed.

## Causal conclusion: specific engineered feedback is not necessary to produce this improvement

The original B42 TRUE arm improves substantially over BLIND_NEVER, but **FORCED_CLONE performs 17 more correct, 22 fewer wrong** in baseline heldout windows, and **17 more correct, 17 fewer wrong** in phase+1 holdouts, without using the feedback equality to select the candidate. The forced policy synthesizes the same investigator-prescribed Write clone, pays for its construction, and retains it even when the trial would otherwise falsely accept a zero-valued broken output. TRUE only produces five clones over six sequences; FORCED produces six. The particularly revealing sequence starting with `-1` makes the internally generated B42 diagnostic value zero, permitting a mistaken early acceptance and preventing TRUE from creating the known-correct candidate.

The 4-op static correct copier remains the highest-correct-output comparator. In phase+1 it also has ten wrong outputs caused by the schedule/input-transition boundary; this is an exposure/phase artifact **included** in the score, not a claim that the static source learns. REAL feedback's superiority over BLIND_NEVER alone was not evidence that feedback-selected adaptation was necessary.

**H50 fails prospectively** (the two conjunctive tests against FORCED_CLONE and STATIC fail). This defeats a narrow interpretation of previous B42/B49 apparent adaptive advantage. It does *not* prove blind source edits generally outperform functional feedback: FORCED_CLONE is a carefully authored, stronger control with a fixed known-good candidate and a changed retry branch; its code design is not matched bit-for-bit to TRUE. The conclusion is restricted to these B42 task programs and holdout sequences.

## Consequence for the next study

**Stop preloading the winning executable candidate into the founder's repair recipe.** A future B51 study should preregister a *task-family transfer* experiment where a continuously running native actor must generate and assess at least two **genuinely distinct, charged, executable** candidate constructions from generic local instructions, with no correct candidate known to its evaluator, followed by newly held-out task families. Require real actor-origin candidate diversity, independent audit, successful retention on new tasks, and comparison with an equal-compute unconditional-construction policy and a strong static task-aware oracle. A no-retention or weak-outcome result still matters.

Resource flux (B18/B19) and functional provenance limitations (B44–B46) remain separate unsolved conditions for an organism-level claim.

**B3 cumulative adaptive discovery: NOT MET. B4 endogenous organization: NOT MET. Cyberlife: NOT ESTABLISHED.**
