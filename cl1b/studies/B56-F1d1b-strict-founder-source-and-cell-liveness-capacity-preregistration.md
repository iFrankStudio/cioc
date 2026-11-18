# CL-1B B56-F1d1b — prospective source-capacity and causal identifiability gate (v1)

**FROZEN BEFORE ANY F1d1b implementation or native results. NOT an experiment success.** Predecessor F1d1a was verified at source SHA `8e75d91fc25f6b2b06e7d4f36047907b69ff2439`, Actions 44244193410: 128 *preassembled* native worlds, 10,840 paid instructions and independent replay. This protocol must not promote preassembly into actor invention.

## Falsifiable question

Can a *single* original F1c 162-node PID1 founder, under 180 maximum source nodes, 16 simultaneous data cells and 300 storage units, **pay to install and use** a B53-correlated nonce/echo/commit monitor, then compare two distinct paid monitoring routes without losing the original task-search feedback circuit? Or is the combination infeasible under strict simultaneous resource limits?

Null: no legal same-PID actor-executed installation and comparison exists with the preserved original search circuit and without researcher-preassembled monitoring nodes. A feasible static layout alone does not reject this null.

## Preimplementation source and resource gates

1. Extract exact initial F1c source nodes, occupied node indices, executable successors and dynamic PatchNext/PatchConst/Clone targets from `cl1b_b56_f1c_pilot_ordered_trial.rs`; extract the B53 nonce/checker graph from `cl1b_b53_native_response_correlation.rs`. Report hashes and exact source SHA. Do not treat a `Halt` placeholder as safely reusable if a live or future actor-paid patch can reach it.
2. Record the **maximum** simultaneous live source nodes, cells and storage after each mutation, not just initial/final counts. The 162-node F1c founder leaves at most 18 additional code slots without reclamation; B53's seven-operation nonce generator and 15-operation checker cannot simply be appended as 22 new nodes. Prove any reuse/reclamation by actual actor-paid operations, preserving original training-feedback causal reachability.
3. Explicitly resolve collisions: B53 commit cell28 versus F1c FOURTH counter28, B53 check PC110 versus F1c service continuation110, plus B53 nonce26, echo27, reject29 and remapped counter40. Cell-count liveness must include all inherited task-search, construction and service keys, external packet keys, and rejection markers. A data address may be reused only with proven non-overlapping lifetime.
4. Reject any design relying on an external replacement of PID code, free graph edits, free nonce generation, hidden memory reset, budget replenishment, or mutation-disabled controls that also disable task search.

## Future native test arms (not yet implemented)

Use matched seeds and both A→B/B→A trial orders, identical initial founder and compute credits. Compare truthful FRESH receipts; REPLAY_OLD; TORN_COMMIT; FORGED_CURRENT (current echo and commit but false success); blind constant-success; static EVERY and FOURTH; and a **monitor-edit-only disabled** arm that leaves original task candidate search enabled. Cross with stationary and held-out shifted tasks. Fix trial/selection budgets and record source graph, paid edits, packet interventions, actual feedback reads, real task correctness and full compute/storage occupancy at every step. Score both early adaptation and complete-budget correct throughput, including construction overhead.

Freeze predicted narrow mechanisms: coherent fresh packets should cost more for EVERY than FOURTH (F1d1a four-service cost 60 vs51), while old-echo replays should reverse price ranking (36 vs45); current-serial forgery should pass B53 correlation but not count as truthful feedback. **No prediction that the actor will choose the useful route**; that is the primary falsifiable outcome, not a guaranteed pass. A selector must outperform blind and static controls on preregistered held-out utility to support a useful adaptive-selection claim. Report failed conditions rather than optimizing thresholds after inspection.

## Evidence requirements and stopping rule

First publish a deterministic independently implemented **source graph and cell-liveness feasibility report** with exact-source hashes and explicit counterexamples. If capacity fails, stop before native actor claims and record the minimal conflict. If feasible, implement Rust native execution and an independently authored full paid-CIL event replay, freeze source SHA, run a single scoped CI only when all dependent trace producers are included, archive raw failures as well as success, and update README/VALIDATION only after matching Actions SHA and logs are verified.

**F1d1b is not yet executed.** F1d1a establishes conditional sensor cost and current-serial semantic forgery, not authenticated truth, endogenous policy construction, N56-A/B/C, B3/B4 or cyberlife.
