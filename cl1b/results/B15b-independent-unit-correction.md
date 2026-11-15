# CL-1B / B15b — statistical unit correction (post-hoc)

**Status:** methodological correction to the already committed B15 independent simplified-CIL audit; **not a new prospective success**, not native Rust CI, and not evidence of life. Keep the original 350-world /175-pair results intact; this note changes the *interpretation of independent experimental units*.

## Correct denominator

B15 used five input labels `[-13,0,7,23,101]` for each executable-next-edge lesion. Because the frozen source program only checks the output by equality and does not branch on the numerical input value, the five labels generate the **same control-flow sequence** for the same lesion and policy. They are *deterministic consistency checks*, **not five independently sampled structural mutation events**.

Therefore distinguish:

- **350 executed/simulated worlds**, 175 paired input×lesion comparisons, measuring reproducibility across inputs.
- **35 distinct structural successor categories**, each with five repeated input labels.
- **32 distinct holdout successor edges**, not 160 independent lesion mechanisms.

The main B15 holdout statistic should be stated as **0 better / 2 worse / 30 equal among 32 distinct holdout edge types**, repeated with identical outcome across five input labels. Losses are edge13 (2 fewer service writes per input) and edge15 (1 fewer). There are no holdout wins. The previous `0 / 10 / 150 out of 160` figure is an accurate *condition-count* but is **not an independent-sample effect estimate**. No confidence interval or p-value can treat the five replicas as independent evidence.

For the two development edges: next10 improves by4 service outputs, next11 worsens by1; each relation repeats across five input labels. Healthy next7 is unchanged. Total across **35 unique** edges for one input is +4−1−2−1=0, and repeats five times to the originally reported +15 development/−15 holdout cancellation.

## Scope correction

The other 30 holdout tie cases are **not proof of functional equivalence** or robust generalization. Some lesions bypass the diagnostic policy entirely or trap before it can be evaluated; other ties may reflect both arms failing. Hence the absence of holdout wins should be interpreted as:

> *Under this finite observer-specified one-edge universe and 120-instruction budget, the authored 2→1 policy switch shows **no demonstrated out-of-development benefit**. Two distinct unseen edge types do worse.*

It is not a statistical estimate of performance on all possible digital environments, nor a proof that every adaptation policy fails. A next experiment should stratify by **policy reached / policy change effective / service possible**, report traps and censored trials, and require held-out *functional* evidence rather than blindly comparing all 32 graph edges as though equally viable. This is an explicitly **post-hoc audit stratification**, not a newly preregistered discovery.

## Evidence status

The independent simulation source is [B15 scanner](../analysis/b15_policy_generalization.mjs); the [full Rust differential](../../cl0/crates/world/tests/cl1b_b15_policy_generalization.rs) remains awaiting a verifiable native execution artifact. This correction does not convert simulation to Rust findings or preordain a scientific CL-1B stage gate.
