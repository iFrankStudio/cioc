# CL-1B / B18 — finite compute inventory makes indefinite native continuation impossible

**Prospective protocol (2026-11-15)**, before writing B18's Rust test or inspecting its native outcomes. This is a **physics-level necessary-condition analysis** of the CL-0 kernel, not a sufficient condition for cyberlife, spontaneous metabolism, agency, or evidence for CL-1B acceptance.

## Fundamental issue

All previous B00–B17 fixed-budget assays begin with a finite actor credit inventory and generally **zero compute pool or external replenishment**. Their observation window ends while some actor remains runnable, which does **not** establish the capacity for ongoing self-maintenance. For an autonomous individual to execute without an arbitrary upper bound, it must acquire a stream of usable compute from somewhere; rearranging finite code, checkpoint cells, service outputs or self-editing instructions does not create usable compute.

**Candidate source-level invariant:** Let

\[
E = P + \sum_{i\in\mathrm{processes}} c_i,
\]

where \(P\) is shared compute pool and \(c_i\) is current local compute credit. In the CL-0 deterministic world **with no externally initiated credit/pool intervention or process load after the start**, each successful native scheduler attempt consumes exactly one credit:

\[
E_{k+1}=E_k-1.
\]

Native `DrawCompute`, `ReturnCompute`, `TransferCompute`, and `GraphLaunch` **redistribute** existing credits, not mint them. An opt-in pool-paid `DrawCompute` may charge its instruction from the pool instead of a process but still decreases total E by exactly one. Therefore no execution using only this kernel's native instructions can contain more than \(E_0\) charged attempts when \(E_0\) is finite. Traps/halts/stranded pool credits can make the actual attempt count **strictly smaller**.

The theorem is conditional on **the present source code and world physics**. If an external world model later injects energy via an explicitly modeled resource flux, the premise fails; the bound must then include cumulative inflows \(I_k\): \(\mathrm{attempts}\le E_0+\sum I_k\). Allowing environmental input is physically sensible and not an actor-targeted fitness/reward oracle by itself, but experimenter-assigned free credits to a favorite individual cannot count as autonomous resource acquisition.

## Frozen native counterfactual cases

Use the actual CL-0 world VM and native event ledger; **do not introduce a new organism maintenance opcode**.

1. **FINITE_SINGLE:** actor1 with 10 initial credits, source graph one instruction `Const{dst0,value0,next0}`, no pool. Attempt exactly10 steps, charge1 each, then `step()==false` and future attempts0 despite continuing loop-shaped code. E=0; no death because of biological aging.
2. **FINITE_POOL_CYCLE:** actor1 with4 initial credits and source `Const{r0=1,next1}`, `DrawCompute{amount:r0,next2}`, `ReturnCompute{amount:r0,next0}`, and world pool5. Enable the optional **pool-paid Draw law**; it changes where the scheduler gets the charge but cannot alter E decrease. At most9 attempted instructions; trap or residual unreachable pool is allowed and must remain visible.
3. **FINITE_TWO_ACTORS:** actor1 initially6 credits, actor2 initially4; actor1 program sets recipient2 and amount1 then executes repeated native `TransferCompute` to actor2 (while actor2 loops doing ordinary work). Show compute can move between individuals, but the total of their local credits + pool is consumed one attempt at a time. There may be fewer than10 attempts if both actors become unrunnable.
4. **EXTERNAL_REFILL_POSITIVE_CONTROL:** start with the same single actor as FINITE_SINGLE with only5 credits, exhaust exactly5 opportunities, then the **researcher explicitly** calls `Intervention::SetCredits{process:1,credits:5}`. Five more attempts become possible; the renewed compute inventory and intervention are **externally sourced**. This falsifies any misinterpretation that native CIL created compute from nowhere.

Across each native step, compare `E_before-E_after==1` and check exactly one internally sourced scheduler event. No traps or unexpected newborn processes outside the explicitly anticipated pool-cycle control. Check input credit accounting with integer overflow protection, including zero local credits and nonzero pool credits. No host intervention occurs during the finite closed-system runs. The externally refilled control must be labeled as **energy supplied by the researcher**.

## Scientific consequence / next architectural gate

CL-1B cannot be certified on the basis of a finite 100/120-step survival test. A persistent, resource-responsible digital individual needs (a) a genuine **environmental compute flux** or repeated opportunity to earn/use compute, (b) a mechanism for the individual to **access** that resource with charged CIL decisions, (c) a competing baseline that merely consumes the same endowment without adaptation, and (d) no host-chosen replenishment masquerading as "metabolism."

Future study should add *generic, identity-neutral resource availability* and compare within-world actor strategies under held-out resource gradients. Do not introduce a life-specific repair/energy law or award free credits based on human-scored output. The exact invariant is a **necessary-physics obstruction**, not an emergence breakthrough.
