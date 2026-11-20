# CL1B-POLICY-P1 — Native executable search-policy stride mutation, causal gain and counterexample

**Exact original native source pin:** `f8539f4bec0b1a704435c4de90ae10f6e2edff4b`; [GitHub Actions #44690533465 — SUCCESS](https://github.com/iFrankStudio/cioc/actions/runs/44690533465). In this exact run **44/44 selected native Rust tests**, **63/63 named independent observers**, **0 failures**; the additional dedicated P1 interpreter independently reexecuted **856 original actor-paid CIL instructions in 6 separate native worlds**, exact PCs/registers/credits/cell contents and external feedback, **no first divergence**.

- [Original 62-node frozen program and 6 prospective controls](../studies/CL1B-POLICY-P1-frozen-62-node-feedback-conditioned-executable-search-stride-v1.json).
- [Source-pinned P1 native full replay](https://github.com/iFrankStudio/cioc/blob/cl1b-ci-evidence/run-records/f8539f4bec0b1a704435c4de90ae10f6e2edff4b-44690533465-1/cl1b_policy_p1_independent_native_search_stride_replay.json).
- [Native result and exact source pin](https://github.com/iFrankStudio/cioc/blob/cl1b-ci-evidence/run-records/f8539f4bec0b1a704435c4de90ae10f6e2edff4b-44690533465-1/result.txt).
- [Complete 81-goal source-only counterfactual script](../analysis/cl1b_policy_p1_full81_source_only_counterfactual.py).

## New demonstrated behavior, relative to P0 and I6U

This is an **orthogonal mechanism experiment**, not an integration with the 180-node I6U structural repair founder. Under unchanged Rust `World::with_strict_capacity(16,180,300)`, a single original PID1 has 62 source CIL nodes and private paid compute credits800. The initial task-output guess lives in **data cell30**, NOT a host-edited or source-editable task answer. PC3 actually pays `Write11`; the environment replies with just `SetCell20`; PC5 pays `Read20`. At the **second earned successful task stage**, the original actor pays `PatchConst` at source PC45 targeting **executable PC46 `Const r2, stride`**. This changes how future candidate predictions are generated, rather than directly patching the current answer. The same PID1 remains alive across all four stage changes, and no host writes source.

Prospectively frozen native contrasts:

| Original native world | Final executable stride | Paid CIL | Paid task attempts | Task truth |
|---|---:|---:|---:|---|
| Cyclic goal sequence `[0,1,2,0]`, patch allowed | 1 | 123 | 4 | all4 genuine |
| Same `[0,1,2,0]`, patch suppressed | 1 | 123 | 4 | all4 genuine |
| Cyclic goal sequence `[0,2,1,0]`, patch allowed | **2** | **149** | **5** | all4 genuine |
| Same `[0,2,1,0]`, patch suppressed | 1 | **196** | **7** | all4 genuine |
| Cyclic `[0,2,1,0]`, feedback inverted | 0 | 197 | 7 | **3 true**, actor claims4 |
| Cyclic `[0,2,1,0]`, feedback forced0 | 1 | 68 | 3 | actor halts/fails |

**Causal observation:** With the same private budget, task sequence and original executable code, only the effect of original `PatchConst` is suppressed. The actor-paid policy-stride change reduces actual computation by **47 charged instructions** and real invalid task service attempts by **2** in the stride2 environment. It makes no difference in stride1. The inverted-reward control establishes that external feedback is a causal input but *not a truth oracle*.

## Crucial adverse-transfer falsifier (SOURCE-ONLY, not yet native)

Enumerate all `3^4 = 81` four-stage goals in the frozen alphabet `{0,1,2}` for source-identical patch enabled and suppressed worlds, using the independent original-source finite interpreter. **All81** are task-completable under both policies. The policy patch helps **27** sequences, hurts **27**, and ties **27**; aggregated over the entire finite goal cube, **allowing policy mutation costs 108 *more* charged instructions** compared with suppressing it.

Thus a 47-instruction benefit on a preregistered cyclic environment cannot be promoted to robust adaptive benefit, much less to self-invented learning rules. This source-only counterexample is a priority for native replication rather than a footnote.

## Failure history, attribution and next falsifier

Initial P1 native run [#44689792815](https://github.com/iFrankStudio/cioc/actions/runs/44689792815) found a copied **auditor** PC error (the failed-task `Write21` is PC27; PC28 is `Halt`). The audit scripts were fixed **without any change to the frozen 62-node actor source, goals, expected results or native world kernel**. Exact-source final P1 success is #44690533465.

**Nonclaims:** The researcher supplied the finite task alphabet, stride rule and calibration logic; the actor did not invent them. Source PC46 is a preselected controllable candidate advance parameter, not a newly assembled algorithm. These are 6 separate original PID worlds, not long-term ancestry. The goal-cube counterexample is so far only source-interpreted; no claim of 81 additional native worlds. A next native experiment should verify negative transfer without selecting only favorable seeds; a subsequent adaptation study should support evaluating and rolling back harmful policy patches against an equal-cost, same-source static control.
