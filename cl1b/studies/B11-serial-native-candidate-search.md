# CL-1B / B11 — within-one-actor native candidate search by repeated trials

**Preregistered 2026-11-15 before B11 code implementation, simulation or outcome review.** B11 tests a limited but distinct mechanism: a single CIL process **constructs and tests more than one candidate modification over successive rounds**, using fresh native output-marker feedback and finite resource charges. The *search policy and target site are human-authored* and cannot constitute spontaneous adaptation or cyberlife.

## Question

Can a single resource-limited CIL actor, starting with one of several unknown-to-it altered control-flow successors, recover functionality by successively reading its **current own executable next edge**, proposing a **one-step neighbor** and applying a native edit, then repeating a fresh service trial and retaining the first proposal that creates the required marker? Does suppressing native code edits prevent progress? What side effects occur when a proposed path passes through executable instructions in an unexpected register context?

## Frozen code

Reuse B10's exact 20-node program and fixed marker-reset/trial loop, with **one single difference**: node14 `Const r3=1` (instead of B10's `Const r3=3`). Thus on each unsuccessful trial the actor computes *current* node6.next **minus one** (node13 `ReadNext` → node14 literal1 → node15 `Sub` → node16 `PatchNext`). This is a **hardcoded neighboring-variant search heuristic**, not a host-injected absolute target.

- Service target node6 initially has next7, which enters both PC7 marker writer and PC8/9 output writer. A service trial begins at PC0 by CIL-deleting marker cell13; success is a native PC7 `Write(13,1)` later read at PC11.
- External founder and source input cell7 are fixed; 250 initial credits, strict world capacity64. CIL executes in one process id1. No clones, host code repair after declared initial lesion, pool replenishment or researcher-side candidate scoring.
- The investigator externally chooses the initially damaged next edge in `{8,9,10}` (and sham 7) **before execution**. The actor itself has no branch that directly selects the target edge7: it must arrive there by sequential actual readback and charged native edits.
- One candidate can be harmful: at proposed next9, PC9 `Write` executes with register r3=1, writing source input to **cell1 rather than the service output cell11**. Count this side effect and its finite storage charge. At proposed next8, the output cell11 may be written, but there is still **no fresh marker13**, so the actor must reject the incomplete candidate. Only next7 produces both marker and intended output.
- The marker's meaning remains human-programmed and can be actively spoofed; B11 is not a general trustworthy fitness function.

## Frozen experiment matrix

Five input labels `[-13,0,7,23,101]` × six arms = **30 deterministic worlds**. Exactly **100 CIL instruction opportunities** per world after initial host setup.

1. **SHAM7:** no lesion, node6.next7, native marker and output; no real code edit.
2. **LESION8:** researcher changes node6.next to8, missing marker, but some output can occur before repair. Actor should propose and apply8→7 after one failed trial.
3. **LESION9:** researcher changes node6.next to9, missing marker and potentially writing to cell1. Actor should propose9→8 and then8→7 after successive failed trials.
4. **LESION10:** researcher changes node6.next to10, bypass both outputs; actor should propose10→9, then9→8, then8→7 after three native failed trials.
5. **LESION10_PATCH_DISABLED:** same as LESION10, with `with_native_patchnext_disabled()`; proposals incur paid attempts but no effective edits or valid native outputs.
6. **LESION10_STALE_MARKER_AND_OUTPUT:** host pre-seeds marker13=1, output11=input **before execution** and applies lesion10. Actor's own PC2 Delete must remove the pre-seeded marker before the first trial; outcome should match LESION10's native edit sequence rather than accept the stale output.

All initial host interventions are logged as `Origin::External`; all proposed edits, trial marker and service writes must be `Origin::Internal(1)`. Every arm starts with the same exact 20-node source (pre-intervention). The exploration window is fixed at100 attempts, not tuned to the observed time to success.

## Falsifiable endpoints

- `LESION10` must show **three distinct temporally ordered native edits** `(6,10,9)`, `(6,9,8)`, `(6,8,7)`, each preceded by a failed, CIL-local marker trial, and then a fresh correct PC7-marker/PC9-output pair. `LESION9` must show the latter two edits, `LESION8` the latter one. No researcher injects a candidate target value after execution begins.
- Repaired worlds must retain the winning next7 edge throughout the remaining experiment; no additional effective code edits. The patch-disabled arm retains next10 throughout and no native service output, despite paid edit attempts. The stale-data arm must not prematurely accept old output.
- Record every unexpected cell write (especially key1 in the next9 candidate), compute/storage occupancy, live actor PC, traps and all resource charges; do not omit damaging intermediary candidates from analysis.
- All 30 conditions retained whether successful, failed or trapped. Where an arm does not reach 100 attempts, explicitly mark **non-runnable** and fail this preregistered equal-opportunity calibration rather than adjusting source/threshold post hoc.
- This demonstrates only a **researcher-authored bounded local search policy** expressed in generic native CIL, not the spontaneous creation of a utility function, repair strategy, algorithmic novelty, evolving identity or open-ended improvement.

## What would count as a stronger next stage

B11 separates **internal proposal** from **external proposed code**, but does not solve endogenous **selection criteria**. A subsequent prospective experiment must allow the candidate to revise its own search/diagnostic policy based on *held-out* world evidence, while maintaining resources and avoiding false acceptance from stale/spoofed marker sources. Candidate policies must not be handpicked by a Rust harness and relabeled as individual learning. Existing B00–B10 native Rust evidence remains unverified; source-level success of B11 cannot silently advance scientific acceptance.
