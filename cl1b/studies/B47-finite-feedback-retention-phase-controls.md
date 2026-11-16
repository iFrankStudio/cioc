# B47 — Finite-feedback retention versus phase-matched static baselines (preregistration)

Status: **PROTOCOL ONLY — no B47 native execution or outcome claimed**. Registered after B46's corrected-source changed-only verification. B3/B4 remain NOT MET.

## Scientific question

Can a *single continuing CIL organization*, without an external winner-selection loop, propose, evaluate, and retain a source change that improves later **unseen** task performance, after charging for all native computation, storage and observations, rather than merely shifting the instruction phase (B46) or exploiting a zero-input alias (B41–B44)?

## Falsifiable hypothesis

H47: Under an unpredictable, preregistered task-regime sequence, the continuing actor's **own paid trial feedback** causes at least one retained executable edit, and the edit yields strictly more correct held-out task writes per common world compute budget than (i) the same actor with feedback information replaced by a non-informative matched-cost value, (ii) its edit-disabled counterpart, and (iii) the best preregistered compact immutable source. The advantage must survive a phase-matched control and a second held-out regime. Failure of *any* conjunct falsifies H47. A positive result still does not establish B4 or life.

## Frozen design and strong controls

Use the existing B42 source-parsed CIL graph, B45 opt-in charged last-writer observation, B46 intervention harness, and generic paid operations. Do not add task-specific host instructions, privileged rewards or repair. The same actor remains continuously executing across training and holdout windows. All arms share initial resources, supply packets, input transitions, scheduler order and total attempted-instruction cutoff. If source assembly is not possible under these constraints, report this as a feasibility failure rather than injecting a preselected winning candidate.

Precommit a deterministic family of input regimes (including zero, nonzero, transitions and repeated values), randomized by documented seeds, with training/holdout disjoint in sequence; no selection of seeds after observing results. Report **both** correct and wrong writes, attempts, pool/local charges, storage, traps, retained graph edits and the native causal origin of edits. An output counts correct only against the input at the actual native Write event, not against a delayed read or an authored marker.

Arms: A genuine paid feedback and native self-edit; B feedback replaced with a matched-cost constant or permuted observation (no truthful task information); C patch-disabled but charged attempt; D fixed source with equivalent no-op phase shifts and identical charge opportunities; E compact immutable correct copier; F externally scripted oracle/repair as *engineering upper bound only*, excluded from endogenous claims. Swap process IDs and permute schedule phase. In each arm keep all failed worlds and account for consumed budget.

## Frozen directional predictions (not native observations)

P1: The previously authored B42/B45/B46 guard-based actors will **not** exceed the compact immutable correct copier on correct held-out writes per common budget. P2: The added phase-matched no-op will change some raw write counts even without any change in learned source semantics; therefore any apparent gain disappearing under D is a timing artifact. P3: A misleading or temporally displaced feedback observation will sometimes cause incorrect retention; this would falsify feedback-as-truth even if the writer ID is accurate. P4: H47 is unlikely to pass all three baselines with the current human-authored policy; an unexpected pass requires event-level independent replay and unseen-seed replication.

## Implementation and evidence acceptance

Implement a Rust native integration test in `cl0/crates/world/tests/cl1b_b47_*.rs` plus an **independent** source-parsed CIL observer in `cl1b/analysis/b47_*.mjs`, not a duplicate assertion table. Freeze seed list and source SHA before native CI. Run focused changed-only first; if it passes, request a single full-regression run on the same executable SHA (not a documentation-only later SHA). Archive the original failure logs, native event traces, analysis output and exact CI run/SHA on `cl1b-ci-evidence`. Do not equate observer predictions with native evidence.

## Status boundary

This file is a prospective hypothesis/protocol, **not** a completed experiment, a Rust implementation, a CI pass, or evidence for B3/B4. B46 remains the latest native experiment until a B47 executable implementation is verified.
