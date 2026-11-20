# CL1B-POLICY-P4F1 — Original-native paid retained-policy restoration with fresh heldout regimes

**Status: EXACT-SOURCE ORIGINAL-NATIVE VERIFIED (2026-11-20 UTC).** [GitHub Actions #44710683292 — SUCCESS](https://github.com/iFrankStudio/cioc/actions/runs/44710683292), exact tested main SHA **`70cda196c7698901ab66e5d080555f34ecb6ca2e`**. All **95/95 original Rust native integration targets actually selected and PASS**, **63/63 independent named observer analyses PASS**, **0 failures**. Separate Python CIL interpreter reexecuted **223,579 original actor-paid instructions across 182 original one-PID1 native worlds** with no first divergence, including original source graph PC12.next, executable PC46 policy stride, private credits, registers, all watched data cells, and every external task feedback event.

## Source and exact-SHA evidence

- [Frozen 106-node F1 actor, 91-node F0 native comparator, 72 goal sequences and all 182 arms](../studies/CL1B-POLICY-P4F1-frozen-recoverable-source-policy-with-new-heldout-v1.json)
- [Prospectively frozen protocol including two previously unused heldout families](../studies/CL1B-POLICY-P4F1-paid-policy-rollback-preregistration.md)
- [Original Rust CL0 execution](../../cl0/crates/world/tests/cl1b_policy_p4f1_recoverable_native.rs)
- [Independent original CIL interpreter, not Rust-result inference](../analysis/cl1b_policy_p4f1_independent_native_replay.py)
- [Exact tested source independent 223,579-step replay](https://github.com/iFrankStudio/cioc/blob/cl1b-ci-evidence/run-records/70cda196c7698901ab66e5d080555f34ecb6ca2e-44710683292-1/cl1b_policy_p4f1_independent_native_replay.json)
- [Exact-source workflow success and source SHA](https://github.com/iFrankStudio/cioc/blob/cl1b-ci-evidence/run-records/70cda196c7698901ab66e5d080555f34ecb6ca2e-44710683292-1/result.txt)

**Failed-first-run history preserved:** [#44709364371](https://github.com/iFrankStudio/cioc/actions/runs/44709364371) actually passed its native Rust target, but its additional Python observer was rejected on a mistaken filename for the frozen manifest, before any independent replay. The mismatch is visible in the preserved [stderr evidence](https://github.com/iFrankStudio/cioc/blob/cl1b-ci-evidence/run-records/09ae06985f06b8e7bb55eef1d4ea3617d6565ea0-44709364371-1/cl1b_policy_p4f1_independent_native_replay.stderr.log). Commit `ad477ba8` corrected **only the observer's manifest path**, with no edit to original actor source, frozen tasks, Rust engine or native test. Source-pinned focused replay passed [#44710047706](https://github.com/iFrankStudio/cioc/actions/runs/44710047706), then a deliberately changed CI workflow comment forced the **full 95-target exact-SHA run #44710683292**, which independently passed and published its source-pinned proof. Failed attempts must not be rewritten as success.

## Actual original-PID source mechanism

All worlds use unchanged `World::with_strict_capacity(16,180,300)`. F1 actors start with 106 actual original CIL source nodes, 6,000 private compute credits and a single original PID1 uninterrupted through their world; F0 heldout comparators use their prior frozen 91-node source. Each has at most 24 hidden task stages. The original PID pays `Write11` service, the world then responds via external `SetCell20`, and the actor pays `Read20`. First two genuinely earned task successes drive the original paid `PatchConst PC45` to initialize executable relative stride `PC46`. A paid `PatchNext PC82` then arms the failure-only detector by actually editing the original `Write PC12.next:13→83`.

F1 changes P4F0 in **three actor-paid ways**:

1. The actor pays `Write35` at PC92 to retain its **initially learned** stride value after initial policy edit. This does not copy a privileged source snapshot or receive extra compute.
2. On two actual failed task service attempts, the event-triggered detector pays `Read36` to observe its own active-policy mode. If mode is absent/0, actor executes a paid executable `PatchConst PC88` to author-fixed stride1, and pays `Write36=1`.
3. If the same detector later fires while mode1 is active, the actor pays `Read35` to recover its earlier stored stride and actually executes `PatchConst PC102` to restore original executable `PC46`, then pays `Write36=0`. Every branch, memory access, patch and subsequent trial is charged against the same private budget. **Neither credits nor the full process snapshot is rolled back**; only one stored executable policy parameter is restored.

Across the 72 F1 detector-enabled original worlds, the reconstructed native source executes **60 paid PC102 restoration operations in 21 independent original worlds**. This is native source mutation, not simulation of a planned action; the independently reconstructed source PC and policy value match the original Rust world after each opcode. The mutation and toggling logic were authored by researchers, **not independently invented by PID1**.

## Results: source-identical F1 causal ablation

Every row is nine frozen 24-stage task sequences, each with **same-source F1 detector-enabled vs PatchNext-suppressed native actor**. Suppression leaves initial paid relative-stride `PatchConst` untouched, so it specifically tests the effect of arming the event monitor. Positive = **fewer paid original CIL instructions/probes with the monitor enabled**, negative = worse. All108 previously used and36 fresh heldout F1 truth-feedback native worlds genuinely finish all24 stages.

| Task family | Original F1 native pairs | Paid instruction saving (suppressed−on) | Actual service probes saved |
| --- | ---: | ---: | ---: |
| Stable regularity (prior P4F0 family) | 9 | 0 | 0 |
| Single switch (prior) | 9 | −432 | 0 |
| Alternating regimes (prior) | 9 | +753 | +63 |
| Scrambled deterministic (prior) | 9 | +1,648 | +114 |
| Previously tested return to prior regime | 9 | −216 | 0 |
| Previously tested rare short shock | 9 | −147 | 0 |
| **Fresh P4F1 heldout late switch** | 9 | +969 | +63 |
| **Fresh P4F1 heldout two shocks** | 9 | −672 | −9 |
| **Total within this selected bounded corpus** | **72** | **+1,903** | **+231** |

**Separate cross-source F1-on vs F0-on original-native controls, new heldouts only:** F1 stores more data and has more source nodes, so this comparison measures *absolute implementation consequences*, not the isolated causal PatchNext effect.

| New heldout task family | Fresh native F1-on vs F0-on pairs | Absolute paid CIL saving (F0−F1) | Real service attempts avoided |
| --- | ---: | ---: | ---: |
| Late rule switch | 9 | **−210** | 0 |
| Two short shocks and recovery | 9 | **+936** | **+36** |
| **Total** | **18** | **+726** | **+36** |

In particular the late-switch family can show a **positive F1 on-vs-off detector effect** yet a **negative F1-vs-F0 implementation gain** because added source/memory overhead matters. Conversely the two-shocks family shows the opposite ordering. Both comparisons are necessary; conflating them would generate misleading fitness claims.

## Honest limits and priority next falsifier

- **World count and truth:** 72 F1 goal pairs × 2 = 144 original F1 actors, 18 previously unseen sequences × 2 = 36 original F0 actors, plus 2 adversarial F1 feedback controls, for **182 separate native original worlds**. All **180 truthful-feedback worlds** genuinely complete all24 task goals = **4,320 truthful completed stages**. These are separate worlds, not one continuous cross-world life.
- **Feedback integrity:** inverted external reward makes an actor report completed stages despite only two genuine task matches under the frozen control. Forced-zero control halts and declares failure. The mechanism responds to perceived feedback; it is not a true outcome authentication layer.
- **Regime generality:** the eight schedules are intentionally structured, finite, known to researchers, and only two families were new heldouts for F1. Across those new holdouts, the recovery logic still has costly adverse cases. Neither +1,903 (same-source ablation) nor +726 (source-different) can be extrapolated into a universal gain.
- **Mutation ownership:** yes, PID1 pays for Write35/Read35, mode36 reads/writes and real executable PatchConst restoration, all original-native and independently accounted. But the test does **not** demonstrate that the PID invented its own monitor, regret threshold, fallback grammar or source synthesis.
- **CL-1B not complete:** no autonomous open-world evolution, independently discovered algorithm, trustworthy long-lived task fitness, integrated I6U structural self-repair, or indefinite computation. I6U's 180-node structural repair program is a separate, source-capacity-saturated line.

**Priority next:** actor-paid *evidence quality and policy-value* assessment before switching, with a bounded real counterfactual trial rather than a fixed two-failure toggle. Include prospective new stable, delayed and adversarial regimes, charge all sensing/verification, and reject false improvements measured only as fewer task probes instead of net compute. Evaluate whether F1's 106-node policy mechanism can be compressed before integrating with the separate 180-node I6U actor; do not silently increase the original CL0 source limit.
