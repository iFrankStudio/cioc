# CL-1 Study 24 — preregistered source-graph-read-to-output exposure audit

**Registered 2026-11-13 before implementation, run or Study-24 result inspection.** The underlying ecological Study-22 seed generator, 96 independent worlds, 8 paired arms, 24 epochs, 12-attempt quota, interventions and three windows are frozen and unchanged. Study 23's successful native GraphTagRead and GraphFieldRead event instrumentation is available and CI-validated. Study 24 reruns the **same historical generator and seed IDs** solely to ask a new **instrumentation/exposure** question; it is **NOT** a new independent cohort and MUST NOT be added to ecological world denominators or pooled with old results. No search/seed selection or post-hoc arm edits.

## Falsifiable measurement question

After external compute replenishment, do any processes **actually read another owner's executable graph** (rather than only copy it), and do they subsequently execute a successful native shared-cell `CellWritten` in the **same epoch**? This establishes a **time-ordered candidate opportunity only**; it cannot establish output value dependence on the code read without a *separate paired code-content intervention*.

## Prespecified identity and source rules

- Count `GraphTagRead` and `GraphFieldRead` only when the enclosing event has `Origin::Internal(process)`, `process` equals the actor, `owner != actor`, and the payload explicitly identifies `graph`, `owner`, `node`, and the returned `tag` or `word`. These are foreign-graph read events, not internal novelty claims.
- A *candidate read-then-write exposure* is one such successful foreign-graph read followed by an explicit successful `CellWritten` from the **same actor** later in **the same environmental epoch**, ordered by native `tick` (not merely nearby unrelated program instructions). Count distinct independent **seed worlds** with ≥1 candidate in W1, W2, W3 per matched branch; separately report raw events, source-owner to reader pairs, and successful native writes. Do not infer register taint, effective graph-code use, repair, function, value transformation or intent from temporal order.
- A *read without downstream write*, even with a cross-owner `GraphCopied` earlier, is a measured **read-only exposure**, not a completed functional chain. Unknown graph origins cannot be classified internal. Do not attribute the source graph's program to the reading actor.
- All 96 worlds remain in each branch denominator; `0` means no observed strictly defined exposure in this fixed instrumented cohort, not impossibility or an evaluated functional-maintenance rate. If no successful native write occurred in a window, the registered **source-sensitive output function is NOT EVALUABLE**, not zero.
- The same exact Study-22 `SetCredits` external schedule and `PutGraph` source are still exogenous. No simulator mutation or added native CIL instruction is allowed.

## Registered controls and integrity

Repeat the Study-22 runner **unchanged** under the new Study-24 source revision, writing fresh SQLite (do not append to Study-22's archived corpus). Independently run the complete Study-22 SQLite audit to verify identical 96 × 8 matched interventions, seed range, prefork hashes, source revision, 18,432 hash-checked snapshots, exact replay, sham equivalence, zero-credit null and resource limits. Then run a *separate read-only Study-24 auditor* using the same stored SQLite.

A minimal synthetic positive fixture must explicitly contain foreign graph read then same-actor later write with valid increasing ticks, plus negative fixtures for source-owner=reader, different actor, output preceding read, and separate epochs. No synthetic fixture counts as ecological data. The independent report must include source revision, schema/event version information, raw exposure totals, independently seeded per-arm window proportions, paired discordances for the preregistered refill-vs-control W3 candidate exposure, and a `life_claim=NONE`.

**Scientific stopping rule:** preserve negative exposure and NOT EVALUABLE conclusions. An observed foreign code read and later output is **necessary instrumentation**, but not a causal proof. A future prospective *new-seed* study must preregister actual graph-content perturbations and paired output transformations, external restoration with a valid functional positive arm, and two late autonomous recovery windows before any CL-1 acceptance.
