# CL-1B B36 — generic pool-paid execution removes artificial non-Draw starvation, not poor evolutionary policy

**Full verified native verdict:** [GitHub Actions run 43819086629](https://github.com/iFrankStudio/cioc/actions/runs/43819086629), exact tested SHA `6ae4b13488f98a63e2ad6e343be0035320795eb0`. [Original source-pinned evidence summary](https://github.com/iFrankStudio/cioc/blob/cl1b-ci-evidence/run-records/6ae4b13488f98a63e2ad6e343be0035320795eb0-43819086629-1/summary.txt): **full scope, 36/36 native Rust CL-1B integration targets PASS, 23/23 independent CIL source-parsed observers PASS, B31/B34 mandatory ABI compatibility checks PASS**, zero native/observer failures and separate evidence-publication job successful. [Prospective registered test protocol](../studies/B36-direct-pool-compute-physics.md), [real native Rust energy-law comparative integration](../../cl0/crates/world/tests/cl1b_b36_neutral_direct_pool_execution.rs), [independent source-parsed B35 interpreter with B36 physical-law toggle](../analysis/b36_direct_pool_execution_scan.mjs), [new optional generic VM law in Rust](../../cl0/crates/world/src/lib.rs).

## Substrate-level causality, not agent repair

B35's individual ran out of **local** compute at ordinary branch/sub instructions even though nonzero *shared* compute existed. The existing CL-0 opt-in `with_pool_paid_draw()` law permitted only `DrawCompute` to pay its next instruction directly from the pool. That created a **structural absorbing deadlock** for any process at zero local credits and a non-Draw PC, regardless of whether the compute pool later accumulated more energy.

B36 introduces an alternative, opt-in, **identity-neutral general compute law**: `World::with_pool_paid_all_instructions()`. If a process has no local compute but the shared world pool has at least1 credit, *any* normal CIL instruction is eligible for a **real one-credit pool-paid execution**. The same old native opcode still runs with all of its normal operand, trap, storage and data permissions; the global compute inventory decreases **exactly1** per attempted instruction. No individual gets privileged `SetCredits`, no externally selected repair graph or fitness reward is supplied, and stopped/trapped processes remain stopped. A recoverable `DrawCompute` still requires the **full native request + instruction fee** to execute from zero local credits. When the neutral pool itself is empty, the law cannot execute any zero-local operation.

The new boolean world law defaults **false** and uses omission in the old serde encoding, so legacy world snapshots remain unmodified. A native serde control serializes the explicit enabled law, round-trips it, and verifies an old world round-trips with the law disabled. The existing draw-only default and its exact B35 native service totals remain unchanged.

## Complete 16-world preregistered comparison

The experiment runs the **exact native 12-opcode B35 repeated resource candidate search**, one initially loaded PID1 with14 credits, **and the exact native five-opcode STATIC3 comparison**. Four previously selected schedules; two program sources × two general energy laws = **16 deterministic worlds**. The Rust B36 test imported and executed the original B35 Rust founder source (rather than copying a new candidate), while its independent observer reused B35's restricted CIL VM with only the generic scheduler-energy toggle changed.

| Neutral common-pool packet schedule | B35 ADAPT Draw-only | B36 ADAPT all-native pool paid | STATIC3 under either physics | B36 true pool-paid non-Draw ADAPT attempts |
| --- | ---: | ---: | ---: | ---: |
| `[4,5,5]` | 1 | **1** | **3** | 5 |
| `[4,12,12]` | 3 | **3** | **7** | 1 |
| `[5,5,5]` | 1 | **1** | **4** | 4 |
| `[6,8,12]` | 1 | **2** | **7** | 6 |

Every service number counts only actual native `Origin::Internal(1)` `CellWritten(key9,value1)` at the real CIL service PC; all 16 worlds preserve `attempted instruction charges + remaining local + remaining neutral pool = 14 + sum(external packets)` exactly. **All four B36 ADAPT alternative-world runs finished their observation period with zero neutral pool credits**: unlike B35, compute stranded at PC4/PC5 is now consumed by real native CIL continuation. Real native event flags `pool_paid_attempt=true` on **non-Draw** instructions confirmed the alternative physics actually ran them, rather than a host refilling individual credits.

**Improvement is modest:** Under `[6,8,12]` the altered energy law permitted an additional true service (2 vs1) and further native source adjustment. Under the other three schedules it merely permitted more charged instructions or edits without increasing true service. **STATIC3 remains strictly superior on all schedules**, even with direct pool-funded native execution. Allowing CIL computation to draw energy directly does not invent a good self-modification policy and may divert scarce world energy into unproductive attempts.

## Important scientific and engineering boundaries

- This is an **experimental world-law comparison**, not a discovered organism trait. The alternative scheduling affordance was explicitly installed by the investigator and applies to all native programs; its effect cannot be credited to the actor's self-directed behavior.
- Directly allowing every process to spend the common pool removes a narrow *Draw-only* source of apparent death, but creates a future **multi-actor contention/starvation question**: a long-running low-value actor might spend neutral energy before other actors can access it. The current single-actor results do **not** address multi-actor fairness, ecological balance, rate limits or open-ended evolution.
- The paid TryDraw failure signal is real, but the B35 actor's mutation operator `Q←Q−1`, original code, task and source patch address were all written by researchers. A higher count of paid instructions, patches or even one additional output does not establish adaptation across independent environments.
- The same system still cannot infer **true service quality** from successful resource acquisition; it only receives success/failure for requested compute and has no autonomous model of its goals.

**Disposition:** Preserve **both** generic physical regimes as separately controlled, explicitly named experiment laws. Do not silently make the global pool-paid-any-instruction rule the baseline default. Use the new law to prevent mistaking a scheduler artifact for an organism-level deficiency, while continuing to demand real functional evidence and stronger immutable baselines. **CL-1B B3/B4, self-originating adaptation and cyberlife remain NOT MET**.

**Next discriminator:** under the new law, compare **multi-actor general compute contention** for separate authored service functions and order swaps, checking whether direct pool-paid execution can let one over-consuming actor starve the other. This is a *world fairness* question, distinct from autonomous adaptation. In parallel, seek a nonprivileged CIL-readable *real* service-quality signal or a verifiable task-outcome test that cannot be forged by simply writing a marker.
