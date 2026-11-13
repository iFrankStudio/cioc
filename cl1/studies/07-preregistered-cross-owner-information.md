# CL-1 Study 07 — preregistered generic inter-object information exchange audit

**Registered:** 2026-11-13. This protocol precedes the VM feature and Study-07 experiment implementation. Studies 01–06 are already complete and known; no previous favorable seeds may enter this study.

## Scientific question
Does access-controlled, internally executed copying of instructions across graph objects yield *cross-owner* causal consequences under finite compute and storage, beyond mere owner-local graph edits? This is a feasibility and causal-exposure test, **not** an autonomous-maintenance or cyberlife test.

## Generic VM mechanisms (fixed before study)
- `GraphCopyFrom { target, source, node, next }` reads graph handles and source node index from registers, allows copying only when the actor **owns the target** and **owns or has explicit read permission to the source**. Append one copied node to the target, charge a computation credit and one abstract graph node unit; put the destination node index back in the original `node` register. Check source/target/edge/capacity/permissions before any mutation. The operation must not mint compute or change owner.
- `GraphTransfer { graph, recipient, next }` transfers a graph to an existing recipient process only when invoked by its current owner. Ownership, read rights and all code graph contents persist, but the previous owner loses owner privileges; any explicitly granted read rights remain. No object headers are duplicated; no free resource or life-specific rule.
- Both operations are ordinary CIL instructions with five-cell opcode encodings and fixed tags 28 and 29. The experiment runner and viewer do not call these actions for the agents.

## Predeclared calibration controls (not evidence of emergence)
Focused Rust VM tests must include a hand-authored 2-process fixture proving a **genuinely authorized cross-owner code copy**, a subsequent owner transfer and a revocation/unauthorized failure branch. An exact SQLite replay of a hand-authored fixture verifies mechanical determinism. These are **researcher-designed examples**, never counted as autonomous outcomes.

## Random-ecology negative-capable cohort
- New independent seed inputs **1270000..1270127 inclusive**: 128 fixed-seed-worlds, three investigator-initialized processes each, 16 nodes/process, 28 initial credits, strict total storage 160, 32 shared cells, 32 nodes/process. Fresh seed range and no post-hoc candidate selection.
- Reuse Study-04's deterministic 64-bit LCG and four-register initialization. Change only eleven random opcode positions (4..14): uniform **14-way** selection (0..13). The first twelve cases are Study-04's exact general instruction choices (including its extra LCG draw in the conditional branch choice); cases 12 and 13 are `GraphCopyFrom {target:3,source:0,node:2,next}` and `GraphTransfer {graph:3,recipient:3,next}`. Register contents are allowed to make these new instructions trap. No host-authored communications or favorable initial graph-handle values are added.
- Common pretreatment history: world initialized with externally recorded `SetComputePool { credits:64 }` and executed for up to 32 CIL instruction attempts, even if inert earlier. Eligibility is decided **only from this common world**: lowest **nonempty** graph object handle, if any; owner and authorized reader list recorded from that world. Worlds without an eligible target remain in the denominator.
- Five matched conditions, all fork from exactly that common baseline and execute **20 independent environment epochs × 8 CIL-attempt quota**:
  1. `control`: no intervention;
  2. `sham`: invalid `DropGraph {graph:0}` at epoch 0 (no simulated-state effect);
  3. `target-drop`: delete the eligible graph at epoch 0 (sentinel 0 if none);
  4. `disable-owner`: disable the eligible graph's owner at epoch 0 (invalid process 9999 if none);
  5. `zero-credit`: zero all existing process credits at epoch 0.
- **128 × 5 = 640** matched runs, complete SQLite replay, all 20 post-epoch hashes and snapshots per run.

## Frozen outcomes and validity
- `eligible`: a nonempty internally created graph is present after the common pretreatment period.
- `authorized_cross_owner_action`: a successful internally executed `GraphCopyFrom` or `GraphTransfer` during the postfork epoch window with the source graph owned by another process (`GraphCopyFrom`) or recipient different from current owner (`GraphTransfer`). Distinguish this from *attempts* by replaying **per-epoch** graph owner/code changes and checking the CIL executed instruction at its actual program counter against the *pre-step* code. If the available event granularity cannot demonstrate this unambiguously, **report not evaluable**, never infer from graph counts.
- `nonowner_impact`: compare control-vs-target-drop final **other-owner process states**, excluding target object identity, global event history and external pool, conditional on eligible object but report numerator/128.
- `sham_equivalence`: simulated state of control and sham must match excluding host event log/tick. Violations invalidate full study.
- `zero_credit`: 0 CIL attempts regardless of available world compute-pool credits.
- `new_nonempty_graph`: a graph handle absent from the common prefork object map appears with nonempty nodes, never by itself labeled repair or life.
- All 640 replays and 12,800 epoch snapshots must validate, strict occupancy ≤160 and no silent dropped seeds. Report Wilson 95% intervals based on 128 independent initial seed-worlds.

## Scientific stopping rule
Even if a cross-owner copy or transfer is observed, Study 07 establishes only generic code exchange and causal propagation. A **positive CL-1** claim still requires a *separate preregistered functional, multi-component, bidirectional restoration study*. No biological-likeness proxy, fitness oracle, preferential survival scoring, mutation selection or researcher-installed repair procedure is permitted.
