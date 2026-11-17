# B49 — first full-native CI failure, exact-source record

**First run FAILURE**, tested SHA `839e17f6353a2470efc3741601a01cded46b4982`, [GitHub Actions run 43993215301](https://github.com/iFrankStudio/cioc/actions/runs/43993215301). The full workflow selected **48** Rust native targets, **47 PASS and one B49 FAIL at compilation**, the B31/B34 library ABI checks passed. 35 independent observers were registered: **34 PASS, B49 FAIL**, because the compiled B49 native target never produced the JSONL trace.

Source-pinned [summary](https://github.com/iFrankStudio/cioc/blob/cl1b-ci-evidence/run-records/839e17f6353a2470efc3741601a01cded46b4982-43993215301-1/summary.txt) · [B49 Rust compiler error](https://github.com/iFrankStudio/cioc/blob/cl1b-ci-evidence/run-records/839e17f6353a2470efc3741601a01cded46b4982-43993215301-1/cl1b_b49_dynamic_replay.log) · [B49 independent observer fail-closed reason](https://github.com/iFrankStudio/cioc/blob/cl1b-ci-evidence/run-records/839e17f6353a2470efc3741601a01cded46b4982-43993215301-1/b49_dynamic_native_trace_replay.stderr.log).

### Root cause

The B49 test correctly declared the aggregate as `[[[[usize;2];2];4];2]` representing two phases × four arms × two holdouts × (correct,wrong). But its eight *historically expected B48* pairs were mistakenly written as `[correct,wrong,0,0]` (four values) rather than `[correct,wrong]` (two values). Rust rejected the compile-time `assert_eq!` with `E0277` comparing `[usize;2]` with `[integer;4]`. This is **not** an experimental finding about CIL correctness and does **not** justify changing the expected values or experimental hypotheses.

The independent observer rightly failed closed with **“missing same-CI native trace artifact”**, rather than pretending that source patterns or theoretical predictions were native result equivalence.

### Correction and acceptance

Fix the eight expected B48 literal pairs by removing only the two erroneous trailing zeros per pair; keep **all 16 correct/wrong historical values unchanged**. No CIL code, VM opcode, source policy, seeds, phase offsets, paid instruction caps or independent dynamic interpreter logic may be modified on the basis of this compiler failure. **Re-run the affected B49 Rust target and all35 independent observers** against a new exact source SHA. The first full 47/48 failure is permanently retained. A corrected source's changed-only run is **not** a full48/48 regression, even if the previous47 all passed at the earlier SHA.

**Status at creation: no valid native B49 scientific evidence yet, B3/B4 NOT MET.**
