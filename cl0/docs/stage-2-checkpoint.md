# CL-0 Stage-2 checkpoint (2026-11-13)

## Implemented
- The Rust world accepts explicitly labeled experimenter-only interventions: disable process, set credits, set cell, patch graph edge.
- Every accepted/rejected intervention produces an external event. Attempts by internally executed CIL remain distinguishable.
- `cl0-lab` persists equal-source matched conditions into one SQLite transaction, using serializable pre-intervention world state.
- Every condition stores initial and final serialized worlds, SHA-256 digests, source-revision label, intervention list, and execution limit/attempt count.
- A replay verifier hashes stored representations and recomputes the complete final state (including event trace).
- A disk-backed test closes and reopens SQLite, then checks both the unmodified branch and zero-budget control replay.

## Scientific interpretation
All positive results are **constructed tests of infrastructure**, not autonomous maintenance, organizational closure, artificial life, or open-ended evolution. Experimental interventions are environmental controls, not hidden survival functions. No internal goal and no specialized repair or reproduction opcode has been introduced.

## Observed validation
Prior workflow run on Stage-2 SQLite code: https://github.com/iFrankStudio/cioc/actions/runs/43396033217 — 12 tests passed (4 lab + 8 world), Clippy passed.
The additional disk-backed regression is only confirmed after its own GitHub workflow run passes. Do not count it as passing until then.

## Unresolved
1. The code storage cap and shared cell capacity remain separate, not a fully uniform resource ledger.
2. `PatchNext` edits one graph edge; program objects cannot yet allocate, clone, remove or transfer.
3. Interventions are pre-execution only. Snapshot forks for intermediate time points require a first-class schedule.
4. SHA-256 is not a cryptographic authenticity guarantee against a malicious database administrator.
5. JSON is deterministic for current Rust structures but cross-version canonical encoding and migration are not specified.
6. Dependency versions are resolved by Cargo, and a committed `Cargo.lock` is still required for strict build reproducibility.
7. Negative controls are exercised in tests and the demo but are not mandatory for all uses of the general `run_matched` API.
8. Experiments are externally initialized; there is still no evidence of naturally emerging structures.
