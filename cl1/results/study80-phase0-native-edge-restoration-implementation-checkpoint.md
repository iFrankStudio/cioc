# CL-1 Study80 Phase-0 — source-inspected native edge-restoration fixture (verification pending)

**Status: code submitted to main; NOT YET CI-VERIFIED; ZERO new stochastic genomes.** Test implementation: `cl0/crates/world/tests/study80_native_restoration.rs`, commit `775ebbb2afca4d0ae0161c26b01c414806854191`. This document is an implementation checkpoint, **not a positive experimental finding**.

## Actual Rust/CIL source audit

The Study77/78 seeded grammar uses only Read/Write/Add/Sub/Const/Branch/DrawCompute and did **not** include mutation, native graph repair or inherited code construction. Its externally sampled native energy/feedback successes must not be reused as evidence of self-modification.

The existing `Op::PatchNext` operates on the caller's actual executable `Process.code.nodes` and resolves both edited node and replacement edge from process registers. `Intervention::PatchEdge` changes that same program edge with explicit `Origin::External` attribution. A program may therefore contain an actor-executed edit route that is not itself disabled by an external fault to a different instruction edge. This is a **generic VM capability**, not a life objective or a new repair opcode.

## Human-scripted mechanism sequence, per input source low3/high8

All instructions are authored by investigators, and seeded world count is exactly **zero**. One process loads 6 prewritten opcodes with 48 process credits. Three setup Consts hold input cell key4, output cell key9 and the indexed edge location. The native loop contains `Read -> Write -> PatchNext -> Read`. The researcher externally replaces the successful native `Read.next=Write` edge with `Read.next=PatchNext`, so the immediate post-lesion read no longer produces a new Write. Native `PatchNext` then restores the original edge using ordinary process registers, and the next native Read again reaches the same Write PC. Both source values 3 and 8 are measured in independent scripted fixtures. Successful restored output must equal its own unchanged input; therefore high-low output difference must remain 5.

Three mandatory fixture conditions are encoded:

- **Real external edge lesion + actor-native edit:** first post-lesion native Read skips output; native PatchNext changes actual executable code; later native output returns without any host restoration.
- **Mutation-disabled negative:** same lesion and equivalent native execution-loop opportunity, but the authored edit opcode is replaced by a charged generic Const, so no native edge restoration occurs. This is a **fixture-level source-code comparator**, *not yet* a byte-identical-genome runtime mutation-ablation law suitable for prospective randomized causal inference.
- **Explicit external restoration positive:** same mutation-disabled baseline and lesion; an external PatchEdge restores the exact original edge, and the expected native output returns. Must remain attributed External.

The exact pre-lesion initial program is not byte-identical across the scripted positive and mutation-disabled fixtures. This is acceptable for a **mechanism calibration only**, and is **NOT** acceptable as proof that native self-modification caused evolutionary or endogenous ecological restoration in independently sampled programs.

## Acceptance and blockers

Run `cd cl0 && cargo test --locked -p cl0-world --test study80_native_restoration` and `cargo test --locked --workspace` on the source-pinned commit, then archive test counts and CI URL before reporting Phase-0 executable success. No such CI result is asserted here.

Before any 128-genome Study80 ecology: (1) add a properly preregistered, native-edit-only execution-ablation mechanism preserving byte-identical initial code and one charged attempt where eligible; (2) predefine the precise pre-lesion target and paired source-dependent loss window; (3) independently audit the actor-PC and actual code bytes/edge changes across epoch snapshots; (4) prove at least two post-lesion independent output windows and capture sham and Draw-disabled arms. The six-opcode fixture alone cannot meet the full Study80 ecological restoration gate, multi-component mutual dependence or life.

**Decision:** Preserve the minimal mechanism test and do not execute or interpret Phase-1 stochastic ecological outcomes until this test and the exact ablation semantics have been verified. CL-1 remains OPEN.
