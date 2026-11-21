# CL1B-POLICY-P4F5 — source-pinned original-native one-online-edit budget and transient/persistent tradeoff

**Source-pinned verified result (2026-11-21 UTC):** [FULL GitHub Actions #44860926121 — SUCCESS](https://github.com/iFrankStudio/cioc/actions/runs/44860926121), exact tested `main` source SHA `4657bc31bf3f3c9dd44e9b71fe5c7ad553f561d5`; **98/98 registered original Rust native integration targets PASS, 63/63 named independent observers PASS, failed_target_count=0, failed_observer_count=0**. Independently authored original-CIL Python replayer reconstructs **579,998/579,998 original PID1 paid CIL instructions across 434 original native worlds**, every program counter, register, paid credit, original source edge, executable stride, data-cell snapshot, occupancy and real external feedback row, **first_divergence=null**. Original job and evidence-branch publisher both concluded SUCCESS; previous P4F4 source-pinned evidence still exists after the evidence publisher's sparse checkout optimization.

Permanent, *confirmed-to-exist* source-pinned evidence:
- [Original independent 434-world CIL replay](https://github.com/iFrankStudio/cioc/blob/cl1b-ci-evidence/run-records/4657bc31bf3f3c9dd44e9b71fe5c7ad553f561d5-44860926121-1/cl1b_policy_p4f5_independent_native_replay.json)
- [Original 434-world Rust summary, all 144 paired records plus 2 false-feedback controls](https://github.com/iFrankStudio/cioc/blob/cl1b-ci-evidence/run-records/4657bc31bf3f3c9dd44e9b71fe5c7ad553f561d5-44860926121-1/cl1b_policy_p4f5_native_434_worlds.json)
- [Exact native job/publisher run SUCCESS pin](https://github.com/iFrankStudio/cioc/blob/cl1b-ci-evidence/run-records/4657bc31bf3f3c9dd44e9b71fe5c7ad553f561d5-44860926121-1/result.txt) and [runner/source provenance](https://github.com/iFrankStudio/cioc/blob/cl1b-ci-evidence/run-records/4657bc31bf3f3c9dd44e9b71fe5c7ad553f561d5-44860926121-1/provenance.txt)
- [Native trace artifact #14690355033](https://github.com/iFrankStudio/cioc/actions/runs/44860926121/artifacts/14690355033) includes all **434** original per-instruction JSONL trace files, native summary, independently replayed summary and all logs.
- [Frozen P4F5 study + full original CIL sources before P4F5 native](../studies/CL1B-POLICY-P4F5-frozen-actor-paid-one-edit-budget-434-world-v1.json) at [preregistration commit a97c8b1](https://github.com/iFrankStudio/cioc/commit/a97c8b1be5ff610c3b77a4791a9afaeac8b78c33).

## Exact experiment design and original CL0 physics

Inherited strict unchanged `World::with_strict_capacity(16,180,300)`: up to 16 distinct data cells, 180 loaded original CIL source nodes and 300 total occupied units, original PID1 receives **6,000 private compute credits** per isolated world. P4F5 contains **153 loaded original CIL nodes** compared with 142 for P4F4; no host source edits after load, no PID2, 24 true hidden task-goal stages.

The PID1 actor first pays for the P4F4 inherited failure-path `PatchNext PC82` and cheap-success `PatchNext PC131`. At every genuine failed task service, the actor pays to read its **cell38** ledger via `PC142 Const38 / PC143 Read38`, computes `(paid_edit_count − 1)`, and branches at PC146. Before its first online relative-stride self-edit it may pay `PC139 PatchNext` to arm the expensive success monitor at executable original `PC34.next=106`. After a genuine paid success and valid `Read37`, it may compute the stride and pay `PC122 PatchConst` to rewrite executable `PC46`; it then pays `PC147..PC152` to `Read38`, increment, `Write38=1`, and pays `PC136 PatchNext` to restore `PC34.next=132`. Subsequent failures pay the full budget check and **do not arm the expensive success monitor**, but the older F1 fallback `PatchConst` operations PC45/88/102 remain possible. This is a **budget on online PC122 re-edits**, not a prohibition on all actor self-modification.

**No reward oracle is given to PID1:** actual task truth bits are written to the external cell20 only after the actor has paid original task `Write11`, and it must then pay `Read20` to observe feedback; the frozen actual goal sequence is held by the experimental world. All source edits and accounting within PID1 are charged and independently checked in the original native step trace. Meter address38 was not part of the previously used ten data cells and remains within the unchanged sixteen-cell capacity. A recorded `Read38=0` before initialization is the original native world's ordinary absent-cell behavior, not a free host-side counter.

**Frozen test set:** 108 previously observed P4F4 sequences plus **36 new sequences frozen before original-native P4F5 tests**, four new regimes × 3 initial goals ×3 inherited strides, all 24 stages. For every sequence: native original F5-on, native *same-loaded-original-source* F5 with **all PatchNext effects** suppressed but instruction charges kept, and native different-source F4-on = 432 truthful independent original PID1 worlds; additionally two deceptive-feedback worlds = **434**. The F5 program and goal vectors were unchanged across native implementation/audit fixes. New regimes: permanent shift at stage8, second permanent shift at stage17, isolated spikes at 3/6/9/12/15/18/21, quiet early prefix then late paired shocks at 13/14/17/18/21/22.

## Original-native outcomes and causal labels

Define signed positive as **charged original CIL instructions saved**. `F5-off − F5-on` is *same-loaded-source bundled actor PatchNext edit-effect contrast* — **not** an isolated effect of cell38, one `PatchConst` or one edit. `F4-on − F5-on` is a **different-CIL-source program-cost comparison**, not edit-specific causality. Always keep these estimands separate.

| Native group | Cases | Same-source F5 off−on paid savings | Cross-source F4−F5 paid savings | Cross-source F4−F5 task probes saved |
| --- | ---: | ---: | ---: | ---: |
| **108 reused P4F4 cases** | 108 | **−8,933** | **−1,523** | **−192** |
| **36 new frozen P4F5 cases** | 36 | **−4,099** | **+9,077** | **+129** |
| **All selected cases** | **144** | **−13,032** | **+7,554** | **−63** |

Per each **new 9-case family**:

| P4F5 frozen family | F5 same-source edit-bundle paid saving | F5 vs F4 cross-source paid saving | F5 vs F4 probes saved | F5 paid online PC122 edits |
| --- | ---: | ---: | ---: | ---: |
| f5_single_permanent_shift | +2,544 | **−99** | 0 | 9 |
| f5_second_late_shift | +2,070 | **−1,860** | −54 | 9 |
| f5_periodic_one_stage_spikes | −5,547 | **+8,295** | +144 | 9 |
| f5_quiet_then_dense_two_step_spikes | −3,166 | **+2,741** | +39 | 9 |

For completeness, 12 **previously tested P4F4 regimes**, 9 cases per family, F4−F5 charged savings: stable **0**; switch **−99**; holdout_revert **−954**; holdout_rare_shock **−844**; early_switch **−99**; late_flip **−144**; two_pulse **+817**; oscillating_step **+1,155**; fresh_sparse_pulses **−1,699**; fresh_dense_pulses **+5,062**; fresh_delayed_revert **−642**; fresh_multi_shift **−4,076**. Selection is deliberately balanced by *regime*, not population-weighted by environmental prevalence. Overall cross-source per-case: **54 beneficial, 78 harmful, 12 tied**; in the new frozen cases exactly **18 beneficial / 18 harmful**.

Absolute native accounting across 144 truthful matched sequences: `F5-on=194,823` instructions, `F5-off=181,791`, `F4-on=202,377` — all three programs are independently loaded and run in their own native worlds, so these are sums over separate lives, **not a 144-life continuous PID**. Two adversarial controls add `939+68` paid instructions, yielding **579,998** total exact original paid CIL steps. **All 432 truthful worlds** complete all **24 actual goals** and report success; no truthful native world exceeds its private 6,000 credits.

## Actor-paid meter, self-modification trace and external feedback

- In the 144 **F5-on truthful worlds**, original PID paid **132 online `PatchConst PC122` source edits** total (0 or 1 per world), which correspond to **132 paid `Write38`** events. Original native traces also contain **1,341 charged `Read38`** events — bookkeeping is not free. Original PID paid **153 expensive-handler arming `PatchNext PC139`** instructions.
- In the 144 **F4-on comparator worlds**, original PID paid **849 online `PatchConst PC122`** and **1,278 expensive-handler `PatchNext PC139`** operations, before this new hard-budget limit. The reduced edit frequency is directly traceable to actor-paid cell38 checks and control-flow branching, but does **not** imply improved expected future value.
- **F5-on vs same-source F5-edit-suppressed** produces **648 fewer actual task probes**, yet costs **13,032 more paid CIL instructions**, so task-service savings can be overwhelmed by self-modification overhead.
- **F5-on vs different-source F4-on** costs **7,554 fewer charged CIL instructions**, but requires **63 more task probes** overall. The new-frozen spike families gain strongly; two long-lived persistent shift stages reveal the risk of running out of learning budget too early. This is the central **transient-vs-persistent resource tradeoff**, not a monotonic fitness improvement.
- In **inverted feedback** original native control, the PID **claims 24 successes although only 1 actual task-service output matches** the hidden goal; 25 probes and 939 charged instructions. In **forced-zero feedback**, it claims failure after 3 probes and 68 paid instructions, with one incidental correct output. Reward/feedback authenticity remains unaddressed.

## Failed-run provenance retained

The original P4F5 CIL source and frozen vectors were **not changed** after preregistration. Two earlier full jobs failed due test-side inherited-source assertions, not an independently established CIL program failure: [#44860125656](https://github.com/iFrankStudio/cioc/actions/runs/44860125656) mistakenly asserted F5 `PC122.next=134` immediately after asserting its true `next=147`; [#44860661792](https://github.com/iFrankStudio/cioc/actions/runs/44860661792) mistakenly treated inherited cheap-success `PC133` as F5-only, rejecting the F4 comparator. Other test-side F4 PatchNext audit targets were also corrected to reflect inherited F4 rather than superseded F3. Historical run failures/cancellations and their logs are **not rewritten as successful original experiments**. Final corrected full native and publisher [#44860926121](https://github.com/iFrankStudio/cioc/actions/runs/44860926121) independently passed against exact `4657bc31bf3f3c9dd44e9b71fe5c7ad553f561d5`. Evidence publisher optimized to a depth-1, blob-filtered sparse checkout: it completed successfully **and historical P4F4 SUCCESS evidence was separately re-verified to remain accessible**.

## Scientific checkpoint and next falsifier

P4F5 demonstrates **one original CIL PID1 can actually meter its own source-rewrite expenditure and pay to suppress future expensive relearning**, under original CL0 resource physics and independently verifiable paid computation. It does **not** demonstrate that PID1 discovered the budget, learned the bound `1`, estimated unknown regime probabilities, chose compute-optimal thresholds, invented algorithms or primitives, survived without externally supplied truth, or evolved openly.

**P4F6 target:** actor-paid **replenishable** modification budget based on genuinely observed clean-success streaks and post-edit repair payback, with no free future information; compare the effect against F5's permanent hard budget and F4's unlimited arming on new frozen transient, persistent and adversarial sequences. Genuine self-directed value estimation requires future experimental evidence; a new fixed threshold is still a researcher-authored policy.
