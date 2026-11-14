# CL-1 Study63 — preregistered generic native information-feedback and compute-access calibration

**Status: PROSPECTIVE HUMAN-SCRIPTED MECHANISM FIXTURES ONLY, ZERO stochastic ecology worlds.** Frozen before its Rust runner or independent Python auditor is created. This is the feasibility gate required by the Study62 breakthrough reassessment; **not** a claim of an emerged organism or a fixed life phenotype.

## The code-level gap

Existing 9+72-node ecological continuation lacks generic ordinary `Read`; the random `Write` always reads register3 (the graph handle), while `GraphLaunch` resets child registers. Even a viable copied CIL code graph is not necessarily a source-sensitive feedback function. The scheduler cannot execute `DrawCompute` at zero credits. Before designing a new stochastic substrate or any lesion/recovery endpoint, verify that current, unchanged CIL primitives can in fact realize state-sensitive repeated I/O *and* acquire available pool credit by **their own native action** with an externally passive energy pool. No CIL repair/fitness/reproduce primitives, no new VM opcode and no goal-directed external actor starter.

## Exactly six researcher-written 8-epoch fixtures (0 independent random worlds)

One actor (ID1), 8 nodes (code/PC and all edges frozen), 3 initial process credits, strict world capacity (16 shared cells, 12 process nodes, 40 total storage units), epoch quota12 and 8 hashed epochs. Initial researcher-provided shared cell7 is the sensor datum, and world pool0 or200 is an explicitly `External` initial resource endowment. Actor program, otherwise unchanged:

- PC0 `Const(dst0,value7,next1)`
- PC1 `Const(dst1,value8,next2)`
- PC2 `DrawCompute(amount1,next3)`
- PC3 `Read(dst2,address0,next4)`
- PC4 `Const(dst0,value9,next5)`
- PC5 `Add(dst3,lhs2,rhs1,next6)`
- PC6 `Write(address0,value3,next7)`
- PC7 `Const(dst0,value7,next2)`.

This is a deliberately HUMAN-SCRIPTED functional loop calibration: it is **not** counted as a spontaneous code organization. Do not change it after results.

Six conditions; initial sensor input and pool/credit interventions are provenance-tagged `External` and applied **before** the first epoch; the only postfork action allowed is the specified sensor-value change at epoch4:

1. `sensor-low-native-pool`: input cell7=3, initial pool200, full original `Read`, initial actor credits3. Expected native successful `CellWritten(key9,value11)` in multiple distinct epochs.
2. `sensor-high-native-pool`: input cell7=8, same initial pool and code; expect `CellWritten(key9,value16)` in multiple epochs.
3. `sensor-switch-native-pool`: initial cell7=3, epoch4 `Intervention::SetCell(key7,value8)` as **external information stimulus only**, initial pool200, original code. Both output11 before epoch4 and output16 after epoch4, with internally attributable Read-before-Write chains; **no** external process-credit intervention.
4. `sensor-high-native-pool-read-ablated`: same high input and pool, but PC3 is an explicitly **researcher-ablated** `Const(dst2,value0,next4)` rather than Read. CIL instructions unchanged elsewhere. Expect repeated output8 independent of the input, explicitly **not** source-sensitive behavior.
5. `sensor-high-empty-pool`: high input, original Read, initial pool0, actor credits3. Third attempted instruction is native `DrawCompute` but must trap (no resource), **zero successful shared output**. (A failed Draw still costs credit.)
6. `sensor-high-zero-process-credit`: high input, pool200, original Read, actor initial credits0. No native attempt; 0 output despite positive pool because kernel scheduler cannot begin the requested Draw. This is the ignition negative.

Each of six original SQLite runs contains exactly eight original state SHA256 snapshots and recorded per-epoch charged attempts; 48 archived hashes. External initial sensor and compute-pool provisioning must be present in the saved `initial_json.events` and must never be miscounted as native output. No host actor-credit refill occurs at any epoch. All actual native DrawCompute events are attributed to actor1 and consume 1 charged attempt, while transferring 8 credits from pool to actor without net creation. The resource inventory at epoch k is `initial_process_credits + initial_pool - cumulative_native_attempts`; `SetCell` is not compute.

## Primary mechanism gates (fixed before execution)

A. **Repeated endogenous input-output function**: each of conditions1,2 must contain at least three distinct epochs with internally attributed successful `CellRead(key7,value=input)` preceding internally attributed `CellWritten(key9,value=input+8)`, and **at least one native DrawCompute success**, without host actor-specific credits.

B. **Within-world feedback to changed information**: condition3 must contain at least one internally attributed output11 before the explicit epoch4 stimulus and at least one internally attributed output16 after, each preceded by an internal Read of corresponding input in the same or prior epoch, with no post-stimulus researcher code edit or actor-credit intervention.

C. **Exact negative controls**: condition4 has no native `CellRead(key7)` and no output16; any output must equal8. Condition5 has zero shared output and at least one native `Trapped(insufficient world compute pool)` event. Condition6 has zero native attempts and zero output. Native state/energy accounting, all 48 hash snapshots and exact Rust replay must pass.

Independently reconstruct **all initial node bytes** and all host actions, successful event origins, input perturbation timing, source-data flow (`Read r2 -> Add r3 -> Write r3`) and pool/credit conservation via Python. No fabricated positive outcome from just a `CellWritten` lacking its causal read. No claims of child heredity, self-organization, recovery, organism or life.

## Decision

If all gates pass, the unchanged Rust/CIL VM is **capable** of sustained native environmental feedback and native energy harvesting once initially runnable; the next step is a *separately preregistered* random-fragment composition ecology with generic Read, variable Write operands and stochastic control-flow opportunities, using source-sensitive recurrent work as its first genuine ecological primary endpoint. If the scripted fixture fails, repair only the mechanism or auditor bug, preserving this original protocol and failed CI evidence. No random ecology, injury/repair test or CL-1 acceptance is licensed by success here.

**CL-1 remains OPEN.**
