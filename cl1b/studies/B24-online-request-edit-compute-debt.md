# CL-1B / B24 — runtime-computed acquisition-policy edit versus finite compute debt

**Prospectively registered 2026-11-16, before the native test or independent outcome computation.** This is a mechanistic *negative control* for online policy editing, **not** a demonstration of autonomously learned resource policies or cyberlife. Do not promote a self-modifying instruction to learning unless its new value and edit decision are demonstrably selected from informative world feedback and succeed on unseen conditions.

## The genuine question

Can an actor use CIL arithmetic plus the actual `PatchConst` executable edit to change a resource-request quantity *at runtime*, and does that change yield lasting advantage over an otherwise identical process whose native edits are suppressed? Does the extra CIL work needed to revise its strategy create a new **zero-credit deadlock** under normal resource flux?

The observer compares actual program literals and charged CIL outputs, not researcher's descriptions of "adaptive" behavior.

## Frozen founder

One process id1 loaded with **exactly 29 local compute credits**, common world pool0, strict storage capacity (16 cells, 64 program nodes, 128 aggregate units), world laws `with_pool_paid_draw()` and `with_recoverable_draw_shortage()`. Eight native CIL nodes, all manually designed, all one-credit attempts:

| PC | Native opcode and purpose | Next |
| --- | --- | --- |
|0|Const r1=3 (policy-Const node index)|1|
|1|Const r2=9 (actual service output cell)|2|
|2|Const r3=1 (service value **and** step decrement)|3|
|3|Const r0=4 (editable Draw request source)|4|
|4|DrawCompute amount=r0|5|
|5|Write cell9=1 (real service)|6|
|6|Sub r0=r0-r3 (generates candidate quantity internally: 4→3→2...)|7|
|7|PatchConst(node=r1,value=r0,next3) (edits node3's source literal)|3|

No `ReadComputePool`, magical success primitive or supplied winner opcode is introduced. At startup, the first29 native charges execute PC0–3 then fail/retry the pending Draw, consuming the remaining local credits and blocking at PC4. The experimenter supplies **only neutral external world-pool packets** afterward. The actor's code controls the edit, but the fixed decrement1, timing of the edit, objective output and source design are **investigator-authored**. The edit is NOT conditional on a measured environmental fitness gain; it simply follows a service write.

## Frozen 12-world matrix

Run exactly the same source in three world laws: **EDITS_ENABLED** (actual native source literal modified), **EDITS_DISABLED** (`with_native_code_edits_disabled()`, identical attempted-instruction cost and PC progression), and **NO_ENVIRONMENT** (no packets; both edit laws would be identical). To retain three fully comparable policies, use the first two laws for four different neutral supply schedules (8 worlds), plus **STATIC3** (investigator-authored initial request3 with native code edits disabled) for those same four schedules (4 worlds): **12 worlds**. NO_ENVIRONMENT is an additional standalone paired zero-flux sanity control, not part of the primary denominator. All policies use identical eight-node structure/cost, except the explicit initial policy3 literal in STATIC3.

Four supply sequences frozen before outcome inspection (event-indexed, not time-based):
- **Development:** `[5,4,4]`.
- **Held-out H1:** `[5,5,5]`.
- **Held-out H2:** `[4,4,4]`.
- **Held-out H3:** `[3,3,3,3]`.

After the full initial blocked-state run and each external packet, step native CIL until the scheduler returns false or the actor halts/traps; retain all attempts, ending PC, local credits, pool, source literal at PC3, edits, suppressed edits and actual service writes at PC5.

## Registered mechanism and falsifiers

- In EDITS_ENABLED, after the first actual PC5 service, CIL must execute PC6 Sub and PC7 PatchConst, changing source PC3 `Const r0=4` to `Const r0=3` internally. A later successful second draw may set it to2. The edit exists in the native program, not just an observer-side metric. In EDITS_DISABLED, the same PC7 attempt is paid but source PC3 remains4.
- **Compute-cost boundary:** A Draw3 transfers only3 local compute credits. PC5 Write, PC6 Sub, PC7 PatchConst each cost1. The actor therefore reaches **PC3 with zero credits**, at which point it cannot execute the ordinary `Const` even if new world-pool packets arrive. A seemingly promising lower request can thus cause a permanent deadlock. The STATIC3 control must demonstrate this independent of code mutation.
- No edits occur before a successful native service write. All effective `ExecutableConstPatched` events must have `Origin::Internal(1)`. If a native edit suppression is enabled, count charged `NativeCodeEditSuppressed` events and ensure they do not change executable literals.
- Every successful native attempted instruction reduces total local+pool compute by1, each neutral pool packet increases that inventory exactly by its declared amount. No host `SetCredits`, no process2, no repaired code injected by researcher.
- Evaluate service outputs across the **four complete schedules**, not only the favorable development schedule. Record and preserve any scenario where disabling native edits yields more true service outputs, even if that contradicts a self-modification benefit story.
- Strongest acceptable result: runtime-computed policy modification and exact costs are **causally observed**. Do **not** claim objective-based learning or long-term maintenance: the decrement policy, target and reward proxy are authored, and a changed literal alone says nothing about general adaptation.

## Next scientific decision

If B24 shows edit costs can overwhelm lower request quantities, future native agents should economize on repeated policy evaluation/edits, maintain an executable path to Draw when local credits reach zero, and compare paid candidate outcomes rather than automatically decrement. That optimization must be measured against a simpler static policy and unseen supply sequences. Do not add an uncharged "learn" instruction.
