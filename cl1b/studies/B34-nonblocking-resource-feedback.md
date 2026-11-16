# CL-1B B34 — paid nonblocking resource attempt returns real failure evidence to CIL

**Prospectively registered 2026-11-16, before new VM opcode or outcome inspection.** CL-1B's primary single-individual route. Scientific purpose: test an explicit *generic, charged, local-feedback* alternative to B33's prior-state sensing. **Not an autonomous learner, reward oracle, self-generated goal or cyberlife claim.**

## Generic append-only instruction

Introduce `TryDrawCompute { amount, dst, next }` with numeric code tag **32**, maintaining exact existing tags 0–31. This is a nonblocking local-compute-paid attempt to transfer shared compute into the running actor:
- Every attempt **costs one local compute credit**. At zero local credits, it is **ineligible**, even if the world pool is full. Only the already-existing ordinary `DrawCompute` has the exceptional pool-paid ability. Nothing silently reboots a stranded actor.
- Read a nonnegative request from ordinary register `amount`; invalid register/negative request/overflow traps at normal instruction cost.
- If the shared pool has at least requested credits, **transfer them from pool to the same process** and write **1** to `dst` (true outcome). Otherwise **transfer nothing**, write **0** to `dst`. In both cases advance to `next` and emit observer-only `ComputeDrawTried { requested, available_before, success }`. The actor learns success only from its written register. It is not a success score for any higher-level service.
- Validate destination register and executable next before side effects. Preserve exact global compute inventory: each charged attempt reduces local+pool compute by 1, and the successful transfer is inventory-neutral.
- `TryDrawCompute` is NOT pool-paid at zero local credits and does not bypass resource conservation. The instruction is generic for *any* process, not specific to a cyberlife experiment.
- Add native ABI, serde, graph relocation and invalid operand checks. Old opcode tags must not move.

## Frozen 10-node founder and fixed controls

One researcher-loaded process PID1 with **6 initial local credits**, common pool initially zero. Ordinary generic `with_pool_paid_draw()` and `with_recoverable_draw_shortage()` laws, strict capacity. The founder's entire selection program is explicitly prewritten:

| PC | Source instruction | Behavior |
| ---: | --- | --- |
|0|Const r0=5|first candidate resource request|
|1|Const r1=0|address of executable PC0 literal|
|2|TryDrawCompute(amount=r0,dst=r2,next3)|charged actual attempt; writes success 1 / failure 0|
|3|Branch(r2,nonzero7,zero4)|branch from actual returned resource result|
|4|Const r0=3|hardcoded fallback|
|5|PatchConst(node=r1,value=r0,next6)|native paid edit of PC0 5→3|
|6|DrawCompute(amount=r0,next7)|normal pool-paid resource access|
|7|Const r2=9|true service cell address|
|8|Const r3=1|true service value|
|9|Write(r2,r3,next6)|actual service and recurring Draw|

All instructions charge real compute; after a failed request at pool4, the six initial local credits cover PC0–5 exactly, leaving the actor at **pool-payable PC6** with zero local. The fallback Draw3 requires precisely4 common-pool units (one charged attempt + transfer3), producing one genuine service (PC7–9). This program is still a **fully authored fallback**, not a candidate independently discovered by the actor.

**Controls:**
- **TRY** real generic instruction; branch on real success/failure and edit PC0 on failure.
- **FORCED_FAIL** same 10-node executable geometry; replace PC2 by an ordinary charged `Const r2=0` so the investigator forces the fallback but no real resource attempt occurs. Controls the extra cost/benefit of knowing outcome: a preprogrammed fallback can tie or beat TRY.
- **STATIC3** compact five-node original service loop requesting3, initial six credits (same grant), no evaluation/edits. Crucial best cheap immutable baseline.

Four neutral single-packet amounts `{0,4,5,6}` injected **before any native step** × 3 source arms = **12 registered worlds**. Additional zero-credit nonblocking-attempt control: load TRY with zero local credits and world pool100; at native PC0 the actor cannot step; a second otherwise identical program positioned at PC2 with initialized registers and zero local credits also cannot execute TryDraw even when pool100 (only regular Draw is pool-payable). Optionally use a late-arrival cohort with packet injected **immediately after PC2**, checking that the attempted outcome remains truthful at the *attempt* but can become stale later. Any extra worlds must be preregistered before running them.

## Registered outcomes and limits

- TRY with packet4: a real native resource attempt fails and returns r2=0 (not a service), ordinary CIL then branches to prewritten request3 and patches its own PC0 literal5→3; a subsequent genuine Draw3 and PC9 Write produce actual service. FORCED_FAIL may achieve the same service without making a resource attempt; STATIC3 may be better because it avoids the exploration/edit overhead.
- TRY with packet5/6: the trial request5 can succeed, reports r2=1 and advances to work path PC7, leaving PC0 literal5 unchanged; any later `DrawCompute` scarcity obeys the normal world law. The lack of code modification on successful candidate does not imply that CIL learned a universal best request.
- With packet0: TRY reports failure and consumes normal compute but produces no service. No special environmental grants after the frozen packet.
- Record all actual native `ComputeDrawTried`, `ExecutableConstPatched`, `DrawDeferred` and PC9 `CellWritten(key9,value1)` events; expose exact source code edits, PC, local and pool credits, traps and attempted instruction costs. A host-observer checking actual work is **not accessible as a new CIL reward**.
- Compare against both controls in **all** packet amounts, including cases where FORCED_FAIL or STATIC3 is superior. A successful feature is *resource-outcome evidence available to CIL at real compute cost*, not autonomous optimization or CL-1B acceptance. A credible next experiment must show a continuing actor **repeatedly evaluates alternatives using actual prior functional outcomes**, retains beneficial modifications, and beats compact fixed policies on held-out environmental sequences without an authored always-fallback cheat.
