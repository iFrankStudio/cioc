# CL-1 CI policy — targeted push validation and complete acceptance

## Why

Historical CI previously ran **every preregistered Study** for every push that touched `cl1/**` or `cl0/**`, including text-only documentation updates. Since each experiment runs a charged CIL simulation with full SQLite exact replay, this expended compute, crowded GitHub Actions queues and duplicated scientific evidence without improving the historical source-pinned records.

## Validation routing

1. **Normal code push:** the `select` job reads the complete base-to-head Git diff and runs the `quick` gate once (Rust formatting/workspace tests/strict Clippy and Python CL-1 unit tests). For independent study-runner/analyzer changes, schedule only the corresponding named study job, not all historical cohorts. Both the quick gate and selected study job must succeed for an experimentally changed study to be accepted. Existing jobs still reproduce full SQLite epochs and independent read-only scientific metrics.
2. **Shared substrate or common-auditor changes:** changes to the world kernel, the shared laboratory, Cargo lock/dependencies or a shared baseline auditor trigger **all registered studies** plus the quick gate. Cross-study semantics must never pass solely on local unit tests.
3. **Periodic full replay:** GitHub's `CL-1 preregistered research` workflow runs **weekly at 05:17 UTC Sunday**, and can be launched manually using `workflow_dispatch`. A future stage acceptance must reference a **successful full all-study replay at the relevant source revision**, including any new study jobs added since the last full run.
4. **Docs-only changes:** no expensive experiment is triggered. These still remain versioned in `main`; the next scheduled full run includes the new state.
5. **Legacy experiment benchmark:** only runs on `experiments/**` and its own workflow definition (or on manual dispatch). A CL-1-only code/doc change must not rerun every historical `v0.x` benchmark.
6. **CL-0 acceptance:** triggered by shared kernel/laboratory/analysis/console/dependency changes (or manual dispatch); isolated CL-1 study-binary changes are covered by the CL-1 quick and targeted job instead.

## Integrity rules

- **A skipped job is not a success.** Never describe a targeted push as \"full historical CI passed\". Preserve separate labels: quick passed, changed-study passed, full-regression passed with its actual source revision.
- A previous successful run is source-pinned to that commit. If the kernel changes, require a fresh all-study pass **before** accepting new scientific conclusions.
- Every new study must add its own named job and update the selector's study-ID list, plus paths for its runner and analyzer. Before the corresponding job exists, a preregistration is **not yet experimentally executed**.
- If a change can affect multiple study auditors and the selector does not yet model its dependency, manually run the full suite or classify that path as shared in the selector. Better to overrun once than to silently skip a scientifically necessary revalidation.
- Independent seeded cohorts, external positive controls, and provenance calibration must remain distinct; changing CI routing cannot change frozen hypotheses, seed lists, denominators, SQLite requirements, result classification or the CL-1 acceptance gate.

## Scientific stage

The CL-1 milestone requires endogenous multi-component source-sensitive causal function with internally driven restoration after verified damage and persistent closed organization, independently replicated and paired with sham, zero-compute, no-repair and exact external functional restoration controls. **All technical CI optimizations are infrastructure, not evidence of digital life.**
