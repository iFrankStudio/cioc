# B49 — second native CI failure: test artifact path differs from observer working directory

**Second run overall FAILED**, tested source SHA `6886fb9bf9f281a2250a215ffe23ec68ef7dd60c`, [GitHub Actions run43993441770](https://github.com/iFrankStudio/cioc/actions/runs/43993441770). Unlike the first B49 compile failure, the **B49 Rust native target PASSED**, including both Rust tests, **96 actual native worlds**, real dynamic node edits and the full previously observed B48 raw holdout aggregate. B31/B34 private ABI checks also PASSED. **34/35 independent observers PASSED**, but the new B49 independently executing CIL interpreter **failed closed** because the JSONL file was absent at its expected CI working-directory path.

[Immutable source-pinned summary](https://github.com/iFrankStudio/cioc/blob/cl1b-ci-evidence/run-records/6886fb9bf9f281a2250a215ffe23ec68ef7dd60c-43993441770-1/summary.txt) · [actual B49 native Rust output with real phase/work counts](https://github.com/iFrankStudio/cioc/blob/cl1b-ci-evidence/run-records/6886fb9bf9f281a2250a215ffe23ec68ef7dd60c-43993441770-1/cl1b_b49_dynamic_replay.log) · [independent observer's missing-artifact error](https://github.com/iFrankStudio/cioc/blob/cl1b-ci-evidence/run-records/6886fb9bf9f281a2250a215ffe23ec68ef7dd60c-43993441770-1/b49_dynamic_native_trace_replay.stderr.log).

### Exact failure mechanism

The native Rust target created `cl1b-ci-evidence/b49_native_steps.jsonl` *relative to the Cargo integration test's process CWD*, which Cargo sets to the crate package root `cl0/crates/world`. The CI JavaScript observers and artifact uploader instead read `cl0/cl1b-ci-evidence/`. Consequently the native test correctly ran and reported `B49_NATIVE cases=96`, but the event data landed outside the observer-visible and artifact-uploaded path. Since the independent observer required same-run original native event data, **it correctly refused success**. This was a tooling/path integration defect, not an independently observed step-level mismatch, and not proof that H49 dynamic parity passed.

### Minimal corrective action (not a research result)

Write the original JSONL artifact to a deterministic absolute path resolved from `env!("CARGO_MANIFEST_DIR")`, ascending from `cl0/crates/world` to `cl0/cl1b-ci-evidence`. Do **not** touch the native CIL program, task-family seeds, interpretation, event fields, scorer, independent JavaScript interpreter, cost limits, or preregistered expectations. Rerun B49 native and all registered source-parsed observers under an exact new SHA. The prior failure remains preserved, and the corrected run's scope must be reported precisely.

**Until the new run completes with actual per-step comparisons, B49 independent replay is UNVERIFIED; B3/B4 remain NOT MET.**
