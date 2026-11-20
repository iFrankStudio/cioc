# CL1B-POLICY-P4F3 — Frozen original-native online task-delta self-relearning

**Frozen before native P4F3 execution.** P4F1 remains the independently verified prior. The 72 existing P4F1 sequences were available during source-only candidate exploration; they **are not heldout**. Exactly four new regime families × three initial goals × three initial deltas = **36 new heldout 24-stage goal sequences** are fixed in the JSON before any P4F3 native outcomes.

## Mechanism

One original continuous PID1 learns its initial stride by the earlier paid P2/F1 success-delta chain. F3 changes `PC82.next` at preload, so the original actor pays `Write37` with its second successful goal, then pays `PatchNext PC131` to redirect its own executable successful-task `Write32 PC34.next` to PC106. Every later task success requires charged feedback, and on a task that first had an actual failed `Write11` probe, the actor pays `Read37`, derives `(current−previous) mod3`, and pays `PatchConst PC122` changing the executable stride `PC46`. The actor pays `Write37` after success, and pays `Read32` to restore its task-stage register before the original source continues. Full instrumentation must prove original PID provenance and no host postload code injection.

This deliberately spends extra CIL instructions **even in stable environments**; benefits cannot be inferred from reduced probes alone. Actor does not know hidden goals, receives feedback only following its original paid service `Write11`, and remains within original strict CL0 physics. The source includes no free diagnostic oracle.

## Primary comparisons

Each of 72 cases has 3 original-native independent one-PID worlds: F3-on, identical-source F3-PatchNext-off, and earlier source F1-on. The first pair is the same-source causal **bundled two-edge source-arming** intervention, not separate isolation of completion vs failure hooks. F3 vs F1 is cross-source and must never be mislabeled as the effect of just one opcode. Add F3 inverted and forced-zero external feedback controls: **218 original native worlds** total.

Primary comparisons are net charged original CIL steps, actual service probes, task completion truth, paid source mutation counts, per-regime harm rates and fresh-heldout-only results. Independent CIL replay must fail at the first native opcode divergence, and exact tested source+CI SHA must be recorded. All negative outcomes remain publishable.

## Limits

Original actor follows a researcher-written heuristic; this is online self-relearning of an executable policy, **not the self-invention of a new heuristic**. All worlds use independent PID1; no open-ended single-instance lineage or general online optimality claim.
