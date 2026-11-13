# CL-0 Current System Specification — accepted infrastructure (2026-11-13)

> **Normative current-state reference.** This document supersedes the feature-status claims in the early [Stage-1 specification](specification.md). It does **not** retroactively modify the meaning of historical Stage-1–3 experiment results. The [acceptance contract](CL-0-acceptance.md) defines the scope of “CL-0 complete.”

## 1. Scientific and execution boundary

The world is an **information/computation/storage** environment with no native biological clock, body or organism. Its host enforces generic resource and interpreter rules but never installs a built-in `REPAIR`, `REPRODUCE`, `FITNESS` or life classifier. Program construction, stopping and restarting are mechanical operations, not survival judgments.

The runtime's abstract ordering uses deterministic instruction attempts and logical ticks; no wall-clock duration is observable as an intrinsic environment variable. The scientist supplies initial programs, configured budgets and separately labeled external interventions. Model evidence must never count those inputs as internally evolved knowledge or resources.

The implementation keeps bounded integer registers and finite CIL graph nodes. It is an experimental mathematical world, **not** a fully isolated service for executing adversarial code: persisted snapshots and experimental configurations still require trusted, bounded inputs.

## 2. Components and ownership

| Subsystem | Location | Responsibility |
| --- | --- | --- |
| World kernel | `cl0/crates/world/src/lib.rs` | CIL interpreter, deterministic process scheduler, storage/compute rules, access checks, complete world event trace |
| Laboratory | `cl0/crates/lab/src/lib.rs` | Matched conditions, intermediate scheduled intervention snapshots, SQLite persistence, hashes, deterministic replay |
| Exploratory fixture | `cl0/crates/lab/src/bin/cl0-explore.rs` | Externally seeded initial worlds and a fixed four-condition reproducibility experiment |
| Python analyzer | `cl0/analysis/analyze.py` | Independent read-only SQLite analysis, hash validation, descriptive JSON/CSV |
| Vue observer | `cl0/console/` | Browser-local read-only rendering of exported JSON; cannot issue CIL actions |

Run programs with `cargo run --locked -p cl0-world` or the laboratory binaries. The explorer's CLI accepts `<database> <replica_count> <initial_seed>`; its default reference acceptance invocation is 24 replicas beginning at seed 54129.

## 3. State transitions and provenance

A `World` contains multiple `Process` records (code graph, program counter, four integer registers, remaining credits and stop state), public shared integer cells, owned `GraphObject` records, configured capacity, transferable compute pool, scheduler cursor and events. Round-robin selection uses stable ordered process IDs. Each attempted CIL instruction costs one credit, whether it succeeds or traps. A trap stops that process and records the error. Arithmetic uses specified wrapping integer semantics.

Trace origin distinguishes `Origin::Internal(process_id)` from `Origin::External`. Researcher-side interventions, including credit injection/withholding, shared-cell edits, process disable and graph-edge patching, are explicitly external. Rejected interventions remain observable. The host's interpreter and scheduling rules are neither an agent nor a separate hidden repair process.

`World::run(limit)` is bounded by instruction attempts, not elapsed wall time.

## 4. Resource accounting

The accepted exploratory baseline uses:

```rust
World::with_strict_capacity(shared_cell_limit, max_nodes_per_process, total_units)
```

In **strict** worlds, occupied abstract storage units equal:

```text
shared_data_cells
+ all process code nodes
+ all graph-object headers and contained nodes
+ process headers (one each; includes a fixed register allowance)
+ graph read-grant entries (one each)
```

Creating, copying and releasing objects must respect the relevant limits. The 24-seed baseline starts with 3 processes × (12 nodes + one process header) = **39 units**, under a total capacity of 64 units. Unit counts are not bytes, GPU operations or calibrated physical energy. Neither observer-side SQLite/trace storage nor all actual Rust heap allocations are simulated-world resources.

Instruction credits exist in process-local accounts and may be transferred through generic CIL operations. The world also has a compute pool that CIL may draw from, subject to its instructions and accounting. Crucially, supply or removal of pool credits by the researcher is an **external event**; pool replenishment is not evidence of spontaneous energy generation. The entire inventory must remain arithmetically consistent.

