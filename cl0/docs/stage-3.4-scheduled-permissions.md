# CL-0 Stage 3.4 — Read grants and scheduled causal intervention

## Scientific status
Infrastructure milestone, not evidence of life. Neither the access-control layer nor the intervention runner implements a fitness score, repair policy, reproduction criterion, or agent objective.

## Graph object read permissions
A bounded world now supports:
- `GraphGrantRead { graph, reader, next }`: the graph owner grants **read-only** access to an existing process ID.
- `GraphRevokeRead { graph, reader, next }`: the owner removes the named permission.
- `GraphReadTag { dst, graph, node, next }`: owner or granted reader obtains the **numeric opcode tag** from a graph node. Numeric tags follow `stage-3.3-instruction-abi.md` and currently include 20 (grant), 21 (revoke), 22 (read tag).
- Existing `GraphReadNext` accepts an owner or a granted reader.

A guessed object ID alone grants no access. The reader cannot write, allocate nodes in, launch, or destroy that graph. Owner-only operations retain the previous authorization rule. All read-grant operations cost one VM instruction credit, and traps consume the attempted credit.

Read rights are stored as a sorted process-ID set in the graph object and serialized with the world snapshot. The grant-list metadata is **not yet charged to the shared storage pool**, an explicitly unresolved accounting limitation. Revocation takes effect immediately at its deterministic scheduling point, but a value previously read into registers or ordinary public data cannot be retroactively erased. Grants are process-ID specific, not bearer tokens. There is no delegation or owner transfer. Current introspection exposes opcode tag and a single `next` edge only; full operand introspection remains future work.

## Scheduled intervention experiments
A new `cl0-lab::run_scheduled` API accepts multiple `ScheduledCondition` branches from one byte-identical initial `World` snapshot. Each branch contains a nondecreasing list of `ScheduledAction { after_attempts, action }` entries.

`after_attempts` counts attempted CIL instructions **within the branch**, not elapsed wall-clock time, total world event count, or OS time. Multiple actions at one offset execute in input order and retain `Origin::External`. An action at offset 0 occurs before any CIL step. If the world becomes inert before a scheduled offset, the unreachable action is not executed. Offset validation occurs before writing results; offset beyond the run limit or decreasing ordering is rejected.

The new SQLite `scheduled_runs` table is separate from the historical `runs` table, to preserve Stage-2/3 replay compatibility. Each branch stores initial JSON and SHA-256, ordered action list, instruction-attempt ceiling and observed attempts, final JSON and SHA-256, code revision label, and schema version. `verify_scheduled_replay(id)` re-runs the exact action schedule and compares the complete JSON including events.

## Controls and falsifiability
- A writer process completes a shared-cell write in the untreated condition, but a matched intervention at attempt offset 2 stops it before the write.
- An action scheduled after the world becomes inert is not silently performed.
- Out-of-order scheduled actions are rejected; incomplete experiments do not write half an experimental batch.
- A SQLite database closed and reopened retains scheduled runs and can verify replay; tampered `final_json` is detected.
- A guessed graph handle cannot be used to read another process's graph without a grant. The owner can later revoke rights.
- These are laboratory tests with externally designed programs, not observations of spontaneous cooperation.

## Limitations and follow-ups
1. A scheduled action offsets by executed **attempts**. It is not condition-triggered and does not implement simultaneous physical time or external nondeterministic event ordering.
2. No standalone snapshot fork persisted at each intervention offset; the runner reconstructs intermediate states deterministically from the baseline. Explicit checkpoint sharing/caching remains a separate engineering capability.
3. Read grant lists are not priced as world storage; neither are process headers, registers and runtime heap structures.
4. Read permissions exist only for graph objects. Shared integer data cells are still public.
5. No full graph byte-level ABI export; `GraphReadTag` provides opcode type and `GraphReadNext` provides one edge.
6. Database SHA-256 detects inconsistent records but cannot protect against an adversary rewriting the entire SQLite database.
7. `cargo fmt --all` in CI formats the ephemeral checkout and still does not enforce checked-in formatting; dependency lockfile and cross-version replay schema require completion.
