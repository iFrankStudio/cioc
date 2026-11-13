# CL-1 Study 30 — explicitly retrospective pipeline-exposure bottleneck audit

**Registered 2026-11-13 AFTER Study29 negative 0/256 dual-source additive results were known.** This is an exploratory *same-seed*, *same-result* diagnostic, not an independent prospective hypothesis test. No additional independent ecological worlds or positive life claims may be drawn from it.

## Frozen question and cohort

Study29 permitted native graph1 and graph2 tag reads in the randomized continuation, yet no world met the prescribed four-arm 22/30/30/38 output function. Was the barrier (a) never successfully reading both sources in process1's **same epoch0**, (b) never successfully executing native `Add` *after both sources were read*, or (c) failing to produce a successful `CellWritten` after Add? The diagnostic **does not** revise the frozen Study29 endpoint. It must retain *all* 256 originally sampled seeds, seven paired branches, 8 epochs ×12 attempts, 1,792 runs and 14,336 hash-checked snapshots.

## Fixed read-only native event classification

For each registered condition in Study29, inspect **only epoch0**. Attribute events to process1 only with `Origin::Internal(1)`. Successful graph-source reads must be `GraphTagRead { graph:1, owner:1, node:0, tag:expected }` or `GraphTagRead { graph:2, owner:2, node:0, tag:expected }`, where expected tags are 11 except for externally altered source handles, which use 19. Audit the source provenance against the original source-pinned matched intervention schedule. Count distinct world exposures for:

1. Any successful source1 tag read.
2. Any successful source2 tag read.
3. Both successful source reads in the same epoch0.
4. Both successful source reads followed in native tick order by at least one **successfully charged native** `Op::Add`, identified by joining the event's process ID and PC with the *exact prefork CIL instruction code* in its initial serialized process, not by guessing opcode from an `Executed` event.
5. Both reads → successful Add → a later successful `CellWritten` by process1 (any cell and any numeric value).
6. Both reads → Add → `CellWritten` at the preregistered branch target address 22/30/30/38, as applicable, with exactly one source-read-to-write opportunity per arm.
7. Any successful `CellWritten` by process1 regardless of read/add pipeline, and native `Trapped` events after both reads.

All four necessary events must be from the **same actor and same epoch**, ordered strictly by tick; multi-read paths remain classified descriptively and cannot be treated as a pure register-taint witness. Compute an exact per-arm **world denominator of 256**, report raw event counts separately and stage-by-stage attrition. A missing Add exposure is a descriptive *structural bottleneck of this frozen grammar*, not proof that two-source arithmetic cannot emerge under another grammar.

## Controls

- Execute the unchanged Study29 runner under the current source revision to a **new SQLite file**, verify Rust exact replay, and rerun the original independent Study29 matched source/epoch audit before any Study30 results. Every original registered source edit, strict capacity, untouched/sham final-state equality, exact content restore, zero-credit null, 1,792 matched arms and 14,336 snapshots must pass unchanged.
- A synthetic positive fixture must include two success-only source reads, a native `Executed` event at a prefork code PC containing an `Add` instruction, followed by a successful native `CellWritten`; negative fixtures should exclude wrong source owner, wrong actor, code PC that points to something other than Add, trapping Add, missing write and reversed event order. Synthetic fixtures do not count as ecology.
- Do not mutate the CIL generator, branch schedule, source graph nodes or original science denominator based on the observed attrition. This analysis is explicitly post-outcome and **cannot** alone establish causal register value flow, internally authored code, autonomous repair or two-window persistent organization.

**Scientific stage decision remains CL-1 OPEN** regardless of which bottleneck is found.
