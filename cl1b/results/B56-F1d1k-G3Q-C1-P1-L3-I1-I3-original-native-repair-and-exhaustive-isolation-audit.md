# CL-1B B56-F1d1k G3Q-C1-P1-L3-I1→I3 — original-native one-PID source repair, bounded retries and exhaustive structural certificate soundness

**Checkpoint date:** 2026-11-19. All three native evidence-bearing CI runs below have **final SUCCESS**, have source-pinned original logs on `cl1b-ci-evidence`, and were inspected separately from subsequent unverified changes.

## Exact-source native ledger

| Study | Tested main source SHA | Verified workflow | Actually selected original Rust integration targets | Named legacy independent observers | New source/step replay |
|---|---|---|---:|---:|---|
| **I1** targeted programmed self-edit after structural failure | `881fd388bb858b4466ed89bc1f6d4ca5d3e5b1d6` | [#44544415327 — SUCCESS](https://github.com/iFrankStudio/cioc/actions/runs/44544415327) | **35/35** | **63/63** | **12,317 original CIL steps**, first divergence null |
| **I2** repair value read from actor's own stored first-word bank, bounded one retry | `a7e2d5327d0e8332d0c15c5874cc89eebf5c874a` | [#44549693740 — SUCCESS](https://github.com/iFrankStudio/cioc/actions/runs/44549693740) | **83/83** | **63/63** | **10,663 original CIL steps**, first divergence null |
| **I3** fully isolated three witness flags and all64 precommitted founder genotypes | `5f0dd9e6de02fad273226bae126d72fdb832e880` | [#44551590413 — SUCCESS](https://github.com/iFrankStudio/cioc/actions/runs/44551590413) | **84/84** | **63/63** | **62,993 original CIL steps in 66 worlds**, first divergence null |

The new independent original-source trace replayers are **additional to**, not counted as part of, the 63 named legacy observers.

Primary evidence:

- [I1 complete original native step, source mutation, and disabled-edit replay](https://github.com/iFrankStudio/cioc/blob/cl1b-ci-evidence/run-records/881fd388bb858b4466ed89bc1f6d4ca5d3e5b1d6-44544415327-1/g3q_c1_p1_l3_i1_independent_native_self_repair_replay.json).
- [I2 nine native source-conditioned repair/failure-control arms](https://github.com/iFrankStudio/cioc/blob/cl1b-ci-evidence/run-records/a7e2d5327d0e8332d0c15c5874cc89eebf5c874a-44549693740-1/g3q_c1_p1_l3_i2_independent_native_data_repair_failfast_replay.json).
- [I3 all64 original native genotypes plus two adversarial controls](https://github.com/iFrankStudio/cioc/blob/cl1b-ci-evidence/run-records/5f0dd9e6de02fad273226bae126d72fdb832e880-44551590413-1/g3q_c1_p1_l3_i3_independent_original_native_exhaustive64_replay.json).
- [I3 tested-native exact source and job SUCCESS](https://github.com/iFrankStudio/cioc/blob/cl1b-ci-evidence/run-records/5f0dd9e6de02fad273226bae126d72fdb832e880-44551590413-1/result.txt) and [all84 named original Rust targets](https://github.com/iFrankStudio/cioc/blob/cl1b-ci-evidence/run-records/5f0dd9e6de02fad273226bae126d72fdb832e880-44551590413-1/summary.txt).

## Progress and real causal meaning

### I1: original PID1 repairs its *own executable instruction*

The frozen [178-node I1 original CIL](../studies/B56-F1d1k-G3Q-C1-P1-L3-I1-frozen-178-node-certificate-driven-native-self-repair-v1.json) starts with researcher-authored bank `[2,0,3]`, a rule whose RESET tail makes all three structural probes degenerate. After its own paid coverage check fails, PID1 executes an **original, charged native `PatchConst` against its own executable PC6 at original paid step367**, setting the previously unsafe tail3 to a **fixed, preauthored literal2**, and continues **without a new process**. It then repays certification, authentic Write11 task service, Read20 feedback and task policy adaptation after hidden goal shift, for **1,724 original steps**. The edit-disabled original-world negative arm never passes certification, attempts the repair21 times and consumes all **8,000** private compute credits. This is a real actor-origin code edit, but the patch target and value were predetermined.

### I2: data-conditioned edit and a one-attempt resource safeguard

The pre-native frozen [180-node I2 original source](../studies/B56-F1d1k-G3Q-C1-P1-L3-I2-frozen-180-node-data-conditioned-repair-and-failfast-v1.json) replaces I1's fixed repair value2 by a **native paid Read of the original actor's own first-word bank51**, then performs its original paid executable `PatchConst`. In separate source-genotype original native worlds, it actually repairs tail3 to **0,1,2** respectively and all three complete four hidden-goal episodes after paid re-certification. A persistent paid retry marker in cell11 allows at most one mutation; when the edited rule remains unqualified the original PID1 **self-Halts in 734 or 736 original steps** with zero task services, instead of depleting its full compute budget. Disabling native source edits in a matched world similarly prevents task service and causes fail-closed self-Halt in **730 steps**. Deceptive feedback still can produce false actor-perceived success.

**Critical negative audit:** A separate exhaustive **source-interpreter-only** 64-rule sweep of the I2 frozen source (NOT an I2 original-native 64-rule audit) found **10 false-positive structural certificates** for genotypes outside its nine precommitted native arms. For example initial `[0,2,1]` becomes `[0,2,0]` after repair, which produces only two distinct native semantic outputs `[2,0,0]`, yet actor's coverage memory passed. Causal root: the repair path reset only witness flag7, allowing positive marks on flags8 and9 from a prior *different* rule to combine with a later rule's marks. I2 is a sound result **only in its specific nine original-native tested arms**, NOT a complete soundness certificate.

### I3: no stale witness marks; native all64 exhaustive proof over defined grammar

The prospectively frozen [176-node I3 original source plus all64 predictions](../studies/B56-F1d1k-G3Q-C1-P1-L3-I3-frozen-176-node-exhaustive-certificate-isolation-v1.json) removes redundant first-/third-word bank copies: the original PID stores the first and last words directly at meta-interpreter active addresses62/64 and maintains the middle immutable word at52 with an active middle byte63. This saves8 CIL instructions and two allocated cells. It uses four of those freed source slots to **originally pay for clearing all three distinct structural witness flags7/8/9 between failed certificate attempts**, while retaining the one-attempt fail-fast guard and actor-authored tail value chosen from original actor word62.

For **each of the 4×4×4=64 distinct researcher-seeded three-meta-word genotypes**, Rust starts a fresh **single uninterrupted original PID1** under unchanged `World::with_strict_capacity(16,180,300)`. Every CIL instruction, PC, register, private credit, source-tail literal (including native `PatchConst` changes), occupied cell presence/value, and interleaved external task feedback is checked against an independently coded source interpreter. A separate mathematical three-word evaluator checks actual structural mode coverage rather than trusting native witness values or the final self-declared status.

**Original native exhaustive results:**

| Finite original grammar classification | Actual verified cases |
|---|---:|
| All unique original three-word rule genotypes | **64** |
| Structurally complete before any edit | **24** |
| Structurally incomplete before editing | **40** |
| Original actor executes one paid source edit in those incomplete cases | **40** |
| After one bank-derived source patch, now complete | **6** |
| After one bank-derived patch, still incomplete and self-Halts | **34** |
| Original PID's structural certificate agrees with external semantic three-output oracle | **64/64** |
| **False-positive certificates** | **0** |
| **False-negative certificates** | **0** |
| Source/step replay first divergences | **0 across 62,993 paid CIL steps, including two control worlds** |

The I3 structural witness is **exact for this finite grammar and the observed in-world instruction semantics**, not a theorem for arbitrary software or behavior. The 64 cases are **separate fresh original-PID worlds**, not 64 generations of one persistent individual. It spends **≤176/180 source nodes, ≤14/16 occupied data keys, ≤191/300 occupied storage units**, so no world-cap bypass. The rescue rate among initially unsafe genotypes is only **6/40 (15%)**; the repair proposal remains preprogrammed, data-dependent, and deliberately one-attempt.

**Counterfactual and semantic controls:** An original source-edit-disabled world leaves its unsafe tail unchanged and self-Halts after **718** paid instructions with zero task services. A structurally safe, actor-repaired genotype under inverted feedback demonstrates **false actor success=1 despite 0 true task matches**. Structural certification alone cannot authenticate task fitness; compare prior [F2 native observational alias](B56-F1d1k-G3Q-C1-P1-L3-F2-native-verified-semantic-feedback-observational-alias.md).

## Unresolved frontier, and what NOT to infer

The original PID can now **observe a structural failure, pay to change its own executable node, erase stale certificate state, revalidate the changed rule, and either continue serving real tasks or autonomously stop to conserve remaining computation**. However, the self-edit is not a new learned grammar; the source design, repair target, one candidate proposal and task-feedback decision pattern are all fixed before launch. Neither I3 nor preceding M2 ever proved the original individual **invents, evaluates and improves its own repair *strategy***. There is no continuous ancestry between CI arm worlds, no generalized cross-software adaptation, and no solved problem of deceptive task signals.

**Priority for the next research stage:** make the rule-edit target and candidate value conditional on native in-world evidence instead of investigator-authored one-candidate bank copy, while preserving full per-attempt witness isolation, strictly bounded search, original executable provenance, source-independent semantic/task fitness and unchanged 16/180/300 CL0 physics. A solution must do better than the **6/40 rescue baseline** under precommitted heldout genotypes and must not quietly change author-coded tasks or native VM laws.
