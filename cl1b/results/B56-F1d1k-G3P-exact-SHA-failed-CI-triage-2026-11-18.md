# CL-1B B56-F1d1k-G3P — exact-SHA failed native CI triage (2026-11-18)

## Verified evidence (do not promote frozen predictions to results)

- Latest main inspected at `9c5815f37008dac317eb9db2b8d1c84dc254e82c`; this commit adds `cl0/crates/world/tests/cl1b_b56_f1d1k_g3p_paid_duplicate_repair.rs` (657 lines).
- [GitHub Actions run 44340918846](https://github.com/iFrankStudio/cioc/actions/runs/44340918846) completed **failure**, exact `head_sha=9c5815f37008dac317eb9db2b8d1c84dc254e82c`.
- Job `native-cl1b` (134930330408) failed at both **Run changed CL-1B targets** and **Run all independent observer analyses**. The evidence-publishing job succeeded; publishing evidence does **not** mean native tests succeeded.
- [Raw original evidence summary](https://github.com/iFrankStudio/cioc/blob/cl1b-ci-evidence/run-records/9c5815f37008dac317eb9db2b8d1c84dc254e82c-44340918846-1/summary.txt) records 26 failed replay observers, beginning with B49 and continuing through G2S. [Result marker](https://github.com/iFrankStudio/cioc/blob/cl1b-ci-evidence/run-records/9c5815f37008dac317eb9db2b8d1c84dc254e82c-44340918846-1/result.txt) explicitly says `native_job_result=failure`.
- The prior [G2S exact-SHA run 44333212743](https://github.com/iFrankStudio/cioc/actions/runs/44333212743) succeeded at `b9555fa61b8888c208e6bd71f7c9a3508aa4b52b`; it **does not** validate G3P.
- The G3P preregistration predicts 22/24 vs 20/24 truthful heldouts, but **no G3P native result is verified**. No new scientific conclusion about duplicate repair is accepted.

## Diagnostic distinction

The 26 independent replay failures are *not* 26 independently demonstrated regressions. The added G3P test did not produce an evidenced G3P trace in the original evidence tree, and earlier replay analyzers can fail downstream when expected native trace files are absent. Whether the native target failed to compile, panicked during execution, or failed on an assertion **cannot be determined from the available summary alone**. The exact failed step logs are required; neither the published summary nor the successful evidence upload supplies the root exception.

## Falsifiable repair plan, not yet executed

1. Retrieve the complete failed native job log for 134930330408 and record the first compiler diagnostic or first Rust assertion failure with its exact line; do not patch based on guesswork.
2. Run only `cargo test -p cl0-world --test cl1b_b56_f1d1k_g3p_paid_duplicate_repair -- --nocapture` (confirm package name from Cargo metadata first). If compile fails, fix the smallest diagnostic. If a runtime assertion fails, compare the frozen 180-opcode graph and actual PID1 trace at first divergence; preserve a failing counterexample.
3. Separately verify whether independent observer failures stem from missing native trace artifacts. Do not change or weaken observers merely to make CI green.
4. Only after a native trace exists, add an independent source-pinned G3P event replayer, rerun the source-SHA-pinned CI, and publish both successes and failures. Do not report G3P, full G3/Hk, N56, B3/B4, or cyberlife achieved before evidence supports them.

**Status:** reproducible exact-SHA CI failure and original evidence identified; root cause and native repair unverified. CL-1A remains archived; no API-based auto-research engine.
