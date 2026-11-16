# CL-1B B30 — held-out sibling advantage is a narrow resource threshold, with scheduler-driven parent swaps

**Verified native mechanism, NOT confirmatory success of every preregistered prediction.** The initial preregistered study [B30](../studies/B30-heldout-sibling-advantage-threshold.md) predicted the correct *birth-count pattern* but the **wrong native third-parent identity** at the higher resource packets. Both the initial Rust assay and independent observer failed in [run 43802804592](https://github.com/iFrankStudio/cioc/actions/runs/43802804592). Those failed predictions and their [explicit post-hoc B30b correction](B30b-posthoc-scheduler-parent-correction.md) remain in the record.

The corrected **full native campaign** [run 43803051846](https://github.com/iFrankStudio/cioc/actions/runs/43803051846), SHA `3c89704aa76a2a992c3e163af358b77e9b283cef`, passed **30/30 Rust native integration targets and 17/17 independent source-parsed analyses**, with [source-pinned PASS summary](https://github.com/iFrankStudio/cioc/blob/cl1b-ci-evidence/run-records/3c89704aa76a2a992c3e163af358b77e9b283cef-43803051846-1/summary.txt) and successful independent evidence publication. The B30 Rust assertions share the [single B29 native test target](../../cl0/crates/world/tests/cl1b_b29_mutation_rule_siblings.rs), rather than adding another unnecessary compilation target; the [B30 independent model](../analysis/b30_heldout_sibling_threshold_scan.mjs) reuses the same source-parsed B29 VM model and tests **eight** cases.

## Exact experimental design

Same one externally loaded full-genome founder with initial two compute credits, same 23-op CIL source and deterministic scheduler, same initial two neutral world pool packets `[157,157]`. Only the **third exogenous common-pool packet** varies across `155,156,157,158`, with charged native executable mutation enabled (**FLIP**) or replaced by a same-charge non-edit opcode (**NO_FLIP**). Every actor's code, local computation, shared pool and ancestry events are native accounted. All eight trials are retained.

## Verified outcome

| Held-out third packet | FLIP lineage after three packets | NO_FLIP lineage | FLIP extra births | Residual neutral pool (FLIP / NO_FLIP) |
| ---: | --- | --- | ---: | --- |
| **155** | 1→2, 1→3 | 1→2, 1→3 | 0 | 155 / 155 |
| **156** | 1→2, 1→3, **3→4** | 1→2, 1→3 | **+1** | **0 / 156** |
| **157** | 1→2, 1→3, **3→4** | 1→2, 1→3, **1→4** | 0 | 1 / 0 |
| **158** | 1→2, 1→3, **2→4** | 1→2, 1→3, **2→4** | 0 | 0 / 0 |

**Narrow opportunity window:** FLIP yielded an extra birth in only **one of the four held-out third-packet sizes**. The third packet155 is insufficient for any waiting genotype (all stranded), and both variants regain a birth opportunity by 157. This is an authored deterministic four-point resource comparison, **not** a probability-of-success estimate or evidence of an intrinsic fitness edge.

**Scheduler and genotype interaction:** At third packet157 the FLIP arm's cheaper process3 (request155) reproduces, while NO_FLIP's original process1 (request156) reproduces. At packet158, process2 (request157) reproduces in **both** arms—another actual parent changes despite higher resource availability. A simplistic rule predicting the lowest-request genotype always wins fails; real process cursor and earlier credit charges matter.

**Source editing is still real:** The original B29 actor executes charged native `PatchConst` changing its own copied signed-delta rule, enabling siblings request157 and request155. Nothing in B30 disputes the internal executable mutation or full-genome copying. Rather, B30 rejects the unwarranted promotion from *an example of a favorable mutation-rule run* to *general adaptive self-improvement across environmental resource schedules*.

## Scientific conclusion

The native substrate now supports *authored* multi-generation full-source inheritance, inherited genotype changes, native source mutation of a parent's **mutation operator**, genetically different siblings, and finite common-pool ecological filtering. Its observed advantages remain **context-dependent** and deterministic. The program was explicitly authored by investigators; it does not perceive and learn which mutation rule is superior, nor discover the service task or independently originate reproduction.

**CL-1B primary route stays an individually continuous self-improving organism.** Reproduction is optional and CL-1A's primary group-evolution line is archived. The most useful follow-up is not simply to add more descendants, but to close the earlier B20/B24 gap: an individual currently cannot observe lack of neutral compute at a non-Draw PC, and naive self-edit overhead can halt access to energy. Future experiments should distinguish **generic resource-observability affordances** from life-specific reward or host repair, and require an actor to choose code/policy from paid evidence against robust held-out environments.
