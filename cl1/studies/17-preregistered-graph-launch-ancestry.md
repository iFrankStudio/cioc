# CL-1 Study 17 — preregistered graph-to-process ancestry observation

**Registered 2026-11-13 before the new event implementation.** This is a technical lineage calibration, not a cyberlife experiment.

## Contract

A successful native `GraphLaunch` must emit exactly one observer-only structured event `GraphLaunched { source_graph, parent_process, child_process, transferred_credits, moved_nodes }`. The event is attributed to the internally executing parent process at the original PC and tick; the newly started process receives **no newly minted computation or code nodes**. The event replaces generic `Executed`, and must not appear for a trap, unauthorized graph or insufficient credit.

The observer must be able to link executable graph provenance (GraphCreated, GraphNodeCopied, GraphCopied, GraphEncoded, GraphReplaced and GraphNodeEdited) to a newly launched process; it **must not infer autonomous reproduction or effective novelty** merely from launch or source ancestry. Preserve original world and strict storage invariants, source-tagged external interventions, deterministic full epoch replay and all earlier protocol outcomes.

## Technical gate

Use existing `GraphLaunch` success and insufficient-credit/trapped fixtures, asserting source graph, child ID, parent, transferred credits and moved node count on the success path. Strict Rust format, tests, Clippy, CL-1 independent analysis and full replays must pass. If not, do not call Study 17 technically complete.

## Boundary

Launch ancestry is one necessary piece of a causal test of an internally maintained multi-component function. No new VM language primitive, seeded repair rule, fitness function or hidden scheduler decision is authorized. Positive CL-1 acceptance remains OPEN.
