# CL1B-POLICY-P4F6 — exact-source original-native four-clean-success budget refill, 542-world audit

**Verified final source-pinned original-native checkpoint — 2026-11-21 UTC:** [GitHub Actions run #44862741694 — FULL SUCCESS](https://github.com/iFrankStudio/cioc/actions/runs/44862741694) on exact code SHA `26d94bb8720f57d0593f2b0618e07e1db977759c`. **99/99 registered original Rust native integration targets PASS; 63/63 named independent observers PASS; failed_target_count=0; failed_observer_count=0; independent original CIL replayer exactly matches 752,269/752,269 privately charged opcodes across 542 original one-PID1 worlds, first_divergence=null.** Both native job and source-pinned evidence publisher completed SUCCESS.

The permanent branch paths below were independently fetched and verified after publication:

- [Original source-independent 542-world CIL replay](https://github.com/iFrankStudio/cioc/blob/cl1b-ci-evidence/run-records/26d94bb8720f57d0593f2b0618e07e1db977759c-44862741694-1/cl1b_policy_p4f6_independent_native_replay.json)
- [Original Rust 542-world summary with all matched arms and controls](https://github.com/iFrankStudio/cioc/blob/cl1b-ci-evidence/run-records/26d94bb8720f57d0593f2b0618e07e1db977759c-44862741694-1/cl1b_policy_p4f6_native_542_worlds.json)
- [Full native CI success, source, run ID and run attempt pin](https://github.com/iFrankStudio/cioc/blob/cl1b-ci-evidence/run-records/26d94bb8720f57d0593f2b0618e07e1db977759c-44862741694-1/result.txt) and [runner provenance](https://github.com/iFrankStudio/cioc/blob/cl1b-ci-evidence/run-records/26d94bb8720f57d0593f2b0618e07e1db977759c-44862741694-1/provenance.txt)
- [Original native artifact #14690521939](https://github.com/iFrankStudio/cioc/actions/runs/44862741694/artifacts/14690521939) — original 542 native paid-instruction JSONL traces, summary, independent replay, observer results and logs
- [Pre-native frozen CIL source + 180 24-goal vectors](../studies/CL1B-POLICY-P4F6-frozen-actor-paid-clean-success-refill-542-world-v1.json), [preregistered protocol](../studies/CL1B-POLICY-P4F6-clean-success-refill-preregistration.md), both fixed in [pre-native commit 1cf0fcfc](https://github.com/iFrankStudio/cioc/commit/1cf0fcfc440603a671cd533db01fcec9b863bf22)

## Mechanism: one original PID actually pays to refill its own policy-edit budget

The original, unmodified CL0 Rust world runs with `World::with_strict_capacity(16,180,300)`: no more than **16 distinct world data cells, 180 executable source nodes, 300 occupied units**, original PID1 with **6,000 private compute credits** per independent original native world. P4F6 loads exactly **180/180 executable CIL nodes**, compared with P4F5's 153. Peak native occupied units in sampled traces reach 193, within strict capacity. **No externally rewritten CIL source after load** and no other process is introduced.

P4F6 inherits P4F5's genuine actor-paid `PatchNext PC82` failure handler and `PatchNext PC131` cheap-success control path. On a real original task `Write11`, host evaluation supplies feedback to cell20 *only after* this paid service; original PID must pay `Read20` to observe it. If a failure has been observed, original PID pays `Read38` and its own `Branch PC146` to decide whether it has a remaining online edit. When eligible, PID pays `PatchNext PC139` to arm an expensive successful-task monitor, `Read37` of prior actor-paid successful-task memory, and, when prescribed, `PatchConst PC122` to edit executable source stride at PC46. It then pays `Write38=1` and **`PatchNext PC155` to edit the executable `Write PC133.next=156`**, attaching its own new clean-stage observer.

After monitoring is enabled, actual original PID pays `Read33` to distinguish a feedback-clean task stage from one with failed task attempts. A stage with failures pays to `Write39=0`; one with no failures pays to `Read39`, increment the count, and `Write39`. At the fixed, **researcher-authored threshold of four consecutive feedback-clean successful stages**, original PID pays `Write39=0`, pays `Write38=0` to replenish its online-edit quota and pays **`PatchNext PC177` to restore executable `Write PC133.next=62`**, turning expensive monitoring off. PID pays `Read32` to restore the active stage register and resume normal work. A later failed original task can re-arm another online policy source modification. The original P4F1 fallback `PatchConst PC45/88/102` is not covered by this online PC122 budget. **No host-triggered quota reset, future-goal peek, or free observer.**

This mechanism is **researcher-designed CIL**; four clean successes are an *author-selected* threshold, not self-learned expected utility. Under inverted-feedback control a perceived clean stage need not be a true successful stage.

## Frozen experiment and real native comparison

Preregistered 180 independent **24-stage true-goal sequences**, of which 144 are reused P4F5-known cases and **36 are new P4F6-frozen prospective holdouts**, four independent regime families × 3 starting goals × 3 initial strides: widely spaced permanent pair drift; too-near permanent pair drift; periodic short alias pulses; permanent drift with isolated misleading outliers. Each case runs three **separate original native one-PID1 worlds**: F6-on, source-identical F6 with **all actor PatchNext effects suppressed while still paying all opcode costs**, and distinct-source P4F5-on; add two false-feedback P4F6 controls. **540 truthful + 2 adversarial = 542 independent original native worlds**, not a continuous within-PID lineage across episodes.

For all **540 truthful original worlds** the actor genuinely completes all 24 hidden task stages and asserts success without exhausting its private credits. The separate independent Python interpreter exactly checks each of the **752,269 paid CIL** instructions against native traces, including original PID register values, PC, original budget, all data cells, total occupied units, four mutable source properties (PC12.next, PC34.next, PC46 executable stride and **PC133.next**), and actual external-feedback row after the paid `Write11`. No missing or mismatched opcode, first divergence null.

### Original paid instruction cost (signed positive means F6 saves cost)

**F6-off−F6-on** isolates the **bundle of all F6 actor-paid PatchNext effects** in the *same original loaded source*, NOT any single edit. **F5-on−F6-on** compares two *different original CIL programs*, a net policy/program comparison not identified as the causal benefit of any one particular source edit.

| Frozen sample | Cases | Same-source F6 edit bundle (off−on) CIL paid saving | Cross-source F5−F6 CIL paid saving | Cross-source F5−F6 task probes saved |
| --- | ---: | ---: | ---: | ---: |
| Reused P4F5-known | 144 | **−25,487** | **−12,455** | +114 |
| **NEW frozen P4F6 heldouts** | **36** | **−6,032** | **−3,810** | +45 |
| **All** | **180** | **−31,519** | **−16,265** | **+159** |

Original paid compute totals across the 180 truthful matched goal sequences: F6-on **266,149**, F6 PatchNext-suppressed **234,630**, F5-on **249,884**, plus two adversarial controls **1,538 + 68** = **752,269** independently exactly replayed original CIL opcodes. F6-on is more cost-efficient than F5 in **24/180** goal sequences, worse in **144/180**, tied in **12/180**. This set is deliberately equal-family and *not* a population-weighted natural-world sample.

The **36 genuinely new frozen 24-goal sequences**:

| New preregistered regime (9 cases) | Same-source F6 edit bundle paid saving | Cross-source F5−F6 paid saving | Cross-source task probes saved | Actor-paid quota refills |
| --- | ---: | ---: | ---: | ---: |
| `f6_permanent_pair_wide` | **+3,309** | **+900** | +63 | 18 |
| `f6_permanent_pair_tight` | +444 | **−1,053** | 0 | 0 |
| `f6_periodic_alias_pulses` | **−6,438** | **−1,583** | −3 | 3 |
| `f6_persistent_with_outliers` | **−3,347** | **−2,074** | −15 | 9 |

For scope transparency, the 144 reused P4F5-known cases were used to investigate this strategy before P4F6 original native testing; only the 36 pre-frozen new cases constitute genuinely new P4F6 regime holdouts. The new 36 have **9 beneficial / 27 harmful** F6-vs-F5 cases and no cost ties.

### Proved original actor-paid state and real source edits

Across the 180 F6-on truthful original native worlds:

- **231 actor-paid `PatchConst PC122` executable-stride edits**; each is followed by a **paid `Write38=1`** online budget consumption. Previous fallback edits PC45/88/102 are not counted as online budget edits.
- **231 actor-paid `PatchNext PC155` success-monitor arm edits**, changing the executable `PC133.next` from 62 to156, and **144 actor-paid `PatchNext PC177` success-monitor disarm edits**, restoring executable `PC133.next=62`.
- **144 real paid `Write38=0` replenishments** of online edit quota; paid `Read39` count **1,005** and paid `Write39` count **2,175**. The hard **180/180 source-slot capacity** is retained throughout—this is not unbounded code growth.
- The 36 genuinely new heldout worlds include 57 online edits and **30** replenishments; the reused 144 include 174 online edits and 114 replenishments.
- F6-on versus identical-source PatchNext-suppressed does save **1,092 actual task probes** but spends **31,519 more paid CIL instructions**. Cross-source F6-on versus P4F5-on saves **159 actual probes** at the cost of **16,265 additional paid CIL instructions**. Recovery capability and efficient decision-making are different scientific claims.

### Adversarial controls and limitations

In the **inverted feedback** original native control, original F6 PID1 declares 24 successful stages even though only **12 of 36 real paid task-service guesses** match hidden goals; original actor pays **1,538** instructions. In the **forced-zero** feedback original native control, PID1 halts with a failed claim after 3 real service guesses, one incidental true match, and 68 paid CIL instructions. PID1 cannot authenticate the actual truth of external feedback. No task/feedback oracle was embedded in CIL.

The actor has not invented the clean streak threshold, new operations, code-generation primitives, or a new reward authentication mechanism. Its behavior is real actor-paid source editing following an **authored** algorithm. This is not open-ended evolution, adaptive source-code invention, a self-calibrated optimal budget, proof of cyberlife, or 542-world within-PID continuity.

### Negative test and replay provenance

The protocol, source and new holdout vectors were **frozen before first P4F6 original Rust execution** at [commit 1cf0fcfc](https://github.com/iFrankStudio/cioc/commit/1cf0fcfc440603a671cd533db01fcec9b863bf22) and **never modified afterwards**. An initial all-target [run #44861967602](https://github.com/iFrankStudio/cioc/actions/runs/44861967602) exposed an erroneous P4F5-predecessor test assertion (`PC122.next=134`, a value inherited from P4F4 rather than original P4F5's 147). Corrected by [commit d37f996d](https://github.com/iFrankStudio/cioc/commit/d37f996d607d196cc71e57f18f79711a92a89cf1). Next all-target [run #44862313083](https://github.com/iFrankStudio/cioc/actions/runs/44862313083) actually ran **all 542 original native worlds** successfully and produced all 752,269 paid native traces, but the independent Python replayer's unrelated `observed` variable was overwritten by a newly added branch-value check, causing a final observer-count assertion failure **after exact replaying the first world**. The Python variable was separated from paid feedback observation count in [commit 26d94bb8](https://github.com/iFrankStudio/cioc/commit/26d94bb8720f57d0593f2b0618e07e1db977759c), without changing any frozen CIL source or task vectors. The final exact-source [#44862741694 FULL SUCCESS](https://github.com/iFrankStudio/cioc/actions/runs/44862741694) passes **all** native tests, independent CIL replayer and independent observers, and publishes permanent evidence. Historical failed runs/artifacts are preserved; they are not rewritten as PASS.

## Research conclusion and next falsifier

P4F6 conclusively validates the **ability** of one original CL0 CIL PID to pay to attach self-monitoring, detect an authored clean-success condition, refill its own code-edit budget and physically detach monitoring again. It does **not** validate positive **net compute fitness**: 180 selected original worlds show **16,265 extra** paid instructions for 159 fewer real probes versus P4F5; 36 genuinely new sequences net **3,810 extra** instructions. The only positive new regime is the sufficiently wide spacing between two persistent shifts (+900). It also exposes a second engineering constraint: source occupancy is **180/180**; further actor capabilities require source compression, source replacement or an explicitly redesigned source capacity experiment, rather than quietly raising the original CL0 limit.

**Next P4F7 direction:** study observably **on-demand / lower-cost monitoring**, instead of running a full paid streak monitor on every success. In the strictly identical task-feedback prefix, transient shocks and permanent drifts cannot be distinguished without more genuine paid observation, so demand an explicit value-of-information / price-of-learning benchmark, independent heldouts, and actor-paid decision evidence before claiming general learning-cost adaptation. Never label another researcher-authored heuristic spontaneous algorithm invention.
