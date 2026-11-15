# CL-1B B21 — native CIL companion resource relay across a zero-credit deadlock

**Evidence level:** independent source-parsed JavaScript implementation of CL-0's deterministic two-process scheduling/resource transfers, plus a committed native Rust integration test. **No verified native Rust CI result is claimed.** The [B21 prospective protocol](../studies/B21-cooperative-compute-relay.md) preceded the [native test](../../cl0/crates/world/tests/cl1b_b21_cooperative_compute_relay.rs) and [repeatable three-arm observer simulator](../analysis/b21_cooperative_compute_relay.mjs).

## The same neutral resource supply, three different organizational abilities

A worker process is intentionally trapped in the **B20 post-draw debt state**: it has reached PC4 (its true `Write` instruction) but holds zero local compute credits, so it cannot execute even though its code and service logic remain intact.

A companion process, if present, may execute generic native `DrawCompute` from the **world pool** and `TransferCompute` from its own compute balance to worker process1. No host `SetCredits`, code restore or life-specific rescue instruction exists. Packet schedule is frozen at **+3 then +5 world-pool credits** for all arms.

| Frozen arm | Worker initial credits | Helper initial credits | Pool packets | Paid CIL instruction attempts | True worker PC4 output | Ending pool |
| --- | ---: | ---: | --- | ---: | ---: | ---: |
| **RELAY** | 2 | 2 | 3,5 | **12** | **1** | 0 |
| NO_TRANSFER (helper PC4 is same-cost ordinary Const) | 2 | 2 | 3,5 | **12** | **0** | 0 |
| NO_RELAY (worker only) | 2 | none | 3,5 | **5** | **0** | **5** |

The first packet3 lets the worker acquire2 credits (plus pay one Draw attempt), execute its two setup instructions and run out immediately before the service Write. At that point no CIL worker service has occurred in any arm.

With the subsequent neutral packet5, the RELAY helper executes a pool-paid Draw4, prepares recipient ID1 and amount1, then performs a real native `TransferCompute`. One local credit reaches the previously blocked worker. The worker resumes and emits its PC4 `CellWritten(key9,value1)` **without host process-specific intervention**.

The NO_TRANSFER helper receives the same energy and executes a same-size, same-cost alternative node4; no worker credit is transferred, so no output occurs. The NO_RELAY worker alone cannot execute PC4 even while the environment retains5 credits after packet5. These controls distinguish environmental **availability** from organizational **access** and **redistribution**.

## Provenance and conservation

The neutral pool supply events are explicitly **external**. Each native Draw, helper transfer and worker Write is **internal to its respective CIL process**. The simulator verifies every attempted instruction decreases the combined compute inventory by exactly one; each external pool packet increases it by exactly its declared amount. The Rust test additionally measures the worker's actual credit balance **at the instant helper PC4 executes**, rather than mistaking a generic `Executed` event for a successful resource transfer.

The positive arm's budget is exact: initial endowments4 + external packets3+5 = **12 total native attempts**. Both local and pool balances reach zero afterward; a further attempt cannot proceed without new external resource availability. This is finite cooperative rescue, **not perpetual metabolism**.

## Scientific implication

**An important expressibility fact:** the existing generic CL-0 substrate already supports multi-process resource relays capable of recovering work that **one zero-credit process can never resume by itself** under that same set of world laws. This does not require adding an invulnerable feedback oracle or a rescue primitive to the VM.

But the companion role, target process identity, transfer amount and policy were **fully authored by investigators**. B21 does **not** show spontaneous cooperation, self-generated social/organizational identity, evolving maintenance policy, resource competition, independent fitness or cyberlife.

Next decisive gate: whether a native CIL organization itself **creates, modifies and retains** resource-transfer relationships or acquisition rules as a response to held-out environmental changes, paying finite compute/storage and validating real work despite mutable checker paths. Use the same-world finite pool with competing consumers rather than a host choosing which process receives credits. A native source-SHA-pinned Rust pass is required before any B21 mechanism claim is accepted as a Rust result.
