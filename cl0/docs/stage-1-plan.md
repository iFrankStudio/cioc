# CL-0 First Stage: implementation and acceptance

## Phase 0 — freeze semantics
Deliver architecture and CIOC transition audit; define computation prices, storage bound, invalid-instruction outcomes and prohibited host-level life behaviors.

## Phase 1 — minimal executable Rust kernel
A standalone dependency-free Rust crate with bounded CIL graph, deterministic per-instruction scheduler, integer storage, instruction budget, explicit event origins, and a tiny demonstration CLI. Keep legacy experiments untouched.

## Phase 1 acceptance (required, NOT life criteria)
1. Two independently created equal worlds produce exactly equal events and final states.
2. A zero-credit program performs no mutation, and a cost-limited program cannot run for free.
3. Shared cell capacity is strictly enforced; rejected writes cannot mutate the state.
4. Invalid program/edge/register traps are deterministic.
5. CIL can change a general graph edge without any built-in repair function.
6. An inert-program negative control causes no storage changes.
7. CI runs rustfmt, tests, and Clippy. If the runtime isn't available, record verification as pending rather than claiming it passed.

## Further phases (not part of Phase 1 acceptance)
- Stage 2: external interventions on cloned snapshots, independent causal experiment records, SQLite persistency and replay.
- Stage 3: unified resource-priced code/data objects, graph insertion/removal/copy, permissions, stable serialization.
- Stage 4: Python analysis of independent SQLite records and Vue 3/TypeScript inspection (never controls hidden world outcomes).
- Stage 5: pre-registered exploratory conditions, matched controls and prospective open-ended novelty metrics.

## Explicit non-goals of Phase 1
No agents or organisms; no repair/replication mechanism; no adaptation claims; no statistical emergence analysis; no GUI and no SQLite yet. Initial code and resource grants are externally engineered fixtures.
