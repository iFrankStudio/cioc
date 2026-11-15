# CL-1B — first source-pinned full native verification checkpoint

**Checked on 2026-11-15.** This record is grounded in [GitHub Actions run 43723559666](https://github.com/iFrankStudio/cioc/actions/runs/43723559666) and the independent CI result archive. It replaces earlier "CI unverified" statements **for the tested source revision only**. It does **not** assert spontaneous evolution or cyberlife.

## Exact verification identity

| Field | Value |
| --- | --- |
| Source SHA | `5f4ce96ed596e434d89e5f74ac0aa2b05944fc7e` |
| Workflow | `CL-1B focused native experiments` |
| Run ID / attempt | `43723559666` / `1` |
| Run conclusion | **success** |
| Native job conclusion | **success** |
| Evidence reporting job conclusion | **success** |
| Native selection scope | **full** |
| Native Rust integration-test targets | **22 PASS / 0 FAIL** |
| Observer-side analysis scripts | **8 PASS / 0 FAIL** |
| Runner OS | Linux |
| Rust toolchain | `rustc 1.100.0 (0ef1bc386 2026-11-09)` |
| Cargo | `cargo 1.100.0 (3acc79c0c 2026-11-05)` |

## Durable primary evidence

The runner uploaded per-target `cargo test --locked -p cl0-world --test ... -- --nocapture` logs and all eight independent-analysis JSON outputs to the GitHub Actions artifact. A separately write-privileged job also committed **this run's exact source-pinned result** to the independent branch `cl1b-ci-evidence`. The following URLs are direct GitHub repository files and preserve the report identity:

- [Full-scoped test and analysis summary](https://github.com/iFrankStudio/cioc/blob/cl1b-ci-evidence/run-records/5f4ce96ed596e434d89e5f74ac0aa2b05944fc7e-43723559666-1/summary.txt)
- [Source SHA, toolchain and runner provenance](https://github.com/iFrankStudio/cioc/blob/cl1b-ci-evidence/run-records/5f4ce96ed596e434d89e5f74ac0aa2b05944fc7e-43723559666-1/provenance.txt)
- [GitHub Actions result and run URL](https://github.com/iFrankStudio/cioc/blob/cl1b-ci-evidence/run-records/5f4ce96ed596e434d89e5f74ac0aa2b05944fc7e-43723559666-1/result.txt)
- [Complete GitHub Actions run](https://github.com/iFrankStudio/cioc/actions/runs/43723559666)

Both jobs reported success. Every named B00–B21 Rust target passed, including the B13 and B17 full deterministic mutation-coverage targets and the B18–B21 compute-resource feasibility/negative controls. The evidence branch contains **real returned artifacts**, not merely an unexecuted workflow declaration.

## Reconciled previous failures and why they mattered

1. **B13 main Rust test originally failed** because it asserted **381** *actual consumed false-positive branch decisions*, incorrectly using the figure for **raw misleading marker reads**. Native execution and the independent observer scan agreed on **381 raw reads vs 355 actual branch decisions**. The 26 difference comes from PC11 readbacks at the end of a fixed 80-instruction window which did not reach PC12 before censoring. The two metrics are now separately asserted/documented; [the corrected B13 test run](https://github.com/iFrankStudio/cioc/actions/runs/43723414153) passed before the final full run.
2. **B15 observer script originally failed** on an incorrect set-cardinality assertion. There are **13 unique policy-reached holdout edges** and **22 unique policy-not-reached edges**, with overlap because a given code edge can be reached under one input and not another. Their union is the frozen **32 distinct holdout successors**. The actual per-condition counts remain **53 reached / 107 not reached** and the stronger changed-and-used result remains **31 cases**. The fixed observer script passed in the full run.
3. **Evidence branch publication initially failed** at a `git rm -rf .` after creating an empty orphan branch; the source now omits that invalid deletion. The branch was successfully created and the verified logs are published.
4. **Observer script failure formerly aborted downstream observer scans.** The verified workflow now runs every independent observer, preserves each stdout/stderr and reports a complete per-script PASS/FAIL denominator, even if an individual analysis fails. This is analogous to the native Rust test loop.

## Scientific acceptance distinction

**Engineering verification:** The committed source's deterministic tests and executable semantics in this full campaign are now **natively verified for SHA `5f4ce96...`**. This allows experimental mechanism and falsification claims with specific provenance. It does not automatically endorse model assumptions, hand-authored repair algorithms or external resource supply as living adaptation.

**Scientific interpretation:** B00–B21 mostly use investigator-authored source code and externally designed perturbations. B13/B17 explicitly reveal internally misleading evaluator evidence; B15 demonstrates local policy improvement with poor out-of-development transfer; B18 proves finite closed-inventory operation cannot be perpetual; B19–B21 establish neutral energy availability and the **possibility of preauthored CIL-mediated resource access/relay**. Nothing in a green build demonstrates **endogenous policy origination, non-scripted self-organization or open-ended evolutionary accumulation**.

**Next scientific gate:** Use the verified resource-conserving, multi-process substrate to test whether a single continuing native organization can *itself* form, adjust and retain resource-acquisition/relay relationships under **prospectively held-out world-pool availability and competing consumers**, paying for all native instructions and monitoring actual function with independent negative controls. Do not manufacture a positive result by giving chosen actors host-targeted credits or hidden reward.

When code changes, verification remains scoped to its particular source SHA and selected tests. A targeted green run must not be advertised as another full-campaign pass.
