# CL-1B / B13 — exhaustive single-next-edge mutation map of evidence/function alignment

**Prospective registration: 2026-11-15 before writing B13 scanner or inspecting aggregate output.** B13 is an **observer-side exhaustive robustness screen** over a *human-authored* founder, not endogenous mutation, natural selection, autonomous life, or evidence that an individual proposed any candidate.

## Goal

B12 showed two handpicked cases in which a native CIL marker read falsely indicates service success while the actual service was never performed; a third case misclassifies a working service and triggers damaging code edits. Determine whether these are isolated researcher-selected curiosities or a nontrivial region of the program's **entire single-edge mutation space**.

## Frozen program, mutation operator and finite assay

Program: parse **directly from the checked-out** `fn founder()` in `cl0/crates/world/tests/cl1b_b10_freshness.rs` (do not silently embed a modified alternate founder). It has exactly 20 CIL instructions and 19 `next` fields (all but the Branch node12). The screening lab enumerates all combinations of:
- One editable instruction with a `next` field (19 choices).
- One replacement next address in 0..19 **other than** its original next (19 choices per node).
- Five externally seeded service input labels `[-13,0,7,23,101]`.

Total: **19 × 19 × 5 = 1,805** one-edge mutant worlds plus five unmutated baseline controls. Every world begins with the *same initial input and 200 local credits*, no preseeded output or marker, no external mid-run action, no multiple actors, no pool. Single host `PatchEdge` before execution is explicitly external. Execute **up to 80 charged attempts**; if a trap/unrunnable state occurs, preserve it in the denominator and record charged attempts and reason. No cherry-picking successful mutants.

## Observer metrics, not agent reward

- `service` event = CIL internal `Write` to output key11 with value exactly equal to externally seeded input; record any wrong outputs separately.
- `marker` event = CIL internal `Write` to key13 with value1; event origin and PC must be recorded.
- `self-repair` = CIL internal effective `PatchNext` changing a live code edge; count, target and charges; native suppressions are **not enabled** in this screen.
- **Acceptance false positive (primary):** at an internal PC11 read of key13, observed nonzero is interpreted as healthy by the *human-authored B10 guard*, but no valid service output has occurred since the latest actor PC2 `Delete` of key13 (or beginning of that run). A service emitted later does **not** retroactively validate the earlier acceptance.
- **Acceptance false negative (primary):** at PC11 read key13 observed zero, but at least one valid service output occurred since the latest PC2 Delete.
- **Service-positive but marker-negative** and **marker-positive but service-negative** over entire 80-attempt windows are secondary indicators only; endpoints truncated at 80 attempts create boundary bias. Report both metrics separately and do not call a single write a successful life.
- Report all traps, reasons, attempted instructions, incidental writes (especially unexpected keys), final mutant edge, and output integrity. Never hide harmful intermediate code states.

The scanner is an **independent restricted semantic interpreter** of exactly `Const,Delete,Read,Write,Branch,ReadNext,Sub,PatchNext`. It is required to fail fast on unexpected CIL opcodes or source changes, rather than fabricating compatibility. This is a **static robustness map**; only native Rust VM results (when available) can establish program/VM conformance. Pre-register a separately implemented *native sampled differential* check before claiming quantitative agreement.

## Falsification and next gate

A zero frequency of false acceptance means B12's handpicked concern does not generalize under this mutation operator, and the hypothesis of broad proxy vulnerability is falsified. Any substantial false-acceptance region would show the **engineered evaluator** lacks mutation robustness, not that a digital individual has developed deceptive goals. False negatives must be counted as seriously as false positives: a "conservative" monitor can destroy effective functionality.

No change to founder or mutation distribution after seeing results is permitted without a **new study label**. The more important future question is whether a **native CIL agent changes its own evaluator/search policy** based on held-out, independently sourced observations and paid resources. Observer-side screening cannot be relabeled endogenous adaptive selection.
