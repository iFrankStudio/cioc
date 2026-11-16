# CL-1B B31 — native paid neutral-pool sensing and conditional executable mutation

**Source-pinned native verdict:** [GitHub Actions full run 43807201899](https://github.com/iFrankStudio/cioc/actions/runs/43807201899), SHA `0318e1c0a37cacc436e80b1329bde41e4420b285`, **31/31 native Rust tests PASS and 18/18 independent source-parsed observers PASS**; evidence reporting job succeeded. [Exact full source and test summary](https://github.com/iFrankStudio/cioc/blob/cl1b-ci-evidence/run-records/0318e1c0a37cacc436e80b1329bde41e4420b285-43807201899-1/summary.txt). [Prospective B31 protocol and its pre-test startup-budget correction](../studies/B31-paid-world-pool-sensing.md), [native CIL sensor integration](../../cl0/crates/world/tests/cl1b_b31_pool_sensing.rs), [independent 12-world interpreter](../analysis/b31_paid_pool_sensing_scan.mjs).

## Generic new world affordance (not a fitness primitive)

The Rust CL-0 CIL VM now supports `SenseComputePool {dst,next}`, numeric opcode tag **31**, appended without renumbering the earlier tags0–30. The native instruction costs exactly **one ordinary local compute credit** and writes the currently available shared world-pool amount into a register with checked i64 conversion. It emits a `ComputePoolSensed {available}` event for the external scientific observer. No host success signal, fitness score, code repair, new compute credits, or purpose-specific survival functionality is supplied.

Importantly, **at zero local credits the sensor cannot execute**; only the previously defined generic `DrawCompute` can be paid directly from the shared pool. This preserves B20/B24's fundamental trap: running out of local compute at an ordinary non-Draw opcode can strand a process despite arbitrarily large future environment energy. A signed-pool overflow traps with a charged native instruction rather than exposing a wrapping or negative resource balance.

## Frozen paid policy comparison (12 worlds)

One founder per world, **8 initial local credits**, world pool supplied exogenously with 0/4/5/6 neutral compute **before the first CIL instruction**. Each program has 13 nodes, pays the same eight startup native attempts, performs exactly one true `PatchConst` of its own PC8 resource-request literal, and stops at pool-payable PC9 `DrawCompute` with zero local credits. In SENSE, PC0 reads the actual pool; in BLIND3 and BLIND4, PC0 is replaced by a same-cost constant that forces the fixed request3 or request4. Everything else is identical. A shared-cell service marker is counted only when actually emitted as a native `CellWritten(key9,value1)` event by PC12.

| Neutral external pool at start | SENSE sensed value / adopted request | SENSE actual service outputs | Blind request3 outputs | Blind request4 outputs |
| ---: | --- | ---: | ---: | ---: |
| 0 | 0 / 4 | 0 | 0 | 0 |
| 4 | **4 / 3** | **1** | 1 | 0 |
| 5 | 5 / 4 | 1 | 1 | 1 |
| 6 | 6 / 4 | 1 | 1 | 1 |

The real native CIL source literal at PC8 changes to3 **only if** sensed pool equals4; in other regimes it remains4. All three policies pay the **same startup/edit charge**, and all post-start compute is from the common environment pool, not assigned to the actor. One packet4 can pay the pool-funded Draw3 fee1 plus transfer3 but cannot pay Draw4's fee1 plus transfer4, so native sensor-informed code beats the blind request4 policy for that packet.

**Critical negative control:** The always-request3 policy achieves **as many service outputs as SENSE under every tested environment** while leaving *more* unused pool credits at5/6. Therefore B31 has **zero demonstrated functional advantage over the best fixed policy**, despite proving access to new information and *real conditional source editing*. The comparison tests selected fixed programs, not autonomous policy search.

## Access, resource and security limits

- The native sensor is **not usable if local credits start at zero and PC0 is its instruction**. A world with 100 pool credits and a zero-credit actor does not execute a single native instruction. No zero-credit repair/wake-up was introduced.
- A world pool above signed i64 maximum causes a **charged native trap** on sensing; no silent truncation or overflow.
- At every charged native operation, `sum(local compute + neutral pool)` decreases exactly1; every explicitly exogenous packet increases the combined inventory only by its given quantity. The 12 worlds expose all source edits, actual native services and residual credit.
- `SenseComputePool` returns raw environmental resource state, **not a trusted statement that the actor is healthy or its algorithm is good**. A successful read does not solve the evaluator-corruption demonstrations from B12–B17.
- At t0 the branching threshold, candidate resource requests, mutation target and service are **all manually authored CIL instructions**. This is conditional execution with an environmental sensor, **not trial-and-error learning, endogenous objective formation or proven cyberlife**.

## Next single-individual gate

B32 should test whether a continuing individual can **pay once to simplify/redirect its own executable control flow** to preserve a reachable resource-acquisition instruction after a self-edit, and whether it recoups its one-time edit cost over multiple neutral flux episodes versus both a mutation-disabled matched program and an extremely cheap static actor. It must retain the best static baseline: otherwise all progress is selection of more complicated hand-authored programs by the experimenter.
