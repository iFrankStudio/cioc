# CL-1 Study 08 — preregistered instruction-provenance and inter-owner causality audit

**Registered on 2026-11-13**, before adding successful graph-transition event logging or running Study 08. Study 07's result and **NOT EVALUABLE** cross-owner success metric are already known. Study 07 remains frozen at source revision `0ddbc8d`, and no previous report is silently reclassified.

## Research limitation addressed
CL-0 event records traditionally represented successful CIL graph changes as a generic `Executed` event; post-epoch snapshots alone cannot reliably attribute a graph-copy or owner-transfer action to its *pre-action* graph owners. Study 08 first adds a read-only structured provenance channel to ordinary internal execution events (not a CIL goal, policy or fitness criterion).

**Frozen instrumentation contract:**
- On a **successful** `GraphCopyFrom` attempt, the existing single internal event for that instruction is `GraphCopied { source_graph, source_owner, source_node, target_graph, target_owner, new_node }` instead of generic `Executed`. The event's `Origin::Internal(actor)` and PC remain unchanged. A failed attempt must yield only the existing `Trapped` event with **no successful-copy event**.
- On a **successful** `GraphTransfer`, the existing single internal event is `GraphTransferred { graph, former_owner, new_owner }`; denied or nonexistent-recipient attempts trap without transferring. Preserve the read-only observer channel; there is no VM primitive for observing this event log.
- Event fields describe the **pre-action** owner when relevant, and no extra logical tick, compute credit or graph allocation occurs for instrumentation. Snapshots/replay include the event records; compatibility with earlier serialized `Executed` event variants remains.
- The world kernel must not infer whether a copied graph is useful, alive, or surviving; no implicit trust, grants, object repair or arbitrary process awakening.

## Fresh blinded cohort, frozen ahead of results
- Independent seeds **1380000..1380127** (128 worlds) using the **same fixed 14-way generator** and initial strict resource configuration as Study 07. Initial code is *investigator-generated*, not endogenous. No retuning based on Study 07's small eligible count.
- Three initial 16-node programs, 28 credits/process, 32 public cells, max32 nodes/process, total abstract storage160, externally recorded initial compute pool64. A **common** pretreatment history runs for up to 32 CIL attempts.
- Select the lowest-index existing nonempty internally constructed graph as the lesion target. No eligible graph means deterministic invalid sentinel0; retain all 128 worlds.
- Fork five matched conditions `control`, `sham-drop`, `target-drop`, `disable-owner`, `zero-credit` as in Study 07, running **20 independent external environment epochs ×8 possible CIL attempts**. All 640 runs must have exact SQLite replay and 12,800 saved epoch hashes.

## Prespecified primary metrics
- `successful_cross_owner_copy`: a `GraphCopied` internal event from an instruction executed **after the common pretreatment fork** with actor != `source_owner`. The actor must be the owner of its target before and after append. This metric uses evidence from successful operation events **only**.
- `successful_owner_transfer`: `GraphTransferred` with `former_owner == actor` and `new_owner != former_owner`, after common fork.
- `successful_cross_owner_information_exchange_seed`: a world with >=1 of those two successful events in its untreated control. Summarize count/128 and 95% Wilson interval. Event counts remain separate from independent seed counts.
- `eligible_graph`, `indirect_process_state_effect`, `nonowner_process_state_effect`, `nonempty_graph_after_drop` and `sham_equivalence` as Study 07's explicitly frozen definitions.
- Zero-credit arm must produce 0 postfork internal attempts and 0 successful logged graph-transition events.
- Report source revision, hash consistency, matched initial hashes, denied attempted actions, all null/trapped/inert runs and all 128 seed-worlds. Preserve the original Study 07 NOT EVALUABLE conclusion.

## Controls and threshold
Tests must verify a hand-authored successful **cross-owner** copy produces a fully specified event, a new graph owner transfer produces its event, and denied attempts **never** produce successful provenance. The hand-authored tests are mechanism calibrations, **not** scientific observations.

If zero cross-owner successful events: a bounded negative result for this exact generator and horizon. If positive: evidence of **generic information/ownership exchange**, not autonomous cooperation, mutual regeneration, internal functional persistence or life. The positive CL-1 gate remains unchanged; no observation in this study by itself can satisfy it.
