# CL-1B / B20 — resource-shortage observability barrier and fixed acquisition policies

**Prospectively registered 2026-11-15 before B20 outcomes, native test or independent simulation.** This is a falsifiable negative/feasibility study of the existing CL-0 kernel and **not** a claim about all possible self-adaptive software or about emergent cyberlife.

## Source-grounded obstacle

With the existing optional world laws `with_pool_paid_draw()` + `with_recoverable_draw_shortage()`, when local actor credits are **zero**, a pending `DrawCompute(amount)` is scheduler-eligible **only if the world pool contains at least amount+1 credits** (one instruction charge plus the requested transfer). If the pool is below that threshold, `World::step()` returns false **without executing a CIL instruction or emitting any event**. If local credits are still positive, an insufficient Draw may consume one local instruction credit, emit observer-only `DrawDeferred` and leave PC at the same Draw instruction; it **does not branch to CIL code that could revise the request**. There is no generic `ReadComputePool` instruction and `DrawDeferred` exists in the host event log only.

Thus a **single actor with zero credits already blocked at Draw** cannot reactively change its request to a smaller amount using CIL self-modification, even if the pool contains a smaller available packet. It can be unblocked by later neutral external pool supply crossing the threshold or by code/policy prepared before exhaustion (or a separate funded companion actor). This is a narrowly stated scheduler/feedback accessibility obstruction, not proof that all digital evolution is impossible.

## Frozen founder and paired controls

Use B19's exact five-node program, with `Const r0=request` at PC0; PC1 DrawCompute, PC2 Const r2=9, PC3 Const r3=1, PC4 Write cell9=1→PC1. All arms receive **2 initial local credits, empty pool, one actor**, strict storage caps, and both opt-in world laws. Initial PC0 Const and PC1 deferred Draw consume the two local credits. Hence subsequent actions require a pool-paid Draw at PC1.

**Policy variants** are investigator-provided constants at PC0 before execution; no host code edits after boot. They do not demonstrate that the individual learned to change its own policy. Compare request amounts **2, 3, and 4**, each on the two energy supply schedules:

- Four separate identity-neutral packets **[4,4,4,4]**.
- Four separate identity-neutral packets **[5,5,5,5]**.

Six source/pulse pairs plus a **zero-pool request4** negative control = **7 worlds**. Between packets run all currently schedulable native instructions until blocked or stopped. Preserve each intermediate event, deferred Draw, actual service write, residual credits, pool balance and any trapped actor. Record the distinction between (1) never eligible for an attempted instruction and (2) attempted Draw deferred with a charged event.

Request2 transfers only2 credits, but the post-draw code needs **three** charged instructions to emit a service and return to Draw. This is a deliberate counterexample to 'smaller requests are always better': after PC2 and PC3, it lacks credit to execute PC4, despite potential environmental pool leftovers. Request3 is just sufficient to complete a service and return to Draw with zero local credits. Request4 needs5 pool for a zero-local pool-paid Draw, and its extra local credit may be wasted on a deferred Draw attempt.

## Primary endpoints

- Every native attempted instruction: `E_{after}=E_{before}-1` for total local+pool inventory. Every explicitly external packet: `E_{after}=E_{before}+packet`. No other sources or secret refills.
- Request4 under the first pool packet4 must **remain at PC1, unstopped, with zero credits** and produce **no additional internal events**; packet delivery is external only. Under later accumulated packets, it may resume once pool≥5.
- Request3 with first packet4 must execute a pool-paid Draw and complete at least one genuine PC4 `CellWritten(key9,value1)` before the next packet.
- Request2 must fail to complete any service during the experiment unless the actual VM unexpectedly finds a legitimate way to pay PC4; that would falsify the proposed per-cycle cost argument and must not be hidden.
- Require the frozen equal-input founder to retain one process and no post-start host `SetCredits`. Native Draw/Write events must originate internally, pool supply externally.
- Track *all* seven worlds, output totals per packet window, trapped/stopped status, blocked PC and unused pool. Do not interpret more paid attempts as stronger life. Retained externally supplied pool credits are still part of environment inventory.
- Do not promote B20 to an accepted native result until a source-SHA-pinned Rust CI pass is observed.

## Next experiment gate

The significant follow-on is to **remove this native evidence dead-end without adding a life-specific oracle**: e.g. generic CIL-visible failed-request status / conditional resource-state read, or a funded companion process that can sense task outcomes. Any changed substrate semantics require new CL-0 tests, equal-cost negative controls, and explicit recognition that the law is researcher-designed. Only after a continuing actor itself uses this feedback to revise a resource-request policy on **held-out pulse patterns** can a claim of native adaptive acquisition be entertained; immutable handpicked constants are not adaptation.
