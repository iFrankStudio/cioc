# CL-1B B56-F1d1k — preregistered endogenous mutation-rule synthesis and provenance falsifier (2026-11-18)

Status: **prospective protocol only**. No B56-F1d1k Rust result, no independent native replay, no claim of life. This study follows the completed B56-F1d1j 60-world result; do not repeat F1d1j.

## Source-pinned prior result and motivation

B56-F1d1j full CI run [44305481895](https://github.com/iFrankStudio/cioc/actions/runs/44305481895), tested SHA `5e498437dda8b6d9e1f1ae52ab7423ec7efa0e01`, verified 68 Rust targets and 59 independent observers. Its original PID1 changes a **one-bit persistent rule** between two **preauthored** orders. Repeated NEGATE saves 215 phase-1 charged CIL instructions against frozen R0, but shifted NEGATE→COMPOSED incurs 173 extra, and a targeted false feedback decoy poisons future edits. This is bounded preference tuning, **not rule synthesis**.

## Primary falsifiable hypothesis Hk

Within unchanged `World::with_strict_capacity(16,180,300)`, a single original PID1 can **construct, validate and retain a previously uninstantiated multi-field mutation-rule descriptor** using paid CIL operations, then execute a new descriptor-dependent mutation order on a held-out task, while the host never writes the descriptor and no second PID is spawned.

**Necessary discriminator:** the selected descriptor's ordered mutation sequence must not be reducible to choosing R0 or R1, or to any other complete order prewired as a branch in immutable source. Merely allocating two data fields or computing a numeric variant of R0/R1 does not satisfy Hk. All descriptor-generation grammar primitives must be disclosed as researcher-authored; even Hk success is bounded composition, not de novo ISA invention.

## Feasibility gate before any outcome-bearing native run

1. Extract the **exact** F1d1j 177-node CIL source and all live keys from the frozen JSON and native Rust, and verify equality. Three unused native node slots alone cannot justify appending a new multi-field parser. A separate source refactor is allowed only if it preserves source-identical treatment/control founders and stays <=180.
2. Provide a *time-indexed* cell liveness proof for each candidate descriptor field and its interpreter scratch register. At most16 **simultaneously live shared cell keys** and total storage <=300, including original PID/source. Reuse a key only after demonstrating its old value is dead at that step; do not silently delete the input, feedback, external gate or service output.
3. Freeze the exact graph, data representation, initialization, descriptor-write path, parser, branch predicates, credit budget, and all host intervention timings **in a commit before native outcome collection**. Verify no external writes to the descriptor, no native source changes and no GraphLaunch/PID2.
4. If no valid source/cell allocation is found, publish a minimal counterexample with live ranges and the violated invariant; **do not** call that an exhaustive impossibility proof unless all legal refactorings are searched.

## Outcome-bearing experiment (only after feasibility gate)

Use a minimum of 2 seeds × 3 task schedules × 4 feedback regimes × 4 controls = **96 original-PID worlds**, or preregister a justified smaller complete matrix before running. Schedules must include repeated task, reversed task, and an unreachable function. Regimes: truthful, blind constant success, inverted, and phase-0 targeted decoy; additionally test *genuine absent feedback* as a separate fifth regime if capacity permits, not as fixed zero success.

Controls must use identical original founder source, input sequence and starting budget: (A) generated descriptor enabled; (B) paid descriptor-write shadow with old rule frozen; (C) privileged best static descriptor set by **PID-paid** initialization; (D) yoked descriptor obtained from another world's feedback but paid to install locally. The yoked arm tests whether the new descriptor contains task-specific learned information rather than mere additional search cost. Record every native paid step, candidate evaluation, program slot write, descriptor write/read, score, fallback and heldout service.

**Freeze directional predictions before implementation:** under truthful repeated tasks A should improve future cost against B, but not necessarily beat privileged C; reversed tasks should expose negative transfer; decoy and yoked mismatches should alter future paid edits and may reduce heldout accuracy. If A only toggles R0/R1 or another authored branch, reject Hk regardless of accuracy. If A fails on all previously unseen schedules, report failure without upgrading to generalized adaptation.

## Feedback provenance identifiability (separate hypothesis Hp)

No purely in-band binary feedback, with identical observable bit sequences and no trusted provenance side channel, can distinguish a truthful world from a deceptive world that emits the same bits. **Strong matched-pair control:** hold founder, all prior actor-visible state, inputs, timing and feedback bits equal while varying hidden task correctness; predict identical paid actor traces until an independently observable difference enters. A divergence before any actor-visible difference is a confound or observer bug. This is an epistemic negative control, not a test of self-authenticated feedback.

If an authenticated source mechanism is introduced, preregister its exact charged costs, key/receipt provenance, replay/fabrication controls and 16-cell liveness separately; never treat a host-computed correctness bit as authenticated truth.

## Acceptance and independent verification

Require Rust native world traces plus an independent source-parsed JS step replay (PC, registers, credits, cell presence/value, source fingerprint, writes, PID identity and host interventions), exact-SHA GitHub Actions native logs and original `cl1b-ci-evidence` records. Report failed runs and first divergence, not only final passing jobs. Only after native evidence exists update README and VALIDATION with denominators and a bounded claim.

**Nonclaims:** no F1c founder continuity, B53 authenticated receipt, N56-A/B/C, B3/B4, open-ended evolution or cyberlife from this protocol.
