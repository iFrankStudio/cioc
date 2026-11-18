# CL-1B B56-F1d1f — exact native SAME-PID paid data-program change and causal feedback ablation

**Accepted, narrowly:** an existing, continuously running native PID1 pays to alter its **data-encoded virtual program** in response to a real paid feedback Read, then uses that updated data on a second service trial. **Not accepted:** native executable opcode synthesis within PID1, autonomous invention of either operator or the update algorithm, a reusable general interpreter, B53 receipt authentication, original strict F1d1, B3/B4 or cyberlife.

## Evidence chain and protocol timing

- Frozen **before** native code and results: [F1d1f 12-world causal preregistration](../studies/B56-F1d1f-paid-same-PID-data-bytecode-feedback-switch-manifest-v1.md), committed at `7153089acd918ae3cfee75f3e02bbb158d439f46`, SHA-256 `1827f1da4bbf8a4ce1e6a3d205f675f36350809262563d27cbde6cb4bf664402`.
- [Full original source-pinned CI run44264236954](https://github.com/iFrankStudio/cioc/actions/runs/44264236954) at **tested source SHA `f009c38568555868cb53eddd0d9b86673c991f15`**: **64/64 actual Rust integration targets pass**, **55/55 independent observers pass**, mandatory B31/B34 ABI pass, both failure counters0 and exact run evidence published by separate least-privilege job.
- [F1d1f actual Rust paid CIL native integration source](../../cl0/crates/world/tests/cl1b_b56_f1d1f_same_pid_virtual_bytecode.rs) · [independent per-instruction JS interpreter](../analysis/b56_f1d1f_same_pid_virtual_bytecode_replay.mjs) · [original raw JSONL native history](https://github.com/iFrankStudio/cioc/blob/cl1b-ci-evidence/run-records/f009c38568555868cb53eddd0d9b86673c991f15-44264236954-1/b56_f1d1f_same_pid_virtual_opcode_steps.jsonl) · [complete independent native replay/controls report](https://github.com/iFrankStudio/cioc/blob/cl1b-ci-evidence/run-records/f009c38568555868cb53eddd0d9b86673c991f15-44264236954-1/b56_f1d1f_same_pid_virtual_bytecode_replay.json) · [original source-pinned summary](https://github.com/iFrankStudio/cioc/blob/cl1b-ci-evidence/run-records/f009c38568555868cb53eddd0d9b86673c991f15-44264236954-1/summary.txt).
- This was a real **full** CI campaign, not a changed-only 16-target sample. Native 64 includes the newly committed F1d1f integration target and all original regressions. No test output was retroactively substituted for original evidence.

## What the one-PID actor actually pays for

A **new preauthored** 34-node CIL program, unchanged in all 12 worlds, executes in strict `World::with_strict_capacity(16,180,300)` with **250 finite initial paid credits**, no compute pool and one original PID1. All native executable code graph nodes remain byte-for-byte unchanged across all original events. The external environment supplies only input key7 and after-first-output correctness feedback key20; the host never patches native code, changes the actor's virtual program key30, or provides extra credits.

The source contains a small fixed interpreter for **two investigator-authored virtual opcodes**:
- `key30=0`: return original input using a native Branch(test3) followed by a real service Write11.
- `key30=1`: return arithmetic negation of input using a native Sub followed by a real service Write11.

Before the first service, PID1 itself pays Write30(0) and Write31(0), which sets the initial virtual opcode and stage. After an actually executed first service, the environment supplies a one-bit correctness signal in key20 and a held-out next input in key7 (two observable external interventions per world, plus the initial input intervention). The same PID1 pays for **Read20** and a branch. If feedback is0, PID1 **pays Write30(1)** and then Write31(1); if feedback is1 it only marks the second stage and keeps Write30(0). The same 34-node native CIL program executes a second service with its new interpreted virtual opcode and then halts. Nothing is launched as PID2, and no source opcode type changes.

## Exhaustive recorded outcomes, all preregistered

Two source-independent integer seeds `input5,heldout11` and `input7,heldout−7`; two hidden task types `IDENTITY` and `NEGATE`; three feedback regimes `TRUTH`, `BLIND_SUCCESS`, `INVERTED`. Exactly **12** identical-source native worlds, each producing **two actual charged service Writes** (24 total). Their respective second-trial correctness counts:

| Feedback signal | Matched held-out correct | Genuine PID1 virtual-mode changes | Scientific interpretation |
|---|---:|---:|---|
| TRUTH | **4/4** | 2 | Feedback-dependent correct selection under the 2-option authored toy task |
| BLIND_SUCCESS | **2/4** | 0 | Fails on both hidden NEGATE cases |
| INVERTED | **0/4** | 2 | Incorrect feedback causes systematic wrong selection |

All first NEGATE-task outputs use identity and are wrong; first IDENTITY-task outputs are correct. **No claim that the actor invented the NEGATE algorithm:** both alternatives and the learning rule are present in its authored source. This ablation supports a narrow causal **signal-to-paid-data-write-to-service-output** mechanism, not general adaptability.

Independent replay reconstructs *all CIL nodes directly from the current Rust source*, not a researcher-written predicted output table; executes and compares **400 actual paid native CIL instructions one by one**, **36 source-pinned external interventions**, all 12 real feedback Read20 events, all **4 actual PID1 Write30(1) revisions**, all **24 actual service Write11 events**, every register/PC/credit/cell/storage/source snapshot, no extra processes and final states. **First native divergence = null**. Native `max storage=40/300` in every world, and data cell count at most5 vs strict16; all code graphs exactly34 nodes vs maximum180. Source code SHA-256 `05a9faf3d80ec3a332974f38c3ab11df4094fe87c0658122f8fb9ff605711770`.

Total 400 charged native instructions: **32** for unchanged-mode worlds and **36** when the actor pays extra for a virtual opcode switch. There is **no free mutation**, hidden branch source replacement or process-ID relabeling.

## CL-1B status and next necessary falsifier

- **Improvement over F1d1d:** the updated policy actually executes in the *same continuous PID1* instead of a generated PID2, and updates are traced to paid CIL data writes. This resolves the narrow PID continuity problem for **data-mediated two-choice behavior**, not for native-code opcode creation.
- **F1d1e reachability limit still stands:** generic GraphEncode and GraphReplace are graph-object mutations; direct same-PID native executable source edits cannot invent previously absent opcodes under the unchanged kernel. F1d1f's stable source bypasses this limitation with a *fixed interpreter* and editable data. These are not contradictory results.
- **Critical limits:** the source is manually authored with complete IDENTITY and NEGATE branches and an authored feedback decision rule; feedback truth is supplied by the environment and is spoofable; only two tasks/seeds and one update are tested; no generative novelty, general bytecode VM, composition of unforeseen operators, post-shift long-run utility, de-novo monitoring, or original B51c task-search preservation is established.

**Next proposed gate (prospective, NOT yet tested):** design a genuine multi-instruction *data-bytecode* interpreter retaining PID1, using ordinary priced memory for a sequence of opcodes and program counters, and test whether paid actor modifications can compose an algorithm not present as a complete original source path, with held-out tasks, blind and inverted controls, exact native replay and zero-free-edit accounting. Creating several prewritten branches alone is not an open-ended solution; generic extensibility and failure boundaries must be explicitly scored.

**Original B56-F1d1 paid, authenticated B53 monitor construction, N56-A/B/C, B3/B4 and cyberlife NOT MET.**
