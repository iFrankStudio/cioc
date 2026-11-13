# CL-0 Stage-1 engineering checkpoint — 2026-11-13

## Provenance
- Source commit: `aab1e8e3f83c0983947297262b56dc615cd82e26` on `main`.
- GitHub Actions run: https://github.com/iFrankStudio/cioc/actions/runs/43391739328
- Verified result: `cargo test --workspace` **passed** (six Rust unit tests); `cargo clippy --workspace --all-targets -- -D warnings` **passed**; `cargo fmt --all` ran successfully.
- Verification was on GitHub Actions Ubuntu runner, **not** in the authoring container (which has no Rust toolchain).
- Rustfmt was run in CI (formatting), not as a no-diff formatting gate; establish a format-check gate after applying formatting to repository sources.

## Scientific status
No emergence experiment has been conducted in CL-0. Engineering fixtures use externally constructed CIL programs, with no life-specific behavior in the host. All detected behavior is expected from the provided instruction graph. Do not call this self-organization or cyberlife.

## Verified Stage-1 kernel properties
- deterministic ordering and replay of cloned world state;
- finite per-process instruction credit, with traps charged;
- storage-capacity rejection without shared-write side effects;
- inert/zero-budget negative controls;
- deterministic invalid-edge trap;
- generic patching of one control-flow edge.

## Known limitations / open issues
- In-memory event trace is not an independent durable audit log; SQLite is not implemented.
- Program node count is fixed, and only single outgoing edges can be patched.
- No dynamic process creation, storage reclamation, graph copying, permission model or resource exchange.
- No intervention framework, independent snapshot serialization, hash chain, multiseed batch analysis, quantitative novelty or causal closure metrics.
- Initial graph size cap is enforced at externally provided `load`; code graph memory and durable storage have **different accounting models**. Claiming full resource conservation across all memory types would be premature.
- The instruction credit unit is an abstract tariff, not a calibrated measure of physical energy or CPU time.
- Rust tests demonstrate local expected semantics, not safety against arbitrary untrusted serialized graphs or long-run performance.

## Next gates
1. Make resources and executable/data storage use one auditable allocation model.
2. Add canonical snapshot serialization + SQLite run registry with provenance and replay.
3. Introduce immutable baseline and matched causal intervention runner, clearly recording experimenter inputs and their effects.
4. Add graph object construction/copying/mutation through generic low-level operations with caps; no repair or reproduction primitive.
5. Only then preregister and execute exploratory organizational hypothesis tests.
