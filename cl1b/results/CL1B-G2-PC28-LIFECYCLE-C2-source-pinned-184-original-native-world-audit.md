# CL1B-G2-PC28-LIFECYCLE-C2 · 48-stage original-native mutation-reuse opportunity test

**2026-11-22 UTC · FULL TARGETED C2 NATIVE + INDEPENDENT ORIGINAL CIL REPLAY SUCCESS.** [Reverified GitHub Actions #44987892896](https://github.com/iFrankStudio/cioc/actions/runs/44987892896), exact tested source SHA **9bbb5767bb2e0b830bc78f65cda0382ed997e5f3**. Two actual Rust integration tests in the registered C1 target (legacy C1 and prospective C2) passed, 0 failed. C2 ran **184/184 original-native independent PID1 worlds, 546,615/546,615 privately charged CIL instructions** and all native JSONL feedback rows were independently re-executed from immutable CIL and frozen goals with **first_divergence=null**. The scoped job uploaded [immutable action artifact #14740681368](https://github.com/iFrankStudio/cioc/actions/runs/44987892896/artifacts/14740681368) (SHA-256 39dc6bf4651c1406390a53018c7d36652990059908a7d464cf31b060e5fed4b7). This is a **targeted 2-test C1/C2 run**, not a claim that 104/104 earlier native targets or 63/63 independent observers were rerun by the new workflow.

Prior evidence: [C1 full original-native 184-world causal result](CL1B-G2-PC28-NATIVE-C1-source-identical-184-world-causal-audit.md) and [C1 verified 19/10/14/2 mutation-use funnel](CL1B-G2-PC28-C1-native-mutation-reuse-window-audit.md). The new C2 [preregistration](../studies/CL1B-G2-PC28-LIFECYCLE-C2-preregistration.md) and [fully frozen 45×48 goal vectors + 184 arms](../studies/CL1B-G2-PC28-LIFECYCLE-C2-frozen-184-worlds-v1.json) were both committed **before the first C2 execution**.

## What changed and what did not

C2 is a controlled 48-stage *workload extension*, not an actor-authored algorithm. Exactly **PC38 Const.value 24→48** is changed, once, in each of the original 180-node G2 on and never program sources; all other CIL nodes match C1's frozen source. The actor does not learn this edit or perform it. All source-identical C2 on/off/placebo worlds start from the **same exact frozen G2_on source**; the host-only intervention in off preserves full PC28 charging and suppresses **only PC28's write effect on PC173**, as preregistered. The unreachable PC180 placebo must leave entire native trace identical to on. G2_never is a separate source and a descriptive strong baseline, not a causal same-source arm.

The original CL0 world physics **16 data cells, 180 occupied source slots, 300 maximum total occupied units, 6000 total private paid CIL instructions, single PID1** and external genuine Write11→feedback20 task service are unchanged. All **180 truthful 48-stage worlds completed 48 actual hidden goals** (8,640 verified true stages). Four deceptive-feedback worlds are retained in the 546,615-instruction replay denominator but are not declared true 48-stage successes.

## Preregistered primary original-native causal result: NEGATIVE

The signed metric is Δ = original paid CIL cost(off: PC28 effect suppressed) − cost(on: PC28 effect allowed), summed over all 45 frozen equally weighted new task cases **after successful true stage completion**. Δ>0 means this researcher-authored self-modification saved computation. The result is **Δ = −2,008 private paid CIL instructions**, **4/45 benefit, 8/45 loss, 33/45 tie**. Hence permitting self-modification on these newly frozen 48-stage workloads costs an additional 2008 privately paid instructions relative to its exact same-source ablation.

| Arm (45 separate true 48-stage worlds) | Total native charged CIL | Evidence relation |
|---|---:|---|
| G2_on: actual paid PC28→PC173 source change | **138,123** | Treatment |
| G2_pc28_suppressed: identical initial source, same PC28 full charge | **136,115** | **Identifying causal control** |
| G2_placebo180: unreachable host suppression | **138,123** | Every native CIL + feedback trace strictly identical to on |
| G2_never: preauthored fixed no-refill | **126,866** | **Different-source** stronger descriptive baseline |

All four are evaluated on the same 45 tasks; the strong fixed baseline is **11,257 CIL cheaper than on**, but this cannot be causally attributed exclusively to PC28, as their initial source differs. C2 total of **546,615 CIL** includes the four deceptive-feedback controls as well; do not mistake this for the sum of truthful arm costs.

## Family breakdown, each 9 originally frozen 48-stage cases

| Pre-frozen family | Actual on | Same-source suppressed off | Different-source never | Δ = off−on | Worlds truly re-reading a changed PC173 |
|---|---:|---:|---:|---:|---:|
| early_double_shift | 27,620 | 26,434 | 24,066 | **−1,186** | 4 |
| eight_stage_reversal | 29,484 | 29,484 | 26,184 | **0** | 0 |
| long_plateau_and_drift | 22,230 | 22,224 | 21,310 | **−6** | 4 |
| short_pulse_cascade | 28,361 | 27,539 | 25,859 | **−822** | 6 |
| seeded_sparse_deviation | 30,428 | 30,434 | 29,447 | **+6** | 2 |
| **45-family equal-weight total** | **138,123** | **136,115** | **126,866** | **−2,008** | **16** |

The family totals above are printed in the **successful original-native CI job logs** (not inferred from a model-only counterfactual) and independently reproduced by a separate source-level interpreter yielding identical 546,615 total native charged instructions and −2,008 aggregate. The 48-stage pre-frozen task families are engineered stress tests, not a random sample of all possible future environments; this result cannot be turned into a population-general no-benefit theorem.

The two largest negative families are **early_double_shift (−1,186)** and **short_pulse_cascade (−822)**. The tiny **seeded_sparse_deviation +6** is real and must be preserved, not thrown out to claim universal harmful mutation. In the short-pulse family, even with six re-reading worlds, four cost-loss cases outweigh two substantial cost-win cases; raw native per-world evidence remains in the immutable artifact.

## What the longer lifetime actually changed

For 45 G2_on worlds: **65 paid PC28 executions**, **48 actual parameter value changes**, **24 later reads of the changed PC173 parameter** across **16 separate worlds**. The suppression arm recorded **63 real PC28-effect suppressions**, which need not equal the on-arm count because the worlds may diverge *after* the first intervention. C1's previous 24-stage workload had only 2/45 later-reuse worlds; this new 48-stage stress workload has 16/45. **This is not a randomized isolated causal estimate of length**, because C1 and C2 tasks and authored PC38 differ. It is valid direct evidence that longer C2 lives permit more execution reuse, while paid use remains net counterproductive in aggregate.

A later read is only an opportunity to alter a downstream paid branch, not a guarantee it changes the branch or improves lifecycle cost. The only identified causal net cost contrast is **within C2, under exact identical source, full PC28 fee, equal true completion and frozen goals**. C1 vs C2 cost differences cannot be used as if they measured length effect in a paired-identical workload.

## Integrity, provenance and scientific decision

- [C2 original-native Rust generator / checker](../../cl0/crates/world/tests/support/cl1b_g2_pc28_c2_native.rs), included under the preexisting registered [C1 target](../../cl0/crates/world/tests/cl1b_g2_pc28_native_c1.rs). It asserts exact 180-source-slot identity except PC38 24→48, real task service, capacity and fee, placebo CIL+feedback trace identity, off prefix equivalence, and fails the predeclared completion gate after retaining the native summary.
- [Full C2 original CIL independent replayer](../audits/cl1b_g2_pc28_c2_independent_native_replay.py), which reconstructs registers, all code and data edits, PC, fee, feedback, storage and every native JSONL row, with first_divergence=null.
- [C2 source-edit reuse and family-cost native summary auditor](../audits/cl1b_g2_pc28_c2_native_reuse_audit.py), which requires every truthful world actually complete 48 true stages without budget exhaustion; it asserts placebo matching and cost identities but does not impose a favorable primary sign.
- [Scoped native CI](../../.github/workflows/cl1b-c2-lifecycle.yml) at [#44987892896](https://github.com/iFrankStudio/cioc/actions/runs/44987892896) prints exact native family totals and archives the full source-pin/provenance/JSONL/independent interpreter audit. **No native physical rule or CIL actor privilege was added**.

**Result:** C2 extends the observed self-edit reuse opportunity (16/45 worlds), but it **falsifies the hypothesis that increased opportunity alone yields net resource-efficient adaptation** in this frozen challenge: on is 2,008 instructions worse than same-source ablation and 11,257 worse than different-source never-refill. The G2 parameter update rule remains researcher-written; feedback still lacks trustworthy provenance. **No actor-invented mutation rule, robust expected-value prediction, general positive net-benefit learning, cross-life lineage or CL1B B3/B4 completion.**

**Next decision:** do *not* continue raising task length or manually tuning thresholds by trial-and-error. Concentrate on a true PID1-payable, feedback-identified **decision to mutate vs not mutate, charged over full life**, with matched information, an eligible abstention route, exact same-source controls, genuinely new frozen tasks, and strong fixed baselines. A good candidate should be able to skip edits when expected remaining paid gain does not exceed expected modification and monitoring costs; hard-coding this conditional rule ex ante is only a new authored controller, **not yet self-invented adaptation**.
