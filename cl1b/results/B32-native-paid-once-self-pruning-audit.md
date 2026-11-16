# CL-1B B32 — charged one-time self-pruning restores compute access but loses to simpler static policy

**Native source-pinned verification:** [GitHub Actions full run 43807932500](https://github.com/iFrankStudio/cioc/actions/runs/43807932500), tested SHA `5390ddd2a2a6f6a1a99a95c1974b743dd92e1616`. **32/32 Rust CL-1B integration targets, the mandatory B31 private ABI/serde tests, and 19/19 independent source-parsed analyses PASS**; source-pinned archive publication succeeded. [Actual complete CI summary](https://github.com/iFrankStudio/cioc/blob/cl1b-ci-evidence/run-records/5390ddd2a2a6f6a1a99a95c1974b743dd92e1616-43807932500-1/summary.txt). [B32 preregistration](../studies/B32-paid-once-prune-to-draw.md) · [native CIL test](../../cl0/crates/world/tests/cl1b_b32_prune_metabolic_loop.rs) · [independent deterministic source-shaped 12-world simulator](../analysis/b32_paid_pruning_scan.mjs). All four neutral supply schedules and three policy arms were specified before native outcome inspection. The source-shaped interpreter predictions were fixed in the Rust test before CI verification.

## One continuing organism precursor, no offspring

The only host-loaded process has **2 starting local compute credits** and **no initial common-pool energy**. It enters a native `DrawCompute` after paying two initial instructions. After a first common-pool packet of at least10 credits, it pays for requesting9 units, executes a genuine service write, and spends additional computation making a **native `PatchNext` edit of its own real executable PC4 `Write.next` edge, from PC5 to PC1**.

The edge change removes five repeatedly executed maintenance opcodes from its service loop. Once patched, the individual runs a smaller internal work cycle `Draw3 → Const(key9) → Const(value1) → Write(cell9=1) → Draw3`. On zero local credits it is still stopped at **the pool-fundable Draw PC1**; future neutral energy packets can re-activate it. This is *real paid self-modification of existing executable control flow*, not a host shortcut or injected repair.

The matched **PATCH_SUPPRESSED** world starts with the *identical ten-opcode program* and charges the same PC8 attempted instruction but prevents real `PatchNext` effects via the generic VM testing control. This actor eventually runs out of local energy at **PC5, a non-Draw ordinary Const**, and cannot execute further even while neutral pool energy accumulates.

The simpler **STATIC3** control is a preauthored five-opcode fixed-request actor, with the same initial2 credits and the same environmental pool pulses but much shorter initial program and no adaptation overhead. It is a deliberate **total-cost efficiency baseline**, not a genome-matched one-opcode edit control.

## Four frozen environmental schedules, all twelve worlds included

Count only the **actual** `Origin::Internal(1)` `CellWritten(key9,value1)` at the service PC, not status claims.

| Exogenous neutral world-pool sequence | PRUNE real useful writes | PATCH_SUPPRESSED | STATIC3 cheap baseline | Residual pool: PRUNE / suppressed / STATIC3 |
| --- | ---: | ---: | ---: | --- |
| Development `[10,4,4,4]` | **4** | 2 | **5** | 0 / 8 / 2 |
| Held-out H1 `[10,3,3,3]` | **3** | 2 | **4** | 1 / 5 / 3 |
| Held-out H2 `[10,5,5,5]` | **4** | 2 | **6** | 3 / 11 / 1 |
| Held-out H3 `[9,4,4,4]` | **4** | 2 | **5** | 0 / 8 / 1 |

**Causal result:** PRUNE outperforms the source-identical PATCH_SUPPRESSED arm under all four neutral resource schedules, saving an otherwise stranded individual without additional host credits. It permanently rewires the real executable service loop, not just a proxy success marker. In the first schedule, the native source-pinned trace reports **4 real service writes vs2**, and suppressing the patch leaves **8** unused compute units in the world pool.

**Crucial negative result:** The cheapest five-node, investigator-authored STATIC3 algorithm produces **more actual work than PRUNE in every tested environment**. This is a genuine engineered improvement *relative to an artificially expensive source-identical control*, **not evidence that the actor discovered a globally efficient policy, learned its own maintenance overhead, or developed adaptive goals**. Repeatedly demonstrating repair of hand-built overhead would be circular; the next experiment needs meaningful evidence-driven candidate choice and amortized gains against a strong static baseline.

## Hard scientific boundary

This study crosses a useful **engineering mechanism** threshold for CL-1B: resource-accounted **in-place executable self-simplification** by one continuing native actor can prevent the B24 deadlock and sustain actual function across multiple externally provided neutral resource episodes, *without reproduction*.

It does not cross CL-1B's B3/B4 **endogenous adaptation** threshold. The researcher authored the initial expensive maintenance tail, the exact `PatchNext` repair target, the new request value3, every environmental packet and the original service. There is no internal evaluation selecting from alternatives, no acquired objective, no spontaneous origin of the CIL program, and no unrestricted long-term access to compute without fresh neutral supply. We must preserve the fixed-STATIC3 counterexample as a mandatory comparator.
