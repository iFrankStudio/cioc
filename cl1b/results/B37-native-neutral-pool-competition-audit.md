# CL-1B B37 — neutral global compute can be consumed by unproductive CIL programs

**Verified full native result:** [GitHub Actions run 43819923039](https://github.com/iFrankStudio/cioc/actions/runs/43819923039), exact tested source SHA `e6af4733eaf1fb597510e781b4697d40aebf5d01`. [Fetched source-pinned original CI summary](https://github.com/iFrankStudio/cioc/blob/cl1b-ci-evidence/run-records/e6af4733eaf1fb597510e781b4697d40aebf5d01-43819923039-1/summary.txt): **37/37 native Rust integration targets PASS, 24/24 independent Rust-source-parsed observers PASS**, B31/B34 private ABI checks PASS, zero failures, separate evidence publication PASS. [Prospective registration](../studies/B37-direct-pool-neutral-competition.md), [native two-actor CIL verification](../../cl0/crates/world/tests/cl1b_b37_direct_pool_contention.rs), [independent 14-world interpreter](../analysis/b37_neutral_direct_pool_contention_scan.mjs).

## Why this matters for native cyberlife substrate design

B36 introduced opt-in `with_pool_paid_all_instructions()` world physics: any native CIL instruction, when the actor has zero local compute, can be funded from the *shared* neutral compute pool for its actual 1-unit cost. It cured an artificial `DrawCompute`-only scheduler deadlock but made every executable actor—useful or not—able to consume communal energy. B37 tests the cost of this generality without inventing a built-in `FITNESS` or privileged priority category.

Two fully authored native programs start with **zero local compute** and are identical across all their assigned roles:
- **WORKER**, 3 CIL nodes: `Const(key9) → Const(value1) → Write(cell9=1)`. Exactly three charged native steps produce one genuinely attributed service.
- **BURNER**, 2 CIL nodes: `Const(r0=7) → Branch(back to Const)`. This source spends real compute without ever writing a service event.

All world energy is injected externally into the same shared pool in one of three frozen schedules, **each total18 units**: `[3,6,9]`, `[6,6,6]`, `[1,2,3,4,5,3]`. The scheduler drains after each packet. No actor-specific `SetCredits`, code edits, replication, host repair or award. Four fixed compositions per schedule (solo WORKER, BURNER-first+WORKER-second, WORKER-first+BURNER-second, two WORKERs), plus two zero-local DRAW_ONLY negative-control worlds = **14 complete native worlds**.

## Full native findings

| Native program composition | Total native paid attempts | Genuine service writes | WORKER charged steps | BURNER charged steps |
| --- | ---: | ---: | ---: | ---: |
| SOLO_WORKER | 18 | **6** | 18 | 0 |
| BURNER_FIRST + WORKER | 18 | **3** | 9 | 9 |
| WORKER_FIRST + BURNER | 18 | **3** | 9 | 9 |
| DUAL_WORKER | 18 | **6** | 18 | 0 |

**Each row repeats with the exact same totals in all three distinct packet schedules.** Every actor is zero-local throughout, and every native instruction under the new energy law has `pool_paid_attempt=true`. All18 units are actually charged: `paid_steps + remaining local + remaining pool = 18`, with zero pool remaining. BURNER never emits `CellWritten`, while WORKER emits the native `Origin::Internal(workerPID)` `CellWritten(key9,value1)` only after completing its three-opcode cycle.

**Neutral scheduling is not useful-work optimization.** It gives both executable processes regular access to available compute; an unproductive BURNER consumes nine of eighteen common credits, cutting verified service outputs **in half (6→3)**. Swapping program-to-PID assignment preserves the difference, so it is not a privileged first-ID allocation effect in these deterministic schedules. With two useful WORKER actors, the same eighteen charges produce six service outputs overall.

Two separate **draw-only** physics controls start with the same zero-local non-Draw native source programs and a common pool of18. Neither program can execute a single instruction; all18 credits remain unused. This demonstrates the real substrate tradeoff: the old gateway protects communal compute from arbitrary zero-local execution but can permanently strand a potentially useful actor; the new law lets either useful or wasteful code run and compete.

## What this does not establish

This is an **authored synthetic resource-competition assay**; BURNER is deliberately written to consume energy without useful output. It does not demonstrate spontaneous ecological competition, evolved selfishness, learned cooperation, resource ownership, fair resource scheduling in diverse populations, stable ecosystems, or the existence of a cyberlife organism.

The result shows that choosing **world energy physics** determines which endogenous behaviors are even possible and at what opportunity cost. An arbitrary rule privileging observed service outputs would simply inject a researcher-defined utility criterion into the world and risk turning emergence into simulation. Nor may the researcher secretly award resource grants to the WORKER or call that adaptation.

## Research decision

Preserve both generic energy laws as explicit, opt-in alternatives. Their physical consequences are now experimentally distinguished:

1. **DRAW_ONLY:** shared pool credit is paid directly only at `DrawCompute`, exposing B35's false-death states despite nonzero communal compute.
2. **ALL_NATIVE:** any native opcode may pay its actual compute cost from the pool, reducing artificial deadlock but enabling unproductive consumption and competition, without fitness filtering.

Neither option yet supplies the missing **endogenous task evidence, source mutation selection, rollback or adaptive goal formation** required by the main CL-1B continuous-individual track. The next meaningful direction is to allow resource-funded experimentation under a transparent neutral accounting law, but judge the *same continuing actor* against immutable low-overhead programs using genuine functional endpoints. B3/B4 and open-ended cyberlife **remain unconfirmed**.
