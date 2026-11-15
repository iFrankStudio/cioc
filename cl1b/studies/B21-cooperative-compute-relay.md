# CL-1B / B21 — a funded companion CIL process can relay environmental compute to a blocked worker

**Prospectively registered 2026-11-15 before B21 implementation or outcome inspection.** This is a deterministic authored-mechanism test of B20's single-process deadlock. It is **not** autonomous policy discovery, an emergent organism, or a native CI-verified result.

## Question and scope

B20 established that the worker may sit at PC4 `Write` with zero local credits while the world pool remains nonzero, because the CL-0 opt-in pool-paid scheduler can only finance a **DrawCompute** instruction directly. Could a second, resource-funded process belonging to the **same intended experimental organization** acquire the neutral world resource and then transfer a local credit to the blocked worker through existing native `TransferCompute`, without changing CL-0's opcode set or researcher directly topping up the worker?

Any "organization" interpretation is observational and not guaranteed by process IDs. Both process programs, role allocation and target recipient ID are **preauthored**, not evolved, and the researcher supplies neutral pool packets on an explicit schedule.

## Fixed two-role native founders

**Worker process1**: exact B19 five-node source, with node0 `Const r0=2`, node1 `DrawCompute r0→2`, node2 `Const r2=9→3`, node3 `Const r3=1→4`, node4 `Write cell9=1→1`. Starts with2 local credits.

**Relay process2**: five-node source `Const r0=4→1`, `DrawCompute r0→2`, `Const r2=1→3` (recipient process1), `Const r3=1→4` (transfer amount1), `TransferCompute {recipient:r2,amount:r3,next:1}`. Starts with2 local credits. Both use the same generic world laws `with_pool_paid_draw()` and `with_recoverable_draw_shortage()`.

**NO_TRANSFER** control changes only the helper's node4: `Const{dst:1,value:0,next:1}`, keeping graph size and charged attempt count but preventing resource relay. Worker code, world laws, initial endowments and external supply schedule are identical.

## Frozen three arms and source-pinned event schedule

1. **RELAY**: load worker and helper (2 credits each), run scheduler until both block; externally supply **pool packet3**, run until blocked; this packet allows worker to Draw2, execute PC2/PC3 and become **stuck at PC4 without a service write**. Then supply **pool packet5**, run until blocked. Relay should successfully Draw4 (pool-funded), execute its target/amount instructions, transfer exactly1 local credit to worker, which then executes PC4 and writes one valid service. No host `SetCredits`.
2. **NO_TRANSFER**: same two processes, exact same packet schedule3 then5, but relay node4 is an ordinary `Const`. Worker remains blocked at PC4 with **zero service**.
3. **NO_RELAY**: only worker process1, same packet schedule3 then5; after worker reaches PC4 with no local credit, the final pool5 cannot make that ordinary Write eligible. **Zero service** and an unused pool5.

All arms use `World::with_strict_capacity(16,64,128)`, run to blocked state after each packet, account for all charges, process states, provenance and all external packet actions. The native scheduler chooses execution order deterministically; no host per-step favoring or direct process credit changes.

## Predeclared endpoints and limitations

- RELAY: one actual native `Origin::Internal(1)` `CellWritten(key9,value1)` at worker PC4; native relay PC4 transfer has `Origin::Internal(2)`; after packet5, worker and relay at PC1 with zero local credits, pool0.
- NO_TRANSFER/NO_RELAY: no native worker PC4 write. NO_TRANSFER relay still consumes its own available compute; NO_RELAY retains externally supplied pool5 after worker blocks, demonstrating that resource availability alone is not eligibility.
- Every successful native attempt consumes exactly1 unit total; external pool packets change inventory by exactly +3 and +5. **Resource was acquired/redistributed, never created by the program.** Host process loads are initial endowments and have `Origin::External`.
- Each arm retains all attempts, deferred operations, birth count, trap/stop states and pool residual. The helper cannot be silently interpreted as independently alive or a native adaptation mechanism; it is an engineered multi-process repair-of-fuel deadlock.
- If real Rust scheduling does not match the predicted post-packet states, the preregistered native positive condition fails and source must not be silently revised.

## Next step

B21 would show that CL-0 already contains a *generic, CIL-expressible cooperative path* around a single-process zero-credit deadlock, without a new energy/status oracle. It would **not** show that an individual autonomously formed this cooperation. The meaningful future test is researcher-independent organization of such relays and resource allocation under varying held-out packets and competing consumers, with real resource budgets and agent-created code/relations. No CL-1B acceptance before native CI.
