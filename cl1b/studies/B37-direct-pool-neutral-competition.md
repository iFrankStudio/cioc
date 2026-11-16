# CL-1B B37 — shared compute fairness and productive-work dilution under generic pool-paid execution

**Prospectively registered 2026-11-16, before B37 native source or measured outputs.** This is an **ecological substrate stress test of the B36 experimental world law**, NOT a change of the CL-1B single-individual research priority and NOT a claim that reproduction or communities are necessary for cyberlife.

## Causal concern

B36's generic, opt-in `with_pool_paid_all_instructions()` solves the unnatural zero-credit non-Draw gate: any CIL instruction can consume one neutral world-pool compute unit. A new fairness question follows: the scheduler may fund **unproductive loops** instead of functionally useful programs. The environment itself has no built-in objective or privileged "good organism" category, and should not cheat by favoring service events. This could halve an individual actor's useful work without that actor changing its source.

## Frozen authored programs

All processes are hand-authored, loaded externally at the beginning with **zero local credits** and no code edits or replication. No `SetCredits` targeted at any actor. The only external compute enters the **shared world pool** through fixed neutral packets. World strict capacity16 cells,64 nodes/process,128 occupied units, with `with_pool_paid_all_instructions()` enabled; the old default law remains unchanged and is a separate negative control.

- **WORKER**, 3-node exact native CIL service loop: `Const r2=9 → Const r3=1 → Write(key9,value1) → PC0`. Every **three charged native instructions** produce exactly **one true `CellWritten`**. Only this native event counts as productive work.
- **BURNER**, 2-node CIL loop: `Const r0=7 → Branch(test=r0,nonzero0,zero0)`. Every executed instruction costs one normal unit but the program makes **zero service writes**. This is an adversarially simple control, not a claim that real individuals must behave wastefully.

Four fixed actor compositions under every packet schedule:
1. **SOLO_WORKER**: PID1 is WORKER.
2. **BURNER_FIRST**: PID1 BURNER, PID2 WORKER.
3. **WORKER_FIRST**: PID1 WORKER, PID2 BURNER (source identity swap).
4. **DUAL_WORKER**: PID1 and PID2 WORKER (same total compute demand but *both* useful).

Three preregistered neutral packet schedules, each total **18 compute credits**: `[3,6,9]`, `[6,6,6]`, and `[1,2,3,4,5,3]`. The generic world scheduler runs to exhaustion after each packet. **12 direct-pool native worlds**, plus **two separate DRAW_ONLY worlds** (BURNER_FIRST and SOLO_WORKER with 18 common pool credits, zero initial local credit), testing that the original substrate does not run any non-Draw CIL instruction at zero local. **Total: 14 worlds**.

## Falsifiable primary endpoints

- Charge every emitted CIL step exactly one local or shared compute unit; all 12 all-pool worlds must execute **exactly18 native attempts**, with zero local/shared compute remaining, and all attempts marked `pool_paid_attempt=true`. No actor-specific host funding.
- Native `Origin::Internal(workerPID)` `CellWritten(key9,value1)` is the only positive output. A running BURNER must produce zero. SOLO_WORKER and DUAL_WORKER should provide an upper bound on useful work given this hand-authored program. BURNER_FIRST and WORKER_FIRST must keep the same **program-level** work despite process ID swap if the deterministic scheduler is symmetric over this 18-step budget; if not, report an explicit scheduling-confound counterexample rather than massaging counts.
- Source-parsed independent deterministic interpreter should confirm the 14 world matrix, true per-process instruction work distribution and actual write counts. Do not claim global fitness or statistically inferred ecology from three deterministic packet arrangements.
- DRAW_ONLY controls must have zero executed instructions and retain all shared pool credits when process local balances are zero and programs start at non-Draw PC0.

## Interpretation

If BURNER loses no energy and never gets scheduled, the alternative law implicitly favors useful actors and risks a hidden oracle; if BURNER consumes compute and reduces WORKER output, this is **ordinary resource competition** under a *neutral but non-utility-aware* global scheduler. It shows direct pool-paid execution removes artificial starvation yet raises unproductive compute competition as an ecological tradeoff.

Do **not** solve this by hand-granting more resources to the worker or imposing an explicit externally authored output/fitness preference. This study does not create adaptive organisms or new mutation operators. The central CL-1B goal remains genuine **paid and functional evidence-conditioned self-revision** by a continuing individual, with strong compact immutable controls and no host-selected winner.
