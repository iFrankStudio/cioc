# CL1B-POLICY-P4F1 — Paid retention and reversible fallback (prospective freeze)

**Status: preregistered; not native-verified.** Starts from verified P4F0 exact source and its negative heldout shock result, [CI #44707348399](https://github.com/iFrankStudio/cioc/actions/runs/44707348399).

## Mechanism
Keep the original 91-node event-triggered detector, then append 15 original source CIL instructions. After the original PID pays to learn the initial relative stride, it additionally pays `Write35` to retain it. On future tasks with two paid failed guesses, it reads `cell36` (absence means original policy active). When original policy active, it pays PatchConst PC88 to fallback1 and then Write36=1. When fallback active, it pays Read35, PatchConst PC102 restoring the **original learned stride**, then Write36=0. Each state read/write is priced; the original one PID1 is not cloned, reset or privileged.

The actor also pays `PatchNext PC82` to arm the failure-only detector, so no per-success poll is introduced. Original CL0 strict physics 16/180/300 and private credits6000 are unchanged; F1 initial source106 nodes, original F0 comparator91.

## Preregistered experiment and no data snooping

Six P4F0 regimes (54 goal sequences) are **training/retest**, explicitly NOT heldout for F1: F1 on/off 108 original fresh worlds. Two brand-new previously untested `holdout_late_switch` and `holdout_two_shocks` regimes (18 goal sequences) have F1 on/off AND F0 on/off native source comparisons: 72 more original worlds. Two false-feedback F1 controls. Total **182 independent original one-PID1 worlds**, source-frozen JSON lists every one.

Primary same-source causal comparator is F1 PatchNext effect enabled vs only PatchNext suppressed, with exactly equal prior source and all charged steps. F1-vs-F0 is an explicitly separate **cross-source** overhead/gain comparison with both native actors. Preserve every unfavorable heldout result, and do not conflate reduced task probes with positive net paid compute.

## Negative criteria
A rollback invoked after corrupted feedback is not authenticated fitness. An apparent improvement on known P4F0 rare-shock cases cannot count as new generalization; only the new late-switch/multiple-shock goal schedules are fresh heldout. No claim of spontaneous algorithm invention, open-ended evolution, cross-world lineage, or CL-1B completion.
