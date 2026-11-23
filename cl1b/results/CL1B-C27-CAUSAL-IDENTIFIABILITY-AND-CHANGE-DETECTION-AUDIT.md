# CL-1B — C27 causal identifiability and generalization audit (2026-11-23 UTC)

**Evidence tier: retrospective design/source/report audit only. No new Rust world, no new independent replay, no new holdout.** Source baseline: `21430a519ffb17e5c0e9b472d04c24d9e0dcb236`. Native evidence: [run 45206215513](https://github.com/iFrankStudio/cioc/actions/runs/45206215513), archive [run 45206590441](https://github.com/iFrankStudio/cioc/actions/runs/45206590441), tested SHA `04776b8ee354ef135cf6d9fdf8e22a90e91f7e13`.

## What the current matched intervention identifies

The four pairs start from identical 170-node CIL source and resources. The off intervention charges attempted PatchNext instructions but suppresses **all** their physical effects. The on/off differences 48, 46, 0, 96 CIL (on cheaper in three pairs) therefore identify **total policy-level effect of enabling physical source edits under these four preselected trajectories**, including downstream changes in program counter, live ReadNext observations, branch decisions, and even the number of later PatchNext instructions attempted. They do **not** identify the standalone benefit of PC95, PC169, or any particular edge independently.

In particular, the 1→0 off arm never acquired the B edge, so it need not execute PC169 to revert it; subtracting off from on cannot be interpreted as the isolated rollback cost. The stable 0→0 pair is a useful no-exposure negative control (626/626), but cannot rule out source-editing failures in other modes. Every scenario finishes the same 24 production services; economic differences are charged CIL, not a newly demonstrated functional capability.

## Two distinct forms of prior structure

1. The alternative candidate code paths A/B, their equivalent service semantics, the cost comparison, and the code-edge addresses are researcher-authored. A paid in-agent comparison is real but is not candidate invention.
2. The environmental reversal is injected after a **preselected** stage-12 paid write, and the agent's remeasurement path is likewise keyed to the preselected stage count. Although the environment writes only ordinary cell50, this is not evidence that the individual autonomously detected *when* a change occurred. It was guaranteed to look at a known checkpoint. The two resource modes and all four on/off costs were predicted by a non-native simulator before freezing, so these are engineering feasibility tests rather than blind generalization.

The second limitation is separable from the first: an agent could learn to choose among fixed candidates yet still depend entirely on researcher-scheduled evaluation. Conversely, event-triggered remeasurement alone does not establish candidate discovery.

## Most informative falsifiable next question, not a new registered study

**Hypothesis to test only after reviewing current repository state again:** With the same initial source and no privileged change notification, a charged, internally scheduled remeasurement policy can preserve production correctness and improve *whole-lifecycle net CIL* over both (i) an identical-source physical-edit-suppressed control and (ii) a paid fixed-checkpoint baseline, across a genuinely unseen distribution of change points including no-change and changes after the last economically useful switch.

A negative result is informative if it shows that monitoring cost overwhelms saved execution, a delayed detector misses the remaining payback window, or false-positive switches cause net loss. The measurement must count every SenseOwnCredits, source read, failed probe, candidate trial, PatchNext, rollback, data cell and instruction, not only service costs.

**Necessary design controls before running:** freeze the source and task generator plus immutable unseen holdout manifest before the first native execution; use identical initial source for effect-on/off; include a paid periodic-inspection baseline and a no-change negative control; explicitly count first true exposure, detection delay, stage completion, edit/rollback frequency, and per-world lifetime cost; include late reversals, stable regimes and changes too small to justify switching. Ensure the world does not deliver a free change flag or future time information. A distinct-source simpler policy may be reported as an economic comparator but never as single-factor causal attribution. Independently replay native PC/registers/cells/credits/live edges and retain all failures. Do not call a success spontaneous algorithm invention: the candidate set and inspection policy would still be authored.

**Status:** C27 paid source modification and rollback are supported by prior native evidence; autonomous change-point discovery, profitable generalization, endogenous candidate generation, B3 and B4 remain unproven. This audit changes no frozen protocol or prior results.

## Exact original evidence locator and verification boundary (2026-11-23)

The `cl1b-ci-evidence` branch was directly enumerated rather than guessing a `manifest.json` or `summary.txt` filename. Its exact C27 record is [`run-records/04776b8ee354ef135cf6d9fdf8e22a90e91f7e13-45206215513-1/`](https://github.com/iFrankStudio/cioc/tree/cl1b-ci-evidence/run-records/04776b8ee354ef135cf6d9fdf8e22a90e91f7e13-45206215513-1), containing [`archive-integrity.md`](https://github.com/iFrankStudio/cioc/blob/cl1b-ci-evidence/run-records/04776b8ee354ef135cf6d9fdf8e22a90e91f7e13-45206215513-1/archive-integrity.md), `original-native-artifact.sha256`, and `original-native-artifact.zip` (GitHub reports 68,315 bytes). The archive manifest and `.sha256` file both declare ZIP SHA-256 `11828d4d6947f329e2760738899450d84218e5735841f54c2e083502d6dade1a`. The branch HEAD observed during this check was `08f984e13fcd129ca08c78ba3744052269b654b0`.

**Evidence tier:** authenticated GitHub directory/file-content inspection and agreement of two *archived declarations*, not an independent hash of ZIP bytes and not a fresh paid-CIL replay. This execution environment could not resolve `raw.githubusercontent.com` for binary download. A guessed missing manifest path is not evidence of missing archives; the actual record has no such file. The historical native and archival Actions successes remain separate from any claim of current-HEAD full regression. No new holdout or native world was run for this locator audit.
