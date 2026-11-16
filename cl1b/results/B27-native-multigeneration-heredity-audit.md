# CL-1B B27 — first native, finite-resource multigeneration full-program self-copy

**Actual native verified result:** [GitHub Actions full run 43782314431](https://github.com/iFrankStudio/cioc/actions/runs/43782314431), source SHA `5e9578b73e9967208f05b81bc40e823a3e663f89`, **28/28 Rust integration tests and 14/14 independent source-parsed observers PASS**, both CI jobs successful. [Retrieved source-pinned summary](https://github.com/iFrankStudio/cioc/blob/cl1b-ci-evidence/run-records/5e9578b73e9967208f05b81bc40e823a3e663f89-43782314431-1/summary.txt). [Prospective protocol](../studies/B27-multigeneration-self-copy.md) · [actual CIL founder/native test](../../cl0/crates/world/tests/cl1b_b27_multigeneration_self_copy.rs) · [independent restricted scheduler](../analysis/b27_multigeneration_self_copy_scan.mjs).

## Exact native lineage and budgets

Unlike B26's child, which inherited only a five-node **work** program, each B27 child inherits a **byte/structure-identical copy of its parent's full 12-node executable**, including graph allocation, self-copy loop, branch condition, and `GraphLaunch`. Exactly **one** process is host-loaded at t0, with2 local credits and no world energy. Its code uses generic native `GraphNew`, **12 ordered `GraphCopyNode` executions copying source node0..11**, and `GraphLaunch` with2 transferred child credits. The child can execute the same self-copy routine as the parent; no new `REPRODUCE` opcode, no host `PutGraph`, no individual `SetCredits`.

| Preregistered arm | Neutral external world-pool packets | Actual births | Native internal lineage | Compute used after first two bootstrap charges |
| --- | --- | ---: | --- | ---: |
| THREE_GENERATIONS | 79,79,79 | **3** | **1→2→3→4** | 79+79+79 |
| SPLIT_THRESHOLD | 78,1,79,79 | **3** | **1→2→3→4** | 0,79,79,79 |
| SINGLE_PACKET | 79 | **1** | 1→2 | 79 |
| NO_INFLUX | none | **0** | no child | 0 |
| NATIVE_COPY_DISABLED | 79,79,79 | **0** | copying suppressed, launch traps | 77 then 0,0 |

The full three-generation arm executes **239 native charged steps** (2 initial + 3×79 pulses), consuming **exactly 239 compute units**. Source-provenance/native accounting confirms one internally owned graph creation, **12 native code-copy events per birth**, and `GraphLaunched(parent,child,transferred_credits2,moved_nodes12)` in each generation. Gen0, Gen1, Gen2 and Gen3 all have the **same native 12-node program**; no graph objects remain after successful launches. The newest child waits at pool-payable Draw PC1 with zero local credits; each preceding parent ends with zero local credits at ordinary non-Draw PC0, unable to execute again without a different world affordance. There is no claim of infinitely active parenthood.

**Threshold control:** A shared pool of78 cannot pay the pending Draw78's own instruction fee1 and transfer78. The first external packet generates **zero internal events**. Adding one neutral unit later lets native reproduction proceed. No host chooses which process receives compute.

**Native mutation-disabled control:** The exact same source and explicit pool pulses are supplied, but generic executable edits are suppressed at normal instruction cost. The empty graph cannot launch; a native trap stops the only founder. Subsequent environment credits remain unused. This rejects the hypothesis that the observed offspring were only externally allocated placeholders.

## What the scientific milestone is—and is not

**Established on the exact native Rust VM:** finite-budget, charged, **multigeneration hereditary executable continuity** from one initially loaded CIL algorithm through at least three internally created parent-child links. This is a much stronger mechanism than constructing a preauthored one-off helper. It demonstrates that a generic graph VM with compute conservation is expressive enough for **a scripted self-replicator lineage** without any explicit life-like instruction in the world physics.

**NOT demonstrated:** the replication algorithm itself originated within the world, any genotype changed under environmental selection, multi-generation adaptation, spontaneous metabolism, stable cooperative self-organization, independent goals or open-ended evolution. The entire self-copy loop was a **researcher-written seed**, and three environment energy packets were explicitly supplied. Parent actors become blocked; offspring continuity depends on repeated exogenous neutral supply. These are **authored digital automata capable of exact hereditary copying**, not proven autonomous cyberlife.

**Next discriminator:** test whether the *full reproductive executable itself* can acquire CIL-computed **cumulative inheritable changes**, then see whether resource pressure filters those changed genotypes without host-picked winners. A new literal or descendant by itself is not learning; source lineage, resource conservation, failure controls and genuine inherited function are required. Retain the B27 exact-source full 28/14 CI checkpoint as baseline for follow-on studies.
