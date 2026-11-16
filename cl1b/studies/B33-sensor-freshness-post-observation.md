# CL-1B B33 — sensor freshness, intervention timing and indistinguishable pre-observation histories

**Prospective protocol, 2026-11-16, before experiment implementation or outcome inspection.** Active CL-1B **single continuing individual** route. No new world instruction or researcher-supplied repair. This is an explicit *negative* study of B31's new sensor: the actor may see a correct but stale pool value. In digital environments, knowing resource state is not equivalent to predicting the environment after a future action.

## Frozen founder and causal contrast

Use the **exact 13-opcode B31 source**, imported from the B31 test module, not a newly chosen decision script. B31's authored sensing rule reads world pool at PC0 and chooses executable request3 if the observed value is exactly4; otherwise chooses request4 through ordinary CIL arithmetic/branch and a charged `PatchConst` at PC7.

Two CIL-source arms:
- **SENSE** with real `SenseComputePool` at PC0.
- **BLIND3** with same-cost `Const r0=4` at PC0, always selecting request3.

Each world starts with the exact same **8 initial local credits** and zero pool. Only the exogenous pool-injection schedule changes; all actors are PID1 in their own isolated worlds and no process2 exists. The single external packet is one of `{0,4,5,6}`, no targeted grants, no host code edits or post-hoc allocation.

For each arm and packet run three intervention timings:
1. **EARLY**: neutral packet is injected before any native CIL charge, so SENSE sees the actual packet.
2. **AFTER_SENSE**: one native PC0 charge is completed while pool remains0, then packet is injected before PC1. SENSE has an accurate sensor reading0, but it is stale after the packet.
3. **AFTER_COMMIT**: the actor pays the full eight pre-Draw native CIL charges with pool0, completing any PC7 executable edit and reaching PC9 `DrawCompute` with zero local credits. Only *then* is the neutral packet injected. There is no opportunity to re-sense or revise code before the Draw if no local credits remain.

**2 arms × 4 packets × 3 timings = 24 worlds.** No reservoir reuse between worlds. The intervention for packet0 is omitted but the timing label still denotes at which step the zero packet would have arrived. Include all no-service cases.

## Primary falsifiable endpoints

- Each completed SENSE PC0 `ComputePoolSensed` event must truthfully report the pool value that existed **at PC0**: packet under EARLY, zero under either delayed condition; there is no false sensor data or forged service.
- SENSE must select request3 by real native `PatchConst` if it sensed4; for delayed packet4 it must select request4. BLIND3 must always select request3. Both must complete charged edits at PC7 and reach a zero-credit pool-payable PC9 Draw.
- Under packet4, **EARLY SENSE produces one genuine PC12 service Write**, delayed SENSE produces **zero** and leaves four credits unused. The blinded static3 actor produces one true service under all three arrival timings. That is an explicit stale-measurement counterexample.
- Under packets5 and6, SENSE chooses request4 regardless of timing and produces one actual service, while BLIND3 chooses3 and also produces one; no claim of broad benefit from sensing.
- **No feedback at zero credit:** once the actor has finished the eight pre-Draw charges, a late packet cannot cause it to return to PC0 or re-plan; neither reading more true information in an observer log nor the success of a hidden counterfactual is CIL-visible.

## Accounting and controls

Every real attempted CIL instruction consumes **one** local or pool credit. Actual total `local + pool` declines exactly1 on each step; only the explicitly external `SetComputePool` packet increases it by its declared amount. Native source change must be measured in the process's actual PC8 literal. The genuine service endpoint is an `Origin::Internal(1)` `CellWritten(key9,value1)` at native PC12, not the mere register value, test oracle or an event telling CIL it won.

An independent **source-parsed B31 founder CIL interpreter** must check the entire 24-world matrix and exact observations. Native B33 must fail on disagreement; never silently rewrite B31's original expectations. Do not claim that a paid sensor constitutes memory, policy search, learning, general intelligence or cyberlife.

## Scientific implication

Even a truthful environmental sensor cannot guarantee effective **closed-loop adaptation** if its value becomes stale before a mutation is committed. In particular, at zero local credits, the actor cannot re-evaluate a previously selected candidate outside the one pool-payable Draw gateway. B33 makes this information/timing distinction concrete and source-verified, and preserves the cheaper fixed request3 control.

The subsequent gate is a protocol in which one continuing individual can *repeatedly* obtain new feedback **after useful work, before committing future changes**, while maintaining enough paid resource to execute evaluation/rollback without stranding itself. Stronger invariants must compare against cheap fixed programs under held-out packet changes; success on a single frozen threshold is insufficient.