`World::new` and `with_total_capacity` are retained for historical compatibility, with earlier allocation conventions. **Use `with_strict_capacity` for new CL-0-compatible experiments**; do not compare raw capacity metrics between strict and legacy worlds without labeling the model.

## 5. CIL and executable information

CIL programs are graph nodes with typed operations and explicit successor edges. The kernel supports:
- finite-register arithmetic, branching and shared-data access;
- self-graph inspection, edge changes, instruction-node copying and node literal updates;
- creation, inspection, modification, deletion and process launch of bounded executable graph objects;
- conversion of five ordinary integer data cells into an instruction via `GraphEncode`; decoding covers existing opcode tags and uses atomic validation;
- graph read grants and revocation, plus read-only instruction tag/field introspection;
- generic compute drawing, returning and transferring.

The **[five-cell opcode ABI](stage-3.3-instruction-abi.md)** is the authoritative numeric opcode table. Versioned opcode vocabulary is fixed at the interpreter level: CIL may assemble new combinations and control flows using world data but cannot invent a fundamentally new Rust-host primitive.

`GraphLaunch` moves program nodes from an object into a new process and transfers existing compute credits. It is not a privileged “reproduction” instruction. Read permissions are per-graph/per-process and revocable; public shared integer cells are not private. Full graph ownership delegation and persistent/working-memory tiering are **not** part of CL-0 acceptance.

## 6. Laboratory interventions, snapshots and replay

`run_matched` executes initial-intervention conditions; `run_scheduled` executes `ScheduledCondition` entries using nondecreasing `after_attempts` offsets. Every condition starts from a byte-identical serialized baseline. An action at offset zero precedes CIL execution; an action beyond an inert world's reachable attempt count is **not** silently applied. Multiple actions at one offset follow their specified order.

SQLite stores initial states, interventions, final worlds and hashes; scheduled experiments additionally persist reached pre-intervention checkpoints with hashes, allowing auditable forks at particular execution positions. `verify_replay` and `verify_scheduled_replay` compare reconstructed final state and full event trace. SHA-256 checks provide consistency detection, **not** authenticated protection against an attacker who rewrites the whole database. Cross-version deserialization and replay are **not** guaranteed, so retain format, source revision and pinned dependency versions with every dataset.

## 7. Exploratory baseline: engineering evidence only

The [Stage-5 protocol](stage-5-exploratory-baseline.md) creates three researcher-seeded 12-node processes in each world and compares four conditions: untreated, all process credits zero, disabling one process after eight attempts, and externally supplying compute-pool credits. No CIL process is assigned a biological or predefined cooperative role; the generator deliberately supplies structural diversity **from outside**.

Acceptance evidence: 24 independent seeded initial worlds, **96 recorded conditions**, **96/96 exact replays**, **48 passing Rust tests**, **3 passing Python tests**, successful Rust lint/format checks and Vue production build. See [the exact verified workflow](https://github.com/iFrankStudio/cioc/actions/runs/43406928175), [artifact](https://github.com/iFrankStudio/cioc/actions/runs/43406928175/artifacts/14027303037) and [acceptance report](CL-0-acceptance.md).

This establishes repeatable world mechanics and a falsifiable laboratory. **It does not establish autonomous maintenance, organizational closure, or open-ended evolutionary novelty.**

## 8. Reproduction, change control and next phase

The [CL-0 README](../README.md) contains the exact commands; CI runs pinned Cargo/npm dependency lockfiles, `cargo fmt --all -- --check`, Rust tests and strict Clippy, Python tests, a 96-run explorer, read-only analysis and Vue build. The acceptance result applies to the verified implementation. A substantive future code change requires fresh end-to-end validation and a new experiment revision; do not retrofit historical result records.

CL-1 requires an **independent research protocol** for endogenous dependence, persistence, information novelty and perturbation recovery, with preregistered thresholds and explicit negative outcomes. See [CL-1 handoff](CL-1-handoff.md). No biological-likeness goal is required.
