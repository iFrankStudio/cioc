# CL1B-POLICY-P3 — Original-native eight-stage lifetime policy transfer under stable, switched and scrambled task regimes

**Status: EXACT-SOURCE ORIGINAL-NATIVE VERIFIED (2026-11-20 UTC).** [GitHub Actions #44703650244 — SUCCESS](https://github.com/iFrankStudio/cioc/actions/runs/44703650244), tested exact `main` source SHA **`171bfa1884af8551b7fcd2d899306cfb4739b1de`**. All **93/93 registered original Rust native integration targets PASS**, **63/63 independent named observer analyses PASS**, **0 failures**. The additional independent original-source CIL interpreter replays **25,303 original paid instructions across 56 native one-PID1 worlds, first divergence null**. Source and traces are pinned to one run; no post hoc modification of the frozen regime sequences or original native 80-node founder after results.

## Immutable source and results

- Frozen preregistered eight-stage 54 paired arms + 2 adversity controls: [P3 study JSON](../studies/CL1B-POLICY-P3-frozen-eight-stage-regime-transfer-v1.json), [P3 protocol](../studies/CL1B-POLICY-P3-eight-stage-regime-transfer-preregistration.md).
- Actual original CL0 Rust world test: [cl1b_policy_p3_eight_stage_native.rs](../../cl0/crates/world/tests/cl1b_policy_p3_eight_stage_native.rs).
- Separately implemented original-source per-opcode observer: [cl1b_policy_p3_independent_native_eight_stage_replay.py](../analysis/cl1b_policy_p3_independent_native_eight_stage_replay.py).
- [Exact-source Rust original-native 56-world JSON, paid steps, probes and controls](https://github.com/iFrankStudio/cioc/blob/cl1b-ci-evidence/run-records/171bfa1884af8551b7fcd2d899306cfb4739b1de-44703650244-1/cl1b_policy_p3_native_eight_stage_regime_transfer.json).
- [Exact-source independent original CIL per-charged-instruction replay, first divergence null](https://github.com/iFrankStudio/cioc/blob/cl1b-ci-evidence/run-records/171bfa1884af8551b7fcd2d899306cfb4739b1de-44703650244-1/cl1b_policy_p3_independent_native_eight_stage_replay.json).
- [Exact tested source and CI result](https://github.com/iFrankStudio/cioc/blob/cl1b-ci-evidence/run-records/171bfa1884af8551b7fcd2d899306cfb4739b1de-44703650244-1/result.txt).

## Original world mechanics

All 56 separate experiment worlds use original unchanged `World::with_strict_capacity(16,180,300)` Rust CL0 world physics, **80 original native CIL nodes**, one continuous original PID1 inside each world, initial **1,600 private paid compute credits**. The researcher changed only the *preload* terminal-stage limit at **PC38 from four tasks to eight**, before creating the PID; there is no after-load host patching. The PID itself pays for actual task-service `Write11`, the environment only then externally `SetCell20` to the truth/inverted/forced-zero feedback bit, PID pays `Read20`, pays `Write34/Read34` for its own earlier earned success, calculates normalized `(goal1-goal0) mod 3`, and pays to `PatchConst` executable candidate-stride at **PC46**. Paired match is native program with edit enabled vs same original paid source and edit effect suppressed. It pays the same patch opcode even when effect suppressed. No future goals or true counterfactual fitness are revealed to the actor.

At fixed initial values `a=0..2`, `d=0..2`, **three preregistered task families** of eight hidden goals are enumerated:

- **stable:** `g[i] = (a + i*d) mod 3` for 0..7.
- **switch:** `g[i] = (a+i*d) mod 3` through i=3; after the fourth task, `g[i] = (a+3*d+(i-3)*((d+1) mod 3)) mod 3` for i>=4.
- **scrambled:** first two goals `a, (a+d) mod 3`; for i>=2, `g[i] = (a+(d+1)*[2,0,2,1,0,1][i-2]+(i mod 3)) mod 3`. This is a *precommitted nonarithmetic deterministic schedule* and **must not be presented as a random or independent goal distribution**.

Each family has **9 sequences × 2 matched source-patch arms = 18 original worlds**; **54 truthful-feedback worlds** total, plus **2 controls**: a stable-regime inverted-feedback world and a stable-regime forced-zero-feedback world.

## Actual original-native paired results

Each row sums 9 exact native source-patch off-minus-on matched eight-task comparisons. **Positive savings indicate that actor-paid policy edit is cheaper or uses fewer genuine service attempts than the identical program with the patch effect suppressed.** The totals include all paid calibration, sensing, memory writes/reads and patch instructions; they are not free.

| Frozen task regime | Matched pairs | Net paid instructions saved (off − on) | Net actual task probes saved (off − on) |
| --- | ---: | ---: | ---: |
| Stable eight-stage arithmetic regularity | 9 | **+1,296** | **+54** |
| Four-stage then shifted arithmetic regularity | 9 | **+420** | **+18** |
| Deterministic scrambled continuation | 9 | **−456** | **−18** |
| **Total on this intentionally structured 27-sequence set** | **27** | **+1,260** | **+54** |

All **54/54 truthful original native worlds** (27 patch-enabled +27 suppressed) truthfully complete all eight tasks and emit actor-origin final completion; 54 worlds ×8 = **432 genuinely completed task stages**, though additional failed probes may precede each success. Actual original charged CIL steps in these 54 paired worlds: **24,894**. Inverted-feedback control spends **341** original charged instructions and *claims* eight successes, but has only **2 truthful task matches**. The forced-zero-feedback control spends **68** charged instructions, self-halts and claims failure (with only one incidental truthful match). Native controls together contribute **409** instructions; **25,303 total** replayed charged instructions. All original PC, registers, source PC46, private credits, live data cells, memory and external feedback after every charged step match the separately implemented CIL replayer: **no first divergence**.

## Interpretation, identifiability, and important negative boundaries

1. **Evidence of finite causal policy benefit:** on the nine stable sequences, an original PID1's own paid mutation of an executable search stride produces **1,296 saved** original charged instructions compared with an otherwise identical native source-patch suppression control, with real task attempts reduced by 54. The additional future opportunities amortize the same paid calibration and patch code.
2. **Validated negative transfer:** on frozen scrambled continuations the same source-editable policy loses **456** instructions and causes **18 extra actual probes** compared with suppression. A single favorable cycle is not evidence that the mutation rule is generally beneficial.
3. **Switching cannot be reduced to always harmful:** the deliberately constructed switched sequences still yield +420 aggregate savings; this does *not* prove the actor recognized the shift or chose to change again. It never changed its rule after the stage-two calibration.
4. **No ex-ante regime-identification oracle:** given a,d, all three environments have identical first two goals, and the stable/switch environments share the first four. At the single P2/P3 edit decision the observed history is compatible with all three worlds. Absent prior regime probabilities or additional environment observations, an actor cannot infer the hidden continuation merely from the first two task successes.
5. **Feedback truth is not self-certifying:** inverted environment feedback yields a falsely positive eight-stage actor claim with only two actual task matches. The forced-zero control shows an appropriate negative fail-fast path in the fixed source protocol, not a robust general verifier.
6. **Nonclaims:** the eight-stage limit is researcher-set, the change rule and single source-update location are researcher-authored, the experiment's finite correlated schedules were researcher-selected, and there is no endogenous environment recognition, self-discovered policy-grammar expansion, actor-paid revalidation, source rollback, open-world autonomy, persistent multi-world original PID lineage or demonstration of open-ended cyberlife. The 27 structured tasks **do not overturn** P2's earlier full uniform 81 four-goal no-free-lunch result.

## Next falsifiable engineering target (not achieved here)

**POLICY-P4** should build a genuinely in-world, **cost-aware** confidence or policy revalidation mechanism, with paid actor-derived evidence, paid executable policy rollback or switching after repeated contrary observations, and an equal-horizon, source-identical native comparison. Measure actual net lifetime utility *after* sensors, patching, book-keeping, and verification. A naive first-error reset and an unconditionally paid post-task recalibration both have source-model negative cost projections; do not introduce them as proven improvements. Separate native tests must include useful stable environments, shifted and nonstationary environments, adversarial feedback and source-mutation-suppressed controls, and all original charged instruction replay. Extend past the present eight task horizon if needed for learning cost amortization, while preserving strict CL0 limits and explicitly tracking compute exhaustion.

This result is a bounded individual search-strategy mechanism within the CL-1B investigation; **not CL-1B stage completion**.
