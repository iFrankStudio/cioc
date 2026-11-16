# CL-1B B36 — world-physics audit: draw-only energy gateway versus generic direct pool-paid computation

**Prospective protocol 2026-11-16, before adding the new world-law toggle or seeing B36 outcomes.** Central CL-1B one-individual line, not population evolution. B35 established a *specific environmental-physics absorbing state*: a continuing process at zero local credits and a non-Draw opcode cannot execute even when substantial neutral compute remains in the common world pool. This may be an implementation artifact, **not a necessary property of an abstract compute-energy world**.

## Alternative generic physical law, not organism-specific repair

Add opt-in World `with_pool_paid_all_instructions()` (default **false**, persistent in Rust serde and separate from B20's existing `with_pool_paid_draw()`). In the alternative world:
- If an eligible native process has **zero local credits** and the common compute pool has **at least one credit**, the world scheduler may execute its **next ordinary CIL instruction of any opcode**, paying **exactly one shared pool credit**, preserving the recorded `pool_paid_attempt=true`. If the process already has local credits, spend a local credit as before. No computation is free.
- `DrawCompute` still obeys its actual requested transfer and the existing `with_recoverable_draw_shortage()` affordability rule: it should not repeatedly burn scarce global compute on a known unaffordable Draw. Every other instruction is callable only according to ordinary CIL operand, storage and trap rules. The scheduler does **not** skip a failed opcode, change code, grant a process credits, repair a trap, or reward service.
- When the common pool reaches zero and local credits are also zero, **no native opcode can execute**. A trapped/stopped process is never restarted by this law. Existing default and draw-only worlds serialize and execute with their previous semantics; no global default update.
- The alternative is applied identically to any process, regardless of its identity or authored function. It is not an invented `REPAIR`, `SURVIVE` or `FITNESS` opcode. An actor can compute only while the generic environment has energy.

## Frozen comparison

Use the **exact existing B35 12-node self-decrementing CIL founder** from its Rust integration source, unmodified, initial **14 local credits**, zero world pool. No new process, no individual top-up, identical snapshots and source code. The exact B35 four packet sequences remain fixed: `[4,5,5]`, `[4,12,12]`, `[5,5,5]`, `[6,8,12]`. Repeat B35's full single-process **ADAPT** and compact **STATIC3** source under:
1. **DRAW_ONLY** — existing `with_pool_paid_draw()`, default law, `with_recoverable_draw_shortage()`;
2. **ALL_NATIVE** — same laws plus opt-in `with_pool_paid_all_instructions()`.

Thus **4 schedules × 2 source programs × 2 world laws = 16 deterministic native worlds**. The old DRAW_ONLY results are baseline **ADAPT real services 1,3,1,1 and static3 3,7,4,7**; the B36 native test must ensure these are unchanged by the new world feature. In ALL_NATIVE, report every failed TryDraw, charged `PatchConst`, actual service event, source literal, pool-funded non-Draw event, trap and remaining energy, including bad outcomes.

## Registered decisive claims

- Under ALL_NATIVE, a B35 actor previously stranded at PC4/5 with zero local credits may now execute that CIL instruction **only when actual shared pool credits are present**, consuming one per step. There is no host-supplied `SetCredits` to rescue it; the source and future program behavior are unchanged. The actor may use up all energy on a poor exploration loop, fail due to negative amount or still deliver little work: retain all such results.
- Count native service only from `Origin::Internal(1)` `CellWritten(key9,value1)`. Do **not** substitute pool-paid step count, mutable credit registers, process-alive flag, source mutation count or the observer's evaluation for service utility.
- Complete local+pool compute inventory conservation must hold for every step and each exogenous packet; no free retries, no reproduction. Default DRAW_ONLY runs must reproduce B35 exact results. For new law verify at least one actual **pool-paid non-Draw native event**. With zero world pool and zero local credits no act is legal, and a halted/trapped actor cannot wake.
- Compare ALL_NATIVE vs both DRAW_ONLY and cheap static3. If the new rule only produces longer computation without better actual work, report that negative result. Avoid privileging a complex preauthored adaptive actor and overlooking the energy-efficiency costs of allowing *any* CIL instruction access to shared compute.

## Science interpretation

This test distinguishes a **substrate constraint** from an **organism capability**. If non-Draw starvation disappears under a generic equally charged execution rule, then a portion of B35's difficulty was caused by an unnecessarily narrow energy gateway. This is a valuable design finding but does **not** imply autonomous adaptation or a native cyberlife breakthrough. CIL still implements a researcher-authored decrement/patch decision, does not autonomously invent its objective, and cannot infer real task quality from merely obtaining compute. Open-ended life and B3/B4 acceptance remain **NOT MET**.
