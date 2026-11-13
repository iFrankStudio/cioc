# CL-0 Stage 3.2 checkpoint — 2026-11-13

## Evidence boundary
All behaviors below are from externally constructed CIL fixtures, not spontaneously discovered organizations, organisms, self-maintenance, or open-ended evolution. New graph construction and launch are language/VM abilities, **not** a built-in reproduction policy.

## Implemented
- Bounded worlds can allocate owned graph objects with monotonically increasing numeric handles.
- Graph objects contain instruction nodes copied from existing CIL code and can be patched through universal low-level operations; no semantic repair oracle is invoked.
- Owner authorization is checked before modifications or deletion; guessing a handle does not confer access.
- Empty graph headers cost one storage unit; appended nodes each cost one; dropping returns allocation; launching moves nodes into a new process and returns the object header.
- Launch transfers compute credits from parent to new process; no credits are created.
- SQLite matched branches reconstruct complete world state, including graph objects and launched processes.

## Verification and failures
- Initial implementation commit `74b455c`: CI failed in three Stage-3.2 test fixtures because source-node register values and scheduler-order assumptions were inconsistent; the existing older suite passed.
- Commit `11f5737`: corrected two fixture assumptions. CI still failed for the remaining invalid-edge fixture.
- Commit `6b76b09`: fixed that fixture. CI run https://github.com/iFrankStudio/cioc/actions/runs/43398951758 **passed** workspace tests and strict Clippy.
- A separate laboratory integration test was then added in commit `d665671`; its corresponding workflow must be checked independently. Do not infer it passed from the earlier run.

## Negative controls
- No computation budget means no object allocation or process launch.
- Empty remaining storage blocks new objects and copied nodes.
- Wrong-owner object edits trap without modifying the target graph.
- Invalid graph edges do not modify the graph.
- A dropped object frees its header and node units; graph launch is a move, not a copy.

## Limitations and next falsifiable gate
- Object assembly copies existing opcodes, and only single-successor edges/Const literals can be mutated. Branch edge editing, arbitrary opcode creation and graph object sharing are absent.
- Ownership is process-ID based. There are no transfer capabilities, revocable permissions or interprocess objects.
- No bounded object deserialization schema for adversarial snapshots; SQLite data are scientific inputs, not trusted authority.
- VM resource units are abstract. Process metadata, registers, heap internals and observer-side event logs are still excluded.
- The CI currently runs `cargo fmt --all` rather than `cargo fmt --all -- --check`. This does not enforce checked-in formatting.
- No mandatory experimental negative controls are enforced by the general research runner.
- Cross-version snapshots are not compatible by guarantee, and Cargo.lock has not been committed.
- No emergent maintenance or evolution experiment has been conducted.
