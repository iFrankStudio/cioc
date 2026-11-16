# CL-1B B35 — repeated paid resource trials, inherited candidate decrement, and Draw-reachability traps

**Preregistered 2026-11-16 before B35 test implementation/outcomes.** Primary route: **a single continuing individual**. B34 proved actual charged resource transfer success/failure can be seen in a CIL register but only used an authored one-time fallback. B35 tests a more demanding mechanism: an individual computes the next resource-request candidate by **subtracting an ordinary literal 1**, patches its own executable PC0 request, retries within the same episode and later encounters new resource packets. The decrement strategy and goal remain authored by investigators: this is **not autonomous invention, fitness learning or cyberlife**.

## Frozen 12-node CIL founder

A single host-loaded PID1, **14 local credits**, initial neutral world-pool0; normal `with_pool_paid_draw`, `with_recoverable_draw_shortage`, strict storage16 cells /64 nodes per process/128 combined units. No new VM opcodes, no other process, no host SetCredits or repairs.

| PC | Native CIL | Next |
| ---: | --- | ---: |
| 0 | `Const r0=5` initial candidate resource request | 1 |
| 1 | `Const r1=0` target own executable candidate literal | 2 |
| 2 | `Const r3=1` arithmetic decrement constant | 3 |
| 3 | `TryDrawCompute(amount=r0,dst=r2)` **actual paid trial** | 4 |
| 4 | `Branch test=r2 nonzero→8 zero→5` | — |
| 5 | `Sub r0=r0-r3` **compute lower candidate internally** | 6 |
| 6 | `PatchConst(node=r1,value=r0)` permanently change source PC0 | 7 |
| 7 | `Const r2=0` reset trial flag | 3 |
| 8 | `Const r1=9` service cell address | 9 |
| 9 | `Const r3=1` service value | 10 |
| 10 | `Write(key9,value1)` real function | 11 |
| 11 | `DrawCompute(amount=r0)` **ordinary pool-payable gateway** | 0 |

No direct investigator-coded `Const r0=3` fallback. The first failed `TryDraw5` derives candidate4 from subtraction, and repeated failures can derive3,2,... at real instruction/patch cost until the local compute balance is exhausted. Request state persists as the **actual native source PC0 literal**, not a metadata label; each new episode runs that code again.

## Controls and frozen external schedules

Three arms:
- **ADAPT:** genuine native charged TryDraw result in r2; the decrement/patch/retrial loop reacts to actual transfer failure.
- **FORCED_FAIL:** same 12-node source and charge, but replace PC3 by a `Const r2=0` (no actual trial). Tests whether real feedback is causally needed and exposes the possibility of unpayable endless self-decrement/low-request pathology. This arm must not be called an otherwise identical native acquisition attempt; it is a same-PC-charge negative control.
- **STATIC3:** compact five-node fixed request3 Draw/true-service loop, same 14 initial local credits, same neutral packet sequence. Strong total-energy efficiency baseline.

Four schedules fixed prospectively, initial packet before executing the first native instruction, subsequent packets only **after the native scheduler becomes ineligible**:
- DEV_SPARSE: `[4,5,5]` (first request5 fails, can derive lower candidate; later shortage may strand actor)
- DEV_RICH: `[4,12,12]` (same first experience, greater opportunity to continue)
- HOLDOUT_EQUAL: `[5,5,5]` (initial candidate fits, but later resource access can still fail)
- HOLDOUT_SHIFT: `[6,8,12]` (mixed supply with early capacity and later changed opportunity)

**12 native worlds.** Include failures, traps, stopped and stranded programs in the denominator; no host reset between packet windows. A single actor cannot be considered adapted if it repeatedly self-edits but cannot service real work or becomes stranded at an ordinary non-Draw PC.

## Registered outcome measurements

For each native window and arm: actual `ComputeDrawTried` requested/available/success sequence, actual `Origin::Internal(1)` `CellWritten(key9,value1)` counts, PC0 native executable literal, every paid PatchConst at PC6, spent steps, residual pool/local credits, PC and stop/trap status. **All** attempted native instructions decrease total local+pool compute by1; only external neutral packets increase it. Call any post-pulse process stuck at non-Draw/zero local a deadlock, and report even if the pool still contains enough energy for a different request. Pay special attention to `DrawCompute` at PC11, which can fail/defer without producing a CIL-visible success bit.

The independent interpreter must parse the actual Rust founder source and verify each of the 12 worlds; do not assume abstract idealized trials count. Any numerically frozen prediction added before native CI is separately auditable. The *scientific* null is not that CIL cannot patch its own literal (already proved), but that **even a true resource-feedback-driven candidate search may not yield sustained functional improvement once compute overhead and Draw eligibility are fully charged**. Against STATIC3, code size and paid exploration costs matter.

## Acceptance boundary

Positive outcome would establish that one CIL actor can perform **multiple actual paid candidate attempts**, compute and store executable changes, and perhaps keep serving under later neutral energy arrivals. It cannot on its own demonstrate selection of an internally discovered functional objective, rollback from real task-quality evidence, generalization or a cyberlife organism. If a complicated adaptation loop consistently loses to STATIC3, record it as negative evidence and prefer designing a better *information/credit-control* experiment rather than multiplying hand-authored mutations.
