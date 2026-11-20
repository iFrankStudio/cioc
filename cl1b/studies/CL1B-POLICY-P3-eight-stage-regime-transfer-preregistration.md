# CL1B-POLICY-P3 — Eight-stage lifespan and regime-transfer preregistration

**Status:** Prospectively frozen design; *not* a native-verified result until the exact-source GitHub Actions run and independent per-opcode replay both succeed.

## Why
P2's source-pinned exhaustive four-stage native experiment showed 27 beneficial, 27 harmful, 27 neutral patch contrasts (zero net task probes; −54 net charged instructions across the uniformly weighted goal cube). The eight-stage experiment tests whether a longer lifespan changes actual paid fitness when a real, predictive environmental regularity persists or switches, and where the *researcher-authored* original PID1 has no rollback.

## Frozen intervention and causality
- Use the same original 80-node P2 source with **only the initial pre-load experiment horizon** PC38 literal changed from 4 to 8 by the researcher; **no post-load host source edit**.
- Original CL0 Rust `World::with_strict_capacity(16,180,300)`; each world has exactly one uninterrupted PID1, compute credits1600. Task candidate Write11 by PID, external feedback SetCell20 strictly afterward, actor's paid Read20, Write34/Read34, and actor-paid executable PatchConst46 after the second successful stage. No free memory, task probe, future goal hint, or mutation.
- Evaluate 27 preregistered sequences (3 initial values ×3 initial deltas ×3 regimes), **two fresh matched original worlds each**, one with actor source patch enabled and one with the effect of that same paid patch suppressed. Also one inverted feedback and one forced-zero-feedback control = **56 worlds**.
- Stable: all8 goals follow a constant mod3 delta. Switch: initial4 goals follow original delta, stage4..7 uses `(delta+1)%3`. Scrambled: the precommitted deterministic schedule in the JSON, not claimed i.i.d.
- Outcomes are explicitly *not* provided in the preregistration. Measure charged instruction savings (suppressed minus enabled), actual task probe savings, genuine completed stages, and ability to reach paid terminal claim.
- Independently replay *every charged CIL instruction* and actual feedback in Python against original native Rust logs. Separate world provenance and source pin. Rust must fail on an unexpected source edit, source overflow, PID restart, false true-feedback completion, or an unaccounted paid instruction.

## Falsifiers and limits
If stable long-run benefit remains absent, the proposed relative-delta rule is not worth retaining for even this restricted stable regime. If persistent source patch damages task service in switched or scrambled cases, the unvalidated one-time mutation is brittle. Source cannot react with a second edit: **P3 does not demonstrate actor-owned regime detection or rollback**. That is the next genuinely new capability to construct (P4), contingent on verified P3 data. This is not an entire integrated I6U task/structural repair organism; do not call the 56 separately created worlds a single lifetime.
