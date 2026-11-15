# CL-1B / B19 — identity-neutral compute influx with native CIL acquisition

**Prospective registration on 2026-11-15, before B19 test implementation or independent outcome inspection.** A feasibility calibration for B18's resource-flow necessary condition, **not proof of cyberlife, self-originated strategy, natural evolution or self-sufficiency**. All seed CIL and supply schedules are authored by investigators.

## Question

B18 shows that a finite closed initial compute inventory cannot sustain unlimited native CIL instruction attempts. Can the existing generic `DrawCompute` substrate let an individual **actively obtain new environmentally available compute** while a matched actor lacking access cannot, without the host directly refilling a favored process or recognizing a "life" label?

The shared resource pool is **identity-neutral**: the host creates a series of finite compute packets by **external `SetComputePool` actions** before each event-indexed observation window. Any eligible native process can draw. This is an externally modeled environmental energy influx, **not actor-created compute**. The native actor must pay for its `DrawCompute` instruction, make its own attempted draws and pay for all service writes.

## Frozen five-node actor founder

One actor id1 with exactly2 initial credits, strict capacity (16 cells, 64 code nodes, 128 total units), no spawn, no after-initial `SetCredits`.

| PC | Opcode | next |
| --- | --- | --- |
| 0 | Const r0=4 (resource request) | 1 |
| 1 | DrawCompute amount=r0 | 2 |
| 2 | Const r2=9 (service output key) | 3 |
| 3 | Const r3=1 (service data) | 4 |
| 4 | Write cell9=1 (native service, then attempt another draw) | 1 |

Base world uses both generic opt-in world laws: `with_pool_paid_draw()` (when local credits0, eligible Draw can pay its own fee from the pool) and `with_recoverable_draw_shortage()` (insufficient pool does not trap, but any actually attempted shortage still charges one credit). No other identity-specific energy or protection logic is installed.

After initial two charged attempts, PC1 is blocked with zero local credits and empty pool, without being stopped. Supply events are **experimenter actions**: `SetComputePool { credits: old_pool + packet }`. They do not address a process ID or directly refill an individual.

## Frozen five-arm native matrix

Run one individual per arm, with source graph **the same five nodes** except for the explicitly specified one-opcode no-access control. Source code and energy pulses are not changed after seeing results.

1. **RENEWABLE_ACCESS:** six external packets each amount5. Each packet is followed by `World::run(100)` until no process is eligible. Expect exactly one PC4 service write per packet (six total), and the process to remain at PC1, unstopped and zero local credits between packets. This exceeds its starting 2 local compute credits because 30 additional credits entered the **world pool**, all externally attributed and paid for through native Draw.
2. **NO_DRAW_OPCODE:** same graph length and registers but replace node1 with `Const{dst:1,value:0,next:2}`, so CIL never attempts the Draw primitive. Same initial credits2 and six amount5 packets. The process exhausts initial credits and produces **no service outputs**, while all30 pool credits remain unavailable to it. Matched initial program length and supply, deliberately different native policy.
3. **DRAW_NOT_POOL_PAID:** exact original founder and six packets5, but do **not** enable `with_pool_paid_draw()` (still enable deferred shortage). After initial2 credits are used while pool is empty, it cannot pay a future Draw charge; externally supplied pool accumulates unused. This is a **world-law control**, not a living strategy.
4. **SUBTHRESHOLD_THEN_CROSSING:** exact original founder and world laws; packet sequence `[4,1]`. First packet4 is insufficient to pay a pool-funded draw fee1 **and** transfer request4, so the actor remains blocked. Second packet1 raises available pool to5, allowing exactly one native Draw and one PC4 service write. The test reports the two separate external supply events and no hidden automatic wakeup.
5. **NO_ENVIRONMENTAL_INFLUX:** exact original founder/world laws and **zero packets**; after two native charged attempts, no further service or draw is possible.

## Primary quantitative endpoints

Before/after every **native** charged step, total inventory `local credits + pool` must decrease by **exactly1**. Before/after an explicit external `SetComputePool`, total inventory must increase by **exactly the declared packet**. No other action increases inventory. `Origin::External` is required on packet injection; native Draw is `Origin::Internal(1)`, not a host-granted successful-action reward.

Observe CIL PC4 `CellWritten(key9,value1)`, native Draw execution at PC1 (including `Event.pool_paid_attempt`), `DrawDeferred`, actual ending local/pool compute, traps, stops, and each supply packet. No process births. All failing and zero-output arms included, no censoring or cherry-picking.

**Finite packets imply finite total lifetime work.** Six successful periods are *not* evidence of infinite self-maintenance. Longer work here depends on further external pool supply. The individual can access a **generic environmental input**, not synthesize compute.

## Scientific gate following B19

The next scientifically discriminatory question is whether an **individual-originated variation in compute-acquisition policy** can improve access under previously withheld, identity-neutral resource pulse patterns while preserving function and avoiding evaluator spoofing. This requires paid native policy edits, matched initial energy/compute and same-world competition; giving extra world credit to a designated "winner" is prohibited. B19 should be treated as a **necessary substrate feasibility control**, not a B3/B4 adaptation or life acceptance result.
