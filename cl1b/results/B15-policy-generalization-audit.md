# CL-1B B15 — complete cross-lesion generalization audit

**Evidence tier:** independent simplified CIL instruction-level scan of the committed [B14 founder](../../cl0/crates/world/tests/cl1b_b14_search_policy_mutation.rs), using the source-parsed [B15 scanner](../analysis/b15_policy_generalization.mjs). **NOT the native Rust VM, not independently confirmed GitHub Actions, not spontaneous evolution.** Protocol was preregistered before computing the following outcomes in [B15](../studies/B15-exhaustive-policy-generalization.md). The [350-world native differential](../../cl0/crates/world/tests/cl1b_b15_policy_generalization.rs) must pass against exact source SHA before promoting this observation to an original Rust result.

## Complete denominator and pairing

Five inputs `[-13,0,7,23,101]` × all 35 possible next edges for node6 × two source variants = **350 simulated worlds, 175 matched pairs**. Of these: five healthy-baseline pairs (next7), ten previously explored development pairs (next10/11), and 160 **holdout** pairs (the other 32 successors). Each starts with 240 credits and executes at most120 charged instructions. All trapped or stopped worlds remain in the denominator, with actual attempted instruction count preserved; no synthetic zero padding.

The two source variants differ in **one initial `Const` literal at PC21**: adaptive sets r3=1 so internal PC22 `PatchConst` changes candidate-step PC29 2→1; no-switch sets r3=2 so that *same charged* `PatchConst` leaves PC29 at2. This is a **human-designed cost-matched counterfactual**, not an individual discovering and choosing between policies.

| Partition | Matched pairs | Adaptive wins | Adaptive loses | Ties | Total valid-service difference (adaptive − no-switch) |
| --- | ---: | ---: | ---: | ---: | ---: |
| Healthy next7 | 5 | 0 | 0 | 5 | 0 |
| Development next10/11 | 10 | 5 | 5 | 0 | **+15** |
| Holdout other 32 edges | **160** | **0** | **10** | **150** | **−15** |
| **All pairs** | **175** | **5** | **15** | **155** | **0** |

**Strong negative outcome:** The native-like policy modification has **no wins in 160 held-out comparisons**, and loses ten times. Every loss corresponds to lesion next13 (two fewer valid outputs) or next15 (one fewer), repeated across all five input labels. Every development success corresponds to next10, and every development loss to next11. The apparently promising B14 next10 local advantage **does not generalize to the other 32 edge lesions** under this fixed evaluation.

## Failure and cost exposure

- Holdout trapped worlds: **8 adaptive / 8 no-switch**. Traps are not silently excluded.
- Holdout stopped worlds (including those trapped): **61 adaptive / 45 no-switch**, with 16 matched pairs executing unequal numbers of attempts before stopping. Never claim equal realized work solely because the budget ceiling is equal.
- Healthy baselines match and produce identical service counts; a code change triggered by failed trials must not be counted as an intrinsic benefit when no failure occurs.
- In a single 120-attempt window, the **total service difference over all 175 pairs is exactly zero**: +15 over reused development cases and −15 over holdouts. Aggregate neutrality masks harmed individual conditions; neither win count nor throughput is a life fitness score.

## What was refuted, and why it matters

The hypothesis **“being able to self-modify a candidate-generation policy makes the individual generally more adaptive”** is not supported. A researcher-hardwired heuristic switch may be useful on one code lesion and harmful elsewhere. B15 is more informative than adding another success case because it completes an explicit finite mutation universe without selecting the favorable source outcomes afterward.

**Interpretation limit:** This is a negative generalization result about a hand-authored CIL program, not a result about arbitrary possible digital organisms. The evaluator remains tied to a mutable success marker, whose false-positive and false-negative cases were already documented in B12/B13. Even if native Rust confirms all 350 worlds, that would validate a **mechanistic counterexample**, not autonomously originated evolution.

## Next evidence gate

Retrieve source-SHA-pinned native CI evidence from the isolated `cl1b-ci-evidence` branch or Actions artifacts. Then replace the internally mutable but *researcher-specified* policy switch with a regime in which one continuing CIL actor can evaluate *multiple potential policies* using independent, paid, causally grounded observations across hidden/held-out lesions, and can retain a choice without host winner selection. No completion of B3/B4 can be claimed from scripted step changes or observer-selected best-of-N.
