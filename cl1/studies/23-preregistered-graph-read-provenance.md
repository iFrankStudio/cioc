# CL-1 Study 23 — preregistered observer-only source-graph read provenance

**Registered 2026-11-13 before editing the VM transitions or inspecting a Study-23 result.** The frozen Study-22 random ecology has been completed and has a source-pinned, independently audited negative functional finding: external compute can restore late activity and copying, but not late native shared-cell writes. Study 23 is a **technical provenance calibration**, not a new stochastic cohort and not a cyberlife-positive experiment.

## Falsifiable instrumentation question

Can every successful native CIL `GraphReadTag` and `GraphReadField` instruction identify the *actual code-containing graph object and owning source*, the read node/field, the returned value and the actual reader, without modifying execution or resource budgets?

## Frozen event contract

Introduce two additive observer-only `EventKind` variants:

- `GraphTagRead { graph: u64, owner: ProcessId, node: usize, tag: i64 }`: successful native `Op::GraphReadTag` only.
- `GraphFieldRead { graph: u64, owner: ProcessId, node: usize, field: usize, word: i64 }`: successful native `Op::GraphReadField` only.

Both replace that instruction's generic `Executed` event; the outer event retains `Origin::Internal(actor)`, process/PC, tick and postinstruction credit count. `owner` must be the **actual owner of the read graph**, not the reader or a guessed source. The value is obtained from the actual graph node at execution time and is observer-only metadata. A failed read (unknown graph, revoked permission, invalid node/field, malformed code) must retain `Trapped` with **no successful-read event**. No new instruction or privilege; no extra computation, data, graph nodes, ticks or memory allocation. Previously serialized events/world snapshots remain deserializable because only additive enum variants are introduced.

## Independent targeted controls and regression gate

1. Native permission fixture: owner1 constructs a graph and grants read access to process2; process2 successfully reads the graph's tag. Assert `GraphTagRead` references source graph1/owner1, node0 and actual tag (0 for Const) with reader2's internal origin and original PC/credits, *not* a generic `Executed`.
2. Native field fixture: successful `GraphReadField` must record the precise node, field index and encoded word with source owner; an unauthorized reader must trap without false successful read.
3. Existing permission revocation tests, full Rust workspace format/test/lint, all prior preregistered random and mechanism studies' deterministic SQLite full-world and epoch replays and independent Python audits must pass. Compare successful instruction counts, process registers and compute inventory to preinstrumentation behavior; changed event hashes are expected only on reexecution under the new source revision, not grounds to rewrite archived scientific results.

## Scientific boundary

Source-graph read metadata allows observer discrimination of reading *someone else's graph* from reading one's own code. It does not establish register/dataflow taint through arithmetic or a causal effect on a later shared-cell write. Functional source sensitivity still requires an explicit paired content-change intervention and registered output. An internally executed read of researcher-authored code is not autonomous information invention; exogenous compute and injected graph restoration are not native self-maintenance. **CL-1 positive milestone remains unaccepted** irrespective of the technical result.
