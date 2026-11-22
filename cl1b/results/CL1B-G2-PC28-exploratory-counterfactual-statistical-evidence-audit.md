# CL-1B G2 PC28 exploratory counterfactual: statistical and evidence-level audit

**2026-11-21 UTC — retrospective methodological audit, NOT a new native experiment.**

Source at audit: [main 84956f14](https://github.com/iFrankStudio/cioc/commit/84956f14c303fa71f3812fcec361b9633e3d172e). Original G2 native evidence: [Actions #44917352841](https://github.com/iFrankStudio/cioc/actions/runs/44917352841), source `9cdbeb5f43b897f3f1ddd19fa0d8fc6c8d31404b`, 184 original native worlds / 283,918 independently replayed paid CIL instructions. The **PC28-suppressed worlds are not among those original native worlds**. Their costs come solely from the later [exploratory source-level simulator](../analysis/cl1b_g2_pc28_effect_source_level_exploration.py) and its [CI workflow](../../.github/workflows/cl1b-g2-pc28-source-crosscheck.yml). This audit does not establish whether that workflow ran successfully.

## Verified source-level CI provenance (2026-11-21 UTC)

The previously unconfirmed Python cross-check **did run successfully**: [GitHub Actions #44938518680](https://github.com/iFrankStudio/cioc/actions/runs/44938518680), push event, source SHA `84956f14c303fa71f3812fcec361b9633e3d172e`, run attempt 1, completed **success** at 2026-11-21 19:00:11 UTC. Job `source-level-counterfactual` and step `Run frozen source-level counterfactual` both report success. The [source-level-only artifact](https://github.com/iFrankStudio/cioc/actions/runs/44938518680/artifacts/14720059186) is archived (artifact ID 14720059186, SHA-256 `5c9f95446ee2ef7b78020fb791ee533ba43a3b02434af6f25774205f6fd65cdd`). This closes the *Python execution-status* uncertainty in the earlier audit; its historical wording below is retained to preserve the timeline. The verified result remains **source-level simulation assertions only**. It does **not** verify Rust-native PC28 suppression, per-world independent native replay, a fresh holdout, statistical significance, or self-invented policy. The prior original-native G2 run [#44917352841](https://github.com/iFrankStudio/cioc/actions/runs/44917352841) is a different source and evidence tier.

## What the exploratory result can support

The simulator asserts 45 truth-feedback pairs, five original-native *aggregate* baseline costs summing to **70,281**, and suppressed-minus-active total **+156** paid instructions, or **0.222%** of the active aggregate. The reported pair directions are **4 active wins, 2 active losses, 39 ties**. These are assertions embedded in code; absent a verified execution artifact, they remain *reported exploratory calculations*, not a fresh reproduced measurement. Both branches pay for PC28; the counterfactual suppresses its effect on PC173, not its execution charge.

Only **6/45** pairs have nonzero cost differences. A descriptive exact two-sided sign test conditional on those six nonzero pairs gives **p = 2 × (C(6,4)+C(6,5)+C(6,6))/2^6 = 0.6875** under an equal-sign null. This is **not** a preregistered significance test, does not establish independence of the 45 structured task vectors, and cannot be used to claim a population effect. The positive aggregate +156 may be driven by magnitudes of a few pairs, while the 39 ties dominate exposure. Without the actual per-pair output and native intervention, there is no justified confidence interval for generalization.

## Fail-closed interpretation and verification gaps

1. **The original G2 preregistration did not freeze a PC28-suppression arm.** These 45 already-observed G2 vectors are retrospective exploration, *not fresh holdouts*. Do not relabel them as prospective validation.
2. The simulator checks published native **five-family sums**, not original-native PC/register/cell/credit/source transitions per world. Agreement on sums cannot exclude compensating per-world errors. It also does not independently implement or compare original Rust native traces.
3. The script records `effective` before suppressing the write, counting would-be value changes in the suppressed arm; that counter is an *attempted-change diagnostic*, not evidence that the suppressed executable actually changed.
4. The exploratory difference is a **same-source intervention in a separate source-level model**, not a native causal estimate. No new Rust native holdout, native placebo, independent instruction replay, or CI-pinned intervention artifact is claimed.
5. Even a future positive native same-source effect would validate the **researcher-authored parameter-update rule**, not autonomous invention, reliable adaptation under deceptive feedback, long-lived lineage, or open-ended evolution.

**Scientific decision:** Treat +156 as a weak hypothesis-generating signal. Do not tune or select environments on it. A worthwhile future test must first freeze genuinely new non-overlapping vectors, primary signed total cost and completion criteria, same-source PC28 effect suppression with full instruction charge, no-op placebo, exposed/unexposed accounting, and independent original-native state replay. Negative or zero results are equally informative. This review itself authorizes no experiment and changes no CIL program.

## Independent connector-side reproduction and concentration audit (2026-11-21 UTC)

A separate JavaScript instruction interpreter was executed directly against the current GitHub-fetched, preregistered G2 manifest blob `017d56c65c879f89fef1325dd323f2b54ef0e21f` (45 `G2_changed_allow` truth-feedback arms). It implemented Const/Read/Write/Add/Sub/Branch/PatchConst/PatchNext/Halt, charged every executed instruction including suppressed PC28, delivered feedback after PC3, and checked the 180-PC/16-cell/6,000-instruction bounds. This is an **independent source-level implementation**, NOT original Rust execution, original-native trace replay, or verified GitHub Actions output. All 45 pairs completed 24 stages in both source-level arms. The active five-family costs reproduced the published native aggregate baseline (9,432 / 13,791 / 15,635 / 13,425 / 17,998); this aggregate match is not per-world native validation.

The six nonzero suppressed-minus-active costs were:

| Frozen world (G2_changed_allow) | Suppressed − active paid CIL |
|---|---:|
| persistent_drift_two_shifts_a0_d1 | +3 |
| persistent_drift_two_shifts_a1_d0 | −3 |
| persistent_drift_two_shifts_a2_d1 | +3 |
| rapid_reversal_pulses_a1_d0 | +122 |
| rapid_reversal_pulses_a1_d1 | −89 |
| rapid_reversal_pulses_a2_d0 | +120 |

The other **39/45** pairs were exactly tied in source-level instruction cost. Family-level suppression-minus-active differences: stationary **0**, persistent drift **+3**, rapid reversal **+153**, late flip **0**, noise **0**, totaling **+156**. Thus **153/156 (98.08%) of the signed aggregate** comes from one nine-world regime, with large opposing pair effects inside that regime. This is a concentration diagnostic, not a probability statement or a prospective win. It makes the aggregate +0.222% especially unsuitable as a claim of general adaptive benefit. No environment was added or selected post hoc, and no new holdout was consumed.

**Reproduction limits:** This audit used the GitHub connector's frozen manifest bytes and a separate in-memory JavaScript interpreter. It did not run the repository Python script or the Rust-native intervention; no CI run ID, independent native state-transition comparison, or new experimental source SHA is attributed to this reproduction. The proper next decision must still follow the then-current main/CI state rather than treating these six retrospective cases as a new experiment.

## Retrospective read-exposure stratification (2026-11-22 UTC; source-level only)

After rereading current main and the frozen G2 manifest blob `017d56c65c879f89fef1325dd323f2b54ef0e21f`, a separately executed JavaScript CIL interpreter tracked whether a *changed* PC173 constant was subsequently executed at PC173 in each of the 45 truthful G2_changed_allow worlds. All 45 source-level runs completed 24 stages; suppression retained PC28 instruction charging. **Eight of 45 worlds had a later read of a changed parameter** (one each); **six of those eight** had a nonzero suppressed-minus-active cost difference, and **two had exactly zero difference**. The remaining **37 worlds had no later changed-parameter read and all 37 had zero difference**. Eleven worlds executed PC28 but never later read a changed value, showing why attempted edits are not equivalent to usable adaptive exposure. This cross-tab is retrospective and descriptive; it does not create a new holdout or prove a native causal effect.

The eight read-exposed worlds are persistent_drift_two_shifts_a0_d0 (0), a0_d1 (+3), a1_d0 (−3), a2_d1 (+3), and rapid_reversal_pulses_a1_d0 (+122), a1_d1 (−89), a2_d0 (+120), a2_d1 (0), where parentheses give suppressed-minus-active charged instructions. The **six nonzero differences are confined to read-exposed worlds**, consistent with the hypothesized source-modification path but not sufficient to establish native causality. The existing native G2 replay independently established eight later reads for the active arm; the new 8×2 classification and suppressed-arm costs remain **source-level simulation only**, without native per-instruction suppression traces.

**Scientific implication:** the dominant bottleneck in this frozen 24-stage lifecycle is *opportunity to reuse a paid mutation*, not a demonstrated broadly beneficial update rule. Even among the eight read-exposed cases, the direction is mixed (four active wins, two losses, two ties), and selection on exposure is post-treatment/descriptive rather than a valid preregistered effect estimate. No inference of self-invented algorithm or open-ended evolution follows.
