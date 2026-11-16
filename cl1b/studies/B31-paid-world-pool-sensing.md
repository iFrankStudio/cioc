# CL-1B / B31 — generic paid world-pool sensing, in-CIL conditional mutation and non-Draw deadlock

**Pre-registered 2026-11-16, before adding the generic VM opcode or running outcome trials.** Scope: CL-1B's main **single continuing individual** route. This study tests a missing generic information channel, **not a privileged goal, fitness or repair mechanism**. A CIL actor's ability to read an environmental variable and mutate source conditionally is a necessary but insufficient precursor to endogenous adaptation.

## Substrate change (generic, not life-specific)

Add one append-only numeric CIL opcode `SenseComputePool {dst,next}` with five-cell code ABI **tag31**, preserving all existing tags0–30, graph encode/decode, relocation and serialization compatibility. Semantics:
- Like `Const` or `Read`, it consumes **exactly one normal local compute credit** when native scheduled, then writes the **current shared world-pool compute balance** to one ordinary register as a checked `i64`. It advances via a validated ordinary edge and emits an observer event `ComputePoolSensed{available:u64}` (the event is **not** read by the actor).
- Its `dst` and `next` are validated before mutation; if the pool cannot fit in signed `i64`, trap after the ordinary instruction charge, do not silently wrap or clamp. Never create compute/storage.
- It is **NOT** pool-payable at zero local credits. The existing native exception remains only `DrawCompute`. No implicit host top-up, no autonomous repair, no energy source, no clock, no special life/fitness API.
- Existing 31 opcode tags keep exact prior encodings. Stage 0–B30 native integration tests must continue to pass unchanged.

## Prospectively frozen actor source and controls

One externally loaded founder (PID1), **8 initial local compute credits**, zero shared pool. Supply one neutral shared-pool packet **before the first native instruction** so `SenseComputePool` can observe it while still possessing local compute. The packet is externally attributed `Intervention::SetComputePool`; no process-targeted `SetCredits`.

**Arithmetic correction before writing any B31 integration test or observing results:** the listed PC0..8 startup path executes **eight**, not nine, instructions (PC0–3, exactly one of PC4/5, PC6–8). The founder endowment is therefore **eight**; the sensor must finish at pool-payable PC9 with zero local credits. This preregistered-budget correction is visible in version history and must not be described as an observed result.

A **13-node** CIL program (all one-credit native attempts) uses:
`PC0 SenseComputePool r0 → PC1 Const r2=4 → PC2 Sub r0=r0-r2 → PC3 Branch r0 (nonzero→PC5; zero→PC4) → PC4 Const r0=3 / PC5 Const r0=4 → PC6 Const r1=8 → PC7 PatchConst(node=r1,value=r0) → PC8 Const r0=4 (edited) → PC9 DrawCompute r0 → PC10 Const r2=9 → PC11 Const r3=1 → PC12 Write cell9=1→PC9`.

The source graph **pays eight startup CIL instructions** (PC0,1,2,3, either4/5,6,7,8) and reaches a pending `DrawCompute` with zero local credits. Conditional source mutation at PC7 is a *real native* `PatchConst` of executable PC8. For external pool4, the authored equality comparison chooses request3; for pool5 or6 it chooses request4. This comparison is **not learned**: threshold4, candidates3/4, branch and service target9 are **investigator authored**, and `Branch` supports equality-to-zero rather than general arithmetic comparison.

Three same-size, same-startup-cost arms:
- **SENSE:** actual `SenseComputePool` at PC0.
- **BLIND3:** PC0 is a normal charged `Const r0=4`, forcing the r0−4 branch's zero arm, request3.
- **BLIND4:** PC0 is a normal charged `Const r0=0`, forcing a nonzero branch, request4.

Four frozen neutral environmental supplies `pool∈{0,4,5,6}` × three arms = **12 worlds**. For each, drain all eligible native attempts, count actual `Origin::Internal(1)` `CellWritten(key9,value1,pc12)`, actual executable PC8 literal, paid `PatchConst` event, newly issued native Draw and any deferred attempt. Account external pool supply before execution and all unused compute. No actor or human reward is transferred after its initialization.

Additional preregistered negative physics tests:
1. **Zero-credit sensor block:** load the same CIL source at zero initial credits and neutral pool100. Process PC0 (non-Draw) must not take a step or sense, even with enormous resource available. This distinguishes telemetry visibility *if powered* from miraculous reboot or zero-cost self-rescue.
2. **Overflow discipline:** with initial 8 credits and world pool `i64::MAX+1`, first `SenseComputePool` must trap with a real paid event; registers and PC must not wrap into an apparent signed number. Do not claim `total_compute_inventory()` exists if the aggregate overflows.
3. **ABI determinism:** `SenseComputePool{dst:0,next:1}` must encode to `[31,0,1,0,0]`, decode to itself and preserve old tag23 Draw encoding; an invalid operand must be rejected without creation of a usable executable.

## Registered falsifiers and interpretation

- **Causal visibility:** SENSE must record an *actual native* sensed pool value equal to the externally supplied balance, and the conditional real executable PC8 request value3 or4. BLIND arms receive identical resource packets but cannot sense them and must select the fixed source request. Invalid edge/register tests and compute charges must hold.
- **Work effect:** Under pool4, SENSE request3 can produce genuine service, BLIND4 request4 cannot. Under pool5/6, both SENSE and BLIND4 can service. **BLIND3 may tie or beat SENSE** because always requesting3 can be more resource-efficient; retain these counterexamples and do not select only favorable arms.
- **Hard barrier:** If the program exhausts local compute on a non-Draw PC, `SenseComputePool` cannot revive it. No telemetry opcode is permitted to bypass the same accounting physics as ordinary CIL.
- **Learning NOT claimed:** The sensor merely provides true environmental information. The source code explicitly contains the experimenter-chosen strategy, fixed threshold, mutation target and reward/work service. Source mutation conditioned on a sensed value is **not** independent learning, goal discovery or cyberlife.

If verified, B31 removes the *specific substrate impossibility* that CIL has no access to neutral world compute state **while it can pay for an observation**. It does not fix B20/B24's zero-credit non-Draw deadlock. B32 should test whether an individual can **amortize observation/modification costs and maintain a Draw-reachable execution state** under held-out changes while outperforming a researcher-authored static policy, without an oracle granting extra resources.
