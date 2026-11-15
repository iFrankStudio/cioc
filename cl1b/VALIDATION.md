# CL-1B native verification ledger — experiment code versus evidence

**State as of 2026-11-15.** This ledger prevents source commits, path simulations and prospective protocols from being mistaken for executed native Rust experiments. **No B00–B20 CL-1B native CI success has been independently verified in this research session.** The current GitHub connector exposes commit/legacy-status data but does not list push-triggered Actions workflow runs; an empty combined commit status or PR-only run query is *not* a passing result.

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
| B13 / B13b | exhaustive single-edge evaluator robustness (1,805 mutants) and post-hoc decision-consumption audit | source-parsed scan, decision-edge correction and dedicated Rust VM differential committed; native run unverified | No |
| B14 | one actor modifies its own executable candidate-policy literal, with cross-lesion overfitting control | source integration test and 30-world lightweight path audit; native run unverified | No |
| B15 / B15b | exhaustive 350-world B14 policy comparison; corrected independent mutation units and actual use of policy | independent scan: **0/32 distinct holdout edge types benefit**, 2 types harmed; **31/160** input×edge cases really change and use policy (0 wins/10 losses/21 ties); Rust per-step causal differential committed, native CI unverified | No |
| B16 | actor-generated nonzero functional challenge and actual payload check with corruption/forgery negative controls | 30-world source-parsed independent audit and native Rust test committed; native CI unverified | No |
| B17 | exhaustive B16 direct-payload checker robustness over 899 distinct single-edge mutations ×5 inputs | independent restricted scan: 60 false-accept worlds from 12 distinct mutations; 390 false-reject worlds from 78 mutations; source-matched 4,495-world Rust differential committed, **not run-verified** | No |
| B18 | finite-inventory theorem: total local credits plus pool declines by one per native attempt unless an external supply intervenes | audited VM Rust source and committed 4 native conservation/positive-control tests, **native CI unverified** | No — source-level necessary condition only |
| B19 | generic world-pool environmental compute influx and native actor acquisition under pulse controls | source-shaped 5-world observer model and native per-step conservation/source-provenance tests committed, **native CI unverified** | No — authored resource access feasibility only |
| B20 | CIL resource-feedback accessibility / draw-size threshold under seven policy×pulse worlds | source-shaped independent seven-world model predicts threshold and post-draw debt deadlocks; native CIL scheduler tests committed, **CI unverified** | No — substrate obstruction and fixed-policy controls only |

**Counting:** B03 has separate a/b tests; B05+B06 share a single Rust test binary; therefore **21 named Rust integration-test targets** currently cover the above 12 study labels.

## New machine-readable verification channel (configured, not yet observed)

The [focused workflow](../.github/workflows/cl1b-focused.yml) now has two separated jobs: the Rust test job remains **read-only** and runs **every** active test target, accumulating failures rather than aborting the campaign on the first one; an independent *push-to-main-only* reporting job has narrowly scoped `contents:write` and writes a source-SHA/run-ID result record plus downloadable raw target logs to a separate **`cl1b-ci-evidence` branch**. This branch is not a trigger for the CL-1B focused workflow. No PR code receives a write-privileged reporting job.

**This is only a configured route until GitHub actually creates a report.** Do not infer that a successful native run occurred merely because the workflow was edited. To retrieve evidence through the GitHub connector, search for branch `cl1b-ci-evidence`, then read `run-records/<main-SHA>-<run-ID>-<attempt>/result.txt` and `summary.txt` using `ref="cl1b-ci-evidence"`. Corroborate with the named per-target logs. If the evidence branch does not exist, it may indicate a queued workflow, a disabled Actions workflow, insufficient repository permissions or a job failure; report **UNVERIFIED** and inspect the Actions UI rather than guessing.

## Scope-aware CI note

The native workflow now selects only the modified `cl1b_*.rs` test and known direct imported dependents for routine code changes; a CL-0 VM source/Cargo/workflow change or **manual workflow dispatch** forces the full suite. The `summary.txt` artifact records `scope`, `selected_native_targets` and changed paths. A successful **changed-only** run must never be represented as a passing B00–B20 campaign. Source-pinned reports remain unconfirmed until a real GitHub run/artifact is inspected.

## Native verification runbook

Use [the dedicated focused Actions workflow](../.github/workflows/cl1b-focused.yml), configured to run only the explicitly named B00–B11 targets. The job records the source commit `GITHUB_SHA`, workflow ID/attempt, Rust/Cargo toolchain versions, per-target exact `cargo test --locked -p cl0-world --test ... -- --nocapture` stdout/stderr and explicit PASS/FAIL lines.

1. Open **Actions → CL-1B focused native experiments** in GitHub and locate the run corresponding to the intended **source SHA** (a run being queued, skipped, cancelled or absent does **not** pass).
2. Require a completed **success** conclusion for that exact source revision, with all target lines PASS. A failure must remain archived, not silently reclassified. Note that the very first compilation may fail before later experiments are reached: absence of a result for later targets is neither pass nor fail.
3. Download the automatically uploaded `cl1b-native-<run ID>-<attempt>` artifact (uploaded **even on failed jobs**, if any file was produced). Preserve `provenance.txt`, `summary.txt`, all target logs and corresponding commit SHA in immutable experimental storage, where possible.
4. Check scientific content before promotion: human-authored seed programs, externally supplied interventions, resource charges, exact denominators, and no latent host repair oracle. A green Rust test is **engineering consistency**, not evidence of endogenous life.
5. If no run or artifact is visible, leave status **UNVERIFIED**. Do not infer GitHub execution from a successful commit or an empty commit-status response.

## Policy on future studies

**Do not launch an unbounded series of scripted guards and call it progress toward open-ended evolution.** Native evidence should be recovered before accepting mechanism claims. Subsequent studies must introduce a *scientifically discriminating* change such as agent-constructed competing hypotheses, within-individual policy revision based on paid native evidence, prospective holdouts or substantial falsification. Pre-register assumptions and keep all failures. Maintain CL-1A archive; avoid rerunning its hundreds of studies on each new CL-1B push.
