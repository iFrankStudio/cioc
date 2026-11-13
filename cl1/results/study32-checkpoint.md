# CL-1 Study 32 — prospective native CIL compute-pool uptake and resource recycling (2026-11-14)

**Scientific resource-primitive exposure verified. Long-window endogenous energy self-sufficiency NOT OBSERVED. Autonomous functional maintenance NOT EVALUABLE. CL-1 NOT ACCEPTED.**

## Preregistration, source and evidence

- [Frozen Study32 prospective protocol](../studies/32-preregistered-native-compute-pool-uptake.md), commit `ed0967956b5ef308403c2ccc0d3cdb79d515280c`, registered before the new seed code, runner and outcome inspection.
- Validated source revision `4de431a5bbc4e731b1ba6f90cb4760216d892651`. [Targeted GitHub Actions Study32 job](https://github.com/iFrankStudio/cioc/actions/runs/43464729261) **PASSED**: all Rust workspace unit tests/strict lint, fixed 64-bit LCG generator assertions, 1,152 Rust exact SQLite epoch replays, independently reconstructed compute-pool ledger for every one of 27,648 postepoch states and Python read-only outcome audit. Raw SQLite and report are retained in the workflow artifact for 30 days.
- **192 genuinely new independent seed-worlds** `2,500,000..2,500,191`, with 6 matched conditions, **1,152 paired runs** and **27,648 hash-verified epoch snapshots**; all worlds including trapped/stopped worlds remain in the denominator. Matched arms are not independent samples.
- The environment's initial **48 compute units are externally endowed** via a recorded `Intervention::SetComputePool` before the common prefork. The experimental first nine CIL nodes per process, stable amount register1 and random instruction grammar were authored by the researcher. Native `DrawCompute` and `ReturnCompute` are pre-existing general CIL capabilities and do not contain a built-in energy objective or native repair mechanism.

## Exact native resource readouts

Counts are independent seed-worlds out of 192 with at least one corresponding **successful native CIL opcode**, not calls, bytes, or matched branches.

| Registered endpoint | W1 (0–7) | W2 (8–15) | W3 (16–23) |
| --- | ---: | ---: | ---: |
| Pool=48, native successful `DrawCompute` | **154** | **54** | **0** |
| Pool=48, native successful `ReturnCompute` | **155** | **8** | **0** |
| Pool=48, any charged native CIL activity | **192** | **79** | **0** |
| Initially empty pool, native successful `DrawCompute` | **62** | — | **0** |
| Initially empty pool, any charged native CIL activity | — | — | **0** |
| Exogenous SetCredits at epochs 8 and 16, W3 native activity | — | — | **125** |

**86/192** independently sampled worlds had at least one process successfully invoke a native pool draw and execute **more than its original 47 postprefork charged instruction attempts** without any postfork experimenter process-credit refill. In the initial **empty-pool** condition, **8/192** worlds achieved the same narrow extension. The latter condition still permits `ReturnCompute` by programs that begin with process credits, so internally returning compute to a shared pool and later drawing is real *redistribution*, not new compute creation. The exact external-pool-available arm vs emptied-pool arm had **92 refill-pool-only successful W1 Draw worlds** and **zero emptied-only** worlds. These are *paired same-seed* discordances, not 92 new independent worlds.

For the W1 control native Draw proportion **154/192**, Wilson 95% interval **[0.740043, 0.852273]**; for empty-pool native Draw **62/192**, interval **[0.260810, 0.391970]**. For W1 prolonged internal uptake **86/192**, interval **[0.379285, 0.518592]**. Reported intervals are descriptive only for the frozen specific instruction generator and selected environment.

## Resource provenance and limitations

The independent auditor confirmed each source opcode in process code (not just `Executed` event counts), actual actor ID/PC/success and quantity from its fixed native `register1` value (process1: 2, process2: 3, process3: 1). It replayed successful pool debit/return values and all **24 × 1,152** epoch snapshots; failed/trapped attempts did not mint compute. The zero-credit control never executed native CIL; rejected sham left simulated world state unchanged. External replenishment was separately labeled and did not restart halted/trapped programs.

This is an experimentally verified **resource-use mechanism in some random continuations**, but not strong evidence of autonomous, sustainable metabolism: the world pool is investigator seeded, preexisting CIL code was investigator authored, pool operations have no learned role or survival target, the supply is finite and no untreated world executed in W3. Successful native resource acquisition and process-execution extension are **not equivalent** to self-maintained code-dependent organizational closure, self-repair, free-standing resource production or open-ended evolution.

Future acceptance requires causally linked **endogenous multi-component native functional output**, loss after lesion, internally performed repair with source-provenance, separate post-lesion late windows, matched sham/zero-energy/no-repair/external-functional-positive controls and independent fresh seeds. Current result passes the narrow energy-operation test only.

**CL-1 remains OPEN.**
