# CL-1 Study36 — CI-verified compute ignition deadlock and resource accessibility bottleneck

**Status:** retrospective resource-accounting theorem and historical-state classification, NOT prospective life evidence. **New worlds: 0.** Reuses the already executed Study35 192 worlds, nine arms, 1,728 runs and 41,472 epoch SHA256 snapshots.

## Frozen evidence and verification

- Registration: `cl1/studies/36-registered-retrospective-compute-access-deadlock.md`, prior to the Study36 auditor.
- Independent CI: [43547650082](https://github.com/iFrankStudio/cioc/actions/runs/43547650082), archived `cl1-study36-retrospective-deadlock` report.
- Historical source revision `9ee57f2536b8cd4eed2a84817f2e7e777d9bcc00`; source artifact is the **unaltered CI-passed Study35** SQLite. Study36 downloaded the original artifact and **did not generate new ecology**.
- All 41,472 epoch snapshot SHA256 hashes, source-pinned run labels, 24-epoch native attempt ledgers, no-external-refill inventory conservation, and sham/exact-original native state invariance passed.

## Mechanistic finding — persistent compute exists, but no agent can ignite it

In every untreated control world, prefork total compute inventory is exactly **189 credits** (3 processes × 47, plus a finite researcher-endowed pool of 48). Under native CIL, every attempted step costs 1 credit. `DrawCompute`, `ReturnCompute`, `TransferCompute`, and `GraphLaunch` only move credits between process balances and the pool or another process; they do not create them. In the no-refill controls, all **192/192 worlds** obey the exact identity

`compute_inventory_after = 189 − charged_native_attempts`.

At the start of W3 (after epoch15), **0/192 control worlds** contain an eligible process (`!stopped && credits > 0`), so no native attempts occur in W3. Yet **192/192 retain positive total compute inventory**, with final **median 102.5 credits**, range **19..179**, and world-pool median 47 (range 0..92).

| Mutually exclusive control condition at W3 entry | Worlds |
|---|---:|
| World compute pool positive, some process unstopped but every active process has zero credits | **138 / 192** |
| World compute pool positive, all processes stopped | **33 / 192** |
| World pool zero, some process unstopped but all active processes have zero credits | **12 / 192** |
| World pool zero, all processes stopped | **9 / 192** |
| At least one runnable process | **0 / 192** |

The **138/192** category is an intrinsic **ignition deadlock**: the process needs positive compute credit to execute `DrawCompute`, even if the pool already contains available compute, because the scheduler does not select zero-credit actors. The **33/192** category has a world pool but all processes halted/trapped. Deadlock with positive inventory is different from complete resource exhaustion.

## External intervention contrast

The registered `refill` arm externally sets credits at epochs 8 and 16; it has **139/192 worlds** with W3 native activity. The matching no-refill `control` has **0/192**. That contrast establishes that a host credit injection can restore execution; it is emphatically **not** an endogenous resource-management or autonomous repair result.

## Theorem and caveats

Within any closed, finite-credit world governed by the present Rust kernel, with no external credit changes, the total number of executed native attempts is bounded by initial total compute inventory. Therefore **open-ended indefinite computation is structurally impossible** in this exact closed-resource model. This does **not** assert that all worlds must stop by epoch16 solely because of the 189-credit bound: the *observed* 0/192 W3 activity has the additional, directly verified scheduling-access cause.

A subsequent post-result **exploratory diagnostic** of W3-disabled control processes with positive pool found that 16 of the 138 ignition-deadlocked worlds have an unstopped process whose current next opcode is `DrawCompute`. That count is not an outcome of the registered Study36 primary audit and must not be treated as prospective evidence. It provides a falsifiable premise for a separately preregistered environmental-compute-access experiment.

## Scientific implication

The next useful research question is **not** simply how to increase world compute pool. A pool may remain >0 but be inaccessible to zero-credit processes. A generic, explicitly registered native resource-access mechanism or resource-harvesting policy is needed to test autonomous compute acquisition without a researcher selecting and recharging particular processes. Such a mechanism must conserve total compute inventory and must not hardcode repair, fitness or reproduction.

**CL-1 remains OPEN.** This identifies a mechanical ceiling and a resource topology deadlock, not a cyberlife organism.
