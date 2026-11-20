# CL-1B G3Q-C1-P1-L3-I6U — original-native bounded task-feedback policy under three hidden-goal regimes

**Evidence state: EXACT-SOURCE NATIVE VERIFIED — 2026-11-20 UTC.** [GitHub Actions #44678857111 — SUCCESS](https://github.com/iFrankStudio/cioc/actions/runs/44678857111), exact source SHA `8e2242bb454c44b962e36e13360b6b30d15ef205`. **89/89 registered Rust native targets actually selected and passed; 63/63 existing independent observers passed; zero CI failures**. Additional independent original-CIL source interpreter verified **276,239 individually paid CIL instructions across 195 native one-PID worlds** with **first divergence null**. All original native source edits, PCs, registers, cell presence/value, private compute budget, external Write11→SetCell20 feedback and final actor state match.

Source-pinned records: [native 195-world every-paid-step independent audit](https://github.com/iFrankStudio/cioc/blob/cl1b-ci-evidence/run-records/8e2242bb454c44b962e36e13360b6b30d15ef205-44678857111-1/g3q_c1_p1_l3_i6u_independent_native_three_regimes_replay.json) · [native original 192 worlds +3 causal controls](https://github.com/iFrankStudio/cioc/blob/cl1b-ci-evidence/run-records/8e2242bb454c44b962e36e13360b6b30d15ef205-44678857111-1/b56_f1d1k_g3q_c1_p1_l3_i6u_native_task_policy_shift.json) · [zero-failure summary](https://github.com/iFrankStudio/cioc/blob/cl1b-ci-evidence/run-records/8e2242bb454c44b962e36e13360b6b30d15ef205-44678857111-1/summary.txt) · [final tested SHA and CI result](https://github.com/iFrankStudio/cioc/blob/cl1b-ci-evidence/run-records/8e2242bb454c44b962e36e13360b6b30d15ef205-44678857111-1/result.txt).

## Actual native task-service and source repair coverage

| Prospectively frozen hidden task goal sequence | I6U complete four-stage actor-worlds | Real Write11 attempts | Truthful task outputs | Original actor privately paid CIL instructions |
|---|---:|---:|---:|---:|
| Training `[0,0,2,2]` | **64/64** | 384 | 256 | 83,002 |
| Reversed `[2,2,0,0]` | **64/64** | 455 | 256 | 91,122 |
| Alternating `[1,0,2,1]` | **64/64** | 517 | 256 | 98,239 |
| **Total native original worlds** | **192/192** | **1,356** | **768** | **272,363** |

Three additional native controls bring paid steps to **276,239 over 195 native original worlds**. Across three task regimes the original64 founder genotypes yield **72 initially safe** cases, **96** unsafe cases rescued by one actor-paid executable source-byte edit, and **24** unsafe cases rescued by two actor-paid executable edits. All192 structurally certified with **zero** false certificates/rejections; all192 finish all four actual hidden-goal task stages with genuine source-paid Write11 and external reward feedback. Same unchanged world `World::with_strict_capacity(16,180,300)`, exactly **180 source nodes**, one continuous original PID1 in each native world, peak storage **195/300** and individual paid budget **8000** (maximum observed per original arm **2125**).

Controls establish scope, not general validity: the two source-edit-disabled hard founders fail to structurally certify and perform **zero real task service**. Under deliberately inverted feedback, an original PID1 claims success while only **3/7** real task outputs match externally defined task truth. The reward channel remains a vulnerability and actor self-reports do not certify task truth.

## Which algorithm was actually improved?

I6S reached full structural coverage **64/64**, yet only **60/64** founders completed its original four-goal task sequence. The failing original genotypes `[1,3,0..3]` start with a task candidate3 and fail because the existing increment-to3 wrap-around check does not handle starting at3, allowing output candidates4,5 outside the intended small grammar.

The I6U researcher-authored task policy retains all I6S native original source-repair/certification logic and changes **exactly four existing nodes** in the [frozen I6U original CIL](../studies/B56-F1d1k-G3Q-C1-P1-L3-I6U-frozen-180-node-bounded-task-policy-search-v1.json):

- **PC82:** failed-task attempt budget3→4.
- **PC88:** candidate update `Add`→`Sub` (descending rather than ascending).
- **PC89:** lower-bound sentinel Const3→Const−1.
- **PC92:** sentinel wrap candidate0→2.

No source node added, original PID1 still pays for every source edit, evidence-reset flag and feedback-dependent trial, and real hidden-goal values are not provided directly to the actor.

**Comparative provenance:** the earlier I6S **60/64** result for `[0,0,2,2]` is itself exact-native verified [#44676774431](https://github.com/iFrankStudio/cioc/actions/runs/44676774431). The projected I6S **60/64** baselines for reversed and alternating goal sequences are **original-source interpreter calculations**, not separate I6S native executions. Only I6U's 192/192 is native-measured across three regimes.

## Exhaustiveness is finite and must not be inflated

A separate independent *source-only*, **non-native** exhaustive enumeration evaluated both code variants over all64 initial three-opcode rules and all **81** goal sequences in `{0,1,2}^4` (**5,184** combinations): I6S projected **4,864/5,184**, I6U projected **5,184/5,184**, with candidate per-world privately charged cost ≤2474. This larger figure is a model projection, **not 5,184 native Rust executions**. The actual native evidence is 192 original genotype-by-regime arms plus3 controls.

The I6U four-instruction search change was **researcher-designed after searching 216 bounded scalar configurations on the same 64 training genotypes**, then examined on additional hidden goal sequences before final source freeze. Consequently, this is an **in-sample finite engineering improvement and cross-implementation/native verification**, not a truly blind task-distribution generalization experiment, not an autonomously invented algorithm, not an evolving pedigree across the 195 fresh native worlds and not proof of open-ended cyberlife.

## Next falsifiable problem

Native PID1 still mutates its three-word **meta-rule source** based on its paid structural certification; the human designer, **not PID1**, changed its task-learning search algorithm between I6S and I6U. Next isolate a *task-feedback-caused, source-paid modification to a task-policy executable byte itself*, with edits-disabled, inverted-reward and unchanged-source static controls. Distinguish policy-value adaptation from original self-generation of new repair logic. Test nonstationary hidden goals and external truth separately; a single truthful-looking bit does not authenticate semantic success.
