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

## Follow-on strict matched-program comparator (source implemented; CI outcome unverified)

Subsequent source commits `c6965fb`, `ff3e12a`, `9bd7ba2`, `b200465` and `e167ddf` add a **disabled-by-default, serialized** `World::with_native_code_edits_disabled()` mode, with a typed `NativeCodeEditSuppressed{opcode_tag}` native observer event. It intercepts PatchNext, PatchConst, CloneNode, GraphCopyNode/Local, GraphSetNext/Const, GraphEncode, GraphReplace and GraphCopyFrom; it is **not** a blanket ban on graph allocation, graph destruction or launching preexisting executable graphs. A prospective ecology using other graph-changing instructions must either explicitly exclude them from its preregistered grammar or extend the ablation scope in a separately source-frozen step.

Every intercepted native edit is still an ordinarily charged CIL attempt. A shadow clone validates the unchanged original opcode against the ordinary VM (including malformed operands, ownership, storage limit and next-edge failures); on valid success, only the shadow next-PC is retained and real code remains unchanged. Failed edits continue to trap identically. This costs additional host simulation work but grants no CIL-visible information or credits.

The focused Stage80 test now checks **the same exact loaded Process/code nodes and initial credits** for both laws under input3 and input8, actual externally attributable lesion, absent first post-lesion native output, a charged real native PatchNext versus explicit suppressed event, and repeatable subsequent native output in **two further disjoint 3-attempt execution windows** only when edits are permitted. A malformed PatchNext (invalid target node) must trap identically in both laws and leave executable code unchanged. Host repair remains separately labeled `External`.

These are *human-written deterministic fixtures*, not emergent code repair, not statistically independent samples, not a multi-component organization. The use of `shadow.execute` validates ordinary operation legality; the final hash/replay/CI check has **not** yet been independently observed through the currently available connector (commit combined-status checks are empty and the commit-workflow-run connector only enumerates pull-request-triggered runs). A dedicated short `cl1-study80.yml` push workflow was added specifically to avoid rerunning historical study pipelines. **Do not label Phase 0 passed until a real workflow conclusion or reproducible local test record is checked.**

## Original Phase-0 SQLite and independent observer (implementation after this checkpoint)

Source commits `f463021b`, `e5ae5e8`, `96783be`, `16898a5` introduce a deterministic `cl1-study80` Rust laboratory runner with **10 human-scripted original control runs** (source3/source8 × intact/sham/lesion-edit/lesion-disabled/lesion-host-repair), 12 external epochs ×2 charged native attempts per run, for **120 planned epoch snapshots**, original SQLite full-state hashes and exact Lab replays. No generated stochastic seed-worlds are added. The source program is the same six-opcode graph in all ten runs; only explicit source input and serialized environment edit-ablation law vary. Damage is externally imposed at epoch3 and explicit host repair at epoch4 in its separate control. Epoch3 checks the actual first loss, and later windows check repeated source-conditioned output under internally executed native PatchNext. This runner is source-committed, **not independently confirmed as executed**.

The read-only `cl1/analyze_study80.py` verifies all 120 exact serialized state SHA256 hashes, source revisions, all ten registered labels, identical initial code across low/high and treatment arms, native events and PC, external edit provenance, true mutation-disabled behavior, and compute inventory. GitHub Actions `cl1-study80.yml` runs the focused Rust tests, VM library regressions, original SQLite calibration runner, independent Python audit, then uploads both SQLite and JSON. An invalid PatchNext negative-control diagnostic was corrected to the actual VM `invalid edge` trap wording (commit `fd94a738`), without altering the intended illegal-operation contrast.

**Status strictly pending CI evidence:** Do not add 120 to any *verified* corpus totals or claim repair success until the focused workflow reports success and the original archive can be independently inspected. Stage80 Phase-1 ecology remains gated.
