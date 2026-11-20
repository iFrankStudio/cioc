# CL1B-POLICY-P2 — Original-native paid relative-delta search-policy edit and finite no-free-lunch audit

**Status: NATIVE VERIFIED, EXACT SOURCE-PINNED (2026-11-20 UTC).**
[GitHub Actions #44698165454 — SUCCESS](https://github.com/iFrankStudio/cioc/actions/runs/44698165454), exact `main` source SHA **`dba518f6ad13bd477c2a5b3be818296cf3fff46b`**. This workflow actually executed **92/92 registered Rust native integration targets**, passed **63/63 existing named independent observers**, with **zero failures**. The additional P2 independent observer re-executed **40,735 original paid CIL instructions across 168 native one-PID1 worlds** (81 complete original-world enabled/disabled pairs + six preregistered causal controls), with **first native divergence null**.

- [Prospectively frozen complete 80-node original CIL and all81 paired source-only predictions](../studies/CL1B-POLICY-P2-frozen-80-node-actor-paid-relative-delta-search-policy-v1.json)
- [Native original-PID1 Rust experiment](../../cl0/crates/world/tests/cl1b_policy_p2_relative_delta_original_native.rs)
- [Independently authored 80-node original-source CIL interpreter](../analysis/cl1b_policy_p2_independent_native_relative_delta_replay.py)
- [Exact-SHA original-native per-charged-step replay / first divergence null](https://github.com/iFrankStudio/cioc/blob/cl1b-ci-evidence/run-records/dba518f6ad13bd477c2a5b3be818296cf3fff46b-44698165454-1/cl1b_policy_p2_independent_native_relative_delta_replay.json)
- [Complete paired native summary and six controls](https://github.com/iFrankStudio/cioc/blob/cl1b-ci-evidence/run-records/dba518f6ad13bd477c2a5b3be818296cf3fff46b-44698165454-1/cl1b_policy_p2_native_relative_policy_complete_cube.json)
- [Published CI source pin and result](https://github.com/iFrankStudio/cioc/blob/cl1b-ci-evidence/run-records/dba518f6ad13bd477c2a5b3be818296cf3fff46b-44698165454-1/result.txt)

## Observed algorithm modification — different from task-answer patching

This is a *small, orthogonal policy mechanism probe* (not yet integrated into 180-node I6U). An original PID1 executes 80 frozen CIL nodes inside the unchanged `World::with_strict_capacity(16,180,300)`, with private budget800. The process never restarts within a world; independent goal worlds do **not** constitute a continuous cross-world lineage. Original PID1 actually pays to `Write11` a task candidate, the environment sends `SetCell20` only **after** that event, and the same original process pays `Read20`.

Unlike P0 (edits the executable current answer) and P1 (copies the absolute second successful answer into the future-search stride), P2:

1. After genuine source-paid first successful task, pays to `Write34` its **previously successful guess** as its own in-world memory.
2. At the second earned successful task, pays to `Read34`, subtracts that remembered successful guess from the current successful one, and executes a bounded signed-`mod3` normalization using source CIL branches and arithmetic.
3. Pays to `PatchConst` **its own executable search-policy transition step** at PC46 using this derived relative delta (not the task output `Const`). Full source remains identical except that original actor-owned Const field. The new stride controls future candidate generation across the remaining stages; no host source edits or new PID are permitted.
4. Pays for future task services and feedback, counts failures, stops after the fourth learned task. The original source, register state, private credits, data-cell presence and contents, memory provenance and external feedback bits were independently reconstructed after **every charged opcode**, not guessed from aggregate success.

## Exact complete finite native counterfactual

For all `3^4=81` possible four-goal sequences from `{0,1,2}`, the same frozen original source receives the same task sequence in two fresh CL0 worlds: original source editing **enabled**, and the identical source with the effect of `PatchConst` **suppressed**. Both still pay for the attempted patch and the same initial observations. All **162/162 original native worlds** truthfully complete all four task goals.

| Paired native metric | Verified result |
| --- | ---: |
| Policies where actor-paid edit saves compute | **27** |
| Policies where the same edit costs compute | **27** |
| Policies where edit has zero causal difference | **27** |
| **Net paid CIL saving across all81 comparisons (off minus on)** | **−54 instructions** |
| Net actual task service probes avoided | **0** |
| Original paid instructions, 162 paired worlds | **39,636** |
| Additional six native causal/feedback controls | **1,099** |
| Total independently replayed original paid CIL instructions | **40,735** |
| First native trace divergence | **none** |

Concrete counterfactuals, **both genuinely native**:

- Cyclic `[0,2,1,0]`: source-edit-on **174** paid instructions vs source-edit-suppressed **221**, **47 saved**, actor earns source stride2 at its own paid patch. The actor never edits a task-answer source literal.
- Cyclic `[1,0,2,1]`: **197 vs242**, **45 saved**. Relative-delta normalization accounts for a nonzero first goal.
- Constant `[0,0,0,0]`: **198 vs296**, **98 saved**, source stride0 after paid two-success calibration.
- Noncyclic `[0,0,2,0]`: source patch **271 vs221**, **50 extra** instructions spent.
- Noncyclic `[0,2,0,1]`: source patch **273 vs172**, **101 extra** instructions spent.
- Inverted-feedback control: actor writes a positive completion claim after fewer than four truthful matches; external reward remains a **causal signal, not objective truth**.
- Forced-zero-reward control: original PID1 fails fast without claiming four tasks complete.

## Absolute cost of memory and calibration: not merely relative patch benefit

[POLICY-P1 complete native 81-pair holdout #44693154809](https://github.com/iFrankStudio/cioc/actions/runs/44693154809) confirmed that copying the second absolute successful goal into PC46 helps27, harms27, ties27, with net paid gain **−108** and zero net service probe savings.

P2 improves the *incremental causal patch effect* to −54 rather than −108, **but this does not mean P2 is cheaper**. Relative to the earlier source-pinned original P1 native holdout's 81 patch-enabled arms, P2's corresponding patch-enabled worlds pay **1,917 more original CIL instructions** in total, primarily because P2 explicitly stores earlier success data, reads it again, computes a normalized delta, and executes additional branches. P2 uses one more in-world data cell and 18 more CIL source nodes (80 vs62).

The P2-versus-P1 paired comparison decomposes as **225 instructions saved across nine exactly cyclic goal sequences** but **2,142 additional paid instructions spent across the other 72 sequences**. This is a cross-study exact-source matched native/source-prediction comparison, **not** an example of a single PID competing with another individual inside one world. No improvement in generic task-fitness has been shown.

## Why a guaranteed benefit is impossible in this uniform cube

Conditional on any observed prior goal/feedback history `H`, the next hidden goal `Y` is uniformly distributed over `{0,1,2}` when each of the 81 possible four-goal sequences receives equal weight. A deterministic source-paid candidate policy without extra future information can only choose the **order** in which it tries the three possible outputs. If it tests each distinct output once, the rank of the matching output is uniformly `1,2,3`, hence

`E[task attempts | H] = (1+2+3)/3 = 2`.

A policy which repeats a candidate cannot improve this lower bound. The result applies to each unseen independent stage conditional on the past; it is **not** a proof that adaptation fails in structured environments with predictive correlations. It explains why a causal improvement on a cherry-picked cyclic subset can be exactly balanced by harm on noncyclic or adversarial continuations. Source-mutating computation, sensing and model bookkeeping remain **paid overhead** and cannot be rescued by a universal predictive guarantee in this cube.

Our complete native counterfactual confirms the predicted no-advantage *task-probe* aggregate: **zero net probes saved**. The **−54 net charged-instruction difference** additionally reflects the source program's path-dependent execution overhead. This is a bounded finite-information finding, not an unconditional theorem about all cyberlife.

## Failure audit and clear next falsifier

Before registration, the initial P2 source-only test commit [#44697951404](https://github.com/iFrankStudio/cioc/actions/runs/44697951404) was rejected by the strict CI manifest because the new Rust native integration test had not yet been registered. It did not disprove the frozen actor: the final source-pinned registered P2 native run is [#44698165454 SUCCESS](https://github.com/iFrankStudio/cioc/actions/runs/44698165454), with all test and independent observer counters zero-failure and complete evidence published. The original 80-node founder, all81 frozen paired predictions and adversarial controls were **not rewritten after testing**.

**Established:** original actor retains charged past task evidence, calculates a relative-delta *policy parameter*, and pays to edit an executable instruction that changes subsequent search behavior. Matched native worlds prove both useful and harmful causal effects and independently account for all resources.

**Not established:** the actor did **not invent** its delta formula, search candidate grammar, mutation primitive or update timing. The finite four-task, three-symbol environment is not open-ended digital infrastructure. There is no in-world self-discovered new algorithm, cross-world lineage, robust real-task utility or feedback-truth authentication.

**Priority next experiment:** a longer-lived **single uninterrupted PID1** in preregistered **correlated, decorrelated and abrupt-regime-shift** task sequences, with actor-paid validation of policy benefit, rollback or strategy switching when a supposedly predictive source mutation repeatedly fails. Measure **actual net lifetime compute and truthful service gain after sensing, patch, validation and rollback costs**, against source-identical static and privileged no-cost baselines. For a structural rather than parameter-only advance, examine paid `PatchNext` control-flow mutation or graph synthesis in the *existing* CL0 CIL grammar. Do not claim open-ended policy invention before genuine held-out causal improvement.
