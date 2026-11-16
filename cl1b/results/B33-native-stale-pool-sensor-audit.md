# CL-1B B33 — verified truthful-but-stale CIL resource sensing under delayed common-pool flux

**Source-verified native result:** [GitHub Actions run 43810493380](https://github.com/iFrankStudio/cioc/actions/runs/43810493380), exact tested source SHA `325742d1b6b1ef8b9c4e3469fdbd71b1c9ccdf0c`. The fetched [machine-readable source-pinned summary](https://github.com/iFrankStudio/cioc/blob/cl1b-ci-evidence/run-records/325742d1b6b1ef8b9c4e3469fdbd71b1c9ccdf0c-43810493380-1/summary.txt) confirms **full scope: 33/33 Rust native integration targets PASS and 20/20 independent source-parsed analyses PASS**, zero failures, including mandatory B31 native pool-sensor ABI/serde tests. Evidence-publication job also completed. Original [B33 preregistered protocol](../studies/B33-sensor-freshness-post-observation.md) and [native Rust 24-world differential](../../cl0/crates/world/tests/cl1b_b33_sensor_freshness.rs), [independent source-parsed VM](../analysis/b33_sensor_freshness_scan.mjs).

## Source and question

B31's newly added, **normally paid** CIL `SenseComputePool` faithfully reports the shared compute pool at the instruction's execution. But can a single continuing actor rely on that value when the environment changes **after** sensing, yet **before** executing the resource acquisition it is supposed to guide?

B33 imports B31's **exact same 13-opcode founder source** and runs **24 deterministic worlds**: two one-process native arms (real paid `SenseComputePool` at PC0 versus equal-cost blind constant request3), four external resource packet quantities `0,4,5,6`, and three packet arrival points (`EARLY` before PC0, `AFTER_SENSE` after one native PC0 attempt, and `AFTER_COMMIT` after the eight real paid setup attempts and native `PatchConst` at PC7). Initial **eight local compute credits** are identical. No host code edits or rewards, no child actor, no free conditional sensor, no process-specific credits.

Every native CIL attempt consumes exactly one unit from local+shared compute; the only exogenous action is explicitly injecting one neutral common-pool packet. The positive function endpoint is **actual internal PC12 `CellWritten(key9,value1)`** rather than a mutable self-certified success marker.

## Native results

The informative falsification is the packet4 boundary:

| Packet 4 arrival | True SENSE read at PC0 | SENSE's real edited request at PC8 | SENSE genuine work | Equal-cost blind request3 work | Unused neutral pool for SENSE |
| --- | ---: | ---: | ---: | ---: | ---: |
| **EARLY** before sensor | **4** | **3** | **1** | 1 | 0 |
| **AFTER_SENSE** one CIL charge later | **0** (then packet4 arrives) | **4** | **0** | **1** | **4** |
| **AFTER_COMMIT** once all 8 charges paid | **0** (then packet4 arrives) | **4** | **0** | **1** | **4** |

All 24 worlds and four packet quantities are verified; for packet0, neither arm emits work. For packet5/6, SENSE uses request4 and produces one real work event regardless of packet timing, while blind request3 also produces one. Thus SENSE **never surpasses the strong static3 control**, and becomes strictly worse in both delayed packet4 cases. No post-selection of a favorable timing was performed.

The stale observation is **not a sensor lie**: the actual native sensor correctly reported `pool=0` at its execution. That reading became outdated because exogenous neutral pool energy arrived afterward. The actor's fixed CIL branch chose request4 (which needs5 available pool compute:4 transfer +1 charged Draw instruction), so its pending pool-funded Draw at PC9 cannot run on packet4. Once all local credits have been spent, the actor **cannot execute `SenseComputePool` again or revise code from PC9**; no rescue is automatically initiated. The static3 actor can run from PC9 on packet4 and emits the service event.

## Causal limits and next science gate

This is a verified **information-freshness counterexample**, not a success at endogenous adaptation. B31 established a generic environmental observation channel; B33 demonstrates that truthful observation before code mutation cannot supply information about resource changes after the observation. The gap cannot be solved by an observer reading the event logs for the actor or a host magically patching policy at zero credit. A credible continuously adapting CIL individual will need an **executable, resource-accounted recurring observe/evaluate/act loop with sufficient local credit reserves**, at least where the environment can change between paid actions.

Future test designs must preserve: strong cheap immutable baseline, true native function writes, per-step compute conservation, no privileged fitness/survival opcode, hidden/nonanticipatable environment supply, and a proof that the actor's own runtime evidence caused candidate choice and retention rather than a researcher-authored fixed threshold. **CL-1B B3/B4 remains NOT MET; no verified cyberlife breakthrough**.
