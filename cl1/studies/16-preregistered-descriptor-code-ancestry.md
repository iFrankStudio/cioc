# CL-1 Study 16 — preregistered descriptor-to-code ancestry instrumentation

**Registered 2026-11-13 before modifying the relevant VM transition events.** Prior Studies 01–14 and the Study-15 I/O observation results are known. **This is instrumentation and calibration, not a life hypothesis.**

## Scope and event contract

Record the *successful* conversion of five consecutive shared-cell words into executable graph code. Introduce two observer-only structured events:

- `GraphEncoded { graph: u64, owner: ProcessId, new_node: usize, descriptor_base: i64, descriptor: [i64;5] }`
- `GraphReplaced { graph: u64, owner: ProcessId, node: usize, descriptor_base: i64, descriptor: [i64;5] }`

Emit only after successful native `GraphEncode` or `GraphReplace`, with `Origin::Internal(actor)`, the actual instruction PC and normal one-credit charge. Invalid descriptors, missing cells, denied ownership, storage overflow and invalid node replacements remain `Trapped`, with no positive event. No other event or tick is created; the specific success event replaces plain `Executed` on that instruction. External SetCell interventions stay external. Preserve prior snapshot/event decodability, graph budget and behavior.

An observer can reconstruct *which five cell values* were used, including negative signed literals, and then join those descriptors against known prior CellWritten events or external interventions. If a descriptor cell was never written within observed history, mark its source as **unknown**, never infer spontaneous creation. If multiple writes occurred, use the latest prior writer; write provenance is not automatically causal authorship.

## Independent technical validation

Use two pre-existing generic-code fixtures that assemble valid instructions from shared-cell descriptors; assert successful encode/replace event attributes and lack of a second artifact allocation. Use the existing malformed descriptor fixture to assert `Trapped` and absence of positive event. Exact deterministic replays and full CL-0 / CL-1 Rust/Python/CI suites must pass.

## Scientific boundary

A chain of source cells, decoding and launch still does *not* demonstrate autonomous maintenance: externally authored descriptor values may merely be copied or invoked by native CIL. Functional autonomy requires source-sensitive output counterfactuals, multi-component damage and internally initiated recovery in separate windows with appropriate controls. Stage acceptance stays OPEN regardless of this instrumentation result.
