# CL-1 Study 24 — successful foreign-graph read vs output exposure (2026-11-13)

**Technical audit: PASS. Scientific decision: NO VALIDATED SOURCE-SENSITIVE FUNCTION; CL-1 NOT ACCEPTED.**

## Frozen lineage

- [Preregistration](../studies/24-preregistered-foreign-graph-read-output-audit.md), commit `54baea4b9189618e8854b29d51785f406ae899a0`, before auditor and measured outcome.
- Independent read-only [auditor](../analyze_study24.py), and [synthetic positive/negative fixtures](../tests/test_study24.py).
- Full source-pinned [CL-1 Actions run 43448995236](https://github.com/iFrankStudio/cioc/actions/runs/43448995236), **successful study24 job** under `3d1617dc49c12b33b29f264dce22ab03c6364a3c`. Original Study-22 runner was re-executed after Study-23 observer-only additions, not altered; all 768 paired runs and 18,432 hashed snapshots passed Rust exact replay and both independent Study-22 and Study-24 Python audits.
- **96 reused seed IDs, zero newly independent ecological worlds.** Do not add 768 paired simulations as independent sample units.

## W3 strict exposure

| Condition | Independent worlds with ≥1 successful foreign graph-code read | Worlds with same-actor successful cell output following a foreign graph read |
| --- | ---: | ---: |
| Untreated control | **0/96** | **0/96** |
| Externally compute-replenished | **11/96** | **0/96** |

Under refill W2, **11/96** independently seeded worlds had at least one successful foreign graph read; in W1, **43/96**. In untreated W1, the audit observed 202 native foreign graph reads and seven *individual native same-actor read-then-write event sequences*. These seven events are **not seven independent worlds**, and temporal order alone cannot establish dataflow or source-content causality. The source code in the original scaffolds was investigator authored. No user-supplied resource or injected graph is counted as internally generated.

A read without a same-actor successful cell write never qualifies even for a temporal candidate. The independently audited W3 failure to observe any completed chain means that the full output-function causal outcome is **NOT EVALUABLE**, not estimated as a zero probability of autonomous recovery. The W3 instrumented read exposure itself is a valid negative/positive measurement against the registered event specification.

## Next gate

Pre-register prospective *new, disjoint independent seed-worlds* and introduce **paired exogenous code-content edits** to the precise graph source while retaining intact/sham/exact-restoration, zero-credit, joint-target and matched resource controls. Observe whether registered actor-address-and-epoch functional output values or successful writes change when the graph contents are changed, distinguishing read/graph-code presence from a causally used content word. Preserve dead/trapped worlds, source, external budget records and both postdamage windows. Never infer native maintenance merely because experimenter-supplied code changes an output.

**CL-1 remains OPEN.**
