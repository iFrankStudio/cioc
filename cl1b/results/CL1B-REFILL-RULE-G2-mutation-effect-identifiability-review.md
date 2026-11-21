# CL-1B G2 · What the native parameter-edit evidence does and does not identify

**2026-11-21 UTC. Evidence-only methodological review; no new native experiment or validation claim.**

Source baseline at review: [main 340e6e36d1e6e63181d57ee918b5569965efc4e5](https://github.com/iFrankStudio/cioc/commit/340e6e36d1e6e63181d57ee918b5569965efc4e5). Verified native experiment is **not** this documentation commit: [G2 Actions #44917352841](https://github.com/iFrankStudio/cioc/actions/runs/44917352841), original source `9cdbeb5f43b897f3f1ddd19fa0d8fc6c8d31404b`, [native/replay evidence](https://github.com/iFrankStudio/cioc/blob/cl1b-ci-evidence/run-records/9cdbeb5f43b897f3f1ddd19fa0d8fc6c8d31404b-44917352841-1/cl1b_refill_rule_g2_independent_native_replay.json), [frozen preregistration](../studies/CL1B-REFILL-RULE-G2-preregistration.md).

## Identification gap

G2 has four source variants differing only in **PC175 Branch destinations**, but **all four** execute the paid `PC28 PatchConst PC173 <- r3` when reaching that subroutine. Consequently the four-arm cost comparison estimates **different researcher-authored downstream branch policies in the presence of the same mutation machinery**, *not* the marginal causal value of changing the PC173 parameter. The six reported branch reversals compare a live register snapshot against a hypothetical initial constant at one instruction; they are **local counterfactuals**, not whole-world counterfactual outcomes. Later source edits, task probes, monitor arming, and charged execution may diverge.

The 45 new frozen task vectors are new relative to R1/G1 but share authored families; they are not evidence of unrestricted distributional generalization. A cross-source F6 comparison would be even less identifiable.

## Quantitative check of the frozen equal-weight G2 family totals

Recomputed from the independently archived replay's five `per_regime_total_charged_cil` entries (positive means fewer instructions for the named policy):

| Frozen family (9 vectors each) | never minus changed_allow | always minus changed_allow |
|---|---:|---:|
| stationary_terminal_switch | 0 | 0 |
| persistent_drift_two_shifts | -676 | +352 |
| rapid_reversal_pulses | -846 | +792 |
| late_phase_flip | 0 | 0 |
| seeded_noise_impulses | -154 | +349 |
| **Equal-weight total** | **-1,676** | **+1,493** |

Thus never-refill is no more expensive than changed_allow **at the five-family aggregate level**, with strictly lower cost in three families. This is *not* a per-vector dominance claim, and does not prove that mutation itself is harmful. All truthful arms complete the same number of stages, so this comparison does not conceal an observed task-completion deficit. The predeclared primary comparison is unfavorable to changed_allow against never, even though it beats always.

## Strongest next falsifiable causal question (design review, not preregistration)

**Question:** Holding the exact same G2 `changed_allow` CIL source, paid instruction count and environment constant, does the **effect** of the actor's PC28 write to PC173 improve net task cost relative to charging for that same write but retaining PC173's previous value?

An intervention, if separately preregistered before any new holdout is run, would require:

1. Keep the original 180-node program and all CL0 occupancy, permissions and private compute limits. In the *intervention arm only*, at the exact native PC28 `PatchConst` effect boundary, suppress the mutation of target PC173 while still charging the original instruction. Do not suppress PC26 monitor disarm or unrelated edits. Preserve the original operand and target checks, including failures.
2. Use an exact-source no-op/placebo intervention with an unreachable target, and a no-intervention exposure control. Before the first genuinely effective PC28 intervention, require identical PC, registers, cells, credits, source and service history; afterward allow real trajectories to diverge. Fail closed if a target is never exposed; report exposed and unexposed pairs separately.
3. Freeze **fresh, non-overlapping goal vectors and family weights**, exact source SHA, intervention specification, primary signed cost comparison and real task-completion criterion before execution. Preserve previously observed G2 worlds as exploratory only; do not select favorable regimes after outcomes are visible.
4. Publish native Rust JSONL for each arm and independently reconstruct *every paid step*, including PC, registers, cells, credits, source mutation and feedback. Keep deceptive-feedback controls separate from truthful fitness. Report all costs, failed tasks, timeouts and intervention exposures, including zero-effect cases.
5. Compare the **same-source intervention effect** only; label any different-program performance comparison as descriptive. Even a positive causal effect would establish the value of this **researcher-authored** mutator, not spontaneous invention of a new algorithm, genuine long-lived cross-world lineage, or B3/B4.

**Decision:** The current native result already establishes actor-paid executable parameter mutation and downstream consumption. The open question is *whether this mutation produces useful net adaptation*. Do not spend another native campaign merely re-proving the ability to patch a constant; first require an identifiable, preregistered intervention with a meaningful opportunity to amortize editing cost. This document does not authorize or claim that such an experiment has run.
