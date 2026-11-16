# B47 — source-matched native outcome and limits (2026-11-16)

**Evidence:** [successful focused run 43954298448](https://github.com/iFrankStudio/cioc/actions/runs/43954298448), exact main source SHA `32e068b1fd60491401bdd35472b928a1e633fe2e`; [immutable run summary](https://github.com/iFrankStudio/cioc/blob/cl1b-ci-evidence/run-records/32e068b1fd60491401bdd35472b928a1e633fe2e-43954298448-1/summary.txt). Native B47 target passed, 34/34 independent *source-structure* observers passed, B31/B34 ABI tests passed, evidence job succeeded. This was **changed-only**, not a full B00–B47 regression. The B47 observer does not independently replay dynamic CIL.

## Native event-derived results

These values were transcribed from `native-cl1b` job 133604638049 log `B47_AGGREGATE` at that SHA. Each array below reports the two prospective holdout windows, aggregated over all eight frozen sequences. Native test code scores `CellWritten{key:11}` against the actual input at write time, with its explicit pre-restoration exclusion rule.

| Arm | Holdout 1 correct/wrong | Holdout 2 correct/wrong | Paid attempts | Executable edits | Stopped worlds |
| --- | ---: | ---: | ---: | ---: | ---: |
| Feedback | 65 / 14 | 73 / 7 | 1152 | 22 | 0 |
| Blind feedback | 18 / 54 | 20 / 60 | 1152 | 8 | 0 |
| Patch disabled | 0 / 0 | 0 / 0 | 296 | 0 (17 suppressed) | 8 |
| Immutable phase shift | 74 / 6 | 80 / 0 | 1152 | 0 | 0 |
| Compact immutable copier | 96 / 0 | 96 / 0 | 1152 | 0 | 0 |
| Authored oracle arm | 96 / 0 | 96 / 0 | 1152 | 0 | 0 |

**Preregistered H47 is falsified for this implementation:** feedback does improve correct holdout writes over blind feedback, but it loses to the phase-shift control and compact immutable copier in both holdout windows, and emits incorrect writes. The oracle arm is literally the same immutable source as Compact; it is *not* an independent repair or external oracle experiment. A passing Rust assertion is evidence that the negative screen executed, not that the adaptation hypothesis passed.

## Causal caveats / next falsifier

1. The phase arm is an immutable five-node copier, **not** a cost/structure-matched feedback-actor no-op. It cannot isolate phase from architecture/monitoring overhead.
2. The disabled arm exhausts or stops early (296 attempts total vs 1152), so equal *initial* budgets do not imply equal realized work; preserve this failure rather than imputing zero writes at matched exposure.
3. The test's `restored` flag and `phase>0` exception condition the scoring window. A future audit must independently parse all native `Write` events without relying on that test-side classification and compare raw versus qualified outcomes.
4. The current test hardcodes an assertion that `beats` is false; this is a negative-regression guard, **not** a general prospective acceptance test for a future positive candidate. Do not reuse it unchanged for positive claims.
5. This experiment tests a researcher-authored policy with researcher-authored input sequences; it does not demonstrate endogenous proposal, independent discovery, B3/B4 acceptance, or cyberlife.

**Next scientific experiment B48:** distinguish *semantic benefit of truthful paid feedback* from the architectural/phase overhead by a feedback-source-identical, paid-instruction-matched sham with mutation enabled and native per-write tracing. Preregister seed/phase permutations and count all attempted instructions, correct/wrong writes, retained edits, traps and resource loss. Use a separately implemented dynamic observer or trace-level verifier; do not promote a source-pattern audit to independent replay. Preserve B47 as a negative result.
