# CL-1B / B04 — actor-created code checkpoint and native rollback calibration

**Prospective protocol frozen before implementing B04 test (2026-11-15).** Human-engineered CIL guard; **not** spontaneous repair, open-ended evolution, or B2/B3 scientific acceptance.

## Question

Can a *single existing CIL actor* observe one of its own executable edges, write the observed successor to a resource-accounted world cell **by executing CIL**, and later recover that executable edge after researcher damage by reading the previously stored value rather than using a hardcoded expected successor? Does recoverability depend causally on intact actor-created checkpoint data and permission to execute native edits?

All checkpoint content must originate from CIL `ReadNext`→`Write`, not from a researcher-injected correct edge, host snapshot, or Rust intervention. The actor's check-and-restore policy is nevertheless *researcher authored*. Storage is one ordinary charged shared cell, **not** durable, private or secured against external corruption.

## Frozen source / timeline

One 16-node program; one external `Process::new(1, code, 120)`; strict total capacity64, no pool and no process creation. External initial source input: shared key7. Actor's checkpoint destination: shared key12; output: shared key9.

| PC | CIL instruction | Next |
|---|---|---|
|0|Const r0=6 (protected own code node)|1|
|1|Const r2=12 (checkpoint address)|2|
|2|ReadNext r1,node=r0 (observe actual original PC6.next=7)|3|
|3|Write key=r2,value=r1 (internally materialize checkpoint)|4|
|4|Const r2=7|5|
|5|Const r3=9|6|
|6|Read r1,key=r2 **active code subject to damage**|7|
|7|Write key=r3,value=r1 (service)|8|
|8|ReadNext r1,node=r0|9|
|9|Const r2=12|10|
|10|Read r3,key=r2 (read saved edge)|11|
|11|Sub r1=r1-r3|12|
|12|Branch if nonzero→13 else→14|—|
|13|PatchNext node=r0,target=r3,next14 (native restore)|14|
|14|Const r2=7|15|
|15|Const r3=9|6|

Run 16 CIL attempts (through PC15) and fork from the same complete serialized prefork world: process PC6, one CIL-authored checkpoint cell12=7 and one valid output cell9=input. This prefork state is identical across all arms. External actions are **after** fork only. Native origin must be checked on PC3 checkpoint write and PC13 restoration. Apply specified interventions then run exactly 72 more charged attempts (single-process opportunity matching). Total used attempts88, final credits32.

## Frozen four arms × five inputs

Inputs `[-13,0,7,23,101]`, four arms each, **20 deterministic worlds**. Program and prefork state byte-identical across every arm for each input. External interventions must be counted and labeled.

1. **SHAM:** external `PatchEdge(process1,node6,next7)` no-op. Native checkpoint remains7. Postfork service outputs **8**, no native repairs, eight 9-attempt cycles, ending PC6; one extra external event.
2. **DAMAGE:** external `PatchEdge(process1,node6,next8)` skips service PC7. On first cycle, native check sees next8 != saved7 and repairs node6 8→7; **7** correct postfork service writes across 72 attempts, exactly one native `ExecutableEdgePatched`; ending PC6; one extra external event.
3. **DAMAGE + NATIVE PATCH SUPPRESSED:** same prefork world, external damage to node6.next8, opt-in **world-law negative control** `with_native_patchnext_disabled()` applied before running (explicitly labeled experimental control, not counted as a CIL event). Checkpoint remains7. All eight 9-attempt cycles lack service output, with **8 suppressed patch attempts**, zero native repairs, ending PC6; one external damage event.
4. **DAMAGE + CHECKPOINT CORRUPTED:** external set checkpoint cell12 from7→8 *and* external damage to node6.next8. No native comparison mismatch; **0** postfork outputs, no native repairs, nine 8-attempt cycles, ending PC6; **two** external events. This arm demonstrates dependence on intact internal checkpoint information but is not an actor-produced checkpoint corruption.

## Fixed acceptance / reporting rules

For each arm: natural run remains live after 72 attempted instructions (not terminal); one process, no GraphLaunch, no Trap; actor PC6 after 72, credits32, world compute inventory32, external-event counts2 at prefork (load + source cell) plus arm-specific additions1/1/1/2, exact per-arm outputs and patch/suppression events, no CIL-visible host snapshot. All native writes including the checkpoint are observer-traceable and charged. All 20 conditions, including missing outputs, must remain in denominator.

This is **a positive engineering substrate control only**. A hand-built monitor detecting one specific edge and restoring saved data does not establish autonomously *discovered* self-preservation, safe rollback of arbitrary code, cumulative novelty, security of the checkpoint, or organizational individuality.

## Next stronger science gate

A future study must evaluate non-preselected damage sites and hidden faults, multiple native-owned checkpoints made under real storage scarcity, costs of comparing variants, potential self-corruption of the monitor, recovery from checkpoint damage, and benefits across independent environments. Compare with identical-cost sham, checkpoint corruptions, native-edit-disabled and externally restored controls. Do not retrospectively relax these B04 predictions.
