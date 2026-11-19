# CL-1B G3Q-C1-P1-L3-I4→I5 — native evidence-conditioned repair reaches the finite one-byte optimal bound

**Verified checkpoint date: 2026-11-19 (GitHub Actions UTC).** These are original-native executions under **unchanged** `World::with_strict_capacity(16,180,300)`; each arm is a separate newly initialized original PID1. The individual within any arm does not restart or spawn. No host data-rule/code writes after load; only actual environment feedback `SetCell20` after the actor's source-paid real task service `Write11`.

## Exact GitHub Actions provenance and replay

| Native study | Exact successfully tested source SHA | Final successful workflow | Actually executed Rust targets | Existing named observers | Additional original charged CIL trace replay |
|---|---|---|---:|---:|---|
| I3 baseline (fixed one-tail byte repair) | `5f0dd9e6de02fad273226bae126d72fdb832e880` | [#44551590413 SUCCESS](https://github.com/iFrankStudio/cioc/actions/runs/44551590413) | **84/84** | **63/63** | **62,993 steps**, 66 worlds, no first divergence |
| **I4: actor witnesses choose repair source location** | `5622598f762e0253acbf8f7b7f3f82b21d9c7e82` | [#44561002026 SUCCESS](https://github.com/iFrankStudio/cioc/actions/runs/44561002026) | **38/38** selected | **63/63** | **84,255 steps**, 67 worlds, no first divergence |
| **I5: evidence-selected location plus RESET3 candidate normalization** | `c02b6c67fb81167de5d89ff99b1dd2397737b9cf` | [#44563928096 SUCCESS](https://github.com/iFrankStudio/cioc/actions/runs/44563928096) | **39/39** selected | **63/63** | **84,735 steps**, 68 worlds, no first divergence |

The CI workflow now **registers 86 original Rust CL-1B targets**, but **the latest I5 source-pinned run selected39**. Never claim 86/86 for this run. The source-independent I4 and I5 every-step observers are **additional to** the 63 existing named independent observers.

- [I4 original native source pin, tested 38 targets, and zero CI failures](https://github.com/iFrankStudio/cioc/blob/cl1b-ci-evidence/run-records/5622598f762e0253acbf8f7b7f3f82b21d9c7e82-44561002026-1/summary.txt)
- [I4 original native every-step, original source target selection, and controls](https://github.com/iFrankStudio/cioc/blob/cl1b-ci-evidence/run-records/5622598f762e0253acbf8f7b7f3f82b21d9c7e82-44561002026-1/g3q_c1_p1_l3_i4_independent_native_observed_repair_replay.json)
- [I5 original native source pin, tested 39 targets, and zero CI failures](https://github.com/iFrankStudio/cioc/blob/cl1b-ci-evidence/run-records/c02b6c67fb81167de5d89ff99b1dd2397737b9cf-44563928096-1/summary.txt)
- [I5 original native all68-world, every-charged-step and feedback proof](https://github.com/iFrankStudio/cioc/blob/cl1b-ci-evidence/run-records/c02b6c67fb81167de5d89ff99b1dd2397737b9cf-44563928096-1/g3q_c1_p1_l3_i5_independent_native_optimal_onebyte_replay.json)
- [I5 final full success and tested SHA](https://github.com/iFrankStudio/cioc/blob/cl1b-ci-evidence/run-records/c02b6c67fb81167de5d89ff99b1dd2397737b9cf-44563928096-1/result.txt)

## What changed: from a preset repair target to witness-selected executable code mutation

I3's original PID paid for complete structural three-mode certification and accurate flag isolation but used **one fixed source edit target**, always the third byte of its three-word executable grammar. It rescued only **6 of 40** initially structurally incomplete genotypes.

I4's prospectively frozen [180-node source](../studies/B56-F1d1k-G3Q-C1-P1-L3-I4-frozen-180-node-observation-selected-repair-v1.json) exploits the original actor's own *already source-paid certification residual*, `sum(flag7,flag8,flag9) − 3`. In a failed complete three-probe certification this is **−2 if only one output mode was observed, or −1 if two modes were observed**. The actor preserves that value across its paid one-retry guard read and pays a two-way branch:

- **One observed mode** → native `PatchConst` original executable third word **PC6**, copying the actor's own original first-word data from cell62.
- **Two observed modes** → native `PatchConst` original executable **middle word PC3**, setting it to0 with an actor-held register.

After exactly one selected edit, the **same original PID1** resets **all three** structural witness flags, pays for another complete native three-word interpretation, then **either** begins genuine hidden-goal service/feedback learning **or** naturally halts before burning the full compute budget. Original native results: **40 actor-executed source edits; 16 select executable PC6, 24 select PC3; 30 now structurally safe; 10 remain structurally unsafe and are rejected; 0 false-positive/false-negative certificates**. Source180/180, peak occupancy ≤195/300, ≤16 data keys.

Causality is not only an observer inference: **both source-edit-disabled native controls fail to certify or serve tasks**, and independent replay checks exact PC, registers, edited source Const3/Const6, private credits, cell occupancy and feedback provenance at every paid step. The controlled inverted-feedback arm shows a further limit: the original actor can declare success although real task performance is inadequate, because a structural coverage certificate does not authenticate task fitness.

## I5: attain the exact one-byte repair ceiling

The prospectively frozen [173-node source and all64 precommitted genotypes](../studies/B56-F1d1k-G3Q-C1-P1-L3-I5-frozen-173-node-optimal-one-byte-repair-v1.json) makes two further changes without altering any native world law:

1. **Source-level engineering compression:** remove five pairs of `Const(reg1,1)` immediately followed by unconditional `Branch` whose two edges coincide. Redirect each predecessor to the same destination, preserving deterministic interpreter semantics while removing **10 researcher-redundant source nodes**. This is authored compiler-like cleanup, **not source compression invented by the actor**.
2. **More informative repair candidate:** when the one-mode failure branch has selected third-word patch PC6, read the original first word from native cell62; if it is original opcode3 (RESET), do **not** blindly copy the same unusable RESET into the third word. The original actor pays a signed subtraction/branch and uses a shared existing `PatchConst` to set that tail word to0 instead. For first words0/1/2 it still copies the actor-owned byte. For two-mode failure it continues patching middle source word PC3 to0. The actor performs the branch and edit at run time using paid CIL and observed in-world data.

This recovers the last two previously missed repairable genotypes:
- **[3,0,3] → [3,0,0]**, original native PID1 pays its executable PC6 edit at charged step340, then recertifies successfully and handles task service.
- **[3,1,3] → [3,1,0]**, native original PC6 edit at charged step332 followed by successful recertification.

**Finite upper-bound audit:** Independently enumerate **each of 40 initially incomplete three-byte rules** and **every one-word substitution** (three source positions × each alternative value0..3). Exactly **32 genotypes** have *any* one-edit transition to a structurally complete three-mode rule. The other **eight** have none:

`[0,2,3]`, `[0,3,3]`, `[1,1,3]`, `[1,2,3]`, `[2,1,3]`, `[2,3,3]`, `[3,2,3]`, `[3,3,3]`.

Because I5's native original actor rescues **all32 of those32 feasible seeds**, this is an **optimal 32/40 (80%) finite one-byte repair rate under the frozen three-opcode grammar and one-edit constraint**. In the exact 64 original worlds:
- **24** start structurally complete and need no source edit;
- **40** start incomplete, pay for exactly one CIL executable source edit;
- **32** edits yield complete coverage and a subsequent pass;
- **8** remain incomplete; their original PID1 rejects the rule and self-Halts under budget;
- **56/64** original native worlds end with a structurally complete rule;
- **zero false approvals, zero erroneous refusals**, peak source **173/180** and occupied **≤188/300**.

All native registers, costs, original source Const edits, flag resets, external feedback and exact actor-chosen repair decisions match the independently authored CIL replayer for **84,735 paid original CIL instructions, 68 separate original native worlds, first divergence null**. The three source-edit-disabled original controls block progress, including the RESET3 normalization arm. Inverted task feedback again demonstrates that structurally valid source edits do not guarantee true objective success.

## Failure history preserved; no cherry-picking

I4's initial native test passed in [#44559047440](https://github.com/iFrankStudio/cioc/actions/runs/44559047440) but its new Python auditor rejected a legacy hard-coded task-Halt PC. The second [#44560130779](https://github.com/iFrankStudio/cioc/actions/runs/44560130779) repeated a stale Halt-PC assumption. The source and native physics were not altered; the auditor was repaired to inspect actual frozen `Halt` opcodes rather than use copy-pasted PC lists. Exact-SHA successful I4 evidence is #44561002026.

I5's initial [#44563242356](https://github.com/iFrankStudio/cioc/actions/runs/44563242356) did not compile its Rust integration test because `serde_json::json!` was supplied a raw indexed array expression instead of a local `goal` variable. The *test writer only* was corrected, leaving the originally frozen I5 CIL source, expected64-case predictions, semantics and world kernel unchanged. Exact-SHA successful I5 native + independent + publisher run is #44563928096.

## Research boundaries and next priority

**What is now established:** a bounded single original PID1 can pay to diagnose its *own authored code's structural coverage failure*, choose **which executable source byte to edit** based on its native witnessed failure signature, select a repair candidate from its own persistent data (including a conditional alternative for an invalid RESET value), mutate the original executable code, recertify that code with completely fresh witnesses, then perform task-feedback-conditioned parameter adaptation or self-Halt to conserve compute. The finite 64-rule enumeration provides a reproducible complete soundness check and the one-edit success-rate optimum.

**What is *not* established:** PID1 **did not invent** the grammar, repair branching logic, normalization policy, resource budget or task-feedback algorithm. The 64/68 cases are **separately initialized original-PID worlds**, not a continuous evolving lineage across CI experiments. There is no general programming-language repair theorem, no arbitrarily novel API use, and no semantic objective-truth discovery from the single feedback bit. Therefore I5 is **not** CL-1B completion or proven open-ended cyberlife.

**Next engineering falsifier:** the eight mathematically one-edit-impossible genotypes require **at least two edits**. Any two-edit experiment must source-pay both mutations, independently clear all three witness flags between generations, preserve bounded attempt/compute conservation, and compare against exact one-edit optimal I5 controls. Better still, allow the original actor's repair-*policy* to be revised from feedback, not only rule bytes, and prove that change is causally useful under unseen genotype/task regimes.
