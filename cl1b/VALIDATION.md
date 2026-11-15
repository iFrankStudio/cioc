# CL-1B native verification ledger — experiment code versus evidence

**State as of 2026-11-15.** This ledger prevents source commits, path simulations and prospective protocols from being mistaken for executed native Rust experiments. **No B00–B12 CL-1B native CI success has been independently verified in this research session.** The current GitHub connector exposes commit/legacy-status data but does not list push-triggered Actions workflow runs; an empty combined commit status or PR-only run query is *not* a passing result.

| Study | Declared scope | Present evidence tier | Scientific stage accepted? |
| --- | --- | --- | --- |
| B00 | self-edit control | source integration test; native run unverified | No |
| B01 | functional continuity under two native self-edits | source integration test and lightweight execution audit | No |
| B02 | scripted single-code-edge repair | source integration test and lightweight path audit | No |
| B03a | computed code-edge variation | source integration test and lightweight path audit | No |
| B03b | scripted trial-feedback choice | source integration test and lightweight path audit | No |
| B04 | actor-written checkpoint rollback | source integration test and lightweight path audit | No |
| B05 | mutually guarding monitors / false throughput gains | source integration test and lightweight path audit | No |
| B06 | poisoned checkpoint causing self-damage | native test in B05 file, simplified audit, failed two-patch preregistration preserved | No |
| B07 | two-copy checkpoint agreement tradeoffs | source integration test and simplified audit | No |
| B08 | state-indistinguishable history loss | source integration test and simplified audit | No |
| B09 | trial output readback and zero/stale ambiguity | source integration test and full 25-path independent audit | No |
| B10 | CIL-cleared marker and timed spoof control | source integration test and full 25-path independent audit | No |
| B11 | one-actor repeated CIL candidate generation/trial | source integration test and full 30-path independent audit; native run unverified | No |
| B12 | marker/function divergence, false positives and damaging false-negative self-repair | source integration test and independent path audit; native run unverified | No |

**Counting:** B03 has separate a/b tests; B05+B06 share a single Rust test binary; therefore **13 named Rust integration-test targets** currently cover the above 12 study labels.

## Native verification runbook

Use [the dedicated focused Actions workflow](../.github/workflows/cl1b-focused.yml), configured to run only the explicitly named B00–B11 targets. The job records the source commit `GITHUB_SHA`, workflow ID/attempt, Rust/Cargo toolchain versions, per-target exact `cargo test --locked -p cl0-world --test ... -- --nocapture` stdout/stderr and explicit PASS/FAIL lines.

1. Open **Actions → CL-1B focused native experiments** in GitHub and locate the run corresponding to the intended **source SHA** (a run being queued, skipped, cancelled or absent does **not** pass).
2. Require a completed **success** conclusion for that exact source revision, with all target lines PASS. A failure must remain archived, not silently reclassified. Note that the very first compilation may fail before later experiments are reached: absence of a result for later targets is neither pass nor fail.
3. Download the automatically uploaded `cl1b-native-<run ID>-<attempt>` artifact (uploaded **even on failed jobs**, if any file was produced). Preserve `provenance.txt`, `summary.txt`, all target logs and corresponding commit SHA in immutable experimental storage, where possible.
4. Check scientific content before promotion: human-authored seed programs, externally supplied interventions, resource charges, exact denominators, and no latent host repair oracle. A green Rust test is **engineering consistency**, not evidence of endogenous life.
5. If no run or artifact is visible, leave status **UNVERIFIED**. Do not infer GitHub execution from a successful commit or an empty commit-status response.

## Policy on future studies

**Do not launch an unbounded series of scripted guards and call it progress toward open-ended evolution.** Native evidence should be recovered before accepting mechanism claims. Subsequent studies must introduce a *scientifically discriminating* change such as agent-constructed competing hypotheses, within-individual policy revision based on paid native evidence, prospective holdouts or substantial falsification. Pre-register assumptions and keep all failures. Maintain CL-1A archive; avoid rerunning its hundreds of studies on each new CL-1B push.
