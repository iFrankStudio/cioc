# CL-1B B56-F1d1d — paid GraphEncode constructs a new opcode, but the execution identity is a distinct process

**Result: mechanistic generic opcode construction verified by real Rust VM execution and independent event-by-event state replay. Strong negative boundary: GraphLaunch transfers the generated graph to a NEW PID2, not a self-modification of original PID1.** This is explicitly a separate researcher-authored founder calibration, not an achievement of original F1c, strict B56-F1d1 same-PID selection, or autonomous novelty.

## Source and preregistration

- [Prospective frozen F1d1d preflight protocol](../studies/B56-F1d1d-paid-graph-encode-novel-branch-structural-feasibility-manifest-v1.md), committed BEFORE experiment code under `e1f905f48324bab86cca370d17a99a2ed8406a8e`. Exact SHA-256 `db966ca77a0d9b29258e213a4c0421f5351e07d2e88ec60096a8a768326bac61`.
- [Successful source-pinned changed-only Actions run44259124488](https://github.com/iFrankStudio/cioc/actions/runs/44259124488), tested `f78b8f6d2403c18465c7d4e72067d17b8ff4921d`: **16/16 selected CL-1B native Rust targets PASS**, **54/54 independent analyses PASS**, mandatory B31/B34 ABI checks PASS, both failure counts0, original exact-source native logs archived. This is a SCOPED regression, NOT final-SHA full63 Rust.
- [Native source](../../cl0/crates/world/tests/cl1b_b56_f1d1d_paid_graph_encode_preflight.rs) · [independent source-reconstructed paid CIL/process/graph replay](../analysis/b56_f1d1d_native_graph_encode_paid_replay.mjs) · [original CI summary](https://github.com/iFrankStudio/cioc/blob/cl1b-ci-evidence/run-records/f78b8f6d2403c18465c7d4e72067d17b8ff4921d-44259124488-1/summary.txt) · [all original paid native events](https://github.com/iFrankStudio/cioc/blob/cl1b-ci-evidence/run-records/f78b8f6d2403c18465c7d4e72067d17b8ff4921d-44259124488-1/b56_f1d1d_native_graph_encoded_branch.jsonl) · [independent native instruction replay report](https://github.com/iFrankStudio/cioc/blob/cl1b-ci-evidence/run-records/f78b8f6d2403c18465c7d4e72067d17b8ff4921d-44259124488-1/b56_f1d1d_native_graph_encode_paid_replay.json).
- Preceding [run44258676358](https://github.com/iFrankStudio/cioc/actions/runs/44258676358) had **63/63 Rust native source tests PASS**, but the independent observer initially failed only due a brittle source string literal requiring no whitespace in Rust `self.processes.insert`; the corrected regex observer in `f78b8f6...` now passes all54 observers. The earlier failure is not discarded or rewritten as green.

## Exact executed world and causality

The parent is an investigator-authored 123-node CIL program, **containing no Branch opcodes**. The substrate physics, credits and caps remain unchanged: `World::with_strict_capacity(16,180,300)` and **1,000 total initial credits** per isolated world. No external SetCell interventions, host graph editing, code replacement or credit replenishment occur. The original parent pays ordinary `GraphNew`, repeated `Const` and `Write` instructions to put five-word instruction descriptors into shared cells40–44, and seven `GraphEncode` instructions to turn **data into seven distinct graph nodes**. B53's needed `Branch(test1)` is possible under the existing generic decoder: encoded descriptor `[5,1,2,3,0]` is a real Branch instruction with independently specified nonzero/zero edges.

The new graph contains `Const r1(0 or1)`, `Branch test1`, two output-choice Const nodes, `Const r2=11`, `Write11`, `Halt`. The **descriptor schedule, branching objective and two test inputs are explicitly authored by the researcher**, not discovered by the actor. PID1 pays to construct and `GraphLaunch` the graph using a **transfer of100 existing credits**. The native kernel does NOT replace the parent executable with this new graph. It creates distinct **PID2**, which executes Branch(test1), correctly selects the branch and genuinely writes data key11 with0 or1. Process IDs and resources are independently checked at every original paid event.

Three complete native worlds:
| Arm | Real paid steps | Parent/child charged steps | Descriptor Writes | Paid GraphEncode nodes | PID2 actual Write11 | Peak strict storage |
|---|---:|---:|---:|---:|---|---:|
| BRANCH_ZERO |129|123/6|35|7|0|138/300|
| BRANCH_ONE |129|123/6|35|7|1|138/300|
| NO_ENCODE |18|18/0|5|0|none (no child)|130/300|

**Totals:** 3 worlds; **276 independently reexecuted native CIL instructions**, **75 paid descriptor-cell Writes**, **14 actor-paid GraphEncode instruction events**, two genuine new-process launches, two genuine PID2 Branch(test1) executions and two Write11 outcomes, **first divergence null**. The same independent JS observer reconstructs the native parent source from current Rust constants and factory semantics, consumes each actual source-pinned event once, independently simulates CIL instruction effects, graph objects, source nodes, registers, both process IDs, credit transfer and every step's data/storage snapshot. The native PID scheduling order is read from trace; this is not an independent scheduler proof. No additional untested benchmark outcome is implied.

The **NO_ENCODE** arm is an explicit researcher-authored mutation of the initial CIL at its first GraphEncode PC17 into Halt; it is not source-identical monitor-edit-only inhibition. It still pays5 initial descriptor Writes and one GraphNew but cannot generate a node or launch a child; no result Write11 occurs. This confirms native GraphEncode is required for the original graph-generating chain in this laboratory.

## Scientific boundaries, compared with F1d1c

- **F1d1c remains true**: unchanged F1c contains only Branch(test3) templates plus CloneNode/PatchNext/PatchConst; literal B53 Branch(test1) is unreachable from that exact original founder.
- **F1d1d establishes a distinct capability**: the same preexisting Rust world kernel has a *more expressive*, genuinely charged `GraphEncode` facility capable of decoding new opcode fields from data. This is a new authored founder with that facility, not an observed mutation of F1c.
- **Critical PID discontinuity**: the only generated Branch(test1) executes in PID2. Preserving literal PID1 as the sole evolving individual, as CL-1B requires, is NOT established. Distinct PID2 cannot silently be reinterpreted as the original individual without redefining and independently testing organism continuity/lineage.
- This remains a highly scripted founder and a simple one-bit-output function, not creation of a useful monitoring strategy, feedback-aware algorithm invention, task transfer, service repair, self-revision of mutation rules, or open-ended evolution.

**Next strictly separated research gate:** check if a generic paid **same-PID** source rewriting route exists without adding new kernel physics, or explicitly preregister a separate persistent graph-identity notion that measures continuity through GraphLaunch. Compare stable identity, credits, self-maintenance and replayable causal provenance. Do NOT label that identity change as achieved today. Strict N56-A/B/C, B3/B4 and cyberlife remain UNMET.
