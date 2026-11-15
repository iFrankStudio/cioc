# CL-1B / B15b — statistical unit correction (post-hoc)

**Status:** methodological correction to the already committed B15 independent simplified-CIL audit; **not a new prospective success**, not native Rust CI, and not evidence of life. Keep the original 350-world /175-pair results intact; this note changes the *interpretation of independent experimental units*.

## Correct denominator

B15 used five input labels `[-13,0,7,23,101]` for each executable-next-edge lesion. Although each world is deterministic, the five input values are **not guaranteed to follow the same control-flow sequence** under arbitrary code lesions: damaged paths can reuse an input-bearing register as an instruction address or branch operand. The five labels are fixed experimental conditions, **not five independently sampled structural mutation events**. The original B15 service-output win/loss/tie category happens to repeat for each edge across the five inputs, but some intermediate policy execution paths do **not**.

Therefore distinguish:

- **350 executed/simulated worlds**, 175 paired input×lesion comparisons, measuring reproducibility across inputs.
- **35 distinct structural successor categories**, each with five repeated input labels.
- **32 distinct holdout successor edges**, not 160 independent lesion mechanisms.

The main B15 holdout statistic should be stated as **0 better / 2 worse / 30 equal among 32 distinct holdout edge types**, with the same *service-count effect* across the five tested input labels (but not always the same internal trajectory). Losses are edge13 (2 fewer service writes per input) and edge15 (1 fewer). There are no holdout wins. The previous `0 / 10 / 150 out of 160` figure is an accurate *condition-count* but is **not an independent-sample effect estimate**. No confidence interval or p-value can treat the five replicas as independent evidence.

For the two development edges: next10 improves by4 service outputs, next11 worsens by1; each relation repeats across five input labels. Healthy next7 is unchanged. Total across **35 unique** edges for one input is +4−1−2−1=0, and repeats five times to the originally reported +15 development/−15 holdout cancellation.

## Additional post-hoc policy-reach stratification

Re-reading the **actual source-parsed B15 interpreter** and counting native-like PC22 `PatchConst` executions, the B14 policy switch is reached in **53 of 160 input×holdout comparisons**, spanning **13 distinct holdout successor edges**. The remaining **107 comparisons** do not execute that policy opcode. Outcomes:

| Stratum | Input×edge comparisons | Adaptive better | Adaptive worse | Equal |
| --- | ---: | ---: | ---: | ---: |
| Policy opcode reached | **53** | **0** | **10** | **43** |
| Policy opcode not reached | **107** | **0** | **0** | **107** |

The 13 distinct policy-reachable edge types are `13,15,17,20,21,22,23,24,25,27,29,30,31`. For inputs −13,0,7,101, only the first **10** of these are policy-reachable; for input23, all **13** are. This difference establishes that **input-dependent trajectories exist**, correcting the earlier overly broad claim of identical control flow.

Consequently, the most informative restricted descriptive result is **0 improvements and 2 impaired edge types among 13 policy-reachable holdout types** (or 0/10/43 on 53 concrete input×edge trials). Even in the “policy reached” stratum, a code edit may fail to influence a meaningful output, so these are *not* 13 successful functional trials. Stratification is post-hoc, and should not be used for confirmatory significance tests.

## Stronger causality check: the candidate policy must be **changed and used**

PC22 merely being executed does not establish any change to the **future candidate generator**. With malformed control flow, its operand registers may point at another literal, or `PatchConst` may write a value already present. The B15 source-parsed observer reanalysis therefore distinguishes three nested states and separately counts executions of mutated candidate node29:

| Holdout stratum | Input×edge conditions | Adaptive better | Adaptive worse | Equal | Distinct edge types |
| --- | ---: | ---: | ---: | ---: | ---: |
| Policy opcode never reached | 107 | 0 | 0 | 107 | not independently partitionable by edge, since activity varies with input |
| PC22 reached but candidate policy **not changed and used** | 22 | 0 | 0 | 22 | — |
| Node29 candidate step **actually changed and subsequently executed** | **31** | **0** | **10** | **21** | **7** |

The seven distinct edge lesions whose source-path simulations actually modify **and subsequently use** node29 are `13,15,17,20,22,24,30`. All ten service losses are on lesions13/15 across the five inputs. The `31` conditions are **not** 31 independently sampled mutation mechanisms; they cover only seven structural sites and are correlated by shared B14 source and researcher-authored rule.

The native B15 test was amended to compare PC29's **real before/after literal during each charged VM step**, and to check a future execution of PC29 with that changed value. This is considerably stronger than checking that PC22 ran, but it is **not yet a verified Rust pass**.

## Scope correction

The other 30 holdout tie cases are **not proof of functional equivalence** or robust generalization. Some lesions bypass the diagnostic policy entirely or trap before it can be evaluated; other ties may reflect both arms failing. Hence the absence of holdout wins should be interpreted as:

> *Under this finite observer-specified one-edge universe and 120-instruction budget, the authored 2→1 policy switch shows **no demonstrated out-of-development benefit**. Two distinct unseen edge types do worse, including within the subset that executes the candidate-policy opcode.*

It is not a statistical estimate of performance on all possible digital environments, nor a proof that every adaptation policy fails. A next experiment should stratify by **policy reached / policy change effective / service possible**, report traps and censored trials, and require held-out *functional* evidence rather than blindly comparing all 32 graph edges as though equally viable. This is an explicitly **post-hoc audit stratification**, not a newly preregistered discovery.

## Evidence status

The independent simulation source is [B15 scanner](../analysis/b15_policy_generalization.mjs); the [full Rust differential](../../cl0/crates/world/tests/cl1b_b15_policy_generalization.rs) remains awaiting a verifiable native execution artifact. This correction does not convert simulation to Rust findings or preordain a scientific CL-1B stage gate.
