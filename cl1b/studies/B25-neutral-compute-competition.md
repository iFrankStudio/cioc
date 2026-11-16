# CL-1B / B25 — neutral-pool competition, strategy filtering and scheduler-order confounding

**Prospective registration 2026-11-16, before B25 test and outcome calculation.** A *population/ecology prerequisite* study, **not** endogenous life, natural evolution, reproduction, self-generated policy selection or a fitness function supplied to CIL.

## Research question

In a single deterministic CL-0 world, two equally funded, equal-size, **investigator-authored** native CIL resource-acquisition policies request 3 or 4 shared compute credits per `DrawCompute`. Does environmental scarcity filter those policies independently of numeric process ID? Under more permissive pool pulses, can deterministic scheduler priority determine which policy obtains the shared compute first? The causal objective is **real native service output, not simulated survival label**.

This separates two frequently conflated notions:
1. **Substrate selection pressure:** code that asks for an affordable bundle can receive resource and produce genuine output, whereas code whose Draw threshold is too high cannot.
2. **Scheduler/identity confounding:** when both codes are eligible, winner order may depend on process IDs and last-scheduled cursor, not inherently on resource efficiency.

All candidate programs are fixed **before execution** and are not self-created, self-mutated or chosen by the organism.

## Frozen policies and physics

The two canonical founders are the exact B19/B20 five-node service loop:
`Const r0=request → DrawCompute r0 → Const r2=9 → Const r3=1 → Write cell9=1 → DrawCompute`.
Only the initial requested integer differs, **3 or 4**. Both programs use five instruction nodes, two local bootstrap compute credits, zero initial shared pool, and both generic opt-in world laws `with_pool_paid_draw()` and `with_recoverable_draw_shortage()`. Each charged attempted native instruction consumes one unit of combined process+pool compute. Strict capacity: 16 cells, 64 max nodes/process, 128 aggregate storage.

For each contested world load exactly two processes with 2 credits each:
- **ORDER_34:** process1 requests3, process2 requests4.
- **ORDER_43:** process1 requests4, process2 requests3.

These assignments are not a "winner" designation: both actors have equal grants and structure. The host cannot top up individuals or directly edit policy after loading.

## Fully frozen 16-world matrix

Four identity-neutral **six-packet** world-pool schedules are held fixed. For each schedule, run both contested ID assignments plus two single-actor solo controls (request3 or4 with process ID1): **4×4 =16 worlds**.

- **A (threshold-only):** `[4,4,4,4,4,4]`.
- **B (both possibly eligible):** `[5,5,5,5,5,5]`.
- **C (mixed):** `[4,5,4,5,4,5]`.
- **D (reverse mixed):** `[5,4,5,4,5,4]`.

After loading, run all native eligible instructions until both actors block; initial2 credits per actor should pay one request setup and one deferred Draw, with zero real service before the first packet. Then inject each pool packet **externally and without targeting a process ID**, run native until both block, record actual charged attempts and source ID of each `CellWritten(key9,value1)`. Track each actor's last PC, local balance, residual pool, traps/halt, and actual per-packet service attribution.

The single-actor controls are *opportunity baselines* and differ in initial total bootstrap credits from contested worlds by one actor; after boot they have identical neutral pulse budgets. Do not claim a matched total-inventory ablation across world sizes.

## Falsification endpoints

- All 16 worlds must be retained including zero-output competitors. No researcher-chosen survivor and no fitness reward/credit awards.
- Both ID orders must put request3 and request4 at zero local credits and pending PC1 before the first neutral pulse. Both have the **same code size** and initial credits; the sole initial functional difference is the authored request literal.
- Under schedule A, a four-credit world packet is enough to pay the pool-funded `Draw3` attempt fee1 plus transfer3, but never sufficient for a `Draw4` attempt fee1 plus transfer4 **unless the pool carries unused surplus**. Record actual residual pool and test whether the request3 strategy receives all work opportunities regardless of ID swap.
- Under schedule B, when pool5 is available with both strategies blocked, both are scheduler-eligible. Do **not** assume that observing more work from either proves intrinsic strategy fitness; compare assignment swaps and exact native scheduling event order.
- Schedules C/D must be independently retained even when they disconfirm a simple threshold-only prediction; accumulated unspent pool credits can change which strategies become eligible.
- Every successful attempt spends one compute unit; every external pulse increases world total exactly by its amount. An individual may produce service only via native PC4 `Write`. Shared output cell9's final value **alone** is not evidence of which actor performed the work. Use event `Origin::Internal(pid)` and PC to attribute it.
- A result would show **deterministic resource-based ecological filtering**, not evolution. The actors cannot mutate their request values and have no native fitness-signal output; the host created both competitors.

## Scientific decision

If strict scarcity selects request3 independently of ID but looser supply introduces order effects, the next actual evolutionary experiment must vary executable requests **from native internal mutation** and inherit them through genuine CIL graph/process lineage, then verify whether environmental selection acts on those variants without host-chosen winners. Separate scheduler effects by swapping IDs or initial load order and preserve full lineage/energy conservation. An engineered competition between two fixed founders is **not** a living population.
