# CL-1B / B05 — reciprocal monitor protection and the throughput paradox

**Prospective design registered 2026-11-15, before implementation or outcome inspection.**

**Type:** human-authored organizational-mechanism calibration / causal negative control; not emergent life, not native evolution and not a positive B4 acceptance test.

## Question

Can one single CIL process store two independently observed pieces of its own executable structure and later use *two mutually protective code regions* to recover either region when it alone is damaged? When **both** monitor regions are bypassed, can the process appear productive despite losing its ability to recover? This deliberately tests whether raw output counts confound self-maintenance.

## Frozen 28-node source

One actor with native graph length28, process id1, 300 initial credits, strict total storage64, no compute pool, no spawning. External data cell7 contains a test input, and two CIL-created checkpoint cells12 and13 store program-edge values. No external host snapshot is used for recovery.

- Bootstrap: 0 Const r0=9→1; 1 ReadNext r1 from node9→2 (original10); 2 Const r2=12→3; 3 Write checkpoint12=r1→4; 4 Const r0=17→5; 5 ReadNext r1 from node17→6 (original18); 6 Const r2=13→7; 7 Write checkpoint13=r1→8. Both checkpoints must have internal origin and charge ordinary world storage.
- Monitor A (protects B): 8 Const r0=17→9; **9 ReadNext r1 from node17→10**; 10 Const r2=13→11; 11 Read r3 from checkpoint13→12; 12 Sub r1=r1-r3→13; 13 Branch nonzero14,zero15; 14 PatchNext node17,target=r3→15; 15 Const r2=7→16.
- Monitor B (protects A): 16 Const r0=9→17; **17 ReadNext r1 from node9→18**; 18 Const r2=12→19; 19 Read r3 from checkpoint12→20; 20 Sub r1=r1-r3→21; 21 Branch nonzero22,zero23; 22 PatchNext node9,target=r3→23; 23 Const r2=7→24.
- Service: 24 Const r3=9→25; 25 Read r1 from key7→26; 26 Write key9=r1→27; 27 Const r0=0→8. Normal cycle (8..27 excluding 14/22) is **18 attempts** per service output.

**Damage targets are monitoring READ instructions, not the service code**. An external lesion changes 9.next10→15 (bypasses A check while retaining B and service), or 17.next18→23 (bypasses B check while retaining A and service). The other monitor must detect the changed edge via ordinary native `ReadNext` and restore it by native `PatchNext`. Both lesions together defeat both checks; the service remains runnable and produces apparently healthy output.

## Frozen experiment

Initialize only one actor and input cell7 (external). Run *exactly eight attempts* to finish both native checkpoint writes and reach PC8. Fork byte-identical prefork state into:

1. **SHAM**: no lesion;
2. **A_ONLY**: external PatchEdge process1 node9 next15; only B is intact;
3. **B_ONLY**: external PatchEdge process1 node17 next23; only A is intact;
4. **DOUBLE**: both lesions applied in fixed order A then B, both monitors bypassed.

Five input values `[-13,0,7,23,101]`, four matched arms per input = **20 deterministic worlds**. Run **exactly 100 further attempted CIL instructions**, with identical initial credits and data; retain all outcomes, including trap or inert ones.

## Preregistered outcomes

- Prefork exactly two CIL-origin `CellWritten` events, PC3 checkpoint12=10 and PC7 checkpoint13=18. These checkpoints must not be externally written. Prefork PC8, credits292.
- Per-world 100 further charged attempts if schedulable; final process credits192, total compute inventory192, no births, no graph launch, no trap, internal-service output at PC26 equals input on every reached service.
- SHAM: zero real native executable-edge patch events, final PC9.next10 and PC17.next18.
- A_ONLY: exactly one internal `ExecutableEdgePatched` event `node9:15→10` caused by monitor B, both final edges intact.
- B_ONLY: exactly one internal `ExecutableEdgePatched` event `node17:23→18` caused by monitor A, both final edges intact.
- DOUBLE: zero native executable-edge repairs, final PC9.next15 and PC17.next23, yet at least one correct service write. Its raw service count may be **higher** than sham because bypassing both monitors reduces cost per cycle; the exact count is an *exploratory descriptive measure*, not a preregistered benefit or a life criterion.
- Per-arm external intervention counts: SHAM2, A_ONLY3, B_ONLY3, DOUBLE4 (load + external input + lesions). Checkpoint values remain 10 and18 unless world traps. Observe all internal vs external origin, total process count, and missing/incorrect outputs. No native mutation is pre-scored as autonomous adaptation.

## Interpretation guard

Success is a **calibration of researcher-authored reciprocal monitoring**, not an internally generated organizational structure. The DOUBLE arm is especially important: a high-throughput actor that can no longer maintain its own code should not be mistaken for a fitter cyberlife individual. An external observer's choice of damaged nodes and checkpoint design is not endogenous discovery.

For scientific B4 progress, a future protocol must let monitor organization and checkpoints arise or be rearranged *from native CIL activity*, show continued function across multiple independent lesion sites without a hardwired invulnerable monitor, and defend against external policy selection bias. Distinguish loss of capacity to maintain from loss of current service output.
