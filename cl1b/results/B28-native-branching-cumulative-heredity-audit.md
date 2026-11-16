# CL-1B B28 — native cumulative executable mutation, branching ancestry and resource viability

**Verified outcome:** [GitHub Actions full run 43783328973](https://github.com/iFrankStudio/cioc/actions/runs/43783328973), exact executed source SHA `faf235300eb18b52ad1785684423dbddf1a27c8e`. **29/29 native Rust integration targets PASS**, **15/15 independent observer models PASS**, separate source-pinned evidence-publication job PASS. [Retrieved complete CI summary](https://github.com/iFrankStudio/cioc/blob/cl1b-ci-evidence/run-records/faf235300eb18b52ad1785684423dbddf1a27c8e-43783328973-1/summary.txt) · [provenance](https://github.com/iFrankStudio/cioc/blob/cl1b-ci-evidence/run-records/faf235300eb18b52ad1785684423dbddf1a27c8e-43783328973-1/provenance.txt).

See the [B28 prospective protocol](../studies/B28-cumulative-heritable-resource-mutation.md), [real native CIL integration source](../../cl0/crates/world/tests/cl1b_b28_cumulative_genome_mutation.rs), [independent source-parsed VM](../analysis/b28_cumulative_genome_mutation_scan.mjs), and [B28b transparent failed-prediction correction](B28b-posthoc-branching-lineage-correction.md). **B28's original strict-linear prediction was disproven**, and the final passing code verifies the actual ancestry-based outcome. Do not present the passing revision as retroactive confirmation of that preregistered prediction.

## A new, source-verified substrate capability

One initially loaded process1 has a **complete 19-opcode native CIL self-reproducer**. It copies its **entire executable source** with nineteen `GraphCopyNode` instructions into a separate graph, computes a request mutation through a charged CIL `Add`, edits two cloned `Const` literal sites (PC0 resource-Draw and PC11 mutation base) using two charged native `GraphSetConst` operations, and launches an internally funded process. The new process inherits **all 19 executable nodes**, including its graph copier, mutation operator, and launcher. It can subsequently copy and modify *its own* full genome. Neither child source nor individual credits are supplied by the host after the initial founder load.

Every native attempt decrements total local+world-pool compute exactly1, every research-created resource packet enters only the shared pool with explicit external provenance, and every genotype/ancestry relationship is checked against actual internal `GraphLaunched(parent,child)`, ordered code-copy and source edit events. A successful birth transfers2 existing local compute credits into the child; it does not mint them.

The **mutation rule δ∈{+1,0,−1}, construction algorithm, organism genotype and source resource schedules remain entirely researcher-authored**. This is verified *within-world execution and inheritance of a fixed mutation operator*, not a spontaneous origin of the operator or autonomous learning.

## Complete 11-world results

All mutation conditions start from the same executable request genotype127, two initial process credits and a zero-credit shared world pool. Each neutral resource packet is explicitly external; per-generation request viability depends on the 19-node copy/graph-edit/launch routine's **127 local credit minimum** plus one pool-paid Draw instruction charge.

| Native CIL mutation / environmental supply | Births | Verified native ancestry | Requests in process-ID order | Failed launch / trapped actor |
| --- | ---: | --- | --- | --- |
| δ=+1, **[128,129,130]** | **3** | **1→2, 2→3, 2→4** | **127, 128, 129, 129** | None |
| δ=+1, **[128,128,128]** | 2 | 1→2, 2→3 | 127, 128, 129 | None |
| δ=+1, **[130,129,128]** | 3 | **1→2, 2→3, 2→4** | 127, 128, 129, 129 | None |
| δ=0, any of above 3 schedules | 3 each | 1→2→3→4 | 127,127,127,127 | None |
| δ=−1, any of above 3 schedules | 1 each | 1→2 | 127,126 | **One native child launch trap** |
| No neutral influx, δ=+1 | 0 | None | 127 | None (blocked) |
| Native code edits disabled, δ=+1, [128,129,130] | 0 | None | 127 | **One native launch trap** |

The exact **δ=+1 gradient** native run consumed **388 native attempts** (2 initial +128+129+129), leaving **one shared-pool compute credit**. The original B28 prereg expected a strictly linear chain and 130 native attempts on the final packet; the real VM returned **129**, triggering a failed CI. An independent interpreter independently found **the same branching lineage**. After explicit post-hoc correction to verify actual parent IDs, both native and independent simulations agreed.

### Why the lineage unexpectedly branches

When process2 carries request128, it has **one extra local compute credit** beyond the 127-credit reproduction minimum. After launching child3, process2's PC returns to0; that extra credit pays the ordinary PC0 `Const` and moves process2 back to the pool-funded Draw at PC1. Process3 also waits at its higher-cost request129 Draw. At the next shared-pool packet130, the deterministic scheduler can afford process2's request128 and select it first, so **process2 reproduces again**, creating child4 with request129. Child4 is a **sibling of child3**, not its descendant. The correct inheritance rule is `request(child) = request(actual_native_parent) + δ`, not `127 + δ*(PID−1)`.

This matters because **extra acquired compute can alter who reproduces, when, and with whom they compete**, even under a fixed mutation rule. Native lineage and code-site verification are necessary: a numerical count of descendants or incrementing process IDs alone would have incorrectly inflated "evolutionary depth."

### Why δ=−1 fails despite available energy

Process1 request127 can create child2 request126. The child may obtain126 energy from a common pool and complete nearly all of its cloned-genome edits, but it **cannot pay the remaining 127-credit minimum needed to transfer two startup credits at `GraphLaunch`**. The native VM records a charged trap. One failed organism and its unused environment credits stay visible. The failure is not interpreted as a host fitness award to another genotype.

## Hard limitations

**The new experimental result is cumulative executable-genotype modification *along actual native ancestry*, not population-wide spontaneous adaptation or open-ended evolution.** All fitness-like advantages and failures arise from a **researcher-designed** deterministic mutation delta and finite resource schedules. The generator does not select δ from learned task success, there is no independent organism-generated fitness criterion, no large mutational neighborhood, no broad ecology, no mutation of its own reproductive algorithm, and no evidence that the code originally arose inside the world without human design.

The next meaningful test is native branching mutation into **two or more sibling genotypes from the same progenitor under identical shared resources**, with variants *produced by CIL* and **not chosen by a host evaluator**, and then tracking multiple generations of both resource access and functional work under held-out environments. It must separate parent credit privilege, deterministic scheduling, source templates, and actual heritable novelty. This stage is still **NOT** an accepted CL-1B endogenous cyberlife breakthrough.
