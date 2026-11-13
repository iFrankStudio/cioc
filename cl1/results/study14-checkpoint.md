# CL-1 Study 14 — verified three-window reachability record (2026-11-13)

**Result: no supported claim of recovery or closure. Positive CL-1 scientific milestone NOT ACCEPTED.**

## Provenance

- Preregistration (before code/results): [Study 14 protocol](../studies/14-preregistered-functional-restoration-gate.md), commit `7ddb686a128b28c3f4a14d33ec9a2fcc2551ab2b`.
- Experimental implementation used source `a62d56a5e29666a25c670b96b74c4a579f72ad2a` and generated a fresh cohort.
- Verified execution: [GitHub Actions run 43429948675](https://github.com/iFrankStudio/cioc/actions/runs/43429948675) (all 14 study jobs succeeded).
- Raw Study-14 SQLite + JSON report: [run artifact 14038696780](https://github.com/iFrankStudio/cioc/actions/runs/43429948675/artifacts/14038696780), 30-day retention. The protocol, runner and analyzer are retained in Git to permit reproduction after artifact expiration.

## Fixed randomized-arm denominator

**96 independent seed-worlds** (2,140,000..2,140,095), six identical-prefork conditions per seed, **576 completed matched runs**, 24 externally administered epochs each, **13,824 validated epoch snapshots**. Rust full world-and-event replay succeeded, plus independent Python integrity and control audits. Arms: control, sham, drop-g1, drop-g2, drop-both, zero-credit. Paired arms are *not* independent experimental worlds.

## Preregistered graph-copy exposure markers

| Endpoint | Seed-worlds / 96 |
| --- | ---: |
| Reciprocal copy marker in W1 (epochs 0..7), untreated | 4 / 96 |
| Reciprocal copy marker in W2 (epochs 8..15), untreated | 0 / 96 |
| Reciprocal copy marker in W3 (epochs 16..23), untreated | 0 / 96 |
| Reciprocal marker across all three windows | 0 / 96 |
| W1 reciprocal marker lost with g1 deletion | 4 / 96 (4 / 4 exposed) |
| W1 reciprocal marker lost with g2 deletion | 4 / 96 (4 / 4 exposed) |
| Joint-only additional reciprocal loss | 0 / 96 |
| New internally populated nonempty graph after each lesion arm | 0 / 96 |
| Late reciprocal reappearance candidate after each lesion arm | 0 / 96 |

Untreated cross-owner graph-copy events across all worlds: **332 in W1**, **16 in W2**, **0 in W3**. The bidirectional pairs exposed in W1 were (1,2) once, (1,3) once, (2,3) twice.

Study 14's primary **source-sensitive transformation** and **functional restoration** are explicitly **NOT EVALUABLE**: previous provenance does not attribute an output-cell value to the informational content of the copied graph. Direct external restoration of the damaged graph is also **UNIMPLEMENTED** in the random arm. The four observed early reciprocal states are therefore copy-proxy observations, not proof of functional exchange, internal maintenance, or autonomous life.

Technical controls passed; this is a negative/inconclusive research result, not a failure of the VM. The next action is observer-only cell I/O provenance under a separately preregistered Study-15 calibration, followed by a source-sensitive counterfactual experiment; it would be incorrect to promote Study 14 to scientific success.
