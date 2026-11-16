# CL-1B B26 — native worker-lineage copy with CIL-computed first-generation mutation

**Evidence tier:** verified complete native Rust campaign plus source-parsed independent CIL interpreter. [Run 43781626930](https://github.com/iFrankStudio/cioc/actions/runs/43781626930), exact tested source SHA `1c9c5fa668477c73f4a7620e98567c9d8e897fc7`, **full scope: 27/27 Rust integration targets and 13/13 independent analyses PASS**; evidence publication job PASS. [SHA-pinned native/observer summary](https://github.com/iFrankStudio/cioc/blob/cl1b-ci-evidence/run-records/1c9c5fa668477c73f4a7620e98567c9d8e897fc7-43781626930-1/summary.txt). [Prospective protocol](../studies/B26-native-variant-lineage-selection.md) · [Rust source](../../cl0/crates/world/tests/cl1b_b26_native_variant_lineage.rs) · [independent restricted interpreter](../analysis/b26_native_variant_lineage_scan.mjs).

## What is genuinely new relative to B22/B25

At world initialization the researcher loads **only one 35-node founder** with 34 local compute credits and world pool0. Native CIL operations in that founder perform:

1. `GraphNew` to allocate a graph object.
2. **Five `GraphCopyNodeLocal` operations copying the parent's own running worker source PC30..34** (no inert full helper-only source template).
3. Four charged native `GraphSetNext` edits to complete that same worker graph.
4. Native `Add` in process1, computing child's request from the parent request and a signed delta specified in original CIL code; a charged `GraphSetConst` changes the child graph node0's executable request literal **before** launch.
5. `GraphLaunch` transferring two of the original parent's credits into internally born process2. The child has exactly the same five work opcodes/control edges as the parent worker, with only the request literal changed.

**The host never loads process2, grants either contender individual-specific compute after setup, or scores outcomes to award more energy.** Initial34 compute pays exactly34 native charges including program cloning, child boot and both actors' deferred Draw attempts; both sit at a pending Draw with zero local compute. Later energy only enters **the shared world pool**, in six external fixed packets.

All child construction, five source-code copies, four edge edits, one literal edit and child creation have `Origin::Internal(1)`, with proper `GraphLaunched(parent1,child2,credits2,moved_nodes5)` evidence. Resource accounting is exact at every charged step and every external pool packet.

## Twelve frozen mutant-competition worlds

Three investigator-defined genotype relations under four six-packet shared pool schedules. In the table, pairs are **parent / internally born child genuine PC34 / PC4 service writes**, source-attributed rather than inferred from shared cell value.

| Environmental supply packets | CIL computes parent4→child3 | CIL computes parent3→child4 | CIL copies parent4→child4 (same-cost zero mutation) |
| --- | ---: | ---: | ---: |
| Six packets of4 | **0 / 6** | **6 / 0** | 2 / 2 |
| Six packets of5 | 3 / 3 | 3 / 3 | 3 / 3 |
| `[4,5,4,5,4,5]` | 3 / 3 | 3 / 3 | 2 / 4 |
| `[5,4,5,4,5,4]` | 3 / 3 | 3 / 3 | 2 / 3 |

With scarce packets of4, `Draw3` can use the generic pool-funded attempt and receive3 credits; `Draw4` requires5 and stays blocked. **The advantage follows request3 after swapping whether that request is carried by the parent or the child**, rather than following a privileged PID. With loose/mixed packets, deterministic scheduler history and stored unused pool credit influence which actor executes, so simple counts do not imply general fitness.

## Strong scientific limitations

The *variation's new executable value* is genuinely computed and committed **inside CIL** to an internally assembled child's code. But **the decision to clone, the request delta −1/+1/0, the GraphSetConst target and the task to produce cell9=1 were all explicitly authored by the researcher**. There was no candidate selection based on observed resource success, no spontaneous instruction semantics, no mutation distribution inferred by the population, and **the five-node child lacks any replication instructions**: this experiment has **one generation**, not an evolving lineage.

This is a **confirmed native mechanism for first-generation source-lineage variation coupled to identity-neutral ecological filtering**. It is an important prerequisite for testing digital evolution but **does not satisfy CL-1B B3/B4's endogenous adaptation/open-ended organization requirements**. Future tests must require full-program offspring that inherit reproductive capability and can themselves generate descendants, then multiple charged generations with mutation and prospectively withheld resource schedules, without host-selected winners. Evidence of ancestry must come from native graph/process provenance, not labels or externally supplied replacement processes.
